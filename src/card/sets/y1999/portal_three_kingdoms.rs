//! Portal Three Kingdoms card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// PTK 78 — Imperial Seal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMPERIAL_SEAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("822e30db-40c5-4099-868b-185ad9b7c7dc"),
    "Imperial Seal",
    crate::card::CardArt::new("822e30db-40c5-4099-868b-185ad9b7c7dc", "Li Tie"),
    crate::card::CardSet::PortalThreeKingdoms,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&IMPERIAL_SEAL];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
