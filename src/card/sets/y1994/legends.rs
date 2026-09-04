use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BandingQuality,
    BasicLandType, BattlefieldEntryModificationDef, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, ChoiceVisibilityDef, ChooseDef, ChooseExactDef, ColorChoiceOperationDef, ColorSet,
    ComparisonDef, ControlDurationDef, CostModificationDef, CounterKind, DamageEventMatcherDef,
    DamageKindDef, DamageLimitDef, DamagePreventionDef, DamageRecipientMatcherDef,
    DamageSourceGroupDef, DamageSourceMatcherDef, DiscardFollowUpDef, DiscardSelectionDef,
    DividedTotal, EffectDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef,
    KeywordAbility, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementAbilityDef, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    SacrificedAmountDef, ScaledValueDef, SpellResolutionDestinationDef, SumValueDef,
    TargetChooserDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

/// "One or more target creatures": the count has no printed ceiling, so the
/// declaration is bounded by how many creatures are actually there.
static ONE_OR_MORE_CREATURES: [AbilityTargetDef; 1] = [AbilityTargetDef::one_or_more(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

/// The clause the five Legends band lands grant. One static rather than five,
/// because the lands differ only in which color of legend they reach.
static BANDS_WITH_OTHER_LEGENDS: AbilityDef =
    abilities::bands_with_other(BandingQuality::LegendaryCreatures);

// LEG 1 — Akron Legionnaire
pub(in crate::card::sets) static AKRON_LEGIONNAIRE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Akron Legionnaire",
    "5d074af2-8dbd-42d3-87eb-30f6e7d171ff",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{6}{W}{W}"), &["Giant", "Soldier"], 8, 4).with_ability(
        AbilityDef::static_ability(
            "Except for creatures named Akron Legionnaire and artifact creatures, creatures \
             you control can't attack.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasName(ObjectRefDef::Source),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                        ])),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
    ),
);

// LEG 2 — Alabaster Potion
pub(in crate::card::sets) static ALABASTER_POTION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Alabaster Potion",
    "2806c7f6-8fdd-4e65-9c71-f2e8b0cdede2",
    "Harold McNeill",
    CardRules::new_instant(mana_cost!("{X}{W}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target player gains X life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
            ),
            AbilityDef::spell_with_targets(
                "Prevent the next X damage that would be dealt to any target this turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::amount(
                        DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                        ValueDef::ChosenX,
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// LEG 3 — Amrou Kithkin
pub(in crate::card::sets) static AMROU_KITHKIN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Amrou Kithkin",
    "cbce1c55-123c-4a05-bde4-18a1601fcc5a",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Kithkin"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked by creatures with power 3 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::PowerAtLeast(3),
                )),
            },
        ),
    ),
);

// LEG 4 — Angelic Voices
pub(in crate::card::sets) static ANGELIC_VOICES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Angelic Voices",
    "8068c263-e5fa-4449-8887-418e9d0a4da4",
    "Julie Baroh",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control get +1/+1 as long as you control no nonartifact, nonwhite \
         creatures.",
        EffectDef::IfCondition {
            // An absence rather than a presence, so the anthem switches off the moment
            // one off-plan creature arrives and comes back when it leaves.
            condition: &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            then: &EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        },
    )),
);

// LEG 5 — Cleanse
pub(in crate::card::sets) static CLEANSE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Cleanse",
    "2fbd611b-ac97-4516-bad7-cc9ee4ef74f7",
    "Phil Foglio",
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all black creatures.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// LEG 6 — Clergy of the Holy Nimbus
// Audit: unsupported — Needs a would-be-destroyed replacement that regenerates the source, and an activation restricted to opponents. The turn-scoped regeneration prohibition its second clause applies is available.
pub(in crate::card::sets) static CLERGY_OF_THE_HOLY_NIMBUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Clergy of the Holy Nimbus",
    "db1f578f-fa3b-4447-953b-1490852b6c80",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// LEG 7 — D'Avenant Archer
pub(in crate::card::sets) static DAVENANT_ARCHER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "D'Avenant Archer",
    "b09aee5c-8b9e-46c2-b4d4-508062f8af05",
    "Douglas Shuler",
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
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “When you remove the last intervention counter from this enchantment, the game is a draw”.
pub(in crate::card::sets) static DIVINE_INTERVENTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Divine Intervention",
    "9eae0ba1-1383-4505-b4e7-4f17dd8f20c5",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// LEG 9 — Divine Offering
pub(in crate::card::sets) static DIVINE_OFFERING: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Divine Offering",
    "9c78c2f3-2f40-48ad-9dc4-55d1fa399a56",
    "Jeff A. Menges",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact. You gain life equal to its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
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
    CardSet::Legends,
    "Divine Transformation",
    "a89ad9fd-33a6-4d31-9f4c-8bf192882f21",
    "NéNé Thomas",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                },
            ),
        ]),
);

// LEG 11 — Elder Land Wurm
pub(in crate::card::sets) static ELDER_LAND_WURM: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Elder Land Wurm",
    "ef3651d4-969c-464d-a444-40a640d0c6ba",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{4}{W}{W}{W}"), &["Dragon", "Wurm"], 5, 5).with_abilities(
        &[
            abilities::defender(),
            abilities::trample(),
            AbilityDef::triggered(
                "When this creature blocks, it loses defender.",
                TriggerEventDef::Blocks {
                    blocked: ObjectPredicateDef::Any,
                },
                // The printed clause names no duration, so the removal
                // outlives the combat that paid for it: once it has blocked,
                // it can attack from then on.
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Defender,
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
        ],
    ),
);

// LEG 12 — Enchanted Being
pub(in crate::card::sets) static ENCHANTED_BEING: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Enchanted Being",
    "94c2880d-b37a-43ea-9fee-cd5a8ed75a7e",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Prevent all combat damage that would be dealt to this creature by enchanted \
             creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_combat_damage_from(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Enchanted,
                ])),
            },
        ),
    ),
);

// LEG 13 — Equinox
// Audit: unsupported — Needs a granted ability that can target a spell by prospectively determining whether that spell would destroy one of its controller's lands.
pub(in crate::card::sets) static EQUINOX: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Equinox",
    "840c6586-a7a9-4ae8-96be-a995a0693eb6",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 14 — Fortified Area
pub(in crate::card::sets) static FORTIFIED_AREA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Fortified Area",
    "dc64f19c-5b2b-4697-b4dc-2be9c3790794",
    "Randy Asplund-Faith",
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Wall creatures you control get +1/+0 and have banding.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Subtype("Wall"),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            // Both halves of one clause: the bonus and the keyword arrive together, so
            // they are one applied effect rather than two abilities.
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::banding()),
            ]),
        },
    )),
);

// LEG 15 — Glyph of Life
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Choose target Wall creature. Whenever that creature is dealt damage by an attacking creature this turn, you gain that much life”.
pub(in crate::card::sets) static GLYPH_OF_LIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Glyph of Life",
    "ba1384e5-d140-4074-9548-250af09cb413",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 16 — Great Defender
pub(in crate::card::sets) static GREAT_DEFENDER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Great Defender",
    "879a8653-1538-4f78-a3d3-a900a4d9499b",
    "Mark Poole",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +0/+X until end of turn, where X is its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 17 — Great Wall
pub(in crate::card::sets) static GREAT_WALL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Great Wall",
    "cd860a1d-aa17-4579-b9b1-d101d2416387",
    "Sandra Everingham",
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with plainswalk can be blocked as though they didn't have plainswalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Plains),
    )]),
);

// LEG 18 — Greater Realm of Preservation
pub(in crate::card::sets) static GREATER_REALM_OF_PRESERVATION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Greater Realm of Preservation",
    "5e236816-0c49-4b48-b18b-03add5a80d72",
    "NéNé Thomas",
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
pub(in crate::card::sets) static HEAVENS_GATE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Heaven's Gate",
    "461d7c11-3a7d-42c2-bb6b-0a43779e6842",
    "Douglas Shuler",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "One or more target creatures become white until end of turn.",
        &ONE_OR_MORE_CREATURES,
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 20 — Holy Day
pub(in crate::card::sets) static HOLY_DAY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Holy Day",
    "f6c95a2b-bf44-4ff2-9c6a-916773346edd",
    "Justin Hampton",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 21 — Indestructible Aura
pub(in crate::card::sets) static INDESTRUCTIBLE_AURA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Indestructible Aura",
    "ed2a7333-c9ce-4011-b00e-1304e1eec25e",
    "Mark Poole",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Prevent all damage that would be dealt to target creature this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEG 22 — Infinite Authority
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature blocks or becomes blocked by a creature with toughness 3 or less, destroy the other creature at end of combat. At the beginning of the next end step, if that…”.
pub(in crate::card::sets) static INFINITE_AUTHORITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Infinite Authority",
    "dc60077f-d577-4a6c-a78f-697317024c40",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// LEG 23 — Ivory Guardians
pub(in crate::card::sets) static IVORY_GUARDIANS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Ivory Guardians",
    "9bf9cccd-fe97-4632-a90a-9eeb0d41135e",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Giant", "Cleric"], 3, 3).with_abilities(&[
        abilities::protection_from_color(ManaColor::Red),
        AbilityDef::static_ability(
            "Creatures named Ivory Guardians get +1/+1 as long as an opponent controls a nontoken \
             red permanent.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Color(ManaColor::Red),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                // "Creatures named Ivory Guardians", so a second copy pumps the first and
                // an opponent's is covered too.
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasName(ObjectRefDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
    ]),
);

// LEG 24 — Keepers of the Faith
pub(in crate::card::sets) static KEEPERS_OF_THE_FAITH: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Keepers of the Faith",
    "b63a69ae-99ce-4d26-88b7-784793c43cd4",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 2, 3),
);

// LEG 25 — Kismet
pub(in crate::card::sets) static KISMET: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kismet",
    "7e0651ad-6901-4f9b-8807-d66e53a4ada8",
    "Kaja Foglio",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::replacement_for(
        "Artifacts, creatures, and lands your opponents control enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            controller: PlayerRelation::Opponent,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )),
);

// LEG 26 — Land Tax
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “At the beginning of your upkeep, if an opponent controls more lands than you, you may search your library for up to three basic land reveal them, put them into your hand, then shuffle”.
pub(in crate::card::sets) static LAND_TAX: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Land Tax",
    "d53b20b0-67bc-4587-817b-efbf21cb2512",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 27 — Lifeblood
pub(in crate::card::sets) static LIFEBLOOD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Lifeblood",
    "4ecb1362-9a67-4d4c-8d69-9ac2ebf4d0b0",
    "Mark Tedin",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a Mountain an opponent controls becomes tapped, you gain 1 life.",
        TriggerEventDef::tapped(ObjectPredicateDef::All(&[
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
    CardSet::Legends,
    "Moat",
    "952ba126-0915-47f0-9b6a-a0a6dcd22c6f",
    "Jeff A. Menges",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures without flying can't attack.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        KeywordAbility::Flying,
                    )),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
        },
    )),
);

// LEG 29 — Osai Vultures
pub(in crate::card::sets) static OSAI_VULTURES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Osai Vultures",
    "f85614b3-62a3-4da9-a74a-7ea40fad1b52",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if(
            "At the beginning of each end step, if a creature died this turn, put a carrion \
             counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            // Every end step, not only its controller's: something dying on either turn
            // feeds it.
            &TriggerConditionDef::CreatureDiedThisTurn,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("carrion"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove two carrion counters from this creature: This creature gets +1/+1 until \
             end of turn.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::named("carrion"),
                amount: 2,
            }],
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

// LEG 30 — Petra Sphinx
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “{T}: Target player chooses a card name, then reveals the top card of their library. If that card has the chosen name, that player puts it into their hand. If it doesn't, the player puts…”.
pub(in crate::card::sets) static PETRA_SPHINX: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Petra Sphinx",
    "5ef99f07-c987-451a-b18a-2719eea654cd",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// LEG 31 — Presence of the Master
pub(in crate::card::sets) static PRESENCE_OF_THE_MASTER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Presence of the Master",
    "1cb86b2f-116d-4952-b35a-1398341baaf5",
    "Phil Foglio",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts an enchantment spell, counter it.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::HasType(CardType::Enchantment)),
        EffectDef::Counter {
            object: EffectRecipientDef::TriggeringObject,
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        },
    )),
);

// LEG 32 — Rapid Fire
// Audit: unsupported — Needs a conditional grant that checks whether the target already has rampage before granting rampage 2, and a cast restriction limiting the spell to before blockers are declared.
pub(in crate::card::sets) static RAPID_FIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Rapid Fire",
    "e26e7c9c-e6de-47f4-8394-7e853408f84c",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// LEG 33 — Remove Enchantments
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Return to your hand all enchantments you both own and control, all Auras you own attached to permanents you control, and all Auras you own attached to attacking creatures your opponents…”.
pub(in crate::card::sets) static REMOVE_ENCHANTMENTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Remove Enchantments",
    "bf2e3a8a-b386-474d-b8e9-4c2d56a2b742",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 34 — Righteous Avengers
pub(in crate::card::sets) static RIGHTEOUS_AVENGERS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Righteous Avengers",
    "d96b463e-9579-4e7b-87c2-342527b91e7c",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 3, 1)
        .with_ability(abilities::landwalk(BasicLandType::Plains)),
);

// LEG 35 — Seeker
pub(in crate::card::sets) static SEEKER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Seeker",
    "df608b59-cc07-4e1d-b6d6-f15e69b15b92",
    "Mark Poole",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't be blocked except by artifact creatures and/or white creatures.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Color(ManaColor::White),
                        ])),
                    )),
                },
            ),
        ]),
);

// LEG 36 — Shield Wall
pub(in crate::card::sets) static SHIELD_WALL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Shield Wall",
    "a5032bf0-f9c0-4ef0-8ec2-fe7ccea9bdf3",
    "Douglas Shuler",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+2 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 37 — Spirit Link
pub(in crate::card::sets) static SPIRIT_LINK: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Spirit Link",
    "5e2d35f8-3cf6-4843-9030-0e9a885d836c",
    "Kaja Foglio",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever enchanted creature deals damage, you gain that much life.",
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::AttachedToSource),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
        ]),
);

// LEG 38 — Spiritual Sanctuary
pub(in crate::card::sets) static SPIRITUAL_SANCTUARY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Spiritual Sanctuary",
    "654dd1e0-a91d-44ee-af20-c025bf360c3f",
    "Amy Weber",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of each player's upkeep, if that player controls a Plains, they gain 1 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::EventPlayer,
                ),
                comparison: ComparisonDef::Greater,
                amount: 0,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// LEG 39 — Thunder Spirit
pub(in crate::card::sets) static THUNDER_SPIRIT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Thunder Spirit",
    "61a59775-b1cd-4ed0-8abf-c2b37f7be0d5",
    "Randy Asplund-Faith",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Elemental", "Spirit"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// LEG 40 — Tundra Wolves
pub(in crate::card::sets) static TUNDRA_WOLVES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Tundra Wolves",
    "8f649cb5-e19c-453f-b062-4fd452d92257",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{W}"), &["Wolf"], 1, 1)
        .with_ability(abilities::first_strike()),
);

// LEG 41 — Visions
pub(in crate::card::sets) static VISIONS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Visions",
    "21d00299-e183-4b3d-b015-18808e7135b9",
    "NéNé Thomas",
    CardRules::new_sorcery(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Look at the top five cards of target player's library. You may then have that player \
         shuffle that library.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            abilities::look_at_top_cards(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ]),
    )),
);

// LEG 42 — Wall of Caltrops
// Audit: unsupported — Needs an intervening-if that counts the other creatures blocking the same attacker by subtype for “if at least one other Wall creature is blocking that creature and no non-Wall creatures are blocking that creature”. Granting banding is implemented.
pub(in crate::card::sets) static WALL_OF_CALTROPS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Wall of Caltrops",
    "664ad588-3002-4f63-93bd-38663171018f",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 43 — Wall of Light
