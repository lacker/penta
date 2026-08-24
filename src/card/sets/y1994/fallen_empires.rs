use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardBehavior, CardRules, CardSet, CardType,
    ChoiceVisibilityDef, ComparisonDef, ControlDurationDef, CostModificationDef, CounterKind,
    DamageEventMatcherDef, DamagePreventionDef, DiscardSelectionDef, EffectDef, EffectPaymentDef,
    EffectRecipientDef, InstalledTriggerDef, LikelihoodDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, TopCardSelectionDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static REMOVE_THREE_SPORES: [AbilityCostDef; 1] = [AbilityCostDef::RemoveCountersFromSource {
    kind: CounterKind::Spore,
    amount: 3,
}];

static SHROUD: AbilityDef = abilities::shroud();

// FEM 1a — Combat Medic
pub(in crate::card::sets) static COMBAT_MEDIC: CardRecord = CardRecord::new_with_legacy_id(
    1444,
    "Combat Medic",
    CardArt::new(
        "9cfd96cb-03d6-4845-8595-50bf17b35726",
        "Edward P. Beard, Jr.",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric", "Soldier"], 0, 2)
        .with_ability(AbilityDef::activated_with_targets(
            "{1}{W}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// FEM 1b — Combat Medic (alternate printing)

// FEM 1c — Combat Medic (alternate printing)

// FEM 1d — Combat Medic (alternate printing)

// FEM 2 — Farrel's Mantle
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature attacks and isn't blocked, its controller may have it deal damage equal to its power plus 2 to another target creature. If that player does, the attacking…”.
pub(in crate::card::sets) static FARREL_S_MANTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af092da3-8713-4a59-86d3-827b942d6456"),
    "Farrel's Mantle",
    crate::card::CardArt::new("af092da3-8713-4a59-86d3-827b942d6456", "Anthony S. Waters"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 3a — Farrel's Zealot
static FARRELS_ZEALOT_STRIKE: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(3),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

static FARRELS_ZEALOT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

pub(in crate::card::sets) static FARRELS_ZEALOT: CardRecord = CardRecord::new_with_legacy_id(
    1720,
    "Farrel's Zealot",
    CardArt::new("3b3204be-33b9-41be-b952-081c1ba7e133", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may have it deal 3 damage to \
             target creature. If you do, this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &FARRELS_ZEALOT_TARGET,
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&FARRELS_ZEALOT_STRIKE),
            },
        ),
    ),
);

// FEM 3b — Farrel's Zealot (alternate printing)

// FEM 3c — Farrel's Zealot (alternate printing)

// FEM 4 — Farrelite Priest
// Audit: metadata-only — Needs the mana-ability runtime to pay this ability's mana activation cost for “{1}: Add {W}. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step”.
pub(in crate::card::sets) static FARRELITE_PRIEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e11bf79b-a951-4d0c-acdf-d8ba5290a648"),
    "Farrelite Priest",
    crate::card::CardArt::new("e11bf79b-a951-4d0c-acdf-d8ba5290a648", "Phil Foglio"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 5 — Hand of Justice
// Audit: metadata-only — Needs a persistent tap/untap restriction or event relation for “{T}, Tap three untapped white creatures you control: Destroy target creature”.
pub(in crate::card::sets) static HAND_OF_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a899b2d-825c-4929-a769-f4df70bf6a17"),
    "Hand of Justice",
    crate::card::CardArt::new("7a899b2d-825c-4929-a769-f4df70bf6a17", "Melissa A. Benson"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 6 — Heroism
// Audit: metadata-only — Needs a per-creature optional payment offered to the opposing controller, repeated for each attacking red creature; preventing one creature's combat damage is already expressible.
pub(in crate::card::sets) static HEROISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08ee87a0-a7eb-4472-9045-85d11e8a1501"),
    "Heroism",
    crate::card::CardArt::new("08ee87a0-a7eb-4472-9045-85d11e8a1501", "Mark Poole"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 7a — Icatian Infantry
pub(in crate::card::sets) static ICATIAN_INFANTRY: CardRecord = CardRecord::new_with_legacy_id(
    1779,
    "Icatian Infantry",
    CardArt::new(
        "f95d42d8-ba75-43bf-81b8-b02374f03e83",
        "Edward P. Beard, Jr.",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{1}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{1}: This creature gains banding until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::banding()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 7b — Icatian Infantry (alternate printing)

// FEM 7c — Icatian Infantry (alternate printing)

// FEM 7d — Icatian Infantry (alternate printing)

// FEM 8a — Icatian Javelineers
pub(in crate::card::sets) static ICATIAN_JAVELINEERS: CardRecord = CardRecord::new_with_legacy_id(
    77,
    "Icatian Javelineers",
    CardArt::new("f04b8356-2384-4743-80dd-f15ca7ec65f7", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a javelin counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Javelin,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Remove a javelin counter from this creature: It deals 1 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::Javelin,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FEM 8b — Icatian Javelineers (alternate printing)

// FEM 8c — Icatian Javelineers (alternate printing)

// FEM 9 — Icatian Lieutenant
pub(in crate::card::sets) static ICATIAN_LIEUTENANT: CardRecord = CardRecord::new_with_legacy_id(
    579,
    "Icatian Lieutenant",
    CardArt::new("39fec59a-4ade-4c6f-ae7d-911fbe6da26d", "Pete Venters"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Soldier"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}: Target Soldier creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Soldier"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 10a — Icatian Moneychanger
pub(in crate::card::sets) static ICATIAN_MONEYCHANGER: CardRecord = CardRecord::new_with_legacy_id(
    1587,
    "Icatian Moneychanger",
    CardArt::new("b3d502d4-4a96-47b3-ae26-8b2c9f36623d", "Drew Tucker"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human"], 0, 2).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three credit counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Credit,
                    amount: 3,
                },
            ),
        ),
        abilities::enters_trigger(
            "When this creature enters, it deals 3 damage to you.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a credit counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Credit,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Sacrifice this creature: You gain 1 life for each credit counter on this creature. \
             Activate only during your upkeep.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountersOnSource(CounterKind::Credit),
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ]),
);

// FEM 10b — Icatian Moneychanger (alternate printing)

// FEM 10c — Icatian Moneychanger (alternate printing)

// FEM 11 — Icatian Phalanx
pub(in crate::card::sets) static ICATIAN_PHALANX: CardRecord = CardRecord::new_with_legacy_id(
    1776,
    "Icatian Phalanx",
    CardArt::new("7bc02d30-3eef-4a48-8b11-b4f37219ab3a", "Kaja Foglio"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 2, 4)
        .with_abilities(&[abilities::banding()]),
);

// FEM 12 — Icatian Priest
pub(in crate::card::sets) static ICATIAN_PRIEST: CardRecord = CardRecord::new_with_legacy_id(
    580,
    "Icatian Priest",
    CardArt::new("d7690cdd-6610-4310-9e93-60dc4db2ae8d", "Drew Tucker"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}{W}: Target creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 13a — Icatian Scout
pub(in crate::card::sets) static ICATIAN_SCOUT: CardRecord = CardRecord::new_with_legacy_id(
    581,
    "Icatian Scout",
    CardArt::new(
        "86bf4aaa-a9b1-4798-a96b-c3e35afb77f7",
        "Richard Kane Ferguson",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier", "Scout"], 1, 1)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{1}, {T}: Target creature gains first strike until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// FEM 13b — Icatian Scout (alternate printing)

// FEM 13c — Icatian Scout (alternate printing)

// FEM 13d — Icatian Scout (alternate printing)

// FEM 14 — Icatian Skirmishers
pub(in crate::card::sets) static ICATIAN_SKIRMISHERS: CardRecord = CardRecord::new_with_legacy_id(
    1782,
    "Icatian Skirmishers",
    CardArt::new("15f6d115-c02d-45a3-aa6d-402964df47dd", "Heather Hudson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        abilities::first_strike(),
        abilities::banding(),
        AbilityDef::triggered(
            "Whenever this creature attacks, all creatures banded with it gain first strike \
             until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::BandedWithSource,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 15 — Icatian Town
pub(in crate::card::sets) static ICATIAN_TOWN: CardRecord = CardRecord::new_with_legacy_id(
    582,
    "Icatian Town",
    CardArt::new("cbb7c28d-0366-4d01-84a2-f1bc9f38aa4a", "Tom Wänerstrand"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{5}{W}")).with_abilities(&[AbilityDef::spell(
        "Create four 1/1 white Citizen creature tokens.",
        EffectDef::create_creature_token(&["Citizen"], &[ManaColor::White], 1, 1)
            .with_amount(4)
            .with_art(CardArt::new(
                "165164e7-5693-4d65-b789-8ed8a222365b",
                "Michael Phillippi",
            )),
    )]),
);

// FEM 16a — Order of Leitbur
pub(in crate::card::sets) static ORDER_OF_LEITBUR: CardRecord = CardRecord::new_with_legacy_id(
    85,
    "Order of Leitbur",
    CardArt::new("ebd6e51e-f042-4673-a898-291607105829", "Bryon Wackwitz"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Cleric", "Knight"], 2, 1)
        .with_abilities(&[
            abilities::protection_from_color(ManaColor::Black),
            AbilityDef::activated(
                "{W}: This creature gains first strike until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{W}{W}: This creature gets +1/+0 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}{W}"))],
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

// FEM 16b — Order of Leitbur (alternate printing)

// FEM 16c — Order of Leitbur (alternate printing)

// FEM 17 — Deep Spawn
static DEEP_SPAWN_SHROUD: AbilityDef = abilities::shroud();

/// One activation buys three things at once, and the untap prohibition is
/// what pays for the other two: shroud until end of turn, no untap next turn,
/// and the tap that puts the creature away in the first place.
static DEEP_SPAWN_HIDE: [EffectDef; 3] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&DEEP_SPAWN_SHROUD),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    // Until the upkeep after next, which outlives the untap step it has to
    // reach: an until-end-of-turn effect would be gone before the untap
    // happens at all.
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
        duration: ResolvedEffectDurationDef::UntilYourNextUpkeep,
    },
    EffectDef::Tap {
        object: EffectRecipientDef::Source,
    },
];

static DEEP_SPAWN_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Source,
};

pub(in crate::card::sets) static DEEP_SPAWN: CardRecord = CardRecord::new_with_legacy_id(
    1834,
    "Deep Spawn",
    CardArt::new("69c9e4a5-735f-471c-ab1a-6e6d50ba5724", "Mark Tedin"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{5}{U}{U}{U}"), &["Homarid"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you mill two cards.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef {
                payment: EffectPaymentDef::mill(PlayerSetDef::Related(PlayerRelation::You), 2),
                if_paid: None,
                otherwise: Some(&DEEP_SPAWN_SACRIFICE),
                visibility: ChoiceVisibilityDef::Public,
            }),
        ),
        AbilityDef::activated(
            "{U}: This creature gains shroud until end of turn and doesn't untap during your \
             next untap step. Tap this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Sequence(&DEEP_SPAWN_HIDE),
        ),
    ]),
);

// FEM 18a — High Tide
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “Until end of turn, whenever a player taps an Island for mana, that player adds an additional {U}”.
pub(in crate::card::sets) static HIGH_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4686bbb9-517f-4cce-aa7a-5db41e22c02b"),
    "High Tide",
    crate::card::CardArt::new("4686bbb9-517f-4cce-aa7a-5db41e22c02b", "Drew Tucker"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 18b — High Tide (alternate printing)

// FEM 18c — High Tide (alternate printing)

// FEM 19a — Homarid
static HOMARID_ONE_TIDE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Tide,
    comparison: ComparisonDef::Equal,
    amount: 1,
};

static HOMARID_THREE_TIDE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Tide,
    comparison: ComparisonDef::Equal,
    amount: 3,
};

static HOMARID_FOUR_TIDE: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Tide,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 4,
};

static HOMARID_SHRINK: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::modify_power_toughness(
        ValueDef::Constant(-1),
        ValueDef::Constant(-1),
    ),
};

static HOMARID_GROW: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

pub(in crate::card::sets) static HOMARID: CardRecord = CardRecord::new_with_legacy_id(
    1588,
    "Homarid",
    CardArt::new("d6ffeab4-83b1-4414-ae72-e59a2354ea15", "Quinton Hoover"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Homarid"], 2, 2).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a tide counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Tide,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a tide counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Tide,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "As long as there is exactly one tide counter on this creature, it gets -1/-1.",
            EffectDef::IfCondition {
                condition: &HOMARID_ONE_TIDE,
                then: &HOMARID_SHRINK,
            },
        ),
        AbilityDef::static_ability(
            "As long as there are exactly three tide counters on this creature, it gets +1/+1.",
            EffectDef::IfCondition {
                condition: &HOMARID_THREE_TIDE,
                then: &HOMARID_GROW,
            },
        ),
        AbilityDef::triggered_if(
            "Whenever there are four or more tide counters on this creature, remove all tide \
             counters from it.",
            TriggerEventDef::StateCondition,
            &HOMARID_FOUR_TIDE,
            EffectDef::RemoveAllCounters {
                object: EffectRecipientDef::Source,
                kind: Some(CounterKind::Tide),
            },
        ),
    ]),
);

// FEM 19b — Homarid (alternate printing)

// FEM 19c — Homarid (alternate printing)

// FEM 19d — Homarid (alternate printing)

// FEM 20 — Homarid Shaman
pub(in crate::card::sets) static HOMARID_SHAMAN: CardRecord = CardRecord::new_with_legacy_id(
    583,
    "Homarid Shaman",
    CardArt::new("c17c6416-86d6-46ea-aea1-41b98a66b250", "Amy Weber"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Homarid", "Shaman"], 2, 1).with_abilities(
        &[AbilityDef::activated_with_targets(
            "{U}: Tap target green creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )],
    ),
);

// FEM 21 — Homarid Spawning Bed
// Audit: metadata-only — Needs Camarid token creation whose count is the sacrificed creature's mana value.
pub(in crate::card::sets) static HOMARID_SPAWNING_BED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2cbb62fc-3cd9-41a6-804a-4ff9a766897f"),
    "Homarid Spawning Bed",
    crate::card::CardArt::new("2cbb62fc-3cd9-41a6-804a-4ff9a766897f", "Douglas Shuler"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 22a — Homarid Warrior
pub(in crate::card::sets) static HOMARID_WARRIOR: CardRecord = CardRecord::new_with_legacy_id(
    1658,
    "Homarid Warrior",
    CardArt::new("627ca588-917f-4768-a69d-3d93c1210390", "Daniel Gelon"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Homarid", "Warrior"], 3, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains shroud until end of turn and doesn't untap during your \
             next untap step. Tap it.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&SHROUD),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::Source,
                    count: 1,
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ),
);

// FEM 22b — Homarid Warrior (alternate printing)

// FEM 22c — Homarid Warrior (alternate printing)

// FEM 23a — Merseine (alternate printing)

// FEM 23b — Merseine (alternate printing)

// FEM 23c — Merseine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MERSEINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2dd197f8-ced0-461a-9672-2720a7b70803"),
    "Merseine",
    crate::card::CardArt::new("2dd197f8-ced0-461a-9672-2720a7b70803", "Drew Tucker"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 23d — Merseine (alternate printing)

// FEM 24 — River Merfolk
pub(in crate::card::sets) static RIVER_MERFOLK: CardRecord = CardRecord::new_with_legacy_id(
    584,
    "River Merfolk",
    CardArt::new("27d7fa54-4b89-4a9a-b088-4b89c525c1ea", "Douglas Shuler"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "{U}: This creature gains mountainwalk until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 25 — Seasinger
// Audit: metadata-only — Needs duration-aware control-changing continuous effects for “{T}: Gain control of target creature whose controller controls an Island for as long as you control this creature and this creature remains tapped”.
pub(in crate::card::sets) static SEASINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5266aa1-e2ea-46b9-91ab-b94a7bb7e9f9"),
    "Seasinger",
    crate::card::CardArt::new("c5266aa1-e2ea-46b9-91ab-b94a7bb7e9f9", "Amy Weber"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 26 — Svyelunite Priest
pub(in crate::card::sets) static SVYELUNITE_PRIEST: CardRecord = CardRecord::new_with_legacy_id(
    1459,
    "Svyelunite Priest",
    CardArt::new("316d25ae-7ac6-4f5b-93ab-0e0e28ec104b", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{U}{U}, {T}: Target creature gains shroud until end of turn. Activate only during \
             your upkeep.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&SHROUD),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// FEM 27a — Tidal Flats
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “{U}{U}: For each attacking creature without flying, its controller may pay {1}. If that player doesn't, creatures you control blocking that creature gain first strike until end of turn”.
pub(in crate::card::sets) static TIDAL_FLATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e820f3f-434e-4d09-91b9-0ebd6966b393"),
    "Tidal Flats",
    crate::card::CardArt::new("2e820f3f-434e-4d09-91b9-0ebd6966b393", "Rob Alexander"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 27b — Tidal Flats (alternate printing)

// FEM 27c — Tidal Flats (alternate printing)

// FEM 28 — Tidal Influence
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “As long as there are exactly three tide counters on this enchantment, all blue creatures get +2/+0”.
pub(in crate::card::sets) static TIDAL_INFLUENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2192c7b-ef6f-4ff6-9017-b1a125340517"),
    "Tidal Influence",
    crate::card::CardArt::new("b2192c7b-ef6f-4ff6-9017-b1a125340517", "Tom Wänerstrand"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 29 — Vodalian Knights
static DEFENDER_CONTROLS_AN_ISLAND: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

static YOU_CONTROL_NO_ISLANDS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::Equal,
    amount: 0,
};

pub(in crate::card::sets) static VODALIAN_KNIGHTS: CardRecord = CardRecord::new_with_legacy_id(
    1403,
    "Vodalian Knights",
    CardArt::new("68d97e1b-2526-4740-b354-f158734d1f72", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Merfolk", "Knight"], 2, 2).with_abilities(
        &[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "This creature can't attack unless defending player controls an Island.",
                EffectDef::CannotAttackUnless(&DEFENDER_CONTROLS_AN_ISLAND),
            ),
            AbilityDef::triggered_if(
                "When you control no Islands, sacrifice this creature.",
                TriggerEventDef::StateCondition,
                &YOU_CONTROL_NO_ISLANDS,
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ),
        ],
    ),
);

// FEM 30a — Vodalian Mage
pub(in crate::card::sets) static VODALIAN_MAGE: CardRecord = CardRecord::new_with_legacy_id(
    585,
    "Vodalian Mage",
    CardArt::new("c107e82b-134a-4f2b-98c2-6537fae6a50d", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, {T}: Counter target spell unless its controller pays {1}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
        ),
    ]),
);

// FEM 30b — Vodalian Mage (alternate printing)

// FEM 30c — Vodalian Mage (alternate printing)

// FEM 31a — Vodalian Soldiers
pub(in crate::card::sets) static VODALIAN_SOLDIERS: CardRecord = CardRecord::new_with_legacy_id(
    586,
    "Vodalian Soldiers",
    CardArt::new("7eb50256-9113-4b03-bcef-9aea24be8493", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Soldier"], 1, 2),
);

// FEM 31b — Vodalian Soldiers (alternate printing)

// FEM 31c — Vodalian Soldiers (alternate printing)

// FEM 31d — Vodalian Soldiers (alternate printing)

// FEM 32 — Vodalian War Machine
// Audit: metadata-only — Needs the permanents tapped to pay this card's own costs recorded for the turn, for “When this creature dies, destroy all Merfolk tapped this turn to pay for its abilities”. Tapping another creature as a cost and attacking despite defender are both available.
pub(in crate::card::sets) static VODALIAN_WAR_MACHINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd962ff0-4aa6-453e-931e-bd36fc034273"),
    "Vodalian War Machine",
    crate::card::CardArt::new("cd962ff0-4aa6-453e-931e-bd36fc034273", "Amy Weber"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 33a — Armor Thrull
pub(in crate::card::sets) static ARMOR_THRULL: CardRecord = CardRecord::new_with_legacy_id(
    1585,
    "Armor Thrull",
    CardArt::new("a98384d1-8e7d-4c41-9f23-47bc2ae2ad6a", "Pete Venters"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Thrull"], 1, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Put a +1/+2 counter on target creature.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusTwo,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// FEM 33b — Armor Thrull (alternate printing)

// FEM 33c — Armor Thrull (alternate printing)

// FEM 33d — Armor Thrull (alternate printing)

// FEM 34a — Basal Thrull
pub(in crate::card::sets) static BASAL_THRULL: CardRecord = CardRecord::new_with_legacy_id(
    587,
    "Basal Thrull",
    CardArt::new("0c1d5d13-0160-48cb-8fac-dd86102569b4", "Kaja Foglio"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Thrull"], 1, 2).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}, Sacrifice this creature: Add {B}{B}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(2)),
        ),
    ]),
);

// FEM 34b — Basal Thrull (alternate printing)

// FEM 34c — Basal Thrull (alternate printing)

// FEM 34d — Basal Thrull (alternate printing)

// FEM 35 — Breeding Pit
pub(in crate::card::sets) static BREEDING_PIT: CardRecord = CardRecord::new_with_legacy_id(
    588,
    "Breeding Pit",
    CardArt::new("a0d7e85f-eba5-4fc5-9fc0-109109d368aa", "Anson Maddocks"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {B}{B}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{B}{B}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::triggered(
            "At the beginning of your end step, create a 0/1 black Thrull creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::create_creature_token(&["Thrull"], &[ManaColor::Black], 0, 1).with_art(
                CardArt::new("b9f3042b-784c-4006-9bf1-60a323e60c5c", "Véronique Meignaud"),
            ),
        ),
    ]),
);

// FEM 36 — Derelor
pub(in crate::card::sets) static DERELOR: CardRecord = CardRecord::new_with_legacy_id(
    1844,
    "Derelor",
    CardArt::new("9eb2b79f-f09a-49dc-8e0f-7d711ba78981", "Anson Maddocks"),
    CardSet::FallenEmpires,
    // The tax is coloured, which is the whole joke: a 4/4 for four that makes
    // every black spell after it harder to cast, including itself if a second
    // one is already out.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Thrull"], 4, 4).with_ability(
        AbilityDef::static_ability(
            "Black spells you cast cost {B} more to cast.",
            EffectDef::ModifyCost(CostModificationDef::SpellIncrease {
                spell: ObjectPredicateDef::Color(ManaColor::Black),
                caster: PlayerRelation::You,
                amount: mana_cost!("{B}"),
            }),
        ),
    ),
);

// FEM 37 — Ebon Praetor
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “Sacrifice a creature: Remove a -2/-2 counter from this creature. If the sacrificed creature was a Thrull, put a +1/+0 counter on this creature. Activate only during your upkeep and only…”.
pub(in crate::card::sets) static EBON_PRAETOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40451f7a-692a-422d-99d3-d93a4d9315e0"),
    "Ebon Praetor",
    crate::card::CardArt::new(
        "40451f7a-692a-422d-99d3-d93a4d9315e0",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 38a — Hymn to Tourach
pub(in crate::card::sets) static HYMN_TO_TOURACH: CardRecord = CardRecord::new_with_legacy_id(
    75,
    "Hymn to Tourach",
    CardArt::new("eb9273ea-9a41-42e3-8c9c-0d50b127a818", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player discards two cards at random.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::Random,
            then: None,
        },
    )]),
);

// FEM 38b — Hymn to Tourach (alternate printing)

// FEM 38c — Hymn to Tourach (alternate printing)

// FEM 38d — Hymn to Tourach (alternate printing)

// FEM 39a — Initiates of the Ebon Hand (alternate printing)

// FEM 39b — Initiates of the Ebon Hand
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INITIATES_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("03c7dc01-46d0-42be-a1a9-48f69c846d12"),
    "Initiates of the Ebon Hand",
    crate::card::CardArt::new("03c7dc01-46d0-42be-a1a9-48f69c846d12", "Liz Danforth"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 39c — Initiates of the Ebon Hand (alternate printing)

// FEM 40a — Mindstab Thrull
static MINDSTAB_THRULL_STRIKE: [EffectDef; 2] = [
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Opponent,
        amount: ValueDef::Constant(3),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
];

pub(in crate::card::sets) static MINDSTAB_THRULL: CardRecord = CardRecord::new_with_legacy_id(
    1579,
    "Mindstab Thrull",
    CardArt::new(
        "499a791f-ac4f-4a96-b59b-37043686a79a",
        "Richard Kane Ferguson",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Thrull"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do, \
             defending player discards three cards.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&MINDSTAB_THRULL_STRIKE),
            },
        ),
    ),
);

// FEM 40b — Mindstab Thrull (alternate printing)

// FEM 40c — Mindstab Thrull (alternate printing)

// FEM 41a — Necrite
static NECRITE_STRIKE: [EffectDef; 2] = [
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
    // "It can't be regenerated" is the destruction's own flag rather than a
    // separate prohibition: nothing else this turn is being denied a shield.
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: false,
    },
];

static NECRITE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

pub(in crate::card::sets) static NECRITE: CardRecord = CardRecord::new_with_legacy_id(
    1580,
    "Necrite",
    CardArt::new("311d752a-ce8a-44cb-8aeb-1ed66705eb09", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Thrull"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do, \
             destroy target creature defending player controls. It can't be regenerated.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &NECRITE_TARGET,
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&NECRITE_STRIKE),
            },
        ),
    ),
);

