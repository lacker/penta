use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef,
    AttackDefenderScopeDef, AttackRestrictionDef, BasicLandType, CardArt, CardBehavior, CardRules,
    CardSet, CardSupertype, CardType, CardTypeSet, ChoiceVisibilityDef, ChooseDef, ColorSet,
    ComparisonDef, ControlDurationDef, CostModificationDef, CounterKind, CreatureTypeSetDef,
    DamageEventMatcherDef, DamagePreventionDef, DamagePreventionFollowUpDef,
    DamageRecipientMatcherDef, DamageSourceGroupDef, DiscardSelectionDef, EffectDef,
    EffectPaymentDef, EffectRecipientDef, HalvedValueDef, InstalledTriggerDef, KeywordAbility,
    LikelihoodDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, OngoingEffectDef, PayOrDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ReplacementAbilityDef, ReplacementChoiceDef, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, RoundingDef,
    TriggerConditionDef, TriggerEventDef, TurnKindDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

use abilities::{ENCHANT_CREATURE_TARGET, ENCHANT_LAND_TARGET, aura_spell};

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

/// The Lace cycle's target: anything at all, on the stack or the battlefield.
static SPELL_OR_PERMANENT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Battlefield, ZoneKind::Stack],
        controller: None,
        owner: None,
    },
)];

// LEA 1 — Animate Wall
/// The Aura goes on a Wall specifically, which is narrower than the ordinary
/// "enchant creature" and is what makes the permission below worth anything.
static ENCHANT_WALL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Subtype("Wall"),
)];

pub(in crate::card::sets) static ANIMATE_WALL: CardRecord = CardRecord::new_with_legacy_id(
    1731,
    "Animate Wall",
    CardArt::new("6757e04d-7bfc-4bdc-9dcb-02059a2d4e60", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant Wall", &ENCHANT_WALL_TARGET),
            AbilityDef::static_ability(
                "Enchanted Wall can attack as though it didn't have defender.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                },
            ),
        ]),
);

// LEA 2 — Armageddon
pub(in crate::card::sets) static ARMAGEDDON: CardRecord = CardRecord::new_with_legacy_id(
    58,
    "Armageddon",
    CardArt::new("5b6ddce7-b9c5-431d-a0b0-46d4aa93cbcb", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell(
        "Destroy all lands.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )]),
);

// LEA 3 — Balance
// Audit: custom — Needs a declarative multi-player equalization procedure that resolves lands, hands, and creatures in order without leaking simultaneous choices.
pub(in crate::card::sets) static BALANCE: CardRecord = CardRecord::new_with_legacy_id(
    60,
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

// LEA 4 — Benalish Hero
pub(in crate::card::sets) static BENALISH_HERO: CardRecord = CardRecord::new_with_legacy_id(
    1772,
    "Benalish Hero",
    CardArt::new("11600105-56c6-4073-a4a6-8469030b39c9", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1)
        .with_abilities(&[abilities::banding()]),
);

// LEA 5 — Black Ward
pub(in crate::card::sets) static BLACK_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1553,
    "Black Ward",
    CardArt::new("15967a39-303f-457d-bcde-51837c8d63e1", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            abilities::ward_aura_protection(
                ManaColor::Black,
                "Enchanted creature has protection from black. This effect doesn't remove \
                 this Aura.",
            ),
        ]),
);

// LEA 6 — Blaze of Glory
/// Both halves at once: the ceiling comes off, and what is left is a
/// requirement to use it. Either alone would be a different card.
static BLAZE_OF_GLORY_EFFECT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(u8::MAX)),
    AppliedEffectDef::Rule(AppliedRuleDef::MustBlockEachAttackerIfAble),
];

/// "Defending player" is the nonactive player, which is who it is however the
/// spell got cast -- naming an opponent would read it off the caster instead.
static BLAZE_OF_GLORY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::NonactivePlayer),
        owner: None,
    },
)];

pub(in crate::card::sets) static BLAZE_OF_GLORY: CardRecord = CardRecord::new_with_legacy_id(
    1812,
    "Blaze of Glory",
    CardArt::new("98fba951-c5bb-497c-9292-ce1b2a1e1247", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature defending player controls can block any number of creatures this \
         turn. It blocks each attacking creature this turn if able.",
        &BLAZE_OF_GLORY_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&BLAZE_OF_GLORY_EFFECT),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEA 7 — Blessing
pub(in crate::card::sets) static BLESSING: CardRecord = CardRecord::new_with_legacy_id(
    315,
    "Blessing",
    CardArt::new("f131fd27-18da-47ca-b59f-135bcac83abd", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::activated(
                "{W}: Enchanted creature gets +1/+1 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEA 8 — Blue Ward
pub(in crate::card::sets) static BLUE_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1554,
    "Blue Ward",
    CardArt::new("93f9f0f2-e1cc-4740-888c-1336c6de0a27", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            abilities::ward_aura_protection(
                ManaColor::Blue,
                "Enchanted creature has protection from blue. This effect doesn't remove \
                 this Aura.",
            ),
        ]),
);

// LEA 9 — Castle
pub(in crate::card::sets) static CASTLE: CardRecord = CardRecord::new_with_legacy_id(
    1663,
    "Castle",
    CardArt::new("b0da8d56-3178-44c2-9344-95d2346d326f", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_ability(AbilityDef::static_ability(
        "Untapped creatures you control get +0/+2.",
        EffectDef::StaticApply {
            // The condition rides on the recipient, so a creature tapping
            // loses the bonus without the enchantment being touched.
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
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

// LEA 10 — Circle of Protection: Blue
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_BLUE: CardRecord = CardRecord::new_with_legacy_id(
    1446,
    "Circle of Protection: Blue",
    CardArt::new("848b1a7f-e8ba-40b5-92b7-af1e963a0319", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a blue source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::Blue),
        ),
    ),
);

// LEA 11 — Circle of Protection: Green
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_GREEN: CardRecord = CardRecord::new_with_legacy_id(
    1447,
    "Circle of Protection: Green",
    CardArt::new("1ae32d20-b438-4f43-b603-e8f706ecfb03", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a green source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::Green),
        ),
    ),
);

// LEA 12 — Circle of Protection: Red
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_RED: CardRecord = CardRecord::new_with_legacy_id(
    1448,
    "Circle of Protection: Red",
    CardArt::new("b3dd94c5-42f6-4148-be6e-2a3a4226cc0e", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a red source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::Red),
        ),
    ),
);

// LEA 13 — Circle of Protection: White
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_WHITE: CardRecord = CardRecord::new_with_legacy_id(
    1449,
    "Circle of Protection: White",
    CardArt::new("92df19c9-e127-42d9-8dd2-7fa5a7095428", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a white source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::White),
        ),
    ),
);

// LEA 14 — Consecrate Land
// Audit: metadata-only — Needs this compound indestructibility and attachment-legality effect for “Enchanted land has indestructible and can't be enchanted by other Auras”.
pub(in crate::card::sets) static CONSECRATE_LAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2379f78-c03f-447f-b3c9-10a918d556e9"),
    "Consecrate Land",
    crate::card::CardArt::new("d2379f78-c03f-447f-b3c9-10a918d556e9", "Jeff A. Menges"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 15 — Conversion
pub(in crate::card::sets) static CONVERSION: CardRecord = CardRecord::new_with_legacy_id(
    316,
    "Conversion",
    CardArt::new("13186bc9-8d9c-433b-ba15-121ef94dd68a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {W}{W}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{W}{W}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::static_ability(
            "All Mountains are Plains.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Plains]),
            },
        ),
    ]),
);

// LEA 16 — Crusade
pub(in crate::card::sets) static CRUSADE: CardRecord = CardRecord::new_with_legacy_id(
    67,
    "Crusade",
    CardArt::new("057986c7-20c0-4157-b4df-beae4ef5c66d", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}{W}")).with_abilities(&[AbilityDef::static_ability(
        "White creatures get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    )]),
);

// LEA 17 — Death Ward
pub(in crate::card::sets) static DEATH_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1419,
    "Death Ward",
    CardArt::new("fa5466cc-aa57-4a7f-8b21-d92b2fe02e13", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Regenerate target creature.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::Regenerate {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )]),
);

// LEA 18 — Disenchant
pub(in crate::card::sets) static DISENCHANT: CardRecord = CardRecord::new_with_legacy_id(
    47,
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

// LEA 19 — Farmstead
static FARMSTEAD_LAND_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, you may pay {W}{W}. If you do, you gain 1 life.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef::mana(
            PlayerSetDef::Related(PlayerRelation::You),
            mana_cost!("{W}{W}"),
        ),
        &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

pub(in crate::card::sets) static FARMSTEAD: CardRecord = CardRecord::new_with_legacy_id(
    459,
    "Farmstead",
    CardArt::new("3455b006-9ea5-4aef-8ad2-d0701eb0cacf", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &ENCHANT_LAND_TARGET),
            AbilityDef::static_ability(
                "Enchanted land has \"At the beginning of your upkeep, you may pay {W}{W}. If you do, you gain 1 life.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&FARMSTEAD_LAND_ABILITY),
                },
            ),
        ]),
);

// LEA 20 — Green Ward
pub(in crate::card::sets) static GREEN_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1555,
    "Green Ward",
    CardArt::new("1f6118b2-fe01-425a-a2ed-6d7c42286c8e", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            abilities::ward_aura_protection(
                ManaColor::Green,
                "Enchanted creature has protection from green. This effect doesn't remove \
                 this Aura.",
            ),
        ]),
);

// LEA 21 — Guardian Angel
static GUARDIAN_ANGEL_PAYMENT: AbilityDef = AbilityDef::activated(
    "{1}: Prevent the next 1 damage that would be dealt to the affected permanent or player this turn.",
    &[AbilityCostDef::Mana(mana_cost!("{1}"))],
    EffectDef::PreventDamage {
        prevention: DamagePreventionDef::amount(
            DamageEventMatcherDef::to(EffectRecipientDef::object(ObjectRefDef::Binding(
                ObjectBindingIndex::PRIMARY,
            ))),
            ValueDef::Constant(1),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
)
.with_source_zones(&[ZoneKind::Command]);

pub(in crate::card::sets) static GUARDIAN_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f84d676-5327-454c-a033-b4498a9d28e2"),
    "Guardian Angel",
    CardArt::new("0f84d676-5327-454c-a033-b4498a9d28e2", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{X}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent the next X damage that would be dealt to any target this turn. Until end of turn, you may pay {1} any time you could cast an instant. If you do, prevent the next 1 damage that would be dealt to that permanent or player this turn.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::ChosenX,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateOngoingEffect(OngoingEffectDef::new(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ObjectBindingIndex::PRIMARY,
                &GUARDIAN_ANGEL_PAYMENT,
                ResolvedEffectDurationDef::UntilEndOfTurn,
            )),
        ]),
    )),
);

// LEA 22 — Healing Salve
pub(in crate::card::sets) static HEALING_SALVE: CardRecord = CardRecord::new_with_legacy_id(
    1581,
    "Healing Salve",
    CardArt::new("e28de37e-84d5-4dc7-b36c-e14da5924729", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Target player gains 3 life.\n• Prevent the next 3 damage that would \
         be dealt to any target this turn.",
        &[
            AbilityDef::spell_with_targets(
                "Target player gains 3 life",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::spell_with_targets(
                "Prevent the next 3 damage that would be dealt to any target this turn",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::amount(
                        DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// LEA 23 — Holy Armor
pub(in crate::card::sets) static HOLY_ARMOR: CardRecord = CardRecord::new_with_legacy_id(
    317,
    "Holy Armor",
    CardArt::new("b01041d2-687e-4972-81c8-16690809275b", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::activated(
                "{W}: Enchanted creature gets +0/+1 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEA 24 — Holy Strength
pub(in crate::card::sets) static HOLY_STRENGTH: CardRecord = CardRecord::new_with_legacy_id(
    318,
    "Holy Strength",
    CardArt::new("e945a4cd-0eb1-4f54-898d-169ce2748a03", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                },
            ),
        ]),
);

// LEA 25 — Island Sanctuary
static ISLAND_SANCTUARY_ATTACKERS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
    ObjectPredicateDef::HasKeyword(KeywordAbility::Landwalk(BasicLandType::Island)),
]);

static ISLAND_SANCTUARY_PREVENTED_ATTACKERS: ObjectPredicateDef =
    ObjectPredicateDef::Not(&ISLAND_SANCTUARY_ATTACKERS);

static ISLAND_SANCTUARY_RESTRICTION: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Controller,
    effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
        AttackRestrictionDef::prohibit(
            ISLAND_SANCTUARY_PREVENTED_ATTACKERS,
            AttackDefenderScopeDef::AffectedPlayer,
        ),
    )),
    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
};

static ISLAND_SANCTUARY_REPLACEMENT: [ReplacementEffectDef; 2] = [
    ReplacementEffectDef::ReplaceEventWithNothing,
    ReplacementEffectDef::Perform(&ISLAND_SANCTUARY_RESTRICTION),
];

pub(in crate::card::sets) static ISLAND_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c15e8a42-89de-42bc-8d5f-33426d207c3a"),
    "Island Sanctuary",
    CardArt::new("c15e8a42-89de-42bc-8d5f-33426d207c3a", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::defined_replacement(
            "If you would draw a card during your draw step, instead you may skip that draw. If you do, until your next turn, you can't be attacked except by creatures with flying and/or islandwalk.",
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::WouldDraw {
                    player: PlayerRelation::You,
                    during_own_draw_step: true,
                })
                .optional(),
            ReplacementEffectDef::Sequence(&ISLAND_SANCTUARY_REPLACEMENT),
        ),
    ]),
);

// LEA 26 — Karma
static KARMA_SWAMPS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
    &[ZoneKind::Battlefield],
    PlayerRelation::EventPlayer,
);

pub(in crate::card::sets) static KARMA: CardRecord = CardRecord::new_with_legacy_id(
    319,
    "Karma",
    CardArt::new(
        "6f30ad61-fcb7-4d55-ba86-94de1bf545e4",
        "Richard Thomas",
    ),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of each player's upkeep, this enchantment deals damage to that player equal to the number of Swamps they control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::CountMatchingObjects(&KARMA_SWAMPS),
            },
        ),
    ]),
);

// LEA 27 — Lance
pub(in crate::card::sets) static LANCE: CardRecord = CardRecord::new_with_legacy_id(
    320,
    "Lance",
    CardArt::new("ddb633f5-cc4d-4157-8217-def90cb15e24", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            ),
        ]),
);

// LEA 28 — Mesa Pegasus
pub(in crate::card::sets) static MESA_PEGASUS: CardRecord = CardRecord::new_with_legacy_id(
    1773,
    "Mesa Pegasus",
    CardArt::new("eaac88da-d19e-4771-944c-3709963d04e7", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Pegasus"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::banding()]),
);

// LEA 29 — Northern Paladin
pub(in crate::card::sets) static NORTHERN_PALADIN: CardRecord = CardRecord::new_with_legacy_id(
    321,
    "Northern Paladin",
    CardArt::new("6303233b-35eb-49ca-b844-ba6b9fe1cbd2", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 3, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{W}{W}, {T}: Destroy target black permanent.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Color(ManaColor::Black),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// LEA 30 — Pearled Unicorn
pub(in crate::card::sets) static PEARLED_UNICORN: CardRecord = CardRecord::new_with_legacy_id(
    322,
    "Pearled Unicorn",
    CardArt::new("6daf1aab-1e58-4a5a-bc66-cb3f7c86e0e8", "Cornelius Brudi"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Unicorn"], 2, 2),
);

// LEA 31 — Personal Incarnation
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “{0}: The next 1 damage that would be dealt to this creature this turn is dealt to its owner instead. Only this creatures owner may activate this ability”.
pub(in crate::card::sets) static PERSONAL_INCARNATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("caf9cef4-0f2d-478a-b119-fe1967687f74"),
    "Personal Incarnation",
    crate::card::CardArt::new("caf9cef4-0f2d-478a-b119-fe1967687f74", "Kev Brockschmidt"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 32 — Purelace
pub(in crate::card::sets) static PURELACE: CardRecord = CardRecord::new_with_legacy_id(
    1562,
    "Purelace",
    CardArt::new("2facf462-55cd-4da4-997f-2cf4add75628", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target spell or permanent becomes white. (Mana symbols on that permanent remain unchanged.)",
        &SPELL_OR_PERMANENT_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    )),
);

// LEA 33 — Red Ward
pub(in crate::card::sets) static RED_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1556,
    "Red Ward",
    CardArt::new("e0c64c01-c2aa-470b-88c6-3d3e4a969649", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            abilities::ward_aura_protection(
                ManaColor::Red,
                "Enchanted creature has protection from red. This effect doesn't remove \
                 this Aura.",
            ),
        ]),
);

// LEA 34 — Resurrection
pub(in crate::card::sets) static RESURRECTION: CardRecord = CardRecord::new_with_legacy_id(
    323,
    "Resurrection",
    CardArt::new("4fff6e6f-4ebd-4ec8-9443-59efb22d376c", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the battlefield.",
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
        ),
    ]),
);

// LEA 35 — Reverse Damage
static REVERSE_DAMAGE_SHIELD: EffectDef = EffectDef::PreventDamage {
    prevention: DamagePreventionDef::events(
        DamageEventMatcherDef {
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Controller),
            ..DamageEventMatcherDef::from(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY))
        },
        1,
    )
    .with_follow_up(DamagePreventionFollowUpDef::GainLife(
        PlayerRefDef::EffectController,
    )),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static REVERSE_DAMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1453,
    "Reverse Damage",
    CardArt::new("943baea8-b173-4863-a3ab-dd217d483cd9", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::spell(
        "The next time a source of your choice would deal damage to you this turn, prevent that \
         damage. You gain life equal to the damage prevented this way.",
        abilities::shield_against_a_chosen_source(ObjectPredicateDef::Any, &REVERSE_DAMAGE_SHIELD),
    )),
);

