mod ability_address;
mod emblem;
mod keyword;
mod token;
mod virtual_objects;

pub(super) use crate::card::child_effects;
pub(super) use ability_address::ability_locator;
#[cfg(test)]
pub(super) use ability_address::{
    ability_locator_index, applied_effect_locator_index, mana_payload_key,
    mana_payload_locator_index, replacement_effect_locator_index,
};
use emblem::authored_emblems;
pub(super) use emblem::{catalog_emblem_characteristics, emblem_characteristics_locator};
pub(super) use keyword::{keyword_snapshot, parse_keyword};
use token::authored_tokens;
pub(super) use token::{
    catalog_token_characteristics, face_down_characteristics_from_snapshot,
    face_down_characteristics_snapshot, object_characteristics_from_snapshot,
    object_characteristics_snapshot, token_characteristics_locator,
};

use super::model::{
    AbilityLocator, AppliedEffectLocator, ManaPayloadLocator, ReplacementEffectLocator,
    ScopedEffectSnapshot,
};
use super::model_prevention::DamagePreventionLocator;
use super::{AbilityOrigin, AbilitySourceRef, Mana, ScopedEffect};
use crate::card::{
    AbilityDef, AbilityOperationDef, AbilityProgramDef, AbilityTargetDef, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CharacteristicOperationDef, DamagePreventionDef,
    DamageSourceMatcherDef, DeclarativeAbilityDef, EffectDef, ManaSpendEffectDef,
    ObjectPredicateDef, ReplacementEffectDef, SpellAbilityDef,
};
use crate::{CardCatalog, CardPartId};

/// Locates an authored ability beneath the exact positional origin retained by
/// runtime state. Token origins do not carry a catalog definition, so their
/// root is recovered by matching the frozen ability against token rules that
/// are themselves reachable from a printed creator effect.
pub(super) fn ability_locator_for_origin(
    catalog: &CardCatalog,
    origin: AbilityOrigin,
    mut matches: impl FnMut(&AbilityDef) -> bool,
) -> Option<AbilityLocator> {
    match origin {
        AbilityOrigin::Printed {
            definition,
            part,
            ability,
        }
        | AbilityOrigin::Granted {
            source_definition: definition,
            source_part: part,
            source_ability: ability,
            ..
        } => {
            let root = AbilityLocator::Card {
                definition,
                part_id: part.0,
                ability_id: ability.0,
                nested: Vec::new(),
            };
            locate_beneath_root(catalog, root, &mut matches)
        }
        AbilityOrigin::Token { part, ability }
        | AbilityOrigin::TokenGranted {
            source_part: part,
            source_ability: ability,
            ..
        } => authored_tokens(catalog)
            .into_iter()
            .find_map(|(token, token_locator)| {
                token.part(part)?.rules().ability(ability)?;
                locate_beneath_root(
                    catalog,
                    AbilityLocator::Token {
                        token: token_locator,
                        part_id: part.0,
                        ability_id: ability.0,
                        nested: Vec::new(),
                    },
                    &mut matches,
                )
            }),
        AbilityOrigin::Emblem { ability }
        | AbilityOrigin::EmblemGranted {
            source_ability: ability,
            ..
        } => authored_emblems(catalog)
            .into_iter()
            .find_map(|(emblem, emblem_locator)| {
                emblem.ability(ability)?;
                locate_beneath_root(
                    catalog,
                    AbilityLocator::Emblem {
                        emblem: emblem_locator,
                        ability_id: ability.0,
                        nested: Vec::new(),
                    },
                    &mut matches,
                )
            }),
        AbilityOrigin::FaceDown { .. }
        | AbilityOrigin::FaceDownGranted { .. }
        | AbilityOrigin::IntrinsicBasicLand(_)
        | AbilityOrigin::IntrinsicCounter(_) => None,
    }
}

fn locate_beneath_root(
    catalog: &CardCatalog,
    root: AbilityLocator,
    matches: &mut impl FnMut(&AbilityDef) -> bool,
) -> Option<AbilityLocator> {
    let definition = catalog_ability(catalog, &root)?;
    let mut nested = Vec::new();
    locate_ability(&definition, matches, &mut nested).then(|| with_nested(root, nested))
}

