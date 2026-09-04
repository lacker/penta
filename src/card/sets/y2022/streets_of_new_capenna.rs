//! Streets of New Capenna cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardRules, CardSet, ComparisonDef, ObjectPredicateDef, PlayerRelation,
    QuantifierDef, TriggerConditionDef, TriggerEventDef, abilities,
};
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

// SNC 18 — Inspiring Overseer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRING_OVERSEER: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Inspiring Overseer",
    "35d9da1d-8678-4252-b0f8-9960795642f0",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// SNC 26 — Raffine's Informant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAFFINE_S_INFORMANT: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Raffine's Informant",
    "4e64ff87-2099-4360-94f6-164277b7b514",
    "John Stanko",
    crate::card::CardRules::unsupported(),
);

// SNC 46 — Ledger Shredder
pub(in crate::card::sets) static LEDGER_SHREDDER: CardRecord = CardRecord::new(
    CardSet::StreetsOfNewCapenna,
    "Ledger Shredder",
    "7ea4b5bc-18a4-45db-a56a-ab3f8bd2fb0d",
    "Mila Pesic",
    // Two mana that filters a hand and gets bigger for it, and does both on
    // the opponent's turn too.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird", "Advisor"], 1, 3)
        .with_abilities(&[
            abilities::flying(),
            // A player, not you: the Shredder grows on their turn as readily as on
            // yours, which is what makes it a two-drop worth playing in a deck that
            // is not casting two spells a turn itself.
            AbilityDef::triggered_if(
                "Whenever a player casts their second spell each turn, this creature connives. (Draw a \
                 card, then discard a card. If you discarded a nonland card, put a +1/+1 counter on this \
                 creature.)",
                TriggerEventDef::spell_cast(ObjectPredicateDef::Any),
                // Exactly the second, not the second or later: the spell that caused the
                // trigger has already been counted by the time this is read. "Their"
                // second, so the count is the casting player's own rather than anybody's.
                &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::EventPlayer,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
                abilities::connive(),
            ),
        ]),
);

// SNC 66 — Witness Protection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITNESS_PROTECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Witness Protection",
    "a2be6f2c-8ad0-402d-a7ca-9fe817e83b72",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// SNC 114 — Mayhem Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAYHEM_PATROL: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Mayhem Patrol",
    "50162cdd-ba30-48df-93ff-197c7f4a2913",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// SNC 131 — Witty Roastmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITTY_ROASTMASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Witty Roastmaster",
    "71d13f19-482b-4a2e-9692-b7d7caf2f9f5",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// SNC 151 — Jewel Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEWEL_THIEF: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Jewel Thief",
    "736e498e-1245-40c1-96a4-c9bcfd1cfe1f",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// SNC 168 — Body Dropper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BODY_DROPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::StreetsOfNewCapenna,
    "Body Dropper",
    "0fcb6d47-dccb-4b69-aed4-7a6215857606",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// SNC 250 — Jetmir's Garden
pub(in crate::card::sets) static JETMIRS_GARDEN: CardRecord = CardRecord::new(
    CardSet::StreetsOfNewCapenna,
    "Jetmir's Garden",
    "26d40e03-6de4-4373-9fbf-04c1dd79e995",
    "Kasia 'Kafis' Zielińska",
    triome(&["Mountain", "Forest", "Plains"]),
);

// SNC 254 — Raffine's Tower
pub(in crate::card::sets) static RAFFINES_TOWER: CardRecord = CardRecord::new(
    CardSet::StreetsOfNewCapenna,
    "Raffine's Tower",
    "a2c56479-4bee-4edb-80d7-4af010b7c793",
    "Sam White",
    triome(&["Plains", "Island", "Swamp"]),
);

// SNC 257 — Spara's Headquarters
pub(in crate::card::sets) static SPARAS_HEADQUARTERS: CardRecord = CardRecord::new(
    CardSet::StreetsOfNewCapenna,
    "Spara's Headquarters",
    "7363f1fb-9af3-4212-921f-d59533faf0e5",
    "Kieran Yanner",
    triome(&["Forest", "Plains", "Island"]),
);

// SNC 260 — Xander's Lounge
pub(in crate::card::sets) static XANDERS_LOUNGE: CardRecord = CardRecord::new(
    CardSet::StreetsOfNewCapenna,
    "Xander's Lounge",
    "54f449ff-4025-465e-9ec5-a5cf42c4c9d3",
    "James Paick",
    triome(&["Island", "Swamp", "Mountain"]),
);

// SNC 261 — Ziatora's Proving Ground
pub(in crate::card::sets) static ZIATORAS_PROVING_GROUND: CardRecord = CardRecord::new(
    CardSet::StreetsOfNewCapenna,
    "Ziatora's Proving Ground",
    "75fdce80-e338-4a50-bdc6-786511feaeef",
    "Viko Menezes",
    triome(&["Swamp", "Mountain", "Forest"]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &INSPIRING_OVERSEER,
    &RAFFINE_S_INFORMANT,
    &LEDGER_SHREDDER,
    &WITNESS_PROTECTION,
    &MAYHEM_PATROL,
    &WITTY_ROASTMASTER,
    &JEWEL_THIEF,
    &BODY_DROPPER,
    &JETMIRS_GARDEN,
    &RAFFINES_TOWER,
    &SPARAS_HEADQUARTERS,
    &XANDERS_LOUNGE,
    &ZIATORAS_PROVING_GROUND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
