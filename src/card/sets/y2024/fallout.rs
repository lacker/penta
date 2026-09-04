//! Fallout cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardRules, CardSet, CardType, CopyExceptionsDef, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities,
};
use crate::{AdditionalCostIndex, mana_cost};

// PIP 23 — Securitron Squadron
pub(in crate::card::sets) static SECURITRON_SQUADRON: CardRecord = CardRecord::new(
    CardSet::Fallout,
    "Securitron Squadron",
    "b689a206-aec3-4a31-95cf-3d4b840db04c",
    "Jonas De Ro",
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Robot"], 2, 2).with_abilities(&[
        abilities::squad(mana_cost!("{3}")),
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, create that many tokens that are copies of it.",
            // "Create that many tokens that are copies of it": the count is how many
            // times the squad cost was paid, which the permanent carries over from the
            // cast that made it.
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::Source,
                exceptions: CopyExceptionsDef::NONE,
            })
            .with_count(ValueDef::AdditionalCostPayments(
                AdditionalCostIndex::PRIMARY,
            )),
        ),
        AbilityDef::triggered(
            "Whenever a creature token you control enters, put a +1/+1 counter on it.",
            TriggerEventDef::zone_changed(
                // A creature token you control arriving, whichever ability made it -- this
                // card's own squad copies included, if squad ever pays.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Token,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::TriggeringObject,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SECURITRON_SQUADRON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