// FEM 41b — Necrite (alternate printing)

// FEM 41c — Necrite (alternate printing)

// FEM 42a — Order of the Ebon Hand
pub(in crate::card::sets) static ORDER_OF_THE_EBON_HAND: CardRecord =
    CardRecord::new_with_legacy_id(
        86,
        "Order of the Ebon Hand",
        CardArt::new("9e51f5d8-a7cc-4720-8af5-e002bcfd78a0", "Melissa A. Benson"),
        CardSet::FallenEmpires,
        CardRules::new_creature(mana_cost!("{B}{B}"), &["Cleric", "Knight"], 2, 1).with_abilities(
            &[
                abilities::protection_from_color(ManaColor::White),
                AbilityDef::activated(
                    "{B}: This creature gains first strike until end of turn.",
                    &[AbilityCostDef::Mana(mana_cost!("{B}"))],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
                AbilityDef::activated(
                    "{B}{B}: This creature gets +1/+0 until end of turn.",
                    &[AbilityCostDef::Mana(mana_cost!("{B}{B}"))],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ],
        ),
    );

// FEM 42b — Order of the Ebon Hand (alternate printing)

// FEM 42c — Order of the Ebon Hand (alternate printing)

// FEM 43 — Soul Exchange
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “As an additional cost to cast this spell, exile a creature you control”.
pub(in crate::card::sets) static SOUL_EXCHANGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f73597d-f453-4d37-b2ef-c54ef683a884"),
    "Soul Exchange",
    crate::card::CardArt::new("9f73597d-f453-4d37-b2ef-c54ef683a884", "Anthony S. Waters"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 44 — Thrull Champion
pub(in crate::card::sets) static THRULL_CHAMPION: CardRecord = CardRecord::new_with_legacy_id(
    1473,
    "Thrull Champion",
    CardArt::new("4d3cafdd-a03b-4b08-b9c1-c776f8450d3a", "Daniel Gelon"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Thrull"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Thrull creatures get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Thrull"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Gain control of target Thrull for as long as you control this creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Thrull"),
            )],
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
                controller: PlayerRefDef::EffectController,
            },
        ),
    ]),
);

