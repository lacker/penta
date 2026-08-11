use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardBehavior, CardRules, CardSet, CardSupertype,
    CardType, CardTypeSet, ComparisonDef, CounterKind, EffectDef, EffectDurationDef,
    EffectExecutionDef, EffectRecipientDef, LibraryPlacement, ManaColor, ObjectPredicateDef,
    PlayerRelation, ReplacementEventDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

pub(in crate::card::sets) static ANKH_OF_MISHRA: CardRecord = CardRecord::new(
    cards::ANKH_OF_MISHRA,
    "Ankh of Mishra",
    CardArt::new("f594b7aa-d44e-47c4-989b-565f881e25f1", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a land enters, this artifact deals 2 damage to that land's controller.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::HasType(CardType::Land),
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(2),
        },
    )]),
);

pub(in crate::card::sets) static BLACK_VISE: CardRecord = CardRecord::new(
    cards::BLACK_VISE,
    "Black Vise",
    CardArt::new("76ac72f8-5b1e-4d67-a796-ef69cde27424", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::replacement(
            "As this artifact enters, choose an opponent.",
            EffectDef::ChoosePlayer {
                object: EffectRecipientDef::Source,
                relation: PlayerRelation::Opponent,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of the chosen player's upkeep, this artifact deals X damage to that player, where X is the number of cards in their hand minus 4.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::ChosenPlayer,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::CardsInHandAbove {
                    player: PlayerRelation::EventPlayer,
                    threshold: 4,
                },
            },
        ),
    ]),
);

pub(in crate::card::sets) static COPPER_TABLET: CardRecord = CardRecord::new(
    cards::COPPER_TABLET,
    "Copper Tablet",
    CardArt::new("30935e4a-013e-4c46-ad05-304df8e5dfa4", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::triggered(
        "At the beginning of each player's upkeep, this artifact deals 1 damage to that player.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(1),
        },
    )]),
);

pub(in crate::card::sets) static FIREBALL: CardRecord = CardRecord::new(
    cards::FIREBALL,
    "Fireball",
    CardArt::new("b7623c00-144b-4a8f-9c6c-f5e9e4f65ece", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{R}"))
    .costs_more_per_extra_target(1)
    .with_abilities(&[
        AbilityDef::enforced_when_cast(
            "This spell costs {1} more to cast for each target beyond the first.",
            "The play option adds the generic cost before the spell is offered, \
             so an unaffordable spread of targets is never a legal action.",
        ),
        AbilityDef::custom_full(
            "Fireball deals X damage divided evenly, rounded down, among any number of targets.",
            CardBehavior::Fireball,
            "The card-local selector offers every combination of damage targets, including none, and the resolver divides X by the count it was cast with rather than by the targets that survive.",
        ),
    ]),
);

pub(in crate::card::sets) static FORK: CardRecord = CardRecord::new(
    cards::FORK,
    "Fork",
    CardArt::new("e6b43916-fe2d-417a-a550-d7c795023297", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{R}{R}")).with_abilities(&[
        AbilityDef::custom_partial(
            "Copy target instant or sorcery spell, except that the copy is red. You may choose new targets for the copy.",
            CardBehavior::Fork,
            "Choosing new targets for the copy is offered as a single ordered decision rather than slot by slot.",
        ),
    ]),
);

pub(in crate::card::sets) static GLASSES_OF_URZA: CardRecord = CardRecord::new(
    cards::GLASSES_OF_URZA,
    "Glasses of Urza",
    CardArt::new("cafc2350-5d64-4379-9198-79a114654d45", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Look at target player's hand.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::LookAtHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

pub(in crate::card::sets) static IRON_STAR: CardRecord = CardRecord::new(
    cards::IRON_STAR,
    "Iron Star",
    CardArt::new("5786de12-cade-43c2-a6b0-0c5b294b9d0e", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a red spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Red)),
        EffectDef::OptionalManaPayment {
            cost: mana_cost!("{1}"),
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

pub(in crate::card::sets) static LIGHTNING_BOLT: CardRecord = CardRecord::new(
    cards::LIGHTNING_BOLT,
    "Lightning Bolt",
    CardArt::new("d573ef03-4730-45aa-93dd-e45ac1dbaf4a", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Lightning Bolt deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )]),
);

pub(in crate::card::sets) static MOUNTAIN: CardRecord = CardRecord::new(
    cards::MOUNTAIN,
    "Mountain",
    CardArt::new("eace2c85-976c-425e-9800-5a6ccbd91b56", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_land(&["Mountain"]).with_supertype(CardSupertype::Basic),
);

pub(in crate::card::sets) static RED_ELEMENTAL_BLAST: CardRecord = CardRecord::new(
    cards::RED_ELEMENTAL_BLAST,
    "Red Elemental Blast",
    CardArt::new("776ad9be-3309-4f1d-9f27-6219d9477662", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target blue spell.\n• Destroy target blue permanent.",
        &[
            AbilityDef::counter_target(
                "Counter target blue spell",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Blue)),
            ),
            AbilityDef::destroy_target(
                "Destroy target blue permanent",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Blue,
                )),
                true,
            ),
        ],
    )),
);

