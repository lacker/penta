mod emblem;
mod token;
mod virtual_objects;

pub(super) use crate::card::child_effects;
use emblem::authored_emblems;
pub(super) use emblem::{catalog_emblem_characteristics, emblem_characteristics_locator};
use token::authored_tokens;
pub(super) use token::{
    catalog_token_characteristics, object_characteristics_from_snapshot,
    object_characteristics_snapshot, token_characteristics_locator,
};
use virtual_objects::token_parts;

use super::model::{
    AbilityLocator, AppliedEffectLocator, ManaPayloadLocator, ReplacementEffectLocator,
    ScopedEffectSnapshot,
};
use crate::card::BandingQuality;

use super::model_keyword::KeywordSnapshot;
use super::model_prevention::DamagePreventionLocator;
use super::{AbilityOrigin, AbilitySourceRef, Mana, ScopedEffect};
use crate::card::{
    AbilityDef, AbilityOperationDef, AbilityProgramDef, AbilityTargetDef, AddManaEffectDef,
    AppliedEffectDef, BasicLandType, CharacteristicOperationDef, DamagePreventionDef,
    DamageSourceMatcherDef, DeclarativeAbilityDef, EffectDef, KeywordAbility, ManaColor,
    ManaSpendEffectDef, ObjectPredicateDef, ProtectedCreatureType, ReplacementEffectDef,
    SpellAbilityDef,
};
use crate::{CardCatalog, CardDefinitionId, CardPartId};

