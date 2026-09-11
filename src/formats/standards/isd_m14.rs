use super::super::{CONSTRUCTED_RULES, SetFormatDefinition};
use crate::card::CardSet;
use crate::card::sets;

pub const BANNED_CARDS: &[&str] = &[];
pub const RESTRICTED_CARDS: &[&str] = &[];
pub const ALLOWED_SETS: &[CardSet] = &[
    sets::innistrad::SET,
    sets::dark_ascension::SET,
    sets::avacyn_restored::SET,
    sets::magic_2013::SET,
    sets::return_to_ravnica::SET,
    sets::gatecrash::SET,
    sets::dragons_maze::SET,
    sets::magic_2014::SET,
];

pub(in crate::formats) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: CONSTRUCTED_RULES,
    allowed_sets: ALLOWED_SETS,
    additional_allowed_cards: &[],
    banned_cards: BANNED_CARDS,
    restricted_cards: RESTRICTED_CARDS,
};
