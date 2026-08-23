//! HML card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// HML 110 — Serrated Arrows
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERRATED_ARROWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("849a7d2b-3fdb-4e7f-b0b6-f6559dcb32e2"),
    "Serrated Arrows",
    crate::card::CardArt::new("849a7d2b-3fdb-4e7f-b0b6-f6559dcb32e2", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SERRATED_ARROWS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
