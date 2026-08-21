use crate::GameObjectId;
use crate::card::DeclarativeAbilityDef;

use super::model::AbilitySourceSnapshot;
use super::model_trigger::{InstalledTriggerLifetimeSnapshot, InstalledTriggerSnapshot};
use super::semantics::{ability_locator_for_origin, catalog_ability};
use super::stack::{
    effect_resolution_context_snapshot, parse_effect_resolution_context, parse_target_selection,
    target_selection_snapshot, trigger_capture_has_unrebindable_hidden_reference,
};
use super::{
    AbilitySourceRef, Game, InstalledTrigger, InstalledTriggerLifetime, PlayerId, ScopedEffect,
    StackAbilityResolver, TriggerCapture, ability_origin_from_snapshot, ability_origin_snapshot,
    player_from_index,
};

pub(super) fn installed_trigger_snapshot(
    game: &Game,
    viewer: PlayerId,
    trigger: &InstalledTrigger,
) -> Option<InstalledTriggerSnapshot> {
    let capture = &trigger.capture;
    if trigger_capture_has_unrebindable_hidden_reference(
        game,
        viewer,
        &capture.targets,
        &capture.context,
    ) {
        return None;
    }
    let StackAbilityResolver::Declarative(resolver) = capture.resolver else {
        return None;
    };
    if resolver.effect != capture.effect {
        return None;
    }
    let ability = ability_locator_for_origin(&game.catalog, capture.source.ability, |ability| {
        let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
            return false;
        };
        definition.event == trigger.event
            && ability.text == capture.text
            && ability.declarative_effect() == Some(capture.effect)
            && definition.condition == capture.condition
            && matches!(
                Game::ability_resolver(capture.source.ability, ability),
                StackAbilityResolver::Declarative(candidate)
                    if candidate.effect == capture.effect
            )
    })?;
    Some(InstalledTriggerSnapshot {
        id: trigger.id,
        source: AbilitySourceSnapshot {
            object: capture.source.object.0,
            ability: ability_origin_snapshot(capture.source.ability),
        },
        ability,
        presentation: super::object_characteristics_snapshot(&game.catalog, capture.presentation)?,
        owner: capture.owner.index(),
        controller: capture.controller.index(),
        targets: capture
            .targets
            .iter()
            .map(target_selection_snapshot)
            .collect(),
        context: effect_resolution_context_snapshot(&capture.context),
        lifetime: match trigger.lifetime {
            InstalledTriggerLifetime::Once => InstalledTriggerLifetimeSnapshot::Once,
            InstalledTriggerLifetime::UntilTurn { player, turn } => {
                InstalledTriggerLifetimeSnapshot::UntilTurn {
                    seat: player.index(),
                    turn,
                }
            }
        },
        target_base: resolver.target_base,
        x: capture.x,
    })
}

pub(super) fn parse_installed_trigger(
    snapshot: &InstalledTriggerSnapshot,
    game: &Game,
) -> Result<InstalledTrigger, String> {
    let ability = catalog_ability(&game.catalog, &snapshot.ability)
        .ok_or("installed trigger ability locator is absent from this catalog")?;
    let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
        return Err("installed trigger locator does not identify a triggered ability".into());
    };
    let effect = ability
        .declarative_effect()
        .ok_or("installed trigger does not identify an ordinary declarative program")?;
    let source = AbilitySourceRef {
        object: GameObjectId(snapshot.source.object),
        ability: ability_origin_from_snapshot(snapshot.source.ability),
    };
    if !super::semantics::ability_locator_matches_origin(&snapshot.ability, source.ability) {
        return Err("installed trigger ability locator disagrees with its origin".into());
    }
    let presentation =
        super::object_characteristics_from_snapshot(&game.catalog, &snapshot.presentation)
            .ok_or("installed trigger presentation locator is absent from this catalog")?;
    let targets = snapshot
        .targets
        .iter()
        .map(parse_target_selection)
        .collect::<Result<Vec<_>, _>>()?;
    if snapshot.target_base > targets.len() {
        return Err("installed trigger target base exceeds its lexical selections".into());
    }
    Ok(InstalledTrigger {
        id: snapshot.id,
        event: triggered.event,
        capture: TriggerCapture {
            source,
            presentation,
            owner: player_from_index(snapshot.owner)?,
            controller: player_from_index(snapshot.controller)?,
            text: ability.text,
            // These are lexical selections retained from the installing
            // ability, not targets chosen again when the trigger fires.
            target_defs: Vec::new(),
            targets,
            effect,
            resolver: StackAbilityResolver::Declarative(ScopedEffect {
                effect,
                target_base: snapshot.target_base,
            }),
            context: parse_effect_resolution_context(snapshot.context.clone())?,
            condition: triggered.condition,
            x: snapshot.x,
        },
        lifetime: match snapshot.lifetime {
            InstalledTriggerLifetimeSnapshot::Once => InstalledTriggerLifetime::Once,
            InstalledTriggerLifetimeSnapshot::UntilTurn { seat, turn } => {
                InstalledTriggerLifetime::UntilTurn {
                    player: player_from_index(seat)?,
                    turn,
                }
            }
        },
    })
}
