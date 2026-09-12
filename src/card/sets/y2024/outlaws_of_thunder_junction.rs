//! Outlaws of Thunder Junction card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostObjectIndex;
use crate::ParentBinding;
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
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::BlockRestrictionMatchDef;
use crate::card::BlockRestrictionSubjectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChangeStackTargetsDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
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
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::HalvedValueDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ManaTypeDef;
use crate::card::ManaTypeSetDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::QuantifierDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementAbilityDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementConditionDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::RoundingDef;
use crate::card::ScaledValueDef;
use crate::card::SpellCastQueryDef;
use crate::card::SpellCostConditionDef;
use crate::card::SpellCostModificationDef;
use crate::card::StackTargetChangeDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
use crate::card::TokenStatsDef;
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
use crate::card::sets::y2015::magic_origins as catalog_ori;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2017::ixalan as catalog_xln;
use crate::card::sets::y2020::core_set_2021 as catalog_m21;
use crate::card::sets::y2021::kaldheim as catalog_khm;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;
use crate::card::sets::y2022::streets_of_new_capenna as catalog_snc;
use crate::card::sets::y2023::march_of_the_machine as catalog_mom;

pub const SPREE: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:spree");

/// Choose one or more modes and pay the additional costs of the chosen modes.
///
/// # Panics
///
/// Panics if the mode list is empty or contains more than 255 modes.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn spree(modes: &'static [(&'static [CostDef], AbilityDef)]) -> AbilityDef {
    assert!(!modes.is_empty() && modes.len() <= u8::MAX as usize);
    AbilityDef::defined(
        "Spree (Choose one or more additional costs.)",
        crate::card::DeclarativeAbilityDef::Spell(crate::card::SpellAbilityDef::Modal(
            crate::card::ModalSpellDef::with_costed_modes(modes, 1, modes.len() as u8, false),
        )),
        EffectDef::None,
    )
    .labeled(SPREE)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OTJ",
    slug: "outlaws-of-thunder-junction",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const CLUE_TOKEN: TokenCharacteristics = crate::card::tokens::clue().with_art(CardArt::new(
    "764a906c-8b27-4ffa-bdc3-7825c6919d3e",
    "Clint Lockwood",
));
const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("7ec6f053-96f7-4e57-b2eb-4e7699a40a4f", "Monztre"),
);

const MERCENARY_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Mercenary"], &[ManaColor::Red], 1, 1)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{T}: Target creature you control gets +1/+0 until end of \
                     turn. Activate only as a sorcery.",
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)])
        .with_art(CardArt::new(
            "5f04607f-eed2-462e-897f-82e41e5f7049",
            "Eduardo Francisco",
        ));
const ZOMBIE_ROGUE_TOKEN: TokenCharacteristics = TokenCharacteristics::creature(
    &["Zombie", "Rogue"],
    &[ManaColor::Blue, ManaColor::Black],
    2,
    2,
)
.with_art(CardArt::new(
    "74c7a0bd-6011-495a-b56c-8fa707dd7f12",
    "Caio E Santos",
));

// OTJ 1 — Another Round
// Audit: unsupported — Needs a resolving loop that repeats a fresh optional creature selection and its exile-return sequence X plus one times; Sequence has a fixed authored length and no value-counted repetition.
pub(in crate::card::sets) static ANOTHER_ROUND: CardRecord = CardRecord::new(
    "Another Round",
    "4f8dc511-e307-4412-bb79-375a6077312d",
    "Darrell Riche",
    CardRules::unsupported(),
);

// OTJ 2 — Archangel of Tithes (reprint)
const ARCHANGEL_OF_TITHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::ARCHANGEL_OF_TITHES,
    "c853d04c-864b-491c-8c6f-72d2d4874d2f",
    "Denys Tsiperko",
);

// OTJ 3 — Armored Armadillo
pub(in crate::card::sets) static ARMORED_ARMADILLO: CardRecord = CardRecord::new(
    "Armored Armadillo",
    "263232df-69b8-4205-93ad-c724fe57ec11",
    "Leon Tukker",
    CardRules::new_creature(mana_cost!("{W}"), &["Armadillo"], 0, 4).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{1}"))], "Ward {1}"),
        AbilityDef::activated(
            "{3}{W}: This creature gets +X/+0 until end of turn, where X \
             is its toughness.",
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourceToughness,
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// OTJ 4 — Aven Interrupter
// Audit: unsupported — Needs an effect or external permission that makes other cards plotted, including its later-turn, sorcery-only free cast permission; existing plot only supports the card's own hand special action.
pub(in crate::card::sets) static AVEN_INTERRUPTER: CardRecord = CardRecord::new(
    "Aven Interrupter",
    "d3ca43a4-d194-440f-8099-f1fa103a108d",
    "Daniel Romanovsky",
    CardRules::unsupported(),
);

// OTJ 5 — Bounding Felidar
pub(in crate::card::sets) static BOUNDING_FELIDAR: CardRecord = CardRecord::new(
    "Bounding Felidar",
    "8925279f-c16a-43b4-b791-ce450157275b",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Cat", "Beast", "Mount"], 4, 7).with_abilities(
        &[
            AbilityDef::triggered(
                "Whenever this creature attacks while saddled, put a +1/+1 \
                 counter on each other creature you control. You gain 1 life \
                 for each of those creatures.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Saddled,
                    },
                },
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    binding: crate::Binding!("companions"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("companions"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                crate::Binding!("companions"),
                            )),
                        },
                    ]),
                }),
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 2 }],
                "Saddle 2 (Tap any number of other creatures you control with \
                 total power 2 or more: This Mount becomes saddled until end \
                 of turn. Saddle only as a sorcery.)",
            ),
        ],
    ),
);

// OTJ 6 — Bovine Intervention
pub(in crate::card::sets) static BOVINE_INTERVENTION: CardRecord = CardRecord::new(
    "Bovine Intervention",
    "26c36742-456f-4618-99bc-793ef20b31b0",
    "Julia Metzger",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact or creature. Its controller creates a \
         2/2 white Ox creature token.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Ox"],
                    &[ManaColor::White],
                    2,
                    2,
                )))
                .with_controller(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
            ),
        ]),
    )]),
);

// OTJ 7 — Bridled Bighorn
pub(in crate::card::sets) static BRIDLED_BIGHORN: CardRecord = CardRecord::new(
    "Bridled Bighorn",
    "fa7bf089-fa9b-4ffc-bf84-45cd51c76463",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Sheep", "Mount"], 3, 4).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, create a 1/1 \
             white Sheep creature token.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Sheep"], &[ManaColor::White], 1, 1),
            ))),
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 2 }],
            "Saddle 2 (Tap any number of other creatures you control with \
             total power 2 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// OTJ 8 — Claim Jumper
pub(in crate::card::sets) static CLAIM_JUMPER: CardRecord = CardRecord::new(
    "Claim Jumper",
    "654ade6b-0369-4b90-a744-2f57a45b04f4",
    "Gaboleps",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Rabbit", "Mercenary"], 3, 3).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::triggered_if(
                "When this creature enters, if an opponent controls more lands \
                 than you, you may search your library for a Plains card and \
                 put it onto the battlefield tapped. Then if an opponent \
                 controls more lands than you, repeat this process once. If \
                 you search your library this way, shuffle.",
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
                EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Search",
                            effect: EffectDef::Sequence(&[
                                EffectDef::SearchZone {
                                    player: EffectRecipientDef::Controller,
                                    source: ZoneKind::Library,
                                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                                        "Plains",
                                    )),
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
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ValueComparison(
                                        &ValueComparisonDef {
                                            left: ValueDef::CountMatchingObjects(
                                                &ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Land),
                                                    &[ZoneKind::Battlefield],
                                                    PlayerRelation::Opponent,
                                                ),
                                            ),
                                            comparison: ComparisonDef::Greater,
                                            right: ValueDef::CountMatchingObjects(
                                                &ObjectQueryDef::matching(
                                                    ObjectPredicateDef::HasType(CardType::Land),
                                                    &[ZoneKind::Battlefield],
                                                    PlayerRelation::You,
                                                ),
                                            ),
                                        },
                                    ),
                                    then: &EffectDef::May {
                                        player: EffectRecipientDef::Controller,
                                        effect: &EffectDef::SearchZone {
                                            player: EffectRecipientDef::Controller,
                                            source: ZoneKind::Library,
                                            object: ObjectPredicateDef::Subtype(
                                                SubtypeDef::Literal("Plains"),
                                            ),
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
                                    },
                                },
                                EffectDef::ShuffleLibrary {
                                    player: EffectRecipientDef::Controller,
                                },
                            ]),
                        },
                        EffectChoiceDef {
                            label: "Do not search",
                            effect: EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ValueComparison(
                                    &ValueComparisonDef {
                                        left: ValueDef::CountMatchingObjects(
                                            &ObjectQueryDef::matching(
                                                ObjectPredicateDef::HasType(CardType::Land),
                                                &[ZoneKind::Battlefield],
                                                PlayerRelation::Opponent,
                                            ),
                                        ),
                                        comparison: ComparisonDef::Greater,
                                        right: ValueDef::CountMatchingObjects(
                                            &ObjectQueryDef::matching(
                                                ObjectPredicateDef::HasType(CardType::Land),
                                                &[ZoneKind::Battlefield],
                                                PlayerRelation::You,
                                            ),
                                        ),
                                    },
                                ),
                                then: &EffectDef::May {
                                    player: EffectRecipientDef::Controller,
                                    effect: &EffectDef::SearchZone {
                                        player: EffectRecipientDef::Controller,
                                        source: ZoneKind::Library,
                                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                                            "Plains",
                                        )),
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
                            },
                        },
                    ],
                },
            ),
        ],
    ),
);

// OTJ 9 — Dust Animus
pub(in crate::card::sets) static DUST_ANIMUS: CardRecord = CardRecord::new(
    "Dust Animus",
    "70719e7b-6f02-4c1a-9f11-79b2b0d9846a",
    "Uriah Voth",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::as_enters(
            "If you control five or more untapped lands, this creature \
             enters with two +1/+1 counters and a lifelink counter on it.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 5,
                }),
                if_true: &[
                    ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::PlusOnePlusOne,
                            amount: 2,
                        },
                    ),
                    ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::Lifelink,
                            amount: 1,
                        },
                    ),
                ],
                if_false: &[],
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{W}"))]),
    ]),
);

// OTJ 10 — Eriette's Lullaby
pub(in crate::card::sets) static ERIETTE_S_LULLABY: CardRecord = CardRecord::new(
    "Eriette's Lullaby",
    "184c18b6-40e2-4f7f-a3bb-49bc695b68ec",
    "Anton Solovianchyk",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target tapped creature. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Tapped,
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// OTJ 11 — Final Showdown
// Audit: unsupported — Needs granting an ability to a nontargeted creature chosen during resolution inside a modal spell; the modal-effect validator does not accept a chosen binding as a supported ability-grant recipient.
pub(in crate::card::sets) static FINAL_SHOWDOWN: CardRecord = CardRecord::new(
    "Final Showdown",
    "358968f9-45bd-4022-b6bc-f1f7e0adf0e7",
    "Izzy",
    CardRules::unsupported(),
);

// OTJ 12 — Fortune, Loyal Steed
// Audit: unsupported — Needs the identities of creatures used to saddle this Mount retained for the rest of the turn; current saddle stores only a saddled boolean.
pub(in crate::card::sets) static FORTUNE_LOYAL_STEED: CardRecord = CardRecord::new(
    "Fortune, Loyal Steed",
    "069294a8-e65a-47af-942f-7e99d18658f2",
    "Artur Nakhodkin",
    CardRules::unsupported(),
);

// OTJ 13 — Frontier Seeker
pub(in crate::card::sets) static FRONTIER_SEEKER: CardRecord = CardRecord::new(
    "Frontier Seeker",
    "9368cc76-bee5-4b46-a309-981106c3addf",
    "Raluca Marinescu",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Scout"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, look at the top five cards of your \
             library. You may reveal a Mount creature card or a Plains \
             card from among them and put it into your hand. Put the rest \
             on the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                    ]),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains")),
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
    ]),
);

// OTJ 14 — Getaway Glamer
pub(in crate::card::sets) static GETAWAY_GLAMER: CardRecord = CardRecord::new(
    "Getaway Glamer",
    "69689049-a704-4f16-84ee-4d5d915028ec",
    "Forrest Imel",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Exile target nontoken creature. Return it to the battlefield \
                 under its owner's control at the beginning of the next end \
                 step.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                )],
                EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("blinked"),
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "At the beginning of the next end step, return that card to \
                             the battlefield.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::End,
                                player: PlayerRelation::Any,
                            },
                            EffectDef::move_to_zone(
                                EffectRecipientDef::objects(
                                    ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                        "blinked"
                                    )),
                                ),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                        ),
                    )),
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Destroy target creature if no other creature has greater power.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Sum,
                        }),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::Any,
                            )),
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Maximum,
                        }),
                    }),
                    then: &EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                },
            ),
        ),
    ])]),
);

// OTJ 15 — High Noon
pub(in crate::card::sets) static HIGH_NOON: CardRecord = CardRecord::new(
    "High Noon",
    "9995e0e6-7c9c-4fef-8fd2-8fb1622e6ec8",
    "Eduardo Francisco",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::static_ability(
            "Each player can't cast more than one spell each turn.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::Any,
                    )
                    .after_spells_cast(1),
                )),
            },
        ),
        AbilityDef::activated_with_targets(
            "{4}{R}, Sacrifice this enchantment: It deals 5 damage to any \
             target.",
            &[
                CostDef::Mana(mana_cost!("{4}{R}")),
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
        ),
    ]),
);

// OTJ 16 — Holy Cow
pub(in crate::card::sets) static HOLY_COW: CardRecord = CardRecord::new(
    "Holy Cow",
    "90de84c9-941b-4056-8501-ce8a948b9643",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Ox", "Angel"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life and scry 1. (Look \
             at the top card of your library. You may put that card on the \
             bottom.)",
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                abilities::scry(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// OTJ 17 — Inventive Wingsmith
// Audit: unsupported — Needs spell-cast history filtered by the zone a spell was cast from; the current history query retains caster and spell characteristics but no casting-origin zone.
pub(in crate::card::sets) static INVENTIVE_WINGSMITH: CardRecord = CardRecord::new(
    "Inventive Wingsmith",
    "b6b36bb3-dacc-44f6-adcd-2c2d65513d8c",
    "David Astruga",
    CardRules::unsupported(),
);

// OTJ 18 — Lassoed by the Law
// Audit: unsupported — Needs exile-until-source-leaves with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger would return the card later through the stack.
pub(in crate::card::sets) static LASSOED_BY_THE_LAW: CardRecord = CardRecord::new(
    "Lassoed by the Law",
    "ea96eeac-c316-4247-a81f-0ddf52675ebf",
    "Leanna Crossan",
    CardRules::unsupported(),
);

// OTJ 19 — Mystical Tether
// Audit: unsupported — Needs exile-until-source-leaves with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger would return the card later through the stack.
pub(in crate::card::sets) static MYSTICAL_TETHER: CardRecord = CardRecord::new(
    "Mystical Tether",
    "18344498-952e-489c-8b03-bd1bef4c26ca",
    "Adam Volker",
    CardRules::unsupported(),
);

// OTJ 20 — Nurturing Pixie
pub(in crate::card::sets) static NURTURING_PIXIE: CardRecord = CardRecord::new(
    "Nurturing Pixie",
    "0fe155f6-888b-41a0-a9a0-be7bea998718",
    "Iris Compiet",
    CardRules::new_creature(mana_cost!("{W}"), &["Faerie", "Rogue"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one target \
             non-Faerie, nonland permanent you control to its owner's \
             hand. If a permanent was returned this way, put a +1/+1 \
             counter on this creature.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Faerie",
                        ))),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("returned"),
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                            "returned"
                        )),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            },
        ),
    ]),
);

// OTJ 21 — Omenport Vigilante
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static OMENPORT_VIGILANTE: CardRecord = CardRecord::new(
    "Omenport Vigilante",
    "7ecd8b6f-b9aa-466a-909c-3209beef8244",
    "Forrest Imel",
    CardRules::unsupported(),
);

// OTJ 22 — One Last Job
// Audit: unsupported — Needs legal host selection for an attachment arriving from the graveyard, including its enchant restrictions and protection; LegalAttachmentHosts currently requires the attachment already be on the battlefield.
pub(in crate::card::sets) static ONE_LAST_JOB: CardRecord = CardRecord::new(
    "One Last Job",
    "71bfbfae-e7eb-4f80-81c6-9ab6a1bbd39d",
    "Caroline Gariba",
    CardRules::unsupported(),
);

// OTJ 23 — Outlaw Medic
pub(in crate::card::sets) static OUTLAW_MEDIC: CardRecord = CardRecord::new(
    "Outlaw Medic",
    "2feaa51e-47fb-4849-b420-ee7278f3489a",
    "Nino Vecia",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Rogue"], 1, 3).with_abilities(&[
        abilities::lifelink(),
        abilities::dies_trigger(
            "When this creature dies, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// OTJ 24 — Prairie Dog
// Audit: unsupported — Needs spell-cast history filtered by the zone a spell was cast from; the current history query retains caster and spell characteristics but no casting-origin zone.
pub(in crate::card::sets) static PRAIRIE_DOG: CardRecord = CardRecord::new(
    "Prairie Dog",
    "37302b5d-e528-4baa-947a-c859e4ddcff9",
    "Kevin Sidharta",
    CardRules::unsupported(),
);

// OTJ 25 — Prosperity Tycoon
pub(in crate::card::sets) static PROSPERITY_TYCOON: CardRecord = CardRecord::new(
    "Prosperity Tycoon",
    "f87824f3-aa9f-4d3d-99f7-0fbca43d8a51",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Noble"], 4, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 red Mercenary \
             creature token with \"{T}: Target creature you control gets \
             +1/+0 until end of turn. Activate only as a sorcery.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ),
        AbilityDef::activated(
            "{2}, Sacrifice a token: This creature gains indestructible \
             until end of turn. Tap it. (Damage and effects that say \
             \"destroy\" don't destroy it.)",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::Token),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ]),
);

// OTJ 26 — Requisition Raid
pub(in crate::card::sets) static REQUISITION_RAID: CardRecord = CardRecord::new(
    "Requisition Raid",
    "154e9ba9-0d0e-4b0e-acf2-f66a993cf3a2",
    "Viko Menezes",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
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
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Destroy target enchantment.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Put a +1/+1 counter on each creature target player controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::controlled_by(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ),
    ])]),
);

// OTJ 27 — Rustler Rampage
pub(in crate::card::sets) static RUSTLER_RAMPAGE: CardRecord = CardRecord::new(
    "Rustler Rampage",
    "33ed7ca3-894b-45f4-a15f-51b6bcd3f474",
    "Josu Hernaiz",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Untap all creatures target player controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::objects(ObjectSetDef::PermanentsControlledBy(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                    )),
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Target creature gains double strike until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
    ])),
);

// OTJ 28 — Shepherd of the Clouds
pub(in crate::card::sets) static SHEPHERD_OF_THE_CLOUDS: CardRecord = CardRecord::new(
    "Shepherd of the Clouds",
    "c2245f36-2138-4f01-9b70-151137a5ac59",
    "Valera Lutfullina",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Pegasus"], 4, 3).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target permanent card with \
             mana value 3 or less from your graveyard to your hand. Return \
             that card to the battlefield instead if you control a Mount.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                otherwise: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// OTJ 29 — Sheriff of Safe Passage
pub(in crate::card::sets) static SHERIFF_OF_SAFE_PASSAGE: CardRecord = CardRecord::new(
    "Sheriff of Safe Passage",
    "c38a845d-f25d-45ab-9154-3fa5291b0ba0",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it plus an \
             additional +1/+1 counter on it for each other creature you \
             control.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Sum(&SumValueDef::new(
                        ValueDef::Constant(1),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    )),
                },
            ),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{W}"))]),
    ]),
);

// OTJ 30 — Stagecoach Security
pub(in crate::card::sets) static STAGECOACH_SECURITY: CardRecord = CardRecord::new(
    "Stagecoach Security",
    "21301998-b3e6-4f3d-89f4-5e17aeb79a1e",
    "Nino Vecia",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 4, 5).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, creatures you control get +1/+1 \
             and gain vigilance until end of turn.",
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
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{3}{W}"))]),
    ]),
);

// OTJ 31 — Steer Clear
// Audit: unsupported — Needs a cast-time snapshot of whether the caster controlled a Mount, retained on the spell and copied with it; resolution-time battlefield queries cannot answer that historical condition.
pub(in crate::card::sets) static STEER_CLEAR: CardRecord = CardRecord::new(
    "Steer Clear",
    "523a4d6e-122b-49b4-bf3d-17d29c0007fb",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// OTJ 32 — Sterling Keykeeper
pub(in crate::card::sets) static STERLING_KEYKEEPER: CardRecord = CardRecord::new(
    "Sterling Keykeeper",
    "019d539f-04c2-43f1-8677-6d6fbb0e94f7",
    "David Astruga",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Mercenary"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Tap target non-Mount creature.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Mount",
                    ))),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// OTJ 33 — Sterling Supplier
pub(in crate::card::sets) static STERLING_SUPPLIER: CardRecord = CardRecord::new(
    "Sterling Supplier",
    "b6000be7-67db-440e-87ba-276df20b803e",
    "Camille Alquier",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird", "Soldier"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on another \
             target creature you control.",
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
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// OTJ 34 — Take Up the Shield (reprint)
const TAKE_UP_THE_SHIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::TAKE_UP_THE_SHIELD,
    "76a31968-ba6d-4c01-838f-4cb8c64e73fb",
    "Josiah \"Jo\" Cameron",
);

// OTJ 35 — Thunder Lasso
// Audit: unsupported — Needs target selection relative to the creature that triggered an attached Equipment's attack ability; DefendingPlayer currently reads the ability source's own attack, and an Equipment is not the attacker.
pub(in crate::card::sets) static THUNDER_LASSO: CardRecord = CardRecord::new(
    "Thunder Lasso",
    "73dff5fc-2adf-447a-b35f-e7883e0fd821",
    "Camille Alquier",
    CardRules::unsupported(),
);

// OTJ 36 — Trained Arynx
pub(in crate::card::sets) static TRAINED_ARYNX: CardRecord = CardRecord::new(
    "Trained Arynx",
    "ef32a5f8-f69d-47dc-a800-4f0ddf4eada5",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Beast", "Mount"], 3, 1).with_abilities(
        &[
            AbilityDef::triggered(
                "Whenever this creature attacks while saddled, it gains first \
                 strike until end of turn. Scry 1. (Look at the top card of \
                 your library. You may put that card on the bottom.)",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Saddled,
                    },
                },
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    abilities::scry(ValueDef::Constant(1)),
                ]),
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 2 }],
                "Saddle 2 (Tap any number of other creatures you control with \
                 total power 2 or more: This Mount becomes saddled until end \
                 of turn. Saddle only as a sorcery.)",
            ),
        ],
    ),
);

// OTJ 37 — Vengeful Townsfolk
pub(in crate::card::sets) static VENGEFUL_TOWNSFOLK: CardRecord = CardRecord::new(
    "Vengeful Townsfolk",
    "2df404af-571a-4867-83f5-bb4163b433ff",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Citizen"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever one or more other creatures you control die, put a \
             +1/+1 counter on this creature.",
            TriggerEventDef::ObjectsDied {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// OTJ 38 — Wanted Griffin
pub(in crate::card::sets) static WANTED_GRIFFIN: CardRecord = CardRecord::new(
    "Wanted Griffin",
    "624a176b-fe24-4441-8877-464cf172ccff",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 red Mercenary creature \
             token with \"{T}: Target creature you control gets +1/+0 \
             until end of turn. Activate only as a sorcery.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ),
    ]),
);

// OTJ 39 — Archmage's Newt
pub(in crate::card::sets) static ARCHMAGE_S_NEWT: CardRecord = CardRecord::new(
    "Archmage's Newt",
    "a440bbd6-8e51-4db1-90d9-7fa9fc327ad5",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Salamander", "Mount"], 2, 2).with_abilities(
        &[
            AbilityDef::triggered_with_targets(
                "Whenever this creature deals combat damage to a player, \
                 target instant or sorcery card in your graveyard gains \
                 flashback until end of turn. The flashback cost is equal to \
                 its mana cost. That card gains flashback {0} until end of \
                 turn instead if this creature is saddled. (You may cast that \
                 card from your graveyard for its flashback cost. Then exile \
                 it.)",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
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
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Saddled,
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::flashback(&[
                            CostDef::Mana(mana_cost!("{0}")),
                        ])),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    otherwise: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(
                            &abilities::flashback_for_card_mana_cost(),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 3 }],
                "Saddle 3",
            ),
        ],
    ),
);

// OTJ 40 — Canyon Crab
// Audit: unsupported — Needs spell-cast history filtered by the zone a spell was cast from; the current history query retains caster and spell characteristics but no casting-origin zone.
pub(in crate::card::sets) static CANYON_CRAB: CardRecord = CardRecord::new(
    "Canyon Crab",
    "b740a8a8-e1d3-4642-a214-03731c9b5553",
    "Ignatius Budi",
    CardRules::unsupported(),
);