pub(in crate::card::sets) static WALL_OF_LIGHT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Light",
    "f5758e82-f901-42b7-b705-0e68ca7ba59e",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Wall"], 1, 5).with_abilities(&[
        abilities::defender(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// LEG 44 — Acid Rain
pub(in crate::card::sets) static ACID_RAIN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Acid Rain",
    "ba93c50a-2440-4e92-9cba-d97e20b1d29c",
    "NéNé Thomas",
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Destroy all Forests.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// LEG 45 — Anti-Magic Aura
// Audit: unsupported — Needs the card's exact Aura targeting/attachment restriction rather than the broader existing cannot-be-enchanted effect for “Enchanted creature can't be the target of spells and can't be enchanted by other Auras”.
pub(in crate::card::sets) static ANTI_MAGIC_AURA: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Anti-Magic Aura",
    "ff78eef1-efaa-4a12-bf5d-fec83c14aff8",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// LEG 46 — Azure Drake
pub(in crate::card::sets) static AZURE_DRAKE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Azure Drake",
    "fb5f13a2-0896-4230-8957-6ad1cb2b895b",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Drake"], 2, 4)
        .with_ability(abilities::flying()),
);

// LEG 47 — Backfire
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “Whenever enchanted creature deals damage to you, this Aura deals that much damage to that creature's controller”.
pub(in crate::card::sets) static BACKFIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Backfire",
    "04bc57aa-d4d9-4bd9-ba09-984370c7e23b",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 48 — Boomerang
pub(in crate::card::sets) static BOOMERANG: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Boomerang",
    "b8286edd-644b-4135-8dca-af97f3920de3",
    "Brian Snõddy",
    CardRules::new_instant(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target permanent to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// LEG 49 — Brine Hag
// Audit: unsupported — Needs a characteristic-layer effect or dynamic value for “When this creature dies, change the base power and toughness of all creatures that dealt damage to it this turn to 0/2”.
pub(in crate::card::sets) static BRINE_HAG: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Brine Hag",
    "2a1e7796-fbfb-4976-879f-bb748429d5c7",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// LEG 50 — Devouring Deep
pub(in crate::card::sets) static DEVOURING_DEEP: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Devouring Deep",
    "0855a5a8-8c40-4396-9ad1-8fa0fc6a0c59",
    "Liz Danforth",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Fish"], 1, 2)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// LEG 51 — Dream Coat
// Audit: unsupported — Needs a per-object, per-turn activation quota for “{0}: Enchanted creature becomes the color or colors of your choice. Activate only once each turn”.
pub(in crate::card::sets) static DREAM_COAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Dream Coat",
    "07edbbf4-c3d6-4ec1-ae9b-4ae202fb6998",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// LEG 52 — Elder Spawn
pub(in crate::card::sets) static ELDER_SPAWN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Elder Spawn",
    "99cc045e-01a8-4f14-a86d-0a67ec35d6b7",
    "Jesper Myrfors",
    CardRules::new_creature(mana_cost!("{4}{U}{U}{U}"), &["Spawn"], 6, 6).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, unless you sacrifice an Island, sacrifice this \
             creature and it deals 6 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                then: None,
                // The declined branch: the Spawn goes and takes six with it. Reached when
                // the controller says no *and* when there is no Island to say yes with.
                otherwise: Some(&EffectDef::Sequence(&[
                    EffectDef::Sacrifice {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(6),
                    },
                ])),
                amount: SacrificedAmountDef::Power,
                optional: true,
            },
        ),
        AbilityDef::static_ability(
            "This creature can't be blocked by red creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Color(ManaColor::Red),
                )),
            },
        ),
    ]),
);

// LEG 53 — Enchantment Alteration
// Audit: unsupported — Needs Aura reattachment targeting, enchant-legality validation, and attachment movement for “Attach target Aura attached to a creature or land to another permanent of that type”.
pub(in crate::card::sets) static ENCHANTMENT_ALTERATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Enchantment Alteration",
    "bf52f8a0-d027-47f1-bb91-508ef1a74409",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 54 — Energy Tap
pub(in crate::card::sets) static ENERGY_TAP: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Energy Tap",
    "37e69940-bdc8-48ff-a296-540343910adf",
    "Daniel Gelon",
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target untapped creature you control. If you do, add an amount of {C} equal to \
         that creature's mana value.",
        // "Untapped creature you control", so the tap always lands and the mana
        // always follows: an already-tapped creature is not a legal target.
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::AddManaEqualTo {
                color: ManaColor::Colorless,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// LEG 55 — Field of Dreams
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “Players play with the top card of their libraries revealed”.
pub(in crate::card::sets) static FIELD_OF_DREAMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Field of Dreams",
    "6a63e119-3b1b-4964-a4b9-b10170ff542b",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 56 — Flash Counter
pub(in crate::card::sets) static FLASH_COUNTER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Flash Counter",
    "3c3cd450-f1cd-416b-9271-37d95815c089",
    "Harold McNeill",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::HasType(CardType::Instant),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// LEG 57 — Flash Flood
pub(in crate::card::sets) static FLASH_FLOOD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Flash Flood",
    "5ae88c06-f28c-4fbc-a28c-5eb203a04722",
    "Tom Wänerstrand",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target red permanent.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Color(ManaColor::Red),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target Mountain to its owner's hand.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
    )),
);

// LEG 58 — Force Spike
pub(in crate::card::sets) static FORCE_SPIKE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Force Spike",
    "70e64028-ae96-4950-aa6c-9d347409fad3",
    "Bryon Wackwitz",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(ValueDef::Constant(1)),
    )),
);

// LEG 59 — Gaseous Form
pub(in crate::card::sets) static GASEOUS_FORM: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Gaseous Form",
    "d0266dd4-31da-480b-9a44-4e217f748f06",
    "Phil Foglio",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Prevent all combat damage that would be dealt to and dealt by enchanted creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                            DamageEventMatcherDef::COMBAT_FROM_AFFECTED,
                        )),
                        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                            DamageEventMatcherDef::COMBAT_TO_AFFECTED,
                        )),
                    ]),
                },
            ),
        ]),
);

// LEG 60 — Glyph of Delusion
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Put X glyph counters on target creature that target Wall blocked this turn, where X is the power of that blocked creature. The creature gains "This creature doesn't untap during your…”.
pub(in crate::card::sets) static GLYPH_OF_DELUSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Glyph of Delusion",
    "ee39da13-4b8a-4796-a7c2-aaa11992d573",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 61 — In the Eye of Chaos
// Audit: unsupported — Needs a cast trigger that counters an instant unless its controller pays that spell's dynamically read mana value.
pub(in crate::card::sets) static IN_THE_EYE_OF_CHAOS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "In the Eye of Chaos",
    "733933dd-c871-4f75-8b08-d7c010dddbe6",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 62 — Invoke Prejudice
// Audit: unsupported — Needs cross-object color comparison and a mana-value-based counter-unless payment on the triggering creature spell.
pub(in crate::card::sets) static INVOKE_PREJUDICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Invoke Prejudice",
    "903d9fde-d7da-4a0e-a337-b63023c6d74b",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// LEG 63 — Juxtapose
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “You and target player exchange control of the creature you each control with the greatest mana value. Then exchange control of artifacts the same way. If two or more permanents a player…”.
pub(in crate::card::sets) static JUXTAPOSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Juxtapose",
    "c48d78db-d982-4c28-9308-2d57dc2b947e",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// LEG 64 — Land Equilibrium
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “If an opponent who controls at least as many lands as you do would put a land onto the battlefield, that player instead puts that land onto the battlefield then sacrifices a land of…”.
pub(in crate::card::sets) static LAND_EQUILIBRIUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Land Equilibrium",
    "9c406b25-03f8-4aaa-9ea7-48bf754166b7",
    "Jesper Myrfors",
    crate::card::CardRules::unsupported(),
);

// LEG 65 — Mana Drain
pub(in crate::card::sets) static MANA_DRAIN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Mana Drain",
    "e691adef-3027-4e6a-889f-9f4e2df36a7c",
    "Mark Tedin",
    CardRules::new_instant(mana_cost!("{U}{U}"))
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            // The mana arrives later, so the amount is read from what the countered
            // spell was rather than from anything still on the stack.
            EffectDef::Sequence(&[
                EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.",
                    // "Your next main phase" is whichever main phase comes next: a Drain cast
                    // in your own precombat main, or during your combat, pays out in that same
                    // turn's postcombat main rather than waiting a turn.
                    TriggerEventDef::AnyOf(&[
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PostcombatMain,
                            player: PlayerRelation::You,
                        },
                    ]),
                    EffectDef::AddManaEqualTo {
                        color: ManaColor::Colorless,
                        amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                    },
                ))),
            ]),
        )]),
);

// LEG 66 — Part Water
pub(in crate::card::sets) static PART_WATER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Part Water",
    "4b659475-c8b7-493d-af63-04f34d8cc3b1",
    "NéNé Thomas",
    CardRules::new_sorcery(mana_cost!("{X}{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "X target creatures gain islandwalk until end of turn.",
        // "X target creatures": the count is the X that was paid, and X is doubled
        // in the cost, so each creature reached costs two mana rather than one.
        &[AbilityTargetDef::exactly_chosen_x(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::landwalk(BasicLandType::Island)),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 67 — Psionic Entity
pub(in crate::card::sets) static PSIONIC_ENTITY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Psionic Entity",
    "ec082062-5394-4340-bc29-0efd2af4b822",
    "Justin Hampton",
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
// Audit: unsupported — Needs a hidden-zone decision and continuation for “When a spell or ability an opponent controls causes you to discard this card, that player loses 5 life”.
pub(in crate::card::sets) static PSYCHIC_PURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Psychic Purge",
    "b1103d4d-b50a-4e2c-b18a-a181bc819881",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 69 — Puppet Master
// Audit: unsupported — Needs a zone-object query and identity-preserving continuation for “When enchanted creature dies, return that card to its owner's hand. If that card is returned to its owner's hand this way, you may pay {U}{U}{U}. If you do, return this card to its…”.
pub(in crate::card::sets) static PUPPET_MASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Puppet Master",
    "b5d26ddc-ad1e-4a97-85fb-34da685c3142",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// LEG 70 — Recall
pub(in crate::card::sets) static RECALL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Recall",
    "33296718-0625-4422-a65c-b21cf99c52ec",
    "Brian Snõddy",
    CardRules::new_sorcery(mana_cost!("{X}{X}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Discard X cards, then return a card from your graveyard to your hand for each card discarded this way. Exile Recall.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
                selection: DiscardSelectionDef::RecipientChooses,
                then: Some(DiscardFollowUpDef {
                    counted: ObjectPredicateDef::Any,
                    bound: Some(ParentBinding),
                    effect: &EffectDef::ChooseExact(ChooseExactDef {
                        binding: ParentBinding,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::Related(PlayerRelation::You),
                        )),
                        exclude: None,
                        amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                            ParentBinding,
                        )),
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    }),
                }),
            },
        )
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ]),
);

// LEG 71 — Relic Bind
pub(in crate::card::sets) static RELIC_BIND: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Relic Bind",
    "a9b07dc4-21ad-410b-8f8a-2b034253bfee",
    "Christopher Rush",
    // Three mana to tax an artifact somebody else is using, a point at a
    // time, and to hand out a point of life when the taxing is not what you
    // want.
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact an opponent controls",
                // "Enchant artifact an opponent controls." The restriction is on the Aura
                // spell's target rather than on where it may stay: an artifact that
                // changes hands afterwards keeps the Aura.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
            ),
            AbilityDef::modal_triggered(
                "Whenever enchanted artifact becomes tapped, choose one —\n• This Aura deals 1 \
                 damage to target player or planeswalker.\n• Target player gains 1 life.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                &[
                    AbilityDef::spell_with_targets(
                        "This Aura deals 1 damage to target player or planeswalker.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                        )],
                        EffectDef::DealDamage {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Target player gains 1 life.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Player(PlayerRelation::Any),
                        )],
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    ),
                ],
            ),
        ]),
);

// LEG 72 — Remove Soul
pub(in crate::card::sets) static REMOVE_SOUL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Remove Soul",
    "63de147c-2e62-41b9-8ada-93406387f08b",
    "Brian Snõddy",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target creature spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// LEG 73 — Reset
pub(in crate::card::sets) static RESET: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Reset",
    "1c829d83-d5b8-4be7-80f7-55b42f52b309",
    "Nicola Leonard",
    CardRules::new_instant(mana_cost!("{U}{U}"))
        .cast_only_after_an_opponents_upkeep()
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Cast this spell only during an opponent's turn after their upkeep step.",
                "The play option refuses the cast on your own turn and during their upkeep.",
            ),
            AbilityDef::spell(
                "Untap all lands you control.",
                EffectDef::Untap {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                },
            ),
        ]),
);

// LEG 74 — Reverberation
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “All damage that would be dealt this turn by target sorcery spell is dealt to that spell's controller instead”.
pub(in crate::card::sets) static REVERBERATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Reverberation",
    "a3d1f470-058d-41b7-acaf-4f68431de9ed",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// LEG 75 — Sea Kings' Blessing
pub(in crate::card::sets) static SEA_KINGS_BLESSING: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sea Kings' Blessing",
    "11d1f02d-533e-4b77-a72a-ff5f91ae0626",
    "Randy Asplund-Faith",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "One or more target creatures become blue until end of turn.",
        &ONE_OR_MORE_CREATURES,
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue])),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 76 — Segovian Leviathan
pub(in crate::card::sets) static SEGOVIAN_LEVIATHAN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Segovian Leviathan",
    "e5a814f1-7f8d-4c2c-b706-ee0ed5892f7b",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Leviathan"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// LEG 77 — Silhouette
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “Choose target creature. If a spell or ability that targets that creature would cause a source to deal damage to that creature this turn, prevent that damage”.
pub(in crate::card::sets) static SILHOUETTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Silhouette",
    "e6d6fac6-9a23-465f-a813-92e1ed1cd742",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 78 — Spectral Cloak
pub(in crate::card::sets) static SPECTRAL_CLOAK: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Spectral Cloak",
    "7524fd0d-a675-41d6-bc99-bd3ba336893b",
    "Rob Alexander",
    CardRules::new_enchantment(mana_cost!("{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has shroud as long as it's untapped.",
                EffectDef::StaticApply {
                    // The condition rides on the recipient rather than on the
                    // effect, so tapping the host takes the shroud away
                    // without the Aura being touched.
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AttachedToSource,
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::shroud()),
                },
            ),
        ]),
);

// LEG 79 — Telekinesis
pub(in crate::card::sets) static TELEKINESIS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Telekinesis",
    "d5aa920e-b93f-41c2-b505-a9350353be8b",
    "Daniel Gelon",
    CardRules::new_instant(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target creature. Prevent all combat damage that would be dealt by that creature \
         this turn. It doesn't untap during its controller's next two untap steps.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                count: 2,
            },
        ]),
    )),
);

// LEG 80 — Teleport
pub(in crate::card::sets) static TELEPORT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Teleport",
    "18f86e13-f942-423e-b175-930d768cb811",
    "Douglas Shuler",
    CardRules::new_instant(mana_cost!("{U}{U}{U}"))
        .cast_only_during_declare_attackers()
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Cast this spell only during the declare attackers step.",
                "The play option refuses the cast in any other step.",
            ),
            AbilityDef::spell_with_targets(
                "Target creature can't be blocked this turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEG 81 — Time Elemental
// Audit: unsupported — Needs a zone-object query and identity-preserving continuation for “{2}{U}{U}, {T}: Return target permanent that isn't enchanted to its owner's hand”.
pub(in crate::card::sets) static TIME_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Time Elemental",
    "61510e88-97d0-410a-9431-ebf12990e33d",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// LEG 82 — Undertow
pub(in crate::card::sets) static UNDERTOW: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Undertow",
    "cf05e5c9-b7e4-4bd8-ab73-b54565710527",
    "Randy Asplund-Faith",
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with islandwalk can be blocked as though they didn't have islandwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Island),
    )]),
);

