//! Lorwyn Eclipsed card inventory.

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
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorChoiceOperationDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostAdjustmentDef;
use crate::card::CostAmountDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamagePreventionDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::LookAtObjectsDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ManaTypeDef;
use crate::card::ManaTypeSetDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
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
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::ScaledValueDef;
use crate::card::SpellCostConditionDef;
use crate::card::SpellCostModificationDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenCopyDef;
use crate::card::TokenDef;
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
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2006::dissension as catalog_dis;
use crate::card::sets::y2006::guildpact as catalog_gpt;
use crate::card::sets::y2007::lorwyn as catalog_lrw;
use crate::card::sets::y2008::eventide as catalog_eve;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2019::modern_horizons as catalog_mh1;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2023::march_of_the_machine as catalog_mom;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ECL",
    slug: "lorwyn-eclipsed",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

/// Vivid counts represented colors once each, regardless of the number of permanents.
const VIVID: ValueDef = ValueDef::Sum(&SumValueDef {
    left: ValueDef::Sum(&SumValueDef {
        left: ValueDef::Sum(&SumValueDef {
            left: ValueDef::Sum(&SumValueDef {
                left: ValueDef::AnyMatchingObject(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::White),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                right: ValueDef::AnyMatchingObject(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            }),
            right: ValueDef::AnyMatchingObject(&ObjectQueryDef::matching(
                ObjectPredicateDef::Color(ManaColor::Black),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        }),
        right: ValueDef::AnyMatchingObject(&ObjectQueryDef::matching(
            ObjectPredicateDef::Color(ManaColor::Red),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        )),
    }),
    right: ValueDef::AnyMatchingObject(&ObjectQueryDef::matching(
        ObjectPredicateDef::Color(ManaColor::Green),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
});

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("ac4384b7-853c-417d-b3b4-f54cd0b5d361", "Jeff Miracola"),
);

const KITHKIN_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Kithkin"], &[ManaColor::Green, ManaColor::White], 1, 1)
        .with_art(CardArt::new(
            "2ed11e1b-2289-48d2-8d96-ee7e590ecfd4",
            "Jeff Laubenstein",
        ));
const FAERIE_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Faerie"], &[ManaColor::Blue, ManaColor::Black], 1, 1)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "01524db2-c96f-4902-8394-bc7a7128e573",
            "Iris Compiet",
        ));
const MERFOLK_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Merfolk"], &[ManaColor::White, ManaColor::Blue], 1, 1)
        .with_art(CardArt::new(
            "4c5ad4e1-b489-4023-88ab-1200c5f26ffc",
            "Julia Griffin",
        ));
const GOBLIN_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Goblin"], &[ManaColor::Black, ManaColor::Red], 1, 1)
        .with_art(CardArt::new(
            "6139a45d-ebc7-4bca-8c13-73c85ea5fe0d",
            "Larry MacDougall",
        ));
const ELF_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Elf"], &[ManaColor::Black, ManaColor::Green], 2, 2).with_art(
        CardArt::new("39b36f22-21f9-44fe-8a49-bdc859503342", "Pete Venters"),
    );
const TREEFOLK_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Treefolk"], &[ManaColor::Green], 3, 4)
        .with_abilities(&[abilities::reach()])
        .with_art(CardArt::new(
            "82e01706-ab45-4e52-9ee1-7070567234fd",
            "Jeff Laubenstein",
        ));

// ECL 1 — Changeling Wayfinder
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static CHANGELING_WAYFINDER: CardRecord = CardRecord::new(
    "Changeling Wayfinder",
    "6c6061aa-a4da-4115-85b6-d0aa22f2386c",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// ECL 2 — Rooftop Percher
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static ROOFTOP_PERCHER: CardRecord = CardRecord::new(
    "Rooftop Percher",
    "2d89595c-1542-41fe-8d23-997522922698",
    "Nils Hamm",
    CardRules::unsupported(),
);

// ECL 3 — Adept Watershaper
pub(in crate::card::sets) static ADEPT_WATERSHAPER: CardRecord = CardRecord::new(
    "Adept Watershaper",
    "6e53f246-8347-4632-9d5b-4aeb12f7b762",
    "Pauline Voss",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Merfolk", "Cleric"], 3, 4).with_abilities(&[
        AbilityDef::static_ability(
            "Other tapped creatures you control have indestructible.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Tapped,
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

// ECL 4 — Ajani, Outland Chaperone
pub(in crate::card::sets) static AJANI_OUTLAND_CHAPERONE: CardRecord = CardRecord::new(
    "Ajani, Outland Chaperone",
    "6124a691-ae83-4d22-a177-0aee65b47064",
    "Daren Bader",
    CardRules::new_planeswalker(mana_cost!("{1}{W}{W}"), &["Ajani"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Create a 1/1 green and white Kithkin creature token.",
                &[CostDef::Loyalty(1)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(KITHKIN_TOKEN))),
            ),
            AbilityDef::activated_with_targets(
                "−2: Ajani deals 4 damage to target tapped creature.",
                &[CostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
            AbilityDef::activated(
                "−8: Look at the top X cards of your library, where X is your \
                 life total. You may put any number of nonland permanent cards \
                 with mana value 3 or less from among them onto the \
                 battlefield. Then shuffle.",
                &[CostDef::Loyalty(-8)],
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::LifeTotal(PlayerRelation::You),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    minimum: 0,
                    maximum: 255,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::Controller,
                        },
                    ]),
                }),
            ),
        ]),
);

// ECL 5 — Appeal to Eirdu
pub(in crate::card::sets) static APPEAL_TO_EIRDU: CardRecord = CardRecord::new(
    "Appeal to Eirdu",
    "68ce9752-21f9-48e9-bf48-7f76f1cecbc5",
    "Milivoj Ćeran",
    CardRules::new_instant(mana_cost!("{3}{W}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell_with_targets(
            "One or two target creatures each get +2/+1 until end of turn.",
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
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 6 — Bark of Doran
pub(in crate::card::sets) static BARK_OF_DORAN: CardRecord = CardRecord::new(
    "Bark of Doran",
    "98210276-1b85-4db5-8ab4-ecb08f5d2ee2",
    "Jorge Jacinto",
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +0/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature's toughness is greater than its \
                 power, it assigns combat damage equal to its toughness rather \
                 than its power.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AttachedToSource,
                                ObjectPredicateDef::ToughnessGreaterThanItsPower,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::AssignsCombatDamageEqualToToughness,
                    ),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// ECL 7 — Brigid, Clachan's Heart // Brigid, Doun's Mind
pub(in crate::card::sets) static BRIGID_CLACHAN_S_HEART: CardRecord = CardRecord::new_dfc(
    "Brigid, Clachan's Heart // Brigid, Doun's Mind",
    "cb7d5bbb-4f68-4e38-8bb0-a95af21b24c8",
    "Zoltan Boros",
    &[
        (
            "Brigid, Clachan's Heart",
            CardRules::new_creature(mana_cost!("{2}{W}"), &["Kithkin", "Warrior"], 3, 2)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever this creature enters or transforms into Brigid, \
                         Clachan's Heart, create a 1/1 green and white Kithkin \
                         creature token.",
                        TriggerEventDef::AnyOf(&[
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            TriggerEventDef::Transforms(ObjectPredicateDef::Source),
                        ]),
                        EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                            KITHKIN_TOKEN,
                        ))),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {G}. \
                         If you do, transform Brigid.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{G}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
        (
            "Brigid, Doun's Mind",
            CardRules::new_creature_without_mana_cost(&["Kithkin", "Soldier"], 3, 2)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Green])
                .with_abilities(&[
                    AbilityDef::activated_mana(
                        "{T}: Add X {G} or X {W}, where X is the number of other \
                         creatures you control.",
                        &[CostDef::TapSource],
                        EffectDef::AddMana(
                            AddManaEffectDef::choice(&[ManaColor::Green, ManaColor::White])
                                .with_variable_amount(ValueDef::CountMatchingObjects(
                                    &ObjectQueryDef::matching(
                                        ObjectPredicateDef::All(&[
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                        ]),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    ),
                                )),
                        ),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {W}. \
                         If you do, transform Brigid.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{W}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
    ],
);

// ECL 8 — Burdened Stoneback
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static BURDENED_STONEBACK: CardRecord = CardRecord::new(
    "Burdened Stoneback",
    "3278b8d0-3d2b-4d3d-bbf1-fd9b714b53ed",
    "Carl Critchlow",
    CardRules::unsupported(),
);

// ECL 9 — Champion of the Clachan
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static CHAMPION_OF_THE_CLACHAN: CardRecord = CardRecord::new(
    "Champion of the Clachan",
    "46ce3474-381c-433b-acd8-4e628d0048d2",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// ECL 10 — Clachan Festival
pub(in crate::card::sets) static CLACHAN_FESTIVAL: CardRecord = CardRecord::new(
    "Clachan Festival",
    "324b5234-ffbf-4801-a475-8f693679ae2f",
    "Kev Fang",
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Kithkin"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this enchantment enters, create two 1/1 green and white \
                 Kithkin creature tokens.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(KITHKIN_TOKEN))
                        .with_count(ValueDef::Constant(2)),
                ),
            ),
            AbilityDef::activated(
                "{4}{W}: Create a 1/1 green and white Kithkin creature token.",
                &[CostDef::Mana(mana_cost!("{4}{W}"))],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(KITHKIN_TOKEN))),
            ),
        ])
        .with_type(CardType::Kindred),
);

// ECL 11 — Crib Swap (reprint)
const CRIB_SWAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::CRIB_SWAP,
    "8f2fb3c6-af75-47a3-9f97-521872c32890",
    "Pete Venters",
);

// ECL 12 — Curious Colossus
pub(in crate::card::sets) static CURIOUS_COLOSSUS: CardRecord = CardRecord::new(
    "Curious Colossus",
    "582b6e5d-0bab-471d-af4d-19438c5fd524",
    "Raoul Vitale",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Giant", "Warrior"], 7, 7).with_abilities(
        &[abilities::enters_trigger_with_targets(
            "When this creature enters, each creature target opponent \
             controls loses all abilities, becomes a Coward in addition to \
             its other types, and has base power and toughness 1/1.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Apply {
                duration: ResolvedEffectDurationDef::Permanent,
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::LegalTargets(TargetIndex(0)),
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Coward"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                ]),
            },
        )],
    ),
);

// ECL 13 — Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight
// Audit: unsupported — Needs a continuous convoke grant on prospective creature spells in every castable zone before payments are enumerated; applying an ability to a spell already on the stack is too late.
pub(in crate::card::sets) static EIRDU_CARRIER_OF_DAWN: CardRecord = CardRecord::new(
    "Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight",
    "b2d9d5ca-7e15-437a-bdfc-5972b42148fe",
    "Lucas Graciano",
    CardRules::unsupported(),
);

// ECL 14 — Encumbered Reejerey
pub(in crate::card::sets) static ENCUMBERED_REEJEREY: CardRecord = CardRecord::new(
    "Encumbered Reejerey",
    "15ff6797-f59c-4333-98ea-5711150fd5b8",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Merfolk", "Soldier"], 5, 4).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three -1/-1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes tapped while it has a -1/-1 \
             counter on it, remove a -1/-1 counter from it.",
            TriggerEventDef::While {
                event: &TriggerEventDef::tapped(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    comparison: ComparisonDef::Greater,
                    amount: 0,
                },
            },
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 15 — Evershrike's Gift
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static EVERSHRIKE_S_GIFT: CardRecord = CardRecord::new(
    "Evershrike's Gift",
    "231e46b6-b91a-4582-8894-e7de1c50213f",
    "Drew Tucker",
    CardRules::unsupported(),
);

// ECL 16 — Flock Impostor
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static FLOCK_IMPOSTOR: CardRecord = CardRecord::new(
    "Flock Impostor",
    "d32d0336-5140-41f9-bc67-f3d743b9231d",
    "Ilse Gort",
    CardRules::unsupported(),
);

// ECL 17 — Gallant Fowlknight
pub(in crate::card::sets) static GALLANT_FOWLKNIGHT: CardRecord = CardRecord::new(
    "Gallant Fowlknight",
    "fb6096ba-8083-4207-9a3f-c1e4ff095204",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Kithkin", "Knight"], 3, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, creatures you control get +1/+0 \
             until end of turn. Kithkin creatures you control also gain \
             first strike until end of turn.",
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
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// ECL 18 — Goldmeadow Nomad
pub(in crate::card::sets) static GOLDMEADOW_NOMAD: CardRecord = CardRecord::new(
    "Goldmeadow Nomad",
    "00ddbe6c-11de-4bc6-aabe-d6d8385a838a",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{W}"), &["Kithkin", "Scout"], 1, 2).with_abilities(&[
        AbilityDef::activated(
            "{W}, Exile this card from your graveyard: Create a 1/1 green \
             and white Kithkin creature token. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::ExileSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(KITHKIN_TOKEN))),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// ECL 19 — Keep Out
pub(in crate::card::sets) static KEEP_OUT: CardRecord = CardRecord::new(
    "Keep Out",
    "4ab1601c-634c-4f21-8926-ba3cb92008c1",
    "Ron Spencer",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Keep Out deals 4 damage to target tapped creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
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
        ],
    )]),
);

// ECL 20 — Kinbinding
// Audit: unsupported — Needs per-player creature-entry counts for the current turn, retained after those creatures leave or change controllers; current entry predicates cannot recover that history.
pub(in crate::card::sets) static KINBINDING: CardRecord = CardRecord::new(
    "Kinbinding",
    "b6e35fd5-de7e-40a8-a23c-00d07fd1ac56",
    "Caio Monteiro",
    CardRules::unsupported(),
);

// ECL 21 — Kinsbaile Aspirant
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static KINSBAILE_ASPIRANT: CardRecord = CardRecord::new(
    "Kinsbaile Aspirant",
    "56dfdab1-ea3f-4663-a855-a9e72505f85e",
    "Margaret Organ-Kean",
    CardRules::unsupported(),
);

// ECL 22 — Kinscaer Sentry
// Audit: unsupported — Needs a hand-to-battlefield move with tapped-and-attacking entry state; entering_attacking is supported for token creation, not for an existing card moved from hand.
pub(in crate::card::sets) static KINSCAER_SENTRY: CardRecord = CardRecord::new(
    "Kinscaer Sentry",
    "333bf101-14e8-4753-99bc-9174f42c4122",
    "Kev Fang",
    CardRules::unsupported(),
);

// ECL 23 — Kithkeeper
pub(in crate::card::sets) static KITHKEEPER: CardRecord = CardRecord::new(
    "Kithkeeper",
    "ef29eff7-72be-46e6-9275-0f0a44d29233",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{6}{W}"), &["Elemental"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "Vivid — When this creature enters, create X 1/1 green and \
             white Kithkin creature tokens, where X is the number of \
             colors among permanents you control.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(KITHKIN_TOKEN)).with_count(VIVID),
            ),
        ),
        AbilityDef::activated(
            "Tap three untapped creatures you control: This creature gets \
             +3/+0 and gains flying until end of turn.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 3,
            }],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 24 — Liminal Hold
// Audit: unsupported — Needs immediate return when an exile-until-source-leaves duration ends; a leaves trigger would return the permanent later through the stack.
pub(in crate::card::sets) static LIMINAL_HOLD: CardRecord = CardRecord::new(
    "Liminal Hold",
    "a5a40c16-7a5c-4ad1-be53-6b1b1be2affe",
    "Ovidio Cartagena",
    CardRules::unsupported(),
);

// ECL 25 — Meanders Guide
// Audit: unsupported — Needs a reflexive trigger after an optional tap with targets selected only after that tap; ordinary triggered targets are selected before the choice and payment.
pub(in crate::card::sets) static MEANDERS_GUIDE: CardRecord = CardRecord::new(
    "Meanders Guide",
    "8c41a0ad-138e-4eef-8f7f-35017e3b086f",
    "Julie Dillon",
    CardRules::unsupported(),
);

// ECL 26 — Moonlit Lamenter
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static MOONLIT_LAMENTER: CardRecord = CardRecord::new(
    "Moonlit Lamenter",
    "fb8fc509-cff6-470f-abf6-b07f6c3f94e1",
    "Steve Ellis",
    CardRules::unsupported(),
);

// ECL 27 — Morningtide's Light
pub(in crate::card::sets) static MORNINGTIDE_S_LIGHT: CardRecord = CardRecord::new(
    "Morningtide's Light",
    "181ee045-5650-479a-8c03-015b38fdcd63",
    "Mark Poole",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile any number of target creatures. At the beginning of the \
         next end step, return those cards to the battlefield tapped \
         under their owners' control.\nUntil your next turn, prevent \
         all damage that would be dealt to you.\nExile Morningtide's \
         Light.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            255,
        )],
        EffectDef::Sequence(&[
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("exiled"),
                then: &EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(
                        ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("exiled")),
                    ),
                    binding: crate::Binding!("returning"),
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "At the beginning of the next end step, return those cards to \
                             the battlefield tapped under their owners' control.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::End,
                                player: PlayerRelation::Any,
                            },
                            EffectDef::WithBattlefieldArrival {
                                effect: &EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("returning"),
                                    )),
                                    ZoneKind::Battlefield,
                                    ZonePlacement::Top,
                                ),
                                arrival: BattlefieldArrivalDef {
                                    modifications: &[BattlefieldEntryModificationDef::Tapped],
                                    ..BattlefieldArrivalDef::DEFAULT
                                },
                            },
                        ),
                    )),
                }),
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef {
                    kind: DamageKindDef::Any,
                    source: DamageSourceMatcherDef::Any,
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::Controller,
                    ),
                }),
                duration: ResolvedEffectDurationDef::UntilYourNextTurn,
            },
        ]),
    )
    .with_resolution_destination(SpellResolutionDestinationDef::Exile)]),
);

// ECL 28 — Personify
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static PERSONIFY: CardRecord = CardRecord::new(
    "Personify",
    "1172582d-fb2d-4022-95b1-e48b03df3a95",
    "Slawomir Maniak",
    CardRules::unsupported(),
);

// ECL 29 — Protective Response
pub(in crate::card::sets) static PROTECTIVE_RESPONSE: CardRecord = CardRecord::new(
    "Protective Response",
    "113975e1-7712-4760-96ad-405f8b4e41e3",
    "Gustavo Pelissari",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell_with_targets(
            "Destroy target attacking or blocking creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Attacking,
                        ObjectPredicateDef::Blocking,
                    ]),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// ECL 30 — Pyrrhic Strike
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static PYRRHIC_STRIKE: CardRecord = CardRecord::new(
    "Pyrrhic Strike",
    "cce5b16d-07fb-4e64-8ec9-b8b29ba86cff",
    "Randy Vargas",
    CardRules::unsupported(),
);

// ECL 31 — Reluctant Dounguard
pub(in crate::card::sets) static RELUCTANT_DOUNGUARD: CardRecord = CardRecord::new(
    "Reluctant Dounguard",
    "dbce93c0-5efc-4b60-9cef-9d9b374d397b",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Kithkin", "Soldier"], 4, 4).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with two -1/-1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control enters while this \
             creature has a -1/-1 counter on it, remove a -1/-1 counter \
             from this creature.",
            TriggerEventDef::While {
                event: &TriggerEventDef::zone_changed(
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
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    comparison: ComparisonDef::Greater,
                    amount: 0,
                },
            },
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 32 — Rhys, the Evermore
// Audit: unsupported — Needs an effect choosing any number of counters across arbitrary kinds on a target; RemoveAllCounters cannot represent choosing a subset and fixed-kind removal cannot mix kinds.
pub(in crate::card::sets) static RHYS_THE_EVERMORE: CardRecord = CardRecord::new(
    "Rhys, the Evermore",
    "a7072412-4aa2-40ef-a267-bd717551a42b",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// ECL 33 — Riverguard's Reflexes
pub(in crate::card::sets) static RIVERGUARD_S_REFLEXES: CardRecord = CardRecord::new(
    "Riverguard's Reflexes",
    "88a79f3e-ca74-467c-b0a8-22802ac5b465",
    "Lucas Graciano",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains first strike until end \
         of turn. Untap it.",
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
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

// ECL 34 — Shore Lurker
pub(in crate::card::sets) static SHORE_LURKER: CardRecord = CardRecord::new(
    "Shore Lurker",
    "bb353c27-e311-4677-9277-cab6820562ce",
    "Tiffany Turrill",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Merfolk", "Scout"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, surveil 1. (Look at the top card \
             of your library. You may put it into your graveyard.)",
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// ECL 35 — Slumbering Walker
// Audit: unsupported — Needs an optional payment removing a chosen counter of any kind and a reflexive reanimation trigger with targets selected after that payment; fixed-kind removal costs and ordinary upfront targets do not implement it.
pub(in crate::card::sets) static SLUMBERING_WALKER: CardRecord = CardRecord::new(
    "Slumbering Walker",
    "81e82915-e734-4754-829f-f5da6a7c550d",
    "Jakub Kasper",
    CardRules::unsupported(),
);

// ECL 36 — Spiral into Solitude
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static SPIRAL_INTO_SOLITUDE: CardRecord = CardRecord::new(
    "Spiral into Solitude",
    "e7a12664-a930-4159-8311-19862488fb05",
    "Drew Baker",
    CardRules::unsupported(),
);

// ECL 37 — Sun-Dappled Celebrant
pub(in crate::card::sets) static SUN_DAPPLED_CELEBRANT: CardRecord = CardRecord::new(
    "Sun-Dappled Celebrant",
    "91c715a5-6643-4064-a8a8-3e05dce15979",
    "Steve Ellis",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Treefolk", "Cleric"], 5, 6)
        .with_abilities(&[abilities::convoke(), abilities::vigilance()]),
);

// ECL 38 — Thoughtweft Imbuer
pub(in crate::card::sets) static THOUGHTWEFT_IMBUER: CardRecord = CardRecord::new(
    "Thoughtweft Imbuer",
    "7e585a6c-2d95-450e-98cb-8794d7d8ecca",
    "Ioannis Fiore",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Kithkin", "Advisor"], 0, 5).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control attacks alone, it gets +X/+X \
             until end of turn, where X is the number of Kithkin you \
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
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 39 — Timid Shieldbearer
pub(in crate::card::sets) static TIMID_SHIELDBEARER: CardRecord = CardRecord::new(
    "Timid Shieldbearer",
    "1c672d38-a1a6-4912-a9a6-b11e7bf0cc67",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kithkin", "Soldier"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{4}{W}: Creatures you control get +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{4}{W}"))],
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

// ECL 40 — Tributary Vaulter
pub(in crate::card::sets) static TRIBUTARY_VAULTER: CardRecord = CardRecord::new(
    "Tributary Vaulter",
    "a1db2c83-41d1-4cc6-b9a3-fed504b01127",
    "Tiffany Turrill",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Merfolk", "Warrior"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature becomes tapped, another target Merfolk \
             you control gets +2/+0 until end of turn.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
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
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 41 — Wanderbrine Preacher
pub(in crate::card::sets) static WANDERBRINE_PREACHER: CardRecord = CardRecord::new(
    "Wanderbrine Preacher",
    "3fc3f5f2-5a83-4358-8f23-42f26f345140",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Merfolk", "Cleric"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, you gain 2 life.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ECL 42 — Wanderbrine Trapper
pub(in crate::card::sets) static WANDERBRINE_TRAPPER: CardRecord = CardRecord::new(
    "Wanderbrine Trapper",
    "f4c134ed-adbf-4e88-80e0-75c176ce94c3",
    "Iris Compiet",
    CardRules::new_creature(mana_cost!("{W}"), &["Merfolk", "Scout"], 2, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}, Tap another untapped creature you control: Tap \
             target creature an opponent controls.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::TapPermanents {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                    count: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// ECL 43 — Winnowing
// Audit: unsupported — Needs each player's creatures compared with that player's chosen creature type set, followed by one simultaneous sacrifice of all nonmatching creatures; current cross-object predicates do not compare creature-type intersections.
pub(in crate::card::sets) static WINNOWING: CardRecord = CardRecord::new(
    "Winnowing",
    "f943a7d8-9550-427e-8c45-ef834329d345",
    "David Palumbo",
    CardRules::unsupported(),
);

// ECL 44 — Aquitect's Defenses
pub(in crate::card::sets) static AQUITECT_S_DEFENSES: CardRecord = CardRecord::new(
    "Aquitect's Defenses",
    "9af9a907-fd2e-4ae0-ac7b-529074b79a14",
    "Ioannis Fiore",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature_you_control(),
            abilities::enters_trigger(
                "When this Aura enters, enchanted creature gains hexproof \
                 until end of turn. (It can't be the target of spells or \
                 abilities your opponents control.)",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
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

// ECL 45 — Blossombind
// Audit: unsupported — Needs a permanent-wide prohibition on every untap and on counters of every kind being placed; skipping the untap step does not stop untap spells or abilities.
pub(in crate::card::sets) static BLOSSOMBIND: CardRecord = CardRecord::new(
    "Blossombind",
    "382b83c0-bbbc-4db8-bc04-cfea79aed1b3",
    "Drew Tucker",
    CardRules::unsupported(),
);

// ECL 46 — Champions of the Shoal
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static CHAMPIONS_OF_THE_SHOAL: CardRecord = CardRecord::new(
    "Champions of the Shoal",
    "e1acdd9c-4a6d-4373-950c-f5539b54679f",
    "Daniel Zrom",
    CardRules::unsupported(),
);

// ECL 47 — Disruptor of Currents
pub(in crate::card::sets) static DISRUPTOR_OF_CURRENTS: CardRecord = CardRecord::new(
    "Disruptor of Currents",
    "4c6dbbaa-6844-4d7c-abbb-472a83bb99ab",
    "Pauline Voss",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Merfolk", "Wizard"], 3, 3).with_abilities(
        &[
            abilities::flash(),
            abilities::convoke(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, return up to one other target \
                 nonland permanent to its owner's hand.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
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
        ],
    ),
);

// ECL 48 — Flitterwing Nuisance
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static FLITTERWING_NUISANCE: CardRecord = CardRecord::new(
    "Flitterwing Nuisance",
    "ad0f6536-5295-4835-8883-35d711dfe6de",
    "Evyn Fong",
    CardRules::unsupported(),
);

// ECL 49 — Glamer Gifter
pub(in crate::card::sets) static GLAMER_GIFTER: CardRecord = CardRecord::new(
    "Glamer Gifter",
    "bd764dd4-2395-4138-ac29-260eac1aeaae",
    "Ben Hill",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie", "Wizard"], 1, 2).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, choose up to one other target \
             creature. Until end of turn, that creature has base power and \
             toughness 4/4 and gains all creature types.",
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
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::ALL),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 50 — Glamermite
pub(in crate::card::sets) static GLAMERMITE: CardRecord = CardRecord::new(
    "Glamermite",
    "b8b7c23a-0034-453c-ab44-f6ec0f31d1eb",
    "Pauline Voss",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Faerie", "Rogue"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Tap target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Untap target creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ),
            ],
        ),
    ]),
);

// ECL 51 — Glen Elendra Guardian
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static GLEN_ELENDRA_GUARDIAN: CardRecord = CardRecord::new(
    "Glen Elendra Guardian",
    "388d2e4a-0aa5-4b82-a86c-4777ca60161c",
    "Yohann Schepacz",
    CardRules::unsupported(),
);

// ECL 52 — Glen Elendra's Answer
// Audit: unsupported — Needs one simultaneous counter operation over an opponent's spells and abilities with a successful-counter receipt for token count; individual Counter targets do not expose that aggregate outcome.
pub(in crate::card::sets) static GLEN_ELENDRA_S_ANSWER: CardRecord = CardRecord::new(
    "Glen Elendra's Answer",
    "fa5bfbf9-dca2-42b7-a431-f9afedb54528",
    "Sam Guay",
    CardRules::unsupported(),
);

// ECL 53 — Gravelgill Scoundrel
pub(in crate::card::sets) static GRAVELGILL_SCOUNDREL: CardRecord = CardRecord::new(
    "Gravelgill Scoundrel",
    "320bc30c-7a8a-411a-9f36-9c69126e131b",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Rogue"], 1, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature attacks, you may tap another untapped \
             creature you control. If you do, this creature can't be \
             blocked this turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("chosen")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Tap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                },
            }),
        ),
    ]),
);

