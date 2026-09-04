use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef,
    ComparisonDef, ControlDurationDef, CostModificationDef, CostQuantityDef, CounterKind,
    DamageEventMatcherDef, DamagePreventionDef, DiscardSelectionDef, EffectDef, EffectPaymentDef,
    EffectRecipientDef, InstalledTriggerDef, LikelihoodDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, SpellAdditionalCostDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// FEM 1a — Combat Medic
pub(in crate::card::sets) static COMBAT_MEDIC: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Combat Medic",
    "9cfd96cb-03d6-4845-8595-50bf17b35726",
    "Edward P. Beard, Jr.",
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
const COMBAT_MEDIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COMBAT_MEDIC,
    1,
    "2a324a98-31c2-470a-b792-96b6b098a58c",
    "Susan Van Camp",
);

// FEM 1c — Combat Medic (alternate printing)
const COMBAT_MEDIC_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COMBAT_MEDIC,
    2,
    "ee9d1eac-3ac2-4881-a984-e40d87f60784",
    "Anson Maddocks",
);

// FEM 1d — Combat Medic (alternate printing)
const COMBAT_MEDIC_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &COMBAT_MEDIC,
    3,
    "8f26c079-61ea-436d-89ae-2f1c6f863e91",
    "Liz Danforth",
);

// FEM 2 — Farrel's Mantle
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature attacks and isn't blocked, its controller may have it deal damage equal to its power plus 2 to another target creature. If that player does, the attacking…”.
pub(in crate::card::sets) static FARREL_S_MANTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Farrel's Mantle",
    "af092da3-8713-4a59-86d3-827b942d6456",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// FEM 3a — Farrel's Zealot
pub(in crate::card::sets) static FARRELS_ZEALOT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Farrel's Zealot",
    "0401bd23-9f81-40b7-a6c2-e3f9847d175c",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may have it deal 3 damage to \
             target creature. If you do, this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            },
        ),
    ),
);

// FEM 3b — Farrel's Zealot (alternate printing)
const FARRELS_ZEALOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FARRELS_ZEALOT,
    1,
    "9e3aeee7-975c-419a-bfb3-45bb48ba6918",
    "Richard Kane Ferguson",
);

// FEM 3c — Farrel's Zealot (alternate printing)
const FARRELS_ZEALOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FARRELS_ZEALOT,
    2,
    "54252fd2-21a6-40d1-8515-697f18c78a06",
    "Edward P. Beard, Jr.",
);

// FEM 4 — Farrelite Priest
// Audit: unsupported — Needs the mana-ability runtime to pay this ability's mana activation cost for “{1}: Add {W}. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step”.
pub(in crate::card::sets) static FARRELITE_PRIEST: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Farrelite Priest",
    "e11bf79b-a951-4d0c-acdf-d8ba5290a648",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// FEM 5 — Hand of Justice
// Audit: unsupported — Needs a persistent tap/untap restriction or event relation for “{T}, Tap three untapped white creatures you control: Destroy target creature”.
pub(in crate::card::sets) static HAND_OF_JUSTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Hand of Justice",
    "7a899b2d-825c-4929-a769-f4df70bf6a17",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// FEM 6 — Heroism
// Audit: unsupported — Needs a per-creature optional payment offered to the opposing controller, repeated for each attacking red creature; preventing one creature's combat damage is already expressible.
pub(in crate::card::sets) static HEROISM: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Heroism",
    "08ee87a0-a7eb-4472-9045-85d11e8a1501",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// FEM 7a — Icatian Infantry
pub(in crate::card::sets) static ICATIAN_INFANTRY: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Infantry",
    "f95d42d8-ba75-43bf-81b8-b02374f03e83",
    "Edward P. Beard, Jr.",
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
const ICATIAN_INFANTRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_INFANTRY,
    1,
    "e0e4a9d2-ea43-46ac-8b8b-00496a478103",
    "Christopher Rush",
);

// FEM 7c — Icatian Infantry (alternate printing)
const ICATIAN_INFANTRY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_INFANTRY,
    2,
    "efac583d-a492-45ee-8c52-60a6422b2168",
    "Douglas Shuler",
);

// FEM 7d — Icatian Infantry (alternate printing)
const ICATIAN_INFANTRY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_INFANTRY,
    3,
    "96b2a8d4-7c06-454c-9923-553294aada4f",
    "Drew Tucker",
);

// FEM 8a — Icatian Javelineers
pub(in crate::card::sets) static ICATIAN_JAVELINEERS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Javelineers",
    "f04b8356-2384-4743-80dd-f15ca7ec65f7",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a javelin counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("javelin"),
                    amount: 1,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Remove a javelin counter from this creature: It deals 1 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("javelin"),
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
const ICATIAN_JAVELINEERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_JAVELINEERS,
    1,
    "c70f8f50-866a-4889-b986-48636225638a",
    "Edward P. Beard, Jr.",
);

// FEM 8c — Icatian Javelineers (alternate printing)
const ICATIAN_JAVELINEERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_JAVELINEERS,
    2,
    "2be5ab7a-e7db-4c09-8df2-6fe55fa4a116",
    "Scott Kirschner",
);

// FEM 9 — Icatian Lieutenant
pub(in crate::card::sets) static ICATIAN_LIEUTENANT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Lieutenant",
    "39fec59a-4ade-4c6f-ae7d-911fbe6da26d",
    "Pete Venters",
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
pub(in crate::card::sets) static ICATIAN_MONEYCHANGER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Moneychanger",
    "b3d502d4-4a96-47b3-ae26-8b2c9f36623d",
    "Drew Tucker",
    CardRules::new_creature(mana_cost!("{W}"), &["Human"], 0, 2).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three credit counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("credit"),
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
                kind: CounterKind::named("credit"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Sacrifice this creature: You gain 1 life for each credit counter on this creature. \
             Activate only during your upkeep.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountersOnSource(CounterKind::named("credit")),
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ]),
);

// FEM 10b — Icatian Moneychanger (alternate printing)
const ICATIAN_MONEYCHANGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_MONEYCHANGER,
    1,
    "cbf9194c-8e50-4f50-9a87-3b339a5bc279",
    "Edward P. Beard, Jr.",
);

// FEM 10c — Icatian Moneychanger (alternate printing)
const ICATIAN_MONEYCHANGER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_MONEYCHANGER,
    2,
    "cf9521ae-6fac-4d86-9c60-adecaae5687d",
    "Melissa A. Benson",
);

// FEM 11 — Icatian Phalanx
pub(in crate::card::sets) static ICATIAN_PHALANX: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Phalanx",
    "7bc02d30-3eef-4a48-8b11-b4f37219ab3a",
    "Kaja Foglio",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 2, 4)
        .with_abilities(&[abilities::banding()]),
);

// FEM 12 — Icatian Priest
pub(in crate::card::sets) static ICATIAN_PRIEST: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Priest",
    "d7690cdd-6610-4310-9e93-60dc4db2ae8d",
    "Drew Tucker",
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
pub(in crate::card::sets) static ICATIAN_SCOUT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Scout",
    "86bf4aaa-a9b1-4798-a96b-c3e35afb77f7",
    "Richard Kane Ferguson",
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
const ICATIAN_SCOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_SCOUT,
    1,
    "e9db3442-01cb-4db2-ac33-8eca6880c315",
    "Douglas Shuler",
);

