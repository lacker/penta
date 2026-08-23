use super::super::{CONSTRUCTED_RULES, SetFormatDefinition};
use crate::card::CardSet;

pub const ALLOWED_SETS: &[CardSet] = &[
    CardSet::Innistrad,
    CardSet::DarkAscension,
    CardSet::AvacynRestored,
    CardSet::Magic2013,
    CardSet::ReturnToRavnica,
    CardSet::Gatecrash,
    CardSet::DragonsMaze,
    CardSet::Magic2014,
];

pub(in crate::formats) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: CONSTRUCTED_RULES,
    allowed_sets: ALLOWED_SETS,
    banned_cards: &[],
    restricted_cards: &[],
};
