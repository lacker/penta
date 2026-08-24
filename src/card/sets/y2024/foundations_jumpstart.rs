//! Foundations Jumpstart cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef,
    CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
    tokens,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// J25 19 — Scholar of Combustion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCHOLAR_OF_COMBUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23660e44-8546-438d-a2c4-e1cef6e50855"),
    "Scholar of Combustion",
    crate::card::CardArt::new("23660e44-8546-438d-a2c4-e1cef6e50855", "Nereida"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 24 — Scythecat Cub
/// A land arriving under your control, which is what landfall watches: a
/// land put onto the battlefield by a search counts exactly as one played
/// from hand does.
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static A_CREATURE_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

/// The count includes the resolution asking, so the second land of the turn
/// reads two. A third reads three and is not the second time, which is why
/// the two branches are one condition and its negation rather than a pair of
/// numbers.
static CUB_SECOND_TIME: TriggerConditionDef = TriggerConditionDef::SourceResolutionsThisTurn {
    comparison: ComparisonDef::Equal,
    amount: 2,
};

static CUB_NOT_SECOND_TIME: TriggerConditionDef = TriggerConditionDef::Not(&CUB_SECOND_TIME);

static CUB_GROWS_IT: EffectDef = EffectDef::AddCounters {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    kind: CounterKind::PlusOnePlusOne,
    amount: ValueDef::Constant(1),
};

/// "Double the number of +1/+1 counters on that creature": what it has, not
/// what this ability put there, so a creature somebody else grew doubles
/// just as readily.
static CUB_DOUBLES_IT: EffectDef = EffectDef::DoubleCounters {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    kind: CounterKind::PlusOnePlusOne,
};

static CUB_LANDFALL: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &CUB_NOT_SECOND_TIME,
        then: &CUB_GROWS_IT,
    },
    EffectDef::IfCondition {
        condition: &CUB_SECOND_TIME,
        then: &CUB_DOUBLES_IT,
    },
];

pub(in crate::card::sets) static SCYTHECAT_CUB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3dd3c7d-4685-4579-b483-14ddaaaddf5b"),
    "Scythecat Cub",
    CardArt::new("b3dd3c7d-4685-4579-b483-14ddaaaddf5b", "Gabor Szikszai"),
    CardSet::FoundationsJumpstart,
    // Two mana that turns a land drop into a counter and the second land of
    // the turn into all of them at once -- and trample, so what it grows
    // into does not stop at a blocker.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Cat"], 2, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Landfall \u{2014} Whenever a land you control enters, put a +1/+1 counter on target \
             creature you control. If this is the second time this ability has resolved this \
             turn, double the number of +1/+1 counters on that creature instead.",
            TriggerEventDef::zone_changed(A_LAND_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
            &A_CREATURE_YOU_CONTROL,
            EffectDef::Sequence(&CUB_LANDFALL),
        ),
    ]),
);

// J25 28 — Shardless Outlander
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHARDLESS_OUTLANDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fccb51a4-cb78-4437-b9ab-cc77736af561"),
    "Shardless Outlander",
    crate::card::CardArt::new("fccb51a4-cb78-4437-b9ab-cc77736af561", "Leon Tukker"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 37 — Plagon, Lord of the Beach
/// "Each creature you control with toughness greater than its power": the
/// comparison is between one creature's own two numbers, which is what makes
/// a board of defensive bodies into a handful of cards.
static YOUR_DEFENSIVE_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ToughnessGreaterThanItsPower,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static PLAGON_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

static PLAGON_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{W/U}"))];

pub(in crate::card::sets) static PLAGON_LORD_OF_THE_BEACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f8a6bfe-6033-4f6b-ab45-6b553f8b51a1"),
    "Plagon, Lord of the Beach",
    CardArt::new("7f8a6bfe-6033-4f6b-ab45-6b553f8b51a1", "GOSSAN"),
    CardSet::FoundationsJumpstart,
    // A 0/3 that pays for itself in a deck of walls and then turns them into
    // an offense: the numbers stay what they are, and only the combat
    // assignment reads the other one.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Starfish", "Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "When Plagon enters, draw a card for each creature you control with toughness \
                 greater than its power.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&YOUR_DEFENSIVE_CREATURES),
                },
            ),
            AbilityDef::activated_with_targets(
                "{W/U}: Target creature you control assigns combat damage equal to its toughness \
                 rather than its power this turn.",
                &PLAGON_COST,
                &PLAGON_TARGET,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::AssignsCombatDamageEqualToToughness,
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// J25 50 — Ivora, Insatiable Heir
/// One printed ability with two ways in, which is what "when it enters and
/// whenever it deals combat damage" says. Splitting it would make her two
/// triggered abilities where the card has one.
static IVORA_MAKES_BLOOD: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
];

