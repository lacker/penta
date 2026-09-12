//! Edge of Eternities card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostObjectIndex;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ClassifyObjectsDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DeclarativeAbilityDef;
use crate::card::DestroyFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::HalvedValueDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ModalSpellDef;
use crate::card::MoveObjectsDef;
use crate::card::MoveToZoneCostDef;
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
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::QuantifierDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::RoundingDef;
use crate::card::ScaledValueDef;
use crate::card::SetOperationDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TokenCopyDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TriggeredAbilityDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2006::dissension as catalog_dis;
use crate::card::sets::y2006::guildpact as catalog_gpt;
use crate::card::sets::y2014::journey_into_nyx as catalog_jou;
use crate::card::sets::y2018::rivals_of_ixalan as catalog_rix;
use crate::card::sets::y2022::kamigawa_neon_dynasty as catalog_neo;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EOE",
    slug: "edge-of-eternities",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

pub const STATION: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:station");

/// Station taps another creature and reads its current or last-known power on resolution.
#[must_use]
pub const fn station(text: &'static str) -> AbilityDef {
    AbilityDef::activated(
        text,
        &[CostDef::TapPermanents {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            controller: PlayerRelation::You,
            count: 1,
        }],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::named("charge"),
            amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                objects: ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                    AdditionalCostObjectIndex::PRIMARY,
                )),
                select: ObjectValueDef::Power,
                operation: AggregateOperationDef::Sum,
            }),
        },
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)
    .labeled(STATION)
}

// EOE 1 — Anticausal Vestige
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static ANTICAUSAL_VESTIGE: CardRecord = CardRecord::new(
    "Anticausal Vestige",
    "35372b69-6086-44e0-9f7c-681e362e5142",
    "Chase Stone",
    CardRules::unsupported(),
);

// EOE 2 — Tezzeret, Cruel Captain
static AN_ARTIFACT_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

pub(in crate::card::sets) static TEZZERET_CRUEL_CAPTAIN: CardRecord = CardRecord::new(
    "Tezzeret, Cruel Captain",
    "02e8e540-8aa3-4e6a-9a11-c3949cab5f0f",
    "Chris Rahn",
// Three colourless for a planeswalker that an artifact deck keeps
    // topping up, and whose zero is free every turn.
    CardRules::new_planeswalker(mana_cost!("{3}"), &["Tezzeret"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever an artifact you control enters, put a loyalty counter on Tezzeret.",
                TriggerEventDef::zone_changed(AN_ARTIFACT_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Loyalty,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "0: Untap target artifact or creature. If it\'s an artifact creature, put a +1/+1 counter \
                 on it.",
                &[CostDef::Loyalty(0)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::IfCondition {
                        // The rider is asked of the target as the ability resolves, so an artifact
                        // animated in response is a legal thing to grow.
                        condition: &TriggerConditionDef::TargetMatches {
                                slot: TargetIndex::PRIMARY,
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                ]),
                            },
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    },
                ]),
            ),
            AbilityDef::activated(
                "−3: Search your library for an artifact card with mana value 1 or less, reveal it, put \
                 it into your hand, then shuffle.",
                &[CostDef::Loyalty(-3)],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // A one-mana artifact, which is what the deck this is in is made of.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ManaValueAtMost(1),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "−7: You get an emblem with \"At the beginning of combat on your turn, put three +1/+1 \
                 counters on target artifact you control. If it\'s not a creature, it becomes a 0/0 Robot \
                 artifact creature.\"",
                &[CostDef::Loyalty(-7)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new("Tezzeret, Cruel Captain emblem", &[AbilityDef::triggered_with_targets(
                            "At the beginning of combat on your turn, put three +1/+1 counters on target artifact you \
                             control. If it's not a creature, it becomes a 0/0 Robot artifact creature.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::BeginningOfCombat,
                                player: PlayerRelation::You,
                            },
                            &[AbilityTargetDef::exactly_one_permanent(
                                    AN_ARTIFACT_YOU_CONTROL,
                                )],
                            EffectDef::Sequence(&[
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(3),
                                },
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::TargetMatches {
                                            slot: TargetIndex::PRIMARY,
                                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                                        },
                                    then: &EffectDef::Apply {
                                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        // "If it's not a creature, it becomes a 0/0 Robot artifact creature." The
                                        // counters go on first, so an artifact that was not a creature ends up a
                                        // 3/3: the base is what changes, and the counters sit on top of it.
                                        effect: AppliedEffectDef::Composite(&[
                                            AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Robot"])),
                                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(0)),
                                        ]),
                                        duration: ResolvedEffectDurationDef::Permanent,
                                    },
                                },
                            ]),
                        )]),
                },
            ),
        ]),
);

// EOE 3 — All-Fates Stalker
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static ALL_FATES_STALKER: CardRecord = CardRecord::new(
    "All-Fates Stalker",
    "82ae4f7b-8122-4af6-8079-888eabf1a11e",
    "Lius Lasahido",
    CardRules::unsupported(),
);

// EOE 4 — Astelli Reclaimer
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately. Also needs the source spell's mana-spent amount retained for target selection after the source leaves.
pub(in crate::card::sets) static ASTELLI_RECLAIMER: CardRecord = CardRecord::new(
    "Astelli Reclaimer",
    "4fb36405-cd28-432f-b0a4-e74ff8be928d",
    "Carly Milligan",
    CardRules::unsupported(),
);

// EOE 5 — Auxiliary Boosters
pub(in crate::card::sets) static AUXILIARY_BOOSTERS: CardRecord = CardRecord::new(
    "Auxiliary Boosters",
    "43706295-afd6-442c-8828-8cf978152701",
    "Dmitry Burmak",
    CardRules::new_artifact(mana_cost!("{4}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a 2/2 colorless Robot \
                 artifact creature token and attach this Equipment to it.",
                EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2)
                    .with_created_tokens(CreatedTokensDef {
                        binding: crate::Binding!("robot"),
                        then: &EffectDef::Attach {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("robot"),
                            )),
                        },
                    }),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+2 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{3}"))],
                "Equip {3} ({3}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// EOE 6 — Banishing Light (reprint)
const BANISHING_LIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::BANISHING_LIGHT,
    "c45f11cd-a0aa-4d14-aa21-57f0969f3e2b",
    "Rovina Cai",
);

// EOE 7 — Beyond the Quiet
pub(in crate::card::sets) static BEYOND_THE_QUIET: CardRecord = CardRecord::new(
    "Beyond the Quiet",
    "ce503869-8130-4afe-9691-4e90376b4bc4",
    "Yohann Schepacz",
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Exile all creatures and Spacecraft.",
        EffectDef::move_to_zone(
            EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ))),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )]),
);

// EOE 8 — Brightspear Zealot
pub(in crate::card::sets) static BRIGHTSPEAR_ZEALOT: CardRecord = CardRecord::new(
    "Brightspear Zealot",
    "e7f7541a-5910-4f33-8c1d-3507ce3a426e",
    "Bryan Sola",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 4).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "This creature gets +2/+0 as long as you've cast two or more \
             spells this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::You,
                        spell: ObjectPredicateDef::Any,
                    }),
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

// EOE 9 — Cosmogrand Zenith
pub(in crate::card::sets) static COSMOGRAND_ZENITH: CardRecord = CardRecord::new(
    "Cosmogrand Zenith",
    "b3c1e5e3-4e6b-456a-958c-7a75c38f8183",
    "Anna Steinbauer",
// Three mana for a 2/4 that pays a second time every turn the hand has
    // two spells in it, and the choice is between going wider and going
    // taller.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 4)
        .with_abilities(&[AbilityDef::defined(
            "Whenever you cast your second spell each turn, choose one —\n• Create two 1/1 white Human \
             Soldier creature tokens.\n• Put a +1/+1 counter on each creature you control.",
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(TriggerEventDef::spell_cast(
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ))
                // Exactly the second, not the second or later: the spell that caused the
                // trigger has already been counted by the time this is read.
                .with_condition(&TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                })
                .with_modes(ModalSpellDef::choose_one(&[
                    AbilityDef::spell(
                        "Create two 1/1 white Human Soldier creature tokens.",
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                                &["Human", "Soldier"],
                                &[ManaColor::White],
                                1,
                                1,
                            )))
                            .with_count(ValueDef::Constant(2)),
                        ),
                    ),
                    // Each creature you control as the trigger resolves, which includes the
                    // tokens the other mode would have made and the Zenith itself.
                    AbilityDef::spell(
                        "Put a +1/+1 counter on each creature you control.",
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                ])),
            ),
            EffectDef::None,
        )]),
);

// EOE 10 — Dawnstrike Vanguard
pub(in crate::card::sets) static DAWNSTRIKE_VANGUARD: CardRecord = CardRecord::new(
    "Dawnstrike Vanguard",
    "5a041722-9483-469f-9c17-7f0253b0db50",
    "Arif Wijaya",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Human", "Knight"], 4, 5).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control two or more \
             tapped creatures, put a +1/+1 counter on each creature you \
             control other than this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 11 — Dockworker Drone
// Audit: unsupported — Needs copying the entire last-known inventory of counter kinds and amounts from a departed object to another; existing add/remove effects name one fixed counter kind.
pub(in crate::card::sets) static DOCKWORKER_DRONE: CardRecord = CardRecord::new(
    "Dockworker Drone",
    "eeff069f-427b-42ad-afb1-36f0e547fb74",
    "Marco Gorlei",
    CardRules::unsupported(),
);

// EOE 12 — Dual-Sun Adepts
pub(in crate::card::sets) static DUAL_SUN_ADEPTS: CardRecord = CardRecord::new(
    "Dual-Sun Adepts",
    "c7e8c830-77ae-437f-8e28-ce61c5fde6b6",
    "Ioannis Fiore",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(&[
        abilities::double_strike(),
        AbilityDef::activated(
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
        ),
    ]),
);

// EOE 13 — Dual-Sun Technique
pub(in crate::card::sets) static DUAL_SUN_TECHNIQUE: CardRecord = CardRecord::new(
    "Dual-Sun Technique",
    "f8931132-391f-4f16-b480-0a245ab2ec21",
    "Ioannis Fiore",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gains double strike until end of \
         turn. If it has a +1/+1 counter on it, draw a card.",
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
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                },
                then: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ]),
    )]),
);

// EOE 14 — Emergency Eject
pub(in crate::card::sets) static EMERGENCY_EJECT: CardRecord = CardRecord::new(
    "Emergency Eject",
    "fc98b2d4-86fc-4c47-b2a7-1f3c89463607",
    "Leon Tukker",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target nonland permanent. Its controller creates a \
         Lander token. (It's an artifact with \"{2}, {T}, Sacrifice \
         this token: Search your library for a basic land card, put it \
         onto the battlefield tapped, then shuffle.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            )
            .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                TargetIndex::PRIMARY,
            ))),
        ]),
    )]),
);

// EOE 15 — Exalted Sunborn
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static EXALTED_SUNBORN: CardRecord = CardRecord::new(
    "Exalted Sunborn",
    "7e1fe101-f634-41e5-9aa4-e8d7474535dc",
    "Scott M. Fischer",
    CardRules::unsupported(),
);

// EOE 16 — Exosuit Savior
pub(in crate::card::sets) static EXOSUIT_SAVIOR: CardRecord = CardRecord::new(
    "Exosuit Savior",
    "826c0455-a6ce-43ad-bd5c-0a5df169da90",
    "Benjamin Ee",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one other target \
             permanent you control to its owner's hand.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// EOE 17 — Flight-Deck Coordinator
pub(in crate::card::sets) static FLIGHT_DECK_COORDINATOR: CardRecord = CardRecord::new(
    "Flight-Deck Coordinator",
    "88cc6328-a035-4f74-b786-390e5c7c324c",
    "Diego Gisbert",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 3, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control two or more \
             tapped creatures, you gain 2 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// EOE 18 — Focus Fire
pub(in crate::card::sets) static FOCUS_FIRE: CardRecord = CardRecord::new(
    "Focus Fire",
    "a9ddfcbc-0f84-4315-aaa3-ca54ff64d7de",
    "Borja Pindado",
    // The floor is already two damage in combat, and a board counts twice:
    // each body both survives the trade and raises what this kills.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Focus Fire deals X damage to target attacking or blocking creature, where X is 2 plus \
         the number of creatures and/or Spacecraft you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Sum(&SumValueDef::new(
                ValueDef::Constant(2),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    // A Spacecraft that has stationed up is already a
                    // creature, so the two halves overlap and the query has
                    // to match each permanent once rather than twice.
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        ),
    )),
);

// EOE 19 — Haliya, Guided by Light
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static HALIYA_GUIDED_BY_LIGHT: CardRecord = CardRecord::new(
    "Haliya, Guided by Light",
    "6f7c63ae-5df3-410f-8643-b8c69133ca9d",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// EOE 20 — Hardlight Containment
// Audit: unsupported — Needs exile-until-source-leaves with immediate return when the duration ends (CR 610.3); an ordinary leaves trigger returns through the stack too late.
pub(in crate::card::sets) static HARDLIGHT_CONTAINMENT: CardRecord = CardRecord::new(
    "Hardlight Containment",
    "0b934241-4d6b-4b9c-99f1-c49cb387cf56",
    "Dominik Mayer",
    CardRules::unsupported(),
);

// EOE 21 — Honor
pub(in crate::card::sets) static HONOR: CardRecord = CardRecord::new(
    "Honor",
    "d0b4e925-1d59-41a1-bc06-9982695d778f",
    "Eli Minaya",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// EOE 22 — Honored Knight-Captain
pub(in crate::card::sets) static HONORED_KNIGHT_CAPTAIN: CardRecord = CardRecord::new(
    "Honored Knight-Captain",
    "05a6ab03-f0b9-4738-a5f9-5d95bb22de75",
    "Forrest Imel",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Advisor", "Knight"], 1, 1)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a 1/1 white Human Soldier \
                 creature token.",
                EffectDef::create_creature_token(&["Human", "Soldier"], &[ManaColor::White], 1, 1),
            ),
            AbilityDef::activated(
                "{4}{W}{W}, Sacrifice this creature: Search your library for \
                 an Equipment card, put it onto the battlefield, then shuffle.",
                &[
                    CostDef::Mana(mana_cost!("{4}{W}{W}")),
                    CostDef::SacrificeSource,
                ],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// EOE 23 — Knight Luminary
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static KNIGHT_LUMINARY: CardRecord = CardRecord::new(
    "Knight Luminary",
    "34334971-c1b7-4506-a6dd-77f66b3ae4e7",
    "Aaron Miller",
    CardRules::unsupported(),
);

// EOE 24 — Lightstall Inquisitor
// Audit: unsupported — Needs an indefinite owner play permission that also makes lands played through it enter tapped; existing surcharge permissions do not carry this land-entry modification.
pub(in crate::card::sets) static LIGHTSTALL_INQUISITOR: CardRecord = CardRecord::new(
    "Lightstall Inquisitor",
    "635245e9-c27f-4a51-a6f1-bae62e696542",
    "Arif Wijaya",
    CardRules::unsupported(),
);

// EOE 25 — Lumen-Class Frigate
pub(in crate::card::sets) static LUMEN_CLASS_FRIGATE: CardRecord = CardRecord::new(
    "Lumen-Class Frigate",
    "0cc59b5a-65fa-47cc-8ac7-b7c3f533a782",
    "Zezhou Chen",
    CardRules::new_spacecraft(mana_cost!("{1}{W}"), 3, 5).with_abilities(&[
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 12+.)",
        ),
        AbilityDef::static_ability(
            "2+ | Other creatures you control get +1/+1.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
                then: &EffectDef::StaticApply {
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
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
        AbilityDef::static_ability(
            "12+ | Flying, lifelink",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 12,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 26 — Luxknight Breacher
pub(in crate::card::sets) static LUXKNIGHT_BREACHER: CardRecord = CardRecord::new(
    "Luxknight Breacher",
    "c1236731-0d33-4705-8077-3cf58acf9a39",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it for each \
             other creature and/or artifact you control.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ),
        ),
    ]),
);

// EOE 27 — Pinnacle Starcage
// Audit: unsupported — Needs exile-until-source-leaves with immediate return and a linked group of exiled objects retained for the second ability; a leaves trigger is not the printed duration.
pub(in crate::card::sets) static PINNACLE_STARCAGE: CardRecord = CardRecord::new(
    "Pinnacle Starcage",
    "b1f40c4c-a955-4d9c-8225-251fa4159124",
    "Leon Tukker",
    CardRules::unsupported(),
);

// EOE 28 — Pulsar Squadron Ace
pub(in crate::card::sets) static PULSAR_SQUADRON_ACE: CardRecord = CardRecord::new(
    "Pulsar Squadron Ace",
    "8d989cdc-cbd7-4b71-9589-59618597ac8a",
    "Javier Charro",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Pilot"], 1, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, look at the top five cards of your \
             library. You may reveal a Spacecraft card from among them and \
             put it into your hand. Put the rest on the bottom of your \
             library in a random order. If you didn't put a card into your \
             hand this way, put a +1/+1 counter on this creature.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("taken"),
                        then: &EffectDef::Sequence(&[
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
                            EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ValueComparison(
                                    &ValueComparisonDef {
                                        left: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                            crate::Binding!("taken"),
                                        )),
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
                        ]),
                    },
                ]),
            }),
        ),
    ]),
);

// EOE 29 — Radiant Strike
pub(in crate::card::sets) static RADIANT_STRIKE: CardRecord = CardRecord::new(
    "Radiant Strike",
    "8c38e4cb-918b-4493-872b-66c90dcfd339",
    "Aleksi Briclot",
    CardRules::new_instant(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact or tapped creature. You gain 3 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Tapped,
                ]),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )]),
);

// EOE 30 — Rayblade Trooper
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static RAYBLADE_TROOPER: CardRecord = CardRecord::new(
    "Rayblade Trooper",
    "c08c7bf9-a2ed-45c6-8b48-15122d9d9e37",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// EOE 31 — Reroute Systems
pub(in crate::card::sets) static REROUTE_SYSTEMS: CardRecord = CardRecord::new(
    "Reroute Systems",
    "3bbdba38-2b99-4226-98e3-6d2580345d6d",
    "Sergey Glushakov",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target artifact or creature gains indestructible until end of \
                 turn. (Damage and effects that say \"destroy\" don't destroy \
                 it.)",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Reroute Systems deals 2 damage to target tapped creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
        ],
    )]),
);

// EOE 32 — Rescue Skiff
pub(in crate::card::sets) static RESCUE_SKIFF: CardRecord = CardRecord::new(
    "Rescue Skiff",
    "6fe86bfb-c67d-4df9-88c9-f083091f4cda",
    "Viko Menezes",
    CardRules::new_spacecraft(mana_cost!("{5}{W}"), 5, 6).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Spacecraft enters, return target creature or \
             enchantment card from your graveyard to the battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 10+.)",
        ),
        AbilityDef::static_ability(
            "10+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 10,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 33 — Scout for Survivors
// Audit: unsupported — Needs a total mana-value limit shared by all chosen targets; target declarations currently constrain each object and target count independently.
pub(in crate::card::sets) static SCOUT_FOR_SURVIVORS: CardRecord = CardRecord::new(
    "Scout for Survivors",
    "ebf3a6dd-a447-46f9-8b10-091ac8cbaa18",
    "Greg Staples",
    CardRules::unsupported(),
);

// EOE 34 — Seam Rip
// Audit: unsupported — Needs exile-until-source-leaves with immediate return when the duration ends (CR 610.3); an ordinary leaves trigger returns through the stack too late.
pub(in crate::card::sets) static SEAM_RIP: CardRecord = CardRecord::new(
    "Seam Rip",
    "9d298847-2d02-4593-b4d3-c5b722edac1e",
    "Sam Guay",
    CardRules::unsupported(),
);

// EOE 35 — The Seriema
pub(in crate::card::sets) static THE_SERIEMA: CardRecord = CardRecord::new(
    "The Seriema",
    "dec91ec3-42d3-4922-96f0-dbb50a576084",
    "Sergey Glushakov",
    CardRules::new_spacecraft(mana_cost!("{1}{W}{W}"), 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When The Seriema enters, search your library for a legendary \
                 creature card, reveal it, put it into your hand, then \
                 shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            station(
                "Station (Tap another creature you control: Put charge \
                 counters equal to its power on this Spacecraft. Station only \
                 as a sorcery. It's an artifact creature at 7+.)",
            ),
            AbilityDef::static_ability(
                "7+ | Flying",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 7,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_card_types(
                                CardTypeSet::single(CardType::Artifact)
                                    .union(CardTypeSet::single(CardType::Creature)),
                            ),
                            AppliedEffectDef::add_ability(&abilities::flying()),
                        ]),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Other tapped legendary creatures you control have indestructible.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 7,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                    ObjectPredicateDef::Tapped,
                                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    },
                },
            ),
        ]),
);

// EOE 36 — Squire's Lightblade
pub(in crate::card::sets) static SQUIRE_S_LIGHTBLADE: CardRecord = CardRecord::new(
    "Squire's Lightblade",
    "2a0accba-85d4-4aa4-a70c-80fcce48c261",
    "Edgar Sánchez Hidalgo",
    CardRules::new_artifact(mana_cost!("{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control. That creature gains first strike until end of turn.",
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
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// EOE 37 — Starfield Shepherd
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static STARFIELD_SHEPHERD: CardRecord = CardRecord::new(
    "Starfield Shepherd",
    "1226e575-aa78-4c68-be1d-6e5c2dc6315b",
    "Marta Nael",
    CardRules::unsupported(),
);

// EOE 38 — Starfighter Pilot
pub(in crate::card::sets) static STARFIGHTER_PILOT: CardRecord = CardRecord::new(
    "Starfighter Pilot",
    "1e571246-aadc-4d1f-a284-9a529e150fe0",
    "Nathaniel Himawan",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Pilot"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, surveil 1. (Look at \
             the top card of your library. You may put it into your \
             graveyard.)",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// EOE 39 — Starport Security
pub(in crate::card::sets) static STARPORT_SECURITY: CardRecord = CardRecord::new(
    "Starport Security",
    "238cb1db-6c41-4fe1-bc34-340048dfde18",
    "Lie Setiawan",
    CardRules::new_artifact_creature(mana_cost!("{W}"), &["Robot", "Soldier"], 1, 1)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{3}{W}, {T}: Tap another target creature. This ability costs \
             {2} less to activate if you control a creature with a +1/+1 \
             counter on it.",
            &[CostDef::Mana(mana_cost!("{3}{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        .with_activation_cost_reduction(
            ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(2),
                otherwise: ValueDef::Constant(0),
            }),
            0,
        )]),
);

// EOE 40 — Sunstar Chaplain
// Audit: unsupported — Needs an activation cost choosing a controlled creature and removing exactly one +1/+1 counter from it; ordinary counter-removal costs name only the ability source.
pub(in crate::card::sets) static SUNSTAR_CHAPLAIN: CardRecord = CardRecord::new(
    "Sunstar Chaplain",
    "83719626-3ff8-4566-9911-88212e753c69",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// EOE 41 — Sunstar Expansionist
pub(in crate::card::sets) static SUNSTAR_EXPANSIONIST: CardRecord = CardRecord::new(
    "Sunstar Expansionist",
    "93b0e6e5-9fb6-4342-9322-1a4cc09a7a76",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if an opponent controls more lands \
             than you, create a Lander token. (It's an artifact with \
             \"{2}, {T}, Sacrifice this token: Search your library for a \
             basic land card, put it onto the battlefield tapped, then \
             shuffle.\")",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                )),
                comparison: ComparisonDef::Greater,
                right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            }),
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            ),
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             gets +1/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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

// EOE 42 — Sunstar Lightsmith
pub(in crate::card::sets) static SUNSTAR_LIGHTSMITH: CardRecord = CardRecord::new(
    "Sunstar Lightsmith",
    "5f60b09d-9814-4a36-a57d-59b0e04c1c2f",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Artificer"], 3, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever you cast your second spell each turn, put a +1/+1 \
             counter on this creature and draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Any,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &TriggerConditionDef::SpellsCastThisTurn {
                quantifier: QuantifierDef::Any,
                player: PlayerRelation::You,
                comparison: ComparisonDef::Equal,
                amount: 2,
            },
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// EOE 43 — Wedgelight Rammer
pub(in crate::card::sets) static WEDGELIGHT_RAMMER: CardRecord = CardRecord::new(
    "Wedgelight Rammer",
    "2cb0984f-dc8b-4bb3-a4fd-8d6d4ae20198",
    "Nadia Hurianova",
    CardRules::new_spacecraft(mana_cost!("{3}{W}"), 3, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, create a 2/2 colorless Robot \
             artifact creature token.",
            EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 9+.)",
        ),
        AbilityDef::static_ability(
            "9+ | Flying, first strike",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 9,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 44 — Weftblade Enhancer
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static WEFTBLADE_ENHANCER: CardRecord = CardRecord::new(
    "Weftblade Enhancer",
    "8d72b00c-5043-4630-949a-fc17eeb962bc",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// EOE 45 — Zealous Display
pub(in crate::card::sets) static ZEALOUS_DISPLAY: CardRecord = CardRecord::new(
    "Zealous Display",
    "f5973fd3-d6bc-48f1-8a44-57d2a6dda228",
    "Chris Rallis",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell(
        "Creatures you control get +2/+0 until end of turn. If it's \
         not your turn, untap those creatures.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
                then: &EffectDef::Untap {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                },
            },
        ]),
    )]),
);

