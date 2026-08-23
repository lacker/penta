//! Born of the Gods card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// BNG 119 — Courser of Kruphix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COURSER_OF_KRUPHIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da5a807f-58e8-4d92-a61c-47bb9b28977f"),
    "Courser of Kruphix",
    crate::card::CardArt::new("da5a807f-58e8-4d92-a61c-47bb9b28977f", "Eric Deschamps"),
    crate::card::CardSet::BornOfTheGods,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COURSER_OF_KRUPHIX];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
