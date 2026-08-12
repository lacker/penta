#![allow(clippy::wildcard_imports)]

use super::super::procedure::draw_replacement_referenced_object_ids;
use super::*;

pub(in crate::game::state_checkpoint) fn decision_referenced_object_ids(
    continuation: &DecisionContinuation,
) -> Vec<GameObjectId> {
    let mut ids = Vec::new();
    match continuation {
        DecisionContinuation::OptionalManaPayment {
            object, context, ..
        }
        | DecisionContinuation::ManaPaymentOrElse {
            object, context, ..
        }
        | DecisionContinuation::OptionalEffect {
            object, context, ..
        }
        | DecisionContinuation::ChoosePermanentForEffect {
            object, context, ..
        }
        | DecisionContinuation::TopCardSelection {
            followup: Some((object, context, _)),
            ..
        } => extend_stack_continuation_ids(&mut ids, object, *context),
        DecisionContinuation::ChainLightning { spell, .. }
        | DecisionContinuation::Fork { spell, .. } => {
            ids.extend(referenced_object_ids(spell));
        }
        DecisionContinuation::SacrificeOfChoice {
            followup: Some(followup),
            ..
        } => extend_stack_continuation_ids(&mut ids, &followup.object, followup.context),
        DecisionContinuation::BattlefieldEntryPayment { context, .. } => {
            ids.push(context.source.object);
        }
        DecisionContinuation::BattlefieldEntryReplacement { candidates } => {
            ids.extend(
                candidates
                    .iter()
                    .map(|candidate| candidate.context.source.object),
            );
        }
        DecisionContinuation::DrawReplacement { replacements, .. } => {
            ids.extend(
                replacements
                    .iter()
                    .flat_map(draw_replacement_referenced_object_ids),
            );
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
        } => {
            extend_pending_trigger_ids(&mut ids, trigger);
            for trigger in pending {
                extend_pending_trigger_ids(&mut ids, trigger);
            }
            for batch in remaining {
                extend_trigger_batch_ids(&mut ids, batch);
            }
        }
        DecisionContinuation::SearchZone { .. }
        | DecisionContinuation::ChooseCards { .. }
        | DecisionContinuation::DiscardForEffect { .. }
        | DecisionContinuation::BasicLandTypeTextChange { .. }
        | DecisionContinuation::RecallDiscard { .. }
        | DecisionContinuation::RecallReturn { .. }
        | DecisionContinuation::Duress { .. }
        | DecisionContinuation::MiracleReveal { .. }
        | DecisionContinuation::PileSplit { .. }
        | DecisionContinuation::RevealedPileSplit { .. }
        | DecisionContinuation::RevealedPileChoice { .. }
        | DecisionContinuation::PileChoice { .. }
        | DecisionContinuation::SeparateIntoPiles { .. }
        | DecisionContinuation::ChoosePile { .. }
        | DecisionContinuation::SacrificeOfChoice { followup: None, .. }
        | DecisionContinuation::DestroyOfChoice { .. }
        | DecisionContinuation::CounterUnlessPaid { .. }
        | DecisionContinuation::GrislySalvage { .. }
        | DecisionContinuation::Balance { .. }
        | DecisionContinuation::TimeVault { .. }
        | DecisionContinuation::SylvanOffer { .. }
        | DecisionContinuation::SylvanSelect { .. }
        | DecisionContinuation::SylvanMode { .. }
        | DecisionContinuation::TetravusDetach { .. }
        | DecisionContinuation::TetravusAssemble { .. }
        | DecisionContinuation::ExileFromHand { .. }
        | DecisionContinuation::AugurOfBolas { .. }
        | DecisionContinuation::TopCardSelection { followup: None, .. }
        | DecisionContinuation::BattlefieldEntryCardName { .. }
        | DecisionContinuation::BattlefieldEntryCopy { .. }
        | DecisionContinuation::BattlefieldEntryCreatureType { .. } => {}
    }
    ids
}

fn extend_stack_continuation_ids(
    ids: &mut Vec<GameObjectId>,
    object: &super::super::StackObject,
    context: super::super::TriggerContext,
) {
    ids.extend(referenced_object_ids(object));
    ids.extend(context.object);
    ids.extend(context.chosen_objects.iter().flatten().copied());
}

fn extend_pending_trigger_ids(ids: &mut Vec<GameObjectId>, trigger: &PendingTrigger) {
    ids.push(trigger.source.object);
    ids.extend(trigger.context.object);
    ids.extend(trigger.context.chosen_objects.iter().flatten().copied());
}

fn extend_trigger_batch_ids(ids: &mut Vec<GameObjectId>, batch: &TriggerPlacementBatch) {
    for trigger in &batch.triggers {
        extend_pending_trigger_ids(ids, trigger);
    }
}

