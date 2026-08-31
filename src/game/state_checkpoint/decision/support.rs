#![allow(clippy::wildcard_imports)]

use super::super::model::{ManaCostSnapshot, ResolvedEffectPaymentSnapshot};
use super::super::procedure::draw_replacement_referenced_object_ids;
use super::*;
use crate::card::SacrificedAmountDef;
use crate::game::{ApplicableZoneMoveReplacement, PendingBattlefieldExitBatch};
use crate::game::{ResolvedEffectPayment, SacrificeDeclined};

pub(super) fn discard_follow_up_snapshot(
    game: &Game,
    viewer: PlayerId,
    follow_up: &crate::game::decision_state::DiscardFollowUp,
    visible_rebindings: &[GameObjectId],
) -> Option<Box<super::super::model::EffectContinuationSnapshot>> {
    let mut continuation = effect_continuation_snapshot(
        game,
        viewer,
        &follow_up.object,
        &follow_up.context,
        follow_up.definition,
        visible_rebindings,
    )?;
    if follow_up.context.replaced_draw.is_some() {
        continuation.object.kind = super::super::model::StackObjectKindSnapshot::ReplacementEffect;
    }
    Some(Box::new(continuation))
}

#[allow(clippy::too_many_lines)]
pub(in crate::game::state_checkpoint) fn decision_referenced_object_ids(
    continuation: &DecisionContinuation,
) -> Vec<GameObjectId> {
    let mut ids = Vec::new();
    match continuation {
        DecisionContinuation::PregameActions { actions, .. } => {
            for action in actions {
                ids.push(action.source);
                ids.extend(action.cost_objects.iter().copied());
            }
        }
        DecisionContinuation::BeginTurn {
            applied,
            replacements,
            deferred,
            ..
        } => extend_begin_turn_ids(&mut ids, applied, replacements, deferred),
        DecisionContinuation::Endure { permanent, .. } => ids.push(*permanent),
        DecisionContinuation::ArrivingAttackerDefender { attackers, .. } => {
            ids.extend(attackers.iter().copied());
        }
        DecisionContinuation::ChooseColor {
            object, context, ..
        }
        | DecisionContinuation::ChooseCounter {
            object, context, ..
        }
        | DecisionContinuation::ChooseEffect {
            object, context, ..
        }
        | DecisionContinuation::OptionalEffect {
            object, context, ..
        }
        | DecisionContinuation::MayCastExiled {
            object, context, ..
        }
        | DecisionContinuation::PayOr {
            object, context, ..
        }
        | DecisionContinuation::LookAtObjectsForEffect {
            object, context, ..
        } => extend_stack_continuation_ids(&mut ids, object, context),
        DecisionContinuation::DrawActionWindow { card }
        | DecisionContinuation::CastSuspended { card, .. }
        | DecisionContinuation::MayCastAlternative { card, .. }
        | DecisionContinuation::MayCastGranted { card, .. } => ids.push(*card),
        DecisionContinuation::ChooseForEffect {
            object,
            context,
            candidates,
            ..
        }
        | DecisionContinuation::ChooseObjectOrderForEffect {
            object,
            context,
            candidates,
            ..
        } => {
            extend_stack_continuation_ids(&mut ids, object, context);
            ids.extend(candidates.iter().copied().filter_map(target_object_id));
        }
        DecisionContinuation::PartitionGroupForEffect {
            object,
            context,
            items,
            ..
        } => {
            extend_stack_continuation_ids(&mut ids, object, context);
            ids.extend(items.iter().copied().filter_map(target_object_id));
        }
        DecisionContinuation::ChooseGroupForEffect {
            object,
            context,
            first,
            second,
            ..
        } => {
            extend_stack_continuation_ids(&mut ids, object, context);
            ids.extend(
                first
                    .iter()
                    .chain(second)
                    .copied()
                    .filter_map(target_object_id),
            );
        }
        DecisionContinuation::ChooseOneOfEachForEffect {
            object,
            context,
            candidates,
            remaining,
            chosen,
            ..
        } => {
            extend_stack_continuation_ids(&mut ids, object, context);
            ids.extend(
                candidates
                    .iter()
                    .chain(remaining)
                    .chain(chosen)
                    .copied()
                    .filter_map(target_object_id),
            );
        }
        DecisionContinuation::ChainLightning { spell, .. }
        | DecisionContinuation::CopyStackObject { spell, .. } => {
            ids.extend(referenced_object_ids(spell));
        }
        DecisionContinuation::ChangeStackTargets {
            object,
            target_lists,
        } => {
            ids.push(*object);
            ids.extend(
                target_lists
                    .iter()
                    .flatten()
                    .flat_map(crate::TargetSelection::targets)
                    .copied()
                    .filter_map(target_object_id),
            );
        }
        DecisionContinuation::SacrificeOfChoice {
            followup: Some(followup),
            ..
        } => extend_stack_continuation_ids(&mut ids, &followup.object, &followup.context),

        DecisionContinuation::BattlefieldEntryPayment { context, .. }
        | DecisionContinuation::BattlefieldEntryOptional { context, .. } => {
            ids.push(context.source.object);
        }
        DecisionContinuation::BattlefieldEntryReplacement { candidates } => {
            ids.extend(
                candidates
                    .iter()
                    .map(|candidate| candidate.context.source.object),
            );
        }
        DecisionContinuation::DrawReplacement {
            applied,
            replacements,
            ..
        } => {
            ids.extend(applied.iter().map(|source| source.object));
            ids.extend(
                replacements
                    .iter()
                    .flat_map(draw_replacement_referenced_object_ids),
            );
        }
        DecisionContinuation::DiscardForEffect {
            follow_up: Some(follow_up),
            ..
        } => {
            extend_stack_continuation_ids(&mut ids, &follow_up.object, &follow_up.context);
        }
        DecisionContinuation::ExploredCardPlacement { revealed, .. } => ids.push(*revealed),
        DecisionContinuation::Proliferate { candidates } => {
            ids.extend(candidates.iter().copied().filter_map(target_object_id));
        }
        DecisionContinuation::CascadeCast { card, exiled, .. } => {
            ids.push(*card);
            ids.extend(exiled.iter().copied());
        }
        DecisionContinuation::BattlefieldExitReplacement { batch, candidates } => {
            extend_battlefield_exit_ids(&mut ids, batch, candidates);
        }
        DecisionContinuation::BattlefieldExitOrder { batch, .. } => {
            extend_battlefield_exit_ids(&mut ids, batch, &[]);
        }
        DecisionContinuation::ActivationCostSacrifice {
            pending, chosen, ..
        }
        | DecisionContinuation::ActivationCostTap {
            pending, chosen, ..
        }
        | DecisionContinuation::ActivationCostTapPermanents {
            pending, chosen, ..
        } => {
            ids.push(pending.source);
            ids.push(pending.source_card.id);
            ids.extend(pending.chosen_permanents.iter().copied());
            ids.extend(
                pending
                    .targets
                    .iter()
                    .flat_map(crate::TargetSelection::targets)
                    .filter_map(|target| target_object_id(*target)),
            );
            ids.extend(chosen.iter().copied());
        }
        DecisionContinuation::ActivationTargeting {
            pending,
            candidates,
        } => {
            ids.push(pending.source);
            ids.extend(pending.cost_objects.iter().copied());
            ids.extend(
                pending
                    .targets
                    .iter()
                    .flat_map(crate::TargetSelection::targets)
                    .filter_map(|target| target_object_id(*target)),
            );
            ids.extend(candidates.iter().copied().filter_map(target_object_id));
        }
        DecisionContinuation::TriggerOrder { batch, remaining } => {
            extend_trigger_batch_ids(&mut ids, batch);
            for batch in remaining {
                extend_trigger_batch_ids(&mut ids, batch);
            }
        }
        DecisionContinuation::TriggerPlacement {
            trigger,
            pending,
            remaining,
            ..
        }
        | DecisionContinuation::TriggerMode {
            trigger,
            pending,
            remaining,
            ..
        }
        | DecisionContinuation::TriggerDivision {
            trigger,
            pending,
            remaining,
            ..
        } => extend_trigger_placement_ids(&mut ids, trigger, pending, remaining),
        DecisionContinuation::SacrificeToTotalPower { object, context, .. }
        | DecisionContinuation::BasicLandTypeSubstitution { object, context, .. } => {
            extend_stack_continuation_ids(&mut ids, object, context);
        }
        DecisionContinuation::BattlefieldEntryExile {
            entering,
            candidates,
            ..
        } => {
            ids.push(*entering);
            ids.extend(candidates.iter().map(|(card, _)| *card));
        }
        // The prototype names the object its mana came from, which is a
        // provenance rather than a reference the decision has to keep alive.
        DecisionContinuation::SimultaneousChoose {
            object,
            context,
            candidates,
            chosen,
            ..
        } => {
            extend_stack_continuation_ids(&mut ids, object, context);
            ids.extend(candidates.iter().copied());
            ids.extend(chosen.iter().copied());
        }
        DecisionContinuation::ScryBottom { .. }
        | DecisionContinuation::ScryTop { .. }
        | DecisionContinuation::ChosenColorMana { .. }
        | DecisionContinuation::SearchZone { .. }
        // Nothing in a name choice is an object id.
        | DecisionContinuation::CardNameChoice { .. }
        | DecisionContinuation::ChooseCards { .. }
        | DecisionContinuation::DiscardForEffect {
            follow_up: None, ..
        }
        | DecisionContinuation::BasicLandTypeTextChange { .. }
        | DecisionContinuation::RecallDiscard { .. }
        | DecisionContinuation::RecallReturn { .. }
        | DecisionContinuation::SpellLibraryEnd { .. }
        | DecisionContinuation::SacrificeOfChoice { followup: None, .. }
        | DecisionContinuation::Balance { .. }
        | DecisionContinuation::SearchZonesAndExileRest { .. }
        | DecisionContinuation::Vote { .. }
        | DecisionContinuation::TetravusDetach { .. }
        | DecisionContinuation::TetravusAssemble { .. }
        | DecisionContinuation::BattlefieldEntryScalarChoice { .. }
        | DecisionContinuation::BattlefieldEntryCopy { .. } => {}
    }
    ids
}