// ECL 54 — Harmonized Crescendo
// Audit: unsupported — Needs a resolution-time creature-type choice bound to the following count; entry-time scalar choices cannot be used by a resolving instant.
pub(in crate::card::sets) static HARMONIZED_CRESCENDO: CardRecord = CardRecord::new(
    "Harmonized Crescendo",
    "2715e0c0-9913-4bea-9a42-ad1164f6130a",
    "Tyler Walpole",
    CardRules::unsupported(),
);

// ECL 55 — Illusion Spinners
// Audit: unsupported — Needs a conditional flash permission evaluated in prospective nonbattlefield casting context; battlefield keyword grants do not alter when a card in hand may be cast.
pub(in crate::card::sets) static ILLUSION_SPINNERS: CardRecord = CardRecord::new(
    "Illusion Spinners",
    "eb4229a9-8df4-4adc-9d3e-acd2221fa3e9",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// ECL 56 — Kulrath Mystic
pub(in crate::card::sets) static KULRATH_MYSTIC: CardRecord = CardRecord::new(
    "Kulrath Mystic",
    "377d257c-920c-4dd4-a4b1-01cbc631ef8f",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental", "Wizard"], 2, 4).with_abilities(
        &[AbilityDef::triggered(
            "Whenever you cast a spell with mana value 4 or greater, this \
             creature gets +2/+0 and gains vigilance until end of turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )],
    ),
);

// ECL 57 — Loch Mare
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static LOCH_MARE: CardRecord = CardRecord::new(
    "Loch Mare",
    "ad6c4baf-a803-45e4-81ac-708a41631a28",
    "Chris Rahn",
    CardRules::unsupported(),
);

// ECL 58 — Lofty Dreams
pub(in crate::card::sets) static LOFTY_DREAMS: CardRecord = CardRecord::new(
    "Lofty Dreams",
    "a2908ed1-517e-4f1a-94e5-b06fb033c1a6",
    "Steven Belledin",
    CardRules::new_enchantment(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::convoke(),
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, draw a card.",
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
        ]),
);

// ECL 59 — Mirrorform
pub(in crate::card::sets) static MIRRORFORM: CardRecord = CardRecord::new(
    "Mirrorform",
    "55d30256-f5d8-4f61-a3f5-878970ced6d1",
    "Wayne Reynolds",
    CardRules::new_instant(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Each nonland permanent you control becomes a copy of target \
             non-Aura permanent.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura"))),
            )],
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                copier: Some(EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                ))),
                exceptions: CopyExceptionsDef::NONE,
                duration: None,
            },
        ),
    ]),
);

// ECL 60 — Noggle the Mind
pub(in crate::card::sets) static NOGGLE_THE_MIND: CardRecord = CardRecord::new(
    "Noggle the Mind",
    "076c0ac1-722e-49ef-b815-ace210069972",
    "Thomas M. Baxa",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature loses all abilities and is a colorless \
                 Noggle with base power and toughness 1/1. (It loses all \
                 colors and all other creature types.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Noggle",
                        ])),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    ]),
                },
            ),
        ]),
);

// ECL 61 — Oko, Lorwyn Liege // Oko, Shadowmoor Scion
// Audit: unsupported — Needs a resolution-time creature-type choice stored on a newly created emblem and used by its static predicate; entry choices on permanents do not create that emblem binding.
pub(in crate::card::sets) static OKO_LORWYN_LIEGE: CardRecord = CardRecord::new(
    "Oko, Lorwyn Liege // Oko, Shadowmoor Scion",
    "1dab370a-1067-4d94-be1f-10362d4abf5a",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// ECL 62 — Omni-Changeling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static OMNI_CHANGELING: CardRecord = CardRecord::new(
    "Omni-Changeling",
    "f29ce8f9-42a3-43fa-8197-666cc26e2c76",
    "Jeff Laubenstein",
    CardRules::unsupported(),
);

// ECL 63 — Pestered Wellguard
pub(in crate::card::sets) static PESTERED_WELLGUARD: CardRecord = CardRecord::new(
    "Pestered Wellguard",
    "3e06c99e-ecb2-42e9-ac58-2542de8d54a5",
    "Julie Dillon",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Merfolk", "Soldier"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature becomes tapped, create a 1/1 blue and \
             black Faerie creature token with flying.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(FAERIE_TOKEN))),
        ),
    ]),
);

// ECL 64 — Rime Chill
pub(in crate::card::sets) static RIME_CHILL: CardRecord = CardRecord::new(
    "Rime Chill",
    "a9a425f4-2103-4f96-88a0-91fe554037d7",
    "Igor Krstic",
    CardRules::new_instant(mana_cost!("{6}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Vivid — This spell costs {1} less to cast for each color \
             among permanents you control.",
            EffectDef::ReduceGenericCostBy(ValueDef::Sum(&SumValueDef {
                left: ValueDef::Sum(&SumValueDef {
                    left: ValueDef::Sum(&SumValueDef {
                        left: ValueDef::Sum(&SumValueDef {
                            left: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::Color(ManaColor::White),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::Greater,
                                amount: 0,
                                then: ValueDef::Constant(1),
                                otherwise: ValueDef::Constant(0),
                            }),
                            right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::Color(ManaColor::Blue),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::Greater,
                                amount: 0,
                                then: ValueDef::Constant(1),
                                otherwise: ValueDef::Constant(0),
                            }),
                        }),
                        right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Color(ManaColor::Black),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                            then: ValueDef::Constant(1),
                            otherwise: ValueDef::Constant(0),
                        }),
                    }),
                    right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Color(ManaColor::Red),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::Greater,
                        amount: 0,
                        then: ValueDef::Constant(1),
                        otherwise: ValueDef::Constant(0),
                    }),
                }),
                right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Color(ManaColor::Green),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::Greater,
                    amount: 0,
                    then: ValueDef::Constant(1),
                    otherwise: ValueDef::Constant(0),
                }),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell_with_targets(
            "Tap up to two target creatures. Put a stun counter on each of \
             them. (If a permanent with a stun counter would become \
             untapped, remove one from it instead.)\nDraw a card.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
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
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// ECL 65 — Rimefire Torque
// Audit: unsupported — Needs an independent next-matching-cast trigger that also expires at this turn's cleanup; the installed-trigger lifetime can express either one-shot or this-turn, not both together.
pub(in crate::card::sets) static RIMEFIRE_TORQUE: CardRecord = CardRecord::new(
    "Rimefire Torque",
    "4f8931b2-ff3f-4167-8a40-f33460c2d27e",
    "Jorge Jacinto",
    CardRules::unsupported(),
);

// ECL 66 — Rimekin Recluse
pub(in crate::card::sets) static RIMEKIN_RECLUSE: CardRecord = CardRecord::new(
    "Rimekin Recluse",
    "ba6b5368-3262-4002-bf1e-fce62f7f7901",
    "Aurore Folny",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental", "Wizard"], 3, 2).with_abilities(
        &[abilities::enters_trigger_with_targets(
            "When this creature enters, return up to one other target \
             creature to its owner's hand.",
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
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )],
    ),
);

// ECL 67 — Run Away Together (reprint)
const RUN_AWAY_TOGETHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::RUN_AWAY_TOGETHER,
    "35c56aff-1f0f-464a-b705-d67803e3d060",
    "Annie Stegg",
);

// ECL 68 — Shinestriker
pub(in crate::card::sets) static SHINESTRIKER: CardRecord = CardRecord::new(
    "Shinestriker",
    "214e78f9-5364-49ab-b17e-0c76549c583e",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Elemental"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "Vivid — When this creature enters, draw cards equal to the \
             number of colors among permanents you control.",
            abilities::draw_cards(VIVID),
        ),
    ]),
);

// ECL 69 — Silvergill Mentor
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static SILVERGILL_MENTOR: CardRecord = CardRecord::new(
    "Silvergill Mentor",
    "e6e37fe8-459c-4992-8ae9-f782cddab2fe",
    "Iris Compiet",
    CardRules::unsupported(),
);

// ECL 70 — Silvergill Peddler
pub(in crate::card::sets) static SILVERGILL_PEDDLER: CardRecord = CardRecord::new(
    "Silvergill Peddler",
    "feba2bb6-6005-4ade-a4c5-97bbd54b43a3",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Citizen"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
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
        ),
    ]),
);

// ECL 71 — Spell Snare (reprint)
const SPELL_SNARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::SPELL_SNARE,
    "b7551b61-656e-4f37-b9da-73174db983b7",
    "Iris Compiet",
);

// ECL 72 — Stratosoarer
pub(in crate::card::sets) static STRATOSOARER: CardRecord = CardRecord::new(
    "Stratosoarer",
    "4f607889-6f3f-4511-8920-88a39f3b28ca",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature gains flying until \
             end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::typecycling!(
            "Basic landcycling {1}{U} ({1}{U}, Discard this card: Search \
             your library for a basic land card, reveal it, put it into \
             your hand, then shuffle.)",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            ObjectPredicateDef::Supertype(CardSupertype::Basic)
        ),
    ]),
);

// ECL 73 — Summit Sentinel
pub(in crate::card::sets) static SUMMIT_SENTINEL: CardRecord = CardRecord::new(
    "Summit Sentinel",
    "81251057-f270-4f05-9dc5-205c70e1f295",
    "Jake Murray",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Elemental", "Soldier"], 1, 3).with_abilities(
        &[abilities::dies_trigger(
            "When this creature dies, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        )],
    ),
);

// ECL 74 — Sunderflock
// Audit: unsupported — Needs a self-cost value computing the greatest mana value among controlled Elementals; AggregateObjectValues exists for resolving effects but the cost-reduction evaluator does not evaluate aggregates.
pub(in crate::card::sets) static SUNDERFLOCK: CardRecord = CardRecord::new(
    "Sunderflock",
    "e5b6221e-cb22-45e4-bb98-2b960afc614c",
    "Caio Monteiro",
    CardRules::unsupported(),
);

// ECL 75 — Swat Away
// Audit: unsupported — Needs a self spell-cost condition detecting whether an attacker is attacking the caster specifically; ordinary Attacking predicates include attacks on another player or planeswalker.
pub(in crate::card::sets) static SWAT_AWAY: CardRecord = CardRecord::new(
    "Swat Away",
    "2fb0ea3f-2f6d-4b64-a9d7-e822c8854a03",
    "Julie Dillon",
    CardRules::unsupported(),
);

// ECL 76 — Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield
pub(in crate::card::sets) static SYGG_WANDERWINE_WISDOM: CardRecord = CardRecord::new_dfc(
    "Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield",
    "70adc870-f0db-4d4b-863b-673c2c258751",
    "Justin Gerard",
    &[
        (
            "Sygg, Wanderwine Wisdom",
            CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Wizard"], 2, 2)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::static_ability(
                        "Sygg can't be blocked.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        },
                    ),
                    AbilityDef::triggered_with_targets(
                        "Whenever this creature enters or transforms into Sygg, \
                         Wanderwine Wisdom, target creature gains \"Whenever this \
                         creature deals combat damage to a player or planeswalker, \
                         draw a card\" until end of turn.",
                        TriggerEventDef::AnyOf(&[
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            TriggerEventDef::Transforms(ObjectPredicateDef::Source),
                        ]),
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                                "Whenever this creature deals combat damage to a player or \
                                 planeswalker, draw a card.",
                                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                                    source: DamageSourceMatcherDef::Matching(
                                        ObjectPredicateDef::Source,
                                    ),
                                    recipient: DamageRecipientMatcherDef::PlayerOrPlaneswalker,
                                    kind: DamageKindDef::Combat,
                                    ..DamageEventMatcherDef::ANY
                                }),
                                abilities::draw_cards(ValueDef::Constant(1)),
                            )),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {W}. \
                         If you do, transform Sygg.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{W}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
        (
            "Sygg, Wanderbrine Shield",
            CardRules::new_creature_without_mana_cost(&["Merfolk", "Rogue"], 2, 2)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::White])
                .with_abilities(&[
                    AbilityDef::static_ability(
                        "Sygg can't be blocked.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        },
                    ),
                    AbilityDef::triggered_with_targets(
                        "Whenever this creature transforms into Sygg, Wanderbrine \
                         Shield, target creature you control gains protection from \
                         each color until your next turn.",
                        TriggerEventDef::Transforms(ObjectPredicateDef::Source),
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
                            effect: AppliedEffectDef::add_ability(&AbilityDef::keyword(
                                "Protection from each color",
                                KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Not(
                                    &ObjectPredicateDef::ColorCount(0),
                                )),
                            )),
                            duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                        },
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {U}. \
                         If you do, transform Sygg.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{U}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
    ],
);

// ECL 77 — Tanufel Rimespeaker
pub(in crate::card::sets) static TANUFEL_RIMESPEAKER: CardRecord = CardRecord::new(
    "Tanufel Rimespeaker",
    "b357022c-7cd5-4e82-a183-7144f5a84102",
    "Lauren K. Cannon",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Elemental", "Wizard"], 2, 4).with_abilities(
        &[AbilityDef::triggered(
            "Whenever you cast a spell with mana value 4 or greater, draw \
             a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::draw_cards(ValueDef::Constant(1)),
        )],
    ),
);

// ECL 78 — Temporal Cleansing (reprint)
const TEMPORAL_CLEANSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mom::TEMPORAL_CLEANSING,
    "50ee7315-ec53-43d2-841e-8ec192b850f1",
    "Wylie Beckert",
);

// ECL 79 — Thirst for Identity
pub(in crate::card::sets) static THIRST_FOR_IDENTITY: CardRecord = CardRecord::new(
    "Thirst for Identity",
    "c3949f8c-d1c5-45c2-80ed-a57f4f9af86e",
    "Danny Schwartz",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw three cards. Then discard two cards unless you discard a \
         creature card.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(3)),
            EffectDef::PayOr(PayOrDef::optional_or(
                &[CostDef::discard(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))],
                &EffectDef::None,
                &EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            )),
        ]),
    )]),
);

// ECL 80 — Unexpected Assistance
pub(in crate::card::sets) static UNEXPECTED_ASSISTANCE: CardRecord = CardRecord::new(
    "Unexpected Assistance",
    "7b540a5f-7f1b-421a-864f-5af469556fc6",
    "Gustavo Pelissari",
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell(
            "Draw three cards, then discard a card.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(3)),
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

// ECL 81 — Unwelcome Sprite
pub(in crate::card::sets) static UNWELCOME_SPRITE: CardRecord = CardRecord::new(
    "Unwelcome Sprite",
    "902f8d86-fde2-4cdf-88f0-bf63d616f3af",
    "Iris Compiet",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you cast a spell during an opponent's turn, surveil \
             2. (Look at the top two cards of your library. You may put \
             any number of them into your graveyard and the rest on top of \
             your library in any order.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
            },
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// ECL 82 — Wanderwine Distracter
pub(in crate::card::sets) static WANDERWINE_DISTRACTER: CardRecord = CardRecord::new(
    "Wanderwine Distracter",
    "cbf593a7-d4ae-4771-926a-3c1b2c8c901a",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Merfolk", "Wizard"], 4, 3).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature becomes tapped, target creature an \
             opponent controls gets -3/-0 until end of turn.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
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
                    ValueDef::Constant(-3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 83 — Wanderwine Farewell
pub(in crate::card::sets) static WANDERWINE_FAREWELL: CardRecord = CardRecord::new(
    "Wanderwine Farewell",
    "473123a0-1366-406b-9b9e-154c0e9c224f",
    "Aldo Domínguez",
    CardRules::new_sorcery(mana_cost!("{5}{U}{U}"))
        .with_subtypes(&["Merfolk"])
        .with_abilities(&[
            abilities::convoke(),
            AbilityDef::spell_with_targets(
                "Return one or two target nonland permanents to their owners' \
                 hands. Then if you control a Merfolk, create a 1/1 white and \
                 blue Merfolk creature token for each permanent returned to \
                 its owner's hand this way.",
                &[AbilityTargetDef {
                    minimum: 1,
                    ..AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                        2,
                    )
                }],
                EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    binding: crate::Binding!("returned"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 1,
                        },
                        then: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(MERFOLK_TOKEN)).with_count(
                                ValueDef::CountObjects(
                                    &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                        "returned"
                                    )),
                                ),
                            ),
                        ),
                    },
                },
            ),
        ])
        .with_type(CardType::Kindred),
);

// ECL 84 — Wild Unraveling
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static WILD_UNRAVELING: CardRecord = CardRecord::new(
    "Wild Unraveling",
    "01522fec-9136-4fa6-91a3-370a8bb08b42",
    "Jabari Weathers",
    CardRules::unsupported(),
);

// ECL 85 — Auntie's Sentence
pub(in crate::card::sets) static AUNTIE_S_SENTENCE: CardRecord = CardRecord::new(
    "Auntie's Sentence",
    "e64bfe16-7362-4982-9136-1f4e0d335441",
    "Vincent Christiaens",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target opponent reveals their hand. You choose a nonland \
                 permanent card from it. That player discards that card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::RevealHand {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Enchantment),
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                                ]),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
                            ]),
                            &[ZoneKind::Hand],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::discard_cards(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("chosen")),
                        )),
                    }),
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Target creature gets -2/-2 until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
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
        ],
    )]),
);

// ECL 86 — Barbed Bloodletter
// Audit: unsupported — Needs wither as a damage-to-creature replacement, including its identity when temporarily granted to the equipped creature; the existing damage pipeline does not replace such damage with -1/-1 counters.
pub(in crate::card::sets) static BARBED_BLOODLETTER: CardRecord = CardRecord::new(
    "Barbed Bloodletter",
    "8ff1a13c-e338-42e9-8eb3-b303fabd67de",
    "Warren Mahy",
    CardRules::unsupported(),
);

// ECL 87 — Bile-Vial Boggart
pub(in crate::card::sets) static BILE_VIAL_BOGGART: CardRecord = CardRecord::new(
    "Bile-Vial Boggart",
    "2c0fcf98-1f3c-4cff-9234-7f3d0c8b22e9",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{B}"), &["Goblin", "Assassin"], 1, 1).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature dies, put a -1/-1 counter on up to one \
             target creature.",
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
                1,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 88 — Bitterbloom Bearer
pub(in crate::card::sets) static BITTERBLOOM_BEARER: CardRecord = CardRecord::new(
    "Bitterbloom Bearer",
    "7127164d-f2a3-4d79-b6db-93507ff5ab47",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Faerie", "Rogue"], 1, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you lose 1 life and create a \
             1/1 blue and black Faerie creature token with flying.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(FAERIE_TOKEN))),
            ]),
        ),
    ]),
);

// ECL 89 — Blight Rot
pub(in crate::card::sets) static BLIGHT_ROT: CardRecord = CardRecord::new(
    "Blight Rot",
    "5201bdeb-ba47-459b-ac0d-603367914578",
    "Forrest Schehl",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put four -1/-1 counters on target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            kind: CounterKind::MinusOneMinusOne,
            amount: ValueDef::Constant(4),
        },
    )]),
);

// ECL 90 — Blighted Blackthorn
pub(in crate::card::sets) static BLIGHTED_BLACKTHORN: CardRecord = CardRecord::new(
    "Blighted Blackthorn",
    "3515531a-45d6-4fe0-96a3-7ca1ce545068",
    "Omar Rayyan",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Treefolk", "Warlock"], 3, 7).with_abilities(
        &[AbilityDef::triggered(
            "Whenever this creature enters or attacks, you may blight 2. \
             If you do, you draw a card and lose 1 life. (To blight 2, put \
             two -1/-1 counters on a creature you control.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::EffectController),
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("blighted")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("blighted"),
                            )),
                            kind: CounterKind::MinusOneMinusOne,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::Sequence(&[
                            abilities::draw_cards(ValueDef::Constant(1)),
                            EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                            },
                        ]),
                    ]),
                },
            }),
        )],
    ),
);

// ECL 91 — Bloodline Bidding
// Audit: unsupported — Needs a resolution-time creature-type choice carried into the following graveyard selection; the existing type-choice procedure only runs as a permanent enters.
pub(in crate::card::sets) static BLOODLINE_BIDDING: CardRecord = CardRecord::new(
    "Bloodline Bidding",
    "877d9b75-ad2f-45a6-94d8-68e80d7db789",
    "Drew Baker",
    CardRules::unsupported(),
);

// ECL 92 — Boggart Mischief
pub(in crate::card::sets) static BOGGART_MISCHIEF: CardRecord = CardRecord::new(
    "Boggart Mischief",
    "aaeb9c9c-0f15-49dc-ae6d-2a958680f327",
    "Ron Spears",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Goblin"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this enchantment enters, you may blight 1. If you do, \
                 create two 1/1 black and red Goblin creature tokens. (To \
                 blight 1, put a -1/-1 counter on a creature you control.)",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::EffectController),
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("blighted")),
                                predicate: ObjectSetPredicateDef {
                                    filter: None,
                                    comparison: ComparisonDef::Greater,
                                    amount: 0,
                                },
                            },
                        ),
                        then: &EffectDef::Sequence(&[
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("blighted"),
                                )),
                                kind: CounterKind::MinusOneMinusOne,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::CreateToken(
                                CreateTokenDef::new(TokenDef::Literal(GOBLIN_TOKEN))
                                    .with_count(ValueDef::Constant(2)),
                            ),
                        ]),
                    },
                }),
            ),
            AbilityDef::triggered(
                "Whenever a Goblin creature you control dies, each opponent \
                 loses 1 life and you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
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
        ])
        .with_type(CardType::Kindred),
);

