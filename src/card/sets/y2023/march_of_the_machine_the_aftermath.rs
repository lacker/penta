//! March of the Machine: The Aftermath card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MAT",
    slug: "march-of-the-machine-the-aftermath",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());









// MAT 19 — Reckless Handling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_HANDLING_19: CardRecord = CardRecord::new(
    "Reckless Handling",
    "239165bb-7819-4d54-a84c-2911934253d6",
    "Miguel Mercado",
    crate::card::CardRules::unsupported(),
);

// MAT 164 — Tranquil Frillback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANQUIL_FRILLBACK_164: CardRecord = CardRecord::new(
    "Tranquil Frillback",
    "38c3a774-986d-4702-81c3-915138d3d718",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// MAT 176 — Ob Nixilis, Captive Kingpin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OB_NIXILIS_CAPTIVE_KINGPIN_176: CardRecord = CardRecord::new(
    "Ob Nixilis, Captive Kingpin",
    "7f613db0-3303-4d54-a060-1bd8b37a208f",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// MAT 180 — Samut, Vizier of Naktamun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMUT_VIZIER_OF_NAKTAMUN_180: CardRecord = CardRecord::new(
    "Samut, Vizier of Naktamun",
    "30c3f755-d70d-46d7-80dd-428ca1a355c4",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &RECKLESS_HANDLING_19,
    &TRANQUIL_FRILLBACK_164,
    &OB_NIXILIS_CAPTIVE_KINGPIN_176,
    &SAMUT_VIZIER_OF_NAKTAMUN_180,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