// FEM 45 — Thrull Retainer
pub(in crate::card::sets) static THRULL_RETAINER: CardRecord = CardRecord::new_with_legacy_id(
    1425,
    "Thrull Retainer",
    CardArt::new("d800512b-1492-41d2-931d-57c625044454", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::activated(
                "Sacrifice this Aura: Regenerate enchanted creature.",
                &[AbilityCostDef::SacrificeSource],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// FEM 46 — Thrull Wizard
// Audit: metadata-only — Needs an unless-payment offering a choice between two mana costs. Reading a spell's color is available.
pub(in crate::card::sets) static THRULL_WIZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4e732fb-cbef-4fd8-b704-e4d513a6cf2d"),
    "Thrull Wizard",
    crate::card::CardArt::new("c4e732fb-cbef-4fd8-b704-e4d513a6cf2d", "Anson Maddocks"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 47 — Tourach's Chant
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “Whenever a player puts a Forest onto the battlefield, this enchantment deals 3 damage to that player unless they put a -1/-1 counter on a creature they control”.
pub(in crate::card::sets) static TOURACH_S_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06883fd2-eccd-47c6-8c34-10d95e923685"),
    "Tourach's Chant",
    crate::card::CardArt::new(
        "06883fd2-eccd-47c6-8c34-10d95e923685",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 48 — Tourach's Gate
// Audit: metadata-only — Needs the clause's conditional recipient set or dynamic modifier value for “Tap enchanted land: Attacking creatures you control get +2/-1 until end of turn. Activate only if enchanted land is untapped”.
pub(in crate::card::sets) static TOURACH_S_GATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d77f6401-a9fb-449c-b511-6fb837055bb4"),
    "Tourach's Gate",
    crate::card::CardArt::new("d77f6401-a9fb-449c-b511-6fb837055bb4", "Sandra Everingham"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 49a — Brassclaw Orcs
pub(in crate::card::sets) static BRASSCLAW_ORCS: CardRecord = CardRecord::new_with_legacy_id(
    1729,
    "Brassclaw Orcs",
    CardArt::new("146a0b1b-c92a-4d0d-a9c7-22037dc8bd21", "Rob Alexander"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc"], 3, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't block creatures with power 2 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(2)),
                )),
            },
        ),
    ),
);

// FEM 49b — Brassclaw Orcs (alternate printing)

// FEM 49c — Brassclaw Orcs (alternate printing)

// FEM 49d — Brassclaw Orcs (alternate printing)

// FEM 50 — Dwarven Armorer
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “{R}, {T}, Discard a card: Put a +0/+1 counter or a +1/+0 counter on target creature”.
pub(in crate::card::sets) static DWARVEN_ARMORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d50bf06-97ab-4874-a484-9289f41dc98e"),
    "Dwarven Armorer",
    crate::card::CardArt::new("1d50bf06-97ab-4874-a484-9289f41dc98e", "Bryon Wackwitz"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 51 — Dwarven Catapult
// Audit: metadata-only — Needs damage divided evenly with downward rounding across a dynamically counted opponent creature set.
pub(in crate::card::sets) static DWARVEN_CATAPULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c1c6932-638a-4df7-bf9b-8d921f7484d9"),
    "Dwarven Catapult",
    crate::card::CardArt::new("8c1c6932-638a-4df7-bf9b-8d921f7484d9", "Jeff A. Menges"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 52 — Dwarven Lieutenant
pub(in crate::card::sets) static DWARVEN_LIEUTENANT: CardRecord = CardRecord::new_with_legacy_id(
    591,
    "Dwarven Lieutenant",
    CardArt::new("ea9a38b1-4676-425a-b40d-4fb478966024", "Jeff A. Menges"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{R}{R}"), &["Dwarf", "Soldier"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{R}: Target Dwarf creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Dwarf"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 53a — Dwarven Soldier
pub(in crate::card::sets) static DWARVEN_SOLDIER: CardRecord = CardRecord::new_with_legacy_id(
    1796,
    "Dwarven Soldier",
    CardArt::new("6fe77608-0b33-43f5-83fb-ae993ca1bf7c", "Rob Alexander"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Soldier"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by one or more Orcs, this \
             creature gets +0/+2 until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Subtype("Orc"),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// FEM 53b — Dwarven Soldier (alternate printing)

// FEM 53c — Dwarven Soldier (alternate printing)

// FEM 54a — Goblin Chirurgeon
pub(in crate::card::sets) static GOBLIN_CHIRURGEON: CardRecord = CardRecord::new_with_legacy_id(
    1431,
    "Goblin Chirurgeon",
    CardArt::new("2b710c21-e9f5-4660-80f6-2104ec65f63f", "Daniel Gelon"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Shaman"], 0, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "Sacrifice a Goblin: Regenerate target creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Goblin"),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// FEM 54b — Goblin Chirurgeon (alternate printing)

// FEM 54c — Goblin Chirurgeon (alternate printing)

// FEM 55 — Goblin Flotilla
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “At the beginning of each combat, unless you pay {R}, whenever this creature blocks or becomes blocked by a creature this combat, that creature gains first strike until end of turn”.
pub(in crate::card::sets) static GOBLIN_FLOTILLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87024efe-4a74-49fe-a43a-480bed0a650a"),
    "Goblin Flotilla",
    crate::card::CardArt::new("87024efe-4a74-49fe-a43a-480bed0a650a", "Tom Wänerstrand"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 56a — Goblin Grenade
// Audit: custom — Needs migration to a declarative Goblin sacrifice additional cost, any target, and fixed damage effect.
pub(in crate::card::sets) static GOBLIN_GRENADE: CardRecord = CardRecord::new_with_legacy_id(
    26,
    "Goblin Grenade",
    CardArt::new("8837eaba-9602-4f63-9897-85583fcdcf51", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::custom_full(
            "As an additional cost to cast this spell, sacrifice a Goblin.\nGoblin Grenade deals 5 damage to any target.",
            CardBehavior::GoblinGrenade,
            "The additional cost, target selection, and damage are implemented by the legacy spell resolver.",
        ),
    ]),
);

// FEM 56b — Goblin Grenade (alternate printing)

// FEM 56c — Goblin Grenade (alternate printing)

// FEM 57 — Goblin Kites
/// Berserk's shape with a coin in it: pump now, and a delayed trigger that
/// remembers the same target and may take it away.
static GOBLIN_KITES_EFFECT: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::add_ability(&GOBLIN_KITES_FLYING),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "Flip a coin at the beginning of the next end step. If you lose the flip, sacrifice \
         that creature.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::Randomized {
            likelihood: LikelihoodDef::new(0.5),
            on_success: &EffectDef::None,
            on_failure: &GOBLIN_KITES_SACRIFICE,
        },
    ))),
];

static GOBLIN_KITES_FLYING: AbilityDef = abilities::flying();

static GOBLIN_KITES_SACRIFICE: EffectDef = EffectDef::Sacrifice {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
};

/// "Toughness 2 or less", said as a strict bound because that is the shape the
/// predicate takes.
static GOBLIN_KITES_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

pub(in crate::card::sets) static GOBLIN_KITES: CardRecord = CardRecord::new_with_legacy_id(
    1800,
    "Goblin Kites",
    CardArt::new("a0a27ac3-2273-469a-92ba-3f4a3d55de6f", "Anson Maddocks"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "{R}: Target creature you control with toughness 2 or less gains flying until end \
             of turn. Flip a coin at the beginning of the next end step. If you lose the \
             flip, sacrifice that creature.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            &GOBLIN_KITES_TARGET,
            EffectDef::Sequence(&GOBLIN_KITES_EFFECT),
        ),
    ),
);

// FEM 58a — Goblin War Drums
static GOBLIN_WAR_DRUMS_MENACE: AbilityDef = abilities::menace();

pub(in crate::card::sets) static GOBLIN_WAR_DRUMS: CardRecord = CardRecord::new_with_legacy_id(
    1801,
    "Goblin War Drums",
    CardArt::new("2a2c4e4b-e9a7-4180-927b-589514c21876", "Dan Frazier"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have menace.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&GOBLIN_WAR_DRUMS_MENACE),
        },
    )),
);

// FEM 58b — Goblin War Drums (alternate printing)

// FEM 58c — Goblin War Drums (alternate printing)

// FEM 58d — Goblin War Drums (alternate printing)

// FEM 59 — Goblin Warrens
// Audit: metadata-only — Needs an activated cost that selects and sacrifices two Goblins; only one chosen permanent can currently be sacrificed as a cost.
pub(in crate::card::sets) static GOBLIN_WARRENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbec4aa5-3319-43dc-8347-5633edbd7018"),
    "Goblin Warrens",
    crate::card::CardArt::new("bbec4aa5-3319-43dc-8347-5633edbd7018", "Dan Frazier"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 60 — Orcish Captain
/// A coin is an even chance, which is the whole of what "flip a coin" means
/// to the seeded randomiser.
const COIN_FLIP: LikelihoodDef = LikelihoodDef::new(0.5);

static ORCISH_CAPTAIN_WON: EffectDef = orcish_captain_pump(2, 0);

static ORCISH_CAPTAIN_LOST: EffectDef = orcish_captain_pump(0, -2);

const fn orcish_captain_pump(power: i32, toughness: i32) -> EffectDef {
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(power),
            ValueDef::Constant(toughness),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    }
}

pub(in crate::card::sets) static ORCISH_CAPTAIN: CardRecord = CardRecord::new_with_legacy_id(
    1482,
    "Orcish Captain",
    CardArt::new("e43cf61d-b4d6-4461-a228-47fd8b026d33", "Mark Tedin"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{R}"), &["Orc", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}: Flip a coin. If you win the flip, target Orc creature gets +2/+0 until end of \
             turn. If you lose the flip, it gets -0/-2 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Orc"),
                ]),
            )],
            EffectDef::Randomized {
                likelihood: COIN_FLIP,
                on_success: &ORCISH_CAPTAIN_WON,
                on_failure: &ORCISH_CAPTAIN_LOST,
            },
        ),
    ),
);