// LEA 36 — Righteousness
pub(in crate::card::sets) static RIGHTEOUSNESS: CardRecord = CardRecord::new_with_legacy_id(
    324,
    "Righteousness",
    CardArt::new("d0ba7b76-f3d0-47d0-8a35-0c08e67200fb", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target blocking creature gets +7/+7 until end of turn.",
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
                ValueDef::Constant(7),
                ValueDef::Constant(7),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 37 — Samite Healer
pub(in crate::card::sets) static SAMITE_HEALER: CardRecord = CardRecord::new_with_legacy_id(
    1409,
    "Samite Healer",
    CardArt::new("efba235e-04e5-449c-906c-0ac33f6d7929", "Tom Wänerstrand"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[AbilityCostDef::TapSource],
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
        ),
    ]),
);

// LEA 38 — Savannah Lions
pub(in crate::card::sets) static SAVANNAH_LIONS: CardRecord = CardRecord::new_with_legacy_id(
    92,
    "Savannah Lions",
    CardArt::new("d05b92bd-797e-413f-a8b0-32e0937a1ee0", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 2, 1),
);

// LEA 39 — Serra Angel
pub(in crate::card::sets) static SERRA_ANGEL: CardRecord = CardRecord::new_with_legacy_id(
    53,
    "Serra Angel",
    CardArt::new("f8ac5006-91bd-4803-93da-f87cf196dd2f", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 4, 4)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// LEA 40 — Swords to Plowshares
pub(in crate::card::sets) static SWORDS_TO_PLOWSHARES: CardRecord = CardRecord::new_with_legacy_id(
    54,
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
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// LEA 41 — Veteran Bodyguard
/// "As long as this creature is untapped": the condition rides on the
/// recipient, so tapping it turns the redirection off and untapping turns it
/// back on without the creature being touched.
static UNTAPPED_SELF: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Source,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static VETERAN_BODYGUARD: CardRecord = CardRecord::new_with_legacy_id(
    1684,
    "Veteran Bodyguard",
    CardArt::new("cbd9ab01-a833-4fa4-8dee-151bd9800835", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human"], 2, 5).with_ability(
        AbilityDef::static_ability(
            "As long as this creature is untapped, all damage that would be dealt to you by \
             unblocked creatures is dealt to this creature instead.",
            EffectDef::StaticApply {
                recipient: UNTAPPED_SELF,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::RedirectPlayerDamageToThis(
                    DamageSourceGroupDef::UnblockedCreatures,
                )),
            },
        ),
    ),
);

// LEA 42 — Wall of Swords
pub(in crate::card::sets) static WALL_OF_SWORDS: CardRecord = CardRecord::new_with_legacy_id(
    325,
    "Wall of Swords",
    CardArt::new("99ec4723-b36c-4015-b361-736a6523e8f5", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Wall"], 3, 5)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// LEA 43 — White Knight
pub(in crate::card::sets) static WHITE_KNIGHT: CardRecord = CardRecord::new_with_legacy_id(
    107,
    "White Knight",
    CardArt::new("50abfba8-c9f9-4ebf-965a-4b425fe83129", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// LEA 44 — White Ward
pub(in crate::card::sets) static WHITE_WARD: CardRecord = CardRecord::new_with_legacy_id(
    1557,
    "White Ward",
    CardArt::new("49b22665-1501-420a-82ad-f71f6768bcf8", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            abilities::ward_aura_protection(
                ManaColor::White,
                "Enchanted creature has protection from white. This effect doesn't remove \
                 this Aura.",
            ),
        ]),
);

// LEA 45 — Wrath of God
pub(in crate::card::sets) static WRATH_OF_GOD: CardRecord = CardRecord::new_with_legacy_id(
    128,
    "Wrath of God",
    CardArt::new("a2788d69-6a3a-42f0-8736-cc6b57755ecd", "Quinton Hoover"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all creatures. They can't be regenerated.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: false,
        },
    )),
);

// LEA 46 — Air Elemental
pub(in crate::card::sets) static AIR_ELEMENTAL: CardRecord = CardRecord::new_with_legacy_id(
    326,
    "Air Elemental",
    CardArt::new("69c3b2a3-0daa-4d42-832d-fcdfda6555ea", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Elemental"], 4, 4)
        .with_abilities(&[abilities::flying()]),
);

// LEA 47 — Ancestral Recall
pub(in crate::card::sets) static ANCESTRAL_RECALL: CardRecord = CardRecord::new_with_legacy_id(
    44,
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

// LEA 48 — Animate Artifact
/// One continuous effect begins on the attached noncreature artifact in
/// layer 4 and keeps that same recipient for its layer-7 body under CR 613.6.
static ANIMATE_ARTIFACT_BODY: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(
        ValueDef::AffectedManaValue,
        ValueDef::AffectedManaValue,
    ),
]);

static ANIMATE_ARTIFACT_RECIPIENT: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::AttachedToSource,
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static ANIMATE_ARTIFACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("664b46f5-0424-4f4e-9f26-6bd2cf5e0357"),
    "Animate Artifact",
    CardArt::new("664b46f5-0424-4f4e-9f26-6bd2cf5e0357", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant artifact", &abilities::ENCHANT_ARTIFACT_TARGET),
            AbilityDef::static_ability(
                "As long as enchanted artifact isn't a creature, it's an artifact creature with power and toughness each equal to its mana value.",
                EffectDef::StaticApply {
                    recipient: ANIMATE_ARTIFACT_RECIPIENT,
                    effect: ANIMATE_ARTIFACT_BODY,
                },
            ),
        ]),
);

// LEA 49 — Blue Elemental Blast
pub(in crate::card::sets) static BLUE_ELEMENTAL_BLAST: CardRecord = CardRecord::new_with_legacy_id(
    64,
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

// LEA 50 — Braingeyser
pub(in crate::card::sets) static BRAINGEYSER: CardRecord = CardRecord::new_with_legacy_id(
    45,
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

// LEA 51 — Clone
pub(in crate::card::sets) static CLONE: CardRecord = CardRecord::new_with_legacy_id(
    327,
    "Clone",
    CardArt::new("f00d33dd-4eb2-4446-9813-1923d8e2d2f3", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Shapeshifter"], 0, 0).with_abilities(&[
        AbilityDef::replacement(
            "You may have this creature enter as a copy of any creature on the battlefield.",
            ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                added_types: CardTypeSet::empty(),
                retain_printed_subtypes: false,
                retained_abilities: &[],
            },
        ),
    ]),
);

// LEA 52 — Control Magic
pub(in crate::card::sets) static CONTROL_MAGIC: CardRecord = CardRecord::new_with_legacy_id(
    1802,
    "Control Magic",
    CardArt::new("7b52f459-c703-4a0b-9114-ff69eec61287", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            abilities::enters_trigger(
                "You control enchanted creature.", // The printed clause is a static, and the Aura leaving is
                // what ends it either way: an Aura with nothing under it is
                // put into its owner's graveyard.
                EffectDef::GainControl {
                    object: EffectRecipientDef::AttachedPermanent,
                    duration: ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                    controller: PlayerRefDef::EffectController,
                },
            ),
        ]),
);

// LEA 53 — Copy Artifact
pub(in crate::card::sets) static COPY_ARTIFACT: CardRecord = CardRecord::new_with_legacy_id(
    111,
    "Copy Artifact",
    CardArt::new("fd5ed955-1193-4e6a-a3e2-f54c1f9bf063", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
    .with_abilities(&[AbilityDef::replacement(
        "You may have this enchantment enter as a copy of any artifact on the battlefield, except it's an enchantment in addition to its other types.",
        ReplacementEffectDef::CopyEntering {
            object: ObjectPredicateDef::HasType(CardType::Artifact),
            added_types: CardTypeSet::single(CardType::Enchantment),
            retain_printed_subtypes: false,
            retained_abilities: &[],
        },
    )]),
);

// LEA 54 — Counterspell
pub(in crate::card::sets) static COUNTERSPELL: CardRecord = CardRecord::new_with_legacy_id(
    46,
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
            placement: ZonePlacement::Top,
        },
    )]),
);

// LEA 55 — Creature Bond
pub(in crate::card::sets) static CREATURE_BOND: CardRecord = CardRecord::new_with_legacy_id(
    1811,
    "Creature Bond",
    CardArt::new("ee4bd7d1-77e5-46e5-a594-c24469e88c4c", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::triggered(
                "When enchanted creature dies, this Aura deals damage equal to that \
                 creature's toughness to the creature's controller.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                // Both halves read the creature that died, so both come from
                // last-known information: it is already in the graveyard by
                // the time this resolves.
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::TriggeringObject,
                    )),
                    amount: ValueDef::TriggeringObjectToughness,
                },
            ),
        ]),
);

// LEA 56 — Drain Power
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “Target player activates a mana ability of each land they control. Then that player loses all unspent mana and you add the mana lost this way”.
pub(in crate::card::sets) static DRAIN_POWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea3830c5-cc66-453e-9e53-0636e00ee0ee"),
    "Drain Power",
    crate::card::CardArt::new("ea3830c5-cc66-453e-9e53-0636e00ee0ee", "Douglas Shuler"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 57 — Feedback
pub(in crate::card::sets) static FEEDBACK: CardRecord = CardRecord::new_with_legacy_id(
    1569,
    "Feedback",
    CardArt::new("0eb8f591-d763-49bf-8ef9-86265aaa72f7", "Quinton Hoover"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell(
                "Enchant enchantment",
                &abilities::ENCHANT_ENCHANTMENT_TARGET,
            ),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted enchantment's controller, this Aura \
                 deals 1 damage to that player.",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEA 58 — Flight
pub(in crate::card::sets) static FLIGHT: CardRecord = CardRecord::new_with_legacy_id(
    328,
    "Flight",
    CardArt::new("67c7784b-6b79-4268-a714-895c82809aff", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            ),
        ]),
);

// LEA 59 — Invisibility
pub(in crate::card::sets) static INVISIBILITY: CardRecord = CardRecord::new_with_legacy_id(
    329,
    "Invisibility",
    CardArt::new("1858ac51-e6a7-48d7-8759-166070ca13d8", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature can't be blocked except by Walls.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
                    )),
                },
            ),
        ]),
);

// LEA 60 — Jump
pub(in crate::card::sets) static JUMP: CardRecord = CardRecord::new_with_legacy_id(
    330,
    "Jump",
    CardArt::new("cb3f4b11-ad1b-48e2-a500-787d351b0174", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 61 — Lifetap
pub(in crate::card::sets) static LIFETAP: CardRecord = CardRecord::new_with_legacy_id(
    331,
    "Lifetap",
    CardArt::new("11add837-7ee4-4104-b031-c161bce459ae", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{U}{U}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a Forest an opponent controls becomes tapped, you gain 1 life.",
        TriggerEventDef::tapped(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// LEA 62 — Lord of Atlantis
pub(in crate::card::sets) static LORD_OF_ATLANTIS: CardRecord = CardRecord::new_with_legacy_id(
    1386,
    "Lord of Atlantis",
    CardArt::new("210c4a90-fc7a-4c76-aeaa-20a005e45386", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Merfolk get +1/+1 and have islandwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Merfolk"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::landwalk(BasicLandType::Island)),
                ]),
            },
        ),
    ]),
);

// LEA 63 — Magical Hack
// Audit: partial — Text changing rewrites land type lines and intrinsic mana only, not landwalk, predicates, other rules text, or spell text.
pub(in crate::card::sets) static MAGICAL_HACK: CardRecord = CardRecord::new_with_legacy_id(
    250,
    "Magical Hack",
    CardArt::new("2bd4202c-0477-45aa-82fd-83c85d6d4bef", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(
        AbilityDef::spell_with_targets("Change the text of target spell or permanent by replacing all instances of one basic land type with another. (For example, you may change \"swampwalk\" to \"plainswalk.\" This effect lasts indefinitely.)", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield, ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )], EffectDef::ChangeTextBasicLandType {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            })
        .with_coverage(AbilityCoverageDef::partial(
            "Spell and permanent targets are supported, but only basic land types on permanent type lines and their intrinsic mana abilities are rewritten; landwalk, predicates, and other rules-text occurrences are not.",
        )),
    ),
);

