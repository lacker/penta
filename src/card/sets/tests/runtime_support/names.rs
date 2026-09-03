use crate::card::{CardNameDef, CardNameSetDef, ObjectSetDef};

pub(super) fn shared_card_name(name: CardNameDef) -> bool {
    matches!(
        name,
        CardNameDef::Literal(_)
            | CardNameDef::EffectChoice
            | CardNameDef::Binding(_)
            | CardNameDef::NameOf(_)
    )
}

pub(super) fn shared_card_name_set(names: CardNameSetDef) -> bool {
    match names {
        CardNameSetDef::Union(sets) => sets.iter().copied().all(shared_card_name_set),
        CardNameSetDef::NamesOf(objects)
        | CardNameSetDef::NamesAppearingAtLeast { objects, .. } => {
            matches!(objects, ObjectSetDef::Binding(_)) || super::shared_source_object_set(*objects)
        }
        CardNameSetDef::AllCardNames
        | CardNameSetDef::NonlandCardNames
        | CardNameSetDef::LandCardNames
        | CardNameSetDef::NonbasicLandCardNames
        | CardNameSetDef::CardNamesOtherThanBasicLands
        | CardNameSetDef::BasicLandNames => true,
    }
}
