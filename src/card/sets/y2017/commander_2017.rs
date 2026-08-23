//! Commander 2017 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// C17 37 — Fractured Identity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FRACTURED_IDENTITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2f73f5d-1aad-48c2-9e74-5f7bdd87900f"),
    "Fractured Identity",
    crate::card::CardArt::new("b2f73f5d-1aad-48c2-9e74-5f7bdd87900f", "Yongjae Choi"),
    crate::card::CardSet::Commander2017,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FRACTURED_IDENTITY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
