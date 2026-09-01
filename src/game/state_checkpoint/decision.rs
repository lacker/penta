use serde_json::Value;

use crate::card::{
    AlternativeCastKindDef, CardType, CardTypeSet, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, ReplacementChoiceDef, ReplacementEventDef, TurnKindDef, ZonePlacement,
};
use crate::{
    CardCatalog, CardDefinitionId, CardPartId, GameObjectId, ManaCost, ObjectCharacteristics,
    PlayerId,
};

use super::super::decision_offers::effect_choice_visibility;
use super::super::{
    AbilitySourceRef, ApplicableBeginTurnReplacement, BalanceAction, BalancePhase, BalanceTask,
    CastOffer, CastOfferCost, CastSourceZone, DecisionContinuation, DecisionKind,
    DecisionObservation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone, DeferredBeginTurnEffect, GameEvent, PendingDecision,
    PendingTrigger, SacrificeFollowup, ScopedEffect, Target, TriggerPlacementBatch,
};
use super::model::{
    AbilityLocator, AbilitySourceSnapshot, ApplicableBeginTurnReplacementSnapshot,
    BalanceActionSnapshot, BalancePhaseSnapshot, BalanceTaskSnapshot, CounterKindSnapshot,
    DecisionCardOriginSnapshot, DecisionCardSnapshot, DecisionContinuationSnapshot,
    DecisionOptionSnapshot, DecisionPreferenceSnapshot, DecisionStateSnapshot,
    DecisionZoneSnapshot, DeferredBeginTurnEffectSnapshot, DetachedCardSnapshot,
    DiscardChoiceSnapshot, EffectContinuationSnapshot, PendingTriggerSnapshot,
    PregameAbilityActionSnapshot, ReplacementEffectContextSnapshot, ReplacementEffectLocator,
    TargetSnapshot, TriggerPlacementBatchSnapshot, TurnKindSnapshot, ZoneMoveCauseSnapshot,
    ZonePlacementSnapshot,
};
mod option;
use option::parse_option;

use super::procedure::{draw_replacement_snapshot_allowing, parse_draw_replacement};
use super::semantics::{
    ability_locator, ability_locator_for_origin, ability_target_defs, catalog_ability,
    catalog_replacement_effect, catalog_scoped_effect, replacement_effect_locator_matches_source,
    replacement_effects, resolved_replacement_effect_locator, scoped_effect_snapshot,
};
use super::stack::{
    detached_stack_snapshot_allowing, effect_resolution_context_snapshot,
    object_reference_requires_hidden_rebinding, parse_detached_stack,
    parse_effect_resolution_context, parse_target, parse_target_selection, referenced_object_ids,
    resolution_context_referenced_object_ids, stack_ability_snapshot_allowing,
    target_selection_snapshot, target_selections_referenced_object_ids, target_snapshot,
    trigger_capture_has_unrebindable_hidden_reference,
    trigger_capture_has_unrebindable_hidden_reference_except,
};
use super::{
    DeclarativeAbilityDef, Game, ReplacementEffectContext, ReplacementEffectDef, ZoneMoveCause,
    ability_origin_from_snapshot, ability_origin_snapshot, applicable_replacement_snapshot, array,
    bool_field, card, copiable_ability_snapshot, field, object_characteristics_from_snapshot,
    object_characteristics_snapshot, parse_applicable_replacement, parse_copiable_ability,
    parse_zone_kind, seat_value, str_field, u32_field, usize_field, zone_kind_snapshot,
};

pub(super) fn decision_snapshot(
    game: &Game,
    viewer: PlayerId,
    pending: &PendingDecision,
) -> Option<DecisionStateSnapshot> {
    // A private decision is absent from this viewer's ordinary observation.
    // Serializing its continuation anyway would expose raw candidate ids and
    // effect-local bindings through the checkpoint, so fail reconstruction
    // closed for the non-choosing seat instead.
    if pending.observation.visibility == DecisionVisibility::Private
        && pending.observation.player != viewer
    {
        return None;
    }
    let card_origins = visible_decision_card_origins(game, viewer, pending);
    if decision_referenced_object_ids(&pending.continuation)
        .into_iter()
        .any(|object| {
            object_reference_requires_hidden_rebinding(game, viewer, object)
                && !card_origins
                    .iter()
                    .any(|origin| origin.object_id == object.0)
        })
    {
        return None;
    }
    let visible_rebindings = card_origins
        .iter()
        .map(|origin| GameObjectId(origin.object_id))
        .collect::<Vec<_>>();
    let options = pending
        .observation
        .options
        .iter()
        .map(|option| decision_option_snapshot(&game.catalog, option))
        .collect::<Option<Vec<_>>>()?;
    Some(DecisionStateSnapshot {
        preference: preference_snapshot(pending.observation.preference),
        options,
        card_origins,
        continuation: continuation_snapshot(
            game,
            viewer,
            &pending.continuation,
            &visible_rebindings,
        )?,
    })
}

