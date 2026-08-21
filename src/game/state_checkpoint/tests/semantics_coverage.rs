//! Producer-side audit of the checkpoint's catalog semantics.
//!
//! Reconstruction never ships an executable value. It ships a locator into the
//! catalog and rebuilds the value from it, so a hosted state is representable
//! exactly when every executable value the rules engine can be holding is
//! addressable that way. These tests walk the whole catalog and prove the
//! addressing is total, which is the property `hasDeferredState` is allowed to
//! depend on.

use super::super::ScopedEffect;
use super::super::model::AbilityLocator;
use super::super::semantics::{
    ability_locator, applied_effect_locator, applied_effects, catalog_ability,
    catalog_applied_effect, catalog_mana_payload, catalog_replacement_effect,
    catalog_scoped_effect, child_abilities, mana_effects, mana_payload_locator,
    replacement_effect_locator, replacement_effects, scoped_effect_snapshot,
};
use crate::card::{
    AbilityDef, AbilityProgramDef, AddManaEffectDef, EffectDef, ManaSelectionDef, cards,
};
use crate::game::Mana;
use crate::{CardCatalog, CardDefinitionId, CardPartId};

/// Every ability the catalog can put into play, with the printed card it came
/// from, so failures name a card rather than an anonymous clause.
fn catalog_abilities(catalog: &CardCatalog) -> Vec<(CardDefinitionId, CardPartId, AbilityDef)> {
    let mut found = Vec::new();
    for definition in catalog.definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                collect(definition.id, part.id, &attached.definition, &mut found);
            }
        }
    }
    assert!(
        found.len() > 1_000,
        "the audit walked only {} abilities, so it proves nothing",
        found.len()
    );
    found
}

fn collect(
    definition: CardDefinitionId,
    part: CardPartId,
    ability: &AbilityDef,
    found: &mut Vec<(CardDefinitionId, CardPartId, AbilityDef)>,
) {
    found.push((definition, part, *ability));
    for child in child_abilities(ability) {
        collect(definition, part, child, found);
    }
}

fn card_name(catalog: &CardCatalog, definition: CardDefinitionId) -> String {
    catalog.get(definition).map_or_else(
        || format!("definition {}", definition.0),
        |card| card.name.clone(),
    )
}

#[test]
fn every_catalog_ability_has_a_locator_that_rebuilds_it() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        let rebuilt = ability_locator(&catalog, |candidate| *candidate == ability)
            .and_then(|locator| catalog_ability(&catalog, &locator));
        if rebuilt != Some(ability) {
            unaddressable.push(format!(
                "{}: {}",
                card_name(&catalog, definition),
                ability.text
            ));
        }
    }
    assert!(
        unaddressable.is_empty(),
        "abilities without a stable checkpoint locator: {unaddressable:#?}"
    );
}

#[test]
fn one_shot_cast_grant_has_a_stable_locator() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let definition = catalog
        .get(cards::DREADHORDE_ARCANIST)
        .expect("Dreadhorde Arcanist is in the catalog");
    let (source_ability, granted) = definition
        .rules
        .indexed_abilities()
        .find_map(|attached| match attached.definition.effect.definition {
            AbilityProgramDef::Effects(EffectDef::MayCastTargetWithoutPaying {
                ability, ..
            }) => Some((attached.id, ability)),
            _ => None,
        })
        .expect("Dreadhorde Arcanist has a one-shot cast grant");

    let locator = ability_locator(&catalog, |candidate| std::ptr::eq(candidate, granted))
        .expect("the exact granted ability has a locator");
    assert_eq!(
        locator,
        AbilityLocator::Card {
            definition: cards::DREADHORDE_ARCANIST.0,
            part_id: CardPartId::PRIMARY.0,
            ability_id: source_ability.0,
            nested: vec![0],
        }
    );
    assert_eq!(catalog_ability(&catalog, &locator), Some(*granted));
}

