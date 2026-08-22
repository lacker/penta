//! Wilds of Eldraine cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityOperationDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CharacteristicOperationDef, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRelation, PlayerSetDef, TriggerConditionDef, ValueDef,
    ZoneKind,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// The Cauldron hands its abilities to creatures that are carrying a counter,
/// whoever put it there. Read every time the layer is walked, so a creature
/// that loses its last counter loses the abilities with it.
static COUNTERED_CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// "Target card from a graveyard" reaches every graveyard, not only its
/// controller's.
static CAULDRON_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    }),
    // The counter's target belongs to a reflexive trigger, which this engine
    // declares up front alongside the activation's own target. "Up to one"
    // rather than "one" is what keeps the activation legal for a player who
    // controls no creature, which the printed card allows: the reflexive
    // trigger simply never gets a target.
    AbilityTargetDef::up_to(
        AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Battlefield],
            controller: Some(PlayerRelation::You),
            owner: None,
        },
        1,
    ),
];

/// "When a creature card is exiled this way": asked of the card the
/// activation named, which by then has already moved to exile.
static A_CREATURE_CARD_WAS_EXILED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Creature),
};

static CAULDRON_GROWS_A_CREATURE: EffectDef = EffectDef::AddCounters {
    object: EffectRecipientDef::Target(TargetIndex(1)),
    kind: CounterKind::PlusOnePlusOne,
    amount: ValueDef::Constant(1),
};

static CAULDRON_EXILES_THEN_GROWS: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::IfCondition {
        condition: &A_CREATURE_CARD_WAS_EXILED,
        then: &CAULDRON_GROWS_A_CREATURE,
    },
];

static CAULDRON_TAP: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

static AGATHAS_SOUL_CAULDRON_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "You may spend mana as though it were mana of any color to activate abilities of \
         creatures you control.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
            effect: AppliedEffectDef::Rule(
                AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities,
            ),
        },
    ),
    AbilityDef::static_ability(
        "Creatures you control with +1/+1 counters on them have all activated abilities of all \
         creature cards exiled with Agatha's Soul Cauldron.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                COUNTERED_CREATURES_YOU_CONTROL,
            )),
            effect: AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles,
            )),
        },
    ),
    AbilityDef::activated_with_targets(
        "{T}: Exile target card from a graveyard. When a creature card is exiled this way, put a \
         +1/+1 counter on target creature you control.",
        &CAULDRON_TAP,
        &CAULDRON_TARGETS,
        EffectDef::Sequence(&CAULDRON_EXILES_THEN_GROWS),
    ),
];

// WOE 242 — Agatha's Soul Cauldron
pub(in crate::card::sets) static AGATHAS_SOUL_CAULDRON: CardRecord = CardRecord::new_with_legacy_id(
    2251,
    "Agatha's Soul Cauldron",
    CardArt::new("019b51b0-e5c6-4208-922b-7736686dddcd", "Jason A. Engle"),
    CardSet::WildsOfEldraine,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&AGATHAS_SOUL_CAULDRON_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&AGATHAS_SOUL_CAULDRON];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
