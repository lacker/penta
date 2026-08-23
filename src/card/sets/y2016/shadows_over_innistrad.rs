//! SOI card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// SOI 44 — Thraben Inspector
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRABEN_INSPECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d140c3b7-ca78-483d-baeb-307b624fea8b"),
    "Thraben Inspector",
    crate::card::CardArt::new("d140c3b7-ca78-483d-baeb-307b624fea8b", "Matt Stewart"),
    crate::card::CardSet::ShadowsOverInnistrad,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&THRABEN_INSPECTOR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
