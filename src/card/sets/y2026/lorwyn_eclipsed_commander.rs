//! Lorwyn Eclipsed Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ECC",
    slug: "lorwyn-eclipsed-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());



// ECC 44 — Sodden Verdure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SODDEN_VERDURE_44: CardRecord = CardRecord::new(
    "Sodden Verdure",
    "9030440a-a049-4152-afcf-b19648b20ce6",
    "Raymond Bonilla",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SODDEN_VERDURE_44,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
