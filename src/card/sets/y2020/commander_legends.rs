//! Commander Legends card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// CMR 74 — Hullbreacher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HULLBREACHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4df8aabc-7fcb-4b7b-980b-18f499e6c170"),
    "Hullbreacher",
    crate::card::CardArt::new(
        "4df8aabc-7fcb-4b7b-980b-18f499e6c170",
        "Sidharth Chaturvedi",
    ),
    crate::card::CardSet::CommanderLegends,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&HULLBREACHER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