// EOE 46 — Annul (reprint)
const ANNUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ANNUL,
    "4feeebea-aa55-4599-ab5a-4e41a54d0dfd",
    "Carlos Palma Cruchaga",
);

// EOE 47 — Atomic Microsizer
pub(in crate::card::sets) static ATOMIC_MICROSIZER: CardRecord = CardRecord::new(
    "Atomic Microsizer",
    "3554f0c7-ea73-43e9-a061-bbb8ef6abce1",
    "Gabor Szikszai",
    CardRules::new_artifact(mana_cost!("{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                    ]),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever equipped creature attacks, choose up to one target \
                 creature. That creature can't be blocked this turn and has \
                 base power and toughness 1/1 until end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// EOE 48 — Cerebral Download
pub(in crate::card::sets) static CEREBRAL_DOWNLOAD: CardRecord = CardRecord::new(
    "Cerebral Download",
    "4d3b5d73-694c-4f9a-8b4f-d8d8c58c8d65",
    "Antonio José Manzanedo",
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[AbilityDef::spell(
        "Surveil X, where X is the number of artifacts you control. \
         Then draw three cards. (To surveil X, look at the top X cards \
         of your library, then put any number of them into your \
         graveyard and the rest on top of your library in any order.)",
        EffectDef::Sequence(&[
            abilities::surveil(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            abilities::draw_cards(ValueDef::Constant(3)),
        ]),
    )]),
);

// EOE 49 — Cloudsculpt Technician
pub(in crate::card::sets) static CLOUDSCULPT_TECHNICIAN: CardRecord = CardRecord::new(
    "Cloudsculpt Technician",
    "51077a54-15cf-4088-8e84-088d72e8e861",
    "Elizabeth Peiró",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Jellyfish", "Artificer"], 1, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "As long as you control an artifact, this creature gets +1/+0.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                    },
                },
            ),
        ]),
);

// EOE 50 — Codecracker Hound
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static CODECRACKER_HOUND: CardRecord = CardRecord::new(
    "Codecracker Hound",
    "6723b891-6013-4ec6-b439-2233d270dc48",
    "Julia Metzger",
    CardRules::unsupported(),
);

// EOE 51 — Consult the Star Charts
/// "Where X is the number of lands you control", which is the whole reason
/// the card is playable: it looks at more the longer the game goes.
static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// One selection differs from the other only in how many it keeps, so the
/// two are the same workflow twice rather than a count the spell could carry.
macro_rules! consult_choice {
    ($cards:expr, $chosen:expr, $rest:expr) => {
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects($chosen),
            unchosen: Some($rest),
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Binding(ParentBinding),
            exclude: None,
            minimum: $cards,
            maximum: $cards,
            visibility: ChoiceVisibilityDef::Private,
            then: &EffectDef::Sequence(&[
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding($chosen),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    moved: None,
                    then: &EffectDef::None,
                }),
                EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                    input: ObjectSetDef::Binding($rest),
                    randomized: ParentBinding,
                    then: &EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Bottom,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                }),
            ]),
        })
    };
}

pub(in crate::card::sets) static CONSULT_THE_STAR_CHARTS: CardRecord = CardRecord::new(
    "Consult the Star Charts",
    "a16a6555-2e3a-4587-aacd-0307d696b26c",
    "Antonio José Manzanedo",
    // Two mana to dig as deep as your mana base, and four to keep twice as
    // much of what it finds.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{U}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{U} (You may pay an additional {1}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is the number of lands you \
             control. Put one of those cards into your hand. If this spell was kicked, put two \
             of those cards into your hand instead. Put the rest on the bottom of your library \
             in a random order.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    &consult_choice!(
                        2,
                        Binding!("consult_kicked_chosen"),
                        Binding!("consult_kicked_rest")
                    ),
                ),
                otherwise: &abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    &consult_choice!(
                        1,
                        Binding!("consult_normal_chosen"),
                        Binding!("consult_normal_rest")
                    ),
                ),
            },
        ),
    ]),
);

// EOE 52 — Cryogen Relic
pub(in crate::card::sets) static CRYOGEN_RELIC: CardRecord = CardRecord::new(
    "Cryogen Relic",
    "7bfb33b6-e2bf-498f-8c58-ae21a840cf75",
    "Eelis Kyttanen",
// Sacrificing it draws the second card, so the tap-down costs nothing
    // in cards -- only the two mana and the artifact itself.
    CardRules::new_artifact(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "When this artifact enters or leaves the battlefield, draw a card.",
            // One printed sentence with two ways in, so it is one ability
            // watching both zone changes.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
            ]),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{U}, Sacrifice this artifact: Put a stun counter on up to one target tapped creature.",
            &[
                CostDef::Mana(mana_cost!("{1}{U}")),
                CostDef::SacrificeSource,
            ],
            // "Up to one", so it can be sacrificed purely for the leave
            // trigger's card when nothing is tapped.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Stun,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 53 — Cryoshatter
pub(in crate::card::sets) static CRYOSHATTER: CardRecord = CardRecord::new(
    "Cryoshatter",
    "7b62b1e2-9e43-4a66-a647-7e5de2871f2a",
    "Jeremy Wilson",
    // One mana blanks the creature immediately and kills it the moment it
    // is used for anything, which is what makes the -5/-0 half enough.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -5/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-5),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature becomes tapped or is dealt damage, destroy it.",
                // Two ways into one printed sentence, both read against the
                // creature this Aura is on rather than the Aura itself.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                    TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                        recipient: DamageRecipientMatcherDef::Recipients(
                            EffectRecipientDef::AttachedPermanent,
                        ),
                        ..DamageEventMatcherDef::ANY
                    }),
                ]),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    then: None,
                },
            ),
        ]),
);

// EOE 54 — Desculpting Blast
pub(in crate::card::sets) static DESCULPTING_BLAST: CardRecord = CardRecord::new(
    "Desculpting Blast",
    "77fdbf32-b5f4-4346-846d-d8e0e53e6e53",
    "Jeremy Wilson",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand. If it \
         was attacking, create a 1/1 colorless Drone artifact creature \
         token with flying and \"This token can block only creatures \
         with flying.\"",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::TargetMatches {
                slot: TargetIndex::PRIMARY,
                object: ObjectPredicateDef::Attacking,
            },
            then: &EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                EffectDef::create_artifact_creature_token(&["Drone"], &[], 1, 1).with_abilities(&[
                    abilities::flying(),
                    AbilityDef::static_ability(
                        "This token can block only creatures with flying.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                            )),
                        },
                    ),
                ]),
            ]),
            otherwise: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        },
    )]),
);

// EOE 55 — Divert Disaster
pub(in crate::card::sets) static DIVERT_DISASTER: CardRecord = CardRecord::new(
    "Divert Disaster",
    "c5f7d2fe-628b-4493-8281-0e5f91ce5d61",
    "David Álvarez",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {2}. If they \
         do, you create a Lander token. (It's an artifact with \"{2}, \
         {T}, Sacrifice this token: Search your library for a basic \
         land card, put it onto the battlefield tapped, then \
         shuffle.\")",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::PayOr(
            PayOrDef::optional_or(
                &[CostDef::Mana(mana_cost!("{2}"))],
                &EffectDef::create_token(
                    TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                        AbilityDef::activated(
                            "{2}, {T}, Sacrifice this token: Search your library for a \
                             basic land card, put it onto the battlefield tapped, then \
                             shuffle.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
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
                ),
                &EffectDef::counter_target(TargetIndex::PRIMARY),
            )
            .with_payer(PlayerSetDef::One(PlayerRefDef::ControllerOf(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            ))),
        ),
    )]),
);

// EOE 56 — Emissary Escort
pub(in crate::card::sets) static EMISSARY_ESCORT: CardRecord = CardRecord::new(
    "Emissary Escort",
    "b52ba87f-3ac7-4f32-901c-d089df979f94",
    "Igor Grechanyi",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Robot", "Soldier"], 0, 4)
        .with_abilities(&[AbilityDef::static_ability(
            "This creature gets +X/+0, where X is the greatest mana value \
             among other artifacts you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        select: ObjectValueDef::ManaValue,
                        operation: AggregateOperationDef::Maximum,
                    }),
                    ValueDef::Constant(0),
                ),
            },
        )]),
);

// EOE 57 — Gigastorm Titan
// Audit: unsupported — Needs a self spell-cost condition for another spell cast this turn; cost_reduction_value does not evaluate cast-history values or conditional branches over them.
pub(in crate::card::sets) static GIGASTORM_TITAN: CardRecord = CardRecord::new(
    "Gigastorm Titan",
    "abc83e0a-0ae5-4087-a751-058a1ba6a920",
    "Bryan Sola",
    CardRules::unsupported(),
);

// EOE 58 — Illvoi Galeblade
pub(in crate::card::sets) static ILLVOI_GALEBLADE: CardRecord = CardRecord::new(
    "Illvoi Galeblade",
    "769f7d13-a312-4d79-8639-6ae248452448",
    "Nathaniel Himawan",
    CardRules::new_creature(mana_cost!("{U}"), &["Jellyfish", "Warrior"], 1, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::activated(
            "{2}, Sacrifice this creature: Draw a card.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// EOE 59 — Illvoi Infiltrator
pub(in crate::card::sets) static ILLVOI_INFILTRATOR: CardRecord = CardRecord::new(
    "Illvoi Infiltrator",
    "2db4ed41-0426-4d7f-bb43-e43392bed83b",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Jellyfish", "Rogue"], 1, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked if you've cast two or more \
             spells this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::You,
                        spell: ObjectPredicateDef::Any,
                    }),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(2),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw \
             a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// EOE 60 — Illvoi Light Jammer
pub(in crate::card::sets) static ILLVOI_LIGHT_JAMMER: CardRecord = CardRecord::new(
    "Illvoi Light Jammer",
    "efb3d961-543c-4404-b4ed-1cb28ee411b3",
    "David Álvarez",
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control. That creature gains hexproof until end of turn. (It \
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
                EffectDef::Sequence(&[
                    EffectDef::Attach {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(2),
                        ),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// EOE 61 — Illvoi Operative
pub(in crate::card::sets) static ILLVOI_OPERATIVE: CardRecord = CardRecord::new(
    "Illvoi Operative",
    "d0ae9fc7-1802-4806-9996-1f1f458ff6a7",
    "Quintin Gleim",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Jellyfish", "Rogue"], 2, 1).with_abilities(&[
        AbilityDef::triggered_if(
            "Whenever you cast your second spell each turn, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Any,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &TriggerConditionDef::SpellsCastThisTurn {
                quantifier: QuantifierDef::Any,
                player: PlayerRelation::You,
                comparison: ComparisonDef::Equal,
                amount: 2,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 62 — Lost in Space
pub(in crate::card::sets) static LOST_IN_SPACE: CardRecord = CardRecord::new(
    "Lost in Space",
    "6d9d7979-97af-4c85-86f5-1b3704f74e8b",
    "Allen Panakal",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target artifact or creature's owner puts it on their choice \
         of the top or bottom of their library. Surveil 1. (Look at \
         the top card of your library. You may put it into your \
         graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                choices: &[
                    EffectChoiceDef {
                        label: "Top",
                        effect: EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Library,
                            ZonePlacement::Top,
                        ),
                    },
                    EffectChoiceDef {
                        label: "Bottom",
                        effect: EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    },
                ],
            },
            abilities::surveil(ValueDef::Constant(1)),
        ]),
    )]),
);

// EOE 63 — Mechan Assembler
pub(in crate::card::sets) static MECHAN_ASSEMBLER: CardRecord = CardRecord::new(
    "Mechan Assembler",
    "3fd46726-095e-4eb2-a804-eeeb988eee1d",
    "Mirko Failoni",
    CardRules::new_artifact_creature(mana_cost!("{4}{U}"), &["Robot", "Artificer"], 4, 4)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever another artifact you control enters, create a 2/2 \
             colorless Robot artifact creature token. This ability \
             triggers only once each turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2),
        )
        .triggering_at_most(1)]),
);

// EOE 64 — Mechan Navigator
pub(in crate::card::sets) static MECHAN_NAVIGATOR: CardRecord = CardRecord::new(
    "Mechan Navigator",
    "a1fe1d39-42c8-41d0-8bf0-46973e4b07d4",
    "Konstantin Porubov",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Robot", "Pilot"], 2, 1)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever this creature becomes tapped, draw a card, then \
             discard a card.",
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
        )]),
);

// EOE 65 — Mechan Shieldmate
// Audit: unsupported — Needs artifact-entry history for the current turn retained after the artifact leaves or changes controllers; current entry predicates inspect live permanents.
pub(in crate::card::sets) static MECHAN_SHIELDMATE: CardRecord = CardRecord::new(
    "Mechan Shieldmate",
    "745b2119-4d9f-431f-89b9-10ad48b6dc47",
    "Daniel Ljunggren",
    CardRules::unsupported(),
);

// EOE 66 — Mechanozoa
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static MECHANOZOA: CardRecord = CardRecord::new(
    "Mechanozoa",
    "0cb8d8ce-329a-4a97-b3d8-796703ebcb37",
    "Daarken",
    CardRules::unsupported(),
);

// EOE 67 — Mental Modulation
// Audit: unsupported — Needs a self spell-cost condition based on whose turn it is; self-cost evaluation only reads root reduction values and cannot branch on ActivePlayer.
pub(in crate::card::sets) static MENTAL_MODULATION: CardRecord = CardRecord::new(
    "Mental Modulation",
    "0f2d12fc-38a0-42e6-9caa-7c18bfcf0011",
    "Andreia Ugrai",
    CardRules::unsupported(),
);

// EOE 68 — Mm'menon, the Right Hand
// Audit: unsupported — Needs a mana-spending restriction based on the cast origin zone, excluding the hand; existing CastSpell restrictions only match spell characteristics.
pub(in crate::card::sets) static MM_MENON_THE_RIGHT_HAND: CardRecord = CardRecord::new(
    "Mm'menon, the Right Hand",
    "82add0a0-e402-4b31-b101-81c0bf332015",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// EOE 69 — Moonlit Meditation
// Audit: unsupported — Needs an optional replacement for the first token-creation batch of each turn, substituting copies of an attached object; token replacements do not retain this batch ordinal or optional choice.
pub(in crate::card::sets) static MOONLIT_MEDITATION: CardRecord = CardRecord::new(
    "Moonlit Meditation",
    "f2a56007-5bca-4edf-9cc4-5f77a273636c",
    "Liiga Smilshkalne",
    CardRules::unsupported(),
);

// EOE 70 — Mouth of the Storm
pub(in crate::card::sets) static MOUTH_OF_THE_STORM: CardRecord = CardRecord::new(
    "Mouth of the Storm",
    "380f16d6-ad43-4e0d-9645-6abde6248182",
    "Domenico Cava",
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Elemental"], 6, 6).with_abilities(&[
        abilities::flying(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        abilities::enters_trigger(
            "When this creature enters, creatures your opponents control \
             get -3/-0 until your next turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilYourNextTurn,
            },
        ),
    ]),
);

// EOE 71 — Nanoform Sentinel
pub(in crate::card::sets) static NANOFORM_SENTINEL: CardRecord = CardRecord::new(
    "Nanoform Sentinel",
    "3eeae8c3-7939-4c79-92f0-fbdb9c1b71d3",
    "Tianxing Xu",
    CardRules::new_artifact_creature(mana_cost!("{2}{U}"), &["Robot"], 3, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature becomes tapped, untap another target \
             permanent. This ability triggers only once each turn.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
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
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        .triggering_at_most(1),
    ]),
);

// EOE 72 — Quantum Riddler
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static QUANTUM_RIDDLER: CardRecord = CardRecord::new(
    "Quantum Riddler",
    "120be808-ff3b-4fca-96a1-4db6b9825856",
    "Izzy",
    CardRules::unsupported(),
);

// EOE 73 — Scour for Scrap
pub(in crate::card::sets) static SCOUR_FOR_SCRAP: CardRecord = CardRecord::new(
    "Scour for Scrap",
    "517d1b00-7ec4-489a-ac52-657da24a6379",
    "Filip Burburan",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Search your library for an artifact card, reveal it, put it \
                 into your hand, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target artifact card from your graveyard to your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
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
            ),
        ],
    )
    .with_mode_selection(1, 2, false)]),
);

// EOE 74 — Selfcraft Mechan
// Audit: unsupported — Needs a sacrifice-created reflexive trigger that chooses its target after the sacrifice and survives the source leaving; OptionalEffectTaken only observes battlefield listeners.
pub(in crate::card::sets) static SELFCRAFT_MECHAN: CardRecord = CardRecord::new(
    "Selfcraft Mechan",
    "5b2de056-5c27-44d6-871d-909411bd52dd",
    "Milivoj Ćeran",
    CardRules::unsupported(),
);

// EOE 75 — Sinister Cryologist
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static SINISTER_CRYOLOGIST: CardRecord = CardRecord::new(
    "Sinister Cryologist",
    "e8fbe740-05ec-4ced-bb9d-3084c8c2b631",
    "Domenico Cava",
    CardRules::unsupported(),
);

// EOE 76 — Specimen Freighter
pub(in crate::card::sets) static SPECIMEN_FREIGHTER: CardRecord = CardRecord::new(
    "Specimen Freighter",
    "b862a9f8-2361-4220-99bd-ae2530905195",
    "Sergey Glushakov",
    CardRules::new_spacecraft(mana_cost!("{5}{U}"), 4, 7).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Spacecraft enters, return up to two target \
             non-Spacecraft creatures to their owners' hands.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Spacecraft",
                        ))),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 9+.)",
        ),
        AbilityDef::static_ability(
            "9+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 9,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
        AbilityDef::static_ability(
            "9+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 9,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::triggered(
                            "Whenever this Spacecraft attacks, defending player mills four \
                             cards.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                            EffectDef::Mill {
                                player: EffectRecipientDef::players(PlayerSetDef::Related(
                                    PlayerRelation::DefendingPlayer,
                                )),
                                amount: ValueDef::Constant(4),
                            },
                        ),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 77 — Starbreach Whale
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static STARBREACH_WHALE: CardRecord = CardRecord::new(
    "Starbreach Whale",
    "8a1a0476-7145-4493-97e5-4fc05c85e476",
    "Sam Burley",
    CardRules::unsupported(),
);

// EOE 78 — Starfield Vocalist
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static STARFIELD_VOCALIST: CardRecord = CardRecord::new(
    "Starfield Vocalist",
    "deca0b2a-e7f3-444a-883d-7c41dd62c9cc",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// EOE 79 — Starwinder
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static STARWINDER: CardRecord = CardRecord::new(
    "Starwinder",
    "27d1a010-5790-4b35-9fdc-0e366eed021d",
    "Devin Elle Kurtz",
    CardRules::unsupported(),
);

// EOE 80 — Steelswarm Operator
pub(in crate::card::sets) static STEELSWARM_OPERATOR: CardRecord = CardRecord::new(
    "Steelswarm Operator",
    "ca468b86-f31a-4cd7-a574-eb984bc4bc3e",
    "Cristi Balanescu",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Robot", "Soldier"], 1, 1)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_mana(
                "{T}: Add {U}. Spend this mana only to cast an artifact spell.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue).with_restrictions(&[
                    ManaRestrictionDef::CastSpell(ObjectPredicateDef::HasType(CardType::Artifact)),
                ])),
            ),
            AbilityDef::activated_mana(
                "{T}: Add {U}{U}. Spend this mana only to activate abilities \
                 of artifact sources.",
                &[CostDef::TapSource],
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Blue)
                        .with_amount(2)
                        .with_restrictions(&[ManaRestrictionDef::ActivateAbility(
                            ObjectPredicateDef::HasType(CardType::Artifact),
                        )]),
                ),
            ),
        ]),
);

// EOE 81 — Synthesizer Labship
pub(in crate::card::sets) static SYNTHESIZER_LABSHIP: CardRecord = CardRecord::new(
    "Synthesizer Labship",
    "fba6332c-acba-43f9-877c-ca6c5328aae9",
    "Adrián Rodríguez Pérez",
    CardRules::new_spacecraft(mana_cost!("{U}"), 4, 4).with_abilities(&[
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 9+.)",
        ),
        AbilityDef::static_ability(
            "2+ | At the beginning of combat on your turn, up to one other \
             target artifact you control becomes an artifact creature with \
             base power and toughness 2/2 and gains flying until end of \
             turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::triggered_with_targets(
                            "2+ | At the beginning of combat on your turn, up to one other \
                             target artifact you control becomes an artifact creature with \
                             base power and toughness 2/2 and gains flying until end of \
                             turn.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::BeginningOfCombat,
                                player: PlayerRelation::You,
                            },
                            &[AbilityTargetDef::up_to(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                    ]),
                                    zones: &[ZoneKind::Battlefield],
                                    controller: Some(PlayerRelation::You),
                                    owner: None,
                                },
                                1,
                            )],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                effect: AppliedEffectDef::Composite(&[
                                    AppliedEffectDef::add_card_types(
                                        CardTypeSet::single(CardType::Artifact)
                                            .union(CardTypeSet::single(CardType::Creature)),
                                    ),
                                    AppliedEffectDef::set_base_power_toughness(
                                        ValueDef::Constant(2),
                                        ValueDef::Constant(2),
                                    ),
                                    AppliedEffectDef::add_ability(&abilities::flying()),
                                ]),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        ),
                    )]),
                },
            },
        ),
        AbilityDef::static_ability(
            "9+ | Flying, vigilance",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 9,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 82 — Tractor Beam
pub(in crate::card::sets) static TRACTOR_BEAM: CardRecord = CardRecord::new(
    "Tractor Beam",
    "efc96d54-d6e1-49b6-b4c2-70b997776548",
    "Sergey Glushakov",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature or Spacecraft",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                    ]),
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted permanent.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "You control enchanted permanent.",
                EffectDef::gain_control(
                    EffectRecipientDef::AttachedPermanent,
                    PlayerRefDef::EffectController,
                    ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                ),
            ),
            AbilityDef::static_ability(
                "Enchanted permanent doesn't untap during its controller's \
                 untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// EOE 83 — Unravel
// Audit: unsupported — Needs the amount of mana actually spent on a targeted spell, captured before countering it; current target values expose mana value but not its payment amount.
pub(in crate::card::sets) static UNRAVEL: CardRecord = CardRecord::new(
    "Unravel",
    "e8978214-c853-453d-872d-af56bdaaa3d7",
    "Josh Hass",
    CardRules::unsupported(),
);

// EOE 84 — Uthros Psionicist
// Audit: unsupported — Needs a shared spell-cost condition based on the ordinal of the spell being cast this turn; current external cost conditions only expose Always and TargetsSource.
pub(in crate::card::sets) static UTHROS_PSIONICIST: CardRecord = CardRecord::new(
    "Uthros Psionicist",
    "e23cc5fd-afe4-480c-8858-ed80a082584e",
    "Inkognit",
    CardRules::unsupported(),
);

// EOE 85 — Uthros Scanship
pub(in crate::card::sets) static UTHROS_SCANSHIP: CardRecord = CardRecord::new(
    "Uthros Scanship",
    "1f93887f-35c5-472f-83d0-54227b3bd1d2",
    "Sergey Glushakov",
    CardRules::new_spacecraft(mana_cost!("{3}{U}"), 4, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, draw two cards, then discard a card.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(2)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 8+.)",
        ),
        AbilityDef::static_ability(
            "8+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 86 — Weftwalking
// Audit: unsupported — Needs a player-wide free-cast alternative restricted to the first spell on that player's own turn; existing alternative permissions do not enforce that cast-history condition.
pub(in crate::card::sets) static WEFTWALKING: CardRecord = CardRecord::new(
    "Weftwalking",
    "39d48ddd-4529-4284-9da3-5272ad362b9b",
    "Rovina Cai",
    CardRules::unsupported(),
);

// EOE 87 — Alpharael, Stonechosen
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static ALPHARAEL_STONECHOSEN: CardRecord = CardRecord::new(
    "Alpharael, Stonechosen",
    "33063d26-37f7-4e35-8da2-5770dfabdc41",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// EOE 88 — Archenemy's Charm
pub(in crate::card::sets) static ARCHENEMY_S_CHARM: CardRecord = CardRecord::new(
    "Archenemy's Charm",
    "dcde5f27-e2f0-4d2a-afa3-f300896ec4b1",
    "Brigitte Roka & Clifton Stommel",
    CardRules::new_instant(mana_cost!("{B}{B}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Exile target creature or planeswalker.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell_with_targets(
                "Return one or two target creature and/or planeswalker cards \
                 from your graveyard to your hand.",
                &[AbilityTargetDef {
                    minimum: 1,
                    ..AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                        2,
                    )
                }],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell_with_targets(
                "Put two +1/+1 counters on target creature you control. It \
                 gains lifelink until end of turn.",
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
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ],
    )]),
);

// EOE 89 — Beamsaw Prospector
pub(in crate::card::sets) static BEAMSAW_PROSPECTOR: CardRecord = CardRecord::new(
    "Beamsaw Prospector",
    "6f717a3f-c6db-4e8d-8b62-6361ab33d000",
    "Aurore Folny",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Artificer"], 2, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create a Lander token. (It's an \
             artifact with \"{2}, {T}, Sacrifice this token: Search your \
             library for a basic land card, put it onto the battlefield \
             tapped, then shuffle.\")",
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            ),
        ),
    ]),
);