// LEG 83 — Venarian Gold
pub(in crate::card::sets) static VENARIAN_GOLD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Venarian Gold",
    "11fb92c0-bb1e-463a-a6b6-887a5d0cb873",
    "Daniel Gelon",
    CardRules::new_enchantment(mana_cost!("{X}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature and put X sleep counters on it.",
                // The X is read off the permanent rather than the resolving object: this is
                // an enters trigger, a separate object from the spell that chose it.
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::AttachedPermanent,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::AttachedPermanent,
                        kind: CounterKind::named("sleep"),
                        amount: ValueDef::SourceCastX,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step if it \
                 has a sleep counter on it.",
                EffectDef::IfCondition {
                    // The counters are on the creature rather than on the Aura, so the condition
                    // asks what the Aura is attached to rather than what it carries itself.
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::HasCounter(CounterKind::named("sleep")),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                    },
                },
            ),
            // The host's controller's upkeep, not the Aura controller's: this
            // is on an opponent's creature every time it is played.
            AbilityDef::triggered(
                "At the beginning of the upkeep of enchanted creature's controller, remove a \
                 sleep counter from that creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::ControllerOfAttachedPermanent,
                },
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::named("sleep"),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEG 84 — Wall of Vapor
pub(in crate::card::sets) static WALL_OF_VAPOR: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Vapor",
    "6a6c0a27-d410-4ded-a842-70e1656ea21e",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Wall"], 0, 1).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by creatures it's blocking.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_damage_from(ObjectPredicateDef::BlockedBySource),
            },
        ),
    ]),
);

// LEG 85 — Wall of Wonder
pub(in crate::card::sets) static WALL_OF_WONDER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Wonder",
    "bcd9af40-b46c-44b4-878e-8eb026c96b51",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Wall"], 1, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{2}{U}{U}: This creature gets +4/-4 until end of turn and can attack this turn as \
             though it didn't have defender.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{U}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // One activation, two things applied to the same creature for the same
                // duration, so they ride together rather than as two effects in sequence.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(-4),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 86 — Zephyr Falcon
pub(in crate::card::sets) static ZEPHYR_FALCON: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Zephyr Falcon",
    "25a173fd-e10c-45f8-a6e5-ad7a747a8050",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// LEG 87 — Abomination
pub(in crate::card::sets) static ABOMINATION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Abomination",
    "a69e2cf8-5ecb-485a-92a2-b4e0a7959f1f",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Horror"], 2, 6).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a green or white creature, \
             destroy that creature at end of combat.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
    ),
);

// LEG 88 — All Hallow's Eve
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “At the beginning of your upkeep, if this card is exiled with a scream counter on it, remove a scream counter from it. If there are no more scream counters on it, put it into your…”.
pub(in crate::card::sets) static ALL_HALLOW_S_EVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "All Hallow's Eve",
    "18787a2d-6688-47e9-94bc-ccf229df823f",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// LEG 89 — Blight
pub(in crate::card::sets) static BLIGHT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Blight",
    "9ca19b39-4201-463c-bd40-fbffa31c9eda",
    "Pete Venters",
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::triggered(
                "When enchanted land becomes tapped, destroy it.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    can_regenerate: true,
                    then: None,
                },
            ),
        ]),
);

// LEG 90 — Carrion Ants
pub(in crate::card::sets) static CARRION_ANTS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Carrion Ants",
    "cbc0b009-3951-4aa3-985a-97139882da7e",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Insect"], 0, 1).with_ability(
        AbilityDef::activated(
            "{1}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 91 — Chains of Mephistopheles
pub(in crate::card::sets) static CHAINS_OF_MEPHISTOPHELES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Chains of Mephistopheles",
    "2bb0e884-5bb4-41f3-b04b-6f638357c166",
    "Heather Hudson",
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::defined_replacement(
            "If a player would draw a card except the first one they draw in each of their draw steps, that player discards a card instead. If the player discards a card this way, they draw a card. If the player doesn't discard a card this way, they mill a card.",
            ReplacementAbilityDef::new().with_event(ReplacementEventDef::WouldDraw {
                player: PlayerRelation::Any,
                during_own_draw_step: false,
                except_first_in_draw_step: true,
            }),
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::ReplaceEventWithNothing,
                ReplacementEffectDef::Perform(&EffectDef::Discard {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Any,
                        bound: Some(ParentBinding),
                        effect: &EffectDef::IfElseCondition {
                            condition: &TriggerConditionDef::ValueComparison(
                                &ValueComparisonDef {
                                    left: ValueDef::BoundObjectCount(ParentBinding),
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    right: ValueDef::Constant(1),
                                },
                            ),
                            then: &EffectDef::ContinueReplacedDraw,
                            otherwise: &EffectDef::Mill {
                                player: EffectRecipientDef::EventPlayer,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    }),
                }),
            ]),
        ),
    ),
);

// LEG 92 — Cosmic Horror
// Audit: unsupported — Needs an unless-paid destruction sequence that deals damage only when this ability actually destroys its source.
pub(in crate::card::sets) static COSMIC_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Cosmic Horror",
    "18bc6ac2-19e0-4765-852b-e303a5bb4040",
    "Jesper Myrfors",
    crate::card::CardRules::unsupported(),
);

// LEG 93 — Cyclopean Mummy
pub(in crate::card::sets) static CYCLOPEAN_MUMMY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Cyclopean Mummy",
    "479ccc50-2d72-4adc-901e-fbd4eef2cf92",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 1).with_ability(
        abilities::dies_trigger(
            "When this creature dies, exile it.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::TriggeringZoneChangeResult,
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// LEG 94 — Darkness
pub(in crate::card::sets) static DARKNESS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Darkness",
    "53b04dab-45b7-418b-a0f0-bcf35145fc53",
    "Harold McNeill",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 95 — Demonic Torment
pub(in crate::card::sets) static DEMONIC_TORMENT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Demonic Torment",
    "d3ec14bc-95e9-47ce-b51e-d5eac9b345fe",
    "Anson Maddocks",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                },
            ),
            AbilityDef::static_ability(
                "Prevent all combat damage that would be dealt by enchanted creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                        DamageEventMatcherDef::COMBAT_FROM_AFFECTED,
                    )),
                },
            ),
        ]),
);

// LEG 96 — Evil Eye of Orms-by-Gore
pub(in crate::card::sets) static EVIL_EYE_OF_ORMS_BY_GORE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Evil Eye of Orms-by-Gore",
    "b060f747-f65c-4ee0-923a-76298cb51a03",
    "Jesper Myrfors",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Eye"], 3, 6).with_abilities(&[
        AbilityDef::static_ability(
            "Non-Eye creatures you control can't attack.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Eye")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
        AbilityDef::static_ability(
            "This creature can't be blocked except by Walls.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
                )),
            },
        ),
    ]),
);

// LEG 97 — Fallen Angel
pub(in crate::card::sets) static FALLEN_ANGEL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Fallen Angel",
    "0f4174e4-0be8-49b5-8c52-22001790f6eb",
    "Anson Maddocks",
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 98 — Ghosts of the Damned
pub(in crate::card::sets) static GHOSTS_OF_THE_DAMNED: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Ghosts of the Damned",
    "20275678-3488-43d8-a93b-993e2267ab07",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Spirit"], 0, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gets -1/-0 until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 99 — Giant Slug
// Audit: unsupported — Needs a delayed upkeep trigger that makes a basic-land-type choice on resolution and grants the matching walk; granting a named walk is available.
pub(in crate::card::sets) static GIANT_SLUG: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Giant Slug",
    "43a612e5-a680-4c5b-8ce7-432a86240a6c",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// LEG 100 — Glyph of Doom
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Choose target Wall creature. At this turn's next end of combat, destroy all creatures that were blocked by that creature this turn”.
pub(in crate::card::sets) static GLYPH_OF_DOOM: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Glyph of Doom",
    "332bfce9-052d-42e9-a407-4a1dd59e0f2a",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 101 — Greed
pub(in crate::card::sets) static GREED: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Greed",
    "111a16a2-e875-4756-80db-290f9e8606db",
    "Phil Foglio",
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
    CardSet::Legends,
    "Headless Horseman",
    "d1aa37c8-98fa-4984-b09b-cf65ad84e97b",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Knight"], 2, 2),
);

// LEG 103 — Hell Swarm
pub(in crate::card::sets) static HELL_SWARM: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Hell Swarm",
    "64164d1b-75f4-456e-a717-90ce554dc16c",
    "Christopher Rush",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "All creatures get -1/-0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 104 — Hell's Caretaker
pub(in crate::card::sets) static HELLS_CARETAKER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Hell's Caretaker",
    "336b3b8f-d104-4f06-ad4f-c92b8a9038ca",
    "Sandra Everingham",
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
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// LEG 105 — Hellfire
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “Destroy all nonblack creatures. Hellfire deals X plus 3 damage to you, where X is the number of creatures that died this way”.
pub(in crate::card::sets) static HELLFIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Hellfire",
    "362f1fe9-20af-434c-9957-7a1a564d89e6",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// LEG 106 — Horror of Horrors
pub(in crate::card::sets) static HORROR_OF_HORRORS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Horror of Horrors",
    "b9f68dc2-c048-41ec-b237-c36fdd99c27d",
    "Mark Tedin",
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "Sacrifice a Swamp: Regenerate target black creature.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Swamp"),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// LEG 107 — Imprison
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature attacks or blocks, you may pay {1}. If you do, tap the creature, remove it from combat, and creatures it was blocking that had become blocked by only that…”.
pub(in crate::card::sets) static IMPRISON: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Imprison",
    "12671381-beb7-41b8-9484-97f8aca5c981",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// LEG 108 — Infernal Medusa
pub(in crate::card::sets) static INFERNAL_MEDUSA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Infernal Medusa",
    "26a5333f-2761-42b8-ae8b-1d360b109daf",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Gorgon"], 2, 4).with_abilities(&[
        // The two halves are printed separately and are not the same clause:
        // blocking kills anything, while being blocked spares Walls.
        AbilityDef::triggered(
            "Whenever this creature blocks a creature, destroy that creature at end of combat.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::HasType(CardType::Creature),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a non-Wall creature, destroy that \
             creature at end of combat.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
    ]),
);

// LEG 109 — Jovial Evil
// Audit: unsupported — Needs a dynamic count of white creatures controlled by the targeted opponent and multiplication for the damage value.
pub(in crate::card::sets) static JOVIAL_EVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Jovial Evil",
    "c993c74c-a574-423b-81c8-96b0a7a6e529",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// LEG 110 — Lesser Werewolf
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “{B}: If this creature's power is 1 or more, it gets -1/-0 until end of turn and put a -0/-1 counter on target creature blocking or blocked by this creature. Activate only during the…”.
pub(in crate::card::sets) static LESSER_WEREWOLF: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Lesser Werewolf",
    "baae02e4-7db9-4a7b-a4ee-ecb22fcb77bd",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// LEG 111 — Lost Soul
pub(in crate::card::sets) static LOST_SOUL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Lost Soul",
    "601eed5c-436d-425b-a45f-07881ad893c8",
    "Randy Asplund-Faith",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Spirit", "Minion"], 2, 1)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// LEG 112 — Mold Demon
// Audit: unsupported — Needs linked sacrifice/destruction accounting for “When this creature enters, sacrifice it unless you sacrifice two Swamps”.
pub(in crate::card::sets) static MOLD_DEMON: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Mold Demon",
    "649a33aa-7eac-4161-ae1a-fcbc758abccf",
    "Jesper Myrfors",
    crate::card::CardRules::unsupported(),
);

// LEG 113 — Nether Void
pub(in crate::card::sets) static NETHER_VOID: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Nether Void",
    "2e72f8cb-5bc3-4711-9b7c-a6eea9a0beaf",
    "Harold McNeill",
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::triggered(
            "Whenever a player casts a spell, counter it unless that player pays {3}.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Any),
            abilities::counter_triggering_spell_unless_paid(ValueDef::Constant(3)),
        )),
);

// LEG 114 — Pit Scorpion
pub(in crate::card::sets) static PIT_SCORPION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Pit Scorpion",
    "cc564f84-0d6e-4e09-a58d-a694d918cf12",
    "Scott Kirschner",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Scorpion"], 1, 1).with_ability(
        abilities::poisonous_damage(
            1,
            "Whenever this creature deals damage to a player, that player gets a poison \
             counter. (A player with ten or more poison counters loses the game.)",
        ),
    ),
);

// LEG 115 — Quagmire
pub(in crate::card::sets) static QUAGMIRE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Quagmire",
    "94e2aa9e-af6a-41c6-99a8-ca9335730ddb",
    "Dan Frazier",
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with swampwalk can be blocked as though they didn't have swampwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Swamp),
    )]),
);

// LEG 116 — Shimian Night Stalker
pub(in crate::card::sets) static SHIMIAN_NIGHT_STALKER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Shimian Night Stalker",
    "4e04a1b9-c561-4e34-86d9-129ea0346631",
    "Jesper Myrfors",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Nightstalker"], 4, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, {T}: All damage that would be dealt to you this turn by target attacking \
             creature is dealt to this creature instead.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Attacking,
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::RedirectDamageFromTo {
                    source: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    destination: ObjectRefDef::Source,
                }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 117 — Spirit Shackle
pub(in crate::card::sets) static SPIRIT_SHACKLE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Spirit Shackle",
    "a30bb266-5bd1-4998-ae94-56f0f3354167",
    "Edward P. Beard, Jr.",
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever enchanted creature becomes tapped, put a -0/-2 counter on it.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::MinusZeroMinusTwo,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEG 118 — Syphon Soul
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “Syphon Soul deals 2 damage to each other player. You gain life equal to the damage dealt this way”.
pub(in crate::card::sets) static SYPHON_SOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Syphon Soul",
    "f3020304-7a39-411e-b055-3ade72b4bff8",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// LEG 119 — Takklemaggot
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “When enchanted creature dies, that creature's controller chooses a creature that this card could enchant. If the player does, return this card to the battlefield under your control…”.
pub(in crate::card::sets) static TAKKLEMAGGOT: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Takklemaggot",
    "d8f49b3d-7fcb-4169-9298-cdf7a1dbe3f5",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// LEG 120 — The Abyss
pub(in crate::card::sets) static THE_ABYSS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "The Abyss",
    "86a27d68-3e58-4ade-976d-36381beed451",
    "Pete Venters",
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::triggered(
            "At the beginning of each player's upkeep, destroy target nonartifact creature that player controls of their choice. It can't be regenerated.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EventPlayer,
                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::EventPlayer),
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                    can_regenerate: false,
                    then: None,
                },
            }),
        )),
);

// LEG 121 — The Wretched
pub(in crate::card::sets) static THE_WRETCHED: CardRecord = CardRecord::new(
    CardSet::Legends,
    "The Wretched",
    "14c45416-a826-42e9-9967-8838158cf16d",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Demon"], 2, 5).with_ability(
        AbilityDef::triggered(
            "At end of combat, gain control of all creatures blocking this creature for as long \
             as you control this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::EndOfCombat,
                player: PlayerRelation::Any,
            },
            EffectDef::GainControl {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::BlockingSource,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                duration: ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
                controller: PlayerRefDef::EffectController,
            },
        ),
    ),
);

// LEG 122 — Touch of Darkness
pub(in crate::card::sets) static TOUCH_OF_DARKNESS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Touch of Darkness",
    "eda7177f-1354-4008-aaaa-2c8b823ed5e9",
    "Pete Venters",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "One or more target creatures become black until end of turn.",
        &ONE_OR_MORE_CREATURES,
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Black])),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 123 — Transmutation
pub(in crate::card::sets) static TRANSMUTATION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Transmutation",
    "329f7eb2-eadf-46ec-aed4-63152051f3c1",
    "Susan Van Camp",
    // Removal against a Wall and a pump against nothing: what it does depends
    // entirely on which way the creature was lopsided.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Switch target creature's power and toughness until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::switch_power_toughness(),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 124 — Underworld Dreams
// Audit: unsupported — Needs an opponent-draw event trigger that deals damage to the exact player who drew.
pub(in crate::card::sets) static UNDERWORLD_DREAMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Underworld Dreams",
    "a0e8f8d8-eac0-451c-a167-be84667a8e3d",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// LEG 125 — Vampire Bats
pub(in crate::card::sets) static VAMPIRE_BATS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Vampire Bats",
    "6a6a6f50-7b86-461e-80a7-e35d0e7cf52f",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{B}"), &["Bat"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{B}: This creature gets +1/+0 until end of turn. Activate no more than twice \
             each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(2),
    ]),
);

// LEG 126 — Walking Dead
pub(in crate::card::sets) static WALKING_DEAD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Walking Dead",
    "d7533a72-77d1-40cd-b3a1-7597d566c428",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 1, 1).with_abilities(&[
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// LEG 127 — Wall of Putrid Flesh
pub(in crate::card::sets) static WALL_OF_PUTRID_FLESH: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Putrid Flesh",
    "07a17b74-a9c9-419a-8369-9ab4fec213f2",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Wall"], 2, 4).with_abilities(&[
        abilities::defender(),
        abilities::protection_from_color(ManaColor::White),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by enchanted creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_damage_from(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Enchanted,
                ])),
            },
        ),
    ]),
);