// FEM 61a — Orcish Spy
/// Nothing is taken and nothing moves: the whole effect is the looking, so
/// the selection takes zero of the three and puts them back where they were.
static ORCISH_SPY_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(3),
    object: None,
    minimum: 0,
    maximum: 0,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

static ORCISH_SPY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

pub(in crate::card::sets) static ORCISH_SPY: CardRecord = CardRecord::new_with_legacy_id(
    1725,
    "Orcish Spy",
    CardArt::new("a7d0ae40-0dd1-4230-a7aa-2c2f832159b7", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{R}"), &["Orc", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Look at the top three cards of target player's library.",
            &[AbilityCostDef::TapSource],
            &ORCISH_SPY_TARGET,
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                looker: EffectRecipientDef::Controller,
                selection: &ORCISH_SPY_LOOK,
            },
        ),
    ),
);

// FEM 61b — Orcish Spy (alternate printing)

// FEM 61c — Orcish Spy (alternate printing)

// FEM 62a — Orcish Veteran
/// The restriction is authored as the permission it leaves behind: anything
/// that is not both white and big enough.
static NOT_A_BIG_WHITE_CREATURE: ObjectPredicateDef =
    ObjectPredicateDef::Not(&ObjectPredicateDef::All(&[
        ObjectPredicateDef::Color(ManaColor::White),
        ObjectPredicateDef::PowerAtLeast(2),
    ]));

