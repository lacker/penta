//! The Lost Caverns of Ixalan Commander card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// LCC 86 — Broadside Bombardiers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BROADSIDE_BOMBARDIERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9721f8da-39ed-4ada-a571-61e08a86032b"),
    "Broadside Bombardiers",
    crate::card::CardArt::new("9721f8da-39ed-4ada-a571-61e08a86032b", "Tomek Larek"),
    crate::card::CardSet::LostCavernsOfIxalanCommander,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BROADSIDE_BOMBARDIERS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