// LEG 128 — Wall of Shadows
// Audit: unsupported — Targeting restrictions cannot be conditioned on how narrow the targeting spell or ability is.
pub(in crate::card::sets) static WALL_OF_SHADOWS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Shadows",
    "eb351900-cffd-4d23-b82f-5fb12a4874d9",
    "Pete Venters",
    CardRules::unsupported(),
);

// LEG 129 — Wall of Tombstones
// Audit: unsupported — Needs a zone-object query and identity-preserving continuation for “At the beginning of your upkeep, change this creature's base toughness to 1 plus the number of creature cards in your graveyard”.
pub(in crate::card::sets) static WALL_OF_TOMBSTONES: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Wall of Tombstones",
    "55da1e86-fe18-486a-b510-f941e6f6e378",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// LEG 130 — Active Volcano
pub(in crate::card::sets) static ACTIVE_VOLCANO: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Active Volcano",
    "ad402e65-6fac-4005-a2d4-592983df0c30",
    "Justin Hampton",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target blue permanent.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target Island to its owner's hand.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
    )),
);

// LEG 131 — Aerathi Berserker
pub(in crate::card::sets) static AERATHI_BERSERKER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Aerathi Berserker",
    "06673800-22a7-4ee3-92fa-7c7cd4865d30",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{2}{R}{R}{R}"), &["Human", "Berserker"], 2, 4).with_abilities(&[
        abilities::rampage(3, "Rampage 3 (Whenever this creature becomes blocked, it gets +3/+3 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 132 — Backdraft
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “Choose a player who cast one or more sorcery spells this turn. Backdraft deals damage to that player equal to half the damage dealt by one of those sorcery spells this turn, rounded down”.
pub(in crate::card::sets) static BACKDRAFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Backdraft",
    "58d5b9fe-b66a-48c9-94c4-db783e605f37",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 133 — Beasts of Bogardan
pub(in crate::card::sets) static BEASTS_OF_BOGARDAN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Beasts of Bogardan",
    "f885d776-2953-4ed4-b63f-91dc2b42783b",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Beast"], 3, 3).with_abilities(&[
        abilities::protection_from_color(ManaColor::Red),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as an opponent controls a nontoken white permanent.",
            EffectDef::IfCondition {
                // Nontoken, so a board of white tokens across the table leaves the Beasts at
                // their printed size.
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Color(ManaColor::White),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
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
    ]),
);

// LEG 134 — Blazing Effigy
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “When this creature dies, it deals X damage to target creature, where X is 3 plus the amount of damage dealt to this creature this turn by other sources named Blazing Effigy”.
pub(in crate::card::sets) static BLAZING_EFFIGY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Blazing Effigy",
    "921011ff-1696-4575-9198-abe993a0ee7a",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 135 — Blood Lust
// Audit: unsupported — Needs a characteristic-layer effect or dynamic value for “If target creature has toughness 5 or greater, it gets +4/-4 until end of turn. Otherwise, it gets +4/-X until end of turn, where X is its toughness minus 1”.
pub(in crate::card::sets) static BLOOD_LUST: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Blood Lust",
    "fbbf1a9c-8b94-4ee7-92db-65b531149990",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// LEG 136 — Caverns of Despair
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “No more than two creatures can attack each combat”.
pub(in crate::card::sets) static CAVERNS_OF_DESPAIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Caverns of Despair",
    "209f7479-b3a0-4c27-9602-78babb8d2e99",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// LEG 137 — Chain Lightning
pub(in crate::card::sets) static CHAIN_LIGHTNING: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Chain Lightning",
    "b5883762-ca0a-4932-8d2a-41a45796a5f8",
    "Sandra Everingham",
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Chain Lightning deals 3 damage to any target. Then that player or that permanent's controller may pay {R}{R}. If the player does, they may copy this spell and may choose a new target for that copy.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    mana_cost!("{R}{R}"),
                ),
                &EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                        object: EffectRecipientDef::object(ObjectRefDef::ResolvingObject),
                        controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        count: ValueDef::Constant(1),
                        retarget: true,
                        colors: None,
                    }),
                },
            )),
        ]),
    )),
);

// LEG 138 — Crevasse
pub(in crate::card::sets) static CREVASSE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Crevasse",
    "a432d6ae-a17f-484b-ad55-4b4b6674ba8d",
    "Rob Alexander",
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with mountainwalk can be blocked as though they didn't have mountainwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Mountain),
    )]),
);

// LEG 139 — Crimson Kobolds
pub(in crate::card::sets) static CRIMSON_KOBOLDS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Crimson Kobolds",
    "13696657-aeef-4add-9a3b-8137fce01fe3",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 140 — Crimson Manticore
pub(in crate::card::sets) static CRIMSON_MANTICORE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Crimson Manticore",
    "96f73f9c-1c4e-4343-bfa0-cc5c4a7a562e",
    "Daniel Gelon",
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
    CardSet::Legends,
    "Crookshank Kobolds",
    "7af6b119-7db4-49dd-aaa4-044b8c133f13",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 142 — Disharmony
pub(in crate::card::sets) static DISHARMONY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Disharmony",
    "e09a6b9b-eb0e-475d-8558-08e347412790",
    "Bryon Wackwitz",
    CardRules::new_instant(mana_cost!("{2}{R}"))
        .cast_only_before_blockers_declared()
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Cast this spell only during combat before blockers are declared.",
                "The play option refuses the cast outside combat and once blockers are in.",
            ),
            AbilityDef::spell_with_targets(
                "Untap target attacking creature and remove it from combat. Gain control of \
                 that creature until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                )],
                // Control before the removal, though the card prints it after. A later
                // instruction in the same resolution re-checks the target against "attacking
                // creature", and removing it from combat makes it stop matching. The outcome
                // is the same either way.
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::GainControl {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        duration: ControlDurationDef::UntilEndOfTurn,
                        controller: PlayerRefDef::EffectController,
                    },
                    EffectDef::RemoveFromCombat {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ]),
            ),
        ]),
);

// LEG 143 — Dwarven Song
pub(in crate::card::sets) static DWARVEN_SONG: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Dwarven Song",
    "29a50f72-9524-4440-9380-9d3e0b693351",
    "Dan Frazier",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "One or more target creatures become red until end of turn.",
        &ONE_OR_MORE_CREATURES,
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 144 — Eternal Warrior
pub(in crate::card::sets) static ETERNAL_WARRIOR: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Eternal Warrior",
    "97cdc38e-1d96-4de2-98e2-713f5d4d2180",
    "Anson Maddocks",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            ),
        ]),
);

// LEG 145 — Falling Star
// Audit: unsupported — Needs the EC physical flip, overlap, and landing evaluation needed to choose which creatures take damage and become tapped.
pub(in crate::card::sets) static FALLING_STAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Falling Star",
    "f2b9983e-20d4-4d12-9e2c-ec6d9a345787",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// LEG 146 — Feint
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “Tap all creatures blocking target attacking creature. Prevent all combat damage that would be dealt this turn by that creature and each creature blocking it”.
pub(in crate::card::sets) static FEINT: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Feint",
    "95b265bc-a94d-403b-8232-8fdfa0f8d9d5",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 147 — Firestorm Phoenix
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “If this creature would die, return it to its owner's hand instead. Until that player's next turn, that player plays with that card revealed in their hand and can't play it”.
pub(in crate::card::sets) static FIRESTORM_PHOENIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Firestorm Phoenix",
    "3247a7dd-f48c-4cb4-8475-4864acccef7a",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// LEG 148 — Frost Giant
pub(in crate::card::sets) static FROST_GIANT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Frost Giant",
    "6955d54f-7b37-4e43-8183-51677fb1ee11",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{3}{R}{R}{R}"), &["Giant"], 4, 4).with_abilities(&[
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 149 — Giant Strength
pub(in crate::card::sets) static GIANT_STRENGTH: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Giant Strength",
    "a86190bb-1f41-4128-b9fb-dfb1d178359d",
    "Justin Hampton",
    CardRules::new_enchantment(mana_cost!("{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
        ]),
);

// LEG 150 — Glyph of Destruction
pub(in crate::card::sets) static GLYPH_OF_DESTRUCTION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Glyph of Destruction",
    "8e9c153c-9224-491b-bc84-8a9f0a83ee5a",
    "Susan Van Camp",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target blocking Wall you control gets +10/+0 until end of combat. Prevent all damage \
         that would be dealt to it this turn. Destroy it at the beginning of the next end step.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Wall"),
                    ObjectPredicateDef::Blocking,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        // Three durations in one card, and none of them is the same: the pump ends
        // with combat, the shield lasts the turn, and the destruction waits for the
        // next end step -- which is later than either.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(10),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfCombat,
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next end step, destroy that Wall.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ))),
        ]),
    )),
);

// LEG 151 — Gravity Sphere
pub(in crate::card::sets) static GRAVITY_SPHERE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Gravity Sphere",
    "a2749332-e99a-4a0c-b3a3-5578b552fa11",
    "Brian Snõddy",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures lose flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
            },
        )),
);

// LEG 152 — Hyperion Blacksmith
// Audit: unsupported — Needs a persistent tap/untap restriction or event relation for “{T}: You may tap or untap target artifact an opponent controls”.
pub(in crate::card::sets) static HYPERION_BLACKSMITH: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Hyperion Blacksmith",
    "44d499a9-fe7c-4a1a-9eb3-a7fd9f85ae08",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// LEG 153 — Immolation
pub(in crate::card::sets) static IMMOLATION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Immolation",
    "9b3d34fa-398c-4ea0-a392-6690bd3a615c",
    "Scott Kirschner",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/-2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(-2),
                    ),
                },
            ),
        ]),
);

// LEG 154 — Kobold Drill Sergeant
pub(in crate::card::sets) static KOBOLD_DRILL_SERGEANT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kobold Drill Sergeant",
    "741b14f8-625d-41be-a734-0efe042a6ee8",
    "Julie Baroh",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kobold", "Soldier"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "Other Kobold creatures you control get +0/+1 and have trample.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Kobold"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
            },
        ),
    ),
);

// LEG 155 — Kobold Overlord
pub(in crate::card::sets) static KOBOLD_OVERLORD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kobold Overlord",
    "490eeedb-9c03-4dc7-81fd-ae54a7932e4d",
    "Julie Baroh",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kobold"], 1, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::static_ability(
            "Other Kobold creatures you control have first strike.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Kobold"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            },
        ),
    ]),
);

// LEG 156 — Kobold Taskmaster
pub(in crate::card::sets) static KOBOLD_TASKMASTER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kobold Taskmaster",
    "1b9c63eb-8d4e-4d8b-8637-308459ef036b",
    "Randy Asplund-Faith",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kobold"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "Other Kobold creatures you control get +1/+0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Kobold"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ),
);

// LEG 157 — Kobolds of Kher Keep
pub(in crate::card::sets) static KOBOLDS_OF_KHER_KEEP: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kobolds of Kher Keep",
    "df0320d9-7c2a-456a-9159-1b4fae67bfb5",
    "Julie Baroh",
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 158 — Land's Edge
// Audit: unsupported — Needs an ability any player may activate and a conditional keyed to the type of the card the discard cost actually took.
pub(in crate::card::sets) static LAND_S_EDGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Land's Edge",
    "fb3d9b29-948c-4768-b5ea-db2512817c30",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 159 — Mountain Yeti
pub(in crate::card::sets) static MOUNTAIN_YETI: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Mountain Yeti",
    "09242f08-3bfc-4082-b32f-703c7fed62a0",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Yeti"], 3, 3).with_abilities(&[
        abilities::mountainwalk(),
        abilities::protection_from_color(ManaColor::White),
    ]),
);

// LEG 160 — Primordial Ooze
/// X is read after the counter goes on, so the toll is the size the Ooze has
/// just grown to rather than the size it was.
static PRIMORDIAL_OOZE_X: ValueDef = ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne);

pub(in crate::card::sets) static PRIMORDIAL_OOZE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Primordial Ooze",
    "a46e47e1-8639-48f7-94c4-5f9e9666839a",
    "Sandra Everingham",
    CardRules::new_creature(mana_cost!("{R}"), &["Ooze"], 1, 1).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a +1/+1 counter on this creature. Then you \
             may pay {X}, where X is the number of +1/+1 counters on it. If you don't, tap \
             this creature and it deals X damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::PayOr(PayOrDef {
                    payment: EffectPaymentDef::generic_mana(
                        PlayerSetDef::One(PlayerRefDef::EffectController),
                        PRIMORDIAL_OOZE_X,
                    ),
                    if_paid: None,
                    otherwise: Some(&EffectDef::Sequence(&[
                        EffectDef::Tap {
                            object: EffectRecipientDef::Source,
                        },
                        EffectDef::DealDamage {
                            recipient: EffectRecipientDef::Controller,
                            amount: PRIMORDIAL_OOZE_X,
                        },
                    ])),
                    visibility: ChoiceVisibilityDef::Private,
                    condition: None,
                }),
            ]),
        ),
    ]),
);

// LEG 161 — Pyrotechnics
pub(in crate::card::sets) static PYROTECHNICS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Pyrotechnics",
    "2646284b-a94d-4c99-98d4-7becbb473e2b",
    "Anson Maddocks",
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Pyrotechnics deals 4 damage divided as you choose among any number of targets.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::AnyTarget,
            minimum: 1,
            maximum: 4,
            exact_count: None,
            divided_total: Some(DividedTotal::Fixed(4)),
            another: false,
            excludes_source: false,
            chooser: TargetChooserDef::Controller,
        }],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::DividedAmongTargets,
        },
    )),
);

// LEG 162 — Quarum Trench Gnomes
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “{T}: If target Plains is tapped for mana, it produces colorless mana instead of white mana”.
pub(in crate::card::sets) static QUARUM_TRENCH_GNOMES: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Quarum Trench Gnomes",
    "1c3b33bf-3074-406e-86f3-2a9843cf4862",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// LEG 163 — Raging Bull
pub(in crate::card::sets) static RAGING_BULL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Raging Bull",
    "ec10a51c-d2c3-4d14-9a71-9e59155bf980",
    "Randy Asplund-Faith",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ox"], 2, 2),
);

// LEG 164 — Spinal Villain
pub(in crate::card::sets) static SPINAL_VILLAIN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Spinal Villain",
    "d6d5e36f-0049-4be8-bf85-8dc0186339a4",
    "Anson Maddocks",
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
                then: None,
            },
        ),
    ),
);

// LEG 165 — Storm World
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “At the beginning of each player's upkeep, this enchantment deals X damage to that player, where X is 4 minus the number of cards in their hand”.
pub(in crate::card::sets) static STORM_WORLD: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Storm World",
    "cdf0afa6-7003-454d-9b8a-e3328aaf29ed",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// LEG 166 — Tempest Efreet
// Audit: unsupported — Needs random hand reveal, an opponent life-payment choice, and a permanent ownership exchange between cards in different zones.
pub(in crate::card::sets) static TEMPEST_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Tempest Efreet",
    "eb562143-fdf0-4eed-83ac-551627c576d2",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// LEG 167 — The Brute
pub(in crate::card::sets) static THE_BRUTE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "The Brute",
    "f9ffb265-872f-47b3-974c-92bcbebd557e",
    "Mark Poole",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
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
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks a creature, that creature can't attack during its controller's next turn”.
pub(in crate::card::sets) static WALL_OF_DUST: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Wall of Dust",
    "aaf53dfe-5d48-4811-b2f5-5a5c1cb462ca",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// LEG 169 — Wall of Earth
pub(in crate::card::sets) static WALL_OF_EARTH: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Earth",
    "c12e97c1-ca28-432a-8140-3f08bb4485a3",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Wall"], 0, 6)
        .with_ability(abilities::defender()),
);

// LEG 170 — Wall of Heat
pub(in crate::card::sets) static WALL_OF_HEAT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Heat",
    "a38059a8-be69-4cc1-969b-951c610f2f11",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wall"], 2, 6)
        .with_ability(abilities::defender()),
);

// LEG 171 — Wall of Opposition
pub(in crate::card::sets) static WALL_OF_OPPOSITION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wall of Opposition",
    "2b3d1430-9978-4983-a4fd-d1fa8dea2169",
    "Harold McNeill",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Wall"], 0, 6).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{1}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
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