// EOE 90 — Blade of the Swarm
// Audit: unsupported — Needs an exile target predicate identifying cards with a Warp alternative-cost ability; current object predicates cannot inspect that alternative-cast ability kind.
pub(in crate::card::sets) static BLADE_OF_THE_SWARM: CardRecord = CardRecord::new(
    "Blade of the Swarm",
    "b157330a-2652-4ed9-b8fa-8e72b4eda15c",
    "Nino Is",
    CardRules::unsupported(),
);

// EOE 91 — Chorale of the Void
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static CHORALE_OF_THE_VOID: CardRecord = CardRecord::new(
    "Chorale of the Void",
    "7389fe88-f6ff-4497-a037-9ca283fb89e3",
    "Alix Branwyn",
    CardRules::unsupported(),
);

// EOE 92 — Comet Crawler
pub(in crate::card::sets) static COMET_CRAWLER: CardRecord = CardRecord::new(
    "Comet Crawler",
    "becca990-ab2e-4aa4-be7d-293ec727cb08",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Insect", "Horror"], 2, 3).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever this creature attacks, you may sacrifice another \
             creature or artifact. If you do, this creature gets +2/+0 \
             until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )),
        ),
    ]),
);

// EOE 93 — Dark Endurance
// Audit: unsupported — Needs a self spell-cost reduction that inspects whether its selected target is blocking; current self-cost evaluation does not read selected targets.
pub(in crate::card::sets) static DARK_ENDURANCE: CardRecord = CardRecord::new(
    "Dark Endurance",
    "fac87b49-a0cd-42d5-b30a-efc6d5526fc3",
    "Leon Tukker",
    CardRules::unsupported(),
);

// EOE 94 — Decode Transmissions
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static DECODE_TRANSMISSIONS: CardRecord = CardRecord::new(
    "Decode Transmissions",
    "cecb4936-14ca-49f9-b209-6519cab54b30",
    "Josh Hass",
    CardRules::unsupported(),
);

// EOE 95 — Depressurize
pub(in crate::card::sets) static DEPRESSURIZE: CardRecord = CardRecord::new(
    "Depressurize",
    "25520d5a-1a83-42cc-8ace-8b1156019d64",
    "Danny Schwartz",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -3/-0 until end of turn. Then if that \
         creature's power is 0 or less, destroy it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::PowerLessThan(ValueDef::Constant(1)),
                },
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ]),
    )]),
);

// EOE 96 — Dubious Delicacy
pub(in crate::card::sets) static DUBIOUS_DELICACY: CardRecord = CardRecord::new(
    "Dubious Delicacy",
    "153265b4-0ca4-4245-9226-dd1a083ec91c",
    "Tianxing Xu",
    CardRules::new_artifact(mana_cost!("{2}{B}"))
        .with_subtypes(&["Food"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this artifact enters, up to one target creature gets \
                 -3/-3 until end of turn.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-3),
                        ValueDef::Constant(-3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this artifact: You gain 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::activated_with_targets(
                "{2}, {T}, Sacrifice this artifact: Target opponent loses 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// EOE 97 — Elegy Acolyte
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static ELEGY_ACOLYTE: CardRecord = CardRecord::new(
    "Elegy Acolyte",
    "c69ed7c7-1f49-4299-b3e6-75150258ac59",
    "Diana Franco",
    CardRules::unsupported(),
);

// EOE 98 — Embrace Oblivion
pub(in crate::card::sets) static EMBRACE_OBLIVION: CardRecord = CardRecord::new(
    "Embrace Oblivion",
    "3754fc20-7aaa-437d-97df-d6cf1c29586c",
    "Andreas Zafiratos",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "As an additional cost to cast this spell, sacrifice an \
         artifact or creature.\nDestroy target creature or Spacecraft.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )
    .with_spell_additional_cost(&CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
    ])))]),
);

// EOE 99 — Entropic Battlecruiser
// Audit: unsupported — Needs individual-card discard triggers, including separate trigger objects for a multi-card discard; DiscardedCards emits one event for the entire batch and its amount, so multiplying life loss would change countering and response opportunities.
pub(in crate::card::sets) static ENTROPIC_BATTLECRUISER: CardRecord = CardRecord::new(
    "Entropic Battlecruiser",
    "cc59796b-9025-44b6-a188-cf6684ebffb9",
    "Josiah \"Jo\" Cameron",
    CardRules::unsupported(),
);

// EOE 100 — Faller's Faithful
pub(in crate::card::sets) static FALLER_S_FAITHFUL: CardRecord = CardRecord::new(
    "Faller's Faithful",
    "cbb1b46f-72e0-4c2f-8012-74529bd29a0d",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Wizard"], 3, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy up to one other target \
             creature. If that creature wasn't dealt damage this turn, its \
             controller draws two cards.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::WasDealtDamageThisTurn,
                },
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                otherwise: &EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                    abilities::draw_cards(ValueDef::Constant(2)),
                ]),
            },
        ),
    ]),
);

// EOE 101 — Fell Gravship
pub(in crate::card::sets) static FELL_GRAVSHIP: CardRecord = CardRecord::new(
    "Fell Gravship",
    "e94b130d-3547-43c5-a319-5ebc571c2e2d",
    "David Álvarez",
    CardRules::new_spacecraft(mana_cost!("{2}{B}"), 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, mill three cards, then return a \
             creature or Spacecraft card from your graveyard to your hand.",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
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
            ]),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 8+.)",
        ),
        AbilityDef::static_ability(
            "8+ | Flying, lifelink",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 102 — Gravblade Heavy
pub(in crate::card::sets) static GRAVBLADE_HEAVY: CardRecord = CardRecord::new(
    "Gravblade Heavy",
    "b3872341-d711-407b-85e4-46ccb99988e1",
    "Andrew Mar",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Soldier"], 3, 4).with_abilities(&[
        AbilityDef::static_ability(
            "As long as you control an artifact, this creature gets +1/+0 \
             and has deathtouch.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 103 — Gravkill
pub(in crate::card::sets) static GRAVKILL: CardRecord = CardRecord::new(
    "Gravkill",
    "cfa6c57f-a193-48cc-9764-d8348548a111",
    "Dominik Mayer",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature or Spacecraft.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
            ]),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )]),
);

// EOE 104 — Gravpack Monoist
pub(in crate::card::sets) static GRAVPACK_MONOIST: CardRecord = CardRecord::new(
    "Gravpack Monoist",
    "2a968f01-36ef-4cf6-b1db-630c9cde2064",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Scout"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, create a tapped 2/2 colorless Robot \
             artifact creature token.",
            EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2).entering_tapped(),
        ),
    ]),
);

// EOE 105 — Hullcarver
pub(in crate::card::sets) static HULLCARVER: CardRecord = CardRecord::new(
    "Hullcarver",
    "817b5b18-beb5-48c8-aa45-0515ff9ca5da",
    "Michal Ivan",
    CardRules::new_artifact_creature(mana_cost!("{B}"), &["Robot", "Assassin"], 1, 1)
        .with_abilities(&[abilities::deathtouch()]),
);

// EOE 106 — Hylderblade
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static HYLDERBLADE: CardRecord = CardRecord::new(
    "Hylderblade",
    "ac80cf18-0707-4358-bdd4-0c2b90d0a1d9",
    "Viko Menezes",
    CardRules::unsupported(),
);

// EOE 107 — Hymn of the Faller
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static HYMN_OF_THE_FALLER: CardRecord = CardRecord::new(
    "Hymn of the Faller",
    "e468d528-6cf0-4563-9da2-e388ba56cb9d",
    "Danny Schwartz",
    CardRules::unsupported(),
);

// EOE 108 — Insatiable Skittermaw
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static INSATIABLE_SKITTERMAW: CardRecord = CardRecord::new(
    "Insatiable Skittermaw",
    "e9d30cca-ea33-418f-bba3-5103f1dbd751",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// EOE 109 — Lightless Evangel
pub(in crate::card::sets) static LIGHTLESS_EVANGEL: CardRecord = CardRecord::new(
    "Lightless Evangel",
    "6860556a-6c34-4c41-89ea-f0bc495a159c",
    "Viko Menezes",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Cleric"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you sacrifice another creature or artifact, put a \
             +1/+1 counter on this creature.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
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

// EOE 110 — Monoist Circuit-Feeder
pub(in crate::card::sets) static MONOIST_CIRCUIT_FEEDER: CardRecord = CardRecord::new(
    "Monoist Circuit-Feeder",
    "957ae7aa-98d6-402a-9b20-e3e5b7e8dfe3",
    "Quintin Gleim",
    CardRules::new_artifact_creature(mana_cost!("{4}{B}{B}"), &["Nautilus"], 4, 4).with_abilities(
        &[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, until end of turn, target creature \
                 you control gets +X/+0 and target creature an opponent \
                 controls gets -0/-X, where X is the number of artifacts you \
                 control.",
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
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex(1)),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Scaled(&ScaledValueDef {
                                value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                factor: -1,
                            }),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ],
    ),
);

// EOE 111 — Monoist Sentry
pub(in crate::card::sets) static MONOIST_SENTRY: CardRecord = CardRecord::new(
    "Monoist Sentry",
    "acc503e2-5c3a-4200-beb0-7d193d6c869e",
    "Nino Is",
    CardRules::new_artifact_creature(mana_cost!("{B}"), &["Robot"], 4, 1)
        .with_abilities(&[abilities::defender()]),
);

// EOE 112 — Perigee Beckoner
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static PERIGEE_BECKONER: CardRecord = CardRecord::new(
    "Perigee Beckoner",
    "f3666a08-d449-496f-969a-bf21d4afbd77",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// EOE 113 — Requiem Monolith
pub(in crate::card::sets) static REQUIEM_MONOLITH: CardRecord = CardRecord::new(
    "Requiem Monolith",
    "837d710a-652f-4c60-a52d-d786231160a4",
    "Warren Mahy",
    CardRules::new_artifact(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Until end of turn, target creature gains \"Whenever this \
             creature is dealt damage, you draw that many cards and lose \
             that much life.\" That creature's controller may have this \
             artifact deal 1 damage to it. Activate only as a sorcery.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature is dealt damage, you draw that many \
                         cards and lose that much life.",
                        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                            recipient: DamageRecipientMatcherDef::Recipients(
                                EffectRecipientDef::Source,
                            ),
                            ..DamageEventMatcherDef::ANY
                        }),
                        EffectDef::Sequence(&[
                            abilities::draw_cards(ValueDef::TriggerEventAmount),
                            EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::TriggerEventAmount,
                            },
                        ]),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::May {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    effect: &EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                },
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// EOE 114 — Scrounge for Eternity
pub(in crate::card::sets) static SCROUNGE_FOR_ETERNITY: CardRecord = CardRecord::new(
    "Scrounge for Eternity",
    "baeff907-017e-4dee-aae1-19dfaab309de",
    "Konstantin Porubov",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "As an additional cost to cast this spell, sacrifice an \
         artifact or creature.\nReturn target creature or Spacecraft \
         card with mana value 5 or less from your graveyard to the \
         battlefield. Then create a Lander token. (It's an artifact \
         with \"{2}, {T}, Sacrifice this token: Search your library \
         for a basic land card, put it onto the battlefield tapped, \
         then shuffle.\")",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                    ]),
                    ObjectPredicateDef::ManaValueAtMost(5),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            ),
        ]),
    )
    .with_spell_additional_cost(&CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Creature),
    ])))]),
);

// EOE 115 — Sothera, the Supervoid
pub(in crate::card::sets) static SOTHERA_THE_SUPERVOID: CardRecord = CardRecord::new(
    "Sothera, the Supervoid",
    "e99d6fc0-dcf2-4b25-81c2-02c230a36246",
    "Dominik Mayer",
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever a creature you control dies, each opponent chooses a \
                 creature they control and exiles it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::Opponent,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::ExileLinkedToSource {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("chosen"),
                        )),
                        face_down: false,
                        until_source_leaves: false,
                        then: None,
                    },
                }),
            ),
            AbilityDef::triggered_if(
                "At the beginning of your end step, if a player controls no \
                 creatures, sacrifice Sothera, then put a creature card exiled \
                 with it onto the battlefield under your control with two \
                 additional +1/+1 counters on it.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AnyOf(&[
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                ]),
                EffectDef::Sequence(&[
                    EffectDef::sacrifice(EffectRecipientDef::Source),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("returning")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::LinkedExiles,
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                        },
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::WithBattlefieldArrival {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("returning"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                            arrival: BattlefieldArrivalDef {
                                controller: Some(PlayerRelation::You),
                                modifications: &[BattlefieldEntryModificationDef::AddCounters {
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: 2,
                                }],
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                    }),
                ]),
            ),
        ]),
);

// EOE 116 — Sunset Saboteur
pub(in crate::card::sets) static SUNSET_SABOTEUR: CardRecord = CardRecord::new(
    "Sunset Saboteur",
    "396bca07-82ba-49b7-b79e-7784b3a06d48",
    "Mirko Failoni",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 4, 1).with_abilities(&[
        abilities::menace(),
        abilities::ward(
            &[CostDef::discard(ObjectPredicateDef::Any)],
            "Ward—Discard a card.",
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, put a +1/+1 counter on target \
             creature an opponent controls.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 117 — Susurian Dirgecraft
pub(in crate::card::sets) static SUSURIAN_DIRGECRAFT: CardRecord = CardRecord::new(
    "Susurian Dirgecraft",
    "b67cdb6e-9a3b-4887-924d-318faa3c443d",
    "Mark Poole",
    CardRules::new_spacecraft(mana_cost!("{4}{B}"), 4, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, each opponent sacrifices a \
             nontoken creature of their choice.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::Opponent,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    crate::Binding!("chosen"),
                ))),
            }),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 7+.)",
        ),
        AbilityDef::static_ability(
            "7+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 118 — Susurian Voidborn
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static SUSURIAN_VOIDBORN: CardRecord = CardRecord::new(
    "Susurian Voidborn",
    "beb97e7b-0ae7-4b08-9ceb-6a7f825bcd49",
    "Jehan Choo",
    CardRules::unsupported(),
);

// EOE 119 — Swarm Culler
pub(in crate::card::sets) static SWARM_CULLER: CardRecord = CardRecord::new(
    "Swarm Culler",
    "2a8f583c-88b6-4797-b93e-3086845fc326",
    "April Prime",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Insect", "Warrior"], 2, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, you may sacrifice \
             another creature or artifact. If you do, draw a card.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// EOE 120 — Temporal Intervention
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static TEMPORAL_INTERVENTION: CardRecord = CardRecord::new(
    "Temporal Intervention",
    "79f9525a-4cb7-411e-b7b5-2113e93bcbc3",
    "Chris Rallis",
    CardRules::unsupported(),
);

// EOE 121 — Timeline Culler
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static TIMELINE_CULLER: CardRecord = CardRecord::new(
    "Timeline Culler",
    "33410410-72f2-49c4-9e63-a72202cd075a",
    "Alfonso Santano",
    CardRules::unsupported(),
);

// EOE 122 — Tragic Trajectory
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static TRAGIC_TRAJECTORY: CardRecord = CardRecord::new(
    "Tragic Trajectory",
    "78c8bc60-1378-4028-8ab1-3286e459bffb",
    "Ovidio Cartagena",
    CardRules::unsupported(),
);

// EOE 123 — Umbral Collar Zealot
pub(in crate::card::sets) static UMBRAL_COLLAR_ZEALOT: CardRecord = CardRecord::new(
    "Umbral Collar Zealot",
    "bbcc1d84-9772-475d-924a-75bb54c9bc20",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Cleric"], 3, 2).with_abilities(&[
        AbilityDef::activated(
            "Sacrifice another creature or artifact: Surveil 1. (Look at \
             the top card of your library. You may put it into your \
             graveyard.)",
            &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]))],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// EOE 124 — Virus Beetle (reprint)
const VIRUS_BEETLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_neo::VIRUS_BEETLE,
    "e96c986c-684c-4546-a9c9-b6b903bda101",
    "Leesha Hannigan",
);

// EOE 125 — Voidforged Titan
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static VOIDFORGED_TITAN: CardRecord = CardRecord::new(
    "Voidforged Titan",
    "6119c016-01b2-44f3-9550-6988324c1d1f",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// EOE 126 — Vote Out
pub(in crate::card::sets) static VOTE_OUT: CardRecord = CardRecord::new(
    "Vote Out",
    "4f50bb47-cc4d-4b81-b5f1-817ca8744d12",
    "David Álvarez",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell_with_targets(
            "Destroy target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// EOE 127 — Xu-Ifit, Osteoharmonist
// Audit: unsupported — Needs a return instruction establishing the extra Skeleton subtype and removed abilities as the creature enters, before entry replacement and trigger matching (CR 611.2e).
pub(in crate::card::sets) static XU_IFIT_OSTEOHARMONIST: CardRecord = CardRecord::new(
    "Xu-Ifit, Osteoharmonist",
    "c0838f25-2193-4305-b73a-bf0c0bb4981a",
    "Michal Ivan",
    CardRules::unsupported(),
);

// EOE 128 — Zero Point Ballad
pub(in crate::card::sets) static ZERO_POINT_BALLAD: CardRecord = CardRecord::new(
    "Zero Point Ballad",
    "59cf9f4d-54cd-4cda-9726-65e16100ab46",
    "David Astruga",
    CardRules::new_sorcery(mana_cost!("{X}{B}")).with_abilities(&[AbilityDef::spell(
        "Destroy all creatures with toughness X or less. You lose X \
         life. If X is 6 or more, return a creature card put into a \
         graveyard this way to the battlefield under your control.",
        EffectDef::Destroy {
            object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::ToughnessGreaterThan(
                        ValueDef::ChosenX,
                    )),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ))),
            then: Some(DestroyFollowUpDef {
                binding: crate::Binding!("destroyed"),
                effect: &EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::ChosenX,
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::ChosenX,
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(6),
                        }),
                        then: &EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Binding(crate::Binding!("destroyed")),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                        }),
                    },
                ]),
            }),
        },
    )]),
);

// EOE 129 — Bombard (reprint)
const BOMBARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::BOMBARD,
    "29492df8-3077-4ded-a6e2-51d3bd4669b4",
    "Diego Gisbert",
);

// EOE 130 — Cut Propulsion
pub(in crate::card::sets) static CUT_PROPULSION: CardRecord = CardRecord::new(
    "Cut Propulsion",
    "d96f1c41-3d11-48a8-b962-db46a2d054de",
    "Andrea Piparo",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature deals damage to itself equal to its power. If \
         that creature has flying, it deals twice that much damage to \
         itself instead.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::TargetMatches {
                slot: TargetIndex::PRIMARY,
                object: ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            },
            then: &EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Scaled(&ScaledValueDef {
                    value: ValueDef::TargetPower(TargetIndex::PRIMARY),
                    factor: 2,
                }),
            ),
            otherwise: &EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        },
    )]),
);

// EOE 131 — Debris Field Crusher
pub(in crate::card::sets) static DEBRIS_FIELD_CRUSHER: CardRecord = CardRecord::new(
    "Debris Field Crusher",
    "ea713f35-6442-4388-8839-2714374fb4b6",
    "David Álvarez",
    CardRules::new_spacecraft(mana_cost!("{4}{R}"), 1, 5).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Spacecraft enters, it deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 8+.)",
        ),
        AbilityDef::static_ability(
            "8+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
        AbilityDef::static_ability(
            "8+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::activated(
                            "{1}{R}: This Spacecraft gets +2/+0 until end of turn.",
                            &[CostDef::Mana(mana_cost!("{1}{R}"))],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(2),
                                    ValueDef::Constant(0),
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        ),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 132 — Devastating Onslaught
pub(in crate::card::sets) static DEVASTATING_ONSLAUGHT: CardRecord = CardRecord::new(
    "Devastating Onslaught",
    "fc779971-24e2-46a8-86be-16f0b244a3d2",
    "Chris Seaman",
    CardRules::new_sorcery(mana_cost!("{X}{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Create X tokens that are copies of target artifact or \
             creature you control. Those tokens gain haste until end of \
             turn. Sacrifice them at the beginning of the next end step.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::create_token_from_copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE,
            })
            .with_count(ValueDef::ChosenX)
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("created"),
                then: &EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("created"),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "Sacrifice those tokens.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("created"),
                        ))),
                    ))),
                ]),
            }),
        ),
    ]),
);

// EOE 133 — Drill Too Deep
pub(in crate::card::sets) static DRILL_TOO_DEEP: CardRecord = CardRecord::new(
    "Drill Too Deep",
    "b9d3b6e8-47c8-49e3-b204-8b659e127bde",
    "Bartek Fedyczak",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Put five charge counters on target Spacecraft or Planet you \
                 control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Planet")),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(5),
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )]),
);

// EOE 134 — Frontline War-Rager
pub(in crate::card::sets) static FRONTLINE_WAR_RAGER: CardRecord = CardRecord::new(
    "Frontline War-Rager",
    "fa232943-818b-4944-be60-2d80c806bf62",
    "Jason Rainville",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Kavu", "Soldier"], 2, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control two or more \
             tapped creatures, put a +1/+1 counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 135 — Full Bore
// Audit: unsupported — Needs a predicate reading the Warp alternative-cost choice of a targeted permanent; SourceCastWith only examines the resolving ability source.
pub(in crate::card::sets) static FULL_BORE: CardRecord = CardRecord::new(
    "Full Bore",
    "cfee64ef-d22d-4024-bc65-59cbd1731d1c",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// EOE 136 — Galvanizing Sawship
pub(in crate::card::sets) static GALVANIZING_SAWSHIP: CardRecord = CardRecord::new(
    "Galvanizing Sawship",
    "5bbce9fb-401f-4e78-acd5-9d3b506687fd",
    "Constantin Marin",
    CardRules::new_spacecraft(mana_cost!("{5}{R}"), 6, 5).with_abilities(&[
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 3+.)",
        ),
        AbilityDef::static_ability(
            "3+ | Flying, haste",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 137 — Invasive Maneuvers
pub(in crate::card::sets) static INVASIVE_MANEUVERS: CardRecord = CardRecord::new(
    "Invasive Maneuvers",
    "b68010e2-7810-43cb-a52e-e73b1834e54e",
    "Leon Tukker",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Invasive Maneuvers deals 3 damage to target creature. It \
         deals 5 damage instead if you control a Spacecraft.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            then: &EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
            otherwise: &EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        },
    )]),
);

// EOE 138 — Kav Landseeker
// Audit: unsupported — Needs a delayed trigger at the end step of the controller's next turn, skipping the current turn; installed step triggers do not have that earliest-turn restriction.
pub(in crate::card::sets) static KAV_LANDSEEKER: CardRecord = CardRecord::new(
    "Kav Landseeker",
    "7a5a7e89-50e3-43cb-af93-d7d80a630c11",
    "Karl Kopinski",
    CardRules::unsupported(),
);

// EOE 139 — Kavaron Harrier
pub(in crate::card::sets) static KAVARON_HARRIER: CardRecord = CardRecord::new(
    "Kavaron Harrier",
    "1da2a740-e4ff-4661-ba57-39b15c58e26e",
    "Hardy Fowler",
    CardRules::new_artifact_creature(mana_cost!("{R}"), &["Robot", "Soldier"], 2, 1)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever this creature attacks, you may pay {2}. If you do, \
             create a 2/2 colorless Robot artifact creature token that's \
             tapped and attacking. Sacrifice that token at end of combat.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{2}"))],
                &EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2)
                    .entering_tapped()
                    .entering_attacking()
                    .with_created_tokens(CreatedTokensDef {
                        binding: crate::Binding!("created"),
                        then: &EffectDef::Sequence(&[EffectDef::InstallTrigger(
                            InstalledTriggerDef::once(&AbilityDef::triggered(
                                "Sacrifice those tokens.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::EndOfCombat,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::sacrifice(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("created")),
                                )),
                            )),
                        )]),
                    }),
            )),
        )]),
);

// EOE 140 — Kavaron Skywarden
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static KAVARON_SKYWARDEN: CardRecord = CardRecord::new(
    "Kavaron Skywarden",
    "617038a8-0544-4d0e-8ff1-c786e60ecd59",
    "Diana Franco",
    CardRules::unsupported(),
);

// EOE 141 — Kavaron Turbodrone
pub(in crate::card::sets) static KAVARON_TURBODRONE: CardRecord = CardRecord::new(
    "Kavaron Turbodrone",
    "f5e92cdd-75df-499e-94f7-22287f1000b3",
    "Leesha Hannigan",
    CardRules::new_artifact_creature(mana_cost!("{2}{R}"), &["Robot", "Scout"], 2, 3)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{T}: Target creature you control gets +1/+1 and gains haste \
             until end of turn. Activate only as a sorcery.",
            &[CostDef::TapSource],
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
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)]),
);