pub(super) fn catalog_ability(
    catalog: &CardCatalog,
    locator: &AbilityLocator,
) -> Option<AbilityDef> {
    let (mut current, nested) = match locator {
        AbilityLocator::Card {
            definition,
            part_id,
            ability_id,
            nested,
        } => (
            *catalog
                .get(*definition)?
                .part(CardPartId(*part_id))?
                .rules
                .ability(crate::AbilityId(*ability_id))?,
            nested,
        ),
        AbilityLocator::Token {
            token,
            part_id,
            ability_id,
            nested,
        } => (
            *catalog_token_characteristics(catalog, token)?
                .part(CardPartId(*part_id))?
                .rules()
                .ability(crate::AbilityId(*ability_id))?,
            nested,
        ),
        AbilityLocator::Emblem {
            emblem,
            ability_id,
            nested,
        } => (
            catalog_emblem_characteristics(catalog, emblem)?
                .ability(crate::AbilityId(*ability_id))?,
            nested,
        ),
    };
    for &index in nested {
        current = **child_abilities(&current).get(index)?;
    }
    Some(current)
}

pub(super) fn ability_locator_matches_origin(
    locator: &AbilityLocator,
    origin: AbilityOrigin,
) -> bool {
    match (locator, origin) {
        (
            AbilityLocator::Card {
                definition,
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::Printed {
                definition: expected_definition,
                part,
                ability,
            },
        ) => *definition == expected_definition && *part_id == part.0 && *ability_id == ability.0,
        (
            AbilityLocator::Card {
                definition,
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::Granted {
                source_definition,
                source_part,
                source_ability,
                ..
            },
        ) => {
            *definition == source_definition
                && *part_id == source_part.0
                && *ability_id == source_ability.0
        }
        (
            AbilityLocator::Token {
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::Token { part, ability },
        ) => *part_id == part.0 && *ability_id == ability.0,
        (
            AbilityLocator::Token {
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::TokenGranted {
                source_part,
                source_ability,
                ..
            },
        ) => *part_id == source_part.0 && *ability_id == source_ability.0,
        (AbilityLocator::Emblem { ability_id, .. }, AbilityOrigin::Emblem { ability }) => {
            *ability_id == ability.0
        }
        (
            AbilityLocator::Emblem { ability_id, .. },
            AbilityOrigin::EmblemGranted { source_ability, .. },
        ) => *ability_id == source_ability.0,
        _ => false,
    }
}

fn with_nested(locator: AbilityLocator, nested: Vec<usize>) -> AbilityLocator {
    match locator {
        AbilityLocator::Card {
            definition,
            part_id,
            ability_id,
            ..
        } => AbilityLocator::Card {
            definition,
            part_id,
            ability_id,
            nested,
        },
        AbilityLocator::Token {
            token,
            part_id,
            ability_id,
            ..
        } => AbilityLocator::Token {
            token,
            part_id,
            ability_id,
            nested,
        },
        AbilityLocator::Emblem {
            emblem, ability_id, ..
        } => AbilityLocator::Emblem {
            emblem,
            ability_id,
            nested,
        },
    }
}

pub(super) fn mana_payload_locator(
    catalog: &CardCatalog,
    mana: Mana,
) -> Option<ManaPayloadLocator> {
    if mana.restrictions.is_empty() && mana.spend_effects.is_empty() {
        return None;
    }
    let ability = ability_locator(catalog, |candidate| {
        mana_effects(candidate)
            .iter()
            .any(|effect| mana_effect_matches(*effect, mana))
    })?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = mana_effects(&definition)
        .iter()
        .position(|effect| mana_effect_matches(*effect, mana))?;
    Some(ManaPayloadLocator {
        ability,
        effect_index,
    })
}

pub(super) fn catalog_mana_payload(
    catalog: &CardCatalog,
    locator: &ManaPayloadLocator,
) -> Option<AddManaEffectDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    mana_effects(&ability).get(locator.effect_index).copied()
}

pub(super) fn applied_effect_locator(
    catalog: &CardCatalog,
    expected: AppliedEffectDef,
) -> Option<AppliedEffectLocator> {
    let ability = ability_locator(catalog, |candidate| {
        applied_effects(candidate).contains(&expected)
    })?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = applied_effects(&definition)
        .iter()
        .position(|effect| *effect == expected)?;
    Some(AppliedEffectLocator {
        ability,
        effect_index,
    })
}

/// Locates a resolved leaf beneath the ability provenance that created it.
///
/// The runtime source identifies the exact top-level printed clause. Nested
/// abilities still use the first structurally equal path because the runtime
/// does not retain a nested catalog path, but the search never falls back to a
/// different top-level ability: that would make source-relative predicates
/// reconstruct with different semantics.
pub(super) fn resolved_applied_effect_locator(
    catalog: &CardCatalog,
    source: AbilitySourceRef,
    expected: AppliedEffectDef,
) -> Option<AppliedEffectLocator> {
    let mut contains = |candidate: &AbilityDef| applied_effects(candidate).contains(&expected);
    let ability = ability_locator_for_origin(catalog, source.ability, &mut contains)?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = applied_effects(&definition)
        .iter()
        .position(|effect| *effect == expected)?;
    Some(AppliedEffectLocator {
        ability,
        effect_index,
    })
}

pub(super) fn applied_effect_locator_matches_source(
    locator: &AppliedEffectLocator,
    source: AbilitySourceRef,
) -> bool {
    ability_locator_matches_origin(&locator.ability, source.ability)
}

pub(super) fn catalog_applied_effect(
    catalog: &CardCatalog,
    locator: &AppliedEffectLocator,
) -> Option<AppliedEffectDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    applied_effects(&ability).get(locator.effect_index).copied()
}

pub(super) fn resolved_damage_prevention_locator(
    catalog: &CardCatalog,
    source: AbilitySourceRef,
    predicate: ObjectPredicateDef,
) -> Option<DamagePreventionLocator> {
    let expected = DamageSourceMatcherDef::Matching(predicate);
    let mut contains = |candidate: &AbilityDef| {
        damage_prevention_defs(candidate)
            .iter()
            .any(|prevention| prevention.matcher.source == expected)
    };
    let ability = ability_locator_for_origin(catalog, source.ability, &mut contains)?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = damage_prevention_defs(&definition)
        .iter()
        .position(|prevention| prevention.matcher.source == expected)?;
    Some(DamagePreventionLocator {
        ability,
        effect_index,
    })
}

pub(super) fn catalog_damage_prevention(
    catalog: &CardCatalog,
    locator: &DamagePreventionLocator,
) -> Option<DamagePreventionDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    damage_prevention_defs(&ability)
        .get(locator.effect_index)
        .copied()
}

fn damage_prevention_defs(ability: &AbilityDef) -> Vec<DamagePreventionDef> {
    let mut found = Vec::new();
    match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => {
            collect_damage_prevention_defs(effect, &mut found);
        }
        AbilityProgramDef::Replacement(effect) => {
            for child in replacement_child_effects(effect) {
                collect_damage_prevention_defs(child, &mut found);
            }
        }
    }
    found
}

