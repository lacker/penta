//! EXO card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// EXO 53 — Carnophage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARNOPHAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d17c057f-cb1b-4895-831a-fb35c75d3845"),
    "Carnophage",
    crate::card::CardArt::new("d17c057f-cb1b-4895-831a-fb35c75d3845", "Pete Venters"),
    crate::card::CardSet::Exodus,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CARNOPHAGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
