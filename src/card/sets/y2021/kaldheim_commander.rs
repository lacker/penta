//! Kaldheim Commander card records.

use super::{CardRecord, PrintingRecord};

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "KHC",
    slug: "kaldheim-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// KHC 1 — Lathril, Blade of the Elves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LATHRIL_BLADE_OF_THE_ELVES: CardRecord = CardRecord::new(
    "Lathril, Blade of the Elves",
    "547888c3-a9a6-4413-b29a-6bcd8a9279bf",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&LATHRIL_BLADE_OF_THE_ELVES];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
