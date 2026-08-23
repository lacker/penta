//! RNA card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// RNA 115 — Skewer the Critics
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKEWER_THE_CRITICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97295660-6bea-46ae-9a3b-0fc6abba407f"),
    "Skewer the Critics",
    crate::card::CardArt::new("97295660-6bea-46ae-9a3b-0fc6abba407f", "Heonhwa"),
    crate::card::CardSet::RavnicaAllegiance,
    crate::card::CardRules::unsupported(),
);

// RNA 172 — Fireblade Artist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIREBLADE_ARTIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21e1161f-bd2c-45a7-a86b-3b2e5210f148"),
    "Fireblade Artist",
    crate::card::CardArt::new("21e1161f-bd2c-45a7-a86b-3b2e5210f148", "Steve Argyle"),
    crate::card::CardSet::RavnicaAllegiance,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SKEWER_THE_CRITICS, &FIREBLADE_ARTIST];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