// LEA 64 — Mahamoti Djinn
pub(in crate::card::sets) static MAHAMOTI_DJINN: CardRecord = CardRecord::new_with_legacy_id(
    332,
    "Mahamoti Djinn",
    CardArt::new("36204ddd-ddf7-4b44-ae3c-b4a5a41ac9cb", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Djinn"], 5, 6)
        .with_abilities(&[abilities::flying()]),
);

// LEA 65 — Mana Short
pub(in crate::card::sets) static MANA_SHORT: CardRecord = CardRecord::new_with_legacy_id(
    301,
    "Mana Short",
    CardArt::new("32dc632a-1378-4b3e-b959-1f32ae4d5652", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap all lands target player controls and that player loses all unspent mana.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::objects_controlled_by_target(
                    ObjectPredicateDef::HasType(CardType::Land),
                    TargetIndex::PRIMARY,
                ),
            },
            EffectDef::EmptyManaPool {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// LEA 66 — Merfolk of the Pearl Trident
pub(in crate::card::sets) static MERFOLK_OF_THE_PEARL_TRIDENT: CardRecord =
    CardRecord::new_with_legacy_id(
        333,
        "Merfolk of the Pearl Trident",
        CardArt::new("2b871039-6a66-4ac3-95e7-24759c1f2f92", "Jeff A. Menges"),
        CardSet::Alpha,
        CardRules::new_creature(mana_cost!("{U}"), &["Merfolk"], 1, 1),
    );

// LEA 67 — Phantasmal Forces
pub(in crate::card::sets) static PHANTASMAL_FORCES: CardRecord = CardRecord::new_with_legacy_id(
    334,
    "Phantasmal Forces",
    CardArt::new("0631c7c8-9aa5-4333-8e20-20247fc47033", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Illusion"], 4, 1).with_abilities(&[
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

// LEA 68 — Phantasmal Terrain
// Audit: metadata-only — Needs a persistent dynamic characteristic choice and predicates that consume it for “Enchanted land is the chosen type”.
pub(in crate::card::sets) static PHANTASMAL_TERRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c371aa1-1619-41e3-8364-7bc9b8cf5d14"),
    "Phantasmal Terrain",
    crate::card::CardArt::new("1c371aa1-1619-41e3-8364-7bc9b8cf5d14", "Dameon Willich"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 69 — Phantom Monster
pub(in crate::card::sets) static PHANTOM_MONSTER: CardRecord = CardRecord::new_with_legacy_id(
    335,
    "Phantom Monster",
    CardArt::new("e46d2cf5-e8d0-4fb2-b950-252d52084b63", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Illusion"], 3, 3)
        .with_abilities(&[abilities::flying()]),
);

// LEA 70 — Pirate Ship
pub(in crate::card::sets) static PIRATE_SHIP: CardRecord = CardRecord::new_with_legacy_id(
    1404,
    "Pirate Ship",
    CardArt::new("d0a7cb23-d229-43c5-addd-dcf423984b0c", "Tom Wänerstrand"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Pirate"], 4, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            EffectDef::CannotAttackUnless(&DEFENDER_CONTROLS_AN_ISLAND),
        ),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered_if(
            "When you control no Islands, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &YOU_CONTROL_NO_ISLANDS,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LEA 71 — Power Leak
// Audit: metadata-only — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted enchantment's controller, that player may pay any amount of mana. This Aura deals 2 damage to that player. Prevent X of that damage, where X…”.
pub(in crate::card::sets) static POWER_LEAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccc982b6-35b2-4e33-ace2-86cb79123e4f"),
    "Power Leak",
    crate::card::CardArt::new("ccc982b6-35b2-4e33-ace2-86cb79123e4f", "Drew Tucker"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 72 — Power Sink
// Audit: metadata-only — Needs counter-unless-X resolution whose failed-payment branch taps mana lands and empties that player's mana pool.
pub(in crate::card::sets) static POWER_SINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b342dd3-09b9-4108-bf12-a65d4cef4eb9"),
    "Power Sink",
    crate::card::CardArt::new("1b342dd3-09b9-4108-bf12-a65d4cef4eb9", "Richard Thomas"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 73 — Prodigal Sorcerer
pub(in crate::card::sets) static PRODIGAL_SORCERER: CardRecord = CardRecord::new_with_legacy_id(
    336,
    "Prodigal Sorcerer",
    CardArt::new("e4dc1103-7bf1-47f6-9006-d3ed9ccd7a6a", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard", "Sorcerer"], 1, 1)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        )]),
);

// LEA 74 — Psionic Blast
pub(in crate::card::sets) static PSIONIC_BLAST: CardRecord = CardRecord::new_with_legacy_id(
    88,
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

// LEA 75 — Psychic Venom
pub(in crate::card::sets) static PSYCHIC_VENOM: CardRecord = CardRecord::new_with_legacy_id(
    1652,
    "Psychic Venom",
    CardArt::new("f3f5b68a-6b0e-431e-89f0-ff60f17687a5", "Brian Snõddy"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &abilities::ENCHANT_LAND_TARGET),
            AbilityDef::triggered(
                "Whenever enchanted land becomes tapped, this Aura deals 2 damage to that \
                 land's controller.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// LEA 76 — Sea Serpent
pub(in crate::card::sets) static SEA_SERPENT: CardRecord = CardRecord::new_with_legacy_id(
    1405,
    "Sea Serpent",
    CardArt::new("d0b333b7-db4d-4439-b0de-60414cbf8d7b", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Serpent"], 5, 5).with_abilities(&[
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
    ]),
);

// LEA 77 — Siren's Call
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “At the beginning of the next end step, destroy all non-Wall creatures that player controls that didn't attack this turn. Ignore this effect for each creature the player didn't control…”.
pub(in crate::card::sets) static SIREN_S_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d992b336-3b6e-43e1-8662-d85664349b44"),
    "Siren's Call",
    crate::card::CardArt::new("d992b336-3b6e-43e1-8662-d85664349b44", "Anson Maddocks"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 78 — Sleight of Mind
// Audit: metadata-only — Needs copiable-value or rules-text mutation support for “Change the text of target spell or permanent by replacing all instances of one color word with another”.
pub(in crate::card::sets) static SLEIGHT_OF_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d427790c-e322-446e-8d7d-a6b48ad41a42"),
    "Sleight of Mind",
    crate::card::CardArt::new("d427790c-e322-446e-8d7d-a6b48ad41a42", "Mark Poole"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 79 — Spell Blast
// Audit: partial — A target spell's chosen X is omitted from its stack mana value.
pub(in crate::card::sets) static SPELL_BLAST: CardRecord = CardRecord::new_with_legacy_id(
    337,
    "Spell Blast",
    CardArt::new("845734da-ab03-4dbc-bb5f-96481d3b8e88", "Brian Snõddy"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{X}{U}")).with_abilities(&[
        AbilityDef::counter_target(
            "Counter target spell with mana value X. (For example, if that spell's mana cost is {3}{U}{U}, X is 5.)",
            &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::ManaValueEqualTo(
                ValueDef::ChosenX,
            )),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "A target spell's chosen X is omitted from its stack mana value.",
        )),
    ]),
);

// LEA 80 — Stasis
pub(in crate::card::sets) static STASIS: CardRecord = CardRecord::new_with_legacy_id(
    302,
    "Stasis",
    CardArt::new("1e328704-d1d9-47f4-a923-8b5c187d4dc6", "Fay Jones"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Players skip their untap steps.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {U}.",
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

// LEA 81 — Steal Artifact
pub(in crate::card::sets) static STEAL_ARTIFACT: CardRecord = CardRecord::new_with_legacy_id(
    1803,
    "Steal Artifact",
    CardArt::new("83316930-d6ad-46ce-9b40-48eea856d95b", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant artifact", &abilities::ENCHANT_ARTIFACT_TARGET),
            abilities::enters_trigger(
                "You control enchanted artifact.",
                EffectDef::GainControl {
                    object: EffectRecipientDef::AttachedPermanent,
                    duration: ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                    controller: PlayerRefDef::EffectController,
                },
            ),
        ]),
);

// LEA 82 — Thoughtlace
pub(in crate::card::sets) static THOUGHTLACE: CardRecord = CardRecord::new_with_legacy_id(
    1563,
    "Thoughtlace",
    CardArt::new("23749375-1416-47a4-9251-52f41fe2fae9", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target spell or permanent becomes blue. (Mana symbols on that permanent remain unchanged.)",
        &SPELL_OR_PERMANENT_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue])),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    )),
);

// LEA 83 — Time Walk
pub(in crate::card::sets) static TIME_WALK: CardRecord = CardRecord::new_with_legacy_id(
    55,
    "Time Walk",
    CardArt::new("e0139f60-d48e-46fb-9f5a-1e3d7558c834", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell(
        "Take an extra turn after this one.",
        EffectDef::TakeExtraTurn {
            player: EffectRecipientDef::Controller,
        },
    )]),
);

// LEA 84 — Timetwister
pub(in crate::card::sets) static TIMETWISTER: CardRecord = CardRecord::new_with_legacy_id(
    103,
    "Timetwister",
    CardArt::new("9a49dc44-616e-4bdd-8220-0bb71eccc512", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell(
        "Each player shuffles their hand and graveyard into their library, then draws seven cards. (Then put Timetwister into its owner's graveyard.)",
        abilities::shuffle_back_and_draw_seven(),
    )]),
);

// LEA 85 — Twiddle
// Audit: partial — Tap versus untap is locked while casting instead of chosen, or declined, when the spell resolves.
static TWIDDLE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Land),
    ]),
)];

pub(in crate::card::sets) static TWIDDLE: CardRecord = CardRecord::new_with_legacy_id(
    338,
    "Twiddle",
    CardArt::new("576e811f-26a3-4a7c-bd13-3b1cc3e184eb", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::choose_one_spell(
        "You may tap or untap target artifact, creature, or land.",
        &[
            AbilityDef::spell_with_targets(
                "Tap target artifact, creature, or land",
                &TWIDDLE_TARGET,
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "Tap versus untap is locked while casting instead of chosen, or declined, when the spell resolves.",
            )),
            AbilityDef::spell_with_targets(
                "Untap target artifact, creature, or land",
                &TWIDDLE_TARGET,
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "Tap versus untap is locked while casting instead of chosen, or declined, when the spell resolves.",
            )),
        ],
    )]),
);

// LEA 86 — Unsummon
pub(in crate::card::sets) static UNSUMMON: CardRecord = CardRecord::new_with_legacy_id(
    339,
    "Unsummon",
    CardArt::new("8512f2c1-6361-4b79-843f-80b6bceeeb99", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
        },
    )]),
);

// LEA 87 — Vesuvan Doppelganger
// Audit: metadata-only — Needs copiable-value or rules-text mutation support for “You may have this creature enter as a copy of any creature on the battlefield, except it doesn't copy that creature's color and it has "At the beginning of your upkeep, you may have this…”.
pub(in crate::card::sets) static VESUVAN_DOPPELGANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("768f3a05-bd06-4a23-b9f2-94f6e618fd9f"),
    "Vesuvan Doppelganger",
    crate::card::CardArt::new("768f3a05-bd06-4a23-b9f2-94f6e618fd9f", "Quinton Hoover"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 88 — Volcanic Eruption
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “Destroy X target Mountains. Volcanic Eruption deals damage to each creature and each player equal to the number of Mountains put into a graveyard this way”.
pub(in crate::card::sets) static VOLCANIC_ERUPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a80582b1-09db-45f8-b362-0e5207a5a8e6"),
    "Volcanic Eruption",
    crate::card::CardArt::new("a80582b1-09db-45f8-b362-0e5207a5a8e6", "Douglas Shuler"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 89 — Wall of Air
pub(in crate::card::sets) static WALL_OF_AIR: CardRecord = CardRecord::new_with_legacy_id(
    340,
    "Wall of Air",
    CardArt::new("da56fdf3-6a8f-4833-a5c3-197650cc4889", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Wall"], 1, 5)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// LEA 90 — Wall of Water
pub(in crate::card::sets) static WALL_OF_WATER: CardRecord = CardRecord::new_with_legacy_id(
    341,
    "Wall of Water",
    CardArt::new("41faed1a-ded8-49ee-8e2a-c60d377775d7", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Wall"], 0, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{U}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
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

// LEA 91 — Water Elemental
pub(in crate::card::sets) static WATER_ELEMENTAL: CardRecord = CardRecord::new_with_legacy_id(
    342,
    "Water Elemental",
    CardArt::new("8de940d6-98c0-46a9-b5fd-e2b0899ea19e", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Elemental"], 5, 4),
);

// LEA 92 — Animate Dead
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “When this Aura enters, if it's on the battlefield, it loses "enchant creature card in a graveyard" and gains "enchant creature put onto the battlefield with this Aura." Return enchanted…”.
pub(in crate::card::sets) static ANIMATE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8fd7861d-925f-4b4c-a4ab-60be6f43d50b"),
    "Animate Dead",
    crate::card::CardArt::new("8fd7861d-925f-4b4c-a4ab-60be6f43d50b", "Anson Maddocks"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 93 — Bad Moon
pub(in crate::card::sets) static BAD_MOON: CardRecord = CardRecord::new_with_legacy_id(
    343,
    "Bad Moon",
    CardArt::new("43572906-ea74-4411-a549-5dc401591d2a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::static_ability(
        "Black creatures get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    )]),
);

// LEA 94 — Black Knight
pub(in crate::card::sets) static BLACK_KNIGHT: CardRecord = CardRecord::new_with_legacy_id(
    62,
    "Black Knight",
    CardArt::new("c1662949-0d69-49a3-8c69-daf10717ed4e", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from_color(ManaColor::White),
    ]),
);

// LEA 95 — Bog Wraith
pub(in crate::card::sets) static BOG_WRAITH: CardRecord = CardRecord::new_with_legacy_id(
    1380,
    "Bog Wraith",
    CardArt::new("6701874e-986e-4b81-9268-90b6171e6187", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Wraith"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// LEA 96 — Contract from Below
// Audit: metadata-only — Needs ante-zone and deck-construction handling for “Discard your hand, ante the top card of your library, then draw seven cards”.
pub(in crate::card::sets) static CONTRACT_FROM_BELOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9853b0ce-4763-4877-9741-f9145a3659c6"),
    "Contract from Below",
    crate::card::CardArt::new("9853b0ce-4763-4877-9741-f9145a3659c6", "Douglas Shuler"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 97 — Cursed Land
pub(in crate::card::sets) static CURSED_LAND: CardRecord = CardRecord::new_with_legacy_id(
    1570,
    "Cursed Land",
    CardArt::new("cf5f3c61-1e54-4eea-bf82-311cfa988e6a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &abilities::ENCHANT_LAND_TARGET),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted land's controller, this Aura \
                 deals 1 damage to that player.",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEA 98 — Dark Ritual
pub(in crate::card::sets) static DARK_RITUAL: CardRecord = CardRecord::new_with_legacy_id(
    68,
    "Dark Ritual",
    CardArt::new("ebb6664d-23ca-456e-9916-afcd6f26aa7f", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell(
        "Add {B}{B}{B}.",
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(3)),
    )]),
);

// LEA 99 — Darkpact
// Audit: metadata-only — Needs an ante zone plus a permanent ownership exchange between a chosen ante card and the top card of a library.
pub(in crate::card::sets) static DARKPACT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e78db688-93a2-47f5-9aa5-9158a72cd973"),
    "Darkpact",
    crate::card::CardArt::new("e78db688-93a2-47f5-9aa5-9158a72cd973", "Quinton Hoover"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 100 — Deathgrip
pub(in crate::card::sets) static DEATHGRIP: CardRecord = CardRecord::new_with_legacy_id(
    344,
    "Deathgrip",
    CardArt::new("2371c126-f19a-472a-ba5f-3b1366274ea0", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}{B}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{B}{B}: Counter target green spell.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{B}"))],
            &[AbilityTargetDef::exactly_one_spell(
                ObjectPredicateDef::Color(ManaColor::Green),
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// LEA 101 — Deathlace
pub(in crate::card::sets) static DEATHLACE: CardRecord = CardRecord::new_with_legacy_id(
    1564,
    "Deathlace",
    CardArt::new("6ff1cefc-62cb-4525-b0c5-2b09603b4314", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target spell or permanent becomes black. (Mana symbols on that permanent remain unchanged.)",
        &SPELL_OR_PERMANENT_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Black])),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    )),
);

// LEA 102 — Demonic Attorney
// Audit: metadata-only — Needs the ante procedure and its associated deck-construction handling for “Each player antes the top card of their library”.
pub(in crate::card::sets) static DEMONIC_ATTORNEY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd891fc6-d9d6-494e-ae65-8bea8f44b575"),
    "Demonic Attorney",
    crate::card::CardArt::new("fd891fc6-d9d6-494e-ae65-8bea8f44b575", "Daniel Gelon"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 103 — Demonic Hordes
// Audit: metadata-only — Needs a persistent tap/untap restriction or event relation for “At the beginning of your upkeep, unless you pay {B}{B}{B}, tap this creature and sacrifice a land of an opponent's choice”.
pub(in crate::card::sets) static DEMONIC_HORDES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c9bb8b1-fb79-4b99-ba09-c6e6c860de50"),
    "Demonic Hordes",
    crate::card::CardArt::new("6c9bb8b1-fb79-4b99-ba09-c6e6c860de50", "Jesper Myrfors"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 104 — Demonic Tutor
pub(in crate::card::sets) static DEMONIC_TUTOR: CardRecord = CardRecord::new_with_legacy_id(
    69,
    "Demonic Tutor",
    CardArt::new("711d4d54-5520-4de8-9b93-79902ed8e562", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell(
        "Search your library for a card, put that card into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 1,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    )]),
);

// LEA 105 — Drain Life
pub(in crate::card::sets) static DRAIN_LIFE: CardRecord = CardRecord::new_with_legacy_id(
    71,
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

// LEA 106 — Drudge Skeletons
pub(in crate::card::sets) static DRUDGE_SKELETONS: CardRecord = CardRecord::new_with_legacy_id(
    1369,
    "Drudge Skeletons",
    CardArt::new("23614289-0d73-4747-a849-5cb67cc97d6a", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton"], 1, 1).with_abilities(&[
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// LEA 107 — Evil Presence
pub(in crate::card::sets) static EVIL_PRESENCE: CardRecord = CardRecord::new_with_legacy_id(
    345,
    "Evil Presence",
    CardArt::new("0551d66e-8cd4-48f0-aa17-15f26be9d85f", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &ENCHANT_LAND_TARGET),
            AbilityDef::static_ability(
                "Enchanted land is a Swamp.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Swamp]),
                },
            ),
        ]),
);

// LEA 108 — Fear
// Audit: partial — The blocking restriction is stored directly rather than as a removable granted ability.
pub(in crate::card::sets) static FEAR: CardRecord = CardRecord::new_with_legacy_id(
    346,
    "Fear",
    CardArt::new("0cd927be-e63f-4371-a1d8-7a0489cb187e", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has fear. (It can't be blocked except by artifact creatures and/or black creatures.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Color(ManaColor::Black),
                        ])),
                    )),
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The blocking restriction is stored directly rather than as a removable granted ability.",
            )),
        ]),
);

// LEA 109 — Frozen Shade
pub(in crate::card::sets) static FROZEN_SHADE: CardRecord = CardRecord::new_with_legacy_id(
    347,
    "Frozen Shade",
    CardArt::new("d0bd76c8-4cff-4c15-9686-7a299b589814", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Shade"], 0, 1).with_abilities(&[
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
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

// LEA 110 — Gloom
/// The second clause names white enchantments specifically, not every white
/// permanent: a Circle of Protection is the target, a White Knight is not.
static GLOOM_WHITE_ENCHANTMENT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Color(ManaColor::White),
    ObjectPredicateDef::HasType(CardType::Enchantment),
]);

pub(in crate::card::sets) static GLOOM: CardRecord = CardRecord::new_with_legacy_id(
    1845,
    "Gloom",
    CardArt::new("a8d10bc7-daeb-4c0d-9e4a-8eae8d11699f", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        // "White spells", with no "you cast": it taxes both seats, including
        // the Gloom player's own.
        AbilityDef::static_ability(
            "White spells cost {3} more to cast.",
            EffectDef::ModifyCost(CostModificationDef::SpellIncrease {
                spell: ObjectPredicateDef::Color(ManaColor::White),
                caster: PlayerRelation::Any,
                amount: mana_cost!("{3}"),
            }),
        ),
        AbilityDef::static_ability(
            "Activated abilities of white enchantments cost {3} more to activate.",
            EffectDef::ModifyCost(CostModificationDef::AbilityIncrease {
                permanent: GLOOM_WHITE_ENCHANTMENT,
                amount: mana_cost!("{3}"),
            }),
        ),
    ]),
);

// LEA 111 — Howl from Beyond
pub(in crate::card::sets) static HOWL_FROM_BEYOND: CardRecord = CardRecord::new_with_legacy_id(
    348,
    "Howl from Beyond",
    CardArt::new("67ec17e1-174b-4d07-a27f-91a333c4b2fb", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{X}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +X/+0 until end of turn.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::ChosenX,
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 112 — Hypnotic Specter
pub(in crate::card::sets) static HYPNOTIC_SPECTER: CardRecord = CardRecord::new_with_legacy_id(
    76,
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
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        ),
    ]),
);

// LEA 113 — Lich
// Audit: metadata-only — Needs damage-history/source tracking or card-specific damage processing for “Whenever you're dealt damage, sacrifice that many nontoken permanents. If you can't, you lose the game”.
pub(in crate::card::sets) static LICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4250caec-0e37-41be-9ec4-8938deb5f0d0"),
    "Lich",
    crate::card::CardArt::new("4250caec-0e37-41be-9ec4-8938deb5f0d0", "Daniel Gelon"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 114 — Lord of the Pit
// Audit: metadata-only — Needs a mandatory creature-sacrifice choice with an explicit no-legal-sacrifice damage branch during upkeep.
pub(in crate::card::sets) static LORD_OF_THE_PIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2926777a-4f6e-4965-ba83-22cf7df02602"),
    "Lord of the Pit",
    crate::card::CardArt::new("2926777a-4f6e-4965-ba83-22cf7df02602", "Mark Tedin"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 115 — Mind Twist
static TARGET_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

pub(in crate::card::sets) static MIND_TWIST: CardRecord = CardRecord::new_with_legacy_id(
    82,
    "Mind Twist",
    CardArt::new("eee9e106-a248-49d2-b8c8-6bbcd56ce739", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player discards X cards at random.",
        &TARGET_PLAYER,
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
            selection: DiscardSelectionDef::Random,
            then: None,
        },
    )]),
);

// LEA 116 — Nether Shadow
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “At the beginning of your upkeep, if this card is in your graveyard with three or more creature cards above it, you may put this card onto the battlefield”.
pub(in crate::card::sets) static NETHER_SHADOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f13ad58a-6f9b-420a-bac1-40929f5e616a"),
    "Nether Shadow",
    crate::card::CardArt::new("f13ad58a-6f9b-420a-bac1-40929f5e616a", "Christopher Rush"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 117 — Nettling Imp
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “{T}: Choose target non-Wall creature the active player has controlled continuously since the beginning of the turn. That creature attacks this turn if able. Destroy it at the beginning…”.
pub(in crate::card::sets) static NETTLING_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8105973c-a94d-444c-ba20-ab0fa978bee8"),
    "Nettling Imp",
    crate::card::CardArt::new("8105973c-a94d-444c-ba20-ab0fa978bee8", "Quinton Hoover"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 118 — Nightmare
// Audit: metadata-only — Dynamic power/toughness effects are battlefield-only and cannot implement a characteristic-defining ability in every zone.
pub(in crate::card::sets) static NIGHTMARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8cdd6a7-f772-4ccb-914f-63f52ed54d6b"),
    "Nightmare",
    crate::card::CardArt::new("b8cdd6a7-f772-4ccb-914f-63f52ed54d6b", "Melissa A. Benson"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 119 — Paralyze
static PARALYZE_UNTAP: EffectDef = EffectDef::Untap {
    object: EffectRecipientDef::AttachedPermanent,
};

pub(in crate::card::sets) static PARALYZE: CardRecord = CardRecord::new_with_legacy_id(
    1841,
    "Paralyze",
    CardArt::new("be33a155-de26-43d1-88f1-c926f1b7cb7c", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
            // The host's controller pays, not the Aura's: this is on an
            // opponent's creature every time it is played, so the payer is
            // the player whose upkeep the trigger fired in.
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, that player \
                 may pay {4}. If the player does, untap the creature.",
                EffectDef::PayOr(PayOrDef::optional(
                    EffectPaymentDef::mana(
                        PlayerSetDef::One(PlayerRefDef::EventPlayer),
                        mana_cost!("{4}"),
                    ),
                    &PARALYZE_UNTAP,
                )),
            ),
        ]),
);

// LEA 120 — Pestilence
static PESTILENCE_NO_CREATURES: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasType(CardType::Creature),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    comparison: ComparisonDef::Equal,
    amount: 0,
};

pub(in crate::card::sets) static PESTILENCE: CardRecord = CardRecord::new_with_legacy_id(
    349,
    "Pestilence",
    CardArt::new("d42a6350-b16b-4e10-a273-e6cbb55dcb7a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of the end step, if no creatures are on the battlefield, sacrifice this enchantment.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &PESTILENCE_NO_CREATURES,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated(
            "{B}: This enchantment deals 1 damage to each creature and each player.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Any),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// LEA 121 — Plague Rats
// Audit: partial — Its power and toughness are a battlefield-only continuous effect rather than a characteristic-defining ability, so they read as printed in every other zone.
/// Every Plague Rats counts every other, whoever controls them, which is why
/// this query is name-based rather than controller-based.
static CREATURES_NAMED_LIKE_THE_SOURCE: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::SharesNameWithSource,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static PLAGUE_RATS: CardRecord = CardRecord::new_with_legacy_id(
    1466,
    "Plague Rats",
    CardArt::new("b3724e40-0622-4aee-9334-6c9fff88bcd5", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Rat"], 0, 0)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Plague Rats's power and toughness are each equal to the number of creatures named Plague Rats on the battlefield.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURES_NAMED_LIKE_THE_SOURCE), ValueDef::CountMatchingObjects(&CREATURES_NAMED_LIKE_THE_SOURCE)),
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "A characteristic-defining ability sets power and toughness in every zone. This \
                 is a battlefield-only continuous effect, so the value is right wherever the \
                 card is played and absent for anything reading it in another zone.",
            )),
        ]),
);

// LEA 122 — Raise Dead
pub(in crate::card::sets) static RAISE_DEAD: CardRecord = CardRecord::new_with_legacy_id(
    350,
    "Raise Dead",
    CardArt::new("ce07bede-2219-427c-a61a-56518751de42", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand.",
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
    )]),
);

// LEA 123 — Royal Assassin
static TAPPED_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Tapped,
    ]),
)];