// EOE 142 — Lithobraking
// Audit: unsupported — Needs a spell-created reflexive trigger after the optional sacrifice; OptionalEffectTaken only observes battlefield sources and does not create a surviving spell continuation.
pub(in crate::card::sets) static LITHOBRAKING: CardRecord = CardRecord::new(
    "Lithobraking",
    "5a22024c-2c0f-4487-98d1-ee89cf3dba89",
    "Andrew Mar",
    CardRules::unsupported(),
);

// EOE 143 — Melded Moxite
pub(in crate::card::sets) static MELDED_MOXITE: CardRecord = CardRecord::new(
    "Melded Moxite",
    "474c067f-eb24-4ae8-b4c0-f6f8e24cdb2a",
    "Alexandr Leskinen",
    CardRules::new_artifact(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, you may discard a card. If you do, \
             draw two cards.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::discard(ObjectPredicateDef::Any)],
                &abilities::draw_cards(ValueDef::Constant(2)),
            )),
        ),
        AbilityDef::activated(
            "{3}, Sacrifice this artifact: Create a tapped 2/2 colorless \
             Robot artifact creature token.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::SacrificeSource],
            EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2).entering_tapped(),
        ),
    ]),
);

// EOE 144 — Memorial Team Leader
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static MEMORIAL_TEAM_LEADER: CardRecord = CardRecord::new(
    "Memorial Team Leader",
    "3ddc240a-62df-4773-98d7-48a9adaf1846",
    "Andrew Mar",
    CardRules::unsupported(),
);

// EOE 145 — Memorial Vault
pub(in crate::card::sets) static MEMORIAL_VAULT: CardRecord = CardRecord::new(
    "Memorial Vault",
    "a14ecd13-325a-4555-b1dd-d0d0d0826031",
    "Javier Charro",
    CardRules::new_artifact(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::activated(
        "{T}, Sacrifice another artifact: Exile the top X cards of \
         your library, where X is one plus the mana value of the \
         sacrificed artifact. You may play those cards this turn.",
        &[
            CostDef::TapSource,
            CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ])),
        ],
        EffectDef::BindObjects(BindObjectsDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(1),
                    ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                            AdditionalCostObjectIndex::PRIMARY,
                        )),
                        select: ObjectValueDef::ManaValue,
                        operation: AggregateOperationDef::Sum,
                    }),
                )),
            },
            binding: crate::Binding!("top"),
            then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("top"))),
            },
        }),
    )]),
);

// EOE 146 — Molecular Modifier
pub(in crate::card::sets) static MOLECULAR_MODIFIER: CardRecord = CardRecord::new(
    "Molecular Modifier",
    "7b80b9c9-a871-4c04-b8be-feb81a900591",
    "Konstantin Porubov",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Kavu", "Artificer"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, target creature you \
             control gets +1/+0 and gains first strike until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
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
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EOE 147 — Nebula Dragon
pub(in crate::card::sets) static NEBULA_DRAGON: CardRecord = CardRecord::new(
    "Nebula Dragon",
    "0ee509dd-9fba-4b6d-a9d4-cc8bf5822ddd",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
    ]),
);

// EOE 148 — Nova Hellkite
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static NOVA_HELLKITE: CardRecord = CardRecord::new(
    "Nova Hellkite",
    "424af0d0-398c-4d78-9ad5-2171bf1bcbd1",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// EOE 149 — Orbital Plunge
// Audit: unsupported — Needs an excess-damage result and conditional continuation for an ordinary damage assignment; current damage follow-ups report intended recipients or apply effects, while excess follow-ups are fight-specific.
pub(in crate::card::sets) static ORBITAL_PLUNGE: CardRecord = CardRecord::new(
    "Orbital Plunge",
    "2dc7cc17-5319-4694-99c6-8c56a0b40a44",
    "Inkognit",
    CardRules::unsupported(),
);

// EOE 150 — Oreplate Pangolin
pub(in crate::card::sets) static OREPLATE_PANGOLIN: CardRecord = CardRecord::new(
    "Oreplate Pangolin",
    "90209957-95db-4b59-979a-316d14ef876c",
    "Dmitry Burmak",
    CardRules::new_artifact_creature(mana_cost!("{1}{R}"), &["Robot", "Pangolin"], 2, 2)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever another artifact you control enters, you may pay \
             {1}. If you do, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}"))],
                &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            )),
        )]),
);

// EOE 151 — Pain for All
pub(in crate::card::sets) static PAIN_FOR_ALL: CardRecord = CardRecord::new(
    "Pain for All",
    "d2948913-817b-4715-92d5-ed3cde347be7",
    "Dmitry Burmak",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
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
            abilities::enters_trigger_with_targets(
                "When this Aura enters, enchanted creature deals damage equal \
                 to its power to any other target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyOf(&[
                        AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                ]),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::AttachedToSource),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    ]),
                )],
                EffectDef::damage_from(
                    ObjectRefDef::AttachedToSource,
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::One(ObjectRefDef::AttachedToSource),
                        select: ObjectValueDef::Power,
                        operation: AggregateOperationDef::Sum,
                    }),
                ),
            ),
            AbilityDef::triggered(
                "Whenever enchanted creature is dealt damage, it deals that \
                 much damage to each opponent.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::AttachedPermanent,
                    ),
                    ..DamageEventMatcherDef::ANY
                }),
                EffectDef::damage_from(
                    ObjectRefDef::AttachedToSource,
                    EffectRecipientDef::Opponent,
                    ValueDef::TriggerEventAmount,
                ),
            ),
        ]),
);

// EOE 152 — Plasma Bolt
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static PLASMA_BOLT: CardRecord = CardRecord::new(
    "Plasma Bolt",
    "a1a1834b-76c2-4496-b8c5-18b69ab34c4c",
    "Viko Menezes",
    CardRules::unsupported(),
);

// EOE 153 — Possibility Technician
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately. Also needs an indefinite controller play permission conditional on currently controlling a Kavu.
pub(in crate::card::sets) static POSSIBILITY_TECHNICIAN: CardRecord = CardRecord::new(
    "Possibility Technician",
    "4b146c78-403f-48c8-941d-41114498bb89",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// EOE 154 — Red Tiger Mechan
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static RED_TIGER_MECHAN: CardRecord = CardRecord::new(
    "Red Tiger Mechan",
    "b7b2fa48-cd2d-42ea-afd8-8cbd7a1bcdab",
    "Simon Dominic",
    CardRules::unsupported(),
);

// EOE 155 — Remnant Elemental
pub(in crate::card::sets) static REMNANT_ELEMENTAL: CardRecord = CardRecord::new(
    "Remnant Elemental",
    "830d5532-3b24-470e-912f-f0f5df1cb530",
    "Nereida",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental"], 0, 4).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             gets +2/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EOE 156 — Rig for War
pub(in crate::card::sets) static RIG_FOR_WAR: CardRecord = CardRecord::new(
    "Rig for War",
    "3edd0515-dcc4-4cb5-8b54-9c00173d8a6d",
    "Diana Franco",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains reach and first strike \
         until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::reach()),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// EOE 157 — Roving Actuator
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static ROVING_ACTUATOR: CardRecord = CardRecord::new(
    "Roving Actuator",
    "1111173b-ea49-4de4-b5b0-07d768c626b9",
    "Sergey Glushakov",
    CardRules::unsupported(),
);

// EOE 158 — Ruinous Rampage
pub(in crate::card::sets) static RUINOUS_RAMPAGE: CardRecord = CardRecord::new(
    "Ruinous Rampage",
    "91d7a4c2-1a4b-4e9f-b543-225b6906752f",
    "David Astruga",
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Ruinous Rampage deals 3 damage to each opponent.",
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(3)),
            ),
            AbilityDef::spell(
                "Exile all artifacts with mana value 3 or less.",
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::ManaValueAtMost(3),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )]),
);

// EOE 159 — Rust Harvester
pub(in crate::card::sets) static RUST_HARVESTER: CardRecord = CardRecord::new(
    "Rust Harvester",
    "7ce58765-d2f4-4fbf-8635-580c9400ff2e",
    "Jake Murray",
    CardRules::new_artifact_creature(mana_cost!("{R}"), &["Robot"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::activated_with_targets(
            "{2}, {T}, Exile an artifact card from your graveyard: Put a \
             +1/+1 counter on this creature, then it deals damage equal to \
             its power to any target.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::MoveToZone(MoveToZoneCostDef::new(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    1,
                )),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::SourcePower,
                ),
            ]),
        ),
    ]),
);

// EOE 160 — Slagdrill Scrapper
pub(in crate::card::sets) static SLAGDRILL_SCRAPPER: CardRecord = CardRecord::new(
    "Slagdrill Scrapper",
    "155dcf54-8fdb-4715-97dc-4eb5d3d80d78",
    "Edgar Sánchez Hidalgo",
    CardRules::new_artifact_creature(mana_cost!("{R}"), &["Robot", "Scout"], 1, 2).with_abilities(
        &[AbilityDef::activated(
            "{2}, {T}, Sacrifice another artifact or land: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        )],
    ),
);

// EOE 161 — Systems Override
pub(in crate::card::sets) static SYSTEMS_OVERRIDE: CardRecord = CardRecord::new(
    "Systems Override",
    "2a34c71b-8d3c-435b-9cf8-4902f997d10d",
    "Hardy Fowler",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Gain control of target artifact or creature until end of \
         turn. Untap that permanent. It gains haste until end of turn. \
         If it's a Spacecraft, put ten charge counters on it. If you \
         do, remove ten charge counters from it at the beginning of \
         the next end step.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::Sequence(&[
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
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                },
                then: &EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                        TargetIndex::PRIMARY,
                    )),
                    binding: crate::Binding!("ship"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::named("charge"),
                            amount: ValueDef::Constant(10),
                        },
                        EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "Remove ten charge counters from that permanent.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::RemoveCounters {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("ship"),
                                    )),
                                    kind: CounterKind::named("charge"),
                                    amount: ValueDef::Constant(10),
                                },
                            ),
                        )),
                    ]),
                }),
            },
        ]),
    )]),
);

// EOE 162 — Tannuk, Steadfast Second
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately. Also needs granting a complete Warp alternative to matching cards in hand.
pub(in crate::card::sets) static TANNUK_STEADFAST_SECOND: CardRecord = CardRecord::new(
    "Tannuk, Steadfast Second",
    "44607ed3-9523-40ac-9f61-0edd011cf762",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// EOE 163 — Terminal Velocity
// Audit: unsupported — Needs permanent triggered-ability grants to the newly returned object identified by a zone-move binding; catalog validation rejects these grants because it cannot establish that the bound recipient is on the battlefield.
pub(in crate::card::sets) static TERMINAL_VELOCITY: CardRecord = CardRecord::new(
    "Terminal Velocity",
    "1d18dc06-16f0-4a3b-8d52-dbf4aa2c393d",
    "Xabi Gaztelua",
    CardRules::unsupported(),
);

// EOE 164 — Terrapact Intimidator
pub(in crate::card::sets) static TERRAPACT_INTIMIDATOR: CardRecord = CardRecord::new(
    "Terrapact Intimidator",
    "13fe3358-fd68-4245-a2ad-aa9200cf4655",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Kavu", "Scout"], 2, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent may have you \
             create two Lander tokens. If they don't, put two +1/+1 \
             counters on this creature. (A Lander token is an artifact \
             with \"{2}, {T}, Sacrifice this token: Search your library \
             for a basic land card, put it onto the battlefield tapped, \
             then shuffle.\")",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                choices: &[
                    EffectChoiceDef {
                        label: "Give two Landers",
                        effect: EffectDef::create_token(
                            TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                                AbilityDef::activated(
                                    "{2}, {T}, Sacrifice this token: Search your library for a \
                                     basic land card, put it onto the battlefield tapped, then \
                                     shuffle.",
                                    &[
                                        CostDef::Mana(mana_cost!("{2}")),
                                        CostDef::TapSource,
                                        CostDef::SacrificeSource,
                                    ],
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
                        )
                        .with_count(ValueDef::Constant(2)),
                    },
                    EffectChoiceDef {
                        label: "Decline",
                        effect: EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(2),
                        },
                    },
                ],
            },
        ),
    ]),
);

// EOE 165 — Territorial Bruntar
pub(in crate::card::sets) static TERRITORIAL_BRUNTAR: CardRecord = CardRecord::new(
    "Territorial Bruntar",
    "dbb25585-6048-4a85-828e-675bf0da6508",
    "Julie Dillon",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Beast"], 6, 6).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, exile cards \
             from the top of your library until you exile a nonland card. \
             You may cast that card this turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::BindObjects(BindObjectsDef {
                source: ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
                    player: PlayerRefDef::EffectController,
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                },
                binding: crate::Binding!("top"),
                then: &EffectDef::ClassifyObjects(ClassifyObjectsDef {
                    input: ObjectSetDef::Binding(crate::Binding!("top")),
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    matching: crate::Binding!("lands"),
                    remainder: crate::Binding!("spell"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "lands"
                            ))),
                            ZoneKind::Exile,
                            ZonePlacement::Top,
                        ),
                        EffectDef::ExileGrantingControllerPlayThisTurn {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("spell"),
                            )),
                        },
                    ]),
                }),
            }),
        ),
    ]),
);

// EOE 166 — Vaultguard Trooper
pub(in crate::card::sets) static VAULTGUARD_TROOPER: CardRecord = CardRecord::new(
    "Vaultguard Trooper",
    "36afe3b1-43a2-47b4-bc0a-24efb1e2e5a0",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Kavu", "Soldier"], 5, 5).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of your end step, if you control two or more \
             tapped creatures, you may discard your hand. If you do, draw \
             two cards.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerRelation::You,
                        )),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    abilities::draw_cards(ValueDef::Constant(2)),
                ]),
            },
        ),
    ]),
);

// EOE 167 — Warmaker Gunship
pub(in crate::card::sets) static WARMAKER_GUNSHIP: CardRecord = CardRecord::new(
    "Warmaker Gunship",
    "9e5957f4-1cae-4989-8b40-27fc6e2fcf5e",
    "Julian Kok Joon Wen",
    CardRules::new_spacecraft(mana_cost!("{2}{R}"), 4, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Spacecraft enters, it deals damage equal to the \
             number of artifacts you control to target creature an \
             opponent controls.",
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
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 6+.)",
        ),
        AbilityDef::static_ability(
            "6+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 6,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 168 — Weapons Manufacturing
pub(in crate::card::sets) static WEAPONS_MANUFACTURING: CardRecord = CardRecord::new(
    "Weapons Manufacturing",
    "a058f1a6-318c-4bba-981e-ace079ada806",
    "Marco Gorlei",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a nontoken artifact you control enters, create a \
         colorless artifact token named Munitions with \"When this \
         token leaves the battlefield, it deals 2 damage to any \
         target.\"",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ]),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::create_artifact_token(&[], &[])
            .with_name("Munitions")
            .with_abilities(&[AbilityDef::triggered_with_targets(
                "When this token leaves the battlefield, it deals 2 damage to \
                 any target.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            )]),
    )]),
);

// EOE 169 — Weftstalker Ardent
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static WEFTSTALKER_ARDENT: CardRecord = CardRecord::new(
    "Weftstalker Ardent",
    "cddb48cc-8eb1-47ce-90f0-7aad1e93e2c4",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// EOE 170 — Zookeeper Mechan
pub(in crate::card::sets) static ZOOKEEPER_MECHAN: CardRecord = CardRecord::new(
    "Zookeeper Mechan",
    "8d5cd0be-4337-4aba-a4f6-5adab7735a73",
    "Justyna Dura",
    CardRules::new_artifact_creature(mana_cost!("{1}{R}"), &["Robot"], 1, 3).with_abilities(&[
        abilities::tap_for(ManaColor::Red),
        AbilityDef::activated_with_targets(
            "{6}{R}: Target creature you control gets +4/+0 until end of \
             turn. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{6}{R}"))],
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// EOE 171 — Atmospheric Greenhouse
pub(in crate::card::sets) static ATMOSPHERIC_GREENHOUSE: CardRecord = CardRecord::new(
    "Atmospheric Greenhouse",
    "bf05e378-7a0c-49e3-8c6e-c0fd56796434",
    "Sergey Glushakov",
    CardRules::new_spacecraft(mana_cost!("{4}{G}"), 5, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, put a +1/+1 counter on each \
             creature you control.",
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
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 8+.)",
        ),
        AbilityDef::static_ability(
            "8+ | Flying, trample",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 8,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 172 — Bioengineered Future
// Audit: unsupported — Needs a land-entry tally for the current turn retained after lands leave or change controllers; live entered-this-turn queries cannot supply the printed count.
pub(in crate::card::sets) static BIOENGINEERED_FUTURE: CardRecord = CardRecord::new(
    "Bioengineered Future",
    "800ee479-c1dc-4dd0-9b98-436c78997958",
    "Constantin Marin",
    CardRules::unsupported(),
);

// EOE 173 — Biosynthic Burst
pub(in crate::card::sets) static BIOSYNTHIC_BURST: CardRecord = CardRecord::new(
    "Biosynthic Burst",
    "6d73a4a8-5d52-4c12-a96a-45cd202bcc62",
    "Loïc Canavaggia",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature you control. It gains \
         reach, trample, and indestructible until end of turn. Untap \
         it. (Damage and effects that say \"destroy\" don't destroy \
         it.)",
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
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::reach()),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// EOE 174 — Blooming Stinger
pub(in crate::card::sets) static BLOOMING_STINGER: CardRecord = CardRecord::new(
    "Blooming Stinger",
    "2ee859bd-d99c-4b1d-9372-7ff4fc1e8c6a",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Scorpion"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, another target creature you \
             control gains deathtouch until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EOE 175 — Broodguard Elite
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately. Also needs transferring every last-known counter kind and amount from the departed creature.
pub(in crate::card::sets) static BROODGUARD_ELITE: CardRecord = CardRecord::new(
    "Broodguard Elite",
    "08b1d019-65ab-4dea-9076-041fd6338a35",
    "Paolo Parente",
    CardRules::unsupported(),
);

// EOE 176 — Close Encounter
// Audit: unsupported — Needs an exile identity marker identifying cards exiled specifically by Warp, and an additional-cost choice spanning that group and controlled creatures.
pub(in crate::card::sets) static CLOSE_ENCOUNTER: CardRecord = CardRecord::new(
    "Close Encounter",
    "a8a77351-9115-470f-8141-222c1916b337",
    "Inkognit",
    CardRules::unsupported(),
);

// EOE 177 — Diplomatic Relations
pub(in crate::card::sets) static DIPLOMATIC_RELATIONS: CardRecord = CardRecord::new(
    "Diplomatic Relations",
    "e0a104c5-61fb-4733-97ab-a31a15a49443",
    "Néstor Ossandón Leal",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+0 and gains vigilance \
         until end of turn. It deals damage equal to its power to \
         target creature an opponent controls.",
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
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )]),
);

// EOE 177† — Diplomatic Relations (alternate printing)
const DIPLOMATIC_RELATIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIPLOMATIC_RELATIONS,
    1,
    "143e5853-9a31-4c8d-b21d-5ef120eb6952",
    "Néstor Ossandón Leal",
);

// EOE 178 — Drix Fatemaker
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static DRIX_FATEMAKER: CardRecord = CardRecord::new(
    "Drix Fatemaker",
    "1beb7566-305e-4091-bdc4-cf4c789ac05a",
    "Anna Pavleeva",
    CardRules::unsupported(),
);

// EOE 179 — Edge Rover
pub(in crate::card::sets) static EDGE_ROVER: CardRecord = CardRecord::new(
    "Edge Rover",
    "90741ec9-6893-42d4-b510-8664666094e3",
    "Francisco Badilla",
    CardRules::new_artifact_creature(mana_cost!("{G}"), &["Robot", "Scout"], 2, 2).with_abilities(
        &[
            abilities::reach(),
            abilities::dies_trigger(
                "When this creature dies, each player creates a Lander token. \
                 (It's an artifact with \"{2}, {T}, Sacrifice this token: \
                 Search your library for a basic land card, put it onto the \
                 battlefield tapped, then shuffle.\")",
                EffectDef::Sequence(&[
                    EffectDef::create_token(
                        TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                            AbilityDef::activated(
                                "{2}, {T}, Sacrifice this token: Search your library for a \
                                 basic land card, put it onto the battlefield tapped, then \
                                 shuffle.",
                                &[
                                    CostDef::Mana(mana_cost!("{2}")),
                                    CostDef::TapSource,
                                    CostDef::SacrificeSource,
                                ],
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
                    ),
                    EffectDef::create_token(
                        TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                            AbilityDef::activated(
                                "{2}, {T}, Sacrifice this token: Search your library for a \
                                 basic land card, put it onto the battlefield tapped, then \
                                 shuffle.",
                                &[
                                    CostDef::Mana(mana_cost!("{2}")),
                                    CostDef::TapSource,
                                    CostDef::SacrificeSource,
                                ],
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
                    )
                    .with_controller(PlayerRefDef::Opponent),
                ]),
            ),
        ],
    ),
);

// EOE 180 — Eumidian Terrabotanist
pub(in crate::card::sets) static EUMIDIAN_TERRABOTANIST: CardRecord = CardRecord::new(
    "Eumidian Terrabotanist",
    "64fb2981-86ed-478a-89cd-c6bb078a5bc7",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect", "Druid"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 181 — Eusocial Engineering
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static EUSOCIAL_ENGINEERING: CardRecord = CardRecord::new(
    "Eusocial Engineering",
    "011bd7d8-6d60-482a-91b7-d3f0aad13b71",
    "Francisco Badilla",
    CardRules::unsupported(),
);

// EOE 182 — Famished Worldsire
// Audit: unsupported — Needs an as-entry optional variable sacrifice of lands, with counters derived from the committed sacrifice count; existing entry replacements cannot execute that selection.
pub(in crate::card::sets) static FAMISHED_WORLDSIRE: CardRecord = CardRecord::new(
    "Famished Worldsire",
    "2934c9c8-d23a-462b-83d5-94e88c8663ac",
    "Kev Walker",
    CardRules::unsupported(),
);

// EOE 183 — Frenzied Baloth
pub(in crate::card::sets) static FRENZIED_BALOTH: CardRecord = CardRecord::new(
    "Frenzied Baloth",
    "c72d85e9-a0bc-4f73-8d73-c58843577f4e",
    "Diana Franco",
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Beast"], 3, 2).with_abilities(&[
        abilities::cannot_be_countered(),
        abilities::trample(),
        abilities::haste(),
        AbilityDef::static_ability(
            "Creature spells you control can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Stack],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        ),
        AbilityDef::static_ability(
            "Combat damage can't be prevented.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CombatDamageCannotBePrevented),
            },
        ),
    ]),
);

// EOE 184 — Fungal Colossus
// Audit: unsupported — Needs DistinctNamesAmong in spell-cost evaluation; it is available for resolving effects and conditions, but cost_reduction_value currently returns zero for it.
pub(in crate::card::sets) static FUNGAL_COLOSSUS: CardRecord = CardRecord::new(
    "Fungal Colossus",
    "9dc9559f-cece-4c85-807c-158291666007",
    "Sergey Glushakov",
    CardRules::unsupported(),
);

// EOE 185 — Galactic Wayfarer
pub(in crate::card::sets) static GALACTIC_WAYFARER: CardRecord = CardRecord::new(
    "Galactic Wayfarer",
    "85b898d2-050f-49a2-87af-07d54d105336",
    "Quintin Gleim",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Scout"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Lander token. (It's an \
             artifact with \"{2}, {T}, Sacrifice this token: Search your \
             library for a basic land card, put it onto the battlefield \
             tapped, then shuffle.\")",
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            ),
        ),
    ]),
);

// EOE 186 — Gene Pollinator
// Audit: unsupported — Needs a mana-ability payment that taps a separately chosen untapped permanent in addition to its source; TapPermanents is implemented for ordinary activations but rejected by mana activation enumeration and payment.
pub(in crate::card::sets) static GENE_POLLINATOR: CardRecord = CardRecord::new(
    "Gene Pollinator",
    "ce7a8eec-a029-4ee1-b2d6-405d903d4640",
    "Milivoj Ćeran",
    CardRules::unsupported(),
);

// EOE 187 — Germinating Wurm
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static GERMINATING_WURM: CardRecord = CardRecord::new(
    "Germinating Wurm",
    "fcde173a-6314-4904-bddd-68b2ab1e4867",
    "Monztre",
    CardRules::unsupported(),
);

// EOE 188 — Glacier Godmaw
pub(in crate::card::sets) static GLACIER_GODMAW: CardRecord = CardRecord::new(
    "Glacier Godmaw",
    "d3291c51-e963-4970-813d-9a06a47aa71e",
    "Bruce Brenneise",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Leviathan"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, create a Lander token. (It's an \
             artifact with \"{2}, {T}, Sacrifice this token: Search your \
             library for a basic land card, put it onto the battlefield \
             tapped, then shuffle.\")",
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            ),
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, creatures you \
             control get +1/+1 and gain vigilance and haste until end of \
             turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EOE 189 — Harmonious Grovestrider
pub(in crate::card::sets) static HARMONIOUS_GROVESTRIDER: CardRecord = CardRecord::new(
    "Harmonious Grovestrider",
    "e320abed-f145-42d3-b402-4f82e3a56389",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Beast"], 0, 0).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::static_ability(
            "Harmonious Grovestrider's power and toughness are each equal \
             to the number of lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            },
        ),
    ]),
);