// OTJ 41 — Daring Thunder-Thief
pub(in crate::card::sets) static DARING_THUNDER_THIEF: CardRecord = CardRecord::new(
    "Daring Thunder-Thief",
    "b41a2bf7-248c-4f8b-92ec-3010d276cb59",
    "Inkognit",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Turtle", "Rogue"], 4, 4).with_abilities(&[
        abilities::flash(),
        abilities::enters_tapped(CardType::Creature),
    ]),
);

// OTJ 42 — Deepmuck Desperado
pub(in crate::card::sets) static DEEPMUCK_DESPERADO: CardRecord = CardRecord::new(
    "Deepmuck Desperado",
    "f95ee726-7465-40f8-a954-19b19e636c12",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Homarid", "Mercenary"], 2, 4).with_abilities(
        &[AbilityDef::triggered(
            "Whenever you commit a crime, each opponent mills three cards. \
             This ability triggers only once each turn. (Targeting \
             opponents, anything they control, and/or cards in their \
             graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::Mill {
                player: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(3),
            },
        )
        .triggering_at_most(1)],
    ),
);

// OTJ 43 — Djinn of Fool's Fall
pub(in crate::card::sets) static DJINN_OF_FOOL_S_FALL: CardRecord = CardRecord::new(
    "Djinn of Fool's Fall",
    "48bfc6af-c651-485e-a1cc-f9d00aeaf812",
    "Inkognit",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Djinn"], 4, 3).with_abilities(&[
        abilities::flying(),
        abilities::plot(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// OTJ 44 — Double Down
pub(in crate::card::sets) static DOUBLE_DOWN: CardRecord = CardRecord::new(
    "Double Down",
    "8ecccdf3-98c6-4aed-8757-913623efb677",
    "Javier Charro",
    CardRules::new_enchantment(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::triggered(
        "Whenever you cast an outlaw spell, copy that spell. \
         (Assassins, Mercenaries, Pirates, Rogues, and Warlocks are \
         outlaws. Copies of permanent spells become tokens.)",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
            ]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        EffectDef::CopyStackObject(&CopyStackObjectDef {
            object: EffectRecipientDef::TriggeringObject,
            controller: PlayerRefDef::EffectController,
            count: ValueDef::Constant(1),
            retarget: false,
            colors: None,
        }),
    )]),
);

// OTJ 45 — Duelist of the Mind
pub(in crate::card::sets) static DUELIST_OF_THE_MIND: CardRecord = CardRecord::new(
    "Duelist of the Mind",
    "2b58e47b-c165-4a58-aa2a-033a35645adc",
    "Darren Tan",
// A 0/3 flier that grows with every draw and feeds itself once a turn,
    // provided you point something at your opponent.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Advisor"], 0, 3)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Duelist of the Mind's power is equal to the number of cards you've drawn this turn.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    // The count defines her power outright, which is why it also
                    // answers in a hand or a graveyard; the printed toughness is
                    // left alone.
                    effect: AppliedEffectDef::define_power(ValueDef::CardsDrawnThisTurn(
                        PlayerRelation::You,
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever you commit a crime, you may draw a card. If you do, discard a card. This ability triggers only once each turn.",
                TriggerEventDef::CommittedCrime(PlayerRelation::You),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // "Draw a card. If you do, discard a card." A draw from an empty library
                    // does not happen, so the discard is conditional on the draw rather than
                    // sequenced after it.
                    effect: &EffectDef::Sequence(&[
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                    ]),
                },
            )
            .triggering_at_most(1),
        ]),
);

// OTJ 46 — Emergent Haunting
// Audit: unsupported — Needs spell-cast history filtered by the zone a spell was cast from; the current history query retains caster and spell characteristics but no casting-origin zone.
pub(in crate::card::sets) static EMERGENT_HAUNTING: CardRecord = CardRecord::new(
    "Emergent Haunting",
    "623053a8-7abe-45ec-9f26-97e1c037120b",
    "Jorge Jacinto",
    CardRules::unsupported(),
);

// OTJ 47 — Failed Fording
pub(in crate::card::sets) static FAILED_FORDING: CardRecord = CardRecord::new(
    "Failed Fording",
    "62bbe11b-e959-4080-98ac-09bd57519c00",
    "José Parodi",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand. If you \
         control a Desert, surveil 1. (Look at the top card of your \
         library. You may put it into your graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &abilities::surveil(ValueDef::Constant(1)),
            },
        ]),
    )]),
);

// OTJ 48 — Fblthp, Lost on the Range
// Audit: unsupported — Needs an effect or external permission that makes other cards plotted, including its later-turn, sorcery-only free cast permission; existing plot only supports the card's own hand special action.
pub(in crate::card::sets) static FBLTHP_LOST_ON_THE_RANGE: CardRecord = CardRecord::new(
    "Fblthp, Lost on the Range",
    "01d3e6ea-4791-4948-af22-c1bd04c34c1e",
    "Brian Valeza",
    CardRules::unsupported(),
);

// OTJ 49 — Fleeting Reflection
pub(in crate::card::sets) static FLEETING_REFLECTION: CardRecord = CardRecord::new(
    "Fleeting Reflection",
    "8f8c931e-219f-4032-b55b-b5975fbea1e7",
    "Camille Alquier",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gains hexproof until end of turn. \
         Untap that creature. Until end of turn, it becomes a copy of \
         up to one other target creature.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )
            .another(),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetIndex(1)),
                copier: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                exceptions: CopyExceptionsDef::NONE,
                duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
            },
        ]),
    )]),
);

// OTJ 50 — Geralf, the Fleshwright
// Audit: unsupported — Needs a turn history of Zombies that entered under this controller, including ones that have since left; EnteredThisTurn only filters objects currently present in a queried zone.
pub(in crate::card::sets) static GERALF_THE_FLESHWRIGHT: CardRecord = CardRecord::new(
    "Geralf, the Fleshwright",
    "afe3b678-b340-4c53-bbf6-19252a809d73",
    "Chris Rahn",
    CardRules::unsupported(),
);

// OTJ 51 — Geyser Drake
// Audit: unsupported — Needs a spell-cost adjustment gated by the active player; the cost evaluator accepts direct ModifyCost effects but does not evaluate a surrounding turn condition.
pub(in crate::card::sets) static GEYSER_DRAKE: CardRecord = CardRecord::new(
    "Geyser Drake",
    "b270377b-33eb-4e5e-9d14-0da2876da74f",
    "Daniel Romanovsky",
    CardRules::unsupported(),
);

// OTJ 52 — Harrier Strix
pub(in crate::card::sets) static HARRIER_STRIX: CardRecord = CardRecord::new(
    "Harrier Strix",
    "70ca61a9-8938-41bf-bd14-5759f4de6521",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{U}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, tap target permanent.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated(
            "{2}{U}: Draw a card, then discard a card.",
            &[CostDef::Mana(mana_cost!("{2}{U}"))],
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

// OTJ 53 — Jailbreak Scheme
pub(in crate::card::sets) static JAILBREAK_SCHEME: CardRecord = CardRecord::new(
    "Jailbreak Scheme",
    "a4be8e47-9006-4770-8d99-68a684064a43",
    "Inkognit",
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{3}"))],
            AbilityDef::spell_with_targets(
                "Put a +1/+1 counter on target creature. It can't be blocked \
                 this turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Target artifact or creature's owner puts it on their choice \
                 of the top or bottom of their library.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
                EffectDef::ChooseEffect {
                    player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
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
            ),
        ),
    ])]),
);

// OTJ 54 — The Key to the Vault
pub(in crate::card::sets) static THE_KEY_TO_THE_VAULT: CardRecord = CardRecord::new(
    "The Key to the Vault",
    "166814af-a444-4a62-937e-7491673d9387",
    "Leon Tukker",
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage to a player, \
                 look at that many cards from the top of your library. You may \
                 exile a nonland card from among them. Put the rest on the \
                 bottom of your library in a random order. You may cast the \
                 exiled card without paying its mana cost.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::AttachedToSource),
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::TriggerEventAmount,
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    minimum: 0,
                    maximum: 1,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Exile,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("exiled"),
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
                            EffectDef::MayPlayWithoutPaying(FreePlayDef {
                                objects: ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("exiled"),
                                ),
                                duration: FreePlayDurationDef::WhileResolving,
                                mandatory: false,
                                grants_haste: false,
                            }),
                        ]),
                    },
                }),
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}{U}"))], "Equip {2}{U}"),
        ]),
);

// OTJ 55 — Loan Shark
pub(in crate::card::sets) static LOAN_SHARK: CardRecord = CardRecord::new(
    "Loan Shark",
    "49f12760-ff07-4c9f-a7a9-1e64bd3a9adf",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Shark", "Rogue"], 3, 4).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if you've cast two or more spells \
             this turn, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SpellsCastThisTurn {
                quantifier: QuantifierDef::Any,
                player: PlayerRelation::You,
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 2,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// OTJ 56 — Marauding Sphinx
pub(in crate::card::sets) static MARAUDING_SPHINX: CardRecord = CardRecord::new(
    "Marauding Sphinx",
    "34071884-c5b6-42c0-9eb3-9f32910c29d8",
    "Mila Pesic",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Sphinx", "Rogue"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::triggered(
            "Whenever you commit a crime, surveil 2. This ability triggers \
             only once each turn. (Targeting opponents, anything they \
             control, and/or cards in their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            abilities::surveil(ValueDef::Constant(2)),
        )
        .triggering_at_most(1),
    ]),
);

// OTJ 57 — Metamorphic Blast
pub(in crate::card::sets) static METAMORPHIC_BLAST: CardRecord = CardRecord::new(
    "Metamorphic Blast",
    "bd38d922-cfc1-43a8-82d8-5de441c71076",
    "Michal Ivan",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Until end of turn, target creature becomes a white Rabbit \
                 with base power and toughness 0/1.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Rabbit",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(1),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{3}"))],
            AbilityDef::spell_with_targets(
                "Target player draws two cards.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                abilities::draw_cards(ValueDef::Constant(2)),
            ),
        ),
    ])]),
);

// OTJ 58 — Nimble Brigand
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static NIMBLE_BRIGAND: CardRecord = CardRecord::new(
    "Nimble Brigand",
    "73c74d48-362d-4c3b-9ff7-39bdd19657a6",
    "Kim Sokol",
    CardRules::unsupported(),
);

// OTJ 59 — Outlaw Stitcher
pub(in crate::card::sets) static OUTLAW_STITCHER: CardRecord = CardRecord::new(
    "Outlaw Stitcher",
    "ba9584f0-55b8-448d-99a7-041934053f42",
    "Alix Branwyn",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Warlock"], 1, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 2/2 blue and black Zombie \
             Rogue creature token, then put two +1/+1 counters on that \
             token for each spell you've cast this turn other than the \
             first.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(ZOMBIE_ROGUE_TOKEN)).with_created_tokens(
                    CreatedTokensDef {
                        binding: crate::Binding!("zombie"),
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("zombie"),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Scaled(&ScaledValueDef {
                                value: ValueDef::Sum(&SumValueDef::new(
                                    ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                                        player: PlayerRelation::You,
                                        spell: ObjectPredicateDef::Any,
                                    }),
                                    ValueDef::Constant(-1),
                                )),
                                factor: 2,
                            }),
                        },
                    },
                ),
            ),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{4}{U}"))]),
    ]),
);

// OTJ 60 — Peerless Ropemaster
pub(in crate::card::sets) static PEERLESS_ROPEMASTER: CardRecord = CardRecord::new(
    "Peerless Ropemaster",
    "ae044509-2f31-4506-a124-0e445b7181a2",
    "Wayne Wu",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Rogue"], 4, 4).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one target tapped \
             creature to its owner's hand.",
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
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// OTJ 61 — Phantom Interference
pub(in crate::card::sets) static PHANTOM_INTERFERENCE: CardRecord = CardRecord::new(
    "Phantom Interference",
    "00bf4dd1-5468-4594-9c7b-0737610f19d4",
    "Ruxing Gao",
    // Two mana to counter, four to do both, and never dead: spree is what
    // lets one card be a Spirit on the turn nothing needs answering.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(spree(&[
        (
            &[CostDef::Mana(mana_cost!("{3}"))],
            AbilityDef::spell(
                "Create a 2/2 white Spirit creature token with flying.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Spirit"], &[ManaColor::White], 2, 2)
                        .with_abilities(&[abilities::flying()]),
                ))),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Counter target spell unless its controller pays {2}.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Spell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(
                    2,
                ))]),
            ),
        ),
    ])),
);

// OTJ 62 — Plan the Heist
pub(in crate::card::sets) static PLAN_THE_HEIST: CardRecord = CardRecord::new(
    "Plan the Heist",
    "1e8c3a0e-d61c-457f-ac85-577d0bb94b96",
    "Fariba Khamseh",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Surveil 3 if you have no cards in hand. Then draw three \
             cards. (To surveil 3, look at the top three cards of your \
             library, then put any number of them into your graveyard and \
             the rest on top of your library in any order.)",
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerRelation::You,
                        )),
                        comparison: ComparisonDef::Equal,
                        right: ValueDef::Constant(0),
                    }),
                    then: &abilities::surveil(ValueDef::Constant(3)),
                },
                abilities::draw_cards(ValueDef::Constant(3)),
            ]),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// OTJ 63 — Razzle-Dazzler
pub(in crate::card::sets) static RAZZLE_DAZZLER: CardRecord = CardRecord::new(
    "Razzle-Dazzler",
    "f7d08008-9272-405e-82ef-566e6d42bb17",
    "Wayne Wu",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast your second spell each turn, put a +1/+1 \
             counter on this creature. It can't be blocked this turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
            },
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
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

// OTJ 64 — Seize the Secrets
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static SEIZE_THE_SECRETS: CardRecord = CardRecord::new(
    "Seize the Secrets",
    "1bdbdfa8-aa28-4b3d-95e7-3d0e7e37f982",
    "Miranda Meeks",
    CardRules::unsupported(),
);

// OTJ 65 — Shackle Slinger
pub(in crate::card::sets) static SHACKLE_SLINGER: CardRecord = CardRecord::new(
    "Shackle Slinger",
    "e6cfe383-e483-47e7-99d1-991a06b089bc",
    "Josh Hass",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Soldier"], 3, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever you cast your second spell each turn, choose target \
             creature an opponent controls. If it's tapped, put a stun \
             counter on it. Otherwise, tap it. (If a permanent with a stun \
             counter would become untapped, remove one from it instead.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Tapped,
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(1),
                },
                otherwise: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// OTJ 66 — Shifting Grift
pub(in crate::card::sets) static SHIFTING_GRIFT: CardRecord = CardRecord::new(
    "Shifting Grift",
    "20b8313b-a680-4dca-959a-1a7fa5cb4b1b",
    "Nereida",
    CardRules::new_sorcery(mana_cost!("{U}{U}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Exchange control of two target creatures.",
                &[
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    ))
                    .another(),
                ],
                EffectDef::ExchangeControl {
                    first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    second: EffectRecipientDef::Target(TargetIndex(1)),
                    otherwise: None,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Exchange control of two target artifacts.",
                &[
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Artifact,
                    )),
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Artifact,
                    ))
                    .another(),
                ],
                EffectDef::ExchangeControl {
                    first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    second: EffectRecipientDef::Target(TargetIndex(1)),
                    otherwise: None,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Exchange control of two target enchantments.",
                &[
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Enchantment,
                    )),
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Enchantment,
                    ))
                    .another(),
                ],
                EffectDef::ExchangeControl {
                    first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    second: EffectRecipientDef::Target(TargetIndex(1)),
                    otherwise: None,
                },
            ),
        ),
    ])]),
);

// OTJ 67 — Slickshot Lockpicker
pub(in crate::card::sets) static SLICKSHOT_LOCKPICKER: CardRecord = CardRecord::new(
    "Slickshot Lockpicker",
    "109a9464-ce30-4747-874e-3bbf75913081",
    "Wei Wei",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 2, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target instant or sorcery card in \
             your graveyard gains flashback until end of turn. The \
             flashback cost is equal to its mana cost. (You may cast that \
             card from your graveyard for its flashback cost. Then exile \
             it.)",
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
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flashback_for_card_mana_cost()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
    ]),
);

// OTJ 68 — Slickshot Vault-Buster
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static SLICKSHOT_VAULT_BUSTER: CardRecord = CardRecord::new(
    "Slickshot Vault-Buster",
    "592ccc36-3d10-4a12-8743-9b300b80cb4d",
    "Julia Metzger",
    CardRules::unsupported(),
);

// OTJ 69 — Spring Splasher
pub(in crate::card::sets) static SPRING_SPLASHER: CardRecord = CardRecord::new(
    "Spring Splasher",
    "b6822d12-1a25-42e7-94cc-71bd29daed93",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Frog", "Beast"], 2, 1).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, target creature defending \
             player controls gets -3/-0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::DefendingPlayer),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// OTJ 70 — Step Between Worlds
// Audit: unsupported — Needs all players to commit optional hand-and-graveyard shuffle choices in APNAP order before the simultaneous shuffle, then draw only for the players who chose to shuffle.
pub(in crate::card::sets) static STEP_BETWEEN_WORLDS: CardRecord = CardRecord::new(
    "Step Between Worlds",
    "70ea2054-3d22-42ce-ab50-501ef09c2128",
    "Chris Ostrowski",
    CardRules::unsupported(),
);

// OTJ 71 — Stoic Sphinx
pub(in crate::card::sets) static STOIC_SPHINX: CardRecord = CardRecord::new(
    "Stoic Sphinx",
    "f93f5055-30d8-4fc4-afa5-29212e8c7536",
    "Andreas Zafiratos",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Sphinx"], 5, 3).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature has hexproof as long as you haven't cast a \
             spell this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::You,
                        spell: ObjectPredicateDef::Any,
                    }),
                    comparison: ComparisonDef::Equal,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                },
            },
        ),
    ]),
);

// OTJ 72 — Stop Cold
pub(in crate::card::sets) static STOP_COLD: CardRecord = CardRecord::new(
    "Stop Cold",
    "9ff9c158-0080-427c-8cf9-0289011ea63e",
    "David Astruga",
    CardRules::new_enchantment(mana_cost!("{3}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::spell_with_targets(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted permanent.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent loses all abilities and doesn't untap \
                 during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                        AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                    ]),
                },
            ),
        ]),
);

// OTJ 73 — Take the Fall
pub(in crate::card::sets) static TAKE_THE_FALL: CardRecord = CardRecord::new(
    "Take the Fall",
    "9fea80c4-923c-40a1-9363-bfa4c267a024",
    "Eduardo Francisco",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -1/-0 until end of turn. It gets -4/-0 \
         until end of turn instead if you control an outlaw. \
         (Assassins, Mercenaries, Pirates, Rogues, and Warlocks are \
         outlaws.)\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-4),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// OTJ 74 — This Town Ain't Big Enough
// Audit: unsupported — Needs a self-cost condition testing whether any declared target is a permanent controlled by the caster; current spell-cost predicates only test whether a spell targets the external cost source.
pub(in crate::card::sets) static THIS_TOWN_AIN_T_BIG_ENOUGH: CardRecord = CardRecord::new(
    "This Town Ain't Big Enough",
    "bb206e27-da4d-4abe-9d8c-6d18c5f2f52a",
    "Andrew Mar",
    CardRules::unsupported(),
);

// OTJ 75 — Three Steps Ahead
pub(in crate::card::sets) static THREE_STEPS_AHEAD: CardRecord = CardRecord::new(
    "Three Steps Ahead",
    "8fffd839-2337-4a14-9312-cee085a17f4b",
    "Francisco Miyara",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            AbilityDef::spell_with_targets(
                "Counter target spell.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Spell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::counter_target(TargetIndex::PRIMARY),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{3}"))],
            AbilityDef::spell_with_targets(
                "Create a token that's a copy of target artifact or creature \
                 you control.",
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
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(&TokenCopyDef {
                    object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    exceptions: CopyExceptionsDef::NONE,
                }))),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell(
                "Draw two cards, then discard a card.",
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
        ),
    ])]),
);

// OTJ 76 — Visage Bandit
pub(in crate::card::sets) static VISAGE_BANDIT: CardRecord = CardRecord::new(
    "Visage Bandit",
    "685ec4c6-3332-498f-8b56-d7ad8fc5230c",
    "Miranda Meeks",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Shapeshifter", "Rogue"], 2, 2).with_abilities(
        &[
            AbilityDef::as_enters(
                "You may have this creature enter as a copy of a creature you \
                 control, except it's a Shapeshifter Rogue in addition to its \
                 other types.",
                ReplacementEffectDef::CopyEntering {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    exceptions: CopyExceptionsDef {
                        added_creature_types: CreatureTypeSetDef::named(&["Shapeshifter", "Rogue"]),
                        ..CopyExceptionsDef::NONE
                    },
                },
            ),
            abilities::plot(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
        ],
    ),
);

// OTJ 77 — Ambush Gigapede
pub(in crate::card::sets) static AMBUSH_GIGAPEDE: CardRecord = CardRecord::new(
    "Ambush Gigapede",
    "93b17d09-974a-4e33-b14d-5fe4230ab241",
    "Kekai Kotaki",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Insect"], 6, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature an opponent \
             controls gets -2/-2 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// OTJ 78 — Binding Negotiation
// Audit: unsupported — Needs selecting only face-up cards from an opponent's exile zone for the declined-discard branch; the object predicate vocabulary has no face-up/face-down selector.
pub(in crate::card::sets) static BINDING_NEGOTIATION: CardRecord = CardRecord::new(
    "Binding Negotiation",
    "1c4c26b9-981f-47cf-b0f4-769e788d9537",
    "Caroline Gariba",
    CardRules::unsupported(),
);

// OTJ 79 — Blacksnag Buzzard
pub(in crate::card::sets) static BLACKSNAG_BUZZARD: CardRecord = CardRecord::new(
    "Blacksnag Buzzard",
    "ef10b2ad-9b9b-4c5d-a2c7-3ce742224b50",
    "Michele Giorgi",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Bird"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::defined_replacement(
            "This creature enters with a +1/+1 counter on it if a creature \
             died this turn.",
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::SourceEntersBattlefield)
                .with_condition(ReplacementConditionDef::CreatureDiedThisTurn),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{B}"))]),
    ]),
);

// OTJ 80 — Blood Hustler
pub(in crate::card::sets) static BLOOD_HUSTLER: CardRecord = CardRecord::new(
    "Blood Hustler",
    "1016a750-2a18-4443-a600-957eb4026d3a",
    "Anna Pavleeva",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you commit a crime, put a +1/+1 counter on this \
             creature. This ability triggers only once each turn. \
             (Targeting opponents, anything they control, and/or cards in \
             their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .triggering_at_most(1),
        AbilityDef::activated_with_targets(
            "{3}{B}: Target opponent loses 1 life and you gain 1 life.",
            &[CostDef::Mana(mana_cost!("{3}{B}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
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

// OTJ 81 — Boneyard Desecrator
pub(in crate::card::sets) static BONEYARD_DESECRATOR: CardRecord = CardRecord::new(
    "Boneyard Desecrator",
    "d43981b1-60c8-4896-8885-b07e73b99b30",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Mercenary"], 3, 4).with_abilities(
        &[
            abilities::menace(),
            AbilityDef::activated(
                "{1}{B}, Sacrifice another creature: Put a +1/+1 counter on \
                 this creature. If an outlaw was sacrificed this way, create a \
                 Treasure token. (Assassins, Mercenaries, Pirates, Rogues, and \
                 Warlocks are outlaws.)",
                &[
                    CostDef::Mana(mana_cost!("{1}{B}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ])),
                ],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                                    AdditionalCostObjectIndex::PRIMARY,
                                )),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                                            "Assassin",
                                        )),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                                            "Mercenary",
                                        )),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                                    ]),
                                ),
                            },
                        ),
                        then: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                .with_count(ValueDef::Constant(1)),
                        ),
                    },
                ]),
            ),
        ],
    ),
);

