use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef,
    AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, BasicLandType, CardArt, CardBehavior, CardChoiceSourceDef, CardRules, CardSet,
    CardSupertype, CardType, ComparisonDef, DamageCoverageDef, DamageEventMatcherDef,
    DamagePreventionDef, DamageRecipientMatcherDef, DamageSourceGroupDef, DiscardSelectionDef,
    EffectDef, EffectExecutionDef, EffectPaymentDef, EffectRecipientDef, HalvedValueDef,
    KeywordAbility, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, RoundingDef,
    SacrificedAmountDef, TargetChooserDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ObjectBindingIndex, TargetIndex};
use crate::mana_cost;

static TARGET_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

/// Read live off the attachment, so a land that stops being a Mountain --
/// or an Aura that moves -- switches the anthem off at once.
static ENCHANTED_LAND_IS_A_BASIC_MOUNTAIN: TriggerConditionDef =
    TriggerConditionDef::AttachedPermanentMatches {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
        ]),
    };

static FROM_YOUR_HAND: [CardChoiceSourceDef; 1] = [CardChoiceSourceDef::Zone(ZoneKind::Hand)];

// DRK 1 — Angry Mob
// Audit: metadata-only — Needs a characteristic-layer effect or dynamic value for “During your turn, Angry Mob's power and toughness are each equal to 2 plus the number of Swamps your opponents control. During turns other than yours, Angry Mob's power and toughness are…”.
pub(in crate::card::sets) static ANGRY_MOB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e14db1c-0a05-47d2-9f27-df881f7f37ab"),
    "Angry Mob",
    crate::card::CardArt::new("9e14db1c-0a05-47d2-9f27-df881f7f37ab", "Drew Tucker"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 2 — Blood of the Martyr
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “Until end of turn, if damage would be dealt to any creature, you may have that damage dealt to you instead”.
pub(in crate::card::sets) static BLOOD_OF_THE_MARTYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22d4761d-acf2-4cb3-86a8-a3f30420a92e"),
    "Blood of the Martyr",
    crate::card::CardArt::new("22d4761d-acf2-4cb3-86a8-a3f30420a92e", "Christopher Rush"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 3 — Brainwash
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “Enchanted creature can't attack unless its controller pays {3}”.
pub(in crate::card::sets) static BRAINWASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6da4fb5a-0d24-4bee-b3f5-535ba9fe6850"),
    "Brainwash",
    crate::card::CardArt::new("6da4fb5a-0d24-4bee-b3f5-535ba9fe6850", "Pete Venters"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 4 — Cleansing
// Audit: metadata-only — Needs linked sacrifice/destruction accounting for “For each land, destroy that land unless any player pays 1 life”.
pub(in crate::card::sets) static CLEANSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc1973a3-1410-4c6d-9b09-bd9d18646a1e"),
    "Cleansing",
    crate::card::CardArt::new("fc1973a3-1410-4c6d-9b09-bd9d18646a1e", "Pete Venters"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 5 — Dust to Dust
// Audit: custom — Needs migration to declarative exact-two artifact targeting and exile resolution.
pub(in crate::card::sets) static DUST_TO_DUST: CardRecord = CardRecord::new_with_legacy_id(
    112,
    "Dust to Dust",
    CardArt::new("ade075fd-73ee-4d12-a2da-48e5938043af", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::custom_full(
        "Exile two target artifacts.",
        CardBehavior::DustToDust,
        "Artifact targeting and exile are implemented by the legacy spell resolver.",
    )]),
);

// DRK 6 — Exorcist
pub(in crate::card::sets) static EXORCIST: CardRecord = CardRecord::new_with_legacy_id(
    541,
    "Exorcist",
    CardArt::new("184b7d52-e991-4668-9f6a-bcded97f51ac", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{W}, {T}: Destroy target black creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Black),
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

// DRK 7 — Fasting
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “If you would begin your draw step, you may skip that step instead. If you do, you gain 2 life”.
pub(in crate::card::sets) static FASTING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8da35f9f-e72c-4154-a212-7de98f84ad7d"),
    "Fasting",
    crate::card::CardArt::new("8da35f9f-e72c-4154-a212-7de98f84ad7d", "Douglas Shuler"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 8 — Festival
pub(in crate::card::sets) static FESTIVAL: CardRecord = CardRecord::new_with_legacy_id(
    1822,
    "Festival",
    CardArt::new("e9357990-701a-4336-b545-ac5a24d89cad", "Mark Poole"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{W}"))
        .cast_only_during_opponents_upkeep()
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Cast this spell only during an opponent's upkeep.",
                "The play option refuses the cast outside an opponent's upkeep step.",
            ),
            AbilityDef::spell(
                "Creatures can't attack this turn.",
                // The creatures on the battlefield as this resolves, which is
                // every creature that could attack: the window is an upkeep,
                // so the attack is still two steps away.
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// DRK 9 — Fire and Brimstone
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “Fire and Brimstone deals 4 damage to target player who attacked this turn and 4 damage to you”.
pub(in crate::card::sets) static FIRE_AND_BRIMSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5208dbb-63d2-4789-8ef9-f82499a43b3a"),
    "Fire and Brimstone",
    crate::card::CardArt::new("d5208dbb-63d2-4789-8ef9-f82499a43b3a", "Jeff A. Menges"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 10 — Holy Light
pub(in crate::card::sets) static HOLY_LIGHT: CardRecord = CardRecord::new_with_legacy_id(
    542,
    "Holy Light",
    CardArt::new("c3c8a850-bc99-4679-a316-45ecdea696b2", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell(
        "Nonwhite creatures get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 11 — Knights of Thorn
pub(in crate::card::sets) static KNIGHTS_OF_THORN: CardRecord = CardRecord::new_with_legacy_id(
    1777,
    "Knights of Thorn",
    CardArt::new("ae541c73-9903-49e6-997a-db4701135145", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Red),
        abilities::banding(),
    ]),
);

// DRK 12 — Martyr's Cry
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “Exile all white creatures. For each creature exiled this way, its controller draws a card”.
pub(in crate::card::sets) static MARTYR_S_CRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2c9f463-d1cc-4f11-aad2-d4a4520aa978"),
    "Martyr's Cry",
    crate::card::CardArt::new("e2c9f463-d1cc-4f11-aad2-d4a4520aa978", "Jeff A. Menges"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 13 — Miracle Worker
/// "Attached to a creature you control": the Aura may be either player's, but
/// the creature under it has to be one of yours.
static MIRACLE_WORKER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Subtype("Aura"),
        ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
    ]),
)];

pub(in crate::card::sets) static MIRACLE_WORKER: CardRecord = CardRecord::new_with_legacy_id(
    1678,
    "Miracle Worker",
    CardArt::new("35d29bda-096c-44d4-b45e-c2c507f8efbe", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Destroy target Aura attached to a creature you control.",
            &[AbilityCostDef::TapSource],
            &MIRACLE_WORKER_TARGET,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// DRK 14 — Morale
pub(in crate::card::sets) static MORALE: CardRecord = CardRecord::new_with_legacy_id(
    543,
    "Morale",
    CardArt::new("c4104546-abd9-4bfb-a65e-5928cdd4522f", "Mark Poole"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Attacking creatures get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 15 — Pikemen
pub(in crate::card::sets) static PIKEMEN: CardRecord = CardRecord::new_with_legacy_id(
    1778,
    "Pikemen",
    CardArt::new("bf2f6936-b50c-4907-9b55-ebf8a3fba8f5", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 1)
        .with_abilities(&[abilities::first_strike(), abilities::banding()]),
);

// DRK 16 — Preacher
// Audit: metadata-only — Needs duration-aware control-changing continuous effects for “{T}: For as long as this creature remains tapped, gain control of target creature of an opponent's choice they control”.
pub(in crate::card::sets) static PREACHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e03d335-d259-4ab4-814f-9333cfd3afc9"),
    "Preacher",
    crate::card::CardArt::new("1e03d335-d259-4ab4-814f-9333cfd3afc9", "Quinton Hoover"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 17 — Squire
pub(in crate::card::sets) static SQUIRE: CardRecord = CardRecord::new_with_legacy_id(
    544,
    "Squire",
    CardArt::new("374df061-ebd2-4f1f-9a6e-7940a49197a9", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 1, 2),
);

// DRK 18 — Tivadar's Crusade
pub(in crate::card::sets) static TIVADARS_CRUSADE: CardRecord = CardRecord::new_with_legacy_id(
    545,
    "Tivadar's Crusade",
    CardArt::new("8b6da540-6803-47e5-9af0-7ae8e2f84b6c", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Destroy all Goblins.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Subtype("Goblin"),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )]),
);

// DRK 19 — Witch Hunter
pub(in crate::card::sets) static WITCH_HUNTER: CardRecord = CardRecord::new_with_legacy_id(
    546,
    "Witch Hunter",
    CardArt::new("4eef9bb7-cd3c-422e-a93b-90d98684675a", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target player or planeswalker.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{W}{W}, {T}: Return target creature an opponent controls to its owner's hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}{W}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
    ]),
);

// DRK 20 — Amnesia
static AMNESIA_STRIKE: [EffectDef; 2] = [
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    // "All nonland cards" is not a count, so the hand is queried rather than
    // a number of discards being asked for. The reveal above is what makes
    // the selection public knowledge.
    EffectDef::DiscardCards {
        object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        ))),
    },
];

pub(in crate::card::sets) static AMNESIA: CardRecord = CardRecord::new_with_legacy_id(
    1727,
    "Amnesia",
    CardArt::new("5b650e75-28ae-4f9e-9a04-7e28a246693f", "Mark Poole"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target player reveals their hand and discards all nonland cards.",
            &TARGET_PLAYER,
            EffectDef::Sequence(&AMNESIA_STRIKE),
        ),
    ),
);

// DRK 21 — Apprentice Wizard
pub(in crate::card::sets) static APPRENTICE_WIZARD: CardRecord = CardRecord::new_with_legacy_id(
    1635,
    "Apprentice Wizard",
    CardArt::new("151b332e-164b-4646-8f52-741984cd71ad", "Dan Frazier"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Human", "Wizard"], 0, 1).with_ability(
        AbilityDef::activated_mana(
            "{U}, {T}: Add {C}{C}{C}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
        ),
    ),
);

// DRK 22 — Dance of Many
// Audit: metadata-only — Needs copiable-value or rules-text mutation support for “When this enchantment enters, create a token that's a copy of target nontoken creature”.
pub(in crate::card::sets) static DANCE_OF_MANY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13453abe-3f05-4956-8493-382d7d2af699"),
    "Dance of Many",
    crate::card::CardArt::new("13453abe-3f05-4956-8493-382d7d2af699", "Sandra Everingham"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 23 — Deep Water
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “{U}: Until end of turn, if you tap a land you control for mana, it produces {U} instead of any other type”.
pub(in crate::card::sets) static DEEP_WATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9dd6a230-6bc0-499c-b7fd-4aaa2569f98f"),
    "Deep Water",
    crate::card::CardArt::new("9dd6a230-6bc0-499c-b7fd-4aaa2569f98f", "Jeff A. Menges"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 24 — Drowned
pub(in crate::card::sets) static DROWNED: CardRecord = CardRecord::new_with_legacy_id(
    1376,
    "Drowned",
    CardArt::new("951b6c10-cbba-44b6-aae2-2c386b7ebacb", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Zombie"], 1, 1).with_abilities(&[
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// DRK 25 — Electric Eel
pub(in crate::card::sets) static ELECTRIC_EEL: CardRecord = CardRecord::new_with_legacy_id(
    547,
    "Electric Eel",
    CardArt::new("b8834c18-0e4e-4785-9d15-b33345e3789b", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{U}"), &["Fish"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, it deals 1 damage to you.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{R}{R}: This creature gets +2/+0 until end of turn and deals 1 damage to you.",
            &[AbilityCostDef::Mana(mana_cost!("{R}{R}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DRK 26 — Erosion
// Audit: metadata-only — Needs an upkeep trigger whose event player is derived from the attached permanent's current controller for “At the beginning of the upkeep of enchanted land's controller, destroy that land unless that player pays {1} or 1 life”.
pub(in crate::card::sets) static EROSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f4b6507-89ee-482e-aafd-8e05ada8f1ce"),
    "Erosion",
    crate::card::CardArt::new("5f4b6507-89ee-482e-aafd-8e05ada8f1ce", "Pete Venters"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 27 — Flood
pub(in crate::card::sets) static FLOOD: CardRecord = CardRecord::new_with_legacy_id(
    548,
    "Flood",
    CardArt::new("fabc3267-b59b-4f36-8873-5b4b072711ca", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{U}{U}: Tap target creature without flying.",
            &[AbilityCostDef::Mana(mana_cost!("{U}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// DRK 28 — Ghost Ship
pub(in crate::card::sets) static GHOST_SHIP: CardRecord = CardRecord::new_with_legacy_id(
    1377,
    "Ghost Ship",
    CardArt::new("db591b28-37e5-4e7c-ae4d-d761262b12d0", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Spirit"], 2, 4).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "{U}{U}{U}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}{U}{U}"))],
        ),
    ]),
);

// DRK 29 — Giant Shark
static GIANT_SHARK_DEFENDER_HAS_AN_ISLAND: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

static GIANT_SHARK_NO_ISLANDS: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::Equal,
    amount: 0,
};

static GIANT_SHARK_TRAMPLE: AbilityDef = abilities::trample();

static GIANT_SHARK_FRENZY: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
    AppliedEffectDef::add_ability(&GIANT_SHARK_TRAMPLE),
];

pub(in crate::card::sets) static GIANT_SHARK: CardRecord = CardRecord::new_with_legacy_id(
    1904,
    "Giant Shark",
    CardArt::new("53ec4a19-0f2f-4713-a869-58832484648d", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Shark"], 4, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            EffectDef::CannotAttackUnless(&GIANT_SHARK_DEFENDER_HAS_AN_ISLAND),
        ),
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a creature that has been dealt \
             damage this turn, this creature gets +2/+0 and gains trample until end of turn.",
            // Blood in the water: the other creature's damage this turn, not
            // the marks still showing on it, so a regenerated one still counts.
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::WasDealtDamageThisTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&GIANT_SHARK_FRENZY),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_if(
            "When you control no Islands, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &GIANT_SHARK_NO_ISLANDS,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// DRK 30 — Leviathan
// Audit: metadata-only — Needs a persistent tap/untap restriction or event relation for “This creature enters tapped and doesn't untap during your untap step”.
pub(in crate::card::sets) static LEVIATHAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b638d9be-c533-45c3-92f9-fabf56edc2df"),
    "Leviathan",
    crate::card::CardArt::new("b638d9be-c533-45c3-92f9-fabf56edc2df", "Mark Tedin"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 31 — Mana Vortex
// Audit: metadata-only — Needs linked sacrifice/destruction accounting for “At the beginning of each player's upkeep, that player sacrifices a land of their choice”.
pub(in crate::card::sets) static MANA_VORTEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f857a00a-82e0-4227-86ee-1f9c7ca232ae"),
    "Mana Vortex",
    crate::card::CardArt::new("f857a00a-82e0-4227-86ee-1f9c7ca232ae", "Douglas Shuler"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 32 — Merfolk Assassin
static ISLANDWALKER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasKeyword(KeywordAbility::Landwalk(BasicLandType::Island)),
    ]),
)];

pub(in crate::card::sets) static MERFOLK_ASSASSIN: CardRecord = CardRecord::new_with_legacy_id(
    1436,
    "Merfolk Assassin",
    CardArt::new("36313dc7-6bf2-4d73-b696-969d984a7466", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk", "Assassin"], 1, 2).with_abilities(
        &[AbilityDef::activated_with_targets(
            "{T}: Destroy target creature with islandwalk.",
            &[AbilityCostDef::TapSource],
            &ISLANDWALKER_TARGET,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        )],
    ),
);

// DRK 33 — Mind Bomb
// Audit: metadata-only — Needs an ordered choice for each player to discard up to three cards and damage derived from each actual discard count.
pub(in crate::card::sets) static MIND_BOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ee810a5-f0f9-4b73-8194-3d1344784050"),
    "Mind Bomb",
    crate::card::CardArt::new("0ee810a5-f0f9-4b73-8194-3d1344784050", "Mark Tedin"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 34 — Psychic Allergy
// Audit: metadata-only — Needs a persistent dynamic characteristic choice and predicates that consume it for “At the beginning of each opponent's upkeep, this enchantment deals X damage to that player, where X is the number of nontoken permanents of the chosen color they control”.
pub(in crate::card::sets) static PSYCHIC_ALLERGY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fec3275e-4491-43a8-9f23-d7b48177c103"),
    "Psychic Allergy",
    crate::card::CardArt::new("fec3275e-4491-43a8-9f23-d7b48177c103", "Mark Tedin"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 35 — Riptide
pub(in crate::card::sets) static RIPTIDE: CardRecord = CardRecord::new_with_legacy_id(
    549,
    "Riptide",
    CardArt::new(
        "b0f11ae4-e30e-441d-bb64-439930d9997c",
        "Randy Asplund-Faith",
    ),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell(
        "Tap all blue creatures.",
        EffectDef::Tap {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
        },
    )]),
);

// DRK 36 — Sunken City
pub(in crate::card::sets) static SUNKEN_CITY: CardRecord = CardRecord::new_with_legacy_id(
    550,
    "Sunken City",
    CardArt::new("f1e0f9ec-2b06-4bda-8b80-a716d82d1f13", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{U}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {U}{U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{U}{U}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::static_ability(
            "Blue creatures get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Blue),
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
    ]),
);

// DRK 37 — Tangle Kelp
/// The condition sits on the recipient rather than on the Aura, so the Kelp
/// holds a creature down only on the untap step after it swung. A creature
/// that stayed home unties itself.
static TANGLE_KELP_HOST_THAT_ATTACKED: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::AttachedToSource,
        ObjectPredicateDef::AttackedDuringControllersLastTurn,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static TANGLE_KELP: CardRecord = CardRecord::new_with_legacy_id(
    1724,
    "Tangle Kelp",
    CardArt::new("8ba55fc4-62bb-4515-a209-e914d8cbb303", "Rob Alexander"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step if it \
                 attacked during its controller's last turn.",
                EffectDef::StaticApply {
                    recipient: TANGLE_KELP_HOST_THAT_ATTACKED,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// DRK 38 — Water Wurm
static WATER_WURM_OPPONENT_ISLAND: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[crate::card::BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

pub(in crate::card::sets) static WATER_WURM: CardRecord = CardRecord::new_with_legacy_id(
    575,
    "Water Wurm",
    CardArt::new("e3da4a88-5225-467f-9240-f30bc1eee520", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{U}"), &["Wurm"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +0/+1 as long as an opponent controls an Island.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::AnyMatchingObject(&WATER_WURM_OPPONENT_ISLAND),
                ),
            },
        ),
    ]),
);

// DRK 39 — Ashes to Ashes
pub(in crate::card::sets) static ASHES_TO_ASHES: CardRecord = CardRecord::new_with_legacy_id(
    551,
    "Ashes to Ashes",
    CardArt::new("825496e5-19c7-4f50-8070-0265a58608dc", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile two target nonartifact creatures. Ashes to Ashes deals 5 damage to you.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 2,
                maximum: 2,
                divided_total: None,
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    tapped: false,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            ]),
        ),
    ]),
);

// DRK 40 — Banshee
// Audit: metadata-only — Needs damage-history/source tracking or card-specific damage processing for “{X}, {T}: This creature deals half X damage, rounded down, to any target, and half X damage, rounded up, to you”.
pub(in crate::card::sets) static BANSHEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66eaa7d6-48b2-4b35-a834-790edd679e0e"),
    "Banshee",
    crate::card::CardArt::new("66eaa7d6-48b2-4b35-a834-790edd679e0e", "Jesper Myrfors"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 41 — Bog Imp
pub(in crate::card::sets) static BOG_IMP: CardRecord = CardRecord::new_with_legacy_id(
    552,
    "Bog Imp",
    CardArt::new("e3bb7271-634a-4612-9073-7a5438e8c2b8", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Imp"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// DRK 42 — Bog Rats
pub(in crate::card::sets) static BOG_RATS: CardRecord = CardRecord::new_with_legacy_id(
    553,
    "Bog Rats",
    CardArt::new("d64c9153-bc6d-4a64-885f-c039a5487a31", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1).with_abilities(&[
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

// DRK 43 — Curse Artifact
/// The declined branch. "That player" is the artifact's controller, so
/// stealing the artifact moves both the choice and the damage with it.
static CURSE_ARTIFACT_TOLL: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
        ObjectRefDef::AttachedToSource,
    )),
    amount: ValueDef::Constant(2),
};

pub(in crate::card::sets) static CURSE_ARTIFACT: CardRecord = CardRecord::new_with_legacy_id(
    1966,
    "Curse Artifact",
    CardArt::new("9fc0d070-8a42-4d5e-8f2b-ceb59147de6f", "Mark Tedin"),
    CardSet::TheDark,
    // "That artifact" is the one this Aura is on, so the offer names exactly
    // one permanent rather than any the player controls.
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_artifact(),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted artifact's controller, this Aura \
                 deals 2 damage to that player unless they sacrifice that artifact.",
                EffectDef::SacrificeOfChoice {
                    count: ValueDef::Constant(1),
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    object: ObjectPredicateDef::AttachedToSource,
                    then: None,
                    otherwise: Some(&CURSE_ARTIFACT_TOLL),
                    amount: SacrificedAmountDef::Power,
                    optional: true,
                },
            ),
        ]),
);

// DRK 44 — Eater of the Dead
// Audit: metadata-only — Needs a zone-object query and identity-preserving continuation for “{0}: If this creature is tapped, exile target creature card from a graveyard and untap this creature”.
pub(in crate::card::sets) static EATER_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d89fe2be-bb7e-4bae-9b1f-9f0d58f20ceb"),
    "Eater of the Dead",
    crate::card::CardArt::new("d89fe2be-bb7e-4bae-9b1f-9f0d58f20ceb", "Jesper Myrfors"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 45 — Frankenstein's Monster
// Audit: metadata-only — Needs a duration-scoped replacement/prevention effect for “As this creature enters, exile X creature cards from your graveyard. If you can't, put this creature into its owner's graveyard instead of onto the battlefield. For each creature card…”.
pub(in crate::card::sets) static FRANKENSTEIN_S_MONSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f99894d-5ece-44f1-acce-474494ae2084"),
    "Frankenstein's Monster",
    crate::card::CardArt::new("8f99894d-5ece-44f1-acce-474494ae2084", "Anson Maddocks"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 46 — Grave Robbers
pub(in crate::card::sets) static GRAVE_ROBBERS: CardRecord = CardRecord::new_with_legacy_id(
    555,
    "Grave Robbers",
    CardArt::new("a131605a-f646-4745-a1e4-48d155a3d94f", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{B}, {T}: Exile target artifact card from a graveyard. You gain 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    counters: None,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                    attachment: None,
                    tapped: false,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// DRK 47 — Inquisition
/// Counted after the reveal, from the hand itself: the damage is whatever is
/// there when the spell resolves, not what the caster saw earlier.
static WHITE_CARDS_IN_TARGETS_HAND: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Color(ManaColor::White),
    &[ZoneKind::Hand],
    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
);

static INQUISITION_STRIKE: [EffectDef; 2] = [
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::CountMatchingObjects(&WHITE_CARDS_IN_TARGETS_HAND),
    },
];

pub(in crate::card::sets) static INQUISITION: CardRecord = CardRecord::new_with_legacy_id(
    1728,
    "Inquisition",
    CardArt::new("be36c273-8584-4a8d-b253-b45449300b63", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player reveals their hand. Inquisition deals damage to that player equal to the \
         number of white cards in their hand.",
        &TARGET_PLAYER,
        EffectDef::Sequence(&INQUISITION_STRIKE),
    )),
);

// DRK 48 — Marsh Gas
pub(in crate::card::sets) static MARSH_GAS: CardRecord = CardRecord::new_with_legacy_id(
    556,
    "Marsh Gas",
    CardArt::new("b80ecb15-258b-4fc9-86e4-c2bf01891606", "Douglas Shuler"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -2/-0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 49 — Murk Dwellers
pub(in crate::card::sets) static MURK_DWELLERS: CardRecord = CardRecord::new_with_legacy_id(
    1711,
    "Murk Dwellers",
    CardArt::new("a213450f-02f4-4c08-8da8-891ebfa8e237", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks and isn't blocked, it gets +2/+0 until end of combat.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfCombat,
            },
        ),
    ),
);

// DRK 50 — Nameless Race
// Audit: metadata-only — Needs a characteristic-layer effect or dynamic value for “Nameless Race's power and toughness are each equal to the life paid as it entered”.
pub(in crate::card::sets) static NAMELESS_RACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("348a467a-4661-4fdb-af1d-9171a1a930d9"),
    "Nameless Race",
    crate::card::CardArt::new("348a467a-4661-4fdb-af1d-9171a1a930d9", "Quinton Hoover"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 51 — Rag Man
static RAG_MAN_CREATURE_CARD: ObjectPredicateDef = ObjectPredicateDef::HasType(CardType::Creature);

/// The reveal is what makes the discard public: without it the opponent would
/// learn a card left the hand and nothing about what was there to leave.
static RAG_MAN_STRIKE: [EffectDef; 2] = [
    EffectDef::RevealHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RandomMatching(&RAG_MAN_CREATURE_CARD),
        then: None,
    },
];

pub(in crate::card::sets) static RAG_MAN: CardRecord = CardRecord::new_with_legacy_id(
    1809,
    "Rag Man",
    CardArt::new("f4c133b8-8383-433f-be96-c47a937287b7", "Daniel Gelon"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Human", "Minion"], 2, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{B}{B}{B}, {T}: Target opponent reveals their hand and discards a creature card \
             at random. Activate only during your turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}{B}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&RAG_MAN_STRIKE),
        )
        .with_activation_timing(ActivationTimingDef::YourTurn),
    ),
);

// DRK 52 — Season of the Witch
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “At the beginning of the end step, destroy all untapped creatures that didn't attack this turn, except for creatures that couldn't attack”.
pub(in crate::card::sets) static SEASON_OF_THE_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06900a71-34ca-48c6-94ac-fca744356829"),
    "Season of the Witch",
    crate::card::CardArt::new("06900a71-34ca-48c6-94ac-fca744356829", "Jesper Myrfors"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 53 — The Fallen
// Audit: metadata-only — Needs per-source damage history spanning the whole game for “each opponent and planeswalker it has dealt damage to this game”.
pub(in crate::card::sets) static THE_FALLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4a176e1-b22b-4f36-ba7b-c506cb4e1bed"),
    "The Fallen",
    crate::card::CardArt::new("f4a176e1-b22b-4f36-ba7b-c506cb4e1bed", "Jesper Myrfors"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 54 — Uncle Istvan
pub(in crate::card::sets) static UNCLE_ISTVAN: CardRecord = CardRecord::new_with_legacy_id(
    557,
    "Uncle Istvan",
    CardArt::new("848ad6d5-3a7e-4d6b-9929-36465796871f", "Daniel Gelon"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{B}{B}{B}"), &["Human"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to this creature by creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::prevent_damage_from(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            },
        ),
    ]),
);

// DRK 55 — Word of Binding
/// "X target creatures": the count is the X that was paid, so an X larger
/// than the number of creatures on the battlefield has no legal declaration
/// rather than tapping fewer than were paid for.
static WORD_OF_BINDING_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_chosen_x(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

pub(in crate::card::sets) static WORD_OF_BINDING: CardRecord = CardRecord::new_with_legacy_id(
    1833,
    "Word of Binding",
    CardArt::new("ee30efdb-f1f1-497f-80a6-ec961db67c1d", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{X}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Tap X target creatures.",
        &WORD_OF_BINDING_TARGETS,
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// DRK 56 — Worms of the Earth
// Audit: metadata-only — Needs an any-player upkeep choice between sacrificing two lands and taking damage, followed by conditional self-destruction.
pub(in crate::card::sets) static WORMS_OF_THE_EARTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65a97821-ca5b-46fb-af08-86de81d0daac"),
    "Worms of the Earth",
    crate::card::CardArt::new("65a97821-ca5b-46fb-af08-86de81d0daac", "Anson Maddocks"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 57 — Ball Lightning
pub(in crate::card::sets) static BALL_LIGHTNING: CardRecord = CardRecord::new_with_legacy_id(
    3,
    "Ball Lightning",
    CardArt::new("c1ba83ab-83f5-421d-bba1-0f925870b5c8", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}{R}{R}"), &["Elemental"], 6, 1).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, sacrifice this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// DRK 58 — Blood Moon
pub(in crate::card::sets) static BLOOD_MOON: CardRecord = CardRecord::new_with_legacy_id(
    5,
    "Blood Moon",
    CardArt::new("78373616-e2d6-4ccf-998f-09f02bea45b4", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::static_ability(
        "Nonbasic lands are Mountains.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                        crate::card::CardSupertype::Basic,
                    )),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::set_basic_land_types(&[crate::card::BasicLandType::Mountain]),
        },
    )]),
);

// DRK 59 — Brothers of Fire
pub(in crate::card::sets) static BROTHERS_OF_FIRE: CardRecord = CardRecord::new_with_legacy_id(
    558,
    "Brothers of Fire",
    CardArt::new("ba2cc4a6-fdcc-4082-801a-d2c50e560e8d", "Mark Tedin"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{R}{R}: This creature deals 1 damage to any target and 1 damage to you.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DRK 60 — Cave People
pub(in crate::card::sets) static CAVE_PEOPLE: CardRecord = CardRecord::new_with_legacy_id(
    559,
    "Cave People",
    CardArt::new("72746a5d-faa1-44b7-97b5-0ef9302a3c13", "Drew Tucker"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human"], 1, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +1/-2 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}{R}, {T}: Target creature gains mountainwalk until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DRK 61 — Eternal Flame
static ETERNAL_FLAME_MOUNTAINS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// Rounded up, so an odd Mountain count costs the extra point rather than
/// saving it -- one Mountain is one damage each way.
static ETERNAL_FLAME_RECOIL: HalvedValueDef = HalvedValueDef::new(
    ValueDef::CountMatchingObjects(&ETERNAL_FLAME_MOUNTAINS),
    RoundingDef::Up,
);

static ETERNAL_FLAME_EFFECTS: [EffectDef; 2] = [
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::CountMatchingObjects(&ETERNAL_FLAME_MOUNTAINS),
    },
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Halved(&ETERNAL_FLAME_RECOIL),
    },
];

pub(in crate::card::sets) static ETERNAL_FLAME: CardRecord = CardRecord::new_with_legacy_id(
    1921,
    "Eternal Flame",
    CardArt::new("d646feea-3c20-4737-8d20-ffad42258ced", "Mark Poole"),
    CardSet::TheDark,
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Eternal Flame deals X damage to target opponent or planeswalker and half X damage, \
             rounded up, to you, where X is the number of Mountains you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
        )],
        EffectDef::Sequence(&ETERNAL_FLAME_EFFECTS),
    )),
);

// DRK 62 — Fire Drake
pub(in crate::card::sets) static FIRE_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1462,
    "Fire Drake",
    CardArt::new("d3419db6-1c38-4aa4-b953-1dde7d22b927", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Drake"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ]),
);

// DRK 63 — Fissure
pub(in crate::card::sets) static FISSURE: CardRecord = CardRecord::new_with_legacy_id(
    560,
    "Fissure",
    CardArt::new("aa2d778d-d74b-45ec-a86b-5d52ffad6ba5", "Douglas Shuler"),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{3}{R}{R}")).with_abilities(&[AbilityDef::destroy_target(
        "Destroy target creature or land. It can't be regenerated.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Land),
        ])),
        false,
    )]),
);

// DRK 64 — Goblin Caves
static GOBLIN_CAVES_ANTHEM: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::Subtype("Goblin"),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(2)),
};

pub(in crate::card::sets) static GOBLIN_CAVES: CardRecord = CardRecord::new_with_legacy_id(
    1916,
    "Goblin Caves",
    CardArt::new("c6a415b0-00a2-4a65-8994-4a395c50ae2d", "Drew Tucker"),
    CardSet::TheDark,
    // Every Goblin, not only yours, which is what makes this a card for a
    // mirror rather than an anthem.
    CardRules::new_enchantment(mana_cost!("{1}{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::static_ability(
                "As long as enchanted land is a basic Mountain, Goblin creatures get +0/+2.",
                EffectDef::IfCondition {
                    condition: &ENCHANTED_LAND_IS_A_BASIC_MOUNTAIN,
                    then: &GOBLIN_CAVES_ANTHEM,
                },
            ),
        ]),
);

