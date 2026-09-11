//! Outlaws of Thunder Junction Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OTC",
    slug: "outlaws-of-thunder-junction-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// OTC 3 — Stella Lee, Wild Card
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STELLA_LEE_WILD_CARD_3: CardRecord = CardRecord::new(
    "Stella Lee, Wild Card",
    "2a8a7696-b5d9-4378-9d5c-2c9007e4df63",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// OTC 51 — Lock and Load
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOCK_AND_LOAD_51: CardRecord = CardRecord::new(
    "Lock and Load",
    "3a24979d-a090-4153-9461-ac1aa1f69b74",
    "Anastasia Ovchinnikova",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &STELLA_LEE_WILD_CARD_3,
    &LOCK_AND_LOAD_51,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