// ECL 93 — Boggart Prankster
pub(in crate::card::sets) static BOGGART_PRANKSTER: CardRecord = CardRecord::new(
    "Boggart Prankster",
    "eb228ea6-9235-4093-aea7-708e743b1b44",
    "Karl Kopinski",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Goblin", "Warrior"], 1, 3).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever you attack, target attacking Goblin you control gets \
             +1/+0 until end of turn.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                        ObjectPredicateDef::Attacking,
                    ]),
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
        ),
    ]),
);

// ECL 94 — Bogslither's Embrace
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static BOGSLITHER_S_EMBRACE: CardRecord = CardRecord::new(
    "Bogslither's Embrace",
    "4beca2e7-9c6d-493b-b15f-69e483a8dfff",
    "Justin Gerard",
    CardRules::unsupported(),
);

// ECL 95 — Champion of the Weird
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static CHAMPION_OF_THE_WEIRD: CardRecord = CardRecord::new(
    "Champion of the Weird",
    "e600dd01-65ac-489c-a52a-decbc3a9a4f3",
    "Lucas Graciano",
    CardRules::unsupported(),
);

// ECL 96 — Creakwood Safewright
pub(in crate::card::sets) static CREAKWOOD_SAFEWRIGHT: CardRecord = CardRecord::new(
    "Creakwood Safewright",
    "3bcc24cf-776a-4182-bf77-a611ad90b28f",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Elf", "Warrior"], 5, 5).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three -1/-1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::triggered_if(
            "At the beginning of your end step, if there is an Elf card in \
             your graveyard and this creature has a -1/-1 counter on it, \
             remove a -1/-1 counter from this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::All(&[
                TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                TriggerConditionDef::SourceCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    comparison: ComparisonDef::Greater,
                    amount: 0,
                },
            ]),
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 97 — Darkness Descends
pub(in crate::card::sets) static DARKNESS_DESCENDS: CardRecord = CardRecord::new(
    "Darkness Descends",
    "c8aa3895-df83-4f8e-9e23-1a665614b662",
    "Ralph Horsley",
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "Put two -1/-1 counters on each creature.",
        EffectDef::AddCounters {
            object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ))),
            kind: CounterKind::MinusOneMinusOne,
            amount: ValueDef::Constant(2),
        },
    )]),
);

// ECL 98 — Dawnhand Dissident
// Audit: unsupported — Needs chosen-creature blight payments and a linked-exile casting permission with an additional three-counter payment spanning creatures and arbitrary counter kinds.
pub(in crate::card::sets) static DAWNHAND_DISSIDENT: CardRecord = CardRecord::new(
    "Dawnhand Dissident",
    "6ac1f765-f348-4813-88dc-26376e0f3f33",
    "Jacob Walker",
    CardRules::unsupported(),
);

// ECL 99 — Dawnhand Eulogist
pub(in crate::card::sets) static DAWNHAND_EULOGIST: CardRecord = CardRecord::new(
    "Dawnhand Eulogist",
    "75a97f69-f3dc-4d33-8008-c1f1a7c15a2f",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Elf", "Warlock"], 3, 3).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger(
            "When this creature enters, mill three cards. Then if there is \
             an Elf card in your graveyard, each opponent loses 2 life and \
             you gain 2 life. (To mill three cards, put the top three \
             cards of your library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::Sequence(&[
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ]),
                },
            ]),
        ),
    ]),
);

// ECL 100 — Dose of Dawnglow
// Audit: unsupported — Needs a resolution-time condition identifying the caster's main phase; ActivePlayer alone would also accept that player's upkeep or combat.
pub(in crate::card::sets) static DOSE_OF_DAWNGLOW: CardRecord = CardRecord::new(
    "Dose of Dawnglow",
    "47414323-ca30-45b7-a0b2-6668312bee04",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// ECL 101 — Dream Seizer
pub(in crate::card::sets) static DREAM_SEIZER: CardRecord = CardRecord::new(
    "Dream Seizer",
    "3573f425-9251-4c17-9619-15278ce5d8fb",
    "Omar Rayyan",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Faerie", "Rogue"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you may blight 1. If you do, each \
             opponent discards a card. (To blight 1, put a -1/-1 counter \
             on a creature you control.)",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::EffectController),
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("blighted")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("blighted"),
                            )),
                            kind: CounterKind::MinusOneMinusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Discard {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(1),
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        },
                    ]),
                },
            }),
        ),
    ]),
);

// ECL 102 — Gloom Ripper
pub(in crate::card::sets) static GLOOM_RIPPER: CardRecord = CardRecord::new(
    "Gloom Ripper",
    "e9ee5d4f-05d1-4e3a-b8d6-f22d0fddedf7",
    "Annie Stegg",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Elf", "Assassin"], 4, 4).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature you control gets \
             +X/+0 until end of turn and up to one target creature an \
             opponent controls gets -0/-X until end of turn, where X is \
             the number of Elves you control plus the number of Elf cards \
             in your graveyard.",
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
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                    1,
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Sum(&SumValueDef {
                            left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                &[ZoneKind::Graveyard],
                                PlayerRelation::You,
                            )),
                        }),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::Sum(&SumValueDef {
                                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                )),
                                right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                )),
                            }),
                            factor: -1,
                        }),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// ECL 103 — Gnarlbark Elm
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static GNARLBARK_ELM: CardRecord = CardRecord::new(
    "Gnarlbark Elm",
    "1e9d65b6-22ff-49f2-8b2a-aeaef91088d3",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// ECL 104 — Graveshifter (reprint)
const GRAVESHIFTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::GRAVESHIFTER,
    "dadb02b9-d3a0-4b51-bbc3-53b2316cd70d",
    "Deborah Garcia",
);

// ECL 105 — Grub, Storied Matriarch // Grub, Notorious Auntie
// Audit: unsupported — Needs token copies to enter tapped and attacking; the copy-creation runtime only supports ordinary entry, so neither face is enabled.
pub(in crate::card::sets) static GRUB_STORIED_MATRIARCH: CardRecord = CardRecord::new(
    "Grub, Storied Matriarch // Grub, Notorious Auntie",
    "1f51adf8-8234-4dae-aedf-7633310d5111",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// ECL 106 — Gutsplitter Gang
pub(in crate::card::sets) static GUTSPLITTER_GANG: CardRecord = CardRecord::new(
    "Gutsplitter Gang",
    "9ff74349-693d-4373-a194-9796316dd1f1",
    "Tyler Walpole",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Goblin", "Berserker"], 6, 6).with_abilities(
        &[AbilityDef::triggered(
            "At the beginning of your first main phase, you may blight 2. \
             If you don't, you lose 3 life. (To blight 2, put two -1/-1 \
             counters on a creature you control.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::Controller,
                choices: &[
                    EffectChoiceDef {
                        label: "Blight 2",
                        effect: EffectDef::IfElseCondition {
                            condition: &TriggerConditionDef::ObjectCount {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                            then: &EffectDef::Choose(ChooseDef {
                                binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                    "blighted"
                                )),
                                unchosen: None,
                                chooser: PlayerRefDef::EffectController,
                                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerSetDef::One(PlayerRefDef::EffectController),
                                )),
                                exclude: None,
                                minimum: 1,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Public,
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("blighted"),
                                    )),
                                    kind: CounterKind::MinusOneMinusOne,
                                    amount: ValueDef::Constant(2),
                                },
                            }),
                            otherwise: &EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(3),
                            },
                        },
                    },
                    EffectChoiceDef {
                        label: "Lose 3 life",
                        effect: EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    },
                ],
            },
        )],
    ),
);

// ECL 107 — Heirloom Auntie
pub(in crate::card::sets) static HEIRLOOM_AUNTIE: CardRecord = CardRecord::new(
    "Heirloom Auntie",
    "cac251b2-d2cc-45b6-9ca8-678e0eab56ea",
    "Raph Lomotan",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Goblin", "Warlock"], 4, 4).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with two -1/-1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control dies, surveil 1, then \
             remove a -1/-1 counter from this creature. (To surveil 1, \
             look at the top card of your library. You may put it into \
             your graveyard.)",
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
            EffectDef::Sequence(&[
                abilities::surveil(ValueDef::Constant(1)),
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::MinusOneMinusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// ECL 108 — Iron-Shield Elf
pub(in crate::card::sets) static IRON_SHIELD_ELF: CardRecord = CardRecord::new(
    "Iron-Shield Elf",
    "9e0140b2-0185-4adb-b365-2611ce89a0e2",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Elf", "Warrior"], 3, 1).with_abilities(&[
        AbilityDef::activated(
            "Discard a card: This creature gains indestructible until end \
             of turn. Tap it. (Damage and effects that say \"destroy\" \
             don't destroy it. If its toughness is 0 or less, it still \
             dies.)",
            &[CostDef::discard(ObjectPredicateDef::Any)],
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

// ECL 109 — Moonglove Extractor
pub(in crate::card::sets) static MOONGLOVE_EXTRACTOR: CardRecord = CardRecord::new(
    "Moonglove Extractor",
    "8383e0ab-81b4-4a6b-b87b-dd9180dca1c2",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Elf", "Warlock"], 2, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, you draw a card and lose 1 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// ECL 110 — Moonshadow
// Audit: unsupported — Needs an atomic one-or-more group event for permanent cards entering your graveyard from any zone; per-card zone-change triggers remove too many counters for one batch.
pub(in crate::card::sets) static MOONSHADOW: CardRecord = CardRecord::new(
    "Moonshadow",
    "2573e694-eaa0-42ca-b470-2ab507cbcec1",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// ECL 111 — Mornsong Aria
pub(in crate::card::sets) static MORNSONG_ARIA: CardRecord = CardRecord::new(
    "Mornsong Aria",
    "9985c554-8338-46b1-ac36-526d2eb61570",
    "Scott M. Fischer",
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Players can't draw cards or gain life.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::EachPlayer,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotDrawMoreThanEachTurn(0)),
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of each player's draw step, that player \
                 loses 3 life, searches their library for a card, puts it into \
                 their hand, then shuffles.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Draw,
                    player: PlayerRelation::Any,
                },
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::EventPlayer,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::Any,
                        minimum: 1,
                        maximum: ValueDef::Constant(1),
                        reveal: false,
                        destination: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                ]),
            ),
        ]),
);

// ECL 112 — Mudbutton Cursetosser
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static MUDBUTTON_CURSETOSSER: CardRecord = CardRecord::new(
    "Mudbutton Cursetosser",
    "35bc841a-9a21-4c17-a60a-a3ee01472fcb",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// ECL 113 — Nameless Inversion (reprint)
const NAMELESS_INVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::NAMELESS_INVERSION,
    "7a4944ac-839e-4aa1-8200-78cff36bb2ed",
    "Dominik Mayer",
);

// ECL 114 — Nightmare Sower
pub(in crate::card::sets) static NIGHTMARE_SOWER: CardRecord = CardRecord::new(
    "Nightmare Sower",
    "35dfa0f9-faf3-4a85-b02d-0c5830783511",
    "Tommy Arnold",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Faerie", "Assassin"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::lifelink(),
        AbilityDef::triggered_with_targets(
            "Whenever you cast a spell during an opponent's turn, put a \
             -1/-1 counter on up to one target creature.",
            TriggerEventDef::While {
                event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 115 — Perfect Intimidation
pub(in crate::card::sets) static PERFECT_INTIMIDATION: CardRecord = CardRecord::new(
    "Perfect Intimidation",
    "ff05b7f4-e0d0-4304-99b3-66ac804129fe",
    "Heather Hudson",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one or both —",
        &[
            AbilityDef::spell_with_targets(
                "Target opponent exiles two cards from their hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
                    )),
                    exclude: None,
                    minimum: 2,
                    maximum: 2,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                }),
            ),
            AbilityDef::spell_with_targets(
                "Remove all counters from target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::RemoveAllCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: None,
                },
            ),
        ],
    )
    .with_mode_selection(1, 2, false)]),
);

// ECL 116 — Requiting Hex
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static REQUITING_HEX: CardRecord = CardRecord::new(
    "Requiting Hex",
    "f21b0fb7-91b6-403f-a81a-562665961276",
    "Randy Gallegos",
    CardRules::unsupported(),
);

// ECL 117 — Retched Wretch
// Audit: unsupported — Needs a return-to-battlefield instruction that removes all abilities before entry replacements and enters triggers inspect the returning permanent; applying ability loss after the return is too late (CR 611.2e).
pub(in crate::card::sets) static RETCHED_WRETCH: CardRecord = CardRecord::new(
    "Retched Wretch",
    "c7a4d7f1-976a-4a28-97a9-ff089a241c9d",
    "Raph Lomotan",
    CardRules::unsupported(),
);

// ECL 118 — Scarblade Scout
pub(in crate::card::sets) static SCARBLADE_SCOUT: CardRecord = CardRecord::new(
    "Scarblade Scout",
    "c2412fea-4591-4345-913f-edc2da9ad975",
    "Lorenzo Mastroianni",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Elf", "Scout"], 2, 2).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger(
            "When this creature enters, mill two cards. (Put the top two \
             cards of your library into your graveyard.)",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ECL 119 — Scarblade's Malice
// Audit: unsupported — Needs an independent one-shot dies listener bound to the chosen creature through the end of this turn; installed event matchers cannot name an arbitrary bound object.
pub(in crate::card::sets) static SCARBLADE_S_MALICE: CardRecord = CardRecord::new(
    "Scarblade's Malice",
    "aea9b5c0-3b32-44be-9773-566b9daafa6b",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// ECL 120 — Shimmercreep
pub(in crate::card::sets) static SHIMMERCREEP: CardRecord = CardRecord::new(
    "Shimmercreep",
    "f7c02899-5f0f-4b38-bbbc-fbc8c46419a6",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Elemental"], 3, 5).with_abilities(&[
        abilities::menace(),
        abilities::enters_trigger(
            "Vivid — When this creature enters, each opponent loses X life \
             and you gain X life, where X is the number of colors among \
             permanents you control.",
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: VIVID,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: VIVID,
                },
            ]),
        ),
    ]),
);

// ECL 121 — Taster of Wares
// Audit: unsupported — Needs an opponent-selected partial-hand reveal followed by a cast permission that lasts only while you control the source; the current linked permission lifetime only follows zone presence.
pub(in crate::card::sets) static TASTER_OF_WARES: CardRecord = CardRecord::new(
    "Taster of Wares",
    "bc0b64b6-8984-431c-8a2f-84402b429e2b",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// ECL 122 — Twilight Diviner
// Audit: unsupported — Needs atomic grouped creature-entry events carrying each entrant's arrival and casting origin, then a choice among the qualifying entrants; individual entry triggers cannot model this batch.
pub(in crate::card::sets) static TWILIGHT_DIVINER: CardRecord = CardRecord::new(
    "Twilight Diviner",
    "443b6f30-1493-4d48-93d9-a91e22a7ebb3",
    "Pauline Voss",
    CardRules::unsupported(),
);

// ECL 123 — Unbury
// Audit: unsupported — Needs pairwise creature-type intersection among two graveyard targets; another() enforces different objects but does not compare their subtype sets.
pub(in crate::card::sets) static UNBURY: CardRecord = CardRecord::new(
    "Unbury",
    "b00766db-4109-4225-a62a-fa12fd526970",
    "Kev Fang",
    CardRules::unsupported(),
);

// ECL 124 — Ashling, Rekindled // Ashling, Rimebound
pub(in crate::card::sets) static ASHLING_REKINDLED: CardRecord = CardRecord::new_dfc(
    "Ashling, Rekindled // Ashling, Rimebound",
    "7d7faefe-9c0d-45b6-8ea4-5fa666762a2c",
    "Ilse Gort",
    &[
        (
            "Ashling, Rekindled",
            CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Sorcerer"], 1, 3)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever this creature enters or transforms into Ashling, \
                         Rekindled, you may discard a card. If you do, draw a card.",
                        TriggerEventDef::AnyOf(&[
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            TriggerEventDef::Transforms(ObjectPredicateDef::Source),
                        ]),
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::DiscardCards(1)],
                            &abilities::draw_cards(ValueDef::Constant(1)),
                        )),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {U}. \
                         If you do, transform Ashling.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{U}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
        (
            "Ashling, Rimebound",
            CardRules::new_creature_without_mana_cost(&["Elemental", "Wizard"], 1, 3)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Blue])
                .with_abilities(&[
                    AbilityDef::triggered(
                        "Whenever this creature transforms into Ashling, Rimebound and \
                         at the beginning of your first main phase, add two mana of \
                         any one color. Spend this mana only to cast spells with mana \
                         value 4 or greater.",
                        TriggerEventDef::AnyOf(&[
                            TriggerEventDef::Transforms(ObjectPredicateDef::Source),
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::PrecombatMain,
                                player: PlayerRelation::You,
                            },
                        ]),
                        EffectDef::AddMana(
                            AddManaEffectDef::choice(&[
                                ManaColor::White,
                                ManaColor::Blue,
                                ManaColor::Black,
                                ManaColor::Red,
                                ManaColor::Green,
                            ])
                            .with_variable_amount(ValueDef::Constant(2))
                            .with_restrictions(&[
                                ManaRestrictionDef::CastSpell(ObjectPredicateDef::Not(
                                    &ObjectPredicateDef::ManaValueAtMost(3),
                                )),
                            ]),
                        ),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {R}. \
                         If you do, transform Ashling.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{R}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
    ],
);

// ECL 125 — Boldwyr Aggressor
pub(in crate::card::sets) static BOLDWYR_AGGRESSOR: CardRecord = CardRecord::new(
    "Boldwyr Aggressor",
    "76bbccd2-8a90-49a7-921a-6565b495efdf",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Giant", "Warrior"], 2, 5).with_abilities(
        &[
            abilities::double_strike(),
            AbilityDef::static_ability(
                "Other Giants you control have double strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Giant")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            ),
        ],
    ),
);

// ECL 126 — Boneclub Berserker
pub(in crate::card::sets) static BONECLUB_BERSERKER: CardRecord = CardRecord::new(
    "Boneclub Berserker",
    "b3dbbe30-3d6e-46f8-92c1-caee995cba1a",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Berserker"], 2, 4).with_abilities(
        &[AbilityDef::static_ability(
            "This creature gets +2/+0 for each other Goblin you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        factor: 2,
                    }),
                    ValueDef::Constant(0),
                ),
            },
        )],
    ),
);

// ECL 127 — Boulder Dash
pub(in crate::card::sets) static BOULDER_DASH: CardRecord = CardRecord::new(
    "Boulder Dash",
    "657a2a24-22d1-4bc2-9f22-ed361ae487e3",
    "Chuck Lukacs",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Boulder Dash deals 2 damage to any target and 1 damage to any \
         other target.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget).another(),
        ],
        EffectDef::damage_simultaneously(&[
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::Constant(1),
            ),
        ]),
    )]),
);

// ECL 128 — Brambleback Brute
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static BRAMBLEBACK_BRUTE: CardRecord = CardRecord::new(
    "Brambleback Brute",
    "5ebb8365-c6e1-46e8-a242-6aa27b21e68a",
    "Aaron Miller",
    CardRules::unsupported(),
);

// ECL 129 — Burning Curiosity
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static BURNING_CURIOSITY: CardRecord = CardRecord::new(
    "Burning Curiosity",
    "689ed288-6228-4d7e-b198-56a12b8be299",
    "Jim Pavelec",
    CardRules::unsupported(),
);

// ECL 130 — Champion of the Path
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static CHAMPION_OF_THE_PATH: CardRecord = CardRecord::new(
    "Champion of the Path",
    "e369cd31-3e22-47eb-bf6a-00d823651710",
    "Tyler Walpole",
    CardRules::unsupported(),
);

// ECL 131 — Cinder Strike
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static CINDER_STRIKE: CardRecord = CardRecord::new(
    "Cinder Strike",
    "6fb6faa4-236c-4cae-9140-0981c44d2392",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// ECL 132 — Collective Inferno
// Audit: unsupported — Needs damage multiplication filtered by source controller and its chosen creature type, across combat and noncombat damage; existing prevention modifiers do not multiply incoming damage.
pub(in crate::card::sets) static COLLECTIVE_INFERNO: CardRecord = CardRecord::new(
    "Collective Inferno",
    "1ec084cc-997d-4079-b445-8f701ec3c277",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// ECL 133 — Elder Auntie
pub(in crate::card::sets) static ELDER_AUNTIE: CardRecord = CardRecord::new(
    "Elder Auntie",
    "84678e98-2258-4ea1-aaf0-8ac4cc2ecf8d",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warlock"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 black and red Goblin \
             creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(GOBLIN_TOKEN))),
        ),
    ]),
);

// ECL 134 — End-Blaze Epiphany
// Audit: unsupported — Needs a delayed dies listener bound to the damaged creature and exile-play permission expiring at cleanup of your next turn.
pub(in crate::card::sets) static END_BLAZE_EPIPHANY: CardRecord = CardRecord::new(
    "End-Blaze Epiphany",
    "0f0a90ae-b3b3-4f52-8997-eac514b29e57",
    "Tyler Walpole",
    CardRules::unsupported(),
);

// ECL 135 — Enraged Flamecaster
pub(in crate::card::sets) static ENRAGED_FLAMECASTER: CardRecord = CardRecord::new(
    "Enraged Flamecaster",
    "104baa2c-75c5-44fc-a3b9-b19efcf3d7c2",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Sorcerer"], 3, 2).with_abilities(
        &[
            abilities::reach(),
            AbilityDef::triggered(
                "Whenever you cast a spell with mana value 4 or greater, this \
                 creature deals 2 damage to each opponent.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
            ),
        ],
    ),
);

// ECL 136 — Explosive Prodigy
pub(in crate::card::sets) static EXPLOSIVE_PRODIGY: CardRecord = CardRecord::new(
    "Explosive Prodigy",
    "5515ea5e-ce28-4938-a31e-5e48522a5f93",
    "Joshua Raphael",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Sorcerer"], 1, 1).with_abilities(
        &[abilities::enters_trigger_with_targets(
            "Vivid — When this creature enters, it deals X damage to \
             target creature an opponent controls, where X is the number \
             of colors among permanents you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(EffectRecipientDef::Target(TargetIndex::PRIMARY), VIVID),
        )],
    ),
);