// OTJ 82 — Caustic Bronco
/// The reveal itself: one card off the top, shown to everybody, into your
/// hand, and then the clause above reads what it cost.
pub(in crate::card::sets) static CAUSTIC_BRONCO: CardRecord = CardRecord::new(
    "Caustic Bronco",
    "e9a268ba-c442-4fe4-90b4-2810c8474f4e",
    "Brent Hollowell",
// Two mana for a 2/2 that draws you an extra card every attack. Whether
    // that card costs you or them is what the saddle buys.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Snake", "Horse", "Mount"], 2, 2)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever this creature attacks, reveal the top card of your library and put it \
                 into your hand. You lose life equal to that card's mana value if this creature \
                 isn't saddled. Otherwise, each opponent loses that much life.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(1),
                    &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            then: &EffectDef::None,
                        }),
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                            moved: Some(ParentBinding),
                            then: &EffectDef::IfElseCondition {
                                condition: &TriggerConditionDef::SourceMatches {
                                    object: ObjectPredicateDef::Saddled,
                                },
                                then: &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::Opponent,
                                    amount: ValueDef::AggregateObjectValues(
                                        &ObjectValueAggregateDef {
                                            objects: ObjectSetDef::Binding(ParentBinding),
                                            select: ObjectValueDef::ManaValue,
                                            operation: AggregateOperationDef::Maximum,
                                        },
                                    ),
                                },
                                otherwise: &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::AggregateObjectValues(
                                        &ObjectValueAggregateDef {
                                            objects: ObjectSetDef::Binding(ParentBinding),
                                            select: ObjectValueDef::ManaValue,
                                            operation: AggregateOperationDef::Maximum,
                                        },
                                    ),
                                },
                            },
                        }),
                    ]),
                ),
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 3 }],
                "Saddle 3 (Tap any number of other creatures you control with total power 3 or \
                                         more: This Mount becomes saddled until end of turn. Saddle only as a sorcery.)",
            ),
        ]),
);

// OTJ 83 — Consuming Ashes
pub(in crate::card::sets) static CONSUMING_ASHES: CardRecord = CardRecord::new(
    "Consuming Ashes",
    "54f96be9-60fc-4e2f-9172-4cc53c9a095a",
    "Campbell White",
    CardRules::new_instant(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature. If it had mana value 3 or less, \
             surveil 2. (Look at the top two cards of your library, then \
             put any number of them into your graveyard and the rest on \
             top of your library in any order.)",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMost(3),
                },
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    abilities::surveil(ValueDef::Constant(2)),
                ]),
                otherwise: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// OTJ 84 — Corrupted Conviction (reprint)
const CORRUPTED_CONVICTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mom::CORRUPTED_CONVICTION,
    "8046f892-3317-4ef7-9cf7-97b9060540c8",
    "Inkognit",
);

// OTJ 85 — Desert's Due
pub(in crate::card::sets) static DESERT_S_DUE: CardRecord = CardRecord::new(
    "Desert's Due",
    "35e899e4-e2de-44cf-b6f4-cace8d3770cb",
    "David Palumbo",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -2/-2 until end of turn. It gets an \
         additional -1/-1 until end of turn for each Desert you \
         control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Negate(&ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(2),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ))),
                ValueDef::Negate(&ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(2),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ))),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// OTJ 86 — Desperate Bloodseeker
pub(in crate::card::sets) static DESPERATE_BLOODSEEKER: CardRecord = CardRecord::new(
    "Desperate Bloodseeker",
    "a59da027-b5dd-4920-b3a1-9da05fcb1977",
    "Camille Alquier",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 2).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target player mills two cards. \
             (They put the top two cards of their library into their \
             graveyard.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// OTJ 87 — Fake Your Own Death (reprint)
const FAKE_YOUR_OWN_DEATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::FAKE_YOUR_OWN_DEATH,
    "79a17ab9-13c9-41d4-a143-82d8caacfd8b",
    "Monztre",
);

// OTJ 88 — Forsaken Miner
pub(in crate::card::sets) static FORSAKEN_MINER: CardRecord = CardRecord::new(
    "Forsaken Miner",
    "1679f74d-00f8-436c-9f8c-aa3f843a546c",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{B}"), &["Skeleton", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        AbilityDef::triggered(
            "Whenever you commit a crime, you may pay {B}. If you do, \
             return this card from your graveyard to the battlefield. \
             (Targeting opponents, anything they control, and/or cards in \
             their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{B}"))],
                &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// OTJ 89 — Gisa, the Hellraiser
pub(in crate::card::sets) static GISA_THE_HELLRAISER: CardRecord = CardRecord::new(
    "Gisa, the Hellraiser",
    "db7c07b2-02b2-4e62-bf1b-4848e06eec28",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Human", "Warlock"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ward(
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::PayLife(2)],
                "Ward—{2}, Pay 2 life.",
            ),
            AbilityDef::static_ability(
                "Skeletons and Zombies you control get +1/+1 and have menace.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Skeleton")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::menace()),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "Whenever you commit a crime, create two tapped 2/2 blue and \
                 black Zombie Rogue creature tokens. This ability triggers \
                 only once each turn. (Targeting opponents, anything they \
                 control, and/or cards in their graveyards is a crime.)",
                TriggerEventDef::CommittedCrime(PlayerRelation::You),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(ZOMBIE_ROGUE_TOKEN))
                        .with_count(ValueDef::Constant(2))
                        .entering_tapped(),
                ),
            )
            .triggering_at_most(1),
        ]),
);

// OTJ 90 — Hollow Marauder
// Audit: unsupported — Needs each targeted opponent's discard result retained with its mana value, including an empty-hand failure, to decide the controller's draw count; ordinary Discard does not bind those per-player results.
pub(in crate::card::sets) static HOLLOW_MARAUDER: CardRecord = CardRecord::new(
    "Hollow Marauder",
    "df2913d5-57c7-4f9b-bc96-8a46beef2563",
    "Wero Gallo",
    CardRules::unsupported(),
);

// OTJ 91 — Insatiable Avarice
pub(in crate::card::sets) static INSATIABLE_AVARICE: CardRecord = CardRecord::new(
    "Insatiable Avarice",
    "a108b0e4-1b43-4659-9e91-facb0bd57ebb",
    "Scott Murphy",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell(
                "Search your library for a card, then shuffle and put that \
                 card on top.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Any,
                    minimum: 1,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{B}{B}"))],
            AbilityDef::spell_with_targets(
                "Target player draws three cards and loses 3 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(3)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
        ),
    ])]),
);

// OTJ 92 — Kaervek, the Punisher
// Audit: unsupported — Needs copying a card in exile and offering to cast that copy with normal payment, plus a continuation when that cast is accepted; CopyStackObject only copies an existing spell or ability.
pub(in crate::card::sets) static KAERVEK_THE_PUNISHER: CardRecord = CardRecord::new(
    "Kaervek, the Punisher",
    "7f3affc1-be42-48c7-89ff-b59550ff278c",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// OTJ 93 — Lively Dirge
pub(in crate::card::sets) static LIVELY_DIRGE: CardRecord = CardRecord::new(
"Lively Dirge",
"0c35a0d3-12f7-46f3-a6b3-02a490d45ca0",
"Warren Mahy",
CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[
AbilityDef::spree(&[(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell("Search your library for a card, put it into your graveyard, then shuffle.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::Any, minimum: 1, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Graveyard, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })), (&[CostDef::Mana(mana_cost!("{2}"))], AbilityDef::spell("Return up to two creature cards with total mana value 4 or less from your graveyard to the battlefield.", EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMost(4)]), &[ZoneKind::Graveyard], PlayerRelation::You)), exclude: None, minimum: 0, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("dirge_first")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::IfElseCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::BoundObjectCount(Binding!("dirge_first")), comparison: ComparisonDef::Greater, right: ValueDef::Constant(0) }), then: &EffectDef::ForEachInBinding { objects: Binding!("dirge_first"), binding: Binding!("dirge_first_card"), effect: &EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Matching { objects: &ObjectSetDef::ExceptObject { objects: &ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Graveyard], PlayerRelation::You)), object: ObjectRefDef::Binding(Binding!("dirge_first_card")) }, object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::ManaValueAtMostValue(ValueDef::Sum(&SumValueDef { left: ValueDef::Constant(4), right: ValueDef::Negate(&ValueDef::AggregateObjectValues(&ObjectValueAggregateDef { objects: ObjectSetDef::Binding(Binding!("dirge_first")), select: ObjectValueDef::ManaValue, operation: AggregateOperationDef::Sum })) }))) }, exclude: None, minimum: 0, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("dirge_second")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::MoveObjects(MoveObjectsDef { input: ObjectSetDef::Union(&[ObjectSetDef::Binding(Binding!("dirge_first")), ObjectSetDef::Binding(Binding!("dirge_second"))]), from: Some(ZoneKind::Graveyard), zone: ZoneKind::Battlefield, placement: ZonePlacement::Top, moved: None, then: &EffectDef::None }) }) }, otherwise: &EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMost(4)]), &[ZoneKind::Graveyard], PlayerRelation::You)), exclude: None, minimum: 0, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("dirge_only")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("dirge_only"))), ZoneKind::Battlefield, ZonePlacement::Top) }) } })))])
]),
);

// OTJ 94 — Mourner's Surprise
pub(in crate::card::sets) static MOURNER_S_SURPRISE: CardRecord = CardRecord::new(
    "Mourner's Surprise",
    "980b0b68-7218-49c6-b6bf-022218f3abf4",
    "Zuzanna Wużyk",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return up to one target creature card from your graveyard to \
         your hand. Create a 1/1 red Mercenary creature token with \
         \"{T}: Target creature you control gets +1/+0 until end of \
         turn. Activate only as a sorcery.\"",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
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
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ]),
    )]),
);

// OTJ 95 — Neutralize the Guards
pub(in crate::card::sets) static NEUTRALIZE_THE_GUARDS: CardRecord = CardRecord::new(
    "Neutralize the Guards",
    "60f1a481-598e-4e05-8471-eedb12a39022",
    "Nereida",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Creatures target opponent controls get -1/-1 until end of \
         turn. Surveil 2. (Look at the top two cards of your library, \
         then put any number of them into your graveyard and the rest \
         on top of your library in any order.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::surveil(ValueDef::Constant(2)),
        ]),
    )]),
);

// OTJ 96 — Nezumi Linkbreaker
pub(in crate::card::sets) static NEZUMI_LINKBREAKER: CardRecord = CardRecord::new(
    "Nezumi Linkbreaker",
    "6dd0095b-6136-4368-94cb-4c82621aaf37",
    "Miro Petrov",
    CardRules::new_creature(mana_cost!("{B}"), &["Rat", "Warlock"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 red Mercenary creature \
             token with \"{T}: Target creature you control gets +1/+0 \
             until end of turn. Activate only as a sorcery.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ),
    ]),
);

// OTJ 97 — Overzealous Muscle
pub(in crate::card::sets) static OVERZEALOUS_MUSCLE: CardRecord = CardRecord::new(
    "Overzealous Muscle",
    "ce57d977-e9c5-4ed1-915f-cdc90a54f8f8",
    "Sam White",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Ogre", "Mercenary"], 5, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you commit a crime during your turn, this creature \
             gains indestructible until end of turn. (Targeting opponents, \
             anything they control, and/or cards in their graveyards is a \
             crime. Damage and effects that say \"destroy\" don't destroy \
             a creature with indestructible.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::CommittedCrime(PlayerRelation::You),
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// OTJ 98 — Pitiless Carnage
pub(in crate::card::sets) static PITILESS_CARNAGE: CardRecord = CardRecord::new(
    "Pitiless Carnage",
    "fa76fc45-a106-4dc9-9d44-a005eaa2784d",
    "Richard Kane Ferguson",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Sacrifice any number of permanents you control, then draw \
             that many cards.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::Sequence(&[
                    EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                        crate::Binding!("chosen"),
                    ))),
                    abilities::draw_cards(ValueDef::CountObjects(&ObjectSetDef::Binding(
                        crate::Binding!("chosen"),
                    ))),
                ]),
            }),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{B}{B}"))]),
    ]),
);

// OTJ 99 — Rakish Crew
pub(in crate::card::sets) static RAKISH_CREW: CardRecord = CardRecord::new(
    "Rakish Crew",
    "70f64358-58db-40ab-90e8-6137c3bd0a29",
    "Ilse Gort",
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 1/1 red Mercenary \
             creature token with \"{T}: Target creature you control gets \
             +1/+0 until end of turn. Activate only as a sorcery.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ),
        AbilityDef::triggered(
            "Whenever an outlaw you control dies, each opponent loses 1 \
             life and you gain 1 life. (Assassins, Mercenaries, Pirates, \
             Rogues, and Warlocks are outlaws.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
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

// OTJ 100 — Rattleback Apothecary
pub(in crate::card::sets) static RATTLEBACK_APOTHECARY: CardRecord = CardRecord::new(
    "Rattleback Apothecary",
    "9a88e233-f09c-49e7-b1e3-386fba851fdf",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Gorgon", "Warlock"], 3, 2).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered_with_targets(
            "Whenever you commit a crime, target creature you control \
             gains your choice of menace or lifelink until end of turn. \
             (Targeting opponents, anything they control, and/or cards in \
             their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::Controller,
                choices: &[
                    EffectChoiceDef {
                        label: "Menace",
                        effect: EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&abilities::menace()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    },
                    EffectChoiceDef {
                        label: "Lifelink",
                        effect: EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    },
                ],
            },
        ),
    ]),
);

// OTJ 101 — Raven of Fell Omens
pub(in crate::card::sets) static RAVEN_OF_FELL_OMENS: CardRecord = CardRecord::new(
    "Raven of Fell Omens",
    "e2df3cb4-1658-450a-912a-df336706acdc",
    "Justin Cornell",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Bird"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you commit a crime, each opponent loses 1 life and \
             you gain 1 life. This ability triggers only once each turn. \
             (Targeting opponents, anything they control, and/or cards in \
             their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
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
        )
        .triggering_at_most(1),
    ]),
);

// OTJ 102 — Rictus Robber
pub(in crate::card::sets) static RICTUS_ROBBER: CardRecord = CardRecord::new(
    "Rictus Robber",
    "252def94-2d89-48f7-8ff7-9c8682ca3ec6",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Rogue"], 4, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if a creature died this turn, \
             create a 2/2 blue and black Zombie Rogue creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::CreatureDiedThisTurn,
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ZOMBIE_ROGUE_TOKEN))),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{B}"))]),
    ]),
);

// OTJ 103 — Rooftop Assassin
pub(in crate::card::sets) static ROOFTOP_ASSASSIN: CardRecord = CardRecord::new(
    "Rooftop Assassin",
    "e4e4311a-8583-4cf0-8126-508dbfbcddb6",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Assassin"], 2, 2).with_abilities(
        &[
            abilities::flash(),
            abilities::flying(),
            abilities::lifelink(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, destroy target creature an \
                 opponent controls that was dealt damage this turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::WasDealtDamageThisTurn,
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    ),
);

// OTJ 104 — Rush of Dread
pub(in crate::card::sets) static RUSH_OF_DREAD: CardRecord = CardRecord::new(
    "Rush of Dread",
    "721c7122-91b6-45ea-ba28-de0246a2fc1b",
    "Chris Seaman",
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Target opponent sacrifices half the creatures they control of \
                 their choice, rounded up.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Halved(&HalvedValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        )),
                        rounding: RoundingDef::Up,
                    })),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Target opponent discards half the cards in their hand, \
                 rounded up.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Halved(&HalvedValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerRelation::Opponent,
                        )),
                        rounding: RoundingDef::Up,
                    }),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Target opponent loses half their life, rounded up.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Halved(&HalvedValueDef {
                        value: ValueDef::LifeTotal(PlayerRelation::Opponent),
                        rounding: RoundingDef::Up,
                    }),
                },
            ),
        ),
    ])]),
);

// OTJ 105 — Servant of the Stinger
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static SERVANT_OF_THE_STINGER: CardRecord = CardRecord::new(
    "Servant of the Stinger",
    "b5c98650-b195-4071-8e02-4df35fddddc7",
    "Steven Russell Black",
    CardRules::unsupported(),
);

// OTJ 106 — Shoot the Sheriff
pub(in crate::card::sets) static SHOOT_THE_SHERIFF: CardRecord = CardRecord::new(
    "Shoot the Sheriff",
    "180d6528-c524-4bb8-8a72-b3775cd2c177",
    "Fariba Khamseh",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target non-outlaw creature. (Assassins, Mercenaries, \
         Pirates, Rogues, and Warlocks are outlaws. Everyone else is \
         fair game.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                ])),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )]),
);

// OTJ 107 — Skulduggery (reprint)
const SKULDUGGERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::SKULDUGGERY,
    "03709166-164a-4075-ad0d-ea3b516ab771",
    "Miro Petrov",
);

// OTJ 108 — Tinybones Joins Up
pub(in crate::card::sets) static TINYBONES_JOINS_UP: CardRecord = CardRecord::new(
    "Tinybones Joins Up",
    "5724a15f-0ba0-421a-9cd4-a2b701e6141f",
    "Wylie Beckert",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When Tinybones Joins Up enters, any number of target players \
                 each discard a card.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    2,
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever a legendary creature you control enters, any number \
                 of target players each mill a card and lose 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    2,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Mill {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// OTJ 109 — Tinybones, the Pickpocket
// Audit: unsupported — Needs a resolution-scoped permission to cast the targeted graveyard card with normal costs and mana of any type; existing graveyard offers either waive the mana cost or defer casting until later.
pub(in crate::card::sets) static TINYBONES_THE_PICKPOCKET: CardRecord = CardRecord::new(
    "Tinybones, the Pickpocket",
    "3d3025a2-4a17-4137-ba0b-bd676c6f5f88",
    "Ekaterina Burmak",
    CardRules::unsupported(),
);

// OTJ 110 — Treasure Dredger
pub(in crate::card::sets) static TREASURE_DREDGER: CardRecord = CardRecord::new(
    "Treasure Dredger",
    "c88df498-147c-4609-8a94-d8d10a87e37c",
    "Nereida",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}, Pay 1 life: Create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::PayLife(1),
            ],
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// OTJ 111 — Unfortunate Accident
pub(in crate::card::sets) static UNFORTUNATE_ACCIDENT: CardRecord = CardRecord::new(
    "Unfortunate Accident",
    "0c5a25d9-926e-4004-8830-2b2bf7bc0775",
    "Josiah \"Jo\" Cameron",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}{B}"))],
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
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell(
                "Create a 1/1 red Mercenary creature token with \"{T}: Target \
                 creature you control gets +1/+0 until end of turn. Activate \
                 only as a sorcery.\"",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Mercenary"], &[ManaColor::Red], 1, 1)
                        .with_abilities(&[AbilityDef::activated_with_targets(
                            "{T}: Target creature you control gets +1/+0 until end of \
                         turn. Activate only as a sorcery.",
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
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(0),
                                ),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )
                        .with_activation_timing(ActivationTimingDef::SorcerySpeed)]),
                ))),
            ),
        ),
    ])]),
);

// OTJ 112 — Unscrupulous Contractor
// Audit: unsupported — Needs a reflexive trigger installed by the resolving sacrifice clause and retained after its source leaves, with targets selected when that reflexive trigger goes on the stack.
pub(in crate::card::sets) static UNSCRUPULOUS_CONTRACTOR: CardRecord = CardRecord::new(
    "Unscrupulous Contractor",
    "9e56a8df-db04-4a88-a5ab-6954d3449976",
    "Mila Pesic",
    CardRules::unsupported(),
);

// OTJ 113 — Vadmir, New Blood
pub(in crate::card::sets) static VADMIR_NEW_BLOOD: CardRecord = CardRecord::new(
    "Vadmir, New Blood",
    "828b5855-af5a-46a6-8fd4-0a2e28f3bb01",
    "Andreas Zafiratos",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Rogue"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you commit a crime, put a +1/+1 counter on Vadmir. \
                 This ability triggers only once each turn. (Targeting \
                 opponents, anything they control, and/or cards in their \
                 graveyards is a crime.)",
                TriggerEventDef::CommittedCrime(PlayerRelation::You),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            )
            .triggering_at_most(1),
            AbilityDef::static_ability(
                "As long as Vadmir has four or more +1/+1 counters on it, it \
                 has menace and lifelink.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceCounters {
                        kind: CounterKind::PlusOnePlusOne,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 4,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::menace()),
                            AppliedEffectDef::add_ability(&abilities::lifelink()),
                        ]),
                    },
                },
            ),
        ]),
);

// OTJ 114 — Vault Plunderer
pub(in crate::card::sets) static VAULT_PLUNDERER: CardRecord = CardRecord::new(
    "Vault Plunderer",
    "2e6bf35c-8763-47cc-ab2d-5dbabeb28072",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 3, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target player draws a card and \
             loses 1 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// OTJ 115 — Brimstone Roundup
pub(in crate::card::sets) static BRIMSTONE_ROUNDUP: CardRecord = CardRecord::new(
    "Brimstone Roundup",
    "bd13011e-a4fc-4107-988f-60cfc851ecd3",
    "Milivoj Ćeran",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast your second spell each turn, create a 1/1 \
             red Mercenary creature token with \"{T}: Target creature you \
             control gets +1/+0 until end of turn. Activate only as a \
             sorcery.\"",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{R}"))]),
    ]),
);

// OTJ 116 — Calamity, Galloping Inferno
// Audit: unsupported — Needs the identities of creatures used to saddle this Mount retained for the rest of the turn; current saddle stores only a saddled boolean.
pub(in crate::card::sets) static CALAMITY_GALLOPING_INFERNO: CardRecord = CardRecord::new(
    "Calamity, Galloping Inferno",
    "e7a70f5a-2056-4c26-b6ea-9f751b5d0d8c",
    "Artur Nakhodkin",
    CardRules::unsupported(),
);

// OTJ 117 — Caught in the Crossfire
pub(in crate::card::sets) static CAUGHT_IN_THE_CROSSFIRE: CardRecord = CardRecord::new(
    "Caught in the Crossfire",
    "3a2fd0c4-509e-49c2-ad57-f772efcbc207",
    "Xabi Gaztelua",
    CardRules::new_instant(mana_cost!("{R}{R}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell(
                "Caught in the Crossfire deals 2 damage to each outlaw \
                 creature. (Assassins, Mercenaries, Pirates, Rogues, and \
                 Warlocks are outlaws.)",
                EffectDef::damage(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                            ]),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ))),
                    ValueDef::Constant(2),
                ),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell(
                "Caught in the Crossfire deals 2 damage to each non-outlaw \
                 creature.",
                EffectDef::damage(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                            ])),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ))),
                    ValueDef::Constant(2),
                ),
            ),
        ),
    ])]),
);

// OTJ 118 — Cunning Coyote
pub(in crate::card::sets) static CUNNING_COYOTE: CardRecord = CardRecord::new(
    "Cunning Coyote",
    "5b4ac6ea-c67b-4f90-be4f-aa25882f5794",
    "David Auden Nash",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Coyote"], 2, 2).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, another target creature you \
             control gets +1/+1 and gains haste until end of turn.",
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
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{R}"))]),
    ]),
);

// OTJ 119 — Deadeye Duelist
pub(in crate::card::sets) static DEADEYE_DUELIST: CardRecord = CardRecord::new(
    "Deadeye Duelist",
    "e9a50b7a-8741-4520-8d45-8e6b128c2628",
    "Diana Cearley",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Assassin"], 1, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated_with_targets(
            "{1}, {T}: This creature deals 1 damage to target opponent.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// OTJ 120 — Demonic Ruckus
pub(in crate::card::sets) static DEMONIC_RUCKUS: CardRecord = CardRecord::new(
    "Demonic Ruckus",
    "5b491d11-d00a-4541-8389-2785a455eeee",
    "Andrew Mar",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 and has menace and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::menace()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            abilities::dies_trigger(
                "When this Aura is put into a graveyard from the battlefield, \
                 draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            abilities::plot(&[CostDef::Mana(mana_cost!("{R}"))]),
        ]),
);

// OTJ 121 — Discerning Peddler
pub(in crate::card::sets) static DISCERNING_PEDDLER: CardRecord = CardRecord::new(
    "Discerning Peddler",
    "73b359bf-afd8-439d-a4d2-985db1ad368c",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Rogue"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may discard a card. If you do, \
             draw a card.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ),
    ]),
);

// OTJ 122 — Explosive Derailment
pub(in crate::card::sets) static EXPLOSIVE_DERAILMENT: CardRecord = CardRecord::new(
    "Explosive Derailment",
    "f0e3df9c-0a86-4e6f-a3c7-84a883328a3d",
    "Leon Tukker",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Explosive Derailment deals 4 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::destroy_target(
                "Destroy target artifact.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
            ),
        ),
    ])),
);

// OTJ 123 — Ferocification
pub(in crate::card::sets) static FEROCIFICATION: CardRecord = CardRecord::new(
    "Ferocification",
    "78db4260-6fc8-4500-afd7-2c845ac0d53b",
    "Mila Pesic",
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::modal_triggered(
            "At the beginning of combat on your turn, choose one —",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[
                AbilityDef::spell_with_targets(
                    "Target creature you control gets +2/+0 until end of turn.",
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
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target creature you control gains menace and haste until end \
                     of turn.",
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
                            AppliedEffectDef::add_ability(&abilities::menace()),
                            AppliedEffectDef::add_ability(&abilities::haste()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ],
        ),
    ]),
);

// OTJ 124 — Gila Courser
// Audit: unsupported — Needs exile-play permission expiring at cleanup of its controller's next turn; the current turn-count duration incorrectly remains usable during the following opponent turn.
pub(in crate::card::sets) static GILA_COURSER: CardRecord = CardRecord::new(
    "Gila Courser",
    "f568803d-65c0-48d7-916f-671267a9e00e",
    "Brent Hollowell",
    CardRules::unsupported(),
);

