//! Doctor Who cards cataloged for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WHO",
    slug: "doctor-who",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// WHO 146 — The Master, Multiplied
// Audit: unsupported — Needs a player rule that prevents triggered abilities from causing sacrifice or exile of creature tokens.
pub(in crate::card::sets) static THE_MASTER_MULTIPLIED: CardRecord = CardRecord::new(
    "The Master, Multiplied",
    "7f734ca0-91bc-4496-9bd7-2d09415e850f",
    "Lie Setiawan",
    CardRules::unsupported(),
);

// WHO 186 — Sonic Screwdriver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONIC_SCREWDRIVER_186: CardRecord = CardRecord::new(
    "Sonic Screwdriver",
    "2a55d514-b670-4fe2-825b-9ba955977cac",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// WHO 189 — Ominous Cemetery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMINOUS_CEMETERY_189: CardRecord = CardRecord::new(
    "Ominous Cemetery",
    "2e843c57-fae3-4127-94e7-cad8c8bb9486",
    "Anato Finnstark",
    crate::card::CardRules::unsupported(),
);

// WHO 353 — Auton Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUTON_SOLDIER_353: CardRecord = CardRecord::new(
    "Auton Soldier",
    "2697012c-81a6-48e7-96cd-e6745078e0d9",
    "Greg Opalinski",
    crate::card::CardRules::unsupported(),
);

// WHO 359 — Flesh Duplicate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLESH_DUPLICATE_359: CardRecord = CardRecord::new(
    "Flesh Duplicate",
    "0ebb1d26-13d1-4907-95d6-8766b233fc73",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// WHO 408 — Dinosaurs on a Spaceship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DINOSAURS_ON_A_SPACESHIP_408: CardRecord = CardRecord::new(
    "Dinosaurs on a Spaceship",
    "391e1747-69b0-47e0-9210-317f5febca59",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// WHO 707 — The Foretold Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_FORETOLD_SOLDIER_707: CardRecord = CardRecord::new(
    "The Foretold Soldier",
    "923ce269-f8a6-4d4b-adbf-463241a001b0",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// WHO 745 — Last Night Together
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_NIGHT_TOGETHER_745: CardRecord = CardRecord::new(
    "Last Night Together",
    "08fbaf77-8afa-41c7-804b-ba7b740fffe8",
    "Pierre Loyvet",
    crate::card::CardRules::unsupported(),
);

// WHO 1053 — Gallifrey Council Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLIFREY_COUNCIL_CHAMBER_1053: CardRecord = CardRecord::new(
    "Gallifrey Council Chamber",
    "0e03370a-05a7-4f84-aadd-9eba459e2696",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_MASTER_MULTIPLIED,
    &SONIC_SCREWDRIVER_186,
    &OMINOUS_CEMETERY_189,
    &AUTON_SOLDIER_353,
    &FLESH_DUPLICATE_359,
    &DINOSAURS_ON_A_SPACESHIP_408,
    &THE_FORETOLD_SOLDIER_707,
    &LAST_NIGHT_TOGETHER_745,
    &GALLIFREY_COUNCIL_CHAMBER_1053,
];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
