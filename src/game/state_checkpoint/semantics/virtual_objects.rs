//! Discovery of creator-owned token and emblem characteristics.
//!
//! The walk starts at printed abilities and expands abilities owned by every
//! virtual object it discovers. This makes every locator ultimately rooted in
//! the card catalog without serializing rules or behavior pointers.

use std::collections::HashSet;

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

pub(super) fn authored_virtual_objects(catalog: &CardCatalog) -> AuthoredVirtualObjects {
    let mut found = AuthoredVirtualObjects {
        tokens: Vec::new(),
        emblems: Vec::new(),
    };
    for definition in catalog.definitions() {
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
            for (index, effect) in replacement_child_effects(replacement)
                .into_iter()
                .enumerate()
            {
                collect_effects(effect, &mut vec![index], creator, found);
            }
        }
    }
    if let Some(behavior) = ability.effect.custom_behavior() {
        for (token_index, token) in crate::card::tokens::custom_created_tokens(behavior)
            .into_iter()
            .enumerate()
        {
            found.tokens.push((
                token,
                TokenCharacteristicsLocator::Custom {
                    creator: Box::new(creator.clone()),
                    token_index,
                },
            ));
        }
    }
    for (index, child) in child_abilities(ability).into_iter().enumerate() {
        let mut child_creator = creator.clone();
        match &mut child_creator {
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
        EffectDef::CreateToken { token, .. } | EffectDef::CreateAttachedToken { token } => {
            found.tokens.push((
                token,
                TokenCharacteristicsLocator::EffectPath {
                    creator: Box::new(creator.clone()),
                    effect_path: path.clone(),
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
