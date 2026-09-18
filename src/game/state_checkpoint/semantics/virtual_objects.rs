//! Discovery of creator-owned token and emblem characteristics.
//!
//! The walk starts at printed abilities and expands abilities owned by every
//! virtual object it discovers. This makes every locator ultimately rooted in
//! the card catalog without serializing rules or behavior pointers.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex, Weak};

use super::{child_abilities, child_effects, replacement_child_effects};
use crate::card::{
    AbilityDef, AbilityProgramDef, EffectDef, EmblemCharacteristics, TokenCharacteristics,
    TokenPart,
};
use crate::{AbilityId, CardCatalog};

use super::super::model::{
    AbilityLocator, EmblemCharacteristicsLocator, TokenCharacteristicsLocator,
};

pub(super) struct AuthoredVirtualObjects {
    pub(super) tokens: Vec<(TokenCharacteristics, TokenCharacteristicsLocator)>,
    pub(super) emblems: Vec<(EmblemCharacteristics, EmblemCharacteristicsLocator)>,
}

struct CachedVirtualObjects {
    source: Weak<u8>,
    objects: Arc<AuthoredVirtualObjects>,
}

static CATALOG_OBJECTS: LazyLock<Mutex<HashMap<usize, CachedVirtualObjects>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Locator discovery depends only on the immutable catalog. Keep the complete
/// ordered discovery, including duplicate candidates, so first-match semantics
/// and nested creator paths are identical to the uncached walk.
pub(super) fn authored_virtual_objects(catalog: &CardCatalog) -> Arc<AuthoredVirtualObjects> {
    let (identity, source) = catalog.process_cache_identity();
    {
        let cache = CATALOG_OBJECTS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(entry) = cache.get(&identity)
            && entry.source.ptr_eq(&source)
        {
            return Arc::clone(&entry.objects);
        }
    }
    // Do the recursive catalog walk outside the lock. Concurrent callers may
    // both build, but all retain the same installed value below.
    let objects = Arc::new(collect_authored_virtual_objects(catalog));
    let mut cache = CATALOG_OBJECTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    cache.retain(|_, entry| entry.source.strong_count() > 0);
    let entry = cache
        .entry(identity)
        .or_insert_with(|| CachedVirtualObjects { source, objects });
    Arc::clone(&entry.objects)
}

fn collect_authored_virtual_objects(catalog: &CardCatalog) -> AuthoredVirtualObjects {
    let mut found = AuthoredVirtualObjects {
        tokens: Vec::new(),
        emblems: Vec::new(),
    };
    for definition in catalog.ordered_definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let creator = AbilityLocator::Card {
                    definition: definition.id,
                    part_id: part.id.0,
                    ability_id: attached.id.0,
                    nested: Vec::new(),
                };
                collect_from_ability(&attached.definition, &creator, &mut found);
            }
        }
    }

    let mut expanded_tokens = HashSet::new();
    let mut expanded_emblems = HashSet::new();
    let mut token_index = 0;
    let mut emblem_index = 0;
    while token_index < found.tokens.len() || emblem_index < found.emblems.len() {
        while let Some((token, locator)) = found.tokens.get(token_index).cloned() {
            token_index += 1;
            if !expanded_tokens.insert(token.semantic_identity()) {
                continue;
            }
            for part in token_parts(token) {
                for attached in part.rules().indexed_abilities() {
                    let creator = AbilityLocator::Token {
                        token: locator.clone(),
                        part_id: part.id.0,
                        ability_id: attached.id.0,
                        nested: Vec::new(),
                    };
                    collect_from_ability(&attached.definition, &creator, &mut found);
                }
            }
        }
        while let Some((emblem, locator)) = found.emblems.get(emblem_index).cloned() {
            emblem_index += 1;
            if !expanded_emblems.insert(emblem) {
                continue;
            }
            for (index, ability) in emblem.abilities().iter().enumerate() {
                let ability_id = AbilityId::from_index(index)
                    .expect("validated emblem ability count has positional IDs");
                let creator = AbilityLocator::Emblem {
                    emblem: locator.clone(),
                    ability_id: ability_id.0,
                    nested: Vec::new(),
                };
                collect_from_ability(ability, &creator, &mut found);
            }
        }
    }
    found
}

