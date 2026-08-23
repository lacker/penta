//! SOK card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// SOK 63 — Death Denied
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_DENIED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f66ddc5-f5e6-44de-8189-87b6521d1fea"),
    "Death Denied",
    crate::card::CardArt::new("8f66ddc5-f5e6-44de-8189-87b6521d1fea", "Greg Hildebrandt"),
    crate::card::CardSet::SaviorsOfKamigawa,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DEATH_DENIED];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