pub(in crate::card::sets) static SHATTER: CardRecord = CardRecord::new(
    cards::SHATTER,
    "Shatter",
    CardArt::new("50dc7fc1-cb6a-4c68-b993-1a25cf16226e", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )]),
);

pub(in crate::card::sets) static SMOKE: CardRecord = CardRecord::new(
    cards::SMOKE,
    "Smoke",
    CardArt::new("7c67788e-d713-47c3-ab9f-b8a6212ae24f", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{R}{R}")).with_abilities(&[AbilityDef::custom_full(
        "Players can't untap more than one creature during their untap steps.",
        CardBehavior::Smoke,
        "The untap restriction is implemented by the shared untap procedure.",
    )]),
);

/// The Giant throws a creature small enough to lift, and it does not survive
/// the landing. "Toughness less than this creature's power" is read against
/// the Giant as it is now, so pumping it widens the choice.
static STONE_GIANT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ToughnessLessThan(ValueDef::SourcePower),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

static STONE_GIANT_THROW: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::GrantAbility(&STONE_GIANT_FLYING),
        duration: EffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
        effect: &EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    },
];

static STONE_GIANT_FLYING: AbilityDef = abilities::flying();

pub(in crate::card::sets) static STONE_GIANT: CardRecord = CardRecord::new(
    cards::STONE_GIANT,
    "Stone Giant",
    CardArt::new("7ffaedb9-25f8-4304-9085-e12505b93312", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 3, 4)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{T}: Target creature you control with toughness less than this creature's power gains flying until end of turn. Destroy that creature at the beginning of the next end step.",
                &[AbilityCostDef::TapSource],
                &STONE_GIANT_TARGET,
                EffectDef::Sequence(&STONE_GIANT_THROW),
            ),
        ]),
);

pub(in crate::card::sets) static WINTER_ORB: CardRecord = CardRecord::new(
    cards::WINTER_ORB,
    "Winter Orb",
    CardArt::new("9359f60c-9a27-4e53-b35b-964a121a6fba", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::custom_full(
            "As long as this artifact is untapped, players can't untap more than one land during their untap steps.",
            CardBehavior::WinterOrb,
            "The conditional untap restriction is implemented by the shared untap procedure.",
        ),
    ]),
);

pub(in crate::card::sets) static BLACK_LOTUS: CardRecord = CardRecord::new(
    cards::BLACK_LOTUS,
    "Black Lotus",
    CardArt::new("b0faa7f2-b547-42c4-a810-839da50dadfe", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}, Sacrifice this artifact: Add three mana of any one color.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::AddMana(
            AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])
            .with_amount(3),
        ),
    )]),
);

pub(in crate::card::sets) static CHAOS_ORB: CardRecord = CardRecord::new(
    cards::CHAOS_ORB,
    "Chaos Orb",
    CardArt::new("92274971-7c4a-4326-b0fe-75e2d124f718", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_abilities(&[
            AbilityDef::activated_with_targets("{1}, {T}: If this artifact is on the battlefield, flip it onto the battlefield from a height of at least one foot. If this artifact turns over completely at least once during the flip, destroy all nontoken permanents it touches. Then destroy this artifact.", &[
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::TapSource,
                ], &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )], EffectDef::Special("Resolve the deterministic Chaos Orb approximation"))
            .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::ChaosOrb))
            .with_coverage(AbilityCoverageDef::partial(
                "The engine uses a deterministic chosen-permanent approximation rather than the physical flip procedure.",
            ))
            .with_legacy_procedure(),
        ]),
);

/// The fourth activation is the one that kills it, and the count includes
/// the activation now resolving.
static DRAGON_WHELP_PUMP: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::ModifyPowerToughness {
            power: ValueDef::Constant(1),
            toughness: ValueDef::Constant(0),
        },
        duration: EffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceActivationsThisTurn {
            comparison: ComparisonDef::AtLeast,
            amount: 4,
        },
        then: &EffectDef::AtNextStep {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
            effect: &EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        },
    },
];

pub(in crate::card::sets) static DRAGON_WHELP: CardRecord = CardRecord::new(
    cards::DRAGON_WHELP,
    "Dragon Whelp",
    CardArt::new("6bbf1eab-bc32-4835-b566-8634b1fe81b0", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 2, 3)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated(
                "{R}: This creature gets +1/+0 until end of turn. If this ability has been activated four or more times this turn, sacrifice this creature at the beginning of the next end step.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Sequence(&DRAGON_WHELP_PUMP),
            ),
        ]),
);

pub(in crate::card::sets) static GOBLIN_BALLOON_BRIGADE: CardRecord = CardRecord::new(
    cards::GOBLIN_BALLOON_BRIGADE,
    "Goblin Balloon Brigade",
    CardArt::new("5129b422-7a35-4bc5-b14b-c814012a0d8f", "Andi Rusu"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{R}: This creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::flying()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static GOBLIN_KING: CardRecord = CardRecord::new(
    cards::GOBLIN_KING,
    "Goblin King",
    CardArt::new("5873672d-37ea-4c0f-97f3-12b74fde112d", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Goblins get +1/+1 and have mountainwalk.",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype("Goblin"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Any,
                    },
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype("Goblin"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Any,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ]),
        ),
    ]),
);

