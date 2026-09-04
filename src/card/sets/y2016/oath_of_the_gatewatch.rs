//! Oath of the Gatewatch card records.

use super::{CardRecord, PrintingRecord};

// OGW 141 — Pulse of Murasa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULSE_OF_MURASA: CardRecord = CardRecord::new(
    crate::card::CardSet::OathOfTheGatewatch,
    "Pulse of Murasa",
    "c0c8057f-b45b-4f67-90cd-c808b5e9cbfa",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&PULSE_OF_MURASA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