fn extend_begin_turn_ids(
    ids: &mut Vec<GameObjectId>,
    applied: &[AbilitySourceRef],
    replacements: &[ApplicableBeginTurnReplacement],
    deferred: &[DeferredBeginTurnEffect],
) {
    ids.extend(applied.iter().map(|source| source.object));
    ids.extend(
        replacements
            .iter()
            .map(|replacement| replacement.source.object),
    );
    ids.extend(
        deferred
            .iter()
            .map(|effect| effect.replacement.source.object),
    );
}

fn extend_trigger_placement_ids(
    ids: &mut Vec<GameObjectId>,
    trigger: &PendingTrigger,
    pending: &[PendingTrigger],
    remaining: &[TriggerPlacementBatch],
) {
    extend_pending_trigger_ids(ids, trigger);
    for trigger in pending {
        extend_pending_trigger_ids(ids, trigger);
    }
    for batch in remaining {
        extend_trigger_batch_ids(ids, batch);
    }
}

fn extend_battlefield_exit_ids(
    ids: &mut Vec<GameObjectId>,
    batch: &PendingBattlefieldExitBatch,
    candidates: &[ApplicableZoneMoveReplacement],
) {
    ids.extend(batch.moves.iter().map(|proposed| proposed.object));
    ids.extend(
        batch
            .replacements
            .iter()
            .map(|replacement| replacement.source.object),
    );
    ids.extend(
        candidates
            .iter()
            .map(|candidate| candidate.context.source.object),
    );
}