// LEG 172 — Winds of Change
// Audit: unsupported — Needs a hidden-zone decision and continuation for “Each player shuffles the cards from their hand into their library, then draws that many cards”.
pub(in crate::card::sets) static WINDS_OF_CHANGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Winds of Change",
    "186fd917-8d65-4de5-8546-a32a5f6d3bab",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// LEG 173 — Aisling Leprechaun
pub(in crate::card::sets) static AISLING_LEPRECHAUN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Aisling Leprechaun",
    "640a161d-ad7b-4e5b-8f2d-d3753cb9daa3",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{G}"), &["Faerie"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a creature, that creature \
             becomes green. (This effect lasts indefinitely.)",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ),
);

// LEG 174 — Arboria
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “Creatures can't attack a player unless that player cast a spell or put a nontoken permanent onto the battlefield during their last turn”.
pub(in crate::card::sets) static ARBORIA: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Arboria",
    "095078b0-0f26-442f-9d3b-45e30cdb33c4",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// LEG 175 — Avoid Fate
// Audit: unsupported — Needs a spell-on-stack target predicate that expresses the printed instant/Aura restriction for “Counter target instant or Aura spell that targets a permanent you control”.
pub(in crate::card::sets) static AVOID_FATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Avoid Fate",
    "92f1509e-6ed5-4009-a031-ea84b43cbd1b",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 176 — Barbary Apes
pub(in crate::card::sets) static BARBARY_APES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Barbary Apes",
    "df25ffdd-995d-46ae-856b-f6368f9438ed",
    "Bryon Wackwitz",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ape"], 2, 2),
);

// LEG 177 — Cat Warriors
pub(in crate::card::sets) static CAT_WARRIORS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Cat Warriors",
    "d2187a64-2823-4f58-ad35-70f8913db2dc",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Cat", "Warrior"], 2, 2)
        .with_ability(abilities::forestwalk()),
);

// LEG 178 — Cocoon
static COCOON_STILL_WRAPPED: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::named("pupa"),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

pub(in crate::card::sets) static COCOON: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Cocoon",
    "a82c87b1-de37-4423-a1a4-533a1d8108b2",
    "Mark Tedin",
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature_you_control(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature and put three pupa counters \
                 on this Aura.",
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::AttachedPermanent,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("pupa"),
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during your untap step if this Aura has a \
                 pupa counter on it.",
                EffectDef::IfCondition {
                    condition: &COCOON_STILL_WRAPPED,
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                    },
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, remove a pupa counter from this Aura. If \
                 you can't, sacrifice it, put a +1/+1 counter on enchanted creature, and \
                 that creature gains flying.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                // Choose the branch before removing the last counter: the upkeep that sheds it
                // is not also the upkeep that opens the Cocoon.
                EffectDef::IfElseCondition {
                    condition: &COCOON_STILL_WRAPPED,
                    then: &EffectDef::RemoveCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("pupa"),
                        amount: ValueDef::Constant(1),
                    },
                    // The reward is handed out before the Aura goes, though the card prints it
                    // after: once the Aura leaves there is nothing attached to give it to.
                    otherwise: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::AttachedPermanent,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::AttachedPermanent,
                            effect: AppliedEffectDef::add_ability(&abilities::flying()),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::Source,
                        },
                    ]),
                },
            ),
        ]),
);

// LEG 179 — Concordant Crossroads
pub(in crate::card::sets) static CONCORDANT_CROSSROADS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Concordant Crossroads",
    "3bdcfae4-86c9-4d8a-bcfe-f0a928ec29db",
    "Amy Weber",
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        )),
);

// LEG 180 — Craw Giant
pub(in crate::card::sets) static CRAW_GIANT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Craw Giant",
    "707dadf0-735f-445d-9240-e49660913314",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}{G}"), &["Giant"], 6, 4).with_abilities(&[
        abilities::trample(),
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 181 — Deadfall
pub(in crate::card::sets) static DEADFALL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Deadfall",
    "0d78f0fc-3ab2-46ee-b5a9-55ae97d08c1a",
    "NéNé Thomas",
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with forestwalk can be blocked as though they didn't have forestwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Forest),
    )]),
);

// LEG 182 — Durkwood Boars
pub(in crate::card::sets) static DURKWOOD_BOARS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Durkwood Boars",
    "8d41f08b-68fb-45f2-bdc9-488baedc7d6f",
    "Mike Kimble",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Boar"], 4, 4),
);

// LEG 183 — Elven Riders
pub(in crate::card::sets) static ELVEN_RIDERS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Elven Riders",
    "ad1d349b-b5ab-4b2b-9b39-f8d8f6374aa5",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elf"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by Walls and/or creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Wall"),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ])),
                )),
            },
        ),
    ),
);

// LEG 184 — Emerald Dragonfly
pub(in crate::card::sets) static EMERALD_DRAGONFLY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Emerald Dragonfly",
    "a3e81250-52c3-49f6-be43-17c34339e177",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}{G}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 185 — Eureka
// Audit: unsupported — Needs a hidden-zone decision and continuation for “Starting with you, each player may put a permanent card from their hand onto the battlefield. Repeat this process until no one puts a card onto the battlefield”.
pub(in crate::card::sets) static EUREKA: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Eureka",
    "520db5fb-d961-45a3-af74-6f054b8be3ab",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 186 — Fire Sprites
pub(in crate::card::sets) static FIRE_SPRITES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Fire Sprites",
    "d26fa79a-ede8-4c80-98d5-f49696f8104d",
    "Julie Baroh",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_mana(
            "{G}, {T}: Add {R}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// LEG 187 — Floral Spuzzem
pub(in crate::card::sets) static FLORAL_SPUZZEM: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Floral Spuzzem",
    "d141b9e3-7129-41e5-8b44-d3867e1c7e1d",
    "Rob Alexander",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elemental"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may destroy target artifact \
             defending player controls. If you do, this creature assigns no combat damage this \
             turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: true,
                        then: None,
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

// LEG 188 — Giant Turtle
pub(in crate::card::sets) static GIANT_TURTLE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Giant Turtle",
    "87e5fc19-3b10-476f-9a73-e8bf4b5fbec0",
    "Jeff A. Menges",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Turtle"], 2, 4).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack if it attacked during your last turn.",
            EffectDef::StaticApply {
                // The condition lives on the recipient, so the prohibition is read live: it
                // applies on the turn after the Turtle swung and lifts on the one after
                // that, without anything being installed or expired.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::AttackedDuringControllersLastTurn,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
    ),
);

// LEG 189 — Glyph of Reincarnation
// Audit: unsupported — Needs a duration-scoped prohibition on creating or applying regeneration shields for “Destroy all creatures that were blocked by target Wall this turn. They can't be regenerated. For each creature that died this way, put a creature card from the graveyard of the player…”.
pub(in crate::card::sets) static GLYPH_OF_REINCARNATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Glyph of Reincarnation",
    "a67e8214-a192-4143-9d5e-d0e254e1bf6e",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// LEG 190 — Hornet Cobra
pub(in crate::card::sets) static HORNET_COBRA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Hornet Cobra",
    "27180bad-9bbc-462b-8832-626dc403a3fd",
    "Sandra Everingham",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Snake"], 2, 1)
        .with_ability(abilities::first_strike()),
);

// LEG 191 — Ichneumon Druid
// Audit: unsupported — Needs per-player, per-turn instant-cast counts in the triggering condition.
pub(in crate::card::sets) static ICHNEUMON_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Ichneumon Druid",
    "cf2313bb-6f9b-49d6-b069-9f3b77b6e107",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// LEG 192 — Killer Bees
pub(in crate::card::sets) static KILLER_BEES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Killer Bees",
    "2e30b5ff-1239-4c4d-ac7c-554ecf8e1e27",
    "Phil Foglio",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Insect"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
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

// LEG 193 — Living Plane
pub(in crate::card::sets) static LIVING_PLANE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Living Plane",
    "0341da27-3d77-4959-b7fa-5929b2cc7141",
    "Bryon Wackwitz",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All lands are 1/1 creatures that are still lands.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                ]),
            },
        )),
);

// LEG 194 — Master of the Hunt
pub(in crate::card::sets) static MASTER_OF_THE_HUNT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Master of the Hunt",
    "4e6bf56e-2d74-4e4d-a667-885853979377",
    "Jeff A. Menges",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Human"], 2, 2).with_ability(
        AbilityDef::activated(
            "{2}{G}{G}: Create a 1/1 green Wolf creature token named Wolves of the Hunt. It \
             has \"bands with other creatures named Wolves of the Hunt.\"",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}{G}"))],
            EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Green], 1, 1)
                .with_name("Wolves of the Hunt")
                .with_abilities(&[abilities::bands_with_other(BandingQuality::WolvesOfTheHunt)]),
        ),
    ),
);

// LEG 195 — Moss Monster
pub(in crate::card::sets) static MOSS_MONSTER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Moss Monster",
    "9903c043-9a7a-4994-b532-136d4c46edfd",
    "Jesper Myrfors",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 3, 6),
);

// LEG 196 — Pixie Queen
pub(in crate::card::sets) static PIXIE_QUEEN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Pixie Queen",
    "b9527c2a-23bb-4d33-9e72-6e0ab3de0e6b",
    "Quinton Hoover",
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
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 197 — Pradesh Gypsies
pub(in crate::card::sets) static PRADESH_GYPSIES: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Pradesh Gypsies",
    "0370330d-83d9-44d2-a1ed-c4827edc60fd",
    "Quinton Hoover",
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 198 — Rabid Wombat
static WOMBAT_BONUS: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    // Every Aura on it counts, whoever controls them.
    ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Subtype("Aura"),
            ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
    )),
    2,
));

pub(in crate::card::sets) static RABID_WOMBAT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Rabid Wombat",
    "9d9b9eb8-6367-4ab5-8e00-a9c9e1d69032",
    "Kaja Foglio",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Wombat"], 0, 1).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "This creature gets +2/+2 for each Aura attached to it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(WOMBAT_BONUS, WOMBAT_BONUS),
            },
        ),
    ]),
);

// LEG 199 — Radjan Spirit
pub(in crate::card::sets) static RADJAN_SPIRIT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Radjan Spirit",
    "adf3ab1a-5714-4b69-bc51-3752312b2d1f",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spirit"], 3, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature loses flying until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 200 — Rebirth
// Audit: unsupported — Needs a separate optional ante choice for each player and a conditional life-total-setting continuation for each player who antes.
pub(in crate::card::sets) static REBIRTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Rebirth",
    "a0d3ce01-e344-4608-b709-320be3600019",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// LEG 201 — Reincarnation
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “Choose target creature. When that creature dies this turn, return a creature card from its owner's graveyard to the battlefield under the control of that creature's owner”.
pub(in crate::card::sets) static REINCARNATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Reincarnation",
    "969c104e-daf4-480d-99a2-dd93c498b48e",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// LEG 202 — Revelation
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “Players play with their hands revealed”.
pub(in crate::card::sets) static REVELATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Revelation",
    "07892b6c-08d2-47b5-8d64-0e4d1bdc3080",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 203 — Rust
// Audit: unsupported — Needs stack targeting and countering of ability objects for “Counter target activated ability from an artifact source”.
pub(in crate::card::sets) static RUST: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Rust",
    "ad4974c8-34c5-4290-b325-7586a67f6d56",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// LEG 204 — Shelkin Brownie
pub(in crate::card::sets) static SHELKIN_BROWNIE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Shelkin Brownie",
    "fddcc557-871d-425b-b4ee-bc0c9bc717aa",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ouphe"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature loses all \"bands with other\" abilities until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::AnyBandsWithOther),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// LEG 205 — Storm Seeker
// Audit: unsupported — Needs damage-history/source tracking or card-specific damage processing for “Storm Seeker deals damage to target player equal to the number of cards in that player's hand”.
pub(in crate::card::sets) static STORM_SEEKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Storm Seeker",
    "3b66d0cc-84d7-41ad-b0e7-74ebf604543f",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// LEG 206 — Subdue
pub(in crate::card::sets) static SUBDUE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Subdue",
    "123d6097-8021-46cd-a8c3-01013245e347",
    "Brian Snõddy",
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all combat damage that would be dealt by target creature this turn. That \
         creature gets +0/+X until end of turn, where X is its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// LEG 207 — Sylvan Library
pub(in crate::card::sets) static SYLVAN_LIBRARY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sylvan Library",
    "f486df00-7c4a-4ff0-bb0b-c8b5432ac742",
    "Harold McNeill",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
    .with_abilities(&[AbilityDef::triggered(
        "At the beginning of your draw step, you may draw two additional cards. If you do, choose two cards in your hand drawn this turn. For each of those pay 4 life or put the card on top of your library.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Draw,
            player: PlayerRelation::You,
        },
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &const {
                EffectDef::Sequence(&const {
                    [
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::OrderedObjects(
                                ParentBinding,
                            ),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::CardsDrawnThisTurnInHand(
                                PlayerRefDef::EffectController,
                            ),
                            exclude: None,
                            minimum: 2,
                            maximum: 2,
                            visibility: ChoiceVisibilityDef::Private,
                            then: &const {
                                EffectDef::ForEachInBinding {
                                    objects: ParentBinding,
                                    binding: ParentBinding,
                                    effect: &const {
                                        EffectDef::PayOr(PayOrDef::unless(
                                            EffectPaymentDef::life(
                                                PlayerSetDef::One(
                                                    PlayerRefDef::EffectController,
                                                ),
                                                4,
                                            ),
                                            &const {
                                                EffectDef::MoveToZone {
                                                    object: EffectRecipientDef::object(
                                                        ObjectRefDef::Binding(
                                                            ParentBinding,
                                                        ),
                                                    ),
                                                    zone: ZoneKind::Library,
                                                    placement: ZonePlacement::Top,
                                                }
                                            },
                                        ))
                                    },
                                }
                            },
                        }),
                    ]
                })
            },
        },
    )]),
);

// LEG 208 — Sylvan Paradise
pub(in crate::card::sets) static SYLVAN_PARADISE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sylvan Paradise",
    "f323c3bb-cece-4035-b1a7-c4817cf7a08c",
    "Randy Asplund-Faith",
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "One or more target creatures become green until end of turn.",
        &ONE_OR_MORE_CREATURES,
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 209 — Typhoon
pub(in crate::card::sets) static TYPHOON: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Typhoon",
    "254e0403-67d8-4e73-8d89-c901ebeba49f",
    "Anson Maddocks",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Typhoon deals damage to each opponent equal to the number of Islands that player controls.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            )),
        },
    )),
);

// LEG 210 — Untamed Wilds
pub(in crate::card::sets) static UNTAMED_WILDS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Untamed Wilds",
    "887f22af-8b92-422a-9cd5-f3977674bcdc",
    "NéNé Thomas",
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
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// LEG 211 — Whirling Dervish
pub(in crate::card::sets) static WHIRLING_DERVISH: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Whirling Dervish",
    "eba294e7-7097-4bc3-b396-72e85dd4f441",
    "Susan Van Camp",
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Human", "Monk"], 1, 1)
        .with_abilities(&[
            abilities::protection_from_color(ManaColor::Black),
            AbilityDef::triggered_if(
                "At the beginning of each end step, if this creature dealt damage to an opponent this turn, put a +1/+1 counter on it.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                &TriggerConditionDef::SourceDealtDamageToOpponentThisTurn,
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
    CardSet::Legends,
    "Willow Satyr",
    "0c8b1f49-550e-405f-b17c-1d94589494ad",
    "Jeff A. Menges",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Satyr"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "You may choose not to untap this creature during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
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
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::WhileSourceRemains { while_tapped: true },
                controller: PlayerRefDef::EffectController,
            },
        ),
    ]),
);

// LEG 213 — Winter Blast
// Audit: unsupported — Needs one chosen-X target set with a flying-dependent damage follow-up linked to the creatures it tapped.
pub(in crate::card::sets) static WINTER_BLAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Winter Blast",
    "fb846366-2105-4999-8af1-a11687f42e17",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 214 — Wolverine Pack
pub(in crate::card::sets) static WOLVERINE_PACK: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Wolverine Pack",
    "ba5aee52-095e-4c69-93eb-5adac11ed1fc",
    "Jeff A. Menges",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Wolverine"], 2, 4).with_abilities(&[
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 215 — Wood Elemental
// Audit: unsupported — Needs a characteristic-layer effect or dynamic value for “Wood Elemental's power and toughness are each equal to the number of Forests sacrificed as it entered”.
pub(in crate::card::sets) static WOOD_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Wood Elemental",
    "edc71122-2951-43eb-8ca8-1cda6d231013",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// LEG 216 — Adun Oakenshield
