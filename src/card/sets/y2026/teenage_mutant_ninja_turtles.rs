//! TLE card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// TLE 276 — Wolf Cove Villager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOLF_COVE_VILLAGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7dbeced9-d27d-476c-92d4-3c14d8a40458"),
    "Wolf Cove Villager",
    crate::card::CardArt::new("993652d5-b44b-4142-a081-427edb480dcf", "Gemi"),
    crate::card::CardSet::TeenageMutantNinjaTurtles,
    crate::card::CardRules::unsupported(),
);

// TLE 285 — Warship Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARSHIP_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1a95982-be16-465a-9c1b-1f4d875c0c40"),
    "Warship Scout",
    crate::card::CardArt::new("f47fc407-5b7d-4c9d-90b4-3eb234f9f18b", "Brandon L. Hunt"),
    crate::card::CardSet::TeenageMutantNinjaTurtles,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&WOLF_COVE_VILLAGER, &WARSHIP_SCOUT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
