//! Ravnica: Clue Edition cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardType, ComparisonDef, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, ResolvedEffectDurationDef,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

static ANOTHER_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// X is read once, as this resolves. It sets a base rather than adding to
/// one, so it overwrites an earlier setting effect while leaving counters and
/// ordinary pumps to apply on top.
static KRASIS_LENDS_ITS_BODY: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    effect: AppliedEffectDef::set_base_power_toughness(
        ValueDef::SourcePower,
        ValueDef::SourcePower,
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

/// Adapt is a conditional, not a cost: the ability always activates and
/// always resolves, and finding a counter already there is what makes it do
/// nothing. So a creature that lost its counters can adapt again.
static KRASIS_ADAPTS: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceCounters {
        kind: CounterKind::PlusOnePlusOne,
        comparison: ComparisonDef::LessOrEqual,
        amount: 0,
    },
    then: &EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(3),
    },
};

static UNRULY_KRASIS_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered_with_targets(
        "Whenever this creature attacks, you may have the base power and toughness of another target creature you control become X/X until end of turn, where X is this creature's power.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        &ANOTHER_CREATURE_YOU_CONTROL,
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &KRASIS_LENDS_ITS_BODY,
        },
    ),
    AbilityDef::activated(
        "{3}{G}{U}: Adapt 3. (If this creature has no +1/+1 counters on it, put three +1/+1 counters on it.)",
        &[AbilityCostDef::Mana(mana_cost!("{3}{G}{U}"))],
        KRASIS_ADAPTS,
    ),
];

// CLU 50 — Unruly Krasis
pub(in crate::card::sets) static UNRULY_KRASIS: CardRecord = CardRecord::new_with_legacy_id(
    2144,
    "Unruly Krasis",
    CardArt::new("a3b1b58d-b7f1-404f-aec6-b19cef4bebbd", "Billy Christian"),
    CardSet::RavnicaClueEdition,
    CardRules::new_creature(
        mana_cost!("{1}{G}{U}"),
        &["Shark", "Octopus", "Lizard"],
        4,
        4,
    )
    .with_abilities(&UNRULY_KRASIS_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&UNRULY_KRASIS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