// EOE 190 — Hemosymbic Mite
pub(in crate::card::sets) static HEMOSYMBIC_MITE: CardRecord = CardRecord::new(
    "Hemosymbic Mite",
    "c14137a4-2d44-444c-ad50-e2edf9380571",
    "Amanda Lee",
    CardRules::new_creature(mana_cost!("{G}"), &["Mite"], 1, 1).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature becomes tapped, another target \
             creature you control gets +X/+X until end of turn, where X is \
             this creature's power.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::SourcePower,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EOE 191 — Icecave Crasher
pub(in crate::card::sets) static ICECAVE_CRASHER: CardRecord = CardRecord::new(
    "Icecave Crasher",
    "e6c1ed0c-0c0d-47a7-8ebc-67854cb226e0",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 4, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             gets +1/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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

// EOE 192 — Icetill Explorer (alternate printing)
const ICETILL_EXPLORER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICETILL_EXPLORER,
    1,
    "d9482aab-6ddf-48e1-84fa-b13d5ff81e69",
    "Warren Mahy",
);

// EOE 193 — Intrepid Tenderfoot
pub(in crate::card::sets) static INTREPID_TENDERFOOT: CardRecord = CardRecord::new(
    "Intrepid Tenderfoot",
    "809df0ea-deff-47b9-83df-cc1f360d377e",
    "Xavier Ribeiro",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect", "Citizen"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{3}: Put a +1/+1 counter on this creature. Activate only as a \
             sorcery.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// EOE 194 — Larval Scoutlander
pub(in crate::card::sets) static LARVAL_SCOUTLANDER: CardRecord = CardRecord::new(
    "Larval Scoutlander",
    "d6083f43-58dc-46fc-aeff-347b1080417b",
    "Javier Charro",
    CardRules::new_spacecraft(mana_cost!("{2}{G}"), 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, you may sacrifice a land or \
             Lander. If you do, search your library for up to two basic \
             land cards, put them onto the battlefield tapped, then \
             shuffle.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lander")),
                ]))],
                &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            )),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 7+.)",
        ),
        AbilityDef::static_ability(
            "7+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 195 — Lashwhip Predator
pub(in crate::card::sets) static LASHWHIP_PREDATOR: CardRecord = CardRecord::new(
    "Lashwhip Predator",
    "24553e98-29a9-47e3-91c7-9add708d9ad1",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Plant", "Beast"], 5, 7).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {2} less to cast if your opponents control \
             three or more creatures.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 3,
                then: ValueDef::Constant(2),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::reach(),
    ]),
);

// EOE 196 — Loading Zone
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static LOADING_ZONE: CardRecord = CardRecord::new(
    "Loading Zone",
    "0d2c95bd-79af-4a23-b265-62cc0b164e3e",
    "Matt Stewart",
    CardRules::unsupported(),
);

// EOE 197 — Meltstrider Eulogist
pub(in crate::card::sets) static MELTSTRIDER_EULOGIST: CardRecord = CardRecord::new(
    "Meltstrider Eulogist",
    "df61aa0c-effc-4d57-be19-876a82c41d33",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect", "Soldier"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control with a +1/+1 counter on it \
             dies, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// EOE 198 — Meltstrider's Gear
pub(in crate::card::sets) static MELTSTRIDER_S_GEAR: CardRecord = CardRecord::new(
    "Meltstrider's Gear",
    "d75629cb-91e2-46fa-9c80-6feb29e1ceb8",
    "Camille Alquier",
    CardRules::new_artifact(mana_cost!("{G}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+1 and has reach.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{5}"))],
                "Equip {5} ({5}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// EOE 199 — Meltstrider's Resolve
pub(in crate::card::sets) static MELTSTRIDER_S_RESOLVE: CardRecord = CardRecord::new(
    "Meltstrider's Resolve",
    "53c08af4-b975-4d9d-baba-73e6727f2778",
    "Carlos Palma Cruchaga",
    CardRules::new_enchantment(mana_cost!("{G}"))
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
            abilities::enters_trigger_with_targets(
                "When this Aura enters, enchanted creature fights up to one \
                 target creature an opponent controls. (Each deals damage \
                 equal to its power to the other.)",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Fight {
                    first: ObjectRefDef::AttachedToSource,
                    second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    excess: None,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2 and can't be blocked by more \
                 than one creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                            BlockRestrictionDef::MaximumBlockers(1),
                        )),
                    ]),
                },
            ),
        ]),
);

// EOE 200 — Mightform Harmonizer (alternate printing)
const MIGHTFORM_HARMONIZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIGHTFORM_HARMONIZER,
    1,
    "f32302f1-b54f-4489-9d0b-9b771e59da06",
    "Bartek Fedyczak",
);

// EOE 201 — Ouroboroid
pub(in crate::card::sets) static OUROBOROID: CardRecord = CardRecord::new(
    "Ouroboroid",
    "209c591a-4ab2-4e89-9523-a7b766cf4e51",
    "Samuel Perin",
    // A 1/3 that doubles itself every combat and takes the rest of the board
    // with it: one counter each the first turn, two the next, four after
    // that.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Plant", "Wurm"], 1, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of combat on your turn, put X +1/+1 counters on each creature you \
             control, where X is this creature's power.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            // X is read once, as the ability resolves, and every creature
            // gets that many -- including the Wurm, whose own growth does
            // not raise the number partway through.
            EffectDef::AddCounters {
                // "Each creature you control" includes the Wurm itself, so the counters it
                // hands out make the next round of them bigger.
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::SourcePower,
            },
        ),
    ),
);

// EOE 202 — Pull Through the Weft
pub(in crate::card::sets) static PULL_THROUGH_THE_WEFT: CardRecord = CardRecord::new(
    "Pull Through the Weft",
    "a70d0877-1a1e-436b-bf8c-0ff6df9efc6a",
    "Andrew Mar",
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return up to two target nonland permanent cards from your \
             graveyard to your hand, then return up to two target land \
             cards from your graveyard to the battlefield tapped.",
            &[
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                    2,
                ),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                    2,
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex(1)),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ]),
        ),
    ]),
);

// EOE 203 — Sami's Curiosity
pub(in crate::card::sets) static SAMI_S_CURIOSITY: CardRecord = CardRecord::new(
    "Sami's Curiosity",
    "703ad0f3-bd05-42b0-85fb-0cd37807dc91",
    "Tuan Duong Chu",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell(
        "You gain 2 life. Create a Lander token. (It's an artifact \
         with \"{2}, {T}, Sacrifice this token: Search your library \
         for a basic land card, put it onto the battlefield tapped, \
         then shuffle.\")",
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::create_token(
                TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                    AbilityDef::activated(
                        "{2}, {T}, Sacrifice this token: Search your library for a \
                         basic land card, put it onto the battlefield tapped, then \
                         shuffle.",
                        &[
                            CostDef::Mana(mana_cost!("{2}")),
                            CostDef::TapSource,
                            CostDef::SacrificeSource,
                        ],
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
            ),
        ]),
    )]),
);

// EOE 204 — Seedship Agrarian
pub(in crate::card::sets) static SEEDSHIP_AGRARIAN: CardRecord = CardRecord::new(
    "Seedship Agrarian",
    "2fc7946d-37b0-4dc8-9daa-8d2204d8e4d2",
    "Helge C. Balzer",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Insect", "Scientist"], 3, 3).with_abilities(
        &[
            AbilityDef::triggered(
                "Whenever this creature becomes tapped, create a Lander token. \
                 (It's an artifact with \"{2}, {T}, Sacrifice this token: \
                 Search your library for a basic land card, put it onto the \
                 battlefield tapped, then shuffle.\")",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                EffectDef::create_token(
                    TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                        AbilityDef::activated(
                            "{2}, {T}, Sacrifice this token: Search your library for a \
                             basic land card, put it onto the battlefield tapped, then \
                             shuffle.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
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
                ),
            ),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, put a +1/+1 \
                 counter on this creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
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
        ],
    ),
);

// EOE 205 — Seedship Impact
pub(in crate::card::sets) static SEEDSHIP_IMPACT: CardRecord = CardRecord::new(
    "Seedship Impact",
    "d060d95f-f33a-4fc0-b002-c394d4cd82ce",
    "Constantin Marin",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. If its mana value was \
         2 or less, create a Lander token. (It's an artifact with \
         \"{2}, {T}, Sacrifice this token: Search your library for a \
         basic land card, put it onto the battlefield tapped, then \
         shuffle.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::TargetMatches {
                slot: TargetIndex::PRIMARY,
                object: ObjectPredicateDef::ManaValueAtMost(2),
            },
            then: &EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::create_token(
                    TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                        AbilityDef::activated(
                            "{2}, {T}, Sacrifice this token: Search your library for a \
                             basic land card, put it onto the battlefield tapped, then \
                             shuffle.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
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
                ),
            ]),
            otherwise: &EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        },
    )]),
);

// EOE 206 — Shattered Wings
pub(in crate::card::sets) static SHATTERED_WINGS: CardRecord = CardRecord::new(
    "Shattered Wings",
    "bbece737-bd4f-4dc8-bf5c-f77930246ab1",
    "Sergey Glushakov",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact, enchantment, or creature with \
         flying. Surveil 1. (Look at the top card of your library. You \
         may put it into your graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            abilities::surveil(ValueDef::Constant(1)),
        ]),
    )]),
);

// EOE 207 — Skystinger
pub(in crate::card::sets) static SKYSTINGER: CardRecord = CardRecord::new(
    "Skystinger",
    "dfea9941-3675-4c0f-bc4d-981c28deed36",
    "Carlos Palma Cruchaga",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect", "Warrior"], 3, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever this creature blocks a creature with flying, this \
             creature gets +5/+0 until end of turn.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(5),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EOE 208 — Sledge-Class Seedship
pub(in crate::card::sets) static SLEDGE_CLASS_SEEDSHIP: CardRecord = CardRecord::new(
    "Sledge-Class Seedship",
    "dc0cb4b6-cc20-49ea-84b2-2f3af3b0a19e",
    "Leon Tukker",
    CardRules::new_spacecraft(mana_cost!("{2}{G}"), 4, 5).with_abilities(&[
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 7+.)",
        ),
        AbilityDef::static_ability(
            "7+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
        AbilityDef::static_ability(
            "7+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::triggered(
                            "Whenever this Spacecraft attacks, you may put a creature card \
                             from your hand onto the battlefield.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
                            EffectDef::Choose(ChooseDef {
                                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                                unchosen: None,
                                chooser: PlayerRefDef::EffectController,
                                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Hand],
                                    PlayerRelation::You,
                                )),
                                exclude: None,
                                minimum: 0,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Private,
                                then: &EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("chosen"),
                                    )),
                                    ZoneKind::Battlefield,
                                    ZonePlacement::Top,
                                ),
                            }),
                        ),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 209 — Tapestry Warden
// Audit: unsupported — Needs a continuous modifier that substitutes toughness for power when resolving a Station ability; current tap payments and value projections have no Station contribution override.
pub(in crate::card::sets) static TAPESTRY_WARDEN: CardRecord = CardRecord::new(
    "Tapestry Warden",
    "7cbbab6c-43ae-4e50-97ce-532a3316591a",
    "Andreas Zafiratos",
    CardRules::unsupported(),
);

// EOE 210 — Terrasymbiosis
// Audit: unsupported — Needs counter-placement events identifying the placing player, plus a per-turn use consumed only when the optional draw is accepted.
pub(in crate::card::sets) static TERRASYMBIOSIS: CardRecord = CardRecord::new(
    "Terrasymbiosis",
    "26008c7d-5dbe-4da2-b475-4dd307e7bc68",
    "Viko Menezes",
    CardRules::unsupported(),
);

// EOE 211 — Thawbringer
pub(in crate::card::sets) static THAWBRINGER: CardRecord = CardRecord::new(
    "Thawbringer",
    "0b7f934f-8eb4-408b-a55f-245ec5cc4a8a",
    "Olivier Bernard",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect", "Scout"], 4, 2).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters or dies, surveil 1. (Look at the \
             top card of your library. You may put it into your \
             graveyard.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// EOE 212 — Alpharael, Dreaming Acolyte
pub(in crate::card::sets) static ALPHARAEL_DREAMING_ACOLYTE: CardRecord = CardRecord::new(
    "Alpharael, Dreaming Acolyte",
    "349a2211-2b23-418d-a1ef-1c72ad2e171d",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Human", "Cleric"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Alpharael enters, draw two cards. Then discard two cards \
                 unless you discard an artifact card.",
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::IfElseCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                &[ZoneKind::Hand],
                                PlayerRelation::You,
                            )),
                            comparison: ComparisonDef::Greater,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::ChooseEffect {
                            player: EffectRecipientDef::Controller,
                            choices: &[
                                EffectChoiceDef {
                                    label: "Discard two cards",
                                    effect: EffectDef::Discard {
                                        recipient: EffectRecipientDef::Controller,
                                        amount: ValueDef::Constant(2),
                                        selection: DiscardSelectionDef::RecipientChooses,
                                        then: None,
                                    },
                                },
                                EffectChoiceDef {
                                    label: "Discard an artifact",
                                    effect: EffectDef::Choose(ChooseDef {
                                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                            "chosen"
                                        )),
                                        unchosen: None,
                                        chooser: PlayerRefDef::EffectController,
                                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                                            ObjectPredicateDef::HasType(CardType::Artifact),
                                            &[ZoneKind::Hand],
                                            PlayerRelation::You,
                                        )),
                                        exclude: None,
                                        minimum: 1,
                                        maximum: 1,
                                        visibility: ChoiceVisibilityDef::Private,
                                        then: &EffectDef::discard_cards(
                                            EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                crate::Binding!("chosen"),
                                            )),
                                        ),
                                    }),
                                },
                            ],
                        },
                        otherwise: &EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "During your turn, Alpharael has deathtouch.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    },
                },
            ),
        ]),
);

// EOE 213 — Biomechan Engineer
pub(in crate::card::sets) static BIOMECHAN_ENGINEER: CardRecord = CardRecord::new(
    "Biomechan Engineer",
    "7edcef2c-029c-46bd-bfeb-ff56a57dd63a",
    "Monztre",
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Insect", "Artificer"], 2, 2).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, create a Lander token. (It's an \
                 artifact with \"{2}, {T}, Sacrifice this token: Search your \
                 library for a basic land card, put it onto the battlefield \
                 tapped, then shuffle.\")",
                EffectDef::create_token(
                    TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                        AbilityDef::activated(
                            "{2}, {T}, Sacrifice this token: Search your library for a \
                             basic land card, put it onto the battlefield tapped, then \
                             shuffle.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
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
                ),
            ),
            AbilityDef::activated(
                "{8}: Draw two cards and create a 2/2 colorless Robot artifact \
                 creature token.",
                &[CostDef::Mana(mana_cost!("{8}"))],
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2),
                ]),
            ),
        ],
    ),
);

// EOE 214 — Biotech Specialist
pub(in crate::card::sets) static BIOTECH_SPECIALIST: CardRecord = CardRecord::new(
    "Biotech Specialist",
    "127c221f-94e7-4a0e-a7a6-79ef399862d3",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Insect", "Scientist"], 1, 3).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, create a Lander token. (It's an \
                 artifact with \"{2}, {T}, Sacrifice this token: Search your \
                 library for a basic land card, put it onto the battlefield \
                 tapped, then shuffle.\")",
                EffectDef::create_token(
                    TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                        AbilityDef::activated(
                            "{2}, {T}, Sacrifice this token: Search your library for a \
                             basic land card, put it onto the battlefield tapped, then \
                             shuffle.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
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
                ),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you sacrifice an artifact, this creature deals 2 \
                 damage to target opponent.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
        ],
    ),
);

// EOE 215 — Cosmogoyf
pub(in crate::card::sets) static COSMOGOYF: CardRecord = CardRecord::new(
    "Cosmogoyf",
    "5e07d3c6-60a5-44d1-a926-6414be85bd50",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Elemental", "Lhurgoyf"], 0, 1).with_abilities(
        &[AbilityDef::static_ability(
            "Cosmogoyf's power is equal to the number of cards you own in \
             exile and its toughness is equal to that number plus 1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Exile],
                            PlayerRelation::You,
                        ),
                    )),
                    AppliedEffectDef::define_toughness(ValueDef::Sum(&SumValueDef::new(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Exile],
                            PlayerRelation::You,
                        )),
                        ValueDef::Constant(1),
                    ))),
                ]),
            },
        )],
    ),
);

// EOE 216 — Dyadrine, Synthesis Amalgam
// Audit: unsupported — Needs a resolving payment that chooses two distinct controlled creatures and removes one +1/+1 counter from each; current fixed counter removal is source-only.
pub(in crate::card::sets) static DYADRINE_SYNTHESIS_AMALGAM: CardRecord = CardRecord::new(
    "Dyadrine, Synthesis Amalgam",
    "994ca692-7138-4dcb-bf46-5da530f86036",
    "Igor Grechanyi",
    CardRules::unsupported(),
);

// EOE 217 — Genemorph Imago
pub(in crate::card::sets) static GENEMORPH_IMAGO: CardRecord = CardRecord::new(
    "Genemorph Imago",
    "40f0ecf7-f49e-46ff-aa5b-9ff5361b72c5",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Insect", "Druid"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Landfall — Whenever a land you control enters, target \
             creature has base power and toughness 3/3 until end of turn. \
             If you control six or more lands, that creature has base \
             power and toughness 6/6 until end of turn instead.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(6),
                }),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(6),
                        ValueDef::Constant(6),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// EOE 218 — Haliya, Ascendant Cadet
// Audit: unsupported — Needs a combat-damage batch trigger filtered by controlled creatures with +1/+1 counters, firing once per damaged player rather than once per creature.
pub(in crate::card::sets) static HALIYA_ASCENDANT_CADET: CardRecord = CardRecord::new(
    "Haliya, Ascendant Cadet",
    "683d6eba-7f98-4105-b318-7f2290012f32",
    "Justyna Dura",
    CardRules::unsupported(),
);

// EOE 219 — Infinite Guideline Station
pub(in crate::card::sets) static INFINITE_GUIDELINE_STATION: CardRecord = CardRecord::new(
    "Infinite Guideline Station",
    "5688894a-bbec-476b-ae2e-94000be258d0",
    "Piotr Dura",
    CardRules::new_spacecraft(mana_cost!("{W}{U}{B}{R}{G}"), 7, 15)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Infinite Guideline Station enters, create a tapped 2/2 \
                 colorless Robot artifact creature token for each multicolored \
                 permanent you control.",
                EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2)
                    .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(1)),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )))
                    .entering_tapped(),
            ),
            station(
                "Station (Tap another creature you control: Put charge \
                 counters equal to its power on this Spacecraft. Station only \
                 as a sorcery. It's an artifact creature at 12+.)",
            ),
            AbilityDef::static_ability(
                "12+ | Flying",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 12,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_card_types(
                                CardTypeSet::single(CardType::Artifact)
                                    .union(CardTypeSet::single(CardType::Creature)),
                            ),
                            AppliedEffectDef::add_ability(&abilities::flying()),
                        ]),
                    },
                },
            ),
            AbilityDef::static_ability(
                "12+ | Flying",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 12,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                            &AbilityDef::triggered(
                                "Whenever Infinite Guideline Station attacks, draw a card for \
                                 each multicolored permanent you control.",
                                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                                abilities::draw_cards(ValueDef::CountMatchingObjects(
                                    &ObjectQueryDef::matching(
                                        ObjectPredicateDef::All(&[
                                            ObjectPredicateDef::Not(
                                                &ObjectPredicateDef::ColorCount(0),
                                            ),
                                            ObjectPredicateDef::Not(
                                                &ObjectPredicateDef::ColorCount(1),
                                            ),
                                        ]),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    ),
                                )),
                            ),
                        )]),
                    },
                },
            ),
        ]),
);

// EOE 220 — Interceptor Mechan
// Audit: unsupported — Needs turn history for any nonland permanent leaving the battlefield and for any spell cast using Warp, retained across subsequent zone and control changes; the current permanent-left tally includes lands and the spell history has no Warp-cost predicate.
pub(in crate::card::sets) static INTERCEPTOR_MECHAN: CardRecord = CardRecord::new(
    "Interceptor Mechan",
    "198211af-f413-4e9b-9baf-4b4fcb81eadc",
    "Leonardo Santanna",
    CardRules::unsupported(),
);

// EOE 221 — Mm'menon, Uthros Exile
pub(in crate::card::sets) static MM_MENON_UTHROS_EXILE: CardRecord = CardRecord::new(
    "Mm'menon, Uthros Exile",
    "5546c044-5826-48c3-9d28-866f3c3c5f2c",
    "Fajareka Setiawan",
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Jellyfish", "Advisor"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever an artifact you control enters, put a +1/+1 counter \
                 on target creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// EOE 222 — Mutinous Massacre
// Audit: unsupported — Needs a predicate testing odd or even mana value across a changing creature set; the existing scalar comparison vocabulary has no parity operation.
pub(in crate::card::sets) static MUTINOUS_MASSACRE: CardRecord = CardRecord::new(
    "Mutinous Massacre",
    "42d5034f-18f0-4d57-9840-6be52c286247",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// EOE 223 — Pinnacle Emissary
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static PINNACLE_EMISSARY: CardRecord = CardRecord::new(
    "Pinnacle Emissary",
    "3c922347-f05f-40a4-bbee-6bc02a1e0de5",
    "Alejandro Pacheco",
    CardRules::unsupported(),
);

// EOE 224 — Ragost, Deft Gastronaut
pub(in crate::card::sets) static RAGOST_DEFT_GASTRONAUT: CardRecord = CardRecord::new(
    "Ragost, Deft Gastronaut",
    "011374c3-f69d-4573-8c32-5bd0fe083d6a",
    "Zack Stella",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Lobster", "Citizen"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Artifacts you control are Foods in addition to their other \
                 types and have \"{2}, {T}, Sacrifice this artifact: You gain \
                 3 life.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                            SetOperationDef::Add(&["Food"]),
                        )),
                        AppliedEffectDef::add_ability(&AbilityDef::activated(
                            "{2}, {T}, Sacrifice this artifact: You gain 3 life.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
                            EffectDef::GainLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(3),
                            },
                        )),
                    ]),
                },
            ),
            AbilityDef::activated(
                "{1}, {T}, Sacrifice a Food: Ragost deals 3 damage to each \
                 opponent.",
                &[
                    CostDef::Mana(mana_cost!("{1}")),
                    CostDef::TapSource,
                    CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Food",
                    ))),
                ],
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(3)),
            ),
            AbilityDef::triggered_if(
                "At the beginning of each end step, if you gained life this \
                 turn, untap Ragost.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ),
        ]),
);

// EOE 225 — Sami, Ship's Engineer
pub(in crate::card::sets) static SAMI_SHIP_S_ENGINEER: CardRecord = CardRecord::new(
    "Sami, Ship's Engineer",
    "75c38cc9-07de-46d4-8195-f04b2b7e0fee",
    "Zara Alfonso",
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Human", "Artificer"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_if(
            "At the beginning of your end step, if you control two or more \
             tapped creatures, create a tapped 2/2 colorless Robot \
             artifact creature token.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2).entering_tapped(),
        )]),
);

// EOE 226 — Sami, Wildcat Captain
pub(in crate::card::sets) static SAMI_WILDCAT_CAPTAIN: CardRecord = CardRecord::new(
    "Sami, Wildcat Captain",
    "bed64207-9193-4770-8f8f-e3203289d5a6",
    "Kieran Yanner",
    CardRules::new_creature(
        mana_cost!("{4}{R}{W}"),
        &["Human", "Artificer", "Rogue"],
        4,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::double_strike(),
        abilities::vigilance(),
        AbilityDef::static_ability(
            "Spells you cast have affinity for artifacts. (They cost {1} \
             less to cast for each artifact you control.)",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::Any,
                PlayerRelation::You,
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        ),
    ]),
);

// EOE 227 — Seedship Broodtender
pub(in crate::card::sets) static SEEDSHIP_BROODTENDER: CardRecord = CardRecord::new(
    "Seedship Broodtender",
    "1abc176f-2ccf-4371-b4b5-030dd99ff7fc",
    "Eric Wilkerson",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Insect", "Citizen"], 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill three cards. (Put the top \
             three cards of your library into your graveyard.)",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}{B}{G}, Sacrifice this creature: Return target creature or \
             Spacecraft card from your graveyard to the battlefield. \
             Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{3}{B}{G}")),
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// EOE 228 — Singularity Rupture
pub(in crate::card::sets) static SINGULARITY_RUPTURE: CardRecord = CardRecord::new(
    "Singularity Rupture",
    "a34012e3-ec7a-4713-a2c2-f8efff49e364",
    "Liiga Smilshkalne",
    CardRules::new_sorcery(mana_cost!("{3}{U}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy all creatures, then any number of target players each \
             mill half their library, rounded down.",
            &[
                AbilityTargetDef::up_to(AbilityTargetPredicate::Player(PlayerRelation::Any), 1),
                AbilityTargetDef::up_to(AbilityTargetPredicate::Player(PlayerRelation::Any), 1)
                    .another(),
            ],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    then: None,
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex(0)),
                    amount: ValueDef::Halved(&HalvedValueDef {
                        value: ValueDef::TargetLibrarySize(TargetIndex(0)),
                        rounding: RoundingDef::Down,
                    }),
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex(1)),
                    amount: ValueDef::Halved(&HalvedValueDef {
                        value: ValueDef::TargetLibrarySize(TargetIndex(1)),
                        rounding: RoundingDef::Down,
                    }),
                },
            ]),
        ),
    ]),
);

