//! Avatar: The Last Airbender card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ChooseObjectOrderDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostAdjustmentDef;
use crate::card::CostAmountDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::PowerToughnessOperationDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SetOperationDef;
use crate::card::SpellCostConditionDef;
use crate::card::SpellCostModificationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1999::portal_three_kingdoms as catalog_ptk;
use crate::card::sets::y2013::theros as catalog_ths;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2020::ikoria as catalog_iko;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TLA",
    slug: "avatar-the-last-airbender",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

/// The TLA cycle of tapped duals that cash themselves in: lands that
/// differ only in which two colours they make, so the clauses are written
/// once here. `colors` is a promoted literal at each call site, since a
/// slice assembled inside this function could not be given a `'static`
/// lifetime.
const fn cashable_dual_land(mana_text: &'static str, colors: &'static [ManaColor]) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(AbilityDef::as_enters(
            "This land enters tapped.",
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ))
        // Added one at a time and in printed order: an array holding the
        // parameterized mana ability could not be given a 'static lifetime.
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated(
            "{4}, {T}, Sacrifice this land: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{4}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ))
}

const CLUE_TOKEN: TokenCharacteristics = crate::card::tokens::clue().with_art(CardArt::new(
    "82016bc3-8905-454a-b4aa-b9eac69d42c4",
    "Luc Courtois",
));
const FOOD_TOKEN: TokenCharacteristics = crate::card::tokens::food().with_art(CardArt::new(
    "db202f04-61e7-4e12-848e-6c1761956a58",
    "Andreas Rocha",
));

const ALLY_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Ally"], &[ManaColor::White], 1, 1).with_art(CardArt::new(
        "f842c52a-04b6-4625-ba58-45b043ada340",
        "Rose Benjamin",
    ));
const MONK_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Monk"], &[ManaColor::Red], 1, 1)
        .with_abilities(&[AbilityDef::triggered(
            "Prowess",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )])
        .with_art(CardArt::new(
            "e49c80ac-5c90-4fd2-ad40-cc77af160f64",
            "Dom Lay",
        ));

// TLA 1 — Aang's Journey
pub(in crate::card::sets) static AANG_S_JOURNEY: CardRecord = CardRecord::new(
    "Aang's Journey",
    "5e51f727-5a9b-4bc7-83a9-dbcf1c933e15",
    "Kotakan",
    CardRules::new_sorcery(mana_cost!("{2}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[
            abilities::kicker(&[CostDef::Mana(mana_cost!("{2}"))]),
            AbilityDef::spell(
                "Search your library for a basic land card. If this spell was \
                 kicked, instead search your library for a basic land card and \
                 a Shrine card. Reveal those cards, put them into your hand, \
                 then shuffle.\nYou gain 2 life.",
                EffectDef::Sequence(&[
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: false,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                            crate::AdditionalCostIndex::PRIMARY,
                        ),
                        then: &EffectDef::SearchZone {
                            player: EffectRecipientDef::Controller,
                            source: ZoneKind::Library,
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            minimum: 0,
                            maximum: ValueDef::Constant(1),
                            reveal: true,
                            destination: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                            shuffle: false,
                            enters_tapped: false,
                            attachment: None,
                            binding: None,
                            then: None,
                        },
                    },
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// TLA 2 — Energybending
pub(in crate::card::sets) static ENERGYBENDING: CardRecord = CardRecord::new(
    "Energybending",
    "6c085441-e023-4032-89a0-d24ce5060ac3",
    "Hisashi Momose",
    CardRules::new_instant(mana_cost!("{2}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell(
            "Lands you control gain all basic land types until end of \
             turn.\nDraw a card.",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::BasicLandTypes(SetOperationDef::Add(&[
                            crate::card::BasicLandType::Plains,
                            crate::card::BasicLandType::Island,
                            crate::card::BasicLandType::Swamp,
                            crate::card::BasicLandType::Mountain,
                            crate::card::BasicLandType::Forest,
                        ])),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        )]),
);

// TLA 3 — Zuko's Exile
pub(in crate::card::sets) static ZUKO_S_EXILE: CardRecord = CardRecord::new(
    "Zuko's Exile",
    "9090b055-3406-4fed-a8c6-3f6353a9600e",
    "Eiji Kaneda",
    CardRules::new_instant(mana_cost!("{5}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Exile target artifact, creature, or enchantment. Its \
             controller creates a Clue token. (It's an artifact with \
             \"{2}, Sacrifice this token: Draw a card.\")",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1))
                        .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                            TargetIndex::PRIMARY,
                        ))),
                ),
            ]),
        )]),
);

// TLA 4 — Aang, the Last Airbender
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static AANG_THE_LAST_AIRBENDER: CardRecord = CardRecord::new(
    "Aang, the Last Airbender",
    "245e008c-e073-443f-9592-6f628c0026ec",
    "Yueko",
    CardRules::unsupported(),
);

// TLA 5 — Aang's Iceberg
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs. Also needs immediate exile-until-source-leaves duration semantics instead of a separate return trigger.
pub(in crate::card::sets) static AANG_S_ICEBERG: CardRecord = CardRecord::new(
    "Aang's Iceberg",
    "720fbd87-b1c1-4b3b-97a1-46b943b115e3",
    "Matteo Bassini",
    CardRules::unsupported(),
);

// TLA 6 — Airbender Ascension
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static AIRBENDER_ASCENSION: CardRecord = CardRecord::new(
    "Airbender Ascension",
    "99a90d13-891c-45cc-b1d5-6080ebae5862",
    "Shiren",
    CardRules::unsupported(),
);

// TLA 7 — Airbender's Reversal
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static AIRBENDER_S_REVERSAL: CardRecord = CardRecord::new(
    "Airbender's Reversal",
    "6b7078e4-2892-4b5f-83ab-90369e0d6dba",
    "Kotakan",
    CardRules::unsupported(),
);

// TLA 8 — Airbending Lesson
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static AIRBENDING_LESSON: CardRecord = CardRecord::new(
    "Airbending Lesson",
    "e2bcd0a6-e94d-4a21-a334-a57459c1b8cc",
    "Pisukev",
    CardRules::unsupported(),
);

// TLA 9 — Appa, Loyal Sky Bison
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static APPA_LOYAL_SKY_BISON: CardRecord = CardRecord::new(
    "Appa, Loyal Sky Bison",
    "a9b2a843-c6fe-4d19-801e-1538e4381ab0",
    "Tomoyo Asatani",
    CardRules::unsupported(),
);

// TLA 10 — Appa, Steadfast Guardian
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static APPA_STEADFAST_GUARDIAN: CardRecord = CardRecord::new(
    "Appa, Steadfast Guardian",
    "829d91e9-4878-4e55-a262-ac0d55b65d4e",
    "Maël Ollivier-Henry",
    CardRules::unsupported(),
);

// TLA 11 — Avatar Enthusiasts
pub(in crate::card::sets) static AVATAR_ENTHUSIASTS: CardRecord = CardRecord::new(
    "Avatar Enthusiasts",
    "7292e5f2-d5a1-4e39-8f75-eba471819f04",
    "Leanna Crossan",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Peasant", "Ally"], 2, 2)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever another Ally you control enters, put a +1/+1 counter \
             on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// TLA 12 — Avatar's Wrath
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static AVATAR_S_WRATH: CardRecord = CardRecord::new(
    "Avatar's Wrath",
    "4811072d-fac0-40dd-a5cf-9694d51b12cf",
    "Ainezu",
    CardRules::unsupported(),
);

// TLA 13 — Compassionate Healer
pub(in crate::card::sets) static COMPASSIONATE_HEALER: CardRecord = CardRecord::new(
    "Compassionate Healer",
    "88d5f8fd-d4de-4e64-9b74-e53719ffbcdc",
    "Kuno",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric", "Ally"], 2, 2)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever this creature becomes tapped, you gain 1 life and \
             scry 1. (Look at the top card of your library. You may put it \
             on the bottom.)",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                abilities::scry(ValueDef::Constant(1)),
            ]),
        )]),
);

// TLA 14 — Curious Farm Animals
pub(in crate::card::sets) static CURIOUS_FARM_ANIMALS: CardRecord = CardRecord::new(
    "Curious Farm Animals",
    "2402d759-84b6-41d2-ad78-9333974e9222",
    "John Di Giovanni",
    CardRules::new_creature(mana_cost!("{W}"), &["Boar", "Elk", "Bird", "Ox"], 1, 1)
        .with_abilities(&[
            AbilityDef::triggered(
                "When this creature dies, you gain 3 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::activated_with_targets(
                "{2}, Sacrifice this creature: Destroy up to one target \
                 artifact or enchantment.",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ]),
);

// TLA 15 — Destined Confrontation
// Audit: unsupported — Needs simultaneous per-player creature-group choices constrained by a summed power budget; the existing per-player partition supports fixed counts or one object per predicate, not aggregate power.
pub(in crate::card::sets) static DESTINED_CONFRONTATION: CardRecord = CardRecord::new(
    "Destined Confrontation",
    "d4605cf7-03bc-4a7f-b50a-49b83b09b54d",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// TLA 16 — Earth Kingdom Jailer
// Audit: unsupported — Needs immediate exile-until-this-source-leaves duration semantics (CR 610.3), including the no-exile case when the source has already left; separate leave triggers cannot implement the return timing.
pub(in crate::card::sets) static EARTH_KINGDOM_JAILER: CardRecord = CardRecord::new(
    "Earth Kingdom Jailer",
    "9207362c-3605-4794-803f-ad1d0175fcca",
    "Danciao",
    CardRules::unsupported(),
);

// TLA 17 — Earth Kingdom Protectors
pub(in crate::card::sets) static EARTH_KINGDOM_PROTECTORS: CardRecord = CardRecord::new(
    "Earth Kingdom Protectors",
    "d263472e-1d75-4cb9-8f7b-986fa22bc841",
    "Leonardo Borazio",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier", "Ally"], 1, 1).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::activated_with_targets(
                "Sacrifice this creature: Another target Ally you control \
                 gains indestructible until end of turn. (Damage and effects \
                 that say \"destroy\" don't destroy it.)",
                &[CostDef::SacrificeSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    ),
);

// TLA 18 — Enter the Avatar State
pub(in crate::card::sets) static ENTER_THE_AVATAR_STATE: CardRecord = CardRecord::new(
    "Enter the Avatar State",
    "a8f3d2cb-d073-4df8-8769-49612628a377",
    "Shiren",
    CardRules::new_instant(mana_cost!("{W}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Until end of turn, target creature you control becomes an \
             Avatar in addition to its other types and gains flying, first \
             strike, lifelink, and hexproof. (A creature with hexproof \
             can't be the target of spells or abilities your opponents \
             control.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        SetOperationDef::Add(&["Avatar"]),
                    )),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                    AppliedEffectDef::add_ability(&abilities::hexproof()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// TLA 19 — Fancy Footwork
pub(in crate::card::sets) static FANCY_FOOTWORK: CardRecord = CardRecord::new(
    "Fancy Footwork",
    "9c59f8c5-4063-4e5a-b684-fde175c4a981",
    "Mizutametori",
    CardRules::new_instant(mana_cost!("{2}{W}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Untap one or two target creatures. They each get +2/+2 until \
             end of turn.",
            &[AbilityTargetDef {
                minimum: 1,
                ..AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                )
            }],
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        )]),
);

// TLA 20 — Gather the White Lotus
pub(in crate::card::sets) static GATHER_THE_WHITE_LOTUS: CardRecord = CardRecord::new(
    "Gather the White Lotus",
    "96e2b3d0-f060-4f3c-9d12-a0444b202008",
    "Kozato",
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_abilities(&[AbilityDef::spell(
        "Create a 1/1 white Ally creature token for each Plains you \
         control. Scry 2. (Look at the top two cards of your library, \
         then put any number of them on the bottom and the rest on top \
         in any order.)",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN)).with_count(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            ),
            abilities::scry(ValueDef::Constant(2)),
        ]),
    )]),
);

// TLA 21 — Glider Kids
pub(in crate::card::sets) static GLIDER_KIDS: CardRecord = CardRecord::new(
    "Glider Kids",
    "0613afb5-4cee-4132-8adf-404767ed8d07",
    "ikeda_cpt",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Pilot", "Ally"], 2, 3)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger(
                "When this creature enters, scry 1. (Look at the top card of \
                 your library. You may put it on the bottom.)",
                abilities::scry(ValueDef::Constant(1)),
            ),
        ]),
);

// TLA 22 — Glider Staff
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static GLIDER_STAFF: CardRecord = CardRecord::new(
    "Glider Staff",
    "7517f2eb-a24d-49f6-82bf-08de55d3789a",
    "Eduardo Francisco",
    CardRules::unsupported(),
);

// TLA 23 — Hakoda, Selfless Commander
pub(in crate::card::sets) static HAKODA_SELFLESS_COMMANDER: CardRecord = CardRecord::new(
    "Hakoda, Selfless Commander",
    "9aef3ddb-9bb7-42c8-975b-b1917955a416",
    "Rafater",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Warrior", "Ally"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "You may look at the top card of your library any time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
                },
            ),
            AbilityDef::static_ability(
                "You may cast Ally spells from the top of your library.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                        restriction: PlayRestrictionDef {
                            action: PlayActionMatcherDef::CastSpell,
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                            only_at_sorcery_speed: false,
                            minimum_spells_cast_this_turn: 0,
                        },
                        cost: TopOfLibraryCostDef::Printed,
                    }),
                },
            ),
            AbilityDef::activated(
                "Sacrifice Hakoda: Creatures you control get +0/+5 and gain \
                 indestructible until end of turn.",
                &[CostDef::SacrificeSource],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(5),
                        ),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TLA 24 — Invasion Reinforcements
pub(in crate::card::sets) static INVASION_REINFORCEMENTS: CardRecord = CardRecord::new(
    "Invasion Reinforcements",
    "b8845bba-d116-43cd-8ae0-e553f3324d66",
    "Yuhong Ding",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Warrior", "Ally"], 1, 1)
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger(
                "When this creature enters, create a 1/1 white Ally creature \
                 token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
            ),
        ]),
);

// TLA 25 — Jeong Jeong's Deserters
pub(in crate::card::sets) static JEONG_JEONG_S_DESERTERS: CardRecord = CardRecord::new(
    "Jeong Jeong's Deserters",
    "060966f9-5e56-4512-9ef3-2b216daf093c",
    "Leonardo Borazio",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Rebel", "Ally"], 1, 2)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target \
             creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// TLA 26 — Kyoshi Warriors
pub(in crate::card::sets) static KYOSHI_WARRIORS: CardRecord = CardRecord::new(
    "Kyoshi Warriors",
    "211045e1-85c7-4088-b830-a2afa0fe520b",
    "Awanqi (Angela Wang)",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Warrior", "Ally"], 3, 3)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Ally creature \
             token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
        )]),
);

// TLA 27 — The Legend of Yangchen // Avatar Yangchen
// Audit: unsupported — Needs airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static THE_LEGEND_OF_YANGCHEN: CardRecord = CardRecord::new(
    "The Legend of Yangchen // Avatar Yangchen",
    "a60e8f23-90b2-4bc6-bd54-a95055556389",
    "Kuno",
    CardRules::unsupported(),
);

// TLA 28 — Master Piandao
pub(in crate::card::sets) static MASTER_PIANDAO: CardRecord = CardRecord::new(
    "Master Piandao",
    "3e4a042c-f2a6-45e7-9444-acd1ca838b87",
    "Brian Yuen",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Warrior", "Ally"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered(
                "Whenever Master Piandao attacks, look at the top four cards \
                 of your library. You may reveal an Ally, Equipment, or Lesson \
                 card from among them and put it into your hand. Put the rest \
                 on the bottom of your library in a random order.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(4),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                    ]),
                    minimum: 0,
                    maximum: 1,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                            then: &EffectDef::None,
                        }),
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("random_bottom"),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("random_bottom"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            ),
        ]),
);

// TLA 29 — Momo, Friendly Flier
// Audit: unsupported — Needs filtered per-turn cast history for the first non-Lemur flying creature spell on each of your turns; current spell-cost conditions do not count prior matching casts.
pub(in crate::card::sets) static MOMO_FRIENDLY_FLIER: CardRecord = CardRecord::new(
    "Momo, Friendly Flier",
    "c472ef84-a632-4ad7-853c-60588a7a4b12",
    "Brandon L. Hunt",
    CardRules::unsupported(),
);

// TLA 30 — Momo, Playful Pet
pub(in crate::card::sets) static MOMO_PLAYFUL_PET: CardRecord = CardRecord::new(
    "Momo, Playful Pet",
    "9350bf4a-fa12-4867-b31f-1f1394d99571",
    "Awanqi (Angela Wang)",
    CardRules::new_creature(mana_cost!("{W}"), &["Lemur", "Bat", "Ally"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            AbilityDef::modal_triggered(
                "When Momo leaves the battlefield, choose one —",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                &[
                    AbilityDef::spell(
                        "Create a Food token. (It's an artifact with \"{2}, {T}, \
                         Sacrifice this token: You gain 3 life.\")",
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                                .with_count(ValueDef::Constant(1)),
                        ),
                    ),
                    AbilityDef::spell_with_targets(
                        "Put a +1/+1 counter on target creature you control.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )],
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                    AbilityDef::spell("Scry 2.", abilities::scry(ValueDef::Constant(2))),
                ],
            ),
        ]),
);

// TLA 31 — Path to Redemption
pub(in crate::card::sets) static PATH_TO_REDEMPTION: CardRecord = CardRecord::new(
    "Path to Redemption",
    "f936d64a-0db4-49c1-8a57-6d99e012a555",
    "Hokyoung Kim",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
            AbilityDef::activated(
                "{5}, Sacrifice this Aura: Exile enchanted creature. Create a \
                 1/1 white Ally creature token. Activate only during your \
                 turn.",
                &[CostDef::Mana(mana_cost!("{5}")), CostDef::SacrificeSource],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::AttachedPermanent,
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
                ]),
            )
            .with_activation_timing(ActivationTimingDef::YourTurn),
        ]),
);

// TLA 32 — Rabaroo Troop
pub(in crate::card::sets) static RABAROO_TROOP: CardRecord = CardRecord::new(
    "Rabaroo Troop",
    "62a983b4-ac73-4949-9317-05a75a8ce164",
    "Mizutametori",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Rabbit", "Kangaroo"], 3, 5).with_abilities(
        &[
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, this creature \
                 gains flying until end of turn and you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            abilities::typecycling!(
                "Plainscycling {2}",
                &[CostDef::Mana(mana_cost!("{2}"))],
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains"))
            ),
        ],
    ),
);

// TLA 33 — Razor Rings
// Audit: unsupported — Needs excess-damage output from an ordinary damage instruction, including prevention and lethal deathtouch semantics; current excess-damage continuations are limited to fight.
pub(in crate::card::sets) static RAZOR_RINGS: CardRecord = CardRecord::new(
    "Razor Rings",
    "b05cfee5-ee59-4df6-a5f6-d9ef0fa7f98a",
    "Norikatsu Miyoshi",
    CardRules::unsupported(),
);

// TLA 34 — Sandbenders' Storm
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static SANDBENDERS_STORM: CardRecord = CardRecord::new(
    "Sandbenders' Storm",
    "b3bf4a25-4329-4318-870a-2b06aa620dc4",
    "Robin Har",
    CardRules::unsupported(),
);

// TLA 35 — South Pole Voyager
// Audit: unsupported — Needs per-ability resolution history retained after its source leaves the battlefield; SourceResolutionsThisTurn reads only a live permanent, so the second-resolution draw is lost for a departed Voyager.
pub(in crate::card::sets) static SOUTH_POLE_VOYAGER: CardRecord = CardRecord::new(
    "South Pole Voyager",
    "4b5ad895-be8d-476b-91ca-22fad7a3cc58",
    "Tition",
    CardRules::unsupported(),
);

// TLA 36 — Southern Air Temple
pub(in crate::card::sets) static SOUTHERN_AIR_TEMPLE: CardRecord = CardRecord::new(
    "Southern Air Temple",
    "5c172f9b-2184-4736-9f90-74ce08596292",
    "Salvatorre Zee Yazzie",
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Shrine"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Southern Air Temple enters, put X +1/+1 counters on each \
                 creature you control, where X is the number of Shrines you \
                 control.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever another Shrine you control enters, put a +1/+1 \
                 counter on each creature you control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// TLA 37 — Suki, Courageous Rescuer
pub(in crate::card::sets) static SUKI_COURAGEOUS_RESCUER: CardRecord = CardRecord::new(
    "Suki, Courageous Rescuer",
    "33b97433-e16c-422e-a0ca-f6b1e98d8681",
    "Tky",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Warrior", "Ally"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other creatures you control get +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever another permanent you control leaves the battlefield \
                 during your turn, create a 1/1 white Ally creature token. \
                 This ability triggers only once each turn.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        Some(ZoneKind::Battlefield),
                        None,
                    ),
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                },
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
            )
            .triggering_at_most(1),
        ]),
);

// TLA 38 — Team Avatar
pub(in crate::card::sets) static TEAM_AVATAR: CardRecord = CardRecord::new(
    "Team Avatar",
    "a9a5f6f7-f04d-477f-b67f-fd01a5dcc0f5",
    "Bun Toujo",
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control attacks alone, it gets +X/+X \
             until end of turn, where X is the number of creatures you \
             control.",
            TriggerEventDef::attacks_in_declaration(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                Some(1),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}{W}, Discard this card: It deals damage equal to the \
             number of creatures you control to target creature.",
            &[CostDef::Mana(mana_cost!("{2}{W}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// TLA 39 — United Front
pub(in crate::card::sets) static UNITED_FRONT: CardRecord = CardRecord::new(
    "United Front",
    "d347caf8-0c12-401f-9e33-9978cb347f89",
    "Mengxuan Li",
    CardRules::new_sorcery(mana_cost!("{X}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Create X 1/1 white Ally creature tokens, then put a +1/+1 \
         counter on each creature you control.",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN)).with_count(ValueDef::ChosenX),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// TLA 40 — Vengeful Villagers
pub(in crate::card::sets) static VENGEFUL_VILLAGERS: CardRecord = CardRecord::new(
    "Vengeful Villagers",
    "8cabb6ed-5c80-4dab-b96c-9d4d9fe72db7",
    "Ittoku",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Citizen"], 3, 3).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, choose target creature an \
             opponent controls. Tap it, then you may sacrifice an artifact \
             or creature. If you do, put a stun counter on the chosen \
             creature. (If a permanent with a stun counter would become \
             untapped, remove one from it instead.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]))],
                    &EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(1),
                    },
                )),
            ]),
        ),
    ]),
);

