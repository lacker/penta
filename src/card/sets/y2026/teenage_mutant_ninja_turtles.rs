//! TLE card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet =
    crate::card::CardSet::new("TLE", "teenage-mutant-ninja-turtles");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// TLE 276 — Wolf Cove Villager
pub(in crate::card::sets) static WOLF_COVE_VILLAGER: CardRecord = CardRecord::new(
    "Wolf Cove Villager",
    "993652d5-b44b-4142-a081-427edb480dcf",
    "Gemi",
    // A 2/2 for one, paid for entirely by arriving tapped: it blocks the
    // turn after it lands and never the turn it does.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Peasant"], 2, 2)
        .with_ability(abilities::enters_tapped(CardType::Creature)),
);

// TLE 285 — Warship Scout
pub(in crate::card::sets) static WARSHIP_SCOUT: CardRecord = CardRecord::new(
    "Warship Scout",
    "f47fc407-5b7d-4c9d-90b4-3eb234f9f18b",
    "Brandon L. Hunt",
    // A vanilla 2/1 for one: nothing is missing from the definition, the
    // card simply prints no rules text.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Scout"], 2, 1),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&WOLF_COVE_VILLAGER, &WARSHIP_SCOUT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