// OTJ 125 — Great Train Heist
// Audit: unsupported — Needs an effect adding combat after the current combat phase and a resolving condition identifying that phase; the existing extra-combat operation is tied to a main-phase sequence.
pub(in crate::card::sets) static GREAT_TRAIN_HEIST: CardRecord = CardRecord::new(
    "Great Train Heist",
    "357dd9b2-5d2d-49f6-86f0-f5c4d63474dd",
    "Campbell White",
    CardRules::unsupported(),
);

// OTJ 126 — Hell to Pay
// Audit: unsupported — Needs a damage-result continuation exposing excess damage after prevention and replacement, to determine the number of tapped Treasures; ordinary damage does not bind excess damage.
pub(in crate::card::sets) static HELL_TO_PAY: CardRecord = CardRecord::new(
    "Hell to Pay",
    "84ad8ed7-1429-432e-8217-a4db3b97675c",
    "Liiga Smilshkalne",
    CardRules::unsupported(),
);

// OTJ 127 — Hellspur Brute
pub(in crate::card::sets) static HELLSPUR_BRUTE: CardRecord = CardRecord::new(
    "Hellspur Brute",
    "3b99db5b-cd13-4e69-98b7-753e72c781f8",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Minotaur", "Mercenary"], 5, 4).with_abilities(
        &[
            AbilityDef::static_ability(
                "Affinity for outlaws (This spell costs {1} less to cast for \
                 each Assassin, Mercenary, Pirate, Rogue, and/or Warlock you \
                 control.)",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::trample(),
        ],
    ),
);

// OTJ 128 — Hellspur Posse Boss
pub(in crate::card::sets) static HELLSPUR_POSSE_BOSS: CardRecord = CardRecord::new(
    "Hellspur Posse Boss",
    "5f348c7e-7d72-40f1-a65f-ee7ff4b09412",
    "Artur Nakhodkin",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Lizard", "Rogue"], 2, 4).with_abilities(&[
        AbilityDef::static_ability(
            "Other outlaws you control have haste. (Assassins, \
             Mercenaries, Pirates, Rogues, and Warlocks are outlaws.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        ),
        abilities::enters_trigger(
            "When this creature enters, create two 1/1 red Mercenary \
             creature tokens with \"{T}: Target creature you control gets \
             +1/+0 until end of turn. Activate only as a sorcery.\"",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))
                    .with_count(ValueDef::Constant(2)),
            ),
        ),
    ]),
);

// OTJ 129 — Highway Robbery
pub(in crate::card::sets) static HIGHWAY_ROBBERY: CardRecord = CardRecord::new(
    "Highway Robbery",
    "31a88429-9204-4a23-a7a8-babbd6bab79f",
    "Scott Murphy",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell(
            "You may discard a card or sacrifice a land. If you do, draw \
             two cards.",
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::Controller,
                choices: &[
                    EffectChoiceDef {
                        label: "Discard a card",
                        effect: EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::DiscardCards(1)],
                            &abilities::draw_cards(ValueDef::Constant(2)),
                        )),
                    },
                    EffectChoiceDef {
                        label: "Sacrifice a land",
                        effect: EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                                CardType::Land,
                            ))],
                            &abilities::draw_cards(ValueDef::Constant(2)),
                        )),
                    },
                ],
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{R}"))]),
    ]),
);

// OTJ 130 — Irascible Wolverine
pub(in crate::card::sets) static IRASCIBLE_WOLVERINE: CardRecord = CardRecord::new(
    "Irascible Wolverine",
    "324c0af5-7cdf-4c71-84cc-6349f10e0d66",
    "Darrell Riche",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolverine"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, exile the top card of your \
             library. Until end of turn, you may play that card.",
            EffectDef::BindObjects(BindObjectsDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                },
                binding: crate::Binding!("top"),
                then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "top"
                    ))),
                },
            }),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{R}"))]),
    ]),
);

// OTJ 131 — Iron-Fist Pulverizer
pub(in crate::card::sets) static IRON_FIST_PULVERIZER: CardRecord = CardRecord::new(
    "Iron-Fist Pulverizer",
    "cd7f984a-0b56-45df-958d-6178e4da61ed",
    "Xabi Gaztelua",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Giant", "Warrior"], 4, 5).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered_with_targets(
            "Whenever you cast your second spell each turn, this creature \
             deals 2 damage to target opponent. Scry 1. (Look at the top \
             card of your library. You may put that card on the bottom.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                },
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
                abilities::scry(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// OTJ 132 — Longhorn Sharpshooter
// Audit: unsupported — Needs a committed plotting event dispatched to triggered abilities; the current special action or saddle effect changes state without publishing that event.
pub(in crate::card::sets) static LONGHORN_SHARPSHOOTER: CardRecord = CardRecord::new(
    "Longhorn Sharpshooter",
    "398d9a16-d72c-42e2-a0ea-d9da642ee046",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// OTJ 133 — Magda, the Hoardmaster
pub(in crate::card::sets) static MAGDA_THE_HOARDMASTER: CardRecord = CardRecord::new(
    "Magda, the Hoardmaster",
    "4443d112-209b-49ec-bc40-3a11dcdb092e",
    "Diego Gisbert",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Berserker"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you commit a crime, create a tapped Treasure token. \
                 This ability triggers only once each turn. (Targeting \
                 opponents, anything they control, and/or cards in their \
                 graveyards is a crime.)",
                TriggerEventDef::CommittedCrime(PlayerRelation::You),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1))
                        .entering_tapped(),
                ),
            )
            .triggering_at_most(1),
            AbilityDef::activated(
                "Sacrifice three Treasures: Create a 4/4 red Scorpion Dragon \
                 creature token with flying and haste. Activate only as a \
                 sorcery.",
                &[CostDef::sacrifice_permanents(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                    PlayerRelation::You,
                    3,
                )],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(
                        &["Scorpion", "Dragon"],
                        &[ManaColor::Red],
                        4,
                        4,
                    )
                    .with_abilities(&[abilities::flying(), abilities::haste()]),
                ))),
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ]),
);

// OTJ 134 — Magebane Lizard
pub(in crate::card::sets) static MAGEBANE_LIZARD: CardRecord = CardRecord::new(
    "Magebane Lizard",
    "62e12566-375f-4f31-aa91-1b13a96d9ece",
    "Camille Alquier",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Lizard"], 1, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a player casts a noncreature spell, this creature \
             deals damage to that player equal to the number of \
             noncreature spells they've cast this turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::Any),
            ])),
            EffectDef::damage(
                EffectRecipientDef::EventPlayer,
                ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                    player: PlayerRelation::EventPlayer,
                    spell: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                }),
            ),
        ),
    ]),
);

// OTJ 135 — Mine Raider
pub(in crate::card::sets) static MINE_RAIDER: CardRecord = CardRecord::new(
    "Mine Raider",
    "19cfacff-e884-4954-aa6b-ed56ca942bf2",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Rogue"], 3, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_if(
            "When this creature enters, if you control another outlaw, \
             create a Treasure token. (Assassins, Mercenaries, Pirates, \
             Rogues, and Warlocks are outlaws. A Treasure token is an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// OTJ 136 — Outlaws' Fury
// Audit: unsupported — Needs exile-play permission expiring at cleanup of its controller's next turn; the current turn-count duration incorrectly remains usable during the following opponent turn.
pub(in crate::card::sets) static OUTLAWS_FURY: CardRecord = CardRecord::new(
    "Outlaws' Fury",
    "f7502b9c-b759-499a-8e94-22f87f5eb142",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// OTJ 137 — Prickly Pair
pub(in crate::card::sets) static PRICKLY_PAIR: CardRecord = CardRecord::new(
    "Prickly Pair",
    "e70f2278-8857-46e4-aa2b-fff5589b750f",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Plant", "Mercenary"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 red Mercenary \
             creature token with \"{T}: Target creature you control gets \
             +1/+0 until end of turn. Activate only as a sorcery.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        ),
    ]),
);

// OTJ 138 — Quick Draw
pub(in crate::card::sets) static QUICK_DRAW: CardRecord = CardRecord::new(
    "Quick Draw",
    "56399cd0-1214-42b6-be38-f2cbd770915f",
    "Lie Setiawan",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 and gains first strike \
         until end of turn. Creatures target opponent controls lose \
         first strike and double strike until end of turn.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent)),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
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
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::FirstStrike,
                    )),
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::DoubleStrike,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// OTJ 139 — Quilled Charger
pub(in crate::card::sets) static QUILLED_CHARGER: CardRecord = CardRecord::new(
    "Quilled Charger",
    "56e326c4-39f1-41ee-9e47-ebe468c51718",
    "Diego Gisbert",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Porcupine", "Mount"], 4, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, it gets +1/+2 \
             and gains menace until end of turn. (It can't be blocked \
             except by two or more creatures.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&abilities::menace()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 2 }],
            "Saddle 2 (Tap any number of other creatures you control with \
             total power 2 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// OTJ 140 — Reckless Lackey
pub(in crate::card::sets) static RECKLESS_LACKEY: CardRecord = CardRecord::new(
    "Reckless Lackey",
    "912fcd14-5e81-418c-997b-771f2f38f63d",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Pirate"], 1, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::haste(),
        AbilityDef::activated(
            "{2}{R}, Sacrifice this creature: Draw a card and create a \
             Treasure token. (It's an artifact with \"{T}, Sacrifice this \
             token: Add one mana of any color.\")",
            &[
                CostDef::Mana(mana_cost!("{2}{R}")),
                CostDef::SacrificeSource,
            ],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ]),
);

// OTJ 141 — Resilient Roadrunner
pub(in crate::card::sets) static RESILIENT_ROADRUNNER: CardRecord = CardRecord::new(
    "Resilient Roadrunner",
    "e07d3ee9-d3c4-4f07-839e-ec81c2587ae0",
    "David Auden Nash",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::keyword(
            "Protection from Coyotes",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                "Coyote",
            ))),
        ),
        AbilityDef::activated(
            "{3}: This creature can't be blocked this turn except by \
             creatures with haste.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::prohibit(
                        BlockRestrictionSubjectDef::Blocker,
                        BlockRestrictionMatchDef::Matching(ObjectPredicateDef::Not(
                            &ObjectPredicateDef::HasKeyword(KeywordAbility::Haste),
                        )),
                    ),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// OTJ 142 — Return the Favor
pub(in crate::card::sets) static RETURN_THE_FAVOR: CardRecord = CardRecord::new(
    "Return the Favor",
    "a9cc02d1-799d-42aa-9bc2-4c05452b63b4",
    "Eli Minaya",
CardRules::new_instant(mana_cost!("{R}{R}")).with_ability(spree(&[(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell_with_targets(
                "Copy target instant spell, sorcery spell, activated ability, or triggered ability. You may choose new targets for the copy.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                        ]),
                        ObjectPredicateDef::Ability,
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                })],
                EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            )),
(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell_with_targets(
                "Change the target of target spell or ability with a single target.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::DeclaredTargetCount {
                        minimum: 1,
                        maximum: 1,
                    },
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                })],
                EffectDef::ChangeStackTargets(&ChangeStackTargetsDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    chooser: PlayerRefDef::EffectController,
                    change: StackTargetChangeDef::ChooseNew {
                        optional: false,
                        restriction: None,
                    },
                }),
            ))])),
);

// OTJ 143 — Rodeo Pyromancers
pub(in crate::card::sets) static RODEO_PYROMANCERS: CardRecord = CardRecord::new(
    "Rodeo Pyromancers",
    "df877a29-06e1-474d-8600-410bbec674ae",
    "Kim Sokol",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Mercenary"], 3, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast your first spell each turn, add {R}{R}.",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 1,
                },
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(2)),
        ),
    ]),
);

// OTJ 144 — Scalestorm Summoner
pub(in crate::card::sets) static SCALESTORM_SUMMONER: CardRecord = CardRecord::new(
    "Scalestorm Summoner",
    "d603c00f-048a-4a05-9df9-52844819d523",
    "Xabi Gaztelua",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warlock"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, create a 3/1 red Dinosaur \
             creature token if you control a creature with power 4 or \
             greater.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
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
                },
                then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Dinosaur"], &[ManaColor::Red], 3, 1),
                ))),
            },
        ),
    ]),
);

// OTJ 145 — Scorching Shot
pub(in crate::card::sets) static SCORCHING_SHOT: CardRecord = CardRecord::new(
    "Scorching Shot",
    "f93d8357-83bd-4157-a389-68e4aa4985c8",
    "Caio Monteiro",
    CardRules::new_sorcery(mana_cost!("{R}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Scorching Shot deals 5 damage to target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(5),
        ),
    )]),
);

// OTJ 146 — Slickshot Show-Off (alternate printing)
const SLICKSHOT_SHOW_OFF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLICKSHOT_SHOW_OFF,
    1,
    "7054012b-4f9d-44a0-aaf9-7fd3bddc7b2d",
    "Augusto Quirino",
);

// OTJ 147 — Stingerback Terror
pub(in crate::card::sets) static STINGERBACK_TERROR: CardRecord = CardRecord::new(
    "Stingerback Terror",
    "d84d6e52-5c35-47bc-b160-876a3b0fcbe1",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Scorpion", "Dragon"], 7, 7).with_abilities(
        &[
            abilities::flying(),
            abilities::trample(),
            AbilityDef::static_ability(
                "This creature gets -1/-1 for each card in your hand.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerRelation::You,
                            )),
                            factor: -1,
                        }),
                        ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerRelation::You,
                            )),
                            factor: -1,
                        }),
                    ),
                },
            ),
            abilities::plot(&[CostDef::Mana(mana_cost!("{2}{R}"))]),
        ],
    ),
);

// OTJ 148 — Take for a Ride
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static TAKE_FOR_A_RIDE: CardRecord = CardRecord::new(
    "Take for a Ride",
    "c8e2ff9c-0e98-46f9-a33c-739388c5f3d0",
    "Artur Treffner",
    CardRules::unsupported(),
);

// OTJ 149 — Terror of the Peaks (reprint)
const TERROR_OF_THE_PEAKS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::TERROR_OF_THE_PEAKS,
    "904ff94a-4db4-44a6-8593-89c32905b3fc",
    "Joshua Raphael",
);

// OTJ 150 — Thunder Salvo
pub(in crate::card::sets) static THUNDER_SALVO: CardRecord = CardRecord::new(
    "Thunder Salvo",
    "a0bf0ea9-0929-4d33-815a-6df29c399e7e",
    "Johann Bodin",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Thunder Salvo deals X damage to target creature, where X is 2 \
         plus the number of other spells you've cast this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Sum(&SumValueDef::new(
                ValueDef::Constant(2),
                ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                    player: PlayerRelation::You,
                    spell: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                }),
            )),
        ),
    )]),
);

// OTJ 151 — Trick Shot
pub(in crate::card::sets) static TRICK_SHOT: CardRecord = CardRecord::new(
    "Trick Shot",
    "cd3c2d02-67ca-4858-9b7a-3cfe8a08356c",
    "Brian Valeza",
    CardRules::new_instant(mana_cost!("{4}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Trick Shot deals 6 damage to target creature and 2 damage to \
         up to one other target creature token.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Token,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )
            .another(),
        ],
        EffectDef::damage_simultaneously(&[
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(6),
            ),
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::Constant(2),
            ),
        ]),
    )]),
);

// OTJ 152 — Aloe Alchemist
// Audit: unsupported — Needs a committed plotting event dispatched to triggered abilities; the current special action or saddle effect changes state without publishing that event.
pub(in crate::card::sets) static ALOE_ALCHEMIST: CardRecord = CardRecord::new(
    "Aloe Alchemist",
    "69f2f632-b6cc-4092-acd5-a6b152e90488",
    "Borja Pindado",
    CardRules::unsupported(),
);

// OTJ 153 — Ankle Biter
pub(in crate::card::sets) static ANKLE_BITER: CardRecord = CardRecord::new(
    "Ankle Biter",
    "424972d6-3b2c-449b-b786-749a77020fa1",
    "Monztre",
    CardRules::new_creature(mana_cost!("{G}"), &["Snake"], 1, 1)
        .with_abilities(&[abilities::deathtouch()]),
);

// OTJ 154 — Beastbond Outcaster
pub(in crate::card::sets) static BEASTBOND_OUTCASTER: CardRecord = CardRecord::new(
    "Beastbond Outcaster",
    "073b9ae8-8ac3-4824-aec4-84a80531aa23",
    "Viko Menezes",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 3, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if you control a creature with \
             power 4 or greater, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
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
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{G}"))]),
    ]),
);

// OTJ 155 — Betrayal at the Vault
pub(in crate::card::sets) static BETRAYAL_AT_THE_VAULT: CardRecord = CardRecord::new(
    "Betrayal at the Vault",
    "c494820f-607d-4ed2-8a86-a916ae390272",
    "Andreas Zafiratos",
    CardRules::new_instant(mana_cost!("{4}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control deals damage equal to its power \
             to each of two other target creatures.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef {
                    minimum: 2,
                    maximum: 2,
                    another: true,
                    ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    ))
                },
            ],
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ),
    ]),
);

// OTJ 156 — Bristlepack Sentry
pub(in crate::card::sets) static BRISTLEPACK_SENTRY: CardRecord = CardRecord::new(
    "Bristlepack Sentry",
    "6aea6702-16a8-4073-9c83-fbea20a5fd32",
    "Josiah \"Jo\" Cameron",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Wolf"], 3, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "As long as you control a creature with power 4 or greater, \
             this creature can attack as though it didn't have defender.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
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
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                },
            },
        ),
    ]),
);

// OTJ 157 — Bristly Bill, Spine Sower
pub(in crate::card::sets) static BRISTLY_BILL_SPINE_SOWER: CardRecord =
    CardRecord::new(
    "Bristly Bill, Spine Sower",
    "52eef0d6-24b7-40b7-8403-e8e863d0cd55",
    "Daniel Zrom",
// The counters accumulate for free off lands, and then the activation
        // turns a slow board into a lethal one in a single turn.
        CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Druid"], 2, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::triggered_with_targets(
                    "Landfall — Whenever a land you control enters, put a +1/+1 counter on target creature.",
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
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
                // Each creature doubles its own, so a board of one-counter creatures
                // gains one apiece and a single large one gains everything it has.
                AbilityDef::activated(
                    "{3}{G}{G}: Double the number of +1/+1 counters on each creature you control.",
                    &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
                    EffectDef::DoubleCounters {
                        object: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ),
            ]),
);

// OTJ 158 — Cactarantula
pub(in crate::card::sets) static CACTARANTULA: CardRecord = CardRecord::new(
    "Cactarantula",
    "2e0e27f9-dc2c-4366-b810-3e8d0bdff8c3",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Plant", "Spider"], 6, 5).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast if you control a Desert.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(1),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever this creature becomes the target of a spell or \
             ability an opponent controls, you may draw a card.",
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::ControlledBy(
                PlayerRelation::Opponent,
            )),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ),
    ]),
);

// OTJ 159 — Colossal Rattlewurm
// Audit: unsupported — Needs a conditional flash ability evaluated on the card outside the battlefield; the casting path reads intrinsic flash or permissions from battlefield/resolving sources, not conditional abilities on the card in hand.
pub(in crate::card::sets) static COLOSSAL_RATTLEWURM: CardRecord = CardRecord::new(
    "Colossal Rattlewurm",
    "17a104d3-e4ac-44a0-9c6a-39965b1b9751",
    "Filip Burburan",
    CardRules::unsupported(),
);

// OTJ 160 — Dance of the Tumbleweeds
static DANCE_LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);
pub(in crate::card::sets) static DANCE_OF_THE_TUMBLEWEEDS: CardRecord = CardRecord::new(
    "Dance of the Tumbleweeds",
    "caf0e715-befb-4904-82e6-d3f8c7fbd454",
    "Dan Murayama Scott",
CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(spree(&[(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell(
                "Search your library for a basic land card or a Desert card, put it onto the battlefield, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        ]),
                    ]),
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
            )),
(&[CostDef::Mana(mana_cost!("{3}"))], AbilityDef::spell(
                "Create an X/X green Elemental creature token, where X is the number of lands you control.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature_with_stats(
                        &["Elemental"],
                        &[ManaColor::Green],
                        &TokenStatsDef {
                            power: ValueDef::CountMatchingObjects(&DANCE_LANDS_YOU_CONTROL),
                            toughness: ValueDef::CountMatchingObjects(&DANCE_LANDS_YOU_CONTROL),
                        },
                    ),
                ))),
            ),
        ),
    ])),
);

// OTJ 161 — Drover Grizzly
pub(in crate::card::sets) static DROVER_GRIZZLY: CardRecord = CardRecord::new(
    "Drover Grizzly",
    "560062cd-34f8-4d30-9e25-099b03961724",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear", "Mount"], 4, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, creatures you \
             control gain trample until end of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 1 }],
            "Saddle 1 (Tap any number of other creatures you control with \
             total power 1 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// OTJ 162 — Freestrider Commando
// Audit: unsupported — Needs a prospective entry condition distinguishing no mana spent to cast from other alternative payments, combined with the not-cast case; current entry conditions do not expose the mana-spent total.
pub(in crate::card::sets) static FREESTRIDER_COMMANDO: CardRecord = CardRecord::new(
    "Freestrider Commando",
    "92762169-095e-46e2-82f6-5b2ff2232240",
    "Adrián Rodríguez Pérez",
    CardRules::unsupported(),
);

// OTJ 163 — Freestrider Lookout
pub(in crate::card::sets) static FREESTRIDER_LOOKOUT: CardRecord = CardRecord::new(
    "Freestrider Lookout",
    "32370f05-52a2-405f-b2bb-1b8a9b0b69f8",
    "Matt Zeilinger",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Rogue"], 3, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever you commit a crime, look at the top five cards of \
             your library. You may put a land card from among them onto \
             the battlefield tapped. Put the rest on the bottom of your \
             library in a random order. This ability triggers only once \
             each turn. (Targeting opponents, anything they control, \
             and/or cards in their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::WithBattlefieldArrival {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        arrival: BattlefieldArrivalDef {
                            modifications: &[BattlefieldEntryModificationDef::Tapped],
                            ..BattlefieldArrivalDef::DEFAULT
                        },
                    },
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
        )
        .triggering_at_most(1),
    ]),
);

// OTJ 164 — Full Steam Ahead
pub(in crate::card::sets) static FULL_STEAM_AHEAD: CardRecord = CardRecord::new(
    "Full Steam Ahead",
    "084748d8-7169-4e86-a69c-631c6d7d3a1e",
    "Inkognit",
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_abilities(&[AbilityDef::spell(
        "Until end of turn, each creature you control gets +2/+2 and \
         gains trample and \"This creature can't be blocked by more \
         than one creature.\"",
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
                AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::MaximumBlockers(1),
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// OTJ 165 — Giant Beaver
// Audit: unsupported — Needs the identities of creatures used to saddle this Mount retained for the rest of the turn; current saddle stores only a saddled boolean.
pub(in crate::card::sets) static GIANT_BEAVER: CardRecord = CardRecord::new(
    "Giant Beaver",
    "919826a9-c427-42c6-8885-a87f0b6d2192",
    "Lars Grant-West",
    CardRules::unsupported(),
);

// OTJ 166 — Gold Rush
pub(in crate::card::sets) static GOLD_RUSH: CardRecord = CardRecord::new(
    "Gold Rush",
    "15845be1-919d-4450-9de6-3552c52e8623",
    "Eric Wilkerson",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Create a Treasure token. Until end of turn, up to one target \
         creature gets +2/+2 for each Treasure you control.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        factor: 2,
                    }),
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        factor: 2,
                    }),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// OTJ 167 — Goldvein Hydra
pub(in crate::card::sets) static GOLDVEIN_HYDRA: CardRecord = CardRecord::new(
    "Goldvein Hydra",
    "de0e01e4-e143-47fd-8565-7b48219bb546",
    "David Auden Nash",
    CardRules::new_creature(mana_cost!("{X}{G}"), &["Hydra"], 0, 0).with_abilities(&[
        abilities::vigilance(),
        abilities::trample(),
        abilities::haste(),
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        abilities::dies_trigger(
            "When this creature dies, create a number of tapped Treasure \
             tokens equal to its power.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::SourcePower)
                    .entering_tapped(),
            ),
        ),
    ]),
);

// OTJ 168 — Hardbristle Bandit
pub(in crate::card::sets) static HARDBRISTLE_BANDIT: CardRecord = CardRecord::new(
    "Hardbristle Bandit",
    "cbe544fb-93b7-4640-b886-cb0b3e437357",
    "Francis Tneh",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Rogue"], 1, 1).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::triggered(
            "Whenever you commit a crime, untap this creature. This \
             ability triggers only once each turn. (Targeting opponents, \
             anything they control, and/or cards in their graveyards is a \
             crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        )
        .triggering_at_most(1),
    ]),
);

