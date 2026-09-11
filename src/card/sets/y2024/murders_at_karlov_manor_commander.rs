//! Murders at Karlov Manor Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MKC",
    slug: "murders-at-karlov-manor-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// MKC 326 — Trouble in Pairs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROUBLE_IN_PAIRS_326: CardRecord = CardRecord::new(
    "Trouble in Pairs",
    "0dd4d070-38cf-4517-8152-84c9fcf2c984",
    "Fay Dalton",
    crate::card::CardRules::unsupported(),
);

// MKC 358 — Ransom Note
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANSOM_NOTE_358: CardRecord = CardRecord::new(
    "Ransom Note",
    "05f9437a-50c2-415f-afa9-39f64f3aa3da",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &TROUBLE_IN_PAIRS_326,
    &RANSOM_NOTE_358,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