pub(super) fn ability_locator(
    catalog: &CardCatalog,
    mut matches: impl FnMut(&AbilityDef) -> bool,
) -> Option<AbilityLocator> {
    for definition in catalog.definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let mut nested = Vec::new();
                if locate_ability(&attached.definition, &mut matches, &mut nested) {
                    return Some(AbilityLocator::Card {
                        definition: definition.id.0,
                        part_id: part.id.0,
                        ability_id: attached.id.0,
                        nested,
                    });
                }
            }
        }
    }
    for (token, token_locator) in authored_tokens(catalog) {
        for part in token_parts(token) {
            for attached in part.rules().indexed_abilities() {
                let mut nested = Vec::new();
                if locate_ability(&attached.definition, &mut matches, &mut nested) {
                    return Some(AbilityLocator::Token {
                        token: token_locator,
                        part_id: part.id.0,
                        ability_id: attached.id.0,
                        nested,
                    });
                }
            }
        }
    }
    for (emblem, emblem_locator) in authored_emblems(catalog) {
        for (index, ability) in emblem.abilities().iter().enumerate() {
            let ability_id = crate::AbilityId::from_index(index)
                .expect("validated emblem ability count has positional IDs");
            let mut nested = Vec::new();
            if locate_ability(ability, &mut matches, &mut nested) {
                return Some(AbilityLocator::Emblem {
                    emblem: emblem_locator,
                    ability_id: ability_id.0,
                    nested,
                });
            }
        }
    }
    None
}

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
                definition: definition.0,
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
        AbilityOrigin::IntrinsicBasicLand(_) | AbilityOrigin::IntrinsicCounter(_) => None,
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
                .get(CardDefinitionId(*definition))?
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
        ) => *definition == expected_definition.0 && *part_id == part.0 && *ability_id == ability.0,
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
            *definition == source_definition.0
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
        EffectDef::Apply {
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
    if let AppliedEffectDef::Composite(children) = effect {
        for child in children {
            collect_applied_effect(*child, found);
        }
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

#[cfg(test)]
pub(super) fn replacement_effect_locator(
    catalog: &CardCatalog,
    expected: ReplacementEffectDef,
) -> Option<ReplacementEffectLocator> {
    let ability = ability_locator(catalog, |candidate| {
        replacement_effects(candidate)
            .into_iter()
            .any(|effect| effect == expected)
    })?;
    let definition = catalog_ability(catalog, &ability)?;
    let effect_index = replacement_effects(&definition)
        .into_iter()
        .position(|effect| effect == expected)?;
    Some(ReplacementEffectLocator {
        ability,
        effect_index,
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
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
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
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::CopyEntering { .. } => Vec::new(),
    }
}

fn mana_effect_matches(effect: AddManaEffectDef, mana: Mana) -> bool {
    effect.restrictions == mana.restrictions
        && effect.spend_effects == mana.spend_effects
        && match effect.mana {
            crate::card::ManaSelectionDef::One(color) => color == mana.color,
            crate::card::ManaSelectionDef::Choice(colors)
            | crate::card::ManaSelectionDef::Combination(colors) => colors.contains(&mana.color),
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
    match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => collect_effect_abilities(effect, &mut children),
        AbilityProgramDef::Replacement(effect) => {
            for child in replacement_child_effects(effect) {
                collect_effect_abilities(child, &mut children);
            }
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
        | DeclarativeAbilityDef::SpecialAction(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => &[],
    }
}

fn collect_effect_abilities(effect: EffectDef, abilities: &mut Vec<&'static AbilityDef>) {
    match effect {
        EffectDef::Apply { effect, .. } | EffectDef::StaticApply { effect, .. } => {
            collect_applied_abilities(effect, abilities);
        }
        EffectDef::DealDamageAndApply { applied, .. } => {
            collect_applied_abilities(applied, abilities);
        }
        EffectDef::InstallTrigger(installed) => abilities.push(installed.ability),
        _ => {}
    }
    for child in child_effects(effect) {
        collect_effect_abilities(child, abilities);
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

pub(super) const fn keyword_snapshot(keyword: KeywordAbility) -> KeywordSnapshot {
    match keyword {
        KeywordAbility::Devoid => KeywordSnapshot::Devoid,
        KeywordAbility::Infect => KeywordSnapshot::Infect,
        KeywordAbility::Flying => KeywordSnapshot::Flying,
        KeywordAbility::Trample => KeywordSnapshot::Trample,
        KeywordAbility::Haste => KeywordSnapshot::Haste,
        KeywordAbility::FirstStrike => KeywordSnapshot::FirstStrike,
        KeywordAbility::DoubleStrike => KeywordSnapshot::DoubleStrike,
        KeywordAbility::Banding => KeywordSnapshot::Banding,
        KeywordAbility::BandsWithOther(BandingQuality::LegendaryCreatures) => {
            KeywordSnapshot::BandsWithOtherLegendaryCreatures
        }
        KeywordAbility::BandsWithOther(BandingQuality::WolvesOfTheHunt) => {
            KeywordSnapshot::BandsWithOtherWolvesOfTheHunt
        }
        KeywordAbility::Vigilance => KeywordSnapshot::Vigilance,
        KeywordAbility::Defender => KeywordSnapshot::Defender,
        KeywordAbility::Deathtouch => KeywordSnapshot::Deathtouch,
        KeywordAbility::Lifelink => KeywordSnapshot::Lifelink,
        KeywordAbility::Reach => KeywordSnapshot::Reach,
        KeywordAbility::Flash => KeywordSnapshot::Flash,
        KeywordAbility::Hexproof => KeywordSnapshot::Hexproof,
        KeywordAbility::Shroud => KeywordSnapshot::Shroud,
        KeywordAbility::Unleash => KeywordSnapshot::Unleash,
        KeywordAbility::Intimidate => KeywordSnapshot::Intimidate,
        KeywordAbility::Menace => KeywordSnapshot::Menace,
        KeywordAbility::Undying => KeywordSnapshot::Undying,
        KeywordAbility::Indestructible => KeywordSnapshot::Indestructible,
        KeywordAbility::AttacksEachCombatIfAble => KeywordSnapshot::AttacksEachCombatIfAble,
        KeywordAbility::LegendaryLandwalk => KeywordSnapshot::LegendaryLandwalk,
        KeywordAbility::Landwalk(BasicLandType::Plains) => KeywordSnapshot::Plainswalk,
        KeywordAbility::Landwalk(BasicLandType::Island) => KeywordSnapshot::Islandwalk,
        KeywordAbility::Landwalk(BasicLandType::Swamp) => KeywordSnapshot::Swampwalk,
        KeywordAbility::Landwalk(BasicLandType::Mountain) => KeywordSnapshot::Mountainwalk,
        KeywordAbility::Landwalk(BasicLandType::Forest) => KeywordSnapshot::Forestwalk,
        KeywordAbility::ProtectionFrom(ManaColor::White) => KeywordSnapshot::ProtectionFromWhite,
        KeywordAbility::ProtectionFrom(ManaColor::Blue) => KeywordSnapshot::ProtectionFromBlue,
        KeywordAbility::ProtectionFrom(ManaColor::Black) => KeywordSnapshot::ProtectionFromBlack,
        KeywordAbility::ProtectionFrom(ManaColor::Red) => KeywordSnapshot::ProtectionFromRed,
        KeywordAbility::ProtectionFrom(ManaColor::Green) => KeywordSnapshot::ProtectionFromGreen,
        KeywordAbility::ProtectionFrom(ManaColor::Colorless) => {
            KeywordSnapshot::ProtectionFromColorless
        }
        KeywordAbility::ProtectionFromCreatureType(ProtectedCreatureType::Zombie) => {
            KeywordSnapshot::ProtectionFromZombies
        }
        KeywordAbility::ProtectionFromCreatureType(ProtectedCreatureType::Vampire) => {
            KeywordSnapshot::ProtectionFromVampires
        }
        KeywordAbility::ProtectionFromCreatureType(ProtectedCreatureType::Werewolf) => {
            KeywordSnapshot::ProtectionFromWerewolves
        }
        KeywordAbility::ProtectionFromCreatures => KeywordSnapshot::ProtectionFromCreatures,
        KeywordAbility::ProtectionFromMulticolored => KeywordSnapshot::ProtectionFromMulticolored,
    }
}

pub(super) const fn parse_keyword(value: KeywordSnapshot) -> KeywordAbility {
    match value {
        KeywordSnapshot::Devoid => KeywordAbility::Devoid,
        KeywordSnapshot::Infect => KeywordAbility::Infect,
        KeywordSnapshot::Flying => KeywordAbility::Flying,
        KeywordSnapshot::Trample => KeywordAbility::Trample,
        KeywordSnapshot::Haste => KeywordAbility::Haste,
        KeywordSnapshot::FirstStrike => KeywordAbility::FirstStrike,
        KeywordSnapshot::DoubleStrike => KeywordAbility::DoubleStrike,
        KeywordSnapshot::Banding => KeywordAbility::Banding,
        KeywordSnapshot::BandsWithOtherLegendaryCreatures => {
            KeywordAbility::BandsWithOther(BandingQuality::LegendaryCreatures)
        }
        KeywordSnapshot::BandsWithOtherWolvesOfTheHunt => {
            KeywordAbility::BandsWithOther(BandingQuality::WolvesOfTheHunt)
        }
        KeywordSnapshot::Vigilance => KeywordAbility::Vigilance,
        KeywordSnapshot::Defender => KeywordAbility::Defender,
        KeywordSnapshot::Deathtouch => KeywordAbility::Deathtouch,
        KeywordSnapshot::Lifelink => KeywordAbility::Lifelink,
        KeywordSnapshot::Reach => KeywordAbility::Reach,
        KeywordSnapshot::Flash => KeywordAbility::Flash,
        KeywordSnapshot::Hexproof => KeywordAbility::Hexproof,
        KeywordSnapshot::Shroud => KeywordAbility::Shroud,
        KeywordSnapshot::Unleash => KeywordAbility::Unleash,
        KeywordSnapshot::Intimidate => KeywordAbility::Intimidate,
        KeywordSnapshot::Menace => KeywordAbility::Menace,
        KeywordSnapshot::Undying => KeywordAbility::Undying,
        KeywordSnapshot::Indestructible => KeywordAbility::Indestructible,
        KeywordSnapshot::AttacksEachCombatIfAble => KeywordAbility::AttacksEachCombatIfAble,
        KeywordSnapshot::LegendaryLandwalk => KeywordAbility::LegendaryLandwalk,
        KeywordSnapshot::Plainswalk => KeywordAbility::Landwalk(BasicLandType::Plains),
        KeywordSnapshot::Islandwalk => KeywordAbility::Landwalk(BasicLandType::Island),
        KeywordSnapshot::Swampwalk => KeywordAbility::Landwalk(BasicLandType::Swamp),
        KeywordSnapshot::Mountainwalk => KeywordAbility::Landwalk(BasicLandType::Mountain),
        KeywordSnapshot::Forestwalk => KeywordAbility::Landwalk(BasicLandType::Forest),
        KeywordSnapshot::ProtectionFromWhite => KeywordAbility::ProtectionFrom(ManaColor::White),
        KeywordSnapshot::ProtectionFromBlue => KeywordAbility::ProtectionFrom(ManaColor::Blue),
        KeywordSnapshot::ProtectionFromBlack => KeywordAbility::ProtectionFrom(ManaColor::Black),
        KeywordSnapshot::ProtectionFromRed => KeywordAbility::ProtectionFrom(ManaColor::Red),
        KeywordSnapshot::ProtectionFromGreen => KeywordAbility::ProtectionFrom(ManaColor::Green),
        KeywordSnapshot::ProtectionFromCreatures => KeywordAbility::ProtectionFromCreatures,
        KeywordSnapshot::ProtectionFromMulticolored => KeywordAbility::ProtectionFromMulticolored,
        KeywordSnapshot::ProtectionFromZombies => {
            KeywordAbility::ProtectionFromCreatureType(ProtectedCreatureType::Zombie)
        }
        KeywordSnapshot::ProtectionFromVampires => {
            KeywordAbility::ProtectionFromCreatureType(ProtectedCreatureType::Vampire)
        }
        KeywordSnapshot::ProtectionFromWerewolves => {
            KeywordAbility::ProtectionFromCreatureType(ProtectedCreatureType::Werewolf)
        }
        KeywordSnapshot::ProtectionFromColorless => {
            KeywordAbility::ProtectionFrom(ManaColor::Colorless)
        }
    }
}

#[cfg(test)]
mod tests;
