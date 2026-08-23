//! Dominaria United Commander card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// DMC 47 — Torsten, Founder of Benalia
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TORSTEN_FOUNDER_OF_BENALIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0783b426-a527-42c1-9271-be28b229e1c6"),
    "Torsten, Founder of Benalia",
    crate::card::CardArt::new("0783b426-a527-42c1-9271-be28b229e1c6", "Volkan Baǵa"),
    crate::card::CardSet::DominariaUnitedCommander,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&TORSTEN_FOUNDER_OF_BENALIA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