// FEM 13c — Icatian Scout (alternate printing)
const ICATIAN_SCOUT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_SCOUT,
    2,
    "6c461655-a05d-4eed-85b2-04d554f5ec50",
    "Rob Alexander",
);

// FEM 13d — Icatian Scout (alternate printing)
const ICATIAN_SCOUT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ICATIAN_SCOUT,
    3,
    "db63ad7f-6dc4-4249-b360-46ec5569a5a9",
    "Phil Foglio",
);

// FEM 14 — Icatian Skirmishers
pub(in crate::card::sets) static ICATIAN_SKIRMISHERS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Skirmishers",
    "15f6d115-c02d-45a3-aa6d-402964df47dd",
    "Heather Hudson",
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
pub(in crate::card::sets) static ICATIAN_TOWN: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Town",
    "cbb7c28d-0366-4d01-84a2-f1bc9f38aa4a",
    "Tom Wänerstrand",
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
pub(in crate::card::sets) static ORDER_OF_LEITBUR: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Order of Leitbur",
    "ebd6e51e-f042-4673-a898-291607105829",
    "Bryon Wackwitz",
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
const ORDER_OF_LEITBUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORDER_OF_LEITBUR,
    1,
    "fb537b5a-d725-420d-bc15-0d54ba23331c",
    "Bryon Wackwitz",
);

// FEM 16c — Order of Leitbur (alternate printing)
const ORDER_OF_LEITBUR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORDER_OF_LEITBUR,
    2,
    "1373dea4-3565-4612-8505-ab8fba3ddb67",
    "Randy Asplund-Faith",
);

// FEM 17 — Deep Spawn
pub(in crate::card::sets) static DEEP_SPAWN: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Deep Spawn",
    "69c9e4a5-735f-471c-ab1a-6e6d50ba5724",
    "Mark Tedin",
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
                otherwise: Some(&EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                }),
                visibility: ChoiceVisibilityDef::Public,
                condition: None,
            }),
        ),
        AbilityDef::activated(
            "{U}: This creature gains shroud until end of turn and doesn't untap during your \
             next untap step. Tap this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            // One activation buys three things at once, and the untap prohibition is
            // what pays for the other two: shroud until end of turn, no untap next turn,
            // and the tap that puts the creature away in the first place.
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::shroud()),
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
            ]),
        ),
    ]),
);

// FEM 18a — High Tide
// Audit: unsupported — Needs cost/mana provenance or dynamic payment support for “Until end of turn, whenever a player taps an Island for mana, that player adds an additional {U}”.
pub(in crate::card::sets) static HIGH_TIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "High Tide",
    "4686bbb9-517f-4cce-aa7a-5db41e22c02b",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// FEM 18b — High Tide (alternate printing)
const HIGH_TIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_TIDE,
    1,
    "c2813677-91cc-4c8b-a8ea-403fa776c9f0",
    "Anson Maddocks",
);

// FEM 18c — High Tide (alternate printing)
const HIGH_TIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HIGH_TIDE,
    2,
    "4af611e3-45d6-4aee-bf48-56598b14a242",
    "Amy Weber",
);

// FEM 19a — Homarid
pub(in crate::card::sets) static HOMARID: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Homarid",
    "d6ffeab4-83b1-4414-ae72-e59a2354ea15",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Homarid"], 2, 2).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a tide counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("tide"),
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
                kind: CounterKind::named("tide"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "As long as there is exactly one tide counter on this creature, it gets -1/-1.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("tide"),
                    comparison: ComparisonDef::Equal,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                },
            },
        ),
        AbilityDef::static_ability(
            "As long as there are exactly three tide counters on this creature, it gets +1/+1.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("tide"),
                    comparison: ComparisonDef::Equal,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
        AbilityDef::triggered_if(
            "Whenever there are four or more tide counters on this creature, remove all tide \
             counters from it.",
            TriggerEventDef::StateCondition,
            &TriggerConditionDef::SourceCounters {
                kind: CounterKind::named("tide"),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 4,
            },
            EffectDef::RemoveAllCounters {
                object: EffectRecipientDef::Source,
                kind: Some(CounterKind::named("tide")),
            },
        ),
    ]),
);

// FEM 19b — Homarid (alternate printing)
const HOMARID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOMARID,
    1,
    "cbb6c13f-6019-4ad5-9de6-07844c361b41",
    "Heather Hudson",
);

// FEM 19c — Homarid (alternate printing)
const HOMARID_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOMARID,
    2,
    "33536b0a-1cff-481f-b695-eadaf6897bf0",
    "Mark Tedin",
);

// FEM 19d — Homarid (alternate printing)
const HOMARID_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HOMARID,
    3,
    "18f1cc24-a5fc-43cc-b558-ac7901c48b81",
    "Bryon Wackwitz",
);

// FEM 20 — Homarid Shaman
pub(in crate::card::sets) static HOMARID_SHAMAN: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Homarid Shaman",
    "c17c6416-86d6-46ea-aea1-41b98a66b250",
    "Amy Weber",
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
// Audit: unsupported — Needs Camarid token creation whose count is the sacrificed creature's mana value.
pub(in crate::card::sets) static HOMARID_SPAWNING_BED: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Homarid Spawning Bed",
    "2cbb62fc-3cd9-41a6-804a-4ff9a766897f",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// FEM 22a — Homarid Warrior
pub(in crate::card::sets) static HOMARID_WARRIOR: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Homarid Warrior",
    "627ca588-917f-4768-a69d-3d93c1210390",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Homarid", "Warrior"], 3, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gains shroud until end of turn and doesn't untap during your \
             next untap step. Tap it.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::shroud()),
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
const HOMARID_WARRIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOMARID_WARRIOR,
    1,
    "c9a9bdcf-543b-4140-b836-9e222a4a9233",
    "Randy Asplund-Faith",
);

// FEM 22c — Homarid Warrior (alternate printing)
const HOMARID_WARRIOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOMARID_WARRIOR,
    2,
    "fb1cccdc-9c4d-4ef3-807b-278e6fd23230",
    "Douglas Shuler",
);

// FEM 23a — Merseine (alternate printing)
const MERSEINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MERSEINE,
    1,
    "b1e96895-ef1d-44fa-b263-bce833fc3109",
    "Heather Hudson",
);

// FEM 23b — Merseine (alternate printing)
const MERSEINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MERSEINE,
    2,
    "5c7fb804-65ba-477e-93e8-eea101c1521e",
    "Margaret Organ-Kean",
);

// FEM 23c — Merseine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERSEINE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Merseine",
    "2dd197f8-ced0-461a-9672-2720a7b70803",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// FEM 23d — Merseine (alternate printing)
const MERSEINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MERSEINE,
    3,
    "ae7a9e9a-d1f8-44c5-9f79-a1201acfb5fc",
    "Pete Venters",
);

// FEM 24 — River Merfolk
pub(in crate::card::sets) static RIVER_MERFOLK: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "River Merfolk",
    "27d7fa54-4b89-4a9a-b088-4b89c525c1ea",
    "Douglas Shuler",
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
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “{T}: Gain control of target creature whose controller controls an Island for as long as you control this creature and this creature remains tapped”.
pub(in crate::card::sets) static SEASINGER: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Seasinger",
    "c5266aa1-e2ea-46b9-91ab-b94a7bb7e9f9",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// FEM 26 — Svyelunite Priest
