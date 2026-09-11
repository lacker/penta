//! Commander 2019 card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C19",
    slug: "commander-2019",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());















// C19 5 — Sevinne's Reclamation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEVINNE_S_RECLAMATION_5: CardRecord = CardRecord::new(
    "Sevinne's Reclamation",
    "7e68f4df-88ce-4e09-a03c-7edf40bff167",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// C19 11 — Sudden Substitution
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUDDEN_SUBSTITUTION_11: CardRecord = CardRecord::new(
    "Sudden Substitution",
    "9f7983bf-2a3b-4428-8c01-35285f589da8",
    "Noah Bradley",
    crate::card::CardRules::unsupported(),
);

// C19 16 — Curse of Fool's Wisdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURSE_OF_FOOL_S_WISDOM_16: CardRecord = CardRecord::new(
    "Curse of Fool's Wisdom",
    "9b77ded4-a8af-4065-8c4a-fd76e7cdcc59",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// C19 18 — K'rrik, Son of Yawgmoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static K_RRIK_SON_OF_YAWGMOTH_18: CardRecord = CardRecord::new(
    "K'rrik, Son of Yawgmoth",
    "3592fbe4-8588-486e-99ba-c327b0b6ba24",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// C19 31 — Apex Altisaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APEX_ALTISAUR_31: CardRecord = CardRecord::new(
    "Apex Altisaur",
    "5f8b022e-3e7a-40e6-99a0-53e5adbdafc5",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// C19 37 — Anje Falkenrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANJE_FALKENRATH_37: CardRecord = CardRecord::new(
    "Anje Falkenrath",
    "913dd06f-ed2f-4128-9c9d-9cd0d8a55425",
    "Cynthia Sheppard",
    crate::card::CardRules::unsupported(),
);

// C19 41 — Gerrard, Weatherlight Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_WEATHERLIGHT_HERO_41: CardRecord = CardRecord::new(
    "Gerrard, Weatherlight Hero",
    "c835a37a-776a-44ef-a252-4af1f31bf0b3",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SEVINNE_S_RECLAMATION_5,
    &SUDDEN_SUBSTITUTION_11,
    &CURSE_OF_FOOL_S_WISDOM_16,
    &K_RRIK_SON_OF_YAWGMOTH_18,
    &APEX_ALTISAUR_31,
    &ANJE_FALKENRATH_37,
    &GERRARD_WEATHERLIGHT_HERO_41,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
