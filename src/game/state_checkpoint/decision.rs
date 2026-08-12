use serde_json::Value;

use crate::card::{CardType, CardTypeSet, ZonePlacement};
use crate::{CardDefinitionId, ChoiceIndex, GameObjectId, ManaCost, PlayerId};

use super::super::{
    BalanceAction, BalancePhase, BalanceTask, CounteredSpellZone, DecisionContinuation,
    DecisionKind, DecisionObservation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone, PendingDecision, PendingTrigger, PileSplit,
    SacrificeFollowup, TriggerPlacementBatch,
};
use super::model::{
    AbilitySourceSnapshot, BalanceActionSnapshot, BalancePhaseSnapshot, BalanceTaskSnapshot,
    CounteredSpellZoneSnapshot, DecisionCardSnapshot, DecisionContinuationSnapshot,
    DecisionOptionSnapshot, DecisionPreferenceSnapshot, DecisionStateSnapshot,
    DecisionZoneSnapshot, DetachedCardSnapshot, DiscardChoiceSnapshot, EffectContinuationSnapshot,
    ManaCostSnapshot, PendingTriggerSnapshot, PileSplitSnapshot, ReplacementEffectContextSnapshot,
    TriggerPlacementBatchSnapshot, ZoneMoveCauseSnapshot, ZonePlacementSnapshot,
};
use super::procedure::{draw_replacement_snapshot, parse_draw_replacement};
use super::semantics::{
    ability_locator, catalog_ability, catalog_replacement_effect, catalog_scoped_effect,
    replacement_effect_locator, scoped_effect_snapshot,
};
use super::stack::{
    detached_stack_snapshot, parse_detached_stack, parse_target, parse_target_selection,
    parse_trigger_context, referenced_object_ids, stack_ability_snapshot,
    target_selection_snapshot, target_snapshot, trigger_context_snapshot,
};
use super::{
    DeclarativeAbilityDef, Game, ReplacementEffectContext, ReplacementEffectDef, ZoneMoveCause,
    ability_origin_from_snapshot, ability_origin_snapshot, applicable_replacement_snapshot, array,
    bool_field, card, field, parse_applicable_replacement, parse_zone_kind, seat_value, str_field,
    u32_field, usize_field, zone_kind_snapshot,
};

pub(super) fn decision_snapshot(
    game: &Game,
    viewer: PlayerId,
    pending: &PendingDecision,
) -> Option<DecisionStateSnapshot> {
    Some(DecisionStateSnapshot {
        preference: preference_snapshot(pending.observation.preference),
        continuation: continuation_snapshot(game, viewer, &pending.continuation)?,
    })
}