fn collect_damage_prevention_defs(effect: EffectDef, found: &mut Vec<DamagePreventionDef>) {
    if let EffectDef::PreventDamage { prevention, .. } = effect {
        found.push(prevention);
    }
    for child in child_effects(effect) {
        collect_damage_prevention_defs(child, found);
    }
}

pub(super) fn applied_effects(ability: &AbilityDef) -> Vec<AppliedEffectDef> {
    let mut found = Vec::new();
    match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => {
            collect_applied_effects_from_effect(effect, &mut found);
        }
        AbilityProgramDef::Replacement(effect) => {
            for child in replacement_child_effects(effect) {
                collect_applied_effects_from_effect(child, &mut found);
            }
        }
    }
    for mana in mana_effects(ability) {
        for spend in mana.spend_effects {
            if let ManaSpendEffectDef::ApplyToPaidSpell(effect) = *spend {
                collect_applied_effect(effect, &mut found);
            }
        }
    }
    found
}

fn collect_applied_effects_from_effect(effect: EffectDef, found: &mut Vec<AppliedEffectDef>) {
    // Every effect that carries a rider, not just the one that is nothing but
    // a rider: a damage clause with one attached leaves a resolved effect on
    // the battlefield that has to be locatable again.
    match effect {
        // A static clause is not itself resolved, but what it hands out can
        // be: a permission that grants an ability to what it allowed leaves
        // that grant on a permanent, which has to be locatable afterwards.
        EffectDef::ConditionalStatic(conditional) => {
            collect_applied_effect(conditional.then.effect, found);
        }
        EffectDef::Apply {
            effect: applied, ..
        }
        | EffectDef::StaticApply {
            effect: applied, ..
        }
        | EffectDef::DealDamageAndApply { applied, .. } => collect_applied_effect(applied, found),
        _ => {}
    }
    for child in child_effects(effect) {
        collect_applied_effects_from_effect(child, found);
    }
}