pub(super) fn effect_continuation_snapshot(
    game: &Game,
    object: &super::super::StackObject,
    context: super::super::TriggerContext,
    effect: super::super::ScopedEffect,
) -> Option<EffectContinuationSnapshot> {
    let ability = stack_ability_snapshot(game, object)?.ability_locator?;
    let definition = catalog_ability(&game.catalog, &ability)?;
    let object = detached_stack_snapshot(game, object)?;
    Some(EffectContinuationSnapshot {
        object,
        ability,
        context: trigger_context_snapshot(context),
        effect: scoped_effect_snapshot(&definition, effect)?,
    })
}

pub(super) fn parse_effect_continuation(
    snapshot: &EffectContinuationSnapshot,
    game: &Game,
) -> Result<SacrificeFollowup, String> {
    Ok(SacrificeFollowup {
        object: Box::new(parse_detached_stack(&snapshot.object, game)?),
        context: parse_trigger_context(snapshot.context)?,
        effect: catalog_scoped_effect(&game.catalog, &snapshot.ability, &snapshot.effect)
            .ok_or("effect continuation locator is absent from this catalog")?,
    })
}

pub(in crate::game::state_checkpoint) fn pending_trigger_snapshot(
    game: &Game,
    trigger: &PendingTrigger,
) -> Option<PendingTriggerSnapshot> {
    let ability = ability_locator(&game.catalog, |ability| {
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            return false;
        };
        ability.text == trigger.text
            && definition.targets == trigger.target_defs
            && ability.effect.definition == trigger.effect
            && definition.condition == trigger.condition
            && Game::ability_resolver(trigger.source.ability, ability) == trigger.resolver
    })?;
    Some(PendingTriggerSnapshot {
        id: trigger.id,
        source: AbilitySourceSnapshot {
            object: trigger.source.object.0,
            ability: ability_origin_snapshot(trigger.source.ability),
        },
        ability,
        definition: trigger.definition.0,
        owner: trigger.owner.index(),
        controller: trigger.controller.index(),
        targets: trigger
            .targets
            .iter()
            .map(target_selection_snapshot)
            .collect(),
        context: trigger_context_snapshot(trigger.context),
    })
}

pub(super) fn trigger_batch_snapshot(
    game: &Game,
    batch: &TriggerPlacementBatch,
) -> Option<TriggerPlacementBatchSnapshot> {
    Some(TriggerPlacementBatchSnapshot {
        controller: batch.controller.index(),
        triggers: batch
            .triggers
            .iter()
            .map(|trigger| pending_trigger_snapshot(game, trigger))
            .collect::<Option<Vec<_>>>()?,
    })
}

pub(in crate::game::state_checkpoint) fn parse_pending_trigger(
    snapshot: &PendingTriggerSnapshot,
    game: &Game,
) -> Result<PendingTrigger, String> {
    let ability = catalog_ability(&game.catalog, &snapshot.ability)
        .ok_or("pending trigger ability locator is absent from this catalog")?;
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        return Err("pending trigger locator does not identify a triggered ability".into());
    };
    let source = super::super::AbilitySourceRef {
        object: GameObjectId(snapshot.source.object),
        ability: ability_origin_from_snapshot(snapshot.source.ability),
    };
    Ok(PendingTrigger {
        id: snapshot.id,
        source,
        definition: CardDefinitionId(snapshot.definition),
        owner: player(snapshot.owner)?,
        controller: player(snapshot.controller)?,
        text: ability.text,
        target_defs: triggered.targets,
        targets: snapshot
            .targets
            .iter()
            .map(parse_target_selection)
            .collect::<Result<Vec<_>, _>>()?,
        effect: ability.effect.definition,
        resolver: Game::ability_resolver(source.ability, &ability),
        context: parse_trigger_context(snapshot.context)?,
        condition: triggered.condition,
    })
}

