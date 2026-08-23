//! Ikoria: Lair of Behemoths cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{AbilityDef, CardArt, CardRules, CardSet, abilities};
use crate::mana_cost;

/// A triome is a tapped land with three basic land types and cycling, and
/// nothing else. Its printed mana ability is reminder text for what the
/// subtypes already grant, so it is not restated as a clause.
const TRIOME_ABILITIES: &[AbilityDef] = &[
    abilities::enters_tapped("This land enters tapped."),
    abilities::cycling(
        "Cycling {3} ({3}, Discard this card: Draw a card.)",
        mana_cost!("{3}"),
    ),
];

const fn triome(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(TRIOME_ABILITIES)
}

// IKO 137 — Spelleater Wolverine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLEATER_WOLVERINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5f03ffd-dcdb-441c-8dfc-4fe06a289b22"),
    "Spelleater Wolverine",
    crate::card::CardArt::new("a5f03ffd-dcdb-441c-8dfc-4fe06a289b22", "Uriah Voth"),
    crate::card::CardSet::Ikoria,
    crate::card::CardRules::unsupported(),
);

// IKO 170 — Ram Through
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAM_THROUGH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac0b24e7-14e7-45ee-b5d8-bdb8674b669c"),
    "Ram Through",
    crate::card::CardArt::new("ac0b24e7-14e7-45ee-b5d8-bdb8674b669c", "Zoltan Boros"),
    crate::card::CardSet::Ikoria,
    crate::card::CardRules::unsupported(),
);

// IKO 248 — Indatha Triome
pub(in crate::card::sets) static INDATHA_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2096,
    "Indatha Triome",
    CardArt::new("2b74bb81-fb9a-40e5-a941-e517430b52f5", "Noah Bradley"),
    CardSet::Ikoria,
    triome(&["Plains", "Swamp", "Forest"]),
);

// IKO 250 — Ketria Triome
pub(in crate::card::sets) static KETRIA_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2097,
    "Ketria Triome",
    CardArt::new("a249b1f4-2b22-4b67-a207-e0c4ae95d2e1", "Sam Burley"),
    CardSet::Ikoria,
    triome(&["Forest", "Island", "Mountain"]),
);

// IKO 251 — Raugrin Triome
pub(in crate::card::sets) static RAUGRIN_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2098,
    "Raugrin Triome",
    CardArt::new("02138fbb-3962-4348-8d31-faaefba0b8b2", "Jonas De Ro"),
    CardSet::Ikoria,
    triome(&["Island", "Mountain", "Plains"]),
);

// IKO 253 — Savai Triome
pub(in crate::card::sets) static SAVAI_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2099,
    "Savai Triome",
    CardArt::new("748e6a61-9c1f-4225-9f04-e54002f63ac3", "Titus Lunter"),
    CardSet::Ikoria,
    triome(&["Mountain", "Plains", "Swamp"]),
);

// IKO 259 — Zagoth Triome
pub(in crate::card::sets) static ZAGOTH_TRIOME: CardRecord = CardRecord::new_with_legacy_id(
    2100,
    "Zagoth Triome",
    CardArt::new("cc520518-2063-4b57-a0d4-10cf62a7175e", "Eytan Zana"),
    CardSet::Ikoria,
    triome(&["Swamp", "Forest", "Island"]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SPELLEATER_WOLVERINE,
    &RAM_THROUGH,
    &INDATHA_TRIOME,
    &KETRIA_TRIOME,
    &RAUGRIN_TRIOME,
    &SAVAI_TRIOME,
    &ZAGOTH_TRIOME,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