fn collect_applied_effect(effect: AppliedEffectDef, found: &mut Vec<AppliedEffectDef>) {
    found.push(effect);
    match effect {
        AppliedEffectDef::Composite(children) => {
            for child in children {
                collect_applied_effect(*child, found);
            }
        }
        // "If you do, it gains ...": what a graveyard permission hands to
        // the permanent it allowed is a rider like any other, and the
        // permanent keeps it long after the play, so it has to be findable
        // again from the clause that printed it.
        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(permission)) => {
            if let Some(granted) = permission.grants {
                collect_applied_effect(*granted, found);
            }
        }
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}

pub(super) fn scoped_effect_snapshot(
    ability: &AbilityDef,
    effect: ScopedEffect,
) -> Option<ScopedEffectSnapshot> {
    let mut path = Vec::new();
    let found = match ability.effect.definition {
        AbilityProgramDef::Effects(definition) => {
            locate_effect(definition, effect.effect, &mut path)
        }
        AbilityProgramDef::Replacement(replacement) => replacement_child_effects(replacement)
            .into_iter()
            .enumerate()
            .any(|(index, root)| {
                path.push(index);
                if locate_effect(root, effect.effect, &mut path) {
                    true
                } else {
                    path.pop();
                    false
                }
            }),
    };
    found.then_some(ScopedEffectSnapshot {
        path,
        target_base: effect.target_base,
    })
}

pub(super) fn catalog_scoped_effect(
    catalog: &CardCatalog,
    ability: &AbilityLocator,
    snapshot: &ScopedEffectSnapshot,
) -> Option<ScopedEffect> {
    let ability = catalog_ability(catalog, ability)?;
    let (mut effect, path) = match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => (effect, snapshot.path.as_slice()),
        AbilityProgramDef::Replacement(replacement) => {
            let (&root, path) = snapshot.path.split_first()?;
            (*replacement_child_effects(replacement).get(root)?, path)
        }
    };
    for &index in path {
        effect = *child_effects(effect).get(index)?;
    }
    Some(ScopedEffect {
        effect,
        target_base: snapshot.target_base,
    })
}

/// Locates a replacement operation beneath the exact printed ability that
/// supplied the suspended prospective-event procedure.
pub(super) fn resolved_replacement_effect_locator(
    catalog: &CardCatalog,
    source: AbilitySourceRef,
    expected: ReplacementEffectDef,
) -> Option<ReplacementEffectLocator> {
    let mut contains = |candidate: &AbilityDef| {
        replacement_effects(candidate)
            .into_iter()
            .any(|effect| effect == expected)
    };
    let ability = ability_locator_for_origin(catalog, source.ability, &mut contains)?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = replacement_effects(&definition)
        .into_iter()
        .position(|effect| effect == expected)?;
    Some(ReplacementEffectLocator {
        ability,
        effect_index,
    })
}

pub(super) fn replacement_effect_locator_matches_source(
    locator: &ReplacementEffectLocator,
    source: AbilitySourceRef,
) -> bool {
    ability_locator_matches_origin(&locator.ability, source.ability)
}

pub(super) fn catalog_replacement_effect(
    catalog: &CardCatalog,
    locator: &ReplacementEffectLocator,
) -> Option<ReplacementEffectDef> {
    let ability = catalog_ability(catalog, &locator.ability)?;
    replacement_effects(&ability)
        .get(locator.effect_index)
        .copied()
}

pub(super) fn replacement_effects(ability: &AbilityDef) -> Vec<ReplacementEffectDef> {
    let mut effects = Vec::new();
    if let AbilityProgramDef::Replacement(effect) = ability.effect.definition {
        collect_replacement_effects(effect, &mut effects);
    }
    effects
}