// ECL 137 — Feed the Flames
pub(in crate::card::sets) static FEED_THE_FLAMES: CardRecord = CardRecord::new(
    "Feed the Flames",
    "59740755-c353-4b6c-a84c-3b76133ce3ec",
    "Xabi Gaztelua",
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Feed the Flames deals 5 damage to target creature. If that \
         creature would die this turn, exile it instead.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// ECL 138 — Flame-Chain Mauler
pub(in crate::card::sets) static FLAME_CHAIN_MAULER: CardRecord = CardRecord::new(
    "Flame-Chain Mauler",
    "752d7e8e-0dd0-4ace-9c89-a8f9ce73775e",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Warrior"], 2, 2).with_abilities(
        &[AbilityDef::activated(
            "{1}{R}: This creature gets +1/+0 and gains menace until end \
             of turn. (It can't be blocked except by two or more \
             creatures.)",
            &[CostDef::Mana(mana_cost!("{1}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::menace()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )],
    ),
);

// ECL 139 — Flamebraider
// Audit: unsupported — Needs one mana restriction permitting either an Elemental cast or an ability of an Elemental source; listing both restrictions intersects them rather than expressing that union.
pub(in crate::card::sets) static FLAMEBRAIDER: CardRecord = CardRecord::new(
    "Flamebraider",
    "b8aa428c-5a77-444f-b75e-a113e46fe4e0",
    "Pete Venters",
    CardRules::unsupported(),
);

// ECL 140 — Flamekin Gildweaver
pub(in crate::card::sets) static FLAMEKIN_GILDWEAVER: CardRecord = CardRecord::new(
    "Flamekin Gildweaver",
    "b1628ece-a028-49d4-9065-ee997838c20a",
    "Aurore Folny",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental", "Sorcerer"], 4, 3).with_abilities(
        &[
            abilities::trample(),
            abilities::enters_trigger(
                "When this creature enters, create a Treasure token. (It's an \
                 artifact with \"{T}, Sacrifice this token: Add one mana of \
                 any color.\")",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                        .with_count(ValueDef::Constant(1)),
                ),
            ),
        ],
    ),
);

// ECL 141 — Giantfall
pub(in crate::card::sets) static GIANTFALL: CardRecord = CardRecord::new(
    "Giantfall",
    "1ac52728-adb3-4220-8392-73f7bd379ab4",
    "Drew Baker",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
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

// ECL 142 — Goatnap (reprint)
const GOATNAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::GOATNAP,
    "7d8dbec5-ae71-4f82-898a-b930ec677403",
    "Vincent Christiaens",
);

// ECL 143 — Goliath Daydreamer
// Audit: unsupported — Needs a spell-resolution destination rider that exiles the cast spell with a dream counter, plus permission over owned exile cards carrying that counter independently of this source.
pub(in crate::card::sets) static GOLIATH_DAYDREAMER: CardRecord = CardRecord::new(
    "Goliath Daydreamer",
    "88e8cd13-2a29-4df6-937c-1bed68fbeafa",
    "Omar Rayyan",
    CardRules::unsupported(),
);

// ECL 144 — Gristle Glutton
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static GRISTLE_GLUTTON: CardRecord = CardRecord::new(
    "Gristle Glutton",
    "a4164af6-356e-4de2-8377-dbe70434a996",
    "Filip Burburan",
    CardRules::unsupported(),
);

// ECL 145 — Hexing Squelcher
pub(in crate::card::sets) static HEXING_SQUELCHER: CardRecord = CardRecord::new(
    "Hexing Squelcher",
    "674960ce-ff33-4d5e-a24a-a4582b2e9809",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Sorcerer"], 2, 2).with_abilities(&[
        abilities::cannot_be_countered(),
        abilities::ward(&[CostDef::PayLife(2)], "Ward—Pay 2 life."),
        AbilityDef::static_ability(
            "Spells you control can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Spell,
                        &[ZoneKind::Stack],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        ),
        AbilityDef::static_ability(
            "Other creatures you control have \"Ward—Pay 2 life.\"",
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
                effect: AppliedEffectDef::add_ability(&abilities::ward(
                    &[CostDef::PayLife(2)],
                    "Ward—Pay 2 life.",
                )),
            },
        ),
    ]),
);

// ECL 146 — Impolite Entrance
pub(in crate::card::sets) static IMPOLITE_ENTRANCE: CardRecord = CardRecord::new(
    "Impolite Entrance",
    "45be88d8-0be1-47a2-a1c1-d6e693fc706f",
    "Scott Murphy",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gains trample and haste until end of \
         turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// ECL 147 — Kindle the Inner Flame
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static KINDLE_THE_INNER_FLAME: CardRecord = CardRecord::new(
    "Kindle the Inner Flame",
    "9a2adcea-f6b1-4611-b8b8-f19fdee2c571",
    "Jeff Miracola",
    CardRules::unsupported(),
);

// ECL 148 — Kulrath Zealot
// Audit: unsupported — Needs exile-play permission expiring at cleanup of your next turn; the current turn-count duration can persist into the following opponent turn.
pub(in crate::card::sets) static KULRATH_ZEALOT: CardRecord = CardRecord::new(
    "Kulrath Zealot",
    "3502685d-4e57-4c5c-94c6-ae69048cdfbf",
    "Karl Kopinski",
    CardRules::unsupported(),
);

// ECL 149 — Lasting Tarfire
// Audit: unsupported — Needs per-player history of counters placed on creatures this turn; present counter totals cannot identify who placed them or counters on creatures that have left.
pub(in crate::card::sets) static LASTING_TARFIRE: CardRecord = CardRecord::new(
    "Lasting Tarfire",
    "9c4a95ac-072f-4219-80a0-1ce71f1b8411",
    "Jorge Jacinto",
    CardRules::unsupported(),
);

// ECL 150 — Lavaleaper
pub(in crate::card::sets) static LAVALEAPER: CardRecord = CardRecord::new(
    "Lavaleaper",
    "82902488-d178-4752-bcfb-dd3050654d23",
    "Ron Spears",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 4, 4).with_abilities(&[
        AbilityDef::static_ability(
            "All creatures have haste.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
            },
        ),
        AbilityDef::triggered_mana(
            "Whenever a player taps a basic land for mana, that player \
             adds one mana of any type that land produced.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::Supertype(CardSupertype::Basic)),
            EffectDef::AddMana(
                AddManaEffectDef::choice_from(ManaTypeSetDef::produced_by(
                    ObjectRefDef::TriggeringObject,
                ))
                .to_triggering_objects_controller(),
            ),
        ),
    ]),
);

// ECL 151 — Meek Attack
pub(in crate::card::sets) static MEEK_ATTACK: CardRecord = CardRecord::new(
    "Meek Attack",
    "21157461-5435-4879-80a9-100afc5bbf4c",
    "Karl Kopinski",
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::activated(
        "{1}{R}: You may put a creature card with total power and \
         toughness 5 or less from your hand onto the battlefield. That \
         creature gains haste. At the beginning of the next end step, \
         sacrifice that creature.",
        &[CostDef::Mana(mana_cost!("{1}{R}"))],
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::TotalPowerAndToughnessAtMost(5),
                ]),
                &[ZoneKind::Hand],
                PlayerRelation::You,
            )),
            exclude: None,
            minimum: 0,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Private,
            then: &EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("chosen"))),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("arrived"),
                then: &EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(
                            ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("arrived")),
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::BindObjects(BindObjectsDef {
                        source: ObjectCollectionSourceDef::ObjectSet(
                            ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("arrived")),
                        ),
                        binding: crate::Binding!("returned-permanent"),
                        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of the next end step, sacrifice that creature.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::sacrifice(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("returned-permanent")),
                                )),
                            ),
                        )),
                    }),
                ]),
            },
        }),
    )]),
);

// ECL 152 — Reckless Ransacking
pub(in crate::card::sets) static RECKLESS_RANSACKING: CardRecord = CardRecord::new(
    "Reckless Ransacking",
    "24a5b025-4cdb-416d-aad6-0fc7e8da3df2",
    "Daren Bader",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+2 until end of turn. Create a \
         Treasure token. (It's an artifact with \"{T}, Sacrifice this \
         token: Add one mana of any color.\")",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ]),
    )]),
);

// ECL 153 — Scuzzback Scrounger
pub(in crate::card::sets) static SCUZZBACK_SCROUNGER: CardRecord = CardRecord::new(
    "Scuzzback Scrounger",
    "0ea4a895-19c0-47af-ad9c-5db88ea9ae05",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your first main phase, you may blight 1. \
             If you do, create a Treasure token. (To blight 1, put a -1/-1 \
             counter on a creature you control. A Treasure token is an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::EffectController),
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("blighted")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("blighted"),
                            )),
                            kind: CounterKind::MinusOneMinusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                .with_count(ValueDef::Constant(1)),
                        ),
                    ]),
                },
            }),
        ),
    ]),
);

// ECL 154 — Sear
pub(in crate::card::sets) static SEAR: CardRecord = CardRecord::new(
    "Sear",
    "aeb4612c-758b-4492-ba03-eb6741b4176e",
    "Lars Grant-West",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Sear deals 4 damage to target creature or planeswalker.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(4),
        ),
    )]),
);

// ECL 155 — Sizzling Changeling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static SIZZLING_CHANGELING: CardRecord = CardRecord::new(
    "Sizzling Changeling",
    "e58f0722-9ad1-4952-9aee-ea8137c58911",
    "Chris Seaman",
    CardRules::unsupported(),
);

// ECL 156 — Soul Immolation
// Audit: unsupported — Needs a casting payment that chooses X bounded by the greatest controlled toughness and blights a chosen creature for X, retaining the paid X for the damage instruction.
pub(in crate::card::sets) static SOUL_IMMOLATION: CardRecord = CardRecord::new(
    "Soul Immolation",
    "234b70df-8c34-4da7-946e-b8b55a8df390",
    "Drew Tucker",
    CardRules::unsupported(),
);

// ECL 157 — Soulbright Seeker
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static SOULBRIGHT_SEEKER: CardRecord = CardRecord::new(
    "Soulbright Seeker",
    "895ac890-608a-47de-8bc8-9337fd2064e8",
    "Kev Fang",
    CardRules::unsupported(),
);

// ECL 158 — Sourbread Auntie
pub(in crate::card::sets) static SOURBREAD_AUNTIE: CardRecord = CardRecord::new(
    "Sourbread Auntie",
    "32fd226f-d1c6-432c-b846-9482f1944363",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Goblin", "Warrior"], 4, 3).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, you may blight 2. If you do, \
             create two 1/1 black and red Goblin creature tokens. (To \
             blight 2, put two -1/-1 counters on a creature you control.)",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::EffectController),
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("blighted")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                        },
                    }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("blighted"),
                            )),
                            kind: CounterKind::MinusOneMinusOne,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(GOBLIN_TOKEN))
                                .with_count(ValueDef::Constant(2)),
                        ),
                    ]),
                },
            }),
        )],
    ),
);

// ECL 159 — Spinerock Tyrant
// Audit: unsupported — Needs wither as a damage-to-creature replacement and a copy-effect receipt naming both the original spell and its copy for the granted ability.
pub(in crate::card::sets) static SPINEROCK_TYRANT: CardRecord = CardRecord::new(
    "Spinerock Tyrant",
    "478bbb7a-4b96-4e04-921e-bdf23185de25",
    "Cory Godbey",
    CardRules::unsupported(),
);

// ECL 160 — Squawkroaster
pub(in crate::card::sets) static SQUAWKROASTER: CardRecord = CardRecord::new(
    "Squawkroaster",
    "4112f960-70af-4e2d-bcd4-9e9cf7aac4fb",
    "Alessandra Pisano",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 0, 4).with_abilities(&[
        abilities::double_strike(),
        AbilityDef::static_ability(
            "Vivid — Squawkroaster's power is equal to the number of \
             colors among permanents you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(VIVID),
            },
        ),
    ]),
);

// ECL 161 — Sting-Slinger
// Audit: unsupported — Needs a blight cost that chooses a creature you control and puts the required -1/-1 counters on it before the spell or ability is put on the stack; existing counter costs only name the source.
pub(in crate::card::sets) static STING_SLINGER: CardRecord = CardRecord::new(
    "Sting-Slinger",
    "386c5f73-fb8f-46c8-ad45-56e2c19b7d1f",
    "Ralph Horsley",
    CardRules::unsupported(),
);

// ECL 162 — Tweeze
pub(in crate::card::sets) static TWEEZE: CardRecord = CardRecord::new(
    "Tweeze",
    "3ceab0e6-1bb8-487d-ab4b-2da8457b970a",
    "Scott Gustafson",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tweeze deals 3 damage to any target. You may discard a card. \
         If you do, draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ]),
    )]),
);

// ECL 163 — Warren Torchmaster
// Audit: unsupported — Needs a reflexive trigger following an optional blight, with its target selected after the creature was chosen and counters placed.
pub(in crate::card::sets) static WARREN_TORCHMASTER: CardRecord = CardRecord::new(
    "Warren Torchmaster",
    "8f067a14-6667-4acf-b33d-e1149188a84d",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// ECL 164 — Assert Perfection
pub(in crate::card::sets) static ASSERT_PERFECTION: CardRecord = CardRecord::new(
    "Assert Perfection",
    "6995b308-5582-4ca1-ab10-a536d5ca0a6d",
    "Matt Stewart",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+0 until end of turn. It \
         deals damage equal to its power to up to one target creature \
         an opponent controls.",
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
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
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

// ECL 165 — Aurora Awakener
// Audit: unsupported — Needs reveal-until-X-matches with a computed threshold and a bound collection for selecting permanent cards afterward; TopCardsThroughFirstMatching stops after one match.
pub(in crate::card::sets) static AURORA_AWAKENER: CardRecord = CardRecord::new(
    "Aurora Awakener",
    "913977c2-73f9-466b-bd01-827c1736e070",
    "Paolo Parente",
    CardRules::unsupported(),
);

// ECL 166 — Bloom Tender (reprint)
const BLOOM_TENDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eve::BLOOM_TENDER,
    "ba86688d-18f0-4b5c-a797-42bf125a6c9f",
    "Nils Hamm",
);

// ECL 167 — Blossoming Defense (reprint)
const BLOSSOMING_DEFENSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BLOSSOMING_DEFENSE,
    "3cce80d0-b937-4325-a603-1278c110f244",
    "Eelis Kyttanen",
);

// ECL 168 — Bristlebane Battler
pub(in crate::card::sets) static BRISTLEBANE_BATTLER: CardRecord = CardRecord::new(
    "Bristlebane Battler",
    "c857fa32-1b5d-4139-8809-b4d0df44b472",
    "Steve Ellis",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Kithkin", "Soldier"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::as_enters(
            "This creature enters with five -1/-1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    amount: 5,
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever another creature you control enters while this \
             creature has a -1/-1 counter on it, remove a -1/-1 counter \
             from this creature.",
            TriggerEventDef::While {
                event: &TriggerEventDef::zone_changed(
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
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::MinusOneMinusOne,
                    comparison: ComparisonDef::Greater,
                    amount: 0,
                },
            },
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 169 — Bristlebane Outrider
// Audit: unsupported — Needs creature-entry history for the current turn excluding this source incarnation, retained after the entrant leaves or changes controller.
pub(in crate::card::sets) static BRISTLEBANE_OUTRIDER: CardRecord = CardRecord::new(
    "Bristlebane Outrider",
    "38b17a3c-4457-47e3-986e-ff0b94c41b1a",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// ECL 170 — Celestial Reunion
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static CELESTIAL_REUNION: CardRecord = CardRecord::new(
    "Celestial Reunion",
    "583b2863-aca1-4dab-9196-ea453b5d9454",
    "Justin Gerard",
    CardRules::unsupported(),
);

// ECL 171 — Champions of the Perfect
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static CHAMPIONS_OF_THE_PERFECT: CardRecord = CardRecord::new(
    "Champions of the Perfect",
    "4f359211-8be5-4818-b73c-14f24b7ddb21",
    "Chris Rahn",
    CardRules::unsupported(),
);

// ECL 172 — Chomping Changeling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static CHOMPING_CHANGELING: CardRecord = CardRecord::new(
    "Chomping Changeling",
    "e187dcc6-19ad-4cf6-94b4-daf07f5144e5",
    "Jeff Laubenstein",
    CardRules::unsupported(),
);

// ECL 173 — Crossroads Watcher
pub(in crate::card::sets) static CROSSROADS_WATCHER: CardRecord = CardRecord::new(
    "Crossroads Watcher",
    "6d62fcbb-f1a0-46ce-a4af-2a33bcc3ac8e",
    "Aurore Folny",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Kithkin", "Ranger"], 3, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature \
             gets +1/+0 until end of turn.",
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

// ECL 174 — Dawn's Light Archer
pub(in crate::card::sets) static DAWN_S_LIGHT_ARCHER: CardRecord = CardRecord::new(
    "Dawn's Light Archer",
    "76e80656-6bcb-4d99-8bd2-ca5f75f40daf",
    "Scott Gustafson",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Archer"], 4, 2)
        .with_abilities(&[abilities::flash(), abilities::reach()]),
);

// ECL 175 — Dundoolin Weaver
pub(in crate::card::sets) static DUNDOOLIN_WEAVER: CardRecord = CardRecord::new(
    "Dundoolin Weaver",
    "2260912b-4dfb-49dd-bf95-060d44333645",
    "Olivier Bernard",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Kithkin", "Druid"], 2, 1).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if you control three or more \
             creatures, return target permanent card from your graveyard \
             to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(3),
            }),
            &[AbilityTargetDef::exactly_one(
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
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ECL 176 — Formidable Speaker
pub(in crate::card::sets) static FORMIDABLE_SPEAKER: CardRecord = CardRecord::new(
    "Formidable Speaker",
    "265522eb-4f6a-40e7-b374-3833fa63c80b",
    "Aurore Folny",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 2, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may discard a card. If you do, \
             search your library for a creature card, reveal it, put it \
             into your hand, then shuffle.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
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
            )),
        ),
        AbilityDef::activated_with_targets(
            "{1}, {T}: Untap another target permanent.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// ECL 177 — Gilt-Leaf's Embrace
pub(in crate::card::sets) static GILT_LEAF_S_EMBRACE: CardRecord = CardRecord::new(
    "Gilt-Leaf's Embrace",
    "739e5ab5-d562-407a-906e-c5c5173ad325",
    "Volkan Baǵa",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, enchanted creature gains trample and \
                 indestructible until end of turn. (Damage and effects that \
                 say \"destroy\" don't destroy it. If its toughness is 0 or \
                 less, it still dies.)",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// ECL 178 — Great Forest Druid
pub(in crate::card::sets) static GREAT_FOREST_DRUID: CardRecord = CardRecord::new(
    "Great Forest Druid",
    "8793a19e-6743-4031-86d9-2ff55f384549",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Treefolk", "Druid"], 0, 4).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// ECL 179 — Luminollusk
pub(in crate::card::sets) static LUMINOLLUSK: CardRecord = CardRecord::new(
    "Luminollusk",
    "3d591e66-4bf9-47e7-bcef-57769ec3edc6",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elemental"], 2, 4).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger(
            "Vivid — When this creature enters, you gain life equal to the \
             number of colors among permanents you control.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: VIVID,
            },
        ),
    ]),
);

// ECL 180 — Lys Alana Dignitary
// Audit: unsupported — Needs a behold payment that can choose a controlled permanent or reveal a matching card from hand, retaining the chosen object and any linked exile across the cast; the payment programs cannot express that mixed-zone choice.
pub(in crate::card::sets) static LYS_ALANA_DIGNITARY: CardRecord = CardRecord::new(
    "Lys Alana Dignitary",
    "94e8d6a9-7aa3-4e93-8e4d-e50da7ff09d2",
    "Heather Hudson",
    CardRules::unsupported(),
);

// ECL 181 — Lys Alana Informant
pub(in crate::card::sets) static LYS_ALANA_INFORMANT: CardRecord = CardRecord::new(
    "Lys Alana Informant",
    "a79649c4-559e-4306-a102-5fd8750629c7",
    "Sidharth Chaturvedi",
    // A 3/1 that surveils coming and going, so trading it away is still a
    // profitable turn for a deck that wants its graveyard filled.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Scout"], 3, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters or dies, surveil 1. (Look at the top card of your \
             library. You may put it into your graveyard.)",
            // Entering and dying are two ways for one printed ability to
            // fire, so what it does is written once.
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
    ),
);

// ECL 182 — Midnight Tilling
pub(in crate::card::sets) static MIDNIGHT_TILLING: CardRecord = CardRecord::new(
    "Midnight Tilling",
    "c5112bbf-752c-41e7-9c61-4c81e6a77463",
    "Slawomir Maniak",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell(
        "Mill four cards, then you may return a permanent card from \
         among them to your hand. (To mill four cards, put the top \
         four cards of your library into your graveyard.)",
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
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ])),
                },
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("chosen"))),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            }),
        ]),
    )]),
);

// ECL 183 — Mistmeadow Council
pub(in crate::card::sets) static MISTMEADOW_COUNCIL: CardRecord = CardRecord::new(
    "Mistmeadow Council",
    "d4a7c9bc-81c8-4c31-96a9-eb6ba7715e7f",
    "Jim Pavelec",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Kithkin", "Advisor"], 4, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast if you control a Kithkin.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Greater,
                amount: 0,
                then: ValueDef::Constant(1),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// ECL 184 — Moon-Vigil Adherents
pub(in crate::card::sets) static MOON_VIGIL_ADHERENTS: CardRecord = CardRecord::new(
    "Moon-Vigil Adherents",
    "60621c37-62e1-4261-ae76-3946b4a0cfa3",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elf", "Druid"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each creature you control and \
             each creature card in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Sum(&SumValueDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    }),
                    ValueDef::Sum(&SumValueDef {
                        left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    }),
                ),
            },
        ),
    ]),
);

// ECL 185 — Morcant's Eyes
pub(in crate::card::sets) static MORCANT_S_EYES: CardRecord = CardRecord::new(
    "Morcant's Eyes",
    "a730b254-ff7c-4f89-a559-b44ad7fd6c6c",
    "David Palumbo",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Elf"])
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, surveil 1. (Look at the top \
                 card of your library. You may put it into your graveyard.)",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                abilities::surveil(ValueDef::Constant(1)),
            ),
            AbilityDef::activated(
                "{4}{G}{G}, Sacrifice this enchantment: Create X 2/2 black and \
                 green Elf creature tokens, where X is the number of Elf cards \
                 in your graveyard. Activate only as a sorcery.",
                &[
                    CostDef::Mana(mana_cost!("{4}{G}{G}")),
                    CostDef::SacrificeSource,
                ],
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(ELF_TOKEN)).with_count(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    ),
                ),
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ])
        .with_type(CardType::Kindred),
);

// ECL 186 — Mutable Explorer
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static MUTABLE_EXPLORER: CardRecord = CardRecord::new(
    "Mutable Explorer",
    "8f35d95a-caea-4d5e-b98e-55da1ba7c92d",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// ECL 187 — Pitiless Fists
pub(in crate::card::sets) static PITILESS_FISTS: CardRecord = CardRecord::new(
    "Pitiless Fists",
    "295d828b-b2e7-41c7-afbc-5fb5f4eb242c",
    "A. M. Sartor",
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature_you_control(),
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

// ECL 188 — Prismabasher
// Audit: unsupported — Needs a computed maximum target count evaluated while the trigger is placed; AbilityTargetDef bounds are fixed integers and cannot read the number of colors then.
pub(in crate::card::sets) static PRISMABASHER: CardRecord = CardRecord::new(
    "Prismabasher",
    "65c057a7-70af-4464-bd8d-1e7e158d1ae7",
    "Aaron Miller",
    CardRules::unsupported(),
);

// ECL 189 — Prismatic Undercurrents
pub(in crate::card::sets) static PRISMATIC_UNDERCURRENTS: CardRecord = CardRecord::new(
    "Prismatic Undercurrents",
    "bb7490f2-1425-495f-b7d4-2f1e0df7490e",
    "Steve Ellis",
    CardRules::new_enchantment(mana_cost!("{3}{G}")).with_abilities(&[
        abilities::enters_trigger(
            "Vivid — When this enchantment enters, search your library for \
             up to X basic land cards, where X is the number of colors \
             among permanents you control. Reveal those cards, put them \
             into your hand, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Supertype(CardSupertype::Basic),
                minimum: 0,
                maximum: VIVID,
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
            "You may play an additional land on each of your turns.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
            },
        ),
    ]),
);

// ECL 190 — Pummeler for Hire
pub(in crate::card::sets) static PUMMELER_FOR_HIRE: CardRecord = CardRecord::new(
    "Pummeler for Hire",
    "42208996-7b99-474e-aba7-75190d7ee8e2",
    "Steve Ellis",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Giant", "Mercenary"], 4, 4).with_abilities(&[
        abilities::reach(),
        abilities::vigilance(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        abilities::enters_trigger(
            "When this creature enters, you gain X life, where X is the \
             greatest power among Giants you control.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                    objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Giant")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    select: ObjectValueDef::Power,
                    operation: AggregateOperationDef::Maximum,
                }),
            },
        ),
    ]),
);

