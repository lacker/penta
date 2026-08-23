//! Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, abilities,
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

// MRD 146 — Bonesplitter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONESPLITTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae31d513-7412-4467-b497-a7183ff29a42"),
    "Bonesplitter",
    crate::card::CardArt::new("465a7990-c9f9-4716-a833-fd41458b9cee", "Darrell Riche"),
    crate::card::CardSet::Mirrodin,
    crate::card::CardRules::unsupported(),
);

// MRD 199 — Lightning Greaves
pub(in crate::card::sets) static LIGHTNING_GREAVES: CardRecord = CardRecord::new_with_legacy_id(
    2170,
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BONESPLITTER, &LIGHTNING_GREAVES];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