// DRK 65 — Goblin Digging Team
pub(in crate::card::sets) static GOBLIN_DIGGING_TEAM: CardRecord = CardRecord::new_with_legacy_id(
    25,
    "Goblin Digging Team",
    CardArt::new("8a538b9d-351e-40bb-be11-9ba08c16352b", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Destroy target Wall.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Wall"),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// DRK 66 — Goblin Hero
pub(in crate::card::sets) static GOBLIN_HERO: CardRecord = CardRecord::new_with_legacy_id(
    561,
    "Goblin Hero",
    CardArt::new("7135a569-e5d3-4a1f-924b-bdb86926b4e1", "Mark Tedin"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 2, 2),
);

// DRK 67 — Goblin Rock Sled
static ROCK_SLED_THAT_ATTACKED: EffectRecipientDef = EffectRecipientDef::matching_objects(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Source,
        ObjectPredicateDef::AttackedDuringControllersLastTurn,
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

static DEFENDER_CONTROLS_A_MOUNTAIN: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

pub(in crate::card::sets) static GOBLIN_ROCK_SLED: CardRecord = CardRecord::new_with_legacy_id(
    1723,
    "Goblin Rock Sled",
    CardArt::new("0f0b49dc-da11-4397-8b12-e85b75fc8e63", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 3, 1).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step if it attacked during your last \
             turn.",
            EffectDef::StaticApply {
                recipient: ROCK_SLED_THAT_ATTACKED,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls a Mountain.",
            EffectDef::CannotAttackUnless(&DEFENDER_CONTROLS_A_MOUNTAIN),
        ),
    ]),
);

// DRK 68 — Goblin Shrine
static GOBLIN_SHRINE_ANTHEM: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::Subtype("Goblin"),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
};

