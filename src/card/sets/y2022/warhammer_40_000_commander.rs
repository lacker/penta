//! Warhammer 40,000 Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "40K",
    slug: "warhammer-40-000-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// 40K 8 — Marneus Calgar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARNEUS_CALGAR_8: CardRecord = CardRecord::new(
    "Marneus Calgar",
    "e7517e8e-b424-4731-ba9d-6132bdefa6bf",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// 40K 51★ — Psychomancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHOMANCER_51_: CardRecord = CardRecord::new(
    "Psychomancer",
    "32c7cab2-4fc9-4d53-ba67-902f72799d20",
    "Alex Konstad",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MARNEUS_CALGAR_8,
    &PSYCHOMANCER_51_,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
