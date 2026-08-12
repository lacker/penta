use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, BasicLandType, CardArt, CardBehavior, CardRules, CardSet,
    CardSupertype, CardType, CardTypeSet, ComparisonDef, CostDef, CounterKind, DiscardSelectionDef,
    EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef, KeywordAbility,
    LikelihoodDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, PaymentDef, PlayerRelation,
    ReplacementEventDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, cards,
};
use crate::ids::{ChoiceIndex, TargetIndex};
use crate::mana_cost;

static ENCHANT_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static ENCHANT_LAND_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Land),
)];

const fn aura_spell(text: &'static str, targets: &'static [AbilityTargetDef]) -> AbilityDef {
    AbilityDef::spell_with_targets(
        text,
        targets,
        EffectDef::Attach {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )
}

// LEA 1 — Animate Wall
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Enchanted Wall can attack as though it didn't have defender”.

// LEA 2 — Armageddon
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

// LEA 3 — Balance
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

// LEA 4 — Benalish Hero
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// LEA 5 — Black Ward
// Audit: blocked — Needs the named-color protection rules and Aura self-retention exception for “Enchanted creature has protection from black. This effect doesn't remove this Aura”.

// LEA 6 — Blaze of Glory
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Target creature defending player controls can block any number of creatures this turn. It blocks each attacking creature this turn if able”.

// LEA 7 — Blessing
// Audit: partial — If the Aura leaves before resolution, the former enchanted creature is not retained through source last-known information.
pub(in crate::card::sets) static BLESSING: CardRecord = CardRecord::new(
    cards::BLESSING,
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
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "If the Aura leaves before resolution, the former enchanted creature is not retained through source last-known information.",
            )),
        ]),
);

// LEA 8 — Blue Ward
// Audit: blocked — Needs the named-color protection rules and Aura self-retention exception for “Enchanted creature has protection from blue. This effect doesn't remove this Aura”.

// LEA 9 — Castle
// Audit: blocked — Needs the clause's conditional recipient set or dynamic modifier value for “Untapped creatures you control get +0/+2”.

// LEA 10 — Circle of Protection: Blue
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}: The next time a blue source of your choice would deal damage to you this turn, prevent that damage”.

// LEA 11 — Circle of Protection: Green
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}: The next time a green source of your choice would deal damage to you this turn, prevent that damage”.

// LEA 12 — Circle of Protection: Red
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}: The next time a red source of your choice would deal damage to you this turn, prevent that damage”.

// LEA 13 — Circle of Protection: White
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}: The next time a white source of your choice would deal damage to you this turn, prevent that damage”.

// LEA 14 — Consecrate Land
// Audit: blocked — Needs this compound indestructibility and attachment-legality effect for “Enchanted land has indestructible and can't be enchanted by other Auras”.

// LEA 15 — Conversion
pub(in crate::card::sets) static CONVERSION: CardRecord = CardRecord::new(
    cards::CONVERSION,
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
            EffectDef::UnlessPaid {
                cost: mana_cost!("{W}{W}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
        AbilityDef::static_ability(
            "All Mountains are Plains.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::SetLandTypes(&[BasicLandType::Plains]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// LEA 16 — Crusade
pub(in crate::card::sets) static CRUSADE: CardRecord = CardRecord::new(
    cards::CRUSADE,
    "Crusade",
    CardArt::new("057986c7-20c0-4157-b4df-beae4ef5c66d", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}{W}")).with_abilities(&[AbilityDef::static_ability(
        "White creatures get +1/+1.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::White),
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
    )]),
);

// LEA 17 — Death Ward
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “Regenerate target creature”.

// LEA 18 — Disenchant
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

static FARMSTEAD_LAND_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, you may pay {W}{W}. If you do, you gain 1 life.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::OptionalPayment {
        payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{W}{W}"))]),
        if_paid: &EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    },
);

// LEA 19 — Farmstead
pub(in crate::card::sets) static FARMSTEAD: CardRecord = CardRecord::new(
    cards::FARMSTEAD,
    "Farmstead",
    CardArt::new("3455b006-9ea5-4aef-8ad2-d0701eb0cacf", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &ENCHANT_LAND_TARGET),
            AbilityDef::static_ability(
                "Enchanted land has \"At the beginning of your upkeep, you may pay {W}{W}. If you do, you gain 1 life.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&FARMSTEAD_LAND_ABILITY),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 20 — Green Ward
// Audit: blocked — Needs the named-color protection rules and Aura self-retention exception for “Enchanted creature has protection from green. This effect doesn't remove this Aura”.

// LEA 21 — Guardian Angel
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent the next X damage that would be dealt to any target this turn. Until end of turn, you may pay {1} any time you could cast an instant. If you do, prevent the next 1 damage that…”.

// LEA 22 — Healing Salve
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “• Prevent the next 3 damage that would be dealt to any target this turn”.

// LEA 23 — Holy Armor
// Audit: partial — If the Aura leaves before resolution, the former enchanted creature is not retained through source last-known information.
pub(in crate::card::sets) static HOLY_ARMOR: CardRecord = CardRecord::new(
    cards::HOLY_ARMOR,
    "Holy Armor",
    CardArt::new("b01041d2-687e-4972-81c8-16690809275b", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(0),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::activated(
                "{W}: Enchanted creature gets +0/+1 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(0),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "If the Aura leaves before resolution, the former enchanted creature is not retained through source last-known information.",
            )),
        ]),
);