#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn rules_cache_catalog_discovery_preserves_order_and_lives_with_catalog() {
        let catalog = crate::card::catalog().unwrap();
        let expected = collect_authored_virtual_objects(&catalog);
        let first = authored_virtual_objects(&catalog);
        assert_eq!(first.tokens, expected.tokens);
        assert_eq!(first.emblems, expected.emblems);
        let retained = Arc::downgrade(&first);
        drop(first);
        let second = authored_virtual_objects(&catalog.clone());
        assert!(Arc::ptr_eq(&retained.upgrade().unwrap(), &second));

        // A distinct catalog's absence of creators must not inherit a prior
        // catalog's token/emblem locators, even on the same thread.
        let empty = CardCatalog::new([]).unwrap();
        let other = authored_virtual_objects(&empty);
        assert!(other.tokens.is_empty());
        assert!(other.emblems.is_empty());
        assert!(!Arc::ptr_eq(&second, &other));
    }
}

fn collect_from_ability(
    ability: &AbilityDef,
    creator: &AbilityLocator,
    found: &mut AuthoredVirtualObjects,
) {
    match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => {
            collect_effects(effect, &mut Vec::new(), creator, found);
        }
        AbilityProgramDef::Replacement(replacement) => {
            for (choice_index, token) in crate::card::replacement_tokens(replacement)
                .into_iter()
                .enumerate()
            {
                found.tokens.push((
                    token,
                    TokenCharacteristicsLocator::EntryChoice {
                        creator: Box::new(creator.clone()),
                        choice_index,
                        colors: None,
                        basic_land_type_words: None,
                        color_words: None,
                    },
                ));
            }
            for (index, effect) in replacement_child_effects(replacement)
                .into_iter()
                .enumerate()
            {
                collect_effects(effect, &mut vec![index], creator, found);
            }
        }
    }
    for (index, child) in child_abilities(ability).into_iter().enumerate() {
        let mut child_creator = creator.clone();
        match &mut child_creator {
            AbilityLocator::DynamicGrant { .. } => {
                unreachable!("authored creators are catalog roots")
            }
            AbilityLocator::Card { nested, .. }
            | AbilityLocator::Token { nested, .. }
            | AbilityLocator::Emblem { nested, .. } => nested.push(index),
        }
        collect_from_ability(child, &child_creator, found);
    }
}

fn collect_effects(
    effect: EffectDef,
    path: &mut Vec<usize>,
    creator: &AbilityLocator,
    found: &mut AuthoredVirtualObjects,
) {
    match effect {
        EffectDef::CreateToken(crate::card::CreateTokenDef {
            token: crate::card::TokenDef::Literal(token),
            ..
        })
        | EffectDef::CreateAttachedToken { token, .. } => {
            found.tokens.push((
                token,
                TokenCharacteristicsLocator::EffectPath {
                    creator: Box::new(creator.clone()),
                    effect_path: path.clone(),
                    colors: None,
                    basic_land_type_words: None,
                    color_words: None,
                },
            ));
        }
        EffectDef::CreateEmblem { emblem } => found.emblems.push((
            emblem,
            EmblemCharacteristicsLocator::EffectPath {
                creator: Box::new(creator.clone()),
                effect_path: path.clone(),
            },
        )),
        _ => {}
    }
    for (index, child) in child_effects(effect).into_iter().enumerate() {
        path.push(index);
        collect_effects(child, path, creator, found);
        path.pop();
    }
}

pub(super) fn effect_at_path(ability: &AbilityDef, path: &[usize]) -> Option<EffectDef> {
    let (mut effect, path) = match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => (effect, path),
        AbilityProgramDef::Replacement(replacement) => {
            let (&root, path) = path.split_first()?;
            (*replacement_child_effects(replacement).get(root)?, path)
        }
    };
    for &index in path {
        effect = *child_effects(effect).get(index)?;
    }
    Some(effect)
}

pub(super) fn token_parts(token: TokenCharacteristics) -> Vec<TokenPart> {
    let mut parts = vec![token.primary_part()];
    if let Some(back) = token
        .other_face(token.primary_part_id())
        .and_then(|id| token.part(id))
    {
        parts.push(back);
    }
    parts
}

#[cfg(test)]
mod tests;