fn extend_stack_continuation_ids(
    ids: &mut Vec<GameObjectId>,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
) {
    ids.extend(referenced_object_ids(object));
    ids.extend(resolution_context_referenced_object_ids(context));
}

fn extend_pending_trigger_ids(ids: &mut Vec<GameObjectId>, trigger: &PendingTrigger) {
    ids.push(trigger.source.object);
    ids.extend(target_selections_referenced_object_ids(&trigger.targets));
    ids.extend(resolution_context_referenced_object_ids(&trigger.context));
}

fn target_object_id(target: Target) -> Option<GameObjectId> {
    match target {
        Target::Player(_) => None,
        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
    }
}

fn extend_trigger_batch_ids(ids: &mut Vec<GameObjectId>, batch: &TriggerPlacementBatch) {
    for trigger in &batch.triggers {
        extend_pending_trigger_ids(ids, trigger);
    }
}

pub(super) fn effect_continuation_snapshot(
    game: &Game,
    viewer: PlayerId,
    object: &super::super::StackObject,
    context: &super::super::EffectResolutionContext,
    effect: super::super::ScopedEffect,
    visible_rebindings: &[GameObjectId],
) -> Option<EffectContinuationSnapshot> {
    if trigger_capture_has_unrebindable_hidden_reference_except(
        game,
        viewer,
        &[],
        context,
        visible_rebindings,
    ) {
        return None;
    }
    let ability = stack_ability_snapshot_allowing(game, viewer, object, visible_rebindings)?
        .ability_locator?;
    let definition = catalog_ability(&game.catalog, &ability)?;
    let object = detached_stack_snapshot_allowing(game, viewer, object, visible_rebindings)?;
    Some(EffectContinuationSnapshot {
        object,
        ability,
        context: effect_resolution_context_snapshot(context),
        effect: scoped_effect_snapshot(&definition, effect)?,
        // Set by the sacrifice sites, which are the only continuations that
        // read a characteristic off what was sacrificed.
        reads_toughness: false,
    })
}