// LEA 24 — Holy Strength
pub(in crate::card::sets) static HOLY_STRENGTH: CardRecord = CardRecord::new(
    cards::HOLY_STRENGTH,
    "Holy Strength",
    CardArt::new("e945a4cd-0eb1-4f54-898d-169ce2748a03", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+2.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

static KARMA_SWAMPS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::EventPlayer,
};

// LEA 25 — Island Sanctuary
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “If you would draw a card during your draw step, instead you may skip that draw. If you do, until your next turn, you can't be attacked except by creatures with flying and/or islandwalk”.

// LEA 26 — Karma
pub(in crate::card::sets) static KARMA: CardRecord = CardRecord::new(
    cards::KARMA,
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
pub(in crate::card::sets) static LANCE: CardRecord = CardRecord::new(
    cards::LANCE,
    "Lance",
    CardArt::new("ddb633f5-cc4d-4157-8217-def90cb15e24", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has first strike.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 28 — Mesa Pegasus
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// LEA 29 — Northern Paladin
pub(in crate::card::sets) static NORTHERN_PALADIN: CardRecord = CardRecord::new(
    cards::NORTHERN_PALADIN,
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
pub(in crate::card::sets) static PEARLED_UNICORN: CardRecord = CardRecord::new(
    cards::PEARLED_UNICORN,
    "Pearled Unicorn",
    CardArt::new("6daf1aab-1e58-4a5a-bc66-cb3f7c86e0e8", "Cornelius Brudi"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Unicorn"], 2, 2),
);

// LEA 31 — Personal Incarnation
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{0}: The next 1 damage that would be dealt to this creature this turn is dealt to its owner instead. Only this creatures owner may activate this ability”.

// LEA 32 — Purelace
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “Target spell or permanent becomes white”.

// LEA 33 — Red Ward
// Audit: blocked — Needs the named-color protection rules and Aura self-retention exception for “Enchanted creature has protection from red. This effect doesn't remove this Aura”.

// LEA 34 — Resurrection
pub(in crate::card::sets) static RESURRECTION: CardRecord = CardRecord::new(
    cards::RESURRECTION,
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
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ]),
);

// LEA 35 — Reverse Damage
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “The next time a source of your choice would deal damage to you this turn, prevent that damage. You gain life equal to the damage prevented this way”.

// LEA 36 — Righteousness
pub(in crate::card::sets) static RIGHTEOUSNESS: CardRecord = CardRecord::new(
    cards::RIGHTEOUSNESS,
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
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(7),
                toughness: ValueDef::Constant(7),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 37 — Samite Healer
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{T}: Prevent the next 1 damage that would be dealt to any target this turn”.

// LEA 38 — Savannah Lions
pub(in crate::card::sets) static SAVANNAH_LIONS: CardRecord = CardRecord::new(
    cards::SAVANNAH_LIONS,
    "Savannah Lions",
    CardArt::new("d05b92bd-797e-413f-a8b0-32e0937a1ee0", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{W}"), &["Cat"], 2, 1),
);

// LEA 39 — Serra Angel
pub(in crate::card::sets) static SERRA_ANGEL: CardRecord = CardRecord::new(
    cards::SERRA_ANGEL,
    "Serra Angel",
    CardArt::new("f8ac5006-91bd-4803-93da-f87cf196dd2f", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 4, 4)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// LEA 40 — Swords to Plowshares
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
                placement: ZonePlacement::Top,
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
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “As long as this creature is untapped, all damage that would be dealt to you by unblocked creatures is dealt to this creature instead”.

// LEA 42 — Wall of Swords
pub(in crate::card::sets) static WALL_OF_SWORDS: CardRecord = CardRecord::new(
    cards::WALL_OF_SWORDS,
    "Wall of Swords",
    CardArt::new("99ec4723-b36c-4015-b361-736a6523e8f5", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Wall"], 3, 5)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// LEA 43 — White Knight
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

// LEA 44 — White Ward
// Audit: blocked — Needs the named-color protection rules and Aura self-retention exception for “Enchanted creature has protection from white. This effect doesn't remove this Aura”.

// LEA 45 — Wrath of God
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

// LEA 46 — Air Elemental
pub(in crate::card::sets) static AIR_ELEMENTAL: CardRecord = CardRecord::new(
    cards::AIR_ELEMENTAL,
    "Air Elemental",
    CardArt::new("69c3b2a3-0daa-4d42-832d-fcdfda6555ea", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Elemental"], 4, 4)
        .with_abilities(&[abilities::flying()]),
);

// LEA 47 — Ancestral Recall
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

// LEA 48 — Animate Artifact
// Audit: blocked — Needs Aura-scoped animation with base power and toughness dynamically equal to the enchanted artifact's mana value.

// LEA 49 — Blue Elemental Blast
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

// LEA 50 — Braingeyser
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

// LEA 51 — Clone
pub(in crate::card::sets) static CLONE: CardRecord = CardRecord::new(
    cards::CLONE,
    "Clone",
    CardArt::new("f00d33dd-4eb2-4446-9813-1923d8e2d2f3", "Julie Baroh"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Shapeshifter"], 0, 0).with_abilities(&[
        AbilityDef::replacement(
            "You may have this creature enter as a copy of any creature on the battlefield.",
            EffectDef::CopyPermanentAsItEnters {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                added_types: CardTypeSet::empty(),
            },
        ),
    ]),
);

// LEA 52 — Control Magic
// Audit: blocked — Needs an attachment-scoped control-changing continuous effect for “You control enchanted creature”.

// LEA 53 — Copy Artifact
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

// LEA 54 — Counterspell
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

// LEA 55 — Creature Bond
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “When enchanted creature dies, this Aura deals damage equal to that creature's toughness to the creature's controller”.

// LEA 56 — Drain Power
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Target player activates a mana ability of each land they control. Then that player loses all unspent mana and you add the mana lost this way”.

// LEA 57 — Feedback
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted enchantment's controller, this Aura deals 1 damage to that player”.

// LEA 58 — Flight
pub(in crate::card::sets) static FLIGHT: CardRecord = CardRecord::new(
    cards::FLIGHT,
    "Flight",
    CardArt::new("67c7784b-6b79-4268-a714-895c82809aff", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has flying.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::flying()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 59 — Invisibility
pub(in crate::card::sets) static INVISIBILITY: CardRecord = CardRecord::new(
    cards::INVISIBILITY,
    "Invisibility",
    CardArt::new("1858ac51-e6a7-48d7-8759-166070ca13d8", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature can't be blocked except by Walls.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Not(
                        &ObjectPredicateDef::Subtype("Wall"),
                    )),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 60 — Jump
pub(in crate::card::sets) static JUMP: CardRecord = CardRecord::new(
    cards::JUMP,
    "Jump",
    CardArt::new("cb3f4b11-ad1b-48e2-a500-787d351b0174", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::GrantAbility(&abilities::flying()),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 61 — Lifetap
pub(in crate::card::sets) static LIFETAP: CardRecord = CardRecord::new(
    cards::LIFETAP,
    "Lifetap",
    CardArt::new("11add837-7ee4-4104-b031-c161bce459ae", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{U}{U}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a Forest an opponent controls becomes tapped, you gain 1 life.",
        TriggerEventDef::BecomesTapped(ObjectPredicateDef::All(&[
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
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “Other Merfolk get +1/+1 and have islandwalk”.

// LEA 63 — Magical Hack
// Audit: partial — Text changing rewrites land type lines and intrinsic mana only, not landwalk, predicates, other rules text, or spell text.
pub(in crate::card::sets) static MAGICAL_HACK: CardRecord = CardRecord::new(
    cards::MAGICAL_HACK,
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
pub(in crate::card::sets) static MAHAMOTI_DJINN: CardRecord = CardRecord::new(
    cards::MAHAMOTI_DJINN,
    "Mahamoti Djinn",
    CardArt::new("36204ddd-ddf7-4b44-ae3c-b4a5a41ac9cb", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Djinn"], 5, 6)
        .with_abilities(&[abilities::flying()]),
);

// LEA 65 — Mana Short
pub(in crate::card::sets) static MANA_SHORT: CardRecord = CardRecord::new(
    cards::MANA_SHORT,
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
                object: EffectRecipientDef::ObjectsControlledByTarget {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    slot: TargetIndex::PRIMARY,
                },
            },
            EffectDef::EmptyManaPool {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// LEA 66 — Merfolk of the Pearl Trident
pub(in crate::card::sets) static MERFOLK_OF_THE_PEARL_TRIDENT: CardRecord = CardRecord::new(
    cards::MERFOLK_OF_THE_PEARL_TRIDENT,
    "Merfolk of the Pearl Trident",
    CardArt::new("2b871039-6a66-4ac3-95e7-24759c1f2f92", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk"], 1, 1),
);

// LEA 67 — Phantasmal Forces
pub(in crate::card::sets) static PHANTASMAL_FORCES: CardRecord = CardRecord::new(
    cards::PHANTASMAL_FORCES,
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
            EffectDef::UnlessPaid {
                cost: mana_cost!("{U}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// LEA 68 — Phantasmal Terrain
// Audit: blocked — Needs a persistent dynamic characteristic choice and predicates that consume it for “Enchanted land is the chosen type”.

// LEA 69 — Phantom Monster
pub(in crate::card::sets) static PHANTOM_MONSTER: CardRecord = CardRecord::new(
    cards::PHANTOM_MONSTER,
    "Phantom Monster",
    CardArt::new("e46d2cf5-e8d0-4fb2-b950-252d52084b63", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Illusion"], 3, 3)
        .with_abilities(&[abilities::flying()]),
);

// LEA 70 — Pirate Ship
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack unless defending player controls an Island”.

// LEA 71 — Power Leak
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted enchantment's controller, that player may pay any amount of mana. This Aura deals 2 damage to that player. Prevent X of that damage, where X…”.

// LEA 72 — Power Sink
// Audit: blocked — Needs counter-unless-X resolution whose failed-payment branch taps mana lands and empties that player's mana pool.

// LEA 73 — Prodigal Sorcerer
pub(in crate::card::sets) static PRODIGAL_SORCERER: CardRecord = CardRecord::new(
    cards::PRODIGAL_SORCERER,
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

// LEA 75 — Psychic Venom
// Audit: blocked — Needs a trigger relation for the attached permanent becoming tapped and its controller/characteristics for “Whenever enchanted land becomes tapped, this Aura deals 2 damage to that land's controller”.

// LEA 76 — Sea Serpent
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can't attack unless defending player controls an Island”.

// LEA 77 — Siren's Call
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “At the beginning of the next end step, destroy all non-Wall creatures that player controls that didn't attack this turn. Ignore this effect for each creature the player didn't control…”.

// LEA 78 — Sleight of Mind
// Audit: blocked — Needs copiable-value or rules-text mutation support for “Change the text of target spell or permanent by replacing all instances of one color word with another”.

// LEA 79 — Spell Blast
// Audit: partial — A target spell's chosen X is omitted from its stack mana value.
pub(in crate::card::sets) static SPELL_BLAST: CardRecord = CardRecord::new(
    cards::SPELL_BLAST,
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
pub(in crate::card::sets) static STASIS: CardRecord = CardRecord::new(
    cards::STASIS,
    "Stasis",
    CardArt::new("1e328704-d1d9-47f4-a923-8b5c187d4dc6", "Fay Jones"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Players skip their untap steps.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::DoesNotUntapDuringUntapStep,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::UnlessPaid {
                cost: mana_cost!("{U}"),
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// LEA 81 — Steal Artifact
// Audit: blocked — Needs an attachment-scoped control-changing continuous effect for “You control enchanted artifact”.

// LEA 82 — Thoughtlace
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “Target spell or permanent becomes blue”.

// LEA 83 — Time Walk
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

// LEA 84 — Timetwister
pub(in crate::card::sets) static TIMETWISTER: CardRecord = CardRecord::new(
    cards::TIMETWISTER,
    "Timetwister",
    CardArt::new("9a49dc44-616e-4bdd-8220-0bb71eccc512", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell(
        "Each player shuffles their hand and graveyard into their library, then draws seven cards. (Then put Timetwister into its owner's graveyard.)",
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Hand, ZoneKind::Graveyard],
                    controller: PlayerRelation::Any,
                },
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
                controller: None,
            },
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::EachPlayer,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(7),
            },
        ]),
    )]),
);

static TWIDDLE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Land),
    ]),
)];

// LEA 85 — Twiddle
// Audit: partial — Tap versus untap is locked while casting instead of chosen, or declined, when the spell resolves.
pub(in crate::card::sets) static TWIDDLE: CardRecord = CardRecord::new(
    cards::TWIDDLE,
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
pub(in crate::card::sets) static UNSUMMON: CardRecord = CardRecord::new(
    cards::UNSUMMON,
    "Unsummon",
    CardArt::new("8512f2c1-6361-4b79-843f-80b6bceeeb99", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )]),
);

// LEA 87 — Vesuvan Doppelganger
// Audit: blocked — Needs copiable-value or rules-text mutation support for “You may have this creature enter as a copy of any creature on the battlefield, except it doesn't copy that creature's color and it has "At the beginning of your upkeep, you may have this…”.

// LEA 88 — Volcanic Eruption
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “Destroy X target Mountains. Volcanic Eruption deals damage to each creature and each player equal to the number of Mountains put into a graveyard this way”.

// LEA 89 — Wall of Air
pub(in crate::card::sets) static WALL_OF_AIR: CardRecord = CardRecord::new(
    cards::WALL_OF_AIR,
    "Wall of Air",
    CardArt::new("da56fdf3-6a8f-4833-a5c3-197650cc4889", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Wall"], 1, 5)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// LEA 90 — Wall of Water
pub(in crate::card::sets) static WALL_OF_WATER: CardRecord = CardRecord::new(
    cards::WALL_OF_WATER,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEA 91 — Water Elemental
pub(in crate::card::sets) static WATER_ELEMENTAL: CardRecord = CardRecord::new(
    cards::WATER_ELEMENTAL,
    "Water Elemental",
    CardArt::new("8de940d6-98c0-46a9-b5fd-e2b0899ea19e", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Elemental"], 5, 4),
);

// LEA 92 — Animate Dead
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “When this Aura enters, if it's on the battlefield, it loses "enchant creature card in a graveyard" and gains "enchant creature put onto the battlefield with this Aura." Return enchanted…”.

// LEA 93 — Bad Moon
pub(in crate::card::sets) static BAD_MOON: CardRecord = CardRecord::new(
    cards::BAD_MOON,
    "Bad Moon",
    CardArt::new("43572906-ea74-4411-a549-5dc401591d2a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::static_ability(
        "Black creatures get +1/+1.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
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
    )]),
);

// LEA 94 — Black Knight
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

// LEA 95 — Bog Wraith
// Audit: blocked — Needs the printed landwalk variant and its defending-player land/blocking semantics for “Swampwalk”.

// LEA 96 — Contract from Below
// Audit: blocked — Needs ante-zone and deck-construction handling for “Discard your hand, ante the top card of your library, then draw seven cards”.

// LEA 97 — Cursed Land
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted land's controller, this Aura deals 1 damage to that player”.

// LEA 98 — Dark Ritual
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

// LEA 99 — Darkpact
// Audit: blocked — Needs an ante zone plus a permanent ownership exchange between a chosen ante card and the top card of a library.

// LEA 100 — Deathgrip
pub(in crate::card::sets) static DEATHGRIP: CardRecord = CardRecord::new(
    cards::DEATHGRIP,
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
            },
        ),
    ]),
);

// LEA 101 — Deathlace
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “Target spell or permanent becomes black”.

// LEA 102 — Demonic Attorney
// Audit: blocked — Needs the ante procedure and its associated deck-construction handling for “Each player antes the top card of their library”.

// LEA 103 — Demonic Hordes
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “At the beginning of your upkeep, unless you pay {B}{B}{B}, tap this creature and sacrifice a land of an opponent's choice”.

// LEA 104 — Demonic Tutor
pub(in crate::card::sets) static DEMONIC_TUTOR: CardRecord = CardRecord::new(
    cards::DEMONIC_TUTOR,
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
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
        },
    )]),
);

// LEA 105 — Drain Life
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

// LEA 106 — Drudge Skeletons
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{B}: Regenerate this creature”.

// LEA 107 — Evil Presence
pub(in crate::card::sets) static EVIL_PRESENCE: CardRecord = CardRecord::new(
    cards::EVIL_PRESENCE,
    "Evil Presence",
    CardArt::new("0551d66e-8cd4-48f0-aa17-15f26be9d85f", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant land", &ENCHANT_LAND_TARGET),
            AbilityDef::static_ability(
                "Enchanted land is a Swamp.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::SetLandTypes(&[BasicLandType::Swamp]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 108 — Fear
// Audit: partial — The blocking restriction is stored directly rather than as a removable granted ability.
pub(in crate::card::sets) static FEAR: CardRecord = CardRecord::new(
    cards::FEAR,
    "Fear",
    CardArt::new("0cd927be-e63f-4371-a1d8-7a0489cb187e", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has fear. (It can't be blocked except by artifact creatures and/or black creatures.)",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Not(
                        &ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Color(ManaColor::Black),
                        ]),
                    )),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The blocking restriction is stored directly rather than as a removable granted ability.",
            )),
        ]),
);

// LEA 109 — Frozen Shade
pub(in crate::card::sets) static FROZEN_SHADE: CardRecord = CardRecord::new(
    cards::FROZEN_SHADE,
    "Frozen Shade",
    CardArt::new("d0bd76c8-4cff-4c15-9686-7a299b589814", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Shade"], 0, 1).with_abilities(&[
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
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

// LEA 110 — Gloom
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Activated abilities of white enchantments cost {3} more to activate”.

// LEA 111 — Howl from Beyond
pub(in crate::card::sets) static HOWL_FROM_BEYOND: CardRecord = CardRecord::new(
    cards::HOWL_FROM_BEYOND,
    "Howl from Beyond",
    CardArt::new("67ec17e1-174b-4d07-a27f-91a333c4b2fb", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_instant(mana_cost!("{X}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +X/+0 until end of turn.",
        &ENCHANT_CREATURE_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::ChosenX,
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// LEA 112 — Hypnotic Specter
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
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
            },
        ),
    ]),
);

static TARGET_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// LEA 113 — Lich
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Whenever you're dealt damage, sacrifice that many nontoken permanents. If you can't, you lose the game”.

// LEA 114 — Lord of the Pit
// Audit: blocked — Needs a mandatory creature-sacrifice choice with an explicit no-legal-sacrifice damage branch during upkeep.

// LEA 115 — Mind Twist
pub(in crate::card::sets) static MIND_TWIST: CardRecord = CardRecord::new(
    cards::MIND_TWIST,
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
        },
    )]),
);

static PESTILENCE_NO_CREATURES: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: PlayerRelation::Any,
    },
    comparison: ComparisonDef::Equal,
    amount: 0,
};

// LEA 116 — Nether Shadow
// Audit: blocked — Needs a zone-object query and identity-preserving continuation for “At the beginning of your upkeep, if this card is in your graveyard with three or more creature cards above it, you may put this card onto the battlefield”.

// LEA 117 — Nettling Imp
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{T}: Choose target non-Wall creature the active player has controlled continuously since the beginning of the turn. That creature attacks this turn if able. Destroy it at the beginning…”.

// LEA 118 — Nightmare
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Nightmare's power and toughness are each equal to the number of Swamps you control”.

// LEA 119 — Paralyze
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “Enchanted creature doesn't untap during its controller's untap step”.

// LEA 120 — Pestilence
pub(in crate::card::sets) static PESTILENCE: CardRecord = CardRecord::new(
    cards::PESTILENCE,
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
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Any,
                    },
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
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Plague Rats's power and toughness are each equal to the number of creatures named Plague Rats on the battlefield”.

// LEA 122 — Raise Dead
pub(in crate::card::sets) static RAISE_DEAD: CardRecord = CardRecord::new(
    cards::RAISE_DEAD,
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
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
        },
    )]),
);