// OTJ 169 — Intrepid Stablemaster
pub(in crate::card::sets) static INTREPID_STABLEMASTER: CardRecord = CardRecord::new(
    "Intrepid Stablemaster",
    "4d6cdf2a-026a-41ba-87d9-8fcd8a67f06e",
    "Svetlin Velinov",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Scout"], 2, 2).with_abilities(&[
        abilities::reach(),
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated_mana(
            "{T}: Add two mana of any one color. Spend this mana only to \
             cast Mount or Vehicle spells.",
            &[CostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                    .with_amount(2)
                    .with_restrictions(&[ManaRestrictionDef::CastSpell(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
                    )]),
            ),
        ),
    ]),
);

// OTJ 170 — Map the Frontier
pub(in crate::card::sets) static MAP_THE_FRONTIER: CardRecord = CardRecord::new(
    "Map the Frontier",
    "165f4428-e1a1-477d-bd90-138189e88163",
    "Darrell Riche",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[AbilityDef::spell(
        "Search your library for up to two basic land cards and/or \
         Desert cards, put them onto the battlefield tapped, then \
         shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
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
    )]),
);

// OTJ 171 — Ornery Tumblewagg
pub(in crate::card::sets) static ORNERY_TUMBLEWAGG: CardRecord = CardRecord::new(
    "Ornery Tumblewagg",
    "0020c31b-002a-4121-bc61-2c2c16e9afc8",
    "Izzy",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Brushwagg", "Mount"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, put a +1/+1 counter \
             on target creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks while saddled, double the \
             number of +1/+1 counters on target creature.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DoubleCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
            },
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 2 }],
            "Saddle 2 (Tap any number of other creatures you control with \
             total power 2 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// OTJ 172 — Outcaster Greenblade
pub(in crate::card::sets) static OUTCASTER_GREENBLADE: CardRecord = CardRecord::new(
    "Outcaster Greenblade",
    "c9458f0f-5593-4ac9-934c-e215ef8093a7",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Mercenary"], 1, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, search your library for a basic \
             land card or a Desert card, reveal it, put it into your hand, \
             then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
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
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each Desert you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            },
        ),
    ]),
);

// OTJ 173 — Outcaster Trailblazer
pub(in crate::card::sets) static OUTCASTER_TRAILBLAZER: CardRecord = CardRecord::new(
    "Outcaster Trailblazer",
    "33b9cd6c-d75c-4905-aa38-ff03a9c4b398",
    "Denys Tsiperko",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 4, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, add one mana of any color.",
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control with power 4 or greater \
             enters, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{G}"))]),
    ]),
);

// OTJ 174 — Patient Naturalist
pub(in crate::card::sets) static PATIENT_NATURALIST: CardRecord = CardRecord::new(
    "Patient Naturalist",
    "1dd17cea-9e8c-4dba-b6ab-a6b9de87a306",
    "Inka Schulz",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Scout"], 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill three cards. Put a land card \
             from among the milled cards into your hand. If you can't, \
             create a Treasure token. (To mill three cards, put the top \
             three cards of your library into your graveyard.)",
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
                        minimum: 1,
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
                                then: &EffectDef::CreateToken(
                                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                        .with_count(ValueDef::Constant(1)),
                                ),
                            },
                        },
                    }),
                    otherwise: &EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(1)),
                    ),
                },
            ]),
        ),
    ]),
);

// OTJ 175 — Railway Brawler
pub(in crate::card::sets) static RAILWAY_BRAWLER: CardRecord = CardRecord::new(
    "Railway Brawler",
    "9ec1f76f-f21d-4f06-8c02-be6745183348",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Rhino", "Warrior"], 5, 5).with_abilities(
        &[
            abilities::reach(),
            abilities::trample(),
            AbilityDef::triggered(
                "Whenever another creature you control enters, put X +1/+1 \
                 counters on it, where X is its power.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::TriggeringObject,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::One(ObjectRefDef::TriggeringObject),
                        select: ObjectValueDef::Power,
                        operation: AggregateOperationDef::Sum,
                    }),
                },
            ),
            abilities::plot(&[CostDef::Mana(mana_cost!("{3}{G}"))]),
        ],
    ),
);

// OTJ 176 — Rambling Possum
// Audit: unsupported — Needs the identities of creatures used to saddle this Mount retained for the rest of the turn; current saddle stores only a saddled boolean.
pub(in crate::card::sets) static RAMBLING_POSSUM: CardRecord = CardRecord::new(
    "Rambling Possum",
    "19d1e75f-0fee-4e07-9420-df771b696e85",
    "Adrián Rodríguez Pérez",
    CardRules::unsupported(),
);

// OTJ 177 — Raucous Entertainer
pub(in crate::card::sets) static RAUCOUS_ENTERTAINER: CardRecord = CardRecord::new(
    "Raucous Entertainer",
    "8dc3cbfe-410f-40e3-8021-647a2efb50bf",
    "Forrest Imel",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Bard"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}: Put a +1/+1 counter on each creature you control \
             that entered this turn.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::EnteredThisTurn,
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

// OTJ 178 — Reach for the Sky
pub(in crate::card::sets) static REACH_FOR_THE_SKY: CardRecord = CardRecord::new(
    "Reach for the Sky",
    "eb871985-a11b-4dfe-b0e3-898888c86277",
    "Villarrte",
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+2 and has reach.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                    ]),
                },
            ),
            abilities::dies_trigger(
                "When this Aura is put into a graveyard from the battlefield, \
                 draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// OTJ 179 — Rise of the Varmints
pub(in crate::card::sets) static RISE_OF_THE_VARMINTS: CardRecord = CardRecord::new(
    "Rise of the Varmints",
    "f4e879d8-a058-48e8-9733-f58b1e0da4b9",
    "Ralph Horsley",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create X 2/1 green Varmint creature tokens, where X is the \
             number of creature cards in your graveyard.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Varmint"],
                    &[ManaColor::Green],
                    2,
                    1,
                )))
                .with_count(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                )),
            ),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{G}"))]),
    ]),
);

// OTJ 180 — Smuggler's Surprise
pub(in crate::card::sets) static SMUGGLER_S_SURPRISE: CardRecord = CardRecord::new(
    "Smuggler's Surprise",
    "e7fbb489-e2b5-4278-8162-86802cf124d8",
    "Jonas De Ro",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell(
                "Mill four cards. You may put up to two creature and/or land \
                 cards from among the milled cards into your hand.",
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(4),
                        },
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Land),
                            ])),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 2,
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
        ),
        (
            &[CostDef::Mana(mana_cost!("{4}{G}"))],
            AbilityDef::spell(
                "You may put up to two creature cards from your hand onto the \
                 battlefield.",
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
                    maximum: 2,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                }),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell(
                "Creatures you control with power 4 or greater gain hexproof \
                 and indestructible until end of turn.",
                EffectDef::Apply {
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
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
    ])]),
);

// OTJ 181 — Snakeskin Veil (reprint)
const SNAKESKIN_VEIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::SNAKESKIN_VEIL,
    "133fbdec-0d00-433f-9015-5eb091126e3a",
    "Dan Murayama Scott",
);

// OTJ 182 — Spinewoods Armadillo
pub(in crate::card::sets) static SPINEWOODS_ARMADILLO: CardRecord = CardRecord::new(
    "Spinewoods Armadillo",
    "f79d63e7-a8c6-4750-91c9-c575a4d0561b",
    "Iris Compiet",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Armadillo"], 7, 7).with_abilities(&[
        abilities::reach(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{3}"))], "Ward {3}"),
        AbilityDef::activated(
            "{1}{G}, Discard this card: Search your library for a basic \
             land card or a Desert card, reveal it, put it into your hand, \
             then shuffle. You gain 3 life.",
            &[CostDef::Mana(mana_cost!("{1}{G}")), CostDef::DiscardSource],
            EffectDef::Sequence(&[
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
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
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ]),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// OTJ 183 — Spinewoods Paladin
pub(in crate::card::sets) static SPINEWOODS_PALADIN: CardRecord = CardRecord::new(
    "Spinewoods Paladin",
    "1b2b432d-9e73-4ab2-a098-546d406df6c0",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human", "Knight"], 5, 4).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, you gain 3 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{3}{G}"))]),
    ]),
);

// OTJ 184 — Stubborn Burrowfiend
// Audit: unsupported — Needs a committed saddling event dispatched to triggered abilities; the current special action or saddle effect changes state without publishing that event.
pub(in crate::card::sets) static STUBBORN_BURROWFIEND: CardRecord = CardRecord::new(
    "Stubborn Burrowfiend",
    "6d963eb4-d20b-4d3f-bf5d-c75f7bcb9670",
    "Ângelo Bortolini",
    CardRules::unsupported(),
);

// OTJ 185 — Throw from the Saddle
pub(in crate::card::sets) static THROW_FROM_THE_SADDLE: CardRecord = CardRecord::new(
    "Throw from the Saddle",
    "775874cc-4b78-4904-9c97-431c2e400c64",
    "Eilene Cherie",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 until end of turn. Put \
         a +1/+1 counter on it instead if it's a Mount. Then it deals \
         damage equal to its power to target creature you don't \
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
                controller: Some(PlayerRelation::NotYou),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                },
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )]),
);

// OTJ 186 — Trash the Town
pub(in crate::card::sets) static TRASH_THE_TOWN: CardRecord = CardRecord::new(
    "Trash the Town",
    "eda59f99-1d6d-4051-ac89-b7cbfa19262e",
    "David Auden Nash",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Put two +1/+1 counters on target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Target creature gains trample until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Until end of turn, target creature gains \"Whenever this \
                 creature deals combat damage to a player, draw two cards.\"",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever this creature deals combat damage to a player, draw \
                         two cards.",
                        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                        abilities::draw_cards(ValueDef::Constant(2)),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
    ])]),
);

// OTJ 187 — Tumbleweed Rising
pub(in crate::card::sets) static TUMBLEWEED_RISING: CardRecord = CardRecord::new(
    "Tumbleweed Rising",
    "275d2d2a-ef85-48c9-919d-bc62cdad8a10",
    "Jason Smith",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create an X/X green Elemental creature token, where X is the \
             greatest power among creatures you control.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature_with_stats(
                    &["Elemental"],
                    &[ManaColor::Green],
                    &TokenStatsDef {
                        power: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Maximum,
                        }),
                        toughness: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Maximum,
                        }),
                    },
                ),
            ))),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{2}{G}"))]),
    ]),
);

// OTJ 188 — Voracious Varmint
pub(in crate::card::sets) static VORACIOUS_VARMINT: CardRecord = CardRecord::new(
    "Voracious Varmint",
    "99b74fa3-c1d7-4780-977d-f2d6663a529a",
    "Adrián Rodríguez Pérez",
    // Maindeckable artifact and enchantment removal that is a body until it
    // is needed, which is what vigilance is doing on a two-drop.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Varmint"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or enchantment.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
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
    ]),
);

// OTJ 189 — Akul the Unrepentant
pub(in crate::card::sets) static AKUL_THE_UNREPENTANT: CardRecord = CardRecord::new(
    "Akul the Unrepentant",
    "68fd8548-50db-4243-9154-377f32408d58",
    "Kekai Kotaki",
    CardRules::new_creature(
        mana_cost!("{B}{B}{R}{R}"),
        &["Scorpion", "Dragon", "Rogue"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::trample(),
        AbilityDef::activated(
            "Sacrifice three other creatures: You may put a creature card \
             from your hand onto the battlefield. Activate only as a \
             sorcery and only once each turn.",
            &[CostDef::sacrifice_permanents(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                PlayerRelation::You,
                3,
            )],
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
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("chosen"))),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            }),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .once_each_turn(),
    ]),
);

// OTJ 190 — Annie Flash, the Veteran
pub(in crate::card::sets) static ANNIE_FLASH_THE_VETERAN: CardRecord = CardRecord::new(
    "Annie Flash, the Veteran",
    "8d4af7c3-a70d-4f71-b27d-b268c4a0f81e",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{3}{R}{G}{W}"), &["Human", "Rogue"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::triggered_if_with_targets(
                "When Annie Flash enters, if you cast it, return target \
                 permanent card with mana value 3 or less from your graveyard \
                 to the battlefield tapped.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::SourceWasCast,
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(3),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::WithBattlefieldArrival {
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
            ),
            AbilityDef::triggered(
                "Whenever Annie Flash becomes tapped, exile the top two cards \
                 of your library. You may play those cards this turn.",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(2),
                    },
                    binding: crate::Binding!("top"),
                    then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("top"),
                        )),
                    },
                }),
            ),
        ]),
);

// OTJ 191 — Annie Joins Up
// Audit: unsupported — Needs additional occurrences of arbitrary legendary-creature triggered abilities; current trigger doubling only supports enters triggers.
pub(in crate::card::sets) static ANNIE_JOINS_UP: CardRecord = CardRecord::new(
    "Annie Joins Up",
    "1624a5f4-f5bc-47c9-85de-c5520ee234ce",
    "Wylie Beckert",
    CardRules::unsupported(),
);

// OTJ 192 — Assimilation Aegis
// Audit: unsupported — Needs exile-until-source-leaves with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger would return the card later through the stack.
pub(in crate::card::sets) static ASSIMILATION_AEGIS: CardRecord = CardRecord::new(
    "Assimilation Aegis",
    "014bf3c6-e46f-48f8-902f-82deeba260b2",
    "Matt Stewart",
    CardRules::unsupported(),
);

// OTJ 193 — At Knifepoint
pub(in crate::card::sets) static AT_KNIFEPOINT: CardRecord = CardRecord::new(
    "At Knifepoint",
    "897d594d-b5b0-43dc-b877-b483942416ce",
    "Francisco Miyara",
    CardRules::new_enchantment(mana_cost!("{1}{B}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "During your turn, outlaws you control have first strike. \
             (Assassins, Mercenaries, Pirates, Rogues, and Warlocks are \
             outlaws.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever you commit a crime, create a 1/1 red Mercenary \
             creature token with \"{T}: Target creature you control gets \
             +1/+0 until end of turn. Activate only as a sorcery.\" This \
             ability triggers only once each turn.",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN))),
        )
        .triggering_at_most(1),
    ]),
);

// OTJ 194 — Badlands Revival
pub(in crate::card::sets) static BADLANDS_REVIVAL: CardRecord = CardRecord::new(
    "Badlands Revival",
    "8d3ef971-cdd4-410c-97c3-df98e4f02ab2",
    "Carlos Palma Cruchaga",
    CardRules::new_sorcery(mana_cost!("{3}{B}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return up to one target creature card from your graveyard to \
             the battlefield. Return up to one target permanent card from \
             your graveyard to your hand.",
            &[
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                    1,
                ),
                AbilityTargetDef::up_to(
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
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex(1)),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ]),
        ),
    ]),
);

// OTJ 195 — Baron Bertram Graywater
// Audit: unsupported — Needs a batched token-entry event, including token permanent spells resolving; TokensCreated observes token-creation instructions and is not equivalent to tokens entering.
pub(in crate::card::sets) static BARON_BERTRAM_GRAYWATER: CardRecord = CardRecord::new(
    "Baron Bertram Graywater",
    "e9da18b4-1efc-44b7-8001-a2cfd44c69bf",
    "Johan Grenier",
    CardRules::unsupported(),
);

// OTJ 196 — Bonny Pall, Clearcutter
pub(in crate::card::sets) static BONNY_PALL_CLEARCUTTER: CardRecord = CardRecord::new(
    "Bonny Pall, Clearcutter",
    "4383ae7c-58ea-4354-93e4-677ad185c3bb",
    "Bryan Sola",
    CardRules::new_creature(mana_cost!("{3}{G}{U}{U}"), &["Giant", "Scout"], 6, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            abilities::enters_trigger(
                "When Bonny Pall enters, create Beau, a legendary blue Ox \
                 creature token with \"Beau's power and toughness are each \
                 equal to the number of lands you control.\"",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Ox"], &[ManaColor::Blue], 0, 0)
                        .with_name("Beau")
                        .with_supertype(CardSupertype::Legendary)
                        .with_abilities(&[AbilityDef::static_ability(
                            "Beau's power and toughness are each equal to the number of \
                     lands you control.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::set_base_power_toughness(
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
                        )]),
                ))),
            ),
            AbilityDef::triggered(
                "Whenever you attack, draw a card, then you may put a land \
                 card from your hand or graveyard onto the battlefield.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Hand, ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            ),
        ]),
);

// OTJ 197 — Breeches, the Blastmaker
// Audit: unsupported — Needs reflexive coin-result triggers tied to the resolving optional sacrifice and the particular second spell, with damage targets chosen only after a losing flip.
pub(in crate::card::sets) static BREECHES_THE_BLASTMAKER: CardRecord = CardRecord::new(
    "Breeches, the Blastmaker",
    "cf3bda9e-42af-4f99-a504-c96c25c2794b",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// OTJ 198 — Bruse Tarl, Roving Rancher
// Audit: unsupported — Needs exile-play permission expiring at cleanup of its controller's next turn; the current turn-count duration incorrectly remains usable during the following opponent turn.
pub(in crate::card::sets) static BRUSE_TARL_ROVING_RANCHER: CardRecord = CardRecord::new(
    "Bruse Tarl, Roving Rancher",
    "286c55c2-dcc1-4e87-a83f-9981d28ab62d",
    "Forrest Imel",
    CardRules::unsupported(),
);

// OTJ 199 — Cactusfolk Sureshot
pub(in crate::card::sets) static CACTUSFOLK_SURESHOT: CardRecord = CardRecord::new(
    "Cactusfolk Sureshot",
    "318b8c5d-9fb0-488f-9b32-c2e29d1f1dbb",
    "Artur Nakhodkin",
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Plant", "Mercenary"], 4, 4).with_abilities(
        &[
            abilities::reach(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
            AbilityDef::triggered(
                "At the beginning of combat on your turn, other creatures you \
                 control with power 4 or greater gain trample and haste until \
                 end of turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::PowerAtLeast(4),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    ),
);

// OTJ 200 — Congregation Gryff
pub(in crate::card::sets) static CONGREGATION_GRYFF: CardRecord = CardRecord::new(
    "Congregation Gryff",
    "0ef4907a-2fc1-42c5-bffc-3b3f93601fb9",
    "Joseph Meehan",
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Hippogriff", "Mount"], 1, 4)
        .with_abilities(&[
            abilities::flying(),
            abilities::lifelink(),
            AbilityDef::triggered(
                "Whenever this creature attacks while saddled, it gets +X/+X \
                 until end of turn, where X is the number of Mounts you \
                 control.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Saddled,
                    },
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 3 }],
                "Saddle 3 (Tap any number of other creatures you control with \
                 total power 3 or more: This Mount becomes saddled until end \
                 of turn. Saddle only as a sorcery.)",
            ),
        ]),
);

// OTJ 201 — Doc Aurlock, Grizzled Genius
// Audit: unsupported — Needs discounts for plotting special actions and spell-cost filtering by the casting-origin zone; existing cost adjustments price spells and abilities without those filters.
pub(in crate::card::sets) static DOC_AURLOCK_GRIZZLED_GENIUS: CardRecord = CardRecord::new(
    "Doc Aurlock, Grizzled Genius",
    "6fc27b30-8c8e-434c-a72c-e1d409efc1ae",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// OTJ 202 — Eriette, the Beguiler
// Audit: unsupported — Needs attachment-change events carrying both Aura and host, a comparison of their mana values, and control lasting only while that particular attachment remains.
pub(in crate::card::sets) static ERIETTE_THE_BEGUILER: CardRecord = CardRecord::new(
    "Eriette, the Beguiler",
    "f46c133a-7ae4-431b-88f2-ec606a7baf69",
    "Chris Rallis",
    CardRules::unsupported(),
);

// OTJ 203 — Ertha Jo, Frontier Mentor
// Audit: unsupported — Needs an ability-activation event that matches its declared creature/player targets and exposes the activated stack object for copying; the existing stack-event vocabulary has no activation event.
pub(in crate::card::sets) static ERTHA_JO_FRONTIER_MENTOR: CardRecord = CardRecord::new(
    "Ertha Jo, Frontier Mentor",
    "a4e81be6-6447-4f1e-be00-6fcdb2ab35af",
    "Michal Ivan",
    CardRules::unsupported(),
);

// OTJ 204 — Form a Posse
pub(in crate::card::sets) static FORM_A_POSSE: CardRecord = CardRecord::new(
    "Form a Posse",
    "39ee1387-c24e-4a66-8ad6-9afa9c0abcbb",
    "J.Lonnee",
    CardRules::new_sorcery(mana_cost!("{X}{R}{W}")).with_abilities(&[AbilityDef::spell(
        "Create X 1/1 red Mercenary creature tokens with \"{T}: Target \
         creature you control gets +1/+0 until end of turn. Activate \
         only as a sorcery.\"",
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(MERCENARY_TOKEN)).with_count(ValueDef::ChosenX),
        ),
    )]),
);

// OTJ 205 — Ghired, Mirror of the Wilds
pub(in crate::card::sets) static GHIRED_MIRROR_OF_THE_WILDS: CardRecord = CardRecord::new(
    "Ghired, Mirror of the Wilds",
    "e43e3d71-4fb8-4ab1-8c8f-b65ae3ad4cc4",
    "Diego Gisbert",
    CardRules::new_creature(mana_cost!("{R}{G}{W}"), &["Human", "Shaman"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::static_ability(
                "Nontoken creatures you control have \"{T}: Create a token \
                 that's a copy of target token you control that entered this \
                 turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}: Create a token that is a copy of target token you \
                         control that entered this turn.",
                        &[CostDef::TapSource],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::Token,
                                    ObjectPredicateDef::EnteredThisTurn,
                                ]),
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )],
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                            &TokenCopyDef {
                                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                exceptions: CopyExceptionsDef::NONE,
                            },
                        ))),
                    )),
                },
            ),
        ]),
);

// OTJ 206 — The Gitrog, Ravenous Ride
// Audit: unsupported — Needs the identities of creatures used to saddle this Mount retained for the rest of the turn; current saddle stores only a saddled boolean.
pub(in crate::card::sets) static THE_GITROG_RAVENOUS_RIDE: CardRecord = CardRecord::new(
    "The Gitrog, Ravenous Ride",
    "82512813-8618-483b-a7f0-e6a611d9d487",
    "Johan Grenier",
    CardRules::unsupported(),
);

// OTJ 207 — Honest Rutstein
pub(in crate::card::sets) static HONEST_RUTSTEIN: CardRecord = CardRecord::new(
    "Honest Rutstein",
    "259ddf66-76af-4857-83c3-c812327a6e23",
    "Javier Charro",
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Human", "Warlock"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When Honest Rutstein enters, return target creature card from \
                 your graveyard to your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
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
            AbilityDef::static_ability(
                "Creature spells you cast cost {1} less to cast.",
                EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
                    spell: ObjectPredicateDef::HasType(CardType::Creature),
                    caster: PlayerRelation::You,
                    condition: SpellCostConditionDef::Always,
                    adjustment: CostAdjustmentDef::Subtract(CostAmountDef::Generic(
                        ValueDef::Constant(1),
                    )),
                })),
            ),
        ]),
);

