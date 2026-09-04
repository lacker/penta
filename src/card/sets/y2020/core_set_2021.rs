//! Core Set 2021 card records.

use super::{CardRecord, PrintingRecord};

// M21 71 — Shipwreck Dowser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIPWRECK_DOWSER: CardRecord = CardRecord::new(
    crate::card::CardSet::CoreSet2021,
    "Shipwreck Dowser",
    "59d38ef7-5017-4ea3-b97f-a8fe12d03e98",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

// M21 126 — Village Rites
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILLAGE_RITES: CardRecord = CardRecord::new(
    crate::card::CardSet::CoreSet2021,
    "Village Rites",
    "9c0f60a6-b5c8-4704-8b61-94e8fc463e5d",
    "Bud Cook",
    crate::card::CardRules::unsupported(),
);

// M21 193 — Llanowar Visionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_VISIONARY: CardRecord = CardRecord::new(
    crate::card::CardSet::CoreSet2021,
    "Llanowar Visionary",
    "d6e23afa-7e08-4049-baf0-d4d0134ba2c8",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SHIPWRECK_DOWSER, &VILLAGE_RITES, &LLANOWAR_VISIONARY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
