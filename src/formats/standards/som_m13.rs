use super::super::{CONSTRUCTED_RULES, SetFormatDefinition};
use crate::card::CardSet;

pub const ALLOWED_SETS: &[CardSet] = &[
    CardSet::ScarsOfMirrodin,
    CardSet::MirrodinBesieged,
    CardSet::NewPhyrexia,
    CardSet::Magic2012,
    CardSet::Innistrad,
    CardSet::DarkAscension,
    CardSet::AvacynRestored,
    CardSet::Magic2013,
];

pub(in crate::formats) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: CONSTRUCTED_RULES,
    allowed_sets: ALLOWED_SETS,
    additional_allowed_cards: &[],
    banned_cards: &[],
    restricted_cards: &[],
};
