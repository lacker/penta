//! Bloomburrow Commander cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, ComparisonDef,
    CounterKind, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ReplacementEffectDef, TriggerConditionDef, TriggerEventDef, ValueComparisonDef, ValueDef,
    ZoneKind,
};
use crate::mana_cost;

// BLC 9 — Jacked Rabbit
/// Ravenous reads the X the spell was cast for, which the permanent recorded
/// as it arrived: the entering object is a new one, so the X the spell chose
/// is not on it any more.
static RAVENOUS_REACHES_FIVE: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::SourceCastX,
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(5),
};

static RAVENOUS_WAS_BIG: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&RAVENOUS_REACHES_FIVE);

static JACKED_RABBIT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::as_enters(
        "Ravenous (This creature enters with X +1/+1 counters on it.)",
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCastXCounters {
                kind: CounterKind::PlusOnePlusOne,
            },
        ),
    ),
    AbilityDef::triggered_if(
        "If X is 5 or more, draw a card when this creature enters.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &RAVENOUS_WAS_BIG,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
    AbilityDef::triggered(
        "Whenever this creature attacks, create a number of 1/1 white Rabbit creature tokens \
         equal to this creature's power.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::create_creature_token(&["Rabbit"], &[ManaColor::White], 1, 1)
            .with_art(CardArt::new(
                "81de52ef-7515-4958-abea-fb8ebdcef93c",
                "Gina Matarazzo",
            ))
            .with_count(ValueDef::SourcePower),
    ),
];

pub(in crate::card::sets) static JACKED_RABBIT: CardRecord = CardRecord::new_with_legacy_id(
    2250,
    "Jacked Rabbit",
    CardArt::new("2c695df6-6bf2-4e6b-8500-e3116137ca27", "Scott Murphy"),
    CardSet::BloomburrowCommander,
    // The counters are the body and the body is the token count, so every
    // mana past the second is another Rabbit on every attack.
    CardRules::new_creature(mana_cost!("{X}{1}{W}"), &["Rabbit", "Warrior"], 1, 2)
        .with_abilities(&JACKED_RABBIT_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&JACKED_RABBIT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
