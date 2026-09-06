#![allow(clippy::wildcard_imports)]

use super::*;

pub(super) fn begin_turn_replacement_snapshot(
    game: &Game,
    replacement: &ApplicableBeginTurnReplacement,
) -> Option<ApplicableBeginTurnReplacementSnapshot> {
    let ability_locator =
        ability_locator_for_origin(&game.catalog, replacement.source.ability, |ability| {
            let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                return false;
            };
            matches!(definition.event, ReplacementEventDef::WouldBeginTurn { .. })
                && definition.optional == replacement.optional
                && ability.text == replacement.text
                && ability.declarative_replacement() == Some(replacement.effect)
        })?;
    let ability = catalog_ability(&game.catalog, &ability_locator)?;
    let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
        return None;
    };
    if !matches!(definition.event, ReplacementEventDef::WouldBeginTurn { .. })
        || definition.optional != replacement.optional
        || ability.text != replacement.text
        || ability.declarative_replacement() != Some(replacement.effect)
    {
        return None;
    }
    let effect_index = replacement_effects(&ability)
        .iter()
        .position(|effect| *effect == replacement.effect)?;
    Some(ApplicableBeginTurnReplacementSnapshot {
        source: ability_source_snapshot(replacement.source),
        controller: replacement.controller.index(),
        presentation: object_characteristics_snapshot(&game.catalog, replacement.presentation)?,
        effect: ReplacementEffectLocator {
            ability: ability_locator,
            effect_index,
        },
    })
}

pub(super) fn deferred_begin_turn_effect_snapshot(
    game: &Game,
    deferred: &DeferredBeginTurnEffect,
) -> Option<DeferredBeginTurnEffectSnapshot> {
    let replacement = begin_turn_replacement_snapshot(game, &deferred.replacement)?;
    let ability = catalog_ability(&game.catalog, &replacement.effect.ability)?;
    Some(DeferredBeginTurnEffectSnapshot {
        effect: scoped_effect_snapshot(&ability, ScopedEffect::primary(deferred.effect))?,
        replacement,
    })
}

pub(super) const fn ability_source_snapshot(source: AbilitySourceRef) -> AbilitySourceSnapshot {
    AbilitySourceSnapshot {
        object: source.object.0,
        ability: ability_origin_snapshot(source.ability),
    }
}

pub(super) fn parse_ability_source(source: AbilitySourceSnapshot) -> AbilitySourceRef {
    AbilitySourceRef {
        object: GameObjectId(source.object),
        ability: ability_origin_from_snapshot(source.ability),
    }
}

pub(super) const fn turn_kind_snapshot(kind: TurnKindDef) -> TurnKindSnapshot {
    match kind {
        TurnKindDef::Any => TurnKindSnapshot::Any,
        TurnKindDef::Regular => TurnKindSnapshot::Regular,
        TurnKindDef::Extra => TurnKindSnapshot::Extra,
    }
}

pub(super) const fn parse_turn_kind(kind: TurnKindSnapshot) -> TurnKindDef {
    match kind {
        TurnKindSnapshot::Any => TurnKindDef::Any,
        TurnKindSnapshot::Regular => TurnKindDef::Regular,
        TurnKindSnapshot::Extra => TurnKindDef::Extra,
    }
}

pub(super) fn parse_begin_turn_replacement(
    snapshot: &ApplicableBeginTurnReplacementSnapshot,
    game: &Game,
) -> Result<ApplicableBeginTurnReplacement, String> {
    let source = parse_ability_source(snapshot.source);
    if !super::super::semantics::ability_locator_matches_origin(
        &snapshot.effect.ability,
        source.ability,
    ) {
        return Err("begin-turn replacement locator disagrees with its source".into());
    }
    let ability = catalog_ability(&game.catalog, &snapshot.effect.ability)
        .ok_or("begin-turn replacement ability locator is absent from this catalog")?;
    let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
        return Err(
            "begin-turn replacement locator does not identify a replacement ability".into(),
        );
    };
    if !matches!(definition.event, ReplacementEventDef::WouldBeginTurn { .. }) {
        return Err("begin-turn replacement locator identifies the wrong event".into());
    }
    let effect = catalog_replacement_effect(&game.catalog, &snapshot.effect)
        .ok_or("begin-turn replacement effect locator is absent from this catalog")?;
    if ability.declarative_replacement() != Some(effect) {
        return Err("begin-turn replacement locator does not identify its root program".into());
    }
    Ok(ApplicableBeginTurnReplacement {
        source,
        controller: player(snapshot.controller)?,
        presentation: object_characteristics_from_snapshot(&game.catalog, &snapshot.presentation)
            .ok_or("begin-turn replacement presentation is absent from this catalog")?,
        text: ability.text,
        optional: definition.optional,
        effect,
    })
}

pub(super) fn parse_deferred_begin_turn_effect(
    snapshot: &DeferredBeginTurnEffectSnapshot,
    game: &Game,
) -> Result<DeferredBeginTurnEffect, String> {
    let replacement = parse_begin_turn_replacement(&snapshot.replacement, game)?;
    let effect = catalog_scoped_effect(
        &game.catalog,
        &snapshot.replacement.effect.ability,
        &snapshot.effect,
    )
    .ok_or("deferred begin-turn effect locator is absent from this catalog")?;
    if effect.target_base != 0 {
        return Err("deferred begin-turn effect has a nonzero target base".into());
    }
    Ok(DeferredBeginTurnEffect {
        replacement,
        effect: effect.effect,
    })
}
