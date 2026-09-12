//! Marvel's Spider-Man card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostIndex;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
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
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ChooseObjectOrderDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectPaymentDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
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
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SpellCastQueryDef;
use crate::card::SubtypeDef;
use crate::card::TokenCopyDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::stronghold as catalog_sth;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SPM",
    slug: "marvels-spider-man",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

pub const WEB_SLINGING: crate::card::MechanicId =
    crate::card::MechanicId::from_name("mtg:web-slinging");
pub const fn web_slinging(text: &'static str, costs: &'static [CostDef]) -> AbilityDef {
    AbilityDef::alternative_cast(
        costs,
        AlternativeCastKindDef::AlternativeCost,
        Some(text),
        EffectDef::None,
    )
    .with_alternative_cost_binding(crate::Binding!("web-slinging"))
    .labeled(WEB_SLINGING)
}

// SPM 1 — Anti-Venom, Horrifying Healer
// Audit: unsupported — Needs a static damage-prevention replacement with an immediate counter-placement consequence using the amount actually prevented; prevention follow-ups currently support life gain, not counters.
pub(in crate::card::sets) static ANTI_VENOM_HORRIFYING_HEALER: CardRecord = CardRecord::new(
    "Anti-Venom, Horrifying Healer",
    "560384fe-7be0-4b93-a515-2fe687ab2492",
    "Néstor Ossandón Leal",
    CardRules::unsupported(),
);

// SPM 2 — Arachne, Psionic Weaver
// Audit: unsupported — Needs an as-entry opponent-hand inspection followed by a durable noncreature card-type choice, and a cost modifier keyed to that chosen type; existing scalar entry choices do not include card types.
pub(in crate::card::sets) static ARACHNE_PSIONIC_WEAVER: CardRecord = CardRecord::new(
    "Arachne, Psionic Weaver",
    "7c1f871a-bd85-402e-b474-1deb64c18a52",
    "Steve Argyle",
    CardRules::unsupported(),
);

// SPM 3 — Aunt May
pub(in crate::card::sets) static AUNT_MAY: CardRecord = CardRecord::new(
    "Aunt May",
    "ad96343b-baac-428c-8270-fcffbbbe9fb8",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Citizen"], 0, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 1 \
             life. If it's a Spider, put a +1/+1 counter on it.",
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
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::One(ObjectRefDef::TriggeringObject),
                        predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::Subtype(
                            SubtypeDef::Literal("Spider"),
                        )),
                    }),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::TriggeringObject,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        )]),
);

// SPM 4 — City Pigeon
pub(in crate::card::sets) static CITY_PIGEON: CardRecord = CardRecord::new(
    "City Pigeon",
    "56d67fb4-5b23-432c-9ffb-39545035c117",
    "David Szabo",
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, create a Food \
             token. (It's an artifact with \"{2}, {T}, Sacrifice this \
             token: You gain 3 life.\")",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::create_token(tokens::food()).with_count(ValueDef::Constant(1)),
        ),
    ]),
);

// SPM 5 — Costume Closet
// Audit: unsupported — Needs a complete modified predicate, including Equipment attached to the creature and Auras controlled by its controller; the available counter and enchanted predicates cannot identify all three modifications with their required controller scope.
pub(in crate::card::sets) static COSTUME_CLOSET: CardRecord = CardRecord::new(
    "Costume Closet",
    "cc641f4a-ddbe-4f7d-bb55-eabf11f8b7fb",
    "Bastien Grivet",
    CardRules::unsupported(),
);

// SPM 6 — Daily Bugle Reporters
pub(in crate::card::sets) static DAILY_BUGLE_REPORTERS: CardRecord = CardRecord::new(
    "Daily Bugle Reporters",
    "530dbeb0-b0cd-473e-a43e-7b23c88650a3",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Citizen"], 2, 3).with_abilities(&[
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Puff Piece — Put a +1/+1 counter on each of up to two target \
                     creatures.",
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
                AbilityDef::spell_with_targets(
                    "Investigative Journalism — Return target creature card with \
                     mana value 2 or less from your graveyard to your hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::ManaValueAtMost(2),
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
            ],
        ),
    ]),
);

// SPM 7 — Flash Thompson, Spider-Fan
// Audit: unsupported — Needs selecting and executing both modes of one triggered ability, including both target slots; current triggered-mode placement offers a maximum of one mode even though modal spells support larger selections.
pub(in crate::card::sets) static FLASH_THOMPSON_SPIDER_FAN: CardRecord = CardRecord::new(
    "Flash Thompson, Spider-Fan",
    "44cf372b-f668-45e9-981e-4533295dcc74",
    "Gal Or",
    CardRules::unsupported(),
);

// SPM 8 — Friendly Neighborhood
pub(in crate::card::sets) static FRIENDLY_NEIGHBORHOOD: CardRecord = CardRecord::new(
    "Friendly Neighborhood",
    "17a18e2f-221f-4fc2-8dab-25bf12fb8756",
    "Pablo Mendoza",
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            abilities::enters_trigger(
                "When this Aura enters, create three 1/1 green and white Human \
                 Citizen creature tokens.",
                EffectDef::create_creature_token(
                    &["Human", "Citizen"],
                    &[ManaColor::Green, ManaColor::White],
                    1,
                    1,
                )
                .with_count(ValueDef::Constant(3)),
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{1}, {T}: Target creature gets +1/+1 \
                 until end of turn for each creature you control. Activate \
                 only as a sorcery.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(
                        &AbilityDef::activated_with_targets(
                            "{1}, {T}: Target creature gets +1/+1 until end of turn for \
                             each creature you control. Activate only as a sorcery.",
                            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
                            &[AbilityTargetDef::exactly_one_permanent(
                                ObjectPredicateDef::HasType(CardType::Creature),
                            )],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
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
                        )
                        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                    ),
                },
            ),
        ]),
);

// SPM 9 — Origin of Spider-Man
pub(in crate::card::sets) static ORIGIN_OF_SPIDER_MAN: CardRecord = CardRecord::new(
    "Origin of Spider-Man",
    "a10a7da7-d9cb-495a-9c9f-205d355c390d",
    "Bill Sienkiewicz",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — Create a 2/1 green Spider creature token with reach.",
                EffectDef::create_creature_token(&["Spider"], &[ManaColor::Green], 2, 1)
                    .with_abilities(&[abilities::reach()]),
            ),
            abilities::saga_chapter_with_targets(
                2,
                "II — Put a +1/+1 counter on target creature you control. It \
                 becomes a legendary Spider Hero in addition to its other \
                 types.",
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
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                                "Spider", "Hero",
                            ])),
                            AppliedEffectDef::add_supertype(CardSupertype::Legendary),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                ]),
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Target creature you control gains double strike until \
                 end of turn.",
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
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// SPM 10 — Peter Parker // Amazing Spider-Man
// Audit: unsupported — Needs a battlefield static rule granting a complete web-slinging alternative cost to matching colored legendary spells in all castable zones; current alternative-cost grants are restricted to particular graveyard casts.
pub(in crate::card::sets) static PETER_PARKER: CardRecord = CardRecord::new(
    "Peter Parker // Amazing Spider-Man",
    "3ce33422-5dba-4a42-8375-dd8ccc692a7b",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// SPM 11 — Rent Is Due
pub(in crate::card::sets) static RENT_IS_DUE: CardRecord = CardRecord::new(
    "Rent Is Due",
    "b3f8d221-081f-49f5-a501-07e5eb21a840",
    "Gal Or",
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[AbilityDef::triggered(
        "At the beginning of your end step, you may tap two untapped \
         creatures and/or Treasures you control. If you do, draw a \
         card. Otherwise, sacrifice this enchantment.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Treasure")),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(2),
            }),
            then: &EffectDef::ChooseEffect {
                player: EffectRecipientDef::Controller,
                choices: &[
                    EffectChoiceDef {
                        label: "Tap two permanents",
                        effect: EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::AnyOf(&[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                                            "Treasure",
                                        )),
                                    ]),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            exclude: None,
                            minimum: 2,
                            maximum: 2,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::Sequence(&[
                                EffectDef::Tap {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("chosen"),
                                    )),
                                },
                                abilities::draw_cards(ValueDef::Constant(1)),
                            ]),
                        }),
                    },
                    EffectChoiceDef {
                        label: "Sacrifice this enchantment",
                        effect: EffectDef::sacrifice(EffectRecipientDef::Source),
                    },
                ],
            },
            otherwise: &EffectDef::sacrifice(EffectRecipientDef::Source),
        },
    )]),
);

// SPM 12 — Selfless Police Captain
pub(in crate::card::sets) static SELFLESS_POLICE_CAPTAIN: CardRecord = CardRecord::new(
    "Selfless Police Captain",
    "fb0fd7bd-10d0-4d29-af88-387d1e07f3b7",
    "Aniekan Udofia",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Detective"], 1, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::triggered_with_targets(
            "When this creature leaves the battlefield, put its +1/+1 \
             counters on target creature you control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
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
                amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
            },
        ),
    ]),
);

// SPM 13 — Silver Sable, Mercenary Leader
// Audit: unsupported — Needs a complete modified predicate, including Equipment attached to the creature and Auras controlled by its controller; the available counter and enchanted predicates cannot identify all three modifications with their required controller scope.
pub(in crate::card::sets) static SILVER_SABLE_MERCENARY_LEADER: CardRecord = CardRecord::new(
    "Silver Sable, Mercenary Leader",
    "cf0d4116-acee-4d9a-985c-396d10e03838",
    "JB Casacop",
    CardRules::unsupported(),
);

// SPM 14 — Spectacular Spider-Man
pub(in crate::card::sets) static SPECTACULAR_SPIDER_MAN: CardRecord = CardRecord::new(
    "Spectacular Spider-Man",
    "32bff506-efc0-42ef-8286-ed939bf853d7",
    "Roberta Ingranata",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spider", "Human", "Hero"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::activated(
                "{1}: Spectacular Spider-Man gains flying until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{1}, Sacrifice Spectacular Spider-Man: Creatures you control \
                 gain hexproof and indestructible until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
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
        ]),
);

// SPM 15 — Spectacular Tactics
pub(in crate::card::sets) static SPECTACULAR_TACTICS: CardRecord = CardRecord::new(
    "Spectacular Tactics",
    "836b4246-f1f2-4495-8664-650dda70ed4f",
    "Zoltan Boros",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Put a +1/+1 counter on target creature you control. It gains \
                 hexproof until end of turn.",
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
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target creature with power 4 or greater.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
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

// SPM 16 — Spider-Man, Web-Slinger
pub(in crate::card::sets) static SPIDER_MAN_WEB_SLINGER: CardRecord = CardRecord::new(
    "Spider-Man, Web-Slinger",
    "897418bc-df8c-4c97-b6bf-7c9133a8a577",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spider", "Human", "Hero"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[web_slinging(
            "Web-slinging {W} (You may cast this spell for {W} if you also \
             return a tapped creature you control to its owner's hand.)",
            &[
                CostDef::Mana(mana_cost!("{W}")),
                CostDef::return_to_hand(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    CostQuantityDef::Fixed(1),
                ),
            ],
        )]),
);

// SPM 17 — Spider-UK
// Audit: unsupported — Needs a per-controller count of creatures that entered this turn, retaining entries after those creatures leave; counting live EnteredThisTurn permanents loses the required history.
pub(in crate::card::sets) static SPIDER_UK: CardRecord = CardRecord::new(
    "Spider-UK",
    "6beb4548-1fab-4b9e-bf24-f7b9aadecc87",
    "Allen Morris",
    CardRules::unsupported(),
);

// SPM 18 — Starling, Aerial Ally
pub(in crate::card::sets) static STARLING_AERIAL_ALLY: CardRecord = CardRecord::new(
    "Starling, Aerial Ally",
    "babbf53d-3e10-4110-8725-91f766c8cdad",
    "Aniekan Udofia",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Hero"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When Starling enters, another target creature you control \
                 gains flying until end of turn.",
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
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// SPM 19 — Sudden Strike
pub(in crate::card::sets) static SUDDEN_STRIKE: CardRecord = CardRecord::new(
    "Sudden Strike",
    "6eea2718-93d2-4d83-9b5d-eb943a0f1d11",
    "Le Vuong",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
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
    )]),
);

// SPM 20 — Thwip!
pub(in crate::card::sets) static THWIP: CardRecord = CardRecord::new(
    "Thwip!",
    "b2dac88e-1204-4640-94ce-e1aff434ea06",
    "Lordigan",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains flying until end of \
         turn. If it's a Spider, you gain 2 life.",
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
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                },
                then: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            },
        ]),
    )]),
);

// SPM 21 — Web Up
// Audit: unsupported — Needs immediate return when the exile-until-source-leaves duration ends; the current linked-exile implementation returns through a separate leaves trigger that can be responded to or countered.
pub(in crate::card::sets) static WEB_UP: CardRecord = CardRecord::new(
    "Web Up",
    "1ab7c1e6-54af-4002-8a81-23a1ccafa3ff",
    "David Palumbo",
    CardRules::unsupported(),
);

// SPM 22 — Web-Shooters
pub(in crate::card::sets) static WEB_SHOOTERS: CardRecord = CardRecord::new(
    "Web-Shooters",
    "a0ca1108-6d99-4dc2-96ab-7728d65b0c06",
    "Javier Charro",
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has reach and \"Whenever \
                 this creature attacks, tap target creature an opponent \
                 controls.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                            "Whenever this creature attacks, tap target creature an \
                             opponent controls.",
                            TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
                        )),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip \
                 only as a sorcery.)",
            ),
        ]),
);

// SPM 23 — Wild Pack Squad
pub(in crate::card::sets) static WILD_PACK_SQUAD: CardRecord = CardRecord::new(
    "Wild Pack Squad",
    "7b0eda7c-e44d-4d9b-9042-4a1eb8c4ed4a",
    "John Tyler Christopher",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Mercenary"], 2, 3).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, up to one target \
             creature gains first strike and vigilance until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
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
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SPM 24 — With Great Power . . .
// Audit: unsupported — Needs static redirection of all damage from this Aura's controller to its enchanted creature; RedirectPlayerDamageToThis only supports predefined source groups, and the general RedirectDamageFromTo rule is a resolving effect rather than a static rule.
pub(in crate::card::sets) static WITH_GREAT_POWER: CardRecord = CardRecord::new(
    "With Great Power . . .",
    "f717c096-e161-426e-a8d7-c93b117e16b9",
    "E. M. Gist",
    CardRules::unsupported(),
);

// SPM 25 — Amazing Acrobatics
pub(in crate::card::sets) static AMAZING_ACROBATICS: CardRecord = CardRecord::new(
    "Amazing Acrobatics",
    "9a2f6d84-3d83-4f48-9906-11f681171930",
    "Justyna Dura",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
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
            AbilityDef::spell_with_targets(
                "Tap one or two target creatures.",
                &[AbilityTargetDef {
                    minimum: 1,
                    maximum: 2,
                    ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    ))
                }],
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ],
    )
    .with_mode_selection(1, 2, false)]),
);

// SPM 26 — Beetle, Legacy Criminal
pub(in crate::card::sets) static BEETLE_LEGACY_CRIMINAL: CardRecord = CardRecord::new(
    "Beetle, Legacy Criminal",
    "a194f930-c99f-4915-8a62-e20ab2b4ad1f",
    "Carlos Dattoli",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Rogue", "Villain"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{1}{U}, Exile this card from your graveyard: Put a +1/+1 \
                 counter on target creature. It gains flying until end of \
                 turn. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::ExileSource],
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
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// SPM 27 — Chameleon, Master of Disguise
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static CHAMELEON_MASTER_OF_DISGUISE: CardRecord = CardRecord::new(
    "Chameleon, Master of Disguise",
    "43892ce7-f63a-4294-922b-8f879f684033",
    "Javier Charro",
    CardRules::unsupported(),
);

// SPM 28 — The Clone Saga
// Audit: unsupported — Needs a delayed trigger combining next-occurrence consumption with end-of-turn expiry, plus a nonlegendary exception on a creature spell copy; installed Once and ThisTurn lifetimes cannot be combined, and stack-copy exceptions only include color.
pub(in crate::card::sets) static THE_CLONE_SAGA: CardRecord = CardRecord::new(
    "The Clone Saga",
    "976432b3-bc17-4edb-86d6-00fd1baf9670",
    "Bill Sienkiewicz",
    CardRules::unsupported(),
);

// SPM 29 — Doc Ock, Sinister Scientist
pub(in crate::card::sets) static DOC_OCK_SINISTER_SCIENTIST: CardRecord = CardRecord::new(
    "Doc Ock, Sinister Scientist",
    "de16ad65-c8c3-48c0-9d13-5af91b4e6f01",
    "Piotr Dura",
    CardRules::new_creature(
        mana_cost!("{4}{U}"),
        &["Human", "Scientist", "Villain"],
        4,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::static_ability(
            "As long as there are eight or more cards in your graveyard, \
             Doc Ock has base power and toughness 8/8.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(8),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(8),
                        ValueDef::Constant(8),
                    ),
                },
            },
        ),
        AbilityDef::static_ability(
            "As long as you control another Villain, Doc Ock has hexproof. \
             (He can't be the target of spells or abilities your opponents \
             control.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                },
            },
        ),
    ]),
);

// SPM 30 — Doc Ock's Henchmen
pub(in crate::card::sets) static DOC_OCK_S_HENCHMEN: CardRecord = CardRecord::new(
    "Doc Ock's Henchmen",
    "a383c442-3f4a-4115-97e4-23f0eb88465b",
    "Nathaniel Himawan",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Villain"], 2, 1).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered(
            "Whenever this creature attacks, it connives. (Draw a card, \
             then discard a card. If you discarded a nonland card, put a \
             +1/+1 counter on this creature.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("connived")),
                        effect: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                crate::Binding!("connived"),
                            )),
                        },
                    }),
                },
            ]),
        ),
    ]),
);

// SPM 31 — Flying Octobot
pub(in crate::card::sets) static FLYING_OCTOBOT: CardRecord = CardRecord::new(
    "Flying Octobot",
    "ebadcd4a-f58f-4328-a765-0ea8d8028417",
    "John Tyler Christopher",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Robot", "Villain"], 1, 1)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever another Villain you control enters, put a +1/+1 \
                 counter on this creature. This ability triggers only once \
                 each turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
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
            )
            .triggering_at_most(1),
        ]),
);

// SPM 32 — Hide on the Ceiling
pub(in crate::card::sets) static HIDE_ON_THE_CEILING: CardRecord = CardRecord::new(
    "Hide on the Ceiling",
    "7977e448-01fa-4fa5-a275-0d6a1357b35c",
    "Fariba Khamseh",
    CardRules::new_instant(mana_cost!("{X}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile X target artifacts and/or creatures. Return the exiled \
         cards to the battlefield under their owners' control at the \
         beginning of the next end step.",
        &[AbilityTargetDef {
            minimum: AbilityTargetDef::CHOSEN_X,
            maximum: AbilityTargetDef::CHOSEN_X,
            ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]))
        }],
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
                        "At the beginning of the next end step, return the exiled \
                         cards to the battlefield under their owners' control.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "returning"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                    ),
                )),
            }),
        },
    )]),
);

