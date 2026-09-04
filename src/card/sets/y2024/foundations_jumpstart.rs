//! Foundations Jumpstart cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCHOLAR_OF_COMBUSTION: CardRecord = CardRecord::new(
    crate::card::CardSet::FoundationsJumpstart,
    "Scholar of Combustion",
    "23660e44-8546-438d-a2c4-e1cef6e50855",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// J25 24 — Scythecat Cub
pub(in crate::card::sets) static SCYTHECAT_CUB: CardRecord = CardRecord::new(
    CardSet::FoundationsJumpstart,
    "Scythecat Cub",
    "b3dd3c7d-4685-4579-b483-14ddaaaddf5b",
    "Gabor Szikszai",
    // Two mana that turns a land drop into a counter and the second land of
    // the turn into all of them at once -- and trample, so what it grows
    // into does not stop at a blocker.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Cat"], 2, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Landfall \u{2014} Whenever a land you control enters, put a +1/+1 counter on target \
             creature you control. If this is the second time this ability has resolved this \
             turn, double the number of +1/+1 counters on that creature instead.",
            // A land arriving under your control, which is what landfall watches: a
            // land put onto the battlefield by a search counts exactly as one played
            // from hand does.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::IfElseCondition {
                // The count includes the resolution asking, so the second land of the turn
                // reads two. A third reads three and takes the other branch.
                condition: &TriggerConditionDef::SourceResolutionsThisTurn {
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
                // "Double the number of +1/+1 counters on that creature": what it has, not
                // what this ability put there, so a creature somebody else grew doubles
                // just as readily.
                then: &EffectDef::DoubleCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                },
                otherwise: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// J25 28 — Shardless Outlander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARDLESS_OUTLANDER: CardRecord = CardRecord::new(
    crate::card::CardSet::FoundationsJumpstart,
    "Shardless Outlander",
    "fccb51a4-cb78-4437-b9ab-cc77736af561",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// J25 37 — Plagon, Lord of the Beach
pub(in crate::card::sets) static PLAGON_LORD_OF_THE_BEACH: CardRecord = CardRecord::new(
    CardSet::FoundationsJumpstart,
    "Plagon, Lord of the Beach",
    "7f8a6bfe-6033-4f6b-ab45-6b553f8b51a1",
    "GOSSAN",
    // A 0/3 that pays for itself in a deck of walls and then turns them into
    // an offense: the numbers stay what they are, and only the combat
    // assignment reads the other one.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Starfish", "Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Plagon enters, draw a card for each creature you control with toughness \
                 greater than its power.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    // "Each creature you control with toughness greater than its power": the
                    // comparison is between one creature's own two numbers, which is what makes
                    // a board of defensive bodies into a handful of cards.
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ToughnessGreaterThanItsPower,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ),
            AbilityDef::activated_with_targets(
                "{W/U}: Target creature you control assigns combat damage equal to its toughness \
                 rather than its power this turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W/U}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
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
pub(in crate::card::sets) static IVORA_INSATIABLE_HEIR: CardRecord = CardRecord::new(
    CardSet::FoundationsJumpstart,
    "Ivora, Insatiable Heir",
    "2ba70366-b6ae-423a-a8d8-29d2b8afd939",
    "Canata Katana",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire", "Warrior"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "When Ivora enters and whenever it deals combat damage to a player, create a Blood token.",
                // One printed ability with two ways in, which is what "when it enters and
                // whenever it deals combat damage" says. Splitting it would make her two
                // triggered abilities where the card has one.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                ]),
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
        ]),
);

// J25 114 — Dark Confidant (reprint)
const DARK_CONFIDANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::DARK_CONFIDANT,
    "c74e9388-460d-4dbf-934e-f3ecb48af6e8",
    "Victor Adame Minguez",
);

// J25 212 — Inspiring Overseer (reprint)
const INSPIRING_OVERSEER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2022::streets_of_new_capenna::INSPIRING_OVERSEER,
    "be1c0c41-cd92-49b2-be07-0c44219bcb6a",
    "Irina Nordsol",
);

// J25 343 — Pestermite (reprint)
const PESTERMITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2007::lorwyn::PESTERMITE,
    "4c8b4f64-244c-4944-b23f-c383039d9767",
    "Christopher Moeller",
);

// J25 349 — Remand (reprint)
const REMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::REMAND,
    "36de9999-8d0a-4174-8e38-549bacdc128b",
    "Mark A. Nelson",
);

// J25 641 — Bushwhack (reprint)
const BUSHWHACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2022::the_brothers_war::BUSHWHACK,
    "f6b92766-1ab8-462d-bd45-ccd6f55cbe14",
    "Artur Nakhodkin",
);

// J25 684 — Llanowar Visionary (reprint)
const LLANOWAR_VISIONARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2020::core_set_2021::LLANOWAR_VISIONARY,
    "c2635b0c-c990-4cce-9ac4-97602a757cf0",
    "Cristi Balanescu",
);

// J25 753 — Guardian Idol (reprint)
const GUARDIAN_IDOL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2004::fifth_dawn::GUARDIAN_IDOL,
    "1537f377-64c3-4c3b-a276-28d8234c029b",
    "Igor Kieryluk",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SCHOLAR_OF_COMBUSTION,
    &SCYTHECAT_CUB,
    &SHARDLESS_OUTLANDER,
    &PLAGON_LORD_OF_THE_BEACH,
    &IVORA_INSATIABLE_HEIR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    DARK_CONFIDANT_REPRINT,
    INSPIRING_OVERSEER_REPRINT,
    PESTERMITE_REPRINT,
    REMAND_REPRINT,
    BUSHWHACK_REPRINT,
    LLANOWAR_VISIONARY_REPRINT,
    GUARDIAN_IDOL_REPRINT,
];