// TLA 41 — Water Tribe Captain
pub(in crate::card::sets) static WATER_TRIBE_CAPTAIN: CardRecord = CardRecord::new(
    "Water Tribe Captain",
    "0a0cdf97-1927-47c6-8ef1-29969e3567ee",
    "Yosuke Adachi",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier", "Ally"], 3, 3)
        .with_abilities(&[AbilityDef::activated(
            "{5}: Creatures you control get +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{5}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// TLA 42 — Water Tribe Rallier
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static WATER_TRIBE_RALLIER: CardRecord = CardRecord::new(
    "Water Tribe Rallier",
    "4e744b6c-1c2f-451a-818a-5ee7785b5213",
    "Boell Oyino",
    CardRules::unsupported(),
);

// TLA 43 — Yip Yip!
pub(in crate::card::sets) static YIP_YIP: CardRecord = CardRecord::new(
    "Yip Yip!",
    "43f4b10e-165b-4100-82f3-728e1b0c78ed",
    "Cinkai",
    CardRules::new_instant(mana_cost!("{W}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Target creature you control gets +2/+2 until end of turn. If \
             that creature is an Ally, it also gains flying until end of \
             turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        )]),
);

// TLA 44 — Accumulate Wisdom
pub(in crate::card::sets) static ACCUMULATE_WISDOM: CardRecord = CardRecord::new(
    "Accumulate Wisdom",
    "a6335319-6c92-40d4-ab2d-c06c79049c30",
    "Gemi",
    CardRules::new_instant(mana_cost!("{1}{U}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell(
            "Look at the top three cards of your library. Put one of those \
             cards into your hand and the rest on the bottom of your \
             library in any order. Put each of those cards into your hand \
             instead if there are three or more Lesson cards in your \
             graveyard.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(3),
                }),
                then: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(3),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::Any,
                    minimum: 3,
                    maximum: 3,
                    chosen: crate::Binding!("all_cards"),
                    remainder: crate::Binding!("all_rest"),
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "all_cards"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
                otherwise: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(3),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::Any,
                    minimum: 1,
                    maximum: 1,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                        EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
                            actor: PlayerRefDef::EffectController,
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            ordered: crate::Binding!("ordered"),
                            placement: ZonePlacement::Bottom,
                            visibility: ChoiceVisibilityDef::Private,
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("ordered"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            },
        )]),
);

// TLA 45 — Benevolent River Spirit
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static BENEVOLENT_RIVER_SPIRIT: CardRecord = CardRecord::new(
    "Benevolent River Spirit",
    "7ffb79cd-d170-4047-89c8-6e85188f30da",
    "Mizutametori",
    CardRules::unsupported(),
);

// TLA 46 — Boomerang Basics
pub(in crate::card::sets) static BOOMERANG_BASICS: CardRecord = CardRecord::new(
    "Boomerang Basics",
    "17ab958a-abc6-472e-ad6a-97c731d89c74",
    "Tubaki Halsame",
    CardRules::new_sorcery(mana_cost!("{U}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Return target nonland permanent to its owner's hand. If you \
             controlled that permanent, draw a card.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                ]),
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                },
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
                otherwise: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        )]),
);

// TLA 47 — Crashing Wave
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static CRASHING_WAVE: CardRecord = CardRecord::new(
    "Crashing Wave",
    "9fd02eb4-1ef5-4a14-89a8-b25e720e8016",
    "Mitori",
    CardRules::unsupported(),
);

// TLA 48 — Ember Island Production
pub(in crate::card::sets) static EMBER_ISLAND_PRODUCTION: CardRecord = CardRecord::new(
    "Ember Island Production",
    "0f79a7fc-36b1-4397-8e67-6638379d3a38",
    "Brian Yuen",
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Create a token that's a copy of target creature you control, \
                 except it's not legendary and it's a 4/4 Hero in addition to \
                 its other types.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    exceptions: CopyExceptionsDef {
                        base_power_toughness: Some((4, 4)),
                        removed_supertypes: &[CardSupertype::Legendary],
                        added_creature_types: CreatureTypeSetDef::named(&["Hero"]),
                        ..CopyExceptionsDef::NONE
                    },
                }))),
            ),
            AbilityDef::spell_with_targets(
                "Create a token that's a copy of target creature an opponent \
                 controls, except it's not legendary and it's a 2/2 Coward in \
                 addition to its other types.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    exceptions: CopyExceptionsDef {
                        base_power_toughness: Some((2, 2)),
                        removed_supertypes: &[CardSupertype::Legendary],
                        added_creature_types: CreatureTypeSetDef::named(&["Coward"]),
                        ..CopyExceptionsDef::NONE
                    },
                }))),
            ),
        ],
    )]),
);

// TLA 49 — First-Time Flyer
pub(in crate::card::sets) static FIRST_TIME_FLYER: CardRecord = CardRecord::new(
    "First-Time Flyer",
    "ea22d29d-3a62-4ae2-91d5-21a678be9b48",
    "Mizutametori",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Pilot", "Ally"], 1, 2)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "This creature gets +1/+1 as long as there's a Lesson card in \
                 your graveyard.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
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

// TLA 50 — Flexible Waterbender
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static FLEXIBLE_WATERBENDER: CardRecord = CardRecord::new(
    "Flexible Waterbender",
    "1447ebce-7e48-4b21-a39c-740920538bdd",
    "Rafater",
    CardRules::unsupported(),
);

// TLA 51 — Forecasting Fortune Teller
pub(in crate::card::sets) static FORECASTING_FORTUNE_TELLER: CardRecord = CardRecord::new(
    "Forecasting Fortune Teller",
    "023e7ad4-6af0-4f29-a716-27a313974227",
    "Hisashi Momose",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Advisor", "Ally"], 1, 3)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, create a Clue token. (It's an \
             artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        )]),
);

// TLA 52 — Geyser Leaper
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static GEYSER_LEAPER: CardRecord = CardRecord::new(
    "Geyser Leaper",
    "4d25428d-e5a5-43c4-8544-43120ed1c4d5",
    "Norikatsu Miyoshi",
    CardRules::unsupported(),
);

// TLA 53 — Giant Koi
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static GIANT_KOI: CardRecord = CardRecord::new(
    "Giant Koi",
    "3938ec1f-979c-48e5-bb13-bebace006a8f",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// TLA 54 — Gran-Gran
pub(in crate::card::sets) static GRAN_GRAN: CardRecord = CardRecord::new(
    "Gran-Gran",
    "fa434b41-e5f7-4989-865a-95db67b05cb1",
    "Arou",
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Peasant", "Ally"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever Gran-Gran becomes tapped, draw a card, then discard \
                 a card.",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Noncreature spells you cast cost {1} less to cast as long as \
                 there are three or more Lesson cards in your graveyard.",
                EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
                    spell: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                    caster: PlayerRelation::You,
                    condition: SpellCostConditionDef::Always,
                    adjustment: CostAdjustmentDef::Subtract(CostAmountDef::Generic(
                        ValueDef::IfMatchingObjectCount(&CountConditionDef {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 3,
                            then: ValueDef::Constant(1),
                            otherwise: ValueDef::Constant(0),
                        }),
                    )),
                })),
            ),
        ]),
);

// TLA 55 — Honest Work
// Audit: unsupported — Needs a layer-3 name-setting effect that changes the enchanted permanent to Humble Merchant; creature-type and ability changes do not change its name.
pub(in crate::card::sets) static HONEST_WORK: CardRecord = CardRecord::new(
    "Honest Work",
    "57e0ccb0-22f0-4e4d-9a47-c4fec2c7f251",
    "ikeda_cpt",
    CardRules::unsupported(),
);

// TLA 56 — Iguana Parrot
pub(in crate::card::sets) static IGUANA_PARROT: CardRecord = CardRecord::new(
    "Iguana Parrot",
    "7faa3e3d-c08a-4908-85ef-1370d3f90815",
    "Tyler Smith",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Lizard", "Bird", "Pirate"], 2, 2)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            AbilityDef::triggered(
                "Prowess",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
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

// TLA 57 — Invasion Submersible
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static INVASION_SUBMERSIBLE: CardRecord = CardRecord::new(
    "Invasion Submersible",
    "af5299f5-1633-4c07-ade5-fd47e29ea4aa",
    "Sylvain Sarrailh",
    CardRules::unsupported(),
);

// TLA 58 — It'll Quench Ya!
pub(in crate::card::sets) static IT_LL_QUENCH_YA: CardRecord = CardRecord::new(
    "It'll Quench Ya!",
    "47c25e41-f43c-4447-81b5-b9631448bd29",
    "Nathaniel Himawan",
    CardRules::new_instant(mana_cost!("{1}{U}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {2}.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{2}"))]),
        )]),
);

// TLA 59 — Katara, Bending Prodigy
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static KATARA_BENDING_PRODIGY: CardRecord = CardRecord::new(
    "Katara, Bending Prodigy",
    "e8372167-383f-4302-a8ea-b6bf495c870c",
    "Mephisto",
    CardRules::unsupported(),
);

// TLA 60 — Knowledge Seeker
pub(in crate::card::sets) static KNOWLEDGE_SEEKER: CardRecord = CardRecord::new(
    "Knowledge Seeker",
    "8554e862-f78a-42f5-b876-076aac6c9504",
    "Shiren",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Fox", "Spirit"], 2, 1).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "When this creature dies, create a Clue token. (It's an \
             artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// TLA 61 — The Legend of Kuruk // Avatar Kuruk
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static THE_LEGEND_OF_KURUK: CardRecord = CardRecord::new(
    "The Legend of Kuruk // Avatar Kuruk",
    "5e9a53d3-7f2f-4a9c-9516-0713da740478",
    "Takayama Toshiaki",
    CardRules::unsupported(),
);

// TLA 62 — Lost Days
// Audit: unsupported — Needs a library placement second from the top, selected by the targeted permanent's owner; current placements expose only top and bottom.
pub(in crate::card::sets) static LOST_DAYS: CardRecord = CardRecord::new(
    "Lost Days",
    "db8e88e3-0931-403f-90e7-3c22c0b61dac",
    "Matteo Bassini",
    CardRules::unsupported(),
);

// TLA 63 — Master Pakku
pub(in crate::card::sets) static MASTER_PAKKU: CardRecord = CardRecord::new(
    "Master Pakku",
    "91e30df5-63f7-4281-8d64-8d522d663652",
    "Olena Richards",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Advisor", "Ally"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Prowess",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Master Pakku becomes tapped, target player mills X \
                 cards, where X is the number of Lesson cards in your \
                 graveyard. (They put the top X cards of their library into \
                 their graveyard.)",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                },
            ),
        ]),
);

// TLA 64 — The Mechanist, Aerial Artisan
pub(in crate::card::sets) static THE_MECHANIST_AERIAL_ARTISAN: CardRecord = CardRecord::new(
    "The Mechanist, Aerial Artisan",
    "b910851e-9332-4c83-a790-d379667cabfc",
    "Le Vuong",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Artificer", "Ally"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, create a Clue token. \
                 (It's an artifact with \"{2}, Sacrifice this token: Draw a \
                 card.\")",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::activated_with_targets(
                "{T}: Until end of turn, target artifact token you control \
                 becomes a 3/1 Construct artifact creature with flying.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Token,
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Construct",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TLA 65 — North Pole Patrol
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static NORTH_POLE_PATROL: CardRecord = CardRecord::new(
    "North Pole Patrol",
    "c2c0c138-5b61-4949-8431-4a6d458ead6a",
    "Rose Benjamin",
    CardRules::unsupported(),
);

// TLA 66 — Octopus Form
pub(in crate::card::sets) static OCTOPUS_FORM: CardRecord = CardRecord::new(
    "Octopus Form",
    "ce96a826-53a0-4029-bf34-46779d133b13",
    "Norikatsu Miyoshi",
    CardRules::new_instant(mana_cost!("{U}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Target creature you control gets +1/+1 and gains hexproof \
             until end of turn. Untap it. (It can't be the target of \
             spells or abilities your opponents control.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        )]),
);

// TLA 67 — Otter-Penguin
pub(in crate::card::sets) static OTTER_PENGUIN: CardRecord = CardRecord::new(
    "Otter-Penguin",
    "480b6279-bb34-4b3f-a639-868b52af92b9",
    "Eilene Cherie",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Bird"], 2, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, this creature \
             gets +1/+2 until end of turn and can't be blocked this turn.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// TLA 68 — Rowdy Snowballers
pub(in crate::card::sets) static ROWDY_SNOWBALLERS: CardRecord = CardRecord::new(
    "Rowdy Snowballers",
    "17d71522-b133-4003-b787-0c742a2fd70e",
    "Mizutametori",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Peasant", "Ally"], 2, 2)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "When this creature enters, tap target creature an opponent \
             controls and put a stun counter on it. (If a permanent with a \
             stun counter would become untapped, remove one from it \
             instead.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )]),
);

// TLA 69 — Secret of Bloodbending
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs. Also needs control of another player for a combat phase or turn, including hidden information and all choices.
pub(in crate::card::sets) static SECRET_OF_BLOODBENDING: CardRecord = CardRecord::new(
    "Secret of Bloodbending",
    "9bb928ae-f636-4aee-9146-a7885e6a8976",
    "Olena Richards",
    CardRules::unsupported(),
);

// TLA 70 — Serpent of the Pass
// Audit: unsupported — Needs conditional flash evaluated on a card outside the battlefield; the casting path reads intrinsic flash and permissions from battlefield or resolving sources, not conditional abilities on the card being cast.
pub(in crate::card::sets) static SERPENT_OF_THE_PASS: CardRecord = CardRecord::new(
    "Serpent of the Pass",
    "87595843-15bc-48bb-8a81-3e6ad924ed44",
    "Eiji Kaneda",
    CardRules::unsupported(),
);

// TLA 71 — Sokka's Haiku
pub(in crate::card::sets) static SOKKA_S_HAIKU: CardRecord = CardRecord::new(
    "Sokka's Haiku",
    "004be4bc-ac3c-4026-8d36-f4687ab18c70",
    "Bun Toujo",
    CardRules::new_instant(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Counter target spell.\nDraw a card, then mill three \
             cards.\nUntap target land.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                }),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Land,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::counter_target(TargetIndex::PRIMARY),
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                },
            ]),
        )]),
);

// TLA 72 — The Spirit Oasis
pub(in crate::card::sets) static THE_SPIRIT_OASIS: CardRecord = CardRecord::new(
    "The Spirit Oasis",
    "3a1b1329-531c-47ed-9802-c505501f8fd9",
    "Slawek Fedorczuk",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Shrine"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When The Spirit Oasis enters, draw a card for each Shrine you \
                 control.",
                abilities::draw_cards(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
            ),
            AbilityDef::triggered(
                "Whenever another Shrine you control enters, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// TLA 73 — Spirit Water Revival
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static SPIRIT_WATER_REVIVAL: CardRecord = CardRecord::new(
    "Spirit Water Revival",
    "0c019e76-c88e-4d1b-a546-0f4e462ef44a",
    "Enishi",
    CardRules::unsupported(),
);

// TLA 74 — Teo, Spirited Glider
// Audit: unsupported — Needs a reflexive discard trigger that preserves the discarded card's characteristics and chooses its counter target only after the nonland discard; preselecting the target changes legality.
pub(in crate::card::sets) static TEO_SPIRITED_GLIDER: CardRecord = CardRecord::new(
    "Teo, Spirited Glider",
    "66906ed4-baac-4be0-9359-34f453d1a04a",
    "Robin Har",
    CardRules::unsupported(),
);

// TLA 75 — Tiger-Seal
pub(in crate::card::sets) static TIGER_SEAL: CardRecord = CardRecord::new(
    "Tiger-Seal",
    "7aef2891-1269-4a1a-acea-0aa37ef544c4",
    "Jinho Bae",
    CardRules::new_creature(mana_cost!("{U}"), &["Cat", "Seal"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, tap this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Tap {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, untap this \
             creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// TLA 76 — Ty Lee, Chi Blocker
// Audit: unsupported — Needs an untap restriction lasting only while the same player continuously controls the original Ty Lee object; existing source-presence durations do not end on a control change.
pub(in crate::card::sets) static TY_LEE_CHI_BLOCKER: CardRecord = CardRecord::new(
    "Ty Lee, Chi Blocker",
    "308cc687-9cb2-4e3a-98db-c5ba2a7da115",
    "Gemi",
    CardRules::unsupported(),
);

// TLA 77 — The Unagi of Kyoshi Island
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static THE_UNAGI_OF_KYOSHI_ISLAND: CardRecord = CardRecord::new(
    "The Unagi of Kyoshi Island",
    "0ecd8b38-9ee5-41a5-9b93-21c33fc1a6ff",
    "Miho Midorikawa",
    CardRules::unsupported(),
);

// TLA 78 — Wan Shi Tong, Librarian
// Audit: unsupported — Needs the entering spell's X retained in the resolving trigger after its source leaves; SourceCastX in effect values reads only a current battlefield permanent. Also needs a trigger for an opponent searching their library; no library-search event is published to card triggers.
pub(in crate::card::sets) static WAN_SHI_TONG_LIBRARIAN: CardRecord = CardRecord::new(
    "Wan Shi Tong, Librarian",
    "e20da6b5-1057-4a28-9e85-07de714e262f",
    "Ryota Murayama",
    CardRules::unsupported(),
);

// TLA 79 — Waterbender Ascension
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static WATERBENDER_ASCENSION: CardRecord = CardRecord::new(
    "Waterbender Ascension",
    "3f57e0f9-e232-489c-b991-d0d23f75d8dd",
    "Takeuchi Moto",
    CardRules::unsupported(),
);

// TLA 80 — Waterbending Lesson
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static WATERBENDING_LESSON: CardRecord = CardRecord::new(
    "Waterbending Lesson",
    "4f81e3b5-a0a9-4764-8c2d-b499ce1740b4",
    "Sylvain Sarrailh",
    CardRules::unsupported(),
);

// TLA 81 — Waterbending Scroll
pub(in crate::card::sets) static WATERBENDING_SCROLL: CardRecord = CardRecord::new(
    "Waterbending Scroll",
    "50d0be73-a2c8-44a0-9178-9949f342f6f9",
    "Dee Nguyen",
    CardRules::new_artifact(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::activated(
        "{6}, {T}: Draw a card. This ability costs {1} less to \
         activate for each Island you control.",
        &[CostDef::Mana(mana_cost!("{6}")), CostDef::TapSource],
        abilities::draw_cards(ValueDef::Constant(1)),
    )
    .with_activation_cost_reduction(
        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        )),
        0,
    )]),
);

// TLA 82 — Watery Grasp
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static WATERY_GRASP: CardRecord = CardRecord::new(
    "Watery Grasp",
    "26bcff57-428e-4f30-a153-a778fbfc437d",
    "Rose Benjamin",
    CardRules::unsupported(),
);

// TLA 83 — Yue, the Moon Spirit
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static YUE_THE_MOON_SPIRIT: CardRecord = CardRecord::new(
    "Yue, the Moon Spirit",
    "ecdac50f-c639-43fc-a03e-d488fac96ae2",
    "Yuumei",
    CardRules::unsupported(),
);

// TLA 84 — Azula Always Lies
pub(in crate::card::sets) static AZULA_ALWAYS_LIES: CardRecord = CardRecord::new(
    "Azula Always Lies",
    "416b9207-a2af-44d1-9b87-4943f6e46d42",
    "Robin Har",
    CardRules::new_instant(mana_cost!("{1}{B}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose one or both —",
            &[
                AbilityDef::spell_with_targets(
                    "Target creature gets -1/-1 until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
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
                AbilityDef::spell_with_targets(
                    "Put a +1/+1 counter on target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ],
        )
        .with_mode_selection(1, 2, false)]),
);

// TLA 85 — Azula, On the Hunt
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static AZULA_ON_THE_HUNT: CardRecord = CardRecord::new(
    "Azula, On the Hunt",
    "1335a145-248a-4f1e-8760-9a5d531e14e3",
    "Toraji",
    CardRules::unsupported(),
);

// TLA 86 — Beetle-Headed Merchants
pub(in crate::card::sets) static BEETLE_HEADED_MERCHANTS: CardRecord = CardRecord::new(
    "Beetle-Headed Merchants",
    "c2eed79f-38c1-4aac-9525-d54cb114f17f",
    "Norikatsu Miyoshi",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Human", "Citizen"], 5, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, you may sacrifice another \
             creature or artifact. If you do, draw a card and put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                &EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            )),
        ),
    ]),
);

// TLA 87 — Boiling Rock Rioter
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static BOILING_ROCK_RIOTER: CardRecord = CardRecord::new(
    "Boiling Rock Rioter",
    "739653cc-35c8-4a66-95d8-3e80fa6114f0",
    "Airi Yoshihisa",
    CardRules::unsupported(),
);

// TLA 88 — Buzzard-Wasp Colony
// Audit: unsupported — Needs one effect copying every counter kind and count from a dying creature's last-known counter inventory, including arbitrary named counters.
pub(in crate::card::sets) static BUZZARD_WASP_COLONY: CardRecord = CardRecord::new(
    "Buzzard-Wasp Colony",
    "42d83229-0555-4361-8964-4b525c825843",
    "Thomas Chamberlain-Keen",
    CardRules::unsupported(),
);

// TLA 89 — Callous Inspector
pub(in crate::card::sets) static CALLOUS_INSPECTOR: CardRecord = CardRecord::new(
    "Callous Inspector",
    "f22610e9-6b14-4914-bb67-bd6723bec9aa",
    "Enishi",
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "When this creature dies, it deals 1 damage to you. Create a \
             Clue token. (It's an artifact with \"{2}, Sacrifice this \
             token: Draw a card.\")",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Sequence(&[
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ]),
);

// TLA 90 — Canyon Crawler
pub(in crate::card::sets) static CANYON_CRAWLER: CardRecord = CardRecord::new(
    "Canyon Crawler",
    "30a30bfb-f0f3-425b-b37d-20079ee27046",
    "Kohei Hayama",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Spider", "Beast"], 6, 6).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger(
            "When this creature enters, create a Food token. (It's an \
             artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
             life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        abilities::typecycling!(
            "Swampcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp"))
        ),
    ]),
);

// TLA 91 — Cat-Gator
pub(in crate::card::sets) static CAT_GATOR: CardRecord = CardRecord::new(
    "Cat-Gator",
    "ec83a825-79e6-42a4-919c-3bc498ff4433",
    "Joseph Weston",
    CardRules::new_creature(mana_cost!("{6}{B}"), &["Fish", "Crocodile"], 3, 2).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals damage equal to the \
             number of Swamps you control to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ),
    ]),
);

// TLA 92 — Corrupt Court Official (reprint)
const CORRUPT_COURT_OFFICIAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ptk::CORRUPT_COURT_OFFICIAL,
    "4692610e-d64c-438e-b5ad-0cf67fd57f1f",
    "Norikatsu Miyoshi",
);

// TLA 93 — Dai Li Indoctrination
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static DAI_LI_INDOCTRINATION: CardRecord = CardRecord::new(
    "Dai Li Indoctrination",
    "eca652b8-44f1-4bd9-b4bf-036eeead13aa",
    "Lius Lasahido",
    CardRules::unsupported(),
);

// TLA 94 — Day of Black Sun
pub(in crate::card::sets) static DAY_OF_BLACK_SUN: CardRecord = CardRecord::new(
    "Day of Black Sun",
    "d0e24797-3e45-4c2b-b4b0-1ef44d42eaee",
    "Matteo Bassini",
    CardRules::new_sorcery(mana_cost!("{X}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "Each creature with mana value X or less loses all abilities \
         until end of turn. Destroy those creatures.",
        EffectDef::BindObjects(BindObjectsDef {
            source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            )),
            binding: crate::Binding!("doomed"),
            then: &EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "doomed"
                    ))),
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "doomed"
                    ))),
                    then: None,
                },
            ]),
        }),
    )]),
);

