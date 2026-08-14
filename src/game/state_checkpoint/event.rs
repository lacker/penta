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
    if let Some(linked) = entry.permanent.reanimation_linked {
        ids.push(linked);
    }
    match entry.completion {
        EntryCompletion::SpellResolved { card, .. } => ids.push(card),
        EntryCompletion::AttachSource { source, .. } => ids.push(source),
        EntryCompletion::LandPlayed { .. } | EntryCompletion::Setup | EntryCompletion::None => {}
    }
    ids.extend(entry.permanent.blocking);
    ids.extend(entry.permanent.damage_sources.iter().copied());
    ids.extend(
        entry
            .permanent
            .temporary_granted_abilities
            .iter()
            .map(|grant| grant.source),
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
        effect: entry_replacement_locator(catalog, pending.effect)?,
    })
}

pub(super) fn applicable_replacement_snapshot(
    catalog: &CardCatalog,
    replacement: &ApplicableReplacement,
) -> Option<ApplicableReplacementSnapshot> {
    Some(ApplicableReplacementSnapshot {
        context: replacement_context_snapshot(replacement.context),
        effect: entry_replacement_locator(catalog, replacement.effect)?,
        definition: replacement.definition.0,
    })
}

pub(super) fn parse_applicable_replacement(
    snapshot: &ApplicableReplacementSnapshot,
    catalog: &CardCatalog,
) -> Result<ApplicableReplacement, String> {
    let ability = catalog_ability(catalog, &snapshot.effect.ability)
        .ok_or("entry replacement ability locator is absent from this catalog")?;
    Ok(ApplicableReplacement {
        context: parse_replacement_context_snapshot(snapshot.context)?,
        definition: CardDefinitionId(snapshot.definition),
        text: ability.text,
        effect: entry_replacement_effect(&ability)
            .ok_or("entry replacement locator does not identify an entry replacement")?,
    })
}

pub(super) fn entry_replacement_locator(
    catalog: &CardCatalog,
    expected: BattlefieldEntryReplacementEffect,
) -> Option<EntryReplacementLocator> {
    Some(EntryReplacementLocator {
        ability: ability_locator(catalog, |ability| {
            entry_replacement_effect(ability) == Some(expected)
        })?,
    })
}

pub(super) fn entry_replacement_effect(
    ability: &crate::card::AbilityDef,
) -> Option<BattlefieldEntryReplacementEffect> {
    let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
        return None;
    };
    match (definition.event, ability.declarative_effect()?) {
        (_, EffectDef::Replacement(effect)) => {
            Some(BattlefieldEntryReplacementEffect::Declarative(effect))
        }
        (
            ReplacementEventDef::EntersBattlefield,
            EffectDef::ChooseCreatureType {
                object: EffectRecipientDef::Source,
            },
        ) => Some(BattlefieldEntryReplacementEffect::ChooseCreatureType),
        (
            ReplacementEventDef::EntersBattlefield,
            EffectDef::ChooseCardName {
                object: EffectRecipientDef::Source,
            },
        ) => Some(BattlefieldEntryReplacementEffect::ChooseCardName),
        (
            ReplacementEventDef::EntersBattlefield,
            EffectDef::ChoosePlayer {
                object: EffectRecipientDef::Source,
                relation,
            },
        ) => Some(BattlefieldEntryReplacementEffect::ChoosePlayer(relation)),
        (
            ReplacementEventDef::EntersBattlefield,
            EffectDef::CopyPermanentAsItEnters {
                object,
                added_types,
            },
        ) => Some(BattlefieldEntryReplacementEffect::CopyAsItEnters {
            object,
            added_types,
        }),
        _ => None,
    }
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