// SPM 33 — Hydro-Man, Fluid Felon
pub(in crate::card::sets) static HYDRO_MAN_FLUID_FELON: CardRecord = CardRecord::new(
    "Hydro-Man, Fluid Felon",
    "e53115a4-8959-40fa-b763-931504a1c5a2",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Elemental", "Villain"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_if(
                "Whenever you cast a blue spell, if Hydro-Man is a creature, \
                 he gets +1/+1 until end of turn.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, untap Hydro-Man. Until \
                 your next turn, he becomes a land and gains \"{T}: Add {U}.\" \
                 (He's not a creature during that time.)",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Land)),
                            AppliedEffectDef::add_ability(&abilities::tap_for(ManaColor::Blue)),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                    },
                ]),
            ),
        ]),
);

// SPM 34 — Impostor Syndrome
pub(in crate::card::sets) static IMPOSTOR_SYNDROME: CardRecord = CardRecord::new(
    "Impostor Syndrome",
    "08da9f92-0e25-4f39-aaa4-d8974af81a41",
    "Javier Charro",
    CardRules::new_enchantment(mana_cost!("{4}{U}{U}")).with_abilities(&[AbilityDef::triggered(
        "Whenever a nontoken creature you control deals combat damage \
         to a player, create a token that's a copy of it, except it \
         isn't legendary.",
        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
            ]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        EffectDef::create_token_from_copy(&TokenCopyDef {
            object: &EffectRecipientDef::TriggeringObject,
            exceptions: CopyExceptionsDef {
                removed_supertypes: &[CardSupertype::Legendary],
                ..CopyExceptionsDef::NONE
            },
        }),
    )]),
);

// SPM 35 — Lady Octopus, Inspired Inventor
// Audit: unsupported — Needs an immediate free-cast offer for a selected artifact card in hand; MayPlayWithoutPaying currently offers only exiled cards, and first exiling the hand card would add an unprinted zone change.
pub(in crate::card::sets) static LADY_OCTOPUS_INSPIRED_INVENTOR: CardRecord = CardRecord::new(
    "Lady Octopus, Inspired Inventor",
    "8c5f360b-f9a0-46e0-9e8b-58e5b4b0389e",
    "Fariba Khamseh",
    CardRules::unsupported(),
);

// SPM 36 — Madame Web, Clairvoyant
pub(in crate::card::sets) static MADAME_WEB_CLAIRVOYANT: CardRecord = CardRecord::new(
    "Madame Web, Clairvoyant",
    "a16ca3fc-3cdf-4333-93e0-524afafe367b",
    "Pavel Kolomeyets",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Mutant", "Advisor"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may look at the top card of your library any time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
                },
            ),
            AbilityDef::static_ability(
                "You may cast Spider spells and noncreature spells from the \
                 top of your library.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                        restriction: PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                            ]),
                        ),
                        cost: TopOfLibraryCostDef::Printed,
                    }),
                },
            ),
            AbilityDef::triggered(
                "Whenever you attack, you may mill a card. (You may put the \
                 top card of your library into your graveyard.)",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
        ]),
);

// SPM 37 — Mysterio, Master of Illusion
pub(in crate::card::sets) static MYSTERIO_MASTER_OF_ILLUSION: CardRecord = CardRecord::new(
    "Mysterio, Master of Illusion",
    "facbd96f-c088-4377-b740-5e0fe99102bb",
    "Alexander Gering",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Villain"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::enters_trigger(
            "When Mysterio enters, create a 3/3 blue Illusion Villain \
             creature token for each nontoken Villain you control. Exile \
             those tokens when Mysterio leaves the battlefield.",
            EffectDef::create_creature_token(&["Illusion", "Villain"], &[ManaColor::Blue], 3, 3)
                .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )))
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("illusions"),
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "When Mysterio leaves the battlefield, exile those tokens.",
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                Some(ZoneKind::Battlefield),
                                None,
                            ),
                            EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("illusions"),
                                )),
                                ZoneKind::Exile,
                                ZonePlacement::Top,
                            ),
                        ),
                    )),
                }),
        )]),
);

// SPM 38 — Mysterio's Phantasm
pub(in crate::card::sets) static MYSTERIO_S_PHANTASM: CardRecord = CardRecord::new(
    "Mysterio's Phantasm",
    "79aa0a78-80a2-44be-8a79-92bcba9c040f",
    "Piotr Dura",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Illusion", "Villain"], 1, 3).with_abilities(
        &[
            abilities::flying(),
            abilities::vigilance(),
            AbilityDef::triggered(
                "Whenever this creature attacks, mill a card. (Put the top \
                 card of your library into your graveyard.)",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// SPM 39 — Norman Osborn // Green Goblin
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static NORMAN_OSBORN: CardRecord = CardRecord::new(
    "Norman Osborn // Green Goblin",
    "d5c53af9-7150-4e78-8771-2de7980aa307",
    "Scott M. Fischer",
    CardRules::unsupported(),
);

// SPM 40 — Oscorp Research Team
pub(in crate::card::sets) static OSCORP_RESEARCH_TEAM: CardRecord = CardRecord::new(
    "Oscorp Research Team",
    "a800ffb4-0c48-41eb-b221-cf1d855131d9",
    "Gal Or",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Scientist"], 1, 5).with_abilities(&[
        AbilityDef::activated(
            "{6}{U}: Draw two cards.",
            &[CostDef::Mana(mana_cost!("{6}{U}"))],
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
    ]),
);

// SPM 41 — Robotics Mastery
pub(in crate::card::sets) static ROBOTICS_MASTERY: CardRecord = CardRecord::new(
    "Robotics Mastery",
    "d0e92939-6d86-44b4-8a43-1963e97a2bb3",
    "Domenico Cava",
    CardRules::new_enchantment(mana_cost!("{4}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, create two 1/1 colorless Robot \
                 artifact creature tokens with flying.",
                EffectDef::create_artifact_creature_token(&["Robot"], &[], 1, 1)
                    .with_count(ValueDef::Constant(2))
                    .with_abilities(&[abilities::flying()]),
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

// SPM 42 — School Daze
pub(in crate::card::sets) static SCHOOL_DAZE: CardRecord = CardRecord::new(
    "School Daze",
    "e5b61b9d-31a2-4e09-9612-4980bf8de708",
    "Domenico Cava",
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Do Homework — Draw three cards.",
                abilities::draw_cards(ValueDef::Constant(3)),
            ),
            AbilityDef::spell_with_targets(
                "Fight Crime — Counter target spell. Draw a card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Spell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::counter_target(TargetIndex::PRIMARY),
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            ),
        ],
    )]),
);

// SPM 43 — Secret Identity
pub(in crate::card::sets) static SECRET_IDENTITY: CardRecord = CardRecord::new(
    "Secret Identity",
    "37a31d84-e87b-406e-9249-fae1b5e23e72",
    "rk post",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Conceal — Until end of turn, target creature you control \
                 becomes a Citizen with base power and toughness 1/1 and gains \
                 hexproof.",
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
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Citizen",
                        ])),
                        AppliedEffectDef::add_ability(&abilities::hexproof()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Reveal — Until end of turn, target creature you control \
                 becomes a Hero with base power and toughness 3/4 and gains \
                 flying and vigilance.",
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
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(4),
                        ),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Hero"])),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// SPM 44 — Spider-Byte, Web Warden
pub(in crate::card::sets) static SPIDER_BYTE_WEB_WARDEN: CardRecord = CardRecord::new(
    "Spider-Byte, Web Warden",
    "210ae606-12a4-453b-bfb4-73ca9c22b8b5",
    "Thanh Tuấn",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spider", "Avatar", "Hero"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "When Spider-Byte enters, return up to one target nonland \
             permanent to its owner's hand.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
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
        )]),
);

// SPM 45 — Spider-Man No More
pub(in crate::card::sets) static SPIDER_MAN_NO_MORE: CardRecord = CardRecord::new(
    "Spider-Man No More",
    "72dbab11-96ed-43db-8b59-ceca47c8cd22",
    "Aniekan Udofia",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature is a Citizen with base power and toughness \
                 1/1. It has defender and loses all other abilities. (It also \
                 loses all other creature types.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Citizen",
                        ])),
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                        AppliedEffectDef::add_ability(&abilities::defender()),
                    ]),
                },
            ),
        ]),
);

// SPM 46 — Spider-Sense
pub(in crate::card::sets) static SPIDER_SENSE: CardRecord = CardRecord::new(
    "Spider-Sense",
    "4499a25b-f4a5-4f2c-9ebd-bc68c7840b39",
    "Borja Pindado",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        web_slinging(
            "Web-slinging {U} (You may cast this spell for {U} if you also \
             return a tapped creature you control to its owner's hand.)",
            &[
                CostDef::Mana(mana_cost!("{U}")),
                CostDef::return_to_hand(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    CostQuantityDef::Fixed(1),
                ),
            ],
        ),
        AbilityDef::spell_with_targets(
            "Counter target instant spell, sorcery spell, or triggered \
             ability.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::TriggeredAbility,
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                        ]),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// SPM 47 — Unstable Experiment
pub(in crate::card::sets) static UNSTABLE_EXPERIMENT: CardRecord = CardRecord::new(
    "Unstable Experiment",
    "be9d5985-0a39-4ee3-80de-30d17d08f404",
    "David Palumbo",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws a card, then up to one target creature \
         you control connives. (Draw a card, then discard a card. If \
         you discarded a nonland card, put a +1/+1 counter on that \
         creature.)",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::LegalTargets(TargetIndex(1)),
                    predicate: ObjectSetPredicateDef {
                        filter: None,
                        comparison: ComparisonDef::Greater,
                        amount: 0,
                    },
                }),
                then: &EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                            bound: Some(crate::Binding!("connived")),
                            effect: &EffectDef::AddCounters {
                                object: EffectRecipientDef::Target(TargetIndex(1)),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                    crate::Binding!("connived"),
                                )),
                            },
                        }),
                    },
                ]),
            },
        ]),
    )]),
);

// SPM 48 — Whoosh!
pub(in crate::card::sets) static WHOOSH: CardRecord = CardRecord::new(
    "Whoosh!",
    "ccc05deb-ad8d-4fae-a7a4-2b2a118fc696",
    "Nathaniel Himawan",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        abilities::kicker(&[CostDef::Mana(mana_cost!("{1}{U}"))]),
        AbilityDef::spell_with_targets(
            "Return target nonland permanent to its owner's hand. If this \
             spell was kicked, draw a card.",
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
                    condition: &TriggerConditionDef::SourcePaidAdditionalCost(
                        AdditionalCostIndex::PRIMARY,
                    ),
                    then: &abilities::draw_cards(ValueDef::Constant(1)),
                },
            ]),
        ),
    ]),
);

// SPM 49 — Agent Venom
pub(in crate::card::sets) static AGENT_VENOM: CardRecord = CardRecord::new(
    "Agent Venom",
    "f5f80d82-d64c-466f-8874-9cfb00469f02",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Symbiote", "Soldier", "Hero"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever another nontoken creature you control dies, you draw \
                 a card and lose 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
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

// SPM 50 — Alien Symbiosis
// Audit: unsupported — Needs a graveyard casting permission with a mandatory discard additional cost that composes with other alternative costs; an AlternativeCast clause replaces the mana cost and is not equivalent.
pub(in crate::card::sets) static ALIEN_SYMBIOSIS: CardRecord = CardRecord::new(
    "Alien Symbiosis",
    "b898ccb7-758e-4f11-95e0-b412721d8bf9",
    "JB Casacop",
    CardRules::unsupported(),
);

// SPM 51 — Behold the Sinister Six!
// Audit: unsupported — Needs a distinct-card-name constraint across the selected graveyard targets; current target distinctness compares object identities, not card names.
pub(in crate::card::sets) static BEHOLD_THE_SINISTER_SIX: CardRecord = CardRecord::new(
    "Behold the Sinister Six!",
    "1919bfec-1906-4178-ad32-d4589842e563",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// SPM 52 — Black Cat, Cunning Thief
// Audit: unsupported — Needs selected cards from an inspected opponent library to be exiled face down with enduring play permission and unrestricted mana spending; the combined exile-permission operation only takes the top N cards, not a selected subset.
pub(in crate::card::sets) static BLACK_CAT_CUNNING_THIEF: CardRecord = CardRecord::new(
    "Black Cat, Cunning Thief",
    "0ed36ada-22c8-4e40-86c5-c116a0bee1c2",
    "Alessandra Pisano",
    CardRules::unsupported(),
);

// SPM 53 — Common Crook
pub(in crate::card::sets) static COMMON_CROOK: CardRecord = CardRecord::new(
    "Common Crook",
    "6f5872df-e692-44aa-b18d-22447f5f274c",
    "Ben Harvey",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue", "Villain"], 2, 2)
        .with_abilities(&[abilities::dies_trigger(
            "When this creature dies, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::create_token(tokens::treasure()).with_count(ValueDef::Constant(1)),
        )]),
);

// SPM 54 — The Death of Gwen Stacy
pub(in crate::card::sets) static THE_DEATH_OF_GWEN_STACY: CardRecord = CardRecord::new(
    "The Death of Gwen Stacy",
    "690f1f31-f8c5-4336-9ec9-72ff761e3adc",
    "Bill Sienkiewicz",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Saga"])
        .with_abilities(&[
            abilities::saga_chapter_with_targets(
                1,
                "I — Destroy target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            abilities::saga_chapter(
                2,
                "II — Each player may discard a card. Each player who doesn't \
                 loses 3 life.",
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::Sequence(&[
                        EffectDef::PayOr(
                            PayOrDef::optional_or(
                                &[CostDef::discard(ObjectPredicateDef::Any)],
                                &EffectDef::None,
                                &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::player(
                                        PlayerRefDef::EffectController,
                                    ),
                                    amount: ValueDef::Constant(3),
                                },
                            )
                            .with_payer(PlayerSetDef::One(PlayerRefDef::EffectController)),
                        ),
                        EffectDef::PayOr(
                            PayOrDef::optional_or(
                                &[CostDef::discard(ObjectPredicateDef::Any)],
                                &EffectDef::None,
                                &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::player(PlayerRefDef::Opponent),
                                    amount: ValueDef::Constant(3),
                                },
                            )
                            .with_payer(PlayerSetDef::One(PlayerRefDef::Opponent)),
                        ),
                    ]),
                    otherwise: &EffectDef::Sequence(&[
                        EffectDef::PayOr(
                            PayOrDef::optional_or(
                                &[CostDef::discard(ObjectPredicateDef::Any)],
                                &EffectDef::None,
                                &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::player(PlayerRefDef::Opponent),
                                    amount: ValueDef::Constant(3),
                                },
                            )
                            .with_payer(PlayerSetDef::One(PlayerRefDef::Opponent)),
                        ),
                        EffectDef::PayOr(
                            PayOrDef::optional_or(
                                &[CostDef::discard(ObjectPredicateDef::Any)],
                                &EffectDef::None,
                                &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::player(
                                        PlayerRefDef::EffectController,
                                    ),
                                    amount: ValueDef::Constant(3),
                                },
                            )
                            .with_payer(PlayerSetDef::One(PlayerRefDef::EffectController)),
                        ),
                    ]),
                },
            ),
            abilities::saga_chapter_with_targets(
                3,
                "III — Exile any number of target players' graveyards.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    AbilityTargetDef::UNLIMITED,
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ))),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// SPM 55 — Eddie Brock // Venom, Lethal Protector
pub(in crate::card::sets) static EDDIE_BROCK: CardRecord = CardRecord::new_dfc(
    "Eddie Brock // Venom, Lethal Protector",
    "f3455651-e643-445e-9489-51e4e24fca4c",
    "Greg Staples",
    &[
        (
            "Eddie Brock",
            CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Hero", "Villain"], 3, 3)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&[
                    abilities::enters_trigger_with_targets(
                        "When Eddie Brock enters, return target creature card with \
                         mana value 1 or less from your graveyard to the battlefield.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::ManaValueAtMost(1),
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
                    AbilityDef::activated(
                        "{3}{B}{R}{G}: Transform Eddie Brock. Activate only as a sorcery.",
                        &[CostDef::Mana(mana_cost!("{3}{B}{R}{G}"))],
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    )
                    .with_activation_timing(ActivationTimingDef::SorcerySpeed),
                ]),
        ),
        (
            "Venom, Lethal Protector",
            CardRules::new_creature(
                mana_cost!("{3}{B}{R}{G}"),
                &["Symbiote", "Hero", "Villain"],
                5,
                5,
            )
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::menace(),
                abilities::trample(),
                abilities::haste(),
                AbilityDef::triggered(
                    "Whenever Venom attacks, you may sacrifice another creature. \
                     If you do, draw X cards, then you may put a permanent card \
                     with mana value X or less from your hand onto the \
                     battlefield, where X is the sacrificed creature's mana value.",
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("victim")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                                left: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                    crate::Binding!("victim"),
                                )),
                                comparison: ComparisonDef::Greater,
                                right: ValueDef::Constant(0),
                            }),
                            then: &EffectDef::Sequence(&[
                                EffectDef::sacrifice(EffectRecipientDef::objects(
                                    ObjectSetDef::Binding(crate::Binding!("victim")),
                                )),
                                abilities::draw_cards(ValueDef::AggregateObjectValues(
                                    &ObjectValueAggregateDef {
                                        objects: ObjectSetDef::Binding(crate::Binding!("victim")),
                                        select: ObjectValueDef::ManaValue,
                                        operation: AggregateOperationDef::Sum,
                                    },
                                )),
                                EffectDef::Choose(ChooseDef {
                                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                        "chosen"
                                    )),
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
                                            ObjectPredicateDef::ManaValueAtMostValue(
                                                ValueDef::AggregateObjectValues(
                                                    &ObjectValueAggregateDef {
                                                        objects: ObjectSetDef::Binding(
                                                            crate::Binding!("victim"),
                                                        ),
                                                        select: ObjectValueDef::ManaValue,
                                                        operation: AggregateOperationDef::Sum,
                                                    },
                                                ),
                                            ),
                                        ]),
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
                            ]),
                        },
                    }),
                ),
            ]),
        ),
    ],
);

// SPM 56 — Gwenom, Remorseless
// Audit: unsupported — Needs a resolving, expiring permission to look at the top library card at any time; top-card play permission already supports life payment, but MayLookAtTopOfLibrary is only implemented as a battlefield static rule.
pub(in crate::card::sets) static GWENOM_REMORSELESS: CardRecord = CardRecord::new(
    "Gwenom, Remorseless",
    "46b6cc5d-7a37-4e8b-a1a5-9a573056610c",
    "Lordigan",
    CardRules::unsupported(),
);

