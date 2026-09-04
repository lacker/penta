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
    ability_locator, ability_locator_index, applied_effect_locator_index, applied_effects,
    catalog_ability, catalog_applied_effect, catalog_mana_payload, catalog_replacement_effect,
    catalog_scoped_effect, child_abilities, mana_effects, mana_payload_key,
    mana_payload_locator_index, replacement_effect_locator_index, replacement_effects,
    scoped_effect_snapshot,
};
use crate::card::{
    AbilityDef, AbilityProgramDef, AddManaEffectDef, EffectDef, ManaColor, ManaSelectionDef,
    ManaTypeDef, cards,
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
        || format!("definition {}", definition.get()),
        |card| card.name.clone(),
    )
}

#[test]
fn every_catalog_ability_has_a_locator_that_rebuilds_it() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    // One walk, rather than one search per ability: the index is built by the
    // same traversal the search uses, so it answers for every ability at once
    // without weakening what the audit proves.
    let locators = ability_locator_index(&catalog);
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        let rebuilt = locators
            .get(&ability)
            .and_then(|locator| catalog_ability(&catalog, locator));
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
            definition: cards::DREADHORDE_ARCANIST,
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
    let locators = applied_effect_locator_index(&catalog);
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        for effect in applied_effects(&ability) {
            let rebuilt = locators
                .get(&effect)
                .and_then(|locator| catalog_applied_effect(&catalog, locator));
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
    let locators = replacement_effect_locator_index(&catalog);
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        for effect in replacement_effects(&ability) {
            let rebuilt = locators
                .get(&effect)
                .and_then(|locator| catalog_replacement_effect(&catalog, locator));
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
    let locators = mana_payload_locator_index(&catalog);
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        for effect in mana_effects(&ability) {
            if effect.restrictions.is_empty() && effect.spend_effects.is_empty() {
                continue;
            }
            for mana in produced_mana(effect) {
                let rebuilt = locators
                    .get(&mana_payload_key(mana))
                    .and_then(|locator| catalog_mana_payload(&catalog, locator));
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
        ManaSelectionDef::One(ManaTypeDef::Fixed(color)) => vec![color],
        ManaSelectionDef::One(ManaTypeDef::ChosenColor) => ManaColor::COLORS.to_vec(),
        ManaSelectionDef::Choice(types) | ManaSelectionDef::Combination(types) => {
            match types.source {
                crate::card::ManaTypeSourceDef::Fixed(colors) => colors.to_vec(),
                crate::card::ManaTypeSourceDef::ProducedBy(_)
                | crate::card::ManaTypeSourceDef::CouldBeProducedBy(_) => ManaColor::ALL.to_vec(),
            }
        }
        // Whatever was imprinted, which no printed clause names. Every
        // colour is a possibility, so the sweep covers all five.
        ManaSelectionDef::ColorsOfLinkedExiles => crate::card::ManaColor::COLORS.to_vec(),
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
    let locators = ability_locator_index(&catalog);
    let mut unaddressable = Vec::new();
    for (definition, _, ability) in catalog_abilities(&catalog) {
        let Some(locator) = locators.get(&ability) else {
            continue;
        };
        let roots = match ability.effect.definition {
            AbilityProgramDef::Effects(effect) => vec![effect],
            AbilityProgramDef::Replacement(effect) => {
                super::super::semantics::replacement_child_effects(effect)
            }
        };
        for scoped in roots.into_iter().flat_map(reachable_effects) {
            let rebuilt = scoped_effect_snapshot(&ability, scoped)
                .and_then(|snapshot| catalog_scoped_effect(&catalog, locator, &snapshot));
            if rebuilt != Some(scoped) {
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

#[test]
fn nested_effect_reconstructs_its_effect_local_rules() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let (_, _, ability) = catalog_abilities(&catalog)
        .into_iter()
        .find(|(definition, _, _)| *definition == cards::WRATH_OF_GOD)
        .expect("Wrath of God has an executable ability");
    let locator = ability_locator(&catalog, |candidate| *candidate == ability)
        .expect("the ability is addressable");
    let EffectDef::WithRule { rule, effect } = ability
        .declarative_effect()
        .expect("Wrath has a declarative effect")
    else {
        panic!("Wrath scopes a rule around its destruction")
    };
    let scoped = ScopedEffect::primary(*effect).with_rule(rule);

    let snapshot = scoped_effect_snapshot(&ability, scoped).expect("the child has a stable path");
    let rebuilt = catalog_scoped_effect(&catalog, &locator, &snapshot)
        .expect("the child reconstructs from its path");

    assert_eq!(rebuilt, scoped);
}

#[test]
fn effect_path_distinguishes_equal_leaves_with_different_local_rules() {
    static DESTROY: EffectDef = EffectDef::Destroy {
        object: crate::card::EffectRecipientDef::Source,
        then: None,
    };
    static EFFECTS: [EffectDef; 2] = [
        EffectDef::WithRule {
            rule: crate::card::AppliedRuleDef::CannotRegenerate,
            effect: &DESTROY,
        },
        DESTROY,
    ];
    let ability = AbilityDef::spell("Destroy this twice.", EffectDef::Sequence(&EFFECTS));

    let wrapped =
        ScopedEffect::primary(DESTROY).with_rule(crate::card::AppliedRuleDef::CannotRegenerate);
    let plain = ScopedEffect::primary(DESTROY);

    assert_eq!(
        scoped_effect_snapshot(&ability, wrapped)
            .expect("the wrapped leaf is addressable")
            .path,
        vec![0, 0],
    );
    assert_eq!(
        scoped_effect_snapshot(&ability, plain)
            .expect("the plain leaf is addressable")
            .path,
        vec![1],
    );
}

fn reachable_effects(effect: crate::card::EffectDef) -> Vec<ScopedEffect> {
    let mut found = vec![ScopedEffect::primary(effect)];
    let mut index = 0;
    while index < found.len() {
        let current = found[index];
        index += 1;
        let child_scope = match current.effect {
            EffectDef::WithRule { rule, .. } => current.with_rule(rule),
            _ => current,
        };
        found.extend(
            super::super::semantics::child_effects(current.effect)
                .into_iter()
                .map(|effect| child_scope.with_effect(effect)),
        );
    }
    found
}