// ECL 191 — Safewright Cavalry
pub(in crate::card::sets) static SAFEWRIGHT_CAVALRY: CardRecord = CardRecord::new(
    "Safewright Cavalry",
    "f7de412a-9731-4bfc-8fbc-c95988a3dd70",
    "Milivoj Ćeran",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Warrior"], 4, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by more than one creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::MaximumBlockers(1),
                )),
            },
        ),
        AbilityDef::activated_with_targets(
            "{5}: Target Elf you control gets +2/+2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{5}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
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

// ECL 192 — Sapling Nursery
pub(in crate::card::sets) static SAPLING_NURSERY: CardRecord = CardRecord::new(
    "Sapling Nursery",
    "3199bea9-fef7-45fe-8777-2103d84a9347",
    "Vincent Christiaens",
    CardRules::new_enchantment(mana_cost!("{6}{G}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Forests (This spell costs {1} less to cast for \
             each Forest you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, create a 3/4 \
             green Treefolk creature token with reach.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(TREEFOLK_TOKEN))),
        ),
        AbilityDef::activated(
            "{1}{G}, Exile this enchantment: Treefolk and Forests you \
             control gain indestructible until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{G}")), CostDef::ExileSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treefolk")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 193 — Selfless Safewright
// Audit: unsupported — Needs a resolution-time creature-type choice bound to the following static grant recipients; entry-time choices would occur before this enters trigger resolves.
pub(in crate::card::sets) static SELFLESS_SAFEWRIGHT: CardRecord = CardRecord::new(
    "Selfless Safewright",
    "ac95b1c3-9eb2-4f80-bb32-72b36817d622",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// ECL 194 — Shimmerwilds Growth
pub(in crate::card::sets) static SHIMMERWILDS_GROWTH: CardRecord = CardRecord::new(
    "Shimmerwilds Growth",
    "c122719c-f0d1-4170-a0d1-d62172df1d21",
    "Jorge Jacinto",
CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::as_enters(
                "As this Aura enters, choose a color.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ),
            AbilityDef::static_ability(
                "Enchanted land is the chosen color.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::set_color(ManaTypeDef::ChosenColor),
                },
            ),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an additional one mana of the chosen color.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddMana(
                    AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// ECL 195 — Spry and Mighty
// Audit: unsupported — Needs a frozen value binding for the absolute difference between the chosen creatures' powers, retained through the draw before applying both bonuses; recomputing after the draw can change X.
pub(in crate::card::sets) static SPRY_AND_MIGHTY: CardRecord = CardRecord::new(
    "Spry and Mighty",
    "152b7374-e991-443f-b6ca-914415635c4a",
    "Pete Venters",
    CardRules::unsupported(),
);

// ECL 196 — Surly Farrier
pub(in crate::card::sets) static SURLY_FARRIER: CardRecord = CardRecord::new(
    "Surly Farrier",
    "056f7f51-18a9-4d80-8928-decaf4d12c0d",
    "Jake Murray",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Kithkin", "Citizen"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control gets +1/+1 and gains \
             vigilance until end of turn. Activate only as a sorcery.",
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
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// ECL 197 — Tend the Sprigs
pub(in crate::card::sets) static TEND_THE_SPRIGS: CardRecord = CardRecord::new(
    "Tend the Sprigs",
    "388f6d9d-bb9a-4a3d-93c5-701db194863c",
    "Iris Compiet",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Search your library for a basic land card, put it onto the \
         battlefield tapped, then shuffle. Then if you control seven \
         or more lands and/or Treefolk, create a 3/4 green Treefolk \
         creature token with reach. (It can block creatures with \
         flying.)",
        EffectDef::Sequence(&[
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treefolk")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(7),
                }),
                then: &EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TREEFOLK_TOKEN,
                ))),
            },
        ]),
    )]),
);

// ECL 198 — Thoughtweft Charge
// Audit: unsupported — Needs per-player creature-entry history for the current turn retained after the entering creature leaves or changes controller.
pub(in crate::card::sets) static THOUGHTWEFT_CHARGE: CardRecord = CardRecord::new(
    "Thoughtweft Charge",
    "19cfc015-b6d5-4918-99a7-990209e77441",
    "Josiah \"Jo\" Cameron",
    CardRules::unsupported(),
);

// ECL 199 — Trystan, Callous Cultivator // Trystan, Penitent Culler
pub(in crate::card::sets) static TRYSTAN_CALLOUS_CULTIVATOR: CardRecord = CardRecord::new_dfc(
    "Trystan, Callous Cultivator // Trystan, Penitent Culler",
    "2094f9b6-ce84-47d9-8819-81db14ba483f",
    "Annie Stegg",
    &[
        (
            "Trystan, Callous Cultivator",
            CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 3, 4)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    abilities::deathtouch(),
                    AbilityDef::triggered(
                        "Whenever this creature enters or transforms into Trystan, \
                         Callous Cultivator, mill three cards. Then if there is an Elf \
                         card in your graveyard, you gain 2 life.",
                        TriggerEventDef::AnyOf(&[
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            TriggerEventDef::Transforms(ObjectPredicateDef::Source),
                        ]),
                        EffectDef::Sequence(&[
                            EffectDef::Mill {
                                player: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(3),
                            },
                            EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ObjectCount {
                                    query: ObjectQueryDef::matching(
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                        &[ZoneKind::Graveyard],
                                        PlayerRelation::You,
                                    ),
                                    comparison: ComparisonDef::GreaterOrEqual,
                                    amount: 1,
                                },
                                then: &EffectDef::GainLife {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(2),
                                },
                            },
                        ]),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {B}. \
                         If you do, transform Trystan.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{B}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
        (
            "Trystan, Penitent Culler",
            CardRules::new_creature_without_mana_cost(&["Elf", "Warlock"], 3, 4)
                .with_supertype(CardSupertype::Legendary)
                .printed_colors(&[ManaColor::Black])
                .with_abilities(&[
                    abilities::deathtouch(),
                    AbilityDef::triggered(
                        "Whenever this creature transforms into Trystan, Penitent \
                         Culler, mill three cards, then you may exile an Elf card from \
                         your graveyard. If you do, each opponent loses 2 life.",
                        TriggerEventDef::Transforms(ObjectPredicateDef::Source),
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
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                )),
                                exclude: None,
                                minimum: 0,
                                maximum: 1,
                                visibility: ChoiceVisibilityDef::Public,
                                then: &EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::ObjectSetCount(
                                        &ObjectSetCountConditionDef {
                                            objects: &ObjectSetDef::Binding(crate::Binding!(
                                                "chosen"
                                            )),
                                            predicate: ObjectSetPredicateDef {
                                                filter: None,
                                                comparison: ComparisonDef::Greater,
                                                amount: 0,
                                            },
                                        },
                                    ),
                                    then: &EffectDef::Sequence(&[
                                        EffectDef::move_to_zone(
                                            EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                crate::Binding!("chosen"),
                                            )),
                                            ZoneKind::Exile,
                                            ZonePlacement::Top,
                                        ),
                                        EffectDef::LoseLife {
                                            recipient: EffectRecipientDef::Opponent,
                                            amount: ValueDef::Constant(2),
                                        },
                                    ]),
                                },
                            }),
                        ]),
                    ),
                    AbilityDef::triggered(
                        "At the beginning of your first main phase, you may pay {G}. \
                         If you do, transform Trystan.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::PrecombatMain,
                            player: PlayerRelation::You,
                        },
                        EffectDef::PayOr(PayOrDef::optional(
                            &[CostDef::Mana(mana_cost!("{G}"))],
                            &EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                        )),
                    ),
                ]),
        ),
    ],
);

// ECL 200 — Unforgiving Aim
pub(in crate::card::sets) static UNFORGIVING_AIM: CardRecord = CardRecord::new(
    "Unforgiving Aim",
    "0bef3905-24f8-419b-aeca-396adfc6d0dc",
    "Filip Burburan",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Keyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
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
            AbilityDef::spell(
                "Create a 2/2 black and green Elf creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELF_TOKEN))),
            ),
        ],
    )]),
);

// ECL 201 — Vinebred Brawler
// Audit: unsupported — Needs an at-least-one legal blocker requirement; MustBeBlockedBy requires every matching able creature to block and would overconstrain the printed ability.
pub(in crate::card::sets) static VINEBRED_BRAWLER: CardRecord = CardRecord::new(
    "Vinebred Brawler",
    "63c573fe-e74c-48ca-ad05-92f37dc466f1",
    "Evyn Fong",
    CardRules::unsupported(),
);

// ECL 202 — Virulent Emissary
pub(in crate::card::sets) static VIRULENT_EMISSARY: CardRecord = CardRecord::new(
    "Virulent Emissary",
    "0702efed-915e-466a-96bb-ac09af06b21e",
    "Tiffany Turrill",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Assassin"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 1 life.",
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
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ECL 203 — Wildvine Pummeler
pub(in crate::card::sets) static WILDVINE_PUMMELER: CardRecord = CardRecord::new(
    "Wildvine Pummeler",
    "11bad5c7-fe9a-4d89-a531-8d4f03d5a0e4",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Giant", "Berserker"], 6, 5).with_abilities(&[
        AbilityDef::static_ability(
            "Vivid — This spell costs {1} less to cast for each color \
             among permanents you control.",
            EffectDef::ReduceGenericCostBy(ValueDef::Sum(&SumValueDef {
                left: ValueDef::Sum(&SumValueDef {
                    left: ValueDef::Sum(&SumValueDef {
                        left: ValueDef::Sum(&SumValueDef {
                            left: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::Color(ManaColor::White),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::Greater,
                                amount: 0,
                                then: ValueDef::Constant(1),
                                otherwise: ValueDef::Constant(0),
                            }),
                            right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                                query: ObjectQueryDef::matching(
                                    ObjectPredicateDef::Color(ManaColor::Blue),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                                comparison: ComparisonDef::Greater,
                                amount: 0,
                                then: ValueDef::Constant(1),
                                otherwise: ValueDef::Constant(0),
                            }),
                        }),
                        right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Color(ManaColor::Black),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::Greater,
                            amount: 0,
                            then: ValueDef::Constant(1),
                            otherwise: ValueDef::Constant(0),
                        }),
                    }),
                    right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Color(ManaColor::Red),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::Greater,
                        amount: 0,
                        then: ValueDef::Constant(1),
                        otherwise: ValueDef::Constant(0),
                    }),
                }),
                right: ValueDef::IfMatchingObjectCount(&CountConditionDef {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Color(ManaColor::Green),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::Greater,
                    amount: 0,
                    then: ValueDef::Constant(1),
                    otherwise: ValueDef::Constant(0),
                }),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::reach(),
        abilities::trample(),
    ]),
);

// ECL 204 — Abigale, Eloquent First-Year
pub(in crate::card::sets) static ABIGALE_ELOQUENT_FIRST_YEAR: CardRecord = CardRecord::new(
    "Abigale, Eloquent First-Year",
    "bf708169-a307-494b-b8d8-baae53b2e2f2",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{W/B}{W/B}"), &["Bird", "Bard"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::first_strike(),
            abilities::lifelink(),
            abilities::enters_trigger_with_targets(
                "When Abigale enters, up to one other target creature loses \
                 all abilities. Put a flying counter, a first strike counter, \
                 and a lifelink counter on that creature.",
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
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        duration: ResolvedEffectDurationDef::Permanent,
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Flying,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::FirstStrike,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Lifelink,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// ECL 205 — Ashling's Command
pub(in crate::card::sets) static ASHLING_S_COMMAND: CardRecord = CardRecord::new(
    "Ashling's Command",
    "fe3a421d-07b6-4b94-b177-aec44c7fe689",
    "Iris Compiet",
    CardRules::new_instant(mana_cost!("{3}{U}{R}"))
        .with_subtypes(&["Elemental"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose two —",
            &[
                AbilityDef::spell_with_targets(
                    "Create a token that's a copy of target Elemental you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elemental")),
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
                AbilityDef::spell_with_targets(
                    "Target player draws two cards.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Ashling's Command deals 2 damage to each creature target \
                     player controls.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::damage(
                        EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::controlled_by(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerSetDef::LegalTargets(TargetIndex(0)),
                            ),
                        )),
                        ValueDef::Constant(2),
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Target player creates two Treasure tokens.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                            .with_count(ValueDef::Constant(2))
                            .with_controller(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    ),
                ),
            ],
        )
        .with_mode_selection(2, 2, false)])
        .with_type(CardType::Kindred),
);

// ECL 206 — Boggart Cursecrafter
pub(in crate::card::sets) static BOGGART_CURSECRAFTER: CardRecord = CardRecord::new(
    "Boggart Cursecrafter",
    "b0b67eb9-0d88-4f2c-8063-e8bedfa78556",
    "Alex Stone",
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Goblin", "Warlock"], 2, 3).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever another Goblin you control dies, this creature deals \
             1 damage to each opponent.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
    ]),
);

// ECL 207 — Bre of Clan Stoutarm
// Audit: unsupported — Needs a free-cast offer filtered by the chosen spell form's mana value, plus a return-to-hand fallback for a card with no permitted form; filtering the exile card's normal mana value mishandles alternate forms.
pub(in crate::card::sets) static BRE_OF_CLAN_STOUTARM: CardRecord = CardRecord::new(
    "Bre of Clan Stoutarm",
    "013cefb7-a059-45c2-81b0-187f35aac4a2",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// ECL 208 — Brigid's Command
pub(in crate::card::sets) static BRIGID_S_COMMAND: CardRecord = CardRecord::new(
    "Brigid's Command",
    "cf034777-3da7-4d5c-9213-5e9d235c315a",
    "Sam Guay",
    CardRules::new_sorcery(mana_cost!("{1}{G}{W}"))
        .with_subtypes(&["Kithkin"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose two —",
            &[
                AbilityDef::spell_with_targets(
                    "Create a token that's a copy of target Kithkin you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
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
                AbilityDef::spell_with_targets(
                    "Target player creates a 1/1 green and white Kithkin creature \
                     token.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::CreateToken(
                        CreateTokenDef::new(TokenDef::Literal(KITHKIN_TOKEN))
                            .with_controller(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    ),
                ),
                AbilityDef::spell_with_targets(
                    "Target creature you control gets +3/+3 until end of turn.",
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
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target creature you control fights target creature an \
                     opponent controls.",
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
                    EffectDef::Fight {
                        first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                        second: ObjectRefDef::Target(TargetIndex(1)),
                        excess: None,
                    },
                ),
            ],
        )
        .with_mode_selection(2, 2, false)])
        .with_type(CardType::Kindred),
);

// ECL 209 — Catharsis
// Audit: unsupported — Needs the number of mana units of each color spent to cast the source retained for its enters triggers; ColorsOfManaSpent counts distinct colors and does not distinguish one unit from two of the same color.
pub(in crate::card::sets) static CATHARSIS: CardRecord = CardRecord::new(
    "Catharsis",
    "affb4500-2704-49fc-bbb4-02ed4bfb3b76",
    "Alex Horley-Orlandelli",
    CardRules::unsupported(),
);

// ECL 210 — Chaos Spewer
pub(in crate::card::sets) static CHAOS_SPEWER: CardRecord = CardRecord::new(
    "Chaos Spewer",
    "b5918fa5-1d13-447b-8838-633b6b61e791",
    "Quintin Gleim",
    CardRules::new_creature(mana_cost!("{2}{B/R}"), &["Goblin", "Warlock"], 5, 4).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, you may pay {2}. If you don't, \
             blight 2. (To blight 2, put two -1/-1 counters on a creature \
             you control.)",
            EffectDef::PayOr(PayOrDef::optional_or(
                &[CostDef::Mana(mana_cost!("{2}"))],
                &EffectDef::None,
                &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::EffectController),
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("blighted"),
                        )),
                        kind: CounterKind::MinusOneMinusOne,
                        amount: ValueDef::Constant(2),
                    },
                }),
            )),
        )],
    ),
);

// ECL 211 — Chitinous Graspling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static CHITINOUS_GRASPLING: CardRecord = CardRecord::new(
    "Chitinous Graspling",
    "a9767360-d536-4902-9d2d-1f3474ce89d6",
    "Richard Kane Ferguson",
    CardRules::unsupported(),
);

// ECL 212 — Deceit
// Audit: unsupported — Needs the number of mana units of each color spent to cast the source retained for its enters triggers; ColorsOfManaSpent counts distinct colors and does not distinguish one unit from two of the same color.
pub(in crate::card::sets) static DECEIT: CardRecord = CardRecord::new(
    "Deceit",
    "bd82c9e4-9871-4e6d-b691-ee00b4b9a3c6",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// ECL 213 — Deepchannel Duelist
pub(in crate::card::sets) static DEEPCHANNEL_DUELIST: CardRecord = CardRecord::new(
    "Deepchannel Duelist",
    "1b742172-7118-45e7-9945-62bd77d94e85",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Merfolk", "Soldier"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of your end step, untap target Merfolk you \
             control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::static_ability(
            "Other Merfolk you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
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

// ECL 214 — Deepway Navigator
// Audit: unsupported — Needs per-player history of how many Merfolk attacked this turn, retained after those attackers leave or change types.
pub(in crate::card::sets) static DEEPWAY_NAVIGATOR: CardRecord = CardRecord::new(
    "Deepway Navigator",
    "d988e28b-fa60-4b60-8229-7a15932c784b",
    "Jacob Walker",
    CardRules::unsupported(),
);

// ECL 215 — Doran, Besieged by Time
// Audit: unsupported — Needs a blocks-declared event identifying each controlled blocking creature once, independently of how many attackers it blocks; Blocks only listens to the ability source and names the creature it blocked.
pub(in crate::card::sets) static DORAN_BESIEGED_BY_TIME: CardRecord = CardRecord::new(
    "Doran, Besieged by Time",
    "568aa70a-6765-486a-bd37-5d38b16c46de",
    "Carl Critchlow",
    CardRules::unsupported(),
);

// ECL 216 — Dream Harvest
// Audit: unsupported — Needs exile-until cumulative mana value reaches a threshold, with a bound collection for subsequent free casts; existing top-through-match collections cannot accumulate mana value.
pub(in crate::card::sets) static DREAM_HARVEST: CardRecord = CardRecord::new(
    "Dream Harvest",
    "a4ebc0e7-02d7-4d38-9376-a39963e6d3fa",
    "Ben Hill",
    CardRules::unsupported(),
);

// ECL 217 — Eclipsed Boggart
pub(in crate::card::sets) static ECLIPSED_BOGGART: CardRecord = CardRecord::new(
    "Eclipsed Boggart",
    "0ac1d0b2-92a4-4a10-b2d1-e9bb90265cc3",
    "Tiffany Turrill",
    CardRules::new_creature(mana_cost!("{B/R}{B/R}{B/R}"), &["Goblin", "Scout"], 2, 3)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal a Goblin, Swamp, or Mountain card \
             from among them and put it into your hand. Put the rest on \
             the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
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

// ECL 218 — Eclipsed Elf
pub(in crate::card::sets) static ECLIPSED_ELF: CardRecord = CardRecord::new(
    "Eclipsed Elf",
    "9c8579f3-6125-4f22-b1c2-b7a0cfc50eed",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{B/G}{B/G}{B/G}"), &["Elf", "Scout"], 3, 2).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal an Elf, Swamp, or Forest card from \
             among them and put it into your hand. Put the rest on the \
             bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
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
        )],
    ),
);

// ECL 219 — Eclipsed Flamekin
pub(in crate::card::sets) static ECLIPSED_FLAMEKIN: CardRecord = CardRecord::new(
    "Eclipsed Flamekin",
    "d907ae44-cd07-4409-946c-e97f584d9a81",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{1}{U/R}{U/R}"), &["Elemental", "Scout"], 1, 4)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal an Elemental, Island, or Mountain \
             card from among them and put it into your hand. Put the rest \
             on the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elemental")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
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

// ECL 220 — Eclipsed Kithkin
pub(in crate::card::sets) static ECLIPSED_KITHKIN: CardRecord = CardRecord::new(
    "Eclipsed Kithkin",
    "29e1cfa4-0ad8-4228-9f7e-cbab114d1d5f",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{G/W}{G/W}"), &["Kithkin", "Scout"], 2, 1).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal a Kithkin, Forest, or Plains card \
             from among them and put it into your hand. Put the rest on \
             the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
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
        )],
    ),
);

// ECL 221 — Eclipsed Merrow
pub(in crate::card::sets) static ECLIPSED_MERROW: CardRecord = CardRecord::new(
    "Eclipsed Merrow",
    "2352750d-404d-4928-9bdb-1b0db599b70f",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{W/U}{W/U}{W/U}"), &["Merfolk", "Scout"], 2, 3)
        .with_abilities(&[abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your \
             library. You may reveal a Merfolk, Plains, or Island card \
             from among them and put it into your hand. Put the rest on \
             the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
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

// ECL 222 — Emptiness
// Audit: unsupported — Needs the number of mana units of each color spent to cast the source retained for its enters triggers; ColorsOfManaSpent counts distinct colors and does not distinguish one unit from two of the same color.
pub(in crate::card::sets) static EMPTINESS: CardRecord = CardRecord::new(
    "Emptiness",
    "c6409eca-bef6-4f3a-8bbb-d69ec5dbfc13",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// ECL 223 — Feisty Spikeling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static FEISTY_SPIKELING: CardRecord = CardRecord::new(
    "Feisty Spikeling",
    "f69f3a27-ecda-4d27-82fe-612ed57dbb28",
    "Tiffany Turrill",
    CardRules::unsupported(),
);

// ECL 224 — Figure of Fable
pub(in crate::card::sets) static FIGURE_OF_FABLE: CardRecord = CardRecord::new(
    "Figure of Fable",
    "e0ef33dd-5f6d-48fa-8ef6-a8092868d50f",
    "Omar Rayyan",
    CardRules::new_creature(mana_cost!("{G/W}"), &["Kithkin"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{G/W}: This creature becomes a Kithkin Scout with base power \
             and toughness 2/3.",
            &[CostDef::Mana(mana_cost!("{G/W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                        "Kithkin", "Scout",
                    ])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(3),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
        AbilityDef::activated(
            "{1}{G/W}{G/W}: If this creature is a Scout, it becomes a \
             Kithkin Soldier with base power and toughness 4/5.",
            &[CostDef::Mana(mana_cost!("{1}{G/W}{G/W}"))],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Scout")),
                },
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Kithkin", "Soldier",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(5),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
        AbilityDef::activated(
            "{3}{G/W}{G/W}{G/W}: If this creature is a Soldier, it becomes \
             a Kithkin Avatar with base power and toughness 7/8 and \
             protection from each of your opponents.",
            &[CostDef::Mana(mana_cost!("{3}{G/W}{G/W}{G/W}"))],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Soldier")),
                },
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Kithkin", "Avatar",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(7),
                            ValueDef::Constant(8),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::keyword(
                            "Protection from each of your opponents",
                            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::ControlledBy(
                                PlayerRelation::Opponent,
                            )),
                        )),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
    ]),
);

// ECL 225 — Flaring Cinder
pub(in crate::card::sets) static FLARING_CINDER: CardRecord = CardRecord::new(
    "Flaring Cinder",
    "0691a1a9-18e7-44b7-9a34-764b1ab45a76",
    "Kai Carpenter",
    CardRules::new_creature(
        mana_cost!("{1}{U/R}{U/R}"),
        &["Elemental", "Sorcerer"],
        3,
        2,
    )
    .with_abilities(&[AbilityDef::triggered(
        "When this creature enters and whenever you cast a spell with \
         mana value 4 or greater, you may discard a card. If you do, \
         draw a card.",
        TriggerEventDef::AnyOf(&[
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
        ]),
        EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::DiscardCards(1)],
            &abilities::draw_cards(ValueDef::Constant(1)),
        )),
    )]),
);

// ECL 226 — Gangly Stompling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static GANGLY_STOMPLING: CardRecord = CardRecord::new(
    "Gangly Stompling",
    "502000a7-a3c1-4259-aea5-ff01724396a1",
    "Scott Murphy",
    CardRules::unsupported(),
);

// ECL 227 — Glister Bairn
pub(in crate::card::sets) static GLISTER_BAIRN: CardRecord = CardRecord::new(
    "Glister Bairn",
    "dcccdd98-a1ba-41f4-a9ae-84d263da0af9",
    "Nils Hamm",
    CardRules::new_creature(mana_cost!("{2}{G/U}{G/U}{G/U}"), &["Ouphe"], 1, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Vivid — At the beginning of combat on your turn, another \
             target creature you control gets +X/+X until end of turn, \
             where X is the number of colors among permanents you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
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
                effect: AppliedEffectDef::modify_power_toughness(VIVID, VIVID),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 228 — Grub's Command
pub(in crate::card::sets) static GRUB_S_COMMAND: CardRecord = CardRecord::new(
    "Grub's Command",
    "73013908-feff-495a-a62e-1e19548ffe6f",
    "Iris Compiet",
    CardRules::new_sorcery(mana_cost!("{3}{B}{R}"))
        .with_subtypes(&["Goblin"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose two —",
            &[
                AbilityDef::spell_with_targets(
                    "Create a token that's a copy of target Goblin you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
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
                AbilityDef::spell_with_targets(
                    "Creatures target player controls get +1/+1 and gain haste \
                     until end of turn.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::controlled_by(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerSetDef::LegalTargets(TargetIndex(0)),
                            ),
                        )),
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
                AbilityDef::spell_with_targets(
                    "Destroy target artifact or creature.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                        ]),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target player mills five cards, then puts each Goblin card \
                     milled this way into their hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::BindOutput {
                            binding: crate::Binding!("milled"),
                            effect: &EffectDef::Mill {
                                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(5),
                            },
                        },
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Matching {
                                objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                                object: ObjectSetFilterDef::Predicate(
                                    &ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                                ),
                            }),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    ]),
                ),
            ],
        )
        .with_mode_selection(2, 2, false)])
        .with_type(CardType::Kindred),
);