// SPM 57 — Inner Demons Gangsters
pub(in crate::card::sets) static INNER_DEMONS_GANGSTERS: CardRecord = CardRecord::new(
    "Inner Demons Gangsters",
    "f0252819-4eda-457d-9688-b08b83b1edc9",
    "Nathaniel Himawan",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Rogue", "Villain"], 3, 4)
        .with_abilities(&[AbilityDef::activated(
            "Discard a card: This creature gets +1/+0 and gains menace \
             until end of turn. Activate only as a sorcery. (It can't be \
             blocked except by two or more creatures.)",
            &[CostDef::discard(ObjectPredicateDef::Any)],
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
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)]),
);

// SPM 58 — Merciless Enforcers
pub(in crate::card::sets) static MERCILESS_ENFORCERS: CardRecord = CardRecord::new(
    "Merciless Enforcers",
    "fba9c76c-1432-4554-88bd-3f5e8709a963",
    "Alex Horley-Orlandelli",
    CardRules::new_creature(
        mana_cost!("{1}{B}"),
        &["Human", "Mercenary", "Villain"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::lifelink(),
        AbilityDef::activated(
            "{3}{B}: This creature deals 1 damage to each opponent.",
            &[CostDef::Mana(mana_cost!("{3}{B}"))],
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
    ]),
);

// SPM 59 — Morlun, Devourer of Spiders
// Audit: unsupported — Needs retained cast X for the enters trigger after Morlun leaves the battlefield; SourceCastX currently reads the live permanent, so it loses X when the source departs before resolution.
pub(in crate::card::sets) static MORLUN_DEVOURER_OF_SPIDERS: CardRecord = CardRecord::new(
    "Morlun, Devourer of Spiders",
    "1beb2eb9-90b5-43ba-8b04-cfce7dcb744b",
    "Randy Gallegos",
    CardRules::unsupported(),
);

// SPM 60 — Parker Luck
// Audit: unsupported — Needs distinctness enforced across separately addressable player-target slots on a triggered ability; current trigger placement ignores another across slots, while a single two-player slot has no indexed-member reference to pair each reveal with the other player.
pub(in crate::card::sets) static PARKER_LUCK: CardRecord = CardRecord::new(
    "Parker Luck",
    "e375bcf0-7fcb-4fe4-a7e8-a4cbf9b23e3c",
    "Raoul Vitale",
    CardRules::unsupported(),
);

// SPM 61 — Prison Break
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static PRISON_BREAK: CardRecord = CardRecord::new(
    "Prison Break",
    "6c45a5df-048e-4b73-89c6-5cdaa330319e",
    "John Tyler Christopher",
    CardRules::unsupported(),
);

// SPM 62 — Risky Research
pub(in crate::card::sets) static RISKY_RESEARCH: CardRecord = CardRecord::new(
    "Risky Research",
    "1f8aa705-6177-42e9-95cb-e7f880c186e3",
    "Rafater",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell(
        "Surveil 2, then draw two cards. You lose 2 life. (To surveil \
         2, look at the top two cards of your library, then put any \
         number of them into your graveyard and the rest on top of \
         your library in any order.)",
        EffectDef::Sequence(&[
            abilities::surveil(ValueDef::Constant(2)),
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// SPM 63 — Sandman's Quicksand
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static SANDMAN_S_QUICKSAND: CardRecord = CardRecord::new(
    "Sandman's Quicksand",
    "b7795e17-6717-464c-9ae3-20da52ba005a",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// SPM 64 — Scorpion, Seething Striker
pub(in crate::card::sets) static SCORPION_SEETHING_STRIKER: CardRecord = CardRecord::new(
    "Scorpion, Seething Striker",
    "cf407e08-b27f-42ba-b824-75846a80e238",
    "Simon Dominic",
    CardRules::new_creature(
        mana_cost!("{3}{B}"),
        &["Scorpion", "Human", "Villain"],
        3,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered_if_with_targets(
            "At the beginning of your end step, if a creature died this \
             turn, target creature you control connives. (Draw a card, \
             then discard a card. If you discarded a nonland card, put a \
             +1/+1 counter on that creature.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::CreatureDiedThisTurn,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("connived")),
                        effect: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                crate::Binding!("connived"),
                            )),
                        },
                    }),
                },
            ]),
        ),
    ]),
);

// SPM 65 — Scorpion's Sting
pub(in crate::card::sets) static SCORPION_S_STING: CardRecord = CardRecord::new(
    "Scorpion's Sting",
    "0fb03437-32cf-4c97-bf91-ea8b2ad3f964",
    "Lee Woo-chul",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -3/-3 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-3),
                ValueDef::Constant(-3),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SPM 66 — The Soul Stone
// Audit: unsupported — Needs a durable harnessed designation and the infinity ability enabled by that designation; this is not a removable counter, an activated-ability use limit, or a gained ability.
pub(in crate::card::sets) static THE_SOUL_STONE: CardRecord = CardRecord::new(
    "The Soul Stone",
    "1982f910-a9bd-4e94-a187-84381b22aacc",
    "Volkan Baǵa",
    CardRules::unsupported(),
);

// SPM 67 — Spider-Man Noir
// Audit: unsupported — Needs a scalar counting every kind of counter on the triggering creature; existing counter projections require one specific CounterKind and cannot total an arbitrary inventory.
pub(in crate::card::sets) static SPIDER_MAN_NOIR: CardRecord = CardRecord::new(
    "Spider-Man Noir",
    "bc64366c-2691-48cd-bb4b-a4b088c6f16b",
    "Xabi Gaztelua",
    CardRules::unsupported(),
);

// SPM 68 — The Spot's Portal
pub(in crate::card::sets) static THE_SPOT_S_PORTAL: CardRecord = CardRecord::new(
    "The Spot's Portal",
    "67a8bf52-7562-4cdd-b970-106717a0aad6",
    "Carlos Dattoli",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put target creature on the bottom of its owner's library. You \
         lose 2 life unless you control a Villain.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Library,
                ZonePlacement::Bottom,
            ),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                }),
                then: &EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            },
        ]),
    )]),
);

// SPM 69 — Swarm, Being of Bees
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static SWARM_BEING_OF_BEES: CardRecord = CardRecord::new(
    "Swarm, Being of Bees",
    "cb83d54e-6641-4929-99ad-c0ba5b610902",
    "Alex Horley-Orlandelli",
    CardRules::unsupported(),
);

// SPM 70 — Tombstone, Career Criminal
pub(in crate::card::sets) static TOMBSTONE_CAREER_CRIMINAL: CardRecord = CardRecord::new(
    "Tombstone, Career Criminal",
    "313189ef-fe6e-4511-9386-920a88a49a88",
    "Bartek Fedyczak",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Villain"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When Tombstone enters, return target Villain card from your \
                 graveyard to your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
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
                "Villain spells you cast cost {1} less to cast.",
                EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                    PlayerRelation::You,
                    ValueDef::Constant(1),
                )),
            ),
        ]),
);

// SPM 71 — Venom, Evil Unleashed
pub(in crate::card::sets) static VENOM_EVIL_UNLEASHED: CardRecord = CardRecord::new(
    "Venom, Evil Unleashed",
    "ab3d51a4-40f0-4606-b5f9-2686c12fd54b",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Symbiote", "Villain"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::activated_with_targets(
                "{2}{B}, Exile this card from your graveyard: Put two +1/+1 \
                 counters on target creature. It gains deathtouch until end of \
                 turn. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{2}{B}")), CostDef::ExileSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// SPM 72 — Venomized Cat
pub(in crate::card::sets) static VENOMIZED_CAT: CardRecord = CardRecord::new(
    "Venomized Cat",
    "6330f3e9-e031-4d55-b5d8-536c16bba063",
    "Jessica Fong",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Symbiote", "Cat", "Villain"], 2, 3)
        .with_abilities(&[
            abilities::deathtouch(),
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

// SPM 73 — Venom's Hunger
pub(in crate::card::sets) static VENOM_S_HUNGER: CardRecord = CardRecord::new(
    "Venom's Hunger",
    "01d276cd-e4ad-488f-8447-004aefad1ebb",
    "Dave DeVries",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {2} less to cast if you control a Villain.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Greater,
                amount: 0,
                then: ValueDef::Constant(2),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell_with_targets(
            "Destroy target creature. You gain 2 life.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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
        ),
    ]),
);

// SPM 74 — Villainous Wrath
pub(in crate::card::sets) static VILLAINOUS_WRATH: CardRecord = CardRecord::new(
    "Villainous Wrath",
    "d78e36fd-5817-4c4a-8880-dabe6dd4ba81",
    "InHyuk Lee",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent loses life equal to the number of creatures \
             they control. Then destroy all creatures.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    )),
                },
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
            ]),
        ),
    ]),
);

// SPM 75 — Angry Rabble
pub(in crate::card::sets) static ANGRY_RABBLE: CardRecord = CardRecord::new(
    "Angry Rabble",
    "938730fa-496f-4871-80ec-3e9843ecb219",
    "Bartek Fedyczak",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Citizen"], 2, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever you cast a spell with mana value 4 or greater, this \
             creature deals 1 damage to each opponent.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
        AbilityDef::activated(
            "{5}{R}: Put two +1/+1 counters on this creature. Activate \
             only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{5}{R}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// SPM 76 — Electro, Assaulting Battery
// Audit: unsupported — Needs a resolving optional X-mana payment followed by a reflexive targeted trigger after the source has left; OptionalEffectTaken listeners require a live battlefield source and do not retain a selected resolving X.
pub(in crate::card::sets) static ELECTRO_ASSAULTING_BATTERY: CardRecord = CardRecord::new(
    "Electro, Assaulting Battery",
    "d672cfad-e656-47f8-bf93-64f262aff33e",
    "Piotr Dura",
    CardRules::unsupported(),
);

// SPM 77 — Electro's Bolt
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static ELECTRO_S_BOLT: CardRecord = CardRecord::new(
    "Electro's Bolt",
    "25fe063f-35e4-4fca-9889-06834a8ef9b9",
    "JB Casacop",
    CardRules::unsupported(),
);

// SPM 78 — Gwen Stacy // Ghost-Spider
// Audit: unsupported — Needs exile-play permission lasting only while this exact creature remains continuously under your control, including termination on control changes; existing permissions have no source-control lifetime.
pub(in crate::card::sets) static GWEN_STACY: CardRecord = CardRecord::new(
    "Gwen Stacy // Ghost-Spider",
    "b0f1597f-1dc7-465e-8fcb-0afe61bcca46",
    "Victor Adame Minguez",
    CardRules::unsupported(),
);

// SPM 79 — Heroes' Hangout
// Audit: unsupported — Needs play permission granted to a chosen already-exiled card until the end of your next turn; current duration-bearing exile permissions operate on the top-card exile move itself.
pub(in crate::card::sets) static HEROES_HANGOUT: CardRecord = CardRecord::new(
    "Heroes' Hangout",
    "4148d7e8-6371-468c-858b-35254995409a",
    "Smirtouille",
    CardRules::unsupported(),
);

// SPM 80 — Hobgoblin, Mantled Marauder
pub(in crate::card::sets) static HOBGOBLIN_MANTLED_MARAUDER: CardRecord = CardRecord::new(
    "Hobgoblin, Mantled Marauder",
    "50716fe3-7a19-431e-8758-984fc48d714e",
    "Dave DeVries",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Human", "Villain"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever you discard a card, Hobgoblin gets +2/+0 until end \
                 of turn.",
                TriggerEventDef::Discarded(PlayerRelation::You),
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

// SPM 81 — J. Jonah Jameson
// Audit: unsupported — Needs durable suspected status, including its inherent menace and cannot-block rules; granting these abilities does not model status independently of ability removal.
pub(in crate::card::sets) static J_JONAH_JAMESON: CardRecord = CardRecord::new(
    "J. Jonah Jameson",
    "9ee905d6-b647-4eb1-a8d9-89add9bafc31",
    "Paolo Rivera",
    CardRules::unsupported(),
);

// SPM 82 — Masked Meower
pub(in crate::card::sets) static MASKED_MEOWER: CardRecord = CardRecord::new(
    "Masked Meower",
    "6aa0dc1f-6c83-4ac9-b4f2-428e0e0bbf88",
    "Narendra Bintara Adi",
    CardRules::new_creature(mana_cost!("{R}"), &["Spider", "Cat", "Hero"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated(
            "Discard a card, Sacrifice this creature: Draw a card.",
            &[
                CostDef::discard(ObjectPredicateDef::Any),
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// SPM 83 — Maximum Carnage
// Audit: unsupported — Needs an attack requirement preferring a player other than the effect controller when able; existing attack restrictions cannot express this second, independently satisfiable requirement.
pub(in crate::card::sets) static MAXIMUM_CARNAGE: CardRecord = CardRecord::new(
    "Maximum Carnage",
    "7d72d867-6ed2-4900-a8ae-9d86f581ce32",
    "Bill Sienkiewicz",
    CardRules::unsupported(),
);

// SPM 84 — Molten Man, Inferno Incarnate
pub(in crate::card::sets) static MOLTEN_MAN_INFERNO_INCARNATE: CardRecord = CardRecord::new(
    "Molten Man, Inferno Incarnate",
    "f469d621-25d0-4d8e-909f-47dac0b9c5b0",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Villain"], 0, 0)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Molten Man enters, search your library for a basic \
                 Mountain card, put it onto the battlefield tapped, then \
                 shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
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
            AbilityDef::static_ability(
                "Molten Man gets +1/+1 for each Mountain you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    ),
                },
            ),
            AbilityDef::triggered(
                "When Molten Man leaves the battlefield, sacrifice a land.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
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
            ),
        ]),
);

// SPM 85 — Raging Goblinoids
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static RAGING_GOBLINOIDS: CardRecord = CardRecord::new(
    "Raging Goblinoids",
    "8519598f-ab7f-49b0-90cc-c0b6422ebdf8",
    "Filipe Pagliuso",
    CardRules::unsupported(),
);

// SPM 86 — Romantic Rendezvous
pub(in crate::card::sets) static ROMANTIC_RENDEZVOUS: CardRecord = CardRecord::new(
    "Romantic Rendezvous",
    "38120361-153f-414e-8a45-f86bb2e35a17",
    "Nereida",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell(
        "Discard a card, then draw two cards.",
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            abilities::draw_cards(ValueDef::Constant(2)),
        ]),
    )]),
);

// SPM 87 — Shadow of the Goblin
// Audit: unsupported — Needs land-play events that retain the zone from which the land was played; ordinary enters events also include lands put onto the battlefield and cannot substitute for a play event.
pub(in crate::card::sets) static SHADOW_OF_THE_GOBLIN: CardRecord = CardRecord::new(
    "Shadow of the Goblin",
    "854b6898-c480-435b-8952-a077c7977cec",
    "Pavel Kolomeyets",
    CardRules::unsupported(),
);

// SPM 88 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::SHOCK,
    "760b41a1-c087-4b11-b8a0-fb01d8a4c0c6",
    "Piotr Dura",
);

// SPM 89 — Shocker, Unshakable
pub(in crate::card::sets) static SHOCKER_UNSHAKABLE: CardRecord = CardRecord::new(
    "Shocker, Unshakable",
    "8b2c0d9a-364a-4823-aa4c-fe473d4463f0",
    "Kevin Glint",
    CardRules::new_creature(
        mana_cost!("{4}{R}{R}"),
        &["Human", "Rogue", "Villain"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::static_ability(
            "During your turn, Shocker has first strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
        abilities::enters_trigger_with_targets(
            "Vibro-Shock Gauntlets — When Shocker enters, he deals 2 \
             damage to target creature and 2 damage to that creature's \
             controller.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage_simultaneously(&[
                DamageAssignmentDef::from_effect(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
                DamageAssignmentDef::from_effect(
                    EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    ValueDef::Constant(2),
                ),
            ]),
        ),
    ]),
);

// SPM 90 — Spider-Gwen, Free Spirit
pub(in crate::card::sets) static SPIDER_GWEN_FREE_SPIRIT: CardRecord = CardRecord::new(
    "Spider-Gwen, Free Spirit",
    "3bc04fa7-6265-4549-91f6-eebdcd67398a",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Spider", "Human", "Hero"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered(
                "Whenever Spider-Gwen becomes tapped, you may discard a card. \
                 If you do, draw a card.",
                TriggerEventDef::tapped(ObjectPredicateDef::Source),
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::discard(ObjectPredicateDef::Any)],
                    &abilities::draw_cards(ValueDef::Constant(1)),
                )),
            ),
        ]),
);

// SPM 91 — Spider-Islanders
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static SPIDER_ISLANDERS: CardRecord = CardRecord::new(
    "Spider-Islanders",
    "c9132e45-4ddb-4565-ac45-86f1ecc6230d",
    "Helge C. Balzer",
    CardRules::unsupported(),
);

// SPM 92 — Spider-Punk
// Audit: unsupported — Needs a static grant of riot applied during other Spiders' prospective entries, including their counter-or-haste choice; a battlefield ability grant begins after the entry replacement must run.
pub(in crate::card::sets) static SPIDER_PUNK: CardRecord = CardRecord::new(
    "Spider-Punk",
    "0bd41879-fcd4-4211-9b98-47e7cdba5399",
    "Forrest Imel",
    CardRules::unsupported(),
);

// SPM 93 — Spider-Verse
// Audit: unsupported — Needs copied permanent spells to carry haste into their resulting permanents and an optional once-per-turn limit consumed only on acceptance; stack-copy effects lack this arrival rider, and trigger limits count the trigger itself.
pub(in crate::card::sets) static SPIDER_VERSE: CardRecord = CardRecord::new(
    "Spider-Verse",
    "f8779eb2-1210-430d-8d42-3077053441ee",
    "Alexander Gering",
    CardRules::unsupported(),
);

// SPM 94 — Spinneret and Spiderling
// Audit: unsupported — Needs a source damage-event matcher for a single simultaneous event totaling four or more damage across recipients; current damage triggers are captured per recipient and lose the event-wide total.
pub(in crate::card::sets) static SPINNERET_AND_SPIDERLING: CardRecord = CardRecord::new(
    "Spinneret and Spiderling",
    "a27834b7-e763-48ac-845e-ed49f2fa6c6d",
    "Le Vuong",
    CardRules::unsupported(),
);

// SPM 95 — Stegron the Dinosaur Man
pub(in crate::card::sets) static STEGRON_THE_DINOSAUR_MAN: CardRecord = CardRecord::new(
    "Stegron the Dinosaur Man",
    "485ceacb-fa76-4517-8466-c3c6bf6bcd6e",
    "John Tyler Christopher",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Dinosaur", "Villain"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::activated_with_targets(
                "Dinosaur Formula — {1}{R}, Discard this card: Until end of \
                 turn, target creature you control gets +3/+1 and becomes a \
                 Dinosaur in addition to its other types.",
                &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::DiscardSource],
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
                            ValueDef::Constant(3),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Dinosaur",
                        ])),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .with_source_zones(&[ZoneKind::Hand]),
        ]),
);