// LEA 123 — Royal Assassin
// Audit: blocked — Needs an object predicate for a permanent that is currently tapped.

// LEA 124 — Sacrifice
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Add an amount of {B} equal to the sacrificed creature's mana value”.

// LEA 125 — Scathe Zombies
pub(in crate::card::sets) static SCATHE_ZOMBIES: CardRecord = CardRecord::new(
    cards::SCATHE_ZOMBIES,
    "Scathe Zombies",
    CardArt::new("e9be6dcf-5e25-4b8c-9cd0-badf3771f81e", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 2),
);

// LEA 126 — Scavenging Ghoul
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of each end step, put a corpse counter on this creature for each creature that died this turn”.

// LEA 127 — Sengir Vampire
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

// LEA 128 — Simulacrum
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “You gain life equal to the damage dealt to you this turn. Simulacrum deals damage to target creature you control equal to the damage dealt to you this turn”.

// LEA 129 — Sinkhole
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

// LEA 130 — Terror
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

// LEA 131 — Unholy Strength
pub(in crate::card::sets) static UNHOLY_STRENGTH: CardRecord = CardRecord::new(
    cards::UNHOLY_STRENGTH,
    "Unholy Strength",
    CardArt::new("90563f90-0127-4164-b43b-f0321dc63a1d", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 132 — Wall of Bone
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{B}: Regenerate this creature”.

// LEA 133 — Warp Artifact
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted artifact's controller, this Aura deals 1 damage to that player”.

// LEA 134 — Weakness
pub(in crate::card::sets) static WEAKNESS: CardRecord = CardRecord::new(
    cards::WEAKNESS,
    "Weakness",
    CardArt::new("36ca06a1-9b9a-49a2-9c47-9b72228621bc", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-1.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(-2),
                        toughness: ValueDef::Constant(-1),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 135 — Will-o'-the-Wisp
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{B}: Regenerate this creature”.

// LEA 136 — Word of Command
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Look at target opponent's hand and choose a card from it. You control that player until Word of Command finishes resolving. The player plays that card if able. While doing so, the player…”.

// LEA 137 — Zombie Master
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “Other Zombies have "{B}: Regenerate this permanent."”.

// LEA 138 — Burrowing
pub(in crate::card::sets) static BURROWING: CardRecord = CardRecord::new(
    cards::BURROWING,
    "Burrowing",
    CardArt::new("a14c05e4-8df3-450b-8a98-5028e73b14c1", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has mountainwalk. (It can't be blocked as long as defending player controls a Mountain.)",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
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
            comparison: ComparisonDef::GreaterOrEqual,
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

// LEA 139 — Chaoslace
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “Target spell or permanent becomes red”.

// LEA 140 — Disintegrate
// Audit: blocked — Needs a duration-scoped prohibition on creating or applying regeneration shields for “Disintegrate deals X damage to any target. If it's a creature, it can't be regenerated this turn, and if it would die this turn, exile it instead”.

// LEA 141 — Dragon Whelp
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

// LEA 142 — Dwarven Demolition Team
pub(in crate::card::sets) static DWARVEN_DEMOLITION_TEAM: CardRecord = CardRecord::new(
    cards::DWARVEN_DEMOLITION_TEAM,
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
// Audit: partial — The target's power omits modifiers from static continuous effects.
pub(in crate::card::sets) static DWARVEN_WARRIORS: CardRecord = CardRecord::new(
    cards::DWARVEN_WARRIORS,
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
            EffectDef::MakeUnblockableThisTurn {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The target's power omits modifiers from static continuous effects.",
        )),
    ]),
);

// LEA 144 — Earth Elemental
pub(in crate::card::sets) static EARTH_ELEMENTAL: CardRecord = CardRecord::new(
    cards::EARTH_ELEMENTAL,
    "Earth Elemental",
    CardArt::new("b24b5864-44c0-4bc8-8705-9504f83b2c03", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental"], 4, 5),
);

// LEA 145 — Earthbind
// Audit: blocked — Needs an Aura-entry condition on the attached creature plus a persistent removal of flying created during resolution.

// LEA 146 — Earthquake
// Audit: partial — The flying predicate omits abilities granted or removed by static continuous effects.
pub(in crate::card::sets) static EARTHQUAKE: CardRecord = CardRecord::new(
    cards::EARTHQUAKE,
    "Earthquake",
    CardArt::new("e68ac362-6cdc-48a6-bdd3-4f8ea32add64", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[AbilityDef::spell(
        "Earthquake deals X damage to each creature without flying and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                amount: ValueDef::ChosenX,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::ChosenX,
            },
        ]),
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The flying predicate omits abilities granted or removed by static continuous effects.",
    ))]),
);

// LEA 147 — False Orders
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Remove target creature defending player controls from combat. Creatures it was blocking that had become blocked by only that creature this combat become unblocked. You may have it block…”.

// LEA 148 — Fire Elemental
pub(in crate::card::sets) static FIRE_ELEMENTAL: CardRecord = CardRecord::new(
    cards::FIRE_ELEMENTAL,
    "Fire Elemental",
    CardArt::new("da237992-2919-4e37-8f56-2164095f59b5", "Melissa A. Benson"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental"], 5, 4),
);

// LEA 149 — Fireball
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

// LEA 150 — Firebreathing
// Audit: partial — If the Aura leaves before resolution, the former enchanted creature is not retained through source last-known information.
pub(in crate::card::sets) static FIREBREATHING: CardRecord = CardRecord::new(
    cards::FIREBREATHING,
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
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_coverage(AbilityCoverageDef::partial(
                "If the Aura leaves before resolution, the former enchanted creature is not retained through source last-known information.",
            )),
        ]),
);

// LEA 151 — Flashfires
pub(in crate::card::sets) static FLASHFIRES: CardRecord = CardRecord::new(
    cards::FLASHFIRES,
    "Flashfires",
    CardArt::new("ee8a05a4-0ce3-4abe-bb60-08af53cf08e5", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell(
        "Destroy all Plains.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )]),
);

// LEA 152 — Fork
// Audit: partial — Copy retargeting is offered as one ordered decision instead of independent choices for each target slot.
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

// LEA 153 — Goblin Balloon Brigade
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

// LEA 154 — Goblin King
pub(in crate::card::sets) static GOBLIN_KING: CardRecord = CardRecord::new(
    cards::GOBLIN_KING,
    "Goblin King",
    CardArt::new("5873672d-37ea-4c0f-97f3-12b74fde112d", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Goblins get +1/+1 and have mountainwalk.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Goblin"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(1),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                ]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ]),
);

// LEA 155 — Granite Gargoyle
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

// LEA 156 — Gray Ogre
pub(in crate::card::sets) static GRAY_OGRE: CardRecord = CardRecord::new(
    cards::GRAY_OGRE,
    "Gray Ogre",
    CardArt::new("73ae5276-b607-4f23-a9d2-e8cc7b8e3693", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ogre"], 2, 2),
);

// LEA 157 — Hill Giant
pub(in crate::card::sets) static HILL_GIANT: CardRecord = CardRecord::new(
    cards::HILL_GIANT,
    "Hill Giant",
    CardArt::new("0ddb98e8-13fe-4786-83f7-b72c56db135a", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Giant"], 3, 3),
);

// LEA 158 — Hurloon Minotaur
pub(in crate::card::sets) static HURLOON_MINOTAUR: CardRecord = CardRecord::new(
    cards::HURLOON_MINOTAUR,
    "Hurloon Minotaur",
    CardArt::new("78a9088f-8755-47cb-aa93-51d992ccab90", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Minotaur"], 2, 3),
);

// LEA 159 — Ironclaw Orcs
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

// LEA 160 — Keldon Warlord
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Keldon Warlord's power and toughness are each equal to the number of non-Wall creatures you control”.

// LEA 161 — Lightning Bolt
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

// LEA 162 — Mana Flare
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Whenever a player taps a land for mana, that player adds one mana of any type that land produced”.

// LEA 163 — Manabarbs
pub(in crate::card::sets) static MANABARBS: CardRecord = CardRecord::new(
    cards::MANABARBS,
    "Manabarbs",
    CardArt::new("6121f72f-680f-4bb4-ae4d-37ee4ebed4d8", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player taps a land for mana, this enchantment deals 1 damage to that player.",
        TriggerEventDef::TappedForMana(ObjectPredicateDef::HasType(CardType::Land)),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// LEA 164 — Mons's Goblin Raiders
pub(in crate::card::sets) static MONSS_GOBLIN_RAIDERS: CardRecord = CardRecord::new(
    cards::MONSS_GOBLIN_RAIDERS,
    "Mons's Goblin Raiders",
    CardArt::new("b4eb3db3-6a7c-488a-9433-d5d1d3133816", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1),
);

// LEA 165 — Orcish Artillery
pub(in crate::card::sets) static ORCISH_ARTILLERY: CardRecord = CardRecord::new(
    cards::ORCISH_ARTILLERY,
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
pub(in crate::card::sets) static ORCISH_ORIFLAMME: CardRecord = CardRecord::new(
    cards::ORCISH_ORIFLAMME,
    "Orcish Oriflamme",
    CardArt::new("911538ea-322c-4c40-a9c3-35e47fe60fce", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Attacking creatures you control get +1/+0.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
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
    )]),
);

// LEA 167 — Power Surge
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “At the beginning of each player's upkeep, this enchantment deals X damage to that player, where X is the number of untapped lands they controlled at the beginning of this turn”.

// LEA 168 — Raging River
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever one or more creatures you control attack, each defending player divides all creatures without flying they control into a "left" pile and a "right" pile. Then, for each attacking…”.

// LEA 169 — Red Elemental Blast
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

// LEA 170 — Roc of Kher Ridges
pub(in crate::card::sets) static ROC_OF_KHER_RIDGES: CardRecord = CardRecord::new(
    cards::ROC_OF_KHER_RIDGES,
    "Roc of Kher Ridges",
    CardArt::new("731a4b86-c213-4d8e-bf01-0a0e8cff0ff1", "Andi Rusu"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Bird"], 3, 3)
        .with_abilities(&[abilities::flying()]),
);

// The chosen presentation art is its Beta printing; the definition debuted in Alpha.
// LEA 171 — Rock Hydra
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “For each 1 damage that would be dealt to this creature, if it has a +1/+1 counter on it, remove a +1/+1 counter from it and prevent that 1 damage”.

// LEA 172 — Sedge Troll
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

// LEA 173 — Shatter
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

// LEA 174 — Shivan Dragon
pub(in crate::card::sets) static SHIVAN_DRAGON: CardRecord = CardRecord::new(
    cards::SHIVAN_DRAGON,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEA 175 — Smoke
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

// LEA 176 — Stone Giant
// Audit: partial — The source's power and target's toughness omit modifiers from static continuous effects.
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
            )
            .with_coverage(AbilityCoverageDef::partial(
                "The source's power and target's toughness omit modifiers from static continuous effects.",
            )),
        ]),
);

// LEA 177 — Stone Rain
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

// LEA 178 — Tunnel
pub(in crate::card::sets) static TUNNEL: CardRecord = CardRecord::new(
    cards::TUNNEL,
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
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “This creature can block an additional creature each combat”.

// LEA 180 — Uthden Troll
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{R}: Regenerate this creature”.

// LEA 181 — Wall of Fire
pub(in crate::card::sets) static WALL_OF_FIRE: CardRecord = CardRecord::new(
    cards::WALL_OF_FIRE,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// LEA 182 — Wall of Stone
pub(in crate::card::sets) static WALL_OF_STONE: CardRecord = CardRecord::new(
    cards::WALL_OF_STONE,
    "Wall of Stone",
    CardArt::new("f7fd8b8e-98fd-4b0d-8bb9-06bd25a1e30f", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Wall"], 0, 8)
        .with_abilities(&[abilities::defender()]),
);

/// `Discard` saturates at the recipient's hand size. Using the largest
/// declarative amount therefore says "their hand" while retaining the shared
/// recipient-chosen discard procedure.
const ENTIRE_HAND: ValueDef = ValueDef::Constant(i32::MAX);

// LEA 183 — Wheel of Fortune
pub(in crate::card::sets) static WHEEL_OF_FORTUNE: CardRecord = CardRecord::new(
    cards::WHEEL_OF_FORTUNE,
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
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(7),
            },
        ]),
    )]),
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

// LEA 184 — Aspect of Wolf
// Audit: blocked — Needs rounded division in dynamic power/toughness values for “Enchanted creature gets +X/+Y, where X is half the number of Forests you control, rounded down, and Y is half the number of Forests you control, rounded up”.

// LEA 185 — Berserk
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

// LEA 186 — Birds of Paradise
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
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// LEA 187 — Camouflage
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “This turn, instead of declaring blockers, each defending player chooses any number of creatures they control and divides them into a number of piles equal to the number of attacking…”.

// LEA 188 — Channel
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

// LEA 189 — Cockatrice
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked by a non-Wall creature, destroy that creature at end of combat”.

// LEA 190 — Craw Wurm
pub(in crate::card::sets) static CRAW_WURM: CardRecord = CardRecord::new(
    cards::CRAW_WURM,
    "Craw Wurm",
    CardArt::new("bfed1a95-bd67-4e16-a781-81866028af2f", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Wurm"], 6, 4),
);

// LEA 191 — Elvish Archers
pub(in crate::card::sets) static ELVISH_ARCHERS: CardRecord = CardRecord::new(
    cards::ELVISH_ARCHERS,
    "Elvish Archers",
    CardArt::new("1cb9d405-f2b5-4e10-a405-feafd2a87d90", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Archer"], 2, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// LEA 192 — Fastbond
// Audit: blocked — Needs damage-history/source tracking or card-specific damage processing for “Whenever you play a land, if it wasn't the first land you played this turn, this enchantment deals 1 damage to you”.

// LEA 193 — Fog
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “Prevent all combat damage that would be dealt this turn”.

// LEA 194 — Force of Nature
pub(in crate::card::sets) static FORCE_OF_NATURE: CardRecord = CardRecord::new(
    cards::FORCE_OF_NATURE,
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
                EffectDef::UnlessPaid {
                    cost: mana_cost!("{G}{G}{G}{G}"),
                    otherwise: &EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(8),
                    },
                },
            ),
        ]),
);

// LEA 195 — Fungusaur
// Audit: partial — Simultaneous damage from multiple creatures produces one trigger per source instead of one trigger for the event.
pub(in crate::card::sets) static FUNGUSAUR: CardRecord = CardRecord::new(
    cards::FUNGUSAUR,
    "Fungusaur",
    CardArt::new("5ad89f0d-b09b-40a0-84d6-3ee60dec7e23", "Daniel Gelon"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Fungus", "Dinosaur"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, put a +1/+1 counter on it.",
            TriggerEventDef::DamageDealt {
                source: ObjectPredicateDef::Any,
                recipient: EffectRecipientDef::Source,
            },
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
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “As long as Gaea's Liege isn't attacking, its power and toughness are each equal to the number of Forests you control. As long as Gaea's Liege is attacking, its power and toughness are…”.

// LEA 197 — Giant Growth
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

// LEA 198 — Giant Spider
pub(in crate::card::sets) static GIANT_SPIDER: CardRecord = CardRecord::new(
    cards::GIANT_SPIDER,
    "Giant Spider",
    CardArt::new("77636b4c-faea-4bf5-b88c-dd5bb88dc930", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spider"], 2, 4)
        .with_abilities(&[abilities::reach()]),
);

// LEA 199 — Grizzly Bears
pub(in crate::card::sets) static GRIZZLY_BEARS: CardRecord = CardRecord::new(
    cards::GRIZZLY_BEARS,
    "Grizzly Bears",
    CardArt::new("ce2d603a-3231-4a8c-bf39-1617586ea870", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2),
);

// LEA 200 — Hurricane
// Audit: partial — The flying predicate omits abilities granted or removed by static continuous effects.
pub(in crate::card::sets) static HURRICANE: CardRecord = CardRecord::new(
    cards::HURRICANE,
    "Hurricane",
    CardArt::new("52f5a19f-16e4-4d35-89e1-969ac8202f88", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_abilities(&[AbilityDef::spell(
        "Hurricane deals X damage to each creature with flying and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                amount: ValueDef::ChosenX,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::ChosenX,
            },
        ]),
    )
    .with_coverage(AbilityCoverageDef::partial(
        "The flying predicate omits abilities granted or removed by static continuous effects.",
    ))]),
);

// LEA 201 — Ice Storm
pub(in crate::card::sets) static ICE_STORM: CardRecord = CardRecord::new(
    cards::ICE_STORM,
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
// Audit: blocked — Needs a persistent tap/untap restriction or event relation for “{0}: Untap enchanted creature. Activate only during your turn and only once each turn”.

// LEA 203 — Ironroot Treefolk
pub(in crate::card::sets) static IRONROOT_TREEFOLK: CardRecord = CardRecord::new(
    cards::IRONROOT_TREEFOLK,
    "Ironroot Treefolk",
    CardArt::new("b93c5869-7777-44bb-967a-e9439b25ced4", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Treefolk"], 3, 5),
);

// LEA 204 — Kudzu
// Audit: blocked — Needs a trigger relation for the attached permanent becoming tapped and its controller/characteristics for “When enchanted land becomes tapped, destroy it. That land's controller may attach this Aura to a land of their choice”.

// LEA 205 — Ley Druid
pub(in crate::card::sets) static LEY_DRUID: CardRecord = CardRecord::new(
    cards::LEY_DRUID,
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
pub(in crate::card::sets) static LIFEFORCE: CardRecord = CardRecord::new(
    cards::LIFEFORCE,
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
            },
        ),
    ]),
);

// LEA 207 — Lifelace
// Audit: blocked — Needs a duration-aware color-setting characteristic-layer effect for “Target spell or permanent becomes green”.

// LEA 208 — Living Artifact
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “At the beginning of your upkeep, you may remove a vitality counter from this Aura. If you do, you gain 1 life”.

// LEA 209 — Living Lands
// Audit: blocked — Needs static animation to continuously turn the matching lands into creatures for “All Forests are 1/1 creatures that are still lands”.

// LEA 210 — Llanowar Elves
pub(in crate::card::sets) static LLANOWAR_ELVES: CardRecord = CardRecord::new(
    cards::LLANOWAR_ELVES,
    "Llanowar Elves",
    CardArt::new("d4f1cc9e-4f99-4c26-ac1b-8ef069fa8ceb", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
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

// LEA 211 — Lure
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “All creatures able to block enchanted creature do so”.

// LEA 212 — Natural Selection
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “Look at the top three cards of target player's library, then put them back in any order. You may have that player shuffle”.

// LEA 213 — Regeneration
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{G}: Regenerate enchanted creature”.

// LEA 214 — Regrowth
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
            placement: ZonePlacement::Top,
        },
    )]),
);

