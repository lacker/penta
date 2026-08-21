//! Bloomburrow cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    CardArt, CardRules, CardSet, CardType, ComparisonDef, CounterKind, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRelation, TriggerConditionDef,
    TriggerEventDef, ZoneKind, ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static AN_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

static AN_INSTANT_OR_SORCERY_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ]),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Object {
            object: AN_INSTANT_OR_SORCERY,
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        },
    )];

static MAKE_AN_OTTER: EffectDef =
    EffectDef::create_creature_token(&["Otter"], &[ManaColor::Blue, ManaColor::Red], 1, 1)
        .with_abilities(&[abilities::prowess()])
        .with_art(CardArt::new(
            "e6b2c465-c446-4dee-9101-763105dcf813",
            "Julia Griffin",
        ));

/// A Class is level 1 with no counters on it, so climbing to two takes one
/// counter and to three takes two. Each level is bought separately and only
/// at sorcery speed (CR 717.2b), and only from below it.
static BELOW_LEVEL_TWO: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Level,
    comparison: ComparisonDef::Less,
    amount: 1,
};

static BELOW_LEVEL_THREE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Level,
    comparison: ComparisonDef::Less,
    amount: 2,
};

/// The level-3 clause only functions once the Class is there. Written as an
/// intervening-if, which is checked when it would trigger and again as it
/// resolves -- so a Class knocked back down between the two does not make
/// the Otter.
static AT_LEVEL_THREE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Level,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static STORMCHASERS_TALENT_ABILITIES: [AbilityDef; 5] = [
    AbilityDef::triggered(
        "When this Class enters, create a 1/1 blue and red Otter creature token with prowess.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        MAKE_AN_OTTER,
    ),
    AbilityDef::activated(
        "{3}{U}: Level 2",
        &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
        EffectDef::GainClassLevel { level: 2 },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
    .with_activation_condition(&BELOW_LEVEL_TWO),
    AbilityDef::triggered_with_targets(
        "When this Class becomes level 2, return target instant or sorcery card from your \
         graveyard to your hand.",
        TriggerEventDef::BecomesLevel(2),
        &AN_INSTANT_OR_SORCERY_IN_YOUR_GRAVEYARD,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
        },
    ),
    AbilityDef::activated(
        "{5}{U}: Level 3",
        &[AbilityCostDef::Mana(mana_cost!("{5}{U}"))],
        EffectDef::GainClassLevel { level: 3 },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
    .with_activation_condition(&BELOW_LEVEL_THREE),
    AbilityDef::triggered_if(
        "Whenever you cast an instant or sorcery spell, create a 1/1 blue and red Otter creature \
         token with prowess.",
        TriggerEventDef::SpellCast(AN_INSTANT_OR_SORCERY_YOU_CAST),
        &AT_LEVEL_THREE,
        MAKE_AN_OTTER,
    ),
];

// BLB 75 — Stormchaser's Talent
pub(in crate::card::sets) static STORMCHASERS_TALENT: CardRecord = CardRecord::new(
    cards::STORMCHASERS_TALENT,
    "Stormchaser's Talent",
    CardArt::new("a36e682d-b43d-4e08-bf5b-70d7e924dbe5", "Christina Kraus"),
    CardSet::Bloomburrow,
    // One mana for a body, and a mana sink that buys back a spell and then
    // turns every cantrip afterwards into another creature.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Class"])
        .with_abilities(&STORMCHASERS_TALENT_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&STORMCHASERS_TALENT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