#[allow(clippy::too_many_lines)]
fn continuation_snapshot(
    game: &Game,
    viewer: PlayerId,
    continuation: &DecisionContinuation,
) -> Option<DecisionContinuationSnapshot> {
    let value = match continuation {
        DecisionContinuation::SearchZone {
            controller,
            source,
            destination,
            placement,
            reveal,
            shuffle,
        } => DecisionContinuationSnapshot::SearchZone {
            controller: controller.index(),
            source: zone_kind_snapshot(*source),
            destination: zone_kind_snapshot(*destination),
            placement: zone_placement_snapshot(*placement),
            reveal: *reveal,
            shuffle: *shuffle,
        },
        DecisionContinuation::ChooseCards {
            controller,
            destination,
            placement,
            reveal,
        } => DecisionContinuationSnapshot::ChooseCards {
            controller: controller.index(),
            destination: zone_kind_snapshot(*destination),
            placement: zone_placement_snapshot(*placement),
            reveal: *reveal,
        },
        DecisionContinuation::DrawReplacement {
            player,
            replacements,
        } => DecisionContinuationSnapshot::DrawReplacement {
            player: player.index(),
            replacements: replacements
                .iter()
                .map(|replacement| draw_replacement_snapshot(game, replacement))
                .collect::<Option<Vec<_>>>()?,
        },
        DecisionContinuation::DiscardForEffect {
            player,
            amount,
            remaining,
            chosen,
            cause,
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
        },
        DecisionContinuation::BasicLandTypeTextChange { target } => {
            DecisionContinuationSnapshot::BasicLandTypeTextChange {
                target: target_snapshot(*target),
            }
        }
        DecisionContinuation::ExileFromHand { victim } => {
            DecisionContinuationSnapshot::ExileFromHand {
                victim: victim.index(),
            }
        }
        DecisionContinuation::GrislySalvage { player, revealed } => {
            DecisionContinuationSnapshot::GrislySalvage {
                player: player.index(),
                revealed: revealed.iter().map(detached_card_snapshot).collect(),
            }
        }
        DecisionContinuation::AugurOfBolas { player, revealed } => {
            DecisionContinuationSnapshot::AugurOfBolas {
                player: player.index(),
                revealed: revealed.iter().map(detached_card_snapshot).collect(),
            }
        }
        DecisionContinuation::TopCardSelection {
            player,
            revealed,
            selected_zone,
            selected_placement,
            rest_zone,
            rest_placement,
            followup,
        } => DecisionContinuationSnapshot::TopCardSelection {
            player: player.index(),
            revealed: revealed.iter().map(detached_card_snapshot).collect(),
            selected_zone: zone_kind_snapshot(*selected_zone),
            selected_placement: zone_placement_snapshot(*selected_placement),
            rest_zone: zone_kind_snapshot(*rest_zone),
            rest_placement: zone_placement_snapshot(*rest_placement),
            followup: match followup {
                Some((object, context, effect)) => Some(effect_continuation_snapshot(
                    game, object, *context, *effect,
                )?),
                None => None,
            },
        },
        DecisionContinuation::OptionalManaPayment {
            player,
            cost,
            object,
            context,
            effect,
        } => {
            let continuation = effect_continuation_snapshot(game, object, *context, *effect)?;
            DecisionContinuationSnapshot::OptionalManaPayment {
                player: player.index(),
                cost: mana_cost_snapshot(*cost),
                object: continuation.object,
                ability: continuation.ability,
                context: continuation.context,
                effect: continuation.effect,
            }
        }
        DecisionContinuation::ManaPaymentOrElse {
            player,
            cost,
            object,
            context,
            effect,
        } => {
            let continuation = effect_continuation_snapshot(game, object, *context, *effect)?;
            DecisionContinuationSnapshot::ManaPaymentOrElse {
                player: player.index(),
                cost: mana_cost_snapshot(*cost),
                object: continuation.object,
                ability: continuation.ability,
                context: continuation.context,
                effect: continuation.effect,
            }
        }
        DecisionContinuation::ChainLightning {
            player,
            spell,
            targets,
        } => DecisionContinuationSnapshot::ChainLightning {
            player: player.index(),
            spell: detached_stack_snapshot(game, spell)?,
            targets: targets.iter().copied().map(target_snapshot).collect(),
        },
        DecisionContinuation::Fork {
            player,
            spell,
            target_lists,
        } => DecisionContinuationSnapshot::Fork {
            player: player.index(),
            spell: detached_stack_snapshot(game, spell)?,
            target_lists: target_lists
                .iter()
                .map(|targets| targets.iter().map(target_selection_snapshot).collect())
                .collect(),
        },
        DecisionContinuation::OptionalEffect {
            object,
            context,
            effect,
        } => {
            let continuation = effect_continuation_snapshot(game, object, *context, *effect)?;
            DecisionContinuationSnapshot::OptionalEffect {
                object: continuation.object,
                ability: continuation.ability,
                context: continuation.context,
                effect: continuation.effect,
            }
        }
        DecisionContinuation::ChoosePermanentForEffect {
            choice,
            object,
            context,
            effect,
        } => DecisionContinuationSnapshot::ChoosePermanentForEffect {
            choice: u8::try_from(choice.index()).ok()?,
            continuation: effect_continuation_snapshot(game, object, *context, *effect)?,
        },
        DecisionContinuation::BattlefieldEntryPayment {
            context,
            payment,
            if_paid,
            if_declined,
        } => DecisionContinuationSnapshot::BattlefieldEntryPayment {
            context: replacement_context_snapshot(*context),
            effect: replacement_effect_locator(
                &game.catalog,
                ReplacementEffectDef::OptionalPayment {
                    payment: *payment,
                    if_paid,
                    if_declined,
                },
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
        DecisionContinuation::BattlefieldEntryCardName { choices } => {
            DecisionContinuationSnapshot::BattlefieldEntryCardName {
                choices: choices.clone(),
            }
        }
        DecisionContinuation::BattlefieldEntryCreatureType { choices } => {
            DecisionContinuationSnapshot::BattlefieldEntryCreatureType {
                choices: choices.clone(),
            }
        }
        DecisionContinuation::BattlefieldEntryCopy {
            choices,
            added_types,
        } => DecisionContinuationSnapshot::BattlefieldEntryCopy {
            choices: ids(choices),
            added_types: CardType::ALL.map(|card_type| added_types.contains(card_type)),
        },
        DecisionContinuation::TriggerOrder { batch, remaining } => {
            DecisionContinuationSnapshot::TriggerOrder {
                batch: trigger_batch_snapshot(game, batch)?,
                remaining: remaining
                    .iter()
                    .map(|batch| trigger_batch_snapshot(game, batch))
                    .collect::<Option<Vec<_>>>()?,
            }
        }
        DecisionContinuation::TriggerPlacement {
            trigger,
            pending,
            remaining,
            candidates,
        } => DecisionContinuationSnapshot::TriggerPlacement {
            trigger: pending_trigger_snapshot(game, trigger)?,
            pending: pending
                .iter()
                .map(|trigger| pending_trigger_snapshot(game, trigger))
                .collect::<Option<Vec<_>>>()?,
            remaining: remaining
                .iter()
                .map(|batch| trigger_batch_snapshot(game, batch))
                .collect::<Option<Vec<_>>>()?,
            candidates: candidates.iter().copied().map(target_snapshot).collect(),
        },
        DecisionContinuation::MiracleReveal { card } => {
            DecisionContinuationSnapshot::MiracleReveal { card: card.0 }
        }
        DecisionContinuation::PileSplit { owner } => DecisionContinuationSnapshot::PileSplit {
            owner: owner.index(),
        },
        DecisionContinuation::RevealedPileSplit {
            player,
            revealed,
            rest,
            placement,
        } => DecisionContinuationSnapshot::RevealedPileSplit {
            player: player.index(),
            revealed: revealed.iter().map(detached_card_snapshot).collect(),
            rest: zone_kind_snapshot(*rest),
            placement: zone_placement_snapshot(*placement),
        },
        DecisionContinuation::RevealedPileChoice {
            player,
            first,
            second,
            rest,
            placement,
        } => DecisionContinuationSnapshot::RevealedPileChoice {
            player: player.index(),
            first: first.iter().map(detached_card_snapshot).collect(),
            second: second.iter().map(detached_card_snapshot).collect(),
            rest: zone_kind_snapshot(*rest),
            placement: zone_placement_snapshot(*placement),
        },
        DecisionContinuation::PileChoice { first, second } => {
            DecisionContinuationSnapshot::PileChoice {
                first: ids(first),
                second: ids(second),
            }
        }
        DecisionContinuation::SeparateIntoPiles {
            resolving_controller,
            subject,
            items,
            on_complete,
        } => DecisionContinuationSnapshot::SeparateIntoPiles {
            resolving_controller: resolving_controller.index(),
            subject: subject.index(),
            items: items.iter().map(decision_option_snapshot).collect(),
            on_complete: on_complete.key().to_owned(),
        },
        DecisionContinuation::ChoosePile { piles, on_complete } => {
            DecisionContinuationSnapshot::ChoosePile {
                piles: pile_split_snapshot(piles),
                on_complete: on_complete.key().to_owned(),
            }
        }
        DecisionContinuation::SacrificeOfChoice { followup, optional } => {
            DecisionContinuationSnapshot::SacrificeOfChoice {
                followup: match followup {
                    Some(followup) => Some(effect_continuation_snapshot(
                        game,
                        &followup.object,
                        followup.context,
                        followup.effect,
                    )?),
                    None => None,
                },
                optional: *optional,
            }
        }
        DecisionContinuation::DestroyOfChoice { can_regenerate } => {
            DecisionContinuationSnapshot::DestroyOfChoice {
                can_regenerate: *can_regenerate,
            }
        }
        DecisionContinuation::CounterUnlessPaid {
            spell,
            player,
            cost,
            zone,
        } => DecisionContinuationSnapshot::CounterUnlessPaid {
            spell: spell.0,
            player: player.index(),
            cost: mana_cost_snapshot(*cost),
            zone: countered_spell_zone_snapshot(*zone),
        },
        DecisionContinuation::RecallDiscard { player } => {
            DecisionContinuationSnapshot::RecallDiscard {
                player: player.index(),
            }
        }
        DecisionContinuation::RecallReturn { player } => {
            DecisionContinuationSnapshot::RecallReturn {
                player: player.index(),
            }
        }
        DecisionContinuation::Duress { victim, cause } => DecisionContinuationSnapshot::Duress {
            victim: victim.index(),
            cause: cause_snapshot(*cause),
        },
        DecisionContinuation::Balance {
            controller,
            phase,
            task,
            remaining,
        } => DecisionContinuationSnapshot::Balance {
            controller: controller.index(),
            phase: balance_phase_snapshot(*phase),
            task: balance_task_snapshot(viewer, task),
            remaining: remaining
                .iter()
                .map(|task| balance_task_snapshot(viewer, task))
                .collect(),
        },
        DecisionContinuation::TimeVault {
            permanent,
            remaining,
        } => DecisionContinuationSnapshot::TimeVault {
            permanent: permanent.0,
            remaining: ids(remaining),
        },
        DecisionContinuation::SylvanOffer { player } => DecisionContinuationSnapshot::SylvanOffer {
            player: player.index(),
        },
        DecisionContinuation::SylvanSelect {
            player,
            candidates,
            choices_left,
        } => DecisionContinuationSnapshot::SylvanSelect {
            player: player.index(),
            candidates: ids(candidates),
            choices_left: *choices_left,
        },
        DecisionContinuation::SylvanMode {
            player,
            card,
            candidates,
            choices_left,
        } => DecisionContinuationSnapshot::SylvanMode {
            player: player.index(),
            card: card.0,
            candidates: ids(candidates),
            choices_left: *choices_left,
        },
        DecisionContinuation::TetravusDetach { source } => {
            DecisionContinuationSnapshot::TetravusDetach { source: source.0 }
        }
        DecisionContinuation::TetravusAssemble { source } => {
            DecisionContinuationSnapshot::TetravusAssemble { source: source.0 }
        }
    };
    Some(value)
}

pub(super) fn parse_pending_decision(
    observation: &Value,
    state: Option<&DecisionStateSnapshot>,
    hidden: &Value,
    game: &Game,
) -> Result<Option<PendingDecision>, String> {
    let Some(visible) = observation.get("decision").filter(|value| !value.is_null()) else {
        if state.is_some() {
            return Err("checkpoint decision is not visible to its viewer".into());
        }
        return Ok(None);
    };
    let state = state.ok_or("decision continuation lacks a semantic checkpoint encoding")?;
    Ok(Some(PendingDecision {
        observation: parse_decision_observation(visible, &state.preference)?,
        continuation: parse_continuation(&state.continuation, hidden, game)?,
    }))
}

fn parse_decision_observation(
    value: &Value,
    preference: &DecisionPreferenceSnapshot,
) -> Result<DecisionObservation, String> {
    Ok(DecisionObservation {
        id: u32_field(value, "id")?,
        player: seat_value(field(value, "seat")?)?,
        kind: match str_field(value, "kind")? {
            "Choice" => DecisionKind::Choice,
            "TriggerOrder" => DecisionKind::TriggerOrder,
            "TriggerPlacement" => DecisionKind::TriggerPlacement,
            other => return Err(format!("unknown decision kind {other}")),
        },
        order_semantics: value
            .get("orderSemantics")
            .filter(|value| !value.is_null())
            .map(|value| match value.as_str() {
                Some("resolution") => Ok(DecisionOrderSemantics::Resolution),
                _ => Err("unknown decision order semantics".to_owned()),
            })
            .transpose()?,
        prompt: str_field(value, "prompt")?.to_owned(),
        visibility: match str_field(value, "visibility")? {
            "Public" => DecisionVisibility::Public,
            "Private" => DecisionVisibility::Private,
            other => return Err(format!("unknown decision visibility {other}")),
        },
        preference: parse_preference(preference)?,
        minimum: usize_field(value, "minimum")?,
        maximum: usize_field(value, "maximum")?,
        cancellable: bool_field(value, "cancellable")?,
        options: array(field(value, "options")?)?
            .iter()
            .map(parse_option)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn parse_option(value: &Value) -> Result<DecisionOption, String> {
    let parse_card = |value: &Value| {
        Ok((
            GameObjectId(u32_field(value, "objectId")?),
            CardDefinitionId(
                u16::try_from(usize_field(value, "definition")?)
                    .map_err(|_| "decision card definition is too large")?,
            ),
        ))
    };
    Ok(DecisionOption {
        id: u32_field(value, "id")?,
        label: str_field(value, "label")?.to_owned(),
        card: value
            .get("card")
            .filter(|value| !value.is_null())
            .map(parse_card)
            .transpose()?,
        members: array(field(value, "members")?)?
            .iter()
            .map(parse_card)
            .collect::<Result<Vec<_>, String>>()?,
        ability_text: value
            .get("abilityText")
            .and_then(Value::as_str)
            .map(str::to_owned),
        zone: parse_decision_zone(str_field(value, "zone")?)?,
    })
}

#[allow(clippy::too_many_lines)]
fn parse_continuation(
    value: &DecisionContinuationSnapshot,
    hidden: &Value,
    game: &Game,
) -> Result<DecisionContinuation, String> {
    Ok(match value {
        DecisionContinuationSnapshot::SearchZone {
            controller,
            source,
            destination,
            placement,
            reveal,
            shuffle,
        } => DecisionContinuation::SearchZone {
            controller: player(*controller)?,
            source: parse_zone_kind(*source),
            destination: parse_zone_kind(*destination),
            placement: parse_zone_placement(*placement),
            reveal: *reveal,
            shuffle: *shuffle,
        },
        DecisionContinuationSnapshot::ChooseCards {
            controller,
            destination,
            placement,
            reveal,
        } => DecisionContinuation::ChooseCards {
            controller: player(*controller)?,
            destination: parse_zone_kind(*destination),
            placement: parse_zone_placement(*placement),
            reveal: *reveal,
        },
        DecisionContinuationSnapshot::DrawReplacement {
            player: owner,
            replacements,
        } => DecisionContinuation::DrawReplacement {
            player: player(*owner)?,
            replacements: replacements
                .iter()
                .map(|replacement| parse_draw_replacement(replacement, game))
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::DiscardForEffect {
            player: current,
            amount,
            remaining,
            chosen,
            cause,
        } => DecisionContinuation::DiscardForEffect {
            player: player(*current)?,
            amount: *amount,
            remaining: remaining
                .iter()
                .copied()
                .map(player)
                .collect::<Result<Vec<_>, _>>()?,
            chosen: chosen
                .iter()
                .map(|choice| {
                    let owner = player(choice.player)?;
                    let cards = match &choice.cards {
                        Some(cards) => game_ids(cards),
                        None => hidden_discard_choices(hidden, owner, choice.count, game)?,
                    };
                    Ok((owner, cards))
                })
                .collect::<Result<Vec<_>, String>>()?,
            cause: parse_cause(*cause)?,
        },
        DecisionContinuationSnapshot::BasicLandTypeTextChange { target } => {
            DecisionContinuation::BasicLandTypeTextChange {
                target: parse_target(*target),
            }
        }
        DecisionContinuationSnapshot::ExileFromHand { victim } => {
            DecisionContinuation::ExileFromHand {
                victim: player(*victim)?,
            }
        }
        DecisionContinuationSnapshot::GrislySalvage {
            player: owner,
            revealed,
        } => DecisionContinuation::GrislySalvage {
            player: player(*owner)?,
            revealed: parse_detached_cards(revealed, game)?,
        },
        DecisionContinuationSnapshot::AugurOfBolas {
            player: owner,
            revealed,
        } => DecisionContinuation::AugurOfBolas {
            player: player(*owner)?,
            revealed: parse_detached_cards(revealed, game)?,
        },
        DecisionContinuationSnapshot::TopCardSelection {
            player: owner,
            revealed,
            selected_zone,
            selected_placement,
            rest_zone,
            rest_placement,
            followup,
        } => DecisionContinuation::TopCardSelection {
            player: player(*owner)?,
            revealed: parse_detached_cards(revealed, game)?,
            selected_zone: parse_zone_kind(*selected_zone),
            selected_placement: parse_zone_placement(*selected_placement),
            rest_zone: parse_zone_kind(*rest_zone),
            rest_placement: parse_zone_placement(*rest_placement),
            followup: followup
                .as_ref()
                .map(|snapshot| {
                    let followup = parse_effect_continuation(snapshot, game)?;
                    Ok::<_, String>((followup.object, followup.context, followup.effect))
                })
                .transpose()?,
        },
        DecisionContinuationSnapshot::OptionalManaPayment {
            player: owner,
            cost,
            object,
            ability,
            context,
            effect,
        } => DecisionContinuation::OptionalManaPayment {
            player: player(*owner)?,
            cost: parse_mana_cost(cost)?,
            object: Box::new(parse_detached_stack(object, game)?),
            context: parse_trigger_context(*context)?,
            effect: catalog_scoped_effect(&game.catalog, ability, effect)
                .ok_or("optional mana payment effect locator is absent from this catalog")?,
        },
        DecisionContinuationSnapshot::ManaPaymentOrElse {
            player: owner,
            cost,
            object,
            ability,
            context,
            effect,
        } => DecisionContinuation::ManaPaymentOrElse {
            player: player(*owner)?,
            cost: parse_mana_cost(cost)?,
            object: Box::new(parse_detached_stack(object, game)?),
            context: parse_trigger_context(*context)?,
            effect: catalog_scoped_effect(&game.catalog, ability, effect)
                .ok_or("mana-payment-or-else effect locator is absent from this catalog")?,
        },
        DecisionContinuationSnapshot::ChainLightning {
            player: owner,
            spell,
            targets,
        } => DecisionContinuation::ChainLightning {
            player: player(*owner)?,
            spell: parse_detached_stack(spell, game)?,
            targets: targets.iter().copied().map(parse_target).collect(),
        },
        DecisionContinuationSnapshot::Fork {
            player: owner,
            spell,
            target_lists,
        } => DecisionContinuation::Fork {
            player: player(*owner)?,
            spell: parse_detached_stack(spell, game)?,
            target_lists: target_lists
                .iter()
                .map(|targets| {
                    targets
                        .iter()
                        .map(parse_target_selection)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::OptionalEffect {
            object,
            ability,
            context,
            effect,
        } => DecisionContinuation::OptionalEffect {
            object: Box::new(parse_detached_stack(object, game)?),
            context: parse_trigger_context(*context)?,
            effect: catalog_scoped_effect(&game.catalog, ability, effect)
                .ok_or("optional effect locator is absent from this catalog")?,
        },
        DecisionContinuationSnapshot::ChoosePermanentForEffect {
            choice,
            continuation,
        } => {
            let continuation = parse_effect_continuation(continuation, game)?;
            DecisionContinuation::ChoosePermanentForEffect {
                choice: ChoiceIndex::from_index(usize::from(*choice))
                    .ok_or("choice index is out of range")?,
                object: continuation.object,
                context: continuation.context,
                effect: continuation.effect,
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryPayment { context, effect } => {
            let ReplacementEffectDef::OptionalPayment {
                payment,
                if_paid,
                if_declined,
            } = catalog_replacement_effect(&game.catalog, effect)
                .ok_or("battlefield entry payment locator is absent from this catalog")?
            else {
                return Err("battlefield entry payment locator is not an optional payment".into());
            };
            DecisionContinuation::BattlefieldEntryPayment {
                context: parse_replacement_context(*context)?,
                payment,
                if_paid,
                if_declined,
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryReplacement { candidates } => {
            DecisionContinuation::BattlefieldEntryReplacement {
                candidates: candidates
                    .iter()
                    .map(|candidate| parse_applicable_replacement(candidate, &game.catalog))
                    .collect::<Result<Vec<_>, _>>()?,
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryCardName { choices } => {
            DecisionContinuation::BattlefieldEntryCardName {
                choices: choices.clone(),
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryCreatureType { choices } => {
            DecisionContinuation::BattlefieldEntryCreatureType {
                choices: choices.clone(),
            }
        }
        DecisionContinuationSnapshot::BattlefieldEntryCopy {
            choices,
            added_types,
        } => DecisionContinuation::BattlefieldEntryCopy {
            choices: game_ids(choices),
            added_types: parse_card_type_set(*added_types),
        },
        DecisionContinuationSnapshot::TriggerOrder { batch, remaining } => {
            DecisionContinuation::TriggerOrder {
                batch: parse_trigger_batch(batch, game)?,
                remaining: remaining
                    .iter()
                    .map(|batch| parse_trigger_batch(batch, game))
                    .collect::<Result<Vec<_>, _>>()?,
            }
        }
        DecisionContinuationSnapshot::TriggerPlacement {
            trigger,
            pending,
            remaining,
            candidates,
        } => DecisionContinuation::TriggerPlacement {
            trigger: parse_pending_trigger(trigger, game)?,
            pending: pending
                .iter()
                .map(|trigger| parse_pending_trigger(trigger, game))
                .collect::<Result<Vec<_>, _>>()?,
            remaining: remaining
                .iter()
                .map(|batch| parse_trigger_batch(batch, game))
                .collect::<Result<Vec<_>, _>>()?,
            candidates: candidates.iter().copied().map(parse_target).collect(),
        },
        DecisionContinuationSnapshot::MiracleReveal { card } => {
            DecisionContinuation::MiracleReveal {
                card: GameObjectId(*card),
            }
        }
        DecisionContinuationSnapshot::PileSplit { owner } => DecisionContinuation::PileSplit {
            owner: player(*owner)?,
        },
        DecisionContinuationSnapshot::RevealedPileSplit {
            player: owner,
            revealed,
            rest,
            placement,
        } => DecisionContinuation::RevealedPileSplit {
            player: player(*owner)?,
            revealed: parse_detached_cards(revealed, game)?,
            rest: parse_zone_kind(*rest),
            placement: parse_zone_placement(*placement),
        },
        DecisionContinuationSnapshot::RevealedPileChoice {
            player: owner,
            first,
            second,
            rest,
            placement,
        } => DecisionContinuation::RevealedPileChoice {
            player: player(*owner)?,
            first: parse_detached_cards(first, game)?,
            second: parse_detached_cards(second, game)?,
            rest: parse_zone_kind(*rest),
            placement: parse_zone_placement(*placement),
        },
        DecisionContinuationSnapshot::PileChoice { first, second } => {
            DecisionContinuation::PileChoice {
                first: game_ids(first),
                second: game_ids(second),
            }
        }
        DecisionContinuationSnapshot::SeparateIntoPiles {
            resolving_controller,
            subject,
            items,
            on_complete,
        } => DecisionContinuation::SeparateIntoPiles {
            resolving_controller: player(*resolving_controller)?,
            subject: player(*subject)?,
            items: items.iter().map(parse_decision_option_snapshot).collect(),
            on_complete: crate::card::sets::piles_separated_resolver(on_complete)
                .ok_or("unknown piles-separated resolver")?,
        },
        DecisionContinuationSnapshot::ChoosePile { piles, on_complete } => {
            DecisionContinuation::ChoosePile {
                piles: parse_pile_split_snapshot(piles)?,
                on_complete: crate::card::sets::pile_chosen_resolver(on_complete)
                    .ok_or("unknown pile-chosen resolver")?,
            }
        }
        DecisionContinuationSnapshot::SacrificeOfChoice { followup, optional } => {
            DecisionContinuation::SacrificeOfChoice {
                followup: followup
                    .as_ref()
                    .map(|followup| parse_effect_continuation(followup, game))
                    .transpose()?,
                optional: *optional,
            }
        }
        DecisionContinuationSnapshot::DestroyOfChoice { can_regenerate } => {
            DecisionContinuation::DestroyOfChoice {
                can_regenerate: *can_regenerate,
            }
        }
        DecisionContinuationSnapshot::CounterUnlessPaid {
            spell,
            player: owner,
            cost,
            zone,
        } => DecisionContinuation::CounterUnlessPaid {
            spell: GameObjectId(*spell),
            player: player(*owner)?,
            cost: parse_mana_cost(cost)?,
            zone: parse_countered_spell_zone(*zone),
        },
        DecisionContinuationSnapshot::RecallDiscard { player: owner } => {
            DecisionContinuation::RecallDiscard {
                player: player(*owner)?,
            }
        }
        DecisionContinuationSnapshot::RecallReturn { player: owner } => {
            DecisionContinuation::RecallReturn {
                player: player(*owner)?,
            }
        }
        DecisionContinuationSnapshot::Duress { victim, cause } => DecisionContinuation::Duress {
            victim: player(*victim)?,
            cause: parse_cause(*cause)?,
        },
        DecisionContinuationSnapshot::Balance {
            controller,
            phase,
            task,
            remaining,
        } => DecisionContinuation::Balance {
            controller: player(*controller)?,
            phase: parse_balance_phase(*phase),
            task: parse_balance_task(task, game)?,
            remaining: remaining
                .iter()
                .map(|task| parse_balance_task(task, game))
                .collect::<Result<Vec<_>, _>>()?,
        },
        DecisionContinuationSnapshot::TimeVault {
            permanent,
            remaining,
        } => DecisionContinuation::TimeVault {
            permanent: GameObjectId(*permanent),
            remaining: game_ids(remaining),
        },
        DecisionContinuationSnapshot::SylvanOffer { player: owner } => {
            DecisionContinuation::SylvanOffer {
                player: player(*owner)?,
            }
        }
        DecisionContinuationSnapshot::SylvanSelect {
            player: owner,
            candidates,
            choices_left,
        } => DecisionContinuation::SylvanSelect {
            player: player(*owner)?,
            candidates: game_ids(candidates),
            choices_left: *choices_left,
        },
        DecisionContinuationSnapshot::SylvanMode {
            player: owner,
            card,
            candidates,
            choices_left,
        } => DecisionContinuation::SylvanMode {
            player: player(*owner)?,
            card: GameObjectId(*card),
            candidates: game_ids(candidates),
            choices_left: *choices_left,
        },
        DecisionContinuationSnapshot::TetravusDetach { source } => {
            DecisionContinuation::TetravusDetach {
                source: GameObjectId(*source),
            }
        }
        DecisionContinuationSnapshot::TetravusAssemble { source } => {
            DecisionContinuation::TetravusAssemble {
                source: GameObjectId(*source),
            }
        }
    })
}

mod support;

pub(super) use support::decision_referenced_object_ids;
#[allow(clippy::wildcard_imports)]
use support::*;
pub(super) use support::{parse_pending_trigger, pending_trigger_snapshot};
