//! ARB card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// ARB 29 — Soul Manipulation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_MANIPULATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcd3cb05-c6f9-435a-a0e7-1f85da4a36eb"),
    "Soul Manipulation",
    crate::card::CardArt::new("bcd3cb05-c6f9-435a-a0e7-1f85da4a36eb", "Carl Critchlow"),
    crate::card::CardSet::AlaraReborn,
    crate::card::CardRules::unsupported(),
);

// ARB 95 — Putrid Leech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aaa47568-5668-4a9f-ad1c-9a13010ffc2b"),
    "Putrid Leech",
    crate::card::CardArt::new("aaa47568-5668-4a9f-ad1c-9a13010ffc2b", "Dave Allsop"),
    crate::card::CardSet::AlaraReborn,
    crate::card::CardRules::unsupported(),
);

// ARB 133 — Thopter Foundry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THOPTER_FOUNDRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42b8d797-b01d-49cf-9818-d84bba17029d"),
    "Thopter Foundry",
    crate::card::CardArt::new("42b8d797-b01d-49cf-9818-d84bba17029d", "Ralph Horsley"),
    crate::card::CardSet::AlaraReborn,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&SOUL_MANIPULATION, &PUTRID_LEECH, &THOPTER_FOUNDRY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
