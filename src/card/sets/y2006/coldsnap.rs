//! Coldsnap card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// CSP 138 — Mishra's Bauble
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MISHRA_S_BAUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a720448-017f-4f4a-9501-678245eaed17"),
    "Mishra's Bauble",
    crate::card::CardArt::new("8a720448-017f-4f4a-9501-678245eaed17", "Chippy"),
    crate::card::CardSet::Coldsnap,
    crate::card::CardRules::unsupported(),
);

// CSP 145 — Dark Depths
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_DEPTHS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92409c3a-fb1a-4205-9fe1-0f5affc7b21d"),
    "Dark Depths",
    crate::card::CardArt::new("92409c3a-fb1a-4205-9fe1-0f5affc7b21d", "Stephan Martiniere"),
    crate::card::CardSet::Coldsnap,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MISHRA_S_BAUBLE, &DARK_DEPTHS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
