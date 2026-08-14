use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef,
    AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardBehavior, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CounterKind, DiscardSelectionDef, DividedTotal, EffectDef,
    EffectDurationDef, EffectExecutionDef, EffectRecipientDef, KeywordAbility, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, PlayerRelation, ReplacementEffectDef, ReplacementEventDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static INDESTRUCTIBLE_AURA_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

// LEG 1 — Akron Legionnaire
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Except for creatures named Akron Legionnaire and artifact creatures, creatures you control can't attack”.

// LEG 2 — Alabaster Potion
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “• Prevent the next X damage that would be dealt to any target this turn”.

// LEG 3 — Amrou Kithkin
// Audit: partial — Its blocker power predicate omits modifiers from static continuous effects.
pub(in crate::card::sets) static AMROU_KITHKIN: CardRecord = CardRecord::new(
    cards::AMROU_KITHKIN,
    "Amrou Kithkin",
    CardArt::new("cbce1c55-123c-4a05-bde4-18a1601fcc5a", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Kithkin"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked by creatures with power 3 or greater.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::PowerAtLeast(3)),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Blocking checks printed and temporary power but not power from static continuous effects.",
        )),
    ),
);

// LEG 4 — Angelic Voices
// Audit: blocked — Needs a continuously reevaluated absence predicate over permanents you control for “Creatures you control get +1/+1 as long as you control no nonartifact, nonwhite creatures”.

// LEG 5 — Cleanse
pub(in crate::card::sets) static CLEANSE: CardRecord = CardRecord::new(
    cards::CLEANSE,
    "Cleanse",
    CardArt::new("2fbd611b-ac97-4516-bad7-cc9ee4ef74f7", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all black creatures.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )),
);

// LEG 6 — Clergy of the Holy Nimbus
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “{1}: This creature can't be regenerated this turn. Only your opponents may activate this ability”.

// LEG 7 — D'Avenant Archer
pub(in crate::card::sets) static DAVENANT_ARCHER: CardRecord = CardRecord::new(
    cards::DAVENANT_ARCHER,
    "D'Avenant Archer",
    CardArt::new("b09aee5c-8b9e-46c2-b4d4-508062f8af05", "Douglas Shuler"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier", "Archer"], 1, 2)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target attacking or blocking creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        )),
);

// LEG 8 — Divine Intervention
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “When you remove the last intervention counter from this enchantment, the game is a draw”.

// LEG 9 — Divine Offering
pub(in crate::card::sets) static DIVINE_OFFERING: CardRecord = CardRecord::new(
    cards::DIVINE_OFFERING,
    "Divine Offering",
    CardArt::new("9c78c2f3-2f40-48ad-9dc4-55d1fa399a56", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact. You gain life equal to its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// LEG 10 — Divine Transformation
pub(in crate::card::sets) static DIVINE_TRANSFORMATION: CardRecord = CardRecord::new(
    cards::DIVINE_TRANSFORMATION,
    "Divine Transformation",
    CardArt::new("a89ad9fd-33a6-4d31-9f4c-8bf192882f21", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+3.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(3),
                        toughness: ValueDef::Constant(3),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEG 11 — Elder Land Wurm
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “When this creature blocks, it loses defender”.

// LEG 12 — Enchanted Being
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all combat damage that would be dealt to this creature by enchanted creatures”.

// LEG 13 — Equinox
// Audit: blocked — Needs a granted ability that can target a spell by prospectively determining whether that spell would destroy one of its controller's lands.

// LEG 14 — Fortified Area
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “Wall creatures you control get +1/+0 and have banding”.

// LEG 15 — Glyph of Life
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Choose target Wall creature. Whenever that creature is dealt damage by an attacking creature this turn, you gain that much life”.

// LEG 16 — Great Defender
pub(in crate::card::sets) static GREAT_DEFENDER: CardRecord = CardRecord::new(
    cards::GREAT_DEFENDER,
    "Great Defender",
    CardArt::new("879a8653-1538-4f78-a3d3-a900a4d9499b", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +0/+X until end of turn, where X is its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(0),
                toughness: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 17 — Great Wall
pub(in crate::card::sets) static GREAT_WALL: CardRecord = CardRecord::new(
    cards::GREAT_WALL,
    "Great Wall",
    CardArt::new("cd860a1d-aa17-4579-b9b1-d101d2416387", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with plainswalk can be blocked as though they didn't have plainswalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Plains),
    )]),
);

// LEG 18 — Greater Realm of Preservation
pub(in crate::card::sets) static GREATER_REALM_OF_PRESERVATION: CardRecord = CardRecord::new(
    cards::GREATER_REALM_OF_PRESERVATION,
    "Greater Realm of Preservation",
    CardArt::new("5e236816-0c49-4b48-b18b-03add5a80d72", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}{W}: The next time a black or red source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Color(ManaColor::Black),
                ObjectPredicateDef::Color(ManaColor::Red),
            ]),
        ),
    ),
);

// LEG 19 — Heaven's Gate
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “One or more target creatures become white until end of turn”.

// LEG 20 — Holy Day
pub(in crate::card::sets) static HOLY_DAY: CardRecord = CardRecord::new(
    cards::HOLY_DAY,
    "Holy Day",
    CardArt::new("f6c95a2b-bf44-4ff2-9c6a-916773346edd", "Justin Hampton"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventAllCombatDamageThisTurn,
    )),
);

// LEG 21 — Indestructible Aura
pub(in crate::card::sets) static INDESTRUCTIBLE_AURA: CardRecord = CardRecord::new(
    cards::INDESTRUCTIBLE_AURA,
    "Indestructible Aura",
    CardArt::new("ed2a7333-c9ce-4011-b00e-1304e1eec25e", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Prevent all damage that would be dealt to target creature this turn.",
        &INDESTRUCTIBLE_AURA_TARGET,
        EffectDef::PreventAllDamageThisTurn {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )]),
);

// LEG 22 — Infinite Authority
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature blocks or becomes blocked by a creature with toughness 3 or less, destroy the other creature at end of combat. At the beginning of the next end step, if that…”.

// LEG 23 — Ivory Guardians
// Audit: blocked — Needs the exact token definition and creation/lifecycle behavior for “Creatures named Ivory Guardians get +1/+1 as long as an opponent controls a nontoken red permanent”.

// LEG 24 — Keepers of the Faith
pub(in crate::card::sets) static KEEPERS_OF_THE_FAITH: CardRecord = CardRecord::new(
    cards::KEEPERS_OF_THE_FAITH,
    "Keepers of the Faith",
    CardArt::new("b63a69ae-99ce-4d26-88b7-784793c43cd4", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 2, 3),
);

// LEG 25 — Kismet
pub(in crate::card::sets) static KISMET: CardRecord = CardRecord::new(
    cards::KISMET,
    "Kismet",
    CardArt::new("7e0651ad-6901-4f9b-8807-d66e53a4ada8", "Kaja Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::replacement_for(
        "Artifacts, creatures, and lands your opponents control enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            controller: PlayerRelation::Opponent,
        },
        EffectDef::Replacement(ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::Tapped,
        )),
    )),
);

// LEG 26 — Land Tax
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “At the beginning of your upkeep, if an opponent controls more lands than you, you may search your library for up to three basic land cards, reveal them, put them into your hand, then shuffle”.

// LEG 27 — Lifeblood
pub(in crate::card::sets) static LIFEBLOOD: CardRecord = CardRecord::new(
    cards::LIFEBLOOD,
    "Lifeblood",
    CardArt::new("4ecb1362-9a67-4d4c-8d69-9ac2ebf4d0b0", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a Mountain an opponent controls becomes tapped, you gain 1 life.",
        TriggerEventDef::BecomesTapped(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// LEG 28 — Moat
pub(in crate::card::sets) static MOAT: CardRecord = CardRecord::new(
    cards::MOAT,
    "Moat",
    CardArt::new("952ba126-0915-47f0-9b6a-a0a6dcd22c6f", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[AbilityDef::custom_full(
        "Creatures without flying can't attack.",
        CardBehavior::Moat,
        "The attack restriction is implemented by the legacy combat legality check.",
    )]),
);

// LEG 29 — Osai Vultures
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Remove two carrion counters from this creature: This creature gets +1/+1 until end of turn”.

// LEG 30 — Petra Sphinx
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{T}: Target player chooses a card name, then reveals the top card of their library. If that card has the chosen name, that player puts it into their hand. If it doesn't, the player puts…”.

// LEG 31 — Presence of the Master
pub(in crate::card::sets) static PRESENCE_OF_THE_MASTER: CardRecord = CardRecord::new(
    cards::PRESENCE_OF_THE_MASTER,
    "Presence of the Master",
    CardArt::new("610288d2-4e44-4e26-883b-8b0bdd74bf3e", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts an enchantment spell, counter it.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::HasType(CardType::Enchantment)),
        EffectDef::Counter {
            object: EffectRecipientDef::TriggeringObject,
            zone: ZoneKind::Graveyard,
        },
    )),
);

// LEG 32 — Rapid Fire
// Audit: blocked — Needs a conditional grant that checks whether the target already has rampage before granting rampage 2, and a cast restriction limiting the spell to before blockers are declared.

// LEG 33 — Remove Enchantments
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Return to your hand all enchantments you both own and control, all Auras you own attached to permanents you control, and all Auras you own attached to attacking creatures your opponents…”.

// LEG 34 — Righteous Avengers
pub(in crate::card::sets) static RIGHTEOUS_AVENGERS: CardRecord = CardRecord::new(
    cards::RIGHTEOUS_AVENGERS,
    "Righteous Avengers",
    CardArt::new("d96b463e-9579-4e7b-87c2-342527b91e7c", "Heather Hudson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 3, 1)
        .with_ability(abilities::landwalk(BasicLandType::Plains)),
);

// LEG 35 — Seeker
pub(in crate::card::sets) static SEEKER: CardRecord = CardRecord::new(
    cards::SEEKER,
    "Seeker",
    CardArt::new("df608b59-cc07-4e1d-b6d6-f15e69b15b92", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature can't be blocked except by artifact creatures and/or white creatures.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Not(
                        &ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Color(ManaColor::White),
                        ]),
                    )),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEG 36 — Shield Wall
pub(in crate::card::sets) static SHIELD_WALL: CardRecord = CardRecord::new(
    cards::SHIELD_WALL,
    "Shield Wall",
    CardArt::new("a5032bf0-f9c0-4ef0-8ec2-fe7ccea9bdf3", "Douglas Shuler"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+2 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(0),
                toughness: ValueDef::Constant(2),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

static SPIRITUAL_SANCTUARY_PLAINS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::EventPlayer,
    },
    comparison: ComparisonDef::Greater,
    amount: 0,
};

// LEG 37 — Spirit Link
pub(in crate::card::sets) static SPIRIT_LINK: CardRecord = CardRecord::new(
    cards::SPIRIT_LINK,
    "Spirit Link",
    CardArt::new("5e2d35f8-3cf6-4843-9030-0e9a885d836c", "Kaja Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::triggered(
                "Whenever enchanted creature deals damage, you gain that much life.",
                TriggerEventDef::DamageDealtBy {
                    source: ObjectPredicateDef::AttachedToSource,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
        ]),
);

