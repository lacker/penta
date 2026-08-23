//! M19 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// M19 29 — Militia Bugler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MILITIA_BUGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43c5bf25-937c-4e17-9ed4-b4c4579fa9dc"),
    "Militia Bugler",
    crate::card::CardArt::new("43c5bf25-937c-4e17-9ed4-b4c4579fa9dc", "David Gaillet"),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

// M19 125 — Vampire Sovereign
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRE_SOVEREIGN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee338221-ead9-4b89-8b0c-12745c4ca13d"),
    "Vampire Sovereign",
    crate::card::CardArt::new("ee338221-ead9-4b89-8b0c-12745c4ca13d", "Volkan Baǵa"),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

// M19 134 — Dark-Dweller Oracle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_DWELLER_ORACLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69a57bfc-1de2-4b3a-84bc-19ec41087f0d"),
    "Dark-Dweller Oracle",
    crate::card::CardArt::new(
        "69a57bfc-1de2-4b3a-84bc-19ec41087f0d",
        "Deruchenko Alexander",
    ),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

// M19 143 — Goblin Motivator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MOTIVATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2"),
    "Goblin Motivator",
    crate::card::CardArt::new("94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2", "Johann Bodin"),
    crate::card::CardSet::CoreSet2019,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MILITIA_BUGLER,
    &VAMPIRE_SOVEREIGN,
    &DARK_DWELLER_ORACLE,
    &GOBLIN_MOTIVATOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