pub(in crate::card::sets) static GOBLIN_SHRINE: CardRecord = CardRecord::new_with_legacy_id(
    1917,
    "Goblin Shrine",
    CardArt::new("cd69a6dc-27f3-42aa-9e63-4417796e4ef5", "Ron Spencer"),
    CardSet::TheDark,
    // The parting shot hits every Goblin including the ones it was pumping,
    // so a board of 1/1s dies with it.
    CardRules::new_enchantment(mana_cost!("{1}{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::static_ability(
                "As long as enchanted land is a basic Mountain, Goblin creatures get +1/+0.",
                EffectDef::IfCondition {
                    condition: &ENCHANTED_LAND_IS_A_BASIC_MOUNTAIN,
                    then: &GOBLIN_SHRINE_ANTHEM,
                },
            ),
            AbilityDef::triggered(
                "When this Aura leaves the battlefield, it deals 1 damage to each Goblin creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype("Goblin"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// DRK 69 — Goblin Wizard
/// A minimum of zero is the "you may": the choice is offered and may be
/// answered with nothing.
static GOBLIN_WIZARD_CHOICE: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &FROM_YOUR_HAND,
    object: ObjectPredicateDef::Subtype("Goblin"),
    minimum: 0,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    arrival_effect: None,
};

pub(in crate::card::sets) static GOBLIN_WIZARD: CardRecord = CardRecord::new_with_legacy_id(
    2005,
    "Goblin Wizard",
    CardArt::new("9b73dfb4-d930-4a89-b621-129dd9f6328c", "Daniel Gelon"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goblin", "Wizard"], 1, 1).with_abilities(
        &[
            AbilityDef::activated(
                "{T}: You may put a Goblin permanent card from your hand onto the battlefield.",
                &[AbilityCostDef::TapSource],
                GOBLIN_WIZARD_CHOICE,
            ),
            AbilityDef::activated_with_targets(
                "{R}: Target Goblin gains protection from white until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype("Goblin"),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                        ManaColor::White,
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    ),
);

// DRK 70 — Goblins of the Flarg
/// Any Dwarf you control at all, which is why this is a count of at least one
/// rather than an exact number.
static GOBLINS_OF_THE_FLARG_DWARF_CONDITION: TriggerConditionDef =
    TriggerConditionDef::ObjectCount {
        query: ObjectQueryDef::matching(
            ObjectPredicateDef::Subtype("Dwarf"),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    };

pub(in crate::card::sets) static GOBLINS_OF_THE_FLARG: CardRecord = CardRecord::new_with_legacy_id(
    28,
    "Goblins of the Flarg",
    CardArt::new("fd333b18-b896-4ab8-9c46-eed4efdd94f2", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        abilities::mountainwalk(),
        AbilityDef::triggered_if(
            "When you control a Dwarf, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &GOBLINS_OF_THE_FLARG_DWARF_CONDITION,
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// DRK 71 — Inferno
pub(in crate::card::sets) static INFERNO: CardRecord = CardRecord::new_with_legacy_id(
    562,
    "Inferno",
    CardArt::new(
        "a6b61512-5b24-424c-966f-36b595781e14",
        "Randy Asplund-Faith",
    ),
    CardSet::TheDark,
    CardRules::new_instant(mana_cost!("{5}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Inferno deals 6 damage to each creature and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(6),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(6),
            },
        ]),
    )]),
);

// DRK 72 — Mana Clash
// Audit: metadata-only — Needs a deterministic recorded coin-flip choice and both result branches for “You and target opponent each flip a coin. Mana Clash deals 1 damage to each player whose coin comes up tails. Repeat this process until both players' coins come up heads on the same flip”.
pub(in crate::card::sets) static MANA_CLASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72955141-d990-459f-adbe-7d3d0f5f6c95"),
    "Mana Clash",
    crate::card::CardArt::new("72955141-d990-459f-adbe-7d3d0f5f6c95", "Mark Tedin"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 73 — Orc General
pub(in crate::card::sets) static ORC_GENERAL: CardRecord = CardRecord::new_with_legacy_id(
    590,
    "Orc General",
    CardArt::new("65a10fd5-506e-46bf-87e6-fde134c0dc04", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Warrior"], 2, 2)
        .with_abilities(&[AbilityDef::activated(
        "{T}, Sacrifice another Orc or Goblin: Other Orc creatures get +1/+1 until end of turn.",
        &[
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Orc"),
                        ObjectPredicateDef::Subtype("Goblin"),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Orc"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DRK 74 — Sisters of the Flame
pub(in crate::card::sets) static SISTERS_OF_THE_FLAME: CardRecord = CardRecord::new_with_legacy_id(
    563,
    "Sisters of the Flame",
    CardArt::new("564e0ccd-decb-48d2-981f-cefa8045340f", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DRK 75 — Carnivorous Plant
pub(in crate::card::sets) static CARNIVOROUS_PLANT: CardRecord = CardRecord::new_with_legacy_id(
    564,
    "Carnivorous Plant",
    CardArt::new("6a615650-4da3-4efc-aa5e-c1f2c4f79478", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Plant", "Wall"], 4, 5)
        .with_abilities(&[abilities::defender()]),
);

// DRK 76 — Elves of Deep Shadow
pub(in crate::card::sets) static ELVES_OF_DEEP_SHADOW: CardRecord = CardRecord::new_with_legacy_id(
    1477,
    "Elves of Deep Shadow",
    CardArt::new("f395278e-6d74-4f35-af9d-21bad7b19763", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {B}. This creature deals 1 damage to you.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Black).with_damage_to_controller(1),
            ),
        ),
    ),
);

// DRK 77 — Gaea's Touch
/// A basic Forest specifically, so a nonbasic that happens to make green
/// mana is not on offer.
static GAEAS_TOUCH_CHOICE: EffectDef = EffectDef::ChooseCards {
    player: EffectRecipientDef::Controller,
    sources: &FROM_YOUR_HAND,
    object: ObjectPredicateDef::All(&[
        ObjectPredicateDef::Supertype(CardSupertype::Basic),
        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    ]),
    minimum: 0,
    maximum: 1,
    reveal: false,
    destination: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    arrival_effect: None,
};

pub(in crate::card::sets) static GAEAS_TOUCH: CardRecord = CardRecord::new_with_legacy_id(
    2006,
    "Gaea's Touch",
    CardArt::new("0e1ae3d6-6d96-4db6-bbc4-cee91bae6cf7", "Mark Poole"),
    CardSet::TheDark,
    // A free extra land each turn, or two green mana when you are done with
    // it.
    CardRules::new_enchantment(mana_cost!("{G}{G}")).with_abilities(&[
        AbilityDef::activated(
            "{0}: You may put a basic Forest card from your hand onto the battlefield. Activate only as a sorcery and only once each turn.",
            &[],
            GAEAS_TOUCH_CHOICE,
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .once_each_turn(),
        AbilityDef::activated_mana(
            "Sacrifice this enchantment: Add {G}{G}.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green).with_amount(2)),
        ),
    ]),
);

// DRK 77† — Gaea's Touch (alternate printing)

// DRK 78 — Hidden Path
pub(in crate::card::sets) static HIDDEN_PATH: CardRecord = CardRecord::new_with_legacy_id(
    565,
    "Hidden Path",
    CardArt::new("cbc93c0b-0ac8-4b8f-b2f6-96887d1acd77", "Rob Alexander"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}{G}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Green creatures have forestwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::forestwalk()),
            },
        ),
    ]),
);

// DRK 79 — Land Leeches
pub(in crate::card::sets) static LAND_LEECHES: CardRecord = CardRecord::new_with_legacy_id(
    566,
    "Land Leeches",
    CardArt::new("ff99543d-86a1-44f8-88ec-aaec071d6c05", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Leech"], 2, 2)
        .with_abilities(&[abilities::first_strike()]),
);

// DRK 80 — Lurker
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “This creature can't be the target of spells unless it attacked or blocked this turn”.
pub(in crate::card::sets) static LURKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b39eb671-e17e-4c5a-8913-1e3be7faedfb"),
    "Lurker",
    crate::card::CardArt::new("b39eb671-e17e-4c5a-8913-1e3be7faedfb", "Anson Maddocks"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 81 — Marsh Viper
pub(in crate::card::sets) static MARSH_VIPER: CardRecord = CardRecord::new_with_legacy_id(
    1558,
    "Marsh Viper",
    CardArt::new("109cce7a-96f7-4e67-878a-bd5c93ea8643", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Snake"], 1, 2).with_ability(
        abilities::poisonous_damage(
            2,
            "Whenever this creature deals damage to a player, that player gets two poison \
             counters. (A player with ten or more poison counters loses the game.)",
        ),
    ),
);

// DRK 82 — Niall Silvain
pub(in crate::card::sets) static NIALL_SILVAIN: CardRecord = CardRecord::new_with_legacy_id(
    1423,
    "Niall Silvain",
    CardArt::new("9d5911b5-a54e-4ebb-9c36-d4dc8e97bb4b", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{G}{G}{G}"), &["Ouphe"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}{G}{G}, {T}: Regenerate target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// DRK 83 — People of the Woods
static FORESTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static PEOPLE_OF_THE_WOODS: CardRecord = CardRecord::new_with_legacy_id(
    1470,
    "People of the Woods",
    CardArt::new("2fb5926f-9988-4bc0-b2b7-e286db208310", "Drew Tucker"),
    CardSet::TheDark,
    // Only the toughness is counted, so the printed power stays on the body.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Human"], 1, 0).with_ability(
        AbilityDef::static_ability(
            "People of the Woods's toughness is equal to the number of Forests you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_toughness(ValueDef::CountMatchingObjects(
                    &FORESTS_YOU_CONTROL,
                )),
            },
        ),
    ),
);

// DRK 84 — Savaen Elves
static SAVAEN_ELVES_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::Subtype("Aura"),
        ObjectPredicateDef::AttachedTo(&ObjectPredicateDef::HasType(CardType::Land)),
    ]),
)];

pub(in crate::card::sets) static SAVAEN_ELVES: CardRecord = CardRecord::new_with_legacy_id(
    1679,
    "Savaen Elves",
    CardArt::new("38fb3014-f631-4a75-92cd-7e626b13a4c3", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{G}{G}, {T}: Destroy target Aura attached to a land.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &SAVAEN_ELVES_TARGET,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// DRK 85 — Scarwood Bandits
// Audit: metadata-only — Needs duration-aware control-changing continuous effects for “{2}{G}, {T}: Unless an opponent pays {2}, gain control of target artifact for as long as this creature remains on the battlefield”.
pub(in crate::card::sets) static SCARWOOD_BANDITS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46b762a7-a774-4cb4-8ecf-dd6486a066c3"),
    "Scarwood Bandits",
    crate::card::CardArt::new("46b762a7-a774-4cb4-8ecf-dd6486a066c3", "Mark Poole"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 86 — Scarwood Hag
pub(in crate::card::sets) static SCARWOOD_HAG: CardRecord = CardRecord::new_with_legacy_id(
    578,
    "Scarwood Hag",
    CardArt::new("ac2655e4-3a4d-4f73-820a-02fab675d42e", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Hag"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}{G}{G}, {T}: Target creature gains forestwalk until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::forestwalk()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Target creature loses forestwalk until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                    KeywordAbility::Landwalk(BasicLandType::Forest),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DRK 87 — Scavenger Folk
pub(in crate::card::sets) static SCAVENGER_FOLK: CardRecord = CardRecord::new_with_legacy_id(
    567,
    "Scavenger Folk",
    CardArt::new("8e99870c-b2b9-431b-b8a8-3f4a80aa8fa5", "Dennis Detwiller"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{G}"), &["Human"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}, {T}, Sacrifice this creature: Destroy target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// DRK 88 — Spitting Slug
static SPITTING_SLUG_FIRST_STRIKE: AbilityDef = abilities::first_strike();

static SPITTING_SLUG_KEEPS_IT: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::add_ability(&SPITTING_SLUG_FIRST_STRIKE),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

/// The other side of the same block, whichever way round it happened.
static SPITTING_SLUG_OPPONENTS: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::BlockedBySource,
            ObjectPredicateDef::BlockingSource,
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::Any,
    ),
    effect: AppliedEffectDef::add_ability(&SPITTING_SLUG_FIRST_STRIKE),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static SPITTING_SLUG: CardRecord = CardRecord::new_with_legacy_id(
    1900,
    "Spitting Slug",
    CardArt::new("7011356e-7516-4ca0-ac54-d30af7ce03a2", "Anson Maddocks"),
    CardSet::TheDark,
    // Declining is not nothing: the first strike goes to the other side of
    // the block instead, which is what makes the {1}{G} worth paying.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Slug"], 2, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked, you may pay {1}{G}. If you do, \
             this creature gains first strike until end of turn. Otherwise, each creature \
             blocking or blocked by this creature gains first strike until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::PayOr(PayOrDef::optional_or(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{1}{G}"),
                ),
                &SPITTING_SLUG_KEEPS_IT,
                &SPITTING_SLUG_OPPONENTS,
            )),
        ),
    ),
);

// DRK 89 — Tracker
pub(in crate::card::sets) static TRACKER: CardRecord = CardRecord::new_with_legacy_id(
    568,
    "Tracker",
    CardArt::new("35ffc69e-26f2-434f-8c89-2df108dd984a", "Jeff A. Menges"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}{G}, {T}: This creature deals damage equal to its power to target creature. That creature deals damage equal to its power to this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::SourcePower,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Source,
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// DRK 90 — Venom
/// Handed to the host rather than kept on the Aura, so "this creature" in the
/// trigger is the enchanted creature and the pair it is part of is the one
/// being read.
static VENOMOUS_TOUCH: AbilityDef = AbilityDef::triggered(
    "Whenever this creature blocks or becomes blocked by a non-Wall creature, destroy the other \
     creature at end of combat.",
    TriggerEventDef::BlocksOrBecomesBlockedBy {
        creature: ObjectPredicateDef::Source,
        other: ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
    },
    abilities::destroy_triggering_object_at_end_of_combat(),
);

pub(in crate::card::sets) static VENOM: CardRecord = CardRecord::new_with_legacy_id(
    1734,
    "Venom",
    CardArt::new("7b89b81b-7b32-42a0-acf0-67784015b59a", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Whenever enchanted creature blocks or becomes blocked by a non-Wall creature, \
                 destroy the other creature at end of combat.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&VENOMOUS_TOUCH),
                },
            ),
        ]),
);

// DRK 91 — Whippoorwill
// Audit: metadata-only — Needs a duration-scoped prohibition on creating or applying regeneration shields for “{G}{G}, {T}: Target creature can't be regenerated this turn. Damage that would be dealt to that creature this turn can't be prevented or dealt instead to another permanent or player.…”.
pub(in crate::card::sets) static WHIPPOORWILL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e56146bf-5db0-4bef-83bb-efa5ebec6684"),
    "Whippoorwill",
    crate::card::CardArt::new("e56146bf-5db0-4bef-83bb-efa5ebec6684", "Douglas Shuler"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 92 — Wormwood Treefolk
/// The two clauses differ only in the land type they name, so each is the
/// same pair: grant the walk for the turn, then take the two damage that
/// paying for it costs beyond the mana.
static WORMWOOD_FORESTWALK: [EffectDef; 2] = wormwood_clause(BasicLandType::Forest);

static WORMWOOD_SWAMPWALK: [EffectDef; 2] = wormwood_clause(BasicLandType::Swamp);

const fn wormwood_clause(land_type: BasicLandType) -> [EffectDef; 2] {
    [
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(match land_type {
                BasicLandType::Forest => &FORESTWALK,
                _ => &SWAMPWALK,
            }),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ]
}

static FORESTWALK: AbilityDef = abilities::landwalk(BasicLandType::Forest);

static SWAMPWALK: AbilityDef = abilities::landwalk(BasicLandType::Swamp);

pub(in crate::card::sets) static WORMWOOD_TREEFOLK: CardRecord = CardRecord::new_with_legacy_id(
    1437,
    "Wormwood Treefolk",
    CardArt::new("2fa20173-e88a-4b14-9c54-14567ca5571c", "Jesper Myrfors"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Treefolk"], 4, 4).with_abilities(&[
        AbilityDef::activated(
            "{G}{G}: This creature gains forestwalk until end of turn and deals 2 damage to you.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{G}"))],
            EffectDef::Sequence(&WORMWOOD_FORESTWALK),
        ),
        AbilityDef::activated(
            "{B}{B}: This creature gains swampwalk until end of turn and deals 2 damage to you.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{B}"))],
            EffectDef::Sequence(&WORMWOOD_SWAMPWALK),
        ),
    ]),
);