pub(in crate::card::sets) static SVYELUNITE_PRIEST: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Svyelunite Priest",
    "316d25ae-7ac6-4f5b-93ab-0e0e28ec104b",
    "Ron Spencer",
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
                effect: AppliedEffectDef::add_ability(&abilities::shroud()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// FEM 27a — Tidal Flats
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “{U}{U}: For each attacking creature without flying, its controller may pay {1}. If that player doesn't, creatures you control blocking that creature gain first strike until end of turn”.
pub(in crate::card::sets) static TIDAL_FLATS: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Tidal Flats",
    "2e820f3f-434e-4d09-91b9-0ebd6966b393",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// FEM 27b — Tidal Flats (alternate printing)
const TIDAL_FLATS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TIDAL_FLATS,
    1,
    "50e7d376-3e22-44aa-9c96-a3b8eb1568fe",
    "Rob Alexander",
);

// FEM 27c — Tidal Flats (alternate printing)
const TIDAL_FLATS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TIDAL_FLATS,
    2,
    "445c4767-6261-449c-bb57-713e2a2bb0bf",
    "Sandra Everingham",
);

// FEM 28 — Tidal Influence
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “As long as there are exactly three tide counters on this enchantment, all blue creatures get +2/+0”.
pub(in crate::card::sets) static TIDAL_INFLUENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Tidal Influence",
    "b2192c7b-ef6f-4ff6-9017-b1a125340517",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// FEM 29 — Vodalian Knights
pub(in crate::card::sets) static VODALIAN_KNIGHTS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Vodalian Knights",
    "68d97e1b-2526-4740-b354-f158734d1f72",
    "Susan Van Camp",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Merfolk", "Knight"], 2, 2).with_abilities(
        &[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "This creature can't attack unless defending player controls an Island.",
                EffectDef::CannotAttackUnless(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                )),
            ),
            AbilityDef::triggered_if(
                "When you control no Islands, sacrifice this creature.",
                TriggerEventDef::StateCondition,
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::Equal,
                    amount: 0,
                },
                EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            ),
        ],
    ),
);

// FEM 30a — Vodalian Mage
pub(in crate::card::sets) static VODALIAN_MAGE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Vodalian Mage",
    "c107e82b-134a-4f2b-98c2-6537fae6a50d",
    "Susan Van Camp",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}, {T}: Counter target spell unless its controller pays {1}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
        ),
    ]),
);

// FEM 30b — Vodalian Mage (alternate printing)
const VODALIAN_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VODALIAN_MAGE,
    1,
    "a47beac4-161d-4f8e-9778-78293ff9b383",
    "Mark Poole",
);

// FEM 30c — Vodalian Mage (alternate printing)
const VODALIAN_MAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VODALIAN_MAGE,
    2,
    "2b3cc91d-6f87-4f2e-b3c7-8181d19a1f0b",
    "Quinton Hoover",
);

// FEM 31a — Vodalian Soldiers
pub(in crate::card::sets) static VODALIAN_SOLDIERS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Vodalian Soldiers",
    "7eb50256-9113-4b03-bcef-9aea24be8493",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Soldier"], 1, 2),
);

// FEM 31b — Vodalian Soldiers (alternate printing)
const VODALIAN_SOLDIERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VODALIAN_SOLDIERS,
    1,
    "bc85a68c-14d6-4447-a894-0e48d1662bc3",
    "Jeff A. Menges",
);

// FEM 31c — Vodalian Soldiers (alternate printing)
const VODALIAN_SOLDIERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VODALIAN_SOLDIERS,
    2,
    "d8d1ceac-bb75-4c46-9ab4-1ef623ed3027",
    "Richard Kane Ferguson",
);

// FEM 31d — Vodalian Soldiers (alternate printing)
const VODALIAN_SOLDIERS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VODALIAN_SOLDIERS,
    3,
    "99d22f83-1171-4b5c-8a72-956db26d7c60",
    "Susan Van Camp",
);

// FEM 32 — Vodalian War Machine
// Audit: unsupported — Needs the permanents tapped to pay this card's own costs recorded for the turn, for “When this creature dies, destroy all Merfolk tapped this turn to pay for its abilities”. Tapping another creature as a cost and attacking despite defender are both available.
pub(in crate::card::sets) static VODALIAN_WAR_MACHINE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Vodalian War Machine",
    "cd962ff0-4aa6-453e-931e-bd36fc034273",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// FEM 33a — Armor Thrull
pub(in crate::card::sets) static ARMOR_THRULL: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Armor Thrull",
    "a98384d1-8e7d-4c41-9f23-47bc2ae2ad6a",
    "Pete Venters",
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
const ARMOR_THRULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARMOR_THRULL,
    1,
    "9c6120e6-ceb8-4eab-86b0-18d38ed97d8f",
    "Ron Spencer",
);

// FEM 33c — Armor Thrull (alternate printing)
const ARMOR_THRULL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ARMOR_THRULL,
    2,
    "18a91ed4-131e-455b-a3bd-0bd42aa754e5",
    "Jeff A. Menges",
);

// FEM 33d — Armor Thrull (alternate printing)
const ARMOR_THRULL_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ARMOR_THRULL,
    3,
    "3d653ca4-c21f-4594-b900-2526a912001b",
    "Scott Kirschner",
);

// FEM 34a — Basal Thrull
pub(in crate::card::sets) static BASAL_THRULL: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Basal Thrull",
    "0c1d5d13-0160-48cb-8fac-dd86102569b4",
    "Kaja Foglio",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Thrull"], 1, 2).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}, Sacrifice this creature: Add {B}{B}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(2)),
        ),
    ]),
);

// FEM 34b — Basal Thrull (alternate printing)
const BASAL_THRULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BASAL_THRULL,
    1,
    "fcf60db5-4f69-4db4-9dc2-1a6fbdec0429",
    "Phil Foglio",
);

// FEM 34c — Basal Thrull (alternate printing)
const BASAL_THRULL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BASAL_THRULL,
    2,
    "a86d9647-3a87-4620-aa07-26f996fc6fa3",
    "Richard Kane Ferguson",
);

// FEM 34d — Basal Thrull (alternate printing)
const BASAL_THRULL_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BASAL_THRULL,
    3,
    "b6908e4c-f94d-4b0d-b9a5-64c04751f108",
    "Christopher Rush",
);

// FEM 35 — Breeding Pit
pub(in crate::card::sets) static BREEDING_PIT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Breeding Pit",
    "a0d7e85f-eba5-4fc5-9fc0-109109d368aa",
    "Anson Maddocks",
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
pub(in crate::card::sets) static DERELOR: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Derelor",
    "9eb2b79f-f09a-49dc-8e0f-7d711ba78981",
    "Anson Maddocks",
    // The tax is coloured, which is the whole joke: a 4/4 for four that makes
    // every black spell after it harder to cast, including itself if a second
    // one is already out.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Thrull"], 4, 4).with_ability(
        AbilityDef::static_ability(
            "Black spells you cast cost {B} more to cast.",
            EffectDef::ModifyCost(CostModificationDef::increase_spell(
                ObjectPredicateDef::Color(ManaColor::Black),
                PlayerRelation::You,
                mana_cost!("{B}"),
            )),
        ),
    ),
);

