//! TLA card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{CardRules, CardSet};

// TLA 144 — The Last Agni Kai
pub(in crate::card::sets) static THE_LAST_AGNI_KAI: CardRecord = CardRecord::new(
    CardSet::AvatarTheLastAirbender,
    "The Last Agni Kai",
    "61eaebc6-7575-48ed-b212-ff8b0c7ae694",
    "Pablo Rivera",
    // Audit: unsupported — Needs an effect-scoped mana-retention duration for only the excess mana it creates.
    CardRules::unsupported(),
);

// TLA 267 — Boiling Rock Prison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOILING_ROCK_PRISON: CardRecord = CardRecord::new(
    crate::card::CardSet::AvatarTheLastAirbender,
    "Boiling Rock Prison",
    "1c2e2220-54d1-4180-93a0-964e3b0ba8b8",
    "Matteo Bassini",
    crate::card::CardRules::unsupported(),
);

// TLA 271 — Kyoshi Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KYOSHI_VILLAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::AvatarTheLastAirbender,
    "Kyoshi Village",
    "8d5f3008-2af8-4e81-8847-1c91f524e747",
    "Luc Courtois",
    crate::card::CardRules::unsupported(),
);

// TLA 279 — Serpent's Pass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENT_S_PASS: CardRecord = CardRecord::new(
    crate::card::CardSet::AvatarTheLastAirbender,
    "Serpent's Pass",
    "ad87bff5-9b8c-44e4-a6d3-8cc71be9640a",
    "Matteo Bassini",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_LAST_AGNI_KAI,
    &BOILING_ROCK_PRISON,
    &KYOSHI_VILLAGE,
    &SERPENT_S_PASS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
