//! Stronghold cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::TargetIndex;
use crate::card::CostQuantityDef;
use crate::card::sets::y2011::innistrad as catalog_isd;
use crate::card::sets::y2012::dark_ascension as catalog_dka;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType, CostDef,
    CounterKind, DamageEventMatcherDef, DamagePreventionDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, ManaColor, MillUntilDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetPredicateDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

static TRANSFER_SPIKE_COUNTER: AbilityDef = AbilityDef::activated_with_targets(
    "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1 counter on target creature.",
    &[
        CostDef::Mana(mana_cost!("{2}")),
        CostDef::RemoveCountersFromSource {
            kind: CounterKind::PlusOnePlusOne,
            amount: 1,
        },
    ],
    &[AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )],
    EffectDef::AddCounters {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
);

// STH 1 — Bandage
pub(in crate::card::sets) static BANDAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79ed559a-9a88-43d2-85d1-4bbec038f71e"),
    "Bandage",
    crate::card::CardArt::new("79ed559a-9a88-43d2-85d1-4bbec038f71e", "Rebecca Guay"),
    crate::card::CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent the next 1 damage that would be dealt to any target this turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// STH 2 — Calming Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALMING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("155fc239-c2eb-41cc-9a0a-2414da3c3d7b"),
    "Calming Licid",
    crate::card::CardArt::new(
        "155fc239-c2eb-41cc-9a0a-2414da3c3d7b",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 3 — Change of Heart
// Audit: unsupported — Resolving effects cannot apply a temporary attack restriction to a permanent.
pub(in crate::card::sets) static CHANGE_OF_HEART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afaf277e-b430-4c96-880c-ae654973478c"),
    "Change of Heart",
    crate::card::CardArt::new("afaf277e-b430-4c96-880c-ae654973478c", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 4 — Contemplation
pub(in crate::card::sets) static CONTEMPLATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("848672be-b1cd-40ef-a7ed-ce1620aebd2e"),
    "Contemplation",
    crate::card::CardArt::new("848672be-b1cd-40ef-a7ed-ce1620aebd2e", "Brom"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::triggered(
        "Whenever you cast a spell, you gain 1 life.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// STH 5 — Conviction
pub(in crate::card::sets) static CONVICTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("190332b3-6a1c-4c25-af61-d8923fc9a0c3"),
    "Conviction",
    crate::card::CardArt::new("190332b3-6a1c-4c25-af61-d8923fc9a0c3", "Paolo Parente"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(3),
                    ),
                },
            ),
            AbilityDef::activated(
                "{W}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// STH 6 — Hidden Retreat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_RETREAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5be86f1f-c85a-4396-aa16-a39fbf493c96"),
    "Hidden Retreat",
    crate::card::CardArt::new("5be86f1f-c85a-4396-aa16-a39fbf493c96", "Terese Nielsen"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 7 — Honor Guard
pub(in crate::card::sets) static HONOR_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35c44137-ddcc-42fa-baff-a0c3785b84ee"),
    "Honor Guard",
    crate::card::CardArt::new("35c44137-ddcc-42fa-baff-a0c3785b84ee", "Joel Biske"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// STH 8 — Lancers en-Kor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LANCERS_EN_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7cd99c2-6d4a-48e8-8848-6fdc0788525d"),
    "Lancers en-Kor",
    crate::card::CardArt::new("e7cd99c2-6d4a-48e8-8848-6fdc0788525d", "Pete Venters"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 9 — Nomads en-Kor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOMADS_EN_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3a03c68-0ebe-488a-8e6c-7cbf7a448416"),
    "Nomads en-Kor",
    crate::card::CardArt::new("b3a03c68-0ebe-488a-8e6c-7cbf7a448416", "Val Mayerik"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 10 — Pursuit of Knowledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURSUIT_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eaa577b1-2604-4065-a973-166521682a86"),
    "Pursuit of Knowledge",
    crate::card::CardArt::new("eaa577b1-2604-4065-a973-166521682a86", "DiTerlizzi"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 11 — Rolling Stones
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROLLING_STONES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd27e9c9-0a47-4fa3-9da5-1993f20305c2"),
    "Rolling Stones",
    crate::card::CardArt::new("dd27e9c9-0a47-4fa3-9da5-1993f20305c2", "John Matson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 12 — Sacred Ground
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_GROUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37ae4b01-a9c1-4eec-9204-78cb2508e0df"),
    "Sacred Ground",
    crate::card::CardArt::new("37ae4b01-a9c1-4eec-9204-78cb2508e0df", "Terese Nielsen"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 13 — Samite Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b21aa2e-df36-4101-89b6-515858f2ab88"),
    "Samite Blessing",
    crate::card::CardArt::new("3b21aa2e-df36-4101-89b6-515858f2ab88", "Rebecca Guay"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 14 — Scapegoat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCAPEGOAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20454d36-d98d-4421-b3aa-d2dd4b368d84"),
    "Scapegoat",
    crate::card::CardArt::new("20454d36-d98d-4421-b3aa-d2dd4b368d84", "Daren Bader"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 15 — Shaman en-Kor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAMAN_EN_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9a13c4c-d5c0-4947-bb68-d2e9611bcdea"),
    "Shaman en-Kor",
    crate::card::CardArt::new("a9a13c4c-d5c0-4947-bb68-d2e9611bcdea", "Jeff Miracola"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 16 — Skyshroud Falcon
pub(in crate::card::sets) static SKYSHROUD_FALCON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afb86621-1a6e-4d56-ab0e-531105775c56"),
    "Skyshroud Falcon",
    crate::card::CardArt::new("afb86621-1a6e-4d56-ab0e-531105775c56", "Mike Raabe"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// STH 17 — Smite
// Audit: unsupported — Target predicates cannot require an attacking creature to currently be blocked.
pub(in crate::card::sets) static SMITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14f165ad-cfe6-4a5d-8073-a70969494855"),
    "Smite",
    crate::card::CardArt::new("14f165ad-cfe6-4a5d-8073-a70969494855", "Daren Bader"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 18 — Soltari Champion
pub(in crate::card::sets) static SOLTARI_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a112edf3-c976-426c-a407-e86255586e41"),
    "Soltari Champion",
    crate::card::CardArt::new("a112edf3-c976-426c-a407-e86255586e41", "Adam Rex"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Soltari", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::shadow(),
            AbilityDef::triggered(
                "Whenever this creature attacks, other creatures you control get +1/+1 until end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// STH 19 — Spirit en-Kor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_EN_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8adc368-ede3-4b1c-af93-a5e0f2e24d83"),
    "Spirit en-Kor",
    crate::card::CardArt::new("f8adc368-ede3-4b1c-af93-a5e0f2e24d83", "John Matson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 20 — Temper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e4bff910-24ab-45a0-b014-30bef6263c38"),
    "Temper",
    crate::card::CardArt::new("e4bff910-24ab-45a0-b014-30bef6263c38", "Matthew D. Wilson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 21 — Venerable Monk
pub(in crate::card::sets) static VENERABLE_MONK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72322032-c287-4a9e-9d61-a452f6c45bfb"),
    "Venerable Monk",
    crate::card::CardArt::new("704b8be3-4ed8-4e94-aa66-c7187a299088", "Terese Nielsen"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Monk", "Cleric"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// STH 22 — Wall of Essence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_ESSENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f237426-a657-4234-9f79-0a06558eeb39"),
    "Wall of Essence",
    crate::card::CardArt::new("2f237426-a657-4234-9f79-0a06558eeb39", "Adam Rex"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 23 — Warrior en-Kor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARRIOR_EN_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16a9ca33-3e9b-4ed9-b503-a1c09c17ca8b"),
    "Warrior en-Kor",
    crate::card::CardArt::new("16a9ca33-3e9b-4ed9-b503-a1c09c17ca8b", "Stephen Daniele"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 24 — Warrior Angel
pub(in crate::card::sets) static WARRIOR_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81d2c6e3-e556-4cc6-94b6-fdbc62d6853e"),
    "Warrior Angel",
    crate::card::CardArt::new("81d2c6e3-e556-4cc6-94b6-fdbc62d6853e", "Brom"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Angel", "Warrior"], 3, 4).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals damage, you gain that much life.",
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
        ],
    ),
);

// STH 25 — Youthful Knight
pub(in crate::card::sets) static YOUTHFUL_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcd3b7a7-c5e8-4179-ade3-31625620f3a9"),
    "Youthful Knight",
    crate::card::CardArt::new("fcd3b7a7-c5e8-4179-ade3-31625620f3a9", "Rebecca Guay"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 1)
        .with_ability(abilities::first_strike()),
);

// STH 26 — Cloud Spirit
pub(in crate::card::sets) static CLOUD_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc7547aa-fcf7-4b6e-955d-cc5ebc40cd7d"),
    "Cloud Spirit",
    crate::card::CardArt::new("938d6c51-903b-4e0b-8702-291666581f2a", "Randy Gallegos"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spirit"], 3, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// STH 27 — Contempt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTEMPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1fa8e60-c32f-4586-b133-224f0aec3355"),
    "Contempt",
    crate::card::CardArt::new("c1fa8e60-c32f-4586-b133-224f0aec3355", "Val Mayerik"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 28 — Dream Halls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_HALLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff4a22d9-007b-4eb7-af9e-b5c2cae36238"),
    "Dream Halls",
    crate::card::CardArt::new("ff4a22d9-007b-4eb7-af9e-b5c2cae36238", "Matthew D. Wilson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 29 — Dream Prowler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_PROWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1444f76b-c876-4166-b957-b17313221fea"),
    "Dream Prowler",
    crate::card::CardArt::new(
        "1444f76b-c876-4166-b957-b17313221fea",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 30 — Evacuation
pub(in crate::card::sets) static EVACUATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1cb8ae53-a53f-4a0f-94f7-559aca041797"),
    "Evacuation",
    crate::card::CardArt::new("1cb8ae53-a53f-4a0f-94f7-559aca041797", "Rob Alexander"),
    crate::card::CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::spell(
        "Return all creatures to their owners' hands.",
        EffectDef::MoveToZone {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// STH 31 — Gliding Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLIDING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0148f3ca-4dba-4408-86d0-1190c387ee69"),
    "Gliding Licid",
    crate::card::CardArt::new("0148f3ca-4dba-4408-86d0-1190c387ee69", "Heather Hudson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 32 — Hammerhead Shark
pub(in crate::card::sets) static HAMMERHEAD_SHARK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("854627ab-38bd-4894-94d8-9ef51a57579c"),
    "Hammerhead Shark",
    CardArt::new("854627ab-38bd-4894-94d8-9ef51a57579c", "Stephen Daniele"),
    CardSet::Stronghold,
    // A 2/3 for two that blocks anything early and attacks only in the
    // blue mirror.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Shark"], 2, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            // Read at declaration, and off the defending player rather
            // than the controller: it is their Islands that let it swim.
            EffectDef::CannotAttackUnless(
                &const {
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )
                },
            ),
        ),
    ),
);

// STH 33 — Hesitation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HESITATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3ce5dbc-3597-42e8-859a-cac105c8102e"),
    "Hesitation",
    crate::card::CardArt::new("e3ce5dbc-3597-42e8-859a-cac105c8102e", "Pete Venters"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 34 — Intruder Alarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTRUDER_ALARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e55f653-0952-4713-8d43-ca50acff9e3b"),
    "Intruder Alarm",
    crate::card::CardArt::new("7e55f653-0952-4713-8d43-ca50acff9e3b", "Donato Giancola"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 35 — Leap
pub(in crate::card::sets) static LEAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0841423d-7a27-4bc5-9ac2-0ba47648c6f1"),
    "Leap",
    crate::card::CardArt::new("0841423d-7a27-4bc5-9ac2-0ba47648c6f1", "Kev Walker"),
    crate::card::CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// STH 36 — Mana Leak
pub(in crate::card::sets) static MANA_LEAK: CardRecord = CardRecord::new_with_legacy_id(
    272,
    "Mana Leak",
    CardArt::new("abcaf16d-aa02-43e2-aa38-bb1835d47a05", "Christopher Rush"),
    CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {3}.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(ValueDef::Constant(3)),
    )),
);

// STH 37 — Mask of the Mimic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASK_OF_THE_MIMIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09891c87-eb74-4174-ad46-11a7f79de859"),
    "Mask of the Mimic",
    crate::card::CardArt::new("09891c87-eb74-4174-ad46-11a7f79de859", "Heather Hudson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 38 — Mind Games
pub(in crate::card::sets) static MIND_GAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4da50979-1f5d-48d1-9406-dfc785273c04"),
    "Mind Games",
    crate::card::CardArt::new("4da50979-1f5d-48d1-9406-dfc785273c04", "Andrew Robinson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        abilities::buyback(mana_cost!("{2}{U}")),
        AbilityDef::spell_with_targets(
            "Tap target artifact, creature, or land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// STH 39 — Ransack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANSACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b438802b-629a-42c8-824d-f081a1619f68"),
    "Ransack",
    crate::card::CardArt::new("b438802b-629a-42c8-824d-f081a1619f68", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 40 — Rebound
pub(in crate::card::sets) static REBOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb6ca66e-1116-4739-8375-87af99e9bba5"),
    "Rebound",
    crate::card::CardArt::new("bb6ca66e-1116-4739-8375-87af99e9bba5", "Doug Chaffee"),
    crate::card::CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Change the target of target spell that targets only a player. The new target must be a player.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::DeclaredTargetCount {
                        minimum: 1,
                        maximum: 1,
                    },
                    ObjectPredicateDef::HasDeclaredTarget(
                        crate::card::TargetPredicate::Player,
                    ),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
            object: EffectRecipientDef::Target(crate::TargetIndex::PRIMARY),
            chooser: PlayerRefDef::EffectController,
            change: crate::card::StackTargetChangeDef::ChooseNew {
                optional: false,
                restriction: Some(crate::card::TargetPredicate::Player),
            },
        }),
    )),
);

// STH 41 — Reins of Power
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REINS_OF_POWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21a9242f-0516-4dae-8cbc-1f7f78931782"),
    "Reins of Power",
    crate::card::CardArt::new("21a9242f-0516-4dae-8cbc-1f7f78931782", "Colin MacNeil"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 42 — Sift
pub(in crate::card::sets) static SIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf5b23da-9ba8-4b43-a5c9-51e1fc253913"),
    "Sift",
    crate::card::CardArt::new("bf5b23da-9ba8-4b43-a5c9-51e1fc253913", "Pete Venters"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Draw three cards, then discard a card.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// STH 43 — Silver Wyvern
pub(in crate::card::sets) static SILVER_WYVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02a20067-4ac2-4688-b8e8-3463185c4a41"),
    "Silver Wyvern",
    crate::card::CardArt::new("02a20067-4ac2-4688-b8e8-3463185c4a41", "Colin MacNeil"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Drake"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{U}: Change the target of target spell or ability that targets only this creature. The new target must be a creature.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::TargetsObjectMatching(
                            &ObjectPredicateDef::Source,
                        ),
                        ObjectPredicateDef::DeclaredTargetCount {
                            minimum: 1,
                            maximum: 1,
                        },
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                object: EffectRecipientDef::Target(crate::TargetIndex::PRIMARY),
                chooser: PlayerRefDef::EffectController,
                change: crate::card::StackTargetChangeDef::ChooseNew {
                    optional: false,
                    restriction: Some(crate::card::TargetPredicate::CreaturePermanent),
                },
            }),
        ),
    ]),
);

// STH 44 — Spindrift Drake
pub(in crate::card::sets) static SPINDRIFT_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c881c388-430f-4a4b-9cd1-fba0a2bc2cd6"),
    "Spindrift Drake",
    crate::card::CardArt::new("c881c388-430f-4a4b-9cd1-fba0a2bc2cd6", "Anthony S. Waters"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{U}"), &["Drake"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{U}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// STH 45 — Thalakos Deceiver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THALAKOS_DECEIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae4cc2f5-3a50-40c8-87ea-6b92f081f55c"),
    "Thalakos Deceiver",
    crate::card::CardArt::new("ae4cc2f5-3a50-40c8-87ea-6b92f081f55c", "Andrew Robinson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 46 — Tidal Surge
pub(in crate::card::sets) static TIDAL_SURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a027c31d-c662-4ce1-a0d1-a32e62f6a724"),
    "Tidal Surge",
    crate::card::CardArt::new("8737440b-0bf0-483f-895b-aa24da2b9cfe", "Doug Chaffee"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to three target creatures without flying.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            3,
        )],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// STH 47 — Tidal Warrior
pub(in crate::card::sets) static TIDAL_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab89a30a-ac4d-4a5e-bbff-a02366bc9cf6"),
    "Tidal Warrior",
    crate::card::CardArt::new("ab89a30a-ac4d-4a5e-bbff-a02366bc9cf6", "Daren Bader"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target land becomes an Island until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Island]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// STH 48 — Volrath's Shapeshifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_S_SHAPESHIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec89ee13-34ef-41c8-8fe4-0a6b21df2032"),
    "Volrath's Shapeshifter",
    crate::card::CardArt::new("ec89ee13-34ef-41c8-8fe4-0a6b21df2032", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 49 — Walking Dream
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_DREAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd2f906d-b127-400a-809b-449f1e7f647b"),
    "Walking Dream",
    crate::card::CardArt::new(
        "bd2f906d-b127-400a-809b-449f1e7f647b",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 50 — Wall of Tears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_TEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4638d815-fb86-490c-84d6-777a1c6131d6"),
    "Wall of Tears",
    crate::card::CardArt::new("4638d815-fb86-490c-84d6-777a1c6131d6", "Rebecca Guay"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 51 — Bottomless Pit
pub(in crate::card::sets) static BOTTOMLESS_PIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91f05fc3-da6e-45d4-8566-f4e7bdce1fe5"),
    "Bottomless Pit",
    crate::card::CardArt::new("91f05fc3-da6e-45d4-8566-f4e7bdce1fe5", "Kev Walker"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::triggered(
        "At the beginning of each player's upkeep, that player discards a card at random.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::Discard {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(1),
            selection: DiscardSelectionDef::Random,
            then: None,
        },
    )),
);

// STH 52 — Brush with Death
pub(in crate::card::sets) static BRUSH_WITH_DEATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04013bc6-7774-400b-a6af-de755b4c324d"),
    "Brush with Death",
    crate::card::CardArt::new("04013bc6-7774-400b-a6af-de755b4c324d", "Stephen Daniele"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::buyback(mana_cost!("{2}{B}{B}")),
        AbilityDef::spell_with_targets(
            "Target opponent loses 2 life. You gain 2 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// STH 53 — Cannibalize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANNIBALIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0896ae6-fe96-44e8-ba62-a6d3fc1aae4f"),
    "Cannibalize",
    crate::card::CardArt::new("c0896ae6-fe96-44e8-ba62-a6d3fc1aae4f", "Robert Bliss"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 54 — Corrupting Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPTING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c12c6548-20e6-4697-ba26-e711590f9e28"),
    "Corrupting Licid",
    crate::card::CardArt::new("c12c6548-20e6-4697-ba26-e711590f9e28", "Thomas M. Baxa"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 55 — Crovax the Cursed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROVAX_THE_CURSED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88a9b76d-6ce1-40d4-bc04-7ae6237f5eaf"),
    "Crovax the Cursed",
    crate::card::CardArt::new("88a9b76d-6ce1-40d4-bc04-7ae6237f5eaf", "Pete Venters"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 56 — Dauthi Trapper
pub(in crate::card::sets) static DAUTHI_TRAPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d83770e-16ff-49c6-b4e7-eb7fc566eef8"),
    "Dauthi Trapper",
    crate::card::CardArt::new("6d83770e-16ff-49c6-b4e7-eb7fc566eef8", "Thomas M. Baxa"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Dauthi", "Minion"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains shadow until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::shadow()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// STH 57 — Death Stroke
pub(in crate::card::sets) static DEATH_STROKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7478a471-3bd2-4038-a4eb-70c38a43afa9"),
    "Death Stroke",
    crate::card::CardArt::new("7478a471-3bd2-4038-a4eb-70c38a43afa9", "Colin MacNeil"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target tapped creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Tapped,
        ])),
    )),
);

// STH 58 — Dungeon Shade
pub(in crate::card::sets) static DUNGEON_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a818483-de47-454d-bb24-3410aa95289c"),
    "Dungeon Shade",
    crate::card::CardArt::new(
        "6a818483-de47-454d-bb24-3410aa95289c",
        "Jason Alexander Behnke",
    ),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade", "Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 59 — Foul Imp
pub(in crate::card::sets) static FOUL_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e35ab62e-d6c6-448c-bbe0-42abe5780adc"),
    "Foul Imp",
    crate::card::CardArt::new("e35ab62e-d6c6-448c-bbe0-42abe5780adc", "Jim Nelson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Imp"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you lose 2 life.",
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// STH 60 — Grave Pact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVE_PACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("940536be-fce1-4a90-9126-195815a43e0f"),
    "Grave Pact",
    crate::card::CardArt::new("940536be-fce1-4a90-9126-195815a43e0f", "Scott Kirschner"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 61 — Lab Rats
pub(in crate::card::sets) static LAB_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3132c128-e0bd-4524-9526-914b3c7181fc"),
    "Lab Rats",
    crate::card::CardArt::new("3132c128-e0bd-4524-9526-914b3c7181fc", "DiTerlizzi"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        abilities::buyback(mana_cost!("{4}")),
        AbilityDef::spell(
            "Create a 1/1 black Rat creature token.",
            EffectDef::create_creature_token(&["Rat"], &[ManaColor::Black], 1, 1),
        ),
    ]),
);

// STH 62 — Megrim
pub(in crate::card::sets) static MEGRIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4eacd05b-078a-468c-b137-a802346d348a"),
    "Megrim",
    crate::card::CardArt::new("4eacd05b-078a-468c-b137-a802346d348a", "Donato Giancola"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent discards a card, this enchantment deals 2 damage to that player.",
        TriggerEventDef::Discarded(PlayerRelation::Opponent),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(2),
        },
    )),
);

// STH 63 — Mind Peel
pub(in crate::card::sets) static MIND_PEEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92bd531c-fdd1-4a22-816e-258b2975c1a3"),
    "Mind Peel",
    crate::card::CardArt::new("92bd531c-fdd1-4a22-816e-258b2975c1a3", "Adam Rex"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        abilities::buyback(mana_cost!("{2}{B}{B}")),
        AbilityDef::spell_with_targets(
            "Target player discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// STH 64 — Mindwarper
pub(in crate::card::sets) static MINDWARPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebe796e1-97f9-469a-a6b6-a09161058e12"),
    "Mindwarper",
    crate::card::CardArt::new("ebe796e1-97f9-469a-a6b6-a09161058e12", "Paolo Parente"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Spirit"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}{B}, Remove a +1/+1 counter from this creature: Target player discards a card. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{B}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// STH 65 — Morgue Thrull
pub(in crate::card::sets) static MORGUE_THRULL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("270a26ce-3435-4fef-bc34-abbc6f1cf3f4"),
    "Morgue Thrull",
    crate::card::CardArt::new("270a26ce-3435-4fef-bc34-abbc6f1cf3f4", "Robert Bliss"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Thrull"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: Mill three cards.",
            &[CostDef::SacrificeSource],
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// STH 66 — Mortuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("860f46ea-6fd0-456d-971d-8d1eccc7fa60"),
    "Mortuary",
    crate::card::CardArt::new("860f46ea-6fd0-456d-971d-8d1eccc7fa60", "Robert Bliss"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 67 — Rabid Rats
pub(in crate::card::sets) static RABID_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed8c028e-6656-47ef-97fb-de99f8833d5f"),
    "Rabid Rats",
    crate::card::CardArt::new("ed8c028e-6656-47ef-97fb-de99f8833d5f", "Matthew D. Wilson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target blocking creature gets -1/-1 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Blocking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// STH 68 — Revenant
static REVENANT_CREATURE_CARDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

pub(in crate::card::sets) static REVENANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74d11d10-90c7-4deb-bf7c-2208a0fcc19f"),
    "Revenant",
    crate::card::CardArt::new("0da40601-b6a3-47ca-b5b6-8fdbdf81f3d4", "Terese Nielsen"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Spirit"], 0, 0).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Revenant's power and toughness are each equal to the number of creature cards in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&REVENANT_CREATURE_CARDS),
                    ValueDef::CountMatchingObjects(&REVENANT_CREATURE_CARDS),
                ),
            },
        ),
    ]),
);

// STH 69 — Serpent Warrior
pub(in crate::card::sets) static SERPENT_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c364fd06-64c5-45f6-8ed5-64f44a1e8bda"),
    "Serpent Warrior",
    crate::card::CardArt::new("ab726e7d-171f-48b2-9652-545e17913330", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Snake", "Warrior"], 3, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you lose 3 life.",
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// STH 70 — Skeleton Scavengers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKELETON_SCAVENGERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57af1b3f-661a-4b02-be08-0ec721d7cab8"),
    "Skeleton Scavengers",
    crate::card::CardArt::new("57af1b3f-661a-4b02-be08-0ec721d7cab8", "Brian Snõddy"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 71 — Stronghold Assassin
pub(in crate::card::sets) static STRONGHOLD_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc0f043c-efb3-4392-ae25-b3ec180b0cb2"),
    "Stronghold Assassin",
    crate::card::CardArt::new("cc0f043c-efb3-4392-ae25-b3ec180b0cb2", "Matthew D. Wilson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(
        mana_cost!("{1}{B}{B}"),
        &["Phyrexian", "Zombie", "Assassin"],
        2,
        1,
    )
    .with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice a creature: Destroy target nonblack creature.",
        &[
            CostDef::TapSource,
            CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// STH 72 — Stronghold Taskmaster
pub(in crate::card::sets) static STRONGHOLD_TASKMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6171c210-01b5-45a9-9dd3-dbf96a33a750"),
    "Stronghold Taskmaster",
    crate::card::CardArt::new("6171c210-01b5-45a9-9dd3-dbf96a33a750", "Brom"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Giant", "Minion"], 4, 3).with_ability(
        AbilityDef::static_ability(
            "Other black creatures get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ),
);

// STH 73 — Torment
pub(in crate::card::sets) static TORMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe74a415-ea8f-4f16-8889-ae649f1483b2"),
    "Torment",
    crate::card::CardArt::new("fe74a415-ea8f-4f16-8889-ae649f1483b2", "Paolo Parente"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -3/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-3),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// STH 74 — Tortured Existence
pub(in crate::card::sets) static TORTURED_EXISTENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1754b92b-d6f9-4503-af01-dee03f72a048"),
    "Tortured Existence",
    crate::card::CardArt::new("1754b92b-d6f9-4503-af01-dee03f72a048", "Keith Parkinson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, Discard a creature card: Return target creature card from your graveyard to your hand.",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::DiscardCardMatching(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// STH 75 — Wall of Souls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_SOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bca2c333-405f-4c1b-a02a-f1fadb3e1d29"),
    "Wall of Souls",
    crate::card::CardArt::new("bca2c333-405f-4c1b-a02a-f1fadb3e1d29", "John Matson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 76 — Amok
pub(in crate::card::sets) static AMOK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c5bdb8c-2a2e-47cf-a502-2d62f9ada3fa"),
    "Amok",
    crate::card::CardArt::new("5c5bdb8c-2a2e-47cf-a502-2d62f9ada3fa", "Dermot Power"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "{1}, Discard a card at random: Put a +1/+1 counter on target creature.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::DiscardCardsAtRandom(1),
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// STH 77 — Convulsing Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONVULSING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5e50a1d-b9f5-4a03-a1c2-ca45ace53a52"),
    "Convulsing Licid",
    crate::card::CardArt::new("d5e50a1d-b9f5-4a03-a1c2-ca45ace53a52", "Scott Kirschner"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 78 — Craven Giant
pub(in crate::card::sets) static CRAVEN_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a2e1c12-f848-43b4-9505-851c66a509f1"),
    "Craven Giant",
    crate::card::CardArt::new("ea3cf964-88f6-4e62-97ce-cf0e179a53fb", "Brian Snõddy"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Giant"], 4, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ),
);

// STH 79 — Duct Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUCT_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90af852f-57a2-4a00-82dc-7c0f23899361"),
    "Duct Crawler",
    crate::card::CardArt::new("90af852f-57a2-4a00-82dc-7c0f23899361", "Stephen Daniele"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 80 — Fanning the Flames
pub(in crate::card::sets) static FANNING_THE_FLAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79075361-e6ee-4cc9-990b-88fef27bbb1c"),
    "Fanning the Flames",
    crate::card::CardArt::new("79075361-e6ee-4cc9-990b-88fef27bbb1c", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{X}{R}{R}")).with_abilities(&[
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::spell_with_targets(
            "Fanning the Flames deals X damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

// STH 81 — Flame Wave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_WAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e069d90a-e7d9-4967-a872-0dd8a0a9934a"),
    "Flame Wave",
    crate::card::CardArt::new("e069d90a-e7d9-4967-a872-0dd8a0a9934a", "Donato Giancola"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 82 — Fling (reprint)

// STH 83 — Flowstone Blade
pub(in crate::card::sets) static FLOWSTONE_BLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab9781fd-10c9-4790-8279-51c3cc6653cf"),
    "Flowstone Blade",
    crate::card::CardArt::new("ab9781fd-10c9-4790-8279-51c3cc6653cf", "Allen Williams"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/-1 until end of turn.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// STH 84 — Flowstone Hellion
pub(in crate::card::sets) static FLOWSTONE_HELLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("680ccbc7-aa97-4f01-9d26-0df184af3c3e"),
    "Flowstone Hellion",
    crate::card::CardArt::new("680ccbc7-aa97-4f01-9d26-0df184af3c3e", "Daren Bader"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Hellion", "Beast"], 3, 3).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated(
            "{0}: This creature gets +1/-1 until end of turn.",
            &[],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 85 — Flowstone Mauler
pub(in crate::card::sets) static FLOWSTONE_MAULER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3165251-6ac6-4294-8bca-595c362f4ceb"),
    "Flowstone Mauler",
    crate::card::CardArt::new("a3165251-6ac6-4294-8bca-595c362f4ceb", "Paolo Parente"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Beast"], 4, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "{R}: This creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 86 — Flowstone Shambler
pub(in crate::card::sets) static FLOWSTONE_SHAMBLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f2b70a5-db13-4c3f-829d-d4b9e0a16245"),
    "Flowstone Shambler",
    crate::card::CardArt::new("6f2b70a5-db13-4c3f-829d-d4b9e0a16245", "Jim Nelson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Beast"], 2, 2).with_ability(
        AbilityDef::activated(
            "{R}: This creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// STH 87 — Furnace Spirit
pub(in crate::card::sets) static FURNACE_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6a79dc7-ce46-41f7-9375-8d12afe6355a"),
    "Furnace Spirit",
    crate::card::CardArt::new("b6a79dc7-ce46-41f7-9375-8d12afe6355a", "Jeff Miracola"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 88 — Heat of Battle
// Audit: unsupported — Needs an event for any creature becoming a blocker. TriggerEventDef::Blocks and BecomesBlockedBy both require the source to be the creature in combat, and BlocksOrBecomesBlockedBy fires once per orientation with the other side as the triggering object, so neither can name an arbitrary blocker's controller.
pub(in crate::card::sets) static HEAT_OF_BATTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dbb98db-f2ee-446f-9170-dd05b1a7dbd8"),
    "Heat of Battle",
    crate::card::CardArt::new("8dbb98db-f2ee-446f-9170-dd05b1a7dbd8", "Matthew D. Wilson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 89 — Invasion Plans
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVASION_PLANS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f52c2f83-9535-4ee5-9964-5cc01e617981"),
    "Invasion Plans",
    crate::card::CardArt::new("f52c2f83-9535-4ee5-9964-5cc01e617981", "Pete Venters"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 90 — Mob Justice
pub(in crate::card::sets) static MOB_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b790d789-bb21-4119-a0e3-43af9bef8acc"),
    "Mob Justice",
    crate::card::CardArt::new("b790d789-bb21-4119-a0e3-43af9bef8acc", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Mob Justice deals damage to target player or planeswalker equal to the number of creatures you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// STH 91 — Mogg Bombers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_BOMBERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16064ad4-eec2-4c32-b66d-a2bdb1a6191f"),
    "Mogg Bombers",
    crate::card::CardArt::new("16064ad4-eec2-4c32-b66d-a2bdb1a6191f", "Dermot Power"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 92 — Mogg Flunkies
pub(in crate::card::sets) static MOGG_FLUNKIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b1b384e-a7d5-4b5f-87d5-95ac1a6c6320"),
    "Mogg Flunkies",
    CardArt::new("1b1b384e-a7d5-4b5f-87d5-95ac1a6c6320", "Brom"),
    CardSet::Stronghold,
    // Three power for two mana, rented against ever attacking or blocking
    // on its own, which a Goblin deck rarely wants to do anyway.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack or block alone.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotAttackAlone),
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotBlockAlone),
                ]),
            },
        ),
    ),
);

// STH 93 — Mogg Infestation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_INFESTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a91aa6f-cb2f-4aad-9415-bba4eb9b76ca"),
    "Mogg Infestation",
    crate::card::CardArt::new("5a91aa6f-cb2f-4aad-9415-bba4eb9b76ca", "Pete Venters"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 94 — Mogg Maniac
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_MANIAC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ddd33c58-f25d-4117-a266-75a91e9ddc75"),
    "Mogg Maniac",
    crate::card::CardArt::new("ddd33c58-f25d-4117-a266-75a91e9ddc75", "Brian Snõddy"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 95 — Ruination
pub(in crate::card::sets) static RUINATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d45eea06-806b-4163-ac86-de6ed6d1a91e"),
    "Ruination",
    crate::card::CardArt::new("d45eea06-806b-4163-ac86-de6ed6d1a91e", "Dermot Power"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::spell(
        "Destroy all nonbasic lands.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            then: None,
        },
    )),
);

// STH 96 — Seething Anger
pub(in crate::card::sets) static SEETHING_ANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6f1edb8-fff4-4f84-944c-fa5a032f4fb1"),
    "Seething Anger",
    crate::card::CardArt::new("e6f1edb8-fff4-4f84-944c-fa5a032f4fb1", "Val Mayerik"),
    crate::card::CardSet::Stronghold,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::spell_with_targets(
            "Target creature gets +3/+0 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 97 — Shard Phoenix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHARD_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe6a6d56-e73f-4dc6-a7df-d105010e52ab"),
    "Shard Phoenix",
    crate::card::CardArt::new("fe6a6d56-e73f-4dc6-a7df-d105010e52ab", "Paolo Parente"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 98 — Shock (reprint)

// STH 99 — Spitting Hydra
pub(in crate::card::sets) static SPITTING_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38c25d65-16bd-4628-b6ec-9e5495818277"),
    "Spitting Hydra",
    crate::card::CardArt::new("38c25d65-16bd-4628-b6ec-9e5495818277", "Daren Bader"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Hydra"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with four +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 4,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}, Remove a +1/+1 counter from this creature: It deals 1 damage to target creature.",
            &[
                CostDef::Mana(mana_cost!("{1}{R}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// STH 100 — Wall of Razors
pub(in crate::card::sets) static WALL_OF_RAZORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bb37bcd-0bbd-4f3f-9623-803885750344"),
    "Wall of Razors",
    crate::card::CardArt::new("0bb37bcd-0bbd-4f3f-9623-803885750344", "Michael Sutfin"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Wall"], 4, 1)
        .with_abilities(&[abilities::defender(), abilities::first_strike()]),
);

// STH 101 — Awakening
pub(in crate::card::sets) static AWAKENING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59612c02-6028-4953-ac4d-aca94b0ce4b9"),
    "Awakening",
    crate::card::CardArt::new("59612c02-6028-4953-ac4d-aca94b0ce4b9", "Dan Frazier"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::triggered(
        "At the beginning of each upkeep, untap all creatures and lands.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::Untap {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
        },
    )),
);

// STH 102 — Burgeoning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURGEONING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa9af8e5-bc97-4704-ae5c-e3d2d5b72586"),
    "Burgeoning",
    crate::card::CardArt::new("fa9af8e5-bc97-4704-ae5c-e3d2d5b72586", "Randy Gallegos"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 103 — Carnassid
pub(in crate::card::sets) static CARNASSID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae10e7fe-ee51-4c39-86ec-503324d19f6c"),
    "Carnassid",
    crate::card::CardArt::new("ae10e7fe-ee51-4c39-86ec-503324d19f6c", "Brom"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Beast"], 5, 4).with_abilities(&[
        abilities::trample(),
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ]),
);

// STH 104 — Constant Mists
pub(in crate::card::sets) static CONSTANT_MISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97a8a5fe-0391-489b-9556-0a1bf7e1900d"),
    "Constant Mists",
    CardArt::new("97a8a5fe-0391-489b-9556-0a1bf7e1900d", "Dermot Power"),
    CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        abilities::buyback_with_additional_cost(
            "Buyback—Sacrifice a land. (You may sacrifice a land in addition to any other costs as you cast this spell. If you do, put this card into your hand as it resolves.)",
            &CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Land),
                CostQuantityDef::Fixed(1),
            ),
        ),
        AbilityDef::spell(
            "Prevent all combat damage that would be dealt this turn.",
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 105 — Crossbow Ambush
pub(in crate::card::sets) static CROSSBOW_AMBUSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("283146fd-2307-4019-b23d-7fb1893dc46c"),
    "Crossbow Ambush",
    crate::card::CardArt::new("283146fd-2307-4019-b23d-7fb1893dc46c", "Kev Walker"),
    crate::card::CardSet::Stronghold,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Creatures you control gain reach until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::reach()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// STH 106 — Elven Rite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVEN_RITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("daba8742-c246-4e64-8af1-8f4ebcdc4b5f"),
    "Elven Rite",
    crate::card::CardArt::new("daba8742-c246-4e64-8af1-8f4ebcdc4b5f", "Jeff Miracola"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 107 — Endangered Armodon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDANGERED_ARMODON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29d43e8f-a914-44ed-bbd3-3746fb4ea6da"),
    "Endangered Armodon",
    crate::card::CardArt::new("29d43e8f-a914-44ed-bbd3-3746fb4ea6da", "Kev Walker"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 108 — Hermit Druid
pub(in crate::card::sets) static HERMIT_DRUID: CardRecord = CardRecord::new_with_legacy_id(
    2070,
    "Hermit Druid",
    CardArt::new("a912f57d-9622-453d-826d-ef3d83644850", "Heather Hudson"),
    CardSet::Stronghold,
    // Printed as land smoothing. A deck with no basic lands at all reads the
    // same ability as "put your library into your graveyard", which is the
    // only reason anyone plays it.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 1, 1).with_ability(
        AbilityDef::activated(
            "{G}, {T}: Reveal cards from the top of your library until you reveal a basic land card. Put that card into your hand and all other cards revealed this way into your graveyard.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::TapSource,
            ],
            EffectDef::MillUntil(&MillUntilDef {
                player: EffectRecipientDef::Controller,
                // Basic lands only, which is why the Druid empties a library that holds
                // none: what it does not find, it passes over into the graveyard.
                until: ObjectSetPredicateDef::contains(
                    &ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                ),
                matched_zone: ZoneKind::Hand,
            }),
        ),
    ),
);

// STH 109 — Lowland Basilisk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOWLAND_BASILISK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("454d6938-b87a-46b1-99d8-74573544ac5b"),
    "Lowland Basilisk",
    crate::card::CardArt::new("454d6938-b87a-46b1-99d8-74573544ac5b", "Randy Gallegos"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 110 — Mulch (reprint)

// STH 111 — Overgrowth
pub(in crate::card::sets) static OVERGROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb9179f5-c3e0-4499-9cfb-6cb7e8329a59"),
    "Overgrowth",
    crate::card::CardArt::new("bb9179f5-c3e0-4499-9cfb-6cb7e8329a59", "Rob Alexander"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an additional {G}{G}.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Green)
                        .with_amount(2)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// STH 112 — Primal Rage
pub(in crate::card::sets) static PRIMAL_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e78026e4-edb8-4f55-bd80-4152f4dfb461"),
    "Primal Rage",
    crate::card::CardArt::new("e78026e4-edb8-4f55-bd80-4152f4dfb461", "Brian Snõddy"),
    crate::card::CardSet::Stronghold,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have trample.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::trample()),
        },
    )),
);

// STH 113 — Provoke
// Audit: unsupported — Needs provoke; see Deftblade Elite.
pub(in crate::card::sets) static PROVOKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa3c58f1-5276-420e-8c09-d256492ee87b"),
    "Provoke",
    crate::card::CardArt::new("fa3c58f1-5276-420e-8c09-d256492ee87b", "Terese Nielsen"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 114 — Skyshroud Archer
pub(in crate::card::sets) static SKYSHROUD_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("485aeabb-4a9b-4d88-80ad-83e31da6804b"),
    "Skyshroud Archer",
    crate::card::CardArt::new("485aeabb-4a9b-4d88-80ad-83e31da6804b", "Jeff Miracola"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Archer"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature with flying gets -1/-1 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// STH 115 — Skyshroud Troopers
pub(in crate::card::sets) static SKYSHROUD_TROOPERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5197937-023c-412c-bf2c-b8e811ca04e1"),
    "Skyshroud Troopers",
    crate::card::CardArt::new("d5197937-023c-412c-bf2c-b8e811ca04e1", "DiTerlizzi"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Druid", "Warrior"], 3, 3)
        .with_ability(abilities::tap_for(ManaColor::Green)),
);

// STH 116 — Spike Breeder
pub(in crate::card::sets) static SPIKE_BREEDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7e11ef7-18a9-4ab1-981e-b337b1488ebd"),
    "Spike Breeder",
    crate::card::CardArt::new("f7e11ef7-18a9-4ab1-981e-b337b1488ebd", "Adam Rex"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        TRANSFER_SPIKE_COUNTER,
        AbilityDef::activated(
            "{2}, Remove a +1/+1 counter from this creature: Create a 1/1 green Spike creature token.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            EffectDef::create_creature_token(&["Spike"], &[ManaColor::Green], 1, 1),
        ),
    ]),
);

// STH 117 — Spike Colony
pub(in crate::card::sets) static SPIKE_COLONY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6ea4703-8521-4576-8405-4b923e0a9522"),
    "Spike Colony",
    crate::card::CardArt::new("a6ea4703-8521-4576-8405-4b923e0a9522", "Douglas Shuler"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with four +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 4,
                },
            ),
        ),
        TRANSFER_SPIKE_COUNTER,
    ]),
);

// STH 118 — Spike Feeder
pub(in crate::card::sets) static SPIKE_FEEDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3751b2ae-a234-4691-984b-2f9f6b1cd1df"),
    "Spike Feeder",
    crate::card::CardArt::new("3751b2ae-a234-4691-984b-2f9f6b1cd1df", "Heather Hudson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with two +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        TRANSFER_SPIKE_COUNTER,
        AbilityDef::activated(
            "Remove a +1/+1 counter from this creature: You gain 2 life.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            }],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// STH 119 — Spike Soldier
pub(in crate::card::sets) static SPIKE_SOLDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa45664c-ac39-40f8-9f56-cf25ed60a84a"),
    "Spike Soldier",
    crate::card::CardArt::new("aa45664c-ac39-40f8-9f56-cf25ed60a84a", "Randy Elliott"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Spike", "Soldier"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        TRANSFER_SPIKE_COUNTER,
        AbilityDef::activated(
            "Remove a +1/+1 counter from this creature: This creature gets +2/+2 until end of turn.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// STH 120 — Spike Worker
pub(in crate::card::sets) static SPIKE_WORKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2713de6c-b754-435c-8d11-5215fd602a40"),
    "Spike Worker",
    crate::card::CardArt::new("2713de6c-b754-435c-8d11-5215fd602a40", "Daniel Gelon"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with two +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        TRANSFER_SPIKE_COUNTER,
    ]),
);

// STH 121 — Spined Wurm
pub(in crate::card::sets) static SPINED_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0053bd00-90fd-48c2-8f79-952d5d1e3e74"),
    "Spined Wurm",
    crate::card::CardArt::new("113fad70-36bc-4ab7-962a-cda3bddd02fc", "Keith Parkinson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Wurm"], 5, 4),
);

// STH 122 — Tempting Licid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPTING_LICID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da7f3e0b-0600-4451-b621-e40c902a16cb"),
    "Tempting Licid",
    crate::card::CardArt::new("da7f3e0b-0600-4451-b621-e40c902a16cb", "Randy Gallegos"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 123 — Verdant Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDANT_TOUCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7079bf55-4827-4b8b-a178-8c7b903d93b9"),
    "Verdant Touch",
    crate::card::CardArt::new(
        "7079bf55-4827-4b8b-a178-8c7b903d93b9",
        "M. W. Kaluta & DiTerlizzi",
    ),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 124 — Volrath's Gardens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_S_GARDENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d74f7d8-10eb-4082-9b07-41de13359b8c"),
    "Volrath's Gardens",
    crate::card::CardArt::new("7d74f7d8-10eb-4082-9b07-41de13359b8c", "Rob Alexander"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 125 — Wall of Blossoms
pub(in crate::card::sets) static WALL_OF_BLOSSOMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7eb4a1a3-efcf-4c9a-ad1f-0a3f8f2b456f"),
    "Wall of Blossoms",
    crate::card::CardArt::new("7eb4a1a3-efcf-4c9a-ad1f-0a3f8f2b456f", "Heather Hudson"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Wall"], 0, 4).with_abilities(&[
        abilities::defender(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// STH 126 — Acidic Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACIDIC_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d92c3f7-a589-4c87-aa17-0d9707605ff4"),
    "Acidic Sliver",
    crate::card::CardArt::new("2d92c3f7-a589-4c87-aa17-0d9707605ff4", "Jeff Miracola"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 127 — Crystalline Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTALLINE_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06551990-713c-4b8b-bebb-4e849babb5bb"),
    "Crystalline Sliver",
    crate::card::CardArt::new("06551990-713c-4b8b-bebb-4e849babb5bb", "Allen Williams"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 128 — Hibernation Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIBERNATION_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94934d64-6518-4c5e-90d9-a3bf23b8973f"),
    "Hibernation Sliver",
    crate::card::CardArt::new("94934d64-6518-4c5e-90d9-a3bf23b8973f", "Scott Kirschner"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 129 — Sliver Queen
pub(in crate::card::sets) static SLIVER_QUEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("235c0ece-aed0-4120-99cc-5d0e28fa70ab"),
    "Sliver Queen",
    crate::card::CardArt::new("235c0ece-aed0-4120-99cc-5d0e28fa70ab", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    CardRules::new_creature(mana_cost!("{W}{U}{B}{R}{G}"), &["Sliver"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{2}: Create a 1/1 colorless Sliver creature token.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::create_creature_token(&["Sliver"], &[], 1, 1),
        )),
);

// STH 130 — Spined Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINED_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a8a9442-7b08-4cc8-94ec-bddb8feab1a8"),
    "Spined Sliver",
    crate::card::CardArt::new("9a8a9442-7b08-4cc8-94ec-bddb8feab1a8", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 131 — Victual Sliver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VICTUAL_SLIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e58908e-7095-4fb9-b7cb-a67d12f33b8a"),
    "Victual Sliver",
    crate::card::CardArt::new("5e58908e-7095-4fb9-b7cb-a67d12f33b8a", "Terese Nielsen"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 132 — Bullwhip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BULLWHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff312dd7-456c-4ce5-9614-e2cbe3c2219c"),
    "Bullwhip",
    crate::card::CardArt::new("ff312dd7-456c-4ce5-9614-e2cbe3c2219c", "Brom"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 133 — Ensnaring Bridge
pub(in crate::card::sets) static ENSNARING_BRIDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27d838a1-2739-45f7-a856-6202334fa76a"),
    "Ensnaring Bridge",
    CardArt::new("27d838a1-2739-45f7-a856-6202334fa76a", "Pete Venters"),
    CardSet::Stronghold,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::static_ability(
        "Creatures with power greater than the number of cards in your hand can't attack.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::EachPlayer,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                AttackRestrictionDef::prohibit(
                    ObjectPredicateDef::PowerGreaterThan(ValueDef::CardsInHandAbove {
                        player: PlayerRelation::You,
                        threshold: 0,
                    }),
                    AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker,
                ),
            )),
        },
    )),
);

// STH 134 — Heartstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ed17325-6d2f-404e-b13f-d2d419d522b7"),
    "Heartstone",
    crate::card::CardArt::new("0ed17325-6d2f-404e-b13f-d2d419d522b7", "John Matson"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 135 — Horn of Greed
pub(in crate::card::sets) static HORN_OF_GREED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("950660f9-b185-4979-a08b-cce4ec6ce07d"),
    "Horn of Greed",
    crate::card::CardArt::new("950660f9-b185-4979-a08b-cce4ec6ce07d", "Jeff Miracola"),
    crate::card::CardSet::Stronghold,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::triggered(
        "Whenever a player plays a land, that player draws a card.",
        TriggerEventDef::LandPlayed {
            land: ObjectPredicateDef::Any,
            player: PlayerRelation::Any,
        },
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(1),
        },
    )),
);

// STH 136 — Hornet Cannon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORNET_CANNON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51e9afeb-4d27-441c-b379-3dfe974053b8"),
    "Hornet Cannon",
    crate::card::CardArt::new("51e9afeb-4d27-441c-b379-3dfe974053b8", "Ron Spencer"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 137 — Jinxed Ring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JINXED_RING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c675814-094c-45d4-93ee-0f50b4755e79"),
    "Jinxed Ring",
    crate::card::CardArt::new(
        "1c675814-094c-45d4-93ee-0f50b4755e79",
        "M. W. Kaluta & DiTerlizzi",
    ),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 138 — Mox Diamond
pub(in crate::card::sets) static MOX_DIAMOND: CardRecord = CardRecord::new_with_legacy_id(
    2052,
    "Mox Diamond",
    CardArt::new("28028830-83ed-45e2-b495-3b9ad9d3e988", "Dan Frazier"),
    CardSet::Stronghold,
    // Free mana that costs a land: the deck playing one is trading a card for
    // the turn it comes down.
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[
        AbilityDef::replacement(
            "If this artifact would enter, you may discard a land card instead. If you do, put this artifact onto the battlefield. If you don't, put it into its owner's graveyard.",
            ReplacementEffectDef::PayOr {
                payment: EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    // A land card from hand, which is the whole cost. A hand with none cannot
                    // pay at all, and the Mox goes straight to the graveyard.
                    cost: CostDef::DiscardMatching(ObjectPredicateDef::HasType(CardType::Land)),
                },
                // Paying changes nothing about the entry: the Mox arrives as it was
                // going to. Declining is what redirects it.
                if_paid: &[],
                if_declined: &[ReplacementEffectDef::MoveToZone(ZoneKind::Graveyard)],
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// STH 139 — Portcullis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PORTCULLIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4de4af2-fa75-4a87-b0e1-0117727917a5"),
    "Portcullis",
    crate::card::CardArt::new("a4de4af2-fa75-4a87-b0e1-0117727917a5", "Kev Walker"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 140 — Shifting Wall
pub(in crate::card::sets) static SHIFTING_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49b7a950-bf79-46fa-8b29-b7856b38e0fd"),
    "Shifting Wall",
    crate::card::CardArt::new("49b7a950-bf79-46fa-8b29-b7856b38e0fd", "Michael Sutfin"),
    crate::card::CardSet::Stronghold,
    CardRules::new_artifact_creature(mana_cost!("{X}"), &["Wall"], 0, 0).with_abilities(&[
        abilities::defender(),
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
    ]),
);

// STH 141 — Sword of the Chosen
pub(in crate::card::sets) static SWORD_OF_THE_CHOSEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("401678d7-11dc-47ec-be28-4facdd949bc1"),
    "Sword of the Chosen",
    crate::card::CardArt::new("401678d7-11dc-47ec-be28-4facdd949bc1", "Adam Rex"),
    crate::card::CardSet::Stronghold,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Target legendary creature gets +2/+2 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// STH 142 — Volrath's Laboratory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_S_LABORATORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23e9e60a-5718-460d-b031-b47d6f6715d1"),
    "Volrath's Laboratory",
    crate::card::CardArt::new("23e9e60a-5718-460d-b031-b47d6f6715d1", "Brom"),
    crate::card::CardSet::Stronghold,
    crate::card::CardRules::unsupported(),
);

// STH 143 — Volrath's Stronghold
pub(in crate::card::sets) static VOLRATH_S_STRONGHOLD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43bf015b-152e-4d67-b773-e75fb2487a32"),
    "Volrath's Stronghold",
    crate::card::CardArt::new("43bf015b-152e-4d67-b773-e75fb2487a32", "Kev Walker"),
    crate::card::CardSet::Stronghold,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated_with_targets(
                "{1}{B}, {T}: Put target creature card from your graveyard on top of your library.",
                &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BANDAGE,
    &CALMING_LICID,
    &CHANGE_OF_HEART,
    &CONTEMPLATION,
    &CONVICTION,
    &HIDDEN_RETREAT,
    &HONOR_GUARD,
    &LANCERS_EN_KOR,
    &NOMADS_EN_KOR,
    &PURSUIT_OF_KNOWLEDGE,
    &ROLLING_STONES,
    &SACRED_GROUND,
    &SAMITE_BLESSING,
    &SCAPEGOAT,
    &SHAMAN_EN_KOR,
    &SKYSHROUD_FALCON,
    &SMITE,
    &SOLTARI_CHAMPION,
    &SPIRIT_EN_KOR,
    &TEMPER,
    &VENERABLE_MONK,
    &WALL_OF_ESSENCE,
    &WARRIOR_EN_KOR,
    &WARRIOR_ANGEL,
    &YOUTHFUL_KNIGHT,
    &CLOUD_SPIRIT,
    &CONTEMPT,
    &DREAM_HALLS,
    &DREAM_PROWLER,
    &EVACUATION,
    &GLIDING_LICID,
    &HAMMERHEAD_SHARK,
    &HESITATION,
    &INTRUDER_ALARM,
    &LEAP,
    &MANA_LEAK,
    &MASK_OF_THE_MIMIC,
    &MIND_GAMES,
    &RANSACK,
    &REBOUND,
    &REINS_OF_POWER,
    &SIFT,
    &SILVER_WYVERN,
    &SPINDRIFT_DRAKE,
    &THALAKOS_DECEIVER,
    &TIDAL_SURGE,
    &TIDAL_WARRIOR,
    &VOLRATH_S_SHAPESHIFTER,
    &WALKING_DREAM,
    &WALL_OF_TEARS,
    &BOTTOMLESS_PIT,
    &BRUSH_WITH_DEATH,
    &CANNIBALIZE,
    &CORRUPTING_LICID,
    &CROVAX_THE_CURSED,
    &DAUTHI_TRAPPER,
    &DEATH_STROKE,
    &DUNGEON_SHADE,
    &FOUL_IMP,
    &GRAVE_PACT,
    &LAB_RATS,
    &MEGRIM,
    &MIND_PEEL,
    &MINDWARPER,
    &MORGUE_THRULL,
    &MORTUARY,
    &RABID_RATS,
    &REVENANT,
    &SERPENT_WARRIOR,
    &SKELETON_SCAVENGERS,
    &STRONGHOLD_ASSASSIN,
    &STRONGHOLD_TASKMASTER,
    &TORMENT,
    &TORTURED_EXISTENCE,
    &WALL_OF_SOULS,
    &AMOK,
    &CONVULSING_LICID,
    &CRAVEN_GIANT,
    &DUCT_CRAWLER,
    &FANNING_THE_FLAMES,
    &FLAME_WAVE,
    &FLOWSTONE_BLADE,
    &FLOWSTONE_HELLION,
    &FLOWSTONE_MAULER,
    &FLOWSTONE_SHAMBLER,
    &FURNACE_SPIRIT,
    &HEAT_OF_BATTLE,
    &INVASION_PLANS,
    &MOB_JUSTICE,
    &MOGG_BOMBERS,
    &MOGG_FLUNKIES,
    &MOGG_INFESTATION,
    &MOGG_MANIAC,
    &RUINATION,
    &SEETHING_ANGER,
    &SHARD_PHOENIX,
    &SPITTING_HYDRA,
    &WALL_OF_RAZORS,
    &AWAKENING,
    &BURGEONING,
    &CARNASSID,
    &CONSTANT_MISTS,
    &CROSSBOW_AMBUSH,
    &ELVEN_RITE,
    &ENDANGERED_ARMODON,
    &HERMIT_DRUID,
    &LOWLAND_BASILISK,
    &OVERGROWTH,
    &PRIMAL_RAGE,
    &PROVOKE,
    &SKYSHROUD_ARCHER,
    &SKYSHROUD_TROOPERS,
    &SPIKE_BREEDER,
    &SPIKE_COLONY,
    &SPIKE_FEEDER,
    &SPIKE_SOLDIER,
    &SPIKE_WORKER,
    &SPINED_WURM,
    &TEMPTING_LICID,
    &VERDANT_TOUCH,
    &VOLRATH_S_GARDENS,
    &WALL_OF_BLOSSOMS,
    &ACIDIC_SLIVER,
    &CRYSTALLINE_SLIVER,
    &HIBERNATION_SLIVER,
    &SLIVER_QUEEN,
    &SPINED_SLIVER,
    &VICTUAL_SLIVER,
    &BULLWHIP,
    &ENSNARING_BRIDGE,
    &HEARTSTONE,
    &HORN_OF_GREED,
    &HORNET_CANNON,
    &JINXED_RING,
    &MOX_DIAMOND,
    &PORTCULLIS,
    &SHIFTING_WALL,
    &SWORD_OF_THE_CHOSEN,
    &VOLRATH_S_LABORATORY,
    &VOLRATH_S_STRONGHOLD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_dka::FLING), // STH 82
    PrintingRecord::reprint(&catalog_m14::SHOCK), // STH 98
    PrintingRecord::reprint(&catalog_isd::MULCH), // STH 110
];
