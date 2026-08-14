//! Modern Horizons 3 cards cataloged as attachment edge cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, EffectDef, EffectDurationDef,
    EffectRecipientDef, ValueDef, abilities, cards,
};
use crate::mana_cost;

// MH3 148 — Colossal Dreadmask
pub(in crate::card::sets) static COLOSSAL_DREADMASK: CardRecord = CardRecord::new(
    cards::COLOSSAL_DREADMASK,
    "Colossal Dreadmask",
    CardArt::new("98164430-64c1-465f-b786-45753c965f44", "Caio Monteiro"),
    CardSet::ModernHorizons3,
    CardRules::new_artifact(mana_cost!("{4}{G}{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(cards::GERM_TOKEN_0_0_BLACK),
            AbilityDef::static_ability(
                "Equipped creature gets +6/+6 and has trample.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(6),
                            toughness: ValueDef::Constant(6),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::trample()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            abilities::equip(mana_cost!("{3}{G}{G}"), "Equip {3}{G}{G}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COLOSSAL_DREADMASK];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
