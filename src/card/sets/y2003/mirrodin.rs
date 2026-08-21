//! Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, abilities, cards,
};
use crate::mana_cost;

static GREAVES_HASTE: AbilityDef = abilities::haste();

static GREAVES_SHROUD: AbilityDef = abilities::shroud();

/// The two halves are why the card is played: haste makes the creature useful
/// the turn it arrives, and shroud makes it hard to answer -- including by
/// its own controller, who cannot target it either.
static GREAVES_GRANTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&GREAVES_HASTE),
    AppliedEffectDef::add_ability(&GREAVES_SHROUD),
];

// MRD 199 — Lightning Greaves
pub(in crate::card::sets) static LIGHTNING_GREAVES: CardRecord = CardRecord::new(
    cards::LIGHTNING_GREAVES,
    "Lightning Greaves",
    CardArt::new("61a28870-cf78-4323-9d82-cee764067764", "Jeremy Jarvis"),
    CardSet::Mirrodin,
    // Equipping for nothing is the whole card: the Greaves move to whatever
    // just arrived, every turn, for as long as they are on the battlefield.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has haste and shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&GREAVES_GRANTS),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{0}"))], "Equip {0}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&LIGHTNING_GREAVES];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