// SPM 96 — Superior Foes of Spider-Man
// Audit: unsupported — Needs a source-bound exile-play permission group that replaces and expires the previous permission only after another card is successfully exiled with that source.
pub(in crate::card::sets) static SUPERIOR_FOES_OF_SPIDER_MAN: CardRecord = CardRecord::new(
    "Superior Foes of Spider-Man",
    "28e7bf86-5791-4412-8184-fa63fb292be4",
    "Ben Harvey",
    CardRules::unsupported(),
);

// SPM 97 — Taxi Driver
pub(in crate::card::sets) static TAXI_DRIVER: CardRecord = CardRecord::new(
    "Taxi Driver",
    "a80d3ed9-5e81-41b7-bb74-ab86cba841c8",
    "Néstor Ossandón Leal",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Pilot"], 3, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Target creature gains haste until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SPM 98 — Wisecrack
pub(in crate::card::sets) static WISECRACK: CardRecord = CardRecord::new(
    "Wisecrack",
    "8f452dac-bf22-4010-8a10-3c1cfa7d4df6",
    "Wayne Reynolds",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature deals damage equal to its power to itself. If \
         that creature is attacking, Wisecrack deals 2 damage to that \
         creature's controller.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::Attacking,
                },
                then: &EffectDef::damage(
                    EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    ))),
                    ValueDef::Constant(2),
                ),
            },
        ]),
    )]),
);

// SPM 99 — Damage Control Crew
pub(in crate::card::sets) static DAMAGE_CONTROL_CREW: CardRecord = CardRecord::new(
    "Damage Control Crew",
    "ad2cab87-691d-44fe-ab2f-33760b1feb0f",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Human", "Citizen"], 3, 3).with_abilities(&[
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Repair — Return target card with mana value 4 or greater from \
                     your graveyard to your hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(
                                3,
                            )),
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
                AbilityDef::spell_with_targets(
                    "Impound — Exile target artifact or enchantment.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                ),
            ],
        ),
    ]),
);

// SPM 100 — Ezekiel Sims, Spider-Totem
pub(in crate::card::sets) static EZEKIEL_SIMS_SPIDER_TOTEM: CardRecord = CardRecord::new(
    "Ezekiel Sims, Spider-Totem",
    "bb7c3ae2-6b01-4472-8bd1-9a7456401ddc",
    "Wei Guan",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spider", "Human", "Advisor"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::reach(),
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, target Spider you \
                 control gets +2/+2 until end of turn.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
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

// SPM 101 — Grow Extra Arms
// Audit: unsupported — Needs a self casting-cost reduction based on the chosen target's Spider subtype; current self-cost predicates cannot inspect the pending target selection.
pub(in crate::card::sets) static GROW_EXTRA_ARMS: CardRecord = CardRecord::new(
    "Grow Extra Arms",
    "63fab399-00db-4398-922e-c3ca3356731a",
    "Kevin Sidharta",
    CardRules::unsupported(),
);

// SPM 102 — Guy in the Chair
pub(in crate::card::sets) static GUY_IN_THE_CHAIR: CardRecord = CardRecord::new(
    "Guy in the Chair",
    "65e97c06-55a6-4841-be0f-055c015df90a",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Advisor"], 2, 3).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated_with_targets(
            "Web Support — {2}{G}, {T}: Put a +1/+1 counter on target \
             Spider. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}{G}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// SPM 103 — Kapow!
pub(in crate::card::sets) static KAPOW: CardRecord = CardRecord::new(
    "Kapow!",
    "cec575f6-43c9-41c6-a996-bb806bf82185",
    "Jessica Fong",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature you control. It fights \
         target creature an opponent controls. (Each deals damage \
         equal to its power to the other.)",
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
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ]),
    )]),
);

// SPM 104 — Kraven's Cats
pub(in crate::card::sets) static KRAVEN_S_CATS: CardRecord = CardRecord::new(
    "Kraven's Cats",
    "86c415d8-1d2d-4339-955b-0f2aebeb3c95",
    "Kevin Glint",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Cat", "Villain"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{2}{G}: This creature gets +2/+2 until end of turn. Activate \
             only once each turn.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ]),
);

// SPM 105 — Kraven's Last Hunt
// Audit: unsupported — Needs a reflexive targeted trigger after milling that survives the Saga leaving in response to its chapter ability; OptionalEffectTaken listeners require a live battlefield source.
pub(in crate::card::sets) static KRAVEN_S_LAST_HUNT: CardRecord = CardRecord::new(
    "Kraven's Last Hunt",
    "d0c18ffe-a2b9-40df-a6b4-a9381e6dc467",
    "Bill Sienkiewicz",
    CardRules::unsupported(),
);

// SPM 106 — Lizard, Connors's Curse
pub(in crate::card::sets) static LIZARD_CONNORS_S_CURSE: CardRecord = CardRecord::new(
    "Lizard, Connors's Curse",
    "6add5d2a-950e-4bee-9850-e68f5f6d6142",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Lizard", "Villain"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            abilities::enters_trigger_with_targets(
                "Lizard Formula — When Lizard, Connors's Curse enters, up to \
                 one other target creature loses all abilities and becomes a \
                 green Lizard creature with base power and toughness 4/4.",
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
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Lizard",
                        ])),
                        AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
        ]),
);

// SPM 107 — Lurking Lizards
pub(in crate::card::sets) static LURKING_LIZARDS: CardRecord = CardRecord::new(
    "Lurking Lizards",
    "58b5b49c-ddd6-4d1b-9b61-6e02d8fc55ad",
    "Rafater",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Lizard", "Villain"], 1, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever you cast a spell with mana value 4 or greater, put a \
             +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
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

// SPM 108 — Miles Morales // Ultimate Spider-Man
// Audit: unsupported — Needs doubling of every counter kind on a dynamically selected permanent group; DoubleCounters and counter projections require an explicitly named kind.
pub(in crate::card::sets) static MILES_MORALES: CardRecord = CardRecord::new(
    "Miles Morales // Ultimate Spider-Man",
    "9f8b4d9b-208a-4673-a617-5e3edd069c33",
    "L.A. Draws",
    CardRules::unsupported(),
);

// SPM 109 — Pictures of Spider-Man
pub(in crate::card::sets) static PICTURES_OF_SPIDER_MAN: CardRecord = CardRecord::new(
    "Pictures of Spider-Man",
    "e1ec41d4-0180-42f7-9c54-f3c39b4ffb8d",
    "Rafater",
    CardRules::new_artifact(mana_cost!("{2}{G}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, look at the top five cards of your \
             library. You may reveal up to two creature cards from among \
             them and put them into your hand. Put the rest on the bottom \
             of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                minimum: 0,
                maximum: 2,
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
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: Create a Treasure token. \
             (It's an artifact with \"{T}, Sacrifice this token: Add one \
             mana of any color.\")",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::create_token(tokens::treasure()).with_count(ValueDef::Constant(1)),
        ),
    ]),
);

// SPM 110 — Professional Wrestler
pub(in crate::card::sets) static PROFESSIONAL_WRESTLER: CardRecord = CardRecord::new(
    "Professional Wrestler",
    "8a5381e7-ddda-47e7-886d-812250ffb745",
    "Kevin Sidharta",
    CardRules::new_creature(
        mana_cost!("{3}{G}"),
        &["Human", "Warrior", "Performer"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::create_token(tokens::treasure()).with_count(ValueDef::Constant(1)),
        ),
        AbilityDef::static_ability(
            "This creature can't be blocked by more than one creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::MaximumBlockers(1),
                )),
            },
        ),
    ]),
);

// SPM 111 — Radioactive Spider
pub(in crate::card::sets) static RADIOACTIVE_SPIDER: CardRecord = CardRecord::new(
    "Radioactive Spider",
    "f2d267f5-7f12-45f8-8fcb-e0ba3fbdeddc",
    "Pavel Kolomeyets",
    CardRules::new_creature(mana_cost!("{G}"), &["Spider"], 1, 1).with_abilities(&[
        abilities::reach(),
        abilities::deathtouch(),
        AbilityDef::activated(
            "Fateful Bite — {2}, Sacrifice this creature: Search your \
             library for a Spider Hero card, reveal it, put it into your \
             hand, then shuffle. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Hero")),
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
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// SPM 112 — Sandman, Shifting Scoundrel
pub(in crate::card::sets) static SANDMAN_SHIFTING_SCOUNDREL: CardRecord = CardRecord::new(
    "Sandman, Shifting Scoundrel",
    "609ac18c-ec58-4fa7-bbee-3912a69d0ec6",
    "Bartek Fedyczak",
    CardRules::new_creature(
        mana_cost!("{1}{G}{G}"),
        &["Sand", "Elemental", "Villain"],
        0,
        0,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::static_ability(
            "Sandman's power and toughness are each equal to the number of \
             lands you control.",
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
        AbilityDef::static_ability(
            "Sandman can't be blocked by creatures with power 2 or less.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                )),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}{G}{G}: Return this card and target land card from your \
             graveyard to the battlefield tapped.",
            &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Union(&[
                        ObjectSetDef::One(ObjectRefDef::Source),
                        ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ])),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::Tapped],
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// SPM 113 — Scout the City
pub(in crate::card::sets) static SCOUT_THE_CITY: CardRecord = CardRecord::new(
    "Scout the City",
    "90b9504d-d23d-402f-8b16-1964ebd8f6b9",
    "Rafater",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Look Around — Mill three cards. You may put a permanent card \
                 from among them into your hand. You gain 3 life. (To mill \
                 three cards, put the top three cards of your library into \
                 your graveyard.)",
                EffectDef::Sequence(&[
                    EffectDef::Sequence(&[
                        EffectDef::BindOutput {
                            binding: crate::Binding!("milled"),
                            effect: &EffectDef::Mill {
                                player: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(3),
                            },
                        },
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Matching {
                                objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                                object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(
                                    &[
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::HasType(CardType::Land),
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                    ],
                                )),
                            },
                            exclude: None,
                            minimum: 0,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                        }),
                    ]),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Bring Down — Destroy target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
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

// SPM 114 — Spider-Ham, Peter Porker
pub(in crate::card::sets) static SPIDER_HAM_PETER_PORKER: CardRecord = CardRecord::new(
    "Spider-Ham, Peter Porker",
    "41f18f42-b86b-4a12-9f0d-76b761571195",
    "Filipe Pagliuso",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider", "Boar", "Hero"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Spider-Ham enters, create a Food token. (It's an \
                 artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
                 life.\")",
                EffectDef::create_token(tokens::food()).with_count(ValueDef::Constant(1)),
            ),
            AbilityDef::static_ability(
                "Animal May-Ham — Other Spiders, Boars, Bats, Bears, Birds, \
                 Cats, Dogs, Frogs, Jackals, Lizards, Mice, Otters, Rabbits, \
                 Raccoons, Rats, Squirrels, Turtles, and Wolves you control \
                 get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Boar")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bat")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bear")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bird")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dog")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Frog")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Jackal")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Lizard")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mouse")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Otter")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rabbit")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Raccoon")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Squirrel")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Turtle")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wolf")),
                                ]),
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

// SPM 115 — Spider-Man, Brooklyn Visionary
pub(in crate::card::sets) static SPIDER_MAN_BROOKLYN_VISIONARY: CardRecord = CardRecord::new(
    "Spider-Man, Brooklyn Visionary",
    "e19929bc-cbe1-4970-952d-8e9d0193ddce",
    "Aniekan Udofia",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spider", "Human", "Hero"], 4, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            web_slinging(
                "Web-slinging {2}{G} (You may cast this spell for {2}{G} if \
                 you also return a tapped creature you control to its owner's \
                 hand.)",
                &[
                    CostDef::Mana(mana_cost!("{2}{G}")),
                    CostDef::return_to_hand(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Tapped,
                        ]),
                        CostQuantityDef::Fixed(1),
                    ),
                ],
            ),
            abilities::enters_trigger(
                "When Spider-Man enters, search your library for a basic land \
                 card, put it onto the battlefield tapped, then shuffle.",
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

// SPM 116 — Spider-Rex, Daring Dino
pub(in crate::card::sets) static SPIDER_REX_DARING_DINO: CardRecord = CardRecord::new(
    "Spider-Rex, Daring Dino",
    "5b6e0bea-f126-4adb-8808-901950a77c7b",
    "Narendra Bintara Adi",
    CardRules::new_creature(
        mana_cost!("{4}{G}{G}"),
        &["Spider", "Dinosaur", "Hero"],
        6,
        6,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
    ]),
);

// SPM 117 — Spiders-Man, Heroic Horde
pub(in crate::card::sets) static SPIDERS_MAN_HEROIC_HORDE: CardRecord = CardRecord::new(
    "Spiders-Man, Heroic Horde",
    "1183262e-1f02-46b3-8cfa-fe30e0016c11",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider", "Hero"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            web_slinging(
                "Web-slinging {4}{G}{G} (You may cast this spell for {4}{G}{G} \
                 if you also return a tapped creature you control to its \
                 owner's hand.)",
                &[
                    CostDef::Mana(mana_cost!("{4}{G}{G}")),
                    CostDef::return_to_hand(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Tapped,
                        ]),
                        CostQuantityDef::Fixed(1),
                    ),
                ],
            ),
            AbilityDef::triggered_if(
                "When Spiders-Man enters, if they were cast using \
                 web-slinging, you gain 3 life and create two 2/1 green Spider \
                 creature tokens with reach.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &TriggerConditionDef::SourcePaidAlternativeCost(crate::Binding!("web-slinging")),
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::create_creature_token(&["Spider"], &[ManaColor::Green], 2, 1)
                        .with_count(ValueDef::Constant(2))
                        .with_abilities(&[abilities::reach()]),
                ]),
            ),
        ]),
);

// SPM 118 — Strength of Will
pub(in crate::card::sets) static STRENGTH_OF_WILL: CardRecord = CardRecord::new(
    "Strength of Will",
    "68f985c7-7765-46c3-ad31-edae3abb9fbf",
    "Ryan Pancoast",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature you control gains \
         indestructible and \"Whenever this creature is dealt damage, \
         put that many +1/+1 counters on it.\"",
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
                AppliedEffectDef::add_ability(&abilities::indestructible()),
                AppliedEffectDef::add_ability(&AbilityDef::triggered(
                    "Whenever this creature is dealt damage, put that many +1/+1 \
                     counters on it.",
                    TriggerEventDef::damage_to_source(),
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::TriggerEventAmount,
                    },
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SPM 119 — Supportive Parents
// Audit: unsupported — Needs a mana-ability payment that selects and taps two untapped controlled creatures; multi-permanent tap costs work for ordinary activations but the immediate mana-ability payment path does not support them.
pub(in crate::card::sets) static SUPPORTIVE_PARENTS: CardRecord = CardRecord::new(
    "Supportive Parents",
    "d3fe8a5b-4166-46cc-b910-71cd1a19ae1b",
    "Kim Sokol",
    CardRules::unsupported(),
);

// SPM 120 — Terrific Team-Up
pub(in crate::card::sets) static TERRIFIC_TEAM_UP: CardRecord = CardRecord::new(
    "Terrific Team-Up",
    "f3c587b0-66b9-46bf-90ee-a6163c006c9e",
    "InHyuk Lee",
    CardRules::new_instant(mana_cost!("{3}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {2} less to cast if you control a permanent \
             with mana value 4 or greater.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Greater,
                amount: 0,
                then: ValueDef::Constant(2),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell_with_targets(
            "One or two target creatures you control each get +1/+0 until \
             end of turn. They each deal damage equal to their power to \
             target creature an opponent controls.",
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
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                        ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                        ObjectSetDef::LegalTargets(TargetIndex(1)),
                    ])),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::damage_simultaneously(&[
                    DamageAssignmentDef::from(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                        EffectRecipientDef::Target(TargetIndex(2)),
                        ValueDef::TargetPower(TargetIndex::PRIMARY),
                    ),
                    DamageAssignmentDef::from(
                        ObjectRefDef::Target(TargetIndex(1)),
                        EffectRecipientDef::Target(TargetIndex(2)),
                        ValueDef::TargetPower(TargetIndex(1)),
                    ),
                ]),
            ]),
        ),
    ]),
);

// SPM 121 — Wall Crawl
pub(in crate::card::sets) static WALL_CRAWL: CardRecord = CardRecord::new(
    "Wall Crawl",
    "97a2a1ab-57ec-4210-9412-765ae4f02db0",
    "Alexander Gering",
    CardRules::new_enchantment(mana_cost!("{3}{G}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 2/1 green Spider \
             creature token with reach, then you gain 1 life for each \
             Spider you control.",
            EffectDef::Sequence(&[
                EffectDef::create_creature_token(&["Spider"], &[ManaColor::Green], 2, 1)
                    .with_abilities(&[abilities::reach()]),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ]),
        ),
        AbilityDef::static_ability(
            "Spiders you control get +1/+1 and can't be blocked by \
             creatures with defender.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Defender),
                    )),
                ]),
            },
        ),
    ]),
);