// OTJ 208 — Intimidation Campaign
pub(in crate::card::sets) static INTIMIDATION_CAMPAIGN: CardRecord = CardRecord::new(
    "Intimidation Campaign",
    "596fb7a2-bb79-44b7-ad84-414c8139ec13",
    "Svetlin Velinov",
    CardRules::new_enchantment(mana_cost!("{1}{U}{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, each opponent loses 1 life, you \
             gain 1 life, and you draw a card.",
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
        AbilityDef::triggered(
            "Whenever you commit a crime, you may return this enchantment \
             to its owner's hand. (It returns only from the battlefield. \
             Targeting opponents, anything they control, and/or cards in \
             their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// OTJ 209 — Jem Lightfoote, Sky Explorer
// Audit: unsupported — Needs spell-cast history filtered by the zone a spell was cast from; the current history query retains caster and spell characteristics but no casting-origin zone.
pub(in crate::card::sets) static JEM_LIGHTFOOTE_SKY_EXPLORER: CardRecord = CardRecord::new(
    "Jem Lightfoote, Sky Explorer",
    "e24fe6dc-662a-4abc-ad60-a1959b2be006",
    "Darren Tan",
    CardRules::unsupported(),
);

// OTJ 210 — Jolene, Plundering Pugilist
pub(in crate::card::sets) static JOLENE_PLUNDERING_PUGILIST: CardRecord = CardRecord::new(
    "Jolene, Plundering Pugilist",
    "fe30b5c8-4889-4350-bb1d-3e2a67d9dfb2",
    "Andreas Zafiratos",
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Human", "Mercenary"], 4, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you attack with one or more creatures with power 4 \
                 or greater, create a Treasure token.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerAtLeast(4),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
            AbilityDef::activated_with_targets(
                "{1}{R}, Sacrifice a Treasure: Jolene deals 1 damage to any \
                 target.",
                &[
                    CostDef::Mana(mana_cost!("{1}{R}")),
                    CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Treasure",
                    ))),
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
        ]),
);

// OTJ 211 — Kambal, Profiteering Mayor
// Audit: unsupported — Needs a batched token-entry event retaining every entering token so each can be copied; TokensCreated does not cover token spells resolving or retain a bound entry batch.
pub(in crate::card::sets) static KAMBAL_PROFITEERING_MAYOR: CardRecord = CardRecord::new(
    "Kambal, Profiteering Mayor",
    "d53a775d-5898-41a8-b404-9b7d4721c6ba",
    "Andreas Zafiratos",
    CardRules::unsupported(),
);

// OTJ 212 — Kellan Joins Up
// Audit: unsupported — Needs an effect or external permission that makes other cards plotted, including its later-turn, sorcery-only free cast permission; existing plot only supports the card's own hand special action.
pub(in crate::card::sets) static KELLAN_JOINS_UP: CardRecord = CardRecord::new(
    "Kellan Joins Up",
    "2e7f95d5-b279-4469-9c89-1e02630d61e6",
    "Wylie Beckert",
    CardRules::unsupported(),
);

// OTJ 213 — Kellan, the Kid
// Audit: unsupported — Needs a cast event carrying the casting-origin zone, followed by an optional free permanent cast from hand with a mana-value ceiling and a declined-cast continuation.
pub(in crate::card::sets) static KELLAN_THE_KID: CardRecord = CardRecord::new(
    "Kellan, the Kid",
    "04dfbc4c-ab21-45db-bbd9-b9d245d60015",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// OTJ 214 — Kraum, Violent Cacophony
pub(in crate::card::sets) static KRAUM_VIOLENT_CACOPHONY: CardRecord = CardRecord::new(
    "Kraum, Violent Cacophony",
    "958a3e6b-7e20-40ea-8b2c-7c728934b5e5",
    "Artur Nakhodkin",
    CardRules::new_creature(mana_cost!("{2}{U}{R}"), &["Zombie", "Horror"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever you cast your second spell each turn, put a +1/+1 \
                 counter on Kraum and draw a card.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Any,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ])),
                    condition: &TriggerConditionDef::SpellsCastThisTurn {
                        quantifier: QuantifierDef::Any,
                        player: PlayerRelation::You,
                        comparison: ComparisonDef::Equal,
                        amount: 2,
                    },
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

// OTJ 215 — Laughing Jasper Flint
pub(in crate::card::sets) static LAUGHING_JASPER_FLINT: CardRecord = CardRecord::new(
    "Laughing Jasper Flint",
    "af0b3a41-ba99-41e8-bcfb-5796500c17c7",
    "Francis Tneh",
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Lizard", "Rogue"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Creatures you control but don't own are Mercenaries in \
                 addition to their other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef {
                        owner: Some(PlayerSetDef::Related(PlayerRelation::NotYou)),
                        ..ObjectQueryDef::controlled_by(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerSetDef::Related(PlayerRelation::You),
                        )
                    })),
                    effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                        "Mercenary",
                    ])),
                },
            ),
            AbilityDef::triggered_with_targets(
                "At the beginning of your upkeep, exile the top X cards of \
                 target opponent's library, where X is the number of outlaws \
                 you control. Until end of turn, you may cast spells from \
                 among those cards, and mana of any type can be spent to cast \
                 those spells.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    },
                    binding: crate::Binding!("top"),
                    then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("top"),
                        )),
                    },
                }),
            ),
        ]),
);

// OTJ 216 — Lazav, Familiar Stranger
pub(in crate::card::sets) static LAZAV_FAMILIAR_STRANGER: CardRecord = CardRecord::new(
    "Lazav, Familiar Stranger",
    "00293326-3eb2-492c-b565-7abafa037d8c",
    "Tyler Jacobson",
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Shapeshifter"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever you commit a crime, put a +1/+1 counter on Lazav. \
             Then you may exile a card from a graveyard. If a creature \
             card was exiled this way, you may have Lazav become a copy of \
             that card until end of turn. This ability triggers only once \
             each turn. (Targeting opponents, anything they control, \
             and/or cards in their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::Any,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Exile,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("exiled"),
                        then: &EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ObjectSetCount(
                                &ObjectSetCountConditionDef {
                                    objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                        crate::Binding!("exiled"),
                                    ),
                                    predicate: ObjectSetPredicateDef::contains(
                                        &ObjectPredicateDef::HasType(CardType::Creature),
                                    ),
                                },
                            ),
                            then: &EffectDef::May {
                                player: EffectRecipientDef::Controller,
                                effect: &EffectDef::BecomeCopyOf {
                                    object: EffectRecipientDef::objects(
                                        ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                            crate::Binding!("exiled"),
                                        ),
                                    ),
                                    copier: None,
                                    exceptions: CopyExceptionsDef::NONE,
                                    duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
                                },
                            },
                        },
                    },
                }),
            ]),
        )
        .triggering_at_most(1)]),
);

// OTJ 217 — Lilah, Undefeated Slickshot
// Audit: unsupported — Needs an effect or external permission that makes other cards plotted, including its later-turn, sorcery-only free cast permission; existing plot only supports the card's own hand special action.
pub(in crate::card::sets) static LILAH_UNDEFEATED_SLICKSHOT: CardRecord = CardRecord::new(
    "Lilah, Undefeated Slickshot",
    "e21f90ea-5934-4757-8515-38ef116afac1",
    "Andreas Zafiratos",
    CardRules::unsupported(),
);

// OTJ 218 — Make Your Own Luck
// Audit: unsupported — Needs an effect or external permission that makes other cards plotted, including its later-turn, sorcery-only free cast permission; existing plot only supports the card's own hand special action.
pub(in crate::card::sets) static MAKE_YOUR_OWN_LUCK: CardRecord = CardRecord::new(
    "Make Your Own Luck",
    "0557b0a3-2b48-408f-a508-9f4da2ab1cd1",
    "Chris Seaman",
    CardRules::unsupported(),
);

// OTJ 219 — Malcolm, the Eyes
pub(in crate::card::sets) static MALCOLM_THE_EYES: CardRecord = CardRecord::new(
    "Malcolm, the Eyes",
    "521dffaa-813b-41e4-b7c2-a8c407167875",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Siren", "Pirate"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever you cast your second spell each turn, investigate. \
                 (Create a Clue token. It's an artifact with \"{2}, Sacrifice \
                 this token: Draw a card.\")",
                TriggerEventDef::While {
                    event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Any,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ])),
                    condition: &TriggerConditionDef::SpellsCastThisTurn {
                        quantifier: QuantifierDef::Any,
                        player: PlayerRelation::You,
                        comparison: ComparisonDef::Equal,
                        amount: 2,
                    },
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(CLUE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
        ]),
);

// OTJ 220 — Marchesa, Dealer of Death
pub(in crate::card::sets) static MARCHESA_DEALER_OF_DEATH: CardRecord = CardRecord::new(
    "Marchesa, Dealer of Death",
    "ee29b59c-d57c-4a03-bae7-e9dfa57d6bb1",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{U}{B}{R}"), &["Human", "Rogue"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever you commit a crime, you may pay {1}. If you do, look \
             at the top two cards of your library. Put one of them into \
             your hand and the other into your graveyard. (Targeting \
             opponents, anything they control, and/or cards in their \
             graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}"))],
                &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(2),
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
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "rest"
                            ))),
                            ZoneKind::Graveyard,
                            ZonePlacement::Top,
                        ),
                    ]),
                }),
            )),
        )]),
);

// OTJ 221 — Miriam, Herd Whisperer
pub(in crate::card::sets) static MIRIAM_HERD_WHISPERER: CardRecord = CardRecord::new(
    "Miriam, Herd Whisperer",
    "bfa7750a-7c32-4413-b762-62e24d992c6b",
    "Viko Menezes",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Human", "Druid"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "During your turn, Mounts and Vehicles you control have hexproof.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    },
                },
            ),
            AbilityDef::triggered(
                "Whenever a Mount or Vehicle you control attacks, put a +1/+1 \
                 counter on it.",
                TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::TriggeringObject,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// OTJ 222 — Obeka, Splitter of Seconds
// Audit: unsupported — Needs inserting a computed number of additional upkeep steps after the current phase; extra-turn and extra-combat operations do not insert arbitrary steps.
pub(in crate::card::sets) static OBEKA_SPLITTER_OF_SECONDS: CardRecord = CardRecord::new(
    "Obeka, Splitter of Seconds",
    "03415c42-086e-4a2e-9be8-5cdcde83f134",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// OTJ 223 — Oko, the Ringleader
// Audit: unsupported — Needs a controller-relative committed-crime-this-turn fact, including crimes before this permanent entered; the engine publishes crime events but does not retain that turn history.
pub(in crate::card::sets) static OKO_THE_RINGLEADER: CardRecord = CardRecord::new(
    "Oko, the Ringleader",
    "396df8d6-e85d-4486-8116-68841b7e1e2e",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// OTJ 224 — Pillage the Bog
pub(in crate::card::sets) static PILLAGE_THE_BOG: CardRecord = CardRecord::new(
    "Pillage the Bog",
    "fa3b415f-7901-4ab4-84fe-60b90d40ac90",
    "Forrest Imel",
    // Two mana to find the one card the deck is built around, and plot is
    // what makes the two mana free: pay three on a turn with nothing to do,
    // and dig for nothing on the turn it matters.
    CardRules::new_sorcery(mana_cost!("{B}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is twice the number of lands you \
             control. Put one of them into your hand and the rest on the bottom of your library \
             in a random order.",
            abilities::look_at_top_cards_choose_to_hand_rest_random_bottom(
                // "Twice the number of lands you control", which is what makes the card a
                // land-count payoff rather than a fixed dig: six lands look at twelve.
                ValueDef::Scaled(&ScaledValueDef::new(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    2,
                )),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{B}{G}"))]),
    ]),
);

// OTJ 225 — Rakdos Joins Up
pub(in crate::card::sets) static RAKDOS_JOINS_UP: CardRecord = CardRecord::new(
    "Rakdos Joins Up",
    "c7154dca-7e10-4c34-aa56-9f200c6277d1",
    "Wylie Beckert",
    CardRules::new_enchantment(mana_cost!("{3}{B}{R}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When Rakdos Joins Up enters, return target creature card from \
                 your graveyard to the battlefield with two additional +1/+1 \
                 counters on it.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::PlusOnePlusOne,
                            amount: 2,
                        }],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever a legendary creature you control dies, Rakdos Joins \
                 Up deals damage equal to that creature's power to target \
                 opponent.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::TriggeringObjectPower,
                ),
            ),
        ]),
);

// OTJ 226 — Rakdos, the Muscle
pub(in crate::card::sets) static RAKDOS_THE_MUSCLE: CardRecord = CardRecord::new(
    "Rakdos, the Muscle",
    "bb34babd-1b85-4a7d-a066-a8337805056e",
    "Victor Maury",
    CardRules::new_creature(mana_cost!("{2}{B}{B}{R}"), &["Demon", "Mercenary"], 6, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::trample(),
            AbilityDef::triggered_with_targets(
                "Whenever you sacrifice another creature, exile cards equal to \
                 its mana value from the top of target player's library. Until \
                 your next end step, you may play those cards, and mana of any \
                 type can be spent to cast those spells.",
                TriggerEventDef::Sacrificed {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::One(ObjectRefDef::TriggeringObject),
                            select: ObjectValueDef::ManaValue,
                            operation: AggregateOperationDef::Sum,
                        }),
                    },
                    binding: crate::Binding!("top"),
                    then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("top"),
                        )),
                    },
                }),
            ),
            AbilityDef::activated(
                "Sacrifice another creature: Rakdos gains indestructible until \
                 end of turn. Tap it. Activate only once each turn.",
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Tap {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            )
            .once_each_turn(),
        ]),
);

// OTJ 227 — Riku of Many Paths
// Audit: unsupported — Needs exile-play permission expiring at cleanup of its controller's next turn; the current turn-count duration incorrectly remains usable during the following opponent turn.
pub(in crate::card::sets) static RIKU_OF_MANY_PATHS: CardRecord = CardRecord::new(
    "Riku of Many Paths",
    "21b63544-4c31-4f38-9907-0407719a60b1",
    "Denys Tsiperko",
    CardRules::unsupported(),
);

// OTJ 228 — Roxanne, Starfall Savant
pub(in crate::card::sets) static ROXANNE_STARFALL_SAVANT: CardRecord = CardRecord::new(
    "Roxanne, Starfall Savant",
    "11fbe52f-febd-49fc-8391-28d3efe9c3eb",
    "Ina Wong",
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Cat", "Druid"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever Roxanne enters or attacks, create a tapped colorless \
                 artifact token named Meteorite with \"When this token enters, \
                 it deals 2 damage to any target\" and \"{T}: Add one mana of \
                 any color.\"",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::artifact(&[], &[])
                            .with_name("Meteorite")
                            .with_abilities(&[
                                abilities::enters_trigger_with_targets(
                                    "When this token enters, it deals 2 damage to any target.",
                                    &[AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::AnyTarget,
                                    )],
                                    EffectDef::damage(
                                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        ValueDef::Constant(2),
                                    ),
                                ),
                                AbilityDef::activated_mana(
                                    "{T}: Add one mana of any color.",
                                    &[CostDef::TapSource],
                                    EffectDef::AddMana(AddManaEffectDef::any_color()),
                                ),
                            ]),
                    ))
                    .entering_tapped(),
                ),
            ),
            AbilityDef::triggered_mana(
                "Whenever you tap an artifact token for mana, add one mana of \
                 any type that artifact token produced.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Token,
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddMana(AddManaEffectDef::choice_from(ManaTypeSetDef::produced_by(
                    ObjectRefDef::TriggeringObject,
                ))),
            ),
        ]),
);

// OTJ 229 — Ruthless Lawbringer
// Audit: unsupported — Needs a reflexive trigger installed by the resolving sacrifice clause and retained after its source leaves, with targets selected when that reflexive trigger goes on the stack.
pub(in crate::card::sets) static RUTHLESS_LAWBRINGER: CardRecord = CardRecord::new(
    "Ruthless Lawbringer",
    "927b5498-23f1-47c0-b441-7daaeb54f9b8",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// OTJ 230 — Satoru, the Infiltrator
// Audit: unsupported — Needs a batched creature-entry event preserving each entrant's cast status and mana spent, so one draw is gated by the entire simultaneous entry batch.
pub(in crate::card::sets) static SATORU_THE_INFILTRATOR: CardRecord = CardRecord::new(
    "Satoru, the Infiltrator",
    "acc9a5cc-2b3c-4c2f-8176-4a2d86265cc5",
    "Heonhwa",
    CardRules::unsupported(),
);

// OTJ 231 — Selvala, Eager Trailblazer
// Audit: unsupported — Needs counting distinct current powers among controlled creatures when a mana ability resolves; the scalar aggregates support sum/minimum/maximum but not distinct-value cardinality.
pub(in crate::card::sets) static SELVALA_EAGER_TRAILBLAZER: CardRecord = CardRecord::new(
    "Selvala, Eager Trailblazer",
    "7d2e167f-7cb2-4f15-a1db-7ee56b7ba523",
    "Viko Menezes",
    CardRules::unsupported(),
);

// OTJ 232 — Seraphic Steed
pub(in crate::card::sets) static SERAPHIC_STEED: CardRecord = CardRecord::new(
    "Seraphic Steed",
    "7ada7ab4-0dee-4ff3-9817-1e61ca3f2ccf",
    "Jonas De Ro",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Unicorn", "Mount"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, create a 3/3 \
             white Angel creature token with flying.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Angel"], &[ManaColor::White], 3, 3)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 4 }],
            "Saddle 4 (Tap any number of other creatures you control with \
             total power 4 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// OTJ 233 — Slick Sequence
pub(in crate::card::sets) static SLICK_SEQUENCE: CardRecord = CardRecord::new(
    "Slick Sequence",
    "beb1c974-0d35-4e9f-a310-44eb2af64494",
    "Fajareka Setiawan",
    CardRules::new_instant(mana_cost!("{U}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Slick Sequence deals 2 damage to any target. If you've cast \
         another spell this turn, draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::You,
                        spell: ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    }),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ]),
    )]),
);

// OTJ 234 — Taii Wakeen, Perfect Shot
// Audit: unsupported — Needs noncombat-damage matching by amount relative to the damaged creature's toughness, plus an amount-adding damage replacement with the retained activation X.
pub(in crate::card::sets) static TAII_WAKEEN_PERFECT_SHOT: CardRecord = CardRecord::new(
    "Taii Wakeen, Perfect Shot",
    "1643af0b-fcbf-4636-8c50-77ec77eaa34d",
    "David Auden Nash",
    CardRules::unsupported(),
);

// OTJ 235 — Vial Smasher, Gleeful Grenadier
pub(in crate::card::sets) static VIAL_SMASHER_GLEEFUL_GRENADIER: CardRecord = CardRecord::new(
    "Vial Smasher, Gleeful Grenadier",
    "3afce4e6-ac59-4fba-b63a-8fed96bbdc4a",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin", "Mercenary"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever another outlaw you control enters, Vial Smasher \
             deals 1 damage to target opponent. (Assassins, Mercenaries, \
             Pirates, Rogues, and Warlocks are outlaws.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assassin")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mercenary")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rogue")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Warlock")),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        )]),
);

// OTJ 236 — Vraska Joins Up
pub(in crate::card::sets) static VRASKA_JOINS_UP: CardRecord = CardRecord::new(
    "Vraska Joins Up",
    "06e546c2-737e-4b17-bf60-3069b1ccdf31",
    "Wylie Beckert",
    CardRules::new_enchantment(mana_cost!("{B}{G}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Vraska Joins Up enters, put a deathtouch counter on each \
                 creature you control.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::Deathtouch,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever a legendary creature you control deals combat damage \
                 to a player, draw a card.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// OTJ 237 — Vraska, the Silencer
// Audit: unsupported — Needs prospective arrival type replacement and granted mana abilities on a reanimated card, so it enters as a Treasure artifact instead of first entering as a creature (CR 611.2e).
pub(in crate::card::sets) static VRASKA_THE_SILENCER: CardRecord = CardRecord::new(
    "Vraska, the Silencer",
    "b042abf2-c40b-4235-a4fa-2e4901c375c3",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// OTJ 238 — Wrangler of the Damned
// Audit: unsupported — Needs spell-cast history filtered by the zone a spell was cast from; the current history query retains caster and spell characteristics but no casting-origin zone.
pub(in crate::card::sets) static WRANGLER_OF_THE_DAMNED: CardRecord = CardRecord::new(
    "Wrangler of the Damned",
    "b4d163dd-67dc-4aab-afb9-d043352d109c",
    "Michal Ivan",
    CardRules::unsupported(),
);

// OTJ 239 — Wylie Duke, Atiin Hero
pub(in crate::card::sets) static WYLIE_DUKE_ATIIN_HERO: CardRecord = CardRecord::new(
    "Wylie Duke, Atiin Hero",
    "bc97ffcf-4f51-44cd-8daa-a7dae4592ee5",
    "Ekaterina Burmak",
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Human", "Ranger"], 4, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::triggered(
                "Whenever Wylie Duke becomes tapped, you gain 1 life and draw \
                 a card.",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            ),
        ]),
);

// OTJ 240 — Bandit's Haul
pub(in crate::card::sets) static BANDIT_S_HAUL: CardRecord = CardRecord::new(
    "Bandit's Haul",
    "68b2e74b-933b-4285-963b-dda3a986a914",
    "Monztre",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you commit a crime, put a loot counter on this \
             artifact. This ability triggers only once each turn. \
             (Targeting opponents, anything they control, and/or cards in \
             their graveyards is a crime.)",
            TriggerEventDef::CommittedCrime(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("loot"),
                amount: ValueDef::Constant(1),
            },
        )
        .triggering_at_most(1),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated(
            "{2}, {T}, Remove two loot counters from this artifact: Draw a \
             card.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("loot"),
                    amount: 2,
                },
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// OTJ 241 — Boom Box
pub(in crate::card::sets) static BOOM_BOX: CardRecord = CardRecord::new(
    "Boom Box",
    "ea61d964-6d73-422e-9e08-360ac66f237a",
    "Caio Monteiro",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{6}, {T}, Sacrifice this artifact: Destroy up to one target \
             artifact, up to one target creature, and up to one target \
             land.",
            &[
                CostDef::Mana(mana_cost!("{6}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                ),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                ),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                ),
            ],
            EffectDef::Destroy {
                object: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                    ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ObjectSetDef::LegalTargets(TargetIndex(1)),
                    ObjectSetDef::LegalTargets(TargetIndex(2)),
                ])),
                then: None,
            },
        ),
    ]),
);

// OTJ 242 — Gold Pan
pub(in crate::card::sets) static GOLD_PAN: CardRecord = CardRecord::new(
    "Gold Pan",
    "dc098aae-3d9b-453b-a37e-e102f81a8311",
    "Gaboleps",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a Treasure token. (It's an \
                 artifact with \"{T}, Sacrifice this token: Add one mana of \
                 any color.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
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
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// OTJ 243 — Lavaspur Boots
pub(in crate::card::sets) static LAVASPUR_BOOTS: CardRecord = CardRecord::new(
    "Lavaspur Boots",
    "e50709de-e6ef-4dbc-af1e-290fed279f34",
    "Mila Pesic",
CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has haste and ward {1}.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        // Ward reads as one clause on the Boots, so the granted ability carries the
                        // whole of the printed reminder rather than a paraphrase of it.
                        AppliedEffectDef::add_ability(&abilities::ward(
                            &[CostDef::Mana(crate::ManaCost::new(1, 0))],
                            "Ward {1} (Whenever this creature becomes the target of a spell or ability an opponent \
                            controls, counter it unless that player pays {1}.)",
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// OTJ 244 — Luxurious Locomotive
// Audit: unsupported — Needs the number of distinct creatures that crewed this Vehicle retained for the turn; current crew stores the animation result without contributor history.
pub(in crate::card::sets) static LUXURIOUS_LOCOMOTIVE: CardRecord = CardRecord::new(
    "Luxurious Locomotive",
    "cc598338-eeba-4815-a0a6-ff2dc09790d2",
    "Leon Tukker",
    CardRules::unsupported(),
);

// OTJ 245 — Mobile Homestead
pub(in crate::card::sets) static MOBILE_HOMESTEAD: CardRecord = CardRecord::new(
    "Mobile Homestead",
    "d5fa82e4-0b77-498a-bec0-52764d24957a",
    "Artur Nakhodkin",
    CardRules::new_vehicle(mana_cost!("{2}"), 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This Vehicle has haste as long as you control a Mount.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever this Vehicle attacks, look at the top card of your \
             library. If it's a land card, you may put it onto the \
             battlefield tapped.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            }),
        ),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
    ]),
);

// OTJ 246 — Oasis Gardener
pub(in crate::card::sets) static OASIS_GARDENER: CardRecord = CardRecord::new(
    "Oasis Gardener",
    "ee0dc663-4bfb-46d4-af79-d0143c799487",
    "Kristina Carroll",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Scarecrow"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// OTJ 247 — Redrock Sentinel
pub(in crate::card::sets) static REDROCK_SENTINEL: CardRecord = CardRecord::new(
    "Redrock Sentinel",
    "8eb98381-8418-4dfd-b7d1-4353570e611b",
    "Milivoj Ćeran",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Golem"], 2, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice a land: Draw a card and create a Treasure \
             token. (It's an artifact with \"{T}, Sacrifice this token: \
             Add one mana of any color.\")",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Land)),
            ],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ]),
);

// OTJ 248 — Silver Deputy
pub(in crate::card::sets) static SILVER_DEPUTY: CardRecord = CardRecord::new(
    "Silver Deputy",
    "39d2c11d-1eb8-4768-bc61-fa8f20a69462",
    "Artur Nakhodkin",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Mercenary"], 1, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may search your library for a \
             basic land card or a Desert card, reveal it, then shuffle and \
             put it on top.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Library,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control gets +1/+0 until end of \
             turn. Activate only as a sorcery.",
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// OTJ 249 — Sterling Hound
pub(in crate::card::sets) static STERLING_HOUND: CardRecord = CardRecord::new(
    "Sterling Hound",
    "9e5bfcf5-6e5c-47fe-af6c-6b18938261c6",
    "Leon Tukker",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Dog"], 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// OTJ 250 — Tomb Trawler
pub(in crate::card::sets) static TOMB_TRAWLER: CardRecord = CardRecord::new(
    "Tomb Trawler",
    "36e61cb8-219a-4fe6-a2e6-307665ffa38f",
    "Anton Solovianchyk",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Golem"], 0, 4).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}: Put target card from your graveyard on the bottom of \
             your library.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
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

// OTJ 251 — Abraded Bluffs
pub(in crate::card::sets) static ABRADED_BLUFFS: CardRecord = CardRecord::new(
    "Abraded Bluffs",
    "19e96521-b4ce-4a36-a887-200e05ccc804",
    "Piotr Dura",
    // The red-white Desert; only the two colours below are its own.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// OTJ 252 — Arid Archway
pub(in crate::card::sets) static ARID_ARCHWAY: CardRecord = CardRecord::new(
    "Arid Archway",
    "3f8c8fa2-12ab-4f6a-9f7a-2bc69e9ba024",
    "Raymond Bonilla",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::enters_trigger(
            "When this land enters, return a land you control to its \
             owner's hand. If another Desert was returned this way, \
             surveil 1. (Look at the top card of your library. You may put \
             it into your graveyard.)",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("returned"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::All(&[
                            TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("chosen")),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::All(&[
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                    ]),
                                ),
                            }),
                            TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("returned"),
                                ),
                                predicate: ObjectSetPredicateDef {
                                    filter: None,
                                    comparison: ComparisonDef::Greater,
                                    amount: 0,
                                },
                            }),
                        ]),
                        then: &abilities::surveil(ValueDef::Constant(1)),
                    },
                },
            }),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
    ]),
);

