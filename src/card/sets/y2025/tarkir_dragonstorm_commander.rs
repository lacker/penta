//! Tarkir: Dragonstorm Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TDC",
    slug: "tarkir-dragonstorm-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());







// TDC 3 — Eshki, Temur's Roar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESHKI_TEMUR_S_ROAR_3: CardRecord = CardRecord::new(
    "Eshki, Temur's Roar",
    "ff9aa863-8773-452d-946c-ae334c632e11",
    "Billy Christian",
    crate::card::CardRules::unsupported(),
);

// TDC 51 — Ainok Strike Leader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AINOK_STRIKE_LEADER_51: CardRecord = CardRecord::new(
    "Ainok Strike Leader",
    "2d1102e4-447a-4b40-8131-5b36b8979fe0",
    "Alexander Mokhov",
    crate::card::CardRules::unsupported(),
);

// TDC 80 — Will of the Jeskai
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILL_OF_THE_JESKAI_80: CardRecord = CardRecord::new(
    "Will of the Jeskai",
    "3ef5e587-8b80-4ea9-9788-367541101ecc",
    "Jessica Fong",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ESHKI_TEMUR_S_ROAR_3,
    &AINOK_STRIKE_LEADER_51,
    &WILL_OF_THE_JESKAI_80,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