// SPM 122 — Web of Life and Destiny
pub(in crate::card::sets) static WEB_OF_LIFE_AND_DESTINY: CardRecord = CardRecord::new(
    "Web of Life and Destiny",
    "3b3c609e-f7c9-4fe5-84d9-4f4c76020a4b",
    "Jonas De Ro",
    CardRules::new_enchantment(mana_cost!("{6}{G}{G}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::triggered(
            "At the beginning of combat on your turn, look at the top five \
             cards of your library. You may put a creature card from among \
             them onto the battlefield. Put the rest on the bottom of your \
             library in a random order.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                minimum: 0,
                maximum: 1,
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

// SPM 123 — Araña, Heart of the Spider
// Audit: unsupported — Needs a complete modified predicate, including Equipment attached to the creature and Auras controlled by its controller; the available counter and enchanted predicates cannot identify all three modifications with their required controller scope.
pub(in crate::card::sets) static ARANA_HEART_OF_THE_SPIDER: CardRecord = CardRecord::new(
    "Araña, Heart of the Spider",
    "b02bfa0e-f761-45e1-b35c-f44ff7c5d0e8",
    "Kevin Glint",
    CardRules::unsupported(),
);

// SPM 124 — Biorganic Carapace
// Audit: unsupported — Needs a complete modified predicate, including Equipment attached to the creature and Auras controlled by its controller; the available counter and enchanted predicates cannot identify all three modifications with their required controller scope.
pub(in crate::card::sets) static BIORGANIC_CARAPACE: CardRecord = CardRecord::new(
    "Biorganic Carapace",
    "9658fdab-9702-4e13-bc53-01a25a2ed41a",
    "David Álvarez",
    CardRules::unsupported(),
);

// SPM 125 — Carnage, Crimson Chaos
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static CARNAGE_CRIMSON_CHAOS: CardRecord = CardRecord::new(
    "Carnage, Crimson Chaos",
    "930befba-6068-493e-baa2-e9371cd99e93",
    "Lordigan",
    CardRules::unsupported(),
);

// SPM 126 — Cheering Crowd
// Audit: unsupported — Needs resolving mana addition to the active player who accepted the optional counter placement; AddManaEffectDef has a recipient field, but nonmana stack resolution ignores it and always credits the ability controller.
pub(in crate::card::sets) static CHEERING_CROWD: CardRecord = CardRecord::new(
    "Cheering Crowd",
    "5fbce72f-e9a1-4d9f-b9b3-24dbafeef841",
    "Kim Sokol",
    CardRules::unsupported(),
);

// SPM 127 — Cosmic Spider-Man
pub(in crate::card::sets) static COSMIC_SPIDER_MAN: CardRecord = CardRecord::new(
    "Cosmic Spider-Man",
    "f82f4013-7308-4917-9042-19a5909f2134",
    "Zoltan Boros",
    CardRules::new_creature(
        mana_cost!("{W}{U}{B}{R}{G}"),
        &["Spider", "Human", "Hero"],
        5,
        5,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::first_strike(),
        abilities::trample(),
        abilities::lifelink(),
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of combat on your turn, other Spiders you \
             control gain flying, first strike, trample, lifelink, and \
             haste until end of turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SPM 128 — Doctor Octopus, Master Planner
// Audit: unsupported — Needs a maximum-hand-size setting operation with timestamp interactions against other settings and modifiers; the current player rules only remove the limit or add a numeric modifier.
pub(in crate::card::sets) static DOCTOR_OCTOPUS_MASTER_PLANNER: CardRecord = CardRecord::new(
    "Doctor Octopus, Master Planner",
    "76e1d361-18a1-4dec-a203-f83bf0014e02",
    "Xabi Gaztelua",
    CardRules::unsupported(),
);

// SPM 129 — Gallant Citizen
pub(in crate::card::sets) static GALLANT_CITIZEN: CardRecord = CardRecord::new(
    "Gallant Citizen",
    "43790471-6ec8-4c1d-b6d3-74c6cdf8ce43",
    "Allen Morris",
    CardRules::new_creature(mana_cost!("{G/W}{G/W}"), &["Human", "Citizen"], 1, 1).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        )],
    ),
);

// SPM 130 — Green Goblin, Revenant
// Audit: unsupported — Needs the controller's complete discarded-card count for the current turn, including prior resolving effects and costs; the discard continuation only exposes its own batch.
pub(in crate::card::sets) static GREEN_GOBLIN_REVENANT: CardRecord = CardRecord::new(
    "Green Goblin, Revenant",
    "218ef931-46f5-4a4d-9f26-898a1ff8f70f",
    "Chris Rahn",
    CardRules::unsupported(),
);

// SPM 131 — Jackal, Genius Geneticist
// Audit: unsupported — Needs a nonlegendary copy-process exception on the copied creature spell; CopyStackObject currently supports only a color override, while token-copy exceptions do not copy a spell.
pub(in crate::card::sets) static JACKAL_GENIUS_GENETICIST: CardRecord = CardRecord::new(
    "Jackal, Genius Geneticist",
    "c0ab07d6-b7c3-4129-9aef-cfdcfabec4b2",
    "Pavel Kolomeyets",
    CardRules::unsupported(),
);

// SPM 132 — Kraven, Proud Predator
pub(in crate::card::sets) static KRAVEN_PROUD_PREDATOR: CardRecord = CardRecord::new(
    "Kraven, Proud Predator",
    "af7c63e1-eccc-40a2-ba14-ce0d2a6123fc",
    "Alexander Mokhov",
    CardRules::new_creature(
        mana_cost!("{1}{R}{G}"),
        &["Human", "Warrior", "Villain"],
        0,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::vigilance(),
        AbilityDef::static_ability(
            "Top of the Food Chain — Kraven's power is equal to the \
             greatest mana value among permanents you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(ValueDef::AggregateObjectValues(
                    &ObjectValueAggregateDef {
                        objects: ObjectSetDef::Query(ObjectQueryDef::matching(
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
                        select: ObjectValueDef::ManaValue,
                        operation: AggregateOperationDef::Maximum,
                    },
                )),
            },
        ),
    ]),
);

// SPM 133 — Kraven the Hunter
// Audit: unsupported — Needs a death matcher comparing the dying creature's power with the maximum among its controller's creatures immediately before the event, including simultaneous departures.
pub(in crate::card::sets) static KRAVEN_THE_HUNTER: CardRecord = CardRecord::new(
    "Kraven the Hunter",
    "afdab464-3674-449b-be01-1cbd21fced23",
    "Greg Staples",
    CardRules::unsupported(),
);

// SPM 134 — Mary Jane Watson
pub(in crate::card::sets) static MARY_JANE_WATSON: CardRecord = CardRecord::new(
    "Mary Jane Watson",
    "178345f7-8ccd-4e47-80f4-5bd31bab6655",
    "Steve Argyle",
    CardRules::new_creature(mana_cost!("{1}{G/W}"), &["Human", "Performer"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a Spider you control enters, draw a card. This \
             ability triggers only once each turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .triggering_at_most(1)]),
);

// SPM 135 — Mister Negative
// Audit: unsupported — Needs an atomic exchange of player life totals and an actual life-loss result for the draw; two SetLifeTotal effects do not enforce the exchange's all-or-nothing rule or retain that result.
pub(in crate::card::sets) static MISTER_NEGATIVE: CardRecord = CardRecord::new(
    "Mister Negative",
    "2c9cb13d-55ff-4e26-aa49-755f8bcebc11",
    "Thanh Tuấn",
    CardRules::unsupported(),
);

// SPM 136 — Mob Lookout
pub(in crate::card::sets) static MOB_LOOKOUT: CardRecord = CardRecord::new(
    "Mob Lookout",
    "e0e3b660-d391-454b-ba57-bff4ddcf27b7",
    "David Palumbo",
    CardRules::new_creature(mana_cost!("{1}{U/B}"), &["Human", "Rogue", "Villain"], 0, 3)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "When this creature enters, target creature you control \
             connives. (Draw a card, then discard a card. If you discarded \
             a nonland card, put a +1/+1 counter on that creature.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("connived")),
                        effect: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                crate::Binding!("connived"),
                            )),
                        },
                    }),
                },
            ]),
        )]),
);

// SPM 137 — Morbius the Living Vampire
pub(in crate::card::sets) static MORBIUS_THE_LIVING_VAMPIRE: CardRecord = CardRecord::new(
    "Morbius the Living Vampire",
    "b978d0e2-f5a7-4ade-befc-11b406e84477",
    "Borja Pindado",
    CardRules::new_creature(
        mana_cost!("{2}{U}{B}"),
        &["Vampire", "Scientist", "Villain"],
        3,
        1,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::lifelink(),
        AbilityDef::activated(
            "{U}{B}, Exile this card from your graveyard: Look at the top \
             three cards of your library. Put one of them into your hand \
             and the rest on the bottom of your library in any order.",
            &[CostDef::Mana(mana_cost!("{U}{B}")), CostDef::ExileSource],
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
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
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        actor: PlayerRefDef::EffectController,
                        ordered: crate::Binding!("ordered"),
                        placement: ZonePlacement::Bottom,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "ordered"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// SPM 138 — Prowler, Clawed Thief
pub(in crate::card::sets) static PROWLER_CLAWED_THIEF: CardRecord = CardRecord::new(
    "Prowler, Clawed Thief",
    "bd31953a-7259-44e3-a94f-013bda68006d",
    "Anthony Devine",
    CardRules::new_creature(
        mana_cost!("{1}{U}{B}"),
        &["Human", "Rogue", "Villain"],
        2,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever another Villain you control enters, Prowler \
             connives. (Draw a card, then discard a card. If you discarded \
             a nonland card, put a +1/+1 counter on this creature.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        bound: Some(crate::Binding!("connived")),
                        effect: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Source,
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                crate::Binding!("connived"),
                            )),
                        },
                    }),
                },
            ]),
        ),
    ]),
);

// SPM 139 — Pumpkin Bombardment
pub(in crate::card::sets) static PUMPKIN_BOMBARDMENT: CardRecord = CardRecord::new(
    "Pumpkin Bombardment",
    "a268ad73-9a1f-47d9-9a85-a4669a769c3d",
    "Leon Tukker",
    CardRules::new_sorcery(mana_cost!("{B/R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Pumpkin Bombardment deals 3 damage to target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(3),
        ),
    )
    .with_spell_additional_cost(&CostDef::Choice(&[
        CostDef::discard(ObjectPredicateDef::Any),
        CostDef::Mana(mana_cost!("{2}")),
    ]))]),
);

// SPM 140 — Rhino, Barreling Brute
pub(in crate::card::sets) static RHINO_BARRELING_BRUTE: CardRecord = CardRecord::new(
    "Rhino, Barreling Brute",
    "4b8ac400-1e98-49fc-94be-00386d15f2ae",
    "Filipe Pagliuso",
    CardRules::new_creature(mana_cost!("{3}{R}{R}{G}{G}"), &["Human", "Villain"], 6, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::trample(),
            abilities::haste(),
            AbilityDef::triggered_if(
                "Whenever Rhino attacks, if you've cast a spell with mana \
                 value 4 or greater this turn, draw a card.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::You,
                        spell: ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                    }),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]),
);

// SPM 141 — Rhino's Rampage
// Audit: unsupported — Needs excess fight damage to open a reflexive targeted trigger whose artifact target is chosen after the fight; the current fight excess continuation resolves directly rather than creating that independent trigger.
pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    "Rhino's Rampage",
    "f668817c-1cab-44c5-b6a8-95113e480d5e",
    "Nino Is",
    CardRules::unsupported(),
);

// SPM 142 — Scarlet Spider, Ben Reilly
// Audit: unsupported — Needs prospective-entry evaluation of the returned web-slinging cost object's mana value; entry value evaluation cannot project a characteristic from the alternative-cost object receipt.
pub(in crate::card::sets) static SCARLET_SPIDER_BEN_REILLY: CardRecord = CardRecord::new(
    "Scarlet Spider, Ben Reilly",
    "ee771581-f867-48d7-9ddb-897a1ffcdf0a",
    "Javier Charro",
    CardRules::unsupported(),
);

// SPM 143 — Scarlet Spider, Kaine
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static SCARLET_SPIDER_KAINE: CardRecord = CardRecord::new(
    "Scarlet Spider, Kaine",
    "2cb00060-8cc5-42dc-bcbf-affd9e59f8fd",
    "Forrest Imel",
    CardRules::unsupported(),
);

// SPM 144 — Shriek, Treblemaker
// Audit: unsupported — Needs a reflexive trigger after optional discard that remains valid if Shriek leaves before its first-main-phase trigger resolves; current optional-effect listeners require a live battlefield source.
pub(in crate::card::sets) static SHRIEK_TREBLEMAKER: CardRecord = CardRecord::new(
    "Shriek, Treblemaker",
    "01f1900e-b10f-47dd-8b3d-6913fa661186",
    "Borja Pindado",
    CardRules::unsupported(),
);

// SPM 145 — Silk, Web Weaver
pub(in crate::card::sets) static SILK_WEB_WEAVER: CardRecord = CardRecord::new(
    "Silk, Web Weaver",
    "588dc8d9-6ce0-4bd7-afbd-84bb251fdcb1",
    "Carissa Susilo",
    CardRules::new_creature(mana_cost!("{2}{G}{W}"), &["Spider", "Human", "Hero"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            web_slinging(
                "Web-slinging {1}{G}{W} (You may cast this spell for {1}{G}{W} \
                 if you also return a tapped creature you control to its \
                 owner's hand.)",
                &[
                    CostDef::Mana(mana_cost!("{1}{G}{W}")),
                    CostDef::return_to_hand(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Tapped,
                        ]),
                        CostQuantityDef::Fixed(1),
                    ),
                ],
            ),
            AbilityDef::triggered(
                "Whenever you cast a creature spell, create a 1/1 green and \
                 white Human Citizen creature token.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::create_creature_token(
                    &["Human", "Citizen"],
                    &[ManaColor::Green, ManaColor::White],
                    1,
                    1,
                ),
            ),
            AbilityDef::activated(
                "{3}{G}{W}: Creatures you control get +2/+2 and gain vigilance \
                 until end of turn.",
                &[CostDef::Mana(mana_cost!("{3}{G}{W}"))],
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
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// SPM 146 — Skyward Spider
// Audit: unsupported — Needs a complete modified predicate, including Equipment attached to the creature and Auras controlled by its controller; the available counter and enchanted predicates cannot identify all three modifications with their required controller scope.
pub(in crate::card::sets) static SKYWARD_SPIDER: CardRecord = CardRecord::new(
    "Skyward Spider",
    "f5cbb580-cd02-4c60-acb7-b7ed1f1fce59",
    "Bachzim",
    CardRules::unsupported(),
);

// SPM 147 — SP//dr, Piloted by Peni
// Audit: unsupported — Needs a complete modified predicate, including Equipment attached to the creature and Auras controlled by its controller; the available counter and enchanted predicates cannot identify all three modifications with their required controller scope.
pub(in crate::card::sets) static SP_DR_PILOTED_BY_PENI: CardRecord = CardRecord::new(
    "SP//dr, Piloted by Peni",
    "c47c1d83-e76d-4939-9ed6-05a9e709dea1",
    "Toni Infante",
    CardRules::unsupported(),
);

// SPM 148 — Spider Manifestation
pub(in crate::card::sets) static SPIDER_MANIFESTATION: CardRecord = CardRecord::new(
    "Spider Manifestation",
    "99223677-b8a5-48f1-8009-e8475eada7db",
    "Helge C. Balzer",
    CardRules::new_creature(mana_cost!("{1}{R/G}"), &["Spider", "Avatar"], 2, 2).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::triggered(
            "Whenever you cast a spell with mana value 4 or greater, untap \
             this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(3)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// SPM 149 — Spider-Girl, Legacy Hero
pub(in crate::card::sets) static SPIDER_GIRL_LEGACY_HERO: CardRecord = CardRecord::new(
    "Spider-Girl, Legacy Hero",
    "d1f3196a-fe48-446f-ab07-00c66b7816c8",
    "Lixin Yin",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Spider", "Human", "Hero"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "During your turn, Spider-Girl has flying.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                },
            ),
            AbilityDef::triggered(
                "When Spider-Girl leaves the battlefield, create a 1/1 green \
                 and white Human Citizen creature token.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::create_creature_token(
                    &["Human", "Citizen"],
                    &[ManaColor::Green, ManaColor::White],
                    1,
                    1,
                ),
            ),
        ]),
);

// SPM 150 — Spider-Man 2099
// Audit: unsupported — Needs this-turn history of lands played and spells cast from outside the hand; spell-source history alone misses qualifying land plays, and enters history also counts non-play arrivals.
pub(in crate::card::sets) static SPIDER_MAN_2099: CardRecord = CardRecord::new(
    "Spider-Man 2099",
    "2a72c7e7-34f5-4cb0-9959-35516e398e49",
    "Toni Infante",
    CardRules::unsupported(),
);

// SPM 151 — Spider-Man India
pub(in crate::card::sets) static SPIDER_MAN_INDIA: CardRecord = CardRecord::new(
    "Spider-Man India",
    "65b8af30-559b-43a9-8526-62c28c378339",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{3}{G}{W}"), &["Spider", "Human", "Hero"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            web_slinging(
                "Web-slinging {1}{G}{W} (You may cast this spell for {1}{G}{W} \
                 if you also return a tapped creature you control to its \
                 owner's hand.)",
                &[
                    CostDef::Mana(mana_cost!("{1}{G}{W}")),
                    CostDef::return_to_hand(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Tapped,
                        ]),
                        CostQuantityDef::Fixed(1),
                    ),
                ],
            ),
            AbilityDef::triggered_with_targets(
                "Pavitr's Sevā — Whenever you cast a creature spell, put a \
                 +1/+1 counter on target creature you control. It gains flying \
                 until end of turn.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
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
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// SPM 152 — Spider-Woman, Stunning Savior
pub(in crate::card::sets) static SPIDER_WOMAN_STUNNING_SAVIOR: CardRecord = CardRecord::new(
    "Spider-Woman, Stunning Savior",
    "bc9b2a76-3cce-4fd0-a4ef-932747cb11b2",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{1}{W/U}"), &["Spider", "Human", "Hero"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::replacement_for(
                "Venom Blast — Artifacts and creatures your opponents control \
                 enter tapped.",
                ReplacementEventDef::ObjectEntersBattlefield {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    controller: PlayerRelation::Opponent,
                    cast: None,
                },
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
            ),
        ]),
);

// SPM 153 — The Spot, Living Portal
pub(in crate::card::sets) static THE_SPOT_LIVING_PORTAL: CardRecord = CardRecord::new(
    "The Spot, Living Portal",
    "09081740-1180-48ea-b50b-e016d9c3828a",
    "Bastien Grivet",
    CardRules::new_creature(
        mana_cost!("{3}{W}{B}"),
        &["Human", "Scientist", "Villain"],
        4,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When The Spot enters, exile up to one target nonland \
             permanent and up to one target nonland permanent card from a \
             graveyard.",
            &[
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                ),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Any),
                    },
                    1,
                ),
            ],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::objects(ObjectSetDef::Union(&[
                    ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                    ObjectSetDef::LegalTargets(TargetIndex(1)),
                ])),
                face_down: false,
                until_source_leaves: false,
                then: None,
            },
        ),
        abilities::dies_trigger(
            "When The Spot dies, put him on the bottom of his owner's \
             library. If you do, return the exiled cards to their owners' \
             hands.",
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::TriggeringZoneChangeResult,
                    ZoneKind::Library,
                    ZonePlacement::Bottom,
                ),
                binding: crate::Binding!("returned_spot"),
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CountObjects(&ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                            crate::Binding!("returned_spot"),
                        )),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::ReturnLinkedExiles {
                        object: ObjectPredicateDef::Any,
                        counters: None,
                        zone: ZoneKind::Hand,
                        grant: None,
                        controller: None,
                        transformed: false,
                    },
                },
            },
        ),
    ]),
);

// SPM 154 — Sun-Spider, Nimble Webber
pub(in crate::card::sets) static SUN_SPIDER_NIMBLE_WEBBER: CardRecord = CardRecord::new(
    "Sun-Spider, Nimble Webber",
    "2b54d4a5-634f-4ae3-b592-0dc527f60d56",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{3}{W/U}"), &["Spider", "Human", "Hero"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "During your turn, Sun-Spider has flying.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                },
            ),
            abilities::enters_trigger(
                "When Sun-Spider enters, search your library for an Aura or \
                 Equipment card, reveal it, put it into your hand, then \
                 shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
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
        ]),
);