// LEA 215 — Scryb Sprites
pub(in crate::card::sets) static SCRYB_SPRITES: CardRecord = CardRecord::new(
    cards::SCRYB_SPRITES,
    "Scryb Sprites",
    CardArt::new("6d929c38-91e6-457c-937a-d1884f4bba44", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Faerie"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// LEA 216 — Shanodin Dryads
pub(in crate::card::sets) static SHANODIN_DRYADS: CardRecord = CardRecord::new(
    cards::SHANODIN_DRYADS,
    "Shanodin Dryads",
    CardArt::new("814cf35c-f1ad-4bf4-8c10-a5592c3b1be8", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Nymph", "Dryad"], 1, 1)
        .with_abilities(&[abilities::forestwalk()]),
);

// LEA 217 — Stream of Life
pub(in crate::card::sets) static STREAM_OF_LIFE: CardRecord = CardRecord::new(
    cards::STREAM_OF_LIFE,
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
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “Whenever this creature blocks or becomes blocked by a non-Wall creature, destroy that creature at end of combat”.

// LEA 219 — Timber Wolves
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// LEA 220 — Tranquility
pub(in crate::card::sets) static TRANQUILITY: CardRecord = CardRecord::new(
    cards::TRANQUILITY,
    "Tranquility",
    CardArt::new("774cc5a6-3a69-4812-add4-eb5eb6389238", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Destroy all enchantments.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )]),
);

// LEA 221 — Tsunami
pub(in crate::card::sets) static TSUNAMI: CardRecord = CardRecord::new(
    cards::TSUNAMI,
    "Tsunami",
    CardArt::new("9ed67d61-cf47-446b-b454-eb404a8686b7", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[AbilityDef::spell(
        "Destroy all Islands.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )]),
);

// LEA 222 — Verduran Enchantress
pub(in crate::card::sets) static VERDURAN_ENCHANTRESS: CardRecord = CardRecord::new(
    cards::VERDURAN_ENCHANTRESS,
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
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{G}: Regenerate this creature”.

// LEA 224 — Wall of Ice
pub(in crate::card::sets) static WALL_OF_ICE: CardRecord = CardRecord::new(
    cards::WALL_OF_ICE,
    "Wall of Ice",
    CardArt::new("cc743a03-867c-4bb0-8fb0-2bcaa0a8a756", "Richard Thomas"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wall"], 0, 7)
        .with_abilities(&[abilities::defender()]),
);

// LEA 225 — Wall of Wood
pub(in crate::card::sets) static WALL_OF_WOOD: CardRecord = CardRecord::new(
    cards::WALL_OF_WOOD,
    "Wall of Wood",
    CardArt::new("8df80424-3bd9-4982-ad79-e55d9ba3b43d", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{G}"), &["Wall"], 0, 3)
        .with_abilities(&[abilities::defender()]),
);

// LEA 226 — Wanderlust
// Audit: blocked — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted creature's controller, this Aura deals 1 damage to that player”.

// LEA 227 — War Mammoth
pub(in crate::card::sets) static WAR_MAMMOTH: CardRecord = CardRecord::new(
    cards::WAR_MAMMOTH,
    "War Mammoth",
    CardArt::new("c8d6081e-f686-4263-a0a2-21c0d9af5fdb", "Jeff A. Menges"),
    CardSet::Alpha,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elephant"], 3, 3)
        .with_abilities(&[abilities::trample()]),
);

// LEA 228 — Web
pub(in crate::card::sets) static WEB: CardRecord = CardRecord::new(
    cards::WEB,
    "Web",
    CardArt::new("37c7890a-86dc-4a97-a7ce-1436fa22d0c0", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            aura_spell("Enchant creature", &ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2 and has reach. (It can block creatures with flying.)",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(0),
                            toughness: ValueDef::Constant(2),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::reach()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// LEA 229 — Wild Growth
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Whenever enchanted land is tapped for mana, its controller adds an additional {G}”.

// LEA 230 — Ankh of Mishra
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

// LEA 231 — Basalt Monolith
pub(in crate::card::sets) static BASALT_MONOLITH: CardRecord = CardRecord::new(
    cards::BASALT_MONOLITH,
    "Basalt Monolith",
    CardArt::new("66a74c89-6f86-4ec8-af17-391cd5026054", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::DoesNotUntapDuringUntapStep,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
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
pub(in crate::card::sets) static BLACK_LOTUS: CardRecord = CardRecord::new(
    cards::BLACK_LOTUS,
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

// LEA 234 — Celestial Prism
// Audit: metadata-only — The mana-ability runtime cannot activate a mana ability that itself has a mana payment cost.
pub(in crate::card::sets) static CELESTIAL_PRISM: CardRecord = CardRecord::new(
    cards::CELESTIAL_PRISM,
    "Celestial Prism",
    CardArt::new("a47417cb-1ea7-4f65-ba06-e27a99373114", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[AbilityDef::not_implemented(
        "{2}, {T}: Add one mana of any color.",
        "The mana-ability runtime cannot currently pay a mana cost while activating a mana ability.",
    )]),
);

static CHAOS_ORB_FLIP_SUCCESS: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::ChosenPermanent(ChoiceIndex::PRIMARY),
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

// LEA 235 — Chaos Orb
pub(in crate::card::sets) static CHAOS_ORB: CardRecord = CardRecord::new(
    cards::CHAOS_ORB,
    "Chaos Orb",
    CardArt::new("92274971-7c4a-4326-b0fe-75e2d124f718", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_abilities(&[
            AbilityDef::activated("{1}, {T}: Choose a nontoken permanent on the battlefield. If Chaos Orb is on the battlefield, flip Chaos Orb onto the battlefield from a height of at least one foot. If Chaos Orb turns over completely at least 360 degrees during the flip, and lands resting on the chosen permanent, destroy that permanent. Then destroy Chaos Orb.", &[
                    AbilityCostDef::Mana(mana_cost!("{1}")),
                    AbilityCostDef::TapSource,
                ], EffectDef::ChoosePermanent {
                    choice: ChoiceIndex::PRIMARY,
                    chooser: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    controller: PlayerRelation::Any,
                    then: &CHAOS_ORB_IF_PRESENT,
                },
            )
            .with_coverage(AbilityCoverageDef::explained_complete(
                "For reproducible headless 93/94 play, the physical flip is represented by one seeded random trial with a 0.9 success likelihood.",
            )),
        ]),
);

// LEA 236 — Clockwork Beast
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than seven. Activate only during your upkeep”.

// LEA 237 — Conservator
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{3}, {T}: Prevent the next 2 damage that would be dealt to you this turn”.

// LEA 238 — Copper Tablet
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

// LEA 239 — Crystal Rod
pub(in crate::card::sets) static CRYSTAL_ROD: CardRecord = CardRecord::new(
    cards::CRYSTAL_ROD,
    "Crystal Rod",
    CardArt::new("76693233-7961-4b7e-80f2-ed90e494c4aa", "Amy Weber"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a blue spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Blue)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{1}"))]),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// LEA 240 — Cyclopean Tomb
// Audit: blocked — Needs card-specific counter state and counter-consuming effects for “{2}, {T}: Put a mire counter on target non-Swamp land. That land is a Swamp for as long as it has a mire counter on it. Activate only during your upkeep”.

// LEA 241 — Dingus Egg
pub(in crate::card::sets) static DINGUS_EGG: CardRecord = CardRecord::new(
    cards::DINGUS_EGG,
    "Dingus Egg",
    CardArt::new("65eb6cda-e512-40a8-9c1f-335b713409ff", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a land is put into a graveyard from the battlefield, this artifact deals 2 damage to that land's controller.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::HasType(CardType::Land),
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(2),
        },
    )]),
);

// LEA 242 — Disrupting Scepter
// Audit: blocked — Needs a hidden-zone decision and continuation for “{3}, {T}: Target player discards a card. Activate only during your turn”.

// LEA 243 — Forcefield
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}: The next time an unblocked creature of your choice would deal combat damage to you this turn, prevent all but 1 of that damage”.

// LEA 244 — Gauntlet of Might
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “Whenever a Mountain is tapped for mana, its controller adds an additional {R}”.

// LEA 245 — Glasses of Urza
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

// LEA 246 — Helm of Chatzuk
// Audit: blocked — Needs full banding group declaration, blocking, and combat-damage assignment semantics.

// LEA 247 — Howling Mine
// Audit: blocked — Needs a hidden-zone decision and continuation for “At the beginning of each player's draw step, if this artifact is untapped, that player draws an additional card”.

// LEA 248 — Icy Manipulator
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

// LEA 249 — Illusionary Mask
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “{X}: You may choose a creature card in your hand whose mana cost could be paid by some amount of, or all of, the mana you spent on {X}. If you do, you may cast that card face down as a…”.

// LEA 250 — Iron Star
pub(in crate::card::sets) static IRON_STAR: CardRecord = CardRecord::new(
    cards::IRON_STAR,
    "Iron Star",
    CardArt::new("5786de12-cade-43c2-a6b0-0c5b294b9d0e", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a red spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Red)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{1}"))]),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// LEA 251 — Ivory Cup
pub(in crate::card::sets) static IVORY_CUP: CardRecord = CardRecord::new(
    cards::IVORY_CUP,
    "Ivory Cup",
    CardArt::new("9964d8d8-dc97-4e5f-9f52-173f7e2c37fd", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a white spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::White)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{1}"))]),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// LEA 252 — Jade Monolith
// Audit: blocked — Needs a duration-scoped replacement/prevention effect for “{1}: The next time a source of your choice would deal damage to target creature this turn, that source deals that damage to you instead”.

// LEA 253 — Jade Statue
// Audit: blocked — Needs a combat declaration or damage-assignment constraint for “{2}: This artifact becomes a 3/6 Golem artifact creature until end of combat. Activate only during combat”.

// LEA 254 — Jayemdae Tome
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

// LEA 255 — Juggernaut
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

// LEA 256 — Kormus Bell
// Audit: blocked — Needs static animation to continuously turn the matching lands into creatures for “All Swamps are 1/1 black creatures that are still lands”.

// LEA 257 — Library of Leng
// Audit: blocked — Needs ordered-library inspection, selection, and visibility handling for “If an effect causes you to discard a card, discard it, but you may put it on top of your library instead of into your graveyard”.

// LEA 258 — Living Wall
// Audit: blocked — Needs regeneration shields and their destroy-event replacement procedure for “{1}: Regenerate this creature”.

// LEA 259 — Mana Vault
pub(in crate::card::sets) static MANA_VAULT: CardRecord = CardRecord::new(
    cards::MANA_VAULT,
    "Mana Vault",
    CardArt::new("19499cb7-eccb-4e69-af32-6002d447a160", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::DoesNotUntapDuringUntapStep,
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {4}. If you do, untap this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::OptionalPayment {
                payment: PaymentDef::new(
                    PlayerRelation::You,
                    &[CostDef::Mana(mana_cost!("{4}"))],
                ),
                if_paid: &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            },
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
// Audit: blocked — Needs a characteristic-layer effect or dynamic value for “Creatures with power 3 or greater don't untap during their controllers' untap steps”.

// LEA 261 — Mox Emerald
pub(in crate::card::sets) static MOX_EMERALD: CardRecord = CardRecord::new(
    cards::MOX_EMERALD,
    "Mox Emerald",
    CardArt::new("b0e1427c-05cd-465b-be59-97ed6e39f7ba", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Green)]),
);

// LEA 262 — Mox Jet
pub(in crate::card::sets) static MOX_JET: CardRecord = CardRecord::new(
    cards::MOX_JET,
    "Mox Jet",
    CardArt::new("92bcd1ce-19b1-4d78-8b09-95242ca08d76", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Black)]),
);

// LEA 263 — Mox Pearl
pub(in crate::card::sets) static MOX_PEARL: CardRecord = CardRecord::new(
    cards::MOX_PEARL,
    "Mox Pearl",
    CardArt::new("8ebe4be7-e12a-4596-a899-fbd5b152e879", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::White)]),
);

// LEA 264 — Mox Ruby
pub(in crate::card::sets) static MOX_RUBY: CardRecord = CardRecord::new(
    cards::MOX_RUBY,
    "Mox Ruby",
    CardArt::new("8945585f-4773-493d-a0fe-d707db910b38", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Red)]),
);

// LEA 265 — Mox Sapphire
pub(in crate::card::sets) static MOX_SAPPHIRE: CardRecord = CardRecord::new(
    cards::MOX_SAPPHIRE,
    "Mox Sapphire",
    CardArt::new("82da0972-b17b-4600-9efd-e9430a0db04b", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{0}"))
        .with_abilities(&[abilities::tap_for(ManaColor::Blue)]),
);

// LEA 266 — Nevinyrral's Disk
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
            EffectDef::Destroy {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                can_regenerate: true,
            },
        ),
    ]),
);

// LEA 267 — Obsianus Golem
pub(in crate::card::sets) static OBSIANUS_GOLEM: CardRecord = CardRecord::new(
    cards::OBSIANUS_GOLEM,
    "Obsianus Golem",
    CardArt::new("4c8e9f5c-deba-4443-bf9d-fb2be75c5418", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Golem"], 4, 6),
);

// LEA 268 — Rod of Ruin
pub(in crate::card::sets) static ROD_OF_RUIN: CardRecord = CardRecord::new(
    cards::ROD_OF_RUIN,
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

// LEA 270 — Soul Net
pub(in crate::card::sets) static SOUL_NET: CardRecord = CardRecord::new(
    cards::SOUL_NET,
    "Soul Net",
    CardArt::new("2b814198-814b-4619-a158-327af675f8f2", "Dameon Willich"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a creature dies, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            from: Some(ZoneKind::Battlefield),
            to: Some(ZoneKind::Graveyard),
        },
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{1}"))]),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// LEA 271 — Sunglasses of Urza
// Audit: blocked — Needs cost/mana provenance or dynamic payment support for “You may spend white mana as though it were red mana”.

// LEA 272 — The Hive
pub(in crate::card::sets) static THE_HIVE: CardRecord = CardRecord::new(
    cards::THE_HIVE,
    "The Hive",
    CardArt::new("544a7138-eae8-4ff9-9e17-680bfa717183", "Sandra Everingham"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{5}, {T}: Create a 1/1 colorless Insect artifact creature token with flying named Wasp.",
        &[
            AbilityCostDef::Mana(mana_cost!("{5}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::CreateToken {
            token: cards::WASP_TOKEN_1_1_COLORLESS,
            count: ValueDef::Constant(1),
        },
    )),
);

// LEA 273 — Throne of Bone
pub(in crate::card::sets) static THRONE_OF_BONE: CardRecord = CardRecord::new(
    cards::THRONE_OF_BONE,
    "Throne of Bone",
    CardArt::new("a2931ae0-7836-4000-b9ec-f2029ebf5d96", "Anson Maddocks"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a black spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Black)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{1}"))]),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// LEA 274 — Time Vault
// Audit: partial — Its skip-turn replacement is offered after the turn begins and banks a skip for the controller's next turn instead of replacing the current turn.
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

// LEA 275 — Winter Orb
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

// LEA 276 — Wooden Sphere
pub(in crate::card::sets) static WOODEN_SPHERE: CardRecord = CardRecord::new(
    cards::WOODEN_SPHERE,
    "Wooden Sphere",
    CardArt::new("bcae01a2-171b-47cd-87be-f1e4e5314326", "Mark Tedin"),
    CardSet::Alpha,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a player casts a green spell, you may pay {1}. If you do, you gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::Color(ManaColor::Green)),
        EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &[CostDef::Mana(mana_cost!("{1}"))]),
            if_paid: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )]),
);

// LEA 277 — Badlands
pub(in crate::card::sets) static BADLANDS: CardRecord = CardRecord::new(
    cards::BADLANDS,
    "Badlands",
    CardArt::new("717f6d10-9144-4ade-9ac6-a481cc66b875", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp", "Mountain"]),
);

// LEA 278 — Bayou
pub(in crate::card::sets) static BAYOU: CardRecord = CardRecord::new(
    cards::BAYOU,
    "Bayou",
    CardArt::new("412ceddd-2b9a-4551-a6bf-ae2830a2010a", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp", "Forest"]),
);

// LEA 279 — Plateau
pub(in crate::card::sets) static PLATEAU: CardRecord = CardRecord::new(
    cards::PLATEAU,
    "Plateau",
    CardArt::new("6eafa00b-c628-40f6-86eb-88e1361fc7a0", "Drew Tucker"),
    CardSet::Alpha,
    CardRules::new_land(&["Mountain", "Plains"]),
);

// LEA 280 — Savannah
pub(in crate::card::sets) static SAVANNAH: CardRecord = CardRecord::new(
    cards::SAVANNAH,
    "Savannah",
    CardArt::new("94f7e24c-2546-41b6-81ad-5e920b07e64e", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Plains"]),
);

// LEA 281 — Scrubland
pub(in crate::card::sets) static SCRUBLAND: CardRecord = CardRecord::new(
    cards::SCRUBLAND,
    "Scrubland",
    CardArt::new("bebe39d4-21fb-46a4-a1ec-b97102e46c15", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains", "Swamp"]),
);

// LEA 282 — Taiga
pub(in crate::card::sets) static TAIGA: CardRecord = CardRecord::new(
    cards::TAIGA,
    "Taiga",
    CardArt::new("60df6592-0b3b-4b87-aeb2-8fa94b4fb7be", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Mountain"]),
);

// LEA 283 — Tropical Island
pub(in crate::card::sets) static TROPICAL_ISLAND: CardRecord = CardRecord::new(
    cards::TROPICAL_ISLAND,
    "Tropical Island",
    CardArt::new("a9c6c759-aabf-44e7-ba8c-33c5df232b56", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest", "Island"]),
);

// LEA 284 — Tundra
pub(in crate::card::sets) static TUNDRA: CardRecord = CardRecord::new(
    cards::TUNDRA,
    "Tundra",
    CardArt::new("a03e8c5b-f4ed-4fd7-ba05-db813ccc05eb", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains", "Island"]),
);

// LEA 285 — Underground Sea
pub(in crate::card::sets) static UNDERGROUND_SEA: CardRecord = CardRecord::new(
    cards::UNDERGROUND_SEA,
    "Underground Sea",
    CardArt::new("ff76ac86-8a8a-47fe-9388-8950ca3e26c3", "Rob Alexander"),
    CardSet::Alpha,
    CardRules::new_land(&["Island", "Swamp"]),
);

// LEA 286 — Plains
pub(in crate::card::sets) static PLAINS: CardRecord = CardRecord::new(
    cards::PLAINS,
    "Plains",
    CardArt::new("b1623d57-4729-4796-b3f7-f1837a05c6ed", "Jesper Myrfors"),
    CardSet::Alpha,
    CardRules::new_land(&["Plains"]).with_supertype(CardSupertype::Basic),
);

// LEA 288 — Island
pub(in crate::card::sets) static ISLAND: CardRecord = CardRecord::new(
    cards::ISLAND,
    "Island",
    CardArt::new("90a57c0e-fa61-45ef-955d-d296403967d5", "Mark Poole"),
    CardSet::Alpha,
    CardRules::new_land(&["Island"]).with_supertype(CardSupertype::Basic),
);

// LEA 290 — Swamp
pub(in crate::card::sets) static SWAMP: CardRecord = CardRecord::new(
    cards::SWAMP,
    "Swamp",
    CardArt::new("6176936d-72e2-4205-8871-4c5a4f1cb2d8", "Dan Frazier"),
    CardSet::Alpha,
    CardRules::new_land(&["Swamp"]).with_supertype(CardSupertype::Basic),
);

// LEA 292 — Mountain
pub(in crate::card::sets) static MOUNTAIN: CardRecord = CardRecord::new(
    cards::MOUNTAIN,
    "Mountain",
    CardArt::new("eace2c85-976c-425e-9800-5a6ccbd91b56", "Douglas Shuler"),
    CardSet::Alpha,
    CardRules::new_land(&["Mountain"]).with_supertype(CardSupertype::Basic),
);

// LEA 294 — Forest
pub(in crate::card::sets) static FOREST: CardRecord = CardRecord::new(
    cards::FOREST,
    "Forest",
    CardArt::new("6f1c8cb0-38eb-408b-94e8-16db83999b3b", "Christopher Rush"),
    CardSet::Alpha,
    CardRules::new_land(&["Forest"]).with_supertype(CardSupertype::Basic),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARMAGEDDON,
    &BALANCE,
    &BLESSING,
    &CONVERSION,
    &CRUSADE,
    &DISENCHANT,
    &FARMSTEAD,
    &HOLY_ARMOR,
    &HOLY_STRENGTH,
    &KARMA,
    &LANCE,
    &NORTHERN_PALADIN,
    &PEARLED_UNICORN,
    &RESURRECTION,
    &RIGHTEOUSNESS,
    &SAVANNAH_LIONS,
    &SERRA_ANGEL,
    &SWORDS_TO_PLOWSHARES,
    &WALL_OF_SWORDS,
    &WHITE_KNIGHT,
    &WRATH_OF_GOD,
    &AIR_ELEMENTAL,
    &ANCESTRAL_RECALL,
    &BLUE_ELEMENTAL_BLAST,
    &BRAINGEYSER,
    &CLONE,
    &COPY_ARTIFACT,
    &COUNTERSPELL,
    &FLIGHT,
    &INVISIBILITY,
    &JUMP,
    &LIFETAP,
    &MAGICAL_HACK,
    &MAHAMOTI_DJINN,
    &MANA_SHORT,
    &MERFOLK_OF_THE_PEARL_TRIDENT,
    &PHANTASMAL_FORCES,
    &PHANTOM_MONSTER,
    &PRODIGAL_SORCERER,
    &PSIONIC_BLAST,
    &SPELL_BLAST,
    &STASIS,
    &TIME_WALK,
    &TIMETWISTER,
    &TWIDDLE,
    &UNSUMMON,
    &WALL_OF_AIR,
    &WALL_OF_WATER,
    &WATER_ELEMENTAL,
    &BAD_MOON,
    &BLACK_KNIGHT,
    &DARK_RITUAL,
    &DEATHGRIP,
    &DEMONIC_TUTOR,
    &DRAIN_LIFE,
    &EVIL_PRESENCE,
    &FEAR,
    &FROZEN_SHADE,
    &HOWL_FROM_BEYOND,
    &HYPNOTIC_SPECTER,
    &MIND_TWIST,
    &PESTILENCE,
    &RAISE_DEAD,
    &SCATHE_ZOMBIES,
    &SENGIR_VAMPIRE,
    &SINKHOLE,
    &TERROR,
    &UNHOLY_STRENGTH,
    &WEAKNESS,
    &BURROWING,
    &DRAGON_WHELP,
    &DWARVEN_DEMOLITION_TEAM,
    &DWARVEN_WARRIORS,
    &EARTH_ELEMENTAL,
    &EARTHQUAKE,
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
    &LIGHTNING_BOLT,
    &MANABARBS,
    &MONSS_GOBLIN_RAIDERS,
    &ORCISH_ARTILLERY,
    &ORCISH_ORIFLAMME,
    &RED_ELEMENTAL_BLAST,
    &ROC_OF_KHER_RIDGES,
    &SEDGE_TROLL,
    &SHATTER,
    &SHIVAN_DRAGON,
    &SMOKE,
    &STONE_GIANT,
    &STONE_RAIN,
    &TUNNEL,
    &WALL_OF_FIRE,
    &WALL_OF_STONE,
    &WHEEL_OF_FORTUNE,
    &BERSERK,
    &BIRDS_OF_PARADISE,
    &CHANNEL,
    &CRAW_WURM,
    &ELVISH_ARCHERS,
    &FORCE_OF_NATURE,
    &FUNGUSAUR,
    &GIANT_GROWTH,
    &GIANT_SPIDER,
    &GRIZZLY_BEARS,
    &HURRICANE,
    &ICE_STORM,
    &IRONROOT_TREEFOLK,
    &LEY_DRUID,
    &LIFEFORCE,
    &LLANOWAR_ELVES,
    &REGROWTH,
    &SCRYB_SPRITES,
    &SHANODIN_DRYADS,
    &STREAM_OF_LIFE,
    &TRANQUILITY,
    &TSUNAMI,
    &VERDURAN_ENCHANTRESS,
    &WALL_OF_ICE,
    &WALL_OF_WOOD,
    &WAR_MAMMOTH,
    &WEB,
    &ANKH_OF_MISHRA,
    &BASALT_MONOLITH,
    &BLACK_LOTUS,
    &BLACK_VISE,
    &CELESTIAL_PRISM,
    &CHAOS_ORB,
    &COPPER_TABLET,
    &CRYSTAL_ROD,
    &DINGUS_EGG,
    &GLASSES_OF_URZA,
    &ICY_MANIPULATOR,
    &IRON_STAR,
    &IVORY_CUP,
    &JAYEMDAE_TOME,
    &JUGGERNAUT,
    &MANA_VAULT,
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
        AppliedEffectDef, DeclarativeAbilityDef, EffectDef, EffectDurationDef, ValueDef, abilities,
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
        let EffectDef::Apply {
            effect: AppliedEffectDef::Composite(effects),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
            ..
        } = clauses[0].effect.definition
        else {
            panic!("Goblin King's one static ability must apply one composite effect");
        };
        assert_eq!(effects.len(), 2);
        assert_eq!(
            effects[0],
            AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(1),
            }
        );
        assert!(matches!(
            effects[1],
            AppliedEffectDef::GrantAbility(ability) if *ability == abilities::mountainwalk()
        ));
    }
}
