//! Durable semantic paths for inline token characteristics.
//!
//! Checkpoints never serialize [`crate::CardRules`]: rules contain static
//! behavior pointers and are only meaningful in the engine artifact that
//! authored them. A token is instead addressed through its card-, token-, or
//! emblem-owned creating ability and effect-tree position. Recursive virtual-object creators remain durably
//! rooted in a printed card ability.

use super::catalog_ability;
use super::emblem::{catalog_emblem_characteristics, emblem_characteristics_locator};
use super::virtual_objects::{authored_virtual_objects, effect_at_path};
use crate::card::EffectDef;
use crate::{CardCatalog, CardPartId, ObjectCharacteristics, TokenCharacteristics};

use super::super::model::{
    FaceDownCharacteristicsSnapshot, ObjectCharacteristicsSnapshot, TokenCharacteristicsLocator,
};

pub(in crate::game::state_checkpoint) fn face_down_characteristics_snapshot(
    characteristics: crate::FaceDownCharacteristics,
) -> Option<FaceDownCharacteristicsSnapshot> {
    if characteristics == crate::card::face_down::ordinary() {
        Some(FaceDownCharacteristicsSnapshot::OrdinaryTwoTwo)
    } else if characteristics == crate::card::face_down::disguise() {
        Some(FaceDownCharacteristicsSnapshot::WardTwoTwo)
    } else {
        None
    }
}

pub(in crate::game::state_checkpoint) const fn face_down_characteristics_from_snapshot(
    snapshot: FaceDownCharacteristicsSnapshot,
) -> crate::FaceDownCharacteristics {
    match snapshot {
        FaceDownCharacteristicsSnapshot::OrdinaryTwoTwo => crate::card::face_down::ordinary(),
        FaceDownCharacteristicsSnapshot::WardTwoTwo => crate::card::face_down::disguise(),
    }
}

pub(in crate::game::state_checkpoint) fn token_characteristics_locator(
    catalog: &CardCatalog,
    expected: TokenCharacteristics,
) -> Option<TokenCharacteristicsLocator> {
    let expected = expected.semantic_identity();
    authored_tokens(catalog)
        .into_iter()
        .find_map(|(token, mut locator)| {
            let colors = expected.rules().color_set();
            let basic_land_type_words = expected.basic_land_type_word_map();
            let color_words = expected.color_word_map();
            if token
                .with_color_set(colors)
                .with_word_maps(basic_land_type_words, color_words)
                .semantic_identity()
                != expected
            {
                return None;
            }
            let TokenCharacteristicsLocator::EffectPath {
                colors: override_colors,
                basic_land_type_words: override_basic_land_type_words,
                color_words: override_color_words,
                ..
            } = &mut locator;
            if token.rules().color_set() != colors {
                *override_colors = Some(colors.to_flags());
            }
            if token.basic_land_type_word_map() != basic_land_type_words {
                *override_basic_land_type_words =
                    Some(basic_land_type_words.map(super::super::basic_land_type_snapshot));
            }
            if token.color_word_map() != color_words {
                *override_color_words = Some(color_words.map(super::super::mana_color_snapshot));
            }
            Some(locator)
        })
}

pub(in crate::game::state_checkpoint) fn catalog_token_characteristics(
    catalog: &CardCatalog,
    locator: &TokenCharacteristicsLocator,
) -> Option<TokenCharacteristics> {
    let creator = catalog_ability(catalog, locator.creator())?;
    match locator {
        TokenCharacteristicsLocator::EffectPath {
            effect_path,
            colors,
            basic_land_type_words,
            color_words,
            ..
        } => {
            let effect = effect_at_path(&creator, effect_path)?;
            let token = match effect {
                EffectDef::CreateToken {
                    token, copy: None, ..
                }
                | EffectDef::CreateAttachedToken { token, .. } => Some(token),
                _ => None,
            }?;
            let token = colors.map_or(token, |colors| {
                let colors = crate::card::ManaColor::COLORS
                    .into_iter()
                    .zip(colors)
                    .filter_map(|(color, present)| present.then_some(color))
                    .fold(crate::card::ColorSet::empty(), crate::card::ColorSet::with);
                token.with_color_set(colors)
            });
            let basic_land_type_words = basic_land_type_words
                .map_or(token.basic_land_type_word_map(), |words| {
                    words.map(super::super::parse_basic_land_type)
                });
            let color_words = color_words.map_or(token.color_word_map(), |words| {
                words.map(super::super::parse_mana_color)
            });
            Some(token.with_word_maps(basic_land_type_words, color_words))
        }
    }
}

pub(in crate::game::state_checkpoint) fn object_characteristics_snapshot(
    catalog: &CardCatalog,
    characteristics: ObjectCharacteristics,
) -> Option<ObjectCharacteristicsSnapshot> {
    match characteristics {
        ObjectCharacteristics::Card { definition, part } => {
            catalog.get(definition)?.part(part)?;
            Some(ObjectCharacteristicsSnapshot::Card {
                definition,
                part_id: part.0,
            })
        }
        ObjectCharacteristics::Token { token, part } => {
            token.part(part)?;
            Some(ObjectCharacteristicsSnapshot::Token {
                token: token_characteristics_locator(catalog, token)?,
                part_id: part.0,
            })
        }
        ObjectCharacteristics::Emblem { emblem } => Some(ObjectCharacteristicsSnapshot::Emblem {
            emblem: emblem_characteristics_locator(catalog, emblem)?,
        }),
        ObjectCharacteristics::FaceDown { face_down } => {
            Some(ObjectCharacteristicsSnapshot::FaceDown {
                face_down: face_down_characteristics_snapshot(face_down)?,
            })
        }
    }
}

pub(in crate::game::state_checkpoint) fn object_characteristics_from_snapshot(
    catalog: &CardCatalog,
    snapshot: &ObjectCharacteristicsSnapshot,
) -> Option<ObjectCharacteristics> {
    match snapshot {
        ObjectCharacteristicsSnapshot::Card {
            definition,
            part_id,
        } => {
            let definition = *definition;
            let part = CardPartId(*part_id);
            catalog.get(definition)?.part(part)?;
            Some(ObjectCharacteristics::card(definition, part))
        }
        ObjectCharacteristicsSnapshot::Token { token, part_id } => {
            let token = catalog_token_characteristics(catalog, token)?;
            let part = CardPartId(*part_id);
            token.part(part)?;
            Some(ObjectCharacteristics::token(token, part))
        }
        ObjectCharacteristicsSnapshot::Emblem { emblem } => Some(ObjectCharacteristics::emblem(
            catalog_emblem_characteristics(catalog, emblem)?,
        )),
        ObjectCharacteristicsSnapshot::FaceDown { face_down } => Some(
            ObjectCharacteristics::face_down(face_down_characteristics_from_snapshot(*face_down)),
        ),
    }
}

pub(super) fn authored_tokens(
    catalog: &CardCatalog,
) -> Vec<(TokenCharacteristics, TokenCharacteristicsLocator)> {
    authored_virtual_objects(catalog).tokens
}

#[cfg(test)]
mod tests;
