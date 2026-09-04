//! TLE card records required by supported formats.

use super::{CardRecord, PrintingRecord};

// TLE 276 — Wolf Cove Villager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOLF_COVE_VILLAGER: CardRecord = CardRecord::new(
    crate::card::CardSet::TeenageMutantNinjaTurtles,
    "Wolf Cove Villager",
    "993652d5-b44b-4142-a081-427edb480dcf",
    "Gemi",
    crate::card::CardRules::unsupported(),
);

// TLE 285 — Warship Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARSHIP_SCOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::TeenageMutantNinjaTurtles,
    "Warship Scout",
    "f47fc407-5b7d-4c9d-90b4-3eb234f9f18b",
    "Brandon L. Hunt",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&WOLF_COVE_VILLAGER, &WARSHIP_SCOUT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
