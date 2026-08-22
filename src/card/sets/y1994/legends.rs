use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef,
    AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, BandingQuality, BasicLandType, BattlefieldEntryModificationDef, CardArt,
    CardBehavior, CardRules, CardSet, CardSupertype, CardType, CardTypeSet, ChoiceVisibilityDef,
    ChooseDef, ColorChoiceOperationDef, ColorSet, ComparisonDef, ControlDurationDef, CounterKind,
    DamageEventMatcherDef, DamageKindDef, DamageLimitDef, DamagePreventionDef,
    DamageRecipientMatcherDef, DamageSourceGroupDef, DamageSourceMatcherDef, DiscardSelectionDef,
    DividedTotal, EffectDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef,
    KeywordAbility, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    ScaledValueDef, SumValueDef, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex, TargetIndex};
use crate::mana_cost;

static INDESTRUCTIBLE_AURA_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

// LEG 1 — Akron Legionnaire
pub(in crate::card::sets) static AKRON_LEGIONNAIRE: CardRecord = CardRecord::new_with_legacy_id(
    1577,
    "Akron Legionnaire",
    CardArt::new("5d074af2-8dbd-42d3-87eb-30f6e7d171ff", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{6}{W}{W}"), &["Giant", "Soldier"], 8, 4).with_ability(
        AbilityDef::static_ability(
            "Except for creatures named Akron Legionnaire and artifact creatures, creatures \
             you control can't attack.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::SharesNameWithSource,
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
pub(in crate::card::sets) static ALABASTER_POTION: CardRecord = CardRecord::new_with_legacy_id(
    1582,
    "Alabaster Potion",
    CardArt::new("2806c7f6-8fdd-4e65-9c71-f2e8b0cdede2", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{X}{W}{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Target player gains X life.\n• Prevent the next X damage that would \
         be dealt to any target this turn.",
        &[
            AbilityDef::spell_with_targets(
                "Target player gains X life",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
            ),
            AbilityDef::spell_with_targets(
                "Prevent the next X damage that would be dealt to any target this turn",
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
// Audit: partial — Its blocker power predicate omits modifiers from static continuous effects.
pub(in crate::card::sets) static AMROU_KITHKIN: CardRecord = CardRecord::new_with_legacy_id(
    397,
    "Amrou Kithkin",
    CardArt::new("cbce1c55-123c-4a05-bde4-18a1601fcc5a", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Kithkin"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked by creatures with power 3 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::PowerAtLeast(3),
                )),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Blocking checks printed and temporary power but not power from static continuous effects.",
        )),
    ),
);

/// An absence rather than a presence, so the anthem switches off the moment
/// one off-plan creature arrives and comes back when it leaves.
static ANGELIC_VOICES_PURITY: TriggerConditionDef = TriggerConditionDef::ObjectCount {
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
};

static ANGELIC_VOICES_ANTHEM: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

// LEG 4 — Angelic Voices
pub(in crate::card::sets) static ANGELIC_VOICES: CardRecord = CardRecord::new_with_legacy_id(
    1914,
    "Angelic Voices",
    CardArt::new("8068c263-e5fa-4449-8887-418e9d0a4da4", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control get +1/+1 as long as you control no nonartifact, nonwhite \
         creatures.",
        EffectDef::IfCondition {
            condition: &ANGELIC_VOICES_PURITY,
            then: &ANGELIC_VOICES_ANTHEM,
        },
    )),
);

// LEG 5 — Cleanse
pub(in crate::card::sets) static CLEANSE: CardRecord = CardRecord::new_with_legacy_id(
    398,
    "Cleanse",
    CardArt::new("2fbd611b-ac97-4516-bad7-cc9ee4ef74f7", "Phil Foglio"),
    CardSet::Legends,
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
        },
    )),
);

// LEG 6 — Clergy of the Holy Nimbus
// Audit: blocked — Needs a would-be-destroyed replacement that regenerates the source, and an activation restricted to opponents. The turn-scoped regeneration prohibition its second clause applies is available.

// LEG 7 — D'Avenant Archer
pub(in crate::card::sets) static DAVENANT_ARCHER: CardRecord = CardRecord::new_with_legacy_id(
    399,
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
pub(in crate::card::sets) static DIVINE_OFFERING: CardRecord = CardRecord::new_with_legacy_id(
    70,
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
pub(in crate::card::sets) static DIVINE_TRANSFORMATION: CardRecord = CardRecord::new_with_legacy_id(
    400,
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
pub(in crate::card::sets) static ELDER_LAND_WURM: CardRecord = CardRecord::new_with_legacy_id(
    1795,
    "Elder Land Wurm",
    CardArt::new("ef3651d4-969c-464d-a444-40a640d0c6ba", "Quinton Hoover"),
    CardSet::Legends,
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
pub(in crate::card::sets) static ENCHANTED_BEING: CardRecord = CardRecord::new_with_legacy_id(
    1670,
    "Enchanted Being",
    CardArt::new("94c2880d-b37a-43ea-9fee-cd5a8ed75a7e", "Douglas Shuler"),
    CardSet::Legends,
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
// Audit: blocked — Needs a granted ability that can target a spell by prospectively determining whether that spell would destroy one of its controller's lands.

/// Both halves of one clause: the bonus and the keyword arrive together, so
/// they are one applied effect rather than two abilities.
static FORTIFIED_AREA_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&FORTIFIED_AREA_BANDING),
];

static FORTIFIED_AREA_BANDING: AbilityDef = abilities::banding();

// LEG 14 — Fortified Area
pub(in crate::card::sets) static FORTIFIED_AREA: CardRecord = CardRecord::new_with_legacy_id(
    1792,
    "Fortified Area",
    CardArt::new(
        "dc64f19c-5b2b-4697-b4dc-2be9c3790794",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Wall creatures you control get +1/+0 and have banding.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Subtype("Wall"),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&FORTIFIED_AREA_BONUS),
        },
    )),
);

// LEG 15 — Glyph of Life
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Choose target Wall creature. Whenever that creature is dealt damage by an attacking creature this turn, you gain that much life”.

// LEG 16 — Great Defender
pub(in crate::card::sets) static GREAT_DEFENDER: CardRecord = CardRecord::new_with_legacy_id(
    401,
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
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 17 — Great Wall
pub(in crate::card::sets) static GREAT_WALL: CardRecord = CardRecord::new_with_legacy_id(
    1394,
    "Great Wall",
    CardArt::new("cd860a1d-aa17-4579-b9b1-d101d2416387", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with plainswalk can be blocked as though they didn't have plainswalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Plains),
    )]),
);

// LEG 18 — Greater Realm of Preservation
pub(in crate::card::sets) static GREATER_REALM_OF_PRESERVATION: CardRecord = CardRecord::new_with_legacy_id(
    1452,
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

// LEG 19 — Heaven's Gate
pub(in crate::card::sets) static HEAVENS_GATE: CardRecord = CardRecord::new_with_legacy_id(
    1766,
    "Heaven's Gate",
    CardArt::new("6c18b729-41cc-450c-8ee1-cd38a4c07a1c", "Douglas Shuler"),
    CardSet::Legends,
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
pub(in crate::card::sets) static HOLY_DAY: CardRecord = CardRecord::new_with_legacy_id(
    1407,
    "Holy Day",
    CardArt::new("f6c95a2b-bf44-4ff2-9c6a-916773346edd", "Justin Hampton"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 21 — Indestructible Aura
pub(in crate::card::sets) static INDESTRUCTIBLE_AURA: CardRecord = CardRecord::new_with_legacy_id(
    1410,
    "Indestructible Aura",
    CardArt::new("ed2a7333-c9ce-4011-b00e-1304e1eec25e", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Prevent all damage that would be dealt to target creature this turn.",
        &INDESTRUCTIBLE_AURA_TARGET,
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEG 22 — Infinite Authority
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever enchanted creature blocks or becomes blocked by a creature with toughness 3 or less, destroy the other creature at end of combat. At the beginning of the next end step, if that…”.

static AN_OPPONENT_CONTROLS_A_NONTOKEN_RED_PERMANENT: TriggerConditionDef =
    TriggerConditionDef::ObjectCount {
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
    };

/// "Creatures named Ivory Guardians", so a second copy pumps the first and
/// an opponent's is covered too.
static IVORY_GUARDIANS_ANTHEM: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::SharesNameWithSource,
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

// LEG 23 — Ivory Guardians
pub(in crate::card::sets) static IVORY_GUARDIANS: CardRecord = CardRecord::new_with_legacy_id(
    1918,
    "Ivory Guardians",
    CardArt::new("9bf9cccd-fe97-4632-a90a-9eeb0d41135e", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Giant", "Cleric"], 3, 3).with_abilities(&[
        abilities::protection_from(ManaColor::Red),
        AbilityDef::static_ability(
            "Creatures named Ivory Guardians get +1/+1 as long as an opponent controls a nontoken \
             red permanent.",
            EffectDef::IfCondition {
                condition: &AN_OPPONENT_CONTROLS_A_NONTOKEN_RED_PERMANENT,
                then: &IVORY_GUARDIANS_ANTHEM,
            },
        ),
    ]),
);

// LEG 24 — Keepers of the Faith
pub(in crate::card::sets) static KEEPERS_OF_THE_FAITH: CardRecord = CardRecord::new_with_legacy_id(
    402,
    "Keepers of the Faith",
    CardArt::new("b63a69ae-99ce-4d26-88b7-784793c43cd4", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 2, 3),
);

// LEG 25 — Kismet
pub(in crate::card::sets) static KISMET: CardRecord = CardRecord::new_with_legacy_id(
    403,
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
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )),
);

// LEG 26 — Land Tax
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “At the beginning of your upkeep, if an opponent controls more lands than you, you may search your library for up to three basic land reveal them, put them into your hand, then shuffle”.

// LEG 27 — Lifeblood
pub(in crate::card::sets) static LIFEBLOOD: CardRecord = CardRecord::new_with_legacy_id(
    404,
    "Lifeblood",
    CardArt::new("4ecb1362-9a67-4d4c-8d69-9ac2ebf4d0b0", "Mark Tedin"),
    CardSet::Legends,
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

static MOAT_HAS_FLYING: ObjectPredicateDef = ObjectPredicateDef::HasKeyword(KeywordAbility::Flying);
static MOAT_NONFLYING_CREATURES: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&MOAT_HAS_FLYING),
]);

// LEG 28 — Moat
pub(in crate::card::sets) static MOAT: CardRecord = CardRecord::new_with_legacy_id(
    119,
    "Moat",
    CardArt::new("952ba126-0915-47f0-9b6a-a0a6dcd22c6f", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures without flying can't attack.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                MOAT_NONFLYING_CREATURES,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
        },
    )),
);

/// Every end step, not only its controller's: something dying on either turn
/// feeds it.
static OSAI_VULTURES_CARRION: TriggerConditionDef = TriggerConditionDef::CreatureDiedThisTurn;

