//! FDN card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// FDN 18 — Inspiring Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRING_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0763be06-25b2-4d6b-ab33-a1af85aeb443"),
    "Inspiring Paladin",
    crate::card::CardArt::new("0763be06-25b2-4d6b-ab33-a1af85aeb443", "Valera Lutfullina"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 114 — Treetop Snarespinner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_SNARESPINNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88e68fa3-159d-49a6-8ac6-afc9bd6f1718"),
    "Treetop Snarespinner",
    crate::card::CardArt::new("88e68fa3-159d-49a6-8ac6-afc9bd6f1718", "Steve Ellis"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 195 — Fanatical Firebrand
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FANATICAL_FIREBRAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e5565de-028c-4799-a9f6-4dcd685639eb"),
    "Fanatical Firebrand",
    crate::card::CardArt::new("d1296316-7781-4e98-95e6-7020648be6a5", "Wayne Reynolds"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 200 — Goblin Surprise
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SURPRISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e"),
    "Goblin Surprise",
    crate::card::CardArt::new("527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e", "Kevin Sidharta"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 330 — Kellan, Planar Trailblazer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KELLAN_PLANAR_TRAILBLAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e413f37-b59a-4302-86d3-2abce81edc78"),
    "Kellan, Planar Trailblazer",
    crate::card::CardArt::new("0e413f37-b59a-4302-86d3-2abce81edc78", "Aaron J. Riley"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 528 — Undying Malice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNDYING_MALICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8eb38041-043a-4b18-9d9a-f1283684e8f1"),
    "Undying Malice",
    crate::card::CardArt::new("97b3cf11-e352-4ee1-8c03-13898f576ef9", "Igor Kieryluk"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 596 — Shipwreck Dowser
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIPWRECK_DOWSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59d38ef7-5017-4ea3-b97f-a8fe12d03e98"),
    "Shipwreck Dowser",
    crate::card::CardArt::new("1f20fe3d-792a-4030-a25c-e81b48b2bcb4", "Caroline Gariba"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &INSPIRING_PALADIN,
    &TREETOP_SNARESPINNER,
    &FANATICAL_FIREBRAND,
    &GOBLIN_SURPRISE,
    &KELLAN_PLANAR_TRAILBLAZER,
    &UNDYING_MALICE,
    &SHIPWRECK_DOWSER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
