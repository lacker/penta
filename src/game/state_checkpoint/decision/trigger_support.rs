// Pending triggers as a checkpoint carries them: the ability locator that
// names what a trigger will do, the batches waiting behind it, and the parse
// that puts both back. Split out of `support.rs` for the source-size budget;
// included textually, so the imports here are that module's.

pub(in crate::game::state_checkpoint) fn pending_trigger_snapshot(
    game: &Game,
    viewer: PlayerId,
    trigger: &PendingTrigger,
) -> Option<PendingTriggerSnapshot> {
    if object_reference_requires_hidden_rebinding(game, viewer, trigger.source.object) {
        return None;
    }
    if trigger_capture_has_unrebindable_hidden_reference(
        game,
        viewer,
        &trigger.targets,
        &trigger.context,
    ) {
        return None;
    }
    let ability = ability_locator_for_origin(&game.catalog, trigger.source.ability, |ability| {
        let condition = match ability.definition {
            DeclarativeAbilityDef::Triggered(definition) => definition.condition,
            // A modal trigger that has chosen its mode carries that mode's
            // own program, and a mode is an ordinary spell ability nested
            // under the trigger. It states no intervening-if of its own,
            // which is why a modal trigger declares none either.
            DeclarativeAbilityDef::Spell(_) => None,
            DeclarativeAbilityDef::AlternativeCast(alternative)
                if alternative.kind == AlternativeCastKindDef::Miracle =>
            {
                None
            }
            _ => return false,
        };
        ability.text == trigger.text
            && ability.declarative_effect() == Some(trigger.effect)
            && condition == trigger.condition
            && Game::ability_resolver(trigger.source.ability, ability) == trigger.resolver
    })?;
    let target_definition = ability_locator(&game.catalog, |ability| {
        ability_target_defs(ability) == trigger.target_defs
    })?;
    Some(PendingTriggerSnapshot {
        id: trigger.id,
        source: AbilitySourceSnapshot {
            object: trigger.source.object.0,
            ability: ability_origin_snapshot(trigger.source.ability),
        },
        ability,
        target_definition,
        presentation: object_characteristics_snapshot(&game.catalog, trigger.presentation)?,
        owner: trigger.owner.index(),
        controller: trigger.controller.index(),
        targets: trigger
            .targets
            .iter()
            .map(target_selection_snapshot)
            .collect(),
        context: effect_resolution_context_snapshot(&trigger.context),
        x: trigger.x,
    })
}

pub(super) fn trigger_batch_snapshot(
    game: &Game,
    viewer: PlayerId,
    batch: &TriggerPlacementBatch,
) -> Option<TriggerPlacementBatchSnapshot> {
    Some(TriggerPlacementBatchSnapshot {
        controller: batch.controller.index(),
        triggers: batch
            .triggers
            .iter()
            .map(|trigger| pending_trigger_snapshot(game, viewer, trigger))
            .collect::<Option<Vec<_>>>()?,
    })
}

pub(in crate::game::state_checkpoint) fn parse_pending_trigger(
    snapshot: &PendingTriggerSnapshot,
    game: &Game,
) -> Result<PendingTrigger, String> {
    let ability = catalog_ability(&game.catalog, &snapshot.ability)
        .ok_or("pending trigger ability locator is absent from this catalog")?;
    let condition = match ability.definition {
        DeclarativeAbilityDef::Triggered(triggered) => triggered.condition,
        // The chosen mode of a modal trigger, as above.
        DeclarativeAbilityDef::Spell(_) => None,
        DeclarativeAbilityDef::AlternativeCast(alternative)
            if alternative.kind == AlternativeCastKindDef::Miracle =>
        {
            None
        }
        _ => return Err("pending trigger locator does not identify a triggered ability".into()),
    };
    let source = super::super::AbilitySourceRef {
        object: GameObjectId(snapshot.source.object),
        ability: ability_origin_from_snapshot(snapshot.source.ability),
    };
    if !super::super::semantics::ability_locator_matches_origin(&snapshot.ability, source.ability) {
        return Err("pending trigger ability locator disagrees with its origin".into());
    }
    let target_definition = catalog_ability(&game.catalog, &snapshot.target_definition)
        .ok_or("pending trigger target-definition locator is absent from this catalog")?;
    let presentation = object_characteristics_from_snapshot(&game.catalog, &snapshot.presentation)
        .ok_or("pending trigger presentation locator is absent from this catalog")?;
    Ok(PendingTrigger {
        id: snapshot.id,
        source,
        presentation,
        owner: player(snapshot.owner)?,
        controller: player(snapshot.controller)?,
        text: ability.text,
        target_defs: ability_target_defs(&target_definition).to_vec(),
        targets: snapshot
            .targets
            .iter()
            .map(parse_target_selection)
            .collect::<Result<Vec<_>, _>>()?,
        effect: ability
            .declarative_effect()
            .ok_or("pending trigger does not identify an ordinary declarative program")?,
        resolver: Game::ability_resolver(source.ability, &ability),
        context: parse_effect_resolution_context(snapshot.context.clone())?,
        condition,
        // Restored from the ability the locator found: a trigger whose mode
        // is already chosen located that mode, and a mode is not itself
        // modal.
        modes: match ability.definition {
            DeclarativeAbilityDef::Triggered(triggered) => triggered.modes,
            _ => None,
        },
        x: snapshot.x,
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