include!("decision/card_origins.rs");

#[allow(clippy::too_many_lines)]
fn continuation_snapshot(
    game: &Game,
    viewer: PlayerId,
    continuation: &DecisionContinuation,
    visible_rebindings: &[GameObjectId],
) -> Option<DecisionContinuationSnapshot> {
    let value = match continuation {
        DecisionContinuation::PregameActions { player, actions } => {
            DecisionContinuationSnapshot::PregameActions {
                player: player.index(),
                actions: actions
                    .iter()
                    .map(|action| PregameAbilityActionSnapshot {
                        source: action.source.0,
                        ability: ability_origin_snapshot(action.ability),
                        cost_objects: action.cost_objects.iter().map(|object| object.0).collect(),
                    })
                    .collect(),
            }
        }
        DecisionContinuation::ArrivingAttackerDefender {
            player,
            defending,
            attackers,
        } => DecisionContinuationSnapshot::ArrivingAttackerDefender {
            player: player.index(),
            defending: defending.index(),
            attackers: attackers.iter().map(|attacker| attacker.0).collect(),
        },
        DecisionContinuation::ScryBottom { player, revealed } => {
            DecisionContinuationSnapshot::ScryBottom {
                player: player.index(),
                revealed: revealed.iter().map(detached_card_snapshot).collect(),
            }
        }
        DecisionContinuation::ScryTop {
            player,
            top,
            bottom,
        } => DecisionContinuationSnapshot::ScryTop {
            player: player.index(),
            top: top.iter().map(detached_card_snapshot).collect(),
            bottom: bottom.iter().map(detached_card_snapshot).collect(),
        },
        DecisionContinuation::BeginTurn {
            player,
            kind,
            applied,
            replacements,
            deferred,
        } => DecisionContinuationSnapshot::BeginTurn {
            player: player.index(),
            turn_kind: turn_kind_snapshot(*kind),
            applied: applied
                .iter()
                .copied()
                .map(ability_source_snapshot)
                .collect(),
            replacements: replacements
                .iter()
                .map(|replacement| begin_turn_replacement_snapshot(game, replacement))
                .collect::<Option<Vec<_>>>()?,
            deferred: deferred
                .iter()
                .map(|effect| deferred_begin_turn_effect_snapshot(game, effect))
                .collect::<Option<Vec<_>>>()?,
        },
        DecisionContinuation::SearchZone {
            controller,
            source,
            destination,
            placement,
            reveal,
            shuffle,
            enters_tapped,
            attached_player,
            binding,
            follow_up,
        } => DecisionContinuationSnapshot::SearchZone {
            controller: controller.index(),
            source: zone_kind_snapshot(*source),
            destination: zone_kind_snapshot(*destination),
            placement: zone_placement_snapshot(*placement),
            reveal: *reveal,
            shuffle: *shuffle,
            enters_tapped: *enters_tapped,
            attached_player: attached_player.map(PlayerId::index),
            binding: binding.map(crate::ids::ObjectSetBindingIndex::index),
            follow_up: match follow_up {
                // A search whose follow-up cannot be relocated is one this
                // format cannot carry, rather than one written down without
                // the half that matters.
                Some(follow_up) => Some(effect_continuation_snapshot(
                    game,
                    viewer,
                    &follow_up.object,
                    &follow_up.context,
                    follow_up.effect,
                    visible_rebindings,
                )?),
                None => None,
            },
        },
        DecisionContinuation::ChooseCards {
            controller,
            destination,
            placement,
            reveal,
            arrival,
        } => DecisionContinuationSnapshot::ChooseCards {
            controller: controller.index(),
            destination: zone_kind_snapshot(*destination),
            placement: zone_placement_snapshot(*placement),
            reveal: *reveal,
            arrival: match arrival {
                // As with a search's follow-up: a resolution this format
                // cannot relocate makes the whole choice uncarryable, rather
                // than one written down without the half that matters.
                Some(arrival) => Some(effect_continuation_snapshot(
                    game,
                    viewer,
                    &arrival.object,
                    &arrival.context,
                    arrival.effect,
                    visible_rebindings,
                )?),
                None => None,
            },
        },
        DecisionContinuation::DrawReplacement {
            player,
            applied,
            replacements,
        } => DecisionContinuationSnapshot::DrawReplacement {
            player: player.index(),
            applied: applied
                .iter()
                .copied()
                .map(ability_source_snapshot)
                .collect(),
            replacements: replacements
                .iter()
                .map(|replacement| {
                    draw_replacement_snapshot_allowing(
                        game,
                        viewer,
                        replacement,
                        visible_rebindings,
                    )
                })
                .collect::<Option<Vec<_>>>()?,
        },
        DecisionContinuation::DiscardForEffect {
            player,
            amount,
            remaining,
            chosen,
            cause,
            follow_up,
        } => DecisionContinuationSnapshot::DiscardForEffect {
            player: player.index(),
            amount: *amount,
            remaining: remaining.iter().copied().map(PlayerId::index).collect(),
            chosen: chosen
                .iter()
                .map(|(player, cards)| DiscardChoiceSnapshot {
                    player: player.index(),
                    cards: (*player == viewer).then(|| ids(cards)),
                    count: cards.len(),
                })
                .collect(),
            cause: cause_snapshot(*cause),
            follow_up: match follow_up.as_deref() {
                Some(follow_up) => Some(discard_follow_up_snapshot(
                    game,
                    viewer,
                    follow_up,
                    visible_rebindings,
                )?),
                None => None,
            },
        },
        DecisionContinuation::BasicLandTypeTextChange { target } => {
            DecisionContinuationSnapshot::BasicLandTypeTextChange {
                target: target_snapshot(*target),
            }
        }
        DecisionContinuation::ChainLightning {
            player,
            spell,
            targets,
        } => DecisionContinuationSnapshot::ChainLightning {
            player: player.index(),
            spell: detached_stack_snapshot_allowing(game, viewer, spell, visible_rebindings)?,
            targets: targets.iter().copied().map(target_snapshot).collect(),
        },
        DecisionContinuation::CopyStackObject {
            colors,
            remaining,
            player,
            spell,
            target_lists,
        } => DecisionContinuationSnapshot::Fork {
            repainted: colors.is_some(),
            remaining: *remaining,
            player: player.index(),
            spell: detached_stack_snapshot_allowing(game, viewer, spell, visible_rebindings)?,
            target_lists: target_lists
                .iter()
                .map(|targets| targets.iter().map(target_selection_snapshot).collect())
                .collect(),
        },
        DecisionContinuation::ChangeStackTargets {
            object,
            target_lists,
        } => DecisionContinuationSnapshot::ChangeStackTargets {
            object: object.0,
            target_lists: target_lists
                .iter()
                .map(|targets| targets.iter().map(target_selection_snapshot).collect())
                .collect(),
        },
        DecisionContinuation::Endure {
            player,
            permanent,
            amount,
        } => DecisionContinuationSnapshot::Endure {
            player: player.index(),
            permanent: permanent.0,
            amount: *amount,
        },
        DecisionContinuation::OptionalEffect {
            object,
            context,
            effect,
        } => {
            let continuation = effect_continuation_snapshot(
                game,
                viewer,
                object,
                context,
                *effect,
                visible_rebindings,
            )?;
            DecisionContinuationSnapshot::OptionalEffect {
                object: continuation.object,
                ability: continuation.ability,
                context: continuation.context,
                effect: continuation.effect,
            }
        }
        DecisionContinuation::MayCastExiled {
            player,
            card,
            object,
            context,
            definition,
            ..
        } => {
            let continuation = effect_continuation_snapshot(
                game,
                viewer,
                object,
                context,
                *definition,
                visible_rebindings,
            )?;
            DecisionContinuationSnapshot::MayCastExiled {
                player: player.index(),
                card: card.0,
                object: continuation.object,
                ability: continuation.ability,
                context: continuation.context,
                definition: continuation.effect,
            }
        }
        DecisionContinuation::CastSuspended { player, card } => {
            DecisionContinuationSnapshot::CastSuspended {
                player: player.index(),
                card: card.0,
            }
        }
        DecisionContinuation::ChooseForEffect {
            definition,
            object,
            context,
            ..
        } => {
            if !matches!(
                definition.effect,
                EffectDef::Choose(_)
                    | EffectDef::ChooseExact(_)
                    | EffectDef::ChooseCardsFromCollection(_)
            ) {
                return None;
            }
            DecisionContinuationSnapshot::ChooseForEffect {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
            }
        }
        DecisionContinuation::ChooseObjectOrderForEffect {
            definition,
            object,
            context,
            ..
        } => {
            if !matches!(definition.effect, EffectDef::ChooseObjectOrder(_)) {
                return None;
            }
            DecisionContinuationSnapshot::ChooseObjectOrderForEffect {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
            }
        }
        DecisionContinuation::LookAtObjectsForEffect {
            definition,
            object,
            context,
            ..
        } => {
            if !matches!(definition.effect, EffectDef::LookAtObjects(_)) {
                return None;
            }
            DecisionContinuationSnapshot::LookAtObjectsForEffect {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
            }
        }
        DecisionContinuation::PartitionGroupForEffect {
            definition,
            object,
            context,
            ..
        } => {
            if !matches!(definition.effect, EffectDef::PartitionGroup(_)) {
                return None;
            }
            DecisionContinuationSnapshot::PartitionGroupForEffect {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
            }
        }
        DecisionContinuation::ChooseGroupForEffect {
            definition,
            object,
            context,
            ..
        } => {
            if !matches!(definition.effect, EffectDef::ChooseGroup(_)) {
                return None;
            }
            DecisionContinuationSnapshot::ChooseGroupForEffect {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
            }
        }
        DecisionContinuation::ChooseOneOfEachForEffect {
            definition,
            next,
            remaining,
            chosen,
            object,
            context,
            ..
        } => {
            if !matches!(definition.effect, EffectDef::ChooseOneOfEach(_)) {
                return None;
            }
            DecisionContinuationSnapshot::ChooseOneOfEachForEffect {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
                next: *next,
                remaining: remaining.iter().copied().map(target_snapshot).collect(),
                chosen: chosen.iter().copied().map(target_snapshot).collect(),
            }
        }
        DecisionContinuation::SimultaneousChoose {
            definition,
            task,
            players,
            chosen,
            object,
            context,
            ..
        } => {
            if !matches!(definition.effect, EffectDef::SimultaneousChoose(_)) {
                return None;
            }
            DecisionContinuationSnapshot::SimultaneousChoose {
                continuation: effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *definition,
                    visible_rebindings,
                )?,
                task: *task,
                players: players.iter().map(|player| player.index()).collect(),
                chosen: chosen.iter().map(|id| id.0).collect(),
            }
        }
        DecisionContinuation::PayOr {
            player,
            payment,
            definition: scoped,
            object,
            context,
            ..
        } => {
            if trigger_capture_has_unrebindable_hidden_reference_except(
                game,
                viewer,
                &[],
                context,
                visible_rebindings,
            ) {
                return None;
            }
            let ability =
                stack_ability_snapshot_allowing(game, viewer, object, visible_rebindings)?
                    .ability_locator?;
            let definition = catalog_ability(&game.catalog, &ability)?;
            DecisionContinuationSnapshot::PayOr {
                player: player.index(),
                payment: resolved_effect_payment_snapshot(*payment),
                object: detached_stack_snapshot_allowing(game, viewer, object, visible_rebindings)?,
                ability,
                context: effect_resolution_context_snapshot(context),
                definition: scoped_effect_snapshot(&definition, *scoped)?,
            }
        }
        DecisionContinuation::BattlefieldEntryPayment {
            context,
            player,
            payment,
            definition,
        } => DecisionContinuationSnapshot::BattlefieldEntryPayment {
            context: replacement_context_snapshot(*context),
            player: player.index(),
            payment: resolved_effect_payment_snapshot(*payment),
            effect: resolved_replacement_effect_locator(
                &game.catalog,
                context.source,
                *definition,
            )?,
        },
        DecisionContinuation::BattlefieldEntryReplacement { candidates } => {
            DecisionContinuationSnapshot::BattlefieldEntryReplacement {
                candidates: candidates
                    .iter()
                    .map(|candidate| applicable_replacement_snapshot(&game.catalog, candidate))
                    .collect::<Option<Vec<_>>>()?,
            }
        }
        DecisionContinuation::BattlefieldEntryOptional { context, effect } => {
            DecisionContinuationSnapshot::BattlefieldEntryOptional {
                context: replacement_context_snapshot(*context),
                effect: resolved_replacement_effect_locator(
                    &game.catalog,
                    context.source,
                    *effect,
                )?,
            }
        }
        DecisionContinuation::BattlefieldEntryScalarChoice {
            context,
            choice,
            choices,
        } => DecisionContinuationSnapshot::BattlefieldEntryScalarChoice {
            context: replacement_context_snapshot(*context),
            effect: resolved_replacement_effect_locator(
                &game.catalog,
                context.source,
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(*choice)),
            )?,
            choices: choices.clone(),
        },
        DecisionContinuation::BattlefieldEntryCopy {
            choices,
            added_types,
            retain_printed_subtypes,
            base_power_toughness,
            colors,
            added_creature_types,
            no_mana_cost,
            added_abilities,
        } => DecisionContinuationSnapshot::BattlefieldEntryCopy {
            choices: ids(choices),
            added_types: CardType::ALL.map(|card_type| added_types.contains(card_type)),
            retain_printed_subtypes: *retain_printed_subtypes,
            base_power_toughness: base_power_toughness.map(|(power, toughness)| [power, toughness]),
            colors: colors.map(crate::card::ColorSet::to_flags),
            added_creature_types: added_creature_types
                .iter()
                .map(ToString::to_string)
                .collect(),
            no_mana_cost: *no_mana_cost,
            added_abilities: added_abilities
                .iter()
                .map(|ability| copiable_ability_snapshot(&game.catalog, ability))
                .collect::<Option<Vec<_>>>()?,
        },
        DecisionContinuation::TriggerOrder { batch, remaining } => {
            DecisionContinuationSnapshot::TriggerOrder {
                batch: trigger_batch_snapshot(game, viewer, batch)?,
                remaining: remaining
                    .iter()
                    .map(|batch| trigger_batch_snapshot(game, viewer, batch))
                    .collect::<Option<Vec<_>>>()?,
            }
        }
        DecisionContinuation::TriggerPlacement {
            trigger,
            pending,
            remaining,
            candidates,
        } => DecisionContinuationSnapshot::TriggerPlacement {
            trigger: pending_trigger_snapshot(game, viewer, trigger)?,
            pending: pending
                .iter()
                .map(|trigger| pending_trigger_snapshot(game, viewer, trigger))
                .collect::<Option<Vec<_>>>()?,
            remaining: remaining
                .iter()
                .map(|batch| trigger_batch_snapshot(game, viewer, batch))
                .collect::<Option<Vec<_>>>()?,
            candidates: candidates.iter().copied().map(target_snapshot).collect(),
        },
        DecisionContinuation::TriggerMode {
            trigger,
            pending,
            remaining,
            ..
        } => DecisionContinuationSnapshot::TriggerMode {
            trigger: pending_trigger_snapshot(game, viewer, trigger)?,
            pending: pending
                .iter()
                .map(|trigger| pending_trigger_snapshot(game, viewer, trigger))
                .collect::<Option<Vec<_>>>()?,
            remaining: remaining
                .iter()
                .map(|batch| trigger_batch_snapshot(game, viewer, batch))
                .collect::<Option<Vec<_>>>()?,
        },
        DecisionContinuation::TriggerDivision {
            trigger,
            pending,
            remaining,
            targets,
            divisions,
        } => DecisionContinuationSnapshot::TriggerDivision {
            trigger: pending_trigger_snapshot(game, viewer, trigger)?,
            pending: pending
                .iter()
                .map(|trigger| pending_trigger_snapshot(game, viewer, trigger))
                .collect::<Option<Vec<_>>>()?,
            remaining: remaining
                .iter()
                .map(|batch| trigger_batch_snapshot(game, viewer, batch))
                .collect::<Option<Vec<_>>>()?,
            targets: targets.iter().copied().map(target_snapshot).collect(),
            divisions: divisions.clone(),
        },
        DecisionContinuation::DrawActionWindow { card } => {
            DecisionContinuationSnapshot::DrawActionWindow { card: card.0 }
        }
        DecisionContinuation::ExploredCardPlacement { player, revealed } => {
            DecisionContinuationSnapshot::ExploredCardPlacement {
                player: player.index(),
                revealed: revealed.0,
            }
        }
        DecisionContinuation::Proliferate { candidates } => {
            DecisionContinuationSnapshot::Proliferate {
                candidates: candidates.iter().copied().map(target_snapshot).collect(),
            }
        }
        DecisionContinuation::MayCastGranted {
            player,
            card,
            ability,
            grant,
            // Recomputed on restore from where the card is, which is where
            // it came from.
            source_zone: _,
        } => DecisionContinuationSnapshot::MayCastGranted {
            player: player.index(),
            card: card.0,
            ability: ability_locator(&game.catalog, |candidate| candidate == ability)?,
            grant: *grant,
        },
        DecisionContinuation::MayCastAlternative {
            player,
            card,
            ability,
        } => DecisionContinuationSnapshot::MayCastAlternative {
            player: player.index(),
            card: card.0,
            ability: ability_origin_snapshot(*ability),
        },
        DecisionContinuation::CascadeCast {
            player,
            card,
            exiled,
        } => DecisionContinuationSnapshot::CascadeCast {
            player: player.index(),
            card: card.0,
            exiled: exiled.iter().map(|card| card.0).collect(),
        },
        DecisionContinuation::SpellLibraryEnd { owner, spell } => {
            DecisionContinuationSnapshot::SpellLibraryEnd {
                owner: owner.index(),
                spell: spell.0,
            }
        }
        DecisionContinuation::ChooseColor {
            object,
            context,
            scoped,
            targets,
            ..
        } => DecisionContinuationSnapshot::ChooseColor {
            continuation: Box::new(effect_continuation_snapshot(
                game,
                viewer,
                object,
                context,
                *scoped,
                visible_rebindings,
            )?),
            targets: targets.iter().copied().map(target_snapshot).collect(),
        },
        DecisionContinuation::ChooseCounter {
            object,
            context,
            scoped,
            target,
            kinds,
            ..
        } => DecisionContinuationSnapshot::ChooseCounter {
            continuation: Box::new(effect_continuation_snapshot(
                game,
                viewer,
                object,
                context,
                *scoped,
                visible_rebindings,
            )?),
            target: target_snapshot(*target),
            kinds: kinds.iter().copied().map(CounterKindSnapshot).collect(),
        },
        DecisionContinuation::ChooseEffect {
            object,
            context,
            scoped,
        } => DecisionContinuationSnapshot::ChooseEffect {
            continuation: Box::new(effect_continuation_snapshot(
                game,
                viewer,
                object,
                context,
                *scoped,
                visible_rebindings,
            )?),
        },
        DecisionContinuation::ChosenColorMana {
            controller,
            prototype,
            remaining,
            choosable,
        } => DecisionContinuationSnapshot::ChosenColorMana {
            controller: controller.index(),
            prototype: super::mana_snapshot(&game.catalog, *prototype),
            remaining: *remaining,
            choosable: choosable.to_flags(),
        },
        DecisionContinuation::SacrificeOfChoice {
            followup,
            declined,
            optional,
        } => {
            DecisionContinuationSnapshot::SacrificeOfChoice {
                followup: match followup {
                    Some(followup) => {
                        let mut snapshot = effect_continuation_snapshot(
                            game,
                            viewer,
                            &followup.object,
                            &followup.context,
                            followup.effect,
                            visible_rebindings,
                        )?;
                        // The only continuation that reads a characteristic
                        // off what was sacrificed, so the only one that has
                        // to say which.
                        snapshot.reads_toughness =
                            followup.amount == crate::card::SacrificedAmountDef::Toughness;
                        Some(Box::new(snapshot))
                    }
                    None => None,
                },
                declined: match declined {
                    Some(declined) => Some(Box::new(effect_continuation_snapshot(
                        game,
                        viewer,
                        &declined.object,
                        &declined.context,
                        declined.effect,
                        visible_rebindings,
                    )?)),
                    None => None,
                },
                optional: *optional,
            }
        }
        DecisionContinuation::Balance {
            controller,
            phase,
            task,
            remaining,
        } => DecisionContinuationSnapshot::Balance {
            controller: controller.index(),
            phase: balance_phase_snapshot(*phase),
            task: balance_task_snapshot(&game.catalog, viewer, task)?,
            remaining: remaining
                .iter()
                .map(|task| balance_task_snapshot(&game.catalog, viewer, task))
                .collect::<Option<Vec<_>>>()?,
        },
        DecisionContinuation::SearchZonesAndExileRest {
            player,
            zones,
            searched,
        } => DecisionContinuationSnapshot::SearchZonesAndExileRest {
            player: player.index(),
            zones: zones.iter().copied().map(zone_kind_snapshot).collect(),
            searched: ids(searched),
        },
        DecisionContinuation::Vote {
            candidates,
            remaining,
            votes,
        } => DecisionContinuationSnapshot::Vote {
            candidates: ids(candidates),
            remaining: remaining.iter().map(|player| player.index()).collect(),
            votes: ids(votes),
        },
        DecisionContinuation::CardNameChoice {
            choices,
            searched,
            zone,
            binding,
            object,
            context,
            effect,
        } => DecisionContinuationSnapshot::CardNameChoice {
            choices: choices.clone(),
            searched: searched.index(),
            zone: zone_kind_snapshot(*zone),
            binding: binding.index(),
            continuation: effect_continuation_snapshot(
                game,
                viewer,
                object,
                context,
                *effect,
                visible_rebindings,
            )?,
        },
        // A run of sacrifices is one resolution answered a creature at a
        // time, so what it carries is the resolution plus how much is still
        // owed.
        DecisionContinuation::SacrificeToTotalPower {
            player,
            remaining,
            object,
            context,
            if_paid,
        } => DecisionContinuationSnapshot::SacrificeToTotalPower {
            player: player.index(),
            remaining: *remaining,
            object: Box::new(detached_stack_snapshot_allowing(
                game,
                viewer,
                object,
                visible_rebindings,
            )?),
            context: effect_resolution_context_snapshot(context),
            if_paid: match if_paid {
                Some(effect) => Some(Box::new(effect_continuation_snapshot(
                    game,
                    viewer,
                    object,
                    context,
                    *effect,
                    visible_rebindings,
                )?)),
                None => None,
            },
        },
        // The pair is not yet chosen, so what a land substitution would do to
        // the board is not writable down either.
        DecisionContinuation::BasicLandTypeSubstitution { .. }
        // An entry paused mid-flight carries a prospective permanent that
        // this format has no place for yet.
        | DecisionContinuation::BattlefieldEntryExile { .. }
        | DecisionContinuation::BattlefieldExitReplacement { .. }
        | DecisionContinuation::BattlefieldExitOrder { .. }
        // An activation paused mid-payment carries the whole of what it
        // chose -- its frozen ability text, targets, and modes -- which this
        // format has no place for yet.
        | DecisionContinuation::ActivationCostSacrifice { .. }
        | DecisionContinuation::ActivationCostTap { .. }
        | DecisionContinuation::ActivationCostTapPermanents { .. }
        | DecisionContinuation::ActivationTargeting { .. } => return None,
    };
    Some(value)
}
include!("decision/parse_observation.rs");

include!("decision/continuation.rs");
include!("decision/battlefield_entry_continuation.rs");

include!("decision/validation.rs");

mod begin_turn;
mod support;

#[allow(clippy::wildcard_imports)]
use begin_turn::*;
pub(super) use support::decision_referenced_object_ids;
#[allow(clippy::wildcard_imports)]
use support::*;
pub(super) use support::{
    mana_cost_from_snapshot, mana_cost_snapshot, parse_pending_trigger, pending_trigger_snapshot,
};