// OTJ 253 — Bristling Backwoods
pub(in crate::card::sets) static BRISTLING_BACKWOODS: CardRecord = CardRecord::new(
    "Bristling Backwoods",
    "d61dfeb7-7f6b-4601-8396-2cbb98165489",
    "Viko Menezes",
    // A tapped dual that pays a point of damage for the tempo, and a Desert
    // for whatever cares about that.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// OTJ 254 — Conduit Pylons
pub(in crate::card::sets) static CONDUIT_PYLONS: CardRecord = CardRecord::new(
    "Conduit Pylons",
    "5ffa48cc-b991-4d47-b7ec-cf678915c758",
    "Raymond Bonilla",
    // Untapped and colourless by default, so the fixing costs a mana rather
    // than a turn: the Desert deck plays it as a land that is never dead.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, surveil 1. (Look at the top card of your library. You may put \
             it into your graveyard.)",
            abilities::surveil(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// OTJ 255 — Creosote Heath
pub(in crate::card::sets) static CREOSOTE_HEATH: CardRecord = CardRecord::new(
    "Creosote Heath",
    "c5523dac-7aa0-4486-89c8-3b22a1411f26",
    "Leon Tukker",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// OTJ 256 — Eroded Canyon
pub(in crate::card::sets) static ERODED_CANYON: CardRecord = CardRecord::new(
    "Eroded Canyon",
    "5c9d080f-28d7-41d6-a4e0-5b3e3a5ed770",
    "Piotr Dura",
    // The blue-red Desert; only the two colours below are its own.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// OTJ 257 — Festering Gulch
pub(in crate::card::sets) static FESTERING_GULCH: CardRecord = CardRecord::new(
    "Festering Gulch",
    "4ad841eb-da0d-43d4-8b60-efe30922990b",
    "Daniel Romanovsky",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// OTJ 258 — Forlorn Flats
pub(in crate::card::sets) static FORLORN_FLATS: CardRecord = CardRecord::new(
    "Forlorn Flats",
    "963c100e-4e12-438f-b5ae-14391406dff6",
    "Robin Olausson",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// OTJ 259 — Jagged Barrens
pub(in crate::card::sets) static JAGGED_BARRENS: CardRecord = CardRecord::new(
    "Jagged Barrens",
    "5d809f5b-d965-4cb1-a9f8-2048f8534373",
    "Leonardo Borazio",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// OTJ 260 — Lonely Arroyo
pub(in crate::card::sets) static LONELY_ARROYO: CardRecord = CardRecord::new(
    "Lonely Arroyo",
    "4b778b63-e5fc-4d63-a93b-4372f32cade2",
    "Josu Hernaiz",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// OTJ 261 — Lush Oasis
pub(in crate::card::sets) static LUSH_OASIS: CardRecord = CardRecord::new(
    "Lush Oasis",
    "988e44c5-4632-4ebb-b6ae-c3886e49d637",
    "Piotr Dura",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// OTJ 262 — Mirage Mesa
pub(in crate::card::sets) static MIRAGE_MESA: CardRecord = CardRecord::new(
    "Mirage Mesa",
    "c3d2e816-c06d-4c5d-98fe-c350d8cfab27",
    "Andrew Mar",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::as_enters(
            "As this permanent enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
    ]),
);

// OTJ 263 — Sandstorm Verge
pub(in crate::card::sets) static SANDSTORM_VERGE: CardRecord = CardRecord::new(
    "Sandstorm Verge",
    "3ab4e0a4-2faf-456b-99e3-ee06c008538c",
    "Jorge Jacinto",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature can't block this turn. Activate \
             only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// OTJ 264 — Soured Springs
pub(in crate::card::sets) static SOURED_SPRINGS: CardRecord = CardRecord::new(
    "Soured Springs",
    "67daa31c-d9c4-4c22-b29c-1b8a17d577e5",
    "Leonardo Borazio",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// OTJ 265 — Bucolic Ranch
pub(in crate::card::sets) static BUCOLIC_RANCH: CardRecord = CardRecord::new(
    "Bucolic Ranch",
    "6c4f6b81-53d0-49fb-b404-c2ad67de7493",
    "Leonardo Borazio",
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast \
             a Mount spell.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                    "Mount",
                ))),
            ])),
        ),
        AbilityDef::activated(
            "{3}, {T}: Look at the top card of your library. If it's a \
             Mount card, you may reveal it and put it into your hand. If \
             you don't put it into your hand, you may put it on the bottom \
             of your library.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
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
                    EffectDef::May {
                        player: EffectRecipientDef::Controller,
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "rest"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    },
                ]),
            }),
        ),
    ]),
);

// OTJ 266 — Blooming Marsh (reprint)
const BLOOMING_MARSH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BLOOMING_MARSH,
    "861caabb-0573-4e94-8b03-342f90465064",
    "Yeong-Hao Han",
);

// OTJ 267 — Botanical Sanctum (reprint)
const BOTANICAL_SANCTUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BOTANICAL_SANCTUM,
    "cc18d5f4-a56a-4f7d-9f56-ccc92cbfb7f7",
    "Jorge Jacinto",
);

// OTJ 268 — Concealed Courtyard (reprint)
const CONCEALED_COURTYARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::CONCEALED_COURTYARD,
    "b75df1f0-0513-40e4-a449-454f75de6434",
    "Rockey Chen",
);

// OTJ 269 — Inspiring Vantage (reprint)
const INSPIRING_VANTAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::INSPIRING_VANTAGE,
    "85df6b6a-2dcf-4828-a4a8-e07d52e1fddd",
    "Volkan Baǵa",
);

// OTJ 270 — Spirebluff Canal (reprint)
const SPIREBLUFF_CANAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::SPIREBLUFF_CANAL,
    "59a04e16-a767-4112-ab01-6ca1b09c286c",
    "Ron Spears",
);

// OTJ 271 — Jace Reawakened
// Audit: unsupported — Needs an effect or external permission that makes other cards plotted, including its later-turn, sorcery-only free cast permission; existing plot only supports the card's own hand special action.
pub(in crate::card::sets) static JACE_REAWAKENED: CardRecord = CardRecord::new(
    "Jace Reawakened",
    "fd17e8d4-499e-4005-ae3c-bc9c44dc5a67",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// OTJ 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "cfe51d97-66f6-4ac3-b926-01ab7e4c5686",
    "Salvatorre Zee Yazzie",
);

// OTJ 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "a624d656-207d-4e73-b615-59e7cf64ad64",
    "Salvatorre Zee Yazzie",
);

// OTJ 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "3b383b16-8128-4d9e-a0d0-9b8ccc9ad6df",
    "Salvatorre Zee Yazzie",
);

// OTJ 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "0a7dbfd2-cda7-4fc9-9677-e442fb5f5f6f",
    "Salvatorre Zee Yazzie",
);

// OTJ 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "baf8a774-65f3-431e-b084-328ff1000895",
    "Salvatorre Zee Yazzie",
);

// OTJ 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "6f501773-1b39-4a4c-9b45-a950385c9e82",
    "Sergey Glushakov",
);

// OTJ 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "fe60da77-084c-49e8-9948-9ac4b6a6382f",
    "Adam Paquette",
);

// OTJ 279 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "acd6be3f-745c-41ad-95c9-1db66ba56be2",
    "Sergey Glushakov",
);

// OTJ 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "91be4db0-7cb3-4202-939b-cb7a26e90019",
    "Adam Paquette",
);

// OTJ 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "eb7dc259-9949-4673-a8f1-874396948392",
    "Sergey Glushakov",
);

// OTJ 282 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "5c2c9dc0-7f3a-4f3a-ba08-5c2f87e252bc",
    "Adam Paquette",
);

// OTJ 283 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "9137b4aa-2289-4a4d-b1f5-ae75a0928278",
    "Sergey Glushakov",
);

// OTJ 284 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "2237ee9b-fff6-472c-903c-11faf9bb116d",
    "Adam Paquette",
);

// OTJ 285 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "dc20a07e-5f92-49c2-9c97-d5eda536d3f6",
    "Sergey Glushakov",
);

// OTJ 286 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "8e9ac507-7c8f-431f-8d1a-220ceeacf871",
    "Adam Paquette",
);

// OTJ 287 — Geralf, the Fleshwright (alternate printing)
const GERALF_THE_FLESHWRIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GERALF_THE_FLESHWRIGHT,
    1,
    "6ef3f55e-b8e6-4ef1-85e4-2aef5afc15ab",
    "Pedro Potier",
);

// OTJ 288 — Gisa, the Hellraiser (alternate printing)
const GISA_THE_HELLRAISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GISA_THE_HELLRAISER,
    1,
    "37c1ad56-cf6e-4717-a56e-feeb7339b8c3",
    "Greg Staples",
);

// OTJ 289 — Kaervek, the Punisher (alternate printing)
const KAERVEK_THE_PUNISHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAERVEK_THE_PUNISHER,
    1,
    "95e759e4-bda5-4d01-a9a8-3986df1d4428",
    "Pedro Potier",
);

// OTJ 290 — Tinybones, the Pickpocket (alternate printing)
const TINYBONES_THE_PICKPOCKET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_THE_PICKPOCKET,
    1,
    "1b2a3962-315d-48df-9af5-7cb4dd2040de",
    "Michael Walsh",
);

// OTJ 291 — Annie Flash, the Veteran (alternate printing)
const ANNIE_FLASH_THE_VETERAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANNIE_FLASH_THE_VETERAN,
    1,
    "0ea8d21f-a003-4711-ad5c-0bde87e1edc6",
    "Justine Mara Andersen",
);

// OTJ 292 — Breeches, the Blastmaker (alternate printing)
const BREECHES_THE_BLASTMAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BREECHES_THE_BLASTMAKER,
    1,
    "fcde5cd5-4ce6-4b94-8d97-534a647dbfc1",
    "Michael Walsh",
);

// OTJ 293 — Eriette, the Beguiler (alternate printing)
const ERIETTE_THE_BEGUILER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERIETTE_THE_BEGUILER,
    1,
    "f69b5791-a405-4578-acba-959cc181e9ad",
    "Dibujante Nocturno",
);

// OTJ 294 — Kellan, the Kid (alternate printing)
const KELLAN_THE_KID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_THE_KID,
    1,
    "789659f6-af39-4357-985f-a006f192893c",
    "Benjamin Ee",
);

// OTJ 295 — Malcolm, the Eyes (alternate printing)
const MALCOLM_THE_EYES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MALCOLM_THE_EYES,
    1,
    "480de469-3fac-4a2b-8dec-7899ce00551e",
    "Michael Walsh",
);

// OTJ 296 — Oko, the Ringleader (alternate printing)
const OKO_THE_RINGLEADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OKO_THE_RINGLEADER,
    1,
    "9c025072-0d0a-4ca7-a386-f1ed7c97638b",
    "Dibujante Nocturno",
);

// OTJ 297 — Rakdos, the Muscle (alternate printing)
const RAKDOS_THE_MUSCLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_THE_MUSCLE,
    1,
    "b85635e0-fb5b-42ea-8a30-d67922cbca95",
    "Greg Staples",
);

// OTJ 298 — Satoru, the Infiltrator (alternate printing)
const SATORU_THE_INFILTRATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SATORU_THE_INFILTRATOR,
    1,
    "4b9a76f0-b697-4c1d-9b60-c66f7fc4316e",
    "Denis Medri",
);

// OTJ 299 — Vraska, the Silencer (alternate printing)
const VRASKA_THE_SILENCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VRASKA_THE_SILENCER,
    1,
    "2a6cc9ab-a1d8-47de-8ac5-112e5fee00f9",
    "Jarel Threat",
);

// OTJ 300 — Blooming Marsh (alternate printing)
const BLOOMING_MARSH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::BLOOMING_MARSH,
    1,
    "1cfb4a80-3319-41ad-9d97-a9a53c9f84fb",
    "Piotr Dura",
);

// OTJ 301 — Botanical Sanctum (alternate printing)
const BOTANICAL_SANCTUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::BOTANICAL_SANCTUM,
    1,
    "82670072-9851-48c7-a8f5-7842bff6c252",
    "Piotr Dura",
);

// OTJ 302 — Concealed Courtyard (alternate printing)
const CONCEALED_COURTYARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::CONCEALED_COURTYARD,
    1,
    "d7e85ace-7e2c-46f6-adb9-07f33f8e1750",
    "Piotr Dura",
);

// OTJ 303 — Inspiring Vantage (alternate printing)
const INSPIRING_VANTAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::INSPIRING_VANTAGE,
    1,
    "551e216d-a82f-49a1-a3fd-8165715a05c4",
    "Piotr Dura",
);

// OTJ 304 — Spirebluff Canal (alternate printing)
const SPIREBLUFF_CANAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::SPIREBLUFF_CANAL,
    1,
    "9d83bdb3-1f04-4b99-a36b-312da0cbed50",
    "Piotr Dura",
);

// OTJ 305 — Oko, the Ringleader (alternate printing)
const OKO_THE_RINGLEADER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OKO_THE_RINGLEADER,
    2,
    "0bfbb249-f2cc-4295-b6e3-4fd7e1eac183",
    "Lie Setiawan",
);

// OTJ 306 — Jace Reawakened (alternate printing)
const JACE_REAWAKENED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JACE_REAWAKENED,
    1,
    "d73bdf79-6c18-46bc-a8bb-97e07dd23aff",
    "Chris Rallis",
);

// OTJ 307 — Another Round (alternate printing)
const ANOTHER_ROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANOTHER_ROUND,
    1,
    "ddec615b-537a-4fef-a2a7-9bdf74c0192a",
    "Darrell Riche",
);

// OTJ 308 — Archangel of Tithes (alternate printing)
const ARCHANGEL_OF_TITHES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ori::ARCHANGEL_OF_TITHES,
    1,
    "d34f043c-d01e-4462-b381-39748d7fa31b",
    "Denys Tsiperko",
);

// OTJ 309 — Aven Interrupter (alternate printing)
const AVEN_INTERRUPTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVEN_INTERRUPTER,
    1,
    "75143fdb-5651-4289-86df-76c9655a0599",
    "Daniel Romanovsky",
);

// OTJ 310 — Claim Jumper (alternate printing)
const CLAIM_JUMPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLAIM_JUMPER,
    1,
    "dfad3c84-4264-4757-8e83-25dbbed67070",
    "Gaboleps",
);

// OTJ 311 — Dust Animus (alternate printing)
const DUST_ANIMUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DUST_ANIMUS,
    1,
    "d0d9d4f0-48ec-43e6-bb93-a9589790417b",
    "Uriah Voth",
);

// OTJ 312 — Final Showdown (alternate printing)
const FINAL_SHOWDOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FINAL_SHOWDOWN,
    1,
    "94397320-b814-487f-aca2-537517ff9eff",
    "Izzy",
);

// OTJ 313 — Fortune, Loyal Steed (alternate printing)
const FORTUNE_LOYAL_STEED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORTUNE_LOYAL_STEED,
    1,
    "02ffb299-a327-43c1-868e-1c5225204956",
    "Artur Nakhodkin",
);

// OTJ 314 — High Noon (alternate printing)
const HIGH_NOON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_NOON,
    1,
    "0d69fce2-3307-4518-a196-e0b8155dca73",
    "Eduardo Francisco",
);

// OTJ 315 — One Last Job (alternate printing)
const ONE_LAST_JOB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ONE_LAST_JOB,
    1,
    "ad6db7ba-3150-4ae8-84e3-11661bf1d1c4",
    "Caroline Gariba",
);

// OTJ 316 — Archmage's Newt (alternate printing)
const ARCHMAGE_S_NEWT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHMAGE_S_NEWT,
    1,
    "25563be1-71f1-4f70-8527-02c5855a0b9d",
    "Edgar Sánchez Hidalgo",
);

// OTJ 317 — Double Down (alternate printing)
const DOUBLE_DOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOUBLE_DOWN,
    1,
    "d6f24b8b-f9ed-4c77-8cb0-2a94848ee69b",
    "Javier Charro",
);

// OTJ 318 — Duelist of the Mind (alternate printing)
const DUELIST_OF_THE_MIND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DUELIST_OF_THE_MIND,
    1,
    "4126ae45-00b3-419c-8f07-d1286ac6b121",
    "Darren Tan",
);

// OTJ 319 — Fblthp, Lost on the Range (alternate printing)
const FBLTHP_LOST_ON_THE_RANGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FBLTHP_LOST_ON_THE_RANGE,
    1,
    "51bbf861-448b-455b-8922-38515ba65c40",
    "Brian Valeza",
);

// OTJ 320 — The Key to the Vault (alternate printing)
const THE_KEY_TO_THE_VAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_KEY_TO_THE_VAULT,
    1,
    "d70388ee-2f97-4282-ba74-af95462ac0aa",
    "Leon Tukker",
);

// OTJ 321 — Step Between Worlds (alternate printing)
const STEP_BETWEEN_WORLDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STEP_BETWEEN_WORLDS,
    1,
    "0e532e5b-6d84-4669-8788-f471b1498c7b",
    "Chris Ostrowski",
);

// OTJ 322 — Stoic Sphinx (alternate printing)
const STOIC_SPHINX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STOIC_SPHINX,
    1,
    "e346fab1-48db-4cd2-836d-dad5e8437308",
    "Andreas Zafiratos",
);

// OTJ 323 — Three Steps Ahead (alternate printing)
const THREE_STEPS_AHEAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THREE_STEPS_AHEAD,
    1,
    "be980c3b-b4ab-4ed7-9f61-d8db54c226d9",
    "Francisco Miyara",
);

// OTJ 324 — Caustic Bronco (alternate printing)
const CAUSTIC_BRONCO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAUSTIC_BRONCO,
    1,
    "517505a0-0d05-4e81-8582-999b88040f48",
    "Brent Hollowell",
);

// OTJ 325 — Insatiable Avarice (alternate printing)
const INSATIABLE_AVARICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSATIABLE_AVARICE,
    1,
    "aae03d91-8269-4124-ba50-a6c65f47718b",
    "Scott Murphy",
);

// OTJ 326 — Pitiless Carnage (alternate printing)
const PITILESS_CARNAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PITILESS_CARNAGE,
    1,
    "2bea6391-e70d-4f36-86b7-fd08440b4976",
    "Richard Kane Ferguson",
);

// OTJ 327 — Rush of Dread (alternate printing)
const RUSH_OF_DREAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUSH_OF_DREAD,
    1,
    "e4ca7ac5-5552-487f-9ac9-1092fe6cd165",
    "Chris Seaman",
);

// OTJ 328 — Tinybones Joins Up (alternate printing)
const TINYBONES_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_JOINS_UP,
    1,
    "a3afc0c2-95e7-4b9f-94c5-144cce17c7ed",
    "Wylie Beckert",
);

// OTJ 329 — Vadmir, New Blood (alternate printing)
const VADMIR_NEW_BLOOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VADMIR_NEW_BLOOD,
    1,
    "d876a888-e119-494d-857a-e780a277c817",
    "Andreas Zafiratos",
);

// OTJ 330 — Calamity, Galloping Inferno (alternate printing)
const CALAMITY_GALLOPING_INFERNO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CALAMITY_GALLOPING_INFERNO,
    1,
    "295e63c2-b533-4dbf-8c8a-c6493de31457",
    "Artur Nakhodkin",
);

// OTJ 331 — Great Train Heist (alternate printing)
const GREAT_TRAIN_HEIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_TRAIN_HEIST,
    1,
    "b570d9c5-3348-482f-a211-ed8c9777a9fa",
    "Campbell White",
);

// OTJ 332 — Hell to Pay (alternate printing)
const HELL_TO_PAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HELL_TO_PAY,
    1,
    "c522c232-0f86-49ad-969f-a2b086432a97",
    "Liiga Smilshkalne",
);

// OTJ 333 — Hellspur Posse Boss (alternate printing)
const HELLSPUR_POSSE_BOSS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HELLSPUR_POSSE_BOSS,
    1,
    "259262b9-f74b-4e71-9c4e-cf18ba2dc3c2",
    "Artur Nakhodkin",
);

// OTJ 334 — Magda, the Hoardmaster (alternate printing)
const MAGDA_THE_HOARDMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAGDA_THE_HOARDMASTER,
    1,
    "adf42a57-02f0-4a3f-8677-d68ffe3090c0",
    "Diego Gisbert",
);

// OTJ 335 — Slickshot Show-Off
pub(in crate::card::sets) static SLICKSHOT_SHOW_OFF: CardRecord = CardRecord::new(
    "Slickshot Show-Off",
    "304523e7-f332-4c1d-9590-ff9a70daff26",
    "Augusto Quirino",
    // Two mana for a hasty flier that grows with every spell after it, and
    // a plot cost that pays the two a turn early so the whole of a later
    // turn's mana can go into the spells it grows on.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Bird", "Wizard"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this creature gets +2/+0 until end of turn.",
            // A noncreature spell you cast, which is prowess with a bigger number and
            // no toughness: what the Bird wants is one turn with several spells in it.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{R}"))]),
    ]),
);

// OTJ 336 — Stingerback Terror (alternate printing)
const STINGERBACK_TERROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STINGERBACK_TERROR,
    1,
    "2e9fa00b-bc34-4be8-a21f-11ae695f166d",
    "Slawomir Maniak",
);

// OTJ 337 — Terror of the Peaks (alternate printing)
const TERROR_OF_THE_PEAKS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m21::TERROR_OF_THE_PEAKS,
    1,
    "cd20aae9-9de0-498b-8325-f8e8290d56e3",
    "Joshua Raphael",
);

// OTJ 338 — Bristly Bill, Spine Sower (alternate printing)
const BRISTLY_BILL_SPINE_SOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRISTLY_BILL_SPINE_SOWER,
    1,
    "f921e711-168d-4836-bca2-f9bb43992708",
    "Daniel Zrom",
);

// OTJ 339 — Colossal Rattlewurm (alternate printing)
const COLOSSAL_RATTLEWURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLOSSAL_RATTLEWURM,
    1,
    "e6fbb0c7-29ac-4394-87e1-6e8227602aac",
    "Filip Burburan",
);

// OTJ 340 — Freestrider Lookout (alternate printing)
const FREESTRIDER_LOOKOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FREESTRIDER_LOOKOUT,
    1,
    "81289833-ebdb-4fe7-ad17-6e39eb399e69",
    "Matt Zeilinger",
);

// OTJ 341 — Goldvein Hydra (alternate printing)
const GOLDVEIN_HYDRA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLDVEIN_HYDRA,
    1,
    "8700b3a6-327b-4dc0-aa8d-434ca971a7b9",
    "David Auden Nash",
);

// OTJ 342 — Ornery Tumblewagg (alternate printing)
const ORNERY_TUMBLEWAGG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORNERY_TUMBLEWAGG,
    1,
    "3a1729d3-8e27-4db0-a7c6-e629a8faf946",
    "Izzy",
);

// OTJ 343 — Outcaster Trailblazer (alternate printing)
const OUTCASTER_TRAILBLAZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OUTCASTER_TRAILBLAZER,
    1,
    "8cf1fe32-eac2-4d50-b403-25c3666f6d80",
    "Denys Tsiperko",
);

// OTJ 344 — Railway Brawler (alternate printing)
const RAILWAY_BRAWLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAILWAY_BRAWLER,
    1,
    "78db8c96-e6b8-4d4a-9935-29f12385a151",
    "Kevin Sidharta",
);

// OTJ 345 — Smuggler's Surprise (alternate printing)
const SMUGGLER_S_SURPRISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMUGGLER_S_SURPRISE,
    1,
    "41b76d0d-da55-44ec-add8-61696ee40d21",
    "Jonas De Ro",
);

// OTJ 346 — Akul the Unrepentant (alternate printing)
const AKUL_THE_UNREPENTANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AKUL_THE_UNREPENTANT,
    1,
    "6534bdb1-9436-4919-8a8b-5401bc070fc2",
    "Kekai Kotaki",
);

// OTJ 347 — Annie Joins Up (alternate printing)
const ANNIE_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANNIE_JOINS_UP,
    1,
    "9ce19341-e652-448f-8f14-0439bd8fa385",
    "Wylie Beckert",
);

// OTJ 348 — Assimilation Aegis (alternate printing)
const ASSIMILATION_AEGIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASSIMILATION_AEGIS,
    1,
    "0d8e4f7b-bdf6-44d3-9b2b-4420bab84864",
    "Matt Stewart",
);