// LEG 29 — Osai Vultures
pub(in crate::card::sets) static OSAI_VULTURES: CardRecord = CardRecord::new_with_legacy_id(
    1818,
    "Osai Vultures",
    CardArt::new("f85614b3-62a3-4da9-a74a-7ea40fad1b52", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if(
            "At the beginning of each end step, if a creature died this turn, put a carrion \
             counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &OSAI_VULTURES_CARRION,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Carrion,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove two carrion counters from this creature: This creature gets +1/+1 until \
             end of turn.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::Carrion,
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
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{T}: Target player chooses a card name, then reveals the top card of their library. If that card has the chosen name, that player puts it into their hand. If it doesn't, the player puts…”.

// LEG 31 — Presence of the Master
pub(in crate::card::sets) static PRESENCE_OF_THE_MASTER: CardRecord =
    CardRecord::new_with_legacy_id(
        303,
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
pub(in crate::card::sets) static RIGHTEOUS_AVENGERS: CardRecord = CardRecord::new_with_legacy_id(
    1381,
    "Righteous Avengers",
    CardArt::new("d96b463e-9579-4e7b-87c2-342527b91e7c", "Heather Hudson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 3, 1)
        .with_ability(abilities::landwalk(BasicLandType::Plains)),
);

// LEG 35 — Seeker
pub(in crate::card::sets) static SEEKER: CardRecord = CardRecord::new_with_legacy_id(
    405,
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
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
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
pub(in crate::card::sets) static SHIELD_WALL: CardRecord = CardRecord::new_with_legacy_id(
    406,
    "Shield Wall",
    CardArt::new("a5032bf0-f9c0-4ef0-8ec2-fe7ccea9bdf3", "Douglas Shuler"),
    CardSet::Legends,
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

static SPIRITUAL_SANCTUARY_PLAINS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        &[ZoneKind::Battlefield],
        PlayerRelation::EventPlayer,
    ),
    comparison: ComparisonDef::Greater,
    amount: 0,
};

// LEG 37 — Spirit Link
pub(in crate::card::sets) static SPIRIT_LINK: CardRecord = CardRecord::new_with_legacy_id(
    1479,
    "Spirit Link",
    CardArt::new("5e2d35f8-3cf6-4843-9030-0e9a885d836c", "Kaja Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
pub(in crate::card::sets) static SPIRITUAL_SANCTUARY: CardRecord = CardRecord::new_with_legacy_id(
    407,
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
pub(in crate::card::sets) static THUNDER_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    101,
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
pub(in crate::card::sets) static TUNDRA_WOLVES: CardRecord = CardRecord::new_with_legacy_id(
    408,
    "Tundra Wolves",
    CardArt::new("8f649cb5-e19c-453f-b062-4fd452d92257", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{W}"), &["Wolf"], 1, 1)
        .with_ability(abilities::first_strike()),
);

/// The shuffle is the caster's call and comes after the look, which is the
/// point of the card: you decide whether to disturb what you just saw.
static VISIONS_SHUFFLE: EffectDef = EffectDef::May {
    player: EffectRecipientDef::Controller,
    effect: &EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
};

static VISIONS_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(5),
    object: None,
    minimum: 0,
    maximum: 0,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: false,
    then: Some(&VISIONS_SHUFFLE),
    selected_face_down: None,
};

static VISIONS_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// LEG 41 — Visions
pub(in crate::card::sets) static VISIONS: CardRecord = CardRecord::new_with_legacy_id(
    1726,
    "Visions",
    CardArt::new("54830c65-b4dd-406a-8dab-dd283424d0d2", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Look at the top five cards of target player's library. You may then have that player \
         shuffle that library.",
        &VISIONS_TARGET,
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            looker: EffectRecipientDef::Controller,
            selection: &VISIONS_LOOK,
        },
    )),
);

// LEG 42 — Wall of Caltrops
// Audit: blocked — Needs an intervening-if that counts the other creatures blocking the same attacker by subtype for “if at least one other Wall creature is blocking that creature and no non-Wall creatures are blocking that creature”. Granting banding is implemented.

// LEG 43 — Wall of Light
pub(in crate::card::sets) static WALL_OF_LIGHT: CardRecord = CardRecord::new_with_legacy_id(
    409,
    "Wall of Light",
    CardArt::new("f5758e82-f901-42b7-b705-0e68ca7ba59e", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Wall"], 1, 5).with_abilities(&[
        abilities::defender(),
        abilities::protection_from(ManaColor::Black),
    ]),
);

// LEG 44 — Acid Rain
pub(in crate::card::sets) static ACID_RAIN: CardRecord = CardRecord::new_with_legacy_id(
    410,
    "Acid Rain",
    CardArt::new("ba93c50a-2440-4e92-9cba-d97e20b1d29c", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Destroy all Forests.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )),
);

// LEG 45 — Anti-Magic Aura
// Audit: blocked — Needs the card's exact Aura targeting/attachment restriction rather than the broader existing cannot-be-enchanted effect for “Enchanted creature can't be the target of spells and can't be enchanted by other Auras”.

// LEG 46 — Azure Drake
pub(in crate::card::sets) static AZURE_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    411,
    "Azure Drake",
    CardArt::new("fb5f13a2-0896-4230-8957-6ad1cb2b895b", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Drake"], 2, 4)
        .with_ability(abilities::flying()),
);

// LEG 47 — Backfire
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Whenever enchanted creature deals damage to you, this Aura deals that much damage to that creature's controller”.

// LEG 48 — Boomerang
pub(in crate::card::sets) static BOOMERANG: CardRecord = CardRecord::new_with_legacy_id(
    412,
    "Boomerang",
    CardArt::new("b8286edd-644b-4135-8dca-af97f3920de3", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target permanent to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
        },
    )),
);

// LEG 49 — Brine Hag
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “When this creature dies, change the base power and toughness of all creatures that dealt damage to it this turn to 0/2”.

// LEG 50 — Devouring Deep
pub(in crate::card::sets) static DEVOURING_DEEP: CardRecord = CardRecord::new_with_legacy_id(
    1382,
    "Devouring Deep",
    CardArt::new("0855a5a8-8c40-4396-9ad1-8fa0fc6a0c59", "Liz Danforth"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Fish"], 1, 2)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// LEG 51 — Dream Coat
// Audit: blocked — Needs a per-object, per-turn activation quota for “{0}: Enchanted creature becomes the color or colors of your choice. Activate only once each turn”.

/// The declined branch: the Spawn goes and takes six with it. Reached when
/// the controller says no *and* when there is no Island to say yes with.
static ELDER_SPAWN_TOLL: [EffectDef; 2] = [
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(6),
    },
];

static ELDER_SPAWN_TOLL_SEQUENCE: EffectDef = EffectDef::Sequence(&ELDER_SPAWN_TOLL);

static ELDER_SPAWN_RED_BLOCKERS: AppliedEffectDef = AppliedEffectDef::Rule(
    AppliedRuleDef::CannotBeBlockedBy(ObjectPredicateDef::Color(ManaColor::Red)),
);

// LEG 52 — Elder Spawn
pub(in crate::card::sets) static ELDER_SPAWN: CardRecord = CardRecord::new_with_legacy_id(
    1965,
    "Elder Spawn",
    CardArt::new("99cc045e-01a8-4f14-a86d-0a67ec35d6b7", "Jesper Myrfors"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{U}{U}{U}"), &["Spawn"], 6, 6).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, unless you sacrifice an Island, sacrifice this \
             creature and it deals 6 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                then: None,
                otherwise: Some(&ELDER_SPAWN_TOLL_SEQUENCE),
                amount: SacrificedAmountDef::Power,
                optional: true,
            },
        ),
        AbilityDef::static_ability(
            "This creature can't be blocked by red creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: ELDER_SPAWN_RED_BLOCKERS,
            },
        ),
    ]),
);

// LEG 53 — Enchantment Alteration
// Audit: blocked — Needs Aura reattachment targeting, enchant-legality validation, and attachment movement for “Attach target Aura attached to a creature or land to another permanent of that type”.

/// "Untapped creature you control", so the tap always lands and the mana
/// always follows: an already-tapped creature is not a legal target.
static ENERGY_TAP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
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
)];

static ENERGY_TAP_EFFECT: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::AddManaEqualTo {
        color: ManaColor::Colorless,
        amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
    },
];

// LEG 54 — Energy Tap
pub(in crate::card::sets) static ENERGY_TAP: CardRecord = CardRecord::new_with_legacy_id(
    1821,
    "Energy Tap",
    CardArt::new("37e69940-bdc8-48ff-a296-540343910adf", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target untapped creature you control. If you do, add an amount of {C} equal to \
         that creature's mana value.",
        &ENERGY_TAP_TARGET,
        EffectDef::Sequence(&ENERGY_TAP_EFFECT),
    )),
);

// LEG 55 — Field of Dreams
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Players play with the top card of their libraries revealed”.

// LEG 56 — Flash Counter
pub(in crate::card::sets) static FLASH_COUNTER: CardRecord = CardRecord::new_with_legacy_id(
    413,
    "Flash Counter",
    CardArt::new("3c3cd450-f1cd-416b-9271-37d95815c089", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Instant)),
    )),
);

// LEG 57 — Flash Flood
pub(in crate::card::sets) static FLASH_FLOOD: CardRecord = CardRecord::new_with_legacy_id(
    414,
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
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
            ),
        ],
    )),
);

// LEG 58 — Force Spike
pub(in crate::card::sets) static FORCE_SPIKE: CardRecord = CardRecord::new_with_legacy_id(
    415,
    "Force Spike",
    CardArt::new("70e64028-ae96-4950-aa6c-9d347409fad3", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        abilities::counter_target_unless_paid(ValueDef::Constant(1)),
    )),
);

/// The mana arrives later, so the amount is read from what the countered
/// spell was rather than from anything still on the stack.
static MANA_DRAIN_EFFECT: [EffectDef; 2] = [
    EffectDef::Counter {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Graveyard,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of your next precombat main phase, add colorless mana equal to that spell's mana value.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::PrecombatMain,
            player: PlayerRelation::You,
        },
        EffectDef::AddManaEqualTo {
            color: ManaColor::Colorless,
            amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
        },
    ))),
];