pub(in crate::card::sets) static ADUN_OAKENSHIELD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Adun Oakenshield",
    "60252226-a102-4d88-9b80-42d021b5184d",
    "Jeff A. Menges",
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
            },
        )),
);

// LEG 217 — Angus Mackenzie
pub(in crate::card::sets) static ANGUS_MACKENZIE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Angus Mackenzie",
    "57264bd9-94f6-4d4d-baff-2b2900585635",
    "Bryon Wackwitz",
    CardRules::new_creature(mana_cost!("{G}{W}{U}"), &["Human", "Cleric"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(
            AbilityDef::activated(
                "{G}{W}{U}, {T}: Prevent all combat damage that would be dealt this turn. \
                 Activate only before the combat damage step.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{G}{W}{U}")),
                    AbilityCostDef::TapSource,
                ],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_timing(ActivationTimingDef::BeforeCombatDamage),
        ),
);

// LEG 218 — Arcades Sabboth
pub(in crate::card::sets) static ARCADES_SABBOTH: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Arcades Sabboth",
    "2c1dbc62-ceb5-4540-ae38-901e5deafc75",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(
        mana_cost!("{2}{G}{G}{W}{W}{U}{U}"),
        &["Elder", "Dragon"],
        7,
        7,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice Arcades Sabboth unless you pay {G}{W}{U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{G}{W}{U}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::static_ability(
            "Each untapped creature you control gets +0/+2 as long as it's not attacking.",
            // Untapped and not attacking, both read continuously -- so a creature that
            // taps for mana or is declared as an attacker loses the toughness at once.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
            },
        ),
        AbilityDef::activated(
            "{W}: Arcades Sabboth gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 219 — Axelrod Gunnarson
pub(in crate::card::sets) static AXELROD_GUNNARSON: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Axelrod Gunnarson",
    "acce83cf-965b-4e45-8efb-63f814df7a35",
    "Scott Kirschner",
    CardRules::new_creature(
        mana_cost!("{4}{B}{B}{R}{R}"),
        &["Giant"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::trample(),
        abilities::creature_damaged_by_source_dies_trigger_with_targets(
            "Whenever a creature dealt damage by Axelrod Gunnarson this turn dies, you gain 1 life and Axelrod Gunnarson deals 1 damage to any target.",
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
// Audit: unsupported — Needs stack targeting and countering of ability objects for “{T}: Counter target activated ability from an artifact source unless that ability's controller pays {W}”.
pub(in crate::card::sets) static AYESHA_TANAKA: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Ayesha Tanaka",
    "03bd3d4b-d01a-475d-bf3b-cf96f43bc9ef",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// LEG 221 — Barktooth Warbeard
pub(in crate::card::sets) static BARKTOOTH_WARBEARD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Barktooth Warbeard",
    "0ea52228-f8ad-4623-9e05-f162473bfc03",
    "Andi Rusu",
    CardRules::new_creature(mana_cost!("{4}{B}{R}{R}"), &["Human", "Warrior"], 6, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 222 — Bartel Runeaxe
// Audit: unsupported — Needs the card's exact Aura targeting/attachment restriction rather than the broader existing cannot-be-enchanted effect for “Bartel Runeaxe can't be the target of Aura spells”.
pub(in crate::card::sets) static BARTEL_RUNEAXE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Bartel Runeaxe",
    "f1a42691-98bb-4234-9b56-085e6677f3e4",
    "Andi Rusu",
    crate::card::CardRules::unsupported(),
);

// LEG 223 — Boris Devilboon
pub(in crate::card::sets) static BORIS_DEVILBOON: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Boris Devilboon",
    "82ae30e8-2dcd-46b8-925b-cc24e11fb95d",
    "Jesper Myrfors",
    CardRules::new_creature(mana_cost!("{3}{B}{R}"), &["Zombie", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{2}{B}{R}, {T}: Create a 1/1 black and red Demon creature token named Minor Demon.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{R}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::create_creature_token(&["Demon"], &[ManaColor::Black, ManaColor::Red], 1, 1)
                .with_name("Minor Demon"),
        )),
);

// LEG 224 — Chromium
pub(in crate::card::sets) static CHROMIUM: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Chromium",
    "8cd7d7e1-f928-4429-9a59-ba0590a78e98",
    "Edward P. Beard, Jr.",
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
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{W}{U}{B}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// LEG 225 — Dakkon Blackblade
static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static DAKKON_BLACKBLADE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Dakkon Blackblade",
    "fbfd1278-1486-4516-8846-007ce1985ee9",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{2}{W}{U}{U}{B}"), &["Human", "Warrior"], 0, 0)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Dakkon Blackblade's power and toughness are each equal to the number of lands you \
             control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                ),
            },
        )),
);

// LEG 226 — Gabriel Angelfire
// Audit: unsupported — Needs a random choice among four named abilities and a grant of the chosen one; the randomized effect vocabulary selects between two branches, not among four.
pub(in crate::card::sets) static GABRIEL_ANGELFIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Gabriel Angelfire",
    "3b4ce350-b6ed-4e0e-8c70-efc6e5f18a5d",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// LEG 227 — Gosta Dirk
pub(in crate::card::sets) static GOSTA_DIRK: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Gosta Dirk",
    "92ef316b-dd22-40d1-82e8-8890976684c0",
    "Richard Thomas",
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
    CardSet::Legends,
    "Gwendlyn Di Corci",
    "473d70b6-a88c-49f4-9415-19919c4468ae",
    "Julie Baroh",
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
                    then: None,
                },
            )
            .with_activation_timing(ActivationTimingDef::YourTurn),
        ),
);

// LEG 229 — Halfdane
// Audit: unsupported — Needs a characteristic-layer effect or dynamic value for “At the beginning of your upkeep, change Halfdane's base power and toughness to the power and toughness of target creature other than Halfdane until the end of your next upkeep”.
pub(in crate::card::sets) static HALFDANE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Halfdane",
    "2e939761-3542-4044-9038-d1d30c6a38fc",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// LEG 230 — Hazezon Tamar
// Audit: unsupported — Needs delayed token creation that determines X at the future upkeep for “When Hazezon enters, create X 1/1 Sand Warrior creature tokens that are red, green, and white at the beginning of your next upkeep, where X is the number of lands you control at that time”.
pub(in crate::card::sets) static HAZEZON_TAMAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Hazezon Tamar",
    "17fc3a85-c6b9-4fd2-a6a2-d3210708e5ea",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// LEG 231 — Hunding Gjornersen
pub(in crate::card::sets) static HUNDING_GJORNERSEN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Hunding Gjornersen",
    "07d8e501-6857-4a52-a3b9-2bf0bee5b08c",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{3}{W}{U}{U}"), &["Human", "Warrior"], 5, 4)
        .with_supertype(CardSupertype::Legendary).with_abilities(&[
        abilities::rampage(1, "Rampage 1 (Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 232 — Jacques le Vert
pub(in crate::card::sets) static JACQUES_LE_VERT: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Jacques le Vert",
    "ee5a45b1-169b-468e-9251-424c09cd7f0f",
    "Andi Rusu",
    CardRules::new_creature(mana_cost!("{1}{R}{G}{W}"), &["Human", "Warrior"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Green creatures you control get +0/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
            },
        )),
);

// LEG 233 — Jasmine Boreal
pub(in crate::card::sets) static JASMINE_BOREAL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Jasmine Boreal",
    "db6ef678-4ce9-48d6-aa4f-2afd9a1ad724",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Human"], 4, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 234 — Jedit Ojanen
pub(in crate::card::sets) static JEDIT_OJANEN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Jedit Ojanen",
    "97b80124-2b59-425c-93cc-9b032e631c6e",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{4}{W}{W}{U}"), &["Cat", "Warrior"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 235 — Jerrard of the Closed Fist
pub(in crate::card::sets) static JERRARD_OF_THE_CLOSED_FIST: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Jerrard of the Closed Fist",
    "7f841918-813b-4784-ab57-907185b0a355",
    "Andi Rusu",
    CardRules::new_creature(mana_cost!("{3}{R}{G}{G}"), &["Human", "Knight"], 6, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 236 — Johan
// Audit: unsupported — Needs a combat declaration or damage-assignment constraint for “At the beginning of combat on your turn, you may have Johan gain "Johan can't attack" until end of combat. If you do, attacking doesn't cause creatures you control to tap this combat if…”.
pub(in crate::card::sets) static JOHAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Johan",
    "b851d5b4-7991-49d4-8a52-bf233f960cbf",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// LEG 237 — Kasimir the Lone Wolf
pub(in crate::card::sets) static KASIMIR_THE_LONE_WOLF: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kasimir the Lone Wolf",
    "45b1e60d-54dd-41cd-b9a2-00890725a3df",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{4}{W}{U}"), &["Human", "Warrior"], 5, 3)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 238 — Kei Takahashi
pub(in crate::card::sets) static KEI_TAKAHASHI: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kei Takahashi",
    "6a4a524a-fdc7-432d-994b-953808528349",
    "Scott Kirschner",
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Human", "Cleric"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Prevent the next 2 damage that would be dealt to target creature this turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// LEG 239 — Lady Caleria
pub(in crate::card::sets) static LADY_CALERIA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Lady Caleria",
    "d6914ed2-9207-4689-9166-11d2f8949fdd",
    "Bryon Wackwitz",
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
    CardSet::Legends,
    "Lady Evangela",
    "f3e122e9-ffa3-48dd-94d6-8f2886668e59",
    "Mark Poole",
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
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// LEG 241 — Lady Orca
pub(in crate::card::sets) static LADY_ORCA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Lady Orca",
    "b2779553-74eb-42ba-97d0-96269f48c269",
    "Sandra Everingham",
    CardRules::new_creature(mana_cost!("{5}{B}{R}"), &["Demon"], 7, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 242 — Livonya Silone
pub(in crate::card::sets) static LIVONYA_SILONE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Livonya Silone",
    "b9211949-66a5-4039-ac6d-3e42b008b58e",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{2}{R}{R}{G}{G}"), &["Human", "Warrior"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::first_strike(), abilities::legendary_landwalk()]),
);

// LEG 243 — Lord Magnus
pub(in crate::card::sets) static LORD_MAGNUS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Lord Magnus",
    "2a02aabb-c464-4672-b37b-d5d713ef8939",
    "Mark Tedin",
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
    CardSet::Legends,
    "Marhault Elsdragon",
    "67330004-6720-46d9-9de0-c79230110583",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{3}{R}{R}{G}"), &["Elf", "Warrior"], 4, 6)
        .with_supertype(CardSupertype::Legendary).with_abilities(&[
        abilities::rampage(1, "Rampage 1 (Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 245 — Nebuchadnezzar
// Audit: unsupported — Needs seeded random selection with replay-visible provenance for “{X}, {T}: Choose a card name. Target opponent reveals X cards at random from their hand. Then that player discards all cards with that name revealed this way. Activate only during your turn”.
pub(in crate::card::sets) static NEBUCHADNEZZAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Nebuchadnezzar",
    "e3f17ce3-711b-4bd9-addf-dd440fa7d2b7",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// LEG 246 — Nicol Bolas
pub(in crate::card::sets) static NICOL_BOLAS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Nicol Bolas",
    "729feb73-4581-4f9d-ba47-bece72481b86",
    "Edward P. Beard, Jr.",
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
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{U}{B}{R}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::triggered(
            "Whenever Nicol Bolas deals damage to an opponent, that player discards their hand.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(i32::MAX),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// LEG 247 — Palladia-Mors
pub(in crate::card::sets) static PALLADIA_MORS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Palladia-Mors",
    "ad64874d-ce33-4e0a-bcca-723f129ef415",
    "Edward P. Beard, Jr.",
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
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{R}{G}{W}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// LEG 248 — Pavel Maliki
pub(in crate::card::sets) static PAVEL_MALIKI: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Pavel Maliki",
    "304f9d39-3ea2-4274-b23e-e4eaabbc1c4b",
    "Andi Rusu",
    CardRules::new_creature(mana_cost!("{4}{B}{R}"), &["Human"], 5, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{B}{R}: Pavel Maliki gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// LEG 249 — Princess Lucrezia
pub(in crate::card::sets) static PRINCESS_LUCREZIA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Princess Lucrezia",
    "a1dcf48c-2700-4024-807e-9244e4c649ac",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{3}{U}{U}{B}"), &["Human", "Wizard"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::tap_for(ManaColor::Blue)),
);

// LEG 250 — Ragnar
pub(in crate::card::sets) static RAGNAR: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Ragnar",
    "2cf6a3a3-4a06-4eb7-981a-b70cf05b2473",
    "Melissa A. Benson",
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
    CardSet::Legends,
    "Ramirez DePietro",
    "e5c66c61-aadf-433b-9958-fc9b44b327b9",
    "Phil Foglio",
    CardRules::new_creature(mana_cost!("{3}{U}{B}{B}"), &["Human", "Pirate"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::first_strike()),
);

// LEG 252 — Ramses Overdark
pub(in crate::card::sets) static RAMSES_OVERDARK: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Ramses Overdark",
    "f079c74e-a39a-40f9-9c7e-9319c0c189c6",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{2}{U}{U}{B}{B}"), &["Human", "Assassin"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Destroy target enchanted creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Enchanted,
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        )),
);

// LEG 253 — Rasputin Dreamweaver
// Audit: unsupported — Needs dream counters that both pay for and restore themselves across several abilities. Each individual prevention and mana effect is available.
pub(in crate::card::sets) static RASPUTIN_DREAMWEAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Rasputin Dreamweaver",
    "503256f8-3aab-49d0-b78b-6502aa29ce52",
    "Andi Rusu",
    crate::card::CardRules::unsupported(),
);

// LEG 254 — Riven Turnbull
pub(in crate::card::sets) static RIVEN_TURNBULL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Riven Turnbull",
    "d11f90e7-ced1-4d80-8083-99acbf459ad7",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{5}{U}{B}"), &["Human", "Advisor"], 5, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::tap_for(ManaColor::Black)),
);

// LEG 255 — Rohgahh of Kher Keep
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “At the beginning of your upkeep, you may pay {R}{R}{R}. If you don't, tap Rohgahh and all creatures named Kobolds of Kher Keep, then an opponent gains control of them”.
pub(in crate::card::sets) static ROHGAHH_OF_KHER_KEEP: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Rohgahh of Kher Keep",
    "ea0891f0-83ce-4eb7-b0a9-cbc8168bafff",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// LEG 256 — Rubinia Soulsinger
pub(in crate::card::sets) static RUBINIA_SOULSINGER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Rubinia Soulsinger",
    "f13e8dc9-8d0f-4a2c-8c0e-be70a3a7dc8e",
    "Rob Alexander",
    CardRules::new_creature(mana_cost!("{2}{G}{W}{U}"), &["Faerie"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may choose not to untap Rubinia Soulsinger during your untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayChooseNotToUntap),
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Gain control of target creature for as long as you control Rubinia \
                 Soulsinger and Rubinia Soulsinger remains tapped.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::WhileSourceRemains { while_tapped: true },
                    controller: PlayerRefDef::EffectController,
                },
            ),
        ]),
);

// LEG 257 — Sir Shandlar of Eberyn
pub(in crate::card::sets) static SIR_SHANDLAR_OF_EBERYN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sir Shandlar of Eberyn",
    "31570ded-f5e3-44c4-b95f-294ac10b2cd2",
    "Andi Rusu",
    CardRules::new_creature(mana_cost!("{4}{G}{W}"), &["Human", "Knight"], 4, 7)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 258 — Sivitri Scarzam