static IVORA_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::triggered(
        "When Ivora enters and whenever it deals combat damage to a player, create a Blood token.",
        TriggerEventDef::AnyOf(&IVORA_MAKES_BLOOD),
        EffectDef::create_token(tokens::blood()).with_art(CardArt::new(
            "6b563165-b97f-42c6-82a8-65d8ee69e381",
            "Stephen Andrade",
        )),
    ),
    // Any discard, including one paid as a cost -- which is how her own Blood
    // token feeds her.
    AbilityDef::triggered(
        "Whenever you discard a card, put a +1/+1 counter on Ivora.",
        TriggerEventDef::Discarded(PlayerRelation::You),
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
];

pub(in crate::card::sets) static IVORA_INSATIABLE_HEIR: CardRecord = CardRecord::new_with_legacy_id(
    2148,
    "Ivora, Insatiable Heir",
    CardArt::new("2ba70366-b6ae-423a-a8d8-29d2b8afd939", "Canata Katana"),
    CardSet::FoundationsJumpstart,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&IVORA_ABILITIES),
);

// J25 114 — Dark Confidant (reprint)

// J25 212 — Inspiring Overseer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRING_OVERSEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35d9da1d-8678-4252-b0f8-9960795642f0"),
    "Inspiring Overseer",
    crate::card::CardArt::new("be1c0c41-cd92-49b2-be07-0c44219bcb6a", "Irina Nordsol"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 343 — Pestermite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PESTERMITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f252ae53-443c-4a27-b8f0-639a9a2b8598"),
    "Pestermite",
    crate::card::CardArt::new(
        "4c8b4f64-244c-4944-b23f-c383039d9767",
        "Christopher Moeller",
    ),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 349 — Remand
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REMAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("581f3780-c480-48c6-b15c-1618f2feccb9"),
    "Remand",
    crate::card::CardArt::new("36de9999-8d0a-4174-8e38-549bacdc128b", "Mark A. Nelson"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 641 — Bushwhack
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BUSHWHACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("712a0640-d9c8-46fc-b38b-bf20a40fa902"),
    "Bushwhack",
    crate::card::CardArt::new("f6b92766-1ab8-462d-bd45-ccd6f55cbe14", "Artur Nakhodkin"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 684 — Llanowar Visionary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("880c9523-717e-4903-a09e-d6c47614383d"),
    "Llanowar Visionary",
    crate::card::CardArt::new("c2635b0c-c990-4cce-9ac4-97602a757cf0", "Cristi Balanescu"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

// J25 753 — Guardian Idol
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUARDIAN_IDOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6a62a73-b7db-47ec-9b68-65dd7c1a06a5"),
    "Guardian Idol",
    crate::card::CardArt::new("1537f377-64c3-4c3b-a276-28d8234c029b", "Igor Kieryluk"),
    crate::card::CardSet::FoundationsJumpstart,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SCHOLAR_OF_COMBUSTION,
    &SCYTHECAT_CUB,
    &SHARDLESS_OUTLANDER,
    &PLAGON_LORD_OF_THE_BEACH,
    &IVORA_INSATIABLE_HEIR,
    &INSPIRING_OVERSEER,
    &PESTERMITE,
    &REMAND,
    &BUSHWHACK,
    &LLANOWAR_VISIONARY,
    &GUARDIAN_IDOL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_rav::DARK_CONFIDANT), // J25 114
];