fn collect_replacement_effects(
    effect: ReplacementEffectDef,
    found: &mut Vec<ReplacementEffectDef>,
) {
    found.push(effect);
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                collect_replacement_effects(*effect, found);
            }
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                collect_replacement_effects(*effect, found);
            }
        }
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                collect_replacement_effects(*effect, found);
            }
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. } => {}
    }
}

fn locate_effect(current: EffectDef, needle: EffectDef, path: &mut Vec<usize>) -> bool {
    if current == needle {
        return true;
    }
    for (index, child) in child_effects(current).into_iter().enumerate() {
        path.push(index);
        if locate_effect(child, needle, path) {
            return true;
        }
        path.pop();
    }
    false
}

pub(super) fn replacement_child_effects(effect: ReplacementEffectDef) -> Vec<EffectDef> {
    match effect {
        ReplacementEffectDef::Sequence(effects) => effects
            .iter()
            .flat_map(|effect| replacement_child_effects(*effect))
            .collect(),
        ReplacementEffectDef::Perform(effect) => vec![*effect],
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => if_true
            .iter()
            .chain(if_false.iter())
            .flat_map(|effect| replacement_child_effects(*effect))
            .collect(),
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => if_paid
            .iter()
            .chain(if_declined.iter())
            .flat_map(|effect| replacement_child_effects(*effect))
            .collect(),
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. } => Vec::new(),
    }
}

fn mana_effect_matches(effect: AddManaEffectDef, mana: Mana) -> bool {
    effect.restrictions == mana.restrictions
        && effect.spend_effects == mana.spend_effects
        && match effect.mana {
            crate::card::ManaSelectionDef::One(crate::card::ManaTypeDef::Fixed(color)) => {
                color == mana.color
            }
            crate::card::ManaSelectionDef::One(crate::card::ManaTypeDef::ChosenColor)
            | crate::card::ManaSelectionDef::ColorsOfLinkedExiles => true,
            crate::card::ManaSelectionDef::Choice(types)
            | crate::card::ManaSelectionDef::Combination(types) => match types.source {
                crate::card::ManaTypeSourceDef::Fixed(colors) => colors.contains(&mana.color),
                crate::card::ManaTypeSourceDef::ProducedBy(_)
                | crate::card::ManaTypeSourceDef::CouldBeProducedBy(_) => true,
            },
        }
}

pub(super) fn mana_effects(ability: &AbilityDef) -> Vec<AddManaEffectDef> {
    let mut effects = Vec::new();
    match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => collect_mana_effects(effect, &mut effects),
        AbilityProgramDef::Replacement(effect) => {
            for child in replacement_child_effects(effect) {
                collect_mana_effects(child, &mut effects);
            }
        }
    }
    effects
}

fn collect_mana_effects(effect: EffectDef, found: &mut Vec<AddManaEffectDef>) {
    if let EffectDef::AddMana(mana) = effect {
        found.push(mana);
    }
    for child in child_effects(effect) {
        collect_mana_effects(child, found);
    }
}

fn locate_ability(
    ability: &AbilityDef,
    matches: &mut impl FnMut(&AbilityDef) -> bool,
    path: &mut Vec<usize>,
) -> bool {
    if matches(ability) {
        return true;
    }
    for (index, child) in child_abilities(ability).into_iter().enumerate() {
        path.push(index);
        if locate_ability(child, matches, path) {
            return true;
        }
        path.pop();
    }
    false
}