pub(in crate::card::sets) static SIVITRI_SCARZAM: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sivitri Scarzam",
    "9c12ee9e-db13-4b4d-a061-b6566f538f09",
    "NéNé Thomas",
    CardRules::new_creature(mana_cost!("{5}{U}{B}"), &["Human"], 6, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 259 — Sol'kanar the Swamp King
pub(in crate::card::sets) static SOLKANAR_THE_SWAMP_KING: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sol'kanar the Swamp King",
    "7a20dcb0-5350-40e0-82d3-c8d0186fc9d2",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{2}{U}{B}{R}"), &["Demon"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::landwalk(BasicLandType::Swamp),
            AbilityDef::triggered(
                "Whenever a player casts a black spell, you gain 1 life.",
                // Any player's: the predicate names a color and nothing about
                // who cast it.
                TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Black)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEG 260 — Stangg
// Audit: unsupported — Needs a zone-object query and identity-preserving continuation for “When Stangg enters, create Stangg Twin, a legendary 3/4 red and green Human Warrior creature token. Exile that token when Stangg leaves the battlefield. Sacrifice Stangg when that token…”.
pub(in crate::card::sets) static STANGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Stangg",
    "a277775a-6b48-4238-a618-3ae94c4cc85c",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// LEG 261 — Sunastian Falconer
pub(in crate::card::sets) static SUNASTIAN_FALCONER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sunastian Falconer",
    "587075f3-a568-4089-83ca-fe1e473c025d",
    "Christopher Rush",
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
    CardSet::Legends,
    "Tetsuo Umezawa",
    "8384f87b-26c2-45b7-98ef-352c384f205e",
    "Julie Baroh",
    CardRules::new_creature(mana_cost!("{U}{B}{R}"), &["Human", "Archer"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Tetsuo Umezawa can't be the target of Aura spells.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeEnchanted),
                },
            ),
            AbilityDef::activated_with_targets(
                "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking creature.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{U}{B}{B}{R}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Tapped,
                            ObjectPredicateDef::Blocking,
                        ]),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
        ]),
);

// LEG 263 — The Lady of the Mountain
pub(in crate::card::sets) static THE_LADY_OF_THE_MOUNTAIN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "The Lady of the Mountain",
    "83717eb2-220e-4086-be09-dee9174798b8",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{4}{R}{G}"), &["Giant"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 264 — Tobias Andrion
pub(in crate::card::sets) static TOBIAS_ANDRION: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Tobias Andrion",
    "cac56eda-5ed3-4abd-beec-f5063fbf930a",
    "Andi Rusu",
    CardRules::new_creature(mana_cost!("{3}{W}{U}"), &["Human", "Advisor"], 4, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 265 — Tor Wauki
pub(in crate::card::sets) static TOR_WAUKI: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Tor Wauki",
    "241a4854-e62c-4be4-a9cc-1e14db4eede9",
    "Randy Asplund-Faith",
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
    CardSet::Legends,
    "Torsten Von Ursus",
    "5fd99522-4a91-4ccd-91bf-5f32a6ac3510",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{W}"), &["Human", "Soldier"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 267 — Tuknir Deathlock
pub(in crate::card::sets) static TUKNIR_DEATHLOCK: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Tuknir Deathlock",
    "9dfbcb4d-a9ae-4d76-8dde-7312fbad56b0",
    "Liz Danforth",
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
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEG 268 — Ur-Drago
pub(in crate::card::sets) static UR_DRAGO: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Ur-Drago",
    "81a40f34-fc26-4d05-9c52-6ffbf1766a3b",
    "Christopher Rush",
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
    CardSet::Legends,
    "Vaevictis Asmadi",
    "22ea73ec-1325-4437-a23f-dcda1767c713",
    "Andi Rusu",
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
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{B}{R}{G}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::activated(
            "{B}: Vaevictis Asmadi gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{R}: Vaevictis Asmadi gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{G}: Vaevictis Asmadi gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
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

// LEG 270 — Xira Arien
pub(in crate::card::sets) static XIRA_ARIEN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Xira Arien",
    "cc6c7d89-32e7-4c3f-ac90-7db3a46eed4b",
    "Melissa A. Benson",
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
pub(in crate::card::sets) static AL_ABARAS_CARPET: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Al-abara's Carpet",
    "5d5aae6e-fe20-4363-9589-5a54bcbbb77e",
    "Kaja Foglio",
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{5}, {T}: Prevent all damage that would be dealt to you this turn by attacking \
         creatures without flying.",
        &[
            AbilityCostDef::Mana(mana_cost!("{5}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from_group_to(
                DamageSourceGroupDef::AttackingCreaturesWithoutFlying,
                EffectRecipientDef::Controller,
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 272 — Alchor's Tomb
pub(in crate::card::sets) static ALCHORS_TOMB: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Alchor's Tomb",
    "f4395b19-2118-4a09-8932-f9ce9bc54d6d",
    "Jesper Myrfors",
    // "Lasts indefinitely", so the repaint outlives the turn it was made in.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: Target permanent you control becomes the color of your choice.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::ChooseColor {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            operation: ColorChoiceOperationDef::BecomesChosenColor,
            duration: ResolvedEffectDurationDef::Permanent,
        },
    )),
);

// LEG 273 — Arena of the Ancients
pub(in crate::card::sets) static ARENA_OF_THE_ANCIENTS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Arena of the Ancients",
    "9337996e-a119-4529-b422-f6d286c78e3f",
    "Tom Wänerstrand",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "Legendary creatures don't untap during their controllers' untap steps.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        abilities::enters_trigger(
            "When this artifact enters, tap all legendary creatures.",
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            },
        ),
    ]),
);

// LEG 274 — Black Mana Battery
// Audit: unsupported — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {B}, then add an additional {B} for each charge counter removed this way”.
pub(in crate::card::sets) static BLACK_MANA_BATTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Black Mana Battery",
    "d0c66e64-e357-457d-8302-b3a1fc0c56ce",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// LEG 275 — Blue Mana Battery
// Audit: unsupported — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {U}, then add an additional {U} for each charge counter removed this way”.
pub(in crate::card::sets) static BLUE_MANA_BATTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Blue Mana Battery",
    "35393661-2c53-46f0-bb33-2390d552b060",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// LEG 276 — Bronze Horse
// Audit: unsupported — Needs a duration-scoped replacement/prevention effect for “As long as you control another creature, prevent all damage that would be dealt to this creature by spells that target it”.
pub(in crate::card::sets) static BRONZE_HORSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Bronze Horse",
    "936a03a5-73ba-436c-9d49-70e176d118e6",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// LEG 277 — Forethought Amulet
pub(in crate::card::sets) static FORETHOUGHT_AMULET: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Forethought Amulet",
    "700f53d3-0a84-4c55-8495-786f0f0783db",
    "Melissa A. Benson",
    CardRules::new_artifact(mana_cost!("{5}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this artifact unless you pay {3}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{3}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::static_ability(
            "If an instant or sorcery source would deal 3 or more damage to you, it deals 2 \
             damage to you instead.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage {
                    matcher: DamageEventMatcherDef {
                        kind: DamageKindDef::Any,
                        source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ])),
                        recipient: DamageRecipientMatcherDef::Any,
                    },
                    // Capping at two is the whole clause: an event already at
                    // two or less is untouched either way.
                    limit: DamageLimitDef::CapAt(2),
                }),
            },
        ),
    ]),
);

// LEG 278 — Gauntlets of Chaos
// Audit: unsupported — Needs duration-aware control-changing continuous effects for “{5}, Sacrifice this artifact: Exchange control of target artifact, creature, or land you control and target permanent an opponent controls that shares one of those types with it. If…”.
pub(in crate::card::sets) static GAUNTLETS_OF_CHAOS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Gauntlets of Chaos",
    "6f8f11b5-3ba8-4913-b76a-fb469a74864d",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// LEG 279 — Green Mana Battery
// Audit: unsupported — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {G}, then add an additional {G} for each charge counter removed this way”.
pub(in crate::card::sets) static GREEN_MANA_BATTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Green Mana Battery",
    "4671fa01-4a9e-4cd9-8154-b0d45e11b702",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// LEG 280 — Horn of Deafening
pub(in crate::card::sets) static HORN_OF_DEAFENING: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Horn of Deafening",
    "17eff8d9-86de-4f19-bf00-5f20dc1373d4",
    "Dan Frazier",
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: Prevent all combat damage that would be dealt by target creature this turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 281 — Knowledge Vault
// Audit: unsupported — Needs ordered-library inspection, selection, and visibility handling for “{2}, {T}: Exile the top card of your library face down”.
pub(in crate::card::sets) static KNOWLEDGE_VAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Knowledge Vault",
    "65947312-75c2-4baa-805c-238a154156ef",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// LEG 282 — Kry Shield
pub(in crate::card::sets) static KRY_SHIELD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Kry Shield",
    "a558f23c-c2ce-40d0-b894-f8ccbff8f622",
    "Richard Thomas",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: Prevent all damage that would be dealt this turn by target creature you \
         control. That creature gets +0/+X until end of turn, where X is its mana value.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// LEG 283 — Life Chisel
pub(in crate::card::sets) static LIFE_CHISEL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Life Chisel",
    "50052f9a-d667-4a96-a9b1-b4169ee495e6",
    "Anthony S. Waters",
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: You gain life equal to the sacrificed creature's toughness. \
             Activate only during your upkeep.",
            &[],
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: Some(&EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                }),
                amount: SacrificedAmountDef::Toughness,
                otherwise: None,
                optional: false,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// LEG 284 — Life Matrix
// Audit: unsupported — Needs granting a counter-consuming activated ability to a targeted creature and an activation window restricted to your upkeep for “{4}, {T}: Put a matrix counter on target creature and that creature gains "Remove a matrix counter from this creature: Regenerate this creature." Activate only during your upkeep”.
pub(in crate::card::sets) static LIFE_MATRIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Life Matrix",
    "c99a3abc-e2a3-4eee-8f72-b1b25dcd1d0b",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// LEG 285 — Mana Matrix
pub(in crate::card::sets) static MANA_MATRIX: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Mana Matrix",
    "a3eedc11-0b47-430c-8391-577a2d05c2ae",
    "Mark Tedin",
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::static_ability(
        "Instant and enchantment spells you cast cost {2} less to cast.",
        EffectDef::ModifyCost(CostModificationDef::reduce_spell(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            PlayerRelation::You,
            ValueDef::Constant(2),
        )),
    )),
);

// LEG 286 — Marble Priest
pub(in crate::card::sets) static MARBLE_PRIEST: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Marble Priest",
    "459b71d7-34c1-43b9-93ff-364f95aa4789",
    "Melissa A. Benson",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Cleric"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "All Walls able to block this creature do so.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::Subtype("Wall"),
                )),
            },
        ),
        AbilityDef::static_ability(
            "Prevent all combat damage that would be dealt to this creature by Walls.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_combat_damage_from(ObjectPredicateDef::Subtype(
                    "Wall",
                )),
            },
        ),
    ]),
);

// LEG 287 — Mirror Universe
// Audit: unsupported — Needs linked sacrifice/destruction accounting for “{T}, Sacrifice this artifact: Exchange life totals with target opponent. Activate only during your upkeep”.
pub(in crate::card::sets) static MIRROR_UNIVERSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Mirror Universe",
    "a8f05d5e-bb7d-4554-b880-f0c6b4688357",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 288 — North Star
// Audit: unsupported — Needs cost/mana provenance or dynamic payment support for “{4}, {T}: For one spell this turn, you may spend mana as though it were mana of any type to pay that spell's mana cost”.
pub(in crate::card::sets) static NORTH_STAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "North Star",
    "daac2a6b-27c8-4567-9e0c-7b262628d331",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// LEG 289 — Nova Pentacle
// Audit: unsupported — Needs a shield keyed to a source chosen as the ability resolves; prevention shields attach to a recipient and spend on the next damage from any source, not from one named source for “{3}, {T}: The next time a source of your choice would deal damage to you this turn, that damage is dealt to target creature of an opponent's choice instead”.
pub(in crate::card::sets) static NOVA_PENTACLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Nova Pentacle",
    "bc60e209-aa29-48aa-9128-9bb175403c0c",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// LEG 290 — Planar Gate
pub(in crate::card::sets) static PLANAR_GATE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Planar Gate",
    "dd27f0fe-c032-4f61-9f3d-98a6d2e2c426",
    "Melissa A. Benson",
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::static_ability(
        "Creature spells you cast cost {2} less to cast.",
        EffectDef::ModifyCost(CostModificationDef::reduce_spell(
            ObjectPredicateDef::HasType(CardType::Creature),
            PlayerRelation::You,
            ValueDef::Constant(2),
        )),
    )),
);

// LEG 291 — Red Mana Battery
// Audit: unsupported — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {R}, then add an additional {R} for each charge counter removed this way”.
pub(in crate::card::sets) static RED_MANA_BATTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Red Mana Battery",
    "363cc5d6-70f8-4a3c-92bd-8f49774bdce2",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// LEG 292 — Relic Barrier
pub(in crate::card::sets) static RELIC_BARRIER: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Relic Barrier",
    "c062cbae-ce5e-43be-9932-c81a0a3622e8",
    "Harold McNeill",
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
// Audit: unsupported — Needs a spell-on-stack target predicate that expresses the printed instant/Aura restriction for “{3}, {T}: Counter target instant or Aura spell that targets a permanent you control”.
pub(in crate::card::sets) static RING_OF_IMMORTALS: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Ring of Immortals",
    "61706102-67fd-4167-bd7d-ec6da41db362",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// LEG 294 — Sentinel
pub(in crate::card::sets) static SENTINEL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Sentinel",
    "5c970830-95cf-4471-af28-43da635073d0",
    "Randy Asplund-Faith",
    // Toughness alone: the Sentinel keeps its printed power of 1, and a
    // free ability that could be used repeatedly is why the effect sets
    // rather than modifies -- each use replaces the last.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Shapeshifter"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{0}: Change this creature's base toughness to 1 plus the power of target creature \
             blocking or blocked by this creature.",
            &[],
            // The printed "blocking or blocked by this creature" is the two one-sided
            // relationships together: what the Sentinel is blocking, and what is
            // blocking the Sentinel.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::BlockedBySource,
                    ObjectPredicateDef::BlockingSource,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // One plus that creature's power, read as the ability resolves.
                effect: AppliedEffectDef::set_base_toughness(ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(1),
                    ValueDef::TargetPower(TargetIndex::PRIMARY),
                ))),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ),
);

// LEG 295 — Serpent Generator
pub(in crate::card::sets) static SERPENT_GENERATOR: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Serpent Generator",
    "7c350c38-6cbb-4b8b-823f-45d6a16568cc",
    "Mark Tedin",
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::activated(
        "{4}, {T}: Create a 1/1 colorless Snake artifact creature token. It has \"Whenever \
         this creature deals damage to a player, that player gets a poison counter.\" (A player with ten or more poison counters loses the game.)",
        &[
            AbilityCostDef::Mana(mana_cost!("{4}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::create_artifact_creature_token(&["Snake"], &[], 1, 1).with_abilities(&[
            abilities::poisonous_damage(
                1,
                "Whenever this creature deals damage to a player, that player gets a poison counter.",
            ),
        ]),
    )),
);

// LEG 296 — Sword of the Ages
// Audit: unsupported — Needs a characteristic-layer effect or dynamic value for “{T}, Sacrifice this artifact and any number of creatures you control: This artifact deals X damage to any target, where X is the total power of the creatures sacrificed this way, then…”.
pub(in crate::card::sets) static SWORD_OF_THE_AGES: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Sword of the Ages",
    "b37fd4cc-ab5c-4c65-80ff-8f905b31e801",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// LEG 297 — Triassic Egg
// Audit: unsupported — Needs card-specific counter state and counter-consuming effects for “Sacrifice this artifact: Choose one. Activate only if there are two or more hatchling counters on this artifact”.
pub(in crate::card::sets) static TRIASSIC_EGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Triassic Egg",
    "ce3756f7-0d99-4562-b32d-66de18a58fdf",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// LEG 298 — Voodoo Doll
// Audit: unsupported — Needs a mana activation cost whose amount is read from the source rather than printed, for “{X}{X}, {T}: ... X is the number of pin counters on this artifact”. The upkeep tick, the untapped intervening-if, and damage equal to a counter count are all available.
pub(in crate::card::sets) static VOODOO_DOLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Voodoo Doll",
    "0418672d-056e-416d-91b4-8ee6e47201dc",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// LEG 299 — White Mana Battery
// Audit: unsupported — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {W}, then add an additional {W} for each charge counter removed this way”.
pub(in crate::card::sets) static WHITE_MANA_BATTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "White Mana Battery",
    "35fbbe41-d21b-4028-905f-054c44d30eb2",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// LEG 300 — Adventurers' Guildhouse