// LEG 38 — Spiritual Sanctuary
pub(in crate::card::sets) static SPIRITUAL_SANCTUARY: CardRecord = CardRecord::new(
    cards::SPIRITUAL_SANCTUARY,
    "Spiritual Sanctuary",
    CardArt::new("654dd1e0-a91d-44ee-af20-c025bf360c3f", "Amy Weber"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of each player's upkeep, if that player controls a Plains, they gain 1 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            &SPIRITUAL_SANCTUARY_PLAINS,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// LEG 39 — Thunder Spirit
pub(in crate::card::sets) static THUNDER_SPIRIT: CardRecord = CardRecord::new(
    cards::THUNDER_SPIRIT,
    "Thunder Spirit",
    CardArt::new(
        "61a59775-b1cd-4ed0-8abf-c2b37f7be0d5",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Elemental", "Spirit"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// LEG 40 — Tundra Wolves
pub(in crate::card::sets) static TUNDRA_WOLVES: CardRecord = CardRecord::new(
    cards::TUNDRA_WOLVES,
    "Tundra Wolves",
    CardArt::new("8f649cb5-e19c-453f-b062-4fd452d92257", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{W}"), &["Wolf"], 1, 1)
        .with_ability(abilities::first_strike()),
);

// LEG 41 — Visions
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Look at the top five cards of target player's library. You may then have that player shuffle that library”.

// LEG 42 — Wall of Caltrops
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 43 — Wall of Light
pub(in crate::card::sets) static WALL_OF_LIGHT: CardRecord = CardRecord::new(
    cards::WALL_OF_LIGHT,
    "Wall of Light",
    CardArt::new("f5758e82-f901-42b7-b705-0e68ca7ba59e", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Wall"], 1, 5).with_abilities(&[
        abilities::defender(),
        abilities::protection_from(ManaColor::Black),
    ]),
);

// LEG 44 — Acid Rain
pub(in crate::card::sets) static ACID_RAIN: CardRecord = CardRecord::new(
    cards::ACID_RAIN,
    "Acid Rain",
    CardArt::new("ba93c50a-2440-4e92-9cba-d97e20b1d29c", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Destroy all Forests.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )),
);

// LEG 45 — Anti-Magic Aura
// Audit: blocked — Needs the card's exact Aura targeting/attachment restriction rather than the broader existing cannot-be-enchanted effect for “Enchanted creature can't be the target of spells and can't be enchanted by other Auras”.

// LEG 46 — Azure Drake
pub(in crate::card::sets) static AZURE_DRAKE: CardRecord = CardRecord::new(
    cards::AZURE_DRAKE,
    "Azure Drake",
    CardArt::new("fb5f13a2-0896-4230-8957-6ad1cb2b895b", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Drake"], 2, 4)
        .with_ability(abilities::flying()),
);

// LEG 47 — Backfire
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Whenever enchanted creature deals damage to you, this Aura deals that much damage to that creature's controller”.

// LEG 48 — Boomerang
pub(in crate::card::sets) static BOOMERANG: CardRecord = CardRecord::new(
    cards::BOOMERANG,
    "Boomerang",
    CardArt::new("b8286edd-644b-4135-8dca-af97f3920de3", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target permanent to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )),
);

// LEG 49 — Brine Hag
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “When this creature dies, change the base power and toughness of all creatures that dealt damage to it this turn to 0/2”.

// LEG 50 — Devouring Deep
pub(in crate::card::sets) static DEVOURING_DEEP: CardRecord = CardRecord::new(
    cards::DEVOURING_DEEP,
    "Devouring Deep",
    CardArt::new("0855a5a8-8c40-4396-9ad1-8fa0fc6a0c59", "Liz Danforth"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Fish"], 1, 2)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// LEG 51 — Dream Coat
// Audit: blocked — Needs a per-object, per-turn activation quota for “{0}: Enchanted creature becomes the color or colors of your choice. Activate only once each turn”.

// LEG 52 — Elder Spawn
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't be blocked by red creatures”.

// LEG 53 — Enchantment Alteration
// Audit: blocked — Needs Aura reattachment targeting, enchant-legality validation, and attachment movement for “Attach target Aura attached to a creature or land to another permanent of that type”.

// LEG 54 — Energy Tap
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Tap target untapped creature you control. If you do, add an amount of {C} equal to that creature's mana value”.

// LEG 55 — Field of Dreams
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Players play with the top card of their libraries revealed”.

// LEG 56 — Flash Counter
pub(in crate::card::sets) static FLASH_COUNTER: CardRecord = CardRecord::new(
    cards::FLASH_COUNTER,
    "Flash Counter",
    CardArt::new("3c3cd450-f1cd-416b-9271-37d95815c089", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Instant)),
    )),
);

// LEG 57 — Flash Flood
pub(in crate::card::sets) static FLASH_FLOOD: CardRecord = CardRecord::new(
    cards::FLASH_FLOOD,
    "Flash Flood",
    CardArt::new("5ae88c06-f28c-4fbc-a28c-5eb203a04722", "Tom Wänerstrand"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Destroy target red permanent.\n• Return target Mountain to its owner's hand.",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target red permanent",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Color(ManaColor::Red),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target Mountain to its owner's hand",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            ),
        ],
    )),
);

// LEG 58 — Force Spike
pub(in crate::card::sets) static FORCE_SPIKE: CardRecord = CardRecord::new(
    cards::FORCE_SPIKE,
    "Force Spike",
    CardArt::new("70e64028-ae96-4950-aa6c-9d347409fad3", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        EffectDef::CounterUnlessPaid {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
            zone: ZoneKind::Graveyard,
        },
    )),
);

/// The mana arrives later, so the amount is read from what the countered
/// spell was rather than from anything still on the stack.
static MANA_DRAIN_EFFECT: [EffectDef; 2] = [
    EffectDef::Counter {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Graveyard,
    },
    EffectDef::AtNextStep {
        step: TurnStepDef::PrecombatMain,
        player: PlayerRelation::You,
        effect: &EffectDef::AddManaEqualTo {
            color: ManaColor::Colorless,
            amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
        },
    },
];

// LEG 59 — Gaseous Form
pub(in crate::card::sets) static GASEOUS_FORM: CardRecord = CardRecord::new(
    cards::GASEOUS_FORM,
    "Gaseous Form",
    CardArt::new("d0266dd4-31da-480b-9a44-4e217f748f06", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Prevent all combat damage that would be dealt to and dealt by enchanted creature.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::PreventCombatDamage,
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEG 60 — Glyph of Delusion
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Put X glyph counters on target creature that target Wall blocked this turn, where X is the power of that blocked creature. The creature gains "This creature doesn't untap during your…”.

// LEG 61 — In the Eye of Chaos
// Audit: blocked — Needs a cast trigger that counters an instant unless its controller pays that spell's dynamically read mana value.

// LEG 62 — Invoke Prejudice
// Audit: blocked — Needs cross-object color comparison and a mana-value-based counter-unless payment on the triggering creature spell.

// LEG 63 — Juxtapose
// Audit: blocked — Needs duration-aware control-changing continuous effects for “You and target player exchange control of the creature you each control with the greatest mana value. Then exchange control of artifacts the same way. If two or more permanents a player…”.

// LEG 64 — Land Equilibrium
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “If an opponent who controls at least as many lands as you do would put a land onto the battlefield, that player instead puts that land onto the battlefield then sacrifices a land of…”.

// LEG 65 — Mana Drain
pub(in crate::card::sets) static MANA_DRAIN: CardRecord = CardRecord::new(
    cards::MANA_DRAIN,
    "Mana Drain",
    CardArt::new("e691adef-3027-4e6a-889f-9f4e2df36a7c", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}{U}"))
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&MANA_DRAIN_EFFECT),
        )]),
);

// LEG 66 — Part Water
// Audit: blocked — Needs a target slot whose count is the X paid for the spell; granting islandwalk itself is available.

// LEG 67 — Psionic Entity
pub(in crate::card::sets) static PSIONIC_ENTITY: CardRecord = CardRecord::new(
    cards::PSIONIC_ENTITY,
    "Psionic Entity",
    CardArt::new("ec082062-5394-4340-bc29-0efd2af4b822", "Justin Hampton"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Illusion"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 2 damage to any target and 3 damage to itself.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Source,
                    amount: ValueDef::Constant(3),
                },
            ]),
        ),
    ),
);

// LEG 68 — Psychic Purge
// Audit: blocked — Needs a hidden-zone decision and continuation for “When a spell or ability an opponent controls causes you to discard this card, that player loses 5 life”.

// LEG 69 — Puppet Master
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “When enchanted creature dies, return that card to its owner's hand. If that card is returned to its owner's hand this way, you may pay {U}{U}{U}. If you do, return this card to its…”.

// LEG 70 — Recall
pub(in crate::card::sets) static RECALL: CardRecord = CardRecord::new(
    cards::RECALL,
    "Recall",
    CardArt::new("33296718-0625-4422-a65c-b21cf99c52ec", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{X}{X}{U}"))
    .with_abilities(&[AbilityDef::custom_full(
        "Discard X cards, then return a card from your graveyard to your hand for each card discarded this way. Exile Recall.",
        CardBehavior::Recall,
        "The card-local resolver discards on resolution and then returns that many cards, so a countered Recall costs nothing and the discarded cards are themselves returnable.",
    )]),
);

// LEG 71 — Relic Bind
// Audit: blocked — Needs a trigger relation for the attached permanent becoming tapped and its controller/characteristics for “Whenever enchanted artifact becomes tapped, choose one —”.

// LEG 72 — Remove Soul
pub(in crate::card::sets) static REMOVE_SOUL: CardRecord = CardRecord::new(
    cards::REMOVE_SOUL,
    "Remove Soul",
    CardArt::new("63de147c-2e62-41b9-8ada-93406387f08b", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target creature spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Creature)),
    )),
);

// LEG 73 — Reset
// Audit: blocked — Needs a spell-casting timing condition tied to the active turn and step for “Cast this spell only during an opponent's turn after their upkeep step”.

// LEG 74 — Reverberation
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “All damage that would be dealt this turn by target sorcery spell is dealt to that spell's controller instead”.

// LEG 75 — Sea Kings' Blessing
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “One or more target creatures become blue until end of turn”.

// LEG 76 — Segovian Leviathan
pub(in crate::card::sets) static SEGOVIAN_LEVIATHAN: CardRecord = CardRecord::new(
    cards::SEGOVIAN_LEVIATHAN,
    "Segovian Leviathan",
    CardArt::new("e5a814f1-7f8d-4c2c-b706-ee0ed5892f7b", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Leviathan"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// LEG 77 — Silhouette
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Choose target creature. If a spell or ability that targets that creature would cause a source to deal damage to that creature this turn, prevent that damage”.

// LEG 78 — Spectral Cloak
// Audit: blocked — Needs executable shroud target-legality and a temporary keyword grant for “Enchanted creature has shroud as long as it's untapped”.

// LEG 79 — Telekinesis
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Tap target creature. Prevent all combat damage that would be dealt by that creature this turn. It doesn't untap during its controller's next two untap steps”.

// LEG 80 — Teleport
// Audit: blocked — Needs a spell-casting timing condition tied to the active turn and step for “Cast this spell only during the declare attackers step”.

// LEG 81 — Time Elemental
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “{2}{U}{U}, {T}: Return target permanent that isn't enchanted to its owner's hand”.

// LEG 82 — Undertow
pub(in crate::card::sets) static UNDERTOW: CardRecord = CardRecord::new(
    cards::UNDERTOW,
    "Undertow",
    CardArt::new(
        "cf05e5c9-b7e4-4bd8-ab73-b54565710527",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with islandwalk can be blocked as though they didn't have islandwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Island),
    )]),
);

// LEG 83 — Venarian Gold
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Enchanted creature doesn't untap during its controller's untap step if it has a sleep counter on it”.

// LEG 84 — Wall of Vapor
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all damage that would be dealt to this creature by creatures it's blocking”.

// LEG 85 — Wall of Wonder
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{2}{U}{U}: This creature gets +4/-4 until end of turn and can attack this turn as though it didn't have defender”.

