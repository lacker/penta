//! Addressing an authored ability, and the effects beneath it, by a durable
//! catalog locator.
//!
//! Every ability the rules engine can be holding has to be nameable as a
//! position in the catalog, because that position is all a checkpoint ships.
//! One walk defines those positions, and the single-ability search, the
//! effect searches rooted in it, and the audits' whole-catalog indexes are all
//! expressed on top of it.

use std::ops::ControlFlow;

use super::super::model::{
    AbilityLocator, EmblemCharacteristicsLocator, TokenCharacteristicsLocator,
};
use super::child_abilities;
use super::emblem::authored_emblems;
use super::token::authored_tokens;
use super::virtual_objects::token_parts;
use crate::card::AbilityDef;
use crate::{CardCatalog, CardDefinitionId};

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::hash::Hash;

#[cfg(test)]
use super::super::model::{AppliedEffectLocator, ManaPayloadLocator, ReplacementEffectLocator};
#[cfg(test)]
use super::{applied_effects, mana_effect_matches, mana_effects, replacement_effects};
#[cfg(test)]
use crate::card::{ManaColor, ManaRestrictionDef, ManaSpendEffectDef};
#[cfg(test)]
use crate::game::Mana;

/// Where an authored ability tree is rooted, and the identifiers a locator
/// into it is built from. The virtual-object locators are held by reference so
/// a walk that visits every ability only pays to clone one where a locator is
/// actually wanted.
enum AbilityRoot<'a> {
    Card {
        definition: CardDefinitionId,
        part_id: u8,
        ability_id: u8,
    },
    Token {
        token: &'a TokenCharacteristicsLocator,
        part_id: u8,
        ability_id: u8,
    },
    Emblem {
        emblem: &'a EmblemCharacteristicsLocator,
        ability_id: u8,
    },
}

impl AbilityRoot<'_> {
    /// The locator addressing the ability that `nested` reaches beneath this
    /// root.
    fn locator(&self, nested: Vec<usize>) -> AbilityLocator {
        match *self {
            Self::Card {
                definition,
                part_id,
                ability_id,
            } => AbilityLocator::Card {
                definition,
                part_id,
                ability_id,
                nested,
            },
            Self::Token {
                token,
                part_id,
                ability_id,
            } => AbilityLocator::Token {
                token: token.clone(),
                part_id,
                ability_id,
                nested,
            },
            Self::Emblem { emblem, ability_id } => AbilityLocator::Emblem {
                emblem: emblem.clone(),
                ability_id,
                nested,
            },
        }
    }
}