pub(in crate::card::sets) static ORCISH_VETERAN: CardRecord = CardRecord::new_with_legacy_id(
    1730,
    "Orcish Veteran",
    CardArt::new("af5d1e3e-2efa-4804-9c89-5c71e7b8f0cc", "Douglas Shuler"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block white creatures with power 2 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    NOT_A_BIG_WHITE_CREATURE,
                )),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 62b — Orcish Veteran (alternate printing)

// FEM 62c — Orcish Veteran (alternate printing)

// FEM 62d — Orcish Veteran (alternate printing)

// FEM 63 — Orgg
/// "Defending player controls an untapped creature with power 3 or greater."
static ORGG_DETERRENT: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::PowerAtLeast(3),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

pub(in crate::card::sets) static ORGG: CardRecord = CardRecord::new_with_legacy_id(
    1713,
    "Orgg",
    CardArt::new("5af19ab0-4bd0-4d5f-8d2e-507e4fe87c18", "Daniel Gelon"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Orgg"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature can't attack if defending player controls an untapped creature with \
             power 3 or greater.",
            EffectDef::CannotAttackIf(&ORGG_DETERRENT),
        ),
        AbilityDef::static_ability(
            "This creature can't block creatures with power 3 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                )),
            },
        ),
    ]),
);

// FEM 64 — Raiding Party
// Audit: metadata-only — Needs a persistent tap/untap restriction or event relation for “Sacrifice an Orc: Each player may tap any number of untapped white creatures they control. For each creature tapped this way, that player chooses up to two Plains. Then destroy all…”.
pub(in crate::card::sets) static RAIDING_PARTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("907a3396-706b-4ca2-9973-bca758986032"),
    "Raiding Party",
    crate::card::CardArt::new("907a3396-706b-4ca2-9973-bca758986032", "Quinton Hoover"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 65a — Elven Fortress
pub(in crate::card::sets) static ELVEN_FORTRESS: CardRecord = CardRecord::new_with_legacy_id(
    592,
    "Elven Fortress",
    CardArt::new("9387105d-46d0-4db0-8980-dd0fded15eef", "Pete Venters"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{G}: Target blocking creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 65b — Elven Fortress (alternate printing)

// FEM 65c — Elven Fortress (alternate printing)

// FEM 65d — Elven Fortress (alternate printing)

// FEM 66 — Elvish Farmer
pub(in crate::card::sets) static ELVISH_FARMER: CardRecord = CardRecord::new_with_legacy_id(
    1455,
    "Elvish Farmer",
    CardArt::new(
        "40a9710e-b2f8-4746-8640-d450f58a6e49",
        "Richard Kane Ferguson",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf"], 0, 2).with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a spore counter on this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Spore,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
                &REMOVE_THREE_SPORES,
                EffectDef::create_creature_token(
                    &["Saproling"],
                    &[ManaColor::Green],
                    1,
                    1,
                )
                .with_art(CardArt::new(
                    "248ade83-ac57-42d6-985c-1e4cc3639f36",
                    "Joseph Meehan",
                )),
            ),
            AbilityDef::activated(
                "Sacrifice a Saproling: You gain 2 life.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Saproling"),
                controller: PlayerRelation::You,
            }],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
    ]),
);

// FEM 67a — Elvish Hunter
pub(in crate::card::sets) static ELVISH_HUNTER: CardRecord = CardRecord::new_with_legacy_id(
    1669,
    "Elvish Hunter",
    CardArt::new("e00455ac-c7ce-4916-98ed-cca9354e3f22", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Archer"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{G}, {T}: Target creature doesn't untap during its controller's next untap step.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                count: 1,
            },
        ),
    ),
);

// FEM 67b — Elvish Hunter (alternate printing)

// FEM 67c — Elvish Hunter (alternate printing)

// FEM 68a — Elvish Scout
/// The two shields are one printed clause but two rules: prevention names a
/// source or a recipient, never both at once, so "to and dealt by it" is the
/// creature on each side in turn.
static ELVISH_SCOUT_RESCUE: [EffectDef; 3] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_to(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
        )),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
        )),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

static ELVISH_SCOUT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

pub(in crate::card::sets) static ELVISH_SCOUT: CardRecord = CardRecord::new_with_legacy_id(
    1737,
    "Elvish Scout",
    CardArt::new("5477e674-ea0e-400f-bfe3-38465f6a52cc", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}: Untap target attacking creature you control. Prevent all combat damage \
             that would be dealt to and dealt by it this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &ELVISH_SCOUT_TARGET,
            EffectDef::Sequence(&ELVISH_SCOUT_RESCUE),
        ),
    ),
);

// FEM 68b — Elvish Scout (alternate printing)

// FEM 68c — Elvish Scout (alternate printing)