// LEG 86 — Zephyr Falcon
pub(in crate::card::sets) static ZEPHYR_FALCON: CardRecord = CardRecord::new(
    cards::ZEPHYR_FALCON,
    "Zephyr Falcon",
    CardArt::new("25a173fd-e10c-45f8-a6e5-ad7a747a8050", "Heather Hudson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// LEG 87 — Abomination
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked by a green or white creature, destroy that creature at end of combat”.

// LEG 88 — All Hallow's Eve
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of your upkeep, if this card is exiled with a scream counter on it, remove a scream counter from it. If there are no more scream counters on it, put it into your…”.

// LEG 89 — Blight
// Audit: blocked — Needs a trigger relation for the attached permanent becoming tapped and its controller/characteristics for “When enchanted land becomes tapped, destroy it”.

// LEG 90 — Carrion Ants
pub(in crate::card::sets) static CARRION_ANTS: CardRecord = CardRecord::new(
    cards::CARRION_ANTS,
    "Carrion Ants",
    CardArt::new("cbc0b009-3951-4aa3-985a-97139882da7e", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Insect"], 0, 1).with_ability(
        AbilityDef::activated(
            "{1}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 91 — Chains of Mephistopheles
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “If a player would draw a card except the first one they draw in each of their draw steps, that player discards a card instead. If the player discards a card this way, they draw a card.…”.

// LEG 92 — Cosmic Horror
// Audit: blocked — Needs an unless-paid destruction sequence that deals damage only when this ability actually destroys its source.

// LEG 93 — Cyclopean Mummy
// Audit: partial — Its death trigger cannot address the new card object created in the graveyard.
pub(in crate::card::sets) static CYCLOPEAN_MUMMY: CardRecord = CardRecord::new(
    cards::CYCLOPEAN_MUMMY,
    "Cyclopean Mummy",
    CardArt::new(
        "479ccc50-2d72-4adc-901e-fbd4eef2cf92",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 1).with_ability(
        AbilityDef::triggered(
            "When this creature dies, exile it.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                controller: None,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The trigger remembers the old battlefield incarnation but cannot address the new card object in the graveyard.",
        )),
    ),
);

// LEG 94 — Darkness
pub(in crate::card::sets) static DARKNESS: CardRecord = CardRecord::new(
    cards::DARKNESS,
    "Darkness",
    CardArt::new("53b04dab-45b7-418b-a0f0-bcf35145fc53", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventAllCombatDamageThisTurn,
    )),
);

// LEG 95 — Demonic Torment
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all combat damage that would be dealt by enchanted creature”.

// LEG 96 — Evil Eye of Orms-by-Gore
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't be blocked except by Walls”.

// LEG 97 — Fallen Angel
pub(in crate::card::sets) static FALLEN_ANGEL: CardRecord = CardRecord::new(
    cards::FALLEN_ANGEL,
    "Fallen Angel",
    CardArt::new("0f4174e4-0be8-49b5-8c52-22001790f6eb", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "Sacrifice a creature: This creature gets +2/+1 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 98 — Ghosts of the Damned
pub(in crate::card::sets) static GHOSTS_OF_THE_DAMNED: CardRecord = CardRecord::new(
    cards::GHOSTS_OF_THE_DAMNED,
    "Ghosts of the Damned",
    CardArt::new(
        "20275678-3488-43d8-a93b-993e2267ab07",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Spirit"], 0, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gets -1/-0 until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 99 — Giant Slug
// Audit: blocked — Needs a delayed upkeep trigger that makes a basic-land-type choice on resolution and grants the matching walk; granting a named walk is available.

// LEG 100 — Glyph of Doom
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Choose target Wall creature. At this turn's next end of combat, destroy all creatures that were blocked by that creature this turn”.

// LEG 101 — Greed
pub(in crate::card::sets) static GREED: CardRecord = CardRecord::new(
    cards::GREED,
    "Greed",
    CardArt::new("111a16a2-e875-4756-80db-290f9e8606db", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_ability(AbilityDef::activated(
        "{B}, Pay 2 life: Draw a card.",
        &[
            AbilityCostDef::Mana(mana_cost!("{B}")),
            AbilityCostDef::PayLife(2),
        ],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// LEG 102 — Headless Horseman
pub(in crate::card::sets) static HEADLESS_HORSEMAN: CardRecord = CardRecord::new(
    cards::HEADLESS_HORSEMAN,
    "Headless Horseman",
    CardArt::new("d1aa37c8-98fa-4984-b09b-cf65ad84e97b", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Knight"], 2, 2),
);

// LEG 103 — Hell Swarm
pub(in crate::card::sets) static HELL_SWARM: CardRecord = CardRecord::new(
    cards::HELL_SWARM,
    "Hell Swarm",
    CardArt::new("64164d1b-75f4-456e-a717-90ce554dc16c", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "All creatures get -1/-0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-1),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 104 — Hell's Caretaker
pub(in crate::card::sets) static HELLS_CARETAKER: CardRecord = CardRecord::new(
    cards::HELLS_CARETAKER,
    "Hell's Caretaker",
    CardArt::new("336b3b8f-d104-4f06-ad4f-c92b8a9038ca", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Horror"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice a creature: Return target creature card from your graveyard to the \
             battlefield. Activate only during your upkeep.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
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
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                controller: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// LEG 105 — Hellfire
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Destroy all nonblack creatures. Hellfire deals X plus 3 damage to you, where X is the number of creatures that died this way”.

// LEG 106 — Horror of Horrors
pub(in crate::card::sets) static HORROR_OF_HORRORS: CardRecord = CardRecord::new(
    cards::HORROR_OF_HORRORS,
    "Horror of Horrors",
    CardArt::new("b9f68dc2-c048-41ec-b237-c36fdd99c27d", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "Sacrifice a Swamp: Regenerate target black creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Swamp"),
                controller: PlayerRelation::You,
            }],
            &BLACK_CREATURE_TARGET,
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

static BLACK_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Color(ManaColor::Black),
    ]),
)];

// LEG 107 — Imprison
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature attacks or blocks, you may pay {1}. If you do, tap the creature, remove it from combat, and creatures it was blocking that had become blocked by only that…”.

// LEG 108 — Infernal Medusa
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature becomes blocked by a non-Wall creature, destroy that creature at end of combat”.

// LEG 109 — Jovial Evil
// Audit: blocked — Needs a dynamic count of white creatures controlled by the targeted opponent and multiplication for the damage value.

// LEG 110 — Lesser Werewolf
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{B}: If this creature's power is 1 or more, it gets -1/-0 until end of turn and put a -0/-1 counter on target creature blocking or blocked by this creature. Activate only during the…”.

// LEG 111 — Lost Soul
pub(in crate::card::sets) static LOST_SOUL: CardRecord = CardRecord::new(
    cards::LOST_SOUL,
    "Lost Soul",
    CardArt::new(
        "601eed5c-436d-425b-a45f-07881ad893c8",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Spirit", "Minion"], 2, 1)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// LEG 112 — Mold Demon
// Audit: blocked — Needs linked sacrifice/destruction accounting for “When this creature enters, sacrifice it unless you sacrifice two Swamps”.

// LEG 113 — Nether Void
// Audit: partial — Its counter-unless-payment trigger is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static NETHER_VOID: CardRecord = CardRecord::new(
    cards::NETHER_VOID,
    "Nether Void",
    CardArt::new("2e72f8cb-5bc3-4711-9b7c-a6eea9a0beaf", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::triggered(
            "Whenever a player casts a spell, counter it unless that player pays {3}.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::Any),
            EffectDef::CounterUnlessPaid {
                object: EffectRecipientDef::TriggeringObject,
                amount: ValueDef::Constant(3),
                zone: ZoneKind::Graveyard,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The counter trigger is executable, but the world-rule state-based action is not implemented.",
        ))),
);

// LEG 114 — Pit Scorpion
// Audit: blocked — Needs player poison counters and the poison-based state check, including this card's counter placement.

// LEG 115 — Quagmire
pub(in crate::card::sets) static QUAGMIRE: CardRecord = CardRecord::new(
    cards::QUAGMIRE,
    "Quagmire",
    CardArt::new("94e2aa9e-af6a-41c6-99a8-ca9335730ddb", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with swampwalk can be blocked as though they didn't have swampwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Swamp),
    )]),
);

// LEG 116 — Shimian Night Stalker
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{B}, {T}: All damage that would be dealt to you this turn by target attacking creature is dealt to this creature instead”.

// LEG 117 — Spirit Shackle
// Audit: blocked — Needs a trigger relation for the attached permanent becoming tapped and its controller/characteristics for “Whenever enchanted creature becomes tapped, put a -0/-2 counter on it”.

// LEG 118 — Syphon Soul
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Syphon Soul deals 2 damage to each other player. You gain life equal to the damage dealt this way”.

// LEG 119 — Takklemaggot
// Audit: blocked — Needs duration-aware control-changing continuous effects for “When enchanted creature dies, that creature's controller chooses a creature that this card could enchant. If the player does, return this card to the battlefield under your control…”.

// LEG 120 — The Abyss
// Audit: partial — Its upkeep destruction is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static THE_ABYSS: CardRecord = CardRecord::new(
    cards::THE_ABYSS,
    "The Abyss",
    CardArt::new("86a27d68-3e58-4ade-976d-36381beed451", "Pete Venters"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_supertype(CardSupertype::World)
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of each player's upkeep, destroy target nonartifact creature that player controls of their choice. It can't be regenerated.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::DestroyOfChoice {
                player: EffectRecipientDef::EventPlayer,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
                can_regenerate: false,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The upkeep destruction is executable, but the world-rule state-based action is not implemented.",
        ))]),
);

// LEG 121 — The Wretched
// Audit: blocked — Needs duration-aware control-changing continuous effects for “At end of combat, gain control of all creatures blocking this creature for as long as you control this creature”.

// LEG 122 — Touch of Darkness
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “One or more target creatures become black until end of turn”.

// LEG 123 — Transmutation
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Switch target creature's power and toughness until end of turn”.

// LEG 124 — Underworld Dreams
// Audit: blocked — Needs an opponent-draw event trigger that deals damage to the exact player who drew.

// LEG 125 — Vampire Bats
// Audit: blocked — Needs a per-object, per-turn activation quota for “{B}: This creature gets +1/+0 until end of turn. Activate no more than twice each turn”.

// LEG 126 — Walking Dead
pub(in crate::card::sets) static WALKING_DEAD: CardRecord = CardRecord::new(
    cards::WALKING_DEAD,
    "Walking Dead",
    CardArt::new("d7533a72-77d1-40cd-b3a1-7597d566c428", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1).with_abilities(&[
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// LEG 127 — Wall of Putrid Flesh
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all damage that would be dealt to this creature by enchanted creatures”.

// LEG 128 — Wall of Shadows
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all damage that would be dealt to this creature by creatures it's blocking”.

// LEG 129 — Wall of Tombstones
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “At the beginning of your upkeep, change this creature's base toughness to 1 plus the number of creature cards in your graveyard”.

// LEG 130 — Active Volcano
pub(in crate::card::sets) static ACTIVE_VOLCANO: CardRecord = CardRecord::new(
    cards::ACTIVE_VOLCANO,
    "Active Volcano",
    CardArt::new("ad402e65-6fac-4005-a2d4-592983df0c30", "Justin Hampton"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Destroy target blue permanent.\n• Return target Island to its owner's hand.",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target blue permanent",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target Island to its owner's hand",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            ),
        ],
    )),
);

// LEG 131 — Aerathi Berserker
pub(in crate::card::sets) static AERATHI_BERSERKER: CardRecord = CardRecord::new(
    cards::AERATHI_BERSERKER,
    "Aerathi Berserker",
    CardArt::new("06673800-22a7-4ee3-92fa-7c7cd4865d30", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}{R}{R}"), &["Human", "Berserker"], 2, 4).with_abilities(&[
        abilities::rampage(3, "Rampage 3 (Whenever this creature becomes blocked, it gets +3/+3 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 132 — Backdraft
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Choose a player who cast one or more sorcery spells this turn. Backdraft deals damage to that player equal to half the damage dealt by one of those sorcery spells this turn, rounded down”.

// LEG 133 — Beasts of Bogardan
// Audit: blocked — Needs a continuously reevaluated opponent-permanent color and nontoken existence condition for its +1/+1 modifier.

// LEG 134 — Blazing Effigy
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “When this creature dies, it deals X damage to target creature, where X is 3 plus the amount of damage dealt to this creature this turn by other sources named Blazing Effigy”.

// LEG 135 — Blood Lust
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “If target creature has toughness 5 or greater, it gets +4/-4 until end of turn. Otherwise, it gets +4/-X until end of turn, where X is its toughness minus 1”.

// LEG 136 — Caverns of Despair
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “No more than two creatures can attack each combat”.

// LEG 137 — Chain Lightning
pub(in crate::card::sets) static CHAIN_LIGHTNING: CardRecord = CardRecord::new(
    cards::CHAIN_LIGHTNING,
    "Chain Lightning",
    CardArt::new("b5883762-ca0a-4932-8d2a-41a45796a5f8", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets("Chain Lightning deals 3 damage to any target.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )], EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            }),
        AbilityDef::custom_full(
            "Then that player or that permanent's controller may pay {R}{R}. If the player does, they may copy this spell and may choose a new target for that copy.",
            CardBehavior::ChainLightning,
            "The optional payment and spell-copy procedure are implemented by the card-local follow-up resolver.",
        ),
    ]),
);

// LEG 138 — Crevasse
pub(in crate::card::sets) static CREVASSE: CardRecord = CardRecord::new(
    cards::CREVASSE,
    "Crevasse",
    CardArt::new("a432d6ae-a17f-484b-ad55-4b4b6674ba8d", "Rob Alexander"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with mountainwalk can be blocked as though they didn't have mountainwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Mountain),
    )]),
);

// LEG 139 — Crimson Kobolds
pub(in crate::card::sets) static CRIMSON_KOBOLDS: CardRecord = CardRecord::new(
    cards::CRIMSON_KOBOLDS,
    "Crimson Kobolds",
    CardArt::new("13696657-aeef-4add-9a3b-8137fce01fe3", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 140 — Crimson Manticore
pub(in crate::card::sets) static CRIMSON_MANTICORE: CardRecord = CardRecord::new(
    cards::CRIMSON_MANTICORE,
    "Crimson Manticore",
    CardArt::new("96f73f9c-1c4e-4343-bfa0-cc5c4a7a562e", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Manticore"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{R}, {T}: This creature deals 1 damage to target attacking or blocking creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LEG 141 — Crookshank Kobolds
pub(in crate::card::sets) static CROOKSHANK_KOBOLDS: CardRecord = CardRecord::new(
    cards::CROOKSHANK_KOBOLDS,
    "Crookshank Kobolds",
    CardArt::new("7af6b119-7db4-49dd-aaa4-044b8c133f13", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 142 — Disharmony
// Audit: blocked — Needs duration-aware control-changing continuous effects for “Untap target attacking creature and remove it from combat. Gain control of that creature until end of turn”.

// LEG 143 — Dwarven Song
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “One or more target creatures become red until end of turn”.

// LEG 144 — Eternal Warrior
pub(in crate::card::sets) static ETERNAL_WARRIOR: CardRecord = CardRecord::new(
    cards::ETERNAL_WARRIOR,
    "Eternal Warrior",
    CardArt::new("97cdc38e-1d96-4de2-98e2-713f5d4d2180", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has vigilance.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::vigilance()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEG 145 — Falling Star
// Audit: blocked — Needs the EC physical flip, overlap, and landing evaluation needed to choose which creatures take damage and become tapped.

// LEG 146 — Feint
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Tap all creatures blocking target attacking creature. Prevent all combat damage that would be dealt this turn by that creature and each creature blocking it”.

// LEG 147 — Firestorm Phoenix
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “If this creature would die, return it to its owner's hand instead. Until that player's next turn, that player plays with that card revealed in their hand and can't play it”.

// LEG 148 — Frost Giant
pub(in crate::card::sets) static FROST_GIANT: CardRecord = CardRecord::new(
    cards::FROST_GIANT,
    "Frost Giant",
    CardArt::new("6955d54f-7b37-4e43-8183-51677fb1ee11", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{R}{R}{R}"), &["Giant"], 4, 4).with_abilities(&[
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 149 — Giant Strength
pub(in crate::card::sets) static GIANT_STRENGTH: CardRecord = CardRecord::new(
    cards::GIANT_STRENGTH,
    "Giant Strength",
    CardArt::new("a86190bb-1f41-4128-b9fb-dfb1d178359d", "Justin Hampton"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEG 150 — Glyph of Destruction
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Target blocking Wall you control gets +10/+0 until end of combat. Prevent all damage that would be dealt to it this turn. Destroy it at the beginning of the next end step”.

// LEG 151 — Gravity Sphere
// Audit: partial — Its flying-removal effect is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static GRAVITY_SPHERE: CardRecord = CardRecord::new(
    cards::GRAVITY_SPHERE,
    "Gravity Sphere",
    CardArt::new("a2749332-e99a-4a0c-b3a3-5578b552fa11", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures lose flying.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::RemoveAbilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Flying removal is executable, but the world-rule state-based action is not implemented.",
        ))),
);

// LEG 152 — Hyperion Blacksmith
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{T}: You may tap or untap target artifact an opponent controls”.

// LEG 153 — Immolation
pub(in crate::card::sets) static IMMOLATION: CardRecord = CardRecord::new(
    cards::IMMOLATION,
    "Immolation",
    CardArt::new("9b3d34fa-398c-4ea0-a392-6690bd3a615c", "Scott Kirschner"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/-2.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(-2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEG 154 — Kobold Drill Sergeant
pub(in crate::card::sets) static KOBOLD_DRILL_SERGEANT: CardRecord = CardRecord::new(
    cards::KOBOLD_DRILL_SERGEANT,
    "Kobold Drill Sergeant",
    CardArt::new("741b14f8-625d-41be-a734-0efe042a6ee8", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kobold", "Soldier"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "Other Kobold creatures you control get +0/+1 and have trample.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Kobold"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(0),
                        toughness: ValueDef::Constant(1),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::trample()),
                ]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// LEG 155 — Kobold Overlord
pub(in crate::card::sets) static KOBOLD_OVERLORD: CardRecord = CardRecord::new(
    cards::KOBOLD_OVERLORD,
    "Kobold Overlord",
    CardArt::new("490eeedb-9c03-4dc7-81fd-ae54a7932e4d", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kobold"], 1, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::static_ability(
            "Other Kobold creatures you control have first strike.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Kobold"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// LEG 156 — Kobold Taskmaster
pub(in crate::card::sets) static KOBOLD_TASKMASTER: CardRecord = CardRecord::new(
    cards::KOBOLD_TASKMASTER,
    "Kobold Taskmaster",
    CardArt::new(
        "1b9c63eb-8d4e-4d8b-8637-308459ef036b",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kobold"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "Other Kobold creatures you control get +1/+0.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Kobold"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// LEG 157 — Kobolds of Kher Keep
pub(in crate::card::sets) static KOBOLDS_OF_KHER_KEEP: CardRecord = CardRecord::new(
    cards::KOBOLDS_OF_KHER_KEEP,
    "Kobolds of Kher Keep",
    CardArt::new("df0320d9-7c2a-456a-9159-1b4fae67bfb5", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 158 — Land's Edge
// Audit: blocked — Needs an ability any player may activate and a conditional keyed to the type of the card the discard cost actually took.

// LEG 159 — Mountain Yeti
pub(in crate::card::sets) static MOUNTAIN_YETI: CardRecord = CardRecord::new(
    cards::MOUNTAIN_YETI,
    "Mountain Yeti",
    CardArt::new("09242f08-3bfc-4082-b32f-703c7fed62a0", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Yeti"], 3, 3).with_abilities(&[
        abilities::mountainwalk(),
        abilities::protection_from(ManaColor::White),
    ]),
);

// LEG 160 — Primordial Ooze
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature attacks each combat if able”.

// LEG 161 — Pyrotechnics
pub(in crate::card::sets) static PYROTECHNICS: CardRecord = CardRecord::new(
    cards::PYROTECHNICS,
    "Pyrotechnics",
    CardArt::new("2646284b-a94d-4c99-98d4-7becbb473e2b", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Pyrotechnics deals 4 damage divided as you choose among any number of targets.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::AnyTarget,
            minimum: 1,
            maximum: 4,
            divided_total: Some(DividedTotal::Fixed(4)),
        }],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::DividedAmongTargets,
        },
    )),
);

// LEG 162 — Quarum Trench Gnomes
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{T}: If target Plains is tapped for mana, it produces colorless mana instead of white mana”.

// LEG 163 — Raging Bull
pub(in crate::card::sets) static RAGING_BULL: CardRecord = CardRecord::new(
    cards::RAGING_BULL,
    "Raging Bull",
    CardArt::new(
        "ec10a51c-d2c3-4d14-9a71-9e59155bf980",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ox"], 2, 2),
);

// LEG 164 — Spinal Villain
pub(in crate::card::sets) static SPINAL_VILLAIN: CardRecord = CardRecord::new(
    cards::SPINAL_VILLAIN,
    "Spinal Villain",
    CardArt::new("d6d5e36f-0049-4be8-bf85-8dc0186339a4", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Beast"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Destroy target blue creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// LEG 165 — Storm World
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “At the beginning of each player's upkeep, this enchantment deals X damage to that player, where X is 4 minus the number of cards in their hand”.

// LEG 166 — Tempest Efreet
// Audit: blocked — Needs random hand reveal, an opponent life-payment choice, and a permanent ownership exchange between cards in different zones.

// LEG 167 — The Brute
pub(in crate::card::sets) static THE_BRUTE: CardRecord = CardRecord::new(
    cards::THE_BRUTE,
    "The Brute",
    CardArt::new("f9ffb265-872f-47b3-974c-92bcbebd557e", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+0.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::activated(
                "{R}{R}{R}: Regenerate enchanted creature.",
                &[AbilityCostDef::Mana(mana_cost!("{R}{R}{R}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// LEG 168 — Wall of Dust
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks a creature, that creature can't attack during its controller's next turn”.

// LEG 169 — Wall of Earth
pub(in crate::card::sets) static WALL_OF_EARTH: CardRecord = CardRecord::new(
    cards::WALL_OF_EARTH,
    "Wall of Earth",
    CardArt::new("c12e97c1-ca28-432a-8140-3f08bb4485a3", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Wall"], 0, 6)
        .with_ability(abilities::defender()),
);

// LEG 170 — Wall of Heat
pub(in crate::card::sets) static WALL_OF_HEAT: CardRecord = CardRecord::new(
    cards::WALL_OF_HEAT,
    "Wall of Heat",
    CardArt::new("a38059a8-be69-4cc1-969b-951c610f2f11", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wall"], 2, 6)
        .with_ability(abilities::defender()),
);

// LEG 171 — Wall of Opposition
pub(in crate::card::sets) static WALL_OF_OPPOSITION: CardRecord = CardRecord::new(
    cards::WALL_OF_OPPOSITION,
    "Wall of Opposition",
    CardArt::new("2b3d1430-9978-4983-a4fd-d1fa8dea2169", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Wall"], 0, 6).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{1}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 172 — Winds of Change
// Audit: blocked — Needs a hidden-zone decision and continuation for “Each player shuffles the cards from their hand into their library, then draws that many cards”.

// LEG 173 — Aisling Leprechaun
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked by a creature, that creature becomes green”.

// LEG 174 — Arboria
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Creatures can't attack a player unless that player cast a spell or put a nontoken permanent onto the battlefield during their last turn”.

// LEG 175 — Avoid Fate
// Audit: blocked — Needs a spell-on-stack target predicate that expresses the printed instant/Aura restriction for “Counter target instant or Aura spell that targets a permanent you control”.

// LEG 176 — Barbary Apes
pub(in crate::card::sets) static BARBARY_APES: CardRecord = CardRecord::new(
    cards::BARBARY_APES,
    "Barbary Apes",
    CardArt::new("df25ffdd-995d-46ae-856b-f6368f9438ed", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ape"], 2, 2),
);

// LEG 177 — Cat Warriors
pub(in crate::card::sets) static CAT_WARRIORS: CardRecord = CardRecord::new(
    cards::CAT_WARRIORS,
    "Cat Warriors",
    CardArt::new("d2187a64-2823-4f58-ad35-70f8913db2dc", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Cat", "Warrior"], 2, 2)
        .with_ability(abilities::forestwalk()),
);

// LEG 178 — Cocoon
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Enchanted creature doesn't untap during your untap step if this Aura has a pupa counter on it”.

// LEG 179 — Concordant Crossroads
// Audit: partial — Its global haste effect is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static CONCORDANT_CROSSROADS: CardRecord = CardRecord::new(
    cards::CONCORDANT_CROSSROADS,
    "Concordant Crossroads",
    CardArt::new("3bdcfae4-86c9-4d8a-bcfe-f0a928ec29db", "Amy Weber"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures have haste.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Haste granting is executable, but the world-rule state-based action is not implemented.",
        ))),
);

// LEG 180 — Craw Giant
pub(in crate::card::sets) static CRAW_GIANT: CardRecord = CardRecord::new(
    cards::CRAW_GIANT,
    "Craw Giant",
    CardArt::new("707dadf0-735f-445d-9240-e49660913314", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}{G}"), &["Giant"], 6, 4).with_abilities(&[
        abilities::trample(),
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 181 — Deadfall
pub(in crate::card::sets) static DEADFALL: CardRecord = CardRecord::new(
    cards::DEADFALL,
    "Deadfall",
    CardArt::new("0d78f0fc-3ab2-46ee-b5a9-55ae97d08c1a", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with forestwalk can be blocked as though they didn't have forestwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Forest),
    )]),
);

// LEG 182 — Durkwood Boars
pub(in crate::card::sets) static DURKWOOD_BOARS: CardRecord = CardRecord::new(
    cards::DURKWOOD_BOARS,
    "Durkwood Boars",
    CardArt::new("8d41f08b-68fb-45f2-bdc9-488baedc7d6f", "Mike Kimble"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Boar"], 4, 4),
);

// LEG 183 — Elven Riders
pub(in crate::card::sets) static ELVEN_RIDERS: CardRecord = CardRecord::new(
    cards::ELVEN_RIDERS,
    "Elven Riders",
    CardArt::new("ad1d349b-b5ab-4b2b-9b39-f8d8f6374aa5", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elf"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by Walls and/or creatures with flying.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Not(
                    &ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Wall"),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// LEG 184 — Emerald Dragonfly
pub(in crate::card::sets) static EMERALD_DRAGONFLY: CardRecord = CardRecord::new(
    cards::EMERALD_DRAGONFLY,
    "Emerald Dragonfly",
    CardArt::new("a3e81250-52c3-49f6-be43-17c34339e177", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}{G}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 185 — Eureka
// Audit: blocked — Needs a hidden-zone decision and continuation for “Starting with you, each player may put a permanent card from their hand onto the battlefield. Repeat this process until no one puts a card onto the battlefield”.

// LEG 186 — Fire Sprites
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{G}, {T}: Add {R}”.

// LEG 187 — Floral Spuzzem
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature attacks and isn't blocked, you may destroy target artifact defending player controls. If you do, this creature assigns no combat damage this turn”.

// LEG 188 — Giant Turtle
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack if it attacked during your last turn”.

// LEG 189 — Glyph of Reincarnation
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “Destroy all creatures that were blocked by target Wall this turn. They can't be regenerated. For each creature that died this way, put a creature card from the graveyard of the player…”.

// LEG 190 — Hornet Cobra
pub(in crate::card::sets) static HORNET_COBRA: CardRecord = CardRecord::new(
    cards::HORNET_COBRA,
    "Hornet Cobra",
    CardArt::new("27180bad-9bbc-462b-8832-626dc403a3fd", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Snake"], 2, 1)
        .with_ability(abilities::first_strike()),
);

// LEG 191 — Ichneumon Druid
// Audit: blocked — Needs per-player, per-turn instant-cast counts in the triggering condition.

// LEG 192 — Killer Bees
pub(in crate::card::sets) static KILLER_BEES: CardRecord = CardRecord::new(
    cards::KILLER_BEES,
    "Killer Bees",
    CardArt::new("2e30b5ff-1239-4c4d-ac7c-554ecf8e1e27", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Insect"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 193 — Living Plane
// Audit: blocked — Needs static animation to continuously turn the matching lands into creatures for “All lands are 1/1 creatures that are still lands”.

// LEG 194 — Master of the Hunt
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 195 — Moss Monster
pub(in crate::card::sets) static MOSS_MONSTER: CardRecord = CardRecord::new(
    cards::MOSS_MONSTER,
    "Moss Monster",
    CardArt::new("9903c043-9a7a-4994-b532-136d4c46edfd", "Jesper Myrfors"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 3, 6),
);

// LEG 196 — Pixie Queen
pub(in crate::card::sets) static PIXIE_QUEEN: CardRecord = CardRecord::new(
    cards::PIXIE_QUEEN,
    "Pixie Queen",
    CardArt::new("b9527c2a-23bb-4d33-9e72-6e0ab3de0e6b", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{G}{G}{G}, {T}: Target creature gains flying until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::GrantAbility(&abilities::flying()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 197 — Pradesh Gypsies
pub(in crate::card::sets) static PRADESH_GYPSIES: CardRecord = CardRecord::new(
    cards::PRADESH_GYPSIES,
    "Pradesh Gypsies",
    CardArt::new("0370330d-83d9-44d2-a1ed-c4827edc60fd", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Nomad"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{G}, {T}: Target creature gets -2/-0 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(-2),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 198 — Rabid Wombat
// Audit: blocked — Needs a dynamic value that counts attached Auras for “This creature gets +2/+2 for each Aura attached to it”.

// LEG 199 — Radjan Spirit
pub(in crate::card::sets) static RADJAN_SPIRIT: CardRecord = CardRecord::new(
    cards::RADJAN_SPIRIT,
    "Radjan Spirit",
    CardArt::new("adf3ab1a-5714-4b69-bc51-3752312b2d1f", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spirit"], 3, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature loses flying until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::RemoveAbilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 200 — Rebirth
// Audit: blocked — Needs a separate optional ante choice for each player and a conditional life-total-setting continuation for each player who antes.

// LEG 201 — Reincarnation
// Audit: blocked — Needs duration-aware control-changing continuous effects for “Choose target creature. When that creature dies this turn, return a creature card from its owner's graveyard to the battlefield under the control of that creature's owner”.

// LEG 202 — Revelation
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Players play with their hands revealed”.

// LEG 203 — Rust
// Audit: blocked — Needs stack targeting and countering of ability objects for “Counter target activated ability from an artifact source”.

// LEG 204 — Shelkin Brownie
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 205 — Storm Seeker
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Storm Seeker deals damage to target player equal to the number of cards in that player's hand”.

// LEG 206 — Subdue
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all combat damage that would be dealt by target creature this turn. That creature gets +0/+X until end of turn, where X is its mana value”.

// LEG 207 — Sylvan Library
pub(in crate::card::sets) static SYLVAN_LIBRARY: CardRecord = CardRecord::new(
    cards::SYLVAN_LIBRARY,
    "Sylvan Library",
    CardArt::new("f486df00-7c4a-4ff0-bb0b-c8b5432ac742", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
    .with_abilities(&[AbilityDef::triggered(
        "At the beginning of your draw step, you may draw two additional cards. If you do, choose two cards in your hand drawn this turn. For each of those cards, pay 4 life or put the card on top of your library.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Draw,
            player: PlayerRelation::You,
        },
        EffectDef::Special("Offer the extra draws, then settle each chosen card"),
    )
    .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::SylvanLibrary))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The trigger is declarative and uses the shared stack; the card-local resolver offers the draws and then the pay-or-top choice for each card drawn this turn.",
    ))]),
);

static TYPHOON_OPPONENT_ISLANDS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::Opponent,
};

// LEG 208 — Sylvan Paradise
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “One or more target creatures become green until end of turn”.

// LEG 209 — Typhoon
pub(in crate::card::sets) static TYPHOON: CardRecord = CardRecord::new(
    cards::TYPHOON,
    "Typhoon",
    CardArt::new("254e0403-67d8-4e73-8d89-c901ebeba49f", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Typhoon deals damage to each opponent equal to the number of Islands that player controls.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::CountMatchingObjects(&TYPHOON_OPPONENT_ISLANDS),
        },
    )),
);

// LEG 210 — Untamed Wilds
pub(in crate::card::sets) static UNTAMED_WILDS: CardRecord = CardRecord::new(
    cards::UNTAMED_WILDS,
    "Untamed Wilds",
    CardArt::new("887f22af-8b92-422a-9cd5-f3977674bcdc", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Search your library for a basic land card, put that card onto the battlefield, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
        },
    )),
);

static DERVISH_DREW_BLOOD: TriggerConditionDef =
    TriggerConditionDef::SourceDealtDamageToOpponentThisTurn;

// LEG 211 — Whirling Dervish
pub(in crate::card::sets) static WHIRLING_DERVISH: CardRecord = CardRecord::new(
    cards::WHIRLING_DERVISH,
    "Whirling Dervish",
    CardArt::new("eba294e7-7097-4bc3-b396-72e85dd4f441", "Susan Van Camp"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Human", "Monk"], 1, 1)
        .with_abilities(&[
            abilities::protection_from(ManaColor::Black),
            AbilityDef::triggered_if(
                "At the beginning of each end step, if this creature dealt damage to an opponent this turn, put a +1/+1 counter on it.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                &DERVISH_DREW_BLOOD,
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEG 212 — Willow Satyr
pub(in crate::card::sets) static WILLOW_SATYR: CardRecord = CardRecord::new(
    cards::WILLOW_SATYR,
    "Willow Satyr",
    CardArt::new("0c8b1f49-550e-405f-b17c-1d94589494ad", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Satyr"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this creature during your untap step.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::MayChooseNotToUntap,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Gain control of target legendary creature for as long as you control this \
                 creature and this creature remains tapped.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                ]),
            )],
            EffectDef::GainControlWhileSourceRemains {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                while_tapped: true,
            },
        ),
    ]),
);

// LEG 213 — Winter Blast
// Audit: blocked — Needs one chosen-X target set with a flying-dependent damage follow-up linked to the creatures it tapped.

// LEG 214 — Wolverine Pack
pub(in crate::card::sets) static WOLVERINE_PACK: CardRecord = CardRecord::new(
    cards::WOLVERINE_PACK,
    "Wolverine Pack",
    CardArt::new("ba5aee52-095e-4c69-93eb-5adac11ed1fc", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Wolverine"], 2, 4).with_abilities(&[
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 215 — Wood Elemental
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Wood Elemental's power and toughness are each equal to the number of Forests sacrificed as it entered”.

// LEG 216 — Adun Oakenshield
pub(in crate::card::sets) static ADUN_OAKENSHIELD: CardRecord = CardRecord::new(
    cards::ADUN_OAKENSHIELD,
    "Adun Oakenshield",
    CardArt::new("60252226-a102-4d88-9b80-42d021b5184d", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{B}{R}{G}"), &["Human", "Knight"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{B}{R}{G}, {T}: Return target creature card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}{R}{G}")),
                AbilityCostDef::TapSource,
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
                controller: None,
            },
        )),
);

// LEG 217 — Angus Mackenzie
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{G}{W}{U}, {T}: Prevent all combat damage that would be dealt this turn. Activate only before the combat damage step”.

// LEG 218 — Arcades Sabboth
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Each untapped creature you control gets +0/+2 as long as it's not attacking”.

// LEG 219 — Axelrod Gunnarson
pub(in crate::card::sets) static AXELROD_GUNNARSON: CardRecord = CardRecord::new(
    cards::AXELROD_GUNNARSON,
    "Axelrod Gunnarson",
    CardArt::new("acce83cf-965b-4e45-8efb-63f814df7a35", "Scott Kirschner"),
    CardSet::Legends,
    CardRules::new_creature(
        mana_cost!("{4}{B}{B}{R}{R}"),
        &["Giant"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Whenever a creature dealt damage by Axelrod Gunnarson this turn dies, you gain 1 life and Axelrod Gunnarson deals 1 damage to any target.",
            TriggerEventDef::DamagedCreatureDied,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// LEG 220 — Ayesha Tanaka
// Audit: blocked — Needs stack targeting and countering of ability objects for “{T}: Counter target activated ability from an artifact source unless that ability's controller pays {W}”.

// LEG 221 — Barktooth Warbeard
pub(in crate::card::sets) static BARKTOOTH_WARBEARD: CardRecord = CardRecord::new(
    cards::BARKTOOTH_WARBEARD,
    "Barktooth Warbeard",
    CardArt::new("0ea52228-f8ad-4623-9e05-f162473bfc03", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{B}{R}{R}"), &["Human", "Warrior"], 6, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 222 — Bartel Runeaxe
// Audit: blocked — Needs the card's exact Aura targeting/attachment restriction rather than the broader existing cannot-be-enchanted effect for “Bartel Runeaxe can't be the target of Aura spells”.

// LEG 223 — Boris Devilboon
pub(in crate::card::sets) static BORIS_DEVILBOON: CardRecord = CardRecord::new(
    cards::BORIS_DEVILBOON,
    "Boris Devilboon",
    CardArt::new("82ae30e8-2dcd-46b8-925b-cc24e11fb95d", "Jesper Myrfors"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{B}{R}"), &["Zombie", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{2}{B}{R}, {T}: Create a 1/1 black and red Demon creature token named Minor Demon.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{R}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::CreateToken {
                token: cards::MINOR_DEMON_TOKEN_1_1_BLACK_RED,
                count: ValueDef::Constant(1),
            },
        )),
);

// LEG 224 — Chromium
pub(in crate::card::sets) static CHROMIUM: CardRecord = CardRecord::new(
    cards::CHROMIUM,
    "Chromium",
    CardArt::new(
        "8cd7d7e1-f928-4429-9a59-ba0590a78e98",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(
        mana_cost!("{2}{W}{W}{U}{U}{B}{B}"),
        &["Elder", "Dragon"],
        7,
        7,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice Chromium unless you pay {W}{U}{B}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{W}{U}{B}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// LEG 225 — Dakkon Blackblade
// Audit: partial — Its power and toughness are a battlefield-only continuous effect rather than a characteristic-defining ability, so they read as printed in every other zone.
pub(in crate::card::sets) static DAKKON_BLACKBLADE: CardRecord = CardRecord::new(
    cards::DAKKON_BLACKBLADE,
    "Dakkon Blackblade",
    CardArt::new(
        "fbfd1278-1486-4516-8846-007ce1985ee9",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{W}{U}{U}{B}"), &["Human", "Warrior"], 0, 0)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Dakkon Blackblade's power and toughness are each equal to the number of lands you \
             control.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    toughness: ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "A characteristic-defining ability sets power and toughness in every zone. This is a \
             battlefield-only continuous effect, so the value is right wherever the card is \
             played and absent for anything reading it in another zone.",
        ))),
);

static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Land),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// LEG 226 — Gabriel Angelfire
// Audit: blocked — Needs a random choice among four named abilities and a grant of the chosen one; the randomized effect vocabulary selects between two branches, not among four.

// LEG 227 — Gosta Dirk
pub(in crate::card::sets) static GOSTA_DIRK: CardRecord = CardRecord::new(
    cards::GOSTA_DIRK,
    "Gosta Dirk",
    CardArt::new("92ef316b-dd22-40d1-82e8-8890976684c0", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{W}{W}{U}{U}"), &["Human", "Warrior"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "Creatures with islandwalk can be blocked as though they didn't have islandwalk.",
                EffectDef::LandwalkCanBeBlocked(BasicLandType::Island),
            ),
        ]),
);

// LEG 228 — Gwendlyn Di Corci
pub(in crate::card::sets) static GWENDLYN_DI_CORCI: CardRecord = CardRecord::new(
    cards::GWENDLYN_DI_CORCI,
    "Gwendlyn Di Corci",
    CardArt::new("473d70b6-a88c-49f4-9415-19919c4468ae", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{U}{B}{B}{R}"), &["Human", "Rogue"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(
            AbilityDef::activated_with_targets(
                "{T}: Target player discards a card at random. Activate only during your turn.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::Random,
                },
            )
            .with_activation_timing(ActivationTimingDef::YourTurn),
        ),
);

// LEG 229 — Halfdane
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “At the beginning of your upkeep, change Halfdane's base power and toughness to the power and toughness of target creature other than Halfdane until the end of your next upkeep”.

// LEG 230 — Hazezon Tamar
// Audit: blocked — Needs the exact token definition and creation/lifecycle behavior for “When Hazezon enters, create X 1/1 Sand Warrior creature tokens that are red, green, and white at the beginning of your next upkeep, where X is the number of lands you control at that time”.

// LEG 231 — Hunding Gjornersen
pub(in crate::card::sets) static HUNDING_GJORNERSEN: CardRecord = CardRecord::new(
    cards::HUNDING_GJORNERSEN,
    "Hunding Gjornersen",
    CardArt::new("07d8e501-6857-4a52-a3b9-2bf0bee5b08c", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{W}{U}{U}"), &["Human", "Warrior"], 5, 4)
        .with_supertype(CardSupertype::Legendary).with_abilities(&[
        abilities::rampage(1, "Rampage 1 (Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 232 — Jacques le Vert
pub(in crate::card::sets) static JACQUES_LE_VERT: CardRecord = CardRecord::new(
    cards::JACQUES_LE_VERT,
    "Jacques le Vert",
    CardArt::new("ee5a45b1-169b-468e-9251-424c09cd7f0f", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{R}{G}{W}"), &["Human", "Warrior"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Green creatures you control get +0/+2.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(0),
                    toughness: ValueDef::Constant(2),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )),
);

// LEG 233 — Jasmine Boreal
pub(in crate::card::sets) static JASMINE_BOREAL: CardRecord = CardRecord::new(
    cards::JASMINE_BOREAL,
    "Jasmine Boreal",
    CardArt::new(
        "db6ef678-4ce9-48d6-aa4f-2afd9a1ad724",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Human"], 4, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 234 — Jedit Ojanen
pub(in crate::card::sets) static JEDIT_OJANEN: CardRecord = CardRecord::new(
    cards::JEDIT_OJANEN,
    "Jedit Ojanen",
    CardArt::new("97b80124-2b59-425c-93cc-9b032e631c6e", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{W}{W}{U}"), &["Cat", "Warrior"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 235 — Jerrard of the Closed Fist
pub(in crate::card::sets) static JERRARD_OF_THE_CLOSED_FIST: CardRecord = CardRecord::new(
    cards::JERRARD_OF_THE_CLOSED_FIST,
    "Jerrard of the Closed Fist",
    CardArt::new("7f841918-813b-4784-ab57-907185b0a355", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{R}{G}{G}"), &["Human", "Knight"], 6, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 236 — Johan
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “At the beginning of combat on your turn, you may have Johan gain "Johan can't attack" until end of combat. If you do, attacking doesn't cause creatures you control to tap this combat if…”.

// LEG 237 — Kasimir the Lone Wolf
pub(in crate::card::sets) static KASIMIR_THE_LONE_WOLF: CardRecord = CardRecord::new(
    cards::KASIMIR_THE_LONE_WOLF,
    "Kasimir the Lone Wolf",
    CardArt::new(
        "45b1e60d-54dd-41cd-b9a2-00890725a3df",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{W}{U}"), &["Human", "Warrior"], 5, 3)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 238 — Kei Takahashi
pub(in crate::card::sets) static KEI_TAKAHASHI: CardRecord = CardRecord::new(
    cards::KEI_TAKAHASHI,
    "Kei Takahashi",
    CardArt::new("6a4a524a-fdc7-432d-994b-953808528349", "Scott Kirschner"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Human", "Cleric"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Prevent the next 2 damage that would be dealt to target creature this turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventNextDamage {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        )),
);

// LEG 239 — Lady Caleria
pub(in crate::card::sets) static LADY_CALERIA: CardRecord = CardRecord::new(
    cards::LADY_CALERIA,
    "Lady Caleria",
    CardArt::new("d6914ed2-9207-4689-9166-11d2f8949fdd", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{W}{W}"), &["Elf", "Archer"], 3, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Lady Caleria deals 3 damage to target attacking or blocking creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        )),
);

// LEG 240 — Lady Evangela
pub(in crate::card::sets) static LADY_EVANGELA: CardRecord = CardRecord::new(
    cards::LADY_EVANGELA,
    "Lady Evangela",
    CardArt::new("f3e122e9-ffa3-48dd-94d6-8f2886668e59", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{W}{U}{B}"), &["Human", "Cleric"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{W}{B}, {T}: Prevent all combat damage that would be dealt by target creature this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventCombatDamageDealtByThisTurn {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )),
);

// LEG 241 — Lady Orca
pub(in crate::card::sets) static LADY_ORCA: CardRecord = CardRecord::new(
    cards::LADY_ORCA,
    "Lady Orca",
    CardArt::new("b2779553-74eb-42ba-97d0-96269f48c269", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{5}{B}{R}"), &["Demon"], 7, 4)
        .with_supertype(CardSupertype::Legendary),
);

const NICOL_BOLAS_ENTIRE_HAND: ValueDef = ValueDef::Constant(i32::MAX);

// LEG 242 — Livonya Silone
pub(in crate::card::sets) static LIVONYA_SILONE: CardRecord = CardRecord::new(
    cards::LIVONYA_SILONE,
    "Livonya Silone",
    CardArt::new(
        "b9211949-66a5-4039-ac6d-3e42b008b58e",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}{R}{G}{G}"), &["Human", "Warrior"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::first_strike(), abilities::legendary_landwalk()]),
);

// LEG 243 — Lord Magnus
pub(in crate::card::sets) static LORD_MAGNUS: CardRecord = CardRecord::new(
    cards::LORD_MAGNUS,
    "Lord Magnus",
    CardArt::new("2a02aabb-c464-4672-b37b-d5d713ef8939", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{W}{W}"), &["Human", "Druid"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "Creatures with plainswalk can be blocked as though they didn't have plainswalk.",
                EffectDef::LandwalkCanBeBlocked(BasicLandType::Plains),
            ),
            AbilityDef::static_ability(
                "Creatures with forestwalk can be blocked as though they didn't have forestwalk.",
                EffectDef::LandwalkCanBeBlocked(BasicLandType::Forest),
            ),
        ]),
);

// LEG 244 — Marhault Elsdragon
pub(in crate::card::sets) static MARHAULT_ELSDRAGON: CardRecord = CardRecord::new(
    cards::MARHAULT_ELSDRAGON,
    "Marhault Elsdragon",
    CardArt::new("67330004-6720-46d9-9de0-c79230110583", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{R}{R}{G}"), &["Elf", "Warrior"], 4, 6)
        .with_supertype(CardSupertype::Legendary).with_abilities(&[
        abilities::rampage(1, "Rampage 1 (Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 245 — Nebuchadnezzar
// Audit: blocked — Needs seeded random selection with replay-visible provenance for “{X}, {T}: Choose a card name. Target opponent reveals X cards at random from their hand. Then that player discards all cards with that name revealed this way. Activate only during your turn”.

// LEG 246 — Nicol Bolas
pub(in crate::card::sets) static NICOL_BOLAS: CardRecord = CardRecord::new(
    cards::NICOL_BOLAS,
    "Nicol Bolas",
    CardArt::new(
        "729feb73-4581-4f9d-ba47-bece72481b86",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(
        mana_cost!("{2}{U}{U}{B}{B}{R}{R}"),
        &["Elder", "Dragon"],
        7,
        7,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice Nicol Bolas unless you pay {U}{B}{R}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{U}{B}{R}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever Nicol Bolas deals damage to an opponent, that player discards their hand.",
            TriggerEventDef::DamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
                player: PlayerRelation::Opponent,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: NICOL_BOLAS_ENTIRE_HAND,
                selection: DiscardSelectionDef::RecipientChooses,
            },
        ),
    ]),
);

// LEG 247 — Palladia-Mors
pub(in crate::card::sets) static PALLADIA_MORS: CardRecord = CardRecord::new(
    cards::PALLADIA_MORS,
    "Palladia-Mors",
    CardArt::new(
        "ad64874d-ce33-4e0a-bcca-723f129ef415",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}{G}{G}{W}{W}"),
        &["Elder", "Dragon"],
        7,
        7,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::trample(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice Palladia-Mors unless you pay {R}{G}{W}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{R}{G}{W}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// LEG 248 — Pavel Maliki
pub(in crate::card::sets) static PAVEL_MALIKI: CardRecord = CardRecord::new(
    cards::PAVEL_MALIKI,
    "Pavel Maliki",
    CardArt::new("304f9d39-3ea2-4274-b23e-e4eaabbc1c4b", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{B}{R}"), &["Human"], 5, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{B}{R}: Pavel Maliki gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// LEG 249 — Princess Lucrezia
pub(in crate::card::sets) static PRINCESS_LUCREZIA: CardRecord = CardRecord::new(
    cards::PRINCESS_LUCREZIA,
    "Princess Lucrezia",
    CardArt::new(
        "a1dcf48c-2700-4024-807e-9244e4c649ac",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{U}{U}{B}"), &["Human", "Wizard"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::tap_for(ManaColor::Blue)),
);

// LEG 250 — Ragnar
pub(in crate::card::sets) static RAGNAR: CardRecord = CardRecord::new(
    cards::RAGNAR,
    "Ragnar",
    CardArt::new("2cf6a3a3-4a06-4eb7-981a-b70cf05b2473", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{G}{W}{U}"), &["Human", "Cleric"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{G}{W}{U}, {T}: Regenerate target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{W}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )]),
);

// LEG 251 — Ramirez DePietro
pub(in crate::card::sets) static RAMIREZ_DEPIETRO: CardRecord = CardRecord::new(
    cards::RAMIREZ_DEPIETRO,
    "Ramirez DePietro",
    CardArt::new("e5c66c61-aadf-433b-9958-fc9b44b327b9", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{U}{B}{B}"), &["Human", "Pirate"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::first_strike()),
);

// LEG 252 — Ramses Overdark
// Audit: blocked — Needs linked sacrifice/destruction accounting for “{T}: Destroy target enchanted creature”.

// LEG 253 — Rasputin Dreamweaver
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Remove a dream counter from Rasputin: Prevent the next 1 damage that would be dealt to Rasputin this turn”.

// LEG 254 — Riven Turnbull
pub(in crate::card::sets) static RIVEN_TURNBULL: CardRecord = CardRecord::new(
    cards::RIVEN_TURNBULL,
    "Riven Turnbull",
    CardArt::new(
        "d11f90e7-ced1-4d80-8083-99acbf459ad7",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{5}{U}{B}"), &["Human", "Advisor"], 5, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::tap_for(ManaColor::Black)),
);

// LEG 255 — Rohgahh of Kher Keep
// Audit: blocked — Needs duration-aware control-changing continuous effects for “At the beginning of your upkeep, you may pay {R}{R}{R}. If you don't, tap Rohgahh and all creatures named Kobolds of Kher Keep, then an opponent gains control of them”.

// LEG 256 — Rubinia Soulsinger
pub(in crate::card::sets) static RUBINIA_SOULSINGER: CardRecord = CardRecord::new(
    cards::RUBINIA_SOULSINGER,
    "Rubinia Soulsinger",
    CardArt::new("f13e8dc9-8d0f-4a2c-8c0e-be70a3a7dc8e", "Rob Alexander"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{G}{W}{U}"), &["Faerie"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may choose not to untap Rubinia Soulsinger during your untap step.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::MayChooseNotToUntap,
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Gain control of target creature for as long as you control Rubinia \
                 Soulsinger and Rubinia Soulsinger remains tapped.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::GainControlWhileSourceRemains {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    while_tapped: true,
                },
            ),
        ]),
);

// LEG 257 — Sir Shandlar of Eberyn
pub(in crate::card::sets) static SIR_SHANDLAR_OF_EBERYN: CardRecord = CardRecord::new(
    cards::SIR_SHANDLAR_OF_EBERYN,
    "Sir Shandlar of Eberyn",
    CardArt::new("31570ded-f5e3-44c4-b95f-294ac10b2cd2", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{G}{W}"), &["Human", "Knight"], 4, 7)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 258 — Sivitri Scarzam
pub(in crate::card::sets) static SIVITRI_SCARZAM: CardRecord = CardRecord::new(
    cards::SIVITRI_SCARZAM,
    "Sivitri Scarzam",
    CardArt::new("9c12ee9e-db13-4b4d-a061-b6566f538f09", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{5}{U}{B}"), &["Human"], 6, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 259 — Sol'kanar the Swamp King
// Audit: blocked — Needs a spell-color predicate in trigger capture for “Whenever a player casts a black spell, you gain 1 life”.

// LEG 260 — Stangg
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “When Stangg enters, create Stangg Twin, a legendary 3/4 red and green Human Warrior creature token. Exile that token when Stangg leaves the battlefield. Sacrifice Stangg when that token…”.

// LEG 261 — Sunastian Falconer
pub(in crate::card::sets) static SUNASTIAN_FALCONER: CardRecord = CardRecord::new(
    cards::SUNASTIAN_FALCONER,
    "Sunastian Falconer",
    CardArt::new("587075f3-a568-4089-83ca-fe1e473c025d", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Human", "Shaman"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        )),
);

// LEG 262 — Tetsuo Umezawa
pub(in crate::card::sets) static TETSUO_UMEZAWA: CardRecord = CardRecord::new(
    cards::TETSUO_UMEZAWA,
    "Tetsuo Umezawa",
    CardArt::new("8384f87b-26c2-45b7-98ef-352c384f205e", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{U}{B}{R}"), &["Human", "Archer"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Tetsuo Umezawa can't be the target of Aura spells.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::CannotBeEnchanted,
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::activated_with_targets(
                "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking creature.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{U}{B}{B}{R}")),
                    AbilityCostDef::TapSource,
                ],
                &TAPPED_OR_BLOCKING_CREATURE,
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
        ]),
);

static TAPPED_OR_BLOCKING_CREATURE: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Tapped, ObjectPredicateDef::Blocking]),
        ]),
    )];

// LEG 263 — The Lady of the Mountain
pub(in crate::card::sets) static THE_LADY_OF_THE_MOUNTAIN: CardRecord = CardRecord::new(
    cards::THE_LADY_OF_THE_MOUNTAIN,
    "The Lady of the Mountain",
    CardArt::new(
        "83717eb2-220e-4086-be09-dee9174798b8",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{R}{G}"), &["Giant"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 264 — Tobias Andrion
pub(in crate::card::sets) static TOBIAS_ANDRION: CardRecord = CardRecord::new(
    cards::TOBIAS_ANDRION,
    "Tobias Andrion",
    CardArt::new("cac56eda-5ed3-4abd-beec-f5063fbf930a", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{W}{U}"), &["Human", "Advisor"], 4, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 265 — Tor Wauki
pub(in crate::card::sets) static TOR_WAUKI: CardRecord = CardRecord::new(
    cards::TOR_WAUKI,
    "Tor Wauki",
    CardArt::new(
        "241a4854-e62c-4be4-a9cc-1e14db4eede9",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{B}{B}{R}"), &["Human", "Archer"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Tor Wauki deals 2 damage to target attacking or blocking creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        )),
);

// LEG 266 — Torsten Von Ursus
pub(in crate::card::sets) static TORSTEN_VON_URSUS: CardRecord = CardRecord::new(
    cards::TORSTEN_VON_URSUS,
    "Torsten Von Ursus",
    CardArt::new("5fd99522-4a91-4ccd-91bf-5f32a6ac3510", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{W}"), &["Human", "Soldier"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 267 — Tuknir Deathlock
pub(in crate::card::sets) static TUKNIR_DEATHLOCK: CardRecord = CardRecord::new(
    cards::TUKNIR_DEATHLOCK,
    "Tuknir Deathlock",
    CardArt::new("9dfbcb4d-a9ae-4d76-8dde-7312fbad56b0", "Liz Danforth"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{R}{R}{G}{G}"), &["Human", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{R}{G}, {T}: Target creature gets +2/+2 until end of turn.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{R}{G}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEG 268 — Ur-Drago
pub(in crate::card::sets) static UR_DRAGO: CardRecord = CardRecord::new(
    cards::UR_DRAGO,
    "Ur-Drago",
    CardArt::new("81a40f34-fc26-4d05-9c52-6ffbf1766a3b", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{U}{U}{B}{B}"), &["Elemental"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "Creatures with swampwalk can be blocked as though they didn't have swampwalk.",
                EffectDef::LandwalkCanBeBlocked(BasicLandType::Swamp),
            ),
        ]),
);

// LEG 269 — Vaevictis Asmadi
pub(in crate::card::sets) static VAEVICTIS_ASMADI: CardRecord = CardRecord::new(
    cards::VAEVICTIS_ASMADI,
    "Vaevictis Asmadi",
    CardArt::new("22ea73ec-1325-4437-a23f-dcda1767c713", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}{R}{R}{G}{G}"),
        &["Elder", "Dragon"],
        7,
        7,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice Vaevictis Asmadi unless you pay {B}{R}{G}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{B}{R}{G}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
        AbilityDef::activated(
            "{B}: Vaevictis Asmadi gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{R}: Vaevictis Asmadi gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{G}: Vaevictis Asmadi gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 270 — Xira Arien
pub(in crate::card::sets) static XIRA_ARIEN: CardRecord = CardRecord::new(
    cards::XIRA_ARIEN,
    "Xira Arien",
    CardArt::new("cc6c7d89-32e7-4c3f-ac90-7db3a46eed4b", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{B}{R}{G}"), &["Insect", "Wizard"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{B}{R}{G}, {T}: Target player draws a card.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{B}{R}{G}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEG 271 — Al-abara's Carpet
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{5}, {T}: Prevent all damage that would be dealt to you this turn by attacking creatures without flying”.

// LEG 272 — Alchor's Tomb
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “{2}, {T}: Target permanent you control becomes the color of your choice”.

// LEG 273 — Arena of the Ancients
// Audit: partial — External static untap suppression cannot yet apply to every matching legendary creature through the ability-layer fixed point.
pub(in crate::card::sets) static ARENA_OF_THE_ANCIENTS: CardRecord = CardRecord::new(
    cards::ARENA_OF_THE_ANCIENTS,
    "Arena of the Ancients",
    CardArt::new("9337996e-a119-4529-b422-f6d286c78e3f", "Tom Wänerstrand"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::not_implemented(
            "Legendary creatures don't untap during their controllers' untap steps.",
            "Current untap suppression is discovered only on the permanent's own static abilities; an external static effect cannot suppress a matching set without granting an executable static ability, which requires fixed-point ability-layer evaluation.",
        ),
        AbilityDef::triggered(
            "When this artifact enters, tap all legendary creatures.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Tap {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
            },
        ),
    ]),
);

// LEG 274 — Black Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {B}, then add an additional {B} for each charge counter removed this way”.

// LEG 275 — Blue Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {U}, then add an additional {U} for each charge counter removed this way”.

// LEG 276 — Bronze Horse
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “As long as you control another creature, prevent all damage that would be dealt to this creature by spells that target it”.

// LEG 277 — Forethought Amulet
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “If an instant or sorcery source would deal 3 or more damage to you, it deals 2 damage to you instead”.

// LEG 278 — Gauntlets of Chaos
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{5}, Sacrifice this artifact: Exchange control of target artifact, creature, or land you control and target permanent an opponent controls that shares one of those types with it. If…”.

// LEG 279 — Green Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {G}, then add an additional {G} for each charge counter removed this way”.

// LEG 280 — Horn of Deafening
pub(in crate::card::sets) static HORN_OF_DEAFENING: CardRecord = CardRecord::new(
    cards::HORN_OF_DEAFENING,
    "Horn of Deafening",
    CardArt::new("17eff8d9-86de-4f19-bf00-5f20dc1373d4", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: Prevent all combat damage that would be dealt by target creature this turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventCombatDamageDealtByThisTurn {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// LEG 281 — Knowledge Vault
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{2}, {T}: Exile the top card of your library face down”.

// LEG 282 — Kry Shield
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{2}, {T}: Prevent all damage that would be dealt this turn by target creature you control. That creature gets +0/+X until end of turn, where X is its mana value”.

// LEG 283 — Life Chisel
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Sacrifice a creature: You gain life equal to the sacrificed creature's toughness. Activate only during your upkeep”.

// LEG 284 — Life Matrix
// Audit: blocked — Needs granting a counter-consuming activated ability to a targeted creature and an activation window restricted to your upkeep for “{4}, {T}: Put a matrix counter on target creature and that creature gains "Remove a matrix counter from this creature: Regenerate this creature." Activate only during your upkeep”.

// LEG 285 — Mana Matrix
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Instant and enchantment spells you cast cost {2} less to cast”.

// LEG 286 — Marble Priest
// Audit: blocked — Needs a must-block requirement for “All Walls able to block this creature do so”; preventing the damage those Walls deal is already expressible.

// LEG 287 — Mirror Universe
// Audit: blocked — Needs linked sacrifice/destruction accounting for “{T}, Sacrifice this artifact: Exchange life totals with target opponent. Activate only during your upkeep”.

// LEG 288 — North Star
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “{4}, {T}: For one spell this turn, you may spend mana as though it were mana of any type to pay that spell's mana cost”.

// LEG 289 — Nova Pentacle
// Audit: blocked — Needs a shield keyed to a source chosen as the ability resolves; prevention shields attach to a recipient and spend on the next damage from any source, not from one named source for “{3}, {T}: The next time a source of your choice would deal damage to you this turn, that damage is dealt to target creature of an opponent's choice instead”.

// LEG 290 — Planar Gate
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Creature spells you cast cost {2} less to cast”.

// LEG 291 — Red Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {R}, then add an additional {R} for each charge counter removed this way”.

// LEG 292 — Relic Barrier
pub(in crate::card::sets) static RELIC_BARRIER: CardRecord = CardRecord::new(
    cards::RELIC_BARRIER,
    "Relic Barrier",
    CardArt::new("c062cbae-ce5e-43be-9932-c81a0a3622e8", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Tap target artifact.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
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

// LEG 293 — Ring of Immortals
// Audit: blocked — Needs a spell-on-stack target predicate that expresses the printed instant/Aura restriction for “{3}, {T}: Counter target instant or Aura spell that targets a permanent you control”.

// LEG 294 — Sentinel
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{0}: Change this creature's base toughness to 1 plus the power of target creature blocking or blocked by this creature”.

// LEG 295 — Serpent Generator
// Audit: blocked — Needs player poison counters and the poison-based state check, including this card's counter placement.

// LEG 296 — Sword of the Ages
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “{T}, Sacrifice this artifact and any number of creatures you control: This artifact deals X damage to any target, where X is the total power of the creatures sacrificed this way, then…”.

// LEG 297 — Triassic Egg
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Sacrifice this artifact: Choose one. Activate only if there are two or more hatchling counters on this artifact”.

// LEG 298 — Voodoo Doll
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of your end step, if this artifact is untapped, destroy this artifact and it deals damage to you equal to the number of pin counters on it”.

// LEG 299 — White Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {W}, then add an additional {W} for each charge counter removed this way”.

// LEG 300 — Adventurers' Guildhouse
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 301 — Cathedral of Serra
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 302 — Hammerheim
// Audit: blocked — Needs removing every ability of a class from a target for a duration; the vocabulary grants named abilities but does not take them away.

// LEG 303 — Karakas
pub(in crate::card::sets) static KARAKAS: CardRecord = CardRecord::new(
    cards::KARAKAS,
    "Karakas",
    CardArt::new("31d2422a-bb7d-4cdd-9aac-e5a936a4be3b", "Nicola Leonard"),
    CardSet::Legends,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::White),
            AbilityDef::activated_with_targets(
                "{T}: Return target legendary creature to its owner's hand.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    controller: None,
                },
            ),
        ]),
);

/// "Target 1/1 creature" is read as the creature is now, so a creature that
/// has already been pumped is not one, and one that stops being 1/1 before
/// the ability resolves loses the ability with it.
static PENDELHAVEN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::PowerExactly(1),
            ObjectPredicateDef::ToughnessExactly(1),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

// LEG 304 — Mountain Stronghold
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 305 — Pendelhaven
// Audit: partial — The target's power and toughness omit modifiers from static continuous effects.
pub(in crate::card::sets) static PENDELHAVEN: CardRecord = CardRecord::new(
    cards::PENDELHAVEN,
    "Pendelhaven",
    CardArt::new("79427109-c1f3-476d-a029-0049217237b5", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Green),
            AbilityDef::activated_with_targets(
                "{T}: Target 1/1 creature gets +1/+2 until end of turn.",
                &[AbilityCostDef::TapSource],
                &PENDELHAVEN_TARGET,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The target's power and toughness omit modifiers from static continuous effects.",
            )),
        ]),
);

static TABERNACLE_UPKEEP_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, destroy this creature unless you pay {1}.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::UnlessPaid {
        cost: mana_cost!("{1}"),
        otherwise: &EffectDef::Destroy {
            object: EffectRecipientDef::Source,
            can_regenerate: true,
        },
    },
);

// LEG 306 — Seafarer's Quay
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 307 — The Tabernacle at Pendrell Vale
pub(in crate::card::sets) static THE_TABERNACLE_AT_PENDRELL_VALE: CardRecord = CardRecord::new(
    cards::THE_TABERNACLE_AT_PENDRELL_VALE,
    "The Tabernacle at Pendrell Vale",
    CardArt::new("64bc9b1d-5818-4d9e-b771-e49af4ff9a5c", "Nicola Leonard"),
    CardSet::Legends,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "All creatures have \"At the beginning of your upkeep, destroy this creature unless you pay {1}.\"",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::GrantAbility(&TABERNACLE_UPKEEP_ABILITY),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )),
);

// LEG 308 — Tolaria
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 309 — Unholy Citadel
// Audit: blocked — Needs band formation: creatures with banding cannot yet attack as a group, and a band is not blocked as one. Blocking with banding is implemented.

// LEG 310 — Urborg
// Audit: blocked — Needs a modal choice between two named abilities and the removal of the chosen one; the vocabulary grants named abilities but does not take them away.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AMROU_KITHKIN,
    &CLEANSE,
    &DAVENANT_ARCHER,
    &DIVINE_OFFERING,
    &DIVINE_TRANSFORMATION,
    &GREAT_DEFENDER,
    &GREAT_WALL,
    &GREATER_REALM_OF_PRESERVATION,
    &HOLY_DAY,
    &INDESTRUCTIBLE_AURA,
    &KEEPERS_OF_THE_FAITH,
    &KISMET,
    &LIFEBLOOD,
    &MOAT,
    &PRESENCE_OF_THE_MASTER,
    &RIGHTEOUS_AVENGERS,
    &SEEKER,
    &SHIELD_WALL,
    &SPIRIT_LINK,
    &SPIRITUAL_SANCTUARY,
    &THUNDER_SPIRIT,
    &TUNDRA_WOLVES,
    &WALL_OF_LIGHT,
    &ACID_RAIN,
    &AZURE_DRAKE,
    &BOOMERANG,
    &DEVOURING_DEEP,
    &FLASH_COUNTER,
    &FLASH_FLOOD,
    &FORCE_SPIKE,
    &GASEOUS_FORM,
    &MANA_DRAIN,
    &PSIONIC_ENTITY,
    &RECALL,
    &REMOVE_SOUL,
    &SEGOVIAN_LEVIATHAN,
    &UNDERTOW,
    &ZEPHYR_FALCON,
    &CARRION_ANTS,
    &CYCLOPEAN_MUMMY,
    &DARKNESS,
    &FALLEN_ANGEL,
    &GHOSTS_OF_THE_DAMNED,
    &GREED,
    &HEADLESS_HORSEMAN,
    &HELL_SWARM,
    &HELLS_CARETAKER,
    &HORROR_OF_HORRORS,
    &LOST_SOUL,
    &NETHER_VOID,
    &QUAGMIRE,
    &THE_ABYSS,
    &WALKING_DEAD,
    &ACTIVE_VOLCANO,
    &AERATHI_BERSERKER,
    &CHAIN_LIGHTNING,
    &CREVASSE,
    &CRIMSON_KOBOLDS,
    &CRIMSON_MANTICORE,
    &CROOKSHANK_KOBOLDS,
    &ETERNAL_WARRIOR,
    &FROST_GIANT,
    &GIANT_STRENGTH,
    &GRAVITY_SPHERE,
    &IMMOLATION,
    &KOBOLD_DRILL_SERGEANT,
    &KOBOLD_OVERLORD,
    &KOBOLD_TASKMASTER,
    &KOBOLDS_OF_KHER_KEEP,
    &MOUNTAIN_YETI,
    &PYROTECHNICS,
    &RAGING_BULL,
    &SPINAL_VILLAIN,
    &THE_BRUTE,
    &WALL_OF_EARTH,
    &WALL_OF_HEAT,
    &WALL_OF_OPPOSITION,
    &BARBARY_APES,
    &CAT_WARRIORS,
    &CONCORDANT_CROSSROADS,
    &CRAW_GIANT,
    &DEADFALL,
    &DURKWOOD_BOARS,
    &ELVEN_RIDERS,
    &EMERALD_DRAGONFLY,
    &HORNET_COBRA,
    &KILLER_BEES,
    &MOSS_MONSTER,
    &PIXIE_QUEEN,
    &PRADESH_GYPSIES,
    &RADJAN_SPIRIT,
    &SYLVAN_LIBRARY,
    &TYPHOON,
    &UNTAMED_WILDS,
    &WHIRLING_DERVISH,
    &WILLOW_SATYR,
    &WOLVERINE_PACK,
    &ADUN_OAKENSHIELD,
    &AXELROD_GUNNARSON,
    &BARKTOOTH_WARBEARD,
    &BORIS_DEVILBOON,
    &CHROMIUM,
    &DAKKON_BLACKBLADE,
    &GOSTA_DIRK,
    &GWENDLYN_DI_CORCI,
    &HUNDING_GJORNERSEN,
    &JACQUES_LE_VERT,
    &JASMINE_BOREAL,
    &JEDIT_OJANEN,
    &JERRARD_OF_THE_CLOSED_FIST,
    &KASIMIR_THE_LONE_WOLF,
    &KEI_TAKAHASHI,
    &LADY_CALERIA,
    &LADY_EVANGELA,
    &LADY_ORCA,
    &LIVONYA_SILONE,
    &LORD_MAGNUS,
    &MARHAULT_ELSDRAGON,
    &NICOL_BOLAS,
    &PALLADIA_MORS,
    &PAVEL_MALIKI,
    &PRINCESS_LUCREZIA,
    &RAGNAR,
    &RAMIREZ_DEPIETRO,
    &RIVEN_TURNBULL,
    &RUBINIA_SOULSINGER,
    &SIR_SHANDLAR_OF_EBERYN,
    &SIVITRI_SCARZAM,
    &SUNASTIAN_FALCONER,
    &TETSUO_UMEZAWA,
    &THE_LADY_OF_THE_MOUNTAIN,
    &TOBIAS_ANDRION,
    &TOR_WAUKI,
    &TORSTEN_VON_URSUS,
    &TUKNIR_DEATHLOCK,
    &UR_DRAGO,
    &VAEVICTIS_ASMADI,
    &XIRA_ARIEN,
    &ARENA_OF_THE_ANCIENTS,
    &HORN_OF_DEAFENING,
    &RELIC_BARRIER,
    &KARAKAS,
    &PENDELHAVEN,
    &THE_TABERNACLE_AT_PENDRELL_VALE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
