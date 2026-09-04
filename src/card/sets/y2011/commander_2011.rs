//! Commander 2011 card records.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::AbilityDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::CardRules;
use crate::CardSet;
use crate::CardType;
use crate::CounterKind;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ObjectPredicateDef;
use crate::TargetConditionDef;
use crate::TargetIndex;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;

use crate::mana_cost;

// CMD 170 — Scavenging Ooze
/// One when the exiled card was a creature, nothing otherwise.
static EXILED_A_CREATURE: TargetConditionDef = TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    then: ValueDef::Constant(1),
    otherwise: ValueDef::Constant(0),
};

pub(in crate::card::sets) static SCAVENGING_OOZE: CardRecord = CardRecord::new(
    CardSet::Commander2011,
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
        AbilityDef::activated_with_targets("{G}: Exile target card from a graveyard. If it was a creature card, put a +1/+1 counter on this creature and you gain 1 life.", &[AbilityCostDef::Mana(mana_cost!("{G}"))], &[AbilityTargetDef::exactly_one(
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SCAVENGING_OOZE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