pub(super) fn parse_trigger_batch(
    snapshot: &TriggerPlacementBatchSnapshot,
    game: &Game,
) -> Result<TriggerPlacementBatch, String> {
    Ok(TriggerPlacementBatch {
        controller: player(snapshot.controller)?,
        triggers: snapshot
            .triggers
            .iter()
            .map(|trigger| parse_pending_trigger(trigger, game))
            .collect::<Result<Vec<_>, _>>()?,
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
        definition: card.definition.0,
        owner: card.owner.index(),
    }
}

pub(super) fn parse_detached_cards(
    snapshots: &[DetachedCardSnapshot],
    game: &Game,
) -> Result<Vec<super::super::CardInstance>, String> {
    snapshots
        .iter()
        .map(|snapshot| {
            card(
                GameObjectId(snapshot.object_id),
                CardDefinitionId(snapshot.definition),
                player(snapshot.owner)?,
                &game.catalog,
            )
        })
        .collect()
}

pub(super) fn game_ids(ids: &[u32]) -> Vec<GameObjectId> {
    ids.iter().copied().map(GameObjectId).collect()
}

pub(super) fn decision_option_snapshot(option: &DecisionOption) -> DecisionOptionSnapshot {
    let card = |(object, definition): (GameObjectId, CardDefinitionId)| DecisionCardSnapshot {
        object_id: object.0,
        definition: definition.0,
    };
    DecisionOptionSnapshot {
        id: option.id,
        label: option.label.clone(),
        card: option.card.map(card),
        members: option.members.iter().copied().map(card).collect(),
        ability_text: option.ability_text.clone(),
        zone: decision_zone_snapshot(option.zone),
    }
}

pub(super) fn parse_decision_option_snapshot(snapshot: &DecisionOptionSnapshot) -> DecisionOption {
    let card = |card: DecisionCardSnapshot| {
        (
            GameObjectId(card.object_id),
            CardDefinitionId(card.definition),
        )
    };
    DecisionOption {
        id: snapshot.id,
        label: snapshot.label.clone(),
        card: snapshot.card.map(card),
        members: snapshot.members.iter().copied().map(card).collect(),
        ability_text: snapshot.ability_text.clone(),
        zone: parse_decision_zone_snapshot(snapshot.zone),
    }
}

pub(super) fn pile_split_snapshot(piles: &PileSplit) -> PileSplitSnapshot {
    PileSplitSnapshot {
        resolving_controller: piles.resolving_controller.index(),
        subject: piles.subject.index(),
        first: piles.first.iter().map(decision_option_snapshot).collect(),
        second: piles.second.iter().map(decision_option_snapshot).collect(),
    }
}

pub(super) fn parse_pile_split_snapshot(snapshot: &PileSplitSnapshot) -> Result<PileSplit, String> {
    Ok(PileSplit {
        resolving_controller: player(snapshot.resolving_controller)?,
        subject: player(snapshot.subject)?,
        first: snapshot
            .first
            .iter()
            .map(parse_decision_option_snapshot)
            .collect(),
        second: snapshot
            .second
            .iter()
            .map(parse_decision_option_snapshot)
            .collect(),
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

pub(super) fn balance_task_snapshot(viewer: PlayerId, task: &BalanceTask) -> BalanceTaskSnapshot {
    BalanceTaskSnapshot {
        player: task.player.index(),
        prompt: task.prompt.clone(),
        zone: decision_zone_snapshot(task.zone),
        cards: (task.zone != DecisionZone::Hand || task.player == viewer)
            .then(|| task.cards.iter().map(detached_card_snapshot).collect()),
        count: task.count,
        action: match task.action {
            BalanceAction::Sacrifice => BalanceActionSnapshot::Sacrifice,
            BalanceAction::Discard => BalanceActionSnapshot::Discard,
        },
        cause: cause_snapshot(task.cause),
    }
}

pub(super) fn parse_balance_task(
    snapshot: &BalanceTaskSnapshot,
    game: &Game,
) -> Result<BalanceTask, String> {
    let owner = player(snapshot.player)?;
    let zone = parse_decision_zone_snapshot(snapshot.zone);
    let cards = match &snapshot.cards {
        Some(cards) => parse_detached_cards(cards, game)?,
        None if zone == DecisionZone::Hand => game.players[owner.index()].hand.clone(),
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
    }
}

pub(super) const fn parse_zone_placement(placement: ZonePlacementSnapshot) -> ZonePlacement {
    match placement {
        ZonePlacementSnapshot::Top => ZonePlacement::Top,
        ZonePlacementSnapshot::Bottom => ZonePlacement::Bottom,
    }
}

pub(super) fn mana_cost_snapshot(cost: ManaCost) -> ManaCostSnapshot {
    ManaCostSnapshot {
        generic: cost.generic,
        white: cost.white,
        blue: cost.blue,
        black: cost.black,
        red: cost.red,
        green: cost.green,
        hybrid: cost.hybrid.to_vec(),
        variable_x: cost.variable_x,
        x_multiplier: cost.x_multiplier,
    }
}

pub(super) fn parse_mana_cost(value: &ManaCostSnapshot) -> Result<ManaCost, String> {
    if value.hybrid.len() != crate::HybridPair::COUNT {
        return Err("mana cost hybrid vector has the wrong length".into());
    }
    let mut hybrid = [0; crate::HybridPair::COUNT];
    hybrid.copy_from_slice(&value.hybrid);
    Ok(ManaCost {
        generic: value.generic,
        white: value.white,
        blue: value.blue,
        black: value.black,
        red: value.red,
        green: value.green,
        hybrid,
        variable_x: value.variable_x,
        x_multiplier: value.x_multiplier,
    })
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

pub(super) const fn countered_spell_zone_snapshot(
    zone: CounteredSpellZone,
) -> CounteredSpellZoneSnapshot {
    match zone {
        CounteredSpellZone::Graveyard => CounteredSpellZoneSnapshot::Graveyard,
        CounteredSpellZone::Exile => CounteredSpellZoneSnapshot::Exile,
    }
}

pub(super) const fn parse_countered_spell_zone(
    zone: CounteredSpellZoneSnapshot,
) -> CounteredSpellZone {
    match zone {
        CounteredSpellZoneSnapshot::Graveyard => CounteredSpellZone::Graveyard,
        CounteredSpellZoneSnapshot::Exile => CounteredSpellZone::Exile,
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
