use crate::card::{AbilityProcedureDef, CostDef, DeclarativeAbilityDef, ZoneKind};

use super::model::ResolvedOngoingEffectSnapshot;
use super::semantics::{ability_locator_for_origin, catalog_ability};
use super::stack::{
    effect_resolution_context_snapshot, parse_effect_resolution_context,
    trigger_capture_has_unrebindable_hidden_reference,
};
use super::{
    AbilitySourceRef, Game, GameObjectId, PlayerId, ResolvedOngoingEffect,
    ability_origin_from_snapshot, ability_origin_snapshot, expiration_snapshot, parse_expiration,
    player_from_index,
};

pub(super) fn ongoing_effect_snapshot(
    game: &Game,
    viewer: PlayerId,
    ongoing: &ResolvedOngoingEffect,
) -> Option<ResolvedOngoingEffectSnapshot> {
    if trigger_capture_has_unrebindable_hidden_reference(game, viewer, &[], &ongoing.context) {
        return None;
    }
    let ability = ability_locator_for_origin(&game.catalog, ongoing.source.ability, |candidate| {
        *candidate == ongoing.ability
    })?;
    Some(ResolvedOngoingEffectSnapshot {
        object_id: ongoing.source.object.0,
        origin: ability_origin_snapshot(ongoing.source.ability),
        ability,
        presentation: super::object_characteristics_snapshot(&game.catalog, ongoing.presentation)?,
        owner: ongoing.owner.index(),
        controller: ongoing.controller.index(),
        context: effect_resolution_context_snapshot(&ongoing.context),
        expiration: expiration_snapshot(ongoing.expiration),
    })
}

pub(super) fn parse_ongoing_effect(
    snapshot: &ResolvedOngoingEffectSnapshot,
    game: &Game,
) -> Result<ResolvedOngoingEffect, String> {
    let ability = catalog_ability(&game.catalog, &snapshot.ability)
        .ok_or("ongoing effect ability locator is absent from this catalog")?;
    let (definition, mana) = match ability.definition {
        DeclarativeAbilityDef::Activated(definition) => (definition, false),
        DeclarativeAbilityDef::ActivatedMana(definition) => (definition, true),
        _ => return Err("ongoing effect locator does not identify an activated ability".into()),
    };
    if definition.procedure != AbilityProcedureDef::Shared
        || definition.source_zones != [ZoneKind::Command]
        || !definition.targets.is_empty()
        || definition.modes.is_some()
        || definition.activation_limit.is_some()
        || definition.activation_permission != crate::card::ActivationPermissionDef::Controller
        || definition.condition.is_some()
        || definition.costs.as_slice().iter().any(|cost| {
            if mana {
                !matches!(cost, CostDef::PayLife(_))
            } else {
                !matches!(cost, CostDef::Mana(cost) if !cost.variable_x)
            }
        })
    {
        return Err("ongoing effect does not identify a shared command-source ability".into());
    }
    let origin = ability_origin_from_snapshot(snapshot.origin);
    if !super::semantics::ability_locator_matches_origin(&snapshot.ability, origin) {
        return Err("ongoing effect ability locator disagrees with its origin".into());
    }
    let expiration = parse_expiration(&snapshot.expiration)?;
    if expiration == super::ContinuousEffectExpiration::WhileSourceTapped {
        return Err("ongoing effect cannot have a source-tapped expiration".into());
    }
    Ok(ResolvedOngoingEffect {
        source: AbilitySourceRef {
            object: GameObjectId(snapshot.object_id),
            ability: origin,
        },
        owner: player_from_index(snapshot.owner)?,
        controller: player_from_index(snapshot.controller)?,
        presentation: super::object_characteristics_from_snapshot(
            &game.catalog,
            &snapshot.presentation,
        )
        .ok_or("ongoing effect presentation locator is absent from this catalog")?,
        ability,
        context: parse_effect_resolution_context(snapshot.context.clone())?,
        expiration,
    })
}