// OTJ 349 — Bonny Pall, Clearcutter (alternate printing)
const BONNY_PALL_CLEARCUTTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BONNY_PALL_CLEARCUTTER,
    1,
    "ee29f20b-27cd-433a-8874-41f500720109",
    "Bryan Sola",
);

// OTJ 350 — Bruse Tarl, Roving Rancher (alternate printing)
const BRUSE_TARL_ROVING_RANCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRUSE_TARL_ROVING_RANCHER,
    1,
    "026257bf-f4e7-45f8-9c93-7248f201c583",
    "Forrest Imel",
);

// OTJ 351 — Ghired, Mirror of the Wilds (alternate printing)
const GHIRED_MIRROR_OF_THE_WILDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GHIRED_MIRROR_OF_THE_WILDS,
    1,
    "203c84a3-5f1a-440f-93e7-bb65d1e07886",
    "Diego Gisbert",
);

// OTJ 352 — The Gitrog, Ravenous Ride (alternate printing)
const THE_GITROG_RAVENOUS_RIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_GITROG_RAVENOUS_RIDE,
    1,
    "a634b934-710b-4c83-b769-d8a034e297b8",
    "Johan Grenier",
);

// OTJ 353 — Kambal, Profiteering Mayor (alternate printing)
const KAMBAL_PROFITEERING_MAYOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAMBAL_PROFITEERING_MAYOR,
    1,
    "8cfd5a0f-8859-4272-a08f-13b788422f4c",
    "Andreas Zafiratos",
);

// OTJ 354 — Kellan Joins Up (alternate printing)
const KELLAN_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_JOINS_UP,
    1,
    "1d927dc7-77b7-473d-b50b-e81c52d66b55",
    "Wylie Beckert",
);

// OTJ 355 — Laughing Jasper Flint (alternate printing)
const LAUGHING_JASPER_FLINT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAUGHING_JASPER_FLINT,
    1,
    "5df7fadd-33ea-4bf4-9122-362821ef96dc",
    "Francis Tneh",
);

// OTJ 356 — Lilah, Undefeated Slickshot (alternate printing)
const LILAH_UNDEFEATED_SLICKSHOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LILAH_UNDEFEATED_SLICKSHOT,
    1,
    "95efd401-579e-448b-9655-31311c0ae41e",
    "Andreas Zafiratos",
);

// OTJ 357 — Marchesa, Dealer of Death (alternate printing)
const MARCHESA_DEALER_OF_DEATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARCHESA_DEALER_OF_DEATH,
    1,
    "724309cd-fb05-4632-a7be-33a9ba024e4a",
    "Ryan Pancoast",
);

// OTJ 358 — Obeka, Splitter of Seconds (alternate printing)
const OBEKA_SPLITTER_OF_SECONDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OBEKA_SPLITTER_OF_SECONDS,
    1,
    "183b8c06-62ab-41e8-82c1-f23066d832ee",
    "Ryan Pancoast",
);

// OTJ 359 — Pillage the Bog (alternate printing)
const PILLAGE_THE_BOG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PILLAGE_THE_BOG,
    1,
    "c05939b7-1877-4464-98fe-d3b9ae754fb9",
    "Forrest Imel",
);

// OTJ 360 — Rakdos Joins Up (alternate printing)
const RAKDOS_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_JOINS_UP,
    1,
    "761a743a-6358-4e08-a6b7-6eaaf7ab9ddc",
    "Wylie Beckert",
);

// OTJ 361 — Riku of Many Paths (alternate printing)
const RIKU_OF_MANY_PATHS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIKU_OF_MANY_PATHS,
    1,
    "215b4632-adce-4357-bfcf-b6014ed2a24b",
    "Denys Tsiperko",
);

// OTJ 362 — Roxanne, Starfall Savant (alternate printing)
const ROXANNE_STARFALL_SAVANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROXANNE_STARFALL_SAVANT,
    1,
    "7b8aea8d-452a-4b23-bc78-8c7db54610d3",
    "Ina Wong",
);

// OTJ 363 — Selvala, Eager Trailblazer (alternate printing)
const SELVALA_EAGER_TRAILBLAZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SELVALA_EAGER_TRAILBLAZER,
    1,
    "e523de28-b2d5-4be9-be78-883cef2499e9",
    "Viko Menezes",
);

// OTJ 364 — Seraphic Steed (alternate printing)
const SERAPHIC_STEED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SERAPHIC_STEED,
    1,
    "1c9df2d7-6738-496c-8164-8bdf7d011df3",
    "Jonas De Ro",
);

// OTJ 365 — Taii Wakeen, Perfect Shot (alternate printing)
const TAII_WAKEEN_PERFECT_SHOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TAII_WAKEEN_PERFECT_SHOT,
    1,
    "3c558349-87bc-4e0f-96c2-b075f7da97d5",
    "David Auden Nash",
);

// OTJ 366 — Vraska Joins Up (alternate printing)
const VRASKA_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VRASKA_JOINS_UP,
    1,
    "43579701-c89a-4455-8f6a-0589af0c6bd3",
    "Wylie Beckert",
);

// OTJ 367 — Wylie Duke, Atiin Hero (alternate printing)
const WYLIE_DUKE_ATIIN_HERO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WYLIE_DUKE_ATIIN_HERO,
    1,
    "17955d83-89ea-4151-950c-d72acf1a852a",
    "Ekaterina Burmak",
);

// OTJ 368 — Frontier Seeker (alternate printing)
const FRONTIER_SEEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRONTIER_SEEKER,
    1,
    "5fd49911-9ddb-4bef-8cd0-89f558ccd5cc",
    "Raluca Marinescu",
);

// OTJ 369 — Scorching Shot (alternate printing)
const SCORCHING_SHOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCORCHING_SHOT,
    1,
    "188694af-b856-4439-8ce0-8306b38caacb",
    "Caio Monteiro",
);

// OTJ 370 — Honest Rutstein (alternate printing)
const HONEST_RUTSTEIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HONEST_RUTSTEIN,
    1,
    "d8df8b87-3ea1-44eb-9218-9e7e06dfd0db",
    "Javier Charro",
);

// OTJ 371 — Make Your Own Luck (alternate printing)
const MAKE_YOUR_OWN_LUCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAKE_YOUR_OWN_LUCK,
    1,
    "d9073963-5866-4132-a059-a46f96cd2a8d",
    "Chris Seaman",
);

// OTJ 372 — Ruthless Lawbringer (alternate printing)
const RUTHLESS_LAWBRINGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUTHLESS_LAWBRINGER,
    1,
    "f19e6995-4926-4513-b8c1-f35a22aafbe5",
    "Joshua Raphael",
);

// OTJ 373 — The Key to the Vault (alternate printing)
const THE_KEY_TO_THE_VAULT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_KEY_TO_THE_VAULT,
    2,
    "e695db51-f9d5-4eef-84d9-62f7602792b4",
    "Eli Minaya",
);

// OTJ 374 — Magda, the Hoardmaster (alternate printing)
const MAGDA_THE_HOARDMASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MAGDA_THE_HOARDMASTER,
    2,
    "2834e84e-e932-4052-9dad-2e1c8e76fbbc",
    "Loïc Canavaggia",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANOTHER_ROUND,
    &ARMORED_ARMADILLO,
    &AVEN_INTERRUPTER,
    &BOUNDING_FELIDAR,
    &BOVINE_INTERVENTION,
    &BRIDLED_BIGHORN,
    &CLAIM_JUMPER,
    &DUST_ANIMUS,
    &ERIETTE_S_LULLABY,
    &FINAL_SHOWDOWN,
    &FORTUNE_LOYAL_STEED,
    &FRONTIER_SEEKER,
    &GETAWAY_GLAMER,
    &HIGH_NOON,
    &HOLY_COW,
    &INVENTIVE_WINGSMITH,
    &LASSOED_BY_THE_LAW,
    &MYSTICAL_TETHER,
    &NURTURING_PIXIE,
    &OMENPORT_VIGILANTE,
    &ONE_LAST_JOB,
    &OUTLAW_MEDIC,
    &PRAIRIE_DOG,
    &PROSPERITY_TYCOON,
    &REQUISITION_RAID,
    &RUSTLER_RAMPAGE,
    &SHEPHERD_OF_THE_CLOUDS,
    &SHERIFF_OF_SAFE_PASSAGE,
    &STAGECOACH_SECURITY,
    &STEER_CLEAR,
    &STERLING_KEYKEEPER,
    &STERLING_SUPPLIER,
    &THUNDER_LASSO,
    &TRAINED_ARYNX,
    &VENGEFUL_TOWNSFOLK,
    &WANTED_GRIFFIN,
    &ARCHMAGE_S_NEWT,
    &CANYON_CRAB,
    &DARING_THUNDER_THIEF,
    &DEEPMUCK_DESPERADO,
    &DJINN_OF_FOOL_S_FALL,
    &DOUBLE_DOWN,
    &DUELIST_OF_THE_MIND,
    &EMERGENT_HAUNTING,
    &FAILED_FORDING,
    &FBLTHP_LOST_ON_THE_RANGE,
    &FLEETING_REFLECTION,
    &GERALF_THE_FLESHWRIGHT,
    &GEYSER_DRAKE,
    &HARRIER_STRIX,
    &JAILBREAK_SCHEME,
    &THE_KEY_TO_THE_VAULT,
    &LOAN_SHARK,
    &MARAUDING_SPHINX,
    &METAMORPHIC_BLAST,
    &NIMBLE_BRIGAND,
    &OUTLAW_STITCHER,
    &PEERLESS_ROPEMASTER,
    &PHANTOM_INTERFERENCE,
    &PLAN_THE_HEIST,
    &RAZZLE_DAZZLER,
    &SEIZE_THE_SECRETS,
    &SHACKLE_SLINGER,
    &SHIFTING_GRIFT,
    &SLICKSHOT_LOCKPICKER,
    &SLICKSHOT_VAULT_BUSTER,
    &SPRING_SPLASHER,
    &STEP_BETWEEN_WORLDS,
    &STOIC_SPHINX,
    &STOP_COLD,
    &TAKE_THE_FALL,
    &THIS_TOWN_AIN_T_BIG_ENOUGH,
    &THREE_STEPS_AHEAD,
    &VISAGE_BANDIT,
    &AMBUSH_GIGAPEDE,
    &BINDING_NEGOTIATION,
    &BLACKSNAG_BUZZARD,
    &BLOOD_HUSTLER,
    &BONEYARD_DESECRATOR,
    &CAUSTIC_BRONCO,
    &CONSUMING_ASHES,
    &DESERT_S_DUE,
    &DESPERATE_BLOODSEEKER,
    &FORSAKEN_MINER,
    &GISA_THE_HELLRAISER,
    &HOLLOW_MARAUDER,
    &INSATIABLE_AVARICE,
    &KAERVEK_THE_PUNISHER,
    &LIVELY_DIRGE,
    &MOURNER_S_SURPRISE,
    &NEUTRALIZE_THE_GUARDS,
    &NEZUMI_LINKBREAKER,
    &OVERZEALOUS_MUSCLE,
    &PITILESS_CARNAGE,
    &RAKISH_CREW,
    &RATTLEBACK_APOTHECARY,
    &RAVEN_OF_FELL_OMENS,
    &RICTUS_ROBBER,
    &ROOFTOP_ASSASSIN,
    &RUSH_OF_DREAD,
    &SERVANT_OF_THE_STINGER,
    &SHOOT_THE_SHERIFF,
    &TINYBONES_JOINS_UP,
    &TINYBONES_THE_PICKPOCKET,
    &TREASURE_DREDGER,
    &UNFORTUNATE_ACCIDENT,
    &UNSCRUPULOUS_CONTRACTOR,
    &VADMIR_NEW_BLOOD,
    &VAULT_PLUNDERER,
    &BRIMSTONE_ROUNDUP,
    &CALAMITY_GALLOPING_INFERNO,
    &CAUGHT_IN_THE_CROSSFIRE,
    &CUNNING_COYOTE,
    &DEADEYE_DUELIST,
    &DEMONIC_RUCKUS,
    &DISCERNING_PEDDLER,
    &EXPLOSIVE_DERAILMENT,
    &FEROCIFICATION,
    &GILA_COURSER,
    &GREAT_TRAIN_HEIST,
    &HELL_TO_PAY,
    &HELLSPUR_BRUTE,
    &HELLSPUR_POSSE_BOSS,
    &HIGHWAY_ROBBERY,
    &IRASCIBLE_WOLVERINE,
    &IRON_FIST_PULVERIZER,
    &LONGHORN_SHARPSHOOTER,
    &MAGDA_THE_HOARDMASTER,
    &MAGEBANE_LIZARD,
    &MINE_RAIDER,
    &OUTLAWS_FURY,
    &PRICKLY_PAIR,
    &QUICK_DRAW,
    &QUILLED_CHARGER,
    &RECKLESS_LACKEY,
    &RESILIENT_ROADRUNNER,
    &RETURN_THE_FAVOR,
    &RODEO_PYROMANCERS,
    &SCALESTORM_SUMMONER,
    &SCORCHING_SHOT,
    &STINGERBACK_TERROR,
    &TAKE_FOR_A_RIDE,
    &THUNDER_SALVO,
    &TRICK_SHOT,
    &ALOE_ALCHEMIST,
    &ANKLE_BITER,
    &BEASTBOND_OUTCASTER,
    &BETRAYAL_AT_THE_VAULT,
    &BRISTLEPACK_SENTRY,
    &BRISTLY_BILL_SPINE_SOWER,
    &CACTARANTULA,
    &COLOSSAL_RATTLEWURM,
    &DANCE_OF_THE_TUMBLEWEEDS,
    &DROVER_GRIZZLY,
    &FREESTRIDER_COMMANDO,
    &FREESTRIDER_LOOKOUT,
    &FULL_STEAM_AHEAD,
    &GIANT_BEAVER,
    &GOLD_RUSH,
    &GOLDVEIN_HYDRA,
    &HARDBRISTLE_BANDIT,
    &INTREPID_STABLEMASTER,
    &MAP_THE_FRONTIER,
    &ORNERY_TUMBLEWAGG,
    &OUTCASTER_GREENBLADE,
    &OUTCASTER_TRAILBLAZER,
    &PATIENT_NATURALIST,
    &RAILWAY_BRAWLER,
    &RAMBLING_POSSUM,
    &RAUCOUS_ENTERTAINER,
    &REACH_FOR_THE_SKY,
    &RISE_OF_THE_VARMINTS,
    &SMUGGLER_S_SURPRISE,
    &SPINEWOODS_ARMADILLO,
    &SPINEWOODS_PALADIN,
    &STUBBORN_BURROWFIEND,
    &THROW_FROM_THE_SADDLE,
    &TRASH_THE_TOWN,
    &TUMBLEWEED_RISING,
    &VORACIOUS_VARMINT,
    &AKUL_THE_UNREPENTANT,
    &ANNIE_FLASH_THE_VETERAN,
    &ANNIE_JOINS_UP,
    &ASSIMILATION_AEGIS,
    &AT_KNIFEPOINT,
    &BADLANDS_REVIVAL,
    &BARON_BERTRAM_GRAYWATER,
    &BONNY_PALL_CLEARCUTTER,
    &BREECHES_THE_BLASTMAKER,
    &BRUSE_TARL_ROVING_RANCHER,
    &CACTUSFOLK_SURESHOT,
    &CONGREGATION_GRYFF,
    &DOC_AURLOCK_GRIZZLED_GENIUS,
    &ERIETTE_THE_BEGUILER,
    &ERTHA_JO_FRONTIER_MENTOR,
    &FORM_A_POSSE,
    &GHIRED_MIRROR_OF_THE_WILDS,
    &THE_GITROG_RAVENOUS_RIDE,
    &HONEST_RUTSTEIN,
    &INTIMIDATION_CAMPAIGN,
    &JEM_LIGHTFOOTE_SKY_EXPLORER,
    &JOLENE_PLUNDERING_PUGILIST,
    &KAMBAL_PROFITEERING_MAYOR,
    &KELLAN_JOINS_UP,
    &KELLAN_THE_KID,
    &KRAUM_VIOLENT_CACOPHONY,
    &LAUGHING_JASPER_FLINT,
    &LAZAV_FAMILIAR_STRANGER,
    &LILAH_UNDEFEATED_SLICKSHOT,
    &MAKE_YOUR_OWN_LUCK,
    &MALCOLM_THE_EYES,
    &MARCHESA_DEALER_OF_DEATH,
    &MIRIAM_HERD_WHISPERER,
    &OBEKA_SPLITTER_OF_SECONDS,
    &OKO_THE_RINGLEADER,
    &PILLAGE_THE_BOG,
    &RAKDOS_JOINS_UP,
    &RAKDOS_THE_MUSCLE,
    &RIKU_OF_MANY_PATHS,
    &ROXANNE_STARFALL_SAVANT,
    &RUTHLESS_LAWBRINGER,
    &SATORU_THE_INFILTRATOR,
    &SELVALA_EAGER_TRAILBLAZER,
    &SERAPHIC_STEED,
    &SLICK_SEQUENCE,
    &TAII_WAKEEN_PERFECT_SHOT,
    &VIAL_SMASHER_GLEEFUL_GRENADIER,
    &VRASKA_JOINS_UP,
    &VRASKA_THE_SILENCER,
    &WRANGLER_OF_THE_DAMNED,
    &WYLIE_DUKE_ATIIN_HERO,
    &BANDIT_S_HAUL,
    &BOOM_BOX,
    &GOLD_PAN,
    &LAVASPUR_BOOTS,
    &LUXURIOUS_LOCOMOTIVE,
    &MOBILE_HOMESTEAD,
    &OASIS_GARDENER,
    &REDROCK_SENTINEL,
    &SILVER_DEPUTY,
    &STERLING_HOUND,
    &TOMB_TRAWLER,
    &ABRADED_BLUFFS,
    &ARID_ARCHWAY,
    &BRISTLING_BACKWOODS,
    &CONDUIT_PYLONS,
    &CREOSOTE_HEATH,
    &ERODED_CANYON,
    &FESTERING_GULCH,
    &FORLORN_FLATS,
    &JAGGED_BARRENS,
    &LONELY_ARROYO,
    &LUSH_OASIS,
    &MIRAGE_MESA,
    &SANDSTORM_VERGE,
    &SOURED_SPRINGS,
    &BUCOLIC_RANCH,
    &JACE_REAWAKENED,
    &SLICKSHOT_SHOW_OFF,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ARCHANGEL_OF_TITHES_REPRINT,
    TAKE_UP_THE_SHIELD_REPRINT,
    CORRUPTED_CONVICTION_REPRINT,
    FAKE_YOUR_OWN_DEATH_REPRINT,
    SKULDUGGERY_REPRINT,
    SLICKSHOT_SHOW_OFF_ALTERNATE_1,
    TERROR_OF_THE_PEAKS_REPRINT,
    SNAKESKIN_VEIL_REPRINT,
    BLOOMING_MARSH_REPRINT,
    BOTANICAL_SANCTUM_REPRINT,
    CONCEALED_COURTYARD_REPRINT,
    INSPIRING_VANTAGE_REPRINT,
    SPIREBLUFF_CANAL_REPRINT,
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
    GERALF_THE_FLESHWRIGHT_ALTERNATE_1,
    GISA_THE_HELLRAISER_ALTERNATE_1,
    KAERVEK_THE_PUNISHER_ALTERNATE_1,
    TINYBONES_THE_PICKPOCKET_ALTERNATE_1,
    ANNIE_FLASH_THE_VETERAN_ALTERNATE_1,
    BREECHES_THE_BLASTMAKER_ALTERNATE_1,
    ERIETTE_THE_BEGUILER_ALTERNATE_1,
    KELLAN_THE_KID_ALTERNATE_1,
    MALCOLM_THE_EYES_ALTERNATE_1,
    OKO_THE_RINGLEADER_ALTERNATE_1,
    RAKDOS_THE_MUSCLE_ALTERNATE_1,
    SATORU_THE_INFILTRATOR_ALTERNATE_1,
    VRASKA_THE_SILENCER_ALTERNATE_1,
    BLOOMING_MARSH_ALTERNATE_1,
    BOTANICAL_SANCTUM_ALTERNATE_1,
    CONCEALED_COURTYARD_ALTERNATE_1,
    INSPIRING_VANTAGE_ALTERNATE_1,
    SPIREBLUFF_CANAL_ALTERNATE_1,
    OKO_THE_RINGLEADER_ALTERNATE_2,
    JACE_REAWAKENED_ALTERNATE_1,
    ANOTHER_ROUND_ALTERNATE_1,
    ARCHANGEL_OF_TITHES_ALTERNATE_1,
    AVEN_INTERRUPTER_ALTERNATE_1,
    CLAIM_JUMPER_ALTERNATE_1,
    DUST_ANIMUS_ALTERNATE_1,
    FINAL_SHOWDOWN_ALTERNATE_1,
    FORTUNE_LOYAL_STEED_ALTERNATE_1,
    HIGH_NOON_ALTERNATE_1,
    ONE_LAST_JOB_ALTERNATE_1,
    ARCHMAGE_S_NEWT_ALTERNATE_1,
    DOUBLE_DOWN_ALTERNATE_1,
    DUELIST_OF_THE_MIND_ALTERNATE_1,
    FBLTHP_LOST_ON_THE_RANGE_ALTERNATE_1,
    THE_KEY_TO_THE_VAULT_ALTERNATE_1,
    STEP_BETWEEN_WORLDS_ALTERNATE_1,
    STOIC_SPHINX_ALTERNATE_1,
    THREE_STEPS_AHEAD_ALTERNATE_1,
    CAUSTIC_BRONCO_ALTERNATE_1,
    INSATIABLE_AVARICE_ALTERNATE_1,
    PITILESS_CARNAGE_ALTERNATE_1,
    RUSH_OF_DREAD_ALTERNATE_1,
    TINYBONES_JOINS_UP_ALTERNATE_1,
    VADMIR_NEW_BLOOD_ALTERNATE_1,
    CALAMITY_GALLOPING_INFERNO_ALTERNATE_1,
    GREAT_TRAIN_HEIST_ALTERNATE_1,
    HELL_TO_PAY_ALTERNATE_1,
    HELLSPUR_POSSE_BOSS_ALTERNATE_1,
    MAGDA_THE_HOARDMASTER_ALTERNATE_1,
    STINGERBACK_TERROR_ALTERNATE_1,
    TERROR_OF_THE_PEAKS_ALTERNATE_1,
    BRISTLY_BILL_SPINE_SOWER_ALTERNATE_1,
    COLOSSAL_RATTLEWURM_ALTERNATE_1,
    FREESTRIDER_LOOKOUT_ALTERNATE_1,
    GOLDVEIN_HYDRA_ALTERNATE_1,
    ORNERY_TUMBLEWAGG_ALTERNATE_1,
    OUTCASTER_TRAILBLAZER_ALTERNATE_1,
    RAILWAY_BRAWLER_ALTERNATE_1,
    SMUGGLER_S_SURPRISE_ALTERNATE_1,
    AKUL_THE_UNREPENTANT_ALTERNATE_1,
    ANNIE_JOINS_UP_ALTERNATE_1,
    ASSIMILATION_AEGIS_ALTERNATE_1,
    BONNY_PALL_CLEARCUTTER_ALTERNATE_1,
    BRUSE_TARL_ROVING_RANCHER_ALTERNATE_1,
    GHIRED_MIRROR_OF_THE_WILDS_ALTERNATE_1,
    THE_GITROG_RAVENOUS_RIDE_ALTERNATE_1,
    KAMBAL_PROFITEERING_MAYOR_ALTERNATE_1,
    KELLAN_JOINS_UP_ALTERNATE_1,
    LAUGHING_JASPER_FLINT_ALTERNATE_1,
    LILAH_UNDEFEATED_SLICKSHOT_ALTERNATE_1,
    MARCHESA_DEALER_OF_DEATH_ALTERNATE_1,
    OBEKA_SPLITTER_OF_SECONDS_ALTERNATE_1,
    PILLAGE_THE_BOG_ALTERNATE_1,
    RAKDOS_JOINS_UP_ALTERNATE_1,
    RIKU_OF_MANY_PATHS_ALTERNATE_1,
    ROXANNE_STARFALL_SAVANT_ALTERNATE_1,
    SELVALA_EAGER_TRAILBLAZER_ALTERNATE_1,
    SERAPHIC_STEED_ALTERNATE_1,
    TAII_WAKEEN_PERFECT_SHOT_ALTERNATE_1,
    VRASKA_JOINS_UP_ALTERNATE_1,
    WYLIE_DUKE_ATIIN_HERO_ALTERNATE_1,
    FRONTIER_SEEKER_ALTERNATE_1,
    SCORCHING_SHOT_ALTERNATE_1,
    HONEST_RUTSTEIN_ALTERNATE_1,
    MAKE_YOUR_OWN_LUCK_ALTERNATE_1,
    RUTHLESS_LAWBRINGER_ALTERNATE_1,
    THE_KEY_TO_THE_VAULT_ALTERNATE_2,
    MAGDA_THE_HOARDMASTER_ALTERNATE_2,
];
