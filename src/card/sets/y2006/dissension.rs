//! DIS card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// DIS 10 — Guardian of the Guildpact
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUARDIAN_OF_THE_GUILDPACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8dd004b-01e4-4fe1-a164-9f2ea8d7d88e"),
    "Guardian of the Guildpact",
    crate::card::CardArt::new("c8dd004b-01e4-4fe1-a164-9f2ea8d7d88e", "Fred Hooper"),
    crate::card::CardSet::Dissension,
    crate::card::CardRules::unsupported(),
);

// DIS 99 — Utopia Sprawl
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UTOPIA_SPRAWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5047e271-fbf1-402c-9eb9-0806e5988f76"),
    "Utopia Sprawl",
    crate::card::CardArt::new("5047e271-fbf1-402c-9eb9-0806e5988f76", "Ron Spears"),
    crate::card::CardSet::Dissension,
    crate::card::CardRules::unsupported(),
);

// DIS 107 — Coiling Oracle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COILING_ORACLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c7b0fa1-bfc2-4b15-80ea-47e41a17aa2c"),
    "Coiling Oracle",
    crate::card::CardArt::new("55a6ba2a-b372-4b15-9a1e-09b41316eab7", "Mark Zug"),
    crate::card::CardSet::Dissension,
    crate::card::CardRules::unsupported(),
);

// DIS 178 — Rakdos Carnarium
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAKDOS_CARNARIUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34f146f3-6541-4d2a-96e3-a3cd680c0a1e"),
    "Rakdos Carnarium",
    crate::card::CardArt::new("34f146f3-6541-4d2a-96e3-a3cd680c0a1e", "John Avon"),
    crate::card::CardSet::Dissension,
    crate::card::CardRules::unsupported(),
);

// DIS 180 — Simic Growth Chamber
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SIMIC_GROWTH_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("407d0a0c-a6be-4bd5-8355-1715698c6bde"),
    "Simic Growth Chamber",
    crate::card::CardArt::new("407d0a0c-a6be-4bd5-8355-1715698c6bde", "John Avon"),
    crate::card::CardSet::Dissension,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &GUARDIAN_OF_THE_GUILDPACT,
    &UTOPIA_SPRAWL,
    &COILING_ORACLE,
    &RAKDOS_CARNARIUM,
    &SIMIC_GROWTH_CHAMBER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