// ECL 229 — High Perfect Morcant
pub(in crate::card::sets) static HIGH_PERFECT_MORCANT: CardRecord = CardRecord::new(
    "High Perfect Morcant",
    "dfe7b8bf-c150-4be0-aef4-e8bb6f09787a",
    "Victor Adame Minguez",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Elf", "Noble"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever High Perfect Morcant or another Elf you control \
                 enters, each opponent blights 1. (They each put a -1/-1 \
                 counter on a creature they control.)",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("blighted")),
                    unchosen: None,
                    chooser: PlayerRefDef::Opponent,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::Opponent),
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("blighted"),
                        )),
                        kind: CounterKind::MinusOneMinusOne,
                        amount: ValueDef::Constant(1),
                    },
                }),
            ),
            AbilityDef::activated(
                "Tap three untapped Elves you control: Proliferate. Activate \
                 only as a sorcery. (Choose any number of permanents and/or \
                 players, then give each another counter of each kind already \
                 there.)",
                &[CostDef::TapPermanents {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
                    controller: PlayerRelation::You,
                    count: 3,
                }],
                EffectDef::Proliferate,
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ]),
);

// ECL 230 — Hovel Hurler
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static HOVEL_HURLER: CardRecord = CardRecord::new(
    "Hovel Hurler",
    "adc6a4c5-4e92-43ee-8d0c-204042965eb7",
    "Chris Seaman",
    CardRules::unsupported(),
);

// ECL 231 — Kirol, Attentive First-Year
pub(in crate::card::sets) static KIROL_ATTENTIVE_FIRST_YEAR: CardRecord = CardRecord::new(
    "Kirol, Attentive First-Year",
    "c11cb0eb-819e-4905-ad95-e43618d3c81e",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{1}{R/W}{R/W}"), &["Vampire", "Cleric"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "Tap two untapped creatures you control: Copy target triggered \
             ability you control. You may choose new targets for the copy. \
             Activate only once each turn.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 2,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::TriggeredAbility,
                    zones: &[ZoneKind::Stack],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        )
        .once_each_turn()]),
);

// ECL 232 — Lluwen, Imperfect Naturalist
pub(in crate::card::sets) static LLUWEN_IMPERFECT_NATURALIST: CardRecord = CardRecord::new(
    "Lluwen, Imperfect Naturalist",
    "127a30a6-c25a-448a-a242-dc04f273a854",
    "Evyn Fong",
    CardRules::new_creature(mana_cost!("{B/G}{B/G}"), &["Elf", "Druid"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Lluwen enters, mill four cards, then you may put a \
                 creature or land card from among the milled cards on top of \
                 your library.",
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
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            ),
            AbilityDef::activated(
                "{2}{B/G}{B/G}{B/G}, {T}, Discard a land card: Create a 1/1 \
                 black and green Worm creature token for each land card in \
                 your graveyard.",
                &[
                    CostDef::Mana(mana_cost!("{2}{B/G}{B/G}{B/G}")),
                    CostDef::TapSource,
                    CostDef::discard(ObjectPredicateDef::HasType(CardType::Land)),
                ],
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                        &["Worm"],
                        &[ManaColor::Black, ManaColor::Green],
                        1,
                        1,
                    )))
                    .with_count(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                    )),
                ),
            ),
        ]),
);

// ECL 233 — Maralen, Fae Ascendant
// Audit: unsupported — Needs linked exiles tagged by turn and a once-per-turn free-cast permission filtered by the prospective spell form's mana value; existing linked-exile permissions lack that combination.
pub(in crate::card::sets) static MARALEN_FAE_ASCENDANT: CardRecord = CardRecord::new(
    "Maralen, Fae Ascendant",
    "c50f5408-5b5c-41dc-807e-136233403a09",
    "Steve Prescott",
    CardRules::unsupported(),
);

// ECL 234 — Merrow Skyswimmer
pub(in crate::card::sets) static MERROW_SKYSWIMMER: CardRecord = CardRecord::new(
    "Merrow Skyswimmer",
    "075b419a-fd44-4a9a-8c40-474562b7e11a",
    "Richard Kane Ferguson",
    CardRules::new_creature(mana_cost!("{3}{W/U}{W/U}"), &["Merfolk", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::convoke(),
            abilities::flying(),
            abilities::vigilance(),
            abilities::enters_trigger(
                "When this creature enters, create a 1/1 white and blue \
                 Merfolk creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(MERFOLK_TOKEN))),
            ),
        ]),
);

// ECL 235 — Mischievous Sneakling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static MISCHIEVOUS_SNEAKLING: CardRecord = CardRecord::new(
    "Mischievous Sneakling",
    "3b66fa00-2fa6-4060-9e7f-8e3fde6deb73",
    "Ron Spears",
    CardRules::unsupported(),
);

// ECL 236 — Morcant's Loyalist
// Audit: unsupported — Needs graveyard target exclusion by the dying source's zone-change successor; Not(Source) compares the old battlefield identity and can incorrectly target this same card in the graveyard.
pub(in crate::card::sets) static MORCANT_S_LOYALIST: CardRecord = CardRecord::new(
    "Morcant's Loyalist",
    "175c6b1c-8790-41a2-ae15-5031206c410f",
    "Evyn Fong",
    CardRules::unsupported(),
);

// ECL 237 — Noggle Robber
pub(in crate::card::sets) static NOGGLE_ROBBER: CardRecord = CardRecord::new(
    "Noggle Robber",
    "0082ca96-10f3-4823-be16-117556b2afc3",
    "Steve Ellis",
    CardRules::new_creature(mana_cost!("{1}{R/G}{R/G}"), &["Noggle", "Rogue"], 3, 3)
        .with_abilities(&[AbilityDef::triggered(
            "When this creature enters or dies, create a Treasure token. \
             (It's an artifact with \"{T}, Sacrifice this token: Add one \
             mana of any color.\")",
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
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        )]),
);

// ECL 238 — Prideful Feastling
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static PRIDEFUL_FEASTLING: CardRecord = CardRecord::new(
    "Prideful Feastling",
    "7578cb61-f606-4559-a10a-343a583228ab",
    "Raph Lomotan",
    CardRules::unsupported(),
);

// ECL 239 — Raiding Schemes
// Audit: unsupported — Needs a conspire additional-cost offer tapping two creatures that each share a color with the prospective spell, followed by the paid-cost copy trigger.
pub(in crate::card::sets) static RAIDING_SCHEMES: CardRecord = CardRecord::new(
    "Raiding Schemes",
    "0bab5da5-72a7-4340-9b53-492ab14a9f71",
    "Justin Gerard",
    CardRules::unsupported(),
);

// ECL 240 — Reaping Willow
// Audit: unsupported — Needs an activation payment selecting one or several counters of arbitrary kinds on the source; existing removal costs require one fixed CounterKind and cannot pay with a mixture of kinds.
pub(in crate::card::sets) static REAPING_WILLOW: CardRecord = CardRecord::new(
    "Reaping Willow",
    "97c33cc2-3573-410a-908b-c0392fff524b",
    "Igor Krstic",
    CardRules::unsupported(),
);

// ECL 241 — Sanar, Innovative First-Year
// Audit: unsupported — Needs reveal-until a computed number of nonlands, then one optionally selected card per represented color before a single shuffle; existing collection selections cannot express the color-to-card assignment.
pub(in crate::card::sets) static SANAR_INNOVATIVE_FIRST_YEAR: CardRecord = CardRecord::new(
    "Sanar, Innovative First-Year",
    "11215561-bbcd-4564-a2e4-a1d77d177a1d",
    "Steven Belledin",
    CardRules::unsupported(),
);

// ECL 242 — Shadow Urchin
// Audit: unsupported — Needs the total count across all counter kinds on a dying creature's last-known incarnation and a next-end-step play-permission expiration.
pub(in crate::card::sets) static SHADOW_URCHIN: CardRecord = CardRecord::new(
    "Shadow Urchin",
    "4e54c39b-6149-467b-a9a8-7ad09ca0cbd4",
    "Ron Spencer",
    CardRules::unsupported(),
);

// ECL 243 — Stoic Grove-Guide
pub(in crate::card::sets) static STOIC_GROVE_GUIDE: CardRecord = CardRecord::new(
    "Stoic Grove-Guide",
    "3d5a3e25-c17a-47b1-a36d-d24d50e5bab3",
    "Tran Nguyen",
    CardRules::new_creature(mana_cost!("{4}{B/G}"), &["Elf", "Druid"], 5, 4).with_abilities(&[
        AbilityDef::activated(
            "{1}{B/G}, Exile this card from your graveyard: Create a 2/2 \
             black and green Elf creature token. Activate only as a \
             sorcery.",
            &[CostDef::Mana(mana_cost!("{1}{B/G}")), CostDef::ExileSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELF_TOKEN))),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// ECL 244 — Sygg's Command
pub(in crate::card::sets) static SYGG_S_COMMAND: CardRecord = CardRecord::new(
    "Sygg's Command",
    "8bc13fc2-e254-4db5-ad4d-f92711a1a6ca",
    "Margaret Organ-Kean",
    CardRules::new_sorcery(mana_cost!("{1}{W}{U}"))
        .with_subtypes(&["Merfolk"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose two —",
            &[
                AbilityDef::spell_with_targets(
                    "Create a token that's a copy of target Merfolk you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
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
                AbilityDef::spell_with_targets(
                    "Creatures target player controls gain lifelink until end of turn.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::controlled_by(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerSetDef::LegalTargets(TargetIndex(0)),
                            ),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target player draws a card.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Tap target creature. Put a stun counter on it.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
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
                ),
            ],
        )
        .with_mode_selection(2, 2, false)])
        .with_type(CardType::Kindred),
);

// ECL 245 — Tam, Mindful First-Year
// Audit: unsupported — Needs each recipient's current colors used as a hexproof-from source predicate, including several colors at once; source-relative color predicates do not read the protected recipient.
pub(in crate::card::sets) static TAM_MINDFUL_FIRST_YEAR: CardRecord = CardRecord::new(
    "Tam, Mindful First-Year",
    "6cb0f825-b75b-4f2a-803c-08142ca07e76",
    "Zoltan Boros",
    CardRules::unsupported(),
);

// ECL 246 — Thoughtweft Lieutenant
pub(in crate::card::sets) static THOUGHTWEFT_LIEUTENANT: CardRecord = CardRecord::new(
    "Thoughtweft Lieutenant",
    "2c54ec67-9317-455e-a045-fa4ed9cb676f",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Kithkin", "Soldier"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another Kithkin you control enters, \
             target creature you control gets +1/+1 and gains trample \
             until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Source,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Kithkin")),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ECL 247 — Trystan's Command
pub(in crate::card::sets) static TRYSTAN_S_COMMAND: CardRecord = CardRecord::new(
    "Trystan's Command",
    "ba1d99ca-740a-481a-be89-615e40d56d06",
    "Sam Guay",
    CardRules::new_sorcery(mana_cost!("{4}{B}{G}"))
        .with_subtypes(&["Elf"])
        .with_abilities(&[AbilityDef::modal_spell(
            "Choose two —",
            &[
                AbilityDef::spell_with_targets(
                    "Create a token that's a copy of target Elf you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Elf")),
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
                AbilityDef::spell_with_targets(
                    "Return one or two target permanent cards from your graveyard \
                     to your hand.",
                    &[AbilityTargetDef {
                        minimum: 1,
                        ..AbilityTargetDef::up_to(
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
                    "Destroy target creature or enchantment.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Creatures target player controls get +3/+3 until end of turn. \
                     Untap them.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                    )],
                    EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::controlled_by(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerSetDef::LegalTargets(TargetIndex(0)),
                                ),
                            )),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(3),
                                ValueDef::Constant(3),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                        EffectDef::Untap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::controlled_by(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerSetDef::LegalTargets(TargetIndex(0)),
                                ),
                            )),
                        },
                    ]),
                ),
            ],
        )
        .with_mode_selection(2, 2, false)])
        .with_type(CardType::Kindred),
);

// ECL 248 — Twinflame Travelers
// Audit: unsupported — Needs triggered-ability duplication filtered by the triggering ability's source being another controlled Elemental; existing duplication rules only specialize arrival-trigger classes.
pub(in crate::card::sets) static TWINFLAME_TRAVELERS: CardRecord = CardRecord::new(
    "Twinflame Travelers",
    "5fc9f409-3aef-4403-bf17-6e9a72ecfada",
    "Jeff Miracola",
    CardRules::unsupported(),
);

// ECL 249 — Vibrance
// Audit: unsupported — Needs the number of mana units of each color spent to cast the source retained for its enters triggers; ColorsOfManaSpent counts distinct colors and does not distinguish one unit from two of the same color.
pub(in crate::card::sets) static VIBRANCE: CardRecord = CardRecord::new(
    "Vibrance",
    "b9f71c3b-0840-475f-9c17-fdacbc7f3213",
    "Jakub Kasper",
    CardRules::unsupported(),
);

// ECL 250 — Voracious Tome-Skimmer
pub(in crate::card::sets) static VORACIOUS_TOME_SKIMMER: CardRecord = CardRecord::new(
    "Voracious Tome-Skimmer",
    "1a696fe5-410c-4699-b1d1-1a9c771d664a",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{U/B}{U/B}{U/B}"), &["Faerie", "Rogue"], 2, 3)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever you cast a spell during an opponent's turn, you may \
                 pay 1 life. If you do, draw a card.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Any,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ])),
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
                },
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::PayLife(1)],
                    &abilities::draw_cards(ValueDef::Constant(1)),
                )),
            ),
        ]),
);

// ECL 251 — Wary Farmer
// Audit: unsupported — Needs creature-entry history excluding the source incarnation and retained after the entrant leaves the battlefield or changes controller.
pub(in crate::card::sets) static WARY_FARMER: CardRecord = CardRecord::new(
    "Wary Farmer",
    "22d20c0d-176d-49c9-aa0b-2c5778548cc5",
    "Ron Spears",
    CardRules::unsupported(),
);

// ECL 252 — Wistfulness
// Audit: unsupported — Needs the number of mana units of each color spent to cast the source retained for its enters triggers; ColorsOfManaSpent counts distinct colors and does not distinguish one unit from two of the same color.
pub(in crate::card::sets) static WISTFULNESS: CardRecord = CardRecord::new(
    "Wistfulness",
    "db9aa986-ac2a-44bb-a88b-04c5d0d502b2",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// ECL 253 — Chronicle of Victory
pub(in crate::card::sets) static CHRONICLE_OF_VICTORY: CardRecord = CardRecord::new(
    "Chronicle of Victory",
    "b3c2d68d-690b-41e7-99ed-2d20c7e0a9b4",
    "Aldo Domínguez",
    CardRules::new_artifact(mana_cost!("{6}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::as_enters(
                "As Chronicle of Victory enters, choose a creature type.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
                )),
            ),
            AbilityDef::static_ability(
                "Creatures you control of the chosen type get +2/+2 and have \
                 first strike and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasSourcesChosenScalar(
                                    BattlefieldEntryChoiceDestinationDef::CreatureType,
                                ),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "Whenever you cast a spell of the chosen type, draw a card.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasSourcesChosenScalar(
                        BattlefieldEntryChoiceDestinationDef::CreatureType,
                    ),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// ECL 254 — Dawn-Blessed Pennant
// Audit: unsupported — Needs an entry creature-type choice restricted to the eight printed alternatives; ScalarChoiceListDef::CreatureTypes offers the entire vocabulary.
pub(in crate::card::sets) static DAWN_BLESSED_PENNANT: CardRecord = CardRecord::new(
    "Dawn-Blessed Pennant",
    "294266b6-0343-4fa5-90b8-0adf7df490e4",
    "Igor Krstic",
    CardRules::unsupported(),
);

// ECL 255 — Firdoch Core
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static FIRDOCH_CORE: CardRecord = CardRecord::new(
    "Firdoch Core",
    "8e45cd37-bf97-4742-978d-96f96ed653cd",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// ECL 256 — Foraging Wickermaw
// Audit: unsupported — Needs the color selected for an immediate mana ability retained for a same-resolution source-color change; AddMana chooses its type internally without a reusable color binding.
pub(in crate::card::sets) static FORAGING_WICKERMAW: CardRecord = CardRecord::new(
    "Foraging Wickermaw",
    "f524bc08-caeb-4362-b960-eb8e0e4159d0",
    "Ron Spencer",
    CardRules::unsupported(),
);

// ECL 257 — Gathering Stone
pub(in crate::card::sets) static GATHERING_STONE: CardRecord = CardRecord::new(
    "Gathering Stone",
    "81dfbfe1-143a-4637-b683-a34cfc51993d",
    "Paolo Parente",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this artifact enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        AbilityDef::static_ability(
            "Spells you cast of the chosen type cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef {
                spell: ObjectPredicateDef::HasSourcesChosenScalar(
                    BattlefieldEntryChoiceDestinationDef::CreatureType,
                ),
                caster: PlayerRelation::You,
                condition: SpellCostConditionDef::Always,
                adjustment: CostAdjustmentDef::Subtract(CostAmountDef::Generic(
                    ValueDef::Constant(1),
                )),
            })),
        ),
        AbilityDef::triggered(
            "When this artifact enters and at the beginning of your \
             upkeep, look at the top card of your library. If it's a card \
             of the chosen type, you may reveal it and put it into your \
             hand. If you don't put the card into your hand, you may put \
             it into your graveyard.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
            ]),
            EffectDef::BindObjects(BindObjectsDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                },
                binding: crate::Binding!("top"),
                then: &EffectDef::Sequence(&[
                    EffectDef::LookAtObjects(LookAtObjectsDef {
                        actor: PlayerRefDef::EffectController,
                        source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Binding(
                            crate::Binding!("top"),
                        )),
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::None,
                    }),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("top")),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::HasSourcesChosenScalar(
                                        BattlefieldEntryChoiceDestinationDef::CreatureType,
                                    ),
                                ),
                            },
                        ),
                        then: &EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Binding(crate::Binding!("top")),
                            exclude: None,
                            minimum: 0,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Private,
                            then: &EffectDef::Sequence(&[
                                EffectDef::RevealObjects(RevealObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                                    then: &EffectDef::None,
                                }),
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("chosen"),
                                    )),
                                    ZoneKind::Hand,
                                    ZonePlacement::Top,
                                ),
                            ]),
                        }),
                    },
                    EffectDef::May {
                        player: EffectRecipientDef::Controller,
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "top"
                            ))),
                            ZoneKind::Graveyard,
                            ZonePlacement::Top,
                        ),
                    },
                ]),
            }),
        ),
    ]),
);

// ECL 258 — Mirrormind Crown
// Audit: unsupported — Needs a first-token-creation-event ordinal for the turn and an optional copy replacement preserving the original batch size; a per-source replacement limit does not identify the turn's first token event.
pub(in crate::card::sets) static MIRRORMIND_CROWN: CardRecord = CardRecord::new(
    "Mirrormind Crown",
    "061d765e-df27-406a-9ba0-b51b0cbb65da",
    "Dan Frazier",
    CardRules::unsupported(),
);

// ECL 259 — Puca's Eye
pub(in crate::card::sets) static PUCA_S_EYE: CardRecord = CardRecord::new(
    "Puca's Eye",
    "c3a5ca02-3829-4c42-b0dc-98b660f8a8f0",
    "Dan Frazier",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw a card, then choose a color. \
             This artifact becomes the chosen color.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::ChooseColor {
                    object: EffectRecipientDef::Source,
                    operation: ColorChoiceOperationDef::BecomesChosenColor,
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        ),
        AbilityDef::activated(
            "{3}, {T}: Draw a card. Activate only if there are five colors \
             among permanents you control.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .with_activation_condition(&TriggerConditionDef::All(&[
            TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::White),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Black),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Red),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Green),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
        ])),
    ]),
);

// ECL 260 — Springleaf Drum (reprint)
const SPRINGLEAF_DRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::SPRINGLEAF_DRUM,
    "e15ab0aa-4059-4923-9816-6f7a9e5b5a18",
    "Cory Godbey",
);

// ECL 261 — Stalactite Dagger
// Audit: unsupported — Needs an all-zone creature-type characteristic-defining ability whose all-types value is copiable; battlefield all-type modifiers do not implement changeling.
pub(in crate::card::sets) static STALACTITE_DAGGER: CardRecord = CardRecord::new(
    "Stalactite Dagger",
    "6954df09-95f3-46cf-9ba8-2a1aea653d8f",
    "Drew Tucker",
    CardRules::unsupported(),
);

// ECL 262 — Blood Crypt (reprint)
const BLOOD_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::BLOOD_CRYPT,
    "6da63cc5-4624-4491-abd9-9b600c3fefe2",
    "Adam Paquette",
);

// ECL 263 — Eclipsed Realms
// Audit: unsupported — Needs a restricted eight-type entry choice and one mana permission allowing either casting that type or activating a source of that type; the existing restriction list is conjunctive.
pub(in crate::card::sets) static ECLIPSED_REALMS: CardRecord = CardRecord::new(
    "Eclipsed Realms",
    "a174f0db-8b4f-4c37-9583-44c92d37b9c0",
    "Alayna Danner",
    CardRules::unsupported(),
);

// ECL 264 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::EVOLVING_WILDS,
    "8c632984-5176-4c37-91df-6577cc294b85",
    "Alayna Danner",
);

// ECL 265 — Hallowed Fountain (reprint)
const HALLOWED_FOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::HALLOWED_FOUNTAIN,
    "e056b55f-82ed-4fe0-ab0c-bb20fa4a218a",
    "Adam Paquette",
);

// ECL 266 — Overgrown Tomb (reprint)
const OVERGROWN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::OVERGROWN_TOMB,
    "45b92924-baa1-4c9b-9932-9a5eda8f3446",
    "Adam Paquette",
);

// ECL 267 — Steam Vents (reprint)
const STEAM_VENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::STEAM_VENTS,
    "b66daa94-d367-4812-9f18-f35378c1febb",
    "Adam Paquette",
);

// ECL 268 — Temple Garden (reprint)
const TEMPLE_GARDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::TEMPLE_GARDEN,
    "6cdd2a74-63b3-4ff2-9c5a-a85dee63c3c9",
    "Adam Paquette",
);

// ECL 269 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "3a438199-54f8-4702-81cf-a9d42e7cd9f1",
    "Zoltan Boros",
);

// ECL 270 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "b0da67fb-1cb2-4105-ab5c-b7c680b8116c",
    "Ron Spears",
);

// ECL 271 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "1dd4d605-02a2-4183-b191-0bca8dfbf962",
    "Jorge Jacinto",
);

// ECL 272 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "295b92bc-d66f-45d8-9bbe-5f5f13e39fd4",
    "Raymond Bonilla",
);

// ECL 273 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "b460f5f7-c7c9-400c-8419-23d614f45bf9",
    "Jorge Jacinto",
);

// ECL 274 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "6242dbef-8412-4ebd-9486-42cec1dc6794",
    "Justin Gerard",
);

// ECL 275 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "12ebd9c8-6587-456e-aba9-7aad1c2a09ea",
    "Annie Stegg",
);

// ECL 276 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "0cf904da-496c-4d88-a62b-c736ba895078",
    "Raoul Vitale",
);

// ECL 277 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "9b6fe3e2-a2cf-4209-ac9b-e04d02599360",
    "Ralph Horsley",
);

// ECL 278 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "b43297de-7378-474f-a85f-910fa7cbb4f4",
    "Jason Mowry",
);

// ECL 279 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "d8fdfa7d-fd11-4c11-8743-a21538474314",
    "Justin Gerard",
);

// ECL 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "d6a5ba11-3156-4a0c-958d-5756e18b767b",
    "Annie Stegg",
);

// ECL 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "35fe42f9-dd55-4ca4-af0a-59ecdff0dba8",
    "Raoul Vitale",
);

// ECL 282 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "96e14936-5614-46ba-874c-c1357243fe02",
    "Ralph Horsley",
);

// ECL 283 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "bdbce923-c05e-4554-8c4c-5c4e6d791856",
    "Jason Mowry",
);

// ECL 284 — Ajani, Outland Chaperone (alternate printing)
const AJANI_OUTLAND_CHAPERONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AJANI_OUTLAND_CHAPERONE,
    1,
    "b5734a2c-0e8f-4087-97ee-51e26cfbf62d",
    "Greg Staples",
);

// ECL 285 — Brigid, Clachan's Heart // Brigid, Doun's Mind (alternate printing)
const BRIGID_CLACHAN_S_HEART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRIGID_CLACHAN_S_HEART,
    1,
    "887002ae-794b-427c-9f98-e909b22313d9",
    "Jesper Ejsing",
);

// ECL 286 — Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight (alternate printing)
const EIRDU_CARRIER_OF_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EIRDU_CARRIER_OF_DAWN,
    1,
    "ec3c80c4-4935-455c-a14f-a0532b6a41a8",
    "Omar Rayyan",
);

