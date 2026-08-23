//! Phyrexia: All Will Be One cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
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

// ONE 28 — Planar Disruption
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_DISRUPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ee69a1f-aeed-4eb4-8987-fa720fc99715"),
    "Planar Disruption",
    crate::card::CardArt::new("8ee69a1f-aeed-4eb4-8987-fa720fc99715", "Campbell White"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 108 — Sheoldred's Edict
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHEOLDRED_S_EDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9225cc3-90f0-448f-a8d9-7c6c2796d077"),
    "Sheoldred's Edict",
    crate::card::CardArt::new("a9225cc3-90f0-448f-a8d9-7c6c2796d077", "Helge C. Balzer"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 121 — Barbed Batterfist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_BATTERFIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de1d02d1-91dc-47d6-bdbe-87602428abfb"),
    "Barbed Batterfist",
    crate::card::CardArt::new("de1d02d1-91dc-47d6-bdbe-87602428abfb", "Randy Gallegos"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 133 — Furnace Strider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FURNACE_STRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa625ab0-1e79-4497-a5da-98fe1abfd024"),
    "Furnace Strider",
    crate::card::CardArt::new("aa625ab0-1e79-4497-a5da-98fe1abfd024", "Denis Zhbankov"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

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

// ONE 164 — Contagious Vorrac
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAGIOUS_VORRAC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("18af2c85-e58f-4043-99d3-e90121348aca"),
    "Contagious Vorrac",
    crate::card::CardArt::new("18af2c85-e58f-4043-99d3-e90121348aca", "Maxime Minard"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

// ONE 196 — Atraxa, Grand Unifier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ATRAXA_GRAND_UNIFIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a1f905f-1d55-4d02-9d24-e58070793d3f"),
    "Atraxa, Grand Unifier",
    crate::card::CardArt::new("4a1f905f-1d55-4d02-9d24-e58070793d3f", "Marta Nael"),
    crate::card::CardSet::PhyrexiaAllWillBeOne,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PLANAR_DISRUPTION,
    &SHEOLDRED_S_EDICT,
    &BARBED_BATTERFIST,
    &FURNACE_STRIDER,
    &CANKERBLOOM,
    &CONTAGIOUS_VORRAC,
    &ATRAXA_GRAND_UNIFIER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
