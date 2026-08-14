use crate::card::{DeclarativeAbilityDef, PlayerRelation, TurnStepDef};
use crate::{CardDefinitionId, GameObjectId, PlayerId};

use super::super::{DelayedTrigger, FloatingTrigger, ScheduledTrigger, TriggerCapture};
use super::model::AbilitySourceSnapshot;
use super::model_trigger::{
    DelayedTriggerSnapshot, FloatingTriggerSnapshot, PlayerRelationSnapshot,
    ScheduledTriggerSnapshot, TurnStepSnapshot,
};
use super::semantics::{
    ability_locator, catalog_ability, catalog_scoped_effect, scoped_effect_snapshot,
};
use super::stack::{
    detached_stack_snapshot, parse_detached_stack, parse_trigger_context, stack_ability_snapshot,
    trigger_context_snapshot,
};
use super::{AbilitySourceRef, Game, ability_origin_from_snapshot, ability_origin_snapshot};

pub(super) fn delayed_trigger_snapshot(
    game: &Game,
    trigger: &DelayedTrigger,
) -> Option<DelayedTriggerSnapshot> {
    let ability = stack_ability_snapshot(game, &trigger.object)?.ability_locator?;
    let definition = catalog_ability(&game.catalog, &ability)?;
    let object = detached_stack_snapshot(game, &trigger.object)?;
    Some(DelayedTriggerSnapshot {
        object,
        ability,
        context: trigger_context_snapshot(trigger.context),
        step: turn_step_snapshot(trigger.step),
        player: player_relation_snapshot(trigger.player),
        effect: scoped_effect_snapshot(&definition, trigger.effect)?,
    })
}

pub(super) fn parse_delayed_trigger(
    snapshot: &DelayedTriggerSnapshot,
    game: &Game,
) -> Result<DelayedTrigger, String> {
    Ok(DelayedTrigger {
        object: Box::new(parse_detached_stack(&snapshot.object, game)?),
        context: parse_trigger_context(snapshot.context)?,
        step: parse_turn_step(snapshot.step),
        player: parse_player_relation(snapshot.player),
        effect: catalog_scoped_effect(&game.catalog, &snapshot.ability, &snapshot.effect)
            .ok_or("delayed trigger effect locator is absent from this catalog")?,
    })
}

pub(super) fn scheduled_trigger_snapshot(
    game: &Game,
    trigger: &ScheduledTrigger,
) -> Option<ScheduledTriggerSnapshot> {
    let capture = trigger.capture;
    let ability = ability_locator(&game.catalog, |ability| {
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            return false;
        };
        definition.event == trigger.event
            && ability.text == capture.text
            && definition.targets == capture.target_defs
            && ability.effect.definition == capture.effect
            && definition.condition == capture.condition
            && Game::ability_resolver(capture.source.ability, ability) == capture.resolver
    })?;
    Some(ScheduledTriggerSnapshot {
        id: trigger.id,
        source: AbilitySourceSnapshot {
            object: capture.source.object.0,
            ability: ability_origin_snapshot(capture.source.ability),
        },
        ability,
        definition: capture.definition.0,
        owner: capture.owner.index(),
        controller: capture.controller.index(),
        context: trigger_context_snapshot(capture.context),
    })
}

pub(super) fn parse_scheduled_trigger(
    snapshot: &ScheduledTriggerSnapshot,
    game: &Game,
) -> Result<ScheduledTrigger, String> {
    let ability = catalog_ability(&game.catalog, &snapshot.ability)
        .ok_or("scheduled trigger ability locator is absent from this catalog")?;
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        return Err("scheduled trigger locator does not identify a triggered ability".into());
    };
    let source = AbilitySourceRef {
        object: GameObjectId(snapshot.source.object),
        ability: ability_origin_from_snapshot(snapshot.source.ability),
    };
    Ok(ScheduledTrigger {
        id: snapshot.id,
        event: triggered.event,
        capture: TriggerCapture {
            source,
            definition: CardDefinitionId(snapshot.definition),
            owner: player(snapshot.owner)?,
            controller: player(snapshot.controller)?,
            text: ability.text,
            target_defs: triggered.targets,
            effect: ability.effect.definition,
            resolver: Game::ability_resolver(source.ability, &ability),
            context: parse_trigger_context(snapshot.context)?,
            condition: triggered.condition,
        },
    })
}

pub(super) fn floating_trigger_snapshot(
    game: &Game,
    trigger: &FloatingTrigger,
) -> Option<FloatingTriggerSnapshot> {
    let capture = trigger.capture;
    let ability = ability_locator(&game.catalog, |ability| {
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            return false;
        };
        definition.event == trigger.event
            && ability.text == capture.text
            && definition.targets == capture.target_defs
            && ability.effect.definition == capture.effect
            && definition.condition == capture.condition
            && Game::ability_resolver(capture.source.ability, ability) == capture.resolver
    })?;
    Some(FloatingTriggerSnapshot {
        source: AbilitySourceSnapshot {
            object: capture.source.object.0,
            ability: ability_origin_snapshot(capture.source.ability),
        },
        ability,
        definition: capture.definition.0,
        owner: capture.owner.index(),
        controller: capture.controller.index(),
        context: trigger_context_snapshot(capture.context),
        until_turn_of: trigger.until_turn_of.index(),
        created_after_turns: trigger.created_after_turns,
    })
}