pub(in crate::card::sets) static ROYAL_ASSASSIN: CardRecord = CardRecord::new_with_legacy_id(
    1427,
    "Royal Assassin",
    CardArt::new("59590768-fa96-4869-8763-9d5ab6ac22ad", "Tom Wänerstrand"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Assassin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Destroy target tapped creature.",
            &[AbilityCostDef::TapSource],
            &TAPPED_CREATURE_TARGET,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// LEA 124 — Sacrifice
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “Add an amount of {B} equal to the sacrificed creature's mana value”.
pub(in crate::card::sets) static SACRIFICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12164aee-6a27-4246-8d15-2d6dd20d92e9"),
    "Sacrifice",
    crate::card::CardArt::new("12164aee-6a27-4246-8d15-2d6dd20d92e9", "Dan Frazier"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 125 — Scathe Zombies
pub(in crate::card::sets) static SCATHE_ZOMBIES: CardRecord = CardRecord::new_with_legacy_id(
    351,
    "Scathe Zombies",
    CardArt::new("e9be6dcf-5e25-4b8c-9cd0-badf3771f81e", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 2),
);

// LEA 126 — Scavenging Ghoul
pub(in crate::card::sets) static SCAVENGING_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    1839,
    "Scavenging Ghoul",
    CardArt::new("426984e0-88e1-4a2d-9a1c-798b95864df3", "Jeff A. Menges"),
    CardSet::Alpha,
    // The same bank as Khabál Ghoul, spent rather than kept: the counters are
    // regenerations rather than size, so they leave one at a time.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of each end step, put a corpse counter on this creature for each \
             creature that died this turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::Corpse,
                amount: ValueDef::CreaturesDiedThisTurn,
            },
        ),
        AbilityDef::activated(
            "Remove a corpse counter from this creature: Regenerate this creature.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::Corpse,
                amount: 1,
            }],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LEA 127 — Sengir Vampire
pub(in crate::card::sets) static SENGIR_VAMPIRE: CardRecord = CardRecord::new_with_legacy_id(
    95,
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
            TriggerEventDef::ZoneChanged(
                crate::ZoneChangeEventMatcherDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                )
                .previously_damaged_by(ObjectRefDef::Source),
            ),
            EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::Source,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// LEA 128 — Simulacrum
/// Both halves read the same running total, so the life gained and the damage
/// dealt always agree.
static DAMAGE_DEALT_TO_YOU_THIS_TURN: ValueDef = ValueDef::DamageTakenThisTurn {
    player: PlayerRelation::You,
    source: None,
};

static SIMULACRUM_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    },
)];

pub(in crate::card::sets) static SIMULACRUM: CardRecord = CardRecord::new_with_legacy_id(
    1714,
    "Simulacrum",
    CardArt::new("35c3a78d-cc79-4187-929a-8aa1d1469990", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell_with_targets(
        "You gain life equal to the damage dealt to you this turn. Simulacrum deals damage to \
         target creature you control equal to the damage dealt to you this turn.",
        &SIMULACRUM_TARGET,
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: DAMAGE_DEALT_TO_YOU_THIS_TURN,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: DAMAGE_DEALT_TO_YOU_THIS_TURN,
            },
        ]),
    )),
);

