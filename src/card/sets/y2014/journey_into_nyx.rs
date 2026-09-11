//! Journey into Nyx cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::EffectDef;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "JOU",
    slug: "journey-into-nyx",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// JOU 5 — Banishing Light
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BANISHING_LIGHT: CardRecord = CardRecord::new(
    "Banishing Light",
    "fbaa4800-30cc-4a80-a6cc-9a24ada9eb40",
    "Willian Murai",
    crate::card::CardRules::unsupported(),
);

// JOU 37 — Dictate of Kruphix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DICTATE_OF_KRUPHIX: CardRecord = CardRecord::new(
    "Dictate of Kruphix",
    "e8e7916c-f39a-48a0-a47d-7e83ebf028fa",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// JOU 126 — Heroes' Bane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEROES_BANE: CardRecord = CardRecord::new(
    "Heroes' Bane",
    "380e82f6-30ee-49c5-b5a2-85c5cf3e9bb1",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// JOU 163 — Mana Confluence
pub(in crate::card::sets) static MANA_CONFLUENCE: CardRecord = CardRecord::new(
    "Mana Confluence",
    "504a69eb-3c2d-4bb1-b117-252b15acf0c2",
    "Richard Wright",
    // City of Brass charges its life when it becomes tapped, by anyone and
    // for any reason. This charges it as a cost of its own ability, so a land
    // tapped by someone else costs nothing and an activation with no life to
    // spare is simply not offered.
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}, Pay 1 life: Add one mana of any color.",
        &[CostDef::TapSource, CostDef::PayLife(1)],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// JOU 164 — Temple of Epiphany
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPLE_OF_EPIPHANY: CardRecord = CardRecord::new(
    "Temple of Epiphany",
    "b882c6bf-b795-49fe-8242-a928aadb6f13",
    "Noah Bradley",
    crate::card::CardRules::unsupported(),
);

// JOU 165 — Temple of Malady
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPLE_OF_MALADY: CardRecord = CardRecord::new(
    "Temple of Malady",
    "f30220f1-1992-4b5c-9e13-1762fb673155",
    "James Paick",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BANISHING_LIGHT,
    &DICTATE_OF_KRUPHIX,
    &HEROES_BANE,
    &MANA_CONFLUENCE,
    &TEMPLE_OF_EPIPHANY,
    &TEMPLE_OF_MALADY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