pub(super) fn parse_effect_continuation(
    snapshot: &EffectContinuationSnapshot,
    game: &Game,
) -> Result<SacrificeFollowup, String> {
    Ok(SacrificeFollowup {
        object: Box::new(parse_detached_stack(&snapshot.object, game)?),
        context: parse_effect_resolution_context(snapshot.context.clone())?,
        effect: catalog_scoped_effect(&game.catalog, &snapshot.ability, &snapshot.effect)
            .ok_or("effect continuation locator is absent from this catalog")?,
        amount: if snapshot.reads_toughness {
            SacrificedAmountDef::Toughness
        } else {
            SacrificedAmountDef::Power
        },
    })
}

/// The declined branch reads nothing off a sacrificed permanent, so unlike
/// the follow-up it carries no characteristic.
pub(super) fn parse_sacrifice_declined(
    snapshot: &EffectContinuationSnapshot,
    game: &Game,
) -> Result<SacrificeDeclined, String> {
    Ok(SacrificeDeclined {
        object: Box::new(parse_detached_stack(&snapshot.object, game)?),
        context: parse_effect_resolution_context(snapshot.context.clone())?,
        effect: catalog_scoped_effect(&game.catalog, &snapshot.ability, &snapshot.effect)
            .ok_or("declined-sacrifice locator is absent from this catalog")?,
    })
}

pub(super) fn preference_snapshot(preference: DecisionPreference) -> DecisionPreferenceSnapshot {
    match preference {
        DecisionPreference::HigherCardValue => {
            DecisionPreferenceSnapshot::Name("higherCardValue".into())
        }
        DecisionPreference::LowerCardValue => {
            DecisionPreferenceSnapshot::Name("lowerCardValue".into())
        }
        DecisionPreference::BalancedPartition => {
            DecisionPreferenceSnapshot::Name("balancedPartition".into())
        }
        DecisionPreference::LinkedExileTargets => {
            DecisionPreferenceSnapshot::Name("linkedExileTargets".into())
        }
        DecisionPreference::RemovalChoice => {
            DecisionPreferenceSnapshot::Name("removalChoice".into())
        }
        DecisionPreference::PreferOption(prefer_option) => {
            DecisionPreferenceSnapshot::PreferOption { prefer_option }
        }
        DecisionPreference::Neutral => DecisionPreferenceSnapshot::Name("neutral".into()),
    }
}

