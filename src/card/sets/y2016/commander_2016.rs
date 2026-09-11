//! Commander 2016 card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C16",
    slug: "commander-2016",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

















// C16 8 — Faerie Artisans
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_ARTISANS_8: CardRecord = CardRecord::new(
    "Faerie Artisans",
    "ff05c503-2536-48f5-a639-480614a2e5f8",
    "Tony Foti",
    crate::card::CardRules::unsupported(),
);

// C16 34 — Kraum, Ludevic's Opus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAUM_LUDEVIC_S_OPUS_34: CardRecord = CardRecord::new(
    "Kraum, Ludevic's Opus",
    "557fcd17-6cb3-414a-b2b1-ea9ae32e5aec",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// C16 43 — Silas Renn, Seeker Adept
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILAS_RENN_SEEKER_ADEPT_43: CardRecord = CardRecord::new(
    "Silas Renn, Seeker Adept",
    "4e3fe912-1374-47c7-b73f-89ef55c479c1",
    "Joseph Meehan",
    crate::card::CardRules::unsupported(),
);

// C16 45 — Tana, the Bloodsower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANA_THE_BLOODSOWER_45: CardRecord = CardRecord::new(
    "Tana, the Bloodsower",
    "a3d8d64f-a403-42a7-881b-4f70e9fe15a2",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// C16 46 — Thrasios, Triton Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRASIOS_TRITON_HERO_46: CardRecord = CardRecord::new(
    "Thrasios, Triton Hero",
    "21e27b91-c7f1-4709-aa0d-8b5d81b22a0a",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// C16 48 — Tymna the Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TYMNA_THE_WEAVER_48: CardRecord = CardRecord::new(
    "Tymna the Weaver",
    "bc7cbe9b-324e-42b8-94e2-36e91cb32163",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// C16 53 — Conqueror's Flail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONQUEROR_S_FLAIL_53: CardRecord = CardRecord::new(
    "Conqueror's Flail",
    "1644f535-9281-4a88-8fac-916bc0021d1d",
    "Franz Vohwinkel",
    crate::card::CardRules::unsupported(),
);

// C16 56 — Ash Barrens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASH_BARRENS_56: CardRecord = CardRecord::new(
    "Ash Barrens",
    "9d7ac112-bb20-4ea4-b797-5d21f6b7c121",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FAERIE_ARTISANS_8,
    &KRAUM_LUDEVIC_S_OPUS_34,
    &SILAS_RENN_SEEKER_ADEPT_43,
    &TANA_THE_BLOODSOWER_45,
    &THRASIOS_TRITON_HERO_46,
    &TYMNA_THE_WEAVER_48,
    &CONQUEROR_S_FLAIL_53,
    &ASH_BARRENS_56,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
