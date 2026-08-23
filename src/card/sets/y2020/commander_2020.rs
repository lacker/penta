//! C20 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// C20 67 — Bonder's Ornament
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONDER_S_ORNAMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5afe425c-50a7-4d29-ac14-0edb094fc770"),
    "Bonder's Ornament",
    crate::card::CardArt::new("5afe425c-50a7-4d29-ac14-0edb094fc770", "Lindsey Look"),
    crate::card::CardSet::Commander2020,
    crate::card::CardRules::unsupported(),
);

// C20 118 — Murmuring Mystic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MURMURING_MYSTIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fc6adff-dcb3-456d-a8c2-0e77b784ff89"),
    "Murmuring Mystic",
    crate::card::CardArt::new("ab25853c-29d3-4244-88db-813300a262a5", "Mark Winters"),
    crate::card::CardSet::Commander2020,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BONDER_S_ORNAMENT, &MURMURING_MYSTIC];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