pub(in crate::card::sets) static ADVENTURERS_GUILDHOUSE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Adventurers' Guildhouse",
    "32865e68-5842-4f17-b2ea-4ffa743b511f",
    "Tom Wänerstrand",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::static_ability(
        "Green legendary creatures you control have \"bands with other legendary \
         creatures.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&BANDS_WITH_OTHER_LEGENDS),
        },
    )]),
);

// LEG 301 — Cathedral of Serra
pub(in crate::card::sets) static CATHEDRAL_OF_SERRA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Cathedral of Serra",
    "e65356e6-0ead-49fd-b069-be1ea9b1c105",
    "Mark Poole",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::static_ability(
        "White legendary creatures you control have \"bands with other legendary \
         creatures.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&BANDS_WITH_OTHER_LEGENDS),
        },
    )]),
);

// LEG 302 — Hammerheim
// Audit: unsupported — Needs removing every ability of a class from a target for a duration; the vocabulary grants named abilities but does not take them away.
pub(in crate::card::sets) static HAMMERHEIM: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Hammerheim",
    "2816d30e-1e52-4323-b30e-1688fba23368",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// LEG 303 — Karakas
pub(in crate::card::sets) static KARAKAS: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Karakas",
    "31d2422a-bb7d-4cdd-9aac-e5a936a4be3b",
    "Nicola Leonard",
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
                },
            ),
        ]),
);

// LEG 304 — Mountain Stronghold
pub(in crate::card::sets) static MOUNTAIN_STRONGHOLD: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Mountain Stronghold",
    "314fd1d7-4bd8-4d95-b7c2-1aa6660ab88a",
    "Tom Wänerstrand",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::static_ability(
        "Red legendary creatures you control have \"bands with other legendary \
         creatures.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ObjectPredicateDef::Color(ManaColor::Red),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&BANDS_WITH_OTHER_LEGENDS),
        },
    )]),
);

// LEG 305 — Pendelhaven
pub(in crate::card::sets) static PENDELHAVEN: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Pendelhaven",
    "79427109-c1f3-476d-a029-0049217237b5",
    "Bryon Wackwitz",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Green),
            AbilityDef::activated_with_targets(
                "{T}: Target 1/1 creature gets +1/+2 until end of turn.",
                &[AbilityCostDef::TapSource],
                // "Target 1/1 creature" is read as the creature is now, so a creature that
                // has already been pumped is not one, and one that stops being 1/1 before
                // the ability resolves loses the ability with it.
                &[AbilityTargetDef::exactly_one(
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
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEG 306 — Seafarer's Quay
pub(in crate::card::sets) static SEAFARERS_QUAY: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Seafarer's Quay",
    "66641d88-b3f0-4bcd-8d2d-29aa2de69e30",
    "Tom Wänerstrand",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::static_ability(
        "Blue legendary creatures you control have \"bands with other legendary \
         creatures.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&BANDS_WITH_OTHER_LEGENDS),
        },
    )]),
);

// LEG 307 — The Tabernacle at Pendrell Vale
pub(in crate::card::sets) static THE_TABERNACLE_AT_PENDRELL_VALE: CardRecord = CardRecord::new(
    CardSet::Legends,
    "The Tabernacle at Pendrell Vale",
    "64bc9b1d-5818-4d9e-b771-e49af4ff9a5c",
    "Nicola Leonard",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "All creatures have \"At the beginning of your upkeep, destroy this creature unless you pay {1}.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                    "At the beginning of your upkeep, destroy this creature unless you pay {1}.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::You,
                    },
                    EffectDef::PayOr(PayOrDef::unless_mana(
                        mana_cost!("{1}"),
                        &EffectDef::Destroy {
                            object: EffectRecipientDef::Source,
                            can_regenerate: true,
                            then: None,
                        },
                    )),
                )),
            },
        )),
);

// LEG 308 — Tolaria
pub(in crate::card::sets) static TOLARIA: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Tolaria",
    "d43c01b7-443d-4061-a934-6863d230c9b8",
    "Nicola Leonard",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Blue),
            AbilityDef::activated_with_targets(
                "{T}: Target creature loses banding and all \"bands with other\" abilities \
                 until end of turn. Activate only during any upkeep step.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    // Two removals in one clause: the printed text takes plain banding and the
                    // variant together, and neither implies the other.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                            KeywordAbility::Banding,
                        )),
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::AnyBandsWithOther),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_timing(ActivationTimingDef::AnyUpkeep),
        ]),
);

// LEG 309 — Unholy Citadel
pub(in crate::card::sets) static UNHOLY_CITADEL: CardRecord = CardRecord::new(
    CardSet::Legends,
    "Unholy Citadel",
    "9de534ff-fb48-4692-bd0f-dd237ca28502",
    "Mark Poole",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::static_ability(
        "Black legendary creatures you control have \"bands with other legendary \
         creatures.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&BANDS_WITH_OTHER_LEGENDS),
        },
    )]),
);

// LEG 310 — Urborg
// Audit: unsupported — Needs a modal choice between two named abilities and the removal of the chosen one; the vocabulary grants named abilities but does not take them away.
pub(in crate::card::sets) static URBORG: CardRecord = CardRecord::new(
    crate::card::CardSet::Legends,
    "Urborg",
    "d0a261d0-7678-46f7-9285-d541486567d8",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKRON_LEGIONNAIRE,
    &ALABASTER_POTION,
    &AMROU_KITHKIN,
    &ANGELIC_VOICES,
    &CLEANSE,
    &CLERGY_OF_THE_HOLY_NIMBUS,
    &DAVENANT_ARCHER,
    &DIVINE_INTERVENTION,
    &DIVINE_OFFERING,
    &DIVINE_TRANSFORMATION,
    &ELDER_LAND_WURM,
    &ENCHANTED_BEING,
    &EQUINOX,
    &FORTIFIED_AREA,
    &GLYPH_OF_LIFE,
    &GREAT_DEFENDER,
    &GREAT_WALL,
    &GREATER_REALM_OF_PRESERVATION,
    &HEAVENS_GATE,
    &HOLY_DAY,
    &INDESTRUCTIBLE_AURA,
    &INFINITE_AUTHORITY,
    &IVORY_GUARDIANS,
    &KEEPERS_OF_THE_FAITH,
    &KISMET,
    &LAND_TAX,
    &LIFEBLOOD,
    &MOAT,
    &OSAI_VULTURES,
    &PETRA_SPHINX,
    &PRESENCE_OF_THE_MASTER,
    &RAPID_FIRE,
    &REMOVE_ENCHANTMENTS,
    &RIGHTEOUS_AVENGERS,
    &SEEKER,
    &SHIELD_WALL,
    &SPIRIT_LINK,
    &SPIRITUAL_SANCTUARY,
    &THUNDER_SPIRIT,
    &TUNDRA_WOLVES,
    &VISIONS,
    &WALL_OF_CALTROPS,
    &WALL_OF_LIGHT,
    &ACID_RAIN,
    &ANTI_MAGIC_AURA,
    &AZURE_DRAKE,
    &BACKFIRE,
    &BOOMERANG,
    &BRINE_HAG,
    &DEVOURING_DEEP,
    &DREAM_COAT,
    &ELDER_SPAWN,
    &ENCHANTMENT_ALTERATION,
    &ENERGY_TAP,
    &FIELD_OF_DREAMS,
    &FLASH_COUNTER,
    &FLASH_FLOOD,
    &FORCE_SPIKE,
    &GASEOUS_FORM,
    &GLYPH_OF_DELUSION,
    &IN_THE_EYE_OF_CHAOS,
    &INVOKE_PREJUDICE,
    &JUXTAPOSE,
    &LAND_EQUILIBRIUM,
    &MANA_DRAIN,
    &PART_WATER,
    &PSIONIC_ENTITY,
    &PSYCHIC_PURGE,
    &PUPPET_MASTER,
    &RECALL,
    &RELIC_BIND,
    &REMOVE_SOUL,
    &RESET,
    &REVERBERATION,
    &SEA_KINGS_BLESSING,
    &SEGOVIAN_LEVIATHAN,
    &SILHOUETTE,
    &SPECTRAL_CLOAK,
    &TELEKINESIS,
    &TELEPORT,
    &TIME_ELEMENTAL,
    &UNDERTOW,
    &VENARIAN_GOLD,
    &WALL_OF_VAPOR,
    &WALL_OF_WONDER,
    &ZEPHYR_FALCON,
    &ABOMINATION,
    &ALL_HALLOW_S_EVE,
    &BLIGHT,
    &CARRION_ANTS,
    &CHAINS_OF_MEPHISTOPHELES,
    &COSMIC_HORROR,
    &CYCLOPEAN_MUMMY,
    &DARKNESS,
    &DEMONIC_TORMENT,
    &EVIL_EYE_OF_ORMS_BY_GORE,
    &FALLEN_ANGEL,
    &GHOSTS_OF_THE_DAMNED,
    &GIANT_SLUG,
    &GLYPH_OF_DOOM,
    &GREED,
    &HEADLESS_HORSEMAN,
    &HELL_SWARM,
    &HELLS_CARETAKER,
    &HELLFIRE,
    &HORROR_OF_HORRORS,
    &IMPRISON,
    &INFERNAL_MEDUSA,
    &JOVIAL_EVIL,
    &LESSER_WEREWOLF,
    &LOST_SOUL,
    &MOLD_DEMON,
    &NETHER_VOID,
    &PIT_SCORPION,
    &QUAGMIRE,
    &SHIMIAN_NIGHT_STALKER,
    &SPIRIT_SHACKLE,
    &SYPHON_SOUL,
    &TAKKLEMAGGOT,
    &THE_ABYSS,
    &THE_WRETCHED,
    &TOUCH_OF_DARKNESS,
    &TRANSMUTATION,
    &UNDERWORLD_DREAMS,
    &VAMPIRE_BATS,
    &WALKING_DEAD,
    &WALL_OF_PUTRID_FLESH,
    &WALL_OF_SHADOWS,
    &WALL_OF_TOMBSTONES,
    &ACTIVE_VOLCANO,
    &AERATHI_BERSERKER,
    &BACKDRAFT,
    &BEASTS_OF_BOGARDAN,
    &BLAZING_EFFIGY,
    &BLOOD_LUST,
    &CAVERNS_OF_DESPAIR,
    &CHAIN_LIGHTNING,
    &CREVASSE,
    &CRIMSON_KOBOLDS,
    &CRIMSON_MANTICORE,
    &CROOKSHANK_KOBOLDS,
    &DISHARMONY,
    &DWARVEN_SONG,
    &ETERNAL_WARRIOR,
    &FALLING_STAR,
    &FEINT,
    &FIRESTORM_PHOENIX,
    &FROST_GIANT,
    &GIANT_STRENGTH,
    &GLYPH_OF_DESTRUCTION,
    &GRAVITY_SPHERE,
    &HYPERION_BLACKSMITH,
    &IMMOLATION,
    &KOBOLD_DRILL_SERGEANT,
    &KOBOLD_OVERLORD,
    &KOBOLD_TASKMASTER,
    &KOBOLDS_OF_KHER_KEEP,
    &LAND_S_EDGE,
    &MOUNTAIN_YETI,
    &PRIMORDIAL_OOZE,
    &PYROTECHNICS,
    &QUARUM_TRENCH_GNOMES,
    &RAGING_BULL,
    &SPINAL_VILLAIN,
    &STORM_WORLD,
    &TEMPEST_EFREET,
    &THE_BRUTE,
    &WALL_OF_DUST,
    &WALL_OF_EARTH,
    &WALL_OF_HEAT,
    &WALL_OF_OPPOSITION,
    &WINDS_OF_CHANGE,
    &AISLING_LEPRECHAUN,
    &ARBORIA,
    &AVOID_FATE,
    &BARBARY_APES,
    &CAT_WARRIORS,
    &COCOON,
    &CONCORDANT_CROSSROADS,
    &CRAW_GIANT,
    &DEADFALL,
    &DURKWOOD_BOARS,
    &ELVEN_RIDERS,
    &EMERALD_DRAGONFLY,
    &EUREKA,
    &FIRE_SPRITES,
    &FLORAL_SPUZZEM,
    &GIANT_TURTLE,
    &GLYPH_OF_REINCARNATION,
    &HORNET_COBRA,
    &ICHNEUMON_DRUID,
    &KILLER_BEES,
    &LIVING_PLANE,
    &MASTER_OF_THE_HUNT,
    &MOSS_MONSTER,
    &PIXIE_QUEEN,
    &PRADESH_GYPSIES,
    &RABID_WOMBAT,
    &RADJAN_SPIRIT,
    &REBIRTH,
    &REINCARNATION,
    &REVELATION,
    &RUST,
    &SHELKIN_BROWNIE,
    &STORM_SEEKER,
    &SUBDUE,
    &SYLVAN_LIBRARY,
    &SYLVAN_PARADISE,
    &TYPHOON,
    &UNTAMED_WILDS,
    &WHIRLING_DERVISH,
    &WILLOW_SATYR,
    &WINTER_BLAST,
    &WOLVERINE_PACK,
    &WOOD_ELEMENTAL,
    &ADUN_OAKENSHIELD,
    &ANGUS_MACKENZIE,
    &ARCADES_SABBOTH,
    &AXELROD_GUNNARSON,
    &AYESHA_TANAKA,
    &BARKTOOTH_WARBEARD,
    &BARTEL_RUNEAXE,
    &BORIS_DEVILBOON,
    &CHROMIUM,
    &DAKKON_BLACKBLADE,
    &GABRIEL_ANGELFIRE,
    &GOSTA_DIRK,
    &GWENDLYN_DI_CORCI,
    &HALFDANE,
    &HAZEZON_TAMAR,
    &HUNDING_GJORNERSEN,
    &JACQUES_LE_VERT,
    &JASMINE_BOREAL,
    &JEDIT_OJANEN,
    &JERRARD_OF_THE_CLOSED_FIST,
    &JOHAN,
    &KASIMIR_THE_LONE_WOLF,
    &KEI_TAKAHASHI,
    &LADY_CALERIA,
    &LADY_EVANGELA,
    &LADY_ORCA,
    &LIVONYA_SILONE,
    &LORD_MAGNUS,
    &MARHAULT_ELSDRAGON,
    &NEBUCHADNEZZAR,
    &NICOL_BOLAS,
    &PALLADIA_MORS,
    &PAVEL_MALIKI,
    &PRINCESS_LUCREZIA,
    &RAGNAR,
    &RAMIREZ_DEPIETRO,
    &RAMSES_OVERDARK,
    &RASPUTIN_DREAMWEAVER,
    &RIVEN_TURNBULL,
    &ROHGAHH_OF_KHER_KEEP,
    &RUBINIA_SOULSINGER,
    &SIR_SHANDLAR_OF_EBERYN,
    &SIVITRI_SCARZAM,
    &SOLKANAR_THE_SWAMP_KING,
    &STANGG,
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
    &AL_ABARAS_CARPET,
    &ALCHORS_TOMB,
    &ARENA_OF_THE_ANCIENTS,
    &BLACK_MANA_BATTERY,
    &BLUE_MANA_BATTERY,
    &BRONZE_HORSE,
    &FORETHOUGHT_AMULET,
    &GAUNTLETS_OF_CHAOS,
    &GREEN_MANA_BATTERY,
    &HORN_OF_DEAFENING,
    &KNOWLEDGE_VAULT,
    &KRY_SHIELD,
    &LIFE_CHISEL,
    &LIFE_MATRIX,
    &MANA_MATRIX,
    &MARBLE_PRIEST,
    &MIRROR_UNIVERSE,
    &NORTH_STAR,
    &NOVA_PENTACLE,
    &PLANAR_GATE,
    &RED_MANA_BATTERY,
    &RELIC_BARRIER,
    &RING_OF_IMMORTALS,
    &SENTINEL,
    &SERPENT_GENERATOR,
    &SWORD_OF_THE_AGES,
    &TRIASSIC_EGG,
    &VOODOO_DOLL,
    &WHITE_MANA_BATTERY,
    &ADVENTURERS_GUILDHOUSE,
    &CATHEDRAL_OF_SERRA,
    &HAMMERHEIM,
    &KARAKAS,
    &MOUNTAIN_STRONGHOLD,
    &PENDELHAVEN,
    &SEAFARERS_QUAY,
    &THE_TABERNACLE_AT_PENDRELL_VALE,
    &TOLARIA,
    &UNHOLY_CITADEL,
    &URBORG,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