#[test]
fn every_catalog_applied_effect_has_a_locator_that_rebuilds_it() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        for effect in applied_effects(&ability) {
            let rebuilt = applied_effect_locator(&catalog, effect)
                .and_then(|locator| catalog_applied_effect(&catalog, &locator));
            if rebuilt != Some(effect) {
                unaddressable.push(format!(
                    "{}: {}",
                    card_name(&catalog, definition),
                    ability.text
                ));
            }
        }
    }
    assert!(
        unaddressable.is_empty(),
        "applied effects without a stable checkpoint locator: {unaddressable:#?}"
    );
}

#[test]
fn every_catalog_replacement_effect_has_a_locator_that_rebuilds_it() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        for effect in replacement_effects(&ability) {
            let rebuilt = replacement_effect_locator(&catalog, effect)
                .and_then(|locator| catalog_replacement_effect(&catalog, &locator));
            if rebuilt != Some(effect) {
                unaddressable.push(format!(
                    "{}: {}",
                    card_name(&catalog, definition),
                    ability.text
                ));
            }
        }
    }
    assert!(
        unaddressable.is_empty(),
        "replacement effects without a stable checkpoint locator: {unaddressable:#?}"
    );
}

/// Unrestricted mana is carried as a plain colored count, so only mana that
/// arrives with restrictions or spend effects needs a locator. Those are the
/// units that make `hasUnlocatedMana` defer a checkpoint.
#[test]
fn every_catalog_mana_unit_that_needs_a_locator_has_one() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        for effect in mana_effects(&ability) {
            if effect.restrictions.is_empty() && effect.spend_effects.is_empty() {
                continue;
            }
            for mana in produced_mana(effect) {
                let rebuilt = mana_payload_locator(&catalog, mana)
                    .and_then(|locator| catalog_mana_payload(&catalog, &locator));
                let matches = rebuilt.is_some_and(|rebuilt| {
                    rebuilt.restrictions == mana.restrictions
                        && rebuilt.spend_effects == mana.spend_effects
                });
                if !matches {
                    unaddressable.push(format!(
                        "{} ({:?}): {}",
                        card_name(&catalog, definition),
                        mana.color,
                        ability.text
                    ));
                }
            }
        }
    }
    assert!(
        unaddressable.is_empty(),
        "restricted mana without a stable checkpoint locator: {unaddressable:#?}"
    );
}

fn produced_mana(effect: AddManaEffectDef) -> Vec<Mana> {
    let colors = match effect.mana {
        ManaSelectionDef::One(color) => vec![color],
        ManaSelectionDef::Choice(colors) | ManaSelectionDef::Combination(colors) => colors.to_vec(),
    };
    colors
        .into_iter()
        .map(|color| Mana {
            color,
            source: None,
            restrictions: effect.restrictions,
            spend_effects: effect.spend_effects,
        })
        .collect()
}

/// Suspended resolutions carry the remaining effect as a path from the
/// ability's root, so every effect an ability can suspend inside must be
/// reachable by that path.
#[test]
fn every_catalog_effect_is_addressable_from_its_ability_root() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        let Some(locator) = ability_locator(&catalog, |candidate| *candidate == ability) else {
            continue;
        };
        let roots = match ability.effect.definition {
            AbilityProgramDef::Effects(effect) => vec![effect],
            AbilityProgramDef::Replacement(effect) => {
                super::super::semantics::replacement_child_effects(effect)
            }
        };
        for effect in roots.into_iter().flat_map(reachable_effects) {
            let scoped = ScopedEffect {
                effect,
                target_base: 0,
            };
            let rebuilt = scoped_effect_snapshot(&ability, scoped)
                .and_then(|snapshot| catalog_scoped_effect(&catalog, &locator, &snapshot));
            if rebuilt.map(|rebuilt| rebuilt.effect) != Some(effect) {
                unaddressable.push(format!(
                    "{}: {}",
                    card_name(&catalog, definition),
                    ability.text
                ));
                break;
            }
        }
    }
    assert!(
        unaddressable.is_empty(),
        "effects without a stable checkpoint path: {unaddressable:#?}"
    );
}

fn reachable_effects(effect: crate::card::EffectDef) -> Vec<crate::card::EffectDef> {
    let mut found = vec![effect];
    let mut index = 0;
    while index < found.len() {
        let current = found[index];
        index += 1;
        found.extend(super::super::semantics::child_effects(current));
    }
    found
}
