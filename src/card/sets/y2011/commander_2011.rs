//! Commander 2011 card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::CardType;
use crate::CounterKind;
use crate::TargetConditionDef;
use crate::TargetIndex;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("CMD", "commander-2011");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CMD 170 — Scavenging Ooze
/// One when the exiled card was a creature, nothing otherwise.
static EXILED_A_CREATURE: TargetConditionDef = TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new(
    "Scavenging Ooze",
    "371ceb58-f498-4616-a7f0-eb118fe2e4ff",
    "Austin Hsu",
CardRules::new_creature(
        mana_cost!("{1}{G}"),
        &["Ooze"],
        2,
        2,
    )
    .with_ability(
        AbilityDef::activated_with_targets("{G}: Exile target card from a graveyard. If it was a creature card, put a +1/+1 counter on this creature and you gain 1 life.", &[CostDef::Mana(mana_cost!("{G}"))], &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )], // The counter and the life come first so the card is still in the
            // graveyard to be asked what it was. Exiling it first would leave
            // nothing to look at, and nothing here can observe the order.
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::IfTargetMatches(&EXILED_A_CREATURE),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::IfTargetMatches(&EXILED_A_CREATURE),
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
},
            ])),
    ),
);

// CMD 244 — Champion's Helm
pub(in crate::card::sets) static CHAMPIONS_HELM: CardRecord = CardRecord::new(
    "Champion's Helm",
    "dcad6846-0b35-4193-b647-16e597357f9b",
    "Alan Pollack",
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SCAVENGING_OOZE, &CHAMPIONS_HELM];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