// EOE 229 — Space-Time Anomaly
pub(in crate::card::sets) static SPACE_TIME_ANOMALY: CardRecord = CardRecord::new(
    "Space-Time Anomaly",
    "edb8dc2a-ddce-48fa-b57e-0e57c87c6671",
    "Loïc Canavaggia",
    CardRules::new_sorcery(mana_cost!("{2}{W}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player mills cards equal to your life total.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::LifeTotal(PlayerRelation::You),
            },
        ),
    ]),
);

// EOE 230 — Station Monitor
pub(in crate::card::sets) static STATION_MONITOR: CardRecord = CardRecord::new(
    "Station Monitor",
    "ba9f6d16-ee3e-4fbb-b78a-6292188eb61f",
    "Camille Alquier",
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Lizard", "Artificer"], 2, 2).with_abilities(
        &[AbilityDef::triggered_if(
            "Whenever you cast your second spell each turn, create a 1/1 \
             colorless Drone artifact creature token with flying and \
             \"This token can block only creatures with flying.\"",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Any,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &TriggerConditionDef::SpellsCastThisTurn {
                quantifier: QuantifierDef::Any,
                player: PlayerRelation::You,
                comparison: ComparisonDef::Equal,
                amount: 2,
            },
            EffectDef::create_artifact_creature_token(&["Drone"], &[], 1, 1).with_abilities(&[
                abilities::flying(),
                AbilityDef::static_ability(
                    "This token can block only creatures with flying.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        )),
                    },
                ),
            ]),
        )],
    ),
);

// EOE 231 — Syr Vondam, Sunstar Exemplar
pub(in crate::card::sets) static SYR_VONDAM_SUNSTAR_EXEMPLAR: CardRecord = CardRecord::new(
    "Syr Vondam, Sunstar Exemplar",
    "49554198-549b-4066-86ce-77a03fda0a2f",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Human", "Knight"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever another creature you control dies or is put into \
                 exile, put a +1/+1 counter on Syr Vondam and you gain 1 life.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Graveyard),
                    ),
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Exile),
                    ),
                ]),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::triggered_with_targets(
                "When Syr Vondam dies or is put into exile while its power is \
                 4 or greater, destroy up to one target nonland permanent.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Source,
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Graveyard),
                    ),
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Source,
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Exile),
                    ),
                ]),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
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

// EOE 232 — Syr Vondam, the Lucent
pub(in crate::card::sets) static SYR_VONDAM_THE_LUCENT: CardRecord = CardRecord::new(
    "Syr Vondam, the Lucent",
    "ea954205-5ff5-493b-bf30-6212042c2bc9",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{2}{W}{B}{B}"), &["Human", "Knight"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            abilities::lifelink(),
            AbilityDef::triggered(
                "Whenever Syr Vondam enters or attacks, other creatures you \
                 control get +1/+0 and gain deathtouch until end of turn.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::Apply {
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
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// EOE 233 — Tannuk, Memorial Ensign
// Audit: unsupported — Needs per-ability resolution history retained after its source leaves the battlefield; SourceResolutionsThisTurn only reads a live permanent.
pub(in crate::card::sets) static TANNUK_MEMORIAL_ENSIGN: CardRecord = CardRecord::new(
    "Tannuk, Memorial Ensign",
    "52498b7b-0389-4e7b-b29f-7ac86aab9229",
    "David Auden Nash",
    CardRules::unsupported(),
);

// EOE 234 — All-Fates Scroll
pub(in crate::card::sets) static ALL_FATES_SCROLL: CardRecord = CardRecord::new(
    "All-Fates Scroll",
    "3a5ed010-cb17-45df-b169-ebc807dae534",
    "Sam Guay",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated(
            "{7}, {T}, Sacrifice this artifact: Draw X cards, where X is \
             the number of differently named lands you control.",
            &[
                CostDef::Mana(mana_cost!("{7}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::DistinctNamesAmong(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
        ),
    ]),
);

// EOE 235 — Bygone Colossus
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static BYGONE_COLOSSUS: CardRecord = CardRecord::new(
    "Bygone Colossus",
    "4bb8f2ef-4398-4a07-9130-5005356a3b4a",
    "Maxime Minard",
    CardRules::unsupported(),
);

// EOE 236 — Chrome Companion
pub(in crate::card::sets) static CHROME_COMPANION: CardRecord = CardRecord::new(
    "Chrome Companion",
    "8ef269a0-1cb9-4901-81e6-43db3ae3756c",
    "Gray Highsmith",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Dog"], 2, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, you gain 1 life.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}: Put target card from a graveyard on the bottom of \
             its owner's library.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::Any),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Library,
                ZonePlacement::Bottom,
            ),
        ),
    ]),
);

// EOE 237 — Dauntless Scrapbot
pub(in crate::card::sets) static DAUNTLESS_SCRAPBOT: CardRecord = CardRecord::new(
    "Dauntless Scrapbot",
    "0efe5342-42b7-4f49-b4b4-d77055508c4d",
    "Alix Branwyn",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Robot"], 3, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, exile each opponent's graveyard. \
             Create a Lander token. (It's an artifact with \"{2}, {T}, \
             Sacrifice this token: Search your library for a basic land \
             card, put it onto the battlefield tapped, then shuffle.\")",
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::Opponent,
                    ))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::create_token(
                    TokenCharacteristics::artifact(&["Lander"], &[]).with_abilities(&[
                        AbilityDef::activated(
                            "{2}, {T}, Sacrifice this token: Search your library for a \
                             basic land card, put it onto the battlefield tapped, then \
                             shuffle.",
                            &[
                                CostDef::Mana(mana_cost!("{2}")),
                                CostDef::TapSource,
                                CostDef::SacrificeSource,
                            ],
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
                ),
            ]),
        ),
    ]),
);

// EOE 238 — Dawnsire, Sunstar Dreadnought
pub(in crate::card::sets) static DAWNSIRE_SUNSTAR_DREADNOUGHT: CardRecord = CardRecord::new(
    "Dawnsire, Sunstar Dreadnought",
    "6133355c-3dcf-466a-b771-fe6c44d4fa4d",
    "Jaime Jones",
    CardRules::new_spacecraft(mana_cost!("{5}"), 20, 20)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            station(
                "Station (Tap another creature you control: Put charge \
                 counters equal to its power on this Spacecraft. Station only \
                 as a sorcery. It's an artifact creature at 20+.)",
            ),
            AbilityDef::static_ability(
                "10+ | Whenever you attack, Dawnsire deals 100 damage to up to \
                 one target creature or planeswalker.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 10,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                            &AbilityDef::triggered_with_targets(
                                "10+ | Whenever you attack, Dawnsire deals 100 damage to up to \
                                 one target creature or planeswalker.",
                                TriggerEventDef::attack_declared(
                                    ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                    ]),
                                    1,
                                    None,
                                ),
                                &[AbilityTargetDef::up_to(
                                    AbilityTargetPredicate::Object {
                                        object: ObjectPredicateDef::AnyOf(&[
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                                        ]),
                                        zones: &[ZoneKind::Battlefield],
                                        controller: None,
                                        owner: None,
                                    },
                                    1,
                                )],
                                EffectDef::damage(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ValueDef::Constant(100),
                                ),
                            ),
                        )]),
                    },
                },
            ),
            AbilityDef::static_ability(
                "20+ | Flying",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 20,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_card_types(
                                CardTypeSet::single(CardType::Artifact)
                                    .union(CardTypeSet::single(CardType::Creature)),
                            ),
                            AppliedEffectDef::add_ability(&abilities::flying()),
                        ]),
                    },
                },
            ),
        ]),
);

// EOE 239 — The Dominion Bracelet
// Audit: unsupported — Needs control of another player's complete next-turn decisions, including private information, mana choices, and their legal actions.
pub(in crate::card::sets) static THE_DOMINION_BRACELET: CardRecord = CardRecord::new(
    "The Dominion Bracelet",
    "f5360880-2849-45d6-b1aa-08c7e01083af",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// EOE 240 — The Endstone (alternate printing)
const THE_ENDSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ENDSTONE,
    1,
    "a451a459-18e0-4c53-a171-3e9da534ebf1",
    "Ryan Pancoast",
);

// EOE 241 — The Eternity Elevator
pub(in crate::card::sets) static THE_ETERNITY_ELEVATOR: CardRecord = CardRecord::new(
    "The Eternity Elevator",
    "1bb90ab9-43b9-4991-806a-0afc4d8caf5f",
    "Josu Solano",
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_subtypes(&["Spacecraft"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {C}{C}{C}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
            ),
            station(
                "Station (Tap another creature you control: Put charge \
                 counters equal to its power on this Spacecraft. Station only \
                 as a sorcery.)",
            ),
            AbilityDef::static_ability(
                "20+ | {T}: Add X mana of any one color, where X is the number \
                 of charge counters on The Eternity Elevator.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::named("charge"),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 20,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                            &AbilityDef::activated_mana(
                                "20+ | {T}: Add X mana of any one color, where X is the number \
                                 of charge counters on The Eternity Elevator.",
                                &[CostDef::TapSource],
                                EffectDef::AddMana(
                                    AddManaEffectDef::any_color().with_variable_amount(
                                        ValueDef::CountersOnSource(CounterKind::named("charge")),
                                    ),
                                ),
                            ),
                        )]),
                    },
                },
            ),
        ]),
);

// EOE 242 — Extinguisher Battleship
pub(in crate::card::sets) static EXTINGUISHER_BATTLESHIP: CardRecord = CardRecord::new(
    "Extinguisher Battleship",
    "5541cdd2-84a6-4667-83eb-fffbe5b3cd3d",
    "Danny Schwartz",
    CardRules::new_spacecraft(mana_cost!("{8}"), 10, 10).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Spacecraft enters, destroy target noncreature \
             permanent. Then this Spacecraft deals 4 damage to each \
             creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::damage(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ))),
                    ValueDef::Constant(4),
                ),
            ]),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 5+.)",
        ),
        AbilityDef::static_ability(
            "5+ | Flying, trample",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 5,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 243 — Nutrient Block
pub(in crate::card::sets) static NUTRIENT_BLOCK: CardRecord = CardRecord::new(
    "Nutrient Block",
    "a26064bb-c568-4ed6-86db-3aab69b050db",
    "Francisco Miyara",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Food"])
        .with_abilities(&[
            abilities::indestructible(),
            AbilityDef::activated(
                "{2}, {T}, Sacrifice this artifact: You gain 3 life.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            abilities::dies_trigger(
                "When this artifact is put into a graveyard from the \
                 battlefield, draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// EOE 244 — Pinnacle Kill-Ship
pub(in crate::card::sets) static PINNACLE_KILL_SHIP: CardRecord = CardRecord::new(
    "Pinnacle Kill-Ship",
    "bf784de8-5ae2-4c07-92bb-a5b7f593b773",
    "Alexandre Honoré",
    CardRules::new_spacecraft(mana_cost!("{7}"), 7, 7).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Spacecraft enters, it deals 10 damage to up to one \
             target creature.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(10),
            ),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 7+.)",
        ),
        AbilityDef::static_ability(
            "7+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 245 — Survey Mechan
// Audit: unsupported — Needs DistinctNamesAmong in activation-cost evaluation; ordinary cost_reduction_value does not evaluate the distinct-name aggregate.
pub(in crate::card::sets) static SURVEY_MECHAN: CardRecord = CardRecord::new(
    "Survey Mechan",
    "9b4278ea-6cd8-45ad-b024-daf3dedd29e0",
    "Johann Bodin",
    CardRules::unsupported(),
);

// EOE 246 — Thaumaton Torpedo
// Audit: unsupported — Needs player attack history filtered by Spacecraft and retained after those attackers leave or change controllers; current history only inspects individual live attackers.
pub(in crate::card::sets) static THAUMATON_TORPEDO: CardRecord = CardRecord::new(
    "Thaumaton Torpedo",
    "1817f1b5-960a-435c-bdec-8cc8cbcb3358",
    "Madeline Boni",
    CardRules::unsupported(),
);

// EOE 247 — Thrumming Hivepool
pub(in crate::card::sets) static THRUMMING_HIVEPOOL: CardRecord = CardRecord::new(
    "Thrumming Hivepool",
    "85caf659-7b43-462e-a342-34703d46eb57",
    "Rob Rey",
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Slivers (This spell costs {1} less to cast for \
             each Sliver you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::static_ability(
            "Slivers you control have double strike and haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::double_strike()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, create two 1/1 colorless \
             Sliver creature tokens.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::create_creature_token(&["Sliver"], &[], 1, 1)
                .with_count(ValueDef::Constant(2)),
        ),
    ]),
);

// EOE 248 — Virulent Silencer
pub(in crate::card::sets) static VIRULENT_SILENCER: CardRecord = CardRecord::new(
    "Virulent Silencer",
    "4120fcfc-3547-4774-a15d-b9cccac04e76",
    "Kenn Yap",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Robot", "Assassin"], 2, 3)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a nontoken artifact creature you control deals \
             combat damage to a player, that player gets two poison \
             counters. (A player with ten or more poison counters loses \
             the game.)",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::player(PlayerRefDef::EventPlayer),
                kind: CounterKind::Poison,
                amount: ValueDef::Constant(2),
            },
        )]),
);

// EOE 249 — Wurmwall Sweeper
pub(in crate::card::sets) static WURMWALL_SWEEPER: CardRecord = CardRecord::new(
    "Wurmwall Sweeper",
    "9ace282a-5901-4d36-ad21-17eb88bc5138",
    "Hardy Fowler",
    CardRules::new_spacecraft(mana_cost!("{2}"), 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this Spacecraft enters, surveil 2.",
            abilities::surveil(ValueDef::Constant(2)),
        ),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Spacecraft. Station only \
             as a sorcery. It's an artifact creature at 4+.)",
        ),
        AbilityDef::static_ability(
            "4+ | Flying",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 4,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            },
        ),
    ]),
);

// EOE 250 — Adagia, Windswept Bastion
pub(in crate::card::sets) static ADAGIA_WINDSWEPT_BASTION: CardRecord = CardRecord::new(
    "Adagia, Windswept Bastion",
    "c634273a-94b0-4104-9d10-ae522ece1fc7",
    "Adam Paquette",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::White),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Planet. Station only as a \
             sorcery.)",
        ),
        AbilityDef::static_ability(
            "12+ | {3}{W}, {T}: Create a token that's a copy of target \
             artifact or enchantment you control, except it's legendary. \
             Activate only as a sorcery.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 12,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::activated_with_targets(
                            "12+ | {3}{W}, {T}: Create a token that's a copy of target \
                             artifact or enchantment you control, except it's legendary. \
                             Activate only as a sorcery.",
                            &[CostDef::Mana(mana_cost!("{3}{W}")), CostDef::TapSource],
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Object {
                                    object: ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ]),
                                    zones: &[ZoneKind::Battlefield],
                                    controller: Some(PlayerRelation::You),
                                    owner: None,
                                },
                            )],
                            EffectDef::create_token_from_copy(&TokenCopyDef {
                                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                exceptions: CopyExceptionsDef {
                                    added_supertypes: &[CardSupertype::Legendary],
                                    ..CopyExceptionsDef::NONE
                                },
                            }),
                        )
                        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 251 — Breeding Pool (reprint)
const BREEDING_POOL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::BREEDING_POOL,
    "3c750d5a-f743-41ff-b5ba-02025ca0bec2",
    "Constantin Marin",
);

// EOE 252 — Command Bridge
pub(in crate::card::sets) static COMMAND_BRIDGE: CardRecord = CardRecord::new(
    "Command Bridge",
    "247670d2-a7cd-4ed7-9c77-704c7962b815",
    "Constantin Marin",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, sacrifice it unless you tap an \
             untapped permanent you control.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::IfElseCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountObjects(&ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            comparison: ComparisonDef::Greater,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::Tap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                        },
                        otherwise: &EffectDef::sacrifice(EffectRecipientDef::Source),
                    },
                }),
                otherwise: &EffectDef::sacrifice(EffectRecipientDef::Source),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// EOE 253 — Evendo, Waking Haven
pub(in crate::card::sets) static EVENDO_WAKING_HAVEN: CardRecord = CardRecord::new(
    "Evendo, Waking Haven",
    "2fa09104-acbe-4410-b101-2fe6ac28efde",
    "Adam Paquette",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Green),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Planet. Station only as a \
             sorcery.)",
        ),
        AbilityDef::static_ability(
            "12+ | {G}, {T}: Add {G} for each creature you control.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 12,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::activated_mana(
                            "12+ | {G}, {T}: Add {G} for each creature you control.",
                            &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
                            EffectDef::AddMana(
                                AddManaEffectDef::one(ManaColor::Green).with_variable_amount(
                                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    )),
                                ),
                            ),
                        ),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 254 — Godless Shrine (reprint)
const GODLESS_SHRINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::GODLESS_SHRINE,
    "8c542ea4-98c3-4c2d-9066-205ab7aa697a",
    "Rob Rey",
);

// EOE 255 — Kavaron, Memorial World
pub(in crate::card::sets) static KAVARON_MEMORIAL_WORLD: CardRecord = CardRecord::new(
    "Kavaron, Memorial World",
    "60f3ca25-9dcc-4781-bf7b-ab6736d8db29",
    "Adam Paquette",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Red),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Planet. Station only as a \
             sorcery.)",
        ),
        AbilityDef::static_ability(
            "12+ | {1}{R}, {T}, Sacrifice a land: Create a 2/2 colorless \
             Robot artifact creature token, then creatures you control get \
             +1/+0 and gain haste until end of turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 12,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::activated(
                            "12+ | {1}{R}, {T}, Sacrifice a land: Create a 2/2 colorless \
                             Robot artifact creature token, then creatures you control get \
                             +1/+0 and gain haste until end of turn.",
                            &[
                                CostDef::Mana(mana_cost!("{1}{R}")),
                                CostDef::TapSource,
                                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                            ],
                            EffectDef::Sequence(&[
                                EffectDef::create_artifact_creature_token(&["Robot"], &[], 2, 2),
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
                                            ValueDef::Constant(1),
                                            ValueDef::Constant(0),
                                        ),
                                        AppliedEffectDef::add_ability(&abilities::haste()),
                                    ]),
                                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                                },
                            ]),
                        ),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 256 — Sacred Foundry (reprint)
const SACRED_FOUNDRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::SACRED_FOUNDRY,
    "8b4e2642-3c87-4708-b9b4-2e7f7359ac7d",
    "Titus Lunter",
);

// EOE 257 — Secluded Starforge
// Audit: unsupported — Needs a variable-size artifact-tap activation cost with chosen X retained for the effect; ordinary tap-cost enumeration supports fixed counts only.
pub(in crate::card::sets) static SECLUDED_STARFORGE: CardRecord = CardRecord::new(
    "Secluded Starforge",
    "a997ff9f-045a-44a2-983d-f36414cef1ab",
    "Chris Rahn",
    CardRules::unsupported(),
);

// EOE 258 — Stomping Ground (reprint)
const STOMPING_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::STOMPING_GROUND,
    "69be21b4-c613-47c6-ba57-f4785861af3e",
    "Bruce Brenneise",
);

// EOE 259 — Susur Secundi, Void Altar
pub(in crate::card::sets) static SUSUR_SECUNDI_VOID_ALTAR: CardRecord = CardRecord::new(
    "Susur Secundi, Void Altar",
    "aefb8c0d-2bc6-4bec-851e-0137b4abfb22",
    "Adam Paquette",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Black),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Planet. Station only as a \
             sorcery.)",
        ),
        AbilityDef::static_ability(
            "12+ | {1}{B}, {T}, Pay 2 life, Sacrifice a creature: Draw \
             cards equal to the sacrificed creature's power. Activate only \
             as a sorcery.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 12,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::activated(
                            "12+ | {1}{B}, {T}, Pay 2 life, Sacrifice a creature: Draw \
                             cards equal to the sacrificed creature's power. Activate only \
                             as a sorcery.",
                            &[
                                CostDef::Mana(mana_cost!("{1}{B}")),
                                CostDef::TapSource,
                                CostDef::PayLife(2),
                                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                            ],
                            abilities::draw_cards(ValueDef::AggregateObjectValues(
                                &ObjectValueAggregateDef {
                                    objects: ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                                        AdditionalCostObjectIndex::PRIMARY,
                                    )),
                                    select: ObjectValueDef::Power,
                                    operation: AggregateOperationDef::Sum,
                                },
                            )),
                        )
                        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 260 — Uthros, Titanic Godcore
pub(in crate::card::sets) static UTHROS_TITANIC_GODCORE: CardRecord = CardRecord::new(
    "Uthros, Titanic Godcore",
    "11da39d6-cfa6-498d-91b1-11454cc7e5a3",
    "Adam Paquette",
    CardRules::new_land(&["Planet"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Blue),
        station(
            "Station (Tap another creature you control: Put charge \
             counters equal to its power on this Planet. Station only as a \
             sorcery.)",
        ),
        AbilityDef::static_ability(
            "12+ | {U}, {T}: Add {U} for each artifact you control.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("charge"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 12,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(
                        &AbilityDef::activated_mana(
                            "12+ | {U}, {T}: Add {U} for each artifact you control.",
                            &[CostDef::Mana(mana_cost!("{U}")), CostDef::TapSource],
                            EffectDef::AddMana(
                                AddManaEffectDef::one(ManaColor::Blue).with_variable_amount(
                                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    )),
                                ),
                            ),
                        ),
                    )]),
                },
            },
        ),
    ]),
);

// EOE 261 — Watery Grave (reprint)
const WATERY_GRAVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::WATERY_GRAVE,
    "5b8170dc-6a90-46fc-9989-7575f3d402b5",
    "Sergey Glushakov",
);

// EOE 262 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "089ae01b-e042-4255-b0ee-17d8f416a8d9",
    "Adam Paquette",
);

// EOE 263 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "76313c69-8ec7-49e3-a34e-ade26097284c",
    "Adam Paquette",
);

// EOE 264 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "3e724e54-a622-4c77-8183-5a397a7c14a9",
    "Adam Paquette",
);

// EOE 265 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "3204f401-1bcb-4d97-b15d-113a5d3c3e9f",
    "Adam Paquette",
);

// EOE 266 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "8335b9f1-e726-423b-8ba7-6151448ab3fd",
    "Adam Paquette",
);

// EOE 267 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "df31f72c-076e-4d8e-9975-6d6281ab71f6",
    "Alayna Danner",
);

// EOE 268 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "a26fd28e-d053-48b3-ab4c-2dc6fd33fa64",
    "Sergey Glushakov",
);

// EOE 269 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "aaa212be-1db5-4493-86c8-c01d29348e31",
    "Sergey Glushakov",
);

// EOE 270 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "2fb9ecfd-8025-48c0-a306-b4deebde2949",
    "Liiga Smilshkalne",
);

// EOE 271 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "58496a56-437f-4204-9691-f63795e5cdab",
    "Sergey Glushakov",
);

// EOE 272 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "07ab266b-ad6a-422e-9277-27d9f48d2c29",
    "Liiga Smilshkalne",
);

// EOE 273 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "5cd39b01-9a06-4575-9c63-9fb3ba9ef101",
    "Sergey Glushakov",
);

// EOE 274 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "de0ac2b0-175b-4e71-9172-ce2a72234818",
    "Julian Kok Joon Wen",
);

// EOE 275 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "fb62605c-a58e-4e53-8336-b2bee316b5a6",
    "Sergey Glushakov",
);

// EOE 276 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "673141ec-826f-4132-8282-d499990495a7",
    "Julian Kok Joon Wen",
);

// EOE 277 — Adagia, Windswept Bastion (alternate printing)
const ADAGIA_WINDSWEPT_BASTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ADAGIA_WINDSWEPT_BASTION,
    1,
    "4d6901e7-d519-4dfc-a74d-63fff5e4ffae",
    "Piotr Dura",
);

// EOE 278 — Breeding Pool (alternate printing)
const BREEDING_POOL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::BREEDING_POOL,
    1,
    "1c575871-9b46-4cd4-8596-54dc04f76456",
    "Chris Ostrowski",
);

// EOE 279 — Evendo, Waking Haven (alternate printing)
const EVENDO_WAKING_HAVEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EVENDO_WAKING_HAVEN,
    1,
    "91d18bf4-0e95-41f7-a65d-238fc010fd82",
    "Piotr Dura",
);

// EOE 280 — Godless Shrine (alternate printing)
const GODLESS_SHRINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::GODLESS_SHRINE,
    1,
    "7bb18340-5507-44da-b008-6b010daad617",
    "Chris Ostrowski",
);

// EOE 281 — Kavaron, Memorial World (alternate printing)
const KAVARON_MEMORIAL_WORLD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAVARON_MEMORIAL_WORLD,
    1,
    "0f03d025-3ec8-4ae4-93e3-0fb96e105b7f",
    "Piotr Dura",
);

// EOE 282 — Sacred Foundry (alternate printing)
const SACRED_FOUNDRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::SACRED_FOUNDRY,
    1,
    "9f74d8ad-9db9-4162-9d20-9779233a634e",
    "Chris Ostrowski",
);

