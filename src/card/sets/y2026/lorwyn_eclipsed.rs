//! ECL card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};

// ECL 128 — Brambleback Brute
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRAMBLEBACK_BRUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ebb8365-c6e1-46e8-a242-6aa27b21e68a"),
    "Brambleback Brute",
    crate::card::CardArt::new("5ebb8365-c6e1-46e8-a242-6aa27b21e68a", "Aaron Miller"),
    crate::card::CardSet::LorwynEclipsed,
    crate::card::CardRules::unsupported(),
);

// ECL 181 — Lys Alana Informant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LYS_ALANA_INFORMANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a79649c4-559e-4306-a102-5fd8750629c7"),
    "Lys Alana Informant",
    crate::card::CardArt::new(
        "a79649c4-559e-4306-a102-5fd8750629c7",
        "Sidharth Chaturvedi",
    ),
    crate::card::CardSet::LorwynEclipsed,
    crate::card::CardRules::unsupported(),
);

// ECL 251 — Wary Farmer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WARY_FARMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22d20c0d-176d-49c9-aa0b-2c5778548cc5"),
    "Wary Farmer",
    crate::card::CardArt::new("22d20c0d-176d-49c9-aa0b-2c5778548cc5", "Ron Spears"),
    crate::card::CardSet::LorwynEclipsed,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&BRAMBLEBACK_BRUTE, &LYS_ALANA_INFORMANT, &WARY_FARMER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