// TLA 95 — Deadly Precision
pub(in crate::card::sets) static DEADLY_PRECISION: CardRecord = CardRecord::new(
    "Deadly Precision",
    "4661daf6-d960-4278-8946-70948efaf99d",
    "Yuhong Ding",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )
    .with_spell_additional_cost(&CostDef::Choice(&[
        CostDef::Mana(mana_cost!("{4}")),
        CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ])),
    ]))]),
);

// TLA 96 — Epic Downfall (reprint)
const EPIC_DOWNFALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::EPIC_DOWNFALL,
    "3b4a6804-04f5-4467-bb1c-9466a47bc55f",
    "Hristo D. Chukov",
);

// TLA 97 — Fatal Fissure
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static FATAL_FISSURE: CardRecord = CardRecord::new(
    "Fatal Fissure",
    "3343933d-4425-4ede-8d92-876bd0c6df60",
    "Maojin Lee",
    CardRules::unsupported(),
);

// TLA 98 — The Fire Nation Drill
// Audit: unsupported — Needs a reflexive tap-payment trigger independent of a surviving source, choosing the destroy target after the optional tap; current optional-effect listeners only capture battlefield sources.
pub(in crate::card::sets) static THE_FIRE_NATION_DRILL: CardRecord = CardRecord::new(
    "The Fire Nation Drill",
    "54d762f6-e131-480f-b294-10f5a63d9c98",
    "Brandon L. Hunt",
    CardRules::unsupported(),
);

// TLA 99 — Fire Nation Engineer
// Audit: unsupported — Needs player attack history for raid that remains true after the attacking creatures leave or change controllers; current predicates only inspect individual battlefield creatures.
pub(in crate::card::sets) static FIRE_NATION_ENGINEER: CardRecord = CardRecord::new(
    "Fire Nation Engineer",
    "5daba108-277d-46c8-ac35-7610f4786813",
    "Norikatsu Miyoshi",
    CardRules::unsupported(),
);

// TLA 100 — Fire Navy Trebuchet
pub(in crate::card::sets) static FIRE_NAVY_TREBUCHET: CardRecord = CardRecord::new(
    "Fire Navy Trebuchet",
    "e58372cc-9a89-47a4-a0db-df5435e26cd3",
    "Mikio Masuda",
    CardRules::new_artifact_creature(mana_cost!("{2}{B}"), &["Wall"], 0, 4).with_abilities(&[
        abilities::defender(),
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever you attack, create a 2/1 colorless Construct \
             artifact creature token with flying named Ballistic Boulder \
             that's tapped and attacking. Sacrifice that token at the \
             beginning of the next end step.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::artifact_creature(&["Construct"], &[], 2, 1)
                        .with_name("Ballistic Boulder")
                        .with_abilities(&[abilities::flying()]),
                ))
                .entering_tapped()
                .entering_attacking()
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("boulder"),
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "At the beginning of the next end step, sacrifice that token.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::End,
                                player: PlayerRelation::Any,
                            },
                            EffectDef::sacrifice(EffectRecipientDef::objects(
                                ObjectSetDef::Binding(crate::Binding!("boulder")),
                            )),
                        ),
                    )),
                }),
            ),
        ),
    ]),
);

// TLA 101 — Foggy Swamp Hunters
pub(in crate::card::sets) static FOGGY_SWAMP_HUNTERS: CardRecord = CardRecord::new(
    "Foggy Swamp Hunters",
    "3f6e5869-ca25-4b98-a844-8a498cb40aab",
    "Bun Toujo",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Ranger", "Ally"], 3, 4)
        .with_abilities(&[AbilityDef::static_ability(
            "As long as you've drawn two or more cards this turn, this \
             creature has lifelink and menace. (It can't be blocked except \
             by two or more creatures.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(2),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                        AppliedEffectDef::add_ability(&abilities::menace()),
                    ]),
                },
            },
        )]),
);

// TLA 102 — Foggy Swamp Visions
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static FOGGY_SWAMP_VISIONS: CardRecord = CardRecord::new(
    "Foggy Swamp Visions",
    "3a46deaa-88f7-4eec-aa99-b85073847918",
    "Hori Airi",
    CardRules::unsupported(),
);

// TLA 103 — Heartless Act (reprint)
const HEARTLESS_ACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_iko::HEARTLESS_ACT,
    "b57c57a8-d72b-4c6c-a2db-a7ef61190f42",
    "Sylvain Sarrailh",
);

// TLA 104 — Hog-Monkey
pub(in crate::card::sets) static HOG_MONKEY: CardRecord = CardRecord::new(
    "Hog-Monkey",
    "4f442970-5355-4abf-8684-17daaa8e469b",
    "Miho Midorikawa",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Boar", "Monkey"], 3, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, target creature you \
             control with a +1/+1 counter on it gains menace until end of \
             turn. (It can't be blocked except by two or more creatures.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::menace()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        crate::card::sets::aetherdrift::exhaust(AbilityDef::activated(
            "Exhaust — {5}: Put two +1/+1 counters on this creature. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{5}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )),
    ]),
);

// TLA 105 — Joo Dee, One of Many
pub(in crate::card::sets) static JOO_DEE_ONE_OF_MANY: CardRecord = CardRecord::new(
    "Joo Dee, One of Many",
    "7ae1439f-a0c4-42c2-a4f3-8851defa981e",
    "Olena Richards",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Advisor"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{B}, {T}: Surveil 1. Create a token that's a copy of this \
             creature, then sacrifice an artifact or creature. Activate \
             only as a sorcery. (To surveil 1, look at the top card of \
             your library. You may put it into your graveyard.)",
            &[CostDef::Mana(mana_cost!("{B}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                abilities::surveil(ValueDef::Constant(1)),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                    object: &EffectRecipientDef::Source,
                    exceptions: CopyExceptionsDef::NONE,
                }))),
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Controller,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// TLA 106 — June, Bounty Hunter
pub(in crate::card::sets) static JUNE_BOUNTY_HUNTER: CardRecord = CardRecord::new(
    "June, Bounty Hunter",
    "148bbaab-bc7b-46ab-8e18-ade69d71d847",
    "Shiren",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Mercenary"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "June can't be blocked as long as you've drawn two or more \
                 cards this turn.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(2),
                    }),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                    },
                },
            ),
            AbilityDef::activated(
                "{1}, Sacrifice another creature: Create a Clue token. \
                 Activate only during your turn. (It's an artifact with \"{2}, \
                 Sacrifice this token: Draw a card.\")",
                &[
                    CostDef::Mana(mana_cost!("{1}")),
                    CostDef::sacrifice_permanents(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        PlayerRelation::You,
                        1,
                    ),
                ],
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            )
            .with_activation_timing(ActivationTimingDef::YourTurn),
        ]),
);

// TLA 107 — Koh, the Face Stealer
// Audit: unsupported — Needs persistent selection of one linked exiled card and continuous inheritance of both its activated and triggered abilities; the current inheritance operation grants only activated abilities from the entire linked collection.
pub(in crate::card::sets) static KOH_THE_FACE_STEALER: CardRecord = CardRecord::new(
    "Koh, the Face Stealer",
    "28f6fa32-5058-4c29-9ba8-9d8f2057eb6d",
    "Eduardo Francisco",
    CardRules::unsupported(),
);

// TLA 108 — Lo and Li, Twin Tutors
// Audit: unsupported — Needs a continuous lifelink grant to Lesson spells on the stack; current nonpermanent ability-layer support does not grant executable keywords to those spell objects.
pub(in crate::card::sets) static LO_AND_LI_TWIN_TUTORS: CardRecord = CardRecord::new(
    "Lo and Li, Twin Tutors",
    "cba9260d-ef07-4429-99f9-6004cc1fea0f",
    "AKAGI",
    CardRules::unsupported(),
);

// TLA 109 — Mai, Scornful Striker
pub(in crate::card::sets) static MAI_SCORNFUL_STRIKER: CardRecord = CardRecord::new(
    "Mai, Scornful Striker",
    "74dd4c0e-27b8-4c47-b7a6-a281413cd6b4",
    "Hori Airi",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Noble", "Ally"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered(
                "Whenever a player casts a noncreature spell, they lose 2 life.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Any),
                ])),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::EventPlayer),
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// TLA 110 — Merchant of Many Hats
pub(in crate::card::sets) static MERCHANT_OF_MANY_HATS: CardRecord = CardRecord::new(
    "Merchant of Many Hats",
    "752ffa24-93b6-4b33-bf10-7222357ac472",
    "Boell Oyino",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Peasant", "Ally"], 2, 2)
        .with_abilities(&[AbilityDef::activated(
            "{2}{B}: Return this card from your graveyard to your hand.",
            &[CostDef::Mana(mana_cost!("{2}{B}"))],
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )
        .with_source_zones(&[ZoneKind::Graveyard])]),
);

// TLA 111 — Northern Air Temple
pub(in crate::card::sets) static NORTHERN_AIR_TEMPLE: CardRecord = CardRecord::new(
    "Northern Air Temple",
    "06b7675e-e665-49cb-a8d9-7092a654d464",
    "Slawek Fedorczuk",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Shrine"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Northern Air Temple enters, each opponent loses X life \
                 and you gain X life, where X is the number of Shrines you \
                 control.",
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    },
                ]),
            ),
            AbilityDef::triggered(
                "Whenever another Shrine you control enters, each opponent \
                 loses 1 life and you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// TLA 112 — Obsessive Pursuit
// Audit: unsupported — Needs a per-player count of all permanents sacrificed this turn, retained across zone changes and independent of whether this enchantment was present.
pub(in crate::card::sets) static OBSESSIVE_PURSUIT: CardRecord = CardRecord::new(
    "Obsessive Pursuit",
    "e837e29c-d241-43c8-8f45-05056e082b60",
    "Ichiko Milk Tei",
    CardRules::unsupported(),
);

// TLA 113 — Ozai's Cruelty
pub(in crate::card::sets) static OZAI_S_CRUELTY: CardRecord = CardRecord::new(
    "Ozai's Cruelty",
    "22cad680-d46a-4589-a633-b6ee9a78d61e",
    "ikeda_cpt",
    CardRules::new_sorcery(mana_cost!("{2}{B}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Ozai's Cruelty deals 2 damage to target player. That player \
             discards two cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        )]),
);

// TLA 114 — Phoenix Fleet Airship
// Audit: unsupported — Needs player sacrifice history for its end-step intervening-if condition, including sacrifices before this Vehicle entered and objects no longer on the battlefield.
pub(in crate::card::sets) static PHOENIX_FLEET_AIRSHIP: CardRecord = CardRecord::new(
    "Phoenix Fleet Airship",
    "b51d3259-c41c-4f64-9666-0a9e676c812f",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// TLA 115 — Pirate Peddlers
pub(in crate::card::sets) static PIRATE_PEDDLERS: CardRecord = CardRecord::new(
    "Pirate Peddlers",
    "5c998994-6c3b-4f34-9b31-babba2b17266",
    "Eduardo Francisco",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Pirate"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever you sacrifice another permanent, put a +1/+1 counter \
             on this creature.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TLA 116 — Raven Eagle
pub(in crate::card::sets) static RAVEN_EAGLE: CardRecord = CardRecord::new(
    "Raven Eagle",
    "2262f24b-db6b-4f86-96d0-47a20fc015ab",
    "Robin Olausson",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Bird", "Assassin"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature enters or attacks, exile up to one \
             target card from a graveyard. If a creature card is exiled \
             this way, create a Clue token. (It's an artifact with \"{2}, \
             Sacrifice this token: Draw a card.\")",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("exiled"),
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountObjects(&ObjectSetDef::Matching {
                            objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                "exiled"
                            )),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                        }),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, each opponent \
             loses 1 life and you gain 1 life.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// TLA 117 — The Rise of Sozin // Fire Lord Sozin
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat. Also needs a single four-card search limit across graveyard, hand, and library, and target-group bounds based on the later optional X payment.
pub(in crate::card::sets) static THE_RISE_OF_SOZIN: CardRecord = CardRecord::new(
    "The Rise of Sozin // Fire Lord Sozin",
    "14eadf46-90c2-4376-8183-6a922a60174d",
    "Mitori",
    CardRules::unsupported(),
);

// TLA 118 — Ruinous Waterbending
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static RUINOUS_WATERBENDING: CardRecord = CardRecord::new(
    "Ruinous Waterbending",
    "53161747-c82b-41e8-90ea-7791ea262a85",
    "Yuu Fujiki",
    CardRules::unsupported(),
);

// TLA 119 — Sold Out
pub(in crate::card::sets) static SOLD_OUT: CardRecord = CardRecord::new(
    "Sold Out",
    "1affba6d-2cca-4cdf-8690-1e23ffbe8462",
    "Nijihayashi",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature. If it was dealt damage this turn, \
         create a Clue token. (It's an artifact with \"{2}, Sacrifice \
         this token: Draw a card.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::TargetMatches {
                slot: TargetIndex::PRIMARY,
                object: ObjectPredicateDef::WasDealtDamageThisTurn,
            },
            then: &EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
            otherwise: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        },
    )]),
);

// TLA 120 — Swampsnare Trap
// Audit: unsupported — Needs a self spell-cost reduction that reads the declared target's flying ability; the current target-sensitive cost condition applies to other spells targeting the cost-modifying source.
pub(in crate::card::sets) static SWAMPSNARE_TRAP: CardRecord = CardRecord::new(
    "Swampsnare Trap",
    "6348b6bf-08a7-45f0-8b2d-2827ab89f21c",
    "Yoshioka",
    CardRules::unsupported(),
);

// TLA 121 — Tundra Tank
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static TUNDRA_TANK: CardRecord = CardRecord::new(
    "Tundra Tank",
    "13c66fce-8f8e-4a98-8413-9f05aebf6e13",
    "Shishizaru",
    CardRules::unsupported(),
);

// TLA 122 — Wolfbat
pub(in crate::card::sets) static WOLFBAT: CardRecord = CardRecord::new(
    "Wolfbat",
    "2ae83b4e-e5a4-4c98-8d16-eef3c71b8ff2",
    "Daniel Romanovsky",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Wolf", "Bat"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, you may pay \
             {B}. If you do, return this card from your graveyard to the \
             battlefield with a finality counter on it. (If a creature \
             with a finality counter on it would die, exile it instead.)",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{B}"))],
                &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::Finality,
                            amount: 1,
                        }],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// TLA 123 — Zuko's Conviction
pub(in crate::card::sets) static ZUKO_S_CONVICTION: CardRecord = CardRecord::new(
    "Zuko's Conviction",
    "ad8933d6-cdc7-4d60-a78e-b43ffecfb136",
    "Kieran Yanner",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
        abilities::kicker(&[CostDef::Mana(mana_cost!("{4}"))]),
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to your hand. \
             If this spell was kicked, instead put that card onto the \
             battlefield tapped.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                    crate::AdditionalCostIndex::PRIMARY,
                ),
                then: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
                otherwise: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// TLA 124 — Boar-q-pine
pub(in crate::card::sets) static BOAR_Q_PINE: CardRecord = CardRecord::new(
    "Boar-q-pine",
    "087a445c-5f63-45d6-83e8-20d22650db20",
    "Brandon L. Hunt",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Boar", "Porcupine"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, put a +1/+1 counter on \
             this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TLA 125 — Bumi Bash
pub(in crate::card::sets) static BUMI_BASH: CardRecord = CardRecord::new(
    "Bumi Bash",
    "e8895479-26cb-4ee8-98ca-6d46c43f0dbd",
    "Maël Ollivier-Henry",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Bumi Bash deals damage equal to the number of lands you \
                 control to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target land creature or nonbasic land.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )]),
);

// TLA 126 — The Cave of Two Lovers
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static THE_CAVE_OF_TWO_LOVERS: CardRecord = CardRecord::new(
    "The Cave of Two Lovers",
    "50bf5c8b-f218-46b7-843c-8d8083f02fd2",
    "Ittoku",
    CardRules::unsupported(),
);

// TLA 127 — Combustion Man
pub(in crate::card::sets) static COMBUSTION_MAN: CardRecord = CardRecord::new(
    "Combustion Man",
    "86f7399d-6876-4e85-ba34-ff1f97dc144a",
    "Pisukev",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human", "Assassin"], 4, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever Combustion Man attacks, destroy target permanent \
             unless its controller has Combustion Man deal damage to them \
             equal to his power.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                choices: &[
                    EffectChoiceDef {
                        label: "Take damage",
                        effect: EffectDef::damage(
                            EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                                ObjectRefDef::Target(TargetIndex::PRIMARY),
                            )),
                            ValueDef::SourcePower,
                        ),
                    },
                    EffectChoiceDef {
                        label: "Destroy the permanent",
                        effect: EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                    },
                ],
            },
        )]),
);

// TLA 128 — Combustion Technique
pub(in crate::card::sets) static COMBUSTION_TECHNIQUE: CardRecord = CardRecord::new(
    "Combustion Technique",
    "fdcca576-2ef2-44cc-9944-a92bd146444a",
    "Devin Elle Kurtz",
    CardRules::new_instant(mana_cost!("{1}{R}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Combustion Technique deals damage equal to 2 plus the number \
             of Lesson cards in your graveyard to target creature. If that \
             creature would die this turn, exile it instead.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Sum(&SumValueDef::new(
                        ValueDef::Constant(2),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    )),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        )]),
);

// TLA 129 — Crescent Island Temple
pub(in crate::card::sets) static CRESCENT_ISLAND_TEMPLE: CardRecord = CardRecord::new(
    "Crescent Island Temple",
    "04c85150-269e-4158-a1b9-bd11bc6b7d79",
    "Luc Courtois",
    CardRules::new_enchantment(mana_cost!("{3}{R}"))
        .with_subtypes(&["Shrine"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Crescent Island Temple enters, for each Shrine you \
                 control, create a 1/1 red Monk creature token with prowess. \
                 (Whenever you cast a noncreature spell, it gets +1/+1 until \
                 end of turn.)",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(MONK_TOKEN)).with_count(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    ),
                ),
            ),
            AbilityDef::triggered(
                "Whenever another Shrine you control enters, create a 1/1 red \
                 Monk creature token with prowess.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MONK_TOKEN))),
            ),
        ]),
);

// TLA 130 — Cunning Maneuver
pub(in crate::card::sets) static CUNNING_MANEUVER: CardRecord = CardRecord::new(
    "Cunning Maneuver",
    "0ff7c993-ba29-43b0-9639-7c4bc0292fa2",
    "Robin Har",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+1 until end of turn.\nCreate a Clue \
         token. (It's an artifact with \"{2}, Sacrifice this token: \
         Draw a card.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ]),
    )]),
);

// TLA 131 — Deserter's Disciple
pub(in crate::card::sets) static DESERTER_S_DISCIPLE: CardRecord = CardRecord::new(
    "Deserter's Disciple",
    "14f2ed5a-042b-4cce-82ad-cfb4bd511d98",
    "HAIKEI",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Rebel", "Ally"], 2, 2)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{T}: Another target creature you control with power 2 or less \
             can't be blocked this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// TLA 132 — Fated Firepower
// Audit: unsupported — Needs a damage replacement adding the current source counter count to each matching damage event; damage multiplication and prevention do not implement additive modification.
pub(in crate::card::sets) static FATED_FIREPOWER: CardRecord = CardRecord::new(
    "Fated Firepower",
    "51352127-1f86-42e9-b4ca-2fd58b14e86b",
    "Takayama Toshiaki",
    CardRules::unsupported(),
);

// TLA 133 — Fire Nation Attacks
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIRE_NATION_ATTACKS: CardRecord = CardRecord::new(
    "Fire Nation Attacks",
    "9cc10845-8989-46ca-a57a-fd728fca0729",
    "Claudiu-Antoniu Magherusan",
    CardRules::unsupported(),
);

// TLA 134 — Fire Nation Cadets
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIRE_NATION_CADETS: CardRecord = CardRecord::new(
    "Fire Nation Cadets",
    "1a3a862c-9c9a-41cd-94d1-2d0cff22a6cd",
    "Rafater",
    CardRules::unsupported(),
);

// TLA 135 — Fire Nation Raider
// Audit: unsupported — Needs player attack history for raid that remains true after the attacking creatures leave or change controllers; current predicates only inspect individual battlefield creatures.
pub(in crate::card::sets) static FIRE_NATION_RAIDER: CardRecord = CardRecord::new(
    "Fire Nation Raider",
    "ede1bbdb-c726-46e3-aaf1-b8cf2be2c341",
    "Tubaki Halsame",
    CardRules::unsupported(),
);

// TLA 136 — Fire Sages
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIRE_SAGES: CardRecord = CardRecord::new(
    "Fire Sages",
    "71a9fa44-306f-417a-a9d9-991aff95025c",
    "Yuu Fujiki",
    CardRules::unsupported(),
);

// TLA 137 — Firebender Ascension
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat. Also needs a trigger caused by an attacking creature triggering its own ability, with that ability retained as the copy target.
pub(in crate::card::sets) static FIREBENDER_ASCENSION: CardRecord = CardRecord::new(
    "Firebender Ascension",
    "2929b702-03c6-4cf0-a3f7-61b27f4803be",
    "Tetsuko",
    CardRules::unsupported(),
);

// TLA 138 — Firebending Lesson
pub(in crate::card::sets) static FIREBENDING_LESSON: CardRecord = CardRecord::new(
    "Firebending Lesson",
    "fa940e68-010e-4b68-be8a-555d7068f7b4",
    "Toni Infante",
    CardRules::new_instant(mana_cost!("{R}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[
            abilities::kicker(&[CostDef::Mana(mana_cost!("{4}"))]),
            AbilityDef::spell_with_targets(
                "Firebending Lesson deals 2 damage to target creature. If this \
                 spell was kicked, it deals 5 damage to that creature instead.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        crate::AdditionalCostIndex::PRIMARY,
                    ),
                    then: &EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(5),
                    ),
                    otherwise: &EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(2),
                    ),
                },
            ),
        ]),
);

// TLA 139 — Firebending Student
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIREBENDING_STUDENT: CardRecord = CardRecord::new(
    "Firebending Student",
    "3b366f59-16fe-43cb-888d-1f93ef8fd332",
    "Kozato",
    CardRules::unsupported(),
);

// TLA 140 — How to Start a Riot
pub(in crate::card::sets) static HOW_TO_START_A_RIOT: CardRecord = CardRecord::new(
    "How to Start a Riot",
    "23b3bf1e-ea85-47f5-8473-4d16615f68d7",
    "Robin Olausson",
    CardRules::new_instant(mana_cost!("{2}{R}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Target creature gains menace until end of turn. (It can't be \
             blocked except by two or more creatures.)\nCreatures target \
             player controls get +2/+0 until end of turn.",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::menace()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::controlled_by(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex(1))),
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        )]),
);

// TLA 141 — Iroh's Demonstration
pub(in crate::card::sets) static IROH_S_DEMONSTRATION: CardRecord = CardRecord::new(
    "Iroh's Demonstration",
    "18d15fed-1f8f-4407-a221-a47ce75001a8",
    "Song Qijin",
    CardRules::new_sorcery(mana_cost!("{1}{R}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose one —",
            &[
                AbilityDef::spell(
                    "Iroh's Demonstration deals 1 damage to each creature your \
                     opponents control.",
                    EffectDef::damage(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        ))),
                        ValueDef::Constant(1),
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Iroh's Demonstration deals 4 damage to target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(4),
                    ),
                ),
            ],
        )]),
);

