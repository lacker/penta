//! Fallout cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, CardArt, CardRules, CardSet, CardType, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities,
};
use crate::mana_cost;

// PIP 23 — Securitron Squadron
/// A creature token you control arriving, whichever ability made it -- this
/// card's own squad copies included, if squad ever pays.
static A_CREATURE_TOKEN_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Token,
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static SECURITRON_SQUADRON_ABILITIES: [AbilityDef; 3] = [
    // Squad is an additional cost paid any number of times, and the enters
    // trigger reads how many. Nothing here records how a spell was paid for,
    // and no cost repeats, so the clause is shown rather than run.
    AbilityDef::static_ability(
        "Squad {3} (As an additional cost to cast this spell, you may pay {3} any number of times. When this creature enters, create that many tokens that are copies of it.)",
        EffectDef::None,
    )
    .with_coverage(AbilityCoverageDef::metadata_only(
        "Squad is withheld: an additional cost cannot be paid a number of times the caster \
         chooses, and nothing carries that number from the cast to the enters trigger.",
    )),
    abilities::vigilance(),
    AbilityDef::triggered(
        "Whenever a creature token you control enters, put a +1/+1 counter on it.",
        TriggerEventDef::zone_changed(
            A_CREATURE_TOKEN_YOU_CONTROL,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::AddCounters {
            object: EffectRecipientDef::TriggeringObject,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
];

pub(in crate::card::sets) static SECURITRON_SQUADRON: CardRecord = CardRecord::new_with_legacy_id(
    2151,
    "Securitron Squadron",
    CardArt::new("b689a206-aec3-4a31-95cf-3d4b840db04c", "Jonas De Ro"),
    CardSet::Fallout,
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Robot"], 2, 2)
        .with_abilities(&SECURITRON_SQUADRON_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SECURITRON_SQUADRON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