// SPM 155 — Superior Spider-Man
// Audit: unsupported — Needs a reflexive trigger carrying the graveyard card chosen during copy entry so that exact card is exiled after entry; copy-entry exceptions do not expose that selected card to a later trigger.
pub(in crate::card::sets) static SUPERIOR_SPIDER_MAN: CardRecord = CardRecord::new(
    "Superior Spider-Man",
    "ad4adc3e-ec41-4406-8ff2-59ba8067cf4e",
    "Carlos Dattoli",
    CardRules::unsupported(),
);

// SPM 156 — Symbiote Spider-Man
// Audit: unsupported — Needs a grant of this graveyard card's actual other abilities, excluding the activated Find New Host ability, with current text and granted abilities preserved; the existing grant vocabulary takes explicit ability definitions or activated abilities from linked exile, not this source-card projection.
pub(in crate::card::sets) static SYMBIOTE_SPIDER_MAN: CardRecord = CardRecord::new(
    "Symbiote Spider-Man",
    "6a21c0ff-b51a-4946-9737-7872a7eef97b",
    "Paolo Rivera",
    CardRules::unsupported(),
);

// SPM 157 — Ultimate Green Goblin
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static ULTIMATE_GREEN_GOBLIN: CardRecord = CardRecord::new(
    "Ultimate Green Goblin",
    "e82d3f71-8404-40e7-b7fa-35713d1b384e",
    "Jesper Ejsing",
    CardRules::unsupported(),
);

// SPM 158 — Vulture, Scheming Scavenger
pub(in crate::card::sets) static VULTURE_SCHEMING_SCAVENGER: CardRecord = CardRecord::new(
    "Vulture, Scheming Scavenger",
    "e29281be-e722-4149-93a0-6dd3f0f64253",
    "Kevin Sidharta",
    CardRules::new_creature(
        mana_cost!("{5}{U/B}"),
        &["Human", "Artificer", "Villain"],
        4,
        6,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever Vulture attacks, other Villains you control gain \
             flying until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Villain")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SPM 159 — Web-Warriors
pub(in crate::card::sets) static WEB_WARRIORS: CardRecord = CardRecord::new(
    "Web-Warriors",
    "741f5373-b51a-421a-9a74-326f0575c99b",
    "Thanh Tuấn",
    CardRules::new_creature(mana_cost!("{4}{G/W}"), &["Spider", "Hero"], 4, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, put a +1/+1 counter on each other \
             creature you control.",
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

// SPM 160 — Wraith, Vicious Vigilante
pub(in crate::card::sets) static WRAITH_VICIOUS_VIGILANTE: CardRecord = CardRecord::new(
    "Wraith, Vicious Vigilante",
    "5f46ed93-de6d-4180-9018-26ee07f75464",
    "Nereida",
    CardRules::new_creature(
        mana_cost!("{1}{W}{U}"),
        &["Human", "Detective", "Hero"],
        1,
        1,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::double_strike(),
        AbilityDef::static_ability(
            "Fear Gas — Wraith can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ]),
);

// SPM 161 — Bagel and Schmear
pub(in crate::card::sets) static BAGEL_AND_SCHMEAR: CardRecord = CardRecord::new(
    "Bagel and Schmear",
    "7f927f72-fc9b-444f-9e0e-78a5e8e7bcaa",
    "Javier Charro",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Food"])
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "Share — {W}, {T}, Sacrifice this artifact: Put a +1/+1 \
                 counter on up to one target creature. Draw a card. Activate \
                 only as a sorcery.",
                &[
                    CostDef::Mana(mana_cost!("{W}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
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
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
            AbilityDef::activated(
                "Nosh — {2}, {T}, Sacrifice this artifact: You gain 3 life and \
                 draw a card.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                    CostDef::SacrificeSource,
                ],
                EffectDef::Sequence(&[
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            ),
        ]),
);

// SPM 162 — Doc Ock's Tentacles
pub(in crate::card::sets) static DOC_OCK_S_TENTACLES: CardRecord = CardRecord::new(
    "Doc Ock's Tentacles",
    "fed9547c-9d0d-4e62-9639-887ed09231a2",
    "David Álvarez",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever a creature you control with mana value 5 or greater \
                 enters, you may attach this Equipment to it.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(4)),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Attach {
                        object: EffectRecipientDef::TriggeringObject,
                    },
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +4/+4.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{5}"))], "Equip {5}"),
        ]),
);

// SPM 163 — Eerie Gravestone
pub(in crate::card::sets) static EERIE_GRAVESTONE: CardRecord = CardRecord::new(
    "Eerie Gravestone",
    "7675e91f-dba7-4e64-a7ff-1dd56665a4cc",
    "Lordigan",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        AbilityDef::activated(
            "{1}{B}, Sacrifice this artifact: Mill four cards. You may put \
             a creature card from among them into your hand. (To mill four \
             cards, put the top four cards of your library into your \
             graveyard.)",
            &[
                CostDef::Mana(mana_cost!("{1}{B}")),
                CostDef::SacrificeSource,
            ],
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
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                            CardType::Creature,
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
            ]),
        ),
    ]),
);

// SPM 164 — Hot Dog Cart
pub(in crate::card::sets) static HOT_DOG_CART: CardRecord = CardRecord::new(
    "Hot Dog Cart",
    "6ee3b883-e9f5-426f-a2ea-96fe9ff3aba9",
    "David Álvarez",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, create a Food token. (It's an \
             artifact with \"{2}, {T}, Sacrifice this token: You gain 3 \
             life.\")",
            EffectDef::create_token(tokens::food()).with_count(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// SPM 165 — Interdimensional Web Watch
// Audit: unsupported — Needs a mana-spending restriction that tests the zone a spell is cast from; CastSpell accepts characteristic predicates, which cannot inspect cast origin.
pub(in crate::card::sets) static INTERDIMENSIONAL_WEB_WATCH: CardRecord = CardRecord::new(
    "Interdimensional Web Watch",
    "87a8e112-e72f-413f-88a3-e7ce72c2ec53",
    "Toni Infante",
    CardRules::unsupported(),
);

// SPM 166 — Iron Spider, Stark Upgrade
// Audit: unsupported — Needs payment by removing a total of two counters distributed among multiple artifacts; existing activation costs remove counters only from a specified source or one chosen object.
pub(in crate::card::sets) static IRON_SPIDER_STARK_UPGRADE: CardRecord = CardRecord::new(
    "Iron Spider, Stark Upgrade",
    "8da5f34e-7f40-406a-88d2-bb1e3ed25200",
    "Kevin Glint",
    CardRules::unsupported(),
);

// SPM 167 — Living Brain, Mechanical Marvel
pub(in crate::card::sets) static LIVING_BRAIN_MECHANICAL_MARVEL: CardRecord = CardRecord::new(
    "Living Brain, Mechanical Marvel",
    "26833b64-2e6d-4977-9a6e-6fe73c54d671",
    "Nathaniel Himawan",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Robot", "Villain"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, target non-Equipment \
             artifact you control becomes an artifact creature with base \
             power and toughness 3/3 until end of turn. Untap it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Equipment",
                        ))),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
                        ),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        )]),
);

// SPM 168 — Mechanical Mobster
pub(in crate::card::sets) static MECHANICAL_MOBSTER: CardRecord = CardRecord::new(
    "Mechanical Mobster",
    "6c6d9ecc-2dd1-471a-8678-a2461b1084fa",
    "David Szabo",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Human", "Robot", "Villain"], 2, 1)
        .with_abilities(&[abilities::enters_trigger_with_targets(
            "When this creature enters, exile up to one target card from a \
             graveyard. Target creature you control connives. (Draw a \
             card, then discard a card. If you discarded a nonland card, \
             put a +1/+1 counter on that creature.)",
            &[
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Any),
                    },
                    1,
                ),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
            ],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                CardType::Land,
                            )),
                            bound: Some(crate::Binding!("connived")),
                            effect: &EffectDef::AddCounters {
                                object: EffectRecipientDef::Target(TargetIndex(1)),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::CountObjects(&ObjectSetDef::Binding(
                                    crate::Binding!("connived"),
                                )),
                            },
                        }),
                    },
                ]),
            ]),
        )]),
);

// SPM 169 — News Helicopter
pub(in crate::card::sets) static NEWS_HELICOPTER: CardRecord = CardRecord::new(
    "News Helicopter",
    "15717af0-30cd-4417-947a-c27cca06d93a",
    "Lee Woo-chul",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 green and white Human \
             Citizen creature token.",
            EffectDef::create_creature_token(
                &["Human", "Citizen"],
                &[ManaColor::Green, ManaColor::White],
                1,
                1,
            ),
        ),
    ]),
);

// SPM 170 — Passenger Ferry
// Audit: unsupported — Needs a reflexive trigger created after the optional mana payment and retained if the Vehicle has left; current optional-effect listeners require the original source on the battlefield.
pub(in crate::card::sets) static PASSENGER_FERRY: CardRecord = CardRecord::new(
    "Passenger Ferry",
    "2495f477-b88c-4938-a86a-f72c3c861188",
    "Leon Tukker",
    CardRules::unsupported(),
);

// SPM 171 — Peter Parker's Camera
pub(in crate::card::sets) static PETER_PARKER_S_CAMERA: CardRecord = CardRecord::new(
    "Peter Parker's Camera",
    "47875dff-c046-4cb0-b1e3-f926cbe25b59",
    "Lixin Yin",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters with three film counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("film"),
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}, Remove a film counter from this artifact: Copy \
             target activated or triggered ability you control. You may \
             choose new targets for the copy.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("film"),
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::ActivatedAbility,
                        ObjectPredicateDef::TriggeredAbility,
                    ]),
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
        ),
    ]),
);

// SPM 172 — Rocket-Powered Goblin Glider
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static ROCKET_POWERED_GOBLIN_GLIDER: CardRecord = CardRecord::new(
    "Rocket-Powered Goblin Glider",
    "c6c39232-72cc-4363-83d0-b5873f14f231",
    "Pavel Kolomeyets",
    CardRules::unsupported(),
);

// SPM 173 — Spider-Bot
pub(in crate::card::sets) static SPIDER_BOT: CardRecord = CardRecord::new(
    "Spider-Bot",
    "24df824a-c1d6-4f09-b866-313b31fec5fb",
    "Carlos Dattoli",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Spider", "Robot", "Scout"], 2, 1)
        .with_abilities(&[
            abilities::reach(),
            abilities::enters_trigger(
                "When this creature enters, you may search your library for a \
                 basic land card, reveal it, then shuffle and put that card on \
                 top.",
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
                },
            ),
        ]),
);

// SPM 174 — Spider-Mobile
// Audit: unsupported — Needs a blocks trigger that fires once for the blocking declaration even when this Vehicle can block multiple attackers; the current Blocks matcher fires separately for each blocked creature.
pub(in crate::card::sets) static SPIDER_MOBILE: CardRecord = CardRecord::new(
    "Spider-Mobile",
    "f12664c0-d7cd-4acb-87db-cfa3c85f32a9",
    "Bastien Grivet",
    CardRules::unsupported(),
);

// SPM 175 — Spider-Slayer, Hatred Honed
pub(in crate::card::sets) static SPIDER_SLAYER_HATRED_HONED: CardRecord = CardRecord::new(
    "Spider-Slayer, Hatred Honed",
    "ac37ca6f-a6ee-4dfb-949f-2562f98d09d0",
    "David Álvarez",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Human", "Villain"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever Spider-Slayer deals damage to a Spider, destroy that \
                 creature.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    kind: DamageKindDef::Any,
                    source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::Source),
                    recipient: DamageRecipientMatcherDef::MatchingObject(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                    ),
                }),
                EffectDef::Destroy {
                    object: EffectRecipientDef::DamagedObject,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "{6}, Exile this card from your graveyard: Create two tapped \
                 1/1 colorless Robot artifact creature tokens with flying.",
                &[CostDef::Mana(mana_cost!("{6}")), CostDef::ExileSource],
                EffectDef::create_artifact_creature_token(&["Robot"], &[], 1, 1)
                    .with_count(ValueDef::Constant(2))
                    .with_abilities(&[abilities::flying()])
                    .entering_tapped(),
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// SPM 176 — Spider-Suit
pub(in crate::card::sets) static SPIDER_SUIT: CardRecord = CardRecord::new(
    "Spider-Suit",
    "436527ec-5af4-4b6d-a5a0-d21fc466a625",
    "Alex Horley-Orlandelli",
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2 and is a Spider Hero in addition \
                 to its other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Spider", "Hero",
                        ])),
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

// SPM 177 — Steel Wrecking Ball
pub(in crate::card::sets) static STEEL_WRECKING_BALL: CardRecord = CardRecord::new(
    "Steel Wrecking Ball",
    "7f2c74f5-1cfe-4918-a86b-0d58ac8b7469",
    "Michele Giorgi",
    CardRules::new_artifact(mana_cost!("{5}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, it deals 5 damage to target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}, Discard this card: Destroy target artifact.",
            &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::DiscardSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// SPM 178 — Subway Train
pub(in crate::card::sets) static SUBWAY_TRAIN: CardRecord = CardRecord::new(
    "Subway Train",
    "96f869f7-db88-4580-82b7-8749a55e525c",
    "Jonas De Ro",
    CardRules::new_vehicle(mana_cost!("{2}"), 3, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, you may pay {G}. If you do, search \
             your library for a basic land card, reveal it, put it into \
             your hand, then shuffle.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{G}"))],
                &EffectDef::SearchZone {
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
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            )),
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// SPM 179 — Daily Bugle Building
pub(in crate::card::sets) static DAILY_BUGLE_BUILDING: CardRecord = CardRecord::new(
    "Daily Bugle Building",
    "669bbcb1-0981-40e7-905e-b94e74bc4861",
    "David Álvarez",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated_with_targets(
            "Smear Campaign — {1}, {T}: Target legendary creature gains \
             menace until end of turn. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::menace()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// SPM 180 — Multiversal Passage
pub(in crate::card::sets) static MULTIVERSAL_PASSAGE: CardRecord = CardRecord::new(
    "Multiversal Passage",
    "f5fb426a-5618-4dd4-9c51-0cc847be8c1d",
    "Pablo Mendoza",
    // A shock land that is whichever basic type the hand actually wants,
    // which is a different card in a deck with two colours and in one with
    // five. The mana ability comes from the type rather than a printed
    // clause, so choosing is all there is to it.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::replacement(
            "As this land enters, choose a basic land type. Then you may pay 2 life. If you \
             don't, it enters tapped.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::BASIC_LAND_TYPE,
                )),
                ReplacementEffectDef::PayOr {
                    payment: EffectPaymentDef::new(
                        PlayerSetDef::Related(PlayerRelation::You),
                        &[CostDef::PayLife(2)],
                    ),
                    if_paid: &[],
                    // Declining is what makes it a tapped land, so the branch that pays does
                    // nothing at all and the branch that does not is the whole cost.
                    if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::Tapped,
                    )],
                },
            ]),
        ),
        AbilityDef::static_ability(
            "This land is the chosen type.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_chosen_basic_land_type(),
            },
        ),
    ]),
);

/// The OM1 cycle of tapped duals that surveil late: lands that differ only
/// in which two colours they make, so the clauses are written once here.
/// Entering tapped is the price of the two colours, and the surveil is what
/// a flooded late game does with the land instead of drawing it.
///
/// `colors` is a promoted literal at each call site, and the abilities are
/// added one at a time in printed order: an array holding the parameterized
/// mana ability could not be given a `'static` lifetime.
///
/// Spectacle Summit prints the same shape but is not in this cycle -- its
/// surveil costs {2}{U}{R} rather than {4}.
const fn surveilling_dual_land(mana_text: &'static str, colors: &'static [ManaColor]) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(abilities::enters_tapped(CardType::Land))
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated(
            "{4}, {T}: Surveil 1. (Look at the top card of your library. \
             You may put it into your graveyard.)",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ))
}

// SPM 181 — Ominous Asylum
pub(in crate::card::sets) static OMINOUS_ASYLUM: CardRecord = CardRecord::new(
    "Ominous Asylum",
    "4329f94a-9110-4f07-b4a6-f1ccae97ccc9",
    "Pavel Kolomeyets",
    surveilling_dual_land("{T}: Add {B} or {R}.", &[ManaColor::Black, ManaColor::Red]),
);

// SPM 182 — Oscorp Industries
// Audit: unsupported — Needs per-card discard provenance for the current turn and a graveyard casting or land-play permission conditional on that exact discard; the current graveyard permissions and turn histories cannot express mayhem.
pub(in crate::card::sets) static OSCORP_INDUSTRIES: CardRecord = CardRecord::new(
    "Oscorp Industries",
    "1e609d6e-9e37-45d2-87de-8c76675f7cec",
    "Bastien Grivet",
    CardRules::unsupported(),
);

// SPM 183 — Savage Mansion
pub(in crate::card::sets) static SAVAGE_MANSION: CardRecord = CardRecord::new(
    "Savage Mansion",
    "855f59a5-17a8-4aca-8a4d-f98111eba14c",
    "David Álvarez",
    surveilling_dual_land("{T}: Add {R} or {G}.", &[ManaColor::Red, ManaColor::Green]),
);

// SPM 184 — Sinister Hideout
pub(in crate::card::sets) static SINISTER_HIDEOUT: CardRecord = CardRecord::new(
    "Sinister Hideout",
    "23190d7e-5165-49bd-b307-bf81877d228d",
    "Pavel Kolomeyets",
    surveilling_dual_land("{T}: Add {U} or {B}.", &[ManaColor::Blue, ManaColor::Black]),
);

// SPM 185 — Suburban Sanctuary
pub(in crate::card::sets) static SUBURBAN_SANCTUARY: CardRecord = CardRecord::new(
    "Suburban Sanctuary",
    "467df77a-a99c-4cfd-9af4-502eaa2eb2e3",
    "David Frasheski",
    surveilling_dual_land(
        "{T}: Add {G} or {W}.",
        &[ManaColor::Green, ManaColor::White],
    ),
);

// SPM 186 — University Campus
pub(in crate::card::sets) static UNIVERSITY_CAMPUS: CardRecord = CardRecord::new(
    "University Campus",
    "2752f21c-f535-4772-a8b3-e97e1339e9c9",
    "David Álvarez",
    // A Campus that surveils rather than scries, so it does not share the
    // Strixhaven cycle's clause even though the rest of the card matches.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{4}, {T}: Surveil 1.",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// SPM 187 — Urban Retreat
// Audit: unsupported — Needs a hand-zone activation cost selecting and returning a tapped controlled creature; current hand activation payment support does not implement ReturnToHand object costs.
pub(in crate::card::sets) static URBAN_RETREAT: CardRecord = CardRecord::new(
    "Urban Retreat",
    "2581f320-8238-413d-ab04-d5535da55630",
    "Jonas De Ro",
    CardRules::unsupported(),
);

// SPM 188 — Vibrant Cityscape
pub(in crate::card::sets) static VIBRANT_CITYSCAPE: CardRecord = CardRecord::new(
    "Vibrant Cityscape",
    "9c110fa1-2320-4652-b282-ed064a9ec9a9",
    "Wei Guan",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic \
         land card, put it onto the battlefield tapped, then shuffle.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
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

// SPM 189 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "1164f7ec-7b2f-4cc9-90bb-7eaaa331b4cd",
    "Sarah Finnigan",
);

// SPM 190 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "d59cb0b5-fd4f-4dde-a69f-7ca6aa12b89f",
    "Sarah Finnigan",
);