// LEG 59 — Gaseous Form
pub(in crate::card::sets) static GASEOUS_FORM: CardRecord = CardRecord::new_with_legacy_id(
    1445,
    "Gaseous Form",
    CardArt::new("d0266dd4-31da-480b-9a44-4e217f748f06", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
pub(in crate::card::sets) static MANA_DRAIN: CardRecord = CardRecord::new_with_legacy_id(
    80,
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

static PART_WATER_ISLANDWALK: AbilityDef = abilities::landwalk(BasicLandType::Island);

/// "X target creatures": the count is the X that was paid, and X is doubled
/// in the cost, so each creature reached costs two mana rather than one.
static PART_WATER_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_chosen_x(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

// LEG 66 — Part Water
pub(in crate::card::sets) static PART_WATER: CardRecord = CardRecord::new_with_legacy_id(
    1836,
    "Part Water",
    CardArt::new("4b659475-c8b7-493d-af63-04f34d8cc3b1", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{X}{X}{U}")).with_ability(AbilityDef::spell_with_targets(
        "X target creatures gain islandwalk until end of turn.",
        &PART_WATER_TARGETS,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&PART_WATER_ISLANDWALK),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 67 — Psionic Entity
pub(in crate::card::sets) static PSIONIC_ENTITY: CardRecord = CardRecord::new_with_legacy_id(
    416,
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
pub(in crate::card::sets) static RECALL: CardRecord = CardRecord::new_with_legacy_id(
    89,
    "Recall",
    CardArt::new("33296718-0625-4422-a65c-b21cf99c52ec", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{X}{X}{U}"))
    .with_abilities(&[AbilityDef::custom_full(
        "Discard X then return a card from your graveyard to your hand for each card discarded this way. Exile Recall.",
        CardBehavior::Recall,
        "The card-local resolver discards on resolution and then returns that many so a countered Recall costs nothing and the discarded cards are themselves returnable.",
    )]),
);

// LEG 71 — Relic Bind
// Audit: blocked — Needs modal triggered abilities: modes are chosen only while casting a spell, so a trigger has no mode selection to make as it resolves. The trigger event and both modes are available.

// LEG 72 — Remove Soul
pub(in crate::card::sets) static REMOVE_SOUL: CardRecord = CardRecord::new_with_legacy_id(
    417,
    "Remove Soul",
    CardArt::new("63de147c-2e62-41b9-8ada-93406387f08b", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target creature spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Creature)),
    )),
);

// LEG 73 — Reset
pub(in crate::card::sets) static RESET: CardRecord = CardRecord::new_with_legacy_id(
    1824,
    "Reset",
    CardArt::new("1c829d83-d5b8-4be7-80f7-55b42f52b309", "Nicola Leonard"),
    CardSet::Legends,
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
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “All damage that would be dealt this turn by target sorcery spell is dealt to that spell's controller instead”.

// LEG 75 — Sea Kings' Blessing
pub(in crate::card::sets) static SEA_KINGS_BLESSING: CardRecord = CardRecord::new_with_legacy_id(
    1767,
    "Sea Kings' Blessing",
    CardArt::new(
        "975d26dd-d915-42c0-94b3-083fb2a2ce3f",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
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
pub(in crate::card::sets) static SEGOVIAN_LEVIATHAN: CardRecord = CardRecord::new_with_legacy_id(
    1383,
    "Segovian Leviathan",
    CardArt::new("e5a814f1-7f8d-4c2c-b706-ee0ed5892f7b", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Leviathan"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// LEG 77 — Silhouette
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Choose target creature. If a spell or ability that targets that creature would cause a source to deal damage to that creature this turn, prevent that damage”.

static SPECTRAL_SHROUD: AbilityDef = abilities::shroud();

// LEG 78 — Spectral Cloak
pub(in crate::card::sets) static SPECTRAL_CLOAK: CardRecord = CardRecord::new_with_legacy_id(
    1659,
    "Spectral Cloak",
    CardArt::new("7524fd0d-a675-41d6-bc99-bd3ba336893b", "Rob Alexander"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
                    effect: AppliedEffectDef::add_ability(&SPECTRAL_SHROUD),
                },
            ),
        ]),
);

// LEG 79 — Telekinesis
pub(in crate::card::sets) static TELEKINESIS: CardRecord = CardRecord::new_with_legacy_id(
    1567,
    "Telekinesis",
    CardArt::new("d5aa920e-b93f-41c2-b505-a9350353be8b", "Daniel Gelon"),
    CardSet::Legends,
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
pub(in crate::card::sets) static TELEPORT: CardRecord = CardRecord::new_with_legacy_id(
    1823,
    "Teleport",
    CardArt::new("18f86e13-f942-423e-b175-930d768cb811", "Douglas Shuler"),
    CardSet::Legends,
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
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlocked),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEG 81 — Time Elemental
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “{2}{U}{U}, {T}: Return target permanent that isn't enchanted to its owner's hand”.

// LEG 82 — Undertow
pub(in crate::card::sets) static UNDERTOW: CardRecord = CardRecord::new_with_legacy_id(
    1395,
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

/// The X is read off the permanent rather than the resolving object: this is
/// an enters trigger, a separate object from the spell that chose it.
static VENARIAN_GOLD_SLEEP: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::AttachedPermanent,
    },
    EffectDef::AddCounters {
        object: EffectRecipientDef::AttachedPermanent,
        kind: CounterKind::Sleep,
        amount: ValueDef::SourceCastX,
    },
];

/// The counters are on the creature rather than on the Aura, so the condition
/// asks what the Aura is attached to rather than what it carries itself.
static VENARIAN_GOLD_ASLEEP: TriggerConditionDef = TriggerConditionDef::AttachedPermanentMatches {
    object: ObjectPredicateDef::HasCounter(CounterKind::Sleep),
};

static VENARIAN_GOLD_HOLDS_IT_DOWN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
};

// LEG 83 — Venarian Gold
pub(in crate::card::sets) static VENARIAN_GOLD: CardRecord = CardRecord::new_with_legacy_id(
    1820,
    "Venarian Gold",
    CardArt::new("11fb92c0-bb1e-463a-a6b6-887a5d0cb873", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{X}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::triggered(
                "When this Aura enters, tap enchanted creature and put X sleep counters on it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Sequence(&VENARIAN_GOLD_SLEEP),
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step if it \
                 has a sleep counter on it.",
                EffectDef::IfCondition {
                    condition: &VENARIAN_GOLD_ASLEEP,
                    then: &VENARIAN_GOLD_HOLDS_IT_DOWN,
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
                    kind: CounterKind::Sleep,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEG 84 — Wall of Vapor
pub(in crate::card::sets) static WALL_OF_VAPOR: CardRecord = CardRecord::new_with_legacy_id(
    1649,
    "Wall of Vapor",
    CardArt::new("6a6c0a27-d410-4ded-a842-70e1656ea21e", "Richard Thomas"),
    CardSet::Legends,
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

/// One activation, two things applied to the same creature for the same
/// duration, so they ride together rather than as two effects in sequence.
static WALL_OF_WONDER_CHARGE: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(4), ValueDef::Constant(-4)),
    AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
];

// LEG 85 — Wall of Wonder
pub(in crate::card::sets) static WALL_OF_WONDER: CardRecord = CardRecord::new_with_legacy_id(
    1732,
    "Wall of Wonder",
    CardArt::new("cff59416-f4a0-4199-a199-ae3a0e5216df", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Wall"], 1, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{2}{U}{U}: This creature gets +4/-4 until end of turn and can attack this turn as \
             though it didn't have defender.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{U}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&WALL_OF_WONDER_CHARGE),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 86 — Zephyr Falcon
pub(in crate::card::sets) static ZEPHYR_FALCON: CardRecord = CardRecord::new_with_legacy_id(
    418,
    "Zephyr Falcon",
    CardArt::new("25a173fd-e10c-45f8-a6e5-ad7a747a8050", "Heather Hudson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// LEG 87 — Abomination
pub(in crate::card::sets) static ABOMINATION: CardRecord = CardRecord::new_with_legacy_id(
    1575,
    "Abomination",
    CardArt::new("a69e2cf8-5ecb-485a-92a2-b4e0a7959f1f", "Mark Tedin"),
    CardSet::Legends,
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
            EffectDef::DestroyAtEndOfCombat {
                object: EffectRecipientDef::TriggeringObject,
            },
        ),
    ),
);

// LEG 88 — All Hallow's Eve
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of your upkeep, if this card is exiled with a scream counter on it, remove a scream counter from it. If there are no more scream counters on it, put it into your…”.

// LEG 89 — Blight
pub(in crate::card::sets) static BLIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1653,
    "Blight",
    CardArt::new("9ca19b39-4201-463c-bd40-fbffa31c9eda", "Pete Venters"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant land", &abilities::ENCHANT_LAND_TARGET),
            AbilityDef::triggered(
                "When enchanted land becomes tapped, destroy it.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    can_regenerate: true,
                },
            ),
        ]),
);

// LEG 90 — Carrion Ants
pub(in crate::card::sets) static CARRION_ANTS: CardRecord = CardRecord::new_with_legacy_id(
    419,
    "Carrion Ants",
    CardArt::new("cbc0b009-3951-4aa3-985a-97139882da7e", "Richard Thomas"),
    CardSet::Legends,
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
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “If a player would draw a card except the first one they draw in each of their draw steps, that player discards a card instead. If the player discards a card this way, they draw a card.…”.

// LEG 92 — Cosmic Horror
// Audit: blocked — Needs an unless-paid destruction sequence that deals damage only when this ability actually destroys its source.

// LEG 93 — Cyclopean Mummy
// Audit: partial — Its death trigger cannot address the new card object created in the graveyard.
pub(in crate::card::sets) static CYCLOPEAN_MUMMY: CardRecord = CardRecord::new_with_legacy_id(
    420,
    "Cyclopean Mummy",
    CardArt::new(
        "479ccc50-2d72-4adc-901e-fbd4eef2cf92",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 1).with_ability(
        AbilityDef::triggered(
            "When this creature dies, exile it.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The trigger remembers the old battlefield incarnation but cannot address the new card object in the graveyard.",
        )),
    ),
);

// LEG 94 — Darkness
pub(in crate::card::sets) static DARKNESS: CardRecord = CardRecord::new_with_legacy_id(
    1408,
    "Darkness",
    CardArt::new("53b04dab-45b7-418b-a0f0-bcf35145fc53", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 95 — Demonic Torment
pub(in crate::card::sets) static DEMONIC_TORMENT: CardRecord = CardRecord::new_with_legacy_id(
    1671,
    "Demonic Torment",
    CardArt::new("d3ec14bc-95e9-47ce-b51e-d5eac9b345fe", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
pub(in crate::card::sets) static EVIL_EYE_OF_ORMS_BY_GORE: CardRecord =
    CardRecord::new_with_legacy_id(
        1578,
        "Evil Eye of Orms-by-Gore",
        CardArt::new("b060f747-f65c-4ee0-923a-76298cb51a03", "Jesper Myrfors"),
        CardSet::Legends,
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
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
                    )),
                },
            ),
        ]),
    );

// LEG 97 — Fallen Angel
pub(in crate::card::sets) static FALLEN_ANGEL: CardRecord = CardRecord::new_with_legacy_id(
    421,
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
pub(in crate::card::sets) static GHOSTS_OF_THE_DAMNED: CardRecord = CardRecord::new_with_legacy_id(
    422,
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
// Audit: blocked — Needs a delayed upkeep trigger that makes a basic-land-type choice on resolution and grants the matching walk; granting a named walk is available.

// LEG 100 — Glyph of Doom
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Choose target Wall creature. At this turn's next end of combat, destroy all creatures that were blocked by that creature this turn”.

// LEG 101 — Greed
pub(in crate::card::sets) static GREED: CardRecord = CardRecord::new_with_legacy_id(
    423,
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
pub(in crate::card::sets) static HEADLESS_HORSEMAN: CardRecord = CardRecord::new_with_legacy_id(
    424,
    "Headless Horseman",
    CardArt::new("d1aa37c8-98fa-4984-b09b-cf65ad84e97b", "Quinton Hoover"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Knight"], 2, 2),
);

// LEG 103 — Hell Swarm
pub(in crate::card::sets) static HELL_SWARM: CardRecord = CardRecord::new_with_legacy_id(
    425,
    "Hell Swarm",
    CardArt::new("64164d1b-75f4-456e-a717-90ce554dc16c", "Christopher Rush"),
    CardSet::Legends,
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
pub(in crate::card::sets) static HELLS_CARETAKER: CardRecord = CardRecord::new_with_legacy_id(
    1465,
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
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// LEG 105 — Hellfire
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Destroy all nonblack creatures. Hellfire deals X plus 3 damage to you, where X is the number of creatures that died this way”.

// LEG 106 — Horror of Horrors
pub(in crate::card::sets) static HORROR_OF_HORRORS: CardRecord = CardRecord::new_with_legacy_id(
    1429,
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
pub(in crate::card::sets) static INFERNAL_MEDUSA: CardRecord = CardRecord::new_with_legacy_id(
    1733,
    "Infernal Medusa",
    CardArt::new("2d0a7eb3-bb06-4b02-8841-157eefefaba1", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Gorgon"], 2, 4).with_abilities(&[
        // The two halves are printed separately and are not the same clause:
        // blocking kills anything, while being blocked spares Walls.
        AbilityDef::triggered(
            "Whenever this creature blocks a creature, destroy that creature at end of combat.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::DestroyAtEndOfCombat {
                object: EffectRecipientDef::TriggeringObject,
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a non-Wall creature, destroy that \
             creature at end of combat.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
            },
            EffectDef::DestroyAtEndOfCombat {
                object: EffectRecipientDef::TriggeringObject,
            },
        ),
    ]),
);

// LEG 109 — Jovial Evil
// Audit: blocked — Needs a dynamic count of white creatures controlled by the targeted opponent and multiplication for the damage value.

// LEG 110 — Lesser Werewolf
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{B}: If this creature's power is 1 or more, it gets -1/-0 until end of turn and put a -0/-1 counter on target creature blocking or blocked by this creature. Activate only during the…”.

// LEG 111 — Lost Soul
pub(in crate::card::sets) static LOST_SOUL: CardRecord = CardRecord::new_with_legacy_id(
    1384,
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
pub(in crate::card::sets) static NETHER_VOID: CardRecord = CardRecord::new_with_legacy_id(
    426,
    "Nether Void",
    CardArt::new("2e72f8cb-5bc3-4711-9b7c-a6eea9a0beaf", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::triggered(
            "Whenever a player casts a spell, counter it unless that player pays {3}.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::Any),
            abilities::counter_triggering_spell_unless_paid(ValueDef::Constant(3)),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The counter trigger is executable, but the world-rule state-based action is not implemented.",
        ))),
);

// LEG 114 — Pit Scorpion
pub(in crate::card::sets) static PIT_SCORPION: CardRecord = CardRecord::new_with_legacy_id(
    1559,
    "Pit Scorpion",
    CardArt::new("cc564f84-0d6e-4e09-a58d-a694d918cf12", "Scott Kirschner"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Scorpion"], 1, 1).with_ability(
        abilities::poisonous_damage(
            1,
            "Whenever this creature deals damage to a player, that player gets a poison \
             counter. (A player with ten or more poison counters loses the game.)",
        ),
    ),
);

// LEG 115 — Quagmire
pub(in crate::card::sets) static QUAGMIRE: CardRecord = CardRecord::new_with_legacy_id(
    1396,
    "Quagmire",
    CardArt::new("94e2aa9e-af6a-41c6-99a8-ca9335730ddb", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with swampwalk can be blocked as though they didn't have swampwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Swamp),
    )]),
);

static ATTACKING_CREATURE_TARGET: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::Attacking,
    )];

// LEG 116 — Shimian Night Stalker
pub(in crate::card::sets) static SHIMIAN_NIGHT_STALKER: CardRecord = CardRecord::new_with_legacy_id(
    1691,
    "Shimian Night Stalker",
    CardArt::new("4e04a1b9-c561-4e34-86d9-129ea0346631", "Jesper Myrfors"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Nightstalker"], 4, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, {T}: All damage that would be dealt to you this turn by target attacking \
             creature is dealt to this creature instead.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &ATTACKING_CREATURE_TARGET,
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
pub(in crate::card::sets) static SPIRIT_SHACKLE: CardRecord = CardRecord::new_with_legacy_id(
    1654,
    "Spirit Shackle",
    CardArt::new(
        "a30bb266-5bd1-4998-ae94-56f0f3354167",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Syphon Soul deals 2 damage to each other player. You gain life equal to the damage dealt this way”.

// LEG 119 — Takklemaggot
// Audit: blocked — Needs duration-aware control-changing continuous effects for “When enchanted creature dies, that creature's controller chooses a creature that this card could enchant. If the player does, return this card to the battlefield under your control…”.

static THE_ABYSS_DESTROY_CHOICE: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    can_regenerate: false,
};

// LEG 120 — The Abyss
// Audit: partial — Its upkeep destruction is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static THE_ABYSS: CardRecord = CardRecord::new_with_legacy_id(
    127,
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
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
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
                then: &THE_ABYSS_DESTROY_CHOICE,
            }),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The upkeep destruction is executable, but the world-rule state-based action is not implemented.",
        ))]),
);

// LEG 121 — The Wretched
pub(in crate::card::sets) static THE_WRETCHED: CardRecord = CardRecord::new_with_legacy_id(
    1712,
    "The Wretched",
    CardArt::new("14c45416-a826-42e9-9967-8838158cf16d", "Christopher Rush"),
    CardSet::Legends,
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
pub(in crate::card::sets) static TOUCH_OF_DARKNESS: CardRecord = CardRecord::new_with_legacy_id(
    1768,
    "Touch of Darkness",
    CardArt::new("e14f0c39-770b-4ed5-a3ca-0cc31698e2bc", "Pete Venters"),
    CardSet::Legends,
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
pub(in crate::card::sets) static TRANSMUTATION: CardRecord = CardRecord::new_with_legacy_id(
    1992,
    "Transmutation",
    CardArt::new("329f7eb2-eadf-46ec-aed4-63152051f3c1", "Susan Van Camp"),
    CardSet::Legends,
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
// Audit: blocked — Needs an opponent-draw event trigger that deals damage to the exact player who drew.

// LEG 125 — Vampire Bats
pub(in crate::card::sets) static VAMPIRE_BATS: CardRecord = CardRecord::new_with_legacy_id(
    1660,
    "Vampire Bats",
    CardArt::new("6a6a6f50-7b86-461e-80a7-e35d0e7cf52f", "Anson Maddocks"),
    CardSet::Legends,
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
pub(in crate::card::sets) static WALKING_DEAD: CardRecord = CardRecord::new_with_legacy_id(
    1379,
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
pub(in crate::card::sets) static WALL_OF_PUTRID_FLESH: CardRecord = CardRecord::new_with_legacy_id(
    1650,
    "Wall of Putrid Flesh",
    CardArt::new("07a17b74-a9c9-419a-8369-9ab4fec213f2", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Wall"], 2, 4).with_abilities(&[
        abilities::defender(),
        abilities::protection_from(ManaColor::White),
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
// Audit: partial — Targeting restrictions cannot be conditioned on how narrow the targeting spell or ability is.
pub(in crate::card::sets) static WALL_OF_SHADOWS: CardRecord = CardRecord::new_with_legacy_id(
    1651,
    "Wall of Shadows",
    CardArt::new("eb351900-cffd-4d23-b82f-5fb12a4874d9", "Pete Venters"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Wall"], 0, 1).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by creatures it's blocking.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_damage_from(
                    ObjectPredicateDef::BlockedBySource,
                ),
            },
        ),
        AbilityDef::not_implemented(
            "This creature can't be the target of spells that can target only Walls or of abilities that can target only Walls.",
            "Targeting restrictions cannot be conditioned on how narrow the targeting spell or ability is.",
        ),
    ]),
);

// LEG 129 — Wall of Tombstones
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “At the beginning of your upkeep, change this creature's base toughness to 1 plus the number of creature cards in your graveyard”.

// LEG 130 — Active Volcano
pub(in crate::card::sets) static ACTIVE_VOLCANO: CardRecord = CardRecord::new_with_legacy_id(
    427,
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
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    controller: None,
                },
            ),
        ],
    )),
);