/// Walks every authored ability the catalog can put into play, in the order
/// that decides which position a locator search settles on: printed cards,
/// then authored tokens, then emblems, and each ability ahead of its own
/// nested children. `visit` receives the ability together with the root and
/// nested path that address it, and ends the walk by returning
/// `ControlFlow::Break`.
///
/// Every locator search shares this one walk. A second traversal that agreed
/// with it only by inspection would make an audit built on it prove less than
/// it reads as proving.
fn visit_authored_abilities<B>(
    catalog: &CardCatalog,
    visit: &mut impl FnMut(&AbilityDef, &AbilityRoot<'_>, &[usize]) -> ControlFlow<B>,
) -> ControlFlow<B> {
    for definition in catalog.ordered_definitions() {
        for part in &definition.parts {
            for attached in part.rules.indexed_abilities() {
                let root = AbilityRoot::Card {
                    definition: definition.id,
                    part_id: part.id.0,
                    ability_id: attached.id.0,
                };
                visit_ability_tree(&attached.definition, &root, &mut Vec::new(), visit)?;
            }
        }
    }
    for (token, token_locator) in authored_tokens(catalog) {
        for part in token_parts(token) {
            for attached in part.rules().indexed_abilities() {
                let root = AbilityRoot::Token {
                    token: &token_locator,
                    part_id: part.id.0,
                    ability_id: attached.id.0,
                };
                visit_ability_tree(&attached.definition, &root, &mut Vec::new(), visit)?;
            }
        }
    }
    for (emblem, emblem_locator) in authored_emblems(catalog) {
        for (index, ability) in emblem.abilities().iter().enumerate() {
            let ability_id = crate::AbilityId::from_index(index)
                .expect("validated emblem ability count has positional IDs");
            let root = AbilityRoot::Emblem {
                emblem: &emblem_locator,
                ability_id: ability_id.0,
            };
            visit_ability_tree(ability, &root, &mut Vec::new(), visit)?;
        }
    }
    ControlFlow::Continue(())
}

/// Visits `ability` and then its nested children, depth first, extending
/// `path` with the index of each child it descends into.
fn visit_ability_tree<B>(
    ability: &AbilityDef,
    root: &AbilityRoot<'_>,
    path: &mut Vec<usize>,
    visit: &mut impl FnMut(&AbilityDef, &AbilityRoot<'_>, &[usize]) -> ControlFlow<B>,
) -> ControlFlow<B> {
    visit(ability, root, path)?;
    for (index, child) in child_abilities(ability).into_iter().enumerate() {
        path.push(index);
        visit_ability_tree(child, root, path, visit)?;
        path.pop();
    }
    ControlFlow::Continue(())
}

pub(in crate::game::state_checkpoint) fn ability_locator(
    catalog: &CardCatalog,
    mut matches: impl FnMut(&AbilityDef) -> bool,
) -> Option<AbilityLocator> {
    match visit_authored_abilities(catalog, &mut |ability, root, nested| {
        if matches(ability) {
            ControlFlow::Break(root.locator(nested.to_vec()))
        } else {
            ControlFlow::Continue(())
        }
    }) {
        ControlFlow::Break(locator) => Some(locator),
        ControlFlow::Continue(()) => None,
    }
}

/// Every distinct authored ability paired with the locator that
/// [`ability_locator`] resolves it to.
///
/// The catalog-wide audits ask that question once per ability, and answering
/// each one by rescanning the catalog made them quadratic in a catalog that
/// grows with every card added. A single walk answers all of them at once.
/// Keeping the first position seen reproduces the search's own tie-break, so
/// an ability printed on several cards still resolves where it used to.
#[cfg(test)]
pub(in crate::game::state_checkpoint) fn ability_locator_index(
    catalog: &CardCatalog,
) -> HashMap<AbilityDef, AbilityLocator> {
    let mut index = HashMap::new();
    let walk: ControlFlow<()> = visit_authored_abilities(catalog, &mut |ability, root, nested| {
        index
            .entry(*ability)
            .or_insert_with(|| root.locator(nested.to_vec()));
        ControlFlow::Continue(())
    });
    debug_assert!(walk.is_continue(), "the index walk never breaks");
    index
}

/// One catalog-wide index from an effect to the locator that rebuilds it,
/// built by the same walk the matching search uses.
///
/// The search answers "the first ability holding this effect, and where the
/// effect sits inside it". Inserting only on vacancy reproduces both halves of
/// that: the first ability in walk order wins, and within it the earliest
/// position holding an equal effect wins.
#[cfg(test)]
fn effect_locator_index<Effect, Locator>(
    catalog: &CardCatalog,
    effects: impl Fn(&AbilityDef) -> Vec<Effect>,
    locate: impl Fn(AbilityLocator, usize) -> Locator,
) -> HashMap<Effect, Locator>
where
    Effect: Copy + Eq + Hash,
{
    let mut index = HashMap::new();
    let walk: ControlFlow<()> = visit_authored_abilities(catalog, &mut |ability, root, nested| {
        for (effect_index, effect) in effects(ability).into_iter().enumerate() {
            index
                .entry(effect)
                .or_insert_with(|| locate(root.locator(nested.to_vec()), effect_index));
        }
        ControlFlow::Continue(())
    });
    debug_assert!(walk.is_continue(), "the index walk never breaks");
    index
}

/// Every applied effect in the catalog paired with the locator that
/// `applied_effect_locator` resolves it to.
#[cfg(test)]
pub(in crate::game::state_checkpoint) fn applied_effect_locator_index(
    catalog: &CardCatalog,
) -> HashMap<crate::card::AppliedEffectDef, AppliedEffectLocator> {
    effect_locator_index(catalog, applied_effects, |ability, effect_index| {
        AppliedEffectLocator {
            ability,
            effect_index,
        }
    })
}

/// Every replacement effect in the catalog paired with the locator that
/// `replacement_effect_locator` resolves it to.
#[cfg(test)]
pub(in crate::game::state_checkpoint) fn replacement_effect_locator_index(
    catalog: &CardCatalog,
) -> HashMap<crate::card::ReplacementEffectDef, ReplacementEffectLocator> {
    effect_locator_index(catalog, replacement_effects, |ability, effect_index| {
        ReplacementEffectLocator {
            ability,
            effect_index,
        }
    })
}

/// What decides whether a mana unit is served by a given mana effect.
///
/// `source` is deliberately absent: the search ignores it, so folding it into
/// the key would split entries that the search treats as one.
#[cfg(test)]
pub(in crate::game::state_checkpoint) type ManaPayloadKey = (
    &'static [ManaRestrictionDef],
    &'static [ManaSpendEffectDef],
    ManaColor,
);

/// The key under which [`mana_payload_locator_index`] holds `mana`.
#[cfg(test)]
pub(in crate::game::state_checkpoint) fn mana_payload_key(mana: Mana) -> ManaPayloadKey {
    (mana.restrictions, mana.spend_effects, mana.color)
}

/// Every locatable mana unit in the catalog paired with the locator that
/// `mana_payload_locator` resolves it to.
///
/// Unlike the effect indexes above, a mana effect is matched by a relation
/// rather than by equality -- one that names a chosen colour serves any of
/// them. So each effect is offered every colour and the real predicate decides,
/// which keeps the index from restating which selections cover which colours
/// and then drifting from the search that does.
#[cfg(test)]
pub(in crate::game::state_checkpoint) fn mana_payload_locator_index(
    catalog: &CardCatalog,
) -> HashMap<ManaPayloadKey, ManaPayloadLocator> {
    let mut index = HashMap::new();
    let walk: ControlFlow<()> = visit_authored_abilities(catalog, &mut |ability, root, nested| {
        for (effect_index, effect) in mana_effects(ability).into_iter().enumerate() {
            // Unrestricted mana rides as a plain coloured count and the search
            // declines to locate it at all.
            if effect.restrictions.is_empty() && effect.spend_effects.is_empty() {
                continue;
            }
            for color in ManaColor::ALL {
                let offered = Mana {
                    color,
                    source: None,
                    restrictions: effect.restrictions,
                    spend_effects: effect.spend_effects,
                };
                if mana_effect_matches(effect, offered) {
                    index
                        .entry(mana_payload_key(offered))
                        .or_insert_with(|| ManaPayloadLocator {
                            ability: root.locator(nested.to_vec()),
                            effect_index,
                        });
                }
            }
        }
        ControlFlow::Continue(())
    });
    debug_assert!(walk.is_continue(), "the index walk never breaks");
    index
}
