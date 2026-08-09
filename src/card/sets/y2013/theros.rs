//! Theros cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, BasicLandType, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectDurationDef, EffectRecipientDef,
    ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, cards,
};
use crate::ids::TargetSlotId;
use crate::mana_cost;

pub(in crate::card::sets) static NYLEAS_PRESENCE: CardRecord = CardRecord::new(
    cards::NYLEAS_PRESENCE,
    "Nylea's Presence",
    CardArt::new("e68f1fd4-1a2f-405b-a592-6c4af6214eae", "Ralph Horsley"),
    CardSet::Theros,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell(
                "Enchant land",
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetSlotId(0)),
                },
            )
            .with_targets(&[AbilityTargetDef::exactly_one(
                TargetSlotId(0),
                "land",
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )]),
            AbilityDef::triggered(
                "When Nylea's Presence enters, draw a card.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land is every basic land type in addition to its other types.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::AddLandTypes(&BasicLandType::ALL),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&NYLEAS_PRESENCE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