// FEM 37 — Ebon Praetor
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “Sacrifice a creature: Remove a -2/-2 counter from this creature. If the sacrificed creature was a Thrull, put a +1/+0 counter on this creature. Activate only during your upkeep and only…”.
pub(in crate::card::sets) static EBON_PRAETOR: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Ebon Praetor",
    "40451f7a-692a-422d-99d3-d93a4d9315e0",
    "Randy Asplund-Faith",
    crate::card::CardRules::unsupported(),
);

// FEM 38a — Hymn to Tourach
pub(in crate::card::sets) static HYMN_TO_TOURACH: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Hymn to Tourach",
    "eb9273ea-9a41-42e3-8c9c-0d50b127a818",
    "Susan Van Camp",
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
const HYMN_TO_TOURACH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYMN_TO_TOURACH,
    1,
    "8601f082-7e43-44ef-97d0-dead272b7eb4",
    "Liz Danforth",
);

// FEM 38c — Hymn to Tourach (alternate printing)
const HYMN_TO_TOURACH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HYMN_TO_TOURACH,
    2,
    "58e125c6-81dc-4907-aad2-2ccd1cb166f0",
    "Quinton Hoover",
);

// FEM 38d — Hymn to Tourach (alternate printing)
const HYMN_TO_TOURACH_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HYMN_TO_TOURACH,
    3,
    "5bc50e08-dd6f-4ea7-87f8-cce72bafb928",
    "Scott Kirschner",
);

// FEM 39a — Initiates of the Ebon Hand (alternate printing)
const INITIATES_OF_THE_EBON_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INITIATES_OF_THE_EBON_HAND,
    1,
    "5be87527-3b8f-4529-afdb-a61ad4e787e1",
    "Heather Hudson",
);

// FEM 39b — Initiates of the Ebon Hand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INITIATES_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Initiates of the Ebon Hand",
    "03c7dc01-46d0-42be-a1a9-48f69c846d12",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// FEM 39c — Initiates of the Ebon Hand (alternate printing)
const INITIATES_OF_THE_EBON_HAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &INITIATES_OF_THE_EBON_HAND,
    2,
    "62982970-e8b8-4659-bcf0-21aab662d89d",
    "Kaja Foglio",
);

// FEM 40a — Mindstab Thrull
pub(in crate::card::sets) static MINDSTAB_THRULL: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Mindstab Thrull",
    "499a791f-ac4f-4a96-b59b-37043686a79a",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Thrull"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do, \
             defending player discards three cards.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Sacrifice {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(3),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            },
        ),
    ),
);

// FEM 40b — Mindstab Thrull (alternate printing)
const MINDSTAB_THRULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MINDSTAB_THRULL,
    1,
    "781e4b62-3910-4ba1-9e72-e99de8523a94",
    "Heather Hudson",
);

// FEM 40c — Mindstab Thrull (alternate printing)
const MINDSTAB_THRULL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MINDSTAB_THRULL,
    2,
    "923189c6-d407-4cc4-a062-2f09a4c7c1e3",
    "Mark Tedin",
);

// FEM 41a — Necrite
pub(in crate::card::sets) static NECRITE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Necrite",
    "311d752a-ce8a-44cb-8aeb-1ed66705eb09",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Thrull"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may sacrifice it. If you do, \
             destroy target creature defending player controls. It can't be regenerated.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Sacrifice {
                        object: EffectRecipientDef::Source,
                    },
                    // "It can't be regenerated" is the destruction's own flag rather than a
                    // separate prohibition: nothing else this turn is being denied a shield.
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: false,
                        then: None,
                    },
                ]),
            },
        ),
    ),
);

// FEM 41b — Necrite (alternate printing)
const NECRITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NECRITE,
    1,
    "e19a4d41-e7b0-48b3-8e2e-9ac00f119ce2",
    "Christopher Rush",
);

// FEM 41c — Necrite (alternate printing)
const NECRITE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NECRITE,
    2,
    "660ae99f-4e61-45fd-9436-855a38289c8b",
    "Drew Tucker",
);

// FEM 42a — Order of the Ebon Hand
pub(in crate::card::sets) static ORDER_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Order of the Ebon Hand",
    "9e51f5d8-a7cc-4720-8af5-e002bcfd78a0",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Cleric", "Knight"], 2, 1).with_abilities(&[
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
    ]),
);

// FEM 42b — Order of the Ebon Hand (alternate printing)
const ORDER_OF_THE_EBON_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORDER_OF_THE_EBON_HAND,
    1,
    "60ffbb40-13c1-4d01-9421-95b2410d0d3b",
    "Christopher Rush",
);

// FEM 42c — Order of the Ebon Hand (alternate printing)
const ORDER_OF_THE_EBON_HAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORDER_OF_THE_EBON_HAND,
    2,
    "22c32774-5507-4a60-9ed2-2a570f6ff8e3",
    "Ron Spencer",
);

// FEM 43 — Soul Exchange
// Audit: unsupported — Needs a zone-object query and identity-preserving continuation for “As an additional cost to cast this spell, exile a creature you control”.
pub(in crate::card::sets) static SOUL_EXCHANGE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Soul Exchange",
    "9f73597d-f453-4d37-b2ef-c54ef683a884",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// FEM 44 — Thrull Champion
pub(in crate::card::sets) static THRULL_CHAMPION: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Thrull Champion",
    "4d3cafdd-a03b-4b08-b9c1-c776f8450d3a",
    "Daniel Gelon",
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
pub(in crate::card::sets) static THRULL_RETAINER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Thrull Retainer",
    "d800512b-1492-41d2-931d-57c625044454",
    "Ron Spencer",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
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
// Audit: unsupported — Needs an unless-payment offering a choice between two mana costs. Reading a spell's color is available.
pub(in crate::card::sets) static THRULL_WIZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Thrull Wizard",
    "c4e732fb-cbef-4fd8-b704-e4d513a6cf2d",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// FEM 47 — Tourach's Chant
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “Whenever a player puts a Forest onto the battlefield, this enchantment deals 3 damage to that player unless they put a -1/-1 counter on a creature they control”.
pub(in crate::card::sets) static TOURACH_S_CHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Tourach's Chant",
    "06883fd2-eccd-47c6-8c34-10d95e923685",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// FEM 48 — Tourach's Gate
// Audit: unsupported — Needs the clause's conditional recipient set or dynamic modifier value for “Tap enchanted land: Attacking creatures you control get +2/-1 until end of turn. Activate only if enchanted land is untapped”.
pub(in crate::card::sets) static TOURACH_S_GATE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Tourach's Gate",
    "d77f6401-a9fb-449c-b511-6fb837055bb4",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// FEM 49a — Brassclaw Orcs
pub(in crate::card::sets) static BRASSCLAW_ORCS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Brassclaw Orcs",
    "fc0cb8f6-6ba7-402c-9829-251f7443e871",
    "Rob Alexander",
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
const BRASSCLAW_ORCS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRASSCLAW_ORCS,
    1,
    "ac9d0354-9ddd-4fe1-8174-9d3686ca564c",
    "Dan Frazier",
);

// FEM 49c — Brassclaw Orcs (alternate printing)
const BRASSCLAW_ORCS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BRASSCLAW_ORCS,
    2,
    "a2c1e461-f74e-436c-a9df-aff197cf48e1",
    "Rob Alexander",
);

// FEM 49d — Brassclaw Orcs (alternate printing)
const BRASSCLAW_ORCS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BRASSCLAW_ORCS,
    3,
    "50f0f4fe-2dd0-42c1-8f68-5d24a8a9d07d",
    "Heather Hudson",
);