// LEG 131 — Aerathi Berserker
pub(in crate::card::sets) static AERATHI_BERSERKER: CardRecord = CardRecord::new_with_legacy_id(
    1388,
    "Aerathi Berserker",
    CardArt::new("06673800-22a7-4ee3-92fa-7c7cd4865d30", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}{R}{R}"), &["Human", "Berserker"], 2, 4).with_abilities(&[
        abilities::rampage(3, "Rampage 3 (Whenever this creature becomes blocked, it gets +3/+3 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 132 — Backdraft
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Choose a player who cast one or more sorcery spells this turn. Backdraft deals damage to that player equal to half the damage dealt by one of those sorcery spells this turn, rounded down”.

/// Nontoken, so a board of white tokens across the table leaves the Beasts at
/// their printed size.
static AN_OPPONENT_CONTROLS_A_NONTOKEN_WHITE_PERMANENT: TriggerConditionDef =
    TriggerConditionDef::ObjectCount {
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
    };

static BEASTS_OF_BOGARDAN_PUMP: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

// LEG 133 — Beasts of Bogardan
pub(in crate::card::sets) static BEASTS_OF_BOGARDAN: CardRecord = CardRecord::new_with_legacy_id(
    1915,
    "Beasts of Bogardan",
    CardArt::new("f885d776-2953-4ed4-b63f-91dc2b42783b", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Beast"], 3, 3).with_abilities(&[
        abilities::protection_from(ManaColor::Red),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as an opponent controls a nontoken white permanent.",
            EffectDef::IfCondition {
                condition: &AN_OPPONENT_CONTROLS_A_NONTOKEN_WHITE_PERMANENT,
                then: &BEASTS_OF_BOGARDAN_PUMP,
            },
        ),
    ]),
);

// LEG 134 — Blazing Effigy
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “When this creature dies, it deals X damage to target creature, where X is 3 plus the amount of damage dealt to this creature this turn by other sources named Blazing Effigy”.

// LEG 135 — Blood Lust
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “If target creature has toughness 5 or greater, it gets +4/-4 until end of turn. Otherwise, it gets +4/-X until end of turn, where X is its toughness minus 1”.

// LEG 136 — Caverns of Despair
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “No more than two creatures can attack each combat”.

// LEG 137 — Chain Lightning
pub(in crate::card::sets) static CHAIN_LIGHTNING: CardRecord = CardRecord::new_with_legacy_id(
    6,
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
pub(in crate::card::sets) static CREVASSE: CardRecord = CardRecord::new_with_legacy_id(
    1397,
    "Crevasse",
    CardArt::new("a432d6ae-a17f-484b-ad55-4b4b6674ba8d", "Rob Alexander"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with mountainwalk can be blocked as though they didn't have mountainwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Mountain),
    )]),
);

// LEG 139 — Crimson Kobolds
pub(in crate::card::sets) static CRIMSON_KOBOLDS: CardRecord = CardRecord::new_with_legacy_id(
    428,
    "Crimson Kobolds",
    CardArt::new("13696657-aeef-4add-9a3b-8137fce01fe3", "Anson Maddocks"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 140 — Crimson Manticore
pub(in crate::card::sets) static CRIMSON_MANTICORE: CardRecord = CardRecord::new_with_legacy_id(
    429,
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
pub(in crate::card::sets) static CROOKSHANK_KOBOLDS: CardRecord = CardRecord::new_with_legacy_id(
    430,
    "Crookshank Kobolds",
    CardArt::new("7af6b119-7db4-49dd-aaa4-044b8c133f13", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

/// Control before the removal, though the card prints it after. A later
/// instruction in the same resolution re-checks the target against "attacking
/// creature", and removing it from combat makes it stop matching. The outcome
/// is the same either way.
static DISHARMONY_EFFECT: [EffectDef; 3] = [
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
];

// LEG 142 — Disharmony
pub(in crate::card::sets) static DISHARMONY: CardRecord = CardRecord::new_with_legacy_id(
    1804,
    "Disharmony",
    CardArt::new("e09a6b9b-eb0e-475d-8558-08e347412790", "Bryon Wackwitz"),
    CardSet::Legends,
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
                EffectDef::Sequence(&DISHARMONY_EFFECT),
            ),
        ]),
);

// LEG 143 — Dwarven Song
pub(in crate::card::sets) static DWARVEN_SONG: CardRecord = CardRecord::new_with_legacy_id(
    1769,
    "Dwarven Song",
    CardArt::new("9e2ce8c8-c1c2-4297-aa72-f4cad0cb73d1", "Dan Frazier"),
    CardSet::Legends,
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
pub(in crate::card::sets) static ETERNAL_WARRIOR: CardRecord = CardRecord::new_with_legacy_id(
    431,
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
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
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
pub(in crate::card::sets) static FROST_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    1389,
    "Frost Giant",
    CardArt::new("6955d54f-7b37-4e43-8183-51677fb1ee11", "Daniel Gelon"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{R}{R}{R}"), &["Giant"], 4, 4).with_abilities(&[
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 149 — Giant Strength
pub(in crate::card::sets) static GIANT_STRENGTH: CardRecord = CardRecord::new_with_legacy_id(
    432,
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

/// Three durations in one card, and none of them is the same: the pump ends
/// with combat, the shield lasts the turn, and the destruction waits for the
/// next end step -- which is later than either.
static GLYPH_OF_DESTRUCTION_EFFECT: [EffectDef; 3] = [
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
        },
    ))),
];

static BLOCKING_WALL_YOU_CONTROL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::Subtype("Wall"),
            ObjectPredicateDef::Blocking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

// LEG 150 — Glyph of Destruction
pub(in crate::card::sets) static GLYPH_OF_DESTRUCTION: CardRecord = CardRecord::new_with_legacy_id(
    1738,
    "Glyph of Destruction",
    CardArt::new("21e5b4d6-6288-4da6-970e-13c56b384a59", "Susan Van Camp"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target blocking Wall you control gets +10/+0 until end of combat. Prevent all damage \
         that would be dealt to it this turn. Destroy it at the beginning of the next end step.",
        &BLOCKING_WALL_YOU_CONTROL,
        EffectDef::Sequence(&GLYPH_OF_DESTRUCTION_EFFECT),
    )),
);

// LEG 151 — Gravity Sphere
// Audit: partial — Its flying-removal effect is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static GRAVITY_SPHERE: CardRecord = CardRecord::new_with_legacy_id(
    433,
    "Gravity Sphere",
    CardArt::new("a2749332-e99a-4a0c-b3a3-5578b552fa11", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures lose flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Flying removal is executable, but the world-rule state-based action is not implemented.",
        ))),
);

// LEG 152 — Hyperion Blacksmith
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{T}: You may tap or untap target artifact an opponent controls”.

// LEG 153 — Immolation
pub(in crate::card::sets) static IMMOLATION: CardRecord = CardRecord::new_with_legacy_id(
    434,
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
pub(in crate::card::sets) static KOBOLD_DRILL_SERGEANT: CardRecord = CardRecord::new_with_legacy_id(
    435,
    "Kobold Drill Sergeant",
    CardArt::new("741b14f8-625d-41be-a734-0efe042a6ee8", "Julie Baroh"),
    CardSet::Legends,
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
pub(in crate::card::sets) static KOBOLD_OVERLORD: CardRecord = CardRecord::new_with_legacy_id(
    436,
    "Kobold Overlord",
    CardArt::new("490eeedb-9c03-4dc7-81fd-ae54a7932e4d", "Julie Baroh"),
    CardSet::Legends,
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
pub(in crate::card::sets) static KOBOLD_TASKMASTER: CardRecord = CardRecord::new_with_legacy_id(
    437,
    "Kobold Taskmaster",
    CardArt::new(
        "1b9c63eb-8d4e-4d8b-8637-308459ef036b",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
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
pub(in crate::card::sets) static KOBOLDS_OF_KHER_KEEP: CardRecord = CardRecord::new_with_legacy_id(
    438,
    "Kobolds of Kher Keep",
    CardArt::new("df0320d9-7c2a-456a-9159-1b4fae67bfb5", "Julie Baroh"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{0}"), &["Kobold"], 0, 1).printed_colors(&[ManaColor::Red]),
);

// LEG 158 — Land's Edge
// Audit: blocked — Needs an ability any player may activate and a conditional keyed to the type of the card the discard cost actually took.

// LEG 159 — Mountain Yeti
pub(in crate::card::sets) static MOUNTAIN_YETI: CardRecord = CardRecord::new_with_legacy_id(
    439,
    "Mountain Yeti",
    CardArt::new("09242f08-3bfc-4082-b32f-703c7fed62a0", "Dan Frazier"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Yeti"], 3, 3).with_abilities(&[
        abilities::mountainwalk(),
        abilities::protection_from(ManaColor::White),
    ]),
);

/// X is read after the counter goes on, so the toll is the size the Ooze has
/// just grown to rather than the size it was.
static PRIMORDIAL_OOZE_X: ValueDef = ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne);

static PRIMORDIAL_OOZE_TOLL: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::Source,
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Controller,
        amount: PRIMORDIAL_OOZE_X,
    },
];

static PRIMORDIAL_OOZE_TOLL_SEQUENCE: EffectDef = EffectDef::Sequence(&PRIMORDIAL_OOZE_TOLL);

static PRIMORDIAL_OOZE_UPKEEP: [EffectDef; 2] = [
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
        otherwise: Some(&PRIMORDIAL_OOZE_TOLL_SEQUENCE),
        visibility: ChoiceVisibilityDef::Private,
    }),
];

// LEG 160 — Primordial Ooze
pub(in crate::card::sets) static PRIMORDIAL_OOZE: CardRecord = CardRecord::new_with_legacy_id(
    1814,
    "Primordial Ooze",
    CardArt::new("a46e47e1-8639-48f7-94c4-5f9e9666839a", "Sandra Everingham"),
    CardSet::Legends,
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
            EffectDef::Sequence(&PRIMORDIAL_OOZE_UPKEEP),
        ),
    ]),
);