// TLA 142 — Jeong Jeong, the Deserter
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat. Also needs a delayed trigger combining next-matching-cast consumption with expiration at end of turn.
pub(in crate::card::sets) static JEONG_JEONG_THE_DESERTER: CardRecord = CardRecord::new(
    "Jeong Jeong, the Deserter",
    "a9f63d3b-bee5-48bc-8b04-d8b24b0bda6e",
    "Danciao",
    CardRules::unsupported(),
);

// TLA 143 — Jet's Brainwashing
pub(in crate::card::sets) static JET_S_BRAINWASHING: CardRecord = CardRecord::new(
    "Jet's Brainwashing",
    "b17e8bdb-4b91-4a9d-bfe1-8a55f0bd040b",
    "Enishi",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        abilities::kicker(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Target creature can't block this turn. If this spell was \
             kicked, gain control of that creature until end of turn, \
             untap it, and it gains haste until end of turn.\nCreate a \
             Clue token. (It's an artifact with \"{2}, Sacrifice this \
             token: Draw a card.\")",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        crate::AdditionalCostIndex::PRIMARY,
                    ),
                    then: &EffectDef::Sequence(&[
                        EffectDef::gain_control(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            PlayerRefDef::EffectController,
                            ControlDurationDef::UntilEndOfTurn,
                        ),
                        EffectDef::Untap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ]),
);

// TLA 144 — The Last Agni Kai
// Audit: unsupported — Needs retention of unspent red mana across step and phase boundaries until end of turn; current mana units have no such retention effect.
pub(in crate::card::sets) static THE_LAST_AGNI_KAI: CardRecord = CardRecord::new(
    "The Last Agni Kai",
    "61eaebc6-7575-48ed-b212-ff8b0c7ae694",
    "Pablo Rivera",
    CardRules::unsupported(),
);

// TLA 145 — The Legend of Roku // Avatar Roku
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static THE_LEGEND_OF_ROKU: CardRecord = CardRecord::new(
    "The Legend of Roku // Avatar Roku",
    "95f2f5af-d405-4534-8683-5a9001f997b4",
    "Song Qijin",
    CardRules::unsupported(),
);

// TLA 146 — Lightning Strike (reprint)
const LIGHTNING_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::LIGHTNING_STRIKE,
    "5787b0e0-9469-4a6d-8b81-c992628e28c0",
    "Jo Cordisco",
);

// TLA 147 — Mai, Jaded Edge
pub(in crate::card::sets) static MAI_JADED_EDGE: CardRecord = CardRecord::new(
    "Mai, Jaded Edge",
    "732e6bc9-0798-4c00-aea0-5ef4298b45f5",
    "Toraji",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Noble"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Prowess",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            crate::card::sets::aetherdrift::exhaust(AbilityDef::activated(
                "Exhaust — {3}: Put a double strike counter on Mai. (Activate \
                 each exhaust ability only once.)",
                &[CostDef::Mana(mana_cost!("{3}"))],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::DoubleStrike,
                    amount: ValueDef::Constant(1),
                },
            )),
        ]),
);

// TLA 148 — Mongoose Lizard
pub(in crate::card::sets) static MONGOOSE_LIZARD: CardRecord = CardRecord::new(
    "Mongoose Lizard",
    "7d40b9cb-82ef-44c5-8d3c-c8bb4ce9891c",
    "Joseph Weston",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Mongoose", "Lizard"], 5, 6).with_abilities(
        &[
            abilities::menace(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, it deals 1 damage to any target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
            abilities::typecycling!(
                "Mountaincycling {2}",
                &[CostDef::Mana(mana_cost!("{2}"))],
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain"))
            ),
        ],
    ),
);

// TLA 149 — Price of Freedom
pub(in crate::card::sets) static PRICE_OF_FREEDOM: CardRecord = CardRecord::new(
    "Price of Freedom",
    "9fbe94e9-a71d-4a31-9210-c599abe08e3f",
    "Kotakan",
    CardRules::new_sorcery(mana_cost!("{1}{R}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Destroy target artifact or land an opponent controls. Its \
             controller may search their library for a basic land card, \
             put it onto the battlefield tapped, then shuffle.\nDraw a \
             card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::May {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: true,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        )]),
);

// TLA 150 — Ran and Shaw
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static RAN_AND_SHAW: CardRecord = CardRecord::new(
    "Ran and Shaw",
    "6436e2d0-989f-47e3-90bc-9cde82a2ddb4",
    "Miho Midorikawa",
    CardRules::unsupported(),
);

// TLA 151 — Redirect Lightning
// Audit: unsupported — Needs a replayable choice between two objectless additional costs, paying life or paying mana; current cast actions identify cost branches through chosen objects, so these two alternatives serialize identically.
pub(in crate::card::sets) static REDIRECT_LIGHTNING: CardRecord = CardRecord::new(
    "Redirect Lightning",
    "2b5b14a7-1fdd-4efc-b197-cadfa7f7c860",
    "Toni Infante",
    CardRules::unsupported(),
);

// TLA 152 — Rough Rhino Cavalry
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static ROUGH_RHINO_CAVALRY: CardRecord = CardRecord::new(
    "Rough Rhino Cavalry",
    "a8bc29cf-f6f8-4bb8-b722-f016b58b6d2d",
    "Yuhong Ding",
    CardRules::unsupported(),
);

// TLA 153 — Solstice Revelations
// Audit: unsupported — Needs conditional free casting based on the mana value of the actual proposed spell (including alternate faces) versus the current Mountain count, with an uncast selected card moved to hand.
pub(in crate::card::sets) static SOLSTICE_REVELATIONS: CardRecord = CardRecord::new(
    "Solstice Revelations",
    "f22ac19f-66fb-4d54-9f09-495a20a29577",
    "Kotakan",
    CardRules::unsupported(),
);

// TLA 154 — Sozin's Comet
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static SOZIN_S_COMET: CardRecord = CardRecord::new(
    "Sozin's Comet",
    "649e50e5-299b-4191-87a8-36e9378795be",
    "Salvatorre Zee Yazzie",
    CardRules::unsupported(),
);

// TLA 155 — Tiger-Dillo
pub(in crate::card::sets) static TIGER_DILLO: CardRecord = CardRecord::new(
    "Tiger-Dillo",
    "067bd117-04e6-410e-89c4-6d431d627751",
    "John Di Giovanni",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Cat", "Armadillo"], 4, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't attack or block unless you control \
             another creature with power 4 or greater.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            },
        ),
    ]),
);

// TLA 156 — Treetop Freedom Fighters
pub(in crate::card::sets) static TREETOP_FREEDOM_FIGHTERS: CardRecord = CardRecord::new(
    "Treetop Freedom Fighters",
    "a9394200-7ffc-440f-8b5f-c08b7930133c",
    "AKAGI",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Rebel", "Ally"], 2, 1)
        .with_abilities(&[
            abilities::haste(),
            abilities::enters_trigger(
                "When this creature enters, create a 1/1 white Ally creature \
                 token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
            ),
        ]),
);

// TLA 157 — Twin Blades
pub(in crate::card::sets) static TWIN_BLADES: CardRecord = CardRecord::new(
    "Twin Blades",
    "dc399ae0-36b5-4c92-9fe2-138caf8d7a86",
    "Jo Cordisco",
    CardRules::new_artifact(mana_cost!("{2}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control. That creature gains double strike until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Attach {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// TLA 158 — Ty Lee, Artful Acrobat
// Audit: unsupported — Needs a reflexive optional-payment trigger that survives the attacking source leaving and selects the target only after payment; current battlefield optional-effect listeners lose the trigger when that source is gone.
pub(in crate::card::sets) static TY_LEE_ARTFUL_ACROBAT: CardRecord = CardRecord::new(
    "Ty Lee, Artful Acrobat",
    "dcd9df24-272b-4aa1-b05f-6ee6b3d3dfe7",
    "Rose Benjamin",
    CardRules::unsupported(),
);

// TLA 159 — War Balloon
pub(in crate::card::sets) static WAR_BALLOON: CardRecord = CardRecord::new(
    "War Balloon",
    "6829b20d-c2fa-41a6-89ca-f21c522d8866",
    "Matteo Bassini",
    CardRules::new_vehicle(mana_cost!("{2}{R}"), 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}: Put a fire counter on this Vehicle.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("fire"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "As long as this Vehicle has three or more fire counters on \
             it, it's an artifact creature.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("fire"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                },
            },
        ),
        abilities::crew("Crew 3", 3),
    ]),
);

// TLA 160 — Wartime Protestors
pub(in crate::card::sets) static WARTIME_PROTESTORS: CardRecord = CardRecord::new(
    "Wartime Protestors",
    "bac81940-d717-49ff-83b2-16a22bb2c988",
    "Yosuke Adachi",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Rebel", "Ally"], 4, 4)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever another Ally you control enters, put a +1/+1 counter \
                 on that creature and it gains haste until end of turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::TriggeringObject,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::TriggeringObject,
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// TLA 161 — Yuyan Archers
pub(in crate::card::sets) static YUYAN_ARCHERS: CardRecord = CardRecord::new(
    "Yuyan Archers",
    "99244462-a996-4a5b-91fb-947045647d6d",
    "Domco.",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Archer"], 3, 1).with_abilities(&[
        abilities::reach(),
        abilities::enters_trigger(
            "When this creature enters, you may discard a card. If you do, \
             draw a card.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::discard(ObjectPredicateDef::Any)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// TLA 162 — Zhao, the Moon Slayer
pub(in crate::card::sets) static ZHAO_THE_MOON_SLAYER: CardRecord = CardRecord::new(
    "Zhao, the Moon Slayer",
    "f1015bf5-de98-41f3-b6b1-ed95f7465944",
    "Toraji",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Soldier"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::replacement_for(
                "Nonbasic lands enter tapped.",
                ReplacementEventDef::ObjectEntersBattlefield {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Basic,
                        )),
                    ]),
                    controller: PlayerRelation::Any,
                    cast: None,
                },
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
            ),
            AbilityDef::activated(
                "{7}: Put a conqueror counter on Zhao.",
                &[CostDef::Mana(mana_cost!("{7}"))],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("conqueror"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "As long as Zhao has a conqueror counter on him, nonbasic \
                 lands are Mountains. (They lose all other land types and \
                 abilities and have \"{T}: Add {R}.\")",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("conqueror"),
                        comparison: ComparisonDef::Greater,
                        amount: 0,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                        CardSupertype::Basic,
                                    )),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            ),
                        )),
                        effect: AppliedEffectDef::Characteristic(
                            CharacteristicOperationDef::BasicLandTypes(SetOperationDef::Set(&[
                                crate::card::BasicLandType::Mountain,
                            ])),
                        ),
                    },
                },
            ),
        ]),
);

// TLA 163 — Zuko, Exiled Prince
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static ZUKO_EXILED_PRINCE: CardRecord = CardRecord::new(
    "Zuko, Exiled Prince",
    "6a73b372-9c0e-4a85-89d2-440163330687",
    "Nijihayashi",
    CardRules::unsupported(),
);

// TLA 164 — Allies at Last
pub(in crate::card::sets) static ALLIES_AT_LAST: CardRecord = CardRecord::new(
    "Allies at Last",
    "11a77897-2aba-4a9b-bbe6-1768ca9f12cb",
    "Evan Shipard",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Allies (This spell costs {1} less to cast for \
             each Ally you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell_with_targets(
            "Up to two target creatures you control each deal damage equal \
             to their power to target creature an opponent controls.",
            &[
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                ),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )
                .another(),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
            ],
            EffectDef::damage_simultaneously(&[
                DamageAssignmentDef::from(
                    ObjectRefDef::Target(TargetIndex(0)),
                    EffectRecipientDef::Target(TargetIndex(2)),
                    ValueDef::TargetPower(TargetIndex(0)),
                ),
                DamageAssignmentDef::from(
                    ObjectRefDef::Target(TargetIndex(1)),
                    EffectRecipientDef::Target(TargetIndex(2)),
                    ValueDef::TargetPower(TargetIndex(1)),
                ),
            ]),
        ),
    ]),
);

// TLA 165 — Avatar Destiny
pub(in crate::card::sets) static AVATAR_DESTINY: CardRecord = CardRecord::new(
    "Avatar Destiny",
    "4d74da72-8443-4fce-9d64-d2041f6a3292",
    "Iwamoto05",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature you control",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each creature card in your \
                 graveyard and is an Avatar in addition to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Avatar",
                        ])),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature dies, mill cards equal to its power. \
                 Return this card to its owner's hand and up to one creature \
                 card milled this way to the battlefield under your control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                objects: ObjectSetDef::One(ObjectRefDef::TriggeringObject),
                                select: ObjectValueDef::Power,
                                operation: AggregateOperationDef::Maximum,
                            }),
                        },
                    },
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::One(ObjectRefDef::ZoneChangeSuccessor(
                            crate::card::model::ZoneChangeReferenceDef::Source,
                        )),
                        from: Some(ZoneKind::Graveyard),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            arrival: BattlefieldArrivalDef {
                                controller: Some(PlayerRelation::You),
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                    }),
                ]),
            ),
        ]),
);

// TLA 166 — Badgermole
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static BADGERMOLE: CardRecord = CardRecord::new(
    "Badgermole",
    "aabbd420-b7fc-496f-9168-bad823d51d9e",
    "Matteo Bassini",
    CardRules::unsupported(),
);

// TLA 167 — Badgermole Cub
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static BADGERMOLE_CUB: CardRecord = CardRecord::new(
    "Badgermole Cub",
    "340c5799-4964-44dd-8c48-8f3f3aba5211",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// TLA 168 — The Boulder, Ready to Rumble
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static THE_BOULDER_READY_TO_RUMBLE: CardRecord = CardRecord::new(
    "The Boulder, Ready to Rumble",
    "ec27a466-5457-44c6-a842-1de7d3788d66",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// TLA 169 — Bumi, King of Three Trials
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static BUMI_KING_OF_THREE_TRIALS: CardRecord = CardRecord::new(
    "Bumi, King of Three Trials",
    "a268697b-22b0-4e1b-a5b6-d9be95025e57",
    "Thomas Chamberlain-Keen",
    CardRules::unsupported(),
);

// TLA 170 — Cycle of Renewal
pub(in crate::card::sets) static CYCLE_OF_RENEWAL: CardRecord = CardRecord::new(
    "Cycle of Renewal",
    "233f4a05-057b-48f4-8f56-8d4a060a40e4",
    "Jocelin Carmes",
    CardRules::new_instant(mana_cost!("{2}{G}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell(
            "Sacrifice a land. Search your library for up to two basic \
             land cards, put them onto the battlefield tapped, then \
             shuffle.",
            EffectDef::Sequence(&[
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Controller,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Land),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ]),
        )]),
);

// TLA 171 — Diligent Zookeeper
// Audit: unsupported — Needs a continuous per-recipient count of its creature types, capped at ten; current values do not expose creature-type cardinality.
pub(in crate::card::sets) static DILIGENT_ZOOKEEPER: CardRecord = CardRecord::new(
    "Diligent Zookeeper",
    "21f52564-9820-42dc-a08d-459d51afc397",
    "Maojin Lee",
    CardRules::unsupported(),
);

// TLA 172 — The Earth King
// Audit: unsupported — Needs the frozen number of matching power-four attackers from the declaration; the attack matcher filters its trigger but TriggerEventAmount contains the total number of attackers, and a resolution-time battlefield count is not equivalent.
pub(in crate::card::sets) static THE_EARTH_KING: CardRecord = CardRecord::new(
    "The Earth King",
    "a8d5dca6-381a-4361-b265-27de8d04335c",
    "Ryota Murayama",
    CardRules::unsupported(),
);

// TLA 173 — Earth Kingdom General
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change. Also needs counter-placement events identifying the placing player and a use limit consumed only when life gain is accepted.
pub(in crate::card::sets) static EARTH_KINGDOM_GENERAL: CardRecord = CardRecord::new(
    "Earth Kingdom General",
    "8688fce5-74b1-41e1-a59a-05a3878a75cb",
    "Alexandr Leskinen",
    CardRules::unsupported(),
);

// TLA 174 — Earth Rumble
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static EARTH_RUMBLE: CardRecord = CardRecord::new(
    "Earth Rumble",
    "62730505-56f1-4043-a8b9-4fb7bc508b47",
    "Olena Richards",
    CardRules::unsupported(),
);

// TLA 175 — Earthbender Ascension
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change. Also needs a reflexive trigger tied to the quest counter placed by the landfall ability, with the later counter target chosen at that point.
pub(in crate::card::sets) static EARTHBENDER_ASCENSION: CardRecord = CardRecord::new(
    "Earthbender Ascension",
    "590a58ab-5e98-4031-8aa6-ce396dc1429f",
    "Logan Feliciano",
    CardRules::unsupported(),
);

// TLA 176 — Earthbending Lesson
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static EARTHBENDING_LESSON: CardRecord = CardRecord::new(
    "Earthbending Lesson",
    "eccd63b3-3a3a-4661-9d6e-fb8152429bdb",
    "Toni Infante",
    CardRules::unsupported(),
);

// TLA 177 — Earthen Ally
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static EARTHEN_ALLY: CardRecord = CardRecord::new(
    "Earthen Ally",
    "d7a0f2d9-efa4-4a53-a24b-36694457bd1c",
    "Boell Oyino",
    CardRules::unsupported(),
);

// TLA 178 — Elemental Teachings
// Audit: unsupported — Needs a search selection requiring pairwise different card names; individual-card predicates do not constrain relationships within a chosen search group.
pub(in crate::card::sets) static ELEMENTAL_TEACHINGS: CardRecord = CardRecord::new(
    "Elemental Teachings",
    "cac8ac35-6860-4839-ab85-93d409206c08",
    "Yoshioka",
    CardRules::unsupported(),
);

// TLA 179 — Flopsie, Bumi's Buddy
pub(in crate::card::sets) static FLOPSIE_BUMI_S_BUDDY: CardRecord = CardRecord::new(
    "Flopsie, Bumi's Buddy",
    "082f4abc-c09f-42cf-977f-060875dfd411",
    "Alexandr Leskinen",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Ape", "Goat"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Flopsie enters, put a +1/+1 counter on each creature you \
                 control.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Each creature you control with power 4 or greater can't be \
                 blocked by more than one creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::PowerAtLeast(4),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::MaximumBlockers(1),
                    )),
                },
            ),
        ]),
);

// TLA 180 — Foggy Swamp Vinebender
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static FOGGY_SWAMP_VINEBENDER: CardRecord = CardRecord::new(
    "Foggy Swamp Vinebender",
    "78a75317-94f7-47a4-b4da-5a027fa73248",
    "Maël Ollivier-Henry",
    CardRules::unsupported(),
);

// TLA 181 — Great Divide Guide
pub(in crate::card::sets) static GREAT_DIVIDE_GUIDE: CardRecord = CardRecord::new(
    "Great Divide Guide",
    "cc3063ec-5ea6-46c1-8331-c740cbaf6c76",
    "Song Qijin",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Scout", "Ally"], 2, 3)
        .with_abilities(&[AbilityDef::static_ability(
            "Each land and Ally you control has \"{T}: Add one mana of any \
             color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::tap_for_mana(
                    "{T}: Add one mana of any color.",
                    AddManaEffectDef::any_color(),
                )),
            },
        )]),
);

// TLA 182 — Haru, Hidden Talent
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static HARU_HIDDEN_TALENT: CardRecord = CardRecord::new(
    "Haru, Hidden Talent",
    "9657a0c0-2c26-4331-8e62-f032265a01af",
    "Mitori",
    CardRules::unsupported(),
);

// TLA 183 — Invasion Tactics
// Audit: unsupported — Needs combat damage triggers grouped once per damaged player for a filtered set of controlled Allies; current damage batching groups only unrestricted damage-source matchers.
pub(in crate::card::sets) static INVASION_TACTICS: CardRecord = CardRecord::new(
    "Invasion Tactics",
    "27c6330d-49f6-4707-b4d4-d1411fa422eb",
    "Eduardo Francisco",
    CardRules::unsupported(),
);

// TLA 184 — Kyoshi Island Plaza
pub(in crate::card::sets) static KYOSHI_ISLAND_PLAZA: CardRecord = CardRecord::new(
    "Kyoshi Island Plaza",
    "9c25ea98-9823-4487-bf6b-ee29ce5a4ed8",
    "Eilene Cherie",
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Shrine"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Kyoshi Island Plaza enters, search your library for up \
                 to X basic land cards, where X is the number of Shrines you \
                 control. Put those cards onto the battlefield tapped, then \
                 shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::triggered(
                "Whenever another Shrine you control enters, search your \
                 library for a basic land card, put it onto the battlefield \
                 tapped, then shuffle.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// TLA 185 — Leaves from the Vine
pub(in crate::card::sets) static LEAVES_FROM_THE_VINE: CardRecord = CardRecord::new(
    "Leaves from the Vine",
    "b18995c2-efe3-46ca-8204-e2dc0e42f6e3",
    "Ittoku",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Mill three cards, then create a Food token. (It's an \
                 artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
                 life.\")",
                EffectDef::Sequence(&[
                    EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                ]),
            ),
            abilities::saga_chapter_with_targets(
                2,
                "II — Put a +1/+1 counter on each of up to two target \
                 creatures you control.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    2,
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::saga_chapter(
                3,
                "III — Draw a card if there's a creature or Lesson card in \
                 your graveyard.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &abilities::draw_cards(ValueDef::Constant(1)),
                },
            ),
        ]),
);

// TLA 186 — The Legend of Kyoshi // Avatar Kyoshi
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static THE_LEGEND_OF_KYOSHI: CardRecord = CardRecord::new(
    "The Legend of Kyoshi // Avatar Kyoshi",
    "4887ce64-7c98-4bd7-95db-0c43ab71cc6e",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// TLA 187 — Origin of Metalbending
pub(in crate::card::sets) static ORIGIN_OF_METALBENDING: CardRecord = CardRecord::new(
    "Origin of Metalbending",
    "25749f9a-260d-4bdc-bd28-429c12faa4a5",
    "Pauline Voss",
    CardRules::new_instant(mana_cost!("{1}{G}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose one —",
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target artifact or enchantment.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Put a +1/+1 counter on target creature you control. It gains \
                     indestructible until end of turn. (Damage and effects that \
                     say \"destroy\" don't destroy it.)",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                ),
            ],
        )]),
);

// TLA 188 — Ostrich-Horse
pub(in crate::card::sets) static OSTRICH_HORSE: CardRecord = CardRecord::new(
    "Ostrich-Horse",
    "5ca3fd45-9301-44ef-afc0-4d7999d66d36",
    "Pablo Rivera",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bird", "Horse"], 3, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill three cards. You may put a \
             land card from among them into your hand. If you don't, put a \
             +1/+1 counter on this creature. (To mill three cards, put the \
             top three cards of your library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    binding: crate::Binding!("milled"),
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                },
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountObjects(&ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                        }),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::WithZoneMoveResult {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                            binding: crate::Binding!("returned"),
                            then: &EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ValueComparison(
                                    &ValueComparisonDef {
                                        left: ValueDef::CountObjects(
                                            &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                crate::Binding!("returned"),
                                            ),
                                        ),
                                        comparison: ComparisonDef::Equal,
                                        right: ValueDef::Constant(0),
                                    },
                                ),
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::Source,
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                        },
                    }),
                    otherwise: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
    ]),
);