pub(in crate::card::sets) static GRANITE_GARGOYLE: CardRecord = CardRecord::new(
    cards::GRANITE_GARGOYLE,
    "Granite Gargoyle",
    CardArt::new("f15bf2b2-6848-4fbd-b89a-8d8da8ae1cdc", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Gargoyle"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static IRONCLAW_ORCS: CardRecord = CardRecord::new(
    cards::IRONCLAW_ORCS,
    "Ironclaw Orcs",
    CardArt::new("d56421a8-34ae-4033-943f-c59a7bf2b6f9", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Orc"], 2, 2).with_abilities(&[
        AbilityDef::custom_full(
            "This creature can't block creatures with power 2 or greater.",
            CardBehavior::IronclawOrcs,
            "The blocking restriction is implemented by the combat action generator.",
        ),
    ]),
);

pub(in crate::card::sets) static MOX_EMERALD: CardRecord = CardRecord::new(
    cards::MOX_EMERALD,
    "Mox Emerald",
    CardArt::new("b0e1427c-05cd-465b-be59-97ed6e39f7ba", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

pub(in crate::card::sets) static MOX_JET: CardRecord = CardRecord::new(
    cards::MOX_JET,
    "Mox Jet",
    CardArt::new("92bcd1ce-19b1-4d78-8b09-95242ca08d76", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Black)]),
);

pub(in crate::card::sets) static MOX_PEARL: CardRecord = CardRecord::new(
    cards::MOX_PEARL,
    "Mox Pearl",
    CardArt::new("8ebe4be7-e12a-4596-a899-fbd5b152e879", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::White)]),
);

pub(in crate::card::sets) static MOX_RUBY: CardRecord = CardRecord::new(
    cards::MOX_RUBY,
    "Mox Ruby",
    CardArt::new("8945585f-4773-493d-a0fe-d707db910b38", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Red)]),
);

pub(in crate::card::sets) static MOX_SAPPHIRE: CardRecord = CardRecord::new(
    cards::MOX_SAPPHIRE,
    "Mox Sapphire",
    CardArt::new("82da0972-b17b-4600-9efd-e9430a0db04b", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Blue)]),
);

pub(in crate::card::sets) static SOL_RING: CardRecord = CardRecord::new(
    cards::SOL_RING,
    "Sol Ring",
    CardArt::new("c4300d24-1cae-4dd5-be7e-38cc677cf5bd", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}: Add {C}{C}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
    )]),
);

pub(in crate::card::sets) static WHEEL_OF_FORTUNE: CardRecord = CardRecord::new(
    cards::WHEEL_OF_FORTUNE,
    "Wheel of Fortune",
    CardArt::new("67b369c4-faa8-45c8-a1b9-98f228b69682", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::custom_full(
            "Each player discards their hand, then draws seven cards.",
            CardBehavior::WheelOfFortune,
            "The card-local resolver discards both hands and then draws with empty-library losses settled together, so one spell decking both players is a draw.",
        ),
    ]),
);

pub(in crate::card::sets) static JUGGERNAUT: CardRecord = CardRecord::new(
    cards::JUGGERNAUT,
    "Juggernaut",
    CardArt::new("dcd6a291-5282-4f49-8203-d9b416083c48", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Juggernaut"], 5, 3).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::static_ability(
            "This creature can't be blocked by Walls.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Subtype("Wall")),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

pub(in crate::card::sets) static WALL_OF_STONE: CardRecord = CardRecord::new(
    cards::WALL_OF_STONE,
    "Wall of Stone",
    CardArt::new("f7fd8b8e-98fd-4b0d-8bb9-06bd25a1e30f", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Wall"], 0, 8)
        .with_abilities(&[abilities::defender()]),
);

pub(in crate::card::sets) static MANA_VAULT: CardRecord = CardRecord::new(
    cards::MANA_VAULT,
    "Mana Vault",
    CardArt::new("19499cb7-eccb-4e69-af32-6002d447a160", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::Special("Keep this permanent tapped during its controller's untap step"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::ManaVault))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The untap restriction is implemented by the shared untap procedure.",
        )),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {4}. If you do, untap this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Special("Choose whether to pay 4 to untap this permanent"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::ManaVaultUntap))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The trigger uses the shared stack; the card-local resolver offers the payment as it resolves.",
        )),
        AbilityDef::triggered(
            "At the beginning of your draw step, if this artifact is tapped, it deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Draw,
                player: PlayerRelation::You,
            },
            EffectDef::Special("If this permanent is tapped, deal 1 damage to its controller"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::ManaVaultDamage))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The trigger uses the shared stack; the card-local resolver re-reads whether the artifact is still tapped as it resolves.",
        )),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Colorless).with_amount(3),
            ),
        ),
    ]),
);

pub(in crate::card::sets) static ANCESTRAL_RECALL: CardRecord = CardRecord::new(
    cards::ANCESTRAL_RECALL,
    "Ancestral Recall",
    CardArt::new("70e7ddf2-5604-41e7-bb9d-ddd03d3e9d0b", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws three cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )]),
);

