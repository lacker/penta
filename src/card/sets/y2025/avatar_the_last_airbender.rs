//! TLA card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// TLA 267 — Boiling Rock Prison
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOILING_ROCK_PRISON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c2e2220-54d1-4180-93a0-964e3b0ba8b8"),
    "Boiling Rock Prison",
    crate::card::CardArt::new("1c2e2220-54d1-4180-93a0-964e3b0ba8b8", "Matteo Bassini"),
    crate::card::CardSet::AvatarTheLastAirbender,
    crate::card::CardRules::unsupported(),
);

// TLA 271 — Kyoshi Village
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KYOSHI_VILLAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d5f3008-2af8-4e81-8847-1c91f524e747"),
    "Kyoshi Village",
    crate::card::CardArt::new("8d5f3008-2af8-4e81-8847-1c91f524e747", "Luc Courtois"),
    crate::card::CardSet::AvatarTheLastAirbender,
    crate::card::CardRules::unsupported(),
);

// TLA 279 — Serpent's Pass
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENT_S_PASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad87bff5-9b8c-44e4-a6d3-8cc71be9640a"),
    "Serpent's Pass",
    crate::card::CardArt::new("ad87bff5-9b8c-44e4-a6d3-8cc71be9640a", "Matteo Bassini"),
    crate::card::CardSet::AvatarTheLastAirbender,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&BOILING_ROCK_PRISON, &KYOSHI_VILLAGE, &SERPENT_S_PASS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