// SPM 191 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "5cb03b18-d74c-4c89-9539-3549d2e8ff5f",
    "Sarah Finnigan",
);

// SPM 192 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "b044630d-50e7-431b-8e91-bd53e967f594",
    "Sarah Finnigan",
);

// SPM 193 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "7b6c2532-be5a-4f1f-893c-36bcda2a699d",
    "Sarah Finnigan",
);

// SPM 194 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "94e88862-d53d-49b6-8aa5-95f07507c6e1",
    "Jonas De Ro",
);

// SPM 195 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "14fcdc57-12e2-429b-8916-4df752e462d4",
    "Jonas De Ro",
);

// SPM 196 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "a7150a6e-240f-4628-acdc-153d404370ff",
    "Jonas De Ro",
);

// SPM 197 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "fc5004db-8db3-4506-bb01-f41be7968824",
    "Jonas De Ro",
);

// SPM 198 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "c4fec728-8e3f-427d-81ab-e06114910223",
    "Jonas De Ro",
);

// SPM 199 — SP//dr, Piloted by Peni (alternate printing)
const SP_DR_PILOTED_BY_PENI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SP_DR_PILOTED_BY_PENI,
    1,
    "e8c1aace-e8fe-45ee-bc40-225409ff8657",
    "Jim Cheung & Jay David Ramos",
);

// SPM 200 — Miles Morales // Ultimate Spider-Man (alternate printing)
const MILES_MORALES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MILES_MORALES,
    1,
    "b6c129a7-59d3-499e-8279-f374266150be",
    "Jim Cheung & Jay David Ramos",
);

// SPM 201 — Spider-Ham, Peter Porker (alternate printing)
const SPIDER_HAM_PETER_PORKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_HAM_PETER_PORKER,
    1,
    "020426f0-ca38-4b0e-aa20-6fe12dc34853",
    "Jim Cheung & Jay David Ramos",
);

// SPM 202 — Gwen Stacy // Ghost-Spider (alternate printing)
const GWEN_STACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GWEN_STACY,
    1,
    "9126dfa0-b681-44c7-9ae1-7926df8a2767",
    "Jim Cheung & Jay David Ramos",
);

// SPM 203 — Web-Warriors (alternate printing)
const WEB_WARRIORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEB_WARRIORS,
    1,
    "bd672045-fe36-4a03-abf7-9c81f4100bba",
    "Jim Cheung & Jay David Ramos",
);

// SPM 204 — Spider-Man Noir (alternate printing)
const SPIDER_MAN_NOIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_NOIR,
    1,
    "d9a8b9a4-c9d9-4e9f-80b0-23de71a7423b",
    "Jim Cheung & Jay David Ramos",
);

// SPM 205 — Spider-Man 2099 (alternate printing)
const SPIDER_MAN_2099_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_2099,
    1,
    "b680a089-5cf0-494a-98e8-23d28315f572",
    "Jim Cheung & Jay David Ramos",
);

// SPM 206 — Multiversal Passage (alternate printing)
const MULTIVERSAL_PASSAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MULTIVERSAL_PASSAGE,
    1,
    "db88b6b6-7907-4c40-b712-4fa1456d8ad0",
    "Jim Cheung & Jay David Ramos",
);

// SPM 207 — Spider-Punk (alternate printing)
const SPIDER_PUNK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_PUNK,
    1,
    "8807635d-b5f2-4f08-af5e-5792867eb60b",
    "Jim Cheung & Jay David Ramos",
);

// SPM 208 — Peter Parker // Amazing Spider-Man (alternate printing)
const PETER_PARKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PETER_PARKER,
    1,
    "97fcf5a5-54e1-43f3-95e3-edc6215bf973",
    "Lucas Werneck",
);

// SPM 209 — Gwen Stacy // Ghost-Spider (alternate printing)
const GWEN_STACY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GWEN_STACY,
    2,
    "4ab0cb80-4330-408b-bc03-ef96dd34177e",
    "Roberta Ingranata",
);

// SPM 210 — Spider-Punk (alternate printing)
const SPIDER_PUNK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_PUNK,
    2,
    "47cebb95-3a25-4e9e-ad9c-034e0cd5bfd1",
    "Chris Bachalo",
);

// SPM 211 — Miles Morales // Ultimate Spider-Man (alternate printing)
const MILES_MORALES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MILES_MORALES,
    2,
    "b73db1c4-64d1-4d09-852a-7d0d299e5529",
    "Ivan Shavrin",
);

// SPM 212 — Radioactive Spider (alternate printing)
const RADIOACTIVE_SPIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RADIOACTIVE_SPIDER,
    1,
    "a0efb06d-7b91-4d64-98b2-244d56a44ecb",
    "Tyler Walpole",
);

// SPM 213 — Araña, Heart of the Spider (alternate printing)
const ARANA_HEART_OF_THE_SPIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARANA_HEART_OF_THE_SPIDER,
    1,
    "95da1818-7589-44d8-8bba-16ba3ef7b24e",
    "Logan Lubera",
);

// SPM 214 — Scarlet Spider, Ben Reilly (alternate printing)
const SCARLET_SPIDER_BEN_REILLY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCARLET_SPIDER_BEN_REILLY,
    1,
    "6d477a72-d4ad-4230-aee8-2bd78e448bb5",
    "Logan Lubera",
);

// SPM 215 — Silk, Web Weaver (alternate printing)
const SILK_WEB_WEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILK_WEB_WEAVER,
    1,
    "f6ff59e4-c7b7-4fdc-a49e-2d7aab859474",
    "Veronica Fish",
);

// SPM 216 — Spider-Man 2099 (alternate printing)
const SPIDER_MAN_2099_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_2099,
    2,
    "3b4e11a7-b48b-4fd8-9395-7a9bc655439a",
    "Jim Cheung & Jay David Ramos",
);

// SPM 217 — Symbiote Spider-Man (alternate printing)
const SYMBIOTE_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYMBIOTE_SPIDER_MAN,
    1,
    "89e046ae-f27a-484f-986c-61c86e754c92",
    "Tyler Walpole",
);

// SPM 218 — Origin of Spider-Man (alternate printing)
const ORIGIN_OF_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORIGIN_OF_SPIDER_MAN,
    1,
    "634e0542-066b-4fb5-a130-fa94a019f712",
    "Jim Cheung & Jay David Ramos",
);

// SPM 219 — The Clone Saga (alternate printing)
const THE_CLONE_SAGA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_CLONE_SAGA,
    1,
    "903b133f-fd72-4173-9dfe-3dcc411a0b92",
    "Logan Lubera",
);

// SPM 220 — Norman Osborn // Green Goblin (alternate printing)
const NORMAN_OSBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NORMAN_OSBORN,
    1,
    "f35d0be0-19e3-417e-aabf-fbed1aefd73c",
    "Steve Ellis",
);

// SPM 221 — Behold the Sinister Six! (alternate printing)
const BEHOLD_THE_SINISTER_SIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEHOLD_THE_SINISTER_SIX,
    1,
    "62b76152-4017-487e-81ef-008084e5dcd4",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 222 — Black Cat, Cunning Thief (alternate printing)
const BLACK_CAT_CUNNING_THIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLACK_CAT_CUNNING_THIEF,
    1,
    "aae8c437-b172-44cd-98e8-d0ab9fae54c2",
    "Veronica Fish",
);

// SPM 223 — The Death of Gwen Stacy (alternate printing)
const THE_DEATH_OF_GWEN_STACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DEATH_OF_GWEN_STACY,
    1,
    "aa32b689-9c5c-4c6c-b5be-0e036e9a3223",
    "Nicola Scott",
);

// SPM 224 — Eddie Brock // Venom, Lethal Protector (alternate printing)
const EDDIE_BROCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EDDIE_BROCK,
    1,
    "1a172a40-7758-43ca-861b-8375a94d329e",
    "Logan Lubera",
);

// SPM 225 — Maximum Carnage (alternate printing)
const MAXIMUM_CARNAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAXIMUM_CARNAGE,
    1,
    "cdba10bb-7766-45a9-ab2d-7d62309cefea",
    "Jim Cheung & Jay David Ramos",
);

// SPM 226 — Kraven's Last Hunt (alternate printing)
const KRAVEN_S_LAST_HUNT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRAVEN_S_LAST_HUNT,
    1,
    "eae0bf5a-d5b1-4d87-af8e-e7dd512dfb4a",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 227 — Carnage, Crimson Chaos (alternate printing)
const CARNAGE_CRIMSON_CHAOS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARNAGE_CRIMSON_CHAOS,
    1,
    "67a7cbe7-cf78-4a3f-86c3-00d5ab93361e",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 228 — Doctor Octopus, Master Planner (alternate printing)
const DOCTOR_OCTOPUS_MASTER_PLANNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOCTOR_OCTOPUS_MASTER_PLANNER,
    1,
    "7e6e16f4-9448-4161-8e88-fc7734c2668d",
    "Jim Cheung & Jay David Ramos",
);

// SPM 229 — Mary Jane Watson (alternate printing)
const MARY_JANE_WATSON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARY_JANE_WATSON,
    1,
    "2e5e3a7c-ebb9-483f-9482-636c28f539e0",
    "Roberta Ingranata",
);

// SPM 230 — Spider-Woman, Stunning Savior (alternate printing)
const SPIDER_WOMAN_STUNNING_SAVIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_WOMAN_STUNNING_SAVIOR,
    1,
    "4d502370-a5d9-41b6-a1d3-fa9626b4fc8b",
    "Roberta Ingranata",
);

// SPM 231 — The Spot, Living Portal (alternate printing)
const THE_SPOT_LIVING_PORTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SPOT_LIVING_PORTAL,
    1,
    "6891b71d-c7af-4e55-afb4-72ab6887d290",
    "Tyler Walpole",
);

// SPM 232 — Peter Parker // Amazing Spider-Man (alternate printing)
const PETER_PARKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PETER_PARKER,
    2,
    "98912aae-4e42-4434-b979-9c85f09f8d6d",
    "Jack Kirby & Steve Ditko",
);

// SPM 233 — Eddie Brock // Venom, Lethal Protector (alternate printing)
const EDDIE_BROCK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EDDIE_BROCK,
    2,
    "4d174223-b7d5-4c71-8a3d-8c878a5b45b7",
    "Todd McFarlane & Bob Sharen",
);

// SPM 234 — Miles Morales // Ultimate Spider-Man (alternate printing)
const MILES_MORALES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MILES_MORALES,
    3,
    "d938ceec-c45b-482c-8841-c97098697cc8",
    "Sara Pichelli & Justin Ponsor",
);

// SPM 235 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    1,
    "6e9491af-6e93-46b8-b98c-96a2cc621454",
    "Roberta Ingranata",
);

// SPM 236 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    2,
    "ee9d4bf3-a53e-4940-b25b-e927f06df736",
    "Roberta Ingranata",
);

// SPM 237 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    3,
    "59bdb4a0-0f2d-4018-ad74-3970a5cd71ab",
    "Roberta Ingranata",
);

// SPM 238 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    4,
    "49852fa3-272c-4ca4-b0dc-0b80f2982fd9",
    "Roberta Ingranata",
);

// SPM 239 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    5,
    "cae418f9-e001-4580-b743-74d134b06aa6",
    "Roberta Ingranata",
);

// SPM 240 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    6,
    "2362fb6d-161d-479b-b1f4-909360ba5d84",
    "Roberta Ingranata",
);

// SPM 241 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    7,
    "011b71a9-c06d-4fd6-8543-ad21f4759473",
    "Roberta Ingranata",
);

// SPM 242 — The Soul Stone (alternate printing)
const THE_SOUL_STONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SOUL_STONE,
    1,
    "2c3df372-09d5-42fb-9357-3f97745e07c4",
    "Madeline Boni",
);

// SPM 243 — The Soul Stone (alternate printing)
const THE_SOUL_STONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_SOUL_STONE,
    2,
    "f9d80efc-e829-4257-83e8-f37b0b68de57",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 244 — Anti-Venom, Horrifying Healer (alternate printing)
const ANTI_VENOM_HORRIFYING_HEALER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTI_VENOM_HORRIFYING_HEALER,
    1,
    "451f215d-b451-42d7-a277-631b6ad8098f",
    "Néstor Ossandón Leal",
);

// SPM 245 — Arachne, Psionic Weaver (alternate printing)
const ARACHNE_PSIONIC_WEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARACHNE_PSIONIC_WEAVER,
    1,
    "f3b2924f-fad0-4bff-ab99-091b1c234297",
    "Steve Argyle",
);

// SPM 246 — Friendly Neighborhood (alternate printing)
const FRIENDLY_NEIGHBORHOOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRIENDLY_NEIGHBORHOOD,
    1,
    "2e7c80af-4841-4e5f-a3c9-8448b63b3788",
    "Pablo Mendoza",
);

// SPM 247 — Rent Is Due (alternate printing)
const RENT_IS_DUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RENT_IS_DUE,
    1,
    "c60530c0-4ce1-434f-9140-c50370d418b1",
    "Gal Or",
);

// SPM 248 — With Great Power . . . (alternate printing)
const WITH_GREAT_POWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WITH_GREAT_POWER,
    1,
    "fb266506-5ea8-45c2-8740-4a71fb7e5133",
    "E. M. Gist",
);

// SPM 249 — Hide on the Ceiling (alternate printing)
const HIDE_ON_THE_CEILING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIDE_ON_THE_CEILING,
    1,
    "3d9847a8-ae5d-4f24-8605-609f0c1a11bc",
    "Fariba Khamseh",
);

// SPM 250 — Hydro-Man, Fluid Felon (alternate printing)
const HYDRO_MAN_FLUID_FELON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYDRO_MAN_FLUID_FELON,
    1,
    "0a1265db-ff16-4751-9301-78d29b271cda",
    "Borja Pindado",
);

// SPM 251 — Impostor Syndrome (alternate printing)
const IMPOSTOR_SYNDROME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IMPOSTOR_SYNDROME,
    1,
    "72341b76-c3bb-4c39-80d9-f4747d139375",
    "Javier Charro",
);

// SPM 252 — Lady Octopus, Inspired Inventor (alternate printing)
const LADY_OCTOPUS_INSPIRED_INVENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LADY_OCTOPUS_INSPIRED_INVENTOR,
    1,
    "13c85292-7dc8-4824-8e7f-189ab9420af8",
    "Fariba Khamseh",
);

// SPM 253 — Mysterio, Master of Illusion (alternate printing)
const MYSTERIO_MASTER_OF_ILLUSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYSTERIO_MASTER_OF_ILLUSION,
    1,
    "be0cd0c9-9d43-47d1-9836-24e8d5917f65",
    "Alexander Gering",
);

// SPM 254 — Spider-Sense (alternate printing)
const SPIDER_SENSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_SENSE,
    1,
    "426b3c04-4d4b-4f75-984f-89eecf8f09f2",
    "Borja Pindado",
);

// SPM 255 — Agent Venom (alternate printing)
const AGENT_VENOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGENT_VENOM,
    1,
    "5569baf4-0cc9-46da-80cb-2cac5452f460",
    "Kevin Sidharta",
);

// SPM 256 — Gwenom, Remorseless (alternate printing)
const GWENOM_REMORSELESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GWENOM_REMORSELESS,
    1,
    "431f3f3b-42a7-4161-946b-c9bbf7bd5535",
    "Lordigan",
);

// SPM 257 — Morlun, Devourer of Spiders (alternate printing)
const MORLUN_DEVOURER_OF_SPIDERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORLUN_DEVOURER_OF_SPIDERS,
    1,
    "2c5efc7d-7b09-4487-85cb-58867e06b12d",
    "Randy Gallegos",
);

// SPM 258 — Parker Luck (alternate printing)
const PARKER_LUCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PARKER_LUCK,
    1,
    "c69e751f-5eee-48d7-9e0f-ddbcca96491e",
    "Raoul Vitale",
);

// SPM 259 — Villainous Wrath (alternate printing)
const VILLAINOUS_WRATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VILLAINOUS_WRATH,
    1,
    "6f7d7311-2248-4841-960a-8c93497e3de1",
    "InHyuk Lee",
);

// SPM 260 — Electro, Assaulting Battery (alternate printing)
const ELECTRO_ASSAULTING_BATTERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELECTRO_ASSAULTING_BATTERY,
    1,
    "40466912-0d20-42ff-8232-807bf947030b",
    "Piotr Dura",
);

// SPM 261 — J. Jonah Jameson (alternate printing)
const J_JONAH_JAMESON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &J_JONAH_JAMESON,
    1,
    "c81a2ecf-837c-44a9-81d4-476763f89107",
    "Paolo Rivera",
);

// SPM 262 — Shadow of the Goblin (alternate printing)
const SHADOW_OF_THE_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHADOW_OF_THE_GOBLIN,
    1,
    "e96068e3-4c79-4c59-8821-053f21db42c7",
    "Pavel Kolomeyets",
);

// SPM 263 — Spider-Verse (alternate printing)
const SPIDER_VERSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_VERSE,
    1,
    "5e85512d-0b46-48f9-a391-6ae6a1b7fb55",
    "Alexander Gering",
);

// SPM 264 — Spinneret and Spiderling (alternate printing)
const SPINNERET_AND_SPIDERLING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPINNERET_AND_SPIDERLING,
    1,
    "d75ccaee-e5ca-423f-a235-c0e989cbf76f",
    "Le Vuong",
);

// SPM 265 — Lizard, Connors's Curse (alternate printing)
const LIZARD_CONNORS_S_CURSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIZARD_CONNORS_S_CURSE,
    1,
    "fef697cd-7bc7-4c1f-87cd-2921c48bc02a",
    "Steve Prescott",
);

// SPM 266 — Sandman, Shifting Scoundrel (alternate printing)
const SANDMAN_SHIFTING_SCOUNDREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANDMAN_SHIFTING_SCOUNDREL,
    1,
    "496864ab-bdba-4cfe-aa7b-b58d2954cf75",
    "Bartek Fedyczak",
);

// SPM 267 — Strength of Will (alternate printing)
const STRENGTH_OF_WILL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STRENGTH_OF_WILL,
    1,
    "084f283e-88a0-4b7a-9e63-908caeb70f04",
    "Ryan Pancoast",
);

// SPM 268 — Web of Life and Destiny (alternate printing)
const WEB_OF_LIFE_AND_DESTINY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEB_OF_LIFE_AND_DESTINY,
    1,
    "1672374a-585d-4a26-b04f-a129c4da8b0e",
    "Jonas De Ro",
);

