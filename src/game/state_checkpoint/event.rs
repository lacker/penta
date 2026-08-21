#![allow(clippy::wildcard_imports)]

use super::*;

pub(super) fn pending_event_referenced_object_ids(pending: &PendingEvent) -> Vec<GameObjectId> {
    let mut ids = pending
        .applied
        .iter()
        .map(|source| source.object)
        .chain(
            pending
                .effects
                .iter()
                .map(|effect| effect.context.source.object),
        )
        .collect::<Vec<_>>();
    let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
    ids.extend(entry.permanent.created_by);
    ids.extend(entry.permanent.attached_to);
    ids.extend(entry.permanent.blocking.iter().copied());
    ids.extend(entry.permanent.damage_sources.iter().copied());
    ids.extend(
        entry
            .permanent
            .resolved_continuous_effects
            .iter()
            .map(|effect| effect.source.object),
    );
    ids
}

pub(super) fn pending_event_snapshot(
    catalog: &CardCatalog,
    pending: &PendingEvent,
) -> Option<PendingEventSnapshot> {
    let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
    Some(PendingEventSnapshot {
        entry: PendingBattlefieldEntrySnapshot {
            permanent: detached_permanent_snapshot(catalog, &entry.permanent),
            from: zone_kind_snapshot(entry.from),
            completion: completion_snapshot(entry.completion),
        },
        applied: pending
            .applied
            .iter()
            .copied()
            .map(ability_source_snapshot)
            .collect(),
        effects: pending
            .effects
            .iter()
            .map(|effect| pending_replacement_effect_snapshot(catalog, effect))
            .collect::<Option<Vec<_>>>()?,
    })
}

fn pending_replacement_effect_snapshot(
    catalog: &CardCatalog,
    pending: &PendingReplacementEffect,
) -> Option<PendingReplacementEffectSnapshot> {
    Some(PendingReplacementEffectSnapshot {
        context: replacement_context_snapshot(pending.context),
        effect: resolved_replacement_effect_locator(
            catalog,
            pending.context.source,
            pending.effect,
        )?,
    })
}

pub(super) fn applicable_replacement_snapshot(
    catalog: &CardCatalog,
    replacement: &ApplicableReplacement,
) -> Option<ApplicableReplacementSnapshot> {
    Some(ApplicableReplacementSnapshot {
        context: replacement_context_snapshot(replacement.context),
        effect: resolved_replacement_effect_locator(
            catalog,
            replacement.context.source,
            replacement.effect,
        )?,
        presentation: object_characteristics_snapshot(catalog, replacement.presentation)?,
    })
}

pub(super) fn parse_applicable_replacement(
    snapshot: &ApplicableReplacementSnapshot,
    catalog: &CardCatalog,
) -> Result<ApplicableReplacement, String> {
    let context = parse_replacement_context_snapshot(snapshot.context)?;
    if !replacement_effect_locator_matches_source(&snapshot.effect, context.source) {
        return Err("entry replacement locator disagrees with its source".into());
    }
    let ability = catalog_ability(catalog, &snapshot.effect.ability)
        .ok_or("entry replacement ability locator is absent from this catalog")?;
    let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
        return Err("entry replacement locator does not identify a replacement ability".into());
    };
    Ok(ApplicableReplacement {
        context,
        presentation: object_characteristics_from_snapshot(catalog, &snapshot.presentation)
            .ok_or("entry replacement presentation is absent from this catalog")?,
        text: ability.text,
        optional: definition.optional,
        effect: catalog_replacement_effect(catalog, &snapshot.effect)
            .filter(|_| is_entry_replacement_ability(&ability))
            .ok_or("entry replacement locator does not identify an entry replacement")?,
    })
}

fn is_entry_replacement_ability(ability: &crate::card::AbilityDef) -> bool {
    let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
        return false;
    };
    matches!(
        definition.event,
        ReplacementEventDef::SourceEntersBattlefield
            | ReplacementEventDef::ObjectEntersBattlefield { .. }
    ) && ability.declarative_replacement().is_some()
}

pub(super) fn catalog_entry_replacement_effect(
    catalog: &CardCatalog,
    locator: &ReplacementEffectLocator,
) -> Result<ReplacementEffectDef, String> {
    let ability = catalog_ability(catalog, &locator.ability)
        .ok_or("entry replacement ability locator is absent from this catalog")?;
    if !is_entry_replacement_ability(&ability) {
        return Err("locator does not identify an entry replacement ability".into());
    }
    catalog_replacement_effect(catalog, locator)
        .ok_or_else(|| "locator does not identify an entry replacement effect".into())
}

pub(super) const fn replacement_context_snapshot(
    context: ReplacementEffectContext,
) -> ReplacementEffectContextSnapshot {
    ReplacementEffectContextSnapshot {
        source: ability_source_snapshot(context.source),
        controller: context.controller.index(),
    }
}

pub(super) fn parse_replacement_context_snapshot(
    context: ReplacementEffectContextSnapshot,
) -> Result<ReplacementEffectContext, String> {
    Ok(ReplacementEffectContext {
        source: AbilitySourceRef {
            object: GameObjectId(context.source.object),
            ability: ability_origin_from_snapshot(context.source.ability),
        },
        controller: player_from_index(context.controller)?,
    })
}

pub(super) const fn ability_source_snapshot(source: AbilitySourceRef) -> AbilitySourceSnapshot {
    AbilitySourceSnapshot {
        object: source.object.0,
        ability: ability_origin_snapshot(source.ability),
    }
}