// LEA 129 — Sinkhole
pub(in crate::card::sets) static SINKHOLE: CardRecord = CardRecord::new_with_legacy_id(
    96,
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

// LEA 130 — Terror
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

pub(in crate::card::sets) static TERROR: CardRecord = CardRecord::new_with_legacy_id(
    100,
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

// LEA 131 — Unholy Strength
pub(in crate::card::sets) static UNHOLY_STRENGTH: CardRecord = CardRecord::new_with_legacy_id(
    352,
    "Unholy Strength",
    CardArt::new("90563f90-0127-4164-b43b-f0321dc63a1d", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// LEA 132 — Wall of Bone
pub(in crate::card::sets) static WALL_OF_BONE: CardRecord = CardRecord::new_with_legacy_id(
    1370,
    "Wall of Bone",
    CardArt::new("ae20d442-a544-4a03-9ebf-5ecb137c67dd", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Skeleton", "Wall"], 1, 4).with_abilities(&[
        abilities::defender(),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// LEA 133 — Warp Artifact
pub(in crate::card::sets) static WARP_ARTIFACT: CardRecord = CardRecord::new_with_legacy_id(
    1571,
    "Warp Artifact",
    CardArt::new("9e5e07a2-fbdf-4c4c-996a-fce40bab5de5", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant artifact", &abilities::ENCHANT_ARTIFACT_TARGET),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted artifact's controller, this Aura \
                 deals 1 damage to that player.",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEA 134 — Weakness
pub(in crate::card::sets) static WEAKNESS: CardRecord = CardRecord::new_with_legacy_id(
    353,
    "Weakness",
    CardArt::new("36ca06a1-9b9a-49a2-9c47-9b72228621bc", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
        ]),
);

// LEA 135 — Will-o'-the-Wisp
pub(in crate::card::sets) static WILL_O_THE_WISP: CardRecord = CardRecord::new_with_legacy_id(
    1371,
    "Will-o'-the-Wisp",
    CardArt::new("a1a6f8e9-7bc1-4151-b55f-acf877b1a7a6", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{B}"), &["Spirit"], 0, 1).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// LEA 136 — Word of Command
// Audit: metadata-only — Needs ordered-library inspection, selection, and visibility handling for “Look at target opponent's hand and choose a card from it. You control that player until Word of Command finishes resolving. The player plays that card if able. While doing so, the player…”.
pub(in crate::card::sets) static WORD_OF_COMMAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96c21429-98d3-416b-be00-6aa9c4c5a006"),
    "Word of Command",
    crate::card::CardArt::new("96c21429-98d3-416b-be00-6aa9c4c5a006", "Jesper Myrfors"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 137 — Zombie Master
static OTHER_ZOMBIES: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Subtype("Zombie"),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

static ZOMBIE_REGENERATION: AbilityDef = AbilityDef::activated(
    "{B}: Regenerate this permanent.",
    &[AbilityCostDef::Mana(mana_cost!("{B}"))],
    EffectDef::Regenerate {
        object: EffectRecipientDef::Source,
    },
);

pub(in crate::card::sets) static ZOMBIE_MASTER: CardRecord = CardRecord::new_with_legacy_id(
    1426,
    "Zombie Master",
    CardArt::new("3d4255a0-d445-4c00-b936-bbf07851e1c8", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Zombie"], 2, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Other Zombie creatures have swampwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    OTHER_ZOMBIES,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::landwalk(BasicLandType::Swamp)),
            },
        ),
        AbilityDef::static_ability(
            "Other Zombies have \"{B}: Regenerate this permanent.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    OTHER_ZOMBIES,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&ZOMBIE_REGENERATION),
            },
        ),
    ]),
);

// LEA 138 — Burrowing
pub(in crate::card::sets) static BURROWING: CardRecord = CardRecord::new_with_legacy_id(
    460,
    "Burrowing",
    CardArt::new("a14c05e4-8df3-450b-8a98-5028e73b14c1", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has mountainwalk. (It can't be blocked as long as defending player controls a Mountain.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                },
            ),
        ]),
);

// LEA 139 — Chaoslace
pub(in crate::card::sets) static CHAOSLACE: CardRecord = CardRecord::new_with_legacy_id(
    1565,
    "Chaoslace",
    CardArt::new("72ea2048-57bc-43d5-8987-33ca727f1a97", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target spell or permanent becomes red. (Its mana symbols remain unchanged.)",
        &SPELL_OR_PERMANENT_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    )),
);

// LEA 140 — Disintegrate
// Audit: metadata-only — Needs a duration-scoped prohibition on creating or applying regeneration shields for “Disintegrate deals X damage to any target. If it's a creature, it can't be regenerated this turn, and if it would die this turn, exile it instead”.
pub(in crate::card::sets) static DISINTEGRATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8712c49e-f171-4669-bed9-87575a37af11"),
    "Disintegrate",
    crate::card::CardArt::new("8712c49e-f171-4669-bed9-87575a37af11", "Anson Maddocks"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 141 — Dragon Whelp
/// The fourth activation is the one that kills it, and the count includes
/// the activation now resolving.
static DRAGON_WHELP_PUMP: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(0),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceActivationsThisTurn {
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 4,
        },
        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
            "At the beginning of the next end step, sacrifice this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ))),
    },
];

pub(in crate::card::sets) static DRAGON_WHELP: CardRecord = CardRecord::new_with_legacy_id(
    23,
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

// LEA 142 — Dwarven Demolition Team
pub(in crate::card::sets) static DWARVEN_DEMOLITION_TEAM: CardRecord =
    CardRecord::new_with_legacy_id(
        461,
        "Dwarven Demolition Team",
        CardArt::new("03482c9c-1f25-4d73-9243-17462ea37ac4", "Kev Brockschmidt"),
        CardSet::Alpha,
        CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf"], 1, 1).with_abilities(&[
            AbilityDef::activated_with_targets(
                "{T}: Destroy target Wall.",
                &[AbilityCostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype("Wall"),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
            ),
        ]),
    );

// LEA 143 — Dwarven Warriors
pub(in crate::card::sets) static DWARVEN_WARRIORS: CardRecord = CardRecord::new_with_legacy_id(
    462,
    "Dwarven Warriors",
    CardArt::new("2d4d87a3-5f8b-4152-9a8b-538ab49d62e8", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf", "Warrior"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target creature with power 2 or less can't be blocked this turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEA 144 — Earth Elemental
pub(in crate::card::sets) static EARTH_ELEMENTAL: CardRecord = CardRecord::new_with_legacy_id(
    463,
    "Earth Elemental",
    CardArt::new("b24b5864-44c0-4bc8-8705-9504f83b2c03", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental"], 4, 5),
);

// LEA 145 — Earthbind
// Audit: metadata-only — Needs an Aura-entry condition on the attached creature plus a persistent removal of flying created during resolution.
pub(in crate::card::sets) static EARTHBIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6d492b7-b0b3-420e-8d00-6dacb11de77e"),
    "Earthbind",
    crate::card::CardArt::new("a6d492b7-b0b3-420e-8d00-6dacb11de77e", "Quinton Hoover"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 146 — Earthquake
pub(in crate::card::sets) static EARTHQUAKE: CardRecord = CardRecord::new_with_legacy_id(
    72,
    "Earthquake",
    CardArt::new("e68ac362-6cdc-48a6-bdd3-4f8ea32add64", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[AbilityDef::spell(
        "Earthquake deals X damage to each creature without flying and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
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
                amount: ValueDef::ChosenX,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::ChosenX,
            },
        ]),
    )]),
);

// LEA 147 — False Orders
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “Remove target creature defending player controls from combat. Creatures it was blocking that had become blocked by only that creature this combat become unblocked. You may have it block…”.
pub(in crate::card::sets) static FALSE_ORDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7eb71ac4-796d-4011-9002-1129bc09c284"),
    "False Orders",
    crate::card::CardArt::new("7eb71ac4-796d-4011-9002-1129bc09c284", "Anson Maddocks"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 148 — Fire Elemental
pub(in crate::card::sets) static FIRE_ELEMENTAL: CardRecord = CardRecord::new_with_legacy_id(
    464,
    "Fire Elemental",
    CardArt::new("da237992-2919-4e37-8f56-2164095f59b5", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental"], 5, 4),
);

// LEA 149 — Fireball
// Audit: custom — Needs declarative variable target count, per-extra-target casting cost, and damage division frozen from the cast target count.
pub(in crate::card::sets) static FIREBALL: CardRecord = CardRecord::new_with_legacy_id(
    9,
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

// LEA 150 — Firebreathing
pub(in crate::card::sets) static FIREBREATHING: CardRecord = CardRecord::new_with_legacy_id(
    465,
    "Firebreathing",
    CardArt::new("3eb27381-505d-4e47-bf66-9e7ba91a5075", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/+0 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// LEA 151 — Flashfires
pub(in crate::card::sets) static FLASHFIRES: CardRecord = CardRecord::new_with_legacy_id(
    466,
    "Flashfires",
    CardArt::new("ee8a05a4-0ce3-4abe-bb60-08af53cf08e5", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell(
        "Destroy all Plains.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )]),
);

// LEA 152 — Fork
// Audit: partial — Copy retargeting is offered as one ordered decision instead of independent choices for each target slot.
pub(in crate::card::sets) static FORK: CardRecord = CardRecord::new_with_legacy_id(
    10,
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

// LEA 153 — Goblin Balloon Brigade
pub(in crate::card::sets) static GOBLIN_BALLOON_BRIGADE: CardRecord =
    CardRecord::new_with_legacy_id(
        24,
        "Goblin Balloon Brigade",
        CardArt::new("5129b422-7a35-4bc5-b14b-c814012a0d8f", "Andi Rusu"),
        CardSet::Alpha,
        CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
            AbilityDef::activated(
                "{R}: This creature gains flying until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
    );

// LEA 154 — Goblin King
pub(in crate::card::sets) static GOBLIN_KING: CardRecord = CardRecord::new_with_legacy_id(
    27,
    "Goblin King",
    CardArt::new("5873672d-37ea-4c0f-97f3-12b74fde112d", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Goblins get +1/+1 and have mountainwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Goblin"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                ]),
            },
        ),
    ]),
);

// LEA 155 — Granite Gargoyle
pub(in crate::card::sets) static GRANITE_GARGOYLE: CardRecord = CardRecord::new_with_legacy_id(
    29,
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEA 156 — Gray Ogre
pub(in crate::card::sets) static GRAY_OGRE: CardRecord = CardRecord::new_with_legacy_id(
    467,
    "Gray Ogre",
    CardArt::new("73ae5276-b607-4f23-a9d2-e8cc7b8e3693", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ogre"], 2, 2),
);

// LEA 157 — Hill Giant
pub(in crate::card::sets) static HILL_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    468,
    "Hill Giant",
    CardArt::new("0ddb98e8-13fe-4786-83f7-b72c56db135a", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Giant"], 3, 3),
);

// LEA 158 — Hurloon Minotaur
pub(in crate::card::sets) static HURLOON_MINOTAUR: CardRecord = CardRecord::new_with_legacy_id(
    469,
    "Hurloon Minotaur",
    CardArt::new("78a9088f-8755-47cb-aa93-51d992ccab90", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Minotaur"], 2, 3),
);

// LEA 159 — Ironclaw Orcs
pub(in crate::card::sets) static IRONCLAW_ORCS: CardRecord = CardRecord::new_with_legacy_id(
    30,
    "Ironclaw Orcs",
    CardArt::new("d56421a8-34ae-4033-943f-c59a7bf2b6f9", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Orc"], 2, 2).with_ability(
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

// LEA 160 — Keldon Warlord
// Audit: partial — Its power and toughness are a battlefield-only continuous effect rather than a characteristic-defining ability, so they read as printed in every other zone.
static NON_WALL_CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static KELDON_WARLORD: CardRecord = CardRecord::new_with_legacy_id(
    1467,
    "Keldon Warlord",
    CardArt::new("8fe3fd83-969c-4add-888f-86f4306b067c", "Kev Brockschmidt"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Barbarian"], 0, 0)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Keldon Warlord's power and toughness are each equal to the number of non-Wall creatures you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&NON_WALL_CREATURES_YOU_CONTROL), ValueDef::CountMatchingObjects(&NON_WALL_CREATURES_YOU_CONTROL)),
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "A characteristic-defining ability sets power and toughness in every zone. This \
                 is a battlefield-only continuous effect, so the value is right wherever the \
                 card is played and absent for anything reading it in another zone.",
            )),
        ]),
);

// LEA 161 — Lightning Bolt
pub(in crate::card::sets) static LIGHTNING_BOLT: CardRecord = CardRecord::new_with_legacy_id(
    13,
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

// LEA 162 — Mana Flare
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “Whenever a player taps a land for mana, that player adds one mana of any type that land produced”.
pub(in crate::card::sets) static MANA_FLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fb99a26-beeb-4aca-bb02-b2d2ce0595f9"),
    "Mana Flare",
    crate::card::CardArt::new("7fb99a26-beeb-4aca-bb02-b2d2ce0595f9", "Christopher Rush"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 163 — Manabarbs
pub(in crate::card::sets) static MANABARBS: CardRecord = CardRecord::new_with_legacy_id(
    470,
    "Manabarbs",
    CardArt::new("6121f72f-680f-4bb4-ae4d-37ee4ebed4d8", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player taps a land for mana, this enchantment deals 1 damage to that player.",
        TriggerEventDef::tapped_for_mana(ObjectPredicateDef::HasType(CardType::Land)),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// LEA 164 — Mons's Goblin Raiders
pub(in crate::card::sets) static MONSS_GOBLIN_RAIDERS: CardRecord = CardRecord::new_with_legacy_id(
    471,
    "Mons's Goblin Raiders",
    CardArt::new("b4eb3db3-6a7c-488a-9433-d5d1d3133816", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1),
);

// LEA 165 — Orcish Artillery
pub(in crate::card::sets) static ORCISH_ARTILLERY: CardRecord = CardRecord::new_with_legacy_id(
    472,
    "Orcish Artillery",
    CardArt::new("a97208b1-a91b-4129-8a00-2f97b418accc", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Orc", "Warrior"], 1, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 2 damage to any target and 3 damage to you.",
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
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ]),
        ),
    ]),
);

// LEA 166 — Orcish Oriflamme
pub(in crate::card::sets) static ORCISH_ORIFLAMME: CardRecord = CardRecord::new_with_legacy_id(
    473,
    "Orcish Oriflamme",
    CardArt::new("911538ea-322c-4c40-a9c3-35e47fe60fce", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures you control get +1/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
        },
    )]),
);

// LEA 167 — Power Surge
// Audit: metadata-only — Needs damage-history/source tracking or card-specific damage processing for “At the beginning of each player's upkeep, this enchantment deals X damage to that player, where X is the number of untapped lands they controlled at the beginning of this turn”.
pub(in crate::card::sets) static POWER_SURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62858604-ca5a-4f69-a045-a7515ebfabf2"),
    "Power Surge",
    crate::card::CardArt::new("62858604-ca5a-4f69-a045-a7515ebfabf2", "Douglas Shuler"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 168 — Raging River
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “Whenever one or more creatures you control attack, each defending player divides all creatures without flying they control into a "left" pile and a "right" pile. Then, for each attacking…”.
pub(in crate::card::sets) static RAGING_RIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61e4f56d-1f4f-49f2-8534-0d09196a3327"),
    "Raging River",
    crate::card::CardArt::new("61e4f56d-1f4f-49f2-8534-0d09196a3327", "Sandra Everingham"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 169 — Red Elemental Blast
pub(in crate::card::sets) static RED_ELEMENTAL_BLAST: CardRecord = CardRecord::new_with_legacy_id(
    15,
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

// LEA 170 — Roc of Kher Ridges
pub(in crate::card::sets) static ROC_OF_KHER_RIDGES: CardRecord = CardRecord::new_with_legacy_id(
    474,
    "Roc of Kher Ridges",
    CardArt::new("731a4b86-c213-4d8e-bf01-0a0e8cff0ff1", "Andi Rusu"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Bird"], 3, 3)
        .with_abilities(&[abilities::flying()]),
);

// The chosen presentation art is its Beta printing; the definition debuted in Alpha.
// LEA 171 — Rock Hydra
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “For each 1 damage that would be dealt to this creature, if it has a +1/+1 counter on it, remove a +1/+1 counter from it and prevent that 1 damage”.
pub(in crate::card::sets) static ROCK_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("410ac9e6-fbc1-4cc8-84db-84e2eb1bab97"),
    "Rock Hydra",
    crate::card::CardArt::new("410ac9e6-fbc1-4cc8-84db-84e2eb1bab97", "Jeff A. Menges"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 172 — Sedge Troll
pub(in crate::card::sets) static SEDGE_TROLL: CardRecord = CardRecord::new_with_legacy_id(
    123,
    "Sedge Troll",
    CardArt::new("02ec317b-52a6-4490-80e5-a56826b06771", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Troll"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Swamp.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::controls_basic_land_type(
                    PlayerRelation::You,
                    BasicLandType::Swamp,
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
)
.with_identity_anchor(PrintingAnchor::scryfall(
    "b13bf496-f3c0-4c13-8282-e7abfab6a198",
));

// LEA 173 — Shatter
pub(in crate::card::sets) static SHATTER: CardRecord = CardRecord::new_with_legacy_id(
    16,
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

// LEA 174 — Shivan Dragon
pub(in crate::card::sets) static SHIVAN_DRAGON: CardRecord = CardRecord::new_with_legacy_id(
    475,
    "Shivan Dragon",
    CardArt::new("fefbf149-f988-4f8b-9f53-56f5878116a6", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dragon"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
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
    ]),
);

// LEA 175 — Smoke
pub(in crate::card::sets) static SMOKE: CardRecord = CardRecord::new_with_legacy_id(
    17,
    "Smoke",
    CardArt::new("7c67788e-d713-47c3-ab9f-b8a6212ae24f", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{R}{R}")).with_ability(AbilityDef::static_ability(
        "Players can't untap more than one creature during their untap steps.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::All),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(
                ObjectPredicateDef::HasType(CardType::Creature),
            )),
        },
    )),
);

// LEA 176 — Stone Giant
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
        effect: AppliedEffectDef::add_ability(&STONE_GIANT_FLYING),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, destroy that creature.",
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

static STONE_GIANT_FLYING: AbilityDef = abilities::flying();

pub(in crate::card::sets) static STONE_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    18,
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

// LEA 177 — Stone Rain
pub(in crate::card::sets) static STONE_RAIN: CardRecord = CardRecord::new_with_legacy_id(
    125,
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

// LEA 178 — Tunnel
pub(in crate::card::sets) static TUNNEL: CardRecord = CardRecord::new_with_legacy_id(
    476,
    "Tunnel",
    CardArt::new("b21ebc9f-a93e-4d18-b3e8-8459e3abbf31", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::destroy_target(
        "Destroy target Wall. It can't be regenerated.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Subtype("Wall")),
        false,
    )]),
);

// LEA 179 — Two-Headed Giant of Foriys
pub(in crate::card::sets) static TWO_HEADED_GIANT_OF_FORIYS: CardRecord =
    CardRecord::new_with_legacy_id(
        1771,
        "Two-Headed Giant of Foriys",
        CardArt::new("31c687dc-ee0c-4e54-a2b3-5d8e633b3245", "Anson Maddocks"),
        CardSet::Alpha,
        CardRules::new_creature(mana_cost!("{4}{R}"), &["Giant"], 4, 4).with_abilities(&[
            abilities::trample(),
            AbilityDef::static_ability(
                "This creature can block an additional creature each combat.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Source,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
                },
            ),
        ]),
    );

// LEA 180 — Uthden Troll
pub(in crate::card::sets) static UTHDEN_TROLL: CardRecord = CardRecord::new_with_legacy_id(
    1372,
    "Uthden Troll",
    CardArt::new("2ff21a6f-83a7-4bf3-a078-294e303232cc", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Troll"], 2, 2).with_abilities(&[
        abilities::regenerate_self(
            "{R}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
        ),
    ]),
);

// LEA 181 — Wall of Fire
pub(in crate::card::sets) static WALL_OF_FIRE: CardRecord = CardRecord::new_with_legacy_id(
    477,
    "Wall of Fire",
    CardArt::new("efcf12cd-fb70-444e-9641-73ffa0e8f16e", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Wall"], 0, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
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
    ]),
);

// LEA 182 — Wall of Stone
pub(in crate::card::sets) static WALL_OF_STONE: CardRecord = CardRecord::new_with_legacy_id(
    248,
    "Wall of Stone",
    CardArt::new("f7fd8b8e-98fd-4b0d-8bb9-06bd25a1e30f", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Wall"], 0, 8)
        .with_abilities(&[abilities::defender()]),
);

// LEA 183 — Wheel of Fortune
/// `Discard` saturates at the recipient's hand size. Using the largest
/// declarative amount therefore says "their hand" while retaining the shared
/// recipient-chosen discard procedure.
const ENTIRE_HAND: ValueDef = ValueDef::Constant(i32::MAX);

pub(in crate::card::sets) static WHEEL_OF_FORTUNE: CardRecord = CardRecord::new_with_legacy_id(
    40,
    "Wheel of Fortune",
    CardArt::new("67b369c4-faa8-45c8-a1b9-98f228b69682", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell(
        "Each player discards their hand, then draws seven cards.",
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ENTIRE_HAND,
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(7),
            },
        ]),
    )]),
);

// LEA 184 — Aspect of Wolf
static ASPECT_OF_WOLF_FORESTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// One count read twice, rounded opposite ways. An odd number of Forests is
/// the whole reason both halves are spelled out: five gives +2/+3.
static ASPECT_OF_WOLF_POWER: HalvedValueDef = HalvedValueDef::new(
    ValueDef::CountMatchingObjects(&ASPECT_OF_WOLF_FORESTS),
    RoundingDef::Down,
);

static ASPECT_OF_WOLF_TOUGHNESS: HalvedValueDef = HalvedValueDef::new(
    ValueDef::CountMatchingObjects(&ASPECT_OF_WOLF_FORESTS),
    RoundingDef::Up,
);

pub(in crate::card::sets) static ASPECT_OF_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    1837,
    "Aspect of Wolf",
    CardArt::new("fd9ac9e6-1395-4fbd-80e2-645f0d910c29", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +X/+Y, where X is half the number of Forests you \
                 control, rounded down, and Y is half the number of Forests you control, \
                 rounded up.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Halved(&ASPECT_OF_WOLF_POWER),
                        ValueDef::Halved(&ASPECT_OF_WOLF_TOUGHNESS),
                    ),
                },
            ),
        ]),
);

// LEA 185 — Berserk
/// The doubling reads the creature's power as Berserk resolves, and the
/// death only comes for a creature that actually attacked.
static BERSERK_EFFECT: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Composite(&BERSERK_BONUS),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next end step, destroy that creature if it attacked this turn.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::IfCondition {
            condition: &BERSERK_ATTACKED,
            then: &EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        },
    ))),
];

static BERSERK_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&BERSERK_TRAMPLE),
    AppliedEffectDef::modify_power_toughness(
        ValueDef::TargetPower(TargetIndex::PRIMARY),
        ValueDef::Constant(0),
    ),
];

static BERSERK_TRAMPLE: AbilityDef = abilities::trample();

static BERSERK_ATTACKED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::AttackedThisTurn,
};

pub(in crate::card::sets) static BERSERK: CardRecord = CardRecord::new_with_legacy_id(
    109,
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

// LEA 186 — Birds of Paradise
pub(in crate::card::sets) static BIRDS_OF_PARADISE: CardRecord = CardRecord::new_with_legacy_id(
    63,
    "Birds of Paradise",
    CardArt::new("55fe6449-1f23-43dc-adee-d144cd505b5c", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Bird"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// LEA 187 — Camouflage
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “This turn, instead of declaring blockers, each defending player chooses any number of creatures they control and divides them into a number of piles equal to the number of attacking…”.
pub(in crate::card::sets) static CAMOUFLAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3838c2a3-7fab-4976-9c1b-2891aee24e52"),
    "Camouflage",
    crate::card::CardArt::new("3838c2a3-7fab-4976-9c1b-2891aee24e52", "Jesper Myrfors"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 188 — Channel
static CHANNEL_MANA: AbilityDef = AbilityDef::activated_mana(
    "Pay 1 life: Add {C}.",
    &[AbilityCostDef::PayLife(1)],
    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
)
.with_source_zones(&[ZoneKind::Command]);

pub(in crate::card::sets) static CHANNEL: CardRecord = CardRecord::new_with_legacy_id(
    65,
    "Channel",
    CardArt::new("c1862c47-71cc-45a3-8805-a5ddc62e55ea", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{G}{G}"))
    .with_ability(AbilityDef::spell(
        "Until end of turn, any time you could activate a mana ability, you may pay 1 life. If you do, add {C}.",
        EffectDef::CreateOngoingEffect(OngoingEffectDef::unbound(
            &CHANNEL_MANA,
            ResolvedEffectDurationDef::UntilEndOfTurn,
        )),
    )),
);

// LEA 189 — Cockatrice
pub(in crate::card::sets) static COCKATRICE: CardRecord = CardRecord::new_with_legacy_id(
    1573,
    "Cockatrice",
    CardArt::new("9cd91814-6177-4a3d-a1c1-a3be7d7c7957", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Cockatrice"], 2, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a non-Wall creature, \
                 destroy that creature at end of combat.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
    ]),
);

// LEA 190 — Craw Wurm
pub(in crate::card::sets) static CRAW_WURM: CardRecord = CardRecord::new_with_legacy_id(
    478,
    "Craw Wurm",
    CardArt::new("bfed1a95-bd67-4e16-a781-81866028af2f", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Wurm"], 6, 4),
);

// LEA 191 — Elvish Archers
pub(in crate::card::sets) static ELVISH_ARCHERS: CardRecord = CardRecord::new_with_legacy_id(
    479,
    "Elvish Archers",
    CardArt::new("1cb9d405-f2b5-4e10-a405-feafd2a87d90", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Archer"], 2, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// LEA 192 — Fastbond
// Audit: metadata-only — Needs damage-history/source tracking or card-specific damage processing for “Whenever you play a land, if it wasn't the first land you played this turn, this enchantment deals 1 damage to you”.
pub(in crate::card::sets) static FASTBOND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a575a9af-e1de-4a1d-91d8-440585377e4f"),
    "Fastbond",
    crate::card::CardArt::new("a575a9af-e1de-4a1d-91d8-440585377e4f", "Mark Poole"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 193 — Fog
pub(in crate::card::sets) static FOG: CardRecord = CardRecord::new_with_legacy_id(
    1406,
    "Fog",
    CardArt::new("cfba606d-bb55-43ba-aa0c-299649958788", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::COMBAT),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEA 194 — Force of Nature
pub(in crate::card::sets) static FORCE_OF_NATURE: CardRecord = CardRecord::new_with_legacy_id(
    480,
    "Force of Nature",
    CardArt::new(
        "21551cb6-3a53-42dd-9bbd-4bc56304d6d3",
        "Douglas Shuler",
    ),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{G}{G}{G}{G}"), &["Elemental"], 8, 8)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, this creature deals 8 damage to you unless you pay {G}{G}{G}{G}.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::PayOr(PayOrDef::unless_mana(
                    mana_cost!("{G}{G}{G}{G}"),
                    &EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(8),
                    },
                )),
            ),
        ]),
);

// LEA 195 — Fungusaur
// Audit: partial — Simultaneous damage from multiple creatures produces one trigger per source instead of one trigger for the event.
pub(in crate::card::sets) static FUNGUSAUR: CardRecord = CardRecord::new_with_legacy_id(
    481,
    "Fungusaur",
    CardArt::new("5ad89f0d-b09b-40a0-84d6-3ee60dec7e23", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Fungus", "Dinosaur"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, put a +1/+1 counter on it.",
            TriggerEventDef::damage_to_source(),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Simultaneous damage from multiple creatures produces one trigger per source instead of one trigger for the event.",
        )),
    ]),
);

