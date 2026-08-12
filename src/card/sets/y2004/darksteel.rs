//! Darksteel cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, EffectDef,
    ManaColor, abilities, cards,
};
use crate::mana_cost;

// DST 112 — Darksteel Ingot
pub(in crate::card::sets) static DARKSTEEL_INGOT: CardRecord = CardRecord::new(
    cards::DARKSTEEL_INGOT,
    "Darksteel Ingot",
    CardArt::new("b02b9634-77e9-48ae-a6bf-859598d12c52", "Martina Pilcerova"),
    CardSet::Darksteel,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DARKSTEEL_INGOT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
