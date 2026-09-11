//! The Brothers' War Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BRC",
    slug: "the-brothers-war-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// BRC 16 — Machine God's Effigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MACHINE_GOD_S_EFFIGY_16: CardRecord = CardRecord::new(
    "Machine God's Effigy",
    "637f69c2-ba24-42d1-9345-8ebdb04b6904",
    "Martin de Diego Sádaba",
    crate::card::CardRules::unsupported(),
);

// BRC 51 — Urza's Workshop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_WORKSHOP_51: CardRecord = CardRecord::new(
    "Urza's Workshop",
    "1e01c1e2-f657-4a24-b8b0-561911c4e754",
    "Alexander Forssberg",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &MACHINE_GOD_S_EFFIGY_16,
    &URZA_S_WORKSHOP_51,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
