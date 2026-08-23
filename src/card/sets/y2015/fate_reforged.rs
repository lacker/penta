//! FRF card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// FRF 72 — Gurmag Angler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GURMAG_ANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c60a8cf1-a8c7-4f45-bbd3-188fab2652f9"),
    "Gurmag Angler",
    crate::card::CardArt::new("c60a8cf1-a8c7-4f45-bbd3-188fab2652f9", "YW Tang"),
    crate::card::CardSet::FateReforged,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GURMAG_ANGLER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