// FEM 50 — Dwarven Armorer
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “{R}, {T}, Discard a card: Put a +0/+1 counter or a +1/+0 counter on target creature”.
pub(in crate::card::sets) static DWARVEN_ARMORER: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Dwarven Armorer",
    "1d50bf06-97ab-4874-a484-9289f41dc98e",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// FEM 51 — Dwarven Catapult
// Audit: unsupported — Needs damage divided evenly with downward rounding across a dynamically counted opponent creature set.
pub(in crate::card::sets) static DWARVEN_CATAPULT: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Dwarven Catapult",
    "8c1c6932-638a-4df7-bf9b-8d921f7484d9",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// FEM 52 — Dwarven Lieutenant
pub(in crate::card::sets) static DWARVEN_LIEUTENANT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Dwarven Lieutenant",
    "ea9a38b1-4676-425a-b40d-4fb478966024",
    "Jeff A. Menges",
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
pub(in crate::card::sets) static DWARVEN_SOLDIER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Dwarven Soldier",
    "6fe77608-0b33-43f5-83fb-ae993ca1bf7c",
    "Rob Alexander",
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
const DWARVEN_SOLDIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DWARVEN_SOLDIER,
    1,
    "ea7e4c52-dfe1-4b15-a0d6-4f26c294426d",
    "Randy Asplund-Faith",
);

// FEM 53c — Dwarven Soldier (alternate printing)
const DWARVEN_SOLDIER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DWARVEN_SOLDIER,
    2,
    "872c5601-f356-4873-adf9-9a39536e7d4a",
    "Douglas Shuler",
);

// FEM 54a — Goblin Chirurgeon
pub(in crate::card::sets) static GOBLIN_CHIRURGEON: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Goblin Chirurgeon",
    "2b710c21-e9f5-4660-80f6-2104ec65f63f",
    "Daniel Gelon",
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
const GOBLIN_CHIRURGEON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_CHIRURGEON,
    1,
    "982115b2-e1e7-4b2f-8eb6-a1633477d4a8",
    "Phil Foglio",
);

// FEM 54c — Goblin Chirurgeon (alternate printing)
const GOBLIN_CHIRURGEON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_CHIRURGEON,
    2,
    "c9740842-7955-4cf9-8f76-a426858360b1",
    "Dan Frazier",
);

// FEM 55 — Goblin Flotilla
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “At the beginning of each combat, unless you pay {R}, whenever this creature blocks or becomes blocked by a creature this combat, that creature gains first strike until end of turn”.
pub(in crate::card::sets) static GOBLIN_FLOTILLA: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Goblin Flotilla",
    "87024efe-4a74-49fe-a43a-480bed0a650a",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// FEM 56a — Goblin Grenade
pub(in crate::card::sets) static GOBLIN_GRENADE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Goblin Grenade",
    "8837eaba-9602-4f63-9897-85583fcdcf51",
    "Ron Spencer",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a Goblin.\nGoblin Grenade deals 5 damage to any target.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::Subtype("Goblin"),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ]),
);

// FEM 56b — Goblin Grenade (alternate printing)
const GOBLIN_GRENADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_GRENADE,
    1,
    "dee262da-3002-4c08-8043-4e40e1b46822",
    "Dan Frazier",
);

// FEM 56c — Goblin Grenade (alternate printing)
const GOBLIN_GRENADE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_GRENADE,
    2,
    "1befdfc7-a1e3-4a2a-ad68-7d0fee170f3f",
    "Christopher Rush",
);

// FEM 57 — Goblin Kites
pub(in crate::card::sets) static GOBLIN_KITES: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Goblin Kites",
    "a0a27ac3-2273-469a-92ba-3f4a3d55de6f",
    "Anson Maddocks",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "{R}: Target creature you control with toughness 2 or less gains flying until end \
             of turn. Flip a coin at the beginning of the next end step. If you lose the \
             flip, sacrifice that creature.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            // "Toughness 2 or less", said as a strict bound because that is the shape the
            // predicate takes.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            // Berserk's shape with a coin in it: pump now, and a delayed trigger that
            // remembers the same target and may take it away.
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
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
                        on_failure: &EffectDef::SacrificeYours {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    },
                ))),
            ]),
        ),
    ),
);

// FEM 58a — Goblin War Drums
pub(in crate::card::sets) static GOBLIN_WAR_DRUMS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Goblin War Drums",
    "2a2c4e4b-e9a7-4180-927b-589514c21876",
    "Dan Frazier",
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have menace.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::menace()),
        },
    )),
);

// FEM 58b — Goblin War Drums (alternate printing)
const GOBLIN_WAR_DRUMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_WAR_DRUMS,
    1,
    "5988a3d2-748f-4642-9e33-293ddc568111",
    "Richard Kane Ferguson",
);

// FEM 58c — Goblin War Drums (alternate printing)
const GOBLIN_WAR_DRUMS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_WAR_DRUMS,
    2,
    "2232386e-986d-41b5-8b70-e086264f3277",
    "Heather Hudson",
);

// FEM 58d — Goblin War Drums (alternate printing)
const GOBLIN_WAR_DRUMS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GOBLIN_WAR_DRUMS,
    3,
    "2a0185f3-fbc0-44d7-b933-30627cda1bf9",
    "Jeff A. Menges",
);

// FEM 59 — Goblin Warrens
// Audit: unsupported — Needs an activated cost that selects and sacrifices two Goblins; only one chosen permanent can currently be sacrificed as a cost.
pub(in crate::card::sets) static GOBLIN_WARRENS: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Goblin Warrens",
    "bbec4aa5-3319-43dc-8347-5633edbd7018",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// FEM 60 — Orcish Captain
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

pub(in crate::card::sets) static ORCISH_CAPTAIN: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Orcish Captain",
    "e43cf61d-b4d6-4461-a228-47fd8b026d33",
    "Mark Tedin",
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
                // A coin is an even chance, which is the whole of what "flip a coin" means
                // to the seeded randomiser.
                likelihood: LikelihoodDef::new(0.5),
                on_success: &orcish_captain_pump(2, 0),
                on_failure: &orcish_captain_pump(0, -2),
            },
        ),
    ),
);

// FEM 61a — Orcish Spy
pub(in crate::card::sets) static ORCISH_SPY: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Orcish Spy",
    "cd3890d1-563d-4519-ab8c-913031d71918",
    "Susan Van Camp",
    CardRules::new_creature(mana_cost!("{R}"), &["Orc", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Look at the top three cards of target player's library.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            abilities::look_at_top_cards(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
    ),
);

// FEM 61b — Orcish Spy (alternate printing)
const ORCISH_SPY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORCISH_SPY,
    1,
    "8b931cfd-b952-416c-ab2c-271ecaee8e0c",
    "Daniel Gelon",
);

// FEM 61c — Orcish Spy (alternate printing)
const ORCISH_SPY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORCISH_SPY,
    2,
    "28e08767-7e92-4ff4-b0d8-196565fbc23c",
    "Pete Venters",
);