// TLA 189 — Pillar Launch
pub(in crate::card::sets) static PILLAR_LAUNCH: CardRecord = CardRecord::new(
    "Pillar Launch",
    "f177e9cc-0c41-417d-9215-5ff23a1a0657",
    "Jo Cordisco",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains reach until end of turn. \
         Untap it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&abilities::reach()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// TLA 190 — Raucous Audience
pub(in crate::card::sets) static RAUCOUS_AUDIENCE: CardRecord = CardRecord::new(
    "Raucous Audience",
    "ccd03ec3-0fae-48b2-9f04-9c57d2f94267",
    "ikeda_cpt",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Citizen"], 2, 1).with_abilities(&[
        abilities::tap_for_mana(
            "{T}: Add {G}. If you control a creature with power 4 or \
             greater, add {G}{G} instead.",
            AddManaEffectDef::one(ManaColor::Green).with_variable_amount(
                ValueDef::IfMatchingObjectCount(&CountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                    then: ValueDef::Constant(2),
                    otherwise: ValueDef::Constant(1),
                }),
            ),
        ),
    ]),
);

// TLA 191 — Rebellious Captives
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static REBELLIOUS_CAPTIVES: CardRecord = CardRecord::new(
    "Rebellious Captives",
    "e64a4a8b-323c-4c8b-92af-1a3295799e65",
    "Ittoku",
    CardRules::unsupported(),
);

// TLA 192 — Rockalanche
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static ROCKALANCHE: CardRecord = CardRecord::new(
    "Rockalanche",
    "52b213d3-6f68-43e6-91e9-435d8fe1f34c",
    "Yuu Fujiki",
    CardRules::unsupported(),
);

// TLA 193 — Rocky Rebuke
pub(in crate::card::sets) static ROCKY_REBUKE: CardRecord = CardRecord::new(
    "Rocky Rebuke",
    "52fd910f-6d42-41f5-a4be-f375aa254ea2",
    "Hokyoung Kim",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power \
         to target creature an opponent controls.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::damage_from(
            ObjectRefDef::Target(TargetIndex::PRIMARY),
            EffectRecipientDef::Target(TargetIndex(1)),
            ValueDef::TargetPower(TargetIndex::PRIMARY),
        ),
    )]),
);

// TLA 194 — Saber-Tooth Moose-Lion
pub(in crate::card::sets) static SABER_TOOTH_MOOSE_LION: CardRecord = CardRecord::new(
    "Saber-Tooth Moose-Lion",
    "b15f302d-c451-4e79-a5af-75ee3ba4cadf",
    "Shiren",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elk", "Cat"], 7, 7).with_abilities(&[
        abilities::reach(),
        abilities::typecycling!(
            "Forestcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest"))
        ),
    ]),
);

// TLA 195 — Seismic Sense
pub(in crate::card::sets) static SEISMIC_SENSE: CardRecord = CardRecord::new(
    "Seismic Sense",
    "a7b93c2a-c478-4734-8aca-03f3e9c11b20",
    "Jo Cordisco",
    CardRules::new_sorcery(mana_cost!("{G}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell(
            "Look at the top X cards of your library, where X is the \
             number of lands you control. You may reveal a creature or \
             land card from among them and put it into your hand. Put the \
             rest on the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        randomized: crate::Binding!("random_bottom"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "random_bottom"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        )]),
);

// TLA 196 — Shared Roots
pub(in crate::card::sets) static SHARED_ROOTS: CardRecord = CardRecord::new(
    "Shared Roots",
    "e7847ba5-e85e-417f-96c0-aef2e6f83994",
    "Alfven Ato",
    CardRules::new_sorcery(mana_cost!("{1}{G}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell(
            "Search your library for a basic land card, put it onto the \
             battlefield tapped, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        )]),
);

// TLA 197 — Sparring Dummy
pub(in crate::card::sets) static SPARRING_DUMMY: CardRecord = CardRecord::new(
    "Sparring Dummy",
    "520f5a03-ff5a-43ed-8f15-cf24596d60cd",
    "Gemi",
    CardRules::new_artifact_creature(mana_cost!("{1}{G}"), &["Scarecrow"], 1, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{T}: Mill a card. You may put a land card milled this way \
             into your hand. You gain 2 life if a Lesson card is milled \
             this way. (To mill a card, put the top card of your library \
             into your graveyard.)",
            &[CostDef::TapSource],
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    binding: crate::Binding!("milled"),
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Matching {
                        objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                    },
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountObjects(&ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Subtype(
                                SubtypeDef::Literal("Lesson"),
                            )),
                        }),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                },
            ]),
        ),
    ]),
);

// TLA 198 — Toph, the Blind Bandit
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static TOPH_THE_BLIND_BANDIT: CardRecord = CardRecord::new(
    "Toph, the Blind Bandit",
    "ff68fa7b-8065-407b-a8b4-bfbb14f1c99c",
    "Yueko",
    CardRules::unsupported(),
);

// TLA 199 — True Ancestry
pub(in crate::card::sets) static TRUE_ANCESTRY: CardRecord = CardRecord::new(
    "True Ancestry",
    "8c55b333-dc8b-4332-895b-eec5eb45543f",
    "Chibi",
    CardRules::new_sorcery(mana_cost!("{1}{G}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Return up to one target permanent card from your graveyard to \
             your hand.\nCreate a Clue token. (It's an artifact with \
             \"{2}, Sacrifice this token: Draw a card.\")",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        )]),
);

// TLA 200 — Turtle-Duck
pub(in crate::card::sets) static TURTLE_DUCK: CardRecord = CardRecord::new(
    "Turtle-Duck",
    "aefcd734-3916-4c77-9d98-3ea2c2795658",
    "Sylvain Sarrailh",
    CardRules::new_creature(mana_cost!("{G}"), &["Turtle", "Bird"], 0, 4).with_abilities(&[
        AbilityDef::activated(
            "{3}: Until end of turn, this creature has base power 4 and \
             gains trample.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                        PowerToughnessOperationDef::SetBasePower(ValueDef::Constant(4)),
                    )),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TLA 201 — Unlucky Cabbage Merchant
pub(in crate::card::sets) static UNLUCKY_CABBAGE_MERCHANT: CardRecord = CardRecord::new(
    "Unlucky Cabbage Merchant",
    "0a838009-b115-4497-b35d-682e41ead7db",
    "Thomas Chamberlain-Keen",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Citizen"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Food token. (It's an \
             artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
             life.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(FOOD_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice a Food, you may search your library \
             for a basic land card and put it onto the battlefield tapped. \
             If you search your library this way, put this creature on the \
             bottom of its owner's library, then shuffle.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Food")),
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: false,
                        enters_tapped: true,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Source,
                        ZoneKind::Library,
                        ZonePlacement::Bottom,
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                ]),
            },
        ),
    ]),
);

// TLA 202 — Walltop Sentries
pub(in crate::card::sets) static WALLTOP_SENTRIES: CardRecord = CardRecord::new(
    "Walltop Sentries",
    "e55f1c63-58f6-4e2b-aaeb-f2d5faca47a2",
    "Boell Oyino",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Soldier", "Ally"], 2, 3)
        .with_abilities(&[
            abilities::reach(),
            abilities::deathtouch(),
            AbilityDef::triggered_if(
                "When this creature dies, if there's a Lesson card in your \
                 graveyard, you gain 2 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// TLA 203 — Aang, at the Crossroads // Aang, Destined Savior
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static AANG_AT_THE_CROSSROADS: CardRecord = CardRecord::new(
    "Aang, at the Crossroads // Aang, Destined Savior",
    "fea89ca0-8070-4f28-9851-994314f9d248",
    "Evan Shipard",
    CardRules::unsupported(),
);

// TLA 204 — Aang, Swift Savior // Aang and La, Ocean's Fury
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs; airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled.
pub(in crate::card::sets) static AANG_SWIFT_SAVIOR: CardRecord = CardRecord::new(
    "Aang, Swift Savior // Aang and La, Ocean's Fury",
    "82866a0e-485a-4f7e-8c49-f7d9ff3f4ad4",
    "Tetsuko",
    CardRules::unsupported(),
);

// TLA 205 — Abandon Attachments
pub(in crate::card::sets) static ABANDON_ATTACHMENTS: CardRecord = CardRecord::new(
    "Abandon Attachments",
    "74ca45a4-97ab-4255-9129-884e8b42b984",
    "Shahab Alizadeh",
    CardRules::new_instant(mana_cost!("{1}{U/R}"))
        .with_subtypes(&["Lesson"])
        .with_abilities(&[AbilityDef::spell(
            "You may discard a card. If you do, draw two cards.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::discard(ObjectPredicateDef::Any)],
                &abilities::draw_cards(ValueDef::Constant(2)),
            )),
        )]),
);

// TLA 206 — Air Nomad Legacy
pub(in crate::card::sets) static AIR_NOMAD_LEGACY: CardRecord = CardRecord::new(
    "Air Nomad Legacy",
    "10874416-fa4c-4aa3-b885-4351d011d208",
    "AKAGI",
    CardRules::new_enchantment(mana_cost!("{W}{U}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a Clue token. (It's an \
             artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::static_ability(
            "Creatures you control with flying get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// TLA 207 — Avatar Aang // Aang, Master of Elements
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs; earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change; airbend exile with an owner-specific {2} alternative cast permission lasting while that card remains exiled; firebending mana whose individual units survive combat step boundaries and expire at the end of that combat. Also needs per-turn history of the four named actions and a colored cost reduction that can consume generic costs.
pub(in crate::card::sets) static AVATAR_AANG: CardRecord = CardRecord::new(
    "Avatar Aang // Aang, Master of Elements",
    "fe29e909-50e9-4f04-b1a3-2cc5d7e3efe8",
    "Fahmi Fauzi",
    CardRules::unsupported(),
);

// TLA 208 — Azula, Cunning Usurper
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static AZULA_CUNNING_USURPER: CardRecord = CardRecord::new(
    "Azula, Cunning Usurper",
    "daf30e1c-436d-4f23-b1d2-570619a4b7f5",
    "Evyn Fong",
    CardRules::unsupported(),
);

// TLA 209 — Beifong's Bounty Hunters
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static BEIFONG_S_BOUNTY_HUNTERS: CardRecord = CardRecord::new(
    "Beifong's Bounty Hunters",
    "4f88ec3a-44a7-4ae5-a68c-24294dabaeed",
    "Alexandr Leskinen",
    CardRules::unsupported(),
);

// TLA 210 — Bitter Work
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static BITTER_WORK: CardRecord = CardRecord::new(
    "Bitter Work",
    "3e8de6a9-9859-4ed2-8ece-ef80a7209be9",
    "Bun Toujo",
    CardRules::unsupported(),
);

// TLA 211 — Bumi, Unleashed
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change. Also needs an additional combat with an attack restriction specific to that inserted combat.
pub(in crate::card::sets) static BUMI_UNLEASHED: CardRecord = CardRecord::new(
    "Bumi, Unleashed",
    "b20a5185-7e31-4fc9-be11-9423bfc389bf",
    "Toni Infante",
    CardRules::unsupported(),
);

// TLA 212 — Cat-Owl
pub(in crate::card::sets) static CAT_OWL: CardRecord = CardRecord::new(
    "Cat-Owl",
    "d7df3391-dbe5-4eb4-85c1-037c5f3ea971",
    "Thomas Chamberlain-Keen",
    CardRules::new_creature(mana_cost!("{3}{W/U}"), &["Cat", "Bird"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, untap target artifact or \
             creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// TLA 213 — Cruel Administrator
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static CRUEL_ADMINISTRATOR: CardRecord = CardRecord::new(
    "Cruel Administrator",
    "9d70707b-2e25-40b1-80a8-9a5492bca7e2",
    "Norikatsu Miyoshi",
    CardRules::unsupported(),
);

// TLA 214 — Dai Li Agents
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static DAI_LI_AGENTS: CardRecord = CardRecord::new(
    "Dai Li Agents",
    "9ddb8715-341b-400a-b6eb-f3f90518157c",
    "Eduardo Francisco",
    CardRules::unsupported(),
);

// TLA 215 — Dragonfly Swarm
pub(in crate::card::sets) static DRAGONFLY_SWARM: CardRecord = CardRecord::new(
    "Dragonfly Swarm",
    "16be5cc4-b15a-4c5e-8a6e-05c87b519127",
    "John Di Giovanni",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Dragon", "Insect"], 0, 3).with_abilities(
        &[
            abilities::flying(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{1}"))], "Ward {1}"),
            AbilityDef::static_ability(
                "This creature's power is equal to the number of noncreature, \
                 nonland cards in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                    )),
                },
            ),
            AbilityDef::triggered_if(
                "When this creature dies, if there's a Lesson card in your \
                 graveyard, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ],
    ),
);

// TLA 216 — Earth Kingdom Soldier
pub(in crate::card::sets) static EARTH_KINGDOM_SOLDIER: CardRecord = CardRecord::new(
    "Earth Kingdom Soldier",
    "2d543d35-945a-4ffa-beb7-7c5d4f894f79",
    "Rafater",
    CardRules::new_creature(mana_cost!("{4}{G/W}"), &["Human", "Soldier"], 3, 4).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on each of up \
             to two target creatures you control.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                2,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TLA 217 — Earth King's Lieutenant
pub(in crate::card::sets) static EARTH_KING_S_LIEUTENANT: CardRecord = CardRecord::new(
    "Earth King's Lieutenant",
    "4533d155-5c56-41a5-9d76-2d1414ac47c9",
    "Nathaniel Himawan",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Human", "Soldier", "Ally"], 1, 1)
        .with_abilities(&[
            abilities::trample(),
            abilities::enters_trigger(
                "When this creature enters, put a +1/+1 counter on each other \
                 Ally creature you control.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever another Ally you control enters, put a +1/+1 counter \
                 on this creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// TLA 218 — Earth Rumble Wrestlers
// Audit: unsupported — Needs player land-entry history covering entries by effects and lands that subsequently leave or change controllers; LandsPlayedThisTurn counts only land plays.
pub(in crate::card::sets) static EARTH_RUMBLE_WRESTLERS: CardRecord = CardRecord::new(
    "Earth Rumble Wrestlers",
    "74fca0b8-12e1-405f-b999-160d65fe0ade",
    "Thomas Chamberlain-Keen",
    CardRules::unsupported(),
);

// TLA 219 — Earth Village Ruffians
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static EARTH_VILLAGE_RUFFIANS: CardRecord = CardRecord::new(
    "Earth Village Ruffians",
    "56d4e0b7-232f-4953-84a3-e6f17781aa45",
    "Dom Lay",
    CardRules::unsupported(),
);

// TLA 220 — Fire Lord Azula
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIRE_LORD_AZULA: CardRecord = CardRecord::new(
    "Fire Lord Azula",
    "bc6146bf-f0c6-4557-af6a-74c643d5fc01",
    "Fahmi Fauzi",
    CardRules::unsupported(),
);

// TLA 221 — Fire Lord Zuko
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIRE_LORD_ZUKO: CardRecord = CardRecord::new(
    "Fire Lord Zuko",
    "e62d3bcc-7bb4-42be-90a9-caf3c1caa29d",
    "Jo Cordisco",
    CardRules::unsupported(),
);

// TLA 222 — Foggy Swamp Spirit Keeper
pub(in crate::card::sets) static FOGGY_SWAMP_SPIRIT_KEEPER: CardRecord = CardRecord::new(
    "Foggy Swamp Spirit Keeper",
    "cd624be9-e960-4485-a8a9-2d5f2dd51777",
    "Airi Yoshihisa",
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Human", "Druid", "Ally"], 2, 4)
        .with_abilities(&[
            abilities::lifelink(),
            AbilityDef::triggered(
                "Whenever you draw your second card each turn, create a 1/1 \
                 colorless Spirit creature token with \"This token can't block \
                 or be blocked by non-Spirit creatures.\"",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(
                    PlayerRelation::You,
                    2,
                )),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Spirit"], &[], 1, 1).with_abilities(&[
                        AbilityDef::static_ability(
                            "This token can't block or be blocked by non-Spirit creatures.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::Composite(&[
                                    AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spirit")),
                                    )),
                                    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                                            SubtypeDef::Literal("Spirit"),
                                        )),
                                    )),
                                ]),
                            },
                        ),
                    ]),
                ))),
            ),
        ]),
);

// TLA 223 — Guru Pathik
pub(in crate::card::sets) static GURU_PATHIK: CardRecord = CardRecord::new(
    "Guru Pathik",
    "224a2241-0dce-4008-8d9c-86db40ec5f8b",
    "Dee Nguyen",
    CardRules::new_creature(
        mana_cost!("{2}{G/U}{G/U}"),
        &["Human", "Monk", "Ally"],
        2,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::enters_trigger(
            "When Guru Pathik enters, look at the top five cards of your \
             library. You may reveal a Lesson, Saga, or Shrine card from \
             among them and put it into your hand. Put the rest on the \
             bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                ]),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        randomized: crate::Binding!("random_bottom"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "random_bottom"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        ),
        AbilityDef::triggered_with_targets(
            "Whenever you cast a Lesson, Saga, or Shrine spell, put a \
             +1/+1 counter on another target creature you control.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TLA 224 — Hama, the Bloodbender
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static HAMA_THE_BLOODBENDER: CardRecord = CardRecord::new(
    "Hama, the Bloodbender",
    "6fa1197d-7b19-4d86-81e2-5c87de87757b",
    "Le Vuong",
    CardRules::unsupported(),
);

// TLA 225 — Hei Bai, Spirit of Balance
// Audit: unsupported — Needs one effect transferring the departing source's complete last-known counter inventory to another creature, including every arbitrary named counter.
pub(in crate::card::sets) static HEI_BAI_SPIRIT_OF_BALANCE: CardRecord = CardRecord::new(
    "Hei Bai, Spirit of Balance",
    "060d24e0-1567-41f1-ae8c-2d3b1834df3c",
    "Tyler Smith",
    CardRules::unsupported(),
);

// TLA 226 — Hermitic Herbalist
pub(in crate::card::sets) static HERMITIC_HERBALIST: CardRecord = CardRecord::new(
    "Hermitic Herbalist",
    "5d0ee441-a057-41b5-abb8-dc86864ac248",
    "Kozato",
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Human", "Druid", "Ally"], 2, 3)
        .with_abilities(&[
            abilities::tap_for_mana(
                "{T}: Add one mana of any color.",
                AddManaEffectDef::any_color(),
            ),
            abilities::tap_for_mana(
                "{T}: Add two mana in any combination of colors. Spend this \
                 mana only to cast Lesson spells.",
                AddManaEffectDef::combination(
                    &[
                        ManaColor::White,
                        ManaColor::Blue,
                        ManaColor::Black,
                        ManaColor::Red,
                        ManaColor::Green,
                    ],
                    2,
                )
                .with_restrictions(&[ManaRestrictionDef::CastSpell(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                )]),
            ),
        ]),
);

// TLA 227 — Iroh, Grand Lotus
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static IROH_GRAND_LOTUS: CardRecord = CardRecord::new(
    "Iroh, Grand Lotus",
    "879b73d3-4552-4fdc-baee-c4d097ae9a4f",
    "Fahmi Fauzi",
    CardRules::unsupported(),
);

// TLA 228 — Iroh, Tea Master
// Audit: unsupported — Needs a reflexive optional-control-change trigger after the gift succeeds, retaining the gifted object and acting even when Iroh leaves; current optional-effect listeners require a battlefield source.
pub(in crate::card::sets) static IROH_TEA_MASTER: CardRecord = CardRecord::new(
    "Iroh, Tea Master",
    "d1f5e10d-0a89-4129-9717-a921f20d3616",
    "Brian Yuen",
    CardRules::unsupported(),
);

// TLA 229 — Jet, Freedom Fighter
pub(in crate::card::sets) static JET_FREEDOM_FIGHTER: CardRecord = CardRecord::new(
    "Jet, Freedom Fighter",
    "9202c044-15e4-4218-a94d-16287ba19d69",
    "Fahmi Fauzi",
    CardRules::new_creature(
        mana_cost!("{2}{R/W}{R/W}{R/W}"),
        &["Human", "Rebel", "Ally"],
        3,
        1,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When Jet enters, he deals damage equal to the number of \
             creatures you control to target creature an opponent \
             controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ),
        AbilityDef::triggered_with_targets(
            "When Jet dies, put a +1/+1 counter on each of up to two \
             target creatures.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TLA 230 — Katara, the Fearless
// Audit: unsupported — Needs general trigger duplication filtered by the triggering ability's Ally source, across all trigger event kinds; current doubling is limited to specific entry events.
pub(in crate::card::sets) static KATARA_THE_FEARLESS: CardRecord = CardRecord::new(
    "Katara, the Fearless",
    "b0a18f8b-7364-4375-b2e1-e2f15978517f",
    "Hisashi Momose",
    CardRules::unsupported(),
);

// TLA 231 — Katara, Water Tribe's Hope
// Audit: unsupported — Needs waterbend payment combining generic mana with tapping untapped artifacts and creatures, including creatures unable to pay their own tap costs.
pub(in crate::card::sets) static KATARA_WATER_TRIBE_S_HOPE: CardRecord = CardRecord::new(
    "Katara, Water Tribe's Hope",
    "9ec03308-59a2-417a-938f-bdda75080e43",
    "Toraji",
    CardRules::unsupported(),
);

// TLA 232 — The Lion-Turtle
pub(in crate::card::sets) static THE_LION_TURTLE: CardRecord = CardRecord::new(
    "The Lion-Turtle",
    "8491585c-81d0-48be-8ed0-832e42fad9c6",
    "Yuumei",
    CardRules::new_creature(mana_cost!("{1}{G}{U}"), &["Elder", "Cat", "Turtle"], 3, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            abilities::vigilance(),
            abilities::enters_trigger(
                "When The Lion-Turtle enters, you gain 3 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::static_ability(
                "The Lion-Turtle can't attack or block unless there are three \
                 or more Lesson cards in your graveyard.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::Not(&TriggerConditionDef::ValueComparison(
                        &ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(3),
                        },
                    )),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                            AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                        ]),
                    },
                },
            ),
            abilities::tap_for_mana(
                "{T}: Add one mana of any color.",
                AddManaEffectDef::any_color(),
            ),
        ]),
);