// DRK 93 — Marsh Goblins
pub(in crate::card::sets) static MARSH_GOBLINS: CardRecord = CardRecord::new_with_legacy_id(
    1385,
    "Marsh Goblins",
    CardArt::new("8aabd80f-a18a-4bc1-9f05-4c3a63de77ce", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin"], 1, 1)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// DRK 94 — Scarwood Goblins
pub(in crate::card::sets) static SCARWOOD_GOBLINS: CardRecord = CardRecord::new_with_legacy_id(
    569,
    "Scarwood Goblins",
    CardArt::new("5542d236-af43-43b8-b30f-8980d74bbdd0", "Ron Spencer"),
    CardSet::TheDark,
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Goblin"], 2, 2),
);

// DRK 95 — Dark Heart of the Wood
pub(in crate::card::sets) static DARK_HEART_OF_THE_WOOD: CardRecord =
    CardRecord::new_with_legacy_id(
        570,
        "Dark Heart of the Wood",
        CardArt::new("e3d3df64-1e90-4aef-86ae-0062aa23ff30", "Christopher Rush"),
        CardSet::TheDark,
        CardRules::new_enchantment(mana_cost!("{B}{G}")).with_abilities(&[AbilityDef::activated(
            "Sacrifice a Forest: You gain 3 life.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype("Forest"),
                controller: PlayerRelation::You,
            }],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        )]),
    );