// FEM 62a — Orcish Veteran
pub(in crate::card::sets) static ORCISH_VETERAN: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Orcish Veteran",
    "1dbca765-8756-4e28-9faf-25714c9b8838",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block white creatures with power 2 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    // The restriction is authored as the permission it leaves behind: anything
                    // that is not both white and big enough.
                    ObjectPredicateDef::Not(&ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Color(ManaColor::White),
                        ObjectPredicateDef::PowerAtLeast(2),
                    ])),
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
const ORCISH_VETERAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORCISH_VETERAN,
    1,
    "bc37db83-9efc-4d58-90c9-78eef9073ec2",
    "Dan Frazier",
);

// FEM 62c — Orcish Veteran (alternate printing)
const ORCISH_VETERAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ORCISH_VETERAN,
    2,
    "334004e6-bf8c-4a4e-a30c-1537a99819c9",
    "Quinton Hoover",
);

// FEM 62d — Orcish Veteran (alternate printing)
const ORCISH_VETERAN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ORCISH_VETERAN,
    3,
    "4990dd4b-2b18-4e4c-81d4-1cd8d746a7dc",
    "Melissa A. Benson",
);

// FEM 63 — Orgg
pub(in crate::card::sets) static ORGG: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Orgg",
    "5af19ab0-4bd0-4d5f-8d2e-507e4fe87c18",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Orgg"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature can't attack if defending player controls an untapped creature with \
             power 3 or greater.",
            // "Defending player controls an untapped creature with power 3 or greater."
            EffectDef::CannotAttackIf(&ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(3),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            )),
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
// Audit: unsupported — Needs a persistent tap/untap restriction or event relation for “Sacrifice an Orc: Each player may tap any number of untapped white creatures they control. For each creature tapped this way, that player chooses up to two Plains. Then destroy all…”.
pub(in crate::card::sets) static RAIDING_PARTY: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Raiding Party",
    "907a3396-706b-4ca2-9973-bca758986032",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// FEM 65a — Elven Fortress
pub(in crate::card::sets) static ELVEN_FORTRESS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Elven Fortress",
    "9387105d-46d0-4db0-8980-dd0fded15eef",
    "Pete Venters",
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
const ELVEN_FORTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_FORTRESS,
    1,
    "091b5ed4-91f5-47c1-b1a1-5443f7346078",
    "Randy Asplund-Faith",
);

// FEM 65c — Elven Fortress (alternate printing)
const ELVEN_FORTRESS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_FORTRESS,
    2,
    "960b542f-cb24-4f74-92da-d31559d87c2d",
    "Mark Poole",
);

// FEM 65d — Elven Fortress (alternate printing)
const ELVEN_FORTRESS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ELVEN_FORTRESS,
    3,
    "c52743f0-5c5b-46b9-bbbd-67950d4c89e5",
    "Tom Wänerstrand",
);

// FEM 66 — Elvish Farmer
pub(in crate::card::sets) static ELVISH_FARMER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Elvish Farmer",
    "40a9710e-b2f8-4746-8640-d450f58a6e49",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf"], 0, 2).with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a spore counter on this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("spore"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
                &[AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("spore"),
                    amount: 3,
                }],
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
pub(in crate::card::sets) static ELVISH_HUNTER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Elvish Hunter",
    "e00455ac-c7ce-4916-98ed-cca9354e3f22",
    "Mark Poole",
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
const ELVISH_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_HUNTER,
    1,
    "51ff096c-487f-42f9-a394-a298503391da",
    "Anson Maddocks",
);

// FEM 67c — Elvish Hunter (alternate printing)
const ELVISH_HUNTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_HUNTER,
    2,
    "204c8aff-b103-4606-b86b-d794bc5dcde1",
    "Susan Van Camp",
);

// FEM 68a — Elvish Scout
pub(in crate::card::sets) static ELVISH_SCOUT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Elvish Scout",
    "689cd2ed-be81-4769-a8ec-287946301396",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}: Untap target attacking creature you control. Prevent all combat damage \
             that would be dealt to and dealt by it this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            // The two shields are one printed clause but two rules: prevention names a
            // source or a recipient, never both at once, so "to and dealt by it" is the
            // creature on each side in turn.
            EffectDef::Sequence(&[
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
            ]),
        ),
    ),
);

// FEM 68b — Elvish Scout (alternate printing)
const ELVISH_SCOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_SCOUT,
    1,
    "1faff88d-594e-473c-a2d1-cd60f51b2ee7",
    "Pete Venters",
);

// FEM 68c — Elvish Scout (alternate printing)
const ELVISH_SCOUT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_SCOUT,
    2,
    "d414bf5a-2604-426c-8c68-5c1696557b57",
    "Christopher Rush",
);

// FEM 69 — Feral Thallid
pub(in crate::card::sets) static FERAL_THALLID: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Feral Thallid",
    "e585241e-c647-456d-b3b1-3d48dd78c372",
    "Rob Alexander",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Fungus"], 6, 3).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Regenerate this creature.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// FEM 70 — Fungal Bloom
pub(in crate::card::sets) static FUNGAL_BLOOM: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Fungal Bloom",
    "cf1a2cb2-9a6b-41f7-96f7-ec457c69c16c",
    "Daniel Gelon",
    CardRules::new_enchantment(mana_cost!("{G}{G}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}: Put a spore counter on target Fungus.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Fungus"),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FEM 71a — Night Soil
// Audit: unsupported — Needs a zone-object query and identity-preserving continuation for “{1}, Exile two creature cards from a single graveyard: Create a 1/1 green Saproling creature token”.
pub(in crate::card::sets) static NIGHT_SOIL: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Night Soil",
    "4cda6d18-d4b1-4b8a-a72e-f90115adf4c3",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// FEM 71b — Night Soil (alternate printing)
const NIGHT_SOIL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NIGHT_SOIL,
    1,
    "4f25a497-46dc-47aa-8586-d514578a6d25",
    "Heather Hudson",
);

// FEM 71c — Night Soil (alternate printing)
const NIGHT_SOIL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NIGHT_SOIL,
    2,
    "ee3eb61b-698c-42b1-8a33-0ce7c3829e07",
    "Drew Tucker",
);

// FEM 72a — Spore Cloud
pub(in crate::card::sets) static SPORE_CLOUD: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Spore Cloud",
    "1691a9f4-4ea7-440f-9bdc-4214ab3c90f0",
    "Susan Van Camp",
    CardRules::new_instant(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell(
        "Tap all blocking creatures. Prevent all combat damage that would be dealt this turn. \
         Each attacking creature and each blocking creature doesn't untap during its \
         controller's next untap step.",
        // Three clauses in printed order. The tap comes first so it reaches the
        // blockers while they are still blocking; the skip is separate from it,
        // because a creature already tapped still owes the untap step it misses.
        EffectDef::Sequence(&[
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
        ]),
    )),
);

// FEM 72b — Spore Cloud (alternate printing)
const SPORE_CLOUD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPORE_CLOUD,
    1,
    "2c3070f8-6dae-4f22-b186-e2a3a9647cc5",
    "Jesper Myrfors",
);

// FEM 72c — Spore Cloud (alternate printing)
const SPORE_CLOUD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPORE_CLOUD,
    2,
    "17fe098c-c9b5-4bba-92b5-5720d6919073",
    "Amy Weber",
);

// FEM 73 — Spore Flower
pub(in crate::card::sets) static SPORE_FLOWER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Spore Flower",
    "f9681dc0-d0fc-4d5b-a23c-63ec1cc8343d",
    "Margaret Organ-Kean",
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
                    kind: CounterKind::named("spore"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Prevent all combat damage that would be dealt this turn.",
                &[AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("spore"),
                    amount: 3,
                }],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FEM 74a — Thallid