// TLA 233 — Long Feng, Grand Secretariat
pub(in crate::card::sets) static LONG_FENG_GRAND_SECRETARIAT: CardRecord = CardRecord::new(
    "Long Feng, Grand Secretariat",
    "a3eb92fe-bc59-4472-9028-f368bd015609",
    "Robin Har",
    CardRules::new_creature(mana_cost!("{1}{B/G}{B/G}"), &["Human", "Advisor"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever another creature you control or a land you control \
             is put into a graveyard from the battlefield, put a +1/+1 \
             counter on target creature you control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// TLA 234 — Messenger Hawk
pub(in crate::card::sets) static MESSENGER_HAWK: CardRecord = CardRecord::new(
    "Messenger Hawk",
    "6a2946c0-0a40-4cdb-92c8-af2bafecca09",
    "Daniel Romanovsky",
    CardRules::new_creature(mana_cost!("{2}{U/B}"), &["Bird", "Scout"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a Clue token. (It's an \
             artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::static_ability(
            "This creature gets +2/+0 as long as you've drawn two or more \
             cards this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(2),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            },
        ),
    ]),
);

// TLA 235 — Ozai, the Phoenix King
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat. Also needs converting expiring mana to red and a continuous unspent-mana count predicate.
pub(in crate::card::sets) static OZAI_THE_PHOENIX_KING: CardRecord = CardRecord::new(
    "Ozai, the Phoenix King",
    "a98b1550-4609-4a2f-9371-4afe1cdc613e",
    "Kekai Kotaki",
    CardRules::unsupported(),
);

// TLA 236 — Platypus-Bear
pub(in crate::card::sets) static PLATYPUS_BEAR: CardRecord = CardRecord::new(
    "Platypus-Bear",
    "7ebf742a-3995-4b22-9dc7-b05f57889683",
    "Maël Ollivier-Henry",
    CardRules::new_creature(mana_cost!("{1}{G/U}"), &["Platypus", "Bear"], 2, 3).with_abilities(&[
        abilities::defender(),
        abilities::enters_trigger(
            "When this creature enters, mill two cards. (Put the top two \
             cards of your library into your graveyard.)",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::static_ability(
            "As long as there is a Lesson card in your graveyard, this \
             creature can attack as though it didn't have defender.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                },
            },
        ),
    ]),
);

// TLA 237 — Pretending Poxbearers
pub(in crate::card::sets) static PRETENDING_POXBEARERS: CardRecord = CardRecord::new(
    "Pretending Poxbearers",
    "d1af91a5-8681-4a05-910d-96f7a819bfaa",
    "Salvatorre Zee Yazzie",
    CardRules::new_creature(mana_cost!("{1}{W/B}"), &["Human", "Citizen", "Ally"], 2, 1)
        .with_abilities(&[AbilityDef::triggered(
            "When this creature dies, create a 1/1 white Ally creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
        )]),
);

// TLA 238 — Professor Zei, Anthropologist
pub(in crate::card::sets) static PROFESSOR_ZEI_ANTHROPOLOGIST: CardRecord = CardRecord::new(
    "Professor Zei, Anthropologist",
    "27baccc0-7e25-4f39-be0d-31cd98ca0dc5",
    "Pauline Voss",
    CardRules::new_creature(
        mana_cost!("{U/R}{U/R}"),
        &["Human", "Advisor", "Ally"],
        0,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::activated(
            "{T}, Discard a card: Draw a card.",
            &[
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice Professor Zei: Return target instant or \
             sorcery card from your graveyard to your hand. Activate only \
             during your turn.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )
        .with_activation_timing(ActivationTimingDef::YourTurn),
    ]),
);

// TLA 239 — Sandbender Scavengers
// Audit: unsupported — Needs a reflexive trigger after optional self-exile from the graveyard, retaining its last-known power and selecting the reanimation target at that later time.
pub(in crate::card::sets) static SANDBENDER_SCAVENGERS: CardRecord = CardRecord::new(
    "Sandbender Scavengers",
    "9bfe0f7c-6dac-41d4-a013-109445158a5e",
    "Alexander Forssberg",
    CardRules::unsupported(),
);

// TLA 240 — Sokka, Bold Boomeranger
pub(in crate::card::sets) static SOKKA_BOLD_BOOMERANGER: CardRecord = CardRecord::new(
    "Sokka, Bold Boomeranger",
    "4a1f1472-55b4-450d-8e4d-7297130a0cf3",
    "Toni Infante",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Human", "Warrior", "Ally"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Sokka enters, discard up to two cards, then draw that \
                 many cards.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 2,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::Sequence(&[
                        EffectDef::discard_cards(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("chosen")),
                        )),
                        abilities::draw_cards(ValueDef::CountObjects(&ObjectSetDef::Binding(
                            crate::Binding!("chosen"),
                        ))),
                    ]),
                }),
            ),
            AbilityDef::triggered(
                "Whenever you cast an artifact or Lesson spell, put a +1/+1 \
                 counter on Sokka.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// TLA 241 — Sokka, Lateral Strategist
pub(in crate::card::sets) static SOKKA_LATERAL_STRATEGIST: CardRecord = CardRecord::new(
    "Sokka, Lateral Strategist",
    "1326c5db-4615-495f-8e30-376243d91352",
    "Axel Sauerwald",
    CardRules::new_creature(
        mana_cost!("{1}{W/U}{W/U}"),
        &["Human", "Warrior", "Ally"],
        2,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever Sokka and at least one other creature attack, draw a \
             card.",
            TriggerEventDef::attacks_in_declaration(ObjectPredicateDef::Source, 2, None),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// TLA 242 — Sokka, Tenacious Tactician
pub(in crate::card::sets) static SOKKA_TENACIOUS_TACTICIAN: CardRecord = CardRecord::new(
    "Sokka, Tenacious Tactician",
    "f0fa5897-1da7-488f-bb19-1632e969c050",
    "Robin Har",
    CardRules::new_creature(
        mana_cost!("{1}{U}{R}{W}"),
        &["Human", "Warrior", "Ally"],
        3,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Prowess",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::static_ability(
            "Other Allies you control have menace and prowess.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::menace()),
                    AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Prowess",
                        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ])),
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(1),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    )),
                ]),
            },
        ),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, create a 1/1 white \
             Ally creature token.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
        ),
    ]),
);

// TLA 243 — Suki, Kyoshi Warrior
pub(in crate::card::sets) static SUKI_KYOSHI_WARRIOR: CardRecord = CardRecord::new(
    "Suki, Kyoshi Warrior",
    "3e311480-04f8-4b8f-938e-1cc6d06b9902",
    "Yuhong Ding",
    CardRules::new_creature(
        mana_cost!("{2}{G/W}{G/W}"),
        &["Human", "Warrior", "Ally"],
        0,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::static_ability(
            "Suki's power is equal to the number of creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            },
        ),
        AbilityDef::triggered(
            "Whenever Suki attacks, create a 1/1 white Ally creature token \
             that's tapped and attacking.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))
                    .entering_tapped()
                    .entering_attacking(),
            ),
        ),
    ]),
);

// TLA 244 — Sun Warriors
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static SUN_WARRIORS: CardRecord = CardRecord::new(
    "Sun Warriors",
    "e477d750-42d2-48c8-bc1f-8148d15d5f53",
    "Boell Oyino",
    CardRules::unsupported(),
);

// TLA 245 — Tolls of War
pub(in crate::card::sets) static TOLLS_OF_WAR: CardRecord = CardRecord::new(
    "Tolls of War",
    "d822b9ad-a787-4355-88c2-07ae2bd3a78e",
    "Jocelin Carmes",
    CardRules::new_enchantment(mana_cost!("{W}{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a Clue token. (It's an \
             artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice a permanent during your turn, create a \
             1/1 white Ally creature token. This ability triggers only \
             once each turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::Any,
                    player: PlayerRelation::You,
                },
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN))),
        )
        .triggering_at_most(1),
    ]),
);

// TLA 246 — Toph, Hardheaded Teacher
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static TOPH_HARDHEADED_TEACHER: CardRecord = CardRecord::new(
    "Toph, Hardheaded Teacher",
    "e3ba3395-39db-4330-9801-53def924f253",
    "Ruwen Liu",
    CardRules::unsupported(),
);

// TLA 247 — Toph, the First Metalbender
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static TOPH_THE_FIRST_METALBENDER: CardRecord = CardRecord::new(
    "Toph, the First Metalbender",
    "70b6670f-a9fa-4d75-b0a7-c01b5071a514",
    "Eilene Cherie",
    CardRules::unsupported(),
);

// TLA 248 — Uncle Iroh
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static UNCLE_IROH: CardRecord = CardRecord::new(
    "Uncle Iroh",
    "5c88672c-0eba-4e93-a0ec-30bbb1bb7661",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// TLA 249 — Vindictive Warden
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static VINDICTIVE_WARDEN: CardRecord = CardRecord::new(
    "Vindictive Warden",
    "0f34de05-39a9-425f-a3b4-3a9d46c917ac",
    "Jo Cordisco",
    CardRules::unsupported(),
);

// TLA 250 — Wandering Musicians
pub(in crate::card::sets) static WANDERING_MUSICIANS: CardRecord = CardRecord::new(
    "Wandering Musicians",
    "3da81dbd-dc80-47ff-a33b-270d23a24e0a",
    "Rose Benjamin",
    CardRules::new_creature(mana_cost!("{3}{R/W}"), &["Human", "Bard", "Ally"], 2, 5)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever this creature attacks, creatures you control get \
             +1/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]),
);

// TLA 251 — White Lotus Reinforcements
pub(in crate::card::sets) static WHITE_LOTUS_REINFORCEMENTS: CardRecord = CardRecord::new(
    "White Lotus Reinforcements",
    "4c7e3e2f-d7ac-4d40-ab84-cb7efe06a316",
    "Kotakan",
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Human", "Soldier", "Ally"], 2, 3)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Other Allies you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ally")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// TLA 252 — Zhao, Ruthless Admiral
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static ZHAO_RUTHLESS_ADMIRAL: CardRecord = CardRecord::new(
    "Zhao, Ruthless Admiral",
    "3fd48a57-b0bb-4177-a0f3-bd317a179cbe",
    "Yuumei",
    CardRules::unsupported(),
);

// TLA 253 — Zuko, Conflicted
// Audit: unsupported — Needs persistent per-object history of chosen modes and a first-main-phase trigger; ordinary modal effects do not exclude modes chosen on earlier turns.
pub(in crate::card::sets) static ZUKO_CONFLICTED: CardRecord = CardRecord::new(
    "Zuko, Conflicted",
    "9d555cab-ec86-4f27-bca2-7d01f79c3f46",
    "Halil Ural",
    CardRules::unsupported(),
);

// TLA 254 — Barrels of Blasting Jelly
pub(in crate::card::sets) static BARRELS_OF_BLASTING_JELLY: CardRecord = CardRecord::new(
    "Barrels of Blasting Jelly",
    "4fd83083-3779-41f0-90a0-a00e3270e4e6",
    "Salvatorre Zee Yazzie",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{1}: Add one mana of any color. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        )
        .activations_each_turn(1),
        AbilityDef::activated_with_targets(
            "{5}, {T}, Sacrifice this artifact: It deals 5 damage to \
             target creature.",
            &[
                CostDef::Mana(mana_cost!("{5}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
        ),
    ]),
);

// TLA 255 — Bender's Waterskin
// Audit: unsupported — Needs untapping during each other player's untap step as part of the turn-based action; an upkeep trigger would create an incorrect priority and timing window.
pub(in crate::card::sets) static BENDER_S_WATERSKIN: CardRecord = CardRecord::new(
    "Bender's Waterskin",
    "6d4a1712-d19e-4475-9614-a0b1af4da610",
    "Dee Nguyen",
    CardRules::unsupported(),
);

// TLA 256 — Fire Nation Warship
pub(in crate::card::sets) static FIRE_NATION_WARSHIP: CardRecord = CardRecord::new(
    "Fire Nation Warship",
    "41ecc1de-353e-4131-b735-d2ce18c9c2d5",
    "Hisashi Momose",
    CardRules::new_vehicle(mana_cost!("{3}"), 4, 4).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// TLA 257 — Kyoshi Battle Fan
pub(in crate::card::sets) static KYOSHI_BATTLE_FAN: CardRecord = CardRecord::new(
    "Kyoshi Battle Fan",
    "257e6862-96fe-4312-aff2-509d0696843a",
    "VEN",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a 1/1 white Ally creature \
                 token, then attach this Equipment to it.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(ALLY_TOKEN)).with_created_tokens(
                        CreatedTokensDef {
                            binding: crate::Binding!("allies"),
                            then: &EffectDef::Attach {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("allies"),
                                )),
                            },
                        },
                    ),
                ),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// TLA 258 — Meteor Sword
pub(in crate::card::sets) static METEOR_SWORD: CardRecord = CardRecord::new(
    "Meteor Sword",
    "68b1c746-a4b8-4a45-9f0e-3962f0ae40e3",
    "Matteo Bassini",
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, destroy target permanent.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +3/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// TLA 259 — Planetarium of Wan Shi Tong
// Audit: unsupported — Needs a shared trigger for scry or surveil completion and an optional-resolution use limit spent only when the look/cast action is taken.
pub(in crate::card::sets) static PLANETARIUM_OF_WAN_SHI_TONG: CardRecord = CardRecord::new(
    "Planetarium of Wan Shi Tong",
    "0ebaf0bf-7aa2-469d-bdbb-0fbf6741eede",
    "Robin Olausson",
    CardRules::unsupported(),
);

// TLA 260 — Trusty Boomerang
pub(in crate::card::sets) static TRUSTY_BOOMERANG: CardRecord = CardRecord::new(
    "Trusty Boomerang",
    "df99a166-6d58-4d92-9037-2fdfbaf65629",
    "Toni Infante",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has \"{1}, {T}: Tap target creature. Return \
                 Trusty Boomerang to its owner's hand.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{1}, {T}: Tap target creature. Return Trusty Boomerang to its \
                         owner's hand.",
                        &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Sequence(&[
                            EffectDef::Tap {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                            EffectDef::move_to_zone(
                                EffectRecipientDef::object(ObjectRefDef::AbilityGrantSource),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                        ]),
                    )),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// TLA 261 — The Walls of Ba Sing Se
pub(in crate::card::sets) static THE_WALLS_OF_BA_SING_SE: CardRecord = CardRecord::new(
    "The Walls of Ba Sing Se",
    "24a14ac5-0b7e-4441-9963-782a7cda3f4a",
    "Arthur Yuan",
    CardRules::new_artifact_creature(mana_cost!("{8}"), &["Wall"], 0, 30)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::defender(),
            AbilityDef::static_ability(
                "Other permanents you control have indestructible.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                },
            ),
        ]),
);

// TLA 262 — White Lotus Tile
// Audit: unsupported — Needs the maximum size of a controlled creature group sharing any one creature type, including multi-type creatures; current counts cannot maximize over creature types.
pub(in crate::card::sets) static WHITE_LOTUS_TILE: CardRecord = CardRecord::new(
    "White Lotus Tile",
    "1d70f9ec-fdc1-4219-b89b-c030d712c1fc",
    "Dee Nguyen",
    CardRules::unsupported(),
);

// TLA 263 — Abandoned Air Temple
pub(in crate::card::sets) static ABANDONED_AIR_TEMPLE: CardRecord = CardRecord::new(
    "Abandoned Air Temple",
    "9c0433f9-8f1e-4a19-a83f-a41925f1b1a9",
    "Dom Lay",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control a basic land.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::Exists(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        abilities::tap_for(ManaColor::White),
        AbilityDef::activated(
            "{3}{W}, {T}: Put a +1/+1 counter on each creature you control.",
            &[CostDef::Mana(mana_cost!("{3}{W}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TLA 264 — Agna Qel'a
pub(in crate::card::sets) static AGNA_QEL_A: CardRecord = CardRecord::new(
    "Agna Qel'a",
    "6b885829-a323-4f7d-87c9-aa4615dcbe5c",
    "Dom Lay",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control a basic land.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::Exists(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        abilities::tap_for(ManaColor::Blue),
        AbilityDef::activated(
            "{2}{U}, {T}: Draw a card, then discard a card.",
            &[CostDef::Mana(mana_cost!("{2}{U}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ]),
);

// TLA 265 — Airship Engine Room
pub(in crate::card::sets) static AIRSHIP_ENGINE_ROOM: CardRecord = CardRecord::new(
    "Airship Engine Room",
    "0b98ba34-f251-4bd0-9d75-1d4445be5cfd",
    "Andreas Rocha",
    cashable_dual_land("{T}: Add {U} or {R}.", &[ManaColor::Blue, ManaColor::Red]),
);

// TLA 266 — Ba Sing Se
// Audit: unsupported — Needs earthbend land animation and its independent delayed death-or-exile return trigger, preserving the affected object identity through ability removal and its subsequent zone change.
pub(in crate::card::sets) static BA_SING_SE: CardRecord = CardRecord::new(
    "Ba Sing Se",
    "bdf3b2be-d0cd-4a3c-a10e-82d32c12d3bd",
    "Andreas Rocha",
    CardRules::unsupported(),
);

// TLA 267 — Boiling Rock Prison
pub(in crate::card::sets) static BOILING_ROCK_PRISON: CardRecord = CardRecord::new(
    "Boiling Rock Prison",
    "1c2e2220-54d1-4180-93a0-964e3b0ba8b8",
    "Matteo Bassini",
    // Entering tapped is the price of the two colours; cashing it in later
    // is what keeps it from being a dead draw once the mana is there.
    cashable_dual_land("{T}: Add {B} or {R}.", &[ManaColor::Black, ManaColor::Red]),
);

// TLA 268 — Fire Nation Palace
// Audit: unsupported — Needs firebending mana whose individual units survive combat step boundaries and expire at the end of that combat.
pub(in crate::card::sets) static FIRE_NATION_PALACE: CardRecord = CardRecord::new(
    "Fire Nation Palace",
    "c91ac3f2-9fcd-4b41-a168-ae7f70b67d3c",
    "Awanqi (Angela Wang)",
    CardRules::unsupported(),
);

// TLA 269 — Foggy Bottom Swamp
pub(in crate::card::sets) static FOGGY_BOTTOM_SWAMP: CardRecord = CardRecord::new(
    "Foggy Bottom Swamp",
    "226af626-873d-4bd2-b889-000f8316786f",
    "Dom Lay",
    cashable_dual_land(
        "{T}: Add {B} or {G}.",
        &[ManaColor::Black, ManaColor::Green],
    ),
);

// TLA 270 — Jasmine Dragon Tea Shop
// Audit: unsupported — Needs a disjunctive mana spend restriction allowing either an Ally spell or an activated ability of an Ally source; multiple current restrictions combine conjunctively.
pub(in crate::card::sets) static JASMINE_DRAGON_TEA_SHOP: CardRecord = CardRecord::new(
    "Jasmine Dragon Tea Shop",
    "da2c83d4-a95f-47ff-a08f-694eb78d6b9b",
    "Leanna Crossan",
    CardRules::unsupported(),
);

// TLA 271 — Kyoshi Village
pub(in crate::card::sets) static KYOSHI_VILLAGE: CardRecord = CardRecord::new(
    "Kyoshi Village",
    "8d5f3008-2af8-4e81-8847-1c91f524e747",
    "Luc Courtois",
    cashable_dual_land(
        "{T}: Add {G} or {W}.",
        &[ManaColor::Green, ManaColor::White],
    ),
);

// TLA 272 — Meditation Pools
pub(in crate::card::sets) static MEDITATION_POOLS: CardRecord = CardRecord::new(
    "Meditation Pools",
    "cc078fc6-4470-473f-9e24-d2c470e09141",
    "Luc Courtois",
    cashable_dual_land("{T}: Add {G} or {U}.", &[ManaColor::Green, ManaColor::Blue]),
);

// TLA 273 — Misty Palms Oasis
pub(in crate::card::sets) static MISTY_PALMS_OASIS: CardRecord = CardRecord::new(
    "Misty Palms Oasis",
    "7cc29ddb-8f44-4493-985d-2fac35f9f34d",
    "Andreas Rocha",
    cashable_dual_land(
        "{T}: Add {W} or {B}.",
        &[ManaColor::White, ManaColor::Black],
    ),
);

// TLA 274 — North Pole Gates
pub(in crate::card::sets) static NORTH_POLE_GATES: CardRecord = CardRecord::new(
    "North Pole Gates",
    "bcf7b9da-d35d-4338-b4d9-83e47dc058ee",
    "Andreas Rocha",
    cashable_dual_land("{T}: Add {W} or {U}.", &[ManaColor::White, ManaColor::Blue]),
);

// TLA 275 — Omashu City
pub(in crate::card::sets) static OMASHU_CITY: CardRecord = CardRecord::new(
    "Omashu City",
    "c21e413b-96ed-4ded-b567-f1647074ce5f",
    "Andreas Rocha",
    cashable_dual_land("{T}: Add {R} or {G}.", &[ManaColor::Red, ManaColor::Green]),
);

// TLA 276 — Realm of Koh
pub(in crate::card::sets) static REALM_OF_KOH: CardRecord = CardRecord::new(
    "Realm of Koh",
    "061ac694-610c-479f-b038-a4ef5270d5d7",
    "Andreas Rocha",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters tapped unless you control a basic land.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::Exists(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        abilities::tap_for(ManaColor::Black),
        AbilityDef::activated(
            "{3}{B}, {T}: Create a 1/1 colorless Spirit creature token \
             with \"This token can't block or be blocked by non-Spirit \
             creatures.\"",
            &[CostDef::Mana(mana_cost!("{3}{B}")), CostDef::TapSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Spirit"], &[], 1, 1).with_abilities(&[
                    AbilityDef::static_ability(
                        "This token can't block or be blocked by non-Spirit creatures.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spirit")),
                                )),
                                AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                                        SubtypeDef::Literal("Spirit"),
                                    )),
                                )),
                            ]),
                        },
                    ),
                ]),
            ))),
        ),
    ]),
);

// TLA 277 — Rumble Arena
pub(in crate::card::sets) static RUMBLE_ARENA: CardRecord = CardRecord::new(
    "Rumble Arena",
    "3f5d2c38-663c-4f64-8cfc-c102462e82ef",
    "Le Vuong",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this land enters, scry 1. (Look at the top card of your \
             library. You may put it on the bottom.)",
            abilities::scry(ValueDef::Constant(1)),
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// TLA 278 — Secret Tunnel
// Audit: unsupported — Needs a relational target constraint requiring the two targeted creatures to share a creature type; independent target predicates cannot express that pairwise relationship.
pub(in crate::card::sets) static SECRET_TUNNEL: CardRecord = CardRecord::new(
    "Secret Tunnel",
    "2d39a0e1-6484-409c-ab05-5b276925a949",
    "Alexander Forssberg",
    CardRules::unsupported(),
);

// TLA 279 — Serpent's Pass
pub(in crate::card::sets) static SERPENT_S_PASS: CardRecord = CardRecord::new(
    "Serpent's Pass",
    "ad87bff5-9b8c-44e4-a6d3-8cc71be9640a",
    "Matteo Bassini",
    cashable_dual_land("{T}: Add {U} or {B}.", &[ManaColor::Blue, ManaColor::Black]),
);

// TLA 280 — Sun-Blessed Peak
pub(in crate::card::sets) static SUN_BLESSED_PEAK: CardRecord = CardRecord::new(
    "Sun-Blessed Peak",
    "ae04c862-095f-41d2-8aef-21036d5a0cdc",
    "Dom Lay",
    cashable_dual_land("{T}: Add {R} or {W}.", &[ManaColor::Red, ManaColor::White]),
);

// TLA 281 — White Lotus Hideout
pub(in crate::card::sets) static WHITE_LOTUS_HIDEOUT: CardRecord = CardRecord::new(
    "White Lotus Hideout",
    "24d1e22c-0d99-4ed5-94f0-fd055fd8e2be",
    "Luc Courtois",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::tap_for_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast \
             a Lesson or Shrine spell.",
            AddManaEffectDef::any_color().with_restrictions(&[ManaRestrictionDef::CastSpell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lesson")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shrine")),
                ]),
            )]),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// TLA 282 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "4069fb4a-8ee1-41ef-ab93-39a8cc58e0e5",
    "Slawek Fedorczuk",
);

// TLA 283 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "a2e22347-f0cb-4cfd-88a3-4f46a16e4946",
    "Maojin Lee",
);

