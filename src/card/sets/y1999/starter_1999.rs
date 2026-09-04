//! Starter 1999 card records.

use super::{CardRecord, PrintingRecord};

// S99 15 — Eager Cadet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EAGER_CADET: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Eager Cadet",
    "d1e1ce2f-d8af-4fd0-975e-9d910d12b883",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// S99 59 — Vizzerdrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIZZERDRIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Vizzerdrix",
    "25711022-7270-4335-a48b-9f2b8275ceeb",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// S99 71 — Dakmor Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAKMOR_LANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Dakmor Lancer",
    "9d012ddf-abe1-4de9-89cb-78d82afb9e7b",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// S99 99 — Goblin Chariot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CHARIOT: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Goblin Chariot",
    "9ca11a7e-17f8-419f-9ba8-1bcaa3860f8b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// S99 120 — Trained Orgg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINED_ORGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Trained Orgg",
    "425540b0-c826-4814-b0df-032264b1c237",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// S99 139 — Pride of Lions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIDE_OF_LIONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Pride of Lions",
    "f5006984-8e3d-4f13-b12e-1fbecd134bb3",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// S99 143 — Squall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Starter1999,
    "Squall",
    "63c1b2f6-e47f-4f18-a94a-1d08eb009ef3",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EAGER_CADET,
    &VIZZERDRIX,
    &DAKMOR_LANCER,
    &GOBLIN_CHARIOT,
    &TRAINED_ORGG,
    &PRIDE_OF_LIONS,
    &SQUALL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
