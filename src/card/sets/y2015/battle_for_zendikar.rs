//! BFZ card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// BFZ 58 — Eldrazi Skyspawner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELDRAZI_SKYSPAWNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c9c1a10-446e-492a-95cc-a459dc6c08a0"),
    "Eldrazi Skyspawner",
    crate::card::CardArt::new("9c9c1a10-446e-492a-95cc-a459dc6c08a0", "Chase Stone"),
    crate::card::CardSet::BattleForZendikar,
    crate::card::CardRules::unsupported(),
);

// BFZ 106 — Carrier Thrall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARRIER_THRALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd2ab895-9225-4eba-90c3-4023db4f8b70"),
    "Carrier Thrall",
    crate::card::CardArt::new("bd2ab895-9225-4eba-90c3-4023db4f8b70", "Lius Lasahido"),
    crate::card::CardSet::BattleForZendikar,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ELDRAZI_SKYSPAWNER, &CARRIER_THRALL];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
