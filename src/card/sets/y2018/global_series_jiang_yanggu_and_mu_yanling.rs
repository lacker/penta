//! Global Series Jiang Yanggu & Mu Yanling card records.

use super::{CardRecord, PrintingRecord};
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "GS1",
    slug: "global-series-jiang-yanggu-and-mu-yanling",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// GS1 12 — Ancestor Dragon
pub(in crate::card::sets) static ANCESTOR_DRAGON: CardRecord = CardRecord::new(
    "Ancestor Dragon",
    "9ba9d1f0-a864-490c-b258-6bf9de251c4b",
    "Shinchuen Chen",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Dragon"], 5, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever one or more creatures you control attack, you gain 1 \
             life for each attacking creature.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                )),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ANCESTOR_DRAGON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
