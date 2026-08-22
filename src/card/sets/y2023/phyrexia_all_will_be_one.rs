//! Phyrexia: All Will Be One cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    ObjectPredicateDef,
};
use crate::mana_cost;

static AN_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

static AN_ENCHANTMENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Enchantment),
)];

/// Two of the three answer something and the third answers nothing, which is
/// the point: a mode that only needs a counter on the board is what keeps
/// the card from being dead against a deck with no artifacts.
static CANKERBLOOM_MODES: [AbilityDef; 3] = [
    AbilityDef::destroy_target("Destroy target artifact.", &AN_ARTIFACT[0], true),
    AbilityDef::destroy_target("Destroy target enchantment.", &AN_ENCHANTMENT[0], true),
    AbilityDef::spell(
        "Proliferate. (Choose any number of permanents and/or players, then give each another \
         counter of each kind already there.)",
        EffectDef::Proliferate,
    ),
];

static CANKERBLOOM_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::SacrificeSource,
];

// ONE 161 — Cankerbloom
pub(in crate::card::sets) static CANKERBLOOM: CardRecord = CardRecord::new_with_legacy_id(
    2292,
    "Cankerbloom",
    CardArt::new("89b39293-6f57-4294-85fc-c718bdbb4d40", "Nicholas Gregory"),
    CardSet::PhyrexiaAllWillBeOne,
    // A 3/2 for two that is also the artifact removal the deck was going to
    // have to find room for, which is the whole reason it is in a cube.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Phyrexian", "Fungus"], 3, 2).with_ability(
        AbilityDef::modal_activated(
            "{1}, Sacrifice this creature: Choose one —\n• Destroy target artifact.\n• Destroy \
             target enchantment.\n• Proliferate.",
            &CANKERBLOOM_COST,
            &CANKERBLOOM_MODES,
            1,
            1,
            false,
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CANKERBLOOM];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