pub(in crate::card::sets) static THALLID: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Thallid",
    "4caaf31b-86a9-485b-8da7-d5b526ed1233",
    "Edward P. Beard, Jr.",
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
                    kind: CounterKind::named("spore"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
                &[AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("spore"),
                    amount: 3,
                }],
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
const THALLID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THALLID,
    1,
    "80f8f778-ae31-45cd-b27f-f93a07853ede",
    "Jesper Myrfors",
);

// FEM 74c — Thallid (alternate printing)
const THALLID_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THALLID,
    2,
    "2cf2f3da-9101-439d-8caa-910ff40bfbb3",
    "Ron Spencer",
);

// FEM 74d — Thallid (alternate printing)
const THALLID_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THALLID,
    3,
    "01827286-b104-41c5-bac9-7c38414bc40e",
    "Daniel Gelon",
);

// FEM 75 — Thallid Devourer
pub(in crate::card::sets) static THALLID_DEVOURER: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Thallid Devourer",
    "aa533845-4c4b-4072-aa39-8e56ce7ec325",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Fungus"], 2, 2).with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a spore counter on this creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("spore"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
                &[AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("spore"),
                    amount: 3,
                }],
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
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “Whenever a player puts a Swamp onto the battlefield, this enchantment deals 3 damage to that player unless the player puts a -1/-1 counter on a creature they control”.
pub(in crate::card::sets) static THELON_S_CHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Thelon's Chant",
    "9d970195-0a09-4cb4-a2c0-c16fcab5c859",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// FEM 77 — Thelon's Curse
// Audit: unsupported — Needs a persistent tap/untap restriction or event relation for “At the beginning of each player's upkeep, that player may choose any number of tapped blue creatures they control and pay {U} for each creature chosen this way. If the player does, untap…”.
pub(in crate::card::sets) static THELON_S_CURSE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Thelon's Curse",
    "9b868846-cc3c-4756-a5dd-2335bb380567",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// FEM 78 — Thelonite Druid
pub(in crate::card::sets) static THELONITE_DRUID: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Thelonite Druid",
    "cd8772dd-513d-4dd0-a5db-5214dc8da4e0",
    "Margaret Organ-Kean",
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
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_card_types(crate::card::CardTypeSet::single(CardType::Creature)),
                AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(3)),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// FEM 79 — Thelonite Monk
// Audit: unsupported — Needs its permanent-duration target-land characteristic effect and green-creature sacrifice cost authored and tested.
pub(in crate::card::sets) static THELONITE_MONK: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Thelonite Monk",
    "5400ff25-c70e-4095-a228-190601b86043",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// FEM 80a — Thorn Thallid
pub(in crate::card::sets) static THORN_THALLID: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Thorn Thallid",
    "16e61c00-3e94-4f6f-8515-65b430829e91",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Fungus"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "Remove three spore counters from this creature: It deals 1 damage to any target.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
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
const THORN_THALLID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORN_THALLID,
    1,
    "84283348-789b-4236-b406-7fc6338a867d",
    "Heather Hudson",
);

// FEM 80c — Thorn Thallid (alternate printing)
const THORN_THALLID_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THORN_THALLID,
    2,
    "1537a338-3b68-4a41-bac6-554e8e530e46",
    "Jesper Myrfors",
);

// FEM 80d — Thorn Thallid (alternate printing)
const THORN_THALLID_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THORN_THALLID,
    3,
    "1e8f50be-1629-40eb-8916-019903d2e6a4",
    "Mark Tedin",
);

// FEM 81 — Aeolipile
pub(in crate::card::sets) static AEOLIPILE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Aeolipile",
    "a09030ee-415c-45af-bf08-7623197a314f",
    "Heather Hudson",
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
// Audit: unsupported — Needs modal activated abilities: modes are chosen only while casting a spell, so an activated ability has no mode selection to freeze. Both of its modes are available.
pub(in crate::card::sets) static BALM_OF_RESTORATION: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Balm of Restoration",
    "7f95de4a-7fae-42bc-9660-39ea7685ca02",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// FEM 83 — Conch Horn
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “{1}, {T}, Sacrifice this artifact: Draw two cards, then put a card from your hand on top of your library”.
pub(in crate::card::sets) static CONCH_HORN: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Conch Horn",
    "860a9ba3-e4c4-4af9-bdfe-1ada39289fd5",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// FEM 84 — Delif's Cone
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “{T}, Sacrifice this artifact: This turn, when target creature you control attacks and isn't blocked, you may gain life equal to its power. If you do, it assigns no combat damage this turn”.
pub(in crate::card::sets) static DELIF_S_CONE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Delif's Cone",
    "262b8788-c5a0-4c8e-9d58-b769b1b0a2ff",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// FEM 85 — Delif's Cube
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “{2}, {T}: This turn, when target creature you control attacks and isn't blocked, it assigns no combat damage this turn and you put a cube counter on this artifact”.
pub(in crate::card::sets) static DELIF_S_CUBE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Delif's Cube",
    "14749600-9eca-4122-b04f-30ddda091b74",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// FEM 86 — Draconian Cylix
pub(in crate::card::sets) static DRACONIAN_CYLIX: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Draconian Cylix",
    "a419c9e3-5615-44f9-9256-94a3022bb69f",
    "Edward P. Beard, Jr.",
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
pub(in crate::card::sets) static ELVEN_LYRE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Elven Lyre",
    "c3a8cd72-04c0-46f7-a249-f1cecddfdc26",
    "Kaja Foglio",
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
pub(in crate::card::sets) static IMPLEMENTS_OF_SACRIFICE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Implements of Sacrifice",
    "aa5deb95-79a6-4398-b82a-c1df169550d9",
    "Margaret Organ-Kean",
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
pub(in crate::card::sets) static RING_OF_RENEWAL: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Ring of Renewal",
    "a532d38a-809b-4132-8690-be15fe23afab",
    "Douglas Shuler",
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
pub(in crate::card::sets) static SPIRIT_SHIELD: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Spirit Shield",
    "213d6e0d-5ec9-441e-a38d-50ce44583e4b",
    "Scott Kirschner",
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
pub(in crate::card::sets) static ZELYON_SWORD: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Zelyon Sword",
    "4137160b-5248-4fbd-8ae8-25e9afd8fb5c",
    "Scott Kirschner",
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
pub(in crate::card::sets) static BOTTOMLESS_VAULT: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Bottomless Vault",
    "639ae988-d1d1-4ead-b0f8-47fc39eb64a0",
    "Pat Lewis",
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
                kind: CounterKind::named("storage"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {B} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::named("storage")),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// FEM 93 — Dwarven Hold
pub(in crate::card::sets) static DWARVEN_HOLD: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Dwarven Hold",
    "a3142ded-ff62-4817-aa54-75a7ea4498a6",
    "Pat Lewis",
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
                kind: CounterKind::named("storage"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {R} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::named("storage")),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// FEM 94 — Dwarven Ruins