// FEM 69 — Feral Thallid
pub(in crate::card::sets) static FERAL_THALLID: CardRecord = CardRecord::new_with_legacy_id(
    1414,
    "Feral Thallid",
    CardArt::new("e585241e-c647-456d-b3b1-3d48dd78c372", "Rob Alexander"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Fungus"], 6, 3).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Spore,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Regenerate this creature.",
            &REMOVE_THREE_SPORES,
            EffectDef::Regenerate {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// FEM 70 — Fungal Bloom
static FUNGUS_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Subtype("Fungus"),
)];

pub(in crate::card::sets) static FUNGAL_BLOOM: CardRecord = CardRecord::new_with_legacy_id(
    1416,
    "Fungal Bloom",
    CardArt::new("cf1a2cb2-9a6b-41f7-96f7-ec457c69c16c", "Daniel Gelon"),
    CardSet::FallenEmpires,
    CardRules::new_enchantment(mana_cost!("{G}{G}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}: Put a spore counter on target Fungus.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{G}"))],
            &FUNGUS_TARGET,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Spore,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FEM 71a — Night Soil
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “{1}, Exile two creature cards from a single graveyard: Create a 1/1 green Saproling creature token”.
pub(in crate::card::sets) static NIGHT_SOIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4cda6d18-d4b1-4b8a-a72e-f90115adf4c3"),
    "Night Soil",
    crate::card::CardArt::new("4cda6d18-d4b1-4b8a-a72e-f90115adf4c3", "Sandra Everingham"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 71b — Night Soil (alternate printing)

// FEM 71c — Night Soil (alternate printing)

// FEM 72a — Spore Cloud
/// Three clauses in printed order. The tap comes first so it reaches the
/// blockers while they are still blocking; the skip is separate from it,
/// because a creature already tapped still owes the untap step it misses.
static SPORE_CLOUD_EFFECT: [EffectDef; 3] = [
    EffectDef::Tap {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::Blocking,
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
    },
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    // Counted per permanent rather than expressed as a duration: each
    // creature sits out its own controller's next untap step, and the two
    // sides do not reach that step at the same time.
    EffectDef::SkipNextUntapSteps {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::AttackingOrBlocking,
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        count: 1,
    },
];

pub(in crate::card::sets) static SPORE_CLOUD: CardRecord = CardRecord::new_with_legacy_id(
    1842,
    "Spore Cloud",
    CardArt::new("1691a9f4-4ea7-440f-9bdc-4214ab3c90f0", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_instant(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell(
        "Tap all blocking creatures. Prevent all combat damage that would be dealt this turn. \
         Each attacking creature and each blocking creature doesn't untap during its \
         controller's next untap step.",
        EffectDef::Sequence(&SPORE_CLOUD_EFFECT),
    )),
);

// FEM 72b — Spore Cloud (alternate printing)

// FEM 72c — Spore Cloud (alternate printing)

// FEM 73 — Spore Flower
pub(in crate::card::sets) static SPORE_FLOWER: CardRecord = CardRecord::new_with_legacy_id(
    1415,
    "Spore Flower",
    CardArt::new("f9681dc0-d0fc-4d5b-a23c-63ec1cc8343d", "Margaret Organ-Kean"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Fungus"], 0, 1)
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a spore counter on this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Spore,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Prevent all combat damage that would be dealt this turn.",
                &REMOVE_THREE_SPORES,
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FEM 74a — Thallid
pub(in crate::card::sets) static THALLID: CardRecord = CardRecord::new_with_legacy_id(
    1412,
    "Thallid",
    CardArt::new("4caaf31b-86a9-485b-8da7-d5b526ed1233", "Edward P. Beard, Jr."),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{G}"), &["Fungus"], 1, 1)
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a spore counter on this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Spore,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
                &REMOVE_THREE_SPORES,
                EffectDef::create_creature_token(
                    &["Saproling"],
                    &[ManaColor::Green],
                    1,
                    1,
                )
                .with_art(CardArt::new(
                    "248ade83-ac57-42d6-985c-1e4cc3639f36",
                    "Joseph Meehan",
                )),
            ),
        ]),
);

// FEM 74b — Thallid (alternate printing)

// FEM 74c — Thallid (alternate printing)

// FEM 74d — Thallid (alternate printing)

// FEM 75 — Thallid Devourer
pub(in crate::card::sets) static THALLID_DEVOURER: CardRecord = CardRecord::new_with_legacy_id(
    1456,
    "Thallid Devourer",
    CardArt::new("aa533845-4c4b-4072-aa39-8e56ce7ec325", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Fungus"], 2, 2).with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a spore counter on this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Spore,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
                &REMOVE_THREE_SPORES,
                EffectDef::create_creature_token(
                    &["Saproling"],
                    &[ManaColor::Green],
                    1,
                    1,
                )
                .with_art(CardArt::new(
                    "248ade83-ac57-42d6-985c-1e4cc3639f36",
                    "Joseph Meehan",
                )),
            ),
            AbilityDef::activated(
                "Sacrifice a Saproling: This creature gets +1/+2 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Saproling"),
                controller: PlayerRelation::You,
            }],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(2)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
    ]),
);