// TLA 284 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "f0b234d8-d6bb-48ec-8a4d-d8a570a69c62",
    "Matteo Bassini",
);

// TLA 285 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "c44f81ca-f72f-445c-8901-3a894a2a47f9",
    "Salvatorre Zee Yazzie",
);

// TLA 286 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "a305e44f-4253-4754-b83f-1e34103d77b0",
    "Maojin Lee",
);

// TLA 287 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "8a880169-41ca-4507-83fd-306a489d31ce",
    "Salvatorre Zee Yazzie",
);

// TLA 288 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "d894c61a-4062-442e-8ead-5197c3bffd00",
    "Grady Frederick",
);

// TLA 289 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "874f0f27-81a4-4853-aaa0-2c15e07c177e",
    "John Di Giovanni",
);

// TLA 290 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "ea96ecc4-72f3-46a4-8900-6a7c5a83d4df",
    "Lorenzo Lanfranconi",
);

// TLA 291 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "05683891-cdd4-4401-b7d5-0ef17e79c699",
    "Luc Courtois",
);

// TLA 292 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "d812b6f3-83f0-44c9-8583-eb97fd8fade8",
    "Maojin Lee",
);

// TLA 293 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "dd5b22ec-847a-46bd-828e-be6059b4faec",
    "Slawek Fedorczuk",
);

// TLA 294 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "5c818f03-0e20-4ef4-bac7-11b2b7f41b5c",
    "Robin Olausson",
);

// TLA 295 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "488ca216-de75-4acc-9c67-fd37070d7e92",
    "Maojin Lee",
);

// TLA 296 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "7ca91ba8-c08d-4239-b9f2-d6c960466456",
    "Slawek Fedorczuk",
);

// TLA 297 — Fated Firepower (alternate printing)
const FATED_FIREPOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FATED_FIREPOWER,
    1,
    "63968c25-23fc-4257-863a-ad42537130d1",
    "Claudiu-Antoniu Magherusan",
);

// TLA 298 — Aang, Swift Savior // Aang and La, Ocean's Fury (alternate printing)
const AANG_SWIFT_SAVIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AANG_SWIFT_SAVIOR,
    1,
    "de89fec4-f5c8-4513-8504-ac9bafb44054",
    "Claudiu-Antoniu Magherusan",
);

// TLA 299 — Fire Nation Attacks (alternate printing)
const FIRE_NATION_ATTACKS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIRE_NATION_ATTACKS,
    1,
    "ed94f384-ebd2-4c30-a9bb-9cf793d9a810",
    "Claudiu-Antoniu Magherusan",
);

// TLA 300 — Crashing Wave (alternate printing)
const CRASHING_WAVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRASHING_WAVE,
    1,
    "a4cee0c6-8a29-4d08-9b24-c12c45c2309d",
    "Claudiu-Antoniu Magherusan",
);

// TLA 301 — Combustion Technique (alternate printing)
const COMBUSTION_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COMBUSTION_TECHNIQUE,
    1,
    "17d97052-e153-42d8-beb6-6d85815c9b59",
    "Toni Infante",
);

// TLA 302 — Zuko, Conflicted (alternate printing)
const ZUKO_CONFLICTED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZUKO_CONFLICTED,
    1,
    "52f92b27-6dfe-466f-b68b-f5dc6befa271",
    "Toni Infante",
);

// TLA 303 — Azula, Cunning Usurper (alternate printing)
const AZULA_CUNNING_USURPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AZULA_CUNNING_USURPER,
    1,
    "5b3352fd-f100-44c2-9e63-83be172317e9",
    "Toni Infante",
);

// TLA 304 — Aang, at the Crossroads // Aang, Destined Savior (alternate printing)
const AANG_AT_THE_CROSSROADS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AANG_AT_THE_CROSSROADS,
    1,
    "ea886239-c58f-4291-bd32-ed306ae1d6de",
    "Toni Infante",
);

// TLA 305 — Katara, the Fearless (alternate printing)
const KATARA_THE_FEARLESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KATARA_THE_FEARLESS,
    1,
    "397ec95c-0af5-4ed3-93a3-39dbc7a74a7b",
    "Toni Infante",
);

// TLA 306 — Dai Li Agents (alternate printing)
const DAI_LI_AGENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAI_LI_AGENTS,
    1,
    "82822ac1-9288-4bac-8afd-83563e693731",
    "Toni Infante",
);

// TLA 307 — Earthbender Ascension (alternate printing)
const EARTHBENDER_ASCENSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTHBENDER_ASCENSION,
    1,
    "826b4f26-419a-4d74-8c70-275555092095",
    "Dominik Mayer",
);

// TLA 308 — Avatar Aang // Aang, Master of Elements (alternate printing)
const AVATAR_AANG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVATAR_AANG,
    1,
    "257928ba-27ae-4a11-ae41-76dfcd626ed4",
    "Dominik Mayer",
);

// TLA 309 — Sozin's Comet (alternate printing)
const SOZIN_S_COMET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOZIN_S_COMET,
    1,
    "673f6692-fda2-4a79-b7a7-5b02be175cd9",
    "Dominik Mayer",
);

// TLA 310 — Waterbender Ascension (alternate printing)
const WATERBENDER_ASCENSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WATERBENDER_ASCENSION,
    1,
    "e5f2648f-990d-44d7-a7bf-0db99bdc3015",
    "Dominik Mayer",
);

// TLA 311 — Ozai, the Phoenix King (alternate printing)
const OZAI_THE_PHOENIX_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OZAI_THE_PHOENIX_KING,
    1,
    "5c5e8aa5-91d7-49d2-863a-b9db836436fd",
    "Dominik Mayer",
);

// TLA 312 — Firebender Ascension (alternate printing)
const FIREBENDER_ASCENSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIREBENDER_ASCENSION,
    1,
    "05309d0b-302d-4a9e-b147-1b504ecf365e",
    "Dominik Mayer",
);

// TLA 313 — Fire Lord Azula (alternate printing)
const FIRE_LORD_AZULA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIRE_LORD_AZULA,
    1,
    "281b8979-aefc-4fe7-a130-3638cdc0b1dd",
    "Dominik Mayer",
);

// TLA 314 — The Last Agni Kai (alternate printing)
const THE_LAST_AGNI_KAI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_AGNI_KAI,
    1,
    "51af6d52-bda7-40be-8a83-c19916320df5",
    "Dominik Mayer",
);

// TLA 315 — Fire Lord Zuko (alternate printing)
const FIRE_LORD_ZUKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIRE_LORD_ZUKO,
    1,
    "cdcf267f-53b4-406e-8522-14e3e73dffa9",
    "Dominik Mayer",
);

// TLA 316 — Appa, Steadfast Guardian (alternate printing)
const APPA_STEADFAST_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &APPA_STEADFAST_GUARDIAN,
    1,
    "4d822fa1-70f9-4a63-841d-31c94e1e3dd4",
    "Ilse Gort",
);

// TLA 317 — Momo, Friendly Flier (alternate printing)
const MOMO_FRIENDLY_FLIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOMO_FRIENDLY_FLIER,
    1,
    "ad9849d4-6608-43e4-8fac-1c16cfa38a9f",
    "Filip Burburan",
);

// TLA 318 — Tiger-Seal (alternate printing)
const TIGER_SEAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TIGER_SEAL,
    1,
    "d47fd63e-ddf0-4d98-8883-9ac883c20691",
    "Andrea Piparo",
);

// TLA 319 — The Unagi of Kyoshi Island (alternate printing)
const THE_UNAGI_OF_KYOSHI_ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_UNAGI_OF_KYOSHI_ISLAND,
    1,
    "66387ea1-af31-4332-bacc-c32fe3260f4b",
    "Antonio José Manzanedo",
);

// TLA 320 — Wan Shi Tong, Librarian (alternate printing)
const WAN_SHI_TONG_LIBRARIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WAN_SHI_TONG_LIBRARIAN,
    1,
    "d4bf05c4-a924-4006-835e-42f8468ba869",
    "Andrea Piparo",
);

// TLA 321 — The Fire Nation Drill (alternate printing)
const THE_FIRE_NATION_DRILL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_FIRE_NATION_DRILL,
    1,
    "aaf7eb99-b60d-44d7-baf5-30508cc8d6b3",
    "Ben Hill",
);

// TLA 322 — Koh, the Face Stealer (alternate printing)
const KOH_THE_FACE_STEALER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOH_THE_FACE_STEALER,
    1,
    "b3bf2ebe-336a-4b9a-9134-4e3efbcafdd2",
    "Antonio José Manzanedo",
);

// TLA 323 — Phoenix Fleet Airship (alternate printing)
const PHOENIX_FLEET_AIRSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PHOENIX_FLEET_AIRSHIP,
    1,
    "bf9085bd-0ee0-4d8a-8deb-686a87b28c7a",
    "Ben Hill",
);

// TLA 324 — Raven Eagle (alternate printing)
const RAVEN_EAGLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAVEN_EAGLE,
    1,
    "bc9b17de-ddb7-4f4c-826d-2f8e0678b680",
    "Vincent Coviello",
);

// TLA 325 — Ran and Shaw (alternate printing)
const RAN_AND_SHAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAN_AND_SHAW,
    1,
    "b9b12da4-7c5c-4dce-932d-0683bf04df7d",
    "Antonio José Manzanedo",
);

// TLA 326 — Badgermole Cub (alternate printing)
const BADGERMOLE_CUB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BADGERMOLE_CUB,
    1,
    "be16c053-99e1-4921-8530-5135c989149d",
    "Filip Burburan",
);

// TLA 327 — Diligent Zookeeper (alternate printing)
const DILIGENT_ZOOKEEPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DILIGENT_ZOOKEEPER,
    1,
    "5f7e8f5d-c9d6-4f25-af92-f1a6b815f0c1",
    "Andrea Piparo",
);

// TLA 328 — The Lion-Turtle (alternate printing)
const THE_LION_TURTLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LION_TURTLE,
    1,
    "8da5dab3-4505-40d9-8d81-f653cd77e491",
    "Filip Burburan",
);

// TLA 329 — The Walls of Ba Sing Se (alternate printing)
const THE_WALLS_OF_BA_SING_SE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_WALLS_OF_BA_SING_SE,
    1,
    "086b6003-6d90-426b-809e-ea837d868290",
    "Ben Hill",
);

// TLA 330 — White Lotus Tile (alternate printing)
const WHITE_LOTUS_TILE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WHITE_LOTUS_TILE,
    1,
    "15efe915-5e38-4e00-b10f-28e76ab51597",
    "Antonio José Manzanedo",
);

// TLA 331 — United Front (alternate printing)
const UNITED_FRONT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNITED_FRONT,
    1,
    "dad8f56c-1bf2-4641-885d-04b885533a3c",
    "JungShan",
);

// TLA 332 — Sozin's Comet (alternate printing)
const SOZIN_S_COMET_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SOZIN_S_COMET,
    2,
    "e4588e53-a5cf-463a-a7b6-20437aeaabee",
    "JungShan",
);

// TLA 333 — Avatar Destiny (alternate printing)
const AVATAR_DESTINY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVATAR_DESTINY,
    1,
    "19b71bf7-d354-4729-a160-b85402c084f3",
    "Flavio Girón",
);

// TLA 334 — Fire Lord Azula (alternate printing)
const FIRE_LORD_AZULA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FIRE_LORD_AZULA,
    2,
    "1ea7248c-e4b3-4776-8e6a-cba06da2d6cc",
    "JungShan",
);

// TLA 335 — Ozai, the Phoenix King (alternate printing)
const OZAI_THE_PHOENIX_KING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OZAI_THE_PHOENIX_KING,
    2,
    "d11a45f5-9552-46a5-9f75-008b352c8121",
    "Sidharth Chaturvedi",
);

// TLA 336 — Aang's Iceberg (alternate printing)
const AANG_S_ICEBERG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AANG_S_ICEBERG,
    1,
    "6cd25a10-1e4e-4f47-8a67-221df4907b26",
    "Brigitte Roka & Clifton Stommel",
);

// TLA 337 — Secret of Bloodbending (alternate printing)
const SECRET_OF_BLOODBENDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SECRET_OF_BLOODBENDING,
    1,
    "682fabaf-bfb0-4ee8-b297-f7436c7a68f2",
    "Barbara Rosiak",
);

// TLA 338 — Yue, the Moon Spirit (alternate printing)
const YUE_THE_MOON_SPIRIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YUE_THE_MOON_SPIRIT,
    1,
    "b50e8e26-0f3c-4478-8827-dc3245f31d8a",
    "Barbara Rosiak",
);

// TLA 339 — Foggy Swamp Visions (alternate printing)
const FOGGY_SWAMP_VISIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOGGY_SWAMP_VISIONS,
    1,
    "f8f56648-85be-417a-814b-c356df390ba0",
    "Sija Hong",
);

// TLA 340 — Obsessive Pursuit (alternate printing)
const OBSESSIVE_PURSUIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OBSESSIVE_PURSUIT,
    1,
    "2ff0a75c-b679-436f-8150-faaec3ea1df6",
    "Sija Hong",
);

// TLA 341 — Fated Firepower (alternate printing)
const FATED_FIREPOWER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FATED_FIREPOWER,
    2,
    "13688c4d-d63f-4cef-8aae-16aec7b9effc",
    "Brigitte Roka",
);

// TLA 342 — Firebending Student (alternate printing)
const FIREBENDING_STUDENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIREBENDING_STUDENT,
    1,
    "c0b0e696-3cb1-4192-a127-da1d112489b7",
    "Ina Wong",
);

// TLA 343 — Redirect Lightning (alternate printing)
const REDIRECT_LIGHTNING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REDIRECT_LIGHTNING,
    1,
    "2c1c0fb2-cb74-46a1-9ce4-5274feab1146",
    "Perci Chen",
);

// TLA 344 — The Earth King (alternate printing)
const THE_EARTH_KING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_EARTH_KING,
    1,
    "3687f487-eec1-4979-8008-5c907adafe9d",
    "Brigitte Roka & Clifton Stommel",
);

// TLA 345 — Great Divide Guide (alternate printing)
const GREAT_DIVIDE_GUIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_DIVIDE_GUIDE,
    1,
    "7a89533f-1236-49bf-9a25-1b7f7789eb2a",
    "Shane Beresford",
);

// TLA 346 — Aang, at the Crossroads // Aang, Destined Savior (alternate printing)
const AANG_AT_THE_CROSSROADS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AANG_AT_THE_CROSSROADS,
    2,
    "d7f35cd9-71ac-4191-8604-1a653b56c42d",
    "Brigitte Roka & Clifton Stommel",
);

// TLA 347 — Aang, Swift Savior // Aang and La, Ocean's Fury (alternate printing)
const AANG_SWIFT_SAVIOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AANG_SWIFT_SAVIOR,
    2,
    "077a1ad2-6317-4271-b86c-cc999776251d",
    "Shane Beresford & Perci Chen",
);

// TLA 348 — Bumi, Unleashed (alternate printing)
const BUMI_UNLEASHED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BUMI_UNLEASHED,
    1,
    "7143a5ba-c597-4f8b-9f39-15849e298a6f",
    "Brigitte Roka",
);

// TLA 349 — Iroh, Grand Lotus (alternate printing)
const IROH_GRAND_LOTUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IROH_GRAND_LOTUS,
    1,
    "85af39f5-8207-4fc5-a956-1ee2799ac374",
    "Dalton Pencarinha",
);

// TLA 350 — Katara, the Fearless (alternate printing)
const KATARA_THE_FEARLESS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KATARA_THE_FEARLESS,
    2,
    "50f388bf-868c-466b-9a47-2f437336bba4",
    "Barbara Rosiak",
);

// TLA 351 — Katara, Water Tribe's Hope (alternate printing)
const KATARA_WATER_TRIBE_S_HOPE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KATARA_WATER_TRIBE_S_HOPE,
    1,
    "95547532-c056-44a5-9d80-4f6b729b6e0c",
    "Chun Lo",
);

// TLA 352 — Sokka, Tenacious Tactician (alternate printing)
const SOKKA_TENACIOUS_TACTICIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOKKA_TENACIOUS_TACTICIAN,
    1,
    "b9dc53c3-351e-46da-bce6-ab8c651cb4ea",
    "Faustine Dumontier",
);

// TLA 353 — Toph, the First Metalbender (alternate printing)
const TOPH_THE_FIRST_METALBENDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOPH_THE_FIRST_METALBENDER,
    1,
    "aaa3e063-472e-445a-8440-dc1910899d81",
    "Barbara Rosiak",
);

// TLA 354 — The Legend of Yangchen // Avatar Yangchen (alternate printing)
const THE_LEGEND_OF_YANGCHEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LEGEND_OF_YANGCHEN,
    1,
    "944ddbe5-45bf-44c3-aad5-b051ec79ae8b",
    "Sija Hong",
);

// TLA 355 — The Legend of Kuruk // Avatar Kuruk (alternate printing)
const THE_LEGEND_OF_KURUK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LEGEND_OF_KURUK,
    1,
    "f487686d-2b8a-440f-b2f3-8a07dd95ad93",
    "Barbara Rosiak",
);

// TLA 356 — The Rise of Sozin // Fire Lord Sozin (alternate printing)
const THE_RISE_OF_SOZIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_RISE_OF_SOZIN,
    1,
    "75fcadb9-2573-4c10-8e61-ecdf72120c53",
    "Barbara Rosiak",
);

// TLA 357 — The Legend of Roku // Avatar Roku (alternate printing)
const THE_LEGEND_OF_ROKU_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LEGEND_OF_ROKU,
    1,
    "4a8bdef0-c50c-4a30-81bc-507a8289a81b",
    "Barbara Rosiak",
);

// TLA 358 — The Legend of Kyoshi // Avatar Kyoshi (alternate printing)
const THE_LEGEND_OF_KYOSHI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LEGEND_OF_KYOSHI,
    1,
    "83731884-cd1a-4c12-8b1c-9a1c7b34ef12",
    "Sija Hong",
);

// TLA 359 — Aang, Swift Savior // Aang and La, Ocean's Fury (alternate printing)
const AANG_SWIFT_SAVIOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &AANG_SWIFT_SAVIOR,
    3,
    "372afa84-9ca9-46e6-8643-e16249505c59",
    "Flavio Girón",
);

// TLA 360 — Fire Lord Zuko (alternate printing)
const FIRE_LORD_ZUKO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FIRE_LORD_ZUKO,
    2,
    "0c457a8b-8d56-474d-b788-c058a9882fdd",
    "Flavio Girón",
);

// TLA 361 — Katara, the Fearless (alternate printing)
const KATARA_THE_FEARLESS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KATARA_THE_FEARLESS,
    3,
    "15260c74-6400-46bd-a144-9d9e107eab8a",
    "Flavio Girón",
);

// TLA 362 — Toph, the First Metalbender (alternate printing)
const TOPH_THE_FIRST_METALBENDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TOPH_THE_FIRST_METALBENDER,
    2,
    "1381ec47-55fd-47d9-aded-d21776bc06b9",
    "Flavio Girón",
);

// TLA 363 — Avatar Aang // Aang, Master of Elements (alternate printing)
const AVATAR_AANG_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AVATAR_AANG,
    2,
    "d0467b6f-8c7d-4fcd-99f8-d335bb736484",
    "Bryan Konietzko",
);

// TLA 364 — Airbender Ascension (alternate printing)
const AIRBENDER_ASCENSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AIRBENDER_ASCENSION,
    1,
    "eb941485-d85a-4693-b797-890e38c8d056",
    "Shiren",
);

// TLA 365 — Avatar's Wrath (alternate printing)
const AVATAR_S_WRATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVATAR_S_WRATH,
    1,
    "a24b3b8a-7742-4aa1-8da9-162dd1d49441",
    "Ainezu",
);

// TLA 366 — Hakoda, Selfless Commander (alternate printing)
const HAKODA_SELFLESS_COMMANDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAKODA_SELFLESS_COMMANDER,
    1,
    "3fdb2f5f-c881-4c10-8787-d37a4ee57aa4",
    "Rafater",
);

// TLA 367 — South Pole Voyager (alternate printing)
const SOUTH_POLE_VOYAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOUTH_POLE_VOYAGER,
    1,
    "04b58759-e9a2-46c6-b0ce-82f550c5c38d",
    "Tition",
);

// TLA 368 — Suki, Courageous Rescuer (alternate printing)
const SUKI_COURAGEOUS_RESCUER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUKI_COURAGEOUS_RESCUER,
    1,
    "f4e73660-ca67-4587-9aac-02154e984861",
    "Tky",
);

// TLA 369 — The Mechanist, Aerial Artisan (alternate printing)
const THE_MECHANIST_AERIAL_ARTISAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MECHANIST_AERIAL_ARTISAN,
    1,
    "d638ce04-cff8-4253-95f6-4ebdcfe01309",
    "Le Vuong",
);

// TLA 370 — Spirit Water Revival (alternate printing)
const SPIRIT_WATER_REVIVAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIRIT_WATER_REVIVAL,
    1,
    "eadded94-3c18-43a9-bde9-f7ec86a98cce",
    "Enishi",
);

// TLA 371 — Ty Lee, Chi Blocker (alternate printing)
const TY_LEE_CHI_BLOCKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TY_LEE_CHI_BLOCKER,
    1,
    "4d2e5f2b-993a-48b1-834e-a4fe18e119ae",
    "Gemi",
);

// TLA 372 — Boiling Rock Rioter (alternate printing)
const BOILING_ROCK_RIOTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOILING_ROCK_RIOTER,
    1,
    "7fa397df-99c4-4ea8-9ec1-836626ddcca4",
    "Airi Yoshihisa",
);

// TLA 373 — Day of Black Sun (alternate printing)
const DAY_OF_BLACK_SUN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAY_OF_BLACK_SUN,
    1,
    "f42f6bd4-dc1d-4b5f-9489-2cb109a68904",
    "Matteo Bassini",
);

// TLA 374 — Mai, Scornful Striker (alternate printing)
const MAI_SCORNFUL_STRIKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAI_SCORNFUL_STRIKER,
    1,
    "aaf2c3e2-7941-49a6-9299-7135690f01e1",
    "Hori Airi",
);

// TLA 375 — Wartime Protestors (alternate printing)
const WARTIME_PROTESTORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARTIME_PROTESTORS,
    1,
    "17310896-20e9-4844-9c3e-9e114c619bfe",
    "Yosuke Adachi",
);

// TLA 376 — Zhao, the Moon Slayer (alternate printing)
const ZHAO_THE_MOON_SLAYER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZHAO_THE_MOON_SLAYER,
    1,
    "6a438677-5355-4918-897e-fd2d5b693bb1",
    "Toraji",
);

// TLA 377 — Earthen Ally (alternate printing)
const EARTHEN_ALLY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTHEN_ALLY,
    1,
    "5a2754b7-6a0f-4124-ad31-10214d0f4895",
    "Boell Oyino",
);

// TLA 378 — Elemental Teachings (alternate printing)
const ELEMENTAL_TEACHINGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELEMENTAL_TEACHINGS,
    1,
    "809bd283-8587-44df-af27-fe4d9bdeedd0",
    "Yoshioka",
);

// TLA 379 — Beifong's Bounty Hunters (alternate printing)
const BEIFONG_S_BOUNTY_HUNTERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEIFONG_S_BOUNTY_HUNTERS,
    1,
    "993d8c56-068d-4708-a22c-486fc6f8fcf1",
    "Alexandr Leskinen",
);

