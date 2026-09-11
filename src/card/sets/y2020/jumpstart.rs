//! Jumpstart card records.

use super::{CardRecord, PrintingRecord};

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "JMP",
    slug: "jumpstart",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// JMP 4 — Release the Dogs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELEASE_THE_DOGS: CardRecord = CardRecord::new(
    "Release the Dogs",
    "7df3cd89-02c9-4a1c-9a8a-d17a0b1030c9",
    "Jason Kang",
    crate::card::CardRules::unsupported(),
);

// JMP 11 — Corsair Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORSAIR_CAPTAIN: CardRecord = CardRecord::new(
    "Corsair Captain",
    "a9b016d4-ddf6-47d6-b934-a0b979b60680",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&RELEASE_THE_DOGS, &CORSAIR_CAPTAIN];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