// FEM 76 — Thelon's Chant
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “Whenever a player puts a Swamp onto the battlefield, this enchantment deals 3 damage to that player unless the player puts a -1/-1 counter on a creature they control”.
pub(in crate::card::sets) static THELON_S_CHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d970195-0a09-4cb4-a2c0-c16fcab5c859"),
    "Thelon's Chant",
    crate::card::CardArt::new("9d970195-0a09-4cb4-a2c0-c16fcab5c859", "Melissa A. Benson"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 77 — Thelon's Curse
// Audit: metadata-only — Needs a persistent tap/untap restriction or event relation for “At the beginning of each player's upkeep, that player may choose any number of tapped blue creatures they control and pay {U} for each creature chosen this way. If the player does, untap…”.
pub(in crate::card::sets) static THELON_S_CURSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b868846-cc3c-4756-a5dd-2335bb380567"),
    "Thelon's Curse",
    crate::card::CardArt::new("9b868846-cc3c-4756-a5dd-2335bb380567", "Pete Venters"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 78 — Thelonite Druid
static THELONITE_DRUID_ANIMATION: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_card_types(crate::card::CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(3)),
];

pub(in crate::card::sets) static THELONITE_DRUID: CardRecord = CardRecord::new_with_legacy_id(
    601,
    "Thelonite Druid",
    CardArt::new(
        "cd8772dd-513d-4dd0-a5db-5214dc8da4e0",
        "Margaret Organ-Kean",
    ),
    CardSet::FallenEmpires,
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Cleric", "Druid"],
        1,
        1,
    )
    .with_ability(AbilityDef::activated(
        "{1}{G}, {T}, Sacrifice a creature: Forests you control become 2/3 creatures until end of turn. They're still lands.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}{G}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]), &[ZoneKind::Battlefield], PlayerRelation::You),
            effect: AppliedEffectDef::Composite(&THELONITE_DRUID_ANIMATION),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// FEM 79 — Thelonite Monk
// Audit: metadata-only — Needs its permanent-duration target-land characteristic effect and green-creature sacrifice cost authored and tested.
pub(in crate::card::sets) static THELONITE_MONK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5400ff25-c70e-4095-a228-190601b86043"),
    "Thelonite Monk",
    crate::card::CardArt::new("5400ff25-c70e-4095-a228-190601b86043", "Bryon Wackwitz"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 80a — Thorn Thallid
pub(in crate::card::sets) static THORN_THALLID: CardRecord = CardRecord::new_with_legacy_id(
    1413,
    "Thorn Thallid",
    CardArt::new("16e61c00-3e94-4f6f-8515-65b430829e91", "Daniel Gelon"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Fungus"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Spore,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "Remove three spore counters from this creature: It deals 1 damage to any target.",
            &REMOVE_THREE_SPORES,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FEM 80b — Thorn Thallid (alternate printing)

// FEM 80c — Thorn Thallid (alternate printing)

// FEM 80d — Thorn Thallid (alternate printing)

// FEM 81 — Aeolipile
pub(in crate::card::sets) static AEOLIPILE: CardRecord = CardRecord::new_with_legacy_id(
    593,
    "Aeolipile",
    CardArt::new("a09030ee-415c-45af-bf08-7623197a314f", "Heather Hudson"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this artifact: It deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// FEM 82 — Balm of Restoration
// Audit: metadata-only — Needs modal activated abilities: modes are chosen only while casting a spell, so an activated ability has no mode selection to freeze. Both of its modes are available.
pub(in crate::card::sets) static BALM_OF_RESTORATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f95de4a-7fae-42bc-9660-39ea7685ca02"),
    "Balm of Restoration",
    crate::card::CardArt::new(
        "7f95de4a-7fae-42bc-9660-39ea7685ca02",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 83 — Conch Horn
// Audit: metadata-only — Needs ordered-library inspection, selection, and visibility handling for “{1}, {T}, Sacrifice this artifact: Draw two then put a card from your hand on top of your library”.
pub(in crate::card::sets) static CONCH_HORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("860a9ba3-e4c4-4af9-bdfe-1ada39289fd5"),
    "Conch Horn",
    crate::card::CardArt::new("860a9ba3-e4c4-4af9-bdfe-1ada39289fd5", "Phil Foglio"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 84 — Delif's Cone
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “{T}, Sacrifice this artifact: This turn, when target creature you control attacks and isn't blocked, you may gain life equal to its power. If you do, it assigns no combat damage this turn”.
pub(in crate::card::sets) static DELIF_S_CONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("262b8788-c5a0-4c8e-9d58-b769b1b0a2ff"),
    "Delif's Cone",
    crate::card::CardArt::new("262b8788-c5a0-4c8e-9d58-b769b1b0a2ff", "Mark Tedin"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 85 — Delif's Cube
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “{2}, {T}: This turn, when target creature you control attacks and isn't blocked, it assigns no combat damage this turn and you put a cube counter on this artifact”.
pub(in crate::card::sets) static DELIF_S_CUBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14749600-9eca-4122-b04f-30ddda091b74"),
    "Delif's Cube",
    crate::card::CardArt::new("14749600-9eca-4122-b04f-30ddda091b74", "Mark Tedin"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 86 — Draconian Cylix
pub(in crate::card::sets) static DRACONIAN_CYLIX: CardRecord = CardRecord::new_with_legacy_id(
    1808,
    "Draconian Cylix",
    CardArt::new(
        "a419c9e3-5615-44f9-9256-94a3022bb69f",
        "Edward P. Beard, Jr.",
    ),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}, Discard a card at random: Regenerate target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::DiscardCardsAtRandom(1),
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Regenerate {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// FEM 87 — Elven Lyre
pub(in crate::card::sets) static ELVEN_LYRE: CardRecord = CardRecord::new_with_legacy_id(
    594,
    "Elven Lyre",
    CardArt::new("c3a8cd72-04c0-46f7-a249-f1cecddfdc26", "Kaja Foglio"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this artifact: Target creature gets +2/+2 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 88 — Implements of Sacrifice
pub(in crate::card::sets) static IMPLEMENTS_OF_SACRIFICE: CardRecord =
    CardRecord::new_with_legacy_id(
        1638,
        "Implements of Sacrifice",
        CardArt::new(
            "aa5deb95-79a6-4398-b82a-c1df169550d9",
            "Margaret Organ-Kean",
        ),
        CardSet::FallenEmpires,
        CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_mana(
            "{1}, {T}, Sacrifice this artifact: Add two mana of any one color.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(2)),
        )),
    );

// FEM 89 — Ring of Renewal
pub(in crate::card::sets) static RING_OF_RENEWAL: CardRecord = CardRecord::new_with_legacy_id(
    595,
    "Ring of Renewal",
    CardArt::new("a532d38a-809b-4132-8690-be15fe23afab", "Douglas Shuler"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{5}")).with_abilities(&[AbilityDef::activated(
        "{5}, {T}: Discard a card at random, then draw two cards.",
        &[
            AbilityCostDef::Mana(mana_cost!("{5}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// FEM 90 — Spirit Shield
pub(in crate::card::sets) static SPIRIT_SHIELD: CardRecord = CardRecord::new_with_legacy_id(
    1666,
    "Spirit Shield",
    CardArt::new("213d6e0d-5ec9-441e-a38d-50ce44583e4b", "Scott Kirschner"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this artifact during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target creature gets +0/+2 for as long as this artifact remains \
             tapped.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::WhileSourceTapped,
            },
        ),
    ]),
);

// FEM 91 — Zelyon Sword
pub(in crate::card::sets) static ZELYON_SWORD: CardRecord = CardRecord::new_with_legacy_id(
    1667,
    "Zelyon Sword",
    CardArt::new("4137160b-5248-4fbd-8ae8-25e9afd8fb5c", "Scott Kirschner"),
    CardSet::FallenEmpires,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this artifact during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature gets +2/+0 for as long as this artifact remains \
             tapped.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::WhileSourceTapped,
            },
        ),
    ]),
);

// FEM 92 — Bottomless Vault
pub(in crate::card::sets) static BOTTOMLESS_VAULT: CardRecord = CardRecord::new_with_legacy_id(
    1983,
    "Bottomless Vault",
    CardArt::new("639ae988-d1d1-4ead-b0f8-47fc39eb64a0", "Pat Lewis"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::static_ability(
            "You may choose not to untap this land during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this land is tapped, put a storage counter on it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceIsTapped,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Storage,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {B} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::Storage),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// FEM 93 — Dwarven Hold
pub(in crate::card::sets) static DWARVEN_HOLD: CardRecord = CardRecord::new_with_legacy_id(
    1984,
    "Dwarven Hold",
    CardArt::new("a3142ded-ff62-4817-aa54-75a7ea4498a6", "Pat Lewis"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::static_ability(
            "You may choose not to untap this land during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this land is tapped, put a storage counter on it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceIsTapped,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Storage,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {R} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::Storage),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// FEM 94 — Dwarven Ruins
pub(in crate::card::sets) static DWARVEN_RUINS: CardRecord = CardRecord::new_with_legacy_id(
    596,
    "Dwarven Ruins",
    CardArt::new("0dfe1352-27be-4c99-a58f-b961f911f270", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {R}{R}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
        ),
    ]),
);

// FEM 95 — Ebon Stronghold
pub(in crate::card::sets) static EBON_STRONGHOLD: CardRecord = CardRecord::new_with_legacy_id(
    597,
    "Ebon Stronghold",
    CardArt::new("3fb2a11f-a8e4-4acf-871a-11171e3304ef", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {B}{B}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(2)),
        ),
    ]),
);

// FEM 96 — Havenwood Battleground
pub(in crate::card::sets) static HAVENWOOD_BATTLEGROUND: CardRecord =
    CardRecord::new_with_legacy_id(
        598,
        "Havenwood Battleground",
        CardArt::new("9028f200-80dd-4c53-877f-ea380ff417cb", "Mark Poole"),
        CardSet::FallenEmpires,
        CardRules::new_land(&[]).with_abilities(&[
            AbilityDef::as_enters(
                "This land enters tapped.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
            ),
            AbilityDef::activated_mana(
                "{T}: Add {G}.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
            ),
            AbilityDef::activated_mana(
                "{T}, Sacrifice this land: Add {G}{G}.",
                &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(2)),
            ),
        ]),
    );

// FEM 97 — Hollow Trees
pub(in crate::card::sets) static HOLLOW_TREES: CardRecord = CardRecord::new_with_legacy_id(
    1985,
    "Hollow Trees",
    CardArt::new("90845410-e09a-4753-ad4c-bf2b2f3c95ac", "Pat Lewis"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::static_ability(
            "You may choose not to untap this land during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this land is tapped, put a storage counter on it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceIsTapped,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Storage,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {G} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::Storage),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// FEM 98 — Icatian Store
pub(in crate::card::sets) static ICATIAN_STORE: CardRecord = CardRecord::new_with_legacy_id(
    1986,
    "Icatian Store",
    CardArt::new("d7cd8d8c-52c7-402f-92e1-5e5866f2555a", "Pat Lewis"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::static_ability(
            "You may choose not to untap this land during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this land is tapped, put a storage counter on it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceIsTapped,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Storage,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {W} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::Storage),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ]),
);

// FEM 99 — Rainbow Vale
// Audit: metadata-only — Needs duration-aware control-changing continuous effects for “{T}: Add one mana of any color. An opponent gains control of this land at the beginning of the next end step”.
pub(in crate::card::sets) static RAINBOW_VALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1b138e1-f8fc-435c-9aed-98004768479c"),
    "Rainbow Vale",
    crate::card::CardArt::new("c1b138e1-f8fc-435c-9aed-98004768479c", "Kaja Foglio"),
    crate::card::CardSet::FallenEmpires,
    crate::card::CardRules::unsupported(),
);

// FEM 100 — Ruins of Trokair
pub(in crate::card::sets) static RUINS_OF_TROKAIR: CardRecord = CardRecord::new_with_legacy_id(
    599,
    "Ruins of Trokair",
    CardArt::new("4ce2e734-8cff-4bfe-85f8-17b3e1903f18", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {W}{W}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White).with_amount(2)),
        ),
    ]),
);

// FEM 101 — Sand Silos
pub(in crate::card::sets) static SAND_SILOS: CardRecord = CardRecord::new_with_legacy_id(
    1987,
    "Sand Silos",
    CardArt::new("3f6f1fcb-d903-4a31-abab-40488569eef6", "Pat Lewis"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::static_ability(
            "You may choose not to untap this land during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this land is tapped, put a storage counter on it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceIsTapped,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Storage,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {U} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::Storage),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// FEM 102 — Svyelunite Temple
pub(in crate::card::sets) static SVYELUNITE_TEMPLE: CardRecord = CardRecord::new_with_legacy_id(
    600,
    "Svyelunite Temple",
    CardArt::new("8b3fde62-ab21-459b-9c5d-01aa6fe1d08e", "Mark Poole"),
    CardSet::FallenEmpires,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this land: Add {U}{U}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue).with_amount(2)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COMBAT_MEDIC,
    &FARREL_S_MANTLE,
    &FARRELS_ZEALOT,
    &FARRELITE_PRIEST,
    &HAND_OF_JUSTICE,
    &HEROISM,
    &ICATIAN_INFANTRY,
    &ICATIAN_JAVELINEERS,
    &ICATIAN_LIEUTENANT,
    &ICATIAN_MONEYCHANGER,
    &ICATIAN_PHALANX,
    &ICATIAN_PRIEST,
    &ICATIAN_SCOUT,
    &ICATIAN_SKIRMISHERS,
    &ICATIAN_TOWN,
    &ORDER_OF_LEITBUR,
    &DEEP_SPAWN,
    &HIGH_TIDE,
    &HOMARID,
    &HOMARID_SHAMAN,
    &HOMARID_SPAWNING_BED,
    &HOMARID_WARRIOR,
    &MERSEINE,
    &RIVER_MERFOLK,
    &SEASINGER,
    &SVYELUNITE_PRIEST,
    &TIDAL_FLATS,
    &TIDAL_INFLUENCE,
    &VODALIAN_KNIGHTS,
    &VODALIAN_MAGE,
    &VODALIAN_SOLDIERS,
    &VODALIAN_WAR_MACHINE,
    &ARMOR_THRULL,
    &BASAL_THRULL,
    &BREEDING_PIT,
    &DERELOR,
    &EBON_PRAETOR,
    &HYMN_TO_TOURACH,
    &INITIATES_OF_THE_EBON_HAND,
    &MINDSTAB_THRULL,
    &NECRITE,
    &ORDER_OF_THE_EBON_HAND,
    &SOUL_EXCHANGE,
    &THRULL_CHAMPION,
    &THRULL_RETAINER,
    &THRULL_WIZARD,
    &TOURACH_S_CHANT,
    &TOURACH_S_GATE,
    &BRASSCLAW_ORCS,
    &DWARVEN_ARMORER,
    &DWARVEN_CATAPULT,
    &DWARVEN_LIEUTENANT,
    &DWARVEN_SOLDIER,
    &GOBLIN_CHIRURGEON,
    &GOBLIN_FLOTILLA,
    &GOBLIN_GRENADE,
    &GOBLIN_KITES,
    &GOBLIN_WAR_DRUMS,
    &GOBLIN_WARRENS,
    &ORCISH_CAPTAIN,
    &ORCISH_SPY,
    &ORCISH_VETERAN,
    &ORGG,
    &RAIDING_PARTY,
    &ELVEN_FORTRESS,
    &ELVISH_FARMER,
    &ELVISH_HUNTER,
    &ELVISH_SCOUT,
    &FERAL_THALLID,
    &FUNGAL_BLOOM,
    &NIGHT_SOIL,
    &SPORE_CLOUD,
    &SPORE_FLOWER,
    &THALLID,
    &THALLID_DEVOURER,
    &THELON_S_CHANT,
    &THELON_S_CURSE,
    &THELONITE_DRUID,
    &THELONITE_MONK,
    &THORN_THALLID,
    &AEOLIPILE,
    &BALM_OF_RESTORATION,
    &CONCH_HORN,
    &DELIF_S_CONE,
    &DELIF_S_CUBE,
    &DRACONIAN_CYLIX,
    &ELVEN_LYRE,
    &IMPLEMENTS_OF_SACRIFICE,
    &RING_OF_RENEWAL,
    &SPIRIT_SHIELD,
    &ZELYON_SWORD,
    &BOTTOMLESS_VAULT,
    &DWARVEN_HOLD,
    &DWARVEN_RUINS,
    &EBON_STRONGHOLD,
    &HAVENWOOD_BATTLEGROUND,
    &HOLLOW_TREES,
    &ICATIAN_STORE,
    &RAINBOW_VALE,
    &RUINS_OF_TROKAIR,
    &SAND_SILOS,
    &SVYELUNITE_TEMPLE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&COMBAT_MEDIC, 1),     // FEM 1b
    PrintingRecord::alternate(&COMBAT_MEDIC, 2),     // FEM 1c
    PrintingRecord::alternate(&COMBAT_MEDIC, 3),     // FEM 1d
    PrintingRecord::alternate(&FARRELS_ZEALOT, 1),   // FEM 3b
    PrintingRecord::alternate(&FARRELS_ZEALOT, 2),   // FEM 3c
    PrintingRecord::alternate(&ICATIAN_INFANTRY, 1), // FEM 7b
    PrintingRecord::alternate(&ICATIAN_INFANTRY, 2), // FEM 7c
    PrintingRecord::alternate(&ICATIAN_INFANTRY, 3), // FEM 7d
    PrintingRecord::alternate(&ICATIAN_JAVELINEERS, 1), // FEM 8b
    PrintingRecord::alternate(&ICATIAN_JAVELINEERS, 2), // FEM 8c
    PrintingRecord::alternate(&ICATIAN_MONEYCHANGER, 1), // FEM 10b
    PrintingRecord::alternate(&ICATIAN_MONEYCHANGER, 2), // FEM 10c
    PrintingRecord::alternate(&ICATIAN_SCOUT, 1),    // FEM 13b
    PrintingRecord::alternate(&ICATIAN_SCOUT, 2),    // FEM 13c
    PrintingRecord::alternate(&ICATIAN_SCOUT, 3),    // FEM 13d
    PrintingRecord::alternate(&ORDER_OF_LEITBUR, 1), // FEM 16b
    PrintingRecord::alternate(&ORDER_OF_LEITBUR, 2), // FEM 16c
    PrintingRecord::alternate(&HIGH_TIDE, 1),        // FEM 18b
    PrintingRecord::alternate(&HIGH_TIDE, 2),        // FEM 18c
    PrintingRecord::alternate(&HOMARID, 1),          // FEM 19b
    PrintingRecord::alternate(&HOMARID, 2),          // FEM 19c
    PrintingRecord::alternate(&HOMARID, 3),          // FEM 19d
    PrintingRecord::alternate(&HOMARID_WARRIOR, 1),  // FEM 22b
    PrintingRecord::alternate(&HOMARID_WARRIOR, 2),  // FEM 22c
    PrintingRecord::alternate(&MERSEINE, 1),         // FEM 23a
    PrintingRecord::alternate(&MERSEINE, 2),         // FEM 23b
    PrintingRecord::alternate(&MERSEINE, 3),         // FEM 23d
    PrintingRecord::alternate(&TIDAL_FLATS, 1),      // FEM 27b
    PrintingRecord::alternate(&TIDAL_FLATS, 2),      // FEM 27c
    PrintingRecord::alternate(&VODALIAN_MAGE, 1),    // FEM 30b
    PrintingRecord::alternate(&VODALIAN_MAGE, 2),    // FEM 30c
    PrintingRecord::alternate(&VODALIAN_SOLDIERS, 1), // FEM 31b
    PrintingRecord::alternate(&VODALIAN_SOLDIERS, 2), // FEM 31c
    PrintingRecord::alternate(&VODALIAN_SOLDIERS, 3), // FEM 31d
    PrintingRecord::alternate(&ARMOR_THRULL, 1),     // FEM 33b
    PrintingRecord::alternate(&ARMOR_THRULL, 2),     // FEM 33c
    PrintingRecord::alternate(&ARMOR_THRULL, 3),     // FEM 33d
    PrintingRecord::alternate(&BASAL_THRULL, 1),     // FEM 34b
    PrintingRecord::alternate(&BASAL_THRULL, 2),     // FEM 34c
    PrintingRecord::alternate(&BASAL_THRULL, 3),     // FEM 34d
    PrintingRecord::alternate(&HYMN_TO_TOURACH, 1),  // FEM 38b
    PrintingRecord::alternate(&HYMN_TO_TOURACH, 2),  // FEM 38c
    PrintingRecord::alternate(&HYMN_TO_TOURACH, 3),  // FEM 38d
    PrintingRecord::alternate(&INITIATES_OF_THE_EBON_HAND, 1), // FEM 39a
    PrintingRecord::alternate(&INITIATES_OF_THE_EBON_HAND, 2), // FEM 39c
    PrintingRecord::alternate(&MINDSTAB_THRULL, 1),  // FEM 40b
    PrintingRecord::alternate(&MINDSTAB_THRULL, 2),  // FEM 40c
    PrintingRecord::alternate(&NECRITE, 1),          // FEM 41b
    PrintingRecord::alternate(&NECRITE, 2),          // FEM 41c
    PrintingRecord::alternate(&ORDER_OF_THE_EBON_HAND, 1), // FEM 42b
    PrintingRecord::alternate(&ORDER_OF_THE_EBON_HAND, 2), // FEM 42c
    PrintingRecord::alternate(&BRASSCLAW_ORCS, 1),   // FEM 49b
    PrintingRecord::alternate(&BRASSCLAW_ORCS, 2),   // FEM 49c
    PrintingRecord::alternate(&BRASSCLAW_ORCS, 3),   // FEM 49d
    PrintingRecord::alternate(&DWARVEN_SOLDIER, 1),  // FEM 53b
    PrintingRecord::alternate(&DWARVEN_SOLDIER, 2),  // FEM 53c
    PrintingRecord::alternate(&GOBLIN_CHIRURGEON, 1), // FEM 54b
    PrintingRecord::alternate(&GOBLIN_CHIRURGEON, 2), // FEM 54c
    PrintingRecord::alternate(&GOBLIN_GRENADE, 1),   // FEM 56b
    PrintingRecord::alternate(&GOBLIN_GRENADE, 2),   // FEM 56c
    PrintingRecord::alternate(&GOBLIN_WAR_DRUMS, 1), // FEM 58b
    PrintingRecord::alternate(&GOBLIN_WAR_DRUMS, 2), // FEM 58c
    PrintingRecord::alternate(&GOBLIN_WAR_DRUMS, 3), // FEM 58d
    PrintingRecord::alternate(&ORCISH_SPY, 1),       // FEM 61b
    PrintingRecord::alternate(&ORCISH_SPY, 2),       // FEM 61c
    PrintingRecord::alternate(&ORCISH_VETERAN, 1),   // FEM 62b
    PrintingRecord::alternate(&ORCISH_VETERAN, 2),   // FEM 62c
    PrintingRecord::alternate(&ORCISH_VETERAN, 3),   // FEM 62d
    PrintingRecord::alternate(&ELVEN_FORTRESS, 1),   // FEM 65b
    PrintingRecord::alternate(&ELVEN_FORTRESS, 2),   // FEM 65c
    PrintingRecord::alternate(&ELVEN_FORTRESS, 3),   // FEM 65d
    PrintingRecord::alternate(&ELVISH_HUNTER, 1),    // FEM 67b
    PrintingRecord::alternate(&ELVISH_HUNTER, 2),    // FEM 67c
    PrintingRecord::alternate(&ELVISH_SCOUT, 1),     // FEM 68b
    PrintingRecord::alternate(&ELVISH_SCOUT, 2),     // FEM 68c
    PrintingRecord::alternate(&NIGHT_SOIL, 1),       // FEM 71b
    PrintingRecord::alternate(&NIGHT_SOIL, 2),       // FEM 71c
    PrintingRecord::alternate(&SPORE_CLOUD, 1),      // FEM 72b
    PrintingRecord::alternate(&SPORE_CLOUD, 2),      // FEM 72c
    PrintingRecord::alternate(&THALLID, 1),          // FEM 74b
    PrintingRecord::alternate(&THALLID, 2),          // FEM 74c
    PrintingRecord::alternate(&THALLID, 3),          // FEM 74d
    PrintingRecord::alternate(&THORN_THALLID, 1),    // FEM 80b
    PrintingRecord::alternate(&THORN_THALLID, 2),    // FEM 80c
    PrintingRecord::alternate(&THORN_THALLID, 3),    // FEM 80d
];