pub(super) fn parse_floating_trigger(
    snapshot: &FloatingTriggerSnapshot,
    game: &Game,
) -> Result<FloatingTrigger, String> {
    let ability = catalog_ability(&game.catalog, &snapshot.ability)
        .ok_or("floating trigger ability locator is absent from this catalog")?;
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        return Err("floating trigger locator does not identify a triggered ability".into());
    };
    let source = AbilitySourceRef {
        object: GameObjectId(snapshot.source.object),
        ability: ability_origin_from_snapshot(snapshot.source.ability),
    };
    Ok(FloatingTrigger {
        event: triggered.event,
        capture: TriggerCapture {
            source,
            definition: CardDefinitionId(snapshot.definition),
            owner: player(snapshot.owner)?,
            controller: player(snapshot.controller)?,
            text: ability.text,
            target_defs: triggered.targets,
            effect: ability.effect.definition,
            resolver: Game::ability_resolver(source.ability, &ability),
            context: parse_trigger_context(snapshot.context)?,
            condition: triggered.condition,
        },
        until_turn_of: player(snapshot.until_turn_of)?,
        created_after_turns: snapshot.created_after_turns,
    })
}

const fn turn_step_snapshot(step: TurnStepDef) -> TurnStepSnapshot {
    match step {
        TurnStepDef::Untap => TurnStepSnapshot::Untap,
        TurnStepDef::Upkeep => TurnStepSnapshot::Upkeep,
        TurnStepDef::Draw => TurnStepSnapshot::Draw,
        TurnStepDef::PrecombatMain => TurnStepSnapshot::PrecombatMain,
        TurnStepDef::BeginningOfCombat => TurnStepSnapshot::BeginningOfCombat,
        TurnStepDef::DeclareAttackers => TurnStepSnapshot::DeclareAttackers,
        TurnStepDef::DeclareBlockers => TurnStepSnapshot::DeclareBlockers,
        TurnStepDef::CombatDamage => TurnStepSnapshot::CombatDamage,
        TurnStepDef::EndOfCombat => TurnStepSnapshot::EndOfCombat,
        TurnStepDef::PostcombatMain => TurnStepSnapshot::PostcombatMain,
        TurnStepDef::End => TurnStepSnapshot::End,
        TurnStepDef::Cleanup => TurnStepSnapshot::Cleanup,
    }
}

const fn parse_turn_step(step: TurnStepSnapshot) -> TurnStepDef {
    match step {
        TurnStepSnapshot::Untap => TurnStepDef::Untap,
        TurnStepSnapshot::Upkeep => TurnStepDef::Upkeep,
        TurnStepSnapshot::Draw => TurnStepDef::Draw,
        TurnStepSnapshot::PrecombatMain => TurnStepDef::PrecombatMain,
        TurnStepSnapshot::BeginningOfCombat => TurnStepDef::BeginningOfCombat,
        TurnStepSnapshot::DeclareAttackers => TurnStepDef::DeclareAttackers,
        TurnStepSnapshot::DeclareBlockers => TurnStepDef::DeclareBlockers,
        TurnStepSnapshot::CombatDamage => TurnStepDef::CombatDamage,
        TurnStepSnapshot::EndOfCombat => TurnStepDef::EndOfCombat,
        TurnStepSnapshot::PostcombatMain => TurnStepDef::PostcombatMain,
        TurnStepSnapshot::End => TurnStepDef::End,
        TurnStepSnapshot::Cleanup => TurnStepDef::Cleanup,
    }
}

const fn player_relation_snapshot(relation: PlayerRelation) -> PlayerRelationSnapshot {
    match relation {
        PlayerRelation::Any => PlayerRelationSnapshot::Any,
        PlayerRelation::You => PlayerRelationSnapshot::You,
        PlayerRelation::NotYou => PlayerRelationSnapshot::NotYou,
        PlayerRelation::Opponent => PlayerRelationSnapshot::Opponent,
        PlayerRelation::ActivePlayer => PlayerRelationSnapshot::ActivePlayer,
        PlayerRelation::NonactivePlayer => PlayerRelationSnapshot::NonactivePlayer,
        PlayerRelation::EventPlayer => PlayerRelationSnapshot::EventPlayer,
        PlayerRelation::ChosenPlayer => PlayerRelationSnapshot::ChosenPlayer,
    }
}

const fn parse_player_relation(relation: PlayerRelationSnapshot) -> PlayerRelation {
    match relation {
        PlayerRelationSnapshot::Any => PlayerRelation::Any,
        PlayerRelationSnapshot::You => PlayerRelation::You,
        PlayerRelationSnapshot::NotYou => PlayerRelation::NotYou,
        PlayerRelationSnapshot::Opponent => PlayerRelation::Opponent,
        PlayerRelationSnapshot::ActivePlayer => PlayerRelation::ActivePlayer,
        PlayerRelationSnapshot::NonactivePlayer => PlayerRelation::NonactivePlayer,
        PlayerRelationSnapshot::EventPlayer => PlayerRelation::EventPlayer,
        PlayerRelationSnapshot::ChosenPlayer => PlayerRelation::ChosenPlayer,
    }
}

fn player(index: usize) -> Result<PlayerId, String> {
    [PlayerId::One, PlayerId::Two]
        .get(index)
        .copied()
        .ok_or_else(|| "seat index must be 0 or 1".into())
}