pub(in crate::card::sets) static BRAINGEYSER: CardRecord = CardRecord::new(
    cards::BRAINGEYSER,
    "Braingeyser",
    CardArt::new("62b19a12-6914-430e-81ce-dcfca47884df", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player draws X cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

pub(in crate::card::sets) static COUNTERSPELL: CardRecord = CardRecord::new(
    cards::COUNTERSPELL,
    "Counterspell",
    CardArt::new("0df55e3f-14de-46ef-b6b1-616618724d9e", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
        },
    )]),
);

pub(in crate::card::sets) static DISENCHANT: CardRecord = CardRecord::new(
    cards::DISENCHANT,
    "Disenchant",
    CardArt::new("2722d7e2-61c6-4934-9c21-875ee78fd06c", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )]),
);

pub(in crate::card::sets) static ISLAND: CardRecord = CardRecord::new(
    cards::ISLAND,
    "Island",
    CardArt::new("90a57c0e-fa61-45ef-955d-d296403967d5", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_land(&["Island"]).with_supertype(CardSupertype::Basic),
);

pub(in crate::card::sets) static JAYEMDAE_TOME: CardRecord = CardRecord::new(
    cards::JAYEMDAE_TOME,
    "Jayemdae Tome",
    CardArt::new("cac8c421-5b92-481d-b2de-560c0231ab58", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[AbilityDef::activated(
            "{4}, {T}: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )]),
);

pub(in crate::card::sets) static PLAINS: CardRecord = CardRecord::new(
    cards::PLAINS,
    "Plains",
    CardArt::new("b1623d57-4729-4796-b3f7-f1837a05c6ed", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains"]).with_supertype(CardSupertype::Basic),
);

pub(in crate::card::sets) static SERRA_ANGEL: CardRecord = CardRecord::new(
    cards::SERRA_ANGEL,
    "Serra Angel",
    CardArt::new("f8ac5006-91bd-4803-93da-f87cf196dd2f", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 4, 4)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

pub(in crate::card::sets) static SWORDS_TO_PLOWSHARES: CardRecord = CardRecord::new(
    cards::SWORDS_TO_PLOWSHARES,
    "Swords to Plowshares",
    CardArt::new("386ea9eb-abc1-4862-aa2d-8fb808d79490", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature. Its controller gains life equal to its power.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: LibraryPlacement::Top,
                controller: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

pub(in crate::card::sets) static TIME_WALK: CardRecord = CardRecord::new(
    cards::TIME_WALK,
    "Time Walk",
    CardArt::new("e0139f60-d48e-46fb-9f5a-1e3d7558c834", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::custom_full(
        "Take an extra turn after this one.",
        CardBehavior::TimeWalk,
        "The extra turn is implemented by the card-local spell resolver.",
    )]),
);

pub(in crate::card::sets) static TUNDRA: CardRecord = CardRecord::new(
    cards::TUNDRA,
    "Tundra",
    CardArt::new("a03e8c5b-f4ed-4fd7-ba05-db813ccc05eb", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains", "Island"]),
);

pub(in crate::card::sets) static ARMAGEDDON: CardRecord = CardRecord::new(
    cards::ARMAGEDDON,
    "Armageddon",
    CardArt::new("5b6ddce7-b9c5-431d-a0b0-46d4aa93cbcb", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell(
        "Destroy all lands.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )]),
);

pub(in crate::card::sets) static BADLANDS: CardRecord = CardRecord::new(
    cards::BADLANDS,
    "Badlands",
    CardArt::new("717f6d10-9144-4ade-9ac6-a481cc66b875", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp", "Mountain"]),
);

pub(in crate::card::sets) static BALANCE: CardRecord = CardRecord::new(
    cards::BALANCE,
    "Balance",
    CardArt::new("6f9ea46a-411f-40ce-a873-a905180093f4", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{W}"))
    .with_abilities(&[AbilityDef::custom_full(
        "Each player chooses a number of lands they control equal to the number of lands controlled by the player who controls the fewest, then sacrifices the rest. Players discard cards and sacrifice creatures the same way.",
        CardBehavior::Balance,
        "The card-local resolver settles lands, then hands, then creatures, recounting before each. Only whoever is over the shared floor chooses, so a phase never has two choosers whose picks could leak to each other.",
    )]),
);

pub(in crate::card::sets) static BAYOU: CardRecord = CardRecord::new(
    cards::BAYOU,
    "Bayou",
    CardArt::new("412ceddd-2b9a-4551-a6bf-ae2830a2010a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp", "Forest"]),
);

pub(in crate::card::sets) static BLACK_KNIGHT: CardRecord = CardRecord::new(
    cards::BLACK_KNIGHT,
    "Black Knight",
    CardArt::new("c1662949-0d69-49a3-8c69-daf10717ed4e", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from(ManaColor::White),
    ]),
);

pub(in crate::card::sets) static BIRDS_OF_PARADISE: CardRecord = CardRecord::new(
    cards::BIRDS_OF_PARADISE,
    "Birds of Paradise",
    CardArt::new("55fe6449-1f23-43dc-adee-d144cd505b5c", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Bird"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static BLUE_ELEMENTAL_BLAST: CardRecord = CardRecord::new(
    cards::BLUE_ELEMENTAL_BLAST,
    "Blue Elemental Blast",
    CardArt::new("20d666ef-39bf-4fbf-8201-5f1056539da2", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target red spell.\n• Destroy target red permanent.",
        &[
            AbilityDef::counter_target(
                "Counter target red spell",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Red)),
            ),
            AbilityDef::destroy_target(
                "Destroy target red permanent",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(ManaColor::Red)),
                true,
            ),
        ],
    )),
);

pub(in crate::card::sets) static CHANNEL: CardRecord = CardRecord::new(
    cards::CHANNEL,
    "Channel",
    CardArt::new("c1862c47-71cc-45a3-8805-a5ddc62e55ea", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{G}{G}"))
    .with_abilities(&[AbilityDef::custom_full(
        "Until end of turn, any time you could activate a mana ability, you may pay 1 life. If you do, add {C}.",
        CardBehavior::Channel,
        "The life is offered as its own action at priority and is also counted by the payment layer, so a cost can be paid with it mid-cast. Colourless mana pays only the generic part of a cost, and the last point of life is not spendable.",
    )]),
);

pub(in crate::card::sets) static CRUSADE: CardRecord = CardRecord::new(
    cards::CRUSADE,
    "Crusade",
    CardArt::new("057986c7-20c0-4157-b4df-beae4ef5c66d", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}{W}"))
    .with_abilities(&[AbilityDef::custom_full(
        "White creatures get +1/+1.",
        CardBehavior::Crusade,
        "The continuous power/toughness bonus is implemented by the legacy characteristic evaluator.",
    )]),
);

pub(in crate::card::sets) static DARK_RITUAL: CardRecord = CardRecord::new(
    cards::DARK_RITUAL,
    "Dark Ritual",
    CardArt::new("ebb6664d-23ca-456e-9916-afcd6f26aa7f", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell(
        "Add {B}{B}{B}.",
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(3)),
    )]),
);

pub(in crate::card::sets) static DEMONIC_TUTOR: CardRecord = CardRecord::new(
    cards::DEMONIC_TUTOR,
    "Demonic Tutor",
    CardArt::new("711d4d54-5520-4de8-9b93-79902ed8e562", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::custom_full(
        "Search your library for a card, put that card into your hand, then shuffle.",
        CardBehavior::DemonicTutor,
        "The search choice and shuffle are implemented by the card-local resolution procedure.",
    )]),
);

pub(in crate::card::sets) static DRAIN_LIFE: CardRecord = CardRecord::new(
    cards::DRAIN_LIFE,
    "Drain Life",
    CardArt::new("5d077a49-73d4-4958-b42a-31b814e110e8", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{1}{B}"))
    .spend_only_on_x(ManaColor::Black)
    .with_abilities(&[
        AbilityDef::enforced_when_cast(
            "Spend only black mana on X.",
            "The payment layer folds X into the black requirement, so no other \
             mana can cover it.",
        ),
        AbilityDef::spell_with_targets(
            "Drain Life deals X damage to any target. You gain life equal to the damage dealt, but not more life than the player's life total before the damage was dealt, the planeswalker's loyalty before the damage was dealt, or the creature's toughness.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DrainLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

pub(in crate::card::sets) static EARTHQUAKE: CardRecord = CardRecord::new(
    cards::EARTHQUAKE,
    "Earthquake",
    CardArt::new("e68ac362-6cdc-48a6-bdd3-4f8ea32add64", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[AbilityDef::custom_full(
        "Earthquake deals X damage to each creature without flying and each player.",
        CardBehavior::Earthquake,
        "The global damage effect is implemented by the card-local spell resolver.",
    )]),
);

pub(in crate::card::sets) static FOREST: CardRecord = CardRecord::new(
    cards::FOREST,
    "Forest",
    CardArt::new("6f1c8cb0-38eb-408b-94e8-16db83999b3b", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest"]).with_supertype(CardSupertype::Basic),
);

pub(in crate::card::sets) static HYPNOTIC_SPECTER: CardRecord = CardRecord::new(
    cards::HYPNOTIC_SPECTER,
    "Hypnotic Specter",
    CardArt::new("b43b900f-2d9b-442b-9699-058483604ec9", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(
        mana_cost!("{1}{B}{B}"),
        &["Specter"],
        2,
        2,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, that player discards a card at random.",
            TriggerEventDef::DamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
                player: PlayerRelation::Opponent,
            },
            EffectDef::DiscardAtRandom {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

static TARGET_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

pub(in crate::card::sets) static MIND_TWIST: CardRecord = CardRecord::new(
    cards::MIND_TWIST,
    "Mind Twist",
    CardArt::new("eee9e106-a248-49d2-b8c8-6bbcd56ce739", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player discards X cards at random.",
        &TARGET_PLAYER,
        EffectDef::DiscardAtRandom {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )]),
);

pub(in crate::card::sets) static NEVINYRRALS_DISK: CardRecord = CardRecord::new(
    cards::NEVINYRRALS_DISK,
    "Nevinyrral's Disk",
    CardArt::new("12926dc8-8e6f-4a47-a12b-4d674189615a", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        AbilityDef::activated(
            "{1}, {T}: Destroy all artifacts, creatures, and enchantments.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Special("Destroy all artifacts, creatures, and enchantments"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::NevinyrralsDisk))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The global destruction procedure is implemented by the card-local resolver.",
        ))
        .with_legacy_procedure(),
    ]),
);

pub(in crate::card::sets) static PLATEAU: CardRecord = CardRecord::new(
    cards::PLATEAU,
    "Plateau",
    CardArt::new("6eafa00b-c628-40f6-86eb-88e1361fc7a0", "Drew Tucker"),
    CardSet::Alpha,
    CardRules::new_land(&["Mountain", "Plains"]),
);

pub(in crate::card::sets) static PSIONIC_BLAST: CardRecord = CardRecord::new(
    cards::PSIONIC_BLAST,
    "Psionic Blast",
    CardArt::new("a6a86e6e-bfff-46af-9d36-c912901fea92", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Psionic Blast deals 4 damage to any target and 2 damage to you.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

/// Any card, not just a creature: Regrowth is happy to take back a land or
/// the spell that killed something.
static REGROWTH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

pub(in crate::card::sets) static REGROWTH: CardRecord = CardRecord::new(
    cards::REGROWTH,
    "Regrowth",
    CardArt::new("badc73ec-3728-4246-90c7-5f4eb7051ed5", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target card from your graveyard to your hand.",
        &REGROWTH_TARGET,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: LibraryPlacement::Top,
        },
    )]),
);

pub(in crate::card::sets) static SAVANNAH: CardRecord = CardRecord::new(
    cards::SAVANNAH,
    "Savannah",
    CardArt::new("94f7e24c-2546-41b6-81ad-5e920b07e64e", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Plains"]),
);

pub(in crate::card::sets) static SAVANNAH_LIONS: CardRecord = CardRecord::new(
    cards::SAVANNAH_LIONS,
    "Savannah Lions",
    CardArt::new("d05b92bd-797e-413f-a8b0-32e0937a1ee0", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 2, 1),
);

pub(in crate::card::sets) static SCRUBLAND: CardRecord = CardRecord::new(
    cards::SCRUBLAND,
    "Scrubland",
    CardArt::new("bebe39d4-21fb-46a4-a1ec-b97102e46c15", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains", "Swamp"]),
);

pub(in crate::card::sets) static SENGIR_VAMPIRE: CardRecord = CardRecord::new(
    cards::SENGIR_VAMPIRE,
    "Sengir Vampire",
    CardArt::new("510840f4-7c0e-4b47-8ebf-23c20cac4bd9", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(
        mana_cost!("{3}{B}{B}"),
        &["Vampire"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a creature dealt damage by this creature this turn dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::DamagedCreatureDied,
            EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::Source,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static SINKHOLE: CardRecord = CardRecord::new(
    cards::SINKHOLE,
    "Sinkhole",
    CardArt::new("04b31611-9053-4eaf-b392-21bb644fef5f", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target land.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )]),
);

pub(in crate::card::sets) static SWAMP: CardRecord = CardRecord::new(
    cards::SWAMP,
    "Swamp",
    CardArt::new("6176936d-72e2-4205-8871-4c5a4f1cb2d8", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp"]).with_supertype(CardSupertype::Basic),
);

pub(in crate::card::sets) static TAIGA: CardRecord = CardRecord::new(
    cards::TAIGA,
    "Taiga",
    CardArt::new("60df6592-0b3b-4b87-aeb2-8fa94b4fb7be", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Mountain"]),
);

/// Terror is itself a black spell, so protection from black keeps a creature
/// off this list as well; that comes from the shared targeting rules rather
/// than from anything written here.
static TERROR_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
    ]),
)];

pub(in crate::card::sets) static TERROR: CardRecord = CardRecord::new(
    cards::TERROR,
    "Terror",
    CardArt::new("21004958-2c7e-4a55-bc80-411c4d780106", "Ron Spencer"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target nonartifact, nonblack creature. It can't be regenerated.",
        &TERROR_TARGET,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: false,
        },
    )]),
);

pub(in crate::card::sets) static TIME_VAULT: CardRecord = CardRecord::new(
    cards::TIME_VAULT,
    "Time Vault",
    CardArt::new("902441dc-c976-4c92-b897-6376eaa0fe38", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::Special("Keep this artifact tapped during its controller's untap step"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::TimeVault))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The untap restriction is implemented by the shared untap procedure.",
        )),
        AbilityDef::replacement_for(
            "If you would begin your turn while this artifact is tapped, you may skip that turn instead. If you do, untap this artifact.",
            ReplacementEventDef::Special("begin your turn while this artifact is tapped"),
            EffectDef::Special("Optionally skip the turn to untap this artifact"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::TimeVault))
        .with_coverage(AbilityCoverageDef::partial(
            "The wrong turn is skipped. The replacement should apply to the turn that is beginning, but the offer is made during the untap step, after that turn has already started, and accepting banks a skip that is spent on the controller's next turn instead. So the controller keeps the turn the artifact should have cost them.",
        )),
        AbilityDef::activated(
            "{T}: Take an extra turn after this one.",
            &[AbilityCostDef::TapSource],
            EffectDef::Special("Give this ability's controller an extra turn"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::TimeVault))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The extra turn is implemented by the card-local activated-ability resolver.",
        ))
        .with_legacy_procedure(),
    ]),
);

pub(in crate::card::sets) static TIMETWISTER: CardRecord = CardRecord::new(
    cards::TIMETWISTER,
    "Timetwister",
    CardArt::new("9a49dc44-616e-4bdd-8220-0bb71eccc512", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{U}"))
    .with_abilities(&[AbilityDef::custom_full(
        "Each player shuffles their hand and graveyard into their library, then draws seven cards. (Then put Timetwister into its owner's graveyard.)",
        CardBehavior::Timetwister,
        "The card-local resolver shuffles both players back and then draws with empty-library losses settled together, so one spell decking both players is a draw.",
    )]),
);

pub(in crate::card::sets) static TROPICAL_ISLAND: CardRecord = CardRecord::new(
    cards::TROPICAL_ISLAND,
    "Tropical Island",
    CardArt::new("a9c6c759-aabf-44e7-ba8c-33c5df232b56", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Island"]),
);

pub(in crate::card::sets) static UNDERGROUND_SEA: CardRecord = CardRecord::new(
    cards::UNDERGROUND_SEA,
    "Underground Sea",
    CardArt::new("ff76ac86-8a8a-47fe-9388-8950ca3e26c3", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Island", "Swamp"]),
);

pub(in crate::card::sets) static WHITE_KNIGHT: CardRecord = CardRecord::new(
    cards::WHITE_KNIGHT,
    "White Knight",
    CardArt::new("50abfba8-c9f9-4ebf-965a-4b425fe83129", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from(ManaColor::Black),
    ]),
);

/// The doubling reads the creature's power as Berserk resolves, and the
/// death only comes for a creature that actually attacked.
static BERSERK_EFFECT: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Composite(&BERSERK_BONUS),
        duration: EffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::AtNextStep {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
        effect: &EffectDef::IfCondition {
            condition: &BERSERK_ATTACKED,
            then: &EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        },
    },
];

static BERSERK_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::GrantAbility(&BERSERK_TRAMPLE),
    AppliedEffectDef::ModifyPowerToughness {
        power: ValueDef::TargetPower(TargetIndex::PRIMARY),
        toughness: ValueDef::Constant(0),
    },
];

static BERSERK_TRAMPLE: AbilityDef = abilities::trample();

static BERSERK_ATTACKED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::AttackedThisTurn,
};

pub(in crate::card::sets) static BERSERK: CardRecord = CardRecord::new(
    cards::BERSERK,
    "Berserk",
    CardArt::new("e173c8ce-2352-405e-ad00-e3bb94ced1ad", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{G}"))
    .cast_only_before_combat_damage()
    .with_abilities(&[
        AbilityDef::enforced_when_cast(
            "Cast this spell only before the combat damage step.",
            "The play option refuses the cast from the combat damage step onward.",
        ),
        AbilityDef::spell_with_targets(
            "Target creature gains trample and gets +X/+0 until end of turn, where X is its power. At the beginning of the next end step, destroy that creature if it attacked this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&BERSERK_EFFECT),
        ),
    ]),
);

pub(in crate::card::sets) static COPY_ARTIFACT: CardRecord = CardRecord::new(
    cards::COPY_ARTIFACT,
    "Copy Artifact",
    CardArt::new("fd5ed955-1193-4e6a-a3e2-f54c1f9bf063", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
    .with_abilities(&[AbilityDef::replacement(
        "You may have this enchantment enter as a copy of any artifact on the battlefield, except it's an enchantment in addition to its other types.",
        EffectDef::CopyPermanentAsItEnters {
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            added_types: CardTypeSet::single(CardType::Enchantment),
        },
    )]),
);

pub(in crate::card::sets) static GIANT_GROWTH: CardRecord = CardRecord::new(
    cards::GIANT_GROWTH,
    "Giant Growth",
    CardArt::new("367dbefe-3366-408e-9fcf-7dc00f8cc201", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+3 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(3),
                toughness: ValueDef::Constant(3),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

pub(in crate::card::sets) static ICY_MANIPULATOR: CardRecord = CardRecord::new(
    cards::ICY_MANIPULATOR,
    "Icy Manipulator",
    CardArt::new("29dc1596-a2e7-4d60-9f99-89babaef8a06", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Tap target artifact, creature, or land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

pub(in crate::card::sets) static LLANOWAR_ELVES: CardRecord = CardRecord::new(
    cards::LLANOWAR_ELVES,
    "Llanowar Elves",
    CardArt::new("d4f1cc9e-4f99-4c26-ac1b-8ef069fa8ceb", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

pub(in crate::card::sets) static SCRYB_SPRITES: CardRecord = CardRecord::new(
    cards::SCRYB_SPRITES,
    "Scryb Sprites",
    CardArt::new("6d929c38-91e6-457c-937a-d1884f4bba44", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Faerie"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

pub(in crate::card::sets) static STONE_RAIN: CardRecord = CardRecord::new(
    cards::STONE_RAIN,
    "Stone Rain",
    CardArt::new("57ff74cb-a2ed-4123-ac42-f72f9820049e", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target land.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )]),
);

// The chosen presentation art is its Beta printing; the definition debuted in Alpha.
pub(in crate::card::sets) static SEDGE_TROLL: CardRecord = CardRecord::new(
    cards::SEDGE_TROLL,
    "Sedge Troll",
    CardArt::new("02ec317b-52a6-4490-80e5-a56826b06771", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Troll"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Swamp.",
            EffectDef::Special("Give this creature +1/+1 while its controller controls a Swamp"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::SedgeTroll))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The conditional characteristic bonus is implemented by the legacy evaluator.",
        )),
        AbilityDef::activated(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Special("Regenerate the source creature"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::SedgeTroll))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "Regeneration shields are implemented by the card-local activated-ability resolver.",
        ))
        .with_legacy_procedure(),
    ]),
);

pub(in crate::card::sets) static WRATH_OF_GOD: CardRecord = CardRecord::new(
    cards::WRATH_OF_GOD,
    "Wrath of God",
    CardArt::new("a2788d69-6a3a-42f0-8736-cc6b57755ecd", "Quinton Hoover"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all creatures. They can't be regenerated.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: false,
        },
    )),
);

pub(in crate::card::sets) static MAGICAL_HACK: CardRecord = CardRecord::new(
    cards::MAGICAL_HACK,
    "Magical Hack",
    CardArt::new("2bd4202c-0477-45aa-82fd-83c85d6d4bef", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(
        AbilityDef::spell_with_targets("Change the text of target spell or permanent by replacing all instances of one basic land type with another. (For example, you may change \"swampwalk\" to \"plainswalk.\" This effect lasts indefinitely.)", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::ChangeTextBasicLandType {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            })
        .with_coverage(AbilityCoverageDef::partial(
            "Every battlefield permanent is a legal target, including one with no basic-land-type words. Basic-land-type words on type lines are changed; spell targets and substitutions elsewhere in rules text remain deferred.",
        )),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANKH_OF_MISHRA,
    &BLACK_VISE,
    &COPPER_TABLET,
    &FIREBALL,
    &FORK,
    &GLASSES_OF_URZA,
    &IRON_STAR,
    &LIGHTNING_BOLT,
    &MOUNTAIN,
    &RED_ELEMENTAL_BLAST,
    &SHATTER,
    &SMOKE,
    &STONE_GIANT,
    &WINTER_ORB,
    &BLACK_LOTUS,
    &CHAOS_ORB,
    &DRAGON_WHELP,
    &GOBLIN_BALLOON_BRIGADE,
    &GOBLIN_KING,
    &GRANITE_GARGOYLE,
    &IRONCLAW_ORCS,
    &MOX_EMERALD,
    &MOX_JET,
    &MOX_PEARL,
    &MOX_RUBY,
    &MOX_SAPPHIRE,
    &SOL_RING,
    &WHEEL_OF_FORTUNE,
    &JUGGERNAUT,
    &MANA_VAULT,
    &ANCESTRAL_RECALL,
    &BRAINGEYSER,
    &COUNTERSPELL,
    &DISENCHANT,
    &ISLAND,
    &JAYEMDAE_TOME,
    &PLAINS,
    &SERRA_ANGEL,
    &SWORDS_TO_PLOWSHARES,
    &TIME_WALK,
    &TUNDRA,
    &ARMAGEDDON,
    &BADLANDS,
    &BALANCE,
    &BAYOU,
    &BLACK_KNIGHT,
    &BIRDS_OF_PARADISE,
    &BLUE_ELEMENTAL_BLAST,
    &CHANNEL,
    &CRUSADE,
    &DARK_RITUAL,
    &DEMONIC_TUTOR,
    &DRAIN_LIFE,
    &EARTHQUAKE,
    &FOREST,
    &HYPNOTIC_SPECTER,
    &MIND_TWIST,
    &NEVINYRRALS_DISK,
    &PLATEAU,
    &PSIONIC_BLAST,
    &REGROWTH,
    &SAVANNAH,
    &SAVANNAH_LIONS,
    &SCRUBLAND,
    &SENGIR_VAMPIRE,
    &SINKHOLE,
    &SWAMP,
    &TAIGA,
    &TERROR,
    &TIME_VAULT,
    &TIMETWISTER,
    &TROPICAL_ISLAND,
    &UNDERGROUND_SEA,
    &WALL_OF_STONE,
    &WHITE_KNIGHT,
    &BERSERK,
    &COPY_ARTIFACT,
    &GIANT_GROWTH,
    &ICY_MANIPULATOR,
    &LLANOWAR_ELVES,
    &SCRYB_SPRITES,
    &STONE_RAIN,
    &SEDGE_TROLL,
    &WRATH_OF_GOD,
    &MAGICAL_HACK,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&PLAINS, 1),
    PrintingRecord::alternate(&ISLAND, 1),
    PrintingRecord::alternate(&SWAMP, 1),
    PrintingRecord::alternate(&MOUNTAIN, 1),
    PrintingRecord::alternate(&FOREST, 1),
];
