//! Jumpstart 2022 card records.

use super::{CardRecord, PrintingRecord};

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "J22",
    slug: "jumpstart-2022",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// J22 5 — Ingenious Leonin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INGENIOUS_LEONIN: CardRecord = CardRecord::new(
    "Ingenious Leonin",
    "4c6fd0bf-3f02-46f3-9c9a-931eab190584",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// J22 22 — Deadly Plot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADLY_PLOT: CardRecord = CardRecord::new(
    "Deadly Plot",
    "9c78da23-e7ec-4a3d-9f79-09fb86993b26",
    "Peter Polach",
    crate::card::CardRules::unsupported(),
);

// J22 27 — Suspicious Shambler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSPICIOUS_SHAMBLER: CardRecord = CardRecord::new(
    "Suspicious Shambler",
    "ff36347c-1fc1-4493-8e45-ef3372ecce45",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// J22 41 — Mild-Mannered Librarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MILD_MANNERED_LIBRARIAN: CardRecord = CardRecord::new(
    "Mild-Mannered Librarian",
    "3eee6f29-ef06-47f6-99af-fb0ff88f09d0",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &INGENIOUS_LEONIN,
    &DEADLY_PLOT,
    &SUSPICIOUS_SHAMBLER,
    &MILD_MANNERED_LIBRARIAN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
