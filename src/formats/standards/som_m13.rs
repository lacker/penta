use super::super::{CONSTRUCTED_RULES, SetFormatDefinition};
use crate::card::CardSet;
use crate::card::sets;

pub const ALLOWED_SETS: &[CardSet] = &[
    sets::scars_of_mirrodin::SET,
    sets::mirrodin_besieged::SET,
    sets::new_phyrexia::SET,
    sets::magic_2012::SET,
    sets::innistrad::SET,
    sets::dark_ascension::SET,
    sets::avacyn_restored::SET,
    sets::magic_2013::SET,
];

pub(in crate::formats) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: CONSTRUCTED_RULES,
    allowed_sets: ALLOWED_SETS,
    additional_allowed_cards: &[],
    banned_cards: &[],
    restricted_cards: &[],
};