// EOE 283 — Stomping Ground (alternate printing)
const STOMPING_GROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::STOMPING_GROUND,
    1,
    "f802db1f-6533-4118-b677-562bfb135904",
    "Chris Ostrowski",
);

// EOE 284 — Susur Secundi, Void Altar (alternate printing)
const SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUSUR_SECUNDI_VOID_ALTAR,
    1,
    "f6bb05de-4cf5-479d-a0f7-1a2792a6b6d0",
    "Piotr Dura",
);

// EOE 285 — Uthros, Titanic Godcore (alternate printing)
const UTHROS_TITANIC_GODCORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UTHROS_TITANIC_GODCORE,
    1,
    "a76ecc4f-5e77-4d96-9df7-fe703bbc39df",
    "Piotr Dura",
);

// EOE 286 — Watery Grave (alternate printing)
const WATERY_GRAVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::WATERY_GRAVE,
    1,
    "b248ff3a-e10d-4797-8c04-7f4094608d32",
    "Chris Ostrowski",
);

// EOE 287 — Tezzeret, Cruel Captain (alternate printing)
const TEZZERET_CRUEL_CAPTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEZZERET_CRUEL_CAPTAIN,
    1,
    "5cf53baa-b0c3-4190-a5c6-c141d54cff32",
    "Magali Villeneuve",
);

// EOE 288 — Astelli Reclaimer (alternate printing)
const ASTELLI_RECLAIMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASTELLI_RECLAIMER,
    1,
    "f01aaada-2525-4f92-83e8-bbc61fa7b9cd",
    "Benjamin Ee",
);

// EOE 289 — Haliya, Guided by Light (alternate printing)
const HALIYA_GUIDED_BY_LIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HALIYA_GUIDED_BY_LIGHT,
    1,
    "e97f7f80-51f0-4366-a094-ee75ce03adb4",
    "Eleonor Piteira",
);

// EOE 290 — Mm'menon, the Right Hand (alternate printing)
const MM_MENON_THE_RIGHT_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MM_MENON_THE_RIGHT_HAND,
    1,
    "53bd9149-e157-4f0b-becf-0b64f47dbad3",
    "Dominik Mayer",
);

// EOE 291 — Starwinder (alternate printing)
const STARWINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARWINDER,
    1,
    "637a4457-5600-4d33-81c7-f4009df3d8a5",
    "Justin Hernandez & Alexis Hernandez",
);

// EOE 292 — Alpharael, Stonechosen (alternate printing)
const ALPHARAEL_STONECHOSEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALPHARAEL_STONECHOSEN,
    1,
    "cabf4bdb-160a-4d0e-aa07-87bb74aef34a",
    "Jeremy Wilson",
);

// EOE 293 — Elegy Acolyte (alternate printing)
const ELEGY_ACOLYTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELEGY_ACOLYTE,
    1,
    "ffa43efe-15ea-4c38-ab4a-764baa084e44",
    "Justin Hernandez & Alexis Hernandez",
);

// EOE 294 — Xu-Ifit, Osteoharmonist (alternate printing)
const XU_IFIT_OSTEOHARMONIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &XU_IFIT_OSTEOHARMONIST,
    1,
    "98e7fb9e-44c2-4fa6-8d81-895f909ea9b7",
    "Ashley Mackenzie",
);

// EOE 295 — Possibility Technician (alternate printing)
const POSSIBILITY_TECHNICIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POSSIBILITY_TECHNICIAN,
    1,
    "d68ed960-c7ea-454c-8395-6cbb621a7940",
    "Matthew G. Lewis",
);

// EOE 296 — Tannuk, Steadfast Second (alternate printing)
const TANNUK_STEADFAST_SECOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TANNUK_STEADFAST_SECOND,
    1,
    "a94a4760-cf43-4f27-96e1-be9d13611d99",
    "Pascal Blanché",
);

// EOE 297 — Mightform Harmonizer
// Audit: unsupported — Needs Warp to install its delayed exile from the resolving spell, without an extra counterable enters trigger, and owner cast permission that starts only after the exile turn has ended; the existing Warp helper installs an enters trigger and grants permission immediately.
pub(in crate::card::sets) static MIGHTFORM_HARMONIZER: CardRecord = CardRecord::new(
    "Mightform Harmonizer",
    "29bc9be4-4fc3-440a-a851-0c7f8989c9b5",
    "Jessica Fong",
    CardRules::unsupported(),
);

// EOE 298 — Dyadrine, Synthesis Amalgam (alternate printing)
const DYADRINE_SYNTHESIS_AMALGAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DYADRINE_SYNTHESIS_AMALGAM,
    1,
    "3cba1ffa-9b43-41f9-a5ea-57cd48013c1c",
    "Matthew G. Lewis",
);

// EOE 299 — Genemorph Imago (alternate printing)
const GENEMORPH_IMAGO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENEMORPH_IMAGO,
    1,
    "37c2ab54-e856-41d5-86ba-c6a7d2774546",
    "Benjamin Ee",
);

// EOE 300 — Ragost, Deft Gastronaut (alternate printing)
const RAGOST_DEFT_GASTRONAUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGOST_DEFT_GASTRONAUT,
    1,
    "4a62293d-63a7-4038-b3e9-197bb2b9ec90",
    "Dominik Mayer",
);

// EOE 301 — Sami, Wildcat Captain (alternate printing)
const SAMI_WILDCAT_CAPTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAMI_WILDCAT_CAPTAIN,
    1,
    "968fb6d4-f4b9-4c8d-b296-fea002a8c003",
    "Ashley Mackenzie",
);

// EOE 302 — Syr Vondam, Sunstar Exemplar (alternate printing)
const SYR_VONDAM_SUNSTAR_EXEMPLAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYR_VONDAM_SUNSTAR_EXEMPLAR,
    1,
    "9c39b081-ad53-4c43-93f4-3a4ebb24299b",
    "Jeremy Wilson",
);

// EOE 303 — Beyond the Quiet (alternate printing)
const BEYOND_THE_QUIET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEYOND_THE_QUIET,
    1,
    "215f488b-6d33-4b08-88ff-b258c00515d6",
    "Serena Malyon",
);

// EOE 304 — Cosmogrand Zenith (alternate printing)
const COSMOGRAND_ZENITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMOGRAND_ZENITH,
    1,
    "d68d891d-333f-46c9-b5a3-3b1d4d3e4563",
    "Marlene Yui",
);

// EOE 305 — Quantum Riddler (alternate printing)
const QUANTUM_RIDDLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUANTUM_RIDDLER,
    1,
    "2b0e16d5-a3e5-4e58-9f97-3d967618f015",
    "Cacho Rubione",
);

// EOE 306 — Starwinder (alternate printing)
const STARWINDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STARWINDER,
    2,
    "5a9c1d90-b2a3-40a9-86d7-ce6930acee0e",
    "Cacho Rubione",
);

// EOE 307 — Archenemy's Charm (alternate printing)
const ARCHENEMY_S_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHENEMY_S_CHARM,
    1,
    "c9e9b71b-a54c-4891-9c16-28aebfafb644",
    "Peter Diamond",
);

// EOE 308 — Devastating Onslaught (alternate printing)
const DEVASTATING_ONSLAUGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEVASTATING_ONSLAUGHT,
    1,
    "376d4b86-b530-4063-b530-87cc5c32e2fa",
    "Deb JJ Lee",
);

// EOE 309 — Nova Hellkite (alternate printing)
const NOVA_HELLKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NOVA_HELLKITE,
    1,
    "c627d558-98e8-459e-860b-7ac4dfac44de",
    "Micha Huigen",
);

// EOE 310 — Rust Harvester (alternate printing)
const RUST_HARVESTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUST_HARVESTER,
    1,
    "7aca78c5-3ee4-49b8-a2bd-99f5ace9d9f7",
    "Scott Balmer",
);

// EOE 311 — Weapons Manufacturing (alternate printing)
const WEAPONS_MANUFACTURING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEAPONS_MANUFACTURING,
    1,
    "b2c86408-5e19-46ca-a3ce-949eb148acde",
    "Micha Huigen",
);

// EOE 312 — Terrasymbiosis (alternate printing)
const TERRASYMBIOSIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERRASYMBIOSIS,
    1,
    "cc708b6c-6224-42b7-8249-a78fcb31fef4",
    "Jack Hughes",
);

// EOE 313 — Cosmogoyf (alternate printing)
const COSMOGOYF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMOGOYF,
    1,
    "15b98944-23c9-4609-b323-7925adaa9aac",
    "Princess Hidir",
);

// EOE 314 — Mutinous Massacre (alternate printing)
const MUTINOUS_MASSACRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MUTINOUS_MASSACRE,
    1,
    "b4bdb1ac-abe2-4ce7-84cb-eb53196b4b01",
    "Jack Hughes",
);

// EOE 315 — Space-Time Anomaly (alternate printing)
const SPACE_TIME_ANOMALY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPACE_TIME_ANOMALY,
    1,
    "b2e78e22-7a19-4e3a-93c5-6432bc5922ae",
    "Princess Hidir",
);

// EOE 316 — Secluded Starforge (alternate printing)
const SECLUDED_STARFORGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SECLUDED_STARFORGE,
    1,
    "9024ae78-8a60-4856-9d26-97e3d9310542",
    "Jaime A. Zuverza",
);

// EOE 317 — Anticausal Vestige (alternate printing)
const ANTICAUSAL_VESTIGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTICAUSAL_VESTIGE,
    1,
    "fb9ecadf-6145-49e7-972f-bfe28ca92266",
    "Chase Stone",
);

// EOE 318 — Exalted Sunborn (alternate printing)
const EXALTED_SUNBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXALTED_SUNBORN,
    1,
    "c295983e-e8eb-4adc-8c72-9c3ecf3a29df",
    "Scott M. Fischer",
);

// EOE 319 — Hardlight Containment (alternate printing)
const HARDLIGHT_CONTAINMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARDLIGHT_CONTAINMENT,
    1,
    "a84523b1-bfa4-4506-96d3-f655727f1cfc",
    "Dominik Mayer",
);

// EOE 320 — Lightstall Inquisitor (alternate printing)
const LIGHTSTALL_INQUISITOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIGHTSTALL_INQUISITOR,
    1,
    "ad782559-9b45-4a87-8ae6-82be6fd32f76",
    "Arif Wijaya",
);

// EOE 321 — Lumen-Class Frigate (alternate printing)
const LUMEN_CLASS_FRIGATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUMEN_CLASS_FRIGATE,
    1,
    "7fffe390-d16c-4891-b94a-99e895f219d0",
    "Zezhou Chen",
);

// EOE 322 — Pinnacle Starcage (alternate printing)
const PINNACLE_STARCAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PINNACLE_STARCAGE,
    1,
    "884f3feb-994f-4a65-b0e1-958e2fb38af1",
    "Leon Tukker",
);

// EOE 323 — The Seriema (alternate printing)
const THE_SERIEMA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SERIEMA,
    1,
    "9361996c-414b-4d0d-9d94-b6d5fe04e987",
    "Sergey Glushakov",
);

// EOE 324 — Sunstar Chaplain (alternate printing)
const SUNSTAR_CHAPLAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNSTAR_CHAPLAIN,
    1,
    "d1588748-3398-4b9d-97a6-8266f11d0862",
    "Valera Lutfullina",
);

// EOE 325 — Consult the Star Charts (alternate printing)
const CONSULT_THE_STAR_CHARTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONSULT_THE_STAR_CHARTS,
    1,
    "94d11159-c389-4461-abf9-2a3be6b39d8d",
    "Antonio José Manzanedo",
);

// EOE 326 — Emissary Escort (alternate printing)
const EMISSARY_ESCORT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMISSARY_ESCORT,
    1,
    "c2dca493-9f1e-479a-a1d2-893d4799be3e",
    "Igor Grechanyi",
);

// EOE 327 — Moonlit Meditation (alternate printing)
const MOONLIT_MEDITATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONLIT_MEDITATION,
    1,
    "9763f489-6b2b-4a19-84a4-1fc602124890",
    "Liiga Smilshkalne",
);

// EOE 328 — Starfield Vocalist (alternate printing)
const STARFIELD_VOCALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_VOCALIST,
    1,
    "9831b27e-90f5-4799-8b87-644a91b853e6",
    "Nathaniel Himawan",
);

// EOE 329 — Synthesizer Labship (alternate printing)
const SYNTHESIZER_LABSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYNTHESIZER_LABSHIP,
    1,
    "85bbbbed-1dbb-4c85-b585-cde6713890ef",
    "Adrián Rodríguez Pérez",
);

// EOE 330 — Weftwalking (alternate printing)
const WEFTWALKING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEFTWALKING,
    1,
    "59a862ac-ef3b-41ff-abb7-d09879e9a7b1",
    "Rovina Cai",
);

// EOE 331 — Chorale of the Void (alternate printing)
const CHORALE_OF_THE_VOID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHORALE_OF_THE_VOID,
    1,
    "6fda7329-0a1e-4921-bc6f-f2f4f1544878",
    "Alix Branwyn",
);

// EOE 332 — Entropic Battlecruiser (alternate printing)
const ENTROPIC_BATTLECRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENTROPIC_BATTLECRUISER,
    1,
    "c3cf23c5-cef9-4d78-9f09-a737e59b8d70",
    "Josiah \"Jo\" Cameron",
);

// EOE 333 — Requiem Monolith (alternate printing)
const REQUIEM_MONOLITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REQUIEM_MONOLITH,
    1,
    "2c44310c-80e2-4398-9b7a-097eca86ee48",
    "Warren Mahy",
);

// EOE 334 — Sunset Saboteur (alternate printing)
const SUNSET_SABOTEUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNSET_SABOTEUR,
    1,
    "2e75cf29-a084-4ff7-ae58-eb267befa293",
    "Mirko Failoni",
);

// EOE 335 — Zero Point Ballad (alternate printing)
const ZERO_POINT_BALLAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZERO_POINT_BALLAD,
    1,
    "c56ea5ca-f9b3-4ffa-9d2b-0190ab43cabc",
    "David Astruga",
);

// EOE 336 — Memorial Vault (alternate printing)
const MEMORIAL_VAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEMORIAL_VAULT,
    1,
    "6b4cf1fb-9a56-4d75-a42e-5a92bcfe5b08",
    "Javier Charro",
);

// EOE 337 — Pain for All (alternate printing)
const PAIN_FOR_ALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PAIN_FOR_ALL,
    1,
    "2a9e9951-7ded-41de-808f-8a0fea964307",
    "Dmitry Burmak",
);

// EOE 338 — Terminal Velocity (alternate printing)
const TERMINAL_VELOCITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERMINAL_VELOCITY,
    1,
    "d36147a8-dfcd-4d56-a98d-72fee52578d6",
    "Xabi Gaztelua",
);

// EOE 339 — Warmaker Gunship (alternate printing)
const WARMAKER_GUNSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARMAKER_GUNSHIP,
    1,
    "515c0b63-c710-4419-a84d-4777d874bf19",
    "Julian Kok Joon Wen",
);

// EOE 340 — Bioengineered Future (alternate printing)
const BIOENGINEERED_FUTURE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BIOENGINEERED_FUTURE,
    1,
    "8064eb67-5397-4a25-a02e-f2a0a3e839aa",
    "Constantin Marin",
);

// EOE 341 — Famished Worldsire (alternate printing)
const FAMISHED_WORLDSIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAMISHED_WORLDSIRE,
    1,
    "4253d6f1-f568-4419-8b5c-a2e8855057c7",
    "Kev Walker",
);

// EOE 342 — Frenzied Baloth (alternate printing)
const FRENZIED_BALOTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRENZIED_BALOTH,
    1,
    "dbd6cfbd-1cb7-4b15-9d03-f07b3a874e2d",
    "Diana Franco",
);

// EOE 343 — Icetill Explorer (alternate printing)
const ICETILL_EXPLORER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ICETILL_EXPLORER,
    2,
    "f63de2e4-f094-4b10-b754-1285ad64effc",
    "Warren Mahy",
);

// EOE 344 — Loading Zone (alternate printing)
const LOADING_ZONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOADING_ZONE,
    1,
    "d2111f0f-924e-4293-ba1d-15ae31088bb7",
    "Matt Stewart",
);

// EOE 345 — Ouroboroid (alternate printing)
const OUROBOROID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OUROBOROID,
    1,
    "9a6eb356-7418-443f-a51b-37ebc3526fce",
    "Samuel Perin",
);

// EOE 346 — Sledge-Class Seedship (alternate printing)
const SLEDGE_CLASS_SEEDSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLEDGE_CLASS_SEEDSHIP,
    1,
    "4d0c21a9-033c-42af-acbe-84284e0849f7",
    "Leon Tukker",
);

// EOE 347 — Biotech Specialist (alternate printing)
const BIOTECH_SPECIALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BIOTECH_SPECIALIST,
    1,
    "97dc831c-bd78-4c21-a1f3-24794db8eb5f",
    "Alexandre Honoré",
);

// EOE 348 — Infinite Guideline Station (alternate printing)
const INFINITE_GUIDELINE_STATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INFINITE_GUIDELINE_STATION,
    1,
    "334654be-c433-43b6-bf33-8ba040c3c3a8",
    "Piotr Dura",
);

// EOE 349 — Pinnacle Emissary (alternate printing)
const PINNACLE_EMISSARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PINNACLE_EMISSARY,
    1,
    "2b755ebd-a194-4b78-b1e8-606a7d231598",
    "Alejandro Pacheco",
);

// EOE 350 — Singularity Rupture (alternate printing)
const SINGULARITY_RUPTURE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SINGULARITY_RUPTURE,
    1,
    "e88877c1-0a77-4730-8bc3-5ca0dbdd4566",
    "Liiga Smilshkalne",
);

// EOE 351 — Dawnsire, Sunstar Dreadnought (alternate printing)
const DAWNSIRE_SUNSTAR_DREADNOUGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAWNSIRE_SUNSTAR_DREADNOUGHT,
    1,
    "8a242c45-07a7-48bf-8cd0-159fd9f5ecbc",
    "Jaime Jones",
);

// EOE 352 — The Dominion Bracelet (alternate printing)
const THE_DOMINION_BRACELET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DOMINION_BRACELET,
    1,
    "bb1492cf-b56f-40c7-a52b-7e354bcadc1d",
    "Nathaniel Himawan",
);

// EOE 353 — The Endstone (alternate printing)
const THE_ENDSTONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_ENDSTONE,
    2,
    "62a08de0-27d6-466b-8ba8-a1bb7719049d",
    "Ryan Pancoast",
);

// EOE 354 — The Eternity Elevator (alternate printing)
const THE_ETERNITY_ELEVATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ETERNITY_ELEVATOR,
    1,
    "23d2c721-42a6-4108-91c2-93f59b645313",
    "Josu Solano",
);

// EOE 355 — Extinguisher Battleship (alternate printing)
const EXTINGUISHER_BATTLESHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXTINGUISHER_BATTLESHIP,
    1,
    "5062b3a3-0ec6-4c1b-9394-e2f2e2301c43",
    "Danny Schwartz",
);

// EOE 356 — Thrumming Hivepool (alternate printing)
const THRUMMING_HIVEPOOL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRUMMING_HIVEPOOL,
    1,
    "ef76c1af-394d-4ab6-b89f-d0c6009c8299",
    "Rob Rey",
);

// EOE 357 — Anticausal Vestige (alternate printing)
const ANTICAUSAL_VESTIGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ANTICAUSAL_VESTIGE,
    2,
    "a8c94ccb-f6af-4a26-b773-624d6ce54330",
    "Nottsuo",
);

// EOE 358 — Exalted Sunborn (alternate printing)
const EXALTED_SUNBORN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXALTED_SUNBORN,
    2,
    "59183da2-8315-49f4-ab87-01b957b26366",
    "Makoron",
);

// EOE 359 — Starfield Vocalist (alternate printing)
const STARFIELD_VOCALIST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_VOCALIST,
    2,
    "eb6f8c8c-1905-4c3c-aa03-d56bce983559",
    "nina",
);

// EOE 360 — Sothera, the Supervoid (alternate printing)
const SOTHERA_THE_SUPERVOID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOTHERA_THE_SUPERVOID,
    1,
    "80d0b4ed-116a-445a-a571-7ebfc1983654",
    "Mateusz Urbanowicz",
);

// EOE 361 — Devastating Onslaught (alternate printing)
const DEVASTATING_ONSLAUGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEVASTATING_ONSLAUGHT,
    2,
    "3d043992-251c-40cb-b1bc-481ba2830cfe",
    "Naochika Morishita",
);

// EOE 362 — Icetill Explorer
pub(in crate::card::sets) static ICETILL_EXPLORER: CardRecord = CardRecord::new(
    "Icetill Explorer",
    "895e5e9b-84dd-4741-8a2c-442165ea9b15",
    "Raimaru",
    // Four mana for a 2/4 whose three clauses feed each other: the extra
    // land drop wants lands, the mill finds them, and the graveyard is
    // where the mill puts them.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect", "Scout"], 2, 4).with_abilities(&[
        AbilityDef::static_ability(
            "You may play an additional land on each of your turns.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
            },
        ),
        AbilityDef::static_ability(
            "You may play lands from your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                    // Lands only, played the ordinary way: what the permission adds is the
                    // zone, not a way of casting anything out of it.
                    GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                        PlayActionMatcherDef::PlayLand,
                        ObjectPredicateDef::HasType(CardType::Land),
                    )),
                )),
            },
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, mill a card.",
            TriggerEventDef::zone_changed(
                // A land you control arriving, which is what landfall is: a land somebody
                // else plays is not one, and the mill is what turns the extra land drop
                // into more lands to play.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 363 — Mutinous Massacre (alternate printing)
const MUTINOUS_MASSACRE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MUTINOUS_MASSACRE,
    2,
    "b446c5a9-0963-474c-a4d6-13da808dbbc1",
    "Aogachou",
);

// EOE 364 — The Dominion Bracelet (alternate printing)
const THE_DOMINION_BRACELET_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_DOMINION_BRACELET,
    2,
    "2aea7e3a-7dc4-4770-a22e-8ab29f494844",
    "Mai Minamiura",
);

// EOE 365 — The Endstone (alternate printing)
const THE_ENDSTONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_ENDSTONE,
    3,
    "c9f2599c-9fec-4392-8228-03acfdee50e5",
    "Hidetaka Tenjin",
);

// EOE 366 — Secluded Starforge (alternate printing)
const SECLUDED_STARFORGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SECLUDED_STARFORGE,
    2,
    "ec78b16c-be63-4aef-9308-f7a093f032a7",
    "Makoto Yukimura",
);

// EOE 367 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "926aad15-87a8-4510-b327-d1648f89c497",
    "Adam Paquette",
);

// EOE 368 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "81368899-5fef-4f10-80ea-b282eca0f42f",
    "Adam Paquette",
);

// EOE 369 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "b938ac10-bd0f-4ce3-a743-958d5beadf58",
    "Adam Paquette",
);

// EOE 370 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "6b092822-f34f-4384-9d0a-23d863d27231",
    "Adam Paquette",
);

// EOE 371 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "0d6250d3-728b-4412-8efc-911bb6f5e910",
    "Adam Paquette",
);

// EOE 372 — Adagia, Windswept Bastion (alternate printing)
const ADAGIA_WINDSWEPT_BASTION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ADAGIA_WINDSWEPT_BASTION,
    2,
    "bf314021-ba15-46c2-99e0-94141f118bc9",
    "Piotr Dura",
);

// EOE 373 — Breeding Pool (alternate printing)
const BREEDING_POOL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::BREEDING_POOL,
    2,
    "fda15a78-a370-4cc6-a4df-92200d6ca826",
    "Chris Ostrowski",
);

// EOE 374 — Evendo, Waking Haven (alternate printing)
const EVENDO_WAKING_HAVEN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EVENDO_WAKING_HAVEN,
    2,
    "064255cc-445f-4794-aa64-3aee34e11181",
    "Piotr Dura",
);

// EOE 375 — Godless Shrine (alternate printing)
const GODLESS_SHRINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::GODLESS_SHRINE,
    2,
    "da9708aa-c4db-4ed5-bcf2-29ea15af7d8b",
    "Chris Ostrowski",
);

// EOE 376 — Kavaron, Memorial World (alternate printing)
const KAVARON_MEMORIAL_WORLD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KAVARON_MEMORIAL_WORLD,
    2,
    "4d5a29a4-98f6-4de9-b685-120e26c8b785",
    "Piotr Dura",
);

// EOE 377 — Sacred Foundry (alternate printing)
const SACRED_FOUNDRY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::SACRED_FOUNDRY,
    2,
    "15e9f2fe-7703-42bf-a6f7-c4db6e62621f",
    "Chris Ostrowski",
);

// EOE 378 — Stomping Ground (alternate printing)
const STOMPING_GROUND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::STOMPING_GROUND,
    2,
    "3ba7e782-3c6a-4f3b-b00d-cf982ad31376",
    "Chris Ostrowski",
);

// EOE 379 — Susur Secundi, Void Altar (alternate printing)
const SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SUSUR_SECUNDI_VOID_ALTAR,
    2,
    "dd59063a-8ad7-4efa-8f40-7b3c9ccc68f1",
    "Piotr Dura",
);