pub(super) fn child_abilities(ability: &AbilityDef) -> Vec<&AbilityDef> {
    let mut children = Vec::new();
    if let DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(modal)) = ability.definition {
        children.extend(modal.modes);
    }
    // A modal trigger's modes are reached the same way: what goes onto the
    // stack is the chosen mode's own program, so a checkpoint has to be able
    // to name it. Appended only for triggers, because inserting children
    // ahead of an existing ability's would move every path already written
    // down.
    if let DeclarativeAbilityDef::Triggered(triggered) = ability.definition
        && let Some(modal) = triggered.modes
    {
        children.extend(modal.modes);
    }
    if matches!(
        ability.definition,
        DeclarativeAbilityDef::Keyword(crate::card::KeywordAbility::Suspend(_))
    ) {
        children.extend([
            &crate::card::abilities::SUSPEND_UPKEEP_ABILITY,
            &crate::card::abilities::SUSPEND_LAST_COUNTER_ABILITY,
        ]);
    }
    if matches!(
        ability.definition,
        DeclarativeAbilityDef::Keyword(crate::card::KeywordAbility::Rebound)
    ) {
        children.push(&crate::card::abilities::REBOUND_OFFER);
    }
    match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => collect_effect_abilities(effect, &mut children),
        AbilityProgramDef::Replacement(effect) => {
            for child in replacement_child_effects(effect) {
                collect_effect_abilities(child, &mut children);
            }
            collect_replacement_copy_abilities(effect, &mut children);
        }
    }
    children
}

pub(super) const fn ability_target_defs(ability: &AbilityDef) -> &'static [AbilityTargetDef] {
    match ability.definition {
        DeclarativeAbilityDef::Spell(spell) => spell.targets(),
        DeclarativeAbilityDef::ActivatedMana(activated)
        | DeclarativeAbilityDef::Activated(activated) => activated.targets,
        DeclarativeAbilityDef::TriggeredMana(triggered)
        | DeclarativeAbilityDef::Triggered(triggered) => triggered.targets,
        DeclarativeAbilityDef::Static(_)
        | DeclarativeAbilityDef::Replacement(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::SpecialAction(_)
        | DeclarativeAbilityDef::Pregame(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::DeckConstruction(_) => &[],
    }
}

fn collect_effect_abilities(effect: EffectDef, abilities: &mut Vec<&'static AbilityDef>) {
    match effect {
        EffectDef::ConditionalStatic(conditional) => {
            collect_applied_abilities(conditional.then.effect, abilities);
        }
        EffectDef::Apply { effect, .. } | EffectDef::StaticApply { effect, .. } => {
            collect_applied_abilities(effect, abilities);
        }
        EffectDef::DealDamageAndApply { applied, .. } => {
            collect_applied_abilities(applied, abilities);
        }
        EffectDef::InstallTrigger(installed) => abilities.push(installed.ability),
        EffectDef::CreateOngoingEffect(ongoing) => abilities.push(ongoing.ability),
        EffectDef::MayCastTargetWithoutPaying { ability, .. } => abilities.push(ability),
        EffectDef::BecomeCopyOf { exceptions, .. } => {
            abilities.extend(exceptions.added_abilities.iter().filter_map(
                |addition| match addition {
                    crate::card::CopyAbilityDef::This => None,
                    crate::card::CopyAbilityDef::Ability(ability) => Some(*ability),
                },
            ));
        }
        EffectDef::CreateToken {
            copy: Some(copy), ..
        } => {
            abilities.extend(
                copy.exceptions
                    .added_abilities
                    .iter()
                    .filter_map(|addition| match addition {
                        crate::card::CopyAbilityDef::This => None,
                        crate::card::CopyAbilityDef::Ability(ability) => Some(*ability),
                    }),
            );
        }
        _ => {}
    }
    for child in child_effects(effect) {
        collect_effect_abilities(child, abilities);
    }
}

fn collect_replacement_copy_abilities(
    effect: ReplacementEffectDef,
    abilities: &mut Vec<&'static AbilityDef>,
) {
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                collect_replacement_copy_abilities(*effect, abilities);
            }
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        }
        | ReplacementEffectDef::PayOr {
            if_paid: if_true,
            if_declined: if_false,
            ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                collect_replacement_copy_abilities(*effect, abilities);
            }
        }
        ReplacementEffectDef::CopyEntering { exceptions, .. } => {
            abilities.extend(exceptions.added_abilities.iter().filter_map(
                |addition| match addition {
                    crate::card::CopyAbilityDef::This => None,
                    crate::card::CopyAbilityDef::Ability(ability) => Some(*ability),
                },
            ));
        }
        ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_) => {}
    }
}
fn collect_applied_abilities(effect: AppliedEffectDef, abilities: &mut Vec<&'static AbilityDef>) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_abilities(*effect, abilities);
            }
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => abilities.push(ability),
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}

#[cfg(test)]
mod tests;
