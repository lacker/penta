//! Commander 2011 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype, CostDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, TriggerConditionDef, ValueDef, abilities,
};
use crate::mana_cost;

// CMD 244 — Champion's Helm
pub(in crate::card::sets) static CHAMPIONS_HELM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dcad6846-0b35-4193-b647-16e597357f9b"),
    "Champion's Helm",
    CardArt::new("dcad6846-0b35-4193-b647-16e597357f9b", "Alan Pollack"),
    CardSet::Commander2011,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is legendary, it has hexproof. (It can't be the target of spells or abilities your opponents control.)",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    },
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CHAMPIONS_HELM];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