// DRK 96 — Barl's Cage
pub(in crate::card::sets) static BARLS_CAGE: CardRecord = CardRecord::new_with_legacy_id(
    1568,
    "Barl's Cage",
    CardArt::new("6768a307-da2e-435e-8efd-72d82b4d4a2b", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{3}: Target creature doesn't untap during its controller's next untap step.",
        &[AbilityCostDef::Mana(mana_cost!("{3}"))],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::SkipNextUntapSteps {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            count: 1,
        },
    )),
);

// DRK 97 — Bone Flute
pub(in crate::card::sets) static BONE_FLUTE: CardRecord = CardRecord::new_with_legacy_id(
    571,
    "Bone Flute",
    CardArt::new("63a31de0-d764-4ff6-a85f-027e1e58d86c", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[AbilityDef::activated(
        "{2}, {T}: All creatures get -1/-0 until end of turn.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
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
    )]),
);

// DRK 98 — Book of Rass
pub(in crate::card::sets) static BOOK_OF_RASS: CardRecord = CardRecord::new_with_legacy_id(
    572,
    "Book of Rass",
    CardArt::new("5a391ada-e9e3-45db-ae84-17421ac6b44d", "Sandra Everingham"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{6}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[AbilityDef::activated(
            "{2}, Pay 2 life: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::PayLife(2),
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// DRK 99 — Coal Golem
pub(in crate::card::sets) static COAL_GOLEM: CardRecord = CardRecord::new_with_legacy_id(
    1636,
    "Coal Golem",
    CardArt::new("1ad7692d-5a51-493f-a322-7b615446ea8e", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Golem"], 3, 3).with_ability(
        AbilityDef::activated_mana(
            "{3}, Sacrifice this creature: Add {R}{R}{R}.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(3)),
        ),
    ),
);

// DRK 100 — Dark Sphere
static DARK_SPHERE_SHIELD: EffectDef = EffectDef::PreventDamage {
    prevention: DamagePreventionDef::events(
        DamageEventMatcherDef {
            recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::Controller),
            ..DamageEventMatcherDef::from(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY))
        },
        1,
    )
    .with_coverage(DamageCoverageDef::HalfRoundedDown),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static DARK_SPHERE: CardRecord = CardRecord::new_with_legacy_id(
    1454,
    "Dark Sphere",
    CardArt::new("72cfe9b9-677d-4ecb-83ab-67fb6481371d", "Mark Tedin"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this artifact: The next time a source of your choice would deal damage to \
         you this turn, prevent half that damage, rounded down.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        abilities::shield_against_a_chosen_source(ObjectPredicateDef::Any, &DARK_SPHERE_SHIELD),
    )),
);

// DRK 101 — Diabolic Machine
pub(in crate::card::sets) static DIABOLIC_MACHINE: CardRecord = CardRecord::new_with_legacy_id(
    1378,
    "Diabolic Machine",
    CardArt::new("c3b0f228-6b06-4426-a557-1225d547b908", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Construct"], 4, 4).with_abilities(&[
        abilities::regenerate_self(
            "{3}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{3}"))],
        ),
    ]),
);