// TLA 380 — Earth King's Lieutenant (alternate printing)
const EARTH_KING_S_LIEUTENANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTH_KING_S_LIEUTENANT,
    1,
    "6a431eaf-13a0-4052-b887-ca477bb55847",
    "Nathaniel Himawan",
);

// TLA 381 — Iroh, Tea Master (alternate printing)
const IROH_TEA_MASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IROH_TEA_MASTER,
    1,
    "76a39187-317f-40b6-a920-18edcb0bf6ac",
    "Brian Yuen",
);

// TLA 382 — Sandbender Scavengers (alternate printing)
const SANDBENDER_SCAVENGERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANDBENDER_SCAVENGERS,
    1,
    "c2584dda-a1e6-41d4-ae50-a073db428cee",
    "Alexander Forssberg",
);

// TLA 383 — Sokka, Bold Boomeranger (alternate printing)
const SOKKA_BOLD_BOOMERANGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOKKA_BOLD_BOOMERANGER,
    1,
    "0d4788f3-ae08-4698-ac4f-c78a337981c2",
    "Toni Infante",
);

// TLA 384 — Toph, Hardheaded Teacher (alternate printing)
const TOPH_HARDHEADED_TEACHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOPH_HARDHEADED_TEACHER,
    1,
    "3b36026b-15fb-4eeb-9cb3-266f295e4375",
    "Ruwen Liu",
);

// TLA 385 — Planetarium of Wan Shi Tong (alternate printing)
const PLANETARIUM_OF_WAN_SHI_TONG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PLANETARIUM_OF_WAN_SHI_TONG,
    1,
    "e3276b9a-6cb1-4afb-bcc0-ea5a8d8d6a44",
    "Robin Olausson",
);

// TLA 386 — Abandoned Air Temple (alternate printing)
const ABANDONED_AIR_TEMPLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABANDONED_AIR_TEMPLE,
    1,
    "761988b0-fb0e-45e7-97d3-cef259fe47b7",
    "Dom Lay",
);

// TLA 387 — Agna Qel'a (alternate printing)
const AGNA_QEL_A_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGNA_QEL_A,
    1,
    "501c24b4-26e4-47ac-8a5c-3289d937620e",
    "Dom Lay",
);

// TLA 388 — Ba Sing Se (alternate printing)
const BA_SING_SE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BA_SING_SE,
    1,
    "a8fd4587-7c30-4322-8f19-eb395d66c623",
    "Andreas Rocha",
);

// TLA 389 — Fire Nation Palace (alternate printing)
const FIRE_NATION_PALACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIRE_NATION_PALACE,
    1,
    "372e7d4e-c011-423b-9e2c-af57fb0d5403",
    "Awanqi (Angela Wang)",
);

// TLA 390 — Jasmine Dragon Tea Shop (alternate printing)
const JASMINE_DRAGON_TEA_SHOP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JASMINE_DRAGON_TEA_SHOP,
    1,
    "4487d299-1571-4d44-81bb-9bf54fb33fbc",
    "Leanna Crossan",
);

// TLA 391 — Realm of Koh (alternate printing)
const REALM_OF_KOH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REALM_OF_KOH,
    1,
    "191cbf96-5e92-4b83-aef7-d7ae65b8e555",
    "Andreas Rocha",
);

// TLA 392 — Secret Tunnel (alternate printing)
const SECRET_TUNNEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SECRET_TUNNEL,
    1,
    "51003a38-3493-4524-aed8-9681f5a89037",
    "Alexander Forssberg",
);

// TLA 393 — Firebending Student (alternate printing)
const FIREBENDING_STUDENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FIREBENDING_STUDENT,
    2,
    "79d1889c-4b3d-47d4-847e-4343cdaf9750",
    "Airi Yoshihisa",
);

// TLA 394 — Momo, Friendly Flier (alternate printing)
const MOMO_FRIENDLY_FLIER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOMO_FRIENDLY_FLIER,
    2,
    "a7b91b5f-815c-4bdc-97e4-dc7bc8ed29c0",
    "Ryota Murayama",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AANG_S_JOURNEY,
    &ENERGYBENDING,
    &ZUKO_S_EXILE,
    &AANG_THE_LAST_AIRBENDER,
    &AANG_S_ICEBERG,
    &AIRBENDER_ASCENSION,
    &AIRBENDER_S_REVERSAL,
    &AIRBENDING_LESSON,
    &APPA_LOYAL_SKY_BISON,
    &APPA_STEADFAST_GUARDIAN,
    &AVATAR_ENTHUSIASTS,
    &AVATAR_S_WRATH,
    &COMPASSIONATE_HEALER,
    &CURIOUS_FARM_ANIMALS,
    &DESTINED_CONFRONTATION,
    &EARTH_KINGDOM_JAILER,
    &EARTH_KINGDOM_PROTECTORS,
    &ENTER_THE_AVATAR_STATE,
    &FANCY_FOOTWORK,
    &GATHER_THE_WHITE_LOTUS,
    &GLIDER_KIDS,
    &GLIDER_STAFF,
    &HAKODA_SELFLESS_COMMANDER,
    &INVASION_REINFORCEMENTS,
    &JEONG_JEONG_S_DESERTERS,
    &KYOSHI_WARRIORS,
    &THE_LEGEND_OF_YANGCHEN,
    &MASTER_PIANDAO,
    &MOMO_FRIENDLY_FLIER,
    &MOMO_PLAYFUL_PET,
    &PATH_TO_REDEMPTION,
    &RABAROO_TROOP,
    &RAZOR_RINGS,
    &SANDBENDERS_STORM,
    &SOUTH_POLE_VOYAGER,
    &SOUTHERN_AIR_TEMPLE,
    &SUKI_COURAGEOUS_RESCUER,
    &TEAM_AVATAR,
    &UNITED_FRONT,
    &VENGEFUL_VILLAGERS,
    &WATER_TRIBE_CAPTAIN,
    &WATER_TRIBE_RALLIER,
    &YIP_YIP,
    &ACCUMULATE_WISDOM,
    &BENEVOLENT_RIVER_SPIRIT,
    &BOOMERANG_BASICS,
    &CRASHING_WAVE,
    &EMBER_ISLAND_PRODUCTION,
    &FIRST_TIME_FLYER,
    &FLEXIBLE_WATERBENDER,
    &FORECASTING_FORTUNE_TELLER,
    &GEYSER_LEAPER,
    &GIANT_KOI,
    &GRAN_GRAN,
    &HONEST_WORK,
    &IGUANA_PARROT,
    &INVASION_SUBMERSIBLE,
    &IT_LL_QUENCH_YA,
    &KATARA_BENDING_PRODIGY,
    &KNOWLEDGE_SEEKER,
    &THE_LEGEND_OF_KURUK,
    &LOST_DAYS,
    &MASTER_PAKKU,
    &THE_MECHANIST_AERIAL_ARTISAN,
    &NORTH_POLE_PATROL,
    &OCTOPUS_FORM,
    &OTTER_PENGUIN,
    &ROWDY_SNOWBALLERS,
    &SECRET_OF_BLOODBENDING,
    &SERPENT_OF_THE_PASS,
    &SOKKA_S_HAIKU,
    &THE_SPIRIT_OASIS,
    &SPIRIT_WATER_REVIVAL,
    &TEO_SPIRITED_GLIDER,
    &TIGER_SEAL,
    &TY_LEE_CHI_BLOCKER,
    &THE_UNAGI_OF_KYOSHI_ISLAND,
    &WAN_SHI_TONG_LIBRARIAN,
    &WATERBENDER_ASCENSION,
    &WATERBENDING_LESSON,
    &WATERBENDING_SCROLL,
    &WATERY_GRASP,
    &YUE_THE_MOON_SPIRIT,
    &AZULA_ALWAYS_LIES,
    &AZULA_ON_THE_HUNT,
    &BEETLE_HEADED_MERCHANTS,
    &BOILING_ROCK_RIOTER,
    &BUZZARD_WASP_COLONY,
    &CALLOUS_INSPECTOR,
    &CANYON_CRAWLER,
    &CAT_GATOR,
    &DAI_LI_INDOCTRINATION,
    &DAY_OF_BLACK_SUN,
    &DEADLY_PRECISION,
    &FATAL_FISSURE,
    &THE_FIRE_NATION_DRILL,
    &FIRE_NATION_ENGINEER,
    &FIRE_NAVY_TREBUCHET,
    &FOGGY_SWAMP_HUNTERS,
    &FOGGY_SWAMP_VISIONS,
    &HOG_MONKEY,
    &JOO_DEE_ONE_OF_MANY,
    &JUNE_BOUNTY_HUNTER,
    &KOH_THE_FACE_STEALER,
    &LO_AND_LI_TWIN_TUTORS,
    &MAI_SCORNFUL_STRIKER,
    &MERCHANT_OF_MANY_HATS,
    &NORTHERN_AIR_TEMPLE,
    &OBSESSIVE_PURSUIT,
    &OZAI_S_CRUELTY,
    &PHOENIX_FLEET_AIRSHIP,
    &PIRATE_PEDDLERS,
    &RAVEN_EAGLE,
    &THE_RISE_OF_SOZIN,
    &RUINOUS_WATERBENDING,
    &SOLD_OUT,
    &SWAMPSNARE_TRAP,
    &TUNDRA_TANK,
    &WOLFBAT,
    &ZUKO_S_CONVICTION,
    &BOAR_Q_PINE,
    &BUMI_BASH,
    &THE_CAVE_OF_TWO_LOVERS,
    &COMBUSTION_MAN,
    &COMBUSTION_TECHNIQUE,
    &CRESCENT_ISLAND_TEMPLE,
    &CUNNING_MANEUVER,
    &DESERTER_S_DISCIPLE,
    &FATED_FIREPOWER,
    &FIRE_NATION_ATTACKS,
    &FIRE_NATION_CADETS,
    &FIRE_NATION_RAIDER,
    &FIRE_SAGES,
    &FIREBENDER_ASCENSION,
    &FIREBENDING_LESSON,
    &FIREBENDING_STUDENT,
    &HOW_TO_START_A_RIOT,
    &IROH_S_DEMONSTRATION,
    &JEONG_JEONG_THE_DESERTER,
    &JET_S_BRAINWASHING,
    &THE_LAST_AGNI_KAI,
    &THE_LEGEND_OF_ROKU,
    &MAI_JADED_EDGE,
    &MONGOOSE_LIZARD,
    &PRICE_OF_FREEDOM,
    &RAN_AND_SHAW,
    &REDIRECT_LIGHTNING,
    &ROUGH_RHINO_CAVALRY,
    &SOLSTICE_REVELATIONS,
    &SOZIN_S_COMET,
    &TIGER_DILLO,
    &TREETOP_FREEDOM_FIGHTERS,
    &TWIN_BLADES,
    &TY_LEE_ARTFUL_ACROBAT,
    &WAR_BALLOON,
    &WARTIME_PROTESTORS,
    &YUYAN_ARCHERS,
    &ZHAO_THE_MOON_SLAYER,
    &ZUKO_EXILED_PRINCE,
    &ALLIES_AT_LAST,
    &AVATAR_DESTINY,
    &BADGERMOLE,
    &BADGERMOLE_CUB,
    &THE_BOULDER_READY_TO_RUMBLE,
    &BUMI_KING_OF_THREE_TRIALS,
    &CYCLE_OF_RENEWAL,
    &DILIGENT_ZOOKEEPER,
    &THE_EARTH_KING,
    &EARTH_KINGDOM_GENERAL,
    &EARTH_RUMBLE,
    &EARTHBENDER_ASCENSION,
    &EARTHBENDING_LESSON,
    &EARTHEN_ALLY,
    &ELEMENTAL_TEACHINGS,
    &FLOPSIE_BUMI_S_BUDDY,
    &FOGGY_SWAMP_VINEBENDER,
    &GREAT_DIVIDE_GUIDE,
    &HARU_HIDDEN_TALENT,
    &INVASION_TACTICS,
    &KYOSHI_ISLAND_PLAZA,
    &LEAVES_FROM_THE_VINE,
    &THE_LEGEND_OF_KYOSHI,
    &ORIGIN_OF_METALBENDING,
    &OSTRICH_HORSE,
    &PILLAR_LAUNCH,
    &RAUCOUS_AUDIENCE,
    &REBELLIOUS_CAPTIVES,
    &ROCKALANCHE,
    &ROCKY_REBUKE,
    &SABER_TOOTH_MOOSE_LION,
    &SEISMIC_SENSE,
    &SHARED_ROOTS,
    &SPARRING_DUMMY,
    &TOPH_THE_BLIND_BANDIT,
    &TRUE_ANCESTRY,
    &TURTLE_DUCK,
    &UNLUCKY_CABBAGE_MERCHANT,
    &WALLTOP_SENTRIES,
    &AANG_AT_THE_CROSSROADS,
    &AANG_SWIFT_SAVIOR,
    &ABANDON_ATTACHMENTS,
    &AIR_NOMAD_LEGACY,
    &AVATAR_AANG,
    &AZULA_CUNNING_USURPER,
    &BEIFONG_S_BOUNTY_HUNTERS,
    &BITTER_WORK,
    &BUMI_UNLEASHED,
    &CAT_OWL,
    &CRUEL_ADMINISTRATOR,
    &DAI_LI_AGENTS,
    &DRAGONFLY_SWARM,
    &EARTH_KINGDOM_SOLDIER,
    &EARTH_KING_S_LIEUTENANT,
    &EARTH_RUMBLE_WRESTLERS,
    &EARTH_VILLAGE_RUFFIANS,
    &FIRE_LORD_AZULA,
    &FIRE_LORD_ZUKO,
    &FOGGY_SWAMP_SPIRIT_KEEPER,
    &GURU_PATHIK,
    &HAMA_THE_BLOODBENDER,
    &HEI_BAI_SPIRIT_OF_BALANCE,
    &HERMITIC_HERBALIST,
    &IROH_GRAND_LOTUS,
    &IROH_TEA_MASTER,
    &JET_FREEDOM_FIGHTER,
    &KATARA_THE_FEARLESS,
    &KATARA_WATER_TRIBE_S_HOPE,
    &THE_LION_TURTLE,
    &LONG_FENG_GRAND_SECRETARIAT,
    &MESSENGER_HAWK,
    &OZAI_THE_PHOENIX_KING,
    &PLATYPUS_BEAR,
    &PRETENDING_POXBEARERS,
    &PROFESSOR_ZEI_ANTHROPOLOGIST,
    &SANDBENDER_SCAVENGERS,
    &SOKKA_BOLD_BOOMERANGER,
    &SOKKA_LATERAL_STRATEGIST,
    &SOKKA_TENACIOUS_TACTICIAN,
    &SUKI_KYOSHI_WARRIOR,
    &SUN_WARRIORS,
    &TOLLS_OF_WAR,
    &TOPH_HARDHEADED_TEACHER,
    &TOPH_THE_FIRST_METALBENDER,
    &UNCLE_IROH,
    &VINDICTIVE_WARDEN,
    &WANDERING_MUSICIANS,
    &WHITE_LOTUS_REINFORCEMENTS,
    &ZHAO_RUTHLESS_ADMIRAL,
    &ZUKO_CONFLICTED,
    &BARRELS_OF_BLASTING_JELLY,
    &BENDER_S_WATERSKIN,
    &FIRE_NATION_WARSHIP,
    &KYOSHI_BATTLE_FAN,
    &METEOR_SWORD,
    &PLANETARIUM_OF_WAN_SHI_TONG,
    &TRUSTY_BOOMERANG,
    &THE_WALLS_OF_BA_SING_SE,
    &WHITE_LOTUS_TILE,
    &ABANDONED_AIR_TEMPLE,
    &AGNA_QEL_A,
    &AIRSHIP_ENGINE_ROOM,
    &BA_SING_SE,
    &BOILING_ROCK_PRISON,
    &FIRE_NATION_PALACE,
    &FOGGY_BOTTOM_SWAMP,
    &JASMINE_DRAGON_TEA_SHOP,
    &KYOSHI_VILLAGE,
    &MEDITATION_POOLS,
    &MISTY_PALMS_OASIS,
    &NORTH_POLE_GATES,
    &OMASHU_CITY,
    &REALM_OF_KOH,
    &RUMBLE_ARENA,
    &SECRET_TUNNEL,
    &SERPENT_S_PASS,
    &SUN_BLESSED_PEAK,
    &WHITE_LOTUS_HIDEOUT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    CORRUPT_COURT_OFFICIAL_REPRINT,
    EPIC_DOWNFALL_REPRINT,
    HEARTLESS_ACT_REPRINT,
    LIGHTNING_STRIKE_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_ALTERNATE_1,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_1,
    FOREST_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_2,
    FATED_FIREPOWER_ALTERNATE_1,
    AANG_SWIFT_SAVIOR_ALTERNATE_1,
    FIRE_NATION_ATTACKS_ALTERNATE_1,
    CRASHING_WAVE_ALTERNATE_1,
    COMBUSTION_TECHNIQUE_ALTERNATE_1,
    ZUKO_CONFLICTED_ALTERNATE_1,
    AZULA_CUNNING_USURPER_ALTERNATE_1,
    AANG_AT_THE_CROSSROADS_ALTERNATE_1,
    KATARA_THE_FEARLESS_ALTERNATE_1,
    DAI_LI_AGENTS_ALTERNATE_1,
    EARTHBENDER_ASCENSION_ALTERNATE_1,
    AVATAR_AANG_ALTERNATE_1,
    SOZIN_S_COMET_ALTERNATE_1,
    WATERBENDER_ASCENSION_ALTERNATE_1,
    OZAI_THE_PHOENIX_KING_ALTERNATE_1,
    FIREBENDER_ASCENSION_ALTERNATE_1,
    FIRE_LORD_AZULA_ALTERNATE_1,
    THE_LAST_AGNI_KAI_ALTERNATE_1,
    FIRE_LORD_ZUKO_ALTERNATE_1,
    APPA_STEADFAST_GUARDIAN_ALTERNATE_1,
    MOMO_FRIENDLY_FLIER_ALTERNATE_1,
    TIGER_SEAL_ALTERNATE_1,
    THE_UNAGI_OF_KYOSHI_ISLAND_ALTERNATE_1,
    WAN_SHI_TONG_LIBRARIAN_ALTERNATE_1,
    THE_FIRE_NATION_DRILL_ALTERNATE_1,
    KOH_THE_FACE_STEALER_ALTERNATE_1,
    PHOENIX_FLEET_AIRSHIP_ALTERNATE_1,
    RAVEN_EAGLE_ALTERNATE_1,
    RAN_AND_SHAW_ALTERNATE_1,
    BADGERMOLE_CUB_ALTERNATE_1,
    DILIGENT_ZOOKEEPER_ALTERNATE_1,
    THE_LION_TURTLE_ALTERNATE_1,
    THE_WALLS_OF_BA_SING_SE_ALTERNATE_1,
    WHITE_LOTUS_TILE_ALTERNATE_1,
    UNITED_FRONT_ALTERNATE_1,
    SOZIN_S_COMET_ALTERNATE_2,
    AVATAR_DESTINY_ALTERNATE_1,
    FIRE_LORD_AZULA_ALTERNATE_2,
    OZAI_THE_PHOENIX_KING_ALTERNATE_2,
    AANG_S_ICEBERG_ALTERNATE_1,
    SECRET_OF_BLOODBENDING_ALTERNATE_1,
    YUE_THE_MOON_SPIRIT_ALTERNATE_1,
    FOGGY_SWAMP_VISIONS_ALTERNATE_1,
    OBSESSIVE_PURSUIT_ALTERNATE_1,
    FATED_FIREPOWER_ALTERNATE_2,
    FIREBENDING_STUDENT_ALTERNATE_1,
    REDIRECT_LIGHTNING_ALTERNATE_1,
    THE_EARTH_KING_ALTERNATE_1,
    GREAT_DIVIDE_GUIDE_ALTERNATE_1,
    AANG_AT_THE_CROSSROADS_ALTERNATE_2,
    AANG_SWIFT_SAVIOR_ALTERNATE_2,
    BUMI_UNLEASHED_ALTERNATE_1,
    IROH_GRAND_LOTUS_ALTERNATE_1,
    KATARA_THE_FEARLESS_ALTERNATE_2,
    KATARA_WATER_TRIBE_S_HOPE_ALTERNATE_1,
    SOKKA_TENACIOUS_TACTICIAN_ALTERNATE_1,
    TOPH_THE_FIRST_METALBENDER_ALTERNATE_1,
    THE_LEGEND_OF_YANGCHEN_ALTERNATE_1,
    THE_LEGEND_OF_KURUK_ALTERNATE_1,
    THE_RISE_OF_SOZIN_ALTERNATE_1,
    THE_LEGEND_OF_ROKU_ALTERNATE_1,
    THE_LEGEND_OF_KYOSHI_ALTERNATE_1,
    AANG_SWIFT_SAVIOR_ALTERNATE_3,
    FIRE_LORD_ZUKO_ALTERNATE_2,
    KATARA_THE_FEARLESS_ALTERNATE_3,
    TOPH_THE_FIRST_METALBENDER_ALTERNATE_2,
    AVATAR_AANG_ALTERNATE_2,
    AIRBENDER_ASCENSION_ALTERNATE_1,
    AVATAR_S_WRATH_ALTERNATE_1,
    HAKODA_SELFLESS_COMMANDER_ALTERNATE_1,
    SOUTH_POLE_VOYAGER_ALTERNATE_1,
    SUKI_COURAGEOUS_RESCUER_ALTERNATE_1,
    THE_MECHANIST_AERIAL_ARTISAN_ALTERNATE_1,
    SPIRIT_WATER_REVIVAL_ALTERNATE_1,
    TY_LEE_CHI_BLOCKER_ALTERNATE_1,
    BOILING_ROCK_RIOTER_ALTERNATE_1,
    DAY_OF_BLACK_SUN_ALTERNATE_1,
    MAI_SCORNFUL_STRIKER_ALTERNATE_1,
    WARTIME_PROTESTORS_ALTERNATE_1,
    ZHAO_THE_MOON_SLAYER_ALTERNATE_1,
    EARTHEN_ALLY_ALTERNATE_1,
    ELEMENTAL_TEACHINGS_ALTERNATE_1,
    BEIFONG_S_BOUNTY_HUNTERS_ALTERNATE_1,
    EARTH_KING_S_LIEUTENANT_ALTERNATE_1,
    IROH_TEA_MASTER_ALTERNATE_1,
    SANDBENDER_SCAVENGERS_ALTERNATE_1,
    SOKKA_BOLD_BOOMERANGER_ALTERNATE_1,
    TOPH_HARDHEADED_TEACHER_ALTERNATE_1,
    PLANETARIUM_OF_WAN_SHI_TONG_ALTERNATE_1,
    ABANDONED_AIR_TEMPLE_ALTERNATE_1,
    AGNA_QEL_A_ALTERNATE_1,
    BA_SING_SE_ALTERNATE_1,
    FIRE_NATION_PALACE_ALTERNATE_1,
    JASMINE_DRAGON_TEA_SHOP_ALTERNATE_1,
    REALM_OF_KOH_ALTERNATE_1,
    SECRET_TUNNEL_ALTERNATE_1,
    FIREBENDING_STUDENT_ALTERNATE_2,
    MOMO_FRIENDLY_FLIER_ALTERNATE_2,
];