pub(super) fn parse_preference(
    value: &DecisionPreferenceSnapshot,
) -> Result<DecisionPreference, String> {
    match value {
        DecisionPreferenceSnapshot::Name(name) => match name.as_str() {
            "higherCardValue" => Ok(DecisionPreference::HigherCardValue),
            "lowerCardValue" => Ok(DecisionPreference::LowerCardValue),
            "balancedPartition" => Ok(DecisionPreference::BalancedPartition),
            "linkedExileTargets" => Ok(DecisionPreference::LinkedExileTargets),
            "removalChoice" => Ok(DecisionPreference::RemovalChoice),
            "neutral" => Ok(DecisionPreference::Neutral),
            other => Err(format!("unknown decision preference {other}")),
        },
        DecisionPreferenceSnapshot::PreferOption { prefer_option } => {
            Ok(DecisionPreference::PreferOption(*prefer_option))
        }
    }
}

pub(super) fn parse_decision_zone(value: &str) -> Result<DecisionZone, String> {
    match value {
        "Hand" => Ok(DecisionZone::Hand),
        "Graveyard" => Ok(DecisionZone::Graveyard),
        "Battlefield" => Ok(DecisionZone::Battlefield),
        "Stack" => Ok(DecisionZone::Stack),
        "Library" => Ok(DecisionZone::Library),
        "Exile" => Ok(DecisionZone::Exile),
        "OutsideGame" => Ok(DecisionZone::OutsideGame),
        "Command" => Ok(DecisionZone::Command),
        "DrawnThisStep" => Ok(DecisionZone::DrawnThisStep),
        "None" => Ok(DecisionZone::None),
        other => Err(format!("unknown decision zone {other}")),
    }
}

pub(super) fn ids(ids: &[GameObjectId]) -> Vec<u32> {
    ids.iter().map(|id| id.0).collect()
}

pub(super) fn detached_card_snapshot(card: &super::super::CardInstance) -> DetachedCardSnapshot {
    DetachedCardSnapshot {
        object_id: card.id.0,
        definition: card.definition,
        owner: card.owner.index(),
    }
}

pub(super) fn parse_detached_cards(
    snapshots: &[DetachedCardSnapshot],
    game: &Game,
) -> Result<Vec<super::super::CardInstance>, String> {
    let mut object_ids = std::collections::BTreeSet::new();
    snapshots
        .iter()
        .map(|snapshot| {
            if !object_ids.insert(snapshot.object_id) {
                return Err(format!(
                    "checkpoint detached-card list repeats object id {}",
                    snapshot.object_id
                ));
            }
            card(
                GameObjectId(snapshot.object_id),
                snapshot.definition,
                player(snapshot.owner)?,
                &game.catalog,
            )
        })
        .collect()
}

pub(super) fn game_ids(ids: &[u32]) -> Vec<GameObjectId> {
    ids.iter().copied().map(GameObjectId).collect()
}

pub(super) fn decision_option_snapshot(
    catalog: &CardCatalog,
    option: &DecisionOption,
) -> Option<DecisionOptionSnapshot> {
    let snapshot_card = |(object, characteristics): (GameObjectId, ObjectCharacteristics)| {
        Some(DecisionCardSnapshot {
            object_id: object.0,
            characteristics: object_characteristics_snapshot(catalog, characteristics)?,
        })
    };
    let card = match option.card {
        Some(value) => Some(snapshot_card(value)?),
        None => None,
    };
    Some(DecisionOptionSnapshot {
        id: option.id,
        label: option.label.clone(),
        card,
        members: option
            .members
            .iter()
            .copied()
            .map(snapshot_card)
            .collect::<Option<Vec<_>>>()?,
        ability_text: option.ability_text.clone(),
        zone: decision_zone_snapshot(option.zone),
    })
}

pub(super) fn parse_card_type_set(flags: [bool; CardType::COUNT]) -> CardTypeSet {
    CardType::ALL
        .into_iter()
        .zip(flags)
        .filter(|(_, present)| *present)
        .fold(CardTypeSet::empty(), |types, (card_type, _)| {
            types.with(card_type)
        })
}

