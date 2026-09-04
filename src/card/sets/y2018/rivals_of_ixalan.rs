//! Rivals of Ixalan card records.

use super::{CardRecord, PrintingRecord};

// RIX 101 — Fanatical Firebrand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FANATICAL_FIREBRAND: CardRecord = CardRecord::new(
    crate::card::CardSet::RivalsOfIxalan,
    "Fanatical Firebrand",
    "5e5565de-028c-4799-a9f6-4dcd685639eb",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FANATICAL_FIREBRAND];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