// LEA 196 — Gaea's Liege
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “As long as Gaea's Liege isn't attacking, its power and toughness are each equal to the number of Forests you control. As long as Gaea's Liege is attacking, its power and toughness are…”.
pub(in crate::card::sets) static GAEA_S_LIEGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2b15221-c8b0-4861-9f8b-8a65834ad499"),
    "Gaea's Liege",
    crate::card::CardArt::new("e2b15221-c8b0-4861-9f8b-8a65834ad499", "Dameon Willich"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 197 — Giant Growth
pub(in crate::card::sets) static GIANT_GROWTH: CardRecord = CardRecord::new_with_legacy_id(
    114,
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
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(3),
                ValueDef::Constant(3),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 198 — Giant Spider
pub(in crate::card::sets) static GIANT_SPIDER: CardRecord = CardRecord::new_with_legacy_id(
    482,
    "Giant Spider",
    CardArt::new("77636b4c-faea-4bf5-b88c-dd5bb88dc930", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spider"], 2, 4)
        .with_abilities(&[abilities::reach()]),
);

// LEA 199 — Grizzly Bears
pub(in crate::card::sets) static GRIZZLY_BEARS: CardRecord = CardRecord::new_with_legacy_id(
    483,
    "Grizzly Bears",
    CardArt::new("ce2d603a-3231-4a8c-bf39-1617586ea870", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2),
);

// LEA 200 — Hurricane
pub(in crate::card::sets) static HURRICANE: CardRecord = CardRecord::new_with_legacy_id(
    484,
    "Hurricane",
    CardArt::new("52f5a19f-16e4-4d35-89e1-969ac8202f88", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_abilities(&[AbilityDef::spell(
        "Hurricane deals X damage to each creature with flying and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::ChosenX,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::ChosenX,
            },
        ]),
    )]),
);

// LEA 201 — Ice Storm
pub(in crate::card::sets) static ICE_STORM: CardRecord = CardRecord::new_with_legacy_id(
    485,
    "Ice Storm",
    CardArt::new("9914836e-2fa6-4390-94b2-431427848a54", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::destroy_target(
        "Destroy target land.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land)),
        true,
    )]),
);

// LEA 202 — Instill Energy
pub(in crate::card::sets) static INSTILL_ENERGY: CardRecord = CardRecord::new_with_legacy_id(
    1826,
    "Instill Energy",
    CardArt::new("5bd38716-874c-4e3c-a315-837839a6258c", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature can attack as though it had haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackAsThoughHasty),
                },
            ),
            // Free, but rationed: the once-each-turn clause is what keeps this
            // from being an arbitrary number of blocks or an arbitrary number
            // of activations of the creature's own tap ability.
            AbilityDef::activated(
                "{0}: Untap enchanted creature. Activate only during your turn and only once \
                 each turn.",
                &[],
                EffectDef::Untap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            )
            .with_activation_timing(ActivationTimingDef::YourTurn)
            .once_each_turn(),
        ]),
);

// LEA 203 — Ironroot Treefolk
pub(in crate::card::sets) static IRONROOT_TREEFOLK: CardRecord = CardRecord::new_with_legacy_id(
    486,
    "Ironroot Treefolk",
    CardArt::new("b93c5869-7777-44bb-967a-e9439b25ced4", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Treefolk"], 3, 5),
);

// LEA 204 — Kudzu
// Audit: metadata-only — Needs the destroyed land's controller to choose, as the trigger resolves, a new land to attach this Aura to. The destruction half is available.
pub(in crate::card::sets) static KUDZU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2b72dcd-9ea1-4729-baae-ecd262fdff67"),
    "Kudzu",
    crate::card::CardArt::new("b2b72dcd-9ea1-4729-baae-ecd262fdff67", "Mark Poole"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 205 — Ley Druid
pub(in crate::card::sets) static LEY_DRUID: CardRecord = CardRecord::new_with_legacy_id(
    487,
    "Ley Druid",
    CardArt::new("f9232508-d363-4ef3-987a-741f6bff331f", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap target land.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// LEA 206 — Lifeforce
pub(in crate::card::sets) static LIFEFORCE: CardRecord = CardRecord::new_with_legacy_id(
    488,
    "Lifeforce",
    CardArt::new("e292577e-6232-44fa-a9c2-cc09949c6ed3", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{G}{G}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}: Counter target black spell.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{G}"))],
            &[AbilityTargetDef::exactly_one_spell(
                ObjectPredicateDef::Color(ManaColor::Black),
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// LEA 207 — Lifelace
pub(in crate::card::sets) static LIFELACE: CardRecord = CardRecord::new_with_legacy_id(
    1566,
    "Lifelace",
    CardArt::new("38cb601b-a35c-412e-b386-e77dad3daa54", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target spell or permanent becomes green. (Mana symbols on that permanent remain unchanged.)",
        &SPELL_OR_PERMANENT_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
            duration: ResolvedEffectDurationDef::Permanent,
        },
    )),
);

// LEA 208 — Living Artifact
static LIVING_ARTIFACT_BANKED: TriggerConditionDef = TriggerConditionDef::SourceCounters {
    kind: CounterKind::Vitality,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

/// Removing the counter and gaining the life are one choice, so they are one
/// sequence behind a single "may": taking the life without paying the counter
/// is not on offer.
static LIVING_ARTIFACT_SPEND: [EffectDef; 2] = [
    EffectDef::RemoveCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Vitality,
        amount: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
];

static LIVING_ARTIFACT_SPEND_SEQUENCE: EffectDef = EffectDef::Sequence(&LIVING_ARTIFACT_SPEND);

static LIVING_ARTIFACT_OFFER: EffectDef = EffectDef::May {
    player: EffectRecipientDef::Controller,
    effect: &LIVING_ARTIFACT_SPEND_SEQUENCE,
};

pub(in crate::card::sets) static LIVING_ARTIFACT: CardRecord = CardRecord::new_with_legacy_id(
    1825,
    "Living Artifact",
    CardArt::new("c9e753a2-a7d0-4d37-ae65-b5a1b5039a6e", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant artifact", &abilities::ENCHANT_ARTIFACT_TARGET),
            AbilityDef::triggered(
                "Whenever you're dealt damage, put that many vitality counters on this Aura.",
                TriggerEventDef::damage_to_player(ObjectPredicateDef::Any, PlayerRelation::You),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Vitality,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
            // Offered only when there is something to spend: "you may remove a
            // counter" with none banked is not a choice at all.
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, you may remove a vitality counter from this \
                 Aura. If you do, you gain 1 life.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &LIVING_ARTIFACT_BANKED,
                LIVING_ARTIFACT_OFFER,
            ),
        ]),
);

// LEA 209 — Living Lands
/// The lands keep their printed types and abilities; only the creature type
/// line and stats are added.
static LAND_CREATURE: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
];

pub(in crate::card::sets) static LIVING_LANDS: CardRecord = CardRecord::new_with_legacy_id(
    1655,
    "Living Lands",
    CardArt::new("80be0580-7948-4d8e-8c0f-5e2797ac411b", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{G}")).with_ability(AbilityDef::static_ability(
        "All Forests are 1/1 creatures that are still lands.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&LAND_CREATURE),
        },
    )),
);

// LEA 210 — Llanowar Elves
pub(in crate::card::sets) static LLANOWAR_ELVES: CardRecord = CardRecord::new_with_legacy_id(
    118,
    "Llanowar Elves",
    CardArt::new("d4f1cc9e-4f99-4c26-ac1b-8ef069fa8ceb", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

// LEA 211 — Lure
pub(in crate::card::sets) static LURE: CardRecord = CardRecord::new_with_legacy_id(
    1718,
    "Lure",
    CardArt::new("a0865e0d-5699-4545-b3ed-27071c481e41", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "All creatures able to block enchanted creature do so.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )),
                },
            ),
        ]),
);

// LEA 212 — Natural Selection
// Audit: metadata-only — Needs ordered-library inspection, selection, and visibility handling for “Look at the top three cards of target player's library, then put them back in any order. You may have that player shuffle”.
pub(in crate::card::sets) static NATURAL_SELECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8917dc8-01c0-4e72-9310-c4d501775411"),
    "Natural Selection",
    crate::card::CardArt::new("a8917dc8-01c0-4e72-9310-c4d501775411", "Mark Poole"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 213 — Regeneration
pub(in crate::card::sets) static REGENERATION: CardRecord = CardRecord::new_with_legacy_id(
    1420,
    "Regeneration",
    CardArt::new("b7b7aa34-b4f8-41b4-82ce-ab2e204c3bf4", "Quinton Hoover"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::activated(
                "{G}: Regenerate enchanted creature.",
                &[AbilityCostDef::Mana(mana_cost!("{G}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// LEA 214 — Regrowth
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

pub(in crate::card::sets) static REGROWTH: CardRecord = CardRecord::new_with_legacy_id(
    90,
    "Regrowth",
    CardArt::new("badc73ec-3728-4246-90c7-5f4eb7051ed5", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target card from your graveyard to your hand.",
        &REGROWTH_TARGET,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
        },
    )]),
);

// LEA 215 — Scryb Sprites
pub(in crate::card::sets) static SCRYB_SPRITES: CardRecord = CardRecord::new_with_legacy_id(
    124,
    "Scryb Sprites",
    CardArt::new("6d929c38-91e6-457c-937a-d1884f4bba44", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Faerie"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// LEA 216 — Shanodin Dryads
pub(in crate::card::sets) static SHANODIN_DRYADS: CardRecord = CardRecord::new_with_legacy_id(
    489,
    "Shanodin Dryads",
    CardArt::new("814cf35c-f1ad-4bf4-8c10-a5592c3b1be8", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Nymph", "Dryad"], 1, 1)
        .with_abilities(&[abilities::forestwalk()]),
);

// LEA 217 — Stream of Life
pub(in crate::card::sets) static STREAM_OF_LIFE: CardRecord = CardRecord::new_with_legacy_id(
    490,
    "Stream of Life",
    CardArt::new("aa1c4d4b-2645-4cd9-823e-3c9bb2eb48f9", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player gains X life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )]),
);

// LEA 218 — Thicket Basilisk
pub(in crate::card::sets) static THICKET_BASILISK: CardRecord = CardRecord::new_with_legacy_id(
    1574,
    "Thicket Basilisk",
    CardArt::new("e92cce01-b3bd-4307-aae5-9a7c8fa386ab", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Basilisk"], 2, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a non-Wall creature, \
                 destroy that creature at end of combat.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
            },
            abilities::destroy_triggering_object_at_end_of_combat(),
        ),
    ]),
);

// LEA 219 — Timber Wolves
pub(in crate::card::sets) static TIMBER_WOLVES: CardRecord = CardRecord::new_with_legacy_id(
    1775,
    "Timber Wolves",
    CardArt::new("bc2570a4-eef9-430d-b6c2-cd51d29b9d01", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Wolf"], 1, 1)
        .with_abilities(&[abilities::banding()]),
);

// LEA 220 — Tranquility
pub(in crate::card::sets) static TRANQUILITY: CardRecord = CardRecord::new_with_legacy_id(
    491,
    "Tranquility",
    CardArt::new("774cc5a6-3a69-4812-add4-eb5eb6389238", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Destroy all enchantments.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Enchantment),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )]),
);

// LEA 221 — Tsunami
pub(in crate::card::sets) static TSUNAMI: CardRecord = CardRecord::new_with_legacy_id(
    492,
    "Tsunami",
    CardArt::new("9ed67d61-cf47-446b-b454-eb404a8686b7", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[AbilityDef::spell(
        "Destroy all Islands.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
        },
    )]),
);

// LEA 222 — Verduran Enchantress
pub(in crate::card::sets) static VERDURAN_ENCHANTRESS: CardRecord = CardRecord::new_with_legacy_id(
    493,
    "Verduran Enchantress",
    CardArt::new("9f87178b-1221-4d7a-a7a5-20d7f01b8089", "Kev Brockschmidt"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Druid"], 0, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast an enchantment spell, you may draw a card.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// LEA 223 — Wall of Brambles
pub(in crate::card::sets) static WALL_OF_BRAMBLES: CardRecord = CardRecord::new_with_legacy_id(
    1373,
    "Wall of Brambles",
    CardArt::new("af2a4558-db6e-41b2-aff6-b164d93282a0", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Plant", "Wall"], 2, 3).with_abilities(&[
        abilities::defender(),
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
        ),
    ]),
);

// LEA 224 — Wall of Ice
pub(in crate::card::sets) static WALL_OF_ICE: CardRecord = CardRecord::new_with_legacy_id(
    494,
    "Wall of Ice",
    CardArt::new("cc743a03-867c-4bb0-8fb0-2bcaa0a8a756", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wall"], 0, 7)
        .with_abilities(&[abilities::defender()]),
);

// LEA 225 — Wall of Wood
pub(in crate::card::sets) static WALL_OF_WOOD: CardRecord = CardRecord::new_with_legacy_id(
    495,
    "Wall of Wood",
    CardArt::new("8df80424-3bd9-4982-ad79-e55d9ba3b43d", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Wall"], 0, 3)
        .with_abilities(&[abilities::defender()]),
);

// LEA 226 — Wanderlust
pub(in crate::card::sets) static WANDERLUST: CardRecord = CardRecord::new_with_legacy_id(
    1572,
    "Wanderlust",
    CardArt::new("220a03ca-8c9b-4acb-821d-f6577fbb20fb", "Cornelius Brudi"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, this Aura \
                 deals 1 damage to that player.",
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// LEA 227 — War Mammoth
pub(in crate::card::sets) static WAR_MAMMOTH: CardRecord = CardRecord::new_with_legacy_id(
    496,
    "War Mammoth",
    CardArt::new("c8d6081e-f686-4263-a0a2-21c0d9af5fdb", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elephant"], 3, 3)
        .with_abilities(&[abilities::trample()]),
);

// LEA 228 — Web
pub(in crate::card::sets) static WEB: CardRecord = CardRecord::new_with_legacy_id(
    497,
    "Web",
    CardArt::new("37c7890a-86dc-4a97-a7ce-1436fa22d0c0", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2 and has reach. (It can block creatures with flying.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(2)),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                    ]),
                },
            ),
        ]),
);

// LEA 229 — Wild Growth
pub(in crate::card::sets) static WILD_GROWTH: CardRecord = CardRecord::new_with_legacy_id(
    1793,
    "Wild Growth",
    CardArt::new("fd896dfa-66c0-4327-8e5b-489bbe350c95", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &abilities::ENCHANT_LAND_TARGET),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an \
                 additional {G}.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                // The land's controller, not the Aura's: this may be sitting
                // on something an opponent controls.
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Green).to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// LEA 230 — Ankh of Mishra
pub(in crate::card::sets) static ANKH_OF_MISHRA: CardRecord = CardRecord::new_with_legacy_id(
    1,
    "Ankh of Mishra",
    CardArt::new("f594b7aa-d44e-47c4-989b-565f881e25f1", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a land enters, this artifact deals 2 damage to that land's controller.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Land),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(2),
        },
    )]),
);

// LEA 231 — Basalt Monolith
pub(in crate::card::sets) static BASALT_MONOLITH: CardRecord = CardRecord::new_with_legacy_id(
    498,
    "Basalt Monolith",
    CardArt::new("66a74c89-6f86-4ec8-af17-391cd5026054", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
        ),
        AbilityDef::activated(
            "{3}: Untap this artifact.",
            &[AbilityCostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// LEA 232 — Black Lotus
pub(in crate::card::sets) static BLACK_LOTUS: CardRecord = CardRecord::new_with_legacy_id(
    21,
    "Black Lotus",
    CardArt::new("b0faa7f2-b547-42c4-a810-839da50dadfe", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}, Sacrifice this artifact: Add three mana of any one color.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )]),
);

// LEA 233 — Black Vise
pub(in crate::card::sets) static BLACK_VISE: CardRecord = CardRecord::new_with_legacy_id(
    4,
    "Black Vise",
    CardArt::new("76ac72f8-5b1e-4d67-a796-ef69cde27424", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::replacement(
            "As this artifact enters, choose an opponent.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(
                PlayerRelation::Opponent,
            )),
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

// LEA 234 — Celestial Prism
// Audit: metadata-only — The mana-ability runtime cannot activate a mana ability that itself has a mana payment cost.
pub(in crate::card::sets) static CELESTIAL_PRISM: CardRecord = CardRecord::new_with_legacy_id(
    499,
    "Celestial Prism",
    CardArt::new("a47417cb-1ea7-4f65-ba06-e27a99373114", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[AbilityDef::not_implemented(
        "{2}, {T}: Add one mana of any color.",
        "The mana-ability runtime cannot currently pay a mana cost while activating a mana ability.",
    )]),
);

// LEA 235 — Chaos Orb
static CHAOS_ORB_FLIP_SUCCESS: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    can_regenerate: true,
};

static CHAOS_ORB_FLIP: EffectDef = EffectDef::Randomized {
    likelihood: LikelihoodDef::new(0.9),
    on_success: &CHAOS_ORB_FLIP_SUCCESS,
    on_failure: &EffectDef::None,
};