// SPM 269 — Biorganic Carapace (alternate printing)
const BIORGANIC_CARAPACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BIORGANIC_CARAPACE,
    1,
    "2bd4c0aa-f431-423e-8f95-2796a0f05594",
    "David Álvarez",
);

// SPM 270 — Cheering Crowd (alternate printing)
const CHEERING_CROWD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHEERING_CROWD,
    1,
    "44f436ca-0ae8-4c35-871b-4bd2732e5a49",
    "Kim Sokol",
);

// SPM 271 — Cosmic Spider-Man (alternate printing)
const COSMIC_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMIC_SPIDER_MAN,
    1,
    "61b28030-6d05-429e-a4eb-fceead3f7b18",
    "Zoltan Boros",
);

// SPM 272 — Jackal, Genius Geneticist (alternate printing)
const JACKAL_GENIUS_GENETICIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JACKAL_GENIUS_GENETICIST,
    1,
    "185bf150-e032-47af-aafc-3b4b69f0bad4",
    "Pavel Kolomeyets",
);

// SPM 273 — Kraven the Hunter (alternate printing)
const KRAVEN_THE_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRAVEN_THE_HUNTER,
    1,
    "82f977d1-2209-4378-9b79-edb2c338f544",
    "Greg Staples",
);

// SPM 274 — Mister Negative (alternate printing)
const MISTER_NEGATIVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MISTER_NEGATIVE,
    1,
    "e713824a-b5df-4a05-81ad-a426c5eae1ce",
    "Thanh Tuấn",
);

// SPM 275 — Superior Spider-Man (alternate printing)
const SUPERIOR_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPERIOR_SPIDER_MAN,
    1,
    "895a6b80-a5c6-400d-931c-589bfa684b39",
    "Carlos Dattoli",
);

// SPM 276 — Ultimate Green Goblin (alternate printing)
const ULTIMATE_GREEN_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTIMATE_GREEN_GOBLIN,
    1,
    "5e96fe19-9671-48b0-92fc-c46224c433c6",
    "Jesper Ejsing",
);

// SPM 277 — Doc Ock's Tentacles (alternate printing)
const DOC_OCK_S_TENTACLES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOC_OCK_S_TENTACLES,
    1,
    "eab4f4f1-3e4e-403e-8264-b3e73ee8259a",
    "David Álvarez",
);

// SPM 278 — Interdimensional Web Watch (alternate printing)
const INTERDIMENSIONAL_WEB_WATCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INTERDIMENSIONAL_WEB_WATCH,
    1,
    "740cde32-57e6-4633-98e9-724c505eb008",
    "Toni Infante",
);

// SPM 279 — Iron Spider, Stark Upgrade (alternate printing)
const IRON_SPIDER_STARK_UPGRADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_SPIDER_STARK_UPGRADE,
    1,
    "517aeeb2-454f-4de8-b7f7-870010e80ae8",
    "Kevin Glint",
);

// SPM 280 — Peter Parker's Camera (alternate printing)
const PETER_PARKER_S_CAMERA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PETER_PARKER_S_CAMERA,
    1,
    "47c82856-d657-4315-9433-0754a68444eb",
    "Lixin Yin",
);

// SPM 281 — Rocket-Powered Goblin Glider (alternate printing)
const ROCKET_POWERED_GOBLIN_GLIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROCKET_POWERED_GOBLIN_GLIDER,
    1,
    "9fb3e089-17f0-4066-9ab2-c23b7b2a6ee7",
    "Pavel Kolomeyets",
);

// SPM 282 — Oscorp Industries (alternate printing)
const OSCORP_INDUSTRIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OSCORP_INDUSTRIES,
    1,
    "be2154a1-38c2-46b2-bd6f-3d7e509070b6",
    "Bastien Grivet",
);

// SPM 283 — Urban Retreat (alternate printing)
const URBAN_RETREAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &URBAN_RETREAT,
    1,
    "a2b59a26-114a-489c-b898-f71949ca4c7e",
    "Jonas De Ro",
);

// SPM 284 — Spider-Sense (alternate printing)
const SPIDER_SENSE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_SENSE,
    2,
    "d123d19e-fb13-4c1c-b765-8c4f1fef453a",
    "David Álvarez",
);

// SPM 285 — Radioactive Spider (alternate printing)
const RADIOACTIVE_SPIDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RADIOACTIVE_SPIDER,
    2,
    "3173112e-8975-4bba-84b7-6ba27503dc82",
    "Toni Infante",
);

// SPM 286 — Gwenom, Remorseless (alternate printing)
const GWENOM_REMORSELESS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GWENOM_REMORSELESS,
    2,
    "a5b191f5-f01b-4d6c-8363-19591c61e6cc",
    "Jesper Ejsing",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANTI_VENOM_HORRIFYING_HEALER,
    &ARACHNE_PSIONIC_WEAVER,
    &AUNT_MAY,
    &CITY_PIGEON,
    &COSTUME_CLOSET,
    &DAILY_BUGLE_REPORTERS,
    &FLASH_THOMPSON_SPIDER_FAN,
    &FRIENDLY_NEIGHBORHOOD,
    &ORIGIN_OF_SPIDER_MAN,
    &PETER_PARKER,
    &RENT_IS_DUE,
    &SELFLESS_POLICE_CAPTAIN,
    &SILVER_SABLE_MERCENARY_LEADER,
    &SPECTACULAR_SPIDER_MAN,
    &SPECTACULAR_TACTICS,
    &SPIDER_MAN_WEB_SLINGER,
    &SPIDER_UK,
    &STARLING_AERIAL_ALLY,
    &SUDDEN_STRIKE,
    &THWIP,
    &WEB_UP,
    &WEB_SHOOTERS,
    &WILD_PACK_SQUAD,
    &WITH_GREAT_POWER,
    &AMAZING_ACROBATICS,
    &BEETLE_LEGACY_CRIMINAL,
    &CHAMELEON_MASTER_OF_DISGUISE,
    &THE_CLONE_SAGA,
    &DOC_OCK_SINISTER_SCIENTIST,
    &DOC_OCK_S_HENCHMEN,
    &FLYING_OCTOBOT,
    &HIDE_ON_THE_CEILING,
    &HYDRO_MAN_FLUID_FELON,
    &IMPOSTOR_SYNDROME,
    &LADY_OCTOPUS_INSPIRED_INVENTOR,
    &MADAME_WEB_CLAIRVOYANT,
    &MYSTERIO_MASTER_OF_ILLUSION,
    &MYSTERIO_S_PHANTASM,
    &NORMAN_OSBORN,
    &OSCORP_RESEARCH_TEAM,
    &ROBOTICS_MASTERY,
    &SCHOOL_DAZE,
    &SECRET_IDENTITY,
    &SPIDER_BYTE_WEB_WARDEN,
    &SPIDER_MAN_NO_MORE,
    &SPIDER_SENSE,
    &UNSTABLE_EXPERIMENT,
    &WHOOSH,
    &AGENT_VENOM,
    &ALIEN_SYMBIOSIS,
    &BEHOLD_THE_SINISTER_SIX,
    &BLACK_CAT_CUNNING_THIEF,
    &COMMON_CROOK,
    &THE_DEATH_OF_GWEN_STACY,
    &EDDIE_BROCK,
    &GWENOM_REMORSELESS,
    &INNER_DEMONS_GANGSTERS,
    &MERCILESS_ENFORCERS,
    &MORLUN_DEVOURER_OF_SPIDERS,
    &PARKER_LUCK,
    &PRISON_BREAK,
    &RISKY_RESEARCH,
    &SANDMAN_S_QUICKSAND,
    &SCORPION_SEETHING_STRIKER,
    &SCORPION_S_STING,
    &THE_SOUL_STONE,
    &SPIDER_MAN_NOIR,
    &THE_SPOT_S_PORTAL,
    &SWARM_BEING_OF_BEES,
    &TOMBSTONE_CAREER_CRIMINAL,
    &VENOM_EVIL_UNLEASHED,
    &VENOMIZED_CAT,
    &VENOM_S_HUNGER,
    &VILLAINOUS_WRATH,
    &ANGRY_RABBLE,
    &ELECTRO_ASSAULTING_BATTERY,
    &ELECTRO_S_BOLT,
    &GWEN_STACY,
    &HEROES_HANGOUT,
    &HOBGOBLIN_MANTLED_MARAUDER,
    &J_JONAH_JAMESON,
    &MASKED_MEOWER,
    &MAXIMUM_CARNAGE,
    &MOLTEN_MAN_INFERNO_INCARNATE,
    &RAGING_GOBLINOIDS,
    &ROMANTIC_RENDEZVOUS,
    &SHADOW_OF_THE_GOBLIN,
    &SHOCKER_UNSHAKABLE,
    &SPIDER_GWEN_FREE_SPIRIT,
    &SPIDER_ISLANDERS,
    &SPIDER_PUNK,
    &SPIDER_VERSE,
    &SPINNERET_AND_SPIDERLING,
    &STEGRON_THE_DINOSAUR_MAN,
    &SUPERIOR_FOES_OF_SPIDER_MAN,
    &TAXI_DRIVER,
    &WISECRACK,
    &DAMAGE_CONTROL_CREW,
    &EZEKIEL_SIMS_SPIDER_TOTEM,
    &GROW_EXTRA_ARMS,
    &GUY_IN_THE_CHAIR,
    &KAPOW,
    &KRAVEN_S_CATS,
    &KRAVEN_S_LAST_HUNT,
    &LIZARD_CONNORS_S_CURSE,
    &LURKING_LIZARDS,
    &MILES_MORALES,
    &PICTURES_OF_SPIDER_MAN,
    &PROFESSIONAL_WRESTLER,
    &RADIOACTIVE_SPIDER,
    &SANDMAN_SHIFTING_SCOUNDREL,
    &SCOUT_THE_CITY,
    &SPIDER_HAM_PETER_PORKER,
    &SPIDER_MAN_BROOKLYN_VISIONARY,
    &SPIDER_REX_DARING_DINO,
    &SPIDERS_MAN_HEROIC_HORDE,
    &STRENGTH_OF_WILL,
    &SUPPORTIVE_PARENTS,
    &TERRIFIC_TEAM_UP,
    &WALL_CRAWL,
    &WEB_OF_LIFE_AND_DESTINY,
    &ARANA_HEART_OF_THE_SPIDER,
    &BIORGANIC_CARAPACE,
    &CARNAGE_CRIMSON_CHAOS,
    &CHEERING_CROWD,
    &COSMIC_SPIDER_MAN,
    &DOCTOR_OCTOPUS_MASTER_PLANNER,
    &GALLANT_CITIZEN,
    &GREEN_GOBLIN_REVENANT,
    &JACKAL_GENIUS_GENETICIST,
    &KRAVEN_PROUD_PREDATOR,
    &KRAVEN_THE_HUNTER,
    &MARY_JANE_WATSON,
    &MISTER_NEGATIVE,
    &MOB_LOOKOUT,
    &MORBIUS_THE_LIVING_VAMPIRE,
    &PROWLER_CLAWED_THIEF,
    &PUMPKIN_BOMBARDMENT,
    &RHINO_BARRELING_BRUTE,
    &RHINOS_RAMPAGE,
    &SCARLET_SPIDER_BEN_REILLY,
    &SCARLET_SPIDER_KAINE,
    &SHRIEK_TREBLEMAKER,
    &SILK_WEB_WEAVER,
    &SKYWARD_SPIDER,
    &SP_DR_PILOTED_BY_PENI,
    &SPIDER_MANIFESTATION,
    &SPIDER_GIRL_LEGACY_HERO,
    &SPIDER_MAN_2099,
    &SPIDER_MAN_INDIA,
    &SPIDER_WOMAN_STUNNING_SAVIOR,
    &THE_SPOT_LIVING_PORTAL,
    &SUN_SPIDER_NIMBLE_WEBBER,
    &SUPERIOR_SPIDER_MAN,
    &SYMBIOTE_SPIDER_MAN,
    &ULTIMATE_GREEN_GOBLIN,
    &VULTURE_SCHEMING_SCAVENGER,
    &WEB_WARRIORS,
    &WRAITH_VICIOUS_VIGILANTE,
    &BAGEL_AND_SCHMEAR,
    &DOC_OCK_S_TENTACLES,
    &EERIE_GRAVESTONE,
    &HOT_DOG_CART,
    &INTERDIMENSIONAL_WEB_WATCH,
    &IRON_SPIDER_STARK_UPGRADE,
    &LIVING_BRAIN_MECHANICAL_MARVEL,
    &MECHANICAL_MOBSTER,
    &NEWS_HELICOPTER,
    &PASSENGER_FERRY,
    &PETER_PARKER_S_CAMERA,
    &ROCKET_POWERED_GOBLIN_GLIDER,
    &SPIDER_BOT,
    &SPIDER_MOBILE,
    &SPIDER_SLAYER_HATRED_HONED,
    &SPIDER_SUIT,
    &STEEL_WRECKING_BALL,
    &SUBWAY_TRAIN,
    &DAILY_BUGLE_BUILDING,
    &MULTIVERSAL_PASSAGE,
    &OMINOUS_ASYLUM,
    &OSCORP_INDUSTRIES,
    &SAVAGE_MANSION,
    &SINISTER_HIDEOUT,
    &SUBURBAN_SANCTUARY,
    &UNIVERSITY_CAMPUS,
    &URBAN_RETREAT,
    &VIBRANT_CITYSCAPE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    SHOCK_REPRINT,
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
    SP_DR_PILOTED_BY_PENI_ALTERNATE_1,
    MILES_MORALES_ALTERNATE_1,
    SPIDER_HAM_PETER_PORKER_ALTERNATE_1,
    GWEN_STACY_ALTERNATE_1,
    WEB_WARRIORS_ALTERNATE_1,
    SPIDER_MAN_NOIR_ALTERNATE_1,
    SPIDER_MAN_2099_ALTERNATE_1,
    MULTIVERSAL_PASSAGE_ALTERNATE_1,
    SPIDER_PUNK_ALTERNATE_1,
    PETER_PARKER_ALTERNATE_1,
    GWEN_STACY_ALTERNATE_2,
    SPIDER_PUNK_ALTERNATE_2,
    MILES_MORALES_ALTERNATE_2,
    RADIOACTIVE_SPIDER_ALTERNATE_1,
    ARANA_HEART_OF_THE_SPIDER_ALTERNATE_1,
    SCARLET_SPIDER_BEN_REILLY_ALTERNATE_1,
    SILK_WEB_WEAVER_ALTERNATE_1,
    SPIDER_MAN_2099_ALTERNATE_2,
    SYMBIOTE_SPIDER_MAN_ALTERNATE_1,
    ORIGIN_OF_SPIDER_MAN_ALTERNATE_1,
    THE_CLONE_SAGA_ALTERNATE_1,
    NORMAN_OSBORN_ALTERNATE_1,
    BEHOLD_THE_SINISTER_SIX_ALTERNATE_1,
    BLACK_CAT_CUNNING_THIEF_ALTERNATE_1,
    THE_DEATH_OF_GWEN_STACY_ALTERNATE_1,
    EDDIE_BROCK_ALTERNATE_1,
    MAXIMUM_CARNAGE_ALTERNATE_1,
    KRAVEN_S_LAST_HUNT_ALTERNATE_1,
    CARNAGE_CRIMSON_CHAOS_ALTERNATE_1,
    DOCTOR_OCTOPUS_MASTER_PLANNER_ALTERNATE_1,
    MARY_JANE_WATSON_ALTERNATE_1,
    SPIDER_WOMAN_STUNNING_SAVIOR_ALTERNATE_1,
    THE_SPOT_LIVING_PORTAL_ALTERNATE_1,
    PETER_PARKER_ALTERNATE_2,
    EDDIE_BROCK_ALTERNATE_2,
    MILES_MORALES_ALTERNATE_3,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_1,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_2,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_3,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_4,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_5,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_6,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_7,
    THE_SOUL_STONE_ALTERNATE_1,
    THE_SOUL_STONE_ALTERNATE_2,
    ANTI_VENOM_HORRIFYING_HEALER_ALTERNATE_1,
    ARACHNE_PSIONIC_WEAVER_ALTERNATE_1,
    FRIENDLY_NEIGHBORHOOD_ALTERNATE_1,
    RENT_IS_DUE_ALTERNATE_1,
    WITH_GREAT_POWER_ALTERNATE_1,
    HIDE_ON_THE_CEILING_ALTERNATE_1,
    HYDRO_MAN_FLUID_FELON_ALTERNATE_1,
    IMPOSTOR_SYNDROME_ALTERNATE_1,
    LADY_OCTOPUS_INSPIRED_INVENTOR_ALTERNATE_1,
    MYSTERIO_MASTER_OF_ILLUSION_ALTERNATE_1,
    SPIDER_SENSE_ALTERNATE_1,
    AGENT_VENOM_ALTERNATE_1,
    GWENOM_REMORSELESS_ALTERNATE_1,
    MORLUN_DEVOURER_OF_SPIDERS_ALTERNATE_1,
    PARKER_LUCK_ALTERNATE_1,
    VILLAINOUS_WRATH_ALTERNATE_1,
    ELECTRO_ASSAULTING_BATTERY_ALTERNATE_1,
    J_JONAH_JAMESON_ALTERNATE_1,
    SHADOW_OF_THE_GOBLIN_ALTERNATE_1,
    SPIDER_VERSE_ALTERNATE_1,
    SPINNERET_AND_SPIDERLING_ALTERNATE_1,
    LIZARD_CONNORS_S_CURSE_ALTERNATE_1,
    SANDMAN_SHIFTING_SCOUNDREL_ALTERNATE_1,
    STRENGTH_OF_WILL_ALTERNATE_1,
    WEB_OF_LIFE_AND_DESTINY_ALTERNATE_1,
    BIORGANIC_CARAPACE_ALTERNATE_1,
    CHEERING_CROWD_ALTERNATE_1,
    COSMIC_SPIDER_MAN_ALTERNATE_1,
    JACKAL_GENIUS_GENETICIST_ALTERNATE_1,
    KRAVEN_THE_HUNTER_ALTERNATE_1,
    MISTER_NEGATIVE_ALTERNATE_1,
    SUPERIOR_SPIDER_MAN_ALTERNATE_1,
    ULTIMATE_GREEN_GOBLIN_ALTERNATE_1,
    DOC_OCK_S_TENTACLES_ALTERNATE_1,
    INTERDIMENSIONAL_WEB_WATCH_ALTERNATE_1,
    IRON_SPIDER_STARK_UPGRADE_ALTERNATE_1,
    PETER_PARKER_S_CAMERA_ALTERNATE_1,
    ROCKET_POWERED_GOBLIN_GLIDER_ALTERNATE_1,
    OSCORP_INDUSTRIES_ALTERNATE_1,
    URBAN_RETREAT_ALTERNATE_1,
    SPIDER_SENSE_ALTERNATE_2,
    RADIOACTIVE_SPIDER_ALTERNATE_2,
    GWENOM_REMORSELESS_ALTERNATE_2,
];
