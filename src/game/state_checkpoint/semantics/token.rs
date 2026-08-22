//! Durable semantic paths for inline token characteristics.
//!
//! Checkpoints never serialize [`crate::CardRules`]: rules contain static
//! behavior pointers and are only meaningful in the engine artifact that
//! authored them. A token is instead addressed through its card-, token-, or
//! emblem-owned creating ability and effect-tree position, or through a custom
//! creator registry entry. Recursive virtual-object creators remain durably
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
        .find_map(|(token, locator)| (token.semantic_identity() == expected).then_some(locator))
}

pub(in crate::game::state_checkpoint) fn catalog_token_characteristics(
    catalog: &CardCatalog,
    locator: &TokenCharacteristicsLocator,
) -> Option<TokenCharacteristics> {
    let creator = catalog_ability(catalog, locator.creator())?;
    match locator {
        TokenCharacteristicsLocator::EffectPath { effect_path, .. } => {
            let effect = effect_at_path(&creator, effect_path)?;
            match effect {
                EffectDef::CreateToken { token, .. } | EffectDef::CreateAttachedToken { token } => {
                    Some(token)
                }
                _ => None,
            }
        }
        TokenCharacteristicsLocator::Custom { token_index, .. } => {
            let behavior = creator.effect.custom_behavior()?;
            crate::card::tokens::custom_created_tokens(behavior)
                .get(*token_index)
                .copied()
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