static CHAOS_ORB_PRESENT_RESOLUTION: EffectDef = EffectDef::Sequence(&[
    CHAOS_ORB_FLIP,
    EffectDef::Destroy {
        object: EffectRecipientDef::Source,
        can_regenerate: true,
    },
]);

static CHAOS_ORB_IF_PRESENT: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceOnBattlefield,
    then: &CHAOS_ORB_PRESENT_RESOLUTION,
};

pub(in crate::card::sets) static CHAOS_ORB: CardRecord = CardRecord::new_with_legacy_id(
    22,
    "Chaos Orb",
    CardArt::new("92274971-7c4a-4326-b0fe-75e2d124f718", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_abilities(&[
            AbilityDef::activated("{1}, {T}: Choose a nontoken permanent on the battlefield. If Chaos Orb is on the battlefield, flip Chaos Orb onto the battlefield from a height of at least one foot. If Chaos Orb turns over completely at least 360 degrees during the flip, and lands resting on the chosen permanent, destroy that permanent. Then destroy Chaos Orb.", &[
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::TapSource,
                ], EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::new(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        &[ZoneKind::Battlefield],
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &CHAOS_ORB_IF_PRESENT,
                }),
            )
            .with_coverage(AbilityCoverageDef::explained_complete(
                "For reproducible headless 93/94 play, the physical flip is represented by one seeded random trial with a 0.9 success likelihood.",
            )),
        ]),
);

// LEA 236 — Clockwork Beast
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than seven. Activate only during your upkeep”.
pub(in crate::card::sets) static CLOCKWORK_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27f916a2-0ace-44b5-99dc-72979af34db9"),
    "Clockwork Beast",
    crate::card::CardArt::new("27f916a2-0ace-44b5-99dc-72979af34db9", "Drew Tucker"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 237 — Conservator
pub(in crate::card::sets) static CONSERVATOR: CardRecord = CardRecord::new_with_legacy_id(
    1438,
    "Conservator",
    CardArt::new("c7824e2a-4eff-4f72-9216-0db30a4f4252", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated(
        "{3}, {T}: Prevent the next 2 damage that would be dealt to you this turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{3}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::amount(
                DamageEventMatcherDef::to(EffectRecipientDef::Controller),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEA 238 — Copper Tablet
pub(in crate::card::sets) static COPPER_TABLET: CardRecord = CardRecord::new_with_legacy_id(
    7,
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

// LEA 239 — Crystal Rod
pub(in crate::card::sets) static CRYSTAL_ROD: CardRecord = CardRecord::new_with_legacy_id(
    500,
    "Crystal Rod",
    CardArt::new("76693233-7961-4b7e-80f2-ed90e494c4aa", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a blue spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Blue)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// LEA 240 — Cyclopean Tomb
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “{2}, {T}: Put a mire counter on target non-Swamp land. That land is a Swamp for as long as it has a mire counter on it. Activate only during your upkeep”.
pub(in crate::card::sets) static CYCLOPEAN_TOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("894c5cf2-8ae2-427a-bcbc-67df0bdfee9d"),
    "Cyclopean Tomb",
    crate::card::CardArt::new("894c5cf2-8ae2-427a-bcbc-67df0bdfee9d", "Anson Maddocks"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 241 — Dingus Egg
pub(in crate::card::sets) static DINGUS_EGG: CardRecord = CardRecord::new_with_legacy_id(
    501,
    "Dingus Egg",
    CardArt::new("65eb6cda-e512-40a8-9c1f-335b713409ff", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a land is put into a graveyard from the battlefield, this artifact deals 2 damage to that land's controller.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::HasType(CardType::Land), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(2),
        },
    )]),
);

// LEA 242 — Disrupting Scepter
pub(in crate::card::sets) static DISRUPTING_SCEPTER: CardRecord = CardRecord::new_with_legacy_id(
    1457,
    "Disrupting Scepter",
    CardArt::new("ca571ee8-07a2-43b8-9acf-89cbfd3cf7c9", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target player discards a card. Activate only during your turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
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
        .with_activation_timing(ActivationTimingDef::YourTurn),
    ),
);

// LEA 243 — Forcefield
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “{1}: The next time an unblocked creature of your choice would deal combat damage to you this turn, prevent all but 1 of that damage”.
pub(in crate::card::sets) static FORCEFIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f2004c1-8efe-407f-bf48-27b807422eea"),
    "Forcefield",
    crate::card::CardArt::new("3f2004c1-8efe-407f-bf48-27b807422eea", "Dan Frazier"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 244 — Gauntlet of Might
pub(in crate::card::sets) static GAUNTLET_OF_MIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1794,
    "Gauntlet of Might",
    CardArt::new("da248001-ed75-4b68-9532-37d3cd5afc4c", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::static_ability(
            "Red creatures get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Red),
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
        AbilityDef::triggered_mana(
            "Whenever a Mountain is tapped for mana, its controller adds an additional {R}.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Mountain,
            ])),
            // Every Mountain, so the mana follows whoever tapped one rather
            // than whoever owns the Gauntlet.
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Red).to_triggering_objects_controller(),
            ),
        ),
    ]),
);

// LEA 245 — Glasses of Urza
pub(in crate::card::sets) static GLASSES_OF_URZA: CardRecord = CardRecord::new_with_legacy_id(
    11,
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

// LEA 246 — Helm of Chatzuk
pub(in crate::card::sets) static HELM_OF_CHATZUK: CardRecord = CardRecord::new_with_legacy_id(
    1780,
    "Helm of Chatzuk",
    CardArt::new("3792c6ef-c4e6-4923-9a51-7d28fbc5c393", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}: Target creature gains banding until end of turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::banding()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// LEA 247 — Howling Mine
pub(in crate::card::sets) static HOWLING_MINE: CardRecord = CardRecord::new_with_legacy_id(
    1827,
    "Howling Mine",
    CardArt::new("51f8f6e1-a451-4262-90d3-5107caf54175", "Mark Poole"),
    CardSet::Alpha,
    // The condition is checked twice: once when the step begins, and again as
    // the trigger resolves. Tapping the Mine in response to its own trigger is
    // the standard way to deny the extra card, and that only works because the
    // "if" is an intervening-if rather than a one-time reading.
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::triggered_if(
        "At the beginning of each player's draw step, if this artifact is untapped, that \
         player draws an additional card.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Draw,
            player: PlayerRelation::Any,
        },
        &TriggerConditionDef::SourceIsUntapped,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// LEA 248 — Icy Manipulator
pub(in crate::card::sets) static ICY_MANIPULATOR: CardRecord = CardRecord::new_with_legacy_id(
    116,
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

// LEA 249 — Illusionary Mask
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “{X}: You may choose a creature card in your hand whose mana cost could be paid by some amount of, or all of, the mana you spent on {X}. If you do, you may cast that card face down as a…”.
pub(in crate::card::sets) static ILLUSIONARY_MASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62ef2f37-b8ad-47ad-89ca-d6abcb7ff21b"),
    "Illusionary Mask",
    crate::card::CardArt::new("62ef2f37-b8ad-47ad-89ca-d6abcb7ff21b", "Amy Weber"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 250 — Iron Star
pub(in crate::card::sets) static IRON_STAR: CardRecord = CardRecord::new_with_legacy_id(
    12,
    "Iron Star",
    CardArt::new("5786de12-cade-43c2-a6b0-0c5b294b9d0e", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a red spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Red)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// LEA 251 — Ivory Cup
pub(in crate::card::sets) static IVORY_CUP: CardRecord = CardRecord::new_with_legacy_id(
    502,
    "Ivory Cup",
    CardArt::new("9964d8d8-dc97-4e5f-9f52-173f7e2c37fd", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a white spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::White)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// LEA 252 — Jade Monolith
// Audit: metadata-only — Needs a shield keyed to a source chosen as the ability resolves; prevention shields attach to a recipient and spend on the next damage from any source, not from one named source for “{1}: The next time a source of your choice would deal damage to target creature this turn, that source deals that damage to you instead”.
pub(in crate::card::sets) static JADE_MONOLITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a77e0f1-449d-4a7d-9fa0-ba7598f7a73a"),
    "Jade Monolith",
    crate::card::CardArt::new("4a77e0f1-449d-4a7d-9fa0-ba7598f7a73a", "Anson Maddocks"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 253 — Jade Statue
/// The Statue keeps its artifact type, so it is an artifact creature rather
/// than a creature that used to be an artifact.
static JADE_STATUE_ANIMATION: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_card_types(
        CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
    ),
    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Golem"])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(6)),
];

pub(in crate::card::sets) static JADE_STATUE: CardRecord = CardRecord::new_with_legacy_id(
    1828,
    "Jade Statue",
    CardArt::new("8d82d94b-ceef-4533-a4f2-b6442a61b839", "Dan Frazier"),
    CardSet::Alpha,
    // Combat only, and only for that combat. The window opens at the
    // beginning of combat, which is early enough to attack, and stays open
    // past the block declaration, which is what lets a Statue that was never
    // a creature when blockers were chosen still be one when damage is dealt.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(
        AbilityDef::activated(
            "{2}: This artifact becomes a 3/6 Golem artifact creature until end of combat. \
             Activate only during combat.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&JADE_STATUE_ANIMATION),
                duration: ResolvedEffectDurationDef::UntilEndOfCombat,
            },
        )
        .with_activation_timing(ActivationTimingDef::DuringCombat),
    ),
);

// LEA 254 — Jayemdae Tome
pub(in crate::card::sets) static JAYEMDAE_TOME: CardRecord = CardRecord::new_with_legacy_id(
    51,
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

// LEA 255 — Juggernaut
pub(in crate::card::sets) static JUGGERNAUT: CardRecord = CardRecord::new_with_legacy_id(
    41,
    "Juggernaut",
    CardArt::new("dcd6a291-5282-4f49-8203-d9b416083c48", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Juggernaut"], 5, 3).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::static_ability(
            "This creature can't be blocked by Walls.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Subtype("Wall"),
                )),
            },
        ),
    ]),
);

// LEA 256 — Kormus Bell
/// Black as well, which is the only characteristic Kormus Bell repaints.
static BLACK_LAND_CREATURE: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Black])),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
];

pub(in crate::card::sets) static KORMUS_BELL: CardRecord = CardRecord::new_with_legacy_id(
    1656,
    "Kormus Bell",
    CardArt::new("3f4ef7a1-148d-44ac-89ed-0ef379cca0c6", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::static_ability(
        "All Swamps are 1/1 black creatures that are still lands.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&BLACK_LAND_CREATURE),
        },
    )),
);

// LEA 257 — Library of Leng
// Audit: metadata-only — Needs ordered-library inspection, selection, and visibility handling for “If an effect causes you to discard a card, discard it, but you may put it on top of your library instead of into your graveyard”.
pub(in crate::card::sets) static LIBRARY_OF_LENG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2340edcb-8cd5-4ccd-99e2-b9a29f72c495"),
    "Library of Leng",
    crate::card::CardArt::new("2340edcb-8cd5-4ccd-99e2-b9a29f72c495", "Daniel Gelon"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 258 — Living Wall
pub(in crate::card::sets) static LIVING_WALL: CardRecord = CardRecord::new_with_legacy_id(
    1374,
    "Living Wall",
    CardArt::new("4a98ada6-923a-44a5-bdef-ea6a160b481e", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Wall"], 0, 6).with_abilities(&[
        abilities::defender(),
        abilities::regenerate_self(
            "{1}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
        ),
    ]),
);

// LEA 259 — Mana Vault
pub(in crate::card::sets) static MANA_VAULT: CardRecord = CardRecord::new_with_legacy_id(
    42,
    "Mana Vault",
    CardArt::new("19499cb7-eccb-4e69-af32-6002d447a160", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {4}. If you do, untap this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{4}"),
                ),
                &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::triggered_if(
            "At the beginning of your draw step, if this artifact is tapped, it deals 1 damage to you.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Draw,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceIsTapped,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Colorless).with_amount(3),
            ),
        ),
    ]),
);

// LEA 260 — Meekstone
pub(in crate::card::sets) static MEEKSTONE: CardRecord = CardRecord::new_with_legacy_id(
    1677,
    "Meekstone",
    CardArt::new("13a68a17-22ee-47c9-870a-83e911862b94", "Quinton Hoover"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::static_ability(
        "Creatures with power 3 or greater don't untap during their controllers' untap steps.",
        EffectDef::StaticApply {
            // Read live, so a creature pumped past two stays tapped and one
            // shrunk below three untaps as usual.
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(3),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
        },
    )),
);

// LEA 261 — Mox Emerald
pub(in crate::card::sets) static MOX_EMERALD: CardRecord = CardRecord::new_with_legacy_id(
    32,
    "Mox Emerald",
    CardArt::new("b0e1427c-05cd-465b-be59-97ed6e39f7ba", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

// LEA 262 — Mox Jet
pub(in crate::card::sets) static MOX_JET: CardRecord = CardRecord::new_with_legacy_id(
    33,
    "Mox Jet",
    CardArt::new("92bcd1ce-19b1-4d78-8b09-95242ca08d76", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Black)]),
);

// LEA 263 — Mox Pearl
pub(in crate::card::sets) static MOX_PEARL: CardRecord = CardRecord::new_with_legacy_id(
    34,
    "Mox Pearl",
    CardArt::new("8ebe4be7-e12a-4596-a899-fbd5b152e879", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::White)]),
);

// LEA 264 — Mox Ruby
pub(in crate::card::sets) static MOX_RUBY: CardRecord = CardRecord::new_with_legacy_id(
    35,
    "Mox Ruby",
    CardArt::new("8945585f-4773-493d-a0fe-d707db910b38", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Red)]),
);

// LEA 265 — Mox Sapphire
pub(in crate::card::sets) static MOX_SAPPHIRE: CardRecord = CardRecord::new_with_legacy_id(
    36,
    "Mox Sapphire",
    CardArt::new("82da0972-b17b-4600-9efd-e9430a0db04b", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Blue)]),
);

// LEA 266 — Nevinyrral's Disk
pub(in crate::card::sets) static NEVINYRRALS_DISK: CardRecord = CardRecord::new_with_legacy_id(
    84,
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
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
            },
        ),
    ]),
);

// LEA 267 — Obsianus Golem
pub(in crate::card::sets) static OBSIANUS_GOLEM: CardRecord = CardRecord::new_with_legacy_id(
    503,
    "Obsianus Golem",
    CardArt::new("4c8e9f5c-deba-4443-bf9d-fb2be75c5418", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Golem"], 4, 6),
);

// LEA 268 — Rod of Ruin
pub(in crate::card::sets) static ROD_OF_RUIN: CardRecord = CardRecord::new_with_legacy_id(
    504,
    "Rod of Ruin",
    CardArt::new("af957200-c538-4f52-b105-6db7a7abb4dc", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}, {T}: This artifact deals 1 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
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

// LEA 269 — Sol Ring
pub(in crate::card::sets) static SOL_RING: CardRecord = CardRecord::new_with_legacy_id(
    38,
    "Sol Ring",
    CardArt::new("c4300d24-1cae-4dd5-be7e-38cc677cf5bd", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}: Add {C}{C}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
    )]),
);

// LEA 270 — Soul Net
pub(in crate::card::sets) static SOUL_NET: CardRecord = CardRecord::new_with_legacy_id(
    505,
    "Soul Net",
    CardArt::new("2b814198-814b-4619-a158-327af675f8f2", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a creature dies, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Creature),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// LEA 271 — Sunglasses of Urza
// Audit: metadata-only — Needs cost/mana provenance or dynamic payment support for “You may spend white mana as though it were red mana”.
pub(in crate::card::sets) static SUNGLASSES_OF_URZA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0d433a4-76c0-4f27-836d-4c0c13a511fb"),
    "Sunglasses of Urza",
    crate::card::CardArt::new("c0d433a4-76c0-4f27-836d-4c0c13a511fb", "Dan Frazier"),
    crate::card::CardSet::Alpha,
    crate::card::CardRules::unsupported(),
);

// LEA 272 — The Hive
pub(in crate::card::sets) static THE_HIVE: CardRecord = CardRecord::new_with_legacy_id(
    604,
    "The Hive",
    CardArt::new("544a7138-eae8-4ff9-9e17-680bfa717183", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{5}, {T}: Create a 1/1 colorless Insect artifact creature token with flying named Wasp.",
        &[
            AbilityCostDef::Mana(mana_cost!("{5}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::create_artifact_creature_token(&["Insect"], &[], 1, 1)
            .with_name("Wasp")
            .with_abilities(&[abilities::flying()])
            .with_art(CardArt::new(
                "09921372-126f-4c81-b6d8-ea50b1d0eb44",
                "Sandra Everingham",
            )),
    )),
);

// LEA 273 — Throne of Bone
pub(in crate::card::sets) static THRONE_OF_BONE: CardRecord = CardRecord::new_with_legacy_id(
    506,
    "Throne of Bone",
    CardArt::new("a2931ae0-7836-4000-b9ec-f2029ebf5d96", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a black spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Black)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);
// LEA 274 — Time Vault
static TIME_VAULT_UNTAP: EffectDef = EffectDef::Untap {
    object: EffectRecipientDef::Source,
};

static TIME_VAULT_TURN_REPLACEMENT: [ReplacementEffectDef; 2] = [
    ReplacementEffectDef::ReplaceEventWithNothing,
    ReplacementEffectDef::Perform(&TIME_VAULT_UNTAP),
];

pub(in crate::card::sets) static TIME_VAULT: CardRecord = CardRecord::new_with_legacy_id(
    102,
    "Time Vault",
    CardArt::new("902441dc-c976-4c92-b897-6376eaa0fe38", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::defined_replacement(
            "If you would begin your turn while this artifact is tapped, you may skip that turn instead. If you do, untap this artifact.",
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::WouldBeginTurn {
                    player: PlayerRelation::You,
                    kind: TurnKindDef::Any,
                })
                .with_condition(ReplacementConditionDef::SourceTapped)
                .optional(),
            ReplacementEffectDef::Sequence(&TIME_VAULT_TURN_REPLACEMENT),
        ),
        AbilityDef::activated(
            "{T}: Take an extra turn after this one.",
            &[AbilityCostDef::TapSource],
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Controller,
            },
        ),
    ]),
);