pub(super) fn balance_task_snapshot(
    catalog: &CardCatalog,
    viewer: PlayerId,
    task: &BalanceTask,
) -> Option<BalanceTaskSnapshot> {
    let cards = if task.zone != DecisionZone::Hand || task.player == viewer {
        Some(
            task.cards
                .iter()
                .copied()
                .map(|(object, characteristics)| {
                    Some(DecisionCardSnapshot {
                        object_id: object.0,
                        characteristics: object_characteristics_snapshot(catalog, characteristics)?,
                    })
                })
                .collect::<Option<Vec<_>>>()?,
        )
    } else {
        None
    };
    Some(BalanceTaskSnapshot {
        player: task.player.index(),
        prompt: task.prompt.clone(),
        zone: decision_zone_snapshot(task.zone),
        cards,
        count: task.count,
        action: match task.action {
            BalanceAction::Sacrifice => BalanceActionSnapshot::Sacrifice,
            BalanceAction::Discard => BalanceActionSnapshot::Discard,
        },
        cause: cause_snapshot(task.cause),
    })
}

pub(super) fn parse_balance_task(
    snapshot: &BalanceTaskSnapshot,
    game: &Game,
) -> Result<BalanceTask, String> {
    let owner = player(snapshot.player)?;
    let zone = parse_decision_zone_snapshot(snapshot.zone);
    let cards = match &snapshot.cards {
        Some(cards) => cards
            .iter()
            .map(|card| {
                Ok((
                    GameObjectId(card.object_id),
                    object_characteristics_from_snapshot(&game.catalog, &card.characteristics)
                        .ok_or_else(|| {
                            "Balance card characteristics are absent from this catalog".to_owned()
                        })?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?,
        None if zone == DecisionZone::Hand => game.players[owner.index()]
            .hand
            .iter()
            .map(|card| {
                (
                    card.id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )
            })
            .collect(),
        None => return Err("only hidden Balance hand tasks may omit card identities".into()),
    };
    Ok(BalanceTask {
        player: owner,
        prompt: snapshot.prompt.clone(),
        zone,
        cards,
        count: snapshot.count,
        action: match snapshot.action {
            BalanceActionSnapshot::Sacrifice => BalanceAction::Sacrifice,
            BalanceActionSnapshot::Discard => BalanceAction::Discard,
        },
        cause: parse_cause(snapshot.cause)?,
    })
}

pub(super) const fn balance_phase_snapshot(phase: BalancePhase) -> BalancePhaseSnapshot {
    match phase {
        BalancePhase::Lands => BalancePhaseSnapshot::Lands,
        BalancePhase::Hands => BalancePhaseSnapshot::Hands,
        BalancePhase::Creatures => BalancePhaseSnapshot::Creatures,
    }
}

pub(super) const fn parse_balance_phase(phase: BalancePhaseSnapshot) -> BalancePhase {
    match phase {
        BalancePhaseSnapshot::Lands => BalancePhase::Lands,
        BalancePhaseSnapshot::Hands => BalancePhase::Hands,
        BalancePhaseSnapshot::Creatures => BalancePhase::Creatures,
    }
}

pub(super) const fn decision_zone_snapshot(zone: DecisionZone) -> DecisionZoneSnapshot {
    match zone {
        DecisionZone::Hand => DecisionZoneSnapshot::Hand,
        DecisionZone::Graveyard => DecisionZoneSnapshot::Graveyard,
        DecisionZone::Battlefield => DecisionZoneSnapshot::Battlefield,
        DecisionZone::Stack => DecisionZoneSnapshot::Stack,
        DecisionZone::Library => DecisionZoneSnapshot::Library,
        DecisionZone::Exile => DecisionZoneSnapshot::Exile,
        DecisionZone::OutsideGame => DecisionZoneSnapshot::OutsideGame,
        DecisionZone::Command => DecisionZoneSnapshot::Command,
        DecisionZone::DrawnThisStep => DecisionZoneSnapshot::DrawnThisStep,
        DecisionZone::None => DecisionZoneSnapshot::None,
    }
}

pub(super) const fn parse_decision_zone_snapshot(zone: DecisionZoneSnapshot) -> DecisionZone {
    match zone {
        DecisionZoneSnapshot::Hand => DecisionZone::Hand,
        DecisionZoneSnapshot::Graveyard => DecisionZone::Graveyard,
        DecisionZoneSnapshot::Battlefield => DecisionZone::Battlefield,
        DecisionZoneSnapshot::Stack => DecisionZone::Stack,
        DecisionZoneSnapshot::Library => DecisionZone::Library,
        DecisionZoneSnapshot::Exile => DecisionZone::Exile,
        DecisionZoneSnapshot::OutsideGame => DecisionZone::OutsideGame,
        DecisionZoneSnapshot::Command => DecisionZone::Command,
        DecisionZoneSnapshot::DrawnThisStep => DecisionZone::DrawnThisStep,
        DecisionZoneSnapshot::None => DecisionZone::None,
    }
}

pub(super) const fn zone_placement_snapshot(placement: ZonePlacement) -> ZonePlacementSnapshot {
    match placement {
        ZonePlacement::Top => ZonePlacementSnapshot::Top,
        ZonePlacement::Bottom => ZonePlacementSnapshot::Bottom,
        ZonePlacement::FromTop(depth) => ZonePlacementSnapshot::FromTop(depth),
    }
}

pub(super) const fn parse_zone_placement(placement: ZonePlacementSnapshot) -> ZonePlacement {
    match placement {
        ZonePlacementSnapshot::Top => ZonePlacement::Top,
        ZonePlacementSnapshot::Bottom => ZonePlacement::Bottom,
        ZonePlacementSnapshot::FromTop(depth) => ZonePlacement::FromTop(depth),
    }
}

pub(in crate::game::state_checkpoint) fn mana_cost_snapshot(cost: ManaCost) -> ManaCostSnapshot {
    let mut additional_flexible = cost.additional_flexible.to_vec();
    while additional_flexible.last() == Some(&0) {
        additional_flexible.pop();
    }
    ManaCostSnapshot {
        generic: cost.generic,
        white: cost.white,
        blue: cost.blue,
        black: cost.black,
        red: cost.red,
        green: cost.green,
        colorless: cost.colorless,
        hybrid: cost.hybrid.to_vec(),
        // Keep the additive field sparse so an ordinary cost still matches a
        // checkpoint written before flexible-symbol storage existed.
        additional_flexible,
        variable_x: cost.variable_x,
        x_multiplier: cost.x_multiplier,
    }
}

/// The inverse of [`mana_cost_snapshot`]. A hybrid list shorter than the
/// engine's is read as far as it goes and zero beyond, so a checkpoint
/// written before a pair existed restores without one.
pub(in crate::game::state_checkpoint) fn mana_cost_from_snapshot(
    snapshot: &ManaCostSnapshot,
) -> ManaCost {
    let mut hybrid = [0; crate::card::HybridPair::COUNT];
    for (slot, amount) in hybrid.iter_mut().zip(snapshot.hybrid.iter()) {
        *slot = *amount;
    }
    let mut additional_flexible = [0; crate::card::FlexibleManaSymbol::ADDITIONAL_COUNT];
    for (slot, amount) in additional_flexible
        .iter_mut()
        .zip(snapshot.additional_flexible.iter())
    {
        *slot = *amount;
    }
    ManaCost {
        generic: snapshot.generic,
        white: snapshot.white,
        blue: snapshot.blue,
        black: snapshot.black,
        red: snapshot.red,
        green: snapshot.green,
        colorless: snapshot.colorless,
        hybrid,
        additional_flexible,
        variable_x: snapshot.variable_x,
        x_multiplier: snapshot.x_multiplier,
    }
}

pub(super) fn resolved_effect_payment_snapshot(
    payment: ResolvedEffectPayment,
) -> ResolvedEffectPaymentSnapshot {
    match payment {
        ResolvedEffectPayment::Mana(cost) => {
            ResolvedEffectPaymentSnapshot::Mana(mana_cost_snapshot(cost))
        }
        ResolvedEffectPayment::Life(amount) => ResolvedEffectPaymentSnapshot::Life(amount),
        ResolvedEffectPayment::Energy(amount) => ResolvedEffectPaymentSnapshot::Energy(amount),
        ResolvedEffectPayment::Mill(amount) => ResolvedEffectPaymentSnapshot::Mill(amount),
        ResolvedEffectPayment::Discard(amount) => ResolvedEffectPaymentSnapshot::Discard(amount),
        ResolvedEffectPayment::DiscardMatching(_) => ResolvedEffectPaymentSnapshot::DiscardMatching,
        ResolvedEffectPayment::ChosenEnergy => ResolvedEffectPaymentSnapshot::ChosenEnergy,
        ResolvedEffectPayment::ChosenGenericMana => {
            ResolvedEffectPaymentSnapshot::ChosenGenericMana
        }
        ResolvedEffectPayment::MovePermanentMatching { .. } => {
            ResolvedEffectPaymentSnapshot::ReturnPermanentMatching
        }
        ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total) => {
            ResolvedEffectPaymentSnapshot::SacrificeCreaturesWithTotalPower(total)
        }
        ResolvedEffectPayment::SacrificePermanentMatching(_) => {
            ResolvedEffectPaymentSnapshot::SacrificePermanentMatching
        }
    }
}

pub(super) const fn cause_snapshot(cause: ZoneMoveCause) -> ZoneMoveCauseSnapshot {
    match cause {
        ZoneMoveCause::Rules => ZoneMoveCauseSnapshot::Rules,
        ZoneMoveCause::Effect { controller } => ZoneMoveCauseSnapshot::Effect {
            controller: controller.index(),
        },
    }
}

pub(super) fn parse_cause(cause: ZoneMoveCauseSnapshot) -> Result<ZoneMoveCause, String> {
    match cause {
        ZoneMoveCauseSnapshot::Rules => Ok(ZoneMoveCause::Rules),
        ZoneMoveCauseSnapshot::Effect { controller } => Ok(ZoneMoveCause::Effect {
            controller: player(controller)?,
        }),
    }
}

pub(super) fn hidden_discard_choices(
    hidden: &Value,
    owner: PlayerId,
    expected: usize,
    game: &Game,
) -> Result<Vec<GameObjectId>, String> {
    let indices = hidden
        .get("decision")
        .and_then(|decision| decision.get("discardChoices"))
        .and_then(|choices| choices.get(super::super::seat_label(owner)))
        .ok_or("hidden hypothesis lacks opposing discard choices")?;
    let indices = array(indices)?;
    if indices.len() != expected {
        return Err("hidden opposing discard choice count does not match checkpoint".into());
    }
    indices
        .iter()
        .map(|index| {
            let index = index
                .as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .ok_or("hidden discard choices must be hand indices")?;
            game.players[owner.index()]
                .hand
                .get(index)
                .map(|card| card.id)
                .ok_or_else(|| format!("hidden discard hand index {index} is out of range"))
        })
        .collect()
}

pub(super) fn replacement_context_snapshot(
    context: ReplacementEffectContext,
) -> ReplacementEffectContextSnapshot {
    ReplacementEffectContextSnapshot {
        source: AbilitySourceSnapshot {
            object: context.source.object.0,
            ability: ability_origin_snapshot(context.source.ability),
        },
        controller: context.controller.index(),
    }
}

pub(super) fn parse_replacement_context(
    context: ReplacementEffectContextSnapshot,
) -> Result<ReplacementEffectContext, String> {
    Ok(ReplacementEffectContext {
        source: super::super::AbilitySourceRef {
            object: GameObjectId(context.source.object),
            ability: ability_origin_from_snapshot(context.source.ability),
        },
        controller: player(context.controller)?,
    })
}

pub(super) fn player(index: usize) -> Result<PlayerId, String> {
    match index {
        0 => Ok(PlayerId::One),
        1 => Ok(PlayerId::Two),
        _ => Err("seat index must be 0 or 1".into()),
    }
}

include!("trigger_support.rs");

#[cfg(test)]
mod tests {
    use super::mana_cost_snapshot;
    use crate::{FlexibleManaSymbol, ManaCost};

    #[test]
    fn additive_flexible_mana_snapshot_stays_sparse() {
        assert!(
            mana_cost_snapshot(ManaCost::new(2, 0))
                .additional_flexible
                .is_empty(),
            "an older ordinary-mana checkpoint defaults to this same shape",
        );

        let phyrexian =
            ManaCost::new(0, 0).with_flexible_symbol(FlexibleManaSymbol::RedPhyrexian, 1);
        let snapshot = mana_cost_snapshot(phyrexian);
        assert_eq!(
            snapshot.additional_flexible.last(),
            Some(&1),
            "only storage through the last present flexible symbol is retained",
        );
    }
}