// EOE 380 — Uthros, Titanic Godcore (alternate printing)
const UTHROS_TITANIC_GODCORE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &UTHROS_TITANIC_GODCORE,
    2,
    "634bccd9-ab66-4620-a7b1-b9c984641558",
    "Piotr Dura",
);

// EOE 381 — Watery Grave (alternate printing)
const WATERY_GRAVE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::WATERY_GRAVE,
    2,
    "d854480d-a163-422b-a3dc-54dffd7b3eab",
    "Chris Ostrowski",
);

// EOE 382 — Sothera, the Supervoid (alternate printing)
const SOTHERA_THE_SUPERVOID_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SOTHERA_THE_SUPERVOID,
    2,
    "07b10f1b-b03b-4f19-bc7e-69d5eaa5ff07",
    "Micha Huigen",
);

// EOE 383 — Anticausal Vestige (alternate printing)
const ANTICAUSAL_VESTIGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ANTICAUSAL_VESTIGE,
    3,
    "c0a00a77-1f7b-4528-aaf1-b2d78e73a604",
    "Nottsuo",
);

// EOE 384 — Exalted Sunborn (alternate printing)
const EXALTED_SUNBORN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &EXALTED_SUNBORN,
    3,
    "a827ede4-eec6-4a9d-b491-cddfab6aa39b",
    "Makoron",
);

// EOE 385 — Starfield Vocalist (alternate printing)
const STARFIELD_VOCALIST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_VOCALIST,
    3,
    "d8a6759d-e858-41d0-a5e3-a2ac1d074263",
    "nina",
);

// EOE 386 — Sothera, the Supervoid (alternate printing)
const SOTHERA_THE_SUPERVOID_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SOTHERA_THE_SUPERVOID,
    3,
    "668b1796-2722-4d69-9a41-8067ad189ad6",
    "Mateusz Urbanowicz",
);

// EOE 387 — Devastating Onslaught (alternate printing)
const DEVASTATING_ONSLAUGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEVASTATING_ONSLAUGHT,
    3,
    "2dedcdcd-4dbf-4bc1-9de3-823f0c7e3961",
    "Naochika Morishita",
);

// EOE 388 — Icetill Explorer (alternate printing)
const ICETILL_EXPLORER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ICETILL_EXPLORER,
    3,
    "b2ace91b-0329-4511-bf45-cb72e0ebeae0",
    "Raimaru",
);

// EOE 389 — Mutinous Massacre (alternate printing)
const MUTINOUS_MASSACRE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MUTINOUS_MASSACRE,
    3,
    "8f319e32-56e2-402e-8e99-3cf2953ae19d",
    "Aogachou",
);

// EOE 390 — The Dominion Bracelet (alternate printing)
const THE_DOMINION_BRACELET_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_DOMINION_BRACELET,
    3,
    "dac293bd-0724-40e9-856e-a6a0483565ca",
    "Mai Minamiura",
);

// EOE 391 — The Endstone
pub(in crate::card::sets) static THE_ENDSTONE: CardRecord = CardRecord::new(
    "The Endstone",
    "1227eb7f-c2a5-4112-98d0-70275a63c26a",
    "Hidetaka Tenjin",
// Seven mana that draws a card for everything you do and hands the ten
    // life back every end step, which is what makes the seven payable.
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you play a land or cast a spell, draw a card.",
                // One ability with two events rather than two abilities: the card prints
                // one, and a turn with a land and a spell in it draws twice either way.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::LandPlayed {
                        land: ObjectPredicateDef::Any,
                        player: PlayerRelation::You,
                    },
                    TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                ]),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, your life total becomes half your starting life \
                 total, rounded up.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::SetLifeTotal {
                    recipient: EffectRecipientDef::Controller,
                    // Half of what the game began on rather than half of what is left: it sets
                    // the total to the same number every end step, which is a gain from below
                    // it and a loss from above.
                    total: ValueDef::Halved(&HalvedValueDef::new(
                        ValueDef::StartingLifeTotal,
                        RoundingDef::Up,
                    )),
                },
            ),
        ]),
);

// EOE 392 — Secluded Starforge (alternate printing)
const SECLUDED_STARFORGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SECLUDED_STARFORGE,
    3,
    "492a1bc6-6f4b-400c-959f-1bfef03a68c8",
    "Makoto Yukimura",
);

// EOE 393 — Starfield Shepherd (alternate printing)
const STARFIELD_SHEPHERD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_SHEPHERD,
    1,
    "67dfd9e5-bd2e-4140-91c0-da33dc7f46e4",
    "Marta Nael",
);

// EOE 394 — Annul (alternate printing)
const ANNUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ANNUL,
    1,
    "26dd02c4-569b-437f-b9dd-bd2f1d86a968",
    "Carlos Palma Cruchaga",
);

// EOE 395 — Umbral Collar Zealot (alternate printing)
const UMBRAL_COLLAR_ZEALOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UMBRAL_COLLAR_ZEALOT,
    1,
    "9b14b659-9b4d-4099-90a0-d2a673b86648",
    "Dmitry Burmak",
);

// EOE 396 — Kavaron Harrier (alternate printing)
const KAVARON_HARRIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAVARON_HARRIER,
    1,
    "57264a8e-764a-46e6-afea-747f1b3dbf23",
    "Hardy Fowler",
);

// EOE 397 — Pull Through the Weft (alternate printing)
const PULL_THROUGH_THE_WEFT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PULL_THROUGH_THE_WEFT,
    1,
    "07404acc-4f25-45e6-8174-683f296daa8a",
    "Andrew Mar",
);

// EOE 398 — Singularity Rupture (alternate printing)
const SINGULARITY_RUPTURE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SINGULARITY_RUPTURE,
    2,
    "422d7bc1-e2a2-4cfc-a29d-9d67354cdafd",
    "Néstor Ossandón Leal",
);

// EOE 399 — Emissary Escort (alternate printing)
const EMISSARY_ESCORT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EMISSARY_ESCORT,
    2,
    "f32df6a8-d77f-40ce-bcac-9495db458210",
    "Lius Lasahido",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANTICAUSAL_VESTIGE,
    &TEZZERET_CRUEL_CAPTAIN,
    &ALL_FATES_STALKER,
    &ASTELLI_RECLAIMER,
    &AUXILIARY_BOOSTERS,
    &BEYOND_THE_QUIET,
    &BRIGHTSPEAR_ZEALOT,
    &COSMOGRAND_ZENITH,
    &DAWNSTRIKE_VANGUARD,
    &DOCKWORKER_DRONE,
    &DUAL_SUN_ADEPTS,
    &DUAL_SUN_TECHNIQUE,
    &EMERGENCY_EJECT,
    &EXALTED_SUNBORN,
    &EXOSUIT_SAVIOR,
    &FLIGHT_DECK_COORDINATOR,
    &FOCUS_FIRE,
    &HALIYA_GUIDED_BY_LIGHT,
    &HARDLIGHT_CONTAINMENT,
    &HONOR,
    &HONORED_KNIGHT_CAPTAIN,
    &KNIGHT_LUMINARY,
    &LIGHTSTALL_INQUISITOR,
    &LUMEN_CLASS_FRIGATE,
    &LUXKNIGHT_BREACHER,
    &PINNACLE_STARCAGE,
    &PULSAR_SQUADRON_ACE,
    &RADIANT_STRIKE,
    &RAYBLADE_TROOPER,
    &REROUTE_SYSTEMS,
    &RESCUE_SKIFF,
    &SCOUT_FOR_SURVIVORS,
    &SEAM_RIP,
    &THE_SERIEMA,
    &SQUIRE_S_LIGHTBLADE,
    &STARFIELD_SHEPHERD,
    &STARFIGHTER_PILOT,
    &STARPORT_SECURITY,
    &SUNSTAR_CHAPLAIN,
    &SUNSTAR_EXPANSIONIST,
    &SUNSTAR_LIGHTSMITH,
    &WEDGELIGHT_RAMMER,
    &WEFTBLADE_ENHANCER,
    &ZEALOUS_DISPLAY,
    &ATOMIC_MICROSIZER,
    &CEREBRAL_DOWNLOAD,
    &CLOUDSCULPT_TECHNICIAN,
    &CODECRACKER_HOUND,
    &CONSULT_THE_STAR_CHARTS,
    &CRYOGEN_RELIC,
    &CRYOSHATTER,
    &DESCULPTING_BLAST,
    &DIVERT_DISASTER,
    &EMISSARY_ESCORT,
    &GIGASTORM_TITAN,
    &ILLVOI_GALEBLADE,
    &ILLVOI_INFILTRATOR,
    &ILLVOI_LIGHT_JAMMER,
    &ILLVOI_OPERATIVE,
    &LOST_IN_SPACE,
    &MECHAN_ASSEMBLER,
    &MECHAN_NAVIGATOR,
    &MECHAN_SHIELDMATE,
    &MECHANOZOA,
    &MENTAL_MODULATION,
    &MM_MENON_THE_RIGHT_HAND,
    &MOONLIT_MEDITATION,
    &MOUTH_OF_THE_STORM,
    &NANOFORM_SENTINEL,
    &QUANTUM_RIDDLER,
    &SCOUR_FOR_SCRAP,
    &SELFCRAFT_MECHAN,
    &SINISTER_CRYOLOGIST,
    &SPECIMEN_FREIGHTER,
    &STARBREACH_WHALE,
    &STARFIELD_VOCALIST,
    &STARWINDER,
    &STEELSWARM_OPERATOR,
    &SYNTHESIZER_LABSHIP,
    &TRACTOR_BEAM,
    &UNRAVEL,
    &UTHROS_PSIONICIST,
    &UTHROS_SCANSHIP,
    &WEFTWALKING,
    &ALPHARAEL_STONECHOSEN,
    &ARCHENEMY_S_CHARM,
    &BEAMSAW_PROSPECTOR,
    &BLADE_OF_THE_SWARM,
    &CHORALE_OF_THE_VOID,
    &COMET_CRAWLER,
    &DARK_ENDURANCE,
    &DECODE_TRANSMISSIONS,
    &DEPRESSURIZE,
    &DUBIOUS_DELICACY,
    &ELEGY_ACOLYTE,
    &EMBRACE_OBLIVION,
    &ENTROPIC_BATTLECRUISER,
    &FALLER_S_FAITHFUL,
    &FELL_GRAVSHIP,
    &GRAVBLADE_HEAVY,
    &GRAVKILL,
    &GRAVPACK_MONOIST,
    &HULLCARVER,
    &HYLDERBLADE,
    &HYMN_OF_THE_FALLER,
    &INSATIABLE_SKITTERMAW,
    &LIGHTLESS_EVANGEL,
    &MONOIST_CIRCUIT_FEEDER,
    &MONOIST_SENTRY,
    &PERIGEE_BECKONER,
    &REQUIEM_MONOLITH,
    &SCROUNGE_FOR_ETERNITY,
    &SOTHERA_THE_SUPERVOID,
    &SUNSET_SABOTEUR,
    &SUSURIAN_DIRGECRAFT,
    &SUSURIAN_VOIDBORN,
    &SWARM_CULLER,
    &TEMPORAL_INTERVENTION,
    &TIMELINE_CULLER,
    &TRAGIC_TRAJECTORY,
    &UMBRAL_COLLAR_ZEALOT,
    &VOIDFORGED_TITAN,
    &VOTE_OUT,
    &XU_IFIT_OSTEOHARMONIST,
    &ZERO_POINT_BALLAD,
    &CUT_PROPULSION,
    &DEBRIS_FIELD_CRUSHER,
    &DEVASTATING_ONSLAUGHT,
    &DRILL_TOO_DEEP,
    &FRONTLINE_WAR_RAGER,
    &FULL_BORE,
    &GALVANIZING_SAWSHIP,
    &INVASIVE_MANEUVERS,
    &KAV_LANDSEEKER,
    &KAVARON_HARRIER,
    &KAVARON_SKYWARDEN,
    &KAVARON_TURBODRONE,
    &LITHOBRAKING,
    &MELDED_MOXITE,
    &MEMORIAL_TEAM_LEADER,
    &MEMORIAL_VAULT,
    &MOLECULAR_MODIFIER,
    &NEBULA_DRAGON,
    &NOVA_HELLKITE,
    &ORBITAL_PLUNGE,
    &OREPLATE_PANGOLIN,
    &PAIN_FOR_ALL,
    &PLASMA_BOLT,
    &POSSIBILITY_TECHNICIAN,
    &RED_TIGER_MECHAN,
    &REMNANT_ELEMENTAL,
    &RIG_FOR_WAR,
    &ROVING_ACTUATOR,
    &RUINOUS_RAMPAGE,
    &RUST_HARVESTER,
    &SLAGDRILL_SCRAPPER,
    &SYSTEMS_OVERRIDE,
    &TANNUK_STEADFAST_SECOND,
    &TERMINAL_VELOCITY,
    &TERRAPACT_INTIMIDATOR,
    &TERRITORIAL_BRUNTAR,
    &VAULTGUARD_TROOPER,
    &WARMAKER_GUNSHIP,
    &WEAPONS_MANUFACTURING,
    &WEFTSTALKER_ARDENT,
    &ZOOKEEPER_MECHAN,
    &ATMOSPHERIC_GREENHOUSE,
    &BIOENGINEERED_FUTURE,
    &BIOSYNTHIC_BURST,
    &BLOOMING_STINGER,
    &BROODGUARD_ELITE,
    &CLOSE_ENCOUNTER,
    &DIPLOMATIC_RELATIONS,
    &DRIX_FATEMAKER,
    &EDGE_ROVER,
    &EUMIDIAN_TERRABOTANIST,
    &EUSOCIAL_ENGINEERING,
    &FAMISHED_WORLDSIRE,
    &FRENZIED_BALOTH,
    &FUNGAL_COLOSSUS,
    &GALACTIC_WAYFARER,
    &GENE_POLLINATOR,
    &GERMINATING_WURM,
    &GLACIER_GODMAW,
    &HARMONIOUS_GROVESTRIDER,
    &HEMOSYMBIC_MITE,
    &ICECAVE_CRASHER,
    &INTREPID_TENDERFOOT,
    &LARVAL_SCOUTLANDER,
    &LASHWHIP_PREDATOR,
    &LOADING_ZONE,
    &MELTSTRIDER_EULOGIST,
    &MELTSTRIDER_S_GEAR,
    &MELTSTRIDER_S_RESOLVE,
    &OUROBOROID,
    &PULL_THROUGH_THE_WEFT,
    &SAMI_S_CURIOSITY,
    &SEEDSHIP_AGRARIAN,
    &SEEDSHIP_IMPACT,
    &SHATTERED_WINGS,
    &SKYSTINGER,
    &SLEDGE_CLASS_SEEDSHIP,
    &TAPESTRY_WARDEN,
    &TERRASYMBIOSIS,
    &THAWBRINGER,
    &ALPHARAEL_DREAMING_ACOLYTE,
    &BIOMECHAN_ENGINEER,
    &BIOTECH_SPECIALIST,
    &COSMOGOYF,
    &DYADRINE_SYNTHESIS_AMALGAM,
    &GENEMORPH_IMAGO,
    &HALIYA_ASCENDANT_CADET,
    &INFINITE_GUIDELINE_STATION,
    &INTERCEPTOR_MECHAN,
    &MM_MENON_UTHROS_EXILE,
    &MUTINOUS_MASSACRE,
    &PINNACLE_EMISSARY,
    &RAGOST_DEFT_GASTRONAUT,
    &SAMI_SHIP_S_ENGINEER,
    &SAMI_WILDCAT_CAPTAIN,
    &SEEDSHIP_BROODTENDER,
    &SINGULARITY_RUPTURE,
    &SPACE_TIME_ANOMALY,
    &STATION_MONITOR,
    &SYR_VONDAM_SUNSTAR_EXEMPLAR,
    &SYR_VONDAM_THE_LUCENT,
    &TANNUK_MEMORIAL_ENSIGN,
    &ALL_FATES_SCROLL,
    &BYGONE_COLOSSUS,
    &CHROME_COMPANION,
    &DAUNTLESS_SCRAPBOT,
    &DAWNSIRE_SUNSTAR_DREADNOUGHT,
    &THE_DOMINION_BRACELET,
    &THE_ETERNITY_ELEVATOR,
    &EXTINGUISHER_BATTLESHIP,
    &NUTRIENT_BLOCK,
    &PINNACLE_KILL_SHIP,
    &SURVEY_MECHAN,
    &THAUMATON_TORPEDO,
    &THRUMMING_HIVEPOOL,
    &VIRULENT_SILENCER,
    &WURMWALL_SWEEPER,
    &ADAGIA_WINDSWEPT_BASTION,
    &COMMAND_BRIDGE,
    &EVENDO_WAKING_HAVEN,
    &KAVARON_MEMORIAL_WORLD,
    &SECLUDED_STARFORGE,
    &SUSUR_SECUNDI_VOID_ALTAR,
    &UTHROS_TITANIC_GODCORE,
    &MIGHTFORM_HARMONIZER,
    &ICETILL_EXPLORER,
    &THE_ENDSTONE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    BANISHING_LIGHT_REPRINT,
    ANNUL_REPRINT,
    VIRUS_BEETLE_REPRINT,
    BOMBARD_REPRINT,
    DIPLOMATIC_RELATIONS_ALTERNATE_1,
    ICETILL_EXPLORER_ALTERNATE_1,
    MIGHTFORM_HARMONIZER_ALTERNATE_1,
    THE_ENDSTONE_ALTERNATE_1,
    BREEDING_POOL_REPRINT,
    GODLESS_SHRINE_REPRINT,
    SACRED_FOUNDRY_REPRINT,
    STOMPING_GROUND_REPRINT,
    WATERY_GRAVE_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    ADAGIA_WINDSWEPT_BASTION_ALTERNATE_1,
    BREEDING_POOL_ALTERNATE_1,
    EVENDO_WAKING_HAVEN_ALTERNATE_1,
    GODLESS_SHRINE_ALTERNATE_1,
    KAVARON_MEMORIAL_WORLD_ALTERNATE_1,
    SACRED_FOUNDRY_ALTERNATE_1,
    STOMPING_GROUND_ALTERNATE_1,
    SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_1,
    UTHROS_TITANIC_GODCORE_ALTERNATE_1,
    WATERY_GRAVE_ALTERNATE_1,
    TEZZERET_CRUEL_CAPTAIN_ALTERNATE_1,
    ASTELLI_RECLAIMER_ALTERNATE_1,
    HALIYA_GUIDED_BY_LIGHT_ALTERNATE_1,
    MM_MENON_THE_RIGHT_HAND_ALTERNATE_1,
    STARWINDER_ALTERNATE_1,
    ALPHARAEL_STONECHOSEN_ALTERNATE_1,
    ELEGY_ACOLYTE_ALTERNATE_1,
    XU_IFIT_OSTEOHARMONIST_ALTERNATE_1,
    POSSIBILITY_TECHNICIAN_ALTERNATE_1,
    TANNUK_STEADFAST_SECOND_ALTERNATE_1,
    DYADRINE_SYNTHESIS_AMALGAM_ALTERNATE_1,
    GENEMORPH_IMAGO_ALTERNATE_1,
    RAGOST_DEFT_GASTRONAUT_ALTERNATE_1,
    SAMI_WILDCAT_CAPTAIN_ALTERNATE_1,
    SYR_VONDAM_SUNSTAR_EXEMPLAR_ALTERNATE_1,
    BEYOND_THE_QUIET_ALTERNATE_1,
    COSMOGRAND_ZENITH_ALTERNATE_1,
    QUANTUM_RIDDLER_ALTERNATE_1,
    STARWINDER_ALTERNATE_2,
    ARCHENEMY_S_CHARM_ALTERNATE_1,
    DEVASTATING_ONSLAUGHT_ALTERNATE_1,
    NOVA_HELLKITE_ALTERNATE_1,
    RUST_HARVESTER_ALTERNATE_1,
    WEAPONS_MANUFACTURING_ALTERNATE_1,
    TERRASYMBIOSIS_ALTERNATE_1,
    COSMOGOYF_ALTERNATE_1,
    MUTINOUS_MASSACRE_ALTERNATE_1,
    SPACE_TIME_ANOMALY_ALTERNATE_1,
    SECLUDED_STARFORGE_ALTERNATE_1,
    ANTICAUSAL_VESTIGE_ALTERNATE_1,
    EXALTED_SUNBORN_ALTERNATE_1,
    HARDLIGHT_CONTAINMENT_ALTERNATE_1,
    LIGHTSTALL_INQUISITOR_ALTERNATE_1,
    LUMEN_CLASS_FRIGATE_ALTERNATE_1,
    PINNACLE_STARCAGE_ALTERNATE_1,
    THE_SERIEMA_ALTERNATE_1,
    SUNSTAR_CHAPLAIN_ALTERNATE_1,
    CONSULT_THE_STAR_CHARTS_ALTERNATE_1,
    EMISSARY_ESCORT_ALTERNATE_1,
    MOONLIT_MEDITATION_ALTERNATE_1,
    STARFIELD_VOCALIST_ALTERNATE_1,
    SYNTHESIZER_LABSHIP_ALTERNATE_1,
    WEFTWALKING_ALTERNATE_1,
    CHORALE_OF_THE_VOID_ALTERNATE_1,
    ENTROPIC_BATTLECRUISER_ALTERNATE_1,
    REQUIEM_MONOLITH_ALTERNATE_1,
    SUNSET_SABOTEUR_ALTERNATE_1,
    ZERO_POINT_BALLAD_ALTERNATE_1,
    MEMORIAL_VAULT_ALTERNATE_1,
    PAIN_FOR_ALL_ALTERNATE_1,
    TERMINAL_VELOCITY_ALTERNATE_1,
    WARMAKER_GUNSHIP_ALTERNATE_1,
    BIOENGINEERED_FUTURE_ALTERNATE_1,
    FAMISHED_WORLDSIRE_ALTERNATE_1,
    FRENZIED_BALOTH_ALTERNATE_1,
    ICETILL_EXPLORER_ALTERNATE_2,
    LOADING_ZONE_ALTERNATE_1,
    OUROBOROID_ALTERNATE_1,
    SLEDGE_CLASS_SEEDSHIP_ALTERNATE_1,
    BIOTECH_SPECIALIST_ALTERNATE_1,
    INFINITE_GUIDELINE_STATION_ALTERNATE_1,
    PINNACLE_EMISSARY_ALTERNATE_1,
    SINGULARITY_RUPTURE_ALTERNATE_1,
    DAWNSIRE_SUNSTAR_DREADNOUGHT_ALTERNATE_1,
    THE_DOMINION_BRACELET_ALTERNATE_1,
    THE_ENDSTONE_ALTERNATE_2,
    THE_ETERNITY_ELEVATOR_ALTERNATE_1,
    EXTINGUISHER_BATTLESHIP_ALTERNATE_1,
    THRUMMING_HIVEPOOL_ALTERNATE_1,
    ANTICAUSAL_VESTIGE_ALTERNATE_2,
    EXALTED_SUNBORN_ALTERNATE_2,
    STARFIELD_VOCALIST_ALTERNATE_2,
    SOTHERA_THE_SUPERVOID_ALTERNATE_1,
    DEVASTATING_ONSLAUGHT_ALTERNATE_2,
    MUTINOUS_MASSACRE_ALTERNATE_2,
    THE_DOMINION_BRACELET_ALTERNATE_2,
    THE_ENDSTONE_ALTERNATE_3,
    SECLUDED_STARFORGE_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_3,
    ADAGIA_WINDSWEPT_BASTION_ALTERNATE_2,
    BREEDING_POOL_ALTERNATE_2,
    EVENDO_WAKING_HAVEN_ALTERNATE_2,
    GODLESS_SHRINE_ALTERNATE_2,
    KAVARON_MEMORIAL_WORLD_ALTERNATE_2,
    SACRED_FOUNDRY_ALTERNATE_2,
    STOMPING_GROUND_ALTERNATE_2,
    SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_2,
    UTHROS_TITANIC_GODCORE_ALTERNATE_2,
    WATERY_GRAVE_ALTERNATE_2,
    SOTHERA_THE_SUPERVOID_ALTERNATE_2,
    ANTICAUSAL_VESTIGE_ALTERNATE_3,
    EXALTED_SUNBORN_ALTERNATE_3,
    STARFIELD_VOCALIST_ALTERNATE_3,
    SOTHERA_THE_SUPERVOID_ALTERNATE_3,
    DEVASTATING_ONSLAUGHT_ALTERNATE_3,
    ICETILL_EXPLORER_ALTERNATE_3,
    MUTINOUS_MASSACRE_ALTERNATE_3,
    THE_DOMINION_BRACELET_ALTERNATE_3,
    SECLUDED_STARFORGE_ALTERNATE_3,
    STARFIELD_SHEPHERD_ALTERNATE_1,
    ANNUL_ALTERNATE_1,
    UMBRAL_COLLAR_ZEALOT_ALTERNATE_1,
    KAVARON_HARRIER_ALTERNATE_1,
    PULL_THROUGH_THE_WEFT_ALTERNATE_1,
    SINGULARITY_RUPTURE_ALTERNATE_2,
    EMISSARY_ESCORT_ALTERNATE_2,
];