// DRK 102 — Fellwar Stone
// Audit: custom — Needs declarative mana production derived from the colors an opponent's lands could produce.
pub(in crate::card::sets) static FELLWAR_STONE: CardRecord = CardRecord::new_with_legacy_id(
    48,
    "Fellwar Stone",
    CardArt::new("dc47e322-f8b8-4685-b035-fda0cc433e6b", "Quinton Hoover"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::activated_mana(
        "{T}: Add one mana of any color that a land an opponent controls could produce.",
        &[AbilityCostDef::TapSource],
        EffectDef::Special("Add one mana of a color an opponent's land could produce"),
    )
    .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::FellwarStone))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The available colors are computed dynamically from an opponent's lands.",
    ))
    .with_legacy_procedure()]),
);

// DRK 103 — Fountain of Youth
pub(in crate::card::sets) static FOUNTAIN_OF_YOUTH: CardRecord = CardRecord::new_with_legacy_id(
    573,
    "Fountain of Youth",
    CardArt::new("2b60eb23-cb9a-4203-86fb-60e47dbd870b", "Daniel Gelon"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{0}")).with_abilities(&[AbilityDef::activated(
        "{2}, {T}: You gain 1 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )]),
);

// DRK 103† — Fountain of Youth (alternate printing)

// DRK 104 — Living Armor
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “{T}, Sacrifice this artifact: Put X +0/+1 counters on target creature, where X is that creature's mana value”.
pub(in crate::card::sets) static LIVING_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c31a957-ad1e-40cc-b3c4-2f4caa492b77"),
    "Living Armor",
    crate::card::CardArt::new("3c31a957-ad1e-40cc-b3c4-2f4caa492b77", "Anson Maddocks"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 105 — Necropolis
// Audit: metadata-only — Needs card-specific counter state and counter-consuming effects for “Exile a creature card from your graveyard: Put X +0/+1 counters on this creature, where X is the exiled card's mana value”.
pub(in crate::card::sets) static NECROPOLIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("893e8e9c-983e-4db1-8d93-10637025a559"),
    "Necropolis",
    crate::card::CardArt::new("893e8e9c-983e-4db1-8d93-10637025a559", "NéNé Thomas"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 106 — Reflecting Mirror
// Audit: metadata-only — Needs a stack-spell target-change effect plus an activation cost derived from twice that spell's mana value.
pub(in crate::card::sets) static REFLECTING_MIRROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d551ff93-d8da-4c21-bc3c-6451c0dde07e"),
    "Reflecting Mirror",
    crate::card::CardArt::new("d551ff93-d8da-4c21-bc3c-6451c0dde07e", "Mark Poole"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 107 — Runesword
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “{3}, {T}: Target attacking creature gets +2/+0 until end of turn. When that creature leaves the battlefield this turn, sacrifice this artifact. If the creature deals damage to a creature…”.
pub(in crate::card::sets) static RUNESWORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("741dbcf2-3372-45a8-b66f-d2ae12b4aac6"),
    "Runesword",
    crate::card::CardArt::new("741dbcf2-3372-45a8-b66f-d2ae12b4aac6", "Christopher Rush"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 107† — Runesword (alternate printing)

// DRK 108 — Scarecrow
pub(in crate::card::sets) static SCARECROW: CardRecord = CardRecord::new_with_legacy_id(
    1673,
    "Scarecrow",
    CardArt::new("93850e74-744c-4261-a84e-01eaced6e49a", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Scarecrow"], 2, 2).with_ability(
        AbilityDef::activated(
            "{6}, {T}: Prevent all damage that would be dealt to you this turn by creatures \
             with flying.",
            &[
                AbilityCostDef::Mana(mana_cost!("{6}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from_group_to(
                    DamageSourceGroupDef::CreaturesWithFlying,
                    EffectRecipientDef::Controller,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// DRK 109 — Skull of Orm
pub(in crate::card::sets) static SKULL_OF_ORM: CardRecord = CardRecord::new_with_legacy_id(
    574,
    "Skull of Orm",
    CardArt::new("aa1d9bb5-972a-4705-bf22-0fa1e974dd26", "Tom Wänerstrand"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{5}, {T}: Return target enchantment card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{5}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
    ]),
);

// DRK 110 — Standing Stones
pub(in crate::card::sets) static STANDING_STONES: CardRecord = CardRecord::new_with_legacy_id(
    1683,
    "Standing Stones",
    CardArt::new("6d4c853e-2231-4af2-bcb0-1781c18ec3be", "Sandra Everingham"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_mana(
        "{1}, {T}, Pay 1 life: Add one mana of any color.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::PayLife(1),
        ],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// DRK 111 — Stone Calendar
// Audit: metadata-only — Needs a battlefield-wide static spell-cost reduction; the available generic cost reducer applies only to the source card in hand.
pub(in crate::card::sets) static STONE_CALENDAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a49ba1a5-33b1-40f2-9780-26139ed829d7"),
    "Stone Calendar",
    crate::card::CardArt::new("a49ba1a5-33b1-40f2-9780-26139ed829d7", "Amy Weber"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 112 — Tormod's Crypt
pub(in crate::card::sets) static TORMODS_CRYPT: CardRecord = CardRecord::new_with_legacy_id(
    304,
    "Tormod's Crypt",
    CardArt::new("79be5dc2-fab0-4ca1-a044-83e599ed1b41", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Exile target player's graveyard.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::cards_owned_by_target(
                ObjectPredicateDef::Any,
                &[ZoneKind::Graveyard],
                TargetIndex::PRIMARY,
            ),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
            tapped: false,
        },
    )),
);

// DRK 113 — Tower of Coireall
pub(in crate::card::sets) static TOWER_OF_COIREALL: CardRecord = CardRecord::new_with_legacy_id(
    576,
    "Tower of Coireall",
    CardArt::new("64c19977-ac7d-4ce7-925c-33a7503420f5", "Dan Frazier"),
    CardSet::TheDark,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{T}: Target creature can't be blocked by Walls this turn.",
        &[AbilityCostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                ObjectPredicateDef::Subtype("Wall"),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// DRK 114 — Wand of Ith
// Audit: metadata-only — Needs seeded random selection with replay-visible provenance for “{3}, {T}: Target player reveals a card at random from their hand. If it's a land card, that player discards it unless they pay 1 life. If it isn't a land card, the player discards it…”.
pub(in crate::card::sets) static WAND_OF_ITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80c9070a-8c70-480e-a476-e00f8e2c71b9"),
    "Wand of Ith",
    crate::card::CardArt::new("80c9070a-8c70-480e-a476-e00f8e2c71b9", "Quinton Hoover"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 115 — War Barge
// Audit: metadata-only — Needs a duration-scoped prohibition on creating or applying regeneration shields for “{3}: Target creature gains islandwalk until end of turn. When this artifact leaves the battlefield this turn, destroy that creature. A creature destroyed this way can't be regenerated”.
pub(in crate::card::sets) static WAR_BARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9023c078-4169-498b-8626-a4862e0631f8"),
    "War Barge",
    crate::card::CardArt::new("9023c078-4169-498b-8626-a4862e0631f8", "Tom Wänerstrand"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 116 — City of Shadows
// Audit: metadata-only — Needs the mana-ability runtime to pay this ability's mana activation cost for “{T}: Add {C} for each storage counter on this land”.
pub(in crate::card::sets) static CITY_OF_SHADOWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76e5ee8a-34e5-4a2e-a04e-9fcdc7e53dda"),
    "City of Shadows",
    crate::card::CardArt::new("76e5ee8a-34e5-4a2e-a04e-9fcdc7e53dda", "Tom Wänerstrand"),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

// DRK 117 — Maze of Ith
/// The Maze does not remove the creature from combat: it stays an attacker,
/// keeps whatever is blocking it, and simply exchanges no combat damage.
static MAZE_OF_ITH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

static MAZE_OF_ITH_EFFECT: [EffectDef; 2] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Sequence(&[
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
];

pub(in crate::card::sets) static MAZE_OF_ITH: CardRecord = CardRecord::new_with_legacy_id(
    81,
    "Maze of Ith",
    CardArt::new("42dcceee-2a47-4eaa-a6a3-2931b3d50244", "Anson Maddocks"),
    CardSet::TheDark,
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated_with_targets(
        "{T}: Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
        &[AbilityCostDef::TapSource],
        &MAZE_OF_ITH_TARGET,
        EffectDef::Sequence(&MAZE_OF_ITH_EFFECT),
    )]),
);

// DRK 118 — Safe Haven
pub(in crate::card::sets) static SAFE_HAVEN: CardRecord = CardRecord::new_with_legacy_id(
    577,
    "Safe Haven",
    CardArt::new("0d48fb47-1bed-4791-a014-504515f3d36f", "Christopher Rush"),
    CardSet::TheDark,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Exile target creature you control.",
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
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
},
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may sacrifice this land. If you do, return each card exiled with this land to the battlefield under its owner's control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::Source,
                then: Some(&EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    arrival_effect: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                }),
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: true,
            },
        ),
    ]),
);

// DRK 119 — Sorrow's Path
// Audit: metadata-only — Needs a combat declaration or damage-assignment constraint for “{T}: Choose two target blocking creatures controlled by the same opponent. If each of those creatures could block all creatures that the other is blocking, remove both of them from…”.
pub(in crate::card::sets) static SORROW_S_PATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f75946b-1690-43cc-993c-d4e451a1a41c"),
    "Sorrow's Path",
    crate::card::CardArt::new(
        "6f75946b-1690-43cc-993c-d4e451a1a41c",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::TheDark,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGRY_MOB,
    &BLOOD_OF_THE_MARTYR,
    &BRAINWASH,
    &CLEANSING,
    &DUST_TO_DUST,
    &EXORCIST,
    &FASTING,
    &FESTIVAL,
    &FIRE_AND_BRIMSTONE,
    &HOLY_LIGHT,
    &KNIGHTS_OF_THORN,
    &MARTYR_S_CRY,
    &MIRACLE_WORKER,
    &MORALE,
    &PIKEMEN,
    &PREACHER,
    &SQUIRE,
    &TIVADARS_CRUSADE,
    &WITCH_HUNTER,
    &AMNESIA,
    &APPRENTICE_WIZARD,
    &DANCE_OF_MANY,
    &DEEP_WATER,
    &DROWNED,
    &ELECTRIC_EEL,
    &EROSION,
    &FLOOD,
    &GHOST_SHIP,
    &GIANT_SHARK,
    &LEVIATHAN,
    &MANA_VORTEX,
    &MERFOLK_ASSASSIN,
    &MIND_BOMB,
    &PSYCHIC_ALLERGY,
    &RIPTIDE,
    &SUNKEN_CITY,
    &TANGLE_KELP,
    &WATER_WURM,
    &ASHES_TO_ASHES,
    &BANSHEE,
    &BOG_IMP,
    &BOG_RATS,
    &CURSE_ARTIFACT,
    &EATER_OF_THE_DEAD,
    &FRANKENSTEIN_S_MONSTER,
    &GRAVE_ROBBERS,
    &INQUISITION,
    &MARSH_GAS,
    &MURK_DWELLERS,
    &NAMELESS_RACE,
    &RAG_MAN,
    &SEASON_OF_THE_WITCH,
    &THE_FALLEN,
    &UNCLE_ISTVAN,
    &WORD_OF_BINDING,
    &WORMS_OF_THE_EARTH,
    &BALL_LIGHTNING,
    &BLOOD_MOON,
    &BROTHERS_OF_FIRE,
    &CAVE_PEOPLE,
    &ETERNAL_FLAME,
    &FIRE_DRAKE,
    &FISSURE,
    &GOBLIN_CAVES,
    &GOBLIN_DIGGING_TEAM,
    &GOBLIN_HERO,
    &GOBLIN_ROCK_SLED,
    &GOBLIN_SHRINE,
    &GOBLIN_WIZARD,
    &GOBLINS_OF_THE_FLARG,
    &INFERNO,
    &MANA_CLASH,
    &ORC_GENERAL,
    &SISTERS_OF_THE_FLAME,
    &CARNIVOROUS_PLANT,
    &ELVES_OF_DEEP_SHADOW,
    &GAEAS_TOUCH,
    &HIDDEN_PATH,
    &LAND_LEECHES,
    &LURKER,
    &MARSH_VIPER,
    &NIALL_SILVAIN,
    &PEOPLE_OF_THE_WOODS,
    &SAVAEN_ELVES,
    &SCARWOOD_BANDITS,
    &SCARWOOD_HAG,
    &SCAVENGER_FOLK,
    &SPITTING_SLUG,
    &TRACKER,
    &VENOM,
    &WHIPPOORWILL,
    &WORMWOOD_TREEFOLK,
    &MARSH_GOBLINS,
    &SCARWOOD_GOBLINS,
    &DARK_HEART_OF_THE_WOOD,
    &BARLS_CAGE,
    &BONE_FLUTE,
    &BOOK_OF_RASS,
    &COAL_GOLEM,
    &DARK_SPHERE,
    &DIABOLIC_MACHINE,
    &FELLWAR_STONE,
    &FOUNTAIN_OF_YOUTH,
    &LIVING_ARMOR,
    &NECROPOLIS,
    &REFLECTING_MIRROR,
    &RUNESWORD,
    &SCARECROW,
    &SKULL_OF_ORM,
    &STANDING_STONES,
    &STONE_CALENDAR,
    &TORMODS_CRYPT,
    &TOWER_OF_COIREALL,
    &WAND_OF_ITH,
    &WAR_BARGE,
    &CITY_OF_SHADOWS,
    &MAZE_OF_ITH,
    &SAFE_HAVEN,
    &SORROW_S_PATH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&GAEAS_TOUCH, 1), // DRK 77†
    PrintingRecord::alternate(&FOUNTAIN_OF_YOUTH, 1), // DRK 103†
    PrintingRecord::alternate(&RUNESWORD, 1),   // DRK 107†
];