// LEA 275 — Winter Orb
/// The cap names every player, and the Orb's own condition sits outside it:
/// tapping the Orb turns the whole clause off without touching anyone's
/// lands.
static WINTER_ORB_LIMIT: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::players(PlayerSetDef::All),
    effect: AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(ObjectPredicateDef::HasType(
        CardType::Land,
    ))),
};

pub(in crate::card::sets) static WINTER_ORB: CardRecord = CardRecord::new_with_legacy_id(
    20,
    "Winter Orb",
    CardArt::new("9359f60c-9a27-4e53-b35b-964a121a6fba", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "As long as this artifact is untapped, players can't untap more than one land during \
         their untap steps.",
        EffectDef::IfCondition {
            condition: &TriggerConditionDef::SourceUntapped,
            then: &WINTER_ORB_LIMIT,
        },
    )),
);

// LEA 276 — Wooden Sphere
pub(in crate::card::sets) static WOODEN_SPHERE: CardRecord = CardRecord::new_with_legacy_id(
    507,
    "Wooden Sphere",
    CardArt::new("bcae01a2-171b-47cd-87be-f1e4e5314326", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a green spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Green)),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{1}"),
            ),
            &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )]),
);

// LEA 277 — Badlands
pub(in crate::card::sets) static BADLANDS: CardRecord = CardRecord::new_with_legacy_id(
    59,
    "Badlands",
    CardArt::new("717f6d10-9144-4ade-9ac6-a481cc66b875", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp", "Mountain"]),
);

// LEA 278 — Bayou
pub(in crate::card::sets) static BAYOU: CardRecord = CardRecord::new_with_legacy_id(
    61,
    "Bayou",
    CardArt::new("412ceddd-2b9a-4551-a6bf-ae2830a2010a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp", "Forest"]),
);

// LEA 279 — Plateau
pub(in crate::card::sets) static PLATEAU: CardRecord = CardRecord::new_with_legacy_id(
    87,
    "Plateau",
    CardArt::new("6eafa00b-c628-40f6-86eb-88e1361fc7a0", "Drew Tucker"),
    CardSet::Alpha,
    CardRules::new_land(&["Mountain", "Plains"]),
);

// LEA 280 — Savannah
pub(in crate::card::sets) static SAVANNAH: CardRecord = CardRecord::new_with_legacy_id(
    91,
    "Savannah",
    CardArt::new("94f7e24c-2546-41b6-81ad-5e920b07e64e", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Plains"]),
);

// LEA 281 — Scrubland
pub(in crate::card::sets) static SCRUBLAND: CardRecord = CardRecord::new_with_legacy_id(
    93,
    "Scrubland",
    CardArt::new("bebe39d4-21fb-46a4-a1ec-b97102e46c15", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains", "Swamp"]),
);

// LEA 282 — Taiga
pub(in crate::card::sets) static TAIGA: CardRecord = CardRecord::new_with_legacy_id(
    99,
    "Taiga",
    CardArt::new("60df6592-0b3b-4b87-aeb2-8fa94b4fb7be", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Mountain"]),
);

// LEA 283 — Tropical Island
pub(in crate::card::sets) static TROPICAL_ISLAND: CardRecord = CardRecord::new_with_legacy_id(
    104,
    "Tropical Island",
    CardArt::new("a9c6c759-aabf-44e7-ba8c-33c5df232b56", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Island"]),
);

// LEA 284 — Tundra
pub(in crate::card::sets) static TUNDRA: CardRecord = CardRecord::new_with_legacy_id(
    56,
    "Tundra",
    CardArt::new("a03e8c5b-f4ed-4fd7-ba05-db813ccc05eb", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains", "Island"]),
);

// LEA 285 — Underground Sea
pub(in crate::card::sets) static UNDERGROUND_SEA: CardRecord = CardRecord::new_with_legacy_id(
    105,
    "Underground Sea",
    CardArt::new("ff76ac86-8a8a-47fe-9388-8950ca3e26c3", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Island", "Swamp"]),
);

// LEA 286 — Plains
pub(in crate::card::sets) static PLAINS: CardRecord = CardRecord::new_with_legacy_id(
    52,
    "Plains",
    CardArt::new("b1623d57-4729-4796-b3f7-f1837a05c6ed", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains"]).with_supertype(CardSupertype::Basic),
);

// LEA 287 — Plains (alternate printing)

// LEA 288 — Island
pub(in crate::card::sets) static ISLAND: CardRecord = CardRecord::new_with_legacy_id(
    49,
    "Island",
    CardArt::new("90a57c0e-fa61-45ef-955d-d296403967d5", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_land(&["Island"]).with_supertype(CardSupertype::Basic),
);

// LEA 289 — Island (alternate printing)

// LEA 290 — Swamp
pub(in crate::card::sets) static SWAMP: CardRecord = CardRecord::new_with_legacy_id(
    97,
    "Swamp",
    CardArt::new("6176936d-72e2-4205-8871-4c5a4f1cb2d8", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp"]).with_supertype(CardSupertype::Basic),
);

// LEA 291 — Swamp (alternate printing)

// LEA 292 — Mountain
pub(in crate::card::sets) static MOUNTAIN: CardRecord = CardRecord::new_with_legacy_id(
    14,
    "Mountain",
    CardArt::new("eace2c85-976c-425e-9800-5a6ccbd91b56", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_land(&["Mountain"]).with_supertype(CardSupertype::Basic),
);

// LEA 293 — Mountain (alternate printing)

// LEA 294 — Forest
pub(in crate::card::sets) static FOREST: CardRecord = CardRecord::new_with_legacy_id(
    74,
    "Forest",
    CardArt::new("6f1c8cb0-38eb-408b-94e8-16db83999b3b", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest"]).with_supertype(CardSupertype::Basic),
);

// LEA 295 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANIMATE_WALL,
    &ARMAGEDDON,
    &BALANCE,
    &BENALISH_HERO,
    &BLACK_WARD,
    &BLAZE_OF_GLORY,
    &BLESSING,
    &BLUE_WARD,
    &CASTLE,
    &CIRCLE_OF_PROTECTION_BLUE,
    &CIRCLE_OF_PROTECTION_GREEN,
    &CIRCLE_OF_PROTECTION_RED,
    &CIRCLE_OF_PROTECTION_WHITE,
    &CONSECRATE_LAND,
    &CONVERSION,
    &CRUSADE,
    &DEATH_WARD,
    &DISENCHANT,
    &FARMSTEAD,
    &GREEN_WARD,
    &GUARDIAN_ANGEL,
    &HEALING_SALVE,
    &HOLY_ARMOR,
    &HOLY_STRENGTH,
    &ISLAND_SANCTUARY,
    &KARMA,
    &LANCE,
    &MESA_PEGASUS,
    &NORTHERN_PALADIN,
    &PEARLED_UNICORN,
    &PERSONAL_INCARNATION,
    &PURELACE,
    &RED_WARD,
    &RESURRECTION,
    &REVERSE_DAMAGE,
    &RIGHTEOUSNESS,
    &SAMITE_HEALER,
    &SAVANNAH_LIONS,
    &SERRA_ANGEL,
    &SWORDS_TO_PLOWSHARES,
    &VETERAN_BODYGUARD,
    &WALL_OF_SWORDS,
    &WHITE_KNIGHT,
    &WHITE_WARD,
    &WRATH_OF_GOD,
    &AIR_ELEMENTAL,
    &ANCESTRAL_RECALL,
    &ANIMATE_ARTIFACT,
    &BLUE_ELEMENTAL_BLAST,
    &BRAINGEYSER,
    &CLONE,
    &CONTROL_MAGIC,
    &COPY_ARTIFACT,
    &COUNTERSPELL,
    &CREATURE_BOND,
    &DRAIN_POWER,
    &FEEDBACK,
    &FLIGHT,
    &INVISIBILITY,
    &JUMP,
    &LIFETAP,
    &LORD_OF_ATLANTIS,
    &MAGICAL_HACK,
    &MAHAMOTI_DJINN,
    &MANA_SHORT,
    &MERFOLK_OF_THE_PEARL_TRIDENT,
    &PHANTASMAL_FORCES,
    &PHANTASMAL_TERRAIN,
    &PHANTOM_MONSTER,
    &PIRATE_SHIP,
    &POWER_LEAK,
    &POWER_SINK,
    &PRODIGAL_SORCERER,
    &PSIONIC_BLAST,
    &PSYCHIC_VENOM,
    &SEA_SERPENT,
    &SIREN_S_CALL,
    &SLEIGHT_OF_MIND,
    &SPELL_BLAST,
    &STASIS,
    &STEAL_ARTIFACT,
    &THOUGHTLACE,
    &TIME_WALK,
    &TIMETWISTER,
    &TWIDDLE,
    &UNSUMMON,
    &VESUVAN_DOPPELGANGER,
    &VOLCANIC_ERUPTION,
    &WALL_OF_AIR,
    &WALL_OF_WATER,
    &WATER_ELEMENTAL,
    &ANIMATE_DEAD,
    &BAD_MOON,
    &BLACK_KNIGHT,
    &BOG_WRAITH,
    &CONTRACT_FROM_BELOW,
    &CURSED_LAND,
    &DARK_RITUAL,
    &DARKPACT,
    &DEATHGRIP,
    &DEATHLACE,
    &DEMONIC_ATTORNEY,
    &DEMONIC_HORDES,
    &DEMONIC_TUTOR,
    &DRAIN_LIFE,
    &DRUDGE_SKELETONS,
    &EVIL_PRESENCE,
    &FEAR,
    &FROZEN_SHADE,
    &GLOOM,
    &HOWL_FROM_BEYOND,
    &HYPNOTIC_SPECTER,
    &LICH,
    &LORD_OF_THE_PIT,
    &MIND_TWIST,
    &NETHER_SHADOW,
    &NETTLING_IMP,
    &NIGHTMARE,
    &PARALYZE,
    &PESTILENCE,
    &PLAGUE_RATS,
    &RAISE_DEAD,
    &ROYAL_ASSASSIN,
    &SACRIFICE,
    &SCATHE_ZOMBIES,
    &SCAVENGING_GHOUL,
    &SENGIR_VAMPIRE,
    &SIMULACRUM,
    &SINKHOLE,
    &TERROR,
    &UNHOLY_STRENGTH,
    &WALL_OF_BONE,
    &WARP_ARTIFACT,
    &WEAKNESS,
    &WILL_O_THE_WISP,
    &WORD_OF_COMMAND,
    &ZOMBIE_MASTER,
    &BURROWING,
    &CHAOSLACE,
    &DISINTEGRATE,
    &DRAGON_WHELP,
    &DWARVEN_DEMOLITION_TEAM,
    &DWARVEN_WARRIORS,
    &EARTH_ELEMENTAL,
    &EARTHBIND,
    &EARTHQUAKE,
    &FALSE_ORDERS,
    &FIRE_ELEMENTAL,
    &FIREBALL,
    &FIREBREATHING,
    &FLASHFIRES,
    &FORK,
    &GOBLIN_BALLOON_BRIGADE,
    &GOBLIN_KING,
    &GRANITE_GARGOYLE,
    &GRAY_OGRE,
    &HILL_GIANT,
    &HURLOON_MINOTAUR,
    &IRONCLAW_ORCS,
    &KELDON_WARLORD,
    &LIGHTNING_BOLT,
    &MANA_FLARE,
    &MANABARBS,
    &MONSS_GOBLIN_RAIDERS,
    &ORCISH_ARTILLERY,
    &ORCISH_ORIFLAMME,
    &POWER_SURGE,
    &RAGING_RIVER,
    &RED_ELEMENTAL_BLAST,
    &ROC_OF_KHER_RIDGES,
    &ROCK_HYDRA,
    &SEDGE_TROLL,
    &SHATTER,
    &SHIVAN_DRAGON,
    &SMOKE,
    &STONE_GIANT,
    &STONE_RAIN,
    &TUNNEL,
    &TWO_HEADED_GIANT_OF_FORIYS,
    &UTHDEN_TROLL,
    &WALL_OF_FIRE,
    &WALL_OF_STONE,
    &WHEEL_OF_FORTUNE,
    &ASPECT_OF_WOLF,
    &BERSERK,
    &BIRDS_OF_PARADISE,
    &CAMOUFLAGE,
    &CHANNEL,
    &COCKATRICE,
    &CRAW_WURM,
    &ELVISH_ARCHERS,
    &FASTBOND,
    &FOG,
    &FORCE_OF_NATURE,
    &FUNGUSAUR,
    &GAEA_S_LIEGE,
    &GIANT_GROWTH,
    &GIANT_SPIDER,
    &GRIZZLY_BEARS,
    &HURRICANE,
    &ICE_STORM,
    &INSTILL_ENERGY,
    &IRONROOT_TREEFOLK,
    &KUDZU,
    &LEY_DRUID,
    &LIFEFORCE,
    &LIFELACE,
    &LIVING_ARTIFACT,
    &LIVING_LANDS,
    &LLANOWAR_ELVES,
    &LURE,
    &NATURAL_SELECTION,
    &REGENERATION,
    &REGROWTH,
    &SCRYB_SPRITES,
    &SHANODIN_DRYADS,
    &STREAM_OF_LIFE,
    &THICKET_BASILISK,
    &TIMBER_WOLVES,
    &TRANQUILITY,
    &TSUNAMI,
    &VERDURAN_ENCHANTRESS,
    &WALL_OF_BRAMBLES,
    &WALL_OF_ICE,
    &WALL_OF_WOOD,
    &WANDERLUST,
    &WAR_MAMMOTH,
    &WEB,
    &WILD_GROWTH,
    &ANKH_OF_MISHRA,
    &BASALT_MONOLITH,
    &BLACK_LOTUS,
    &BLACK_VISE,
    &CELESTIAL_PRISM,
    &CHAOS_ORB,
    &CLOCKWORK_BEAST,
    &CONSERVATOR,
    &COPPER_TABLET,
    &CRYSTAL_ROD,
    &CYCLOPEAN_TOMB,
    &DINGUS_EGG,
    &DISRUPTING_SCEPTER,
    &FORCEFIELD,
    &GAUNTLET_OF_MIGHT,
    &GLASSES_OF_URZA,
    &HELM_OF_CHATZUK,
    &HOWLING_MINE,
    &ICY_MANIPULATOR,
    &ILLUSIONARY_MASK,
    &IRON_STAR,
    &IVORY_CUP,
    &JADE_MONOLITH,
    &JADE_STATUE,
    &JAYEMDAE_TOME,
    &JUGGERNAUT,
    &KORMUS_BELL,
    &LIBRARY_OF_LENG,
    &LIVING_WALL,
    &MANA_VAULT,
    &MEEKSTONE,
    &MOX_EMERALD,
    &MOX_JET,
    &MOX_PEARL,
    &MOX_RUBY,
    &MOX_SAPPHIRE,
    &NEVINYRRALS_DISK,
    &OBSIANUS_GOLEM,
    &ROD_OF_RUIN,
    &SOL_RING,
    &SOUL_NET,
    &SUNGLASSES_OF_URZA,
    &THE_HIVE,
    &THRONE_OF_BONE,
    &TIME_VAULT,
    &WINTER_ORB,
    &WOODEN_SPHERE,
    &BADLANDS,
    &BAYOU,
    &PLATEAU,
    &SAVANNAH,
    &SCRUBLAND,
    &TAIGA,
    &TROPICAL_ISLAND,
    &TUNDRA,
    &UNDERGROUND_SEA,
    &PLAINS,
    &ISLAND,
    &SWAMP,
    &MOUNTAIN,
    &FOREST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&PLAINS, 1),   // LEA 287
    PrintingRecord::alternate(&ISLAND, 1),   // LEA 289
    PrintingRecord::alternate(&SWAMP, 1),    // LEA 291
    PrintingRecord::alternate(&MOUNTAIN, 1), // LEA 293
    PrintingRecord::alternate(&FOREST, 1),   // LEA 295
];

#[cfg(test)]
mod tests {
    use super::GOBLIN_KING;
    use crate::card::{
        AbilityOperationDef, AppliedEffectDef, CharacteristicOperationDef, DeclarativeAbilityDef,
        EffectDef, ValueDef, abilities,
    };

    #[test]
    fn goblin_king_models_one_static_effect_with_two_components() {
        let definition = GOBLIN_KING.definition();
        let clauses = definition.rules.ability_clauses();
        assert_eq!(clauses.len(), 1);
        assert!(matches!(
            clauses[0].definition,
            DeclarativeAbilityDef::Static(_)
        ));
        let Some(EffectDef::StaticApply {
            effect: AppliedEffectDef::Composite(effects),
            ..
        }) = clauses[0].declarative_effect()
        else {
            panic!("Goblin King's one static ability must apply one composite effect");
        };
        assert_eq!(effects.len(), 2);
        assert_eq!(
            effects[0],
            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1))
        );
        assert!(matches!(
            effects[1],
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability)
            )) if *ability == abilities::mountainwalk()
        ));
    }
}