// ECL 287 — Oko, Lorwyn Liege // Oko, Shadowmoor Scion (alternate printing)
const OKO_LORWYN_LIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OKO_LORWYN_LIEGE,
    1,
    "48d43194-cbcb-472f-b1c8-b97e347b654c",
    "Steve Prescott",
);

// ECL 288 — Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield (alternate printing)
const SYGG_WANDERWINE_WISDOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYGG_WANDERWINE_WISDOM,
    1,
    "62b5ab41-7a85-49aa-8669-8a09a09d02fa",
    "Warren Mahy",
);

// ECL 289 — Grub, Storied Matriarch // Grub, Notorious Auntie (alternate printing)
const GRUB_STORIED_MATRIARCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRUB_STORIED_MATRIARCH,
    1,
    "0f836c3b-d1fb-4b3b-9c14-cc9052be3a94",
    "Zoltan Boros",
);

// ECL 290 — Ashling, Rekindled // Ashling, Rimebound (alternate printing)
const ASHLING_REKINDLED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHLING_REKINDLED,
    1,
    "7110cae6-b0ed-4495-9f36-751ce25c9538",
    "Chuck Lukacs",
);

// ECL 291 — Trystan, Callous Cultivator // Trystan, Penitent Culler (alternate printing)
const TRYSTAN_CALLOUS_CULTIVATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRYSTAN_CALLOUS_CULTIVATOR,
    1,
    "4086c9a8-f9bf-4359-a67a-9f7b45528ccf",
    "Larry MacDougall",
);

// ECL 292 — Catharsis (alternate printing)
const CATHARSIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CATHARSIS,
    1,
    "f7604799-1d32-4c6c-92c6-0d1b274a1115",
    "Wayne Reynolds",
);

// ECL 293 — Deceit (alternate printing)
const DECEIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DECEIT,
    1,
    "0a1c101b-3fad-421c-a081-69e75db95c46",
    "Kev Walker",
);

// ECL 294 — Emptiness (alternate printing)
const EMPTINESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMPTINESS,
    1,
    "9f53e76d-3038-41f7-b050-bb9d7c7d2ea3",
    "Jeff Miracola",
);

// ECL 295 — Vibrance (alternate printing)
const VIBRANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIBRANCE,
    1,
    "ea49c01f-3f32-4c66-893c-56931e0e8981",
    "Mark Zug",
);

// ECL 296 — Wistfulness (alternate printing)
const WISTFULNESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WISTFULNESS,
    1,
    "280eb666-e918-4640-9c43-19bc71af06b6",
    "Paolo Parente",
);

// ECL 297 — Adept Watershaper (alternate printing)
const ADEPT_WATERSHAPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ADEPT_WATERSHAPER,
    1,
    "8700c0ce-2a71-4bc8-a14c-685b5d188b54",
    "Julie Benbassat",
);

// ECL 298 — Curious Colossus (alternate printing)
const CURIOUS_COLOSSUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURIOUS_COLOSSUS,
    1,
    "aef1f79b-c536-4bf3-ab71-0a812c4c0728",
    "Hayden Goodman",
);

// ECL 299 — Kinbinding (alternate printing)
const KINBINDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KINBINDING,
    1,
    "c6ef5697-35ef-4603-92b9-ed1993f5d401",
    "Yas Imamura",
);

// ECL 300 — Kinscaer Sentry (alternate printing)
const KINSCAER_SENTRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KINSCAER_SENTRY,
    1,
    "bf7ab51b-301e-48fe-b1e8-0c33d057e7de",
    "Felicita Sala",
);

// ECL 301 — Morningtide's Light (alternate printing)
const MORNINGTIDE_S_LIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORNINGTIDE_S_LIGHT,
    1,
    "d0de86f5-4dd5-415f-ba9a-6321986fc9e2",
    "adelinaillustration",
);

// ECL 302 — Slumbering Walker (alternate printing)
const SLUMBERING_WALKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLUMBERING_WALKER,
    1,
    "69f6248e-d8d0-40eb-a89e-bc3017327711",
    "Rebecca Green",
);

// ECL 303 — Disruptor of Currents (alternate printing)
const DISRUPTOR_OF_CURRENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DISRUPTOR_OF_CURRENTS,
    1,
    "29c2c66e-a3d3-4993-82cc-adf8f6877204",
    "Julie Benbassat",
);

// ECL 304 — Flitterwing Nuisance (alternate printing)
const FLITTERWING_NUISANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FLITTERWING_NUISANCE,
    1,
    "7ecc95a7-51c8-45a1-bebf-9dfa053f7403",
    "Serena Malyon",
);

// ECL 305 — Glen Elendra Guardian (alternate printing)
const GLEN_ELENDRA_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_GUARDIAN,
    1,
    "8b8996a5-4d22-42d3-a706-44c258c938e9",
    "Danny Schwartz",
);

// ECL 306 — Glen Elendra's Answer (alternate printing)
const GLEN_ELENDRA_S_ANSWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_S_ANSWER,
    1,
    "8375ad0c-5d8d-47f6-bd04-ffd59fa9c0be",
    "Matthew Forsythe",
);

// ECL 307 — Loch Mare (alternate printing)
const LOCH_MARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOCH_MARE,
    1,
    "539a7e8f-97a9-4aa2-957b-0cc30de797bb",
    "Isabella Mazzanti",
);

// ECL 308 — Mirrorform (alternate printing)
const MIRRORFORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIRRORFORM,
    1,
    "02a9adde-1604-46c1-b481-fa0a73c72bd9",
    "Felicita Sala",
);

// ECL 309 — Sunderflock (alternate printing)
const SUNDERFLOCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNDERFLOCK,
    1,
    "ee7486fc-844d-4515-9028-8d53ba396a61",
    "Danny Schwartz",
);

// ECL 310 — Bitterbloom Bearer (alternate printing)
const BITTERBLOOM_BEARER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BITTERBLOOM_BEARER,
    1,
    "58aa273f-9ec7-4024-b30e-e75fa404de72",
    "Taryn Knight",
);

// ECL 311 — Dawnhand Dissident (alternate printing)
const DAWNHAND_DISSIDENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAWNHAND_DISSIDENT,
    1,
    "fea91416-2836-447f-ad07-8b96ba727cbd",
    "Lauren Degraaf",
);

// ECL 312 — Gloom Ripper (alternate printing)
const GLOOM_RIPPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLOOM_RIPPER,
    1,
    "d81ca098-2dca-48a2-876d-06e89c2b1d1c",
    "Isabella Mazzanti",
);

// ECL 313 — Moonshadow (alternate printing)
const MOONSHADOW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONSHADOW,
    1,
    "4366c1d7-d5b7-4f63-94fa-9d0e5edd9a73",
    "Julie Benbassat",
);

// ECL 314 — Taster of Wares (alternate printing)
const TASTER_OF_WARES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TASTER_OF_WARES,
    1,
    "8a6c2e99-2d4c-455a-a394-55ff32a95142",
    "Felicita Sala",
);

// ECL 315 — Twilight Diviner (alternate printing)
const TWILIGHT_DIVINER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWILIGHT_DIVINER,
    1,
    "40e615a7-93a9-4753-a96c-a18237ab240b",
    "Isabella Mazzanti",
);

// ECL 316 — Goliath Daydreamer (alternate printing)
const GOLIATH_DAYDREAMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLIATH_DAYDREAMER,
    1,
    "b5e1905d-4fd8-4923-8612-75678aa3f212",
    "Vanessa Gillings",
);

// ECL 317 — Hexing Squelcher (alternate printing)
const HEXING_SQUELCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEXING_SQUELCHER,
    1,
    "68618675-3e00-4b07-b1da-0e4be5700a1c",
    "Matthew Forsythe",
);

// ECL 318 — Lavaleaper (alternate printing)
const LAVALEAPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAVALEAPER,
    1,
    "0eee01c8-6588-407c-9e19-97699e43e713",
    "Matthew Forsythe",
);

// ECL 319 — Meek Attack (alternate printing)
const MEEK_ATTACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEEK_ATTACK,
    1,
    "7d6a0d52-08af-45d1-9c4c-3d694c40b372",
    "Matt Rockefeller",
);

// ECL 320 — Scuzzback Scrounger (alternate printing)
const SCUZZBACK_SCROUNGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCUZZBACK_SCROUNGER,
    1,
    "9377b69c-9677-4865-a0c4-af0bfd7ebfd5",
    "Matthew Forsythe",
);

// ECL 321 — Soul Immolation (alternate printing)
const SOUL_IMMOLATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOUL_IMMOLATION,
    1,
    "c79d97c0-76aa-439c-b47d-dc4b00ffda96",
    "Serena Malyon",
);

// ECL 322 — Spinerock Tyrant (alternate printing)
const SPINEROCK_TYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPINEROCK_TYRANT,
    1,
    "65825a3c-a442-476d-b594-e61d6104dffa",
    "Danny Schwartz",
);

// ECL 323 — Aurora Awakener (alternate printing)
const AURORA_AWAKENER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AURORA_AWAKENER,
    1,
    "9c87e3f0-4305-4b07-a92b-b61430e80e97",
    "Matthew Forsythe",
);

// ECL 324 — Bloom Tender (alternate printing)
const BLOOM_TENDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_eve::BLOOM_TENDER,
    1,
    "46aa1d12-ee23-4b0f-b574-7f1a625ca279",
    "Danny Schwartz",
);

// ECL 325 — Bristlebane Battler (alternate printing)
const BRISTLEBANE_BATTLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRISTLEBANE_BATTLER,
    1,
    "7028368a-5711-416c-b5d9-874ef373335d",
    "Matt Rockefeller",
);

// ECL 326 — Celestial Reunion (alternate printing)
const CELESTIAL_REUNION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CELESTIAL_REUNION,
    1,
    "e9777cbc-064c-4c11-9a43-8d44f7e8e403",
    "Serena Malyon",
);

// ECL 327 — Mutable Explorer (alternate printing)
const MUTABLE_EXPLORER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MUTABLE_EXPLORER,
    1,
    "8d10a060-5abd-4982-996c-a8fc420e44ba",
    "Felicita Sala",
);

// ECL 328 — Sapling Nursery (alternate printing)
const SAPLING_NURSERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAPLING_NURSERY,
    1,
    "ff984cb9-cd72-47a4-adce-2521c025679b",
    "Julie Benbassat",
);

// ECL 329 — Spry and Mighty (alternate printing)
const SPRY_AND_MIGHTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPRY_AND_MIGHTY,
    1,
    "ddc88ce9-716c-4328-95ec-f1c336d04c33",
    "Matthew Forsythe",
);

// ECL 330 — Ashling's Command (alternate printing)
const ASHLING_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHLING_S_COMMAND,
    1,
    "201c73b6-368f-4dac-b034-49c6aa6408ff",
    "adelinaillustration",
);

// ECL 331 — Boggart Cursecrafter (alternate printing)
const BOGGART_CURSECRAFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOGGART_CURSECRAFTER,
    1,
    "a942f56d-69ee-4151-baae-9a718ff6e7b6",
    "Heikala",
);

// ECL 332 — Brigid's Command (alternate printing)
const BRIGID_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRIGID_S_COMMAND,
    1,
    "70b67bc1-0535-48bb-9f5c-4f43ddeb16cb",
    "Rebecca Green",
);

// ECL 333 — Deepchannel Duelist (alternate printing)
const DEEPCHANNEL_DUELIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEPCHANNEL_DUELIST,
    1,
    "0fe0730e-a549-427d-99be-68d6fded5ac5",
    "Lauren Degraaf",
);

// ECL 334 — Doran, Besieged by Time (alternate printing)
const DORAN_BESIEGED_BY_TIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DORAN_BESIEGED_BY_TIME,
    1,
    "9f0a1644-58bb-4826-9832-0ffc372fac1e",
    "Serena Malyon",
);

// ECL 335 — Eclipsed Boggart (alternate printing)
const ECLIPSED_BOGGART_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_BOGGART,
    1,
    "343a04c9-ebf8-41d9-9ebf-9d17da5ad605",
    "Yas Imamura",
);

// ECL 336 — Eclipsed Elf (alternate printing)
const ECLIPSED_ELF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_ELF,
    1,
    "57708ef7-ef10-49c4-8cf7-7055cf7280a0",
    "Heikala",
);

// ECL 337 — Eclipsed Flamekin (alternate printing)
const ECLIPSED_FLAMEKIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_FLAMEKIN,
    1,
    "6271b82d-5922-4de0-b763-129908ece6ee",
    "Vanessa Gillings",
);

// ECL 338 — Eclipsed Kithkin (alternate printing)
const ECLIPSED_KITHKIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_KITHKIN,
    1,
    "e40d5d7c-4176-4faa-9701-933307eebb26",
    "Yas Imamura",
);

// ECL 339 — Eclipsed Merrow (alternate printing)
const ECLIPSED_MERROW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECLIPSED_MERROW,
    1,
    "45d96555-4976-49d1-b529-09cc7ca37cd3",
    "Felicita Sala",
);

// ECL 340 — Grub's Command (alternate printing)
const GRUB_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRUB_S_COMMAND,
    1,
    "54a38e72-b308-410f-b182-454c935e2242",
    "Phoebe Wahl",
);

// ECL 341 — Morcant's Loyalist (alternate printing)
const MORCANT_S_LOYALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORCANT_S_LOYALIST,
    1,
    "c157b97e-905e-4c68-b6ae-9c43caa7c6e7",
    "Yas Imamura",
);

// ECL 342 — Sygg's Command (alternate printing)
const SYGG_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYGG_S_COMMAND,
    1,
    "a2cb17e2-982e-4914-ab09-6edcf2546509",
    "Felicita Sala",
);

// ECL 343 — Thoughtweft Lieutenant (alternate printing)
const THOUGHTWEFT_LIEUTENANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THOUGHTWEFT_LIEUTENANT,
    1,
    "fea71783-1ac5-483b-8a91-1ba45af4ea3c",
    "Vanessa Gillings",
);

// ECL 344 — Trystan's Command (alternate printing)
const TRYSTAN_S_COMMAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRYSTAN_S_COMMAND,
    1,
    "aefed846-e367-4940-842f-b504c71d0a53",
    "Yas Imamura",
);

// ECL 345 — Twinflame Travelers (alternate printing)
const TWINFLAME_TRAVELERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TRAVELERS,
    1,
    "a422ae59-abb6-423d-8400-670196560920",
    "adelinaillustration",
);

// ECL 346 — Chronicle of Victory (alternate printing)
const CHRONICLE_OF_VICTORY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHRONICLE_OF_VICTORY,
    1,
    "44407660-79ed-442d-a215-d388c27c5130",
    "adelinaillustration",
);

// ECL 347 — Hallowed Fountain // Hallowed Fountain (alternate printing)
const HALLOWED_FOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::HALLOWED_FOUNTAIN,
    1,
    "19cba6be-7291-4788-9241-87dad3b68363",
    "Justin Gerard",
);

// ECL 348 — Steam Vents // Steam Vents (alternate printing)
const STEAM_VENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::STEAM_VENTS,
    1,
    "eb96c335-9ed3-4f7d-b07a-185ff4044976",
    "Raoul Vitale",
);

// ECL 349 — Blood Crypt // Blood Crypt (alternate printing)
const BLOOD_CRYPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::BLOOD_CRYPT,
    1,
    "28d37803-d3f9-4806-9a0b-79362a517a7a",
    "Valera Lutfullina",
);

// ECL 350 — Overgrown Tomb // Overgrown Tomb (alternate printing)
const OVERGROWN_TOMB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::OVERGROWN_TOMB,
    1,
    "307fdde3-43f6-4d80-884a-4a85314ab8e4",
    "Matt Stewart",
);

// ECL 351 — Temple Garden // Temple Garden (alternate printing)
const TEMPLE_GARDEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::TEMPLE_GARDEN,
    1,
    "ec75b840-420d-4512-a74a-d70f19be0085",
    "Annie Stegg",
);

// ECL 352 — Bitterbloom Bearer (alternate printing)
const BITTERBLOOM_BEARER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BITTERBLOOM_BEARER,
    2,
    "895e7a6e-cd10-4a00-bfec-30e6330e193c",
    "Rebecca Guay",
);

// ECL 353 — Champion of the Clachan (alternate printing)
const CHAMPION_OF_THE_CLACHAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPION_OF_THE_CLACHAN,
    1,
    "8a91a96d-843c-4ec6-8078-89f87c5ce85a",
    "Edgar Sánchez Hidalgo",
);

// ECL 354 — Rhys, the Evermore (alternate printing)
const RHYS_THE_EVERMORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RHYS_THE_EVERMORE,
    1,
    "c3bef4e2-d5f4-4e95-a212-d8e02bef99f7",
    "Kai Carpenter",
);

// ECL 355 — Winnowing (alternate printing)
const WINNOWING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINNOWING,
    1,
    "6e434b0d-d438-432c-9684-5d3edf30790a",
    "David Palumbo",
);

// ECL 356 — Champions of the Shoal (alternate printing)
const CHAMPIONS_OF_THE_SHOAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPIONS_OF_THE_SHOAL,
    1,
    "cbb067b0-1bb6-467c-b361-2c9ed6bd3a22",
    "Daniel Zrom",
);

// ECL 357 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    1,
    "42465c20-16f8-46a2-96fe-b52a9eac3df9",
    "Tyler Walpole",
);

// ECL 358 — Rimefire Torque (alternate printing)
const RIMEFIRE_TORQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIMEFIRE_TORQUE,
    1,
    "23d41fc1-5b74-4ab5-9b14-43aaf3a7d125",
    "Jorge Jacinto",
);

// ECL 359 — Bloodline Bidding (alternate printing)
const BLOODLINE_BIDDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOODLINE_BIDDING,
    1,
    "f99ccd3b-07f7-47c2-8de9-6aa5251d7a76",
    "Drew Baker",
);

// ECL 360 — Champion of the Weird (alternate printing)
const CHAMPION_OF_THE_WEIRD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPION_OF_THE_WEIRD,
    1,
    "3aeaeb01-eab1-4bc6-bd58-c00a7e41a8ce",
    "Lucas Graciano",
);

// ECL 361 — Mornsong Aria (alternate printing)
const MORNSONG_ARIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORNSONG_ARIA,
    1,
    "05a553f3-3ff4-4601-bbf6-2f8b5ca6aaca",
    "Scott M. Fischer",
);

// ECL 362 — Champion of the Path (alternate printing)
const CHAMPION_OF_THE_PATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPION_OF_THE_PATH,
    1,
    "874a32a5-0a9b-4225-a877-bb6e74cd6c5f",
    "Tyler Walpole",
);

// ECL 363 — Collective Inferno (alternate printing)
const COLLECTIVE_INFERNO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLLECTIVE_INFERNO,
    1,
    "7250001d-a170-4b23-8290-6fdad48b4ac8",
    "Jason A. Engle",
);

// ECL 364 — End-Blaze Epiphany (alternate printing)
const END_BLAZE_EPIPHANY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &END_BLAZE_EPIPHANY,
    1,
    "d8adeb65-f6f4-4d21-bca7-71fe8ef71f3b",
    "Tyler Walpole",
);

// ECL 365 — Champions of the Perfect (alternate printing)
const CHAMPIONS_OF_THE_PERFECT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAMPIONS_OF_THE_PERFECT,
    1,
    "2e35eb02-d541-45fe-9de7-a3c6599a9b0c",
    "Chris Rahn",
);

// ECL 366 — Formidable Speaker (alternate printing)
const FORMIDABLE_SPEAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORMIDABLE_SPEAKER,
    1,
    "1444f39f-042b-4bb3-8307-6b8878198086",
    "Aurore Folny",
);

// ECL 367 — Selfless Safewright (alternate printing)
const SELFLESS_SAFEWRIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SELFLESS_SAFEWRIGHT,
    1,
    "ee08e321-6011-4545-920f-5b35180e896b",
    "Quintin Gleim",
);

// ECL 368 — Abigale, Eloquent First-Year (alternate printing)
const ABIGALE_ELOQUENT_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABIGALE_ELOQUENT_FIRST_YEAR,
    1,
    "c8d85a84-42a2-47d1-8fc2-1a081fd6bd2a",
    "Mark Zug",
);

// ECL 369 — Bre of Clan Stoutarm (alternate printing)
const BRE_OF_CLAN_STOUTARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRE_OF_CLAN_STOUTARM,
    1,
    "0f5f63df-f401-4c13-a105-e831fd841e80",
    "Jesper Ejsing",
);

// ECL 370 — Deepway Navigator (alternate printing)
const DEEPWAY_NAVIGATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEEPWAY_NAVIGATOR,
    1,
    "c0c2b1a7-d03e-4266-9bae-f2deebd714af",
    "Jacob Walker",
);

// ECL 371 — Dream Harvest (alternate printing)
const DREAM_HARVEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DREAM_HARVEST,
    1,
    "be97b021-0844-4bac-8d17-c96b4920744e",
    "Ben Hill",
);

// ECL 372 — Figure of Fable (alternate printing)
const FIGURE_OF_FABLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIGURE_OF_FABLE,
    1,
    "eb1dc48d-81c4-4ecb-b5b3-d75b198f61db",
    "Omar Rayyan",
);

// ECL 373 — High Perfect Morcant (alternate printing)
const HIGH_PERFECT_MORCANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_PERFECT_MORCANT,
    1,
    "fdb4192b-73ae-41e9-a3b7-a20c69350b79",
    "Victor Adame Minguez",
);

// ECL 374 — Kirol, Attentive First-Year (alternate printing)
const KIROL_ATTENTIVE_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KIROL_ATTENTIVE_FIRST_YEAR,
    1,
    "a77c3a75-4880-4b86-ab1e-92f045abc484",
    "Evyn Fong",
);

// ECL 375 — Lluwen, Imperfect Naturalist (alternate printing)
const LLUWEN_IMPERFECT_NATURALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LLUWEN_IMPERFECT_NATURALIST,
    1,
    "57289725-5ad2-46dd-a7f9-a78db67022e5",
    "Evyn Fong",
);

// ECL 376 — Maralen, Fae Ascendant (alternate printing)
const MARALEN_FAE_ASCENDANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARALEN_FAE_ASCENDANT,
    1,
    "4b1ace1f-84ec-499d-8a52-c0ad26a70d77",
    "Steve Prescott",
);

// ECL 377 — Raiding Schemes (alternate printing)
const RAIDING_SCHEMES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAIDING_SCHEMES,
    1,
    "0e04140a-11a1-4f4a-965a-5c3c6569595a",
    "Justin Gerard",
);

// ECL 378 — Sanar, Innovative First-Year (alternate printing)
const SANAR_INNOVATIVE_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANAR_INNOVATIVE_FIRST_YEAR,
    1,
    "31075628-a238-4ed5-b126-16ef5c23d364",
    "Steven Belledin",
);

// ECL 379 — Shadow Urchin (alternate printing)
const SHADOW_URCHIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHADOW_URCHIN,
    1,
    "ec1b5c33-2025-4d01-9b51-433a40a105ed",
    "Ron Spencer",
);

// ECL 380 — Tam, Mindful First-Year (alternate printing)
const TAM_MINDFUL_FIRST_YEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TAM_MINDFUL_FIRST_YEAR,
    1,
    "ca89fb7a-5575-4875-80d3-6a05f78ea65e",
    "Zoltan Boros",
);

// ECL 381 — Mirrormind Crown (alternate printing)
const MIRRORMIND_CROWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIRRORMIND_CROWN,
    1,
    "00b44e41-828b-4da4-b625-92943b8989ac",
    "Dan Frazier",
);

// ECL 382 — Winnowing (alternate printing)
const WINNOWING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WINNOWING,
    2,
    "9003e4f9-273f-4754-beaa-cd770c674a2b",
    "Yukoring",
);

// ECL 383 — Glen Elendra Guardian (alternate printing)
const GLEN_ELENDRA_GUARDIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_GUARDIAN,
    2,
    "a7c5b57c-f6da-4de0-9645-42e30c25ca32",
    "NgN",
);

// ECL 384 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    2,
    "31764ac0-6acb-496f-bb44-d556235bb47e",
    "konomura",
);

// ECL 385 — Bloodline Bidding (alternate printing)
const BLOODLINE_BIDDING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLOODLINE_BIDDING,
    2,
    "aca1b882-0fe9-4022-98e4-bae528b2a1e2",
    "morizo",
);

// ECL 386 — Moonshadow (alternate printing)
const MOONSHADOW_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOONSHADOW,
    2,
    "485a801c-0e10-4943-8456-852aacb24d9d",
    "Mizugoromaru",
);