pub(in crate::card::sets) static DWARVEN_RUINS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Dwarven Ruins",
    "0dfe1352-27be-4c99-a58f-b961f911f270",
    "Mark Poole",
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
pub(in crate::card::sets) static EBON_STRONGHOLD: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Ebon Stronghold",
    "3fb2a11f-a8e4-4acf-871a-11171e3304ef",
    "Mark Poole",
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
pub(in crate::card::sets) static HAVENWOOD_BATTLEGROUND: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Havenwood Battleground",
    "9028f200-80dd-4c53-877f-ea380ff417cb",
    "Mark Poole",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
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
pub(in crate::card::sets) static HOLLOW_TREES: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Hollow Trees",
    "90845410-e09a-4753-ad4c-bf2b2f3c95ac",
    "Pat Lewis",
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
                kind: CounterKind::named("storage"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {G} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::named("storage")),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// FEM 98 — Icatian Store
pub(in crate::card::sets) static ICATIAN_STORE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Icatian Store",
    "d7cd8d8c-52c7-402f-92e1-5e5866f2555a",
    "Pat Lewis",
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
                kind: CounterKind::named("storage"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {W} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::named("storage")),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ]),
);

// FEM 99 — Rainbow Vale
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “{T}: Add one mana of any color. An opponent gains control of this land at the beginning of the next end step”.
pub(in crate::card::sets) static RAINBOW_VALE: CardRecord = CardRecord::new(
    crate::card::CardSet::FallenEmpires,
    "Rainbow Vale",
    "c1b138e1-f8fc-435c-9aed-98004768479c",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// FEM 100 — Ruins of Trokair
pub(in crate::card::sets) static RUINS_OF_TROKAIR: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Ruins of Trokair",
    "4ce2e734-8cff-4bfe-85f8-17b3e1903f18",
    "Mark Poole",
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
pub(in crate::card::sets) static SAND_SILOS: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Sand Silos",
    "3f6f1fcb-d903-4a31-abab-40488569eef6",
    "Pat Lewis",
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
                kind: CounterKind::named("storage"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Remove any number of storage counters from this land: Add {U} for each storage counter removed this way.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveAnyNumberOfCountersFromSource(CounterKind::named("storage")),
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// FEM 102 — Svyelunite Temple
pub(in crate::card::sets) static SVYELUNITE_TEMPLE: CardRecord = CardRecord::new(
    CardSet::FallenEmpires,
    "Svyelunite Temple",
    "8b3fde62-ab21-459b-9c5d-01aa6fe1d08e",
    "Mark Poole",
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
    COMBAT_MEDIC_ALTERNATE_1,
    COMBAT_MEDIC_ALTERNATE_2,
    COMBAT_MEDIC_ALTERNATE_3,
    FARRELS_ZEALOT_ALTERNATE_1,
    FARRELS_ZEALOT_ALTERNATE_2,
    ICATIAN_INFANTRY_ALTERNATE_1,
    ICATIAN_INFANTRY_ALTERNATE_2,
    ICATIAN_INFANTRY_ALTERNATE_3,
    ICATIAN_JAVELINEERS_ALTERNATE_1,
    ICATIAN_JAVELINEERS_ALTERNATE_2,
    ICATIAN_MONEYCHANGER_ALTERNATE_1,
    ICATIAN_MONEYCHANGER_ALTERNATE_2,
    ICATIAN_SCOUT_ALTERNATE_1,
    ICATIAN_SCOUT_ALTERNATE_2,
    ICATIAN_SCOUT_ALTERNATE_3,
    ORDER_OF_LEITBUR_ALTERNATE_1,
    ORDER_OF_LEITBUR_ALTERNATE_2,
    HIGH_TIDE_ALTERNATE_1,
    HIGH_TIDE_ALTERNATE_2,
    HOMARID_ALTERNATE_1,
    HOMARID_ALTERNATE_2,
    HOMARID_ALTERNATE_3,
    HOMARID_WARRIOR_ALTERNATE_1,
    HOMARID_WARRIOR_ALTERNATE_2,
    MERSEINE_ALTERNATE_1,
    MERSEINE_ALTERNATE_2,
    MERSEINE_ALTERNATE_3,
    TIDAL_FLATS_ALTERNATE_1,
    TIDAL_FLATS_ALTERNATE_2,
    VODALIAN_MAGE_ALTERNATE_1,
    VODALIAN_MAGE_ALTERNATE_2,
    VODALIAN_SOLDIERS_ALTERNATE_1,
    VODALIAN_SOLDIERS_ALTERNATE_2,
    VODALIAN_SOLDIERS_ALTERNATE_3,
    ARMOR_THRULL_ALTERNATE_1,
    ARMOR_THRULL_ALTERNATE_2,
    ARMOR_THRULL_ALTERNATE_3,
    BASAL_THRULL_ALTERNATE_1,
    BASAL_THRULL_ALTERNATE_2,
    BASAL_THRULL_ALTERNATE_3,
    HYMN_TO_TOURACH_ALTERNATE_1,
    HYMN_TO_TOURACH_ALTERNATE_2,
    HYMN_TO_TOURACH_ALTERNATE_3,
    INITIATES_OF_THE_EBON_HAND_ALTERNATE_1,
    INITIATES_OF_THE_EBON_HAND_ALTERNATE_2,
    MINDSTAB_THRULL_ALTERNATE_1,
    MINDSTAB_THRULL_ALTERNATE_2,
    NECRITE_ALTERNATE_1,
    NECRITE_ALTERNATE_2,
    ORDER_OF_THE_EBON_HAND_ALTERNATE_1,
    ORDER_OF_THE_EBON_HAND_ALTERNATE_2,
    BRASSCLAW_ORCS_ALTERNATE_1,
    BRASSCLAW_ORCS_ALTERNATE_2,
    BRASSCLAW_ORCS_ALTERNATE_3,
    DWARVEN_SOLDIER_ALTERNATE_1,
    DWARVEN_SOLDIER_ALTERNATE_2,
    GOBLIN_CHIRURGEON_ALTERNATE_1,
    GOBLIN_CHIRURGEON_ALTERNATE_2,
    GOBLIN_GRENADE_ALTERNATE_1,
    GOBLIN_GRENADE_ALTERNATE_2,
    GOBLIN_WAR_DRUMS_ALTERNATE_1,
    GOBLIN_WAR_DRUMS_ALTERNATE_2,
    GOBLIN_WAR_DRUMS_ALTERNATE_3,
    ORCISH_SPY_ALTERNATE_1,
    ORCISH_SPY_ALTERNATE_2,
    ORCISH_VETERAN_ALTERNATE_1,
    ORCISH_VETERAN_ALTERNATE_2,
    ORCISH_VETERAN_ALTERNATE_3,
    ELVEN_FORTRESS_ALTERNATE_1,
    ELVEN_FORTRESS_ALTERNATE_2,
    ELVEN_FORTRESS_ALTERNATE_3,
    ELVISH_HUNTER_ALTERNATE_1,
    ELVISH_HUNTER_ALTERNATE_2,
    ELVISH_SCOUT_ALTERNATE_1,
    ELVISH_SCOUT_ALTERNATE_2,
    NIGHT_SOIL_ALTERNATE_1,
    NIGHT_SOIL_ALTERNATE_2,
    SPORE_CLOUD_ALTERNATE_1,
    SPORE_CLOUD_ALTERNATE_2,
    THALLID_ALTERNATE_1,
    THALLID_ALTERNATE_2,
    THALLID_ALTERNATE_3,
    THORN_THALLID_ALTERNATE_1,
    THORN_THALLID_ALTERNATE_2,
    THORN_THALLID_ALTERNATE_3,
];