// LEG 161 — Pyrotechnics
pub(in crate::card::sets) static PYROTECHNICS: CardRecord = CardRecord::new_with_legacy_id(
    440,
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
            another: false,
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
pub(in crate::card::sets) static RAGING_BULL: CardRecord = CardRecord::new_with_legacy_id(
    441,
    "Raging Bull",
    CardArt::new(
        "ec10a51c-d2c3-4d14-9a71-9e59155bf980",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ox"], 2, 2),
);

// LEG 164 — Spinal Villain
pub(in crate::card::sets) static SPINAL_VILLAIN: CardRecord = CardRecord::new_with_legacy_id(
    442,
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
pub(in crate::card::sets) static THE_BRUTE: CardRecord = CardRecord::new_with_legacy_id(
    1421,
    "The Brute",
    CardArt::new("f9ffb265-872f-47b3-974c-92bcbebd557e", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks a creature, that creature can't attack during its controller's next turn”.

// LEG 169 — Wall of Earth
pub(in crate::card::sets) static WALL_OF_EARTH: CardRecord = CardRecord::new_with_legacy_id(
    443,
    "Wall of Earth",
    CardArt::new("c12e97c1-ca28-432a-8140-3f08bb4485a3", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Wall"], 0, 6)
        .with_ability(abilities::defender()),
);

// LEG 170 — Wall of Heat
pub(in crate::card::sets) static WALL_OF_HEAT: CardRecord = CardRecord::new_with_legacy_id(
    444,
    "Wall of Heat",
    CardArt::new("a38059a8-be69-4cc1-969b-951c610f2f11", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wall"], 2, 6)
        .with_ability(abilities::defender()),
);

// LEG 171 — Wall of Opposition
pub(in crate::card::sets) static WALL_OF_OPPOSITION: CardRecord = CardRecord::new_with_legacy_id(
    445,
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
// Audit: blocked — Needs a hidden-zone decision and continuation for “Each player shuffles the cards from their hand into their library, then draws that many cards”.

// LEG 173 — Aisling Leprechaun
pub(in crate::card::sets) static AISLING_LEPRECHAUN: CardRecord = CardRecord::new_with_legacy_id(
    1576,
    "Aisling Leprechaun",
    CardArt::new("640a161d-ad7b-4e5b-8f2d-d3753cb9daa3", "Quinton Hoover"),
    CardSet::Legends,
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
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Creatures can't attack a player unless that player cast a spell or put a nontoken permanent onto the battlefield during their last turn”.

// LEG 175 — Avoid Fate
// Audit: blocked — Needs a spell-on-stack target predicate that expresses the printed instant/Aura restriction for “Counter target instant or Aura spell that targets a permanent you control”.

// LEG 176 — Barbary Apes
pub(in crate::card::sets) static BARBARY_APES: CardRecord = CardRecord::new_with_legacy_id(
    446,
    "Barbary Apes",
    CardArt::new("df25ffdd-995d-46ae-856b-f6368f9438ed", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Ape"], 2, 2),
);

// LEG 177 — Cat Warriors
pub(in crate::card::sets) static CAT_WARRIORS: CardRecord = CardRecord::new_with_legacy_id(
    447,
    "Cat Warriors",
    CardArt::new("d2187a64-2823-4f58-ad35-70f8913db2dc", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Cat", "Warrior"], 2, 2)
        .with_ability(abilities::forestwalk()),
);

/// "Enchant creature you control."
static COCOON_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

static COCOON_WRAPPED: [EffectDef; 2] = [
    EffectDef::Tap {
        object: EffectRecipientDef::AttachedPermanent,
    },
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Pupa,
        amount: ValueDef::Constant(3),
    },
];

static COCOON_STILL_WRAPPED: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Pupa,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static COCOON_EMPTY: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Pupa,
    comparison: ComparisonDef::Equal,
    amount: 0,
};

static COCOON_HOLDS_IT_DOWN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
};

static COCOON_FLYING: AbilityDef = abilities::flying();

/// The reward is handed out before the Aura goes, though the card prints it
/// after: once the Aura leaves there is nothing attached to give it to.
static COCOON_HATCH: [EffectDef; 3] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::AttachedPermanent,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::AttachedPermanent,
        effect: AppliedEffectDef::add_ability(&COCOON_FLYING),
        duration: ResolvedEffectDurationDef::Permanent,
    },
    EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
];

static COCOON_HATCH_SEQUENCE: EffectDef = EffectDef::Sequence(&COCOON_HATCH);

static COCOON_SHED: EffectDef = EffectDef::RemoveCounters {
    object: EffectRecipientDef::Source,
    kind: CounterKind::Pupa,
    amount: ValueDef::Constant(1),
};

/// Two complementary conditions rather than a branch: one takes a counter off
/// while any remain, the other opens the Cocoon once none do.
static COCOON_UPKEEP: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &COCOON_STILL_WRAPPED,
        then: &COCOON_SHED,
    },
    EffectDef::IfCondition {
        condition: &COCOON_EMPTY,
        then: &COCOON_HATCH_SEQUENCE,
    },
];

// LEG 178 — Cocoon
pub(in crate::card::sets) static COCOON: CardRecord = CardRecord::new_with_legacy_id(
    1819,
    "Cocoon",
    CardArt::new("a82c87b1-de37-4423-a1a4-533a1d8108b2", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature you control", &COCOON_TARGET),
            AbilityDef::triggered(
                "When this Aura enters, tap enchanted creature and put three pupa counters \
                 on this Aura.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Sequence(&COCOON_WRAPPED),
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during your untap step if this Aura has a \
                 pupa counter on it.",
                EffectDef::IfCondition {
                    condition: &COCOON_STILL_WRAPPED,
                    then: &COCOON_HOLDS_IT_DOWN,
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
                EffectDef::Sequence(&COCOON_UPKEEP),
            ),
        ]),
);

// LEG 179 — Concordant Crossroads
// Audit: partial — Its global haste effect is executable, but the world-rule state-based action is not implemented.
pub(in crate::card::sets) static CONCORDANT_CROSSROADS: CardRecord = CardRecord::new_with_legacy_id(
    448,
    "Concordant Crossroads",
    CardArt::new("3bdcfae4-86c9-4d8a-bcfe-f0a928ec29db", "Amy Weber"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Haste granting is executable, but the world-rule state-based action is not implemented.",
        ))),
);

// LEG 180 — Craw Giant
pub(in crate::card::sets) static CRAW_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    1390,
    "Craw Giant",
    CardArt::new("707dadf0-735f-445d-9240-e49660913314", "Christopher Rush"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}{G}"), &["Giant"], 6, 4).with_abilities(&[
        abilities::trample(),
        abilities::rampage(2, "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 181 — Deadfall
pub(in crate::card::sets) static DEADFALL: CardRecord = CardRecord::new_with_legacy_id(
    1398,
    "Deadfall",
    CardArt::new("0d78f0fc-3ab2-46ee-b5a9-55ae97d08c1a", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures with forestwalk can be blocked as though they didn't have forestwalk.",
        EffectDef::LandwalkCanBeBlocked(BasicLandType::Forest),
    )]),
);

// LEG 182 — Durkwood Boars
pub(in crate::card::sets) static DURKWOOD_BOARS: CardRecord = CardRecord::new_with_legacy_id(
    449,
    "Durkwood Boars",
    CardArt::new("8d41f08b-68fb-45f2-bdc9-488baedc7d6f", "Mike Kimble"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Boar"], 4, 4),
);

// LEG 183 — Elven Riders
pub(in crate::card::sets) static ELVEN_RIDERS: CardRecord = CardRecord::new_with_legacy_id(
    450,
    "Elven Riders",
    CardArt::new("ad1d349b-b5ab-4b2b-9b39-f8d8f6374aa5", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elf"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by Walls and/or creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
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
pub(in crate::card::sets) static EMERALD_DRAGONFLY: CardRecord = CardRecord::new_with_legacy_id(
    451,
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
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 185 — Eureka
// Audit: blocked — Needs a hidden-zone decision and continuation for “Starting with you, each player may put a permanent card from their hand onto the battlefield. Repeat this process until no one puts a card onto the battlefield”.

// LEG 186 — Fire Sprites
pub(in crate::card::sets) static FIRE_SPRITES: CardRecord = CardRecord::new_with_legacy_id(
    1637,
    "Fire Sprites",
    CardArt::new("d26fa79a-ede8-4c80-98d5-f49696f8104d", "Julie Baroh"),
    CardSet::Legends,
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

static FLORAL_SPUZZEM_STRIKE: [EffectDef; 2] = [
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

static FLORAL_SPUZZEM_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Artifact),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

// LEG 187 — Floral Spuzzem
pub(in crate::card::sets) static FLORAL_SPUZZEM: CardRecord = CardRecord::new_with_legacy_id(
    1721,
    "Floral Spuzzem",
    CardArt::new("1fb31eab-7fa2-4948-bd56-7145f40b5686", "Rob Alexander"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elemental"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may destroy target artifact \
             defending player controls. If you do, this creature assigns no combat damage this \
             turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &FLORAL_SPUZZEM_TARGET,
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&FLORAL_SPUZZEM_STRIKE),
            },
        ),
    ),
);

/// The condition lives on the recipient, so the prohibition is read live: it
/// applies on the turn after the Turtle swung and lifts on the one after
/// that, without anything being installed or expired.
static GIANT_TURTLE_RESTING: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Source,
        ObjectPredicateDef::AttackedDuringControllersLastTurn,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

// LEG 188 — Giant Turtle
pub(in crate::card::sets) static GIANT_TURTLE: CardRecord = CardRecord::new_with_legacy_id(
    1722,
    "Giant Turtle",
    CardArt::new("534aae13-58cb-4ffb-aa47-7420435282c9", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Turtle"], 2, 4).with_ability(
        AbilityDef::static_ability(
            "This creature can't attack if it attacked during your last turn.",
            EffectDef::StaticApply {
                recipient: GIANT_TURTLE_RESTING,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
    ),
);

// LEG 189 — Glyph of Reincarnation
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “Destroy all creatures that were blocked by target Wall this turn. They can't be regenerated. For each creature that died this way, put a creature card from the graveyard of the player…”.

// LEG 190 — Hornet Cobra
pub(in crate::card::sets) static HORNET_COBRA: CardRecord = CardRecord::new_with_legacy_id(
    452,
    "Hornet Cobra",
    CardArt::new("27180bad-9bbc-462b-8832-626dc403a3fd", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Snake"], 2, 1)
        .with_ability(abilities::first_strike()),
);

// LEG 191 — Ichneumon Druid
// Audit: blocked — Needs per-player, per-turn instant-cast counts in the triggering condition.

// LEG 192 — Killer Bees
pub(in crate::card::sets) static KILLER_BEES: CardRecord = CardRecord::new_with_legacy_id(
    453,
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

static EVERY_LAND_A_CREATURE: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
];

// LEG 193 — Living Plane
pub(in crate::card::sets) static LIVING_PLANE: CardRecord = CardRecord::new_with_legacy_id(
    1657,
    "Living Plane",
    CardArt::new("0341da27-3d77-4959-b7fa-5929b2cc7141", "Bryon Wackwitz"),
    CardSet::Legends,
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
                effect: AppliedEffectDef::Composite(&EVERY_LAND_A_CREATURE),
            },
        )),
);

// LEG 194 — Master of the Hunt
pub(in crate::card::sets) static MASTER_OF_THE_HUNT: CardRecord = CardRecord::new_with_legacy_id(
    1790,
    "Master of the Hunt",
    CardArt::new("4e6bf56e-2d74-4e4d-a667-885853979377", "Jeff A. Menges"),
    CardSet::Legends,
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
pub(in crate::card::sets) static MOSS_MONSTER: CardRecord = CardRecord::new_with_legacy_id(
    454,
    "Moss Monster",
    CardArt::new("9903c043-9a7a-4994-b532-136d4c46edfd", "Jesper Myrfors"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 3, 6),
);

// LEG 196 — Pixie Queen
pub(in crate::card::sets) static PIXIE_QUEEN: CardRecord = CardRecord::new_with_legacy_id(
    455,
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
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEG 197 — Pradesh Gypsies
pub(in crate::card::sets) static PRADESH_GYPSIES: CardRecord = CardRecord::new_with_legacy_id(
    456,
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

/// Every Aura on it counts, whoever controls them.
static AURAS_ON_THE_WOMBAT: ObjectQueryDef = ObjectQueryDef::new(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Subtype("Aura"),
        ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
);

static WOMBAT_BONUS: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    ValueDef::CountMatchingObjects(&AURAS_ON_THE_WOMBAT),
    2,
));

// LEG 198 — Rabid Wombat
pub(in crate::card::sets) static RABID_WOMBAT: CardRecord = CardRecord::new_with_legacy_id(
    1681,
    "Rabid Wombat",
    CardArt::new("9d9b9eb8-6367-4ab5-8e00-a9c9e1d69032", "Kaja Foglio"),
    CardSet::Legends,
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
pub(in crate::card::sets) static RADJAN_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    457,
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
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Flying,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
pub(in crate::card::sets) static SHELKIN_BROWNIE: CardRecord = CardRecord::new_with_legacy_id(
    1784,
    "Shelkin Brownie",
    CardArt::new("fddcc557-871d-425b-b4ee-bc0c9bc717aa", "Douglas Shuler"),
    CardSet::Legends,
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
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Storm Seeker deals damage to target player equal to the number of cards in that player's hand”.

static SUBDUE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// LEG 206 — Subdue
pub(in crate::card::sets) static SUBDUE: CardRecord = CardRecord::new_with_legacy_id(
    1675,
    "Subdue",
    CardArt::new("123d6097-8021-46cd-a8c3-01013245e347", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all combat damage that would be dealt by target creature this turn. That \
         creature gets +0/+X until end of turn, where X is its mana value.",
        &SUBDUE_TARGET,
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

static SYLVAN_PUT_BACK: EffectDef = EffectDef::MoveToZone {
    counters: None,
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    zone: ZoneKind::Library,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
    controller: None,
};

static SYLVAN_SETTLE_CARD: EffectDef = EffectDef::PayOr(PayOrDef::unless(
    EffectPaymentDef::life(PlayerSetDef::One(PlayerRefDef::EffectController), 4),
    &SYLVAN_PUT_BACK,
));

static SYLVAN_SETTLE_CHOSEN: EffectDef = EffectDef::ForEachInBinding {
    objects: ObjectSetBindingIndex::PRIMARY,
    binding: ObjectBindingIndex::PRIMARY,
    effect: &SYLVAN_SETTLE_CARD,
};

static SYLVAN_CHOOSE_DRAWN: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::OrderedObjects(ObjectSetBindingIndex::PRIMARY),
    unchosen: None,
    chooser: PlayerRefDef::EffectController,
    candidates: ObjectSetDef::CardsDrawnThisTurnInHand(PlayerRefDef::EffectController),
    exclude: None,
    minimum: 2,
    maximum: 2,
    visibility: ChoiceVisibilityDef::Private,
    then: &SYLVAN_SETTLE_CHOSEN,
});

static SYLVAN_DRAW_AND_SETTLE: EffectDef = EffectDef::Sequence(&[
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
    SYLVAN_CHOOSE_DRAWN,
]);

// LEG 207 — Sylvan Library
pub(in crate::card::sets) static SYLVAN_LIBRARY: CardRecord = CardRecord::new_with_legacy_id(
    98,
    "Sylvan Library",
    CardArt::new("f486df00-7c4a-4ff0-bb0b-c8b5432ac742", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
    .with_abilities(&[AbilityDef::triggered(
        "At the beginning of your draw step, you may draw two additional cards. If you do, choose two cards in your hand drawn this turn. For each of those pay 4 life or put the card on top of your library.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Draw,
            player: PlayerRelation::You,
        },
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &SYLVAN_DRAW_AND_SETTLE,
        },
    )]),
);

static TYPHOON_OPPONENT_ISLANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

// LEG 208 — Sylvan Paradise
pub(in crate::card::sets) static SYLVAN_PARADISE: CardRecord = CardRecord::new_with_legacy_id(
    1770,
    "Sylvan Paradise",
    CardArt::new(
        "c9157874-70e5-40c1-88d8-1e9afe4d8221",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
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
pub(in crate::card::sets) static TYPHOON: CardRecord = CardRecord::new_with_legacy_id(
    537,
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
pub(in crate::card::sets) static UNTAMED_WILDS: CardRecord = CardRecord::new_with_legacy_id(
    458,
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
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    )),
);

static DERVISH_DREW_BLOOD: TriggerConditionDef =
    TriggerConditionDef::SourceDealtDamageToOpponentThisTurn;

// LEG 211 — Whirling Dervish
pub(in crate::card::sets) static WHIRLING_DERVISH: CardRecord = CardRecord::new_with_legacy_id(
    106,
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
pub(in crate::card::sets) static WILLOW_SATYR: CardRecord = CardRecord::new_with_legacy_id(
    1474,
    "Willow Satyr",
    CardArt::new("0c8b1f49-550e-405f-b17c-1d94589494ad", "Jeff A. Menges"),
    CardSet::Legends,
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
// Audit: blocked — Needs one chosen-X target set with a flying-dependent damage follow-up linked to the creatures it tapped.

// LEG 214 — Wolverine Pack
pub(in crate::card::sets) static WOLVERINE_PACK: CardRecord = CardRecord::new_with_legacy_id(
    1391,
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
pub(in crate::card::sets) static ADUN_OAKENSHIELD: CardRecord = CardRecord::new_with_legacy_id(
    508,
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
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
        )),
);

// LEG 217 — Angus Mackenzie
pub(in crate::card::sets) static ANGUS_MACKENZIE: CardRecord = CardRecord::new_with_legacy_id(
    1672,
    "Angus Mackenzie",
    CardArt::new("57264bd9-94f6-4d4d-baff-2b2900585635", "Bryon Wackwitz"),
    CardSet::Legends,
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

/// Untapped and not attacking, both read continuously -- so a creature that
/// taps for mana or is declared as an attacker loses the toughness at once.
static ARCADES_SABBOTH_DEFENDERS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(2)),
};

// LEG 218 — Arcades Sabboth
pub(in crate::card::sets) static ARCADES_SABBOTH: CardRecord = CardRecord::new_with_legacy_id(
    1901,
    "Arcades Sabboth",
    CardArt::new(
        "2c1dbc62-ceb5-4540-ae38-901e5deafc75",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Legends,
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
            ARCADES_SABBOTH_DEFENDERS,
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
pub(in crate::card::sets) static AXELROD_GUNNARSON: CardRecord = CardRecord::new_with_legacy_id(
    509,
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
            TriggerEventDef::ZoneChanged(
                crate::ZoneChangeEventMatcherDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                )
                .previously_damaged_by(ObjectRefDef::Source),
            ),
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
pub(in crate::card::sets) static BARKTOOTH_WARBEARD: CardRecord = CardRecord::new_with_legacy_id(
    510,
    "Barktooth Warbeard",
    CardArt::new("0ea52228-f8ad-4623-9e05-f162473bfc03", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{B}{R}{R}"), &["Human", "Warrior"], 6, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 222 — Bartel Runeaxe
// Audit: blocked — Needs the card's exact Aura targeting/attachment restriction rather than the broader existing cannot-be-enchanted effect for “Bartel Runeaxe can't be the target of Aura spells”.

// LEG 223 — Boris Devilboon
pub(in crate::card::sets) static BORIS_DEVILBOON: CardRecord = CardRecord::new_with_legacy_id(
    605,
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
            EffectDef::create_creature_token(&["Demon"], &[ManaColor::Black, ManaColor::Red], 1, 1)
                .with_name("Minor Demon"),
        )),
);

// LEG 224 — Chromium
pub(in crate::card::sets) static CHROMIUM: CardRecord = CardRecord::new_with_legacy_id(
    1430,
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
// Audit: partial — Its power and toughness are a battlefield-only continuous effect rather than a characteristic-defining ability, so they read as printed in every other zone.
pub(in crate::card::sets) static DAKKON_BLACKBLADE: CardRecord = CardRecord::new_with_legacy_id(
    1469,
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
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL), ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL)),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "A characteristic-defining ability sets power and toughness in every zone. This is a \
             battlefield-only continuous effect, so the value is right wherever the card is \
             played and absent for anything reading it in another zone.",
        ))),
);

static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// LEG 226 — Gabriel Angelfire
// Audit: blocked — Needs a random choice among four named abilities and a grant of the chosen one; the randomized effect vocabulary selects between two branches, not among four.

// LEG 227 — Gosta Dirk
pub(in crate::card::sets) static GOSTA_DIRK: CardRecord = CardRecord::new_with_legacy_id(
    1399,
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
pub(in crate::card::sets) static GWENDLYN_DI_CORCI: CardRecord = CardRecord::new_with_legacy_id(
    1460,
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
                    then: None,
                },
            )
            .with_activation_timing(ActivationTimingDef::YourTurn),
        ),
);

// LEG 229 — Halfdane
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “At the beginning of your upkeep, change Halfdane's base power and toughness to the power and toughness of target creature other than Halfdane until the end of your next upkeep”.

// LEG 230 — Hazezon Tamar
// Audit: blocked — Needs delayed token creation that determines X at the future upkeep for “When Hazezon enters, create X 1/1 Sand Warrior creature tokens that are red, green, and white at the beginning of your next upkeep, where X is the number of lands you control at that time”.

// LEG 231 — Hunding Gjornersen
pub(in crate::card::sets) static HUNDING_GJORNERSEN: CardRecord = CardRecord::new_with_legacy_id(
    1392,
    "Hunding Gjornersen",
    CardArt::new("07d8e501-6857-4a52-a3b9-2bf0bee5b08c", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{W}{U}{U}"), &["Human", "Warrior"], 5, 4)
        .with_supertype(CardSupertype::Legendary).with_abilities(&[
        abilities::rampage(1, "Rampage 1 (Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocking it beyond the first.)"),
    ]),
);

// LEG 232 — Jacques le Vert
pub(in crate::card::sets) static JACQUES_LE_VERT: CardRecord = CardRecord::new_with_legacy_id(
    511,
    "Jacques le Vert",
    CardArt::new("ee5a45b1-169b-468e-9251-424c09cd7f0f", "Andi Rusu"),
    CardSet::Legends,
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
pub(in crate::card::sets) static JASMINE_BOREAL: CardRecord = CardRecord::new_with_legacy_id(
    512,
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
pub(in crate::card::sets) static JEDIT_OJANEN: CardRecord = CardRecord::new_with_legacy_id(
    513,
    "Jedit Ojanen",
    CardArt::new("97b80124-2b59-425c-93cc-9b032e631c6e", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{4}{W}{W}{U}"), &["Cat", "Warrior"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 235 — Jerrard of the Closed Fist
pub(in crate::card::sets) static JERRARD_OF_THE_CLOSED_FIST: CardRecord =
    CardRecord::new_with_legacy_id(
        514,
        "Jerrard of the Closed Fist",
        CardArt::new("7f841918-813b-4784-ab57-907185b0a355", "Andi Rusu"),
        CardSet::Legends,
        CardRules::new_creature(mana_cost!("{3}{R}{G}{G}"), &["Human", "Knight"], 6, 5)
            .with_supertype(CardSupertype::Legendary),
    );

// LEG 236 — Johan
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “At the beginning of combat on your turn, you may have Johan gain "Johan can't attack" until end of combat. If you do, attacking doesn't cause creatures you control to tap this combat if…”.

// LEG 237 — Kasimir the Lone Wolf
pub(in crate::card::sets) static KASIMIR_THE_LONE_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    515,
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
pub(in crate::card::sets) static KEI_TAKAHASHI: CardRecord = CardRecord::new_with_legacy_id(
    1441,
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
pub(in crate::card::sets) static LADY_CALERIA: CardRecord = CardRecord::new_with_legacy_id(
    516,
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
pub(in crate::card::sets) static LADY_EVANGELA: CardRecord = CardRecord::new_with_legacy_id(
    1442,
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
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// LEG 241 — Lady Orca
pub(in crate::card::sets) static LADY_ORCA: CardRecord = CardRecord::new_with_legacy_id(
    517,
    "Lady Orca",
    CardArt::new("b2779553-74eb-42ba-97d0-96269f48c269", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{5}{B}{R}"), &["Demon"], 7, 4)
        .with_supertype(CardSupertype::Legendary),
);

const NICOL_BOLAS_ENTIRE_HAND: ValueDef = ValueDef::Constant(i32::MAX);

// LEG 242 — Livonya Silone
pub(in crate::card::sets) static LIVONYA_SILONE: CardRecord = CardRecord::new_with_legacy_id(
    1417,
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
pub(in crate::card::sets) static LORD_MAGNUS: CardRecord = CardRecord::new_with_legacy_id(
    1400,
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
pub(in crate::card::sets) static MARHAULT_ELSDRAGON: CardRecord = CardRecord::new_with_legacy_id(
    1393,
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
pub(in crate::card::sets) static NICOL_BOLAS: CardRecord = CardRecord::new_with_legacy_id(
    518,
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
                amount: NICOL_BOLAS_ENTIRE_HAND,
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// LEG 247 — Palladia-Mors
pub(in crate::card::sets) static PALLADIA_MORS: CardRecord = CardRecord::new_with_legacy_id(
    519,
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
pub(in crate::card::sets) static PAVEL_MALIKI: CardRecord = CardRecord::new_with_legacy_id(
    520,
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// LEG 249 — Princess Lucrezia
pub(in crate::card::sets) static PRINCESS_LUCREZIA: CardRecord = CardRecord::new_with_legacy_id(
    521,
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
pub(in crate::card::sets) static RAGNAR: CardRecord = CardRecord::new_with_legacy_id(
    1424,
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
pub(in crate::card::sets) static RAMIREZ_DEPIETRO: CardRecord = CardRecord::new_with_legacy_id(
    522,
    "Ramirez DePietro",
    CardArt::new("e5c66c61-aadf-433b-9958-fc9b44b327b9", "Phil Foglio"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{U}{B}{B}"), &["Human", "Pirate"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::first_strike()),
);

static RAMSES_OVERDARK_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Enchanted,
    ]),
)];

// LEG 252 — Ramses Overdark
pub(in crate::card::sets) static RAMSES_OVERDARK: CardRecord = CardRecord::new_with_legacy_id(
    1680,
    "Ramses Overdark",
    CardArt::new(
        "f079c74e-a39a-40f9-9c7e-9319c0c189c6",
        "Richard Kane Ferguson",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{2}{U}{U}{B}{B}"), &["Human", "Assassin"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Destroy target enchanted creature.",
            &[AbilityCostDef::TapSource],
            &RAMSES_OVERDARK_TARGET,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        )),
);

// LEG 253 — Rasputin Dreamweaver
// Audit: blocked — Needs dream counters that both pay for and restore themselves across several abilities. Each individual prevention and mana effect is available.

// LEG 254 — Riven Turnbull
pub(in crate::card::sets) static RIVEN_TURNBULL: CardRecord = CardRecord::new_with_legacy_id(
    523,
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
pub(in crate::card::sets) static RUBINIA_SOULSINGER: CardRecord = CardRecord::new_with_legacy_id(
    1475,
    "Rubinia Soulsinger",
    CardArt::new("f13e8dc9-8d0f-4a2c-8c0e-be70a3a7dc8e", "Rob Alexander"),
    CardSet::Legends,
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
pub(in crate::card::sets) static SIR_SHANDLAR_OF_EBERYN: CardRecord =
    CardRecord::new_with_legacy_id(
        524,
        "Sir Shandlar of Eberyn",
        CardArt::new("31570ded-f5e3-44c4-b95f-294ac10b2cd2", "Andi Rusu"),
        CardSet::Legends,
        CardRules::new_creature(mana_cost!("{4}{G}{W}"), &["Human", "Knight"], 4, 7)
            .with_supertype(CardSupertype::Legendary),
    );

// LEG 258 — Sivitri Scarzam
pub(in crate::card::sets) static SIVITRI_SCARZAM: CardRecord = CardRecord::new_with_legacy_id(
    525,
    "Sivitri Scarzam",
    CardArt::new("9c12ee9e-db13-4b4d-a061-b6566f538f09", "NéNé Thomas"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{5}{U}{B}"), &["Human"], 6, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 259 — Sol'kanar the Swamp King
pub(in crate::card::sets) static SOLKANAR_THE_SWAMP_KING: CardRecord =
    CardRecord::new_with_legacy_id(
        1799,
        "Sol'kanar the Swamp King",
        CardArt::new(
            "7a20dcb0-5350-40e0-82d3-c8d0186fc9d2",
            "Richard Kane Ferguson",
        ),
        CardSet::Legends,
        CardRules::new_creature(mana_cost!("{2}{U}{B}{R}"), &["Demon"], 5, 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::landwalk(BasicLandType::Swamp),
                AbilityDef::triggered(
                    "Whenever a player casts a black spell, you gain 1 life.",
                    // Any player's: the predicate names a color and nothing about
                    // who cast it.
                    TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Black)),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ]),
    );

// LEG 260 — Stangg
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “When Stangg enters, create Stangg Twin, a legendary 3/4 red and green Human Warrior creature token. Exile that token when Stangg leaves the battlefield. Sacrifice Stangg when that token…”.

// LEG 261 — Sunastian Falconer
pub(in crate::card::sets) static SUNASTIAN_FALCONER: CardRecord = CardRecord::new_with_legacy_id(
    526,
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
pub(in crate::card::sets) static TETSUO_UMEZAWA: CardRecord = CardRecord::new_with_legacy_id(
    1471,
    "Tetsuo Umezawa",
    CardArt::new("8384f87b-26c2-45b7-98ef-352c384f205e", "Julie Baroh"),
    CardSet::Legends,
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
pub(in crate::card::sets) static THE_LADY_OF_THE_MOUNTAIN: CardRecord =
    CardRecord::new_with_legacy_id(
        527,
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
pub(in crate::card::sets) static TOBIAS_ANDRION: CardRecord = CardRecord::new_with_legacy_id(
    528,
    "Tobias Andrion",
    CardArt::new("cac56eda-5ed3-4abd-beec-f5063fbf930a", "Andi Rusu"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{W}{U}"), &["Human", "Advisor"], 4, 4)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 265 — Tor Wauki
pub(in crate::card::sets) static TOR_WAUKI: CardRecord = CardRecord::new_with_legacy_id(
    529,
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
pub(in crate::card::sets) static TORSTEN_VON_URSUS: CardRecord = CardRecord::new_with_legacy_id(
    530,
    "Torsten Von Ursus",
    CardArt::new("5fd99522-4a91-4ccd-91bf-5f32a6ac3510", "Mark Poole"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{W}"), &["Human", "Soldier"], 5, 5)
        .with_supertype(CardSupertype::Legendary),
);

// LEG 267 — Tuknir Deathlock
pub(in crate::card::sets) static TUKNIR_DEATHLOCK: CardRecord = CardRecord::new_with_legacy_id(
    531,
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
pub(in crate::card::sets) static UR_DRAGO: CardRecord = CardRecord::new_with_legacy_id(
    1401,
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
pub(in crate::card::sets) static VAEVICTIS_ASMADI: CardRecord = CardRecord::new_with_legacy_id(
    532,
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
pub(in crate::card::sets) static XIRA_ARIEN: CardRecord = CardRecord::new_with_legacy_id(
    533,
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
pub(in crate::card::sets) static AL_ABARAS_CARPET: CardRecord = CardRecord::new_with_legacy_id(
    1674,
    "Al-abara's Carpet",
    CardArt::new("5d5aae6e-fe20-4363-9589-5a54bcbbb77e", "Kaja Foglio"),
    CardSet::Legends,
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
pub(in crate::card::sets) static ALCHORS_TOMB: CardRecord = CardRecord::new_with_legacy_id(
    1997,
    "Alchor's Tomb",
    CardArt::new("f4395b19-2118-4a09-8932-f9ce9bc54d6d", "Jesper Myrfors"),
    CardSet::Legends,
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
// Audit: partial — External static untap suppression cannot yet apply to every matching legendary creature through the ability-layer fixed point.
pub(in crate::card::sets) static ARENA_OF_THE_ANCIENTS: CardRecord = CardRecord::new_with_legacy_id(
    534,
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
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]), &[ZoneKind::Battlefield], PlayerRelation::Any),
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

static FORETHOUGHT_AMULET_SPELL_DAMAGE: DamageEventMatcherDef = DamageEventMatcherDef {
    kind: DamageKindDef::Any,
    source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ])),
    recipient: DamageRecipientMatcherDef::Any,
};

// LEG 277 — Forethought Amulet
pub(in crate::card::sets) static FORETHOUGHT_AMULET: CardRecord = CardRecord::new_with_legacy_id(
    1717,
    "Forethought Amulet",
    CardArt::new("700f53d3-0a84-4c55-8495-786f0f0783db", "Melissa A. Benson"),
    CardSet::Legends,
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
                    matcher: FORETHOUGHT_AMULET_SPELL_DAMAGE,
                    // Capping at two is the whole clause: an event already at
                    // two or less is untouched either way.
                    limit: DamageLimitDef::CapAt(2),
                }),
            },
        ),
    ]),
);

// LEG 278 — Gauntlets of Chaos
// Audit: blocked — Needs duration-aware control-changing continuous effects for “{5}, Sacrifice this artifact: Exchange control of target artifact, creature, or land you control and target permanent an opponent controls that shares one of those types with it. If…”.

// LEG 279 — Green Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {G}, then add an additional {G} for each charge counter removed this way”.

// LEG 280 — Horn of Deafening
pub(in crate::card::sets) static HORN_OF_DEAFENING: CardRecord = CardRecord::new_with_legacy_id(
    1443,
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
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEG 281 — Knowledge Vault
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “{2}, {T}: Exile the top card of your library face down”.

static KRY_SHIELD_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

// LEG 282 — Kry Shield
pub(in crate::card::sets) static KRY_SHIELD: CardRecord = CardRecord::new_with_legacy_id(
    1676,
    "Kry Shield",
    CardArt::new("a558f23c-c2ce-40d0-b894-f8ccbff8f622", "Richard Thomas"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: Prevent all damage that would be dealt this turn by target creature you \
         control. That creature gets +0/+X until end of turn, where X is its mana value.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        &KRY_SHIELD_TARGET,
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

static LIFE_CHISEL_PAYOFF: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::TriggerEventAmount,
};

// LEG 283 — Life Chisel
pub(in crate::card::sets) static LIFE_CHISEL: CardRecord = CardRecord::new_with_legacy_id(
    1765,
    "Life Chisel",
    CardArt::new("7d355c8a-a3c7-4878-862b-e0d433414943", "Anthony S. Waters"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: You gain life equal to the sacrificed creature's toughness. \
             Activate only during your upkeep.",
            &[],
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: Some(&LIFE_CHISEL_PAYOFF),
                amount: SacrificedAmountDef::Toughness,
                otherwise: None,
                optional: false,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ),
);

// LEG 284 — Life Matrix
// Audit: blocked — Needs granting a counter-consuming activated ability to a targeted creature and an activation window restricted to your upkeep for “{4}, {T}: Put a matrix counter on target creature and that creature gains "Remove a matrix counter from this creature: Regenerate this creature." Activate only during your upkeep”.

static INSTANT_OR_ENCHANTMENT: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Enchantment),
]);

// LEG 285 — Mana Matrix
pub(in crate::card::sets) static MANA_MATRIX: CardRecord = CardRecord::new_with_legacy_id(
    1763,
    "Mana Matrix",
    CardArt::new("0a2a6d21-7731-4228-ab19-b5bc4e31a0d7", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::static_ability(
        "Instant and enchantment spells you cast cost {2} less to cast.",
        EffectDef::ReduceMatchingSpellCostBy {
            spell: INSTANT_OR_ENCHANTMENT,
            caster: PlayerRelation::You,
            amount: ValueDef::Constant(2),
        },
    )),
);

// LEG 286 — Marble Priest
pub(in crate::card::sets) static MARBLE_PRIEST: CardRecord = CardRecord::new_with_legacy_id(
    1719,
    "Marble Priest",
    CardArt::new("3a4f462c-594b-47df-bbe9-18cccb5092c6", "Melissa A. Benson"),
    CardSet::Legends,
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
// Audit: blocked — Needs linked sacrifice/destruction accounting for “{T}, Sacrifice this artifact: Exchange life totals with target opponent. Activate only during your upkeep”.

// LEG 288 — North Star
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “{4}, {T}: For one spell this turn, you may spend mana as though it were mana of any type to pay that spell's mana cost”.

// LEG 289 — Nova Pentacle
// Audit: blocked — Needs a shield keyed to a source chosen as the ability resolves; prevention shields attach to a recipient and spend on the next damage from any source, not from one named source for “{3}, {T}: The next time a source of your choice would deal damage to you this turn, that damage is dealt to target creature of an opponent's choice instead”.

// LEG 290 — Planar Gate
pub(in crate::card::sets) static PLANAR_GATE: CardRecord = CardRecord::new_with_legacy_id(
    1762,
    "Planar Gate",
    CardArt::new("2cf8dfaa-d844-428a-a940-0da989b4c9d1", "Melissa A. Benson"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::static_ability(
        "Creature spells you cast cost {2} less to cast.",
        EffectDef::ReduceMatchingSpellCostBy {
            spell: ObjectPredicateDef::HasType(CardType::Creature),
            caster: PlayerRelation::You,
            amount: ValueDef::Constant(2),
        },
    )),
);

// LEG 291 — Red Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {R}, then add an additional {R} for each charge counter removed this way”.

// LEG 292 — Relic Barrier
pub(in crate::card::sets) static RELIC_BARRIER: CardRecord = CardRecord::new_with_legacy_id(
    121,
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

/// The printed "blocking or blocked by this creature" is the two one-sided
/// relationships together: what the Sentinel is blocking, and what is
/// blocking the Sentinel.
static SENTINEL_COMBAT_PARTNER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::BlockedBySource,
        ObjectPredicateDef::BlockingSource,
    ]),
)];

/// One plus that creature's power, read as the ability resolves.
static SENTINEL_TOUGHNESS: SumValueDef = SumValueDef::new(
    ValueDef::Constant(1),
    ValueDef::TargetPower(TargetIndex::PRIMARY),
);

// LEG 294 — Sentinel
pub(in crate::card::sets) static SENTINEL: CardRecord = CardRecord::new_with_legacy_id(
    1843,
    "Sentinel",
    CardArt::new(
        "5c970830-95cf-4471-af28-43da635073d0",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    // Toughness alone: the Sentinel keeps its printed power of 1, and a
    // free ability that could be used repeatedly is why the effect sets
    // rather than modifies -- each use replaces the last.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Shapeshifter"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{0}: Change this creature's base toughness to 1 plus the power of target creature \
             blocking or blocked by this creature.",
            &[],
            &SENTINEL_COMBAT_PARTNER,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_base_toughness(ValueDef::Sum(&SENTINEL_TOUGHNESS)),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ),
);

// LEG 295 — Serpent Generator
pub(in crate::card::sets) static SERPENT_GENERATOR: CardRecord = CardRecord::new_with_legacy_id(
    1560,
    "Serpent Generator",
    CardArt::new("7c350c38-6cbb-4b8b-823f-45d6a16568cc", "Mark Tedin"),
    CardSet::Legends,
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
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “{T}, Sacrifice this artifact and any number of creatures you control: This artifact deals X damage to any target, where X is the total power of the creatures sacrificed this way, then…”.

// LEG 297 — Triassic Egg
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “Sacrifice this artifact: Choose one. Activate only if there are two or more hatchling counters on this artifact”.

// LEG 298 — Voodoo Doll
// Audit: blocked — Needs a mana activation cost whose amount is read from the source rather than printed, for “{X}{X}, {T}: ... X is the number of pin counters on this artifact”. The upkeep tick, the untapped intervening-if, and damage equal to a counter count are all available.

// LEG 299 — White Mana Battery
// Audit: blocked — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}, Remove any number of charge counters from this artifact: Add {W}, then add an additional {W} for each charge counter removed this way”.

/// The clause the five Legends band lands grant. One static rather than five,
/// because the lands differ only in which color of legend they reach.
static BANDS_WITH_OTHER_LEGENDS: AbilityDef =
    abilities::bands_with_other(BandingQuality::LegendaryCreatures);

// LEG 300 — Adventurers' Guildhouse
pub(in crate::card::sets) static ADVENTURERS_GUILDHOUSE: CardRecord =
    CardRecord::new_with_legacy_id(
        1785,
        "Adventurers' Guildhouse",
        CardArt::new("32865e68-5842-4f17-b2ea-4ffa743b511f", "Tom Wänerstrand"),
        CardSet::Legends,
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
pub(in crate::card::sets) static CATHEDRAL_OF_SERRA: CardRecord = CardRecord::new_with_legacy_id(
    1786,
    "Cathedral of Serra",
    CardArt::new("e65356e6-0ead-49fd-b069-be1ea9b1c105", "Mark Poole"),
    CardSet::Legends,
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
// Audit: blocked — Needs removing every ability of a class from a target for a duration; the vocabulary grants named abilities but does not take them away.

// LEG 303 — Karakas
pub(in crate::card::sets) static KARAKAS: CardRecord = CardRecord::new_with_legacy_id(
    535,
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
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
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
pub(in crate::card::sets) static MOUNTAIN_STRONGHOLD: CardRecord = CardRecord::new_with_legacy_id(
    1787,
    "Mountain Stronghold",
    CardArt::new("314fd1d7-4bd8-4d95-b7c2-1aa6660ab88a", "Tom Wänerstrand"),
    CardSet::Legends,
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
// Audit: partial — The target's power and toughness omit modifiers from static continuous effects.
pub(in crate::card::sets) static PENDELHAVEN: CardRecord = CardRecord::new_with_legacy_id(
    120,
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
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
    EffectDef::PayOr(PayOrDef::unless_mana(
        mana_cost!("{1}"),
        &EffectDef::Destroy {
            object: EffectRecipientDef::Source,
            can_regenerate: true,
        },
    )),
);

// LEG 306 — Seafarer's Quay
pub(in crate::card::sets) static SEAFARERS_QUAY: CardRecord = CardRecord::new_with_legacy_id(
    1788,
    "Seafarer's Quay",
    CardArt::new("66641d88-b3f0-4bcd-8d2d-29aa2de69e30", "Tom Wänerstrand"),
    CardSet::Legends,
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
pub(in crate::card::sets) static THE_TABERNACLE_AT_PENDRELL_VALE: CardRecord = CardRecord::new_with_legacy_id(
    536,
    "The Tabernacle at Pendrell Vale",
    CardArt::new("64bc9b1d-5818-4d9e-b771-e49af4ff9a5c", "Nicola Leonard"),
    CardSet::Legends,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "All creatures have \"At the beginning of your upkeep, destroy this creature unless you pay {1}.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::add_ability(&TABERNACLE_UPKEEP_ABILITY),
            },
        )),
);

/// Two removals in one clause: the printed text takes plain banding and the
/// variant together, and neither implies the other.
static TOLARIA_REMOVALS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(KeywordAbility::Banding)),
    AppliedEffectDef::remove_abilities(AbilityPredicateDef::AnyBandsWithOther),
];

// LEG 308 — Tolaria
pub(in crate::card::sets) static TOLARIA: CardRecord = CardRecord::new_with_legacy_id(
    1783,
    "Tolaria",
    CardArt::new("d43c01b7-443d-4061-a934-6863d230c9b8", "Nicola Leonard"),
    CardSet::Legends,
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
                    effect: AppliedEffectDef::Composite(&TOLARIA_REMOVALS),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_activation_timing(ActivationTimingDef::AnyUpkeep),
        ]),
);

// LEG 309 — Unholy Citadel
pub(in crate::card::sets) static UNHOLY_CITADEL: CardRecord = CardRecord::new_with_legacy_id(
    1789,
    "Unholy Citadel",
    CardArt::new("9de534ff-fb48-4692-bd0f-dd237ca28502", "Mark Poole"),
    CardSet::Legends,
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
// Audit: blocked — Needs a modal choice between two named abilities and the removal of the chosen one; the vocabulary grants named abilities but does not take them away.

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AKRON_LEGIONNAIRE,
    &ALABASTER_POTION,
    &AMROU_KITHKIN,
    &ANGELIC_VOICES,
    &CLEANSE,
    &DAVENANT_ARCHER,
    &DIVINE_OFFERING,
    &DIVINE_TRANSFORMATION,
    &ELDER_LAND_WURM,
    &ENCHANTED_BEING,
    &FORTIFIED_AREA,
    &GREAT_DEFENDER,
    &GREAT_WALL,
    &GREATER_REALM_OF_PRESERVATION,
    &HEAVENS_GATE,
    &HOLY_DAY,
    &INDESTRUCTIBLE_AURA,
    &IVORY_GUARDIANS,
    &KEEPERS_OF_THE_FAITH,
    &KISMET,
    &LIFEBLOOD,
    &MOAT,
    &OSAI_VULTURES,
    &PRESENCE_OF_THE_MASTER,
    &RIGHTEOUS_AVENGERS,
    &SEEKER,
    &SHIELD_WALL,
    &SPIRIT_LINK,
    &SPIRITUAL_SANCTUARY,
    &THUNDER_SPIRIT,
    &TUNDRA_WOLVES,
    &VISIONS,
    &WALL_OF_LIGHT,
    &ACID_RAIN,
    &AZURE_DRAKE,
    &BOOMERANG,
    &DEVOURING_DEEP,
    &ELDER_SPAWN,
    &ENERGY_TAP,
    &FLASH_COUNTER,
    &FLASH_FLOOD,
    &FORCE_SPIKE,
    &GASEOUS_FORM,
    &MANA_DRAIN,
    &PART_WATER,
    &PSIONIC_ENTITY,
    &RECALL,
    &REMOVE_SOUL,
    &RESET,
    &SEA_KINGS_BLESSING,
    &SEGOVIAN_LEVIATHAN,
    &SPECTRAL_CLOAK,
    &TELEKINESIS,
    &TELEPORT,
    &UNDERTOW,
    &VENARIAN_GOLD,
    &WALL_OF_VAPOR,
    &WALL_OF_WONDER,
    &ZEPHYR_FALCON,
    &ABOMINATION,
    &BLIGHT,
    &CARRION_ANTS,
    &CYCLOPEAN_MUMMY,
    &DARKNESS,
    &DEMONIC_TORMENT,
    &EVIL_EYE_OF_ORMS_BY_GORE,
    &FALLEN_ANGEL,
    &GHOSTS_OF_THE_DAMNED,
    &GREED,
    &HEADLESS_HORSEMAN,
    &HELL_SWARM,
    &HELLS_CARETAKER,
    &HORROR_OF_HORRORS,
    &INFERNAL_MEDUSA,
    &LOST_SOUL,
    &NETHER_VOID,
    &PIT_SCORPION,
    &QUAGMIRE,
    &SHIMIAN_NIGHT_STALKER,
    &SPIRIT_SHACKLE,
    &THE_ABYSS,
    &THE_WRETCHED,
    &TOUCH_OF_DARKNESS,
    &TRANSMUTATION,
    &VAMPIRE_BATS,
    &WALKING_DEAD,
    &WALL_OF_PUTRID_FLESH,
    &WALL_OF_SHADOWS,
    &ACTIVE_VOLCANO,
    &AERATHI_BERSERKER,
    &BEASTS_OF_BOGARDAN,
    &CHAIN_LIGHTNING,
    &CREVASSE,
    &CRIMSON_KOBOLDS,
    &CRIMSON_MANTICORE,
    &CROOKSHANK_KOBOLDS,
    &DISHARMONY,
    &DWARVEN_SONG,
    &ETERNAL_WARRIOR,
    &FROST_GIANT,
    &GIANT_STRENGTH,
    &GLYPH_OF_DESTRUCTION,
    &GRAVITY_SPHERE,
    &IMMOLATION,
    &KOBOLD_DRILL_SERGEANT,
    &KOBOLD_OVERLORD,
    &KOBOLD_TASKMASTER,
    &KOBOLDS_OF_KHER_KEEP,
    &MOUNTAIN_YETI,
    &PRIMORDIAL_OOZE,
    &PYROTECHNICS,
    &RAGING_BULL,
    &SPINAL_VILLAIN,
    &THE_BRUTE,
    &WALL_OF_EARTH,
    &WALL_OF_HEAT,
    &WALL_OF_OPPOSITION,
    &AISLING_LEPRECHAUN,
    &BARBARY_APES,
    &CAT_WARRIORS,
    &COCOON,
    &CONCORDANT_CROSSROADS,
    &CRAW_GIANT,
    &DEADFALL,
    &DURKWOOD_BOARS,
    &ELVEN_RIDERS,
    &EMERALD_DRAGONFLY,
    &FIRE_SPRITES,
    &FLORAL_SPUZZEM,
    &GIANT_TURTLE,
    &HORNET_COBRA,
    &KILLER_BEES,
    &LIVING_PLANE,
    &MASTER_OF_THE_HUNT,
    &MOSS_MONSTER,
    &PIXIE_QUEEN,
    &PRADESH_GYPSIES,
    &RABID_WOMBAT,
    &RADJAN_SPIRIT,
    &SHELKIN_BROWNIE,
    &SUBDUE,
    &SYLVAN_LIBRARY,
    &SYLVAN_PARADISE,
    &TYPHOON,
    &UNTAMED_WILDS,
    &WHIRLING_DERVISH,
    &WILLOW_SATYR,
    &WOLVERINE_PACK,
    &ADUN_OAKENSHIELD,
    &ANGUS_MACKENZIE,
    &ARCADES_SABBOTH,
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
    &RAMSES_OVERDARK,
    &RIVEN_TURNBULL,
    &RUBINIA_SOULSINGER,
    &SIR_SHANDLAR_OF_EBERYN,
    &SIVITRI_SCARZAM,
    &SOLKANAR_THE_SWAMP_KING,
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
    &FORETHOUGHT_AMULET,
    &HORN_OF_DEAFENING,
    &KRY_SHIELD,
    &LIFE_CHISEL,
    &MANA_MATRIX,
    &MARBLE_PRIEST,
    &PLANAR_GATE,
    &RELIC_BARRIER,
    &SENTINEL,
    &SERPENT_GENERATOR,
    &ADVENTURERS_GUILDHOUSE,
    &CATHEDRAL_OF_SERRA,
    &KARAKAS,
    &MOUNTAIN_STRONGHOLD,
    &PENDELHAVEN,
    &SEAFARERS_QUAY,
    &THE_TABERNACLE_AT_PENDRELL_VALE,
    &TOLARIA,
    &UNHOLY_CITADEL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