// ECL 387 — Collective Inferno (alternate printing)
const COLLECTIVE_INFERNO_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COLLECTIVE_INFERNO,
    2,
    "72bd8ce5-795a-4503-b92c-a4a39f0bd4db",
    "YUE",
);

// ECL 388 — Meek Attack (alternate printing)
const MEEK_ATTACK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MEEK_ATTACK,
    2,
    "530aa72f-0a77-443b-9ad9-cce8d91b8657",
    "Yoshioka",
);

// ECL 389 — Spinerock Tyrant (alternate printing)
const SPINEROCK_TYRANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPINEROCK_TYRANT,
    2,
    "933f0eb4-7c8a-44c9-b91b-f3ff35081d1d",
    "D-suzuki",
);

// ECL 390 — Bloom Tender (alternate printing)
const BLOOM_TENDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_eve::BLOOM_TENDER,
    2,
    "0b5f1455-8124-408f-9ca5-9b281ec1708f",
    "Yuyuharu",
);

// ECL 391 — Selfless Safewright (alternate printing)
const SELFLESS_SAFEWRIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SELFLESS_SAFEWRIGHT,
    2,
    "28ab33be-cbfc-46eb-a5fc-9c770e1051d6",
    "Karuta Shiki",
);

// ECL 392 — Winnowing (alternate printing)
const WINNOWING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WINNOWING,
    3,
    "1384b7c7-81fe-4444-89a2-94c9de51df6a",
    "Yukoring",
);

// ECL 393 — Glen Elendra Guardian (alternate printing)
const GLEN_ELENDRA_GUARDIAN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GLEN_ELENDRA_GUARDIAN,
    3,
    "af008f52-d83d-455a-b217-7e40a8849a99",
    "NgN",
);

// ECL 394 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    3,
    "7b200330-6463-4ce0-828e-5e1a4f074478",
    "konomura",
);

// ECL 395 — Bloodline Bidding (alternate printing)
const BLOODLINE_BIDDING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BLOODLINE_BIDDING,
    3,
    "cc187974-f1e7-4fb4-9c81-51cd6b395974",
    "morizo",
);

// ECL 396 — Moonshadow (alternate printing)
const MOONSHADOW_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MOONSHADOW,
    3,
    "deb91ea3-f355-4aa2-a029-908cecc55fc2",
    "Mizugoromaru",
);

// ECL 397 — Collective Inferno (alternate printing)
const COLLECTIVE_INFERNO_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &COLLECTIVE_INFERNO,
    3,
    "2ada6e6f-77bd-49cb-8aa7-933acc02a0f2",
    "YUE",
);

// ECL 398 — Meek Attack (alternate printing)
const MEEK_ATTACK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MEEK_ATTACK,
    3,
    "ed87df86-3a71-444b-a7a0-8221b1627610",
    "Yoshioka",
);

// ECL 399 — Spinerock Tyrant (alternate printing)
const SPINEROCK_TYRANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPINEROCK_TYRANT,
    3,
    "10808193-6ddc-4d53-a3fa-32d7ba80a04f",
    "D-suzuki",
);

// ECL 400 — Bloom Tender (alternate printing)
const BLOOM_TENDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_eve::BLOOM_TENDER,
    3,
    "52c440f6-95f7-473a-8b81-0e423492f205",
    "Yuyuharu",
);

// ECL 401 — Selfless Safewright (alternate printing)
const SELFLESS_SAFEWRIGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SELFLESS_SAFEWRIGHT,
    3,
    "9a7a526e-b5cc-4a44-9a2c-722603343b21",
    "Karuta Shiki",
);

// ECL 402 — Personify (alternate printing)
const PERSONIFY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PERSONIFY,
    1,
    "08ed333b-358a-4e44-954e-562ecf5666b3",
    "Slawomir Maniak",
);

// ECL 403 — Silvergill Mentor (alternate printing)
const SILVERGILL_MENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILVERGILL_MENTOR,
    1,
    "3841d32f-4e41-409a-b1ec-3db9390d61a1",
    "Iris Compiet",
);

// ECL 404 — Iron-Shield Elf (alternate printing)
const IRON_SHIELD_ELF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_SHIELD_ELF,
    1,
    "3d47b1f2-da4a-42c4-992e-e0bfdd1a3e2a",
    "Adrián Rodríguez Pérez",
);

// ECL 405 — Sear (alternate printing)
const SEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEAR,
    1,
    "0a13f8e0-8b23-4553-ad76-9e52f8d4c63c",
    "Lars Grant-West",
);

// ECL 406 — Virulent Emissary (alternate printing)
const VIRULENT_EMISSARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VIRULENT_EMISSARY,
    1,
    "6044d540-70ff-4ce3-9a26-4164e7543baa",
    "Tiffany Turrill",
);

// ECL 407 — Kinbinding (alternate printing)
const KINBINDING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KINBINDING,
    2,
    "df94f428-f097-42cb-be03-3046e6867344",
    "Heather Hudson",
);

// ECL 408 — Harmonized Crescendo (alternate printing)
const HARMONIZED_CRESCENDO_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_CRESCENDO,
    4,
    "2dbd1ec1-2b66-4fcc-84cb-9d35afcec921",
    "Jeff Miracola",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CHANGELING_WAYFINDER,
    &ROOFTOP_PERCHER,
    &ADEPT_WATERSHAPER,
    &AJANI_OUTLAND_CHAPERONE,
    &APPEAL_TO_EIRDU,
    &BARK_OF_DORAN,
    &BRIGID_CLACHAN_S_HEART,
    &BURDENED_STONEBACK,
    &CHAMPION_OF_THE_CLACHAN,
    &CLACHAN_FESTIVAL,
    &CURIOUS_COLOSSUS,
    &EIRDU_CARRIER_OF_DAWN,
    &ENCUMBERED_REEJEREY,
    &EVERSHRIKE_S_GIFT,
    &FLOCK_IMPOSTOR,
    &GALLANT_FOWLKNIGHT,
    &GOLDMEADOW_NOMAD,
    &KEEP_OUT,
    &KINBINDING,
    &KINSBAILE_ASPIRANT,
    &KINSCAER_SENTRY,
    &KITHKEEPER,
    &LIMINAL_HOLD,
    &MEANDERS_GUIDE,
    &MOONLIT_LAMENTER,
    &MORNINGTIDE_S_LIGHT,
    &PERSONIFY,
    &PROTECTIVE_RESPONSE,
    &PYRRHIC_STRIKE,
    &RELUCTANT_DOUNGUARD,
    &RHYS_THE_EVERMORE,
    &RIVERGUARD_S_REFLEXES,
    &SHORE_LURKER,
    &SLUMBERING_WALKER,
    &SPIRAL_INTO_SOLITUDE,
    &SUN_DAPPLED_CELEBRANT,
    &THOUGHTWEFT_IMBUER,
    &TIMID_SHIELDBEARER,
    &TRIBUTARY_VAULTER,
    &WANDERBRINE_PREACHER,
    &WANDERBRINE_TRAPPER,
    &WINNOWING,
    &AQUITECT_S_DEFENSES,
    &BLOSSOMBIND,
    &CHAMPIONS_OF_THE_SHOAL,
    &DISRUPTOR_OF_CURRENTS,
    &FLITTERWING_NUISANCE,
    &GLAMER_GIFTER,
    &GLAMERMITE,
    &GLEN_ELENDRA_GUARDIAN,
    &GLEN_ELENDRA_S_ANSWER,
    &GRAVELGILL_SCOUNDREL,
    &HARMONIZED_CRESCENDO,
    &ILLUSION_SPINNERS,
    &KULRATH_MYSTIC,
    &LOCH_MARE,
    &LOFTY_DREAMS,
    &MIRRORFORM,
    &NOGGLE_THE_MIND,
    &OKO_LORWYN_LIEGE,
    &OMNI_CHANGELING,
    &PESTERED_WELLGUARD,
    &RIME_CHILL,
    &RIMEFIRE_TORQUE,
    &RIMEKIN_RECLUSE,
    &SHINESTRIKER,
    &SILVERGILL_MENTOR,
    &SILVERGILL_PEDDLER,
    &STRATOSOARER,
    &SUMMIT_SENTINEL,
    &SUNDERFLOCK,
    &SWAT_AWAY,
    &SYGG_WANDERWINE_WISDOM,
    &TANUFEL_RIMESPEAKER,
    &THIRST_FOR_IDENTITY,
    &UNEXPECTED_ASSISTANCE,
    &UNWELCOME_SPRITE,
    &WANDERWINE_DISTRACTER,
    &WANDERWINE_FAREWELL,
    &WILD_UNRAVELING,
    &AUNTIE_S_SENTENCE,
    &BARBED_BLOODLETTER,
    &BILE_VIAL_BOGGART,
    &BITTERBLOOM_BEARER,
    &BLIGHT_ROT,
    &BLIGHTED_BLACKTHORN,
    &BLOODLINE_BIDDING,
    &BOGGART_MISCHIEF,
    &BOGGART_PRANKSTER,
    &BOGSLITHER_S_EMBRACE,
    &CHAMPION_OF_THE_WEIRD,
    &CREAKWOOD_SAFEWRIGHT,
    &DARKNESS_DESCENDS,
    &DAWNHAND_DISSIDENT,
    &DAWNHAND_EULOGIST,
    &DOSE_OF_DAWNGLOW,
    &DREAM_SEIZER,
    &GLOOM_RIPPER,
    &GNARLBARK_ELM,
    &GRUB_STORIED_MATRIARCH,
    &GUTSPLITTER_GANG,
    &HEIRLOOM_AUNTIE,
    &IRON_SHIELD_ELF,
    &MOONGLOVE_EXTRACTOR,
    &MOONSHADOW,
    &MORNSONG_ARIA,
    &MUDBUTTON_CURSETOSSER,
    &NIGHTMARE_SOWER,
    &PERFECT_INTIMIDATION,
    &REQUITING_HEX,
    &RETCHED_WRETCH,
    &SCARBLADE_SCOUT,
    &SCARBLADE_S_MALICE,
    &SHIMMERCREEP,
    &TASTER_OF_WARES,
    &TWILIGHT_DIVINER,
    &UNBURY,
    &ASHLING_REKINDLED,
    &BOLDWYR_AGGRESSOR,
    &BONECLUB_BERSERKER,
    &BOULDER_DASH,
    &BRAMBLEBACK_BRUTE,
    &BURNING_CURIOSITY,
    &CHAMPION_OF_THE_PATH,
    &CINDER_STRIKE,
    &COLLECTIVE_INFERNO,
    &ELDER_AUNTIE,
    &END_BLAZE_EPIPHANY,
    &ENRAGED_FLAMECASTER,
    &EXPLOSIVE_PRODIGY,
    &FEED_THE_FLAMES,
    &FLAME_CHAIN_MAULER,
    &FLAMEBRAIDER,
    &FLAMEKIN_GILDWEAVER,
    &GIANTFALL,
    &GOLIATH_DAYDREAMER,
    &GRISTLE_GLUTTON,
    &HEXING_SQUELCHER,
    &IMPOLITE_ENTRANCE,
    &KINDLE_THE_INNER_FLAME,
    &KULRATH_ZEALOT,
    &LASTING_TARFIRE,
    &LAVALEAPER,
    &MEEK_ATTACK,
    &RECKLESS_RANSACKING,
    &SCUZZBACK_SCROUNGER,
    &SEAR,
    &SIZZLING_CHANGELING,
    &SOUL_IMMOLATION,
    &SOULBRIGHT_SEEKER,
    &SOURBREAD_AUNTIE,
    &SPINEROCK_TYRANT,
    &SQUAWKROASTER,
    &STING_SLINGER,
    &TWEEZE,
    &WARREN_TORCHMASTER,
    &ASSERT_PERFECTION,
    &AURORA_AWAKENER,
    &BRISTLEBANE_BATTLER,
    &BRISTLEBANE_OUTRIDER,
    &CELESTIAL_REUNION,
    &CHAMPIONS_OF_THE_PERFECT,
    &CHOMPING_CHANGELING,
    &CROSSROADS_WATCHER,
    &DAWN_S_LIGHT_ARCHER,
    &DUNDOOLIN_WEAVER,
    &FORMIDABLE_SPEAKER,
    &GILT_LEAF_S_EMBRACE,
    &GREAT_FOREST_DRUID,
    &LUMINOLLUSK,
    &LYS_ALANA_DIGNITARY,
    &LYS_ALANA_INFORMANT,
    &MIDNIGHT_TILLING,
    &MISTMEADOW_COUNCIL,
    &MOON_VIGIL_ADHERENTS,
    &MORCANT_S_EYES,
    &MUTABLE_EXPLORER,
    &PITILESS_FISTS,
    &PRISMABASHER,
    &PRISMATIC_UNDERCURRENTS,
    &PUMMELER_FOR_HIRE,
    &SAFEWRIGHT_CAVALRY,
    &SAPLING_NURSERY,
    &SELFLESS_SAFEWRIGHT,
    &SHIMMERWILDS_GROWTH,
    &SPRY_AND_MIGHTY,
    &SURLY_FARRIER,
    &TEND_THE_SPRIGS,
    &THOUGHTWEFT_CHARGE,
    &TRYSTAN_CALLOUS_CULTIVATOR,
    &UNFORGIVING_AIM,
    &VINEBRED_BRAWLER,
    &VIRULENT_EMISSARY,
    &WILDVINE_PUMMELER,
    &ABIGALE_ELOQUENT_FIRST_YEAR,
    &ASHLING_S_COMMAND,
    &BOGGART_CURSECRAFTER,
    &BRE_OF_CLAN_STOUTARM,
    &BRIGID_S_COMMAND,
    &CATHARSIS,
    &CHAOS_SPEWER,
    &CHITINOUS_GRASPLING,
    &DECEIT,
    &DEEPCHANNEL_DUELIST,
    &DEEPWAY_NAVIGATOR,
    &DORAN_BESIEGED_BY_TIME,
    &DREAM_HARVEST,
    &ECLIPSED_BOGGART,
    &ECLIPSED_ELF,
    &ECLIPSED_FLAMEKIN,
    &ECLIPSED_KITHKIN,
    &ECLIPSED_MERROW,
    &EMPTINESS,
    &FEISTY_SPIKELING,
    &FIGURE_OF_FABLE,
    &FLARING_CINDER,
    &GANGLY_STOMPLING,
    &GLISTER_BAIRN,
    &GRUB_S_COMMAND,
    &HIGH_PERFECT_MORCANT,
    &HOVEL_HURLER,
    &KIROL_ATTENTIVE_FIRST_YEAR,
    &LLUWEN_IMPERFECT_NATURALIST,
    &MARALEN_FAE_ASCENDANT,
    &MERROW_SKYSWIMMER,
    &MISCHIEVOUS_SNEAKLING,
    &MORCANT_S_LOYALIST,
    &NOGGLE_ROBBER,
    &PRIDEFUL_FEASTLING,
    &RAIDING_SCHEMES,
    &REAPING_WILLOW,
    &SANAR_INNOVATIVE_FIRST_YEAR,
    &SHADOW_URCHIN,
    &STOIC_GROVE_GUIDE,
    &SYGG_S_COMMAND,
    &TAM_MINDFUL_FIRST_YEAR,
    &THOUGHTWEFT_LIEUTENANT,
    &TRYSTAN_S_COMMAND,
    &TWINFLAME_TRAVELERS,
    &VIBRANCE,
    &VORACIOUS_TOME_SKIMMER,
    &WARY_FARMER,
    &WISTFULNESS,
    &CHRONICLE_OF_VICTORY,
    &DAWN_BLESSED_PENNANT,
    &FIRDOCH_CORE,
    &FORAGING_WICKERMAW,
    &GATHERING_STONE,
    &MIRRORMIND_CROWN,
    &PUCA_S_EYE,
    &STALACTITE_DAGGER,
    &ECLIPSED_REALMS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    CRIB_SWAP_REPRINT,
    RUN_AWAY_TOGETHER_REPRINT,
    SPELL_SNARE_REPRINT,
    TEMPORAL_CLEANSING_REPRINT,
    GRAVESHIFTER_REPRINT,
    NAMELESS_INVERSION_REPRINT,
    GOATNAP_REPRINT,
    BLOOM_TENDER_REPRINT,
    BLOSSOMING_DEFENSE_REPRINT,
    SPRINGLEAF_DRUM_REPRINT,
    BLOOD_CRYPT_REPRINT,
    EVOLVING_WILDS_REPRINT,
    HALLOWED_FOUNTAIN_REPRINT,
    OVERGROWN_TOMB_REPRINT,
    STEAM_VENTS_REPRINT,
    TEMPLE_GARDEN_REPRINT,
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
    AJANI_OUTLAND_CHAPERONE_ALTERNATE_1,
    BRIGID_CLACHAN_S_HEART_ALTERNATE_1,
    EIRDU_CARRIER_OF_DAWN_ALTERNATE_1,
    OKO_LORWYN_LIEGE_ALTERNATE_1,
    SYGG_WANDERWINE_WISDOM_ALTERNATE_1,
    GRUB_STORIED_MATRIARCH_ALTERNATE_1,
    ASHLING_REKINDLED_ALTERNATE_1,
    TRYSTAN_CALLOUS_CULTIVATOR_ALTERNATE_1,
    CATHARSIS_ALTERNATE_1,
    DECEIT_ALTERNATE_1,
    EMPTINESS_ALTERNATE_1,
    VIBRANCE_ALTERNATE_1,
    WISTFULNESS_ALTERNATE_1,
    ADEPT_WATERSHAPER_ALTERNATE_1,
    CURIOUS_COLOSSUS_ALTERNATE_1,
    KINBINDING_ALTERNATE_1,
    KINSCAER_SENTRY_ALTERNATE_1,
    MORNINGTIDE_S_LIGHT_ALTERNATE_1,
    SLUMBERING_WALKER_ALTERNATE_1,
    DISRUPTOR_OF_CURRENTS_ALTERNATE_1,
    FLITTERWING_NUISANCE_ALTERNATE_1,
    GLEN_ELENDRA_GUARDIAN_ALTERNATE_1,
    GLEN_ELENDRA_S_ANSWER_ALTERNATE_1,
    LOCH_MARE_ALTERNATE_1,
    MIRRORFORM_ALTERNATE_1,
    SUNDERFLOCK_ALTERNATE_1,
    BITTERBLOOM_BEARER_ALTERNATE_1,
    DAWNHAND_DISSIDENT_ALTERNATE_1,
    GLOOM_RIPPER_ALTERNATE_1,
    MOONSHADOW_ALTERNATE_1,
    TASTER_OF_WARES_ALTERNATE_1,
    TWILIGHT_DIVINER_ALTERNATE_1,
    GOLIATH_DAYDREAMER_ALTERNATE_1,
    HEXING_SQUELCHER_ALTERNATE_1,
    LAVALEAPER_ALTERNATE_1,
    MEEK_ATTACK_ALTERNATE_1,
    SCUZZBACK_SCROUNGER_ALTERNATE_1,
    SOUL_IMMOLATION_ALTERNATE_1,
    SPINEROCK_TYRANT_ALTERNATE_1,
    AURORA_AWAKENER_ALTERNATE_1,
    BLOOM_TENDER_ALTERNATE_1,
    BRISTLEBANE_BATTLER_ALTERNATE_1,
    CELESTIAL_REUNION_ALTERNATE_1,
    MUTABLE_EXPLORER_ALTERNATE_1,
    SAPLING_NURSERY_ALTERNATE_1,
    SPRY_AND_MIGHTY_ALTERNATE_1,
    ASHLING_S_COMMAND_ALTERNATE_1,
    BOGGART_CURSECRAFTER_ALTERNATE_1,
    BRIGID_S_COMMAND_ALTERNATE_1,
    DEEPCHANNEL_DUELIST_ALTERNATE_1,
    DORAN_BESIEGED_BY_TIME_ALTERNATE_1,
    ECLIPSED_BOGGART_ALTERNATE_1,
    ECLIPSED_ELF_ALTERNATE_1,
    ECLIPSED_FLAMEKIN_ALTERNATE_1,
    ECLIPSED_KITHKIN_ALTERNATE_1,
    ECLIPSED_MERROW_ALTERNATE_1,
    GRUB_S_COMMAND_ALTERNATE_1,
    MORCANT_S_LOYALIST_ALTERNATE_1,
    SYGG_S_COMMAND_ALTERNATE_1,
    THOUGHTWEFT_LIEUTENANT_ALTERNATE_1,
    TRYSTAN_S_COMMAND_ALTERNATE_1,
    TWINFLAME_TRAVELERS_ALTERNATE_1,
    CHRONICLE_OF_VICTORY_ALTERNATE_1,
    HALLOWED_FOUNTAIN_ALTERNATE_1,
    STEAM_VENTS_ALTERNATE_1,
    BLOOD_CRYPT_ALTERNATE_1,
    OVERGROWN_TOMB_ALTERNATE_1,
    TEMPLE_GARDEN_ALTERNATE_1,
    BITTERBLOOM_BEARER_ALTERNATE_2,
    CHAMPION_OF_THE_CLACHAN_ALTERNATE_1,
    RHYS_THE_EVERMORE_ALTERNATE_1,
    WINNOWING_ALTERNATE_1,
    CHAMPIONS_OF_THE_SHOAL_ALTERNATE_1,
    HARMONIZED_CRESCENDO_ALTERNATE_1,
    RIMEFIRE_TORQUE_ALTERNATE_1,
    BLOODLINE_BIDDING_ALTERNATE_1,
    CHAMPION_OF_THE_WEIRD_ALTERNATE_1,
    MORNSONG_ARIA_ALTERNATE_1,
    CHAMPION_OF_THE_PATH_ALTERNATE_1,
    COLLECTIVE_INFERNO_ALTERNATE_1,
    END_BLAZE_EPIPHANY_ALTERNATE_1,
    CHAMPIONS_OF_THE_PERFECT_ALTERNATE_1,
    FORMIDABLE_SPEAKER_ALTERNATE_1,
    SELFLESS_SAFEWRIGHT_ALTERNATE_1,
    ABIGALE_ELOQUENT_FIRST_YEAR_ALTERNATE_1,
    BRE_OF_CLAN_STOUTARM_ALTERNATE_1,
    DEEPWAY_NAVIGATOR_ALTERNATE_1,
    DREAM_HARVEST_ALTERNATE_1,
    FIGURE_OF_FABLE_ALTERNATE_1,
    HIGH_PERFECT_MORCANT_ALTERNATE_1,
    KIROL_ATTENTIVE_FIRST_YEAR_ALTERNATE_1,
    LLUWEN_IMPERFECT_NATURALIST_ALTERNATE_1,
    MARALEN_FAE_ASCENDANT_ALTERNATE_1,
    RAIDING_SCHEMES_ALTERNATE_1,
    SANAR_INNOVATIVE_FIRST_YEAR_ALTERNATE_1,
    SHADOW_URCHIN_ALTERNATE_1,
    TAM_MINDFUL_FIRST_YEAR_ALTERNATE_1,
    MIRRORMIND_CROWN_ALTERNATE_1,
    WINNOWING_ALTERNATE_2,
    GLEN_ELENDRA_GUARDIAN_ALTERNATE_2,
    HARMONIZED_CRESCENDO_ALTERNATE_2,
    BLOODLINE_BIDDING_ALTERNATE_2,
    MOONSHADOW_ALTERNATE_2,
    COLLECTIVE_INFERNO_ALTERNATE_2,
    MEEK_ATTACK_ALTERNATE_2,
    SPINEROCK_TYRANT_ALTERNATE_2,
    BLOOM_TENDER_ALTERNATE_2,
    SELFLESS_SAFEWRIGHT_ALTERNATE_2,
    WINNOWING_ALTERNATE_3,
    GLEN_ELENDRA_GUARDIAN_ALTERNATE_3,
    HARMONIZED_CRESCENDO_ALTERNATE_3,
    BLOODLINE_BIDDING_ALTERNATE_3,
    MOONSHADOW_ALTERNATE_3,
    COLLECTIVE_INFERNO_ALTERNATE_3,
    MEEK_ATTACK_ALTERNATE_3,
    SPINEROCK_TYRANT_ALTERNATE_3,
    BLOOM_TENDER_ALTERNATE_3,
    SELFLESS_SAFEWRIGHT_ALTERNATE_3,
    PERSONIFY_ALTERNATE_1,
    SILVERGILL_MENTOR_ALTERNATE_1,
    IRON_SHIELD_ELF_ALTERNATE_1,
    SEAR_ALTERNATE_1,
    VIRULENT_EMISSARY_ALTERNATE_1,
    KINBINDING_ALTERNATE_2,
    HARMONIZED_CRESCENDO_ALTERNATE_4,
];
