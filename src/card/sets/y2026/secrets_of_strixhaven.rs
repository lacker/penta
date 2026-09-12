//! Secrets of Strixhaven card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::CardNameDef;
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
use crate::card::CostQuantityDef;
use crate::card::CreateTokenDef;
use crate::card::CounterKind;
use crate::card::CreatedTokensDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectCounterValueDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::ScaledValueDef;
use crate::card::SpellCastQueryDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::StackTargetAggregationDef;
use crate::card::StackTargetFilterDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCopyDef;
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
use crate::card::sets::y2006::time_spiral as catalog_tsp;
use crate::card::sets::y2009::magic_2010 as catalog_m10;
use crate::card::sets::y2021::innistrad_crimson_vow as catalog_vow;
use crate::card::sets::y2021::kaldheim as catalog_khm;
use crate::card::sets::y2023::wilds_of_eldraine as catalog_woe;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SOS",
    slug: "secrets-of-strixhaven",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// SOS 1 — The Dawning Archaic
pub(in crate::card::sets) static THE_DAWNING_ARCHAIC: CardRecord = CardRecord::new(
    "The Dawning Archaic",
    "7a451985-37e1-44d8-839b-dc1e88df5c96",
    "Josu Solano",
    CardRules::new_creature(mana_cost!("{10}"), &["Avatar"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This spell costs {1} less to cast for each instant and \
                 sorcery card in your graveyard.",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                )),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::reach(),
            AbilityDef::triggered_with_targets(
                "Whenever The Dawning Archaic attacks, you may cast target \
                 instant or sorcery card from your graveyard without paying \
                 its mana cost. If that spell would be put into your \
                 graveyard, exile it instead.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
                EffectDef::MayCastTargetWithoutPaying {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ability: &AbilityDef::alternative_cast(
                        crate::NO_COSTS,
                        AlternativeCastKindDef::Granted,
                        Some("Cast without paying its mana cost, then exile it."),
                        EffectDef::None,
                    )
                    .with_exile_if_put_into_graveyard(),
                },
            ),
        ]),
);

// SOS 2 — Rancorous Archaic
pub(in crate::card::sets) static RANCOROUS_ARCHAIC: CardRecord = CardRecord::new(
    "Rancorous Archaic",
    "2565e16a-ed31-4867-adb8-f1633d580397",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{5}"), &["Avatar"], 2, 2).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        AbilityDef::as_enters(
            "Converge — This creature enters with a +1/+1 counter on it \
             for each color of mana spent to cast it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::ColorsOfManaSpent,
                },
            ),
        ),
    ]),
);

// SOS 3 — Sundering Archaic
// Audit: unsupported — Needs a targeting value reading colors of mana spent from the source permanent's retained cast facts; ColorsOfManaSpent currently reads stack objects and cannot price this enters-trigger target limit.
pub(in crate::card::sets) static SUNDERING_ARCHAIC: CardRecord = CardRecord::new(
    "Sundering Archaic",
    "c35b57e4-2358-46c0-8f09-cd27c10eaf2d",
    "Quintin Gleim",
    CardRules::unsupported(),
);

// SOS 4 — Together as One
pub(in crate::card::sets) static TOGETHER_AS_ONE: CardRecord = CardRecord::new(
    "Together as One",
    "ac2a8a66-e38c-42ab-83e1-d2d99ee48861",
    "Néstor Ossandón Leal",
    CardRules::new_sorcery(mana_cost!("{6}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Converge — Target player draws X cards, Together as One deals \
         X damage to any target, and you gain X life, where X is the \
         number of colors of mana spent to cast this spell.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
        ],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ColorsOfManaSpent,
            },
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::ColorsOfManaSpent,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ColorsOfManaSpent,
            },
        ]),
    )]),
);

// SOS 5 — Transcendent Archaic
// Audit: unsupported — Needs the source permanent's retained cast-color count in the resolving enters trigger, plus a receipt counting cards actually drawn before deciding whether to discard two.
pub(in crate::card::sets) static TRANSCENDENT_ARCHAIC: CardRecord = CardRecord::new(
    "Transcendent Archaic",
    "1624c680-502b-474a-b9b2-888fe3ca008c",
    "Chris Rahn",
    CardRules::unsupported(),
);

// SOS 6 — Ajani's Response
// Audit: unsupported — Needs the spell's selected target characteristics in its self-cost-reduction condition; the current self-cost reader cannot condition a discount on a tapped or attacking target.
pub(in crate::card::sets) static AJANI_S_RESPONSE: CardRecord = CardRecord::new(
    "Ajani's Response",
    "9cd1417a-badc-4abd-a8ca-5b31f85c1072",
    "April Prime",
    CardRules::unsupported(),
);

// SOS 7 — Antiquities on the Loose
pub(in crate::card::sets) static ANTIQUITIES_ON_THE_LOOSE: CardRecord = CardRecord::new(
    "Antiquities on the Loose",
    "68ee92cd-51af-4de5-bcc8-34d0bb2fd398",
    "Andreas Zafiratos",
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 2/2 red and white Spirit creature tokens. Then if \
             this spell was cast from anywhere other than your hand, put a \
             +1/+1 counter on each Spirit you control.",
            EffectDef::Sequence(&[
                EffectDef::create_creature_token(
                    &["Spirit"],
                    &[ManaColor::Red, ManaColor::White],
                    2,
                    2,
                )
                .with_count(ValueDef::Constant(2)),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::All(&[
                        TriggerConditionDef::SourceWasCast,
                        TriggerConditionDef::Not(&TriggerConditionDef::SourceCastFrom(
                            ZoneKind::Hand,
                        )),
                    ]),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spirit")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{W}{W}"))]),
    ]),
);

// SOS 8 — Ascendant Dustspeaker
pub(in crate::card::sets) static ASCENDANT_DUSTSPEAKER: CardRecord = CardRecord::new(
    "Ascendant Dustspeaker",
    "de3de40b-a7ac-455e-add2-4e451b602d17",
    "Josiah \"Jo\" Cameron",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Orc", "Cleric"], 3, 4).with_abilities(&[
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
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, exile up to one \
             target card from a graveyard.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::Any),
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// SOS 9 — Daydream
pub(in crate::card::sets) static DAYDREAM: CardRecord = CardRecord::new(
    "Daydream",
    "e2b16cb2-b8b2-45df-9695-3c16e9d89e28",
    "Nia Kovalevski",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature you control, then return that card to \
             the battlefield under its owner's control with a +1/+1 \
             counter on it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("exiled"),
                then: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                            crate::Binding!("exiled"),
                        )),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::AddCounters {
                            kind: CounterKind::PlusOnePlusOne,
                            amount: 1,
                        }],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{W}"))]),
    ]),
);

// SOS 10 — Dig Site Inventory
pub(in crate::card::sets) static DIG_SITE_INVENTORY: CardRecord = CardRecord::new(
    "Dig Site Inventory",
    "e52464ee-df8b-41ec-af93-4b0eb004383e",
    "Alexandre Honoré",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on target creature you control. It gains \
             vigilance until end of turn.",
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
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{W}"))]),
    ]),
);

// SOS 11 — Eager Glyphmage
pub(in crate::card::sets) static EAGER_GLYPHMAGE: CardRecord = CardRecord::new(
    "Eager Glyphmage",
    "bf736de9-9bc4-49df-ae60-672ed4f83f32",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Cat", "Cleric"], 3, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white and black \
             Inkling creature token with flying.",
            EffectDef::create_creature_token(
                &["Inkling"],
                &[ManaColor::White, ManaColor::Black],
                1,
                1,
            )
            .with_abilities(&[abilities::flying()]),
        ),
    ]),
);

// SOS 12 — Elite Interceptor
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static ELITE_INTERCEPTOR: CardRecord = CardRecord::new(
    "Elite Interceptor",
    "2970683e-e69c-42cb-a067-34abd56fb42b",
    "Lindsey Look",
    CardRules::unsupported(),
);

// SOS 13 — Emeritus of Truce // Swords to Plowshares
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static EMERITUS_OF_TRUCE: CardRecord = CardRecord::new(
    "Emeritus of Truce // Swords to Plowshares",
    "9869a753-5e41-4098-ab41-e75b4396ec50",
    "Aleksi Briclot",
    CardRules::unsupported(),
);

// SOS 14 — Ennis, Debate Moderator
// Audit: unsupported — Needs a turn-scoped history flag for cards entering exile that remains true after those cards leave exile.
pub(in crate::card::sets) static ENNIS_DEBATE_MODERATOR: CardRecord = CardRecord::new(
    "Ennis, Debate Moderator",
    "d2ef31b4-24fa-4443-9f05-c8e99c3522e5",
    "Marie Magny",
    CardRules::unsupported(),
);

// SOS 15 — Erode
pub(in crate::card::sets) static ERODE: CardRecord = CardRecord::new(
    "Erode",
    "32e670da-7563-4f6a-a7db-4c126a440eb8",
    "Florian Herold",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature or planeswalker. Its controller may \
         search their library for a basic land card, put it onto the \
         battlefield tapped, then shuffle.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::SearchZone {
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
        ]),
    )]),
);

// SOS 16 — Graduation Day
pub(in crate::card::sets) static GRADUATION_DAY: CardRecord = CardRecord::new(
    "Graduation Day",
    "db1cea01-123e-4b23-8b98-f94cabce3912",
    "Brian Valeza",
    CardRules::new_enchantment(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, put a +1/+1 counter on target creature \
             you control.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
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
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// SOS 17 — Group Project
pub(in crate::card::sets) static GROUP_PROJECT: CardRecord = CardRecord::new(
    "Group Project",
    "e8abc1eb-6225-4b18-8502-b5324b818aed",
    "Justine Cruz",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 2/2 red and white Spirit creature token.",
            EffectDef::create_creature_token(
                &["Spirit"],
                &[ManaColor::Red, ManaColor::White],
                2,
                2,
            ),
        ),
        abilities::flashback(&[CostDef::TapPermanents {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            controller: PlayerRelation::You,
            count: 3,
        }]),
    ]),
);

// SOS 18 — Harsh Annotation
pub(in crate::card::sets) static HARSH_ANNOTATION: CardRecord = CardRecord::new(
    "Harsh Annotation",
    "e07a8fc7-c11c-4469-a31d-0abf40e57bbf",
    "Liiga Smilshkalne",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature. Its controller creates a 1/1 white \
         and black Inkling creature token with flying.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::create_creature_token(
                &["Inkling"],
                &[ManaColor::White, ManaColor::Black],
                1,
                1,
            )
            .with_abilities(&[abilities::flying()])
            .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                TargetIndex::PRIMARY,
            ))),
        ]),
    )]),
);

// SOS 19 — Honorbound Page // Forum's Favor
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static HONORBOUND_PAGE: CardRecord = CardRecord::new(
    "Honorbound Page // Forum's Favor",
    "79a70863-860f-4a7b-9cb2-d3546b689d44",
    "Paolo Parente",
    CardRules::unsupported(),
);

// SOS 20 — Informed Inkwright
pub(in crate::card::sets) static INFORMED_INKWRIGHT: CardRecord = CardRecord::new(
    "Informed Inkwright",
    "5defb2d1-d0fb-4e7f-a5c7-3df99fe675d6",
    "Nia Kovalevski",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, create a 1/1 white and black Inkling \
             creature token with flying.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::create_creature_token(
                &["Inkling"],
                &[ManaColor::White, ManaColor::Black],
                1,
                1,
            )
            .with_abilities(&[abilities::flying()]),
        ),
    ]),
);

// SOS 21 — Inkshape Demonstrator
pub(in crate::card::sets) static INKSHAPE_DEMONSTRATOR: CardRecord = CardRecord::new(
    "Inkshape Demonstrator",
    "bcfac992-9984-4529-b9ff-a42d58832b34",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Elephant", "Cleric"], 3, 4).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, this creature gets +1/+0 and gains \
             lifelink until end of turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SOS 22 — Interjection
pub(in crate::card::sets) static INTERJECTION: CardRecord = CardRecord::new(
    "Interjection",
    "0534cff6-299c-4155-b318-eb7581989e8a",
    "Anna Pavleeva",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains first strike until end \
         of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
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
    )]),
);

// SOS 23 — Joined Researchers // Secret Rendezvous
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static JOINED_RESEARCHERS: CardRecord = CardRecord::new(
    "Joined Researchers // Secret Rendezvous",
    "1ebaafe0-3a9a-424c-8698-d26e7be45343",
    "Ryan Pancoast",
    CardRules::unsupported(),
);

// SOS 24 — Owlin Historian
// Audit: unsupported — Needs a grouped one-or-more-cards-leave-your-graveyard event; the current per-card zone-change event would trigger separately for cards moved together.
pub(in crate::card::sets) static OWLIN_HISTORIAN: CardRecord = CardRecord::new(
    "Owlin Historian",
    "5fe99be0-e1ec-485e-82f8-02eba7b82441",
    "Matheus Graef",
    CardRules::unsupported(),
);

// SOS 25 — Practiced Offense
pub(in crate::card::sets) static PRACTICED_OFFENSE: CardRecord = CardRecord::new(
    "Practiced Offense",
    "79c7cf94-c0a1-432d-90d7-7f0599c2e7a8",
    "Raluca Marinescu",
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on each creature target player controls. \
             Target creature gains your choice of double strike or \
             lifelink until end of turn.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::Sequence(&[
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
                EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Double strike",
                            effect: EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        },
                        EffectChoiceDef {
                            label: "Lifelink",
                            effect: EffectDef::Apply {
                                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        },
                    ],
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{1}{W}"))]),
    ]),
);

// SOS 26 — Primary Research
pub(in crate::card::sets) static PRIMARY_RESEARCH: CardRecord = CardRecord::new(
    "Primary Research",
    "f6fdb814-45c6-4d14-afff-7f5bd1bd10a1",
    "Michal Ivan",
    CardRules::new_enchantment(mana_cost!("{4}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, return target nonland permanent \
             card with mana value 3 or less from your graveyard to the \
             battlefield.",
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
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(3),
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
        AbilityDef::triggered_if(
            "At the beginning of your end step, if a card left your \
             graveyard this turn, draw a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn,
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// SOS 27 — Quill-Blade Laureate // Twofold Intent
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static QUILL_BLADE_LAUREATE: CardRecord = CardRecord::new(
    "Quill-Blade Laureate // Twofold Intent",
    "62a47835-5719-48c4-a740-a0c5f00dce11",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// SOS 28 — Rapier Wit
pub(in crate::card::sets) static RAPIER_WIT: CardRecord = CardRecord::new(
    "Rapier Wit",
    "97b50521-5a0f-4dbd-8e15-f0f0d059c258",
    "Joshua Raphael",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tap target creature. If it's your turn, put a stun counter on \
         it. (If a permanent with a stun counter would become \
         untapped, remove one from it instead.)\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(1),
                },
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// SOS 29 — Rehearsed Debater
pub(in crate::card::sets) static REHEARSED_DEBATER: CardRecord = CardRecord::new(
    "Rehearsed Debater",
    "fc6a1425-6733-42a2-94d9-605966398092",
    "Kim Sokol",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Djinn", "Bard"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, this creature gets +1/+1 until end of \
             turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
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

// SOS 30 — Restoration Seminar
// Audit: unsupported — Needs per-player first-resolution history keyed by spell name and the persistent paradigm permission offering a free copy from exile at each first main phase.
pub(in crate::card::sets) static RESTORATION_SEMINAR: CardRecord = CardRecord::new(
    "Restoration Seminar",
    "9ebc4ecf-2fa2-4ab8-afde-3b91cf5eadb6",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// SOS 31 — Shattered Acolyte
pub(in crate::card::sets) static SHATTERED_ACOLYTE: CardRecord = CardRecord::new(
    "Shattered Acolyte",
    "dc0517ca-b271-49a1-a286-c20f4e5b9309",
    "Ashly Lovett",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dwarf", "Warlock"], 2, 2).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or \
             enchantment.",
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

// SOS 32 — Soaring Stoneglider
pub(in crate::card::sets) static SOARING_STONEGLIDER: CardRecord = CardRecord::new(
    "Soaring Stoneglider",
    "2fbe446b-60fd-43da-8358-985392293af8",
    "Pauline Voss",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Elephant", "Cleric"], 4, 3).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile two cards \
             from your graveyard or pay {1}{W}.",
            &[],
            CostDef::Choice(&[
                CostDef::Exile {
                    object: ObjectPredicateDef::Any,
                    from: ZoneKind::Graveyard,
                    quantity: CostQuantityDef::Fixed(2),
                },
                CostDef::Mana(mana_cost!("{1}{W}")),
            ]),
            EffectDef::None,
        ),
        abilities::flying(),
        abilities::vigilance(),
    ]),
);

// SOS 33 — Spiritcall Enthusiast // Scrollboost
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SPIRITCALL_ENTHUSIAST: CardRecord = CardRecord::new(
    "Spiritcall Enthusiast // Scrollboost",
    "c0b85569-2cb3-4b64-b0fe-418195c4dab0",
    "Oriana Menendez",
    CardRules::unsupported(),
);

// SOS 34 — Stand Up for Yourself
pub(in crate::card::sets) static STAND_UP_FOR_YOURSELF: CardRecord = CardRecord::new(
    "Stand Up for Yourself",
    "b756ca13-b904-4510-9bbb-5bc2864abfbd",
    "Nathaniel Himawan",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature with power 3 or greater.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerAtLeast(3),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )]),
);

// SOS 35 — Stirring Hopesinger
pub(in crate::card::sets) static STIRRING_HOPESINGER: CardRecord = CardRecord::new(
    "Stirring Hopesinger",
    "21375667-b318-47f8-a482-9c8c2b5b14c0",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Bird", "Bard"], 1, 3).with_abilities(&[
        abilities::flying(),
        abilities::lifelink(),
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, put a +1/+1 counter on each creature you \
             control.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
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

// SOS 36 — Stone Docent
pub(in crate::card::sets) static STONE_DOCENT: CardRecord = CardRecord::new(
    "Stone Docent",
    "c2abfffb-bf36-44af-9a27-6e109e4d77dd",
    "Inkognit",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit", "Chimera"], 3, 1).with_abilities(&[
        AbilityDef::activated(
            "{W}, Exile this card from your graveyard: You gain 2 life. \
             Surveil 1. Activate only as a sorcery. (Look at the top card \
             of your library. You may put it into your graveyard.)",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::ExileSource],
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                abilities::surveil(ValueDef::Constant(1)),
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// SOS 37 — Summoned Dromedary
pub(in crate::card::sets) static SUMMONED_DROMEDARY: CardRecord = CardRecord::new(
    "Summoned Dromedary",
    "44d0277c-ca82-4334-a15a-cd67a9db0d02",
    "Craig J Spearing",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit", "Camel"], 4, 3).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated(
            "{1}{W}: Return this card from your graveyard to your hand. \
             Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// SOS 38 — Banishing Betrayal
pub(in crate::card::sets) static BANISHING_BETRAYAL: CardRecord = CardRecord::new(
    "Banishing Betrayal",
    "1ae9d2f3-7a9f-433a-aa1f-14337ae6f9d4",
    "Craig Elliott",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target nonland permanent to its owner's hand. Surveil \
         1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            abilities::surveil(ValueDef::Constant(1)),
        ]),
    )]),
);

// SOS 39 — Brush Off
// Audit: unsupported — Needs a full {1}{U} self-cost reduction gated by the selected spell target being an instant or sorcery; the current self-cost reader lacks selected-target context and colored reductions.
pub(in crate::card::sets) static BRUSH_OFF: CardRecord = CardRecord::new(
    "Brush Off",
    "151eab82-d20f-433b-b3bb-1d44e2871d5c",
    "Genel Jumalon",
    CardRules::unsupported(),
);

// SOS 40 — Campus Composer // Aqueous Aria
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static CAMPUS_COMPOSER: CardRecord = CardRecord::new(
    "Campus Composer // Aqueous Aria",
    "fac8ac39-ecb4-4142-bf37-131c65660a9b",
    "Madeline Boni",
    CardRules::unsupported(),
);

// SOS 41 — Chase Inspiration
pub(in crate::card::sets) static CHASE_INSPIRATION: CardRecord = CardRecord::new(
    "Chase Inspiration",
    "06f9f257-c7ef-44b7-8b2b-f038fba900af",
    "Evyn Fong",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +0/+3 and gains hexproof \
         until end of turn. (It can't be the target of spells or \
         abilities your opponents control.)",
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
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
                AppliedEffectDef::add_ability(&abilities::hexproof()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SOS 42 — Deluge Virtuoso
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static DELUGE_VIRTUOSO: CardRecord = CardRecord::new(
    "Deluge Virtuoso",
    "2e3b16ed-8727-48fd-8b1f-c0cbd329385e",
    "Justine Cruz",
    CardRules::unsupported(),
);

// SOS 43 — Divergent Equation
pub(in crate::card::sets) static DIVERGENT_EQUATION: CardRecord = CardRecord::new(
    "Divergent Equation",
    "26295e25-f1bf-4665-ba00-dad35c49bbc2",
    "Inkognit",
    CardRules::new_instant(mana_cost!("{X}{X}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return up to X target instant and/or sorcery cards from your \
             graveyard to your hand.\nExile Divergent Equation.",
            &[AbilityTargetDef::up_to_chosen_x(
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
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ]),
);

// SOS 44 — Echocasting Symposium
// Audit: unsupported — Needs per-player first-resolution history keyed by spell name and the persistent paradigm permission offering a free copy from exile at each first main phase.
pub(in crate::card::sets) static ECHOCASTING_SYMPOSIUM: CardRecord = CardRecord::new(
    "Echocasting Symposium",
    "5d7086a7-dc42-468a-a2cf-a6f89030f947",
    "Fajareka Setiawan",
    CardRules::unsupported(),
);

// SOS 45 — Emeritus of Ideation // Ancestral Recall
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static EMERITUS_OF_IDEATION: CardRecord = CardRecord::new(
    "Emeritus of Ideation // Ancestral Recall",
    "75961d36-acf6-425f-9698-0bf52af74f31",
    "Evyn Fong",
    CardRules::unsupported(),
);

// SOS 46 — Encouraging Aviator // Jump
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static ENCOURAGING_AVIATOR: CardRecord = CardRecord::new(
    "Encouraging Aviator // Jump",
    "72654b84-9902-41db-92ab-a3499c31221c",
    "Oriana Menendez",
    CardRules::unsupported(),
);

// SOS 47 — Essence Scatter (reprint)
const ESSENCE_SCATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m10::ESSENCE_SCATTER,
    "32840097-0531-4c43-b6a8-e76c17420b04",
    "Elliot Lang",
);

// SOS 48 — Exhibition Tidecaller
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static EXHIBITION_TIDECALLER: CardRecord = CardRecord::new(
    "Exhibition Tidecaller",
    "a58c364e-d0c5-41b9-8c8b-2e5a99468cc7",
    "Tulio Brito",
    CardRules::unsupported(),
);

// SOS 49 — Flow State
pub(in crate::card::sets) static FLOW_STATE: CardRecord = CardRecord::new(
    "Flow State",
    "47d6093b-b1b6-4956-8bfd-02cce899f832",
    "Genel Jumalon",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell(
        "Look at the top three cards of your library. Put one of them \
         into your hand and the rest on the bottom of your library in \
         any order. If there is an instant card and a sorcery card in \
         your graveyard, instead put two of them into your hand and \
         the rest on the bottom of your library in any order.",
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::All(&[
                TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Instant),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
            ]),
            then: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(3),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::Any,
                minimum: 2,
                maximum: 2,
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
                chosen: crate::Binding!("otherwise_chosen"),
                remainder: crate::Binding!("otherwise_rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "otherwise_chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("otherwise_rest")),
                        actor: PlayerRefDef::EffectController,
                        ordered: crate::Binding!("otherwise_ordered"),
                        placement: ZonePlacement::Bottom,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "otherwise_ordered"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        },
    )]),
);

// SOS 50 — Fractal Anomaly
pub(in crate::card::sets) static FRACTAL_ANOMALY: CardRecord = CardRecord::new(
    "Fractal Anomaly",
    "e1975a61-aef0-49a6-a6d6-c3a37e2e2b22",
    "Steve Ellis",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell(
        "Create a 0/0 green and blue Fractal creature token and put X \
         +1/+1 counters on it, where X is the number of cards you've \
         drawn this turn.",
        EffectDef::create_creature_token(&["Fractal"], &[ManaColor::Green, ManaColor::Blue], 0, 0)
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("fractals"),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "fractals"
                    ))),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CardsDrawnThisTurn(PlayerRelation::You),
                },
            }),
    )]),
);

// SOS 51 — Fractalize
pub(in crate::card::sets) static FRACTALIZE: CardRecord = CardRecord::new(
    "Fractalize",
    "e3c3b19b-01b6-4f5a-b428-513b778c5d89",
    "Andrew Mar",
    CardRules::new_instant(mana_cost!("{X}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature becomes a green and blue \
         Fractal with base power and toughness each equal to X plus 1. \
         (It loses all other colors and creature types.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Sum(&SumValueDef {
                        left: ValueDef::ChosenX,
                        right: ValueDef::Constant(1),
                    }),
                    ValueDef::Sum(&SumValueDef {
                        left: ValueDef::ChosenX,
                        right: ValueDef::Constant(1),
                    }),
                ),
                AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Fractal"])),
                AppliedEffectDef::set_colors(ColorSet::from_colors(&[
                    ManaColor::Green,
                    ManaColor::Blue,
                ])),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SOS 52 — Harmonized Trio // Brainstorm
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static HARMONIZED_TRIO: CardRecord = CardRecord::new(
    "Harmonized Trio // Brainstorm",
    "617208ff-dd9b-44fd-a740-d3188081e5cc",
    "Marie Magny",
    CardRules::unsupported(),
);

// SOS 53 — Homesickness
pub(in crate::card::sets) static HOMESICKNESS: CardRecord = CardRecord::new(
    "Homesickness",
    "6e4a1f82-b0b1-4608-91f8-130bee731435",
    "Caroline Gariba",
    CardRules::new_instant(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player draws two cards. Tap up to two target \
             creatures. Put a stun counter on each of them. (If a \
             permanent with a stun counter would become untapped, remove \
             one from it instead.)",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// SOS 54 — Hydro-Channeler
pub(in crate::card::sets) static HYDRO_CHANNELER: CardRecord = CardRecord::new(
    "Hydro-Channeler",
    "099f8400-d70a-48ef-8ff6-645eae97e072",
    "Mila Pesic",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Wizard"], 1, 3).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {U}. Spend this mana only to cast an instant or \
             sorcery spell.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue).with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ])),
            ])),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color. Spend this mana only to \
             cast an instant or sorcery spell.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ])),
            ])),
        ),
    ]),
);

// SOS 55 — Jadzi, Steward of Fate // Oracle's Gift
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static JADZI_STEWARD_OF_FATE: CardRecord = CardRecord::new(
    "Jadzi, Steward of Fate // Oracle's Gift",
    "a95b6baf-01e6-49c3-9a26-394b127d53c3",
    "Martina Fačková",
    CardRules::unsupported(),
);

// SOS 56 — Landscape Painter // Vibrant Idea
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static LANDSCAPE_PAINTER: CardRecord = CardRecord::new(
    "Landscape Painter // Vibrant Idea",
    "c0bd30c4-3cdf-4eda-8be5-0fb5e5ddddbf",
    "Vincent Christiaens",
    CardRules::unsupported(),
);

// SOS 57 — Mana Sculpt
// Audit: unsupported — Needs the targeted spell's actual mana payment total, frozen before countering it and retained for the delayed next-main-phase mana trigger.
pub(in crate::card::sets) static MANA_SCULPT: CardRecord = CardRecord::new(
    "Mana Sculpt",
    "200c8e3d-c53b-40c7-a29a-fccc1281bfc6",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// SOS 58 — Mathemagics
// Audit: unsupported — Needs exponentiation of chosen X to compute the draw amount; existing scalar arithmetic does not express 2 to the power X.
pub(in crate::card::sets) static MATHEMAGICS: CardRecord = CardRecord::new(
    "Mathemagics",
    "cd3cc172-5609-4bc8-9d84-50680fed6df9",
    "Liiga Smilshkalne",
    CardRules::unsupported(),
);

// SOS 59 — Matterbending Mage
// Audit: unsupported — Needs a predicate for an X symbol in a card or spell's mana cost, including casts with X equal to zero; chosen X and mana value do not answer that question.
pub(in crate::card::sets) static MATTERBENDING_MAGE: CardRecord = CardRecord::new(
    "Matterbending Mage",
    "460c6afd-cddf-4fea-925f-b27517ff250a",
    "Flavio Greco Paglia",
    CardRules::unsupported(),
);

// SOS 60 — Muse Seeker
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static MUSE_SEEKER: CardRecord = CardRecord::new(
    "Muse Seeker",
    "71cb4a6b-b500-4b28-bcdb-ec4188242f39",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// SOS 61 — Muse's Encouragement
pub(in crate::card::sets) static MUSE_S_ENCOURAGEMENT: CardRecord = CardRecord::new(
    "Muse's Encouragement",
    "c59e0004-1d6a-42a1-8ce4-31da2af2e1bf",
    "Oriana Menendez",
    CardRules::new_instant(mana_cost!("{4}{U}")).with_abilities(&[AbilityDef::spell(
        "Create a 3/3 blue and red Elemental creature token with \
         flying.\nSurveil 2. (Look at the top two cards of your \
         library, then put any number of them into your graveyard and \
         the rest on top of your library in any order.)",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(
                &["Elemental"],
                &[ManaColor::Blue, ManaColor::Red],
                3,
                3,
            )
            .with_abilities(&[abilities::flying()]),
            abilities::surveil(ValueDef::Constant(2)),
        ]),
    )]),
);

// SOS 62 — Orysa, Tide Choreographer
// Audit: unsupported — Needs a self-cost condition reading the aggregate toughness of creatures you control; the self-cost evaluator does not evaluate aggregate object statistics.
pub(in crate::card::sets) static ORYSA_TIDE_CHOREOGRAPHER: CardRecord = CardRecord::new(
    "Orysa, Tide Choreographer",
    "010ed379-63f5-452c-9cd4-00d51647c0e3",
    "Anna Pavleeva",
    CardRules::unsupported(),
);

// SOS 63 — Pensive Professor
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana. Also needs a grouped one-or-more-counters-added event.
pub(in crate::card::sets) static PENSIVE_PROFESSOR: CardRecord = CardRecord::new(
    "Pensive Professor",
    "66d47940-84f9-4479-8562-45e5148435d4",
    "Billy Christian",
    CardRules::unsupported(),
);

// SOS 64 — Procrastinate
pub(in crate::card::sets) static PROCRASTINATE: CardRecord = CardRecord::new(
    "Procrastinate",
    "1edb449d-620f-4e21-9d76-2c840635eb9d",
    "Elizabeth Peiró",
    CardRules::new_sorcery(mana_cost!("{X}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tap target creature. Put twice X stun counters on it. (If a \
         permanent with a stun counter would become untapped, remove \
         one from it instead.)",
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
                amount: ValueDef::Scaled(&ScaledValueDef {
                    value: ValueDef::ChosenX,
                    factor: 2,
                }),
            },
        ]),
    )]),
);

// SOS 65 — Quick Study (reprint)
const QUICK_STUDY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_woe::QUICK_STUDY,
    "2d4f0bc7-da7c-4749-a24c-b01f3eb5860c",
    "Izzy",
);

// SOS 66 — Run Behind
// Audit: unsupported — Needs the spell's selected target characteristics in its self-cost-reduction condition; the current self-cost reader cannot condition a discount on a tapped or attacking target.
pub(in crate::card::sets) static RUN_BEHIND: CardRecord = CardRecord::new(
    "Run Behind",
    "40ecc34b-4cd0-4998-bbf4-7faa6fd3d7e0",
    "Nereida",
    CardRules::unsupported(),
);

// SOS 67 — Skycoach Conductor // All Aboard
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SKYCOACH_CONDUCTOR: CardRecord = CardRecord::new(
    "Skycoach Conductor // All Aboard",
    "4ecbca71-9a1d-44c5-b709-d6f565941d5e",
    "Christina Kraus",
    CardRules::unsupported(),
);

// SOS 68 — Spellbook Seeker
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SPELLBOOK_SEEKER: CardRecord = CardRecord::new(
    "Spellbook Seeker",
    "cc44eaa4-59a4-419e-b1d1-d92f354ff588",
    "Scott Murphy",
    CardRules::unsupported(),
);

// SOS 69 — Tester of the Tangential
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana. Also needs an optional resolving X-mana payment with a retained X and a reflexive targeted counter-transfer trigger.
pub(in crate::card::sets) static TESTER_OF_THE_TANGENTIAL: CardRecord = CardRecord::new(
    "Tester of the Tangential",
    "bbd708ec-eef4-4f45-99dd-60e1cec4b991",
    "Lorenzo Mastroianni",
    CardRules::unsupported(),
);

// SOS 70 — Textbook Tabulator
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana.
pub(in crate::card::sets) static TEXTBOOK_TABULATOR: CardRecord = CardRecord::new(
    "Textbook Tabulator",
    "56f54fee-b48d-4582-8982-ca4c7b8ef553",
    "Josiah \"Jo\" Cameron",
    CardRules::unsupported(),
);

// SOS 71 — Wisdom of Ages
pub(in crate::card::sets) static WISDOM_OF_AGES: CardRecord = CardRecord::new(
    "Wisdom of Ages",
    "b227ef04-33e4-44e8-a357-0ea3dfe5d49b",
    "Jabari Weathers",
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}{U}")).with_abilities(&[AbilityDef::spell(
        "Return all instant and sorcery cards from your graveyard to \
         your hand. You have no maximum hand size for the rest of the \
         game.\nExile Wisdom of Ages.",
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    PlayerRuleDef::NoMaximumHandSize,
                )),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ]),
    )
    .with_resolution_destination(SpellResolutionDestinationDef::Exile)]),
);

// SOS 72 — Adventurous Eater // Have a Bite
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static ADVENTUROUS_EATER: CardRecord = CardRecord::new(
    "Adventurous Eater // Have a Bite",
    "d40cc7da-c731-418e-8547-7033d1939450",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// SOS 73 — Arcane Omens
pub(in crate::card::sets) static ARCANE_OMENS: CardRecord = CardRecord::new(
    "Arcane Omens",
    "d357d997-9d4e-4ade-81f2-37629853f13a",
    "Antonio José Manzanedo",
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Converge — Target player discards X cards, where X is the \
         number of colors of mana spent to cast this spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ColorsOfManaSpent,
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )]),
);

// SOS 74 — Arnyn, Deathbloom Botanist
pub(in crate::card::sets) static ARNYN_DEATHBLOOM_BOTANIST: CardRecord = CardRecord::new(
    "Arnyn, Deathbloom Botanist",
    "6168b472-0930-4db5-9920-407340b99050",
    "Anna Steinbauer",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Druid"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered_with_targets(
                "Whenever a creature you control with power or toughness 1 or \
                 less dies, target opponent loses 2 life and you gain 2 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::PowerLessThan(ValueDef::Constant(2)),
                                ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(2)),
                            ]),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// SOS 75 — Burrog Banemaker
pub(in crate::card::sets) static BURROG_BANEMAKER: CardRecord = CardRecord::new(
    "Burrog Banemaker",
    "3e4f9d23-1a17-4188-ac91-f8ddea46a1c4",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{B}"), &["Frog", "Warlock"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::activated(
            "{1}{B}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
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

// SOS 76 — Cheerful Osteomancer // Raise Dead
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static CHEERFUL_OSTEOMANCER: CardRecord = CardRecord::new(
    "Cheerful Osteomancer // Raise Dead",
    "3c34660c-25e3-4ff5-9b2b-5554ded2bcc3",
    "Diego Gisbert",
    CardRules::unsupported(),
);

// SOS 77 — Cost of Brilliance
pub(in crate::card::sets) static COST_OF_BRILLIANCE: CardRecord = CardRecord::new(
    "Cost of Brilliance",
    "3a46816b-9f75-4c72-9ec6-cded6a4a0d01",
    "Darren Tan",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws two cards and loses 2 life. Put a +1/+1 \
         counter on up to one target creature.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex(1)),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// SOS 78 — Decorum Dissertation
// Audit: unsupported — Needs per-player first-resolution history keyed by spell name and the persistent paradigm permission offering a free copy from exile at each first main phase.
pub(in crate::card::sets) static DECORUM_DISSERTATION: CardRecord = CardRecord::new(
    "Decorum Dissertation",
    "f4ab2d9b-c73d-478d-aac7-4d3bb24296d2",
    "Mila Pesic",
    CardRules::unsupported(),
);

// SOS 79 — Dissection Practice
pub(in crate::card::sets) static DISSECTION_PRACTICE: CardRecord = CardRecord::new(
    "Dissection Practice",
    "ddbf1242-6832-475e-9a77-65dd9b4bb32a",
    "Steve Ellis",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target opponent loses 1 life and you gain 1 life.\nUp to one \
         target creature gets +1/+1 until end of turn.\nUp to one \
         target creature gets -1/-1 until end of turn.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent)),
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
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex(2)),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// SOS 80 — Emeritus of Woe // Demonic Tutor
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static EMERITUS_OF_WOE: CardRecord = CardRecord::new(
    "Emeritus of Woe // Demonic Tutor",
    "7eb9e83d-515d-4911-a06b-9982200277b2",
    "Jodie Muir",
    CardRules::unsupported(),
);

// SOS 81 — End of the Hunt
// Audit: unsupported — Needs a choice candidate set restricted to creatures and planeswalkers with the greatest mana value among those controlled by the targeted opponent; aggregate extrema are not available in choice-target predicate evaluation.
pub(in crate::card::sets) static END_OF_THE_HUNT: CardRecord = CardRecord::new(
    "End of the Hunt",
    "0809b51a-6a05-4f18-9bf4-1b8382da648f",
    "Alexandre Honoré",
    CardRules::unsupported(),
);

// SOS 82 — Eternal Student
pub(in crate::card::sets) static ETERNAL_STUDENT: CardRecord = CardRecord::new(
    "Eternal Student",
    "f97fac69-3e77-4150-9702-cc726daa6d21",
    "Ksenia Kim",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Warlock"], 4, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{B}, Exile this card from your graveyard: Create two 1/1 \
             white and black Inkling creature tokens with flying.",
            &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::ExileSource],
            EffectDef::create_creature_token(
                &["Inkling"],
                &[ManaColor::White, ManaColor::Black],
                1,
                1,
            )
            .with_count(ValueDef::Constant(2))
            .with_abilities(&[abilities::flying()]),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// SOS 83 — Foolish Fate
pub(in crate::card::sets) static FOOLISH_FATE: CardRecord = CardRecord::new(
    "Foolish Fate",
    "d278f4c9-d20b-4a76-8c5c-4d3e985948b9",
    "Danny Schwartz",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature.\nInfusion — If you gained life this \
         turn, that creature's controller loses 3 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::LoseLife {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            },
        ]),
    )]),
);

// SOS 84 — Forum Necroscribe
pub(in crate::card::sets) static FORUM_NECROSCRIBE: CardRecord = CardRecord::new(
    "Forum Necroscribe",
    "67504a12-7414-4209-bf1c-624b4db19d52",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Troll", "Warlock"], 5, 4).with_abilities(&[
        abilities::ward(&[CostDef::DiscardCards(1)], "Ward—Discard a card."),
        AbilityDef::triggered_with_targets(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, return target creature card from your \
             graveyard to the battlefield.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
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
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// SOS 85 — Grave Researcher // Reanimate
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static GRAVE_RESEARCHER: CardRecord = CardRecord::new(
    "Grave Researcher // Reanimate",
    "8b1e10e8-ea14-4761-910b-4072e2a18456",
    "Izzy",
    CardRules::unsupported(),
);

// SOS 86 — Last Gasp (reprint)
const LAST_GASP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::LAST_GASP,
    "da5f3729-6ec7-4482-90cb-83b973edeae4",
    "Madeline Boni",
);

// SOS 87 — Lecturing Scornmage
pub(in crate::card::sets) static LECTURING_SCORNMAGE: CardRecord = CardRecord::new(
    "Lecturing Scornmage",
    "ad07091e-8c24-43af-8ce8-031847bcaf30",
    "Tuan Duong Chu",
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Warlock"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, put a +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
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

// SOS 88 — Leech Collector // Bloodletting
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static LEECH_COLLECTOR: CardRecord = CardRecord::new(
    "Leech Collector // Bloodletting",
    "c715fe4c-c0e7-4342-811f-b74687851097",
    "Chris Rallis",
    CardRules::unsupported(),
);

// SOS 89 — Masterful Flourish
pub(in crate::card::sets) static MASTERFUL_FLOURISH: CardRecord = CardRecord::new(
    "Masterful Flourish",
    "8d93d14d-7760-4af2-a237-2df326dd28d3",
    "Monztre",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+0 and gains \
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
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::indestructible()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SOS 90 — Melancholic Poet
pub(in crate::card::sets) static MELANCHOLIC_POET: CardRecord = CardRecord::new(
    "Melancholic Poet",
    "d8309815-7035-47a5-acf2-2b2ac1e65037",
    "Elizabeth Peiró",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Elf", "Bard"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, each opponent loses 1 life and you gain 1 \
             life.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
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

// SOS 91 — Moseo, Vein's New Dean
// Audit: unsupported — Needs life gained this turn in graveyard target-predicate evaluation; the turn tally is available to effect and condition evaluation, but value_from_source cannot supply this trigger's mana-value limit.
pub(in crate::card::sets) static MOSEO_VEIN_S_NEW_DEAN: CardRecord = CardRecord::new(
    "Moseo, Vein's New Dean",
    "6877180c-22a1-4c4d-9178-316f4c34661b",
    "Abz J Harding",
    CardRules::unsupported(),
);

// SOS 92 — Poisoner's Apprentice
pub(in crate::card::sets) static POISONER_S_APPRENTICE: CardRecord = CardRecord::new(
    "Poisoner's Apprentice",
    "3755a2e9-af55-4625-a006-2a86c7893a96",
    "Kieran Yanner",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Orc", "Warlock"], 2, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "Infusion — When this creature enters, target creature an \
             opponent controls gets -4/-4 until end of turn if you gained \
             life this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-4),
                        ValueDef::Constant(-4),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// SOS 93 — Postmortem Professor
// Audit: unsupported — Needs payment of an exile-other-graveyard-card cost on a graveyard activation; MoveToZone costs are currently offered only for battlefield sources.
pub(in crate::card::sets) static POSTMORTEM_PROFESSOR: CardRecord = CardRecord::new(
    "Postmortem Professor",
    "174f5d7e-5d36-4d13-96bf-9b12cd644716",
    "Nino Vecia",
    CardRules::unsupported(),
);

// SOS 94 — Pox Plague
// Audit: unsupported — Needs per-affected-player quantities for simultaneous life loss and APNAP discard/sacrifice selections; ChooseForEachPlayer currently evaluates its amount in one fixed effect-controller context.
pub(in crate::card::sets) static POX_PLAGUE: CardRecord = CardRecord::new(
    "Pox Plague",
    "9c99c17b-ad3a-4859-97e8-469718b81cd9",
    "Camille Alquier",
    CardRules::unsupported(),
);

// SOS 95 — Pull from the Grave
pub(in crate::card::sets) static PULL_FROM_THE_GRAVE: CardRecord = CardRecord::new(
    "Pull from the Grave",
    "d73612fe-8992-4650-a7e3-c7b662da6a03",
    "Pauline Voss",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return up to two target creature cards from your graveyard to \
         your hand. You gain 2 life.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
            2,
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// SOS 96 — Rabid Attack
pub(in crate::card::sets) static RABID_ATTACK: CardRecord = CardRecord::new(
    "Rabid Attack",
    "f5e67560-3135-4b27-a344-5859edf8bcd9",
    "Izzy",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, any number of target creatures you control \
         each get +1/+0 and gain \"When this creature dies, draw a \
         card.\"",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
            255,
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::dies_trigger(
                    "When this creature dies, draw a card.",
                    abilities::draw_cards(ValueDef::Constant(1)),
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SOS 97 — Ral Zarek, Guest Lecturer
// Audit: unsupported — Needs a multi-coin-flip result counting heads and a player skip-next-N-turns effect driven by that result.
pub(in crate::card::sets) static RAL_ZAREK_GUEST_LECTURER: CardRecord = CardRecord::new(
    "Ral Zarek, Guest Lecturer",
    "8fbad757-4081-42f7-a460-68ac03e77510",
    "Billy Christian",
    CardRules::unsupported(),
);

// SOS 98 — Scathing Shadelock // Venomous Words
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SCATHING_SHADELOCK: CardRecord = CardRecord::new(
    "Scathing Shadelock // Venomous Words",
    "03e664cd-c3a6-4263-b2d8-dd99058fb8ec",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// SOS 99 — Scheming Silvertongue // Sign in Blood
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SCHEMING_SILVERTONGUE: CardRecord = CardRecord::new(
    "Scheming Silvertongue // Sign in Blood",
    "fe85a124-0d8b-4a29-8df1-65888a39147f",
    "Anna Steinbauer",
    CardRules::unsupported(),
);

// SOS 100 — Send in the Pest
pub(in crate::card::sets) static SEND_IN_THE_PEST: CardRecord = CardRecord::new(
    "Send in the Pest",
    "283b508b-89f0-4c23-9686-b049e402b73c",
    "Raluca Marinescu",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell(
        "Each opponent discards a card. You create a 1/1 black and \
         green Pest creature token with \"Whenever this token attacks, \
         you gain 1 life.\"",
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::create_creature_token(
                &["Pest"],
                &[ManaColor::Black, ManaColor::Green],
                1,
                1,
            )
            .with_abilities(&[AbilityDef::triggered(
                "Whenever this token attacks, you gain 1 life.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            )]),
        ]),
    )]),
);

// SOS 101 — Sneering Shadewriter
pub(in crate::card::sets) static SNEERING_SHADEWRITER: CardRecord = CardRecord::new(
    "Sneering Shadewriter",
    "4b4c120e-abe9-4f11-a7e4-bc3f723da4b2",
    "Mila Pesic",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Warlock"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, each opponent loses 2 life and you \
             gain 2 life.",
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// SOS 102 — Tragedy Feaster
pub(in crate::card::sets) static TRAGEDY_FEASTER: CardRecord = CardRecord::new(
    "Tragedy Feaster",
    "b93cbaad-8ed8-4a1d-b95a-20a616dfedc9",
    "Raph Lomotan",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Demon"], 7, 6).with_abilities(&[
        abilities::trample(),
        abilities::ward(&[CostDef::DiscardCards(1)], "Ward—Discard a card."),
        AbilityDef::triggered(
            "Infusion — At the beginning of your end step, sacrifice a \
             permanent unless you gained life this turn.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ValueComparison(
                    &ValueComparisonDef {
                        left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    },
                )),
                then: &EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Controller,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            },
        ),
    ]),
);

// SOS 103 — Ulna Alley Shopkeep
pub(in crate::card::sets) static ULNA_ALLEY_SHOPKEEP: CardRecord = CardRecord::new(
    "Ulna Alley Shopkeep",
    "c25e1ae5-f17c-4eee-98f1-5681981af31c",
    "Inkognit",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Goblin", "Warlock"], 2, 3).with_abilities(&[
        abilities::menace(),
        AbilityDef::static_ability(
            "Infusion — This creature gets +2/+0 as long as you gained \
             life this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
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

// SOS 104 — Wander Off
pub(in crate::card::sets) static WANDER_OFF: CardRecord = CardRecord::new(
    "Wander Off",
    "3d409512-50b9-4a38-91b0-19ba25227992",
    "Elliot Lang",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )]),
);

// SOS 105 — Withering Curse
pub(in crate::card::sets) static WITHERING_CURSE: CardRecord = CardRecord::new(
    "Withering Curse",
    "50aaf618-dddb-4cbe-8231-d634b4498563",
    "Tuan Duong Chu",
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -2/-2 until end of turn.\nInfusion — If you \
         gained life this turn, destroy all creatures instead.",
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                comparison: ComparisonDef::Greater,
                right: ValueDef::Constant(0),
            }),
            then: &EffectDef::Destroy {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ))),
                then: None,
            },
            otherwise: &EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        },
    )]),
);

// SOS 106 — Ancestral Anger (reprint)
const ANCESTRAL_ANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::ANCESTRAL_ANGER,
    "6c5a93d6-d4ab-4062-bb3c-1b5330bf15ad",
    "Gonzalo Kenny",
);

// SOS 107 — Archaic's Agony
// Audit: unsupported — Needs exile-play permission expiring at the end of the specified player's next turn; the existing next-turn permission expires after the following opponent turn.
pub(in crate::card::sets) static ARCHAIC_S_AGONY: CardRecord = CardRecord::new(
    "Archaic's Agony",
    "8d99f8b2-5c1c-4059-bf68-c6b2e9e5b275",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// SOS 108 — Artistic Process
pub(in crate::card::sets) static ARTISTIC_PROCESS: CardRecord = CardRecord::new(
    "Artistic Process",
    "bce9d933-be58-4301-beb4-07b04d0b69f0",
    "Mariah Tekulve",
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Artistic Process deals 6 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(6),
                ),
            ),
            AbilityDef::spell(
                "Artistic Process deals 2 damage to each creature you don't \
                 control.",
                EffectDef::damage(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ))),
                    ValueDef::Constant(2),
                ),
            ),
            AbilityDef::spell(
                "Create a 3/3 blue and red Elemental creature token with \
                 flying. It gains haste until end of turn.",
                EffectDef::create_creature_token(
                    &["Elemental"],
                    &[ManaColor::Blue, ManaColor::Red],
                    3,
                    3,
                )
                .with_abilities(&[abilities::flying()])
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("elementals"),
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("elementals"),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                }),
            ),
        ],
    )]),
);

// SOS 109 — Blazing Firesinger // Seething Song
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static BLAZING_FIRESINGER: CardRecord = CardRecord::new(
    "Blazing Firesinger // Seething Song",
    "3ba971e7-0b7a-4750-896f-7cf063e66b2a",
    "Ashly Lovett",
    CardRules::unsupported(),
);

// SOS 110 — Charging Strifeknight
pub(in crate::card::sets) static CHARGING_STRIFEKNIGHT: CardRecord = CardRecord::new(
    "Charging Strifeknight",
    "9940d992-1ba1-40ec-9b93-17d773452c4b",
    "Antonio José Manzanedo",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit", "Knight"], 3, 3).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated(
            "{T}, Discard a card: Draw a card.",
            &[
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// SOS 111 — Choreographed Sparks
// Audit: unsupported — Needs a spell-cannot-be-copied rule and ability grants to a copied creature spell that continue onto its resulting token, including the end-step sacrifice trigger.
pub(in crate::card::sets) static CHOREOGRAPHED_SPARKS: CardRecord = CardRecord::new(
    "Choreographed Sparks",
    "0cda4235-4dce-48fe-a8a5-2a952dedbe25",
    "Paolo Parente",
    CardRules::unsupported(),
);

// SOS 112 — Duel Tactics
pub(in crate::card::sets) static DUEL_TACTICS: CardRecord = CardRecord::new(
    "Duel Tactics",
    "8f3a1675-0cc7-4dfd-a12e-4740a2cf81e8",
    "Craig J Spearing",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Duel Tactics deals 1 damage to target creature. It can't \
             block this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{1}{R}"))]),
    ]),
);

// SOS 113 — Emeritus of Conflict // Lightning Bolt
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static EMERITUS_OF_CONFLICT: CardRecord = CardRecord::new(
    "Emeritus of Conflict // Lightning Bolt",
    "f58dba4f-1abb-47a3-a684-29c32bab95c0",
    "Alix Branwyn",
    CardRules::unsupported(),
);

// SOS 114 — Expressive Firedancer
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static EXPRESSIVE_FIREDANCER: CardRecord = CardRecord::new(
    "Expressive Firedancer",
    "259b8c45-6241-4206-a34e-34c7f401f47b",
    "Billy Christian",
    CardRules::unsupported(),
);

// SOS 115 — Flashback
pub(in crate::card::sets) static FLASHBACK: CardRecord = CardRecord::new(
    "Flashback",
    "1b832fda-d7c4-4566-884c-2a8b6da15488",
    "Flavio Greco Paglia",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target instant or sorcery card in your graveyard gains \
         flashback until end of turn. The flashback cost is equal to \
         its mana cost. (You may cast that card from your graveyard \
         for its flashback cost. Then exile it.)",
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
    )]),
);

// SOS 116 — Garrison Excavator
// Audit: unsupported — Needs a grouped one-or-more-cards-leave-your-graveyard event; the current per-card zone-change event would trigger separately for cards moved together.
pub(in crate::card::sets) static GARRISON_EXCAVATOR: CardRecord = CardRecord::new(
    "Garrison Excavator",
    "f11d2846-f181-4751-82ac-1e1ced6f46c7",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// SOS 117 — Goblin Glasswright // Craft with Pride
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static GOBLIN_GLASSWRIGHT: CardRecord = CardRecord::new(
    "Goblin Glasswright // Craft with Pride",
    "c85c5f06-dd31-4e2c-97be-2f64d65069ea",
    "David Auden Nash",
    CardRules::unsupported(),
);

// SOS 118 — Heated Argument
pub(in crate::card::sets) static HEATED_ARGUMENT: CardRecord = CardRecord::new(
    "Heated Argument",
    "0038d212-3d95-4f98-8c2e-7b2404d0ced7",
    "Aleksi Briclot",
    CardRules::new_instant(mana_cost!("{4}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Heated Argument deals 6 damage to target creature. You may \
         exile a card from your graveyard. If you do, Heated Argument \
         also deals 2 damage to that creature's controller.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(6),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[crate::card::actions::choose_exile_from_graveyard(1).as_cost()],
                &EffectDef::damage(
                    EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            )),
        ]),
    )]),
);

// SOS 119 — Impractical Joke
pub(in crate::card::sets) static IMPRACTICAL_JOKE: CardRecord = CardRecord::new(
    "Impractical Joke",
    "39a816b4-39b8-421c-b828-68db901d34b7",
    "Caroline Gariba",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Damage can't be prevented this turn. Impractical Joke deals 3 \
         damage to up to one target creature or planeswalker.",
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
        EffectDef::Sequence(&[
            EffectDef::DamageCannotBePreventedThisTurn,
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ]),
    )]),
);

// SOS 120 — Improvisation Capstone
// Audit: unsupported — Needs per-player first-resolution history keyed by spell name and the persistent paradigm permission offering a free copy from exile at each first main phase. Also needs revealing/exiling until a cumulative mana-value threshold, followed by free casting any subset of the exiled spells.
pub(in crate::card::sets) static IMPROVISATION_CAPSTONE: CardRecord = CardRecord::new(
    "Improvisation Capstone",
    "d01fe6e9-49ee-4708-833e-75cd5a9f167c",
    "Marta Nael",
    CardRules::unsupported(),
);

// SOS 121 — Living History
pub(in crate::card::sets) static LIVING_HISTORY: CardRecord = CardRecord::new(
    "Living History",
    "2028792c-fd60-40d4-bff7-3b82dbe1ffb5",
    "Caroline Gariba",
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 2/2 red and white \
             Spirit creature token.",
            EffectDef::create_creature_token(
                &["Spirit"],
                &[ManaColor::Red, ManaColor::White],
                2,
                2,
            ),
        ),
        AbilityDef::triggered_if_with_targets(
            "Whenever you attack, if a card left your graveyard this turn, \
             target attacking creature gets +2/+0 until end of turn.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            &TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
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

// SOS 122 — Maelstrom Artisan // Rocket Volley
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static MAELSTROM_ARTISAN: CardRecord = CardRecord::new(
    "Maelstrom Artisan // Rocket Volley",
    "5c88391d-271f-4021-a5d9-158ebc1e6357",
    "Eelis Kyttanen",
    CardRules::unsupported(),
);

// SOS 123 — Magmablood Archaic
// Audit: unsupported — Needs colors of mana spent on the triggering instant or sorcery, retained in the cast-event context; ColorsOfManaSpent on the triggered ability reads that new stack object instead.
pub(in crate::card::sets) static MAGMABLOOD_ARCHAIC: CardRecord = CardRecord::new(
    "Magmablood Archaic",
    "4d611278-9948-4345-b4dd-aa6eaf21b233",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// SOS 124 — Mica, Reader of Ruins
pub(in crate::card::sets) static MICA_READER_OF_RUINS: CardRecord = CardRecord::new(
    "Mica, Reader of Ruins",
    "949ad3ff-9e80-493c-a3ae-146b919bfcd7",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Artificer"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ward(&[CostDef::PayLife(3)], "Ward—Pay 3 life."),
            AbilityDef::triggered(
                "Whenever you cast an instant or sorcery spell, you may \
                 sacrifice an artifact. If you do, copy that spell and you may \
                 choose new targets for the copy.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::PayOr(PayOrDef::optional(
                    &[crate::card::actions::choose_sacrifice(1)
                        .matching(ObjectPredicateDef::HasType(CardType::Artifact))
                        .as_cost()],
                    &EffectDef::CopyStackObject(&CopyStackObjectDef {
                        object: EffectRecipientDef::TriggeringObject,
                        controller: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(1),
                        retarget: true,
                        colors: None,
                    }),
                )),
            ),
        ]),
);

// SOS 125 — Molten-Core Maestro
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static MOLTEN_CORE_MAESTRO: CardRecord = CardRecord::new(
    "Molten-Core Maestro",
    "326dfe32-3674-4a11-acd8-5ba62371235a",
    "Aleksi Briclot",
    CardRules::unsupported(),
);

// SOS 126 — Pigment Wrangler // Striking Palette
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static PIGMENT_WRANGLER: CardRecord = CardRecord::new(
    "Pigment Wrangler // Striking Palette",
    "c2faf4cf-c4b6-4721-ac06-0e045dd9704a",
    "Gonzalo Kenny",
    CardRules::unsupported(),
);

// SOS 127 — Rearing Embermare
pub(in crate::card::sets) static REARING_EMBERMARE: CardRecord = CardRecord::new(
    "Rearing Embermare",
    "06dd56bd-de92-4202-af31-7e881c34d799",
    "Pauline Voss",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Horse", "Beast"], 4, 5)
        .with_abilities(&[abilities::reach(), abilities::haste()]),
);

// SOS 128 — Rubble Rouser
// Audit: unsupported — Needs an immediate mana-ability payment that exiles a selected graveyard card and records completion for the separate reflexive damage trigger.
pub(in crate::card::sets) static RUBBLE_ROUSER: CardRecord = CardRecord::new(
    "Rubble Rouser",
    "afe61957-a9bb-42b0-98e8-b5fa418cbaff",
    "Craig J Spearing",
    CardRules::unsupported(),
);

// SOS 129 — Seize the Spoils (reprint)
const SEIZE_THE_SPOILS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::SEIZE_THE_SPOILS,
    "4ddf4e34-a1f9-4636-942d-0a08e9f94320",
    "Josiah \"Jo\" Cameron",
);

// SOS 130 — Steal the Show
// Audit: unsupported — Needs a zero-to-all discard selection with a follow-up counting the cards actually discarded after replacements; the current discard operation selects a fixed computed quantity.
pub(in crate::card::sets) static STEAL_THE_SHOW: CardRecord = CardRecord::new(
    "Steal the Show",
    "7ac6649f-980e-4404-9c05-458c30578ecc",
    "Pauline Voss",
    CardRules::unsupported(),
);

// SOS 131 — Strife Scholar // Awaken the Ages
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static STRIFE_SCHOLAR: CardRecord = CardRecord::new(
    "Strife Scholar // Awaken the Ages",
    "8de79312-2046-425e-9919-49afe19be81b",
    "Craig J Spearing",
    CardRules::unsupported(),
);

// SOS 132 — Tablet of Discovery
// Audit: unsupported — Needs a one-turn play permission for the specific milled card while it remains in the graveyard, including land plays.
pub(in crate::card::sets) static TABLET_OF_DISCOVERY: CardRecord = CardRecord::new(
    "Tablet of Discovery",
    "13059664-a940-4a66-8100-0c90b884bab4",
    "Craig J Spearing",
    CardRules::unsupported(),
);

// SOS 133 — Tackle Artist
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static TACKLE_ARTIST: CardRecord = CardRecord::new(
    "Tackle Artist",
    "b87e2474-98c1-4c1a-91ed-340b72d31653",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// SOS 134 — Thunderdrum Soloist
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static THUNDERDRUM_SOLOIST: CardRecord = CardRecord::new(
    "Thunderdrum Soloist",
    "590d1d95-ed13-4121-899f-f5a2d8a6617a",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// SOS 135 — Tome Blast
pub(in crate::card::sets) static TOME_BLAST: CardRecord = CardRecord::new(
    "Tome Blast",
    "72a3b17d-1e00-48e9-8402-c81bacd595a7",
    "Filipe Pagliuso",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tome Blast deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{R}"))]),
    ]),
);

// SOS 136 — Unsubtle Mockery
pub(in crate::card::sets) static UNSUBTLE_MOCKERY: CardRecord = CardRecord::new(
    "Unsubtle Mockery",
    "2b7cb1a3-761e-470e-a164-6e29dd9448cd",
    "Joe Slucher",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Unsubtle Mockery deals 4 damage to target creature. Surveil \
         1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
            abilities::surveil(ValueDef::Constant(1)),
        ]),
    )]),
);

// SOS 137 — Zealous Lorecaster
pub(in crate::card::sets) static ZEALOUS_LORECASTER: CardRecord = CardRecord::new(
    "Zealous Lorecaster",
    "36ab2130-9f21-4d30-873a-aa72d3d15fa8",
    "Lorenzo Mastroianni",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Giant", "Sorcerer"], 4, 4).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target instant or sorcery \
             card from your graveyard to your hand.",
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
        ),
    ]),
);

// SOS 138 — Aberrant Manawurm
// Audit: unsupported — Needs the triggering spell's actual mana payment total as a resolving effect value, rather than its mana value or chosen X.
pub(in crate::card::sets) static ABERRANT_MANAWURM: CardRecord = CardRecord::new(
    "Aberrant Manawurm",
    "797131cf-d80d-4050-bebd-2ce1d7fae5d0",
    "Lars Grant-West",
    CardRules::unsupported(),
);

// SOS 139 — Additive Evolution
pub(in crate::card::sets) static ADDITIVE_EVOLUTION: CardRecord = CardRecord::new(
    "Additive Evolution",
    "b44ec684-d558-45eb-bcd6-8119428634c2",
    "Josiah \"Jo\" Cameron",
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, create a 0/0 green and blue \
             Fractal creature token. Put three +1/+1 counters on it.",
            EffectDef::create_creature_token(
                &["Fractal"],
                &[ManaColor::Green, ManaColor::Blue],
                0,
                0,
            )
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("fractals"),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "fractals"
                    ))),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(3),
                },
            }),
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, put a +1/+1 counter \
             on target creature you control. It gains vigilance until end \
             of turn.",
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
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// SOS 140 — Ambitious Augmenter
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana. Also needs copying every counter kind and quantity from the dying source's last-known state.
pub(in crate::card::sets) static AMBITIOUS_AUGMENTER: CardRecord = CardRecord::new(
    "Ambitious Augmenter",
    "85629088-2007-4db5-9397-bac12a3d7498",
    "Mariah Tekulve",
    CardRules::unsupported(),
);

// SOS 141 — Burrog Barrage
pub(in crate::card::sets) static BURROG_BARRAGE: CardRecord = CardRecord::new(
    "Burrog Barrage",
    "95d5b0a8-2b66-418e-9e5e-ecf7b304c31e",
    "Christina Kraus",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+0 until end of turn if \
         you've cast another instant or sorcery spell this turn. Then \
         it deals damage equal to its power to up to one target \
         creature an opponent controls.",
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
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                        player: PlayerRelation::You,
                        spell: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    }),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(1),
                }),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
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

// SOS 142 — Chelonian Tackle
pub(in crate::card::sets) static CHELONIAN_TACKLE: CardRecord = CardRecord::new(
    "Chelonian Tackle",
    "a82a4d8c-4105-4923-85a2-ef58241f725c",
    "Lorenzo Mastroianni",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +0/+10 until end of turn. \
         Then it fights up to one target creature an opponent \
         controls. (Each deals damage equal to its power to the \
         other.)",
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
                    ValueDef::Constant(0),
                    ValueDef::Constant(10),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ]),
    )]),
);

// SOS 143 — Comforting Counsel
pub(in crate::card::sets) static COMFORTING_COUNSEL: CardRecord = CardRecord::new(
    "Comforting Counsel",
    "5223a04f-6b47-4379-80ce-8489c4a91734",
    "Chris Rahn",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you gain life, put a growth counter on this enchantment.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("growth"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "As long as there are five or more growth counters on this \
             enchantment, creatures you control get +3/+3.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("growth"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 5,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                },
            },
        ),
    ]),
);

// SOS 144 — Efflorescence
pub(in crate::card::sets) static EFFLORESCENCE: CardRecord = CardRecord::new(
    "Efflorescence",
    "79b9ace7-eceb-4f97-9ee7-d5ee3e0b3515",
    "Tuan Duong Chu",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put two +1/+1 counters on target creature.\nInfusion — If you \
         gained life this turn, that creature also gains trample and \
         indestructible until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ]),
    )]),
);

// SOS 145 — Emeritus of Abundance // Regrowth
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static EMERITUS_OF_ABUNDANCE: CardRecord = CardRecord::new(
    "Emeritus of Abundance // Regrowth",
    "ac095763-6f4e-4d4e-9c99-414646368f8d",
    "Justyna Dura",
    CardRules::unsupported(),
);

// SOS 146 — Emil, Vastlands Roamer
pub(in crate::card::sets) static EMIL_VASTLANDS_ROAMER: CardRecord = CardRecord::new(
    "Emil, Vastlands Roamer",
    "3654416d-8558-4af2-9e10-18dbc8f2b376",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Creatures you control with +1/+1 counters on them have trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            ),
            AbilityDef::activated(
                "{4}{G}, {T}: Create a 0/0 green and blue Fractal creature \
                 token. Put X +1/+1 counters on it, where X is the number of \
                 differently named lands you control.",
                &[CostDef::Mana(mana_cost!("{4}{G}")), CostDef::TapSource],
                EffectDef::create_creature_token(
                    &["Fractal"],
                    &[ManaColor::Green, ManaColor::Blue],
                    0,
                    0,
                )
                .with_created_tokens(CreatedTokensDef {
                    binding: crate::Binding!("fractals"),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            crate::Binding!("fractals"),
                        )),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::DistinctNamesAmong(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                    },
                }),
            ),
        ]),
);

// SOS 147 — Environmental Scientist
pub(in crate::card::sets) static ENVIRONMENTAL_SCIENTIST: CardRecord = CardRecord::new(
    "Environmental Scientist",
    "f2bf6b36-43e4-49d9-98b2-cbb4304c248b",
    "Piotr Dura",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Druid"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may search your library for a \
             basic land card, reveal it, put it into your hand, then \
             shuffle.",
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
                    destination: ZoneKind::Hand,
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

// SOS 148 — Follow the Lumarets
pub(in crate::card::sets) static FOLLOW_THE_LUMARETS: CardRecord = CardRecord::new(
    "Follow the Lumarets",
    "f9488480-2b6c-40bc-a93e-29fb1292a2e4",
    "Olivier Bernard",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell(
        "Infusion — Look at the top four cards of your library. You \
         may reveal a creature or land card from among them and put it \
         into your hand. If you gained life this turn, you may instead \
         reveal two creature and/or land cards from among them and put \
         them into your hand. Put the rest on the bottom of your \
         library in a random order.",
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                comparison: ComparisonDef::Greater,
                right: ValueDef::Constant(0),
            }),
            then: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
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
                        randomized: crate::Binding!("randomized"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "randomized"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
            otherwise: &EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("otherwise_chosen"),
                remainder: crate::Binding!("otherwise_rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("otherwise_chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "otherwise_chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("otherwise_rest")),
                        randomized: crate::Binding!("otherwise_randomized"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "otherwise_randomized"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        },
    )]),
);

// SOS 149 — Germination Practicum
// Audit: unsupported — Needs per-player first-resolution history keyed by spell name and the persistent paradigm permission offering a free copy from exile at each first main phase.
pub(in crate::card::sets) static GERMINATION_PRACTICUM: CardRecord = CardRecord::new(
    "Germination Practicum",
    "abe8332f-c76e-44e2-9427-d1228453abec",
    "Johan Grenier",
    CardRules::unsupported(),
);

// SOS 150 — Glorious Decay
pub(in crate::card::sets) static GLORIOUS_DECAY: CardRecord = CardRecord::new(
    "Glorious Decay",
    "a335f396-1004-4fee-842a-a35ff6ba17f2",
    "Camille Alquier",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
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
            AbilityDef::spell_with_targets(
                "Glorious Decay deals 4 damage to target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasAbility(AbilityPredicateDef::Keyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Exile target card from a graveyard. Draw a card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Any),
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Exile,
                        ZonePlacement::Top,
                    ),
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            ),
        ],
    )]),
);

// SOS 151 — Hungry Graffalon
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana.
pub(in crate::card::sets) static HUNGRY_GRAFFALON: CardRecord = CardRecord::new(
    "Hungry Graffalon",
    "030b1272-5990-4bc9-8fc1-82cc05602060",
    "Raph Lomotan",
    CardRules::unsupported(),
);

// SOS 152 — Infirmary Healer // Stream of Life
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static INFIRMARY_HEALER: CardRecord = CardRecord::new(
    "Infirmary Healer // Stream of Life",
    "911442e3-3003-4683-a766-e791e9553667",
    "Nereida",
    CardRules::unsupported(),
);

// SOS 153 — Lumaret's Favor
pub(in crate::card::sets) static LUMARET_S_FAVOR: CardRecord = CardRecord::new(
    "Lumaret's Favor",
    "c5e7c856-8b71-44e6-8998-0b0b3ff0ef99",
    "Mariah Tekulve",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Infusion — When you cast this spell, copy it if you gained \
             life this turn. You may choose new targets for the copy.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Source,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::Source,
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ),
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+4 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SOS 154 — Mindful Biomancer
pub(in crate::card::sets) static MINDFUL_BIOMANCER: CardRecord = CardRecord::new(
    "Mindful Biomancer",
    "2c3a6eb8-ce0c-4dc8-9ed6-d2a9223eef53",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dryad", "Druid"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
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
        .activations_each_turn(1),
    ]),
);

// SOS 155 — Noxious Newt
pub(in crate::card::sets) static NOXIOUS_NEWT: CardRecord = CardRecord::new(
    "Noxious Newt",
    "3a028306-c5d7-4f8f-b6f4-0d103fd47000",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Salamander"], 1, 2).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// SOS 156 — Oracle's Restoration
pub(in crate::card::sets) static ORACLE_S_RESTORATION: CardRecord = CardRecord::new(
    "Oracle's Restoration",
    "0863a19d-4511-4a78-98dd-d194afd1c39b",
    "Elliot Lang",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 until end of turn. You \
         draw a card and gain 1 life.",
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
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// SOS 157 — Pestbrood Sloth
pub(in crate::card::sets) static PESTBROOD_SLOTH: CardRecord = CardRecord::new(
    "Pestbrood Sloth",
    "c1251ae6-2f19-4f84-ab02-6a6cc7ce6056",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Plant", "Sloth"], 4, 4).with_abilities(&[
        abilities::reach(),
        abilities::dies_trigger(
            "When this creature dies, create two 1/1 black and green Pest \
             creature tokens with \"Whenever this token attacks, you gain \
             1 life.\"",
            EffectDef::create_creature_token(
                &["Pest"],
                &[ManaColor::Black, ManaColor::Green],
                1,
                1,
            )
            .with_count(ValueDef::Constant(2))
            .with_abilities(&[AbilityDef::triggered(
                "Whenever this token attacks, you gain 1 life.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            )]),
        ),
    ]),
);

// SOS 158 — Planar Engineering
pub(in crate::card::sets) static PLANAR_ENGINEERING: CardRecord = CardRecord::new(
    "Planar Engineering",
    "c83b96a3-ddfd-4d11-8a85-5bf62087cbb9",
    "Liiga Smilshkalne",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[AbilityDef::spell(
        "Sacrifice two lands. Search your library for four basic land \
         cards, put them onto the battlefield tapped, then shuffle.",
        EffectDef::Sequence(&[
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::Controller,
                zone: ZoneKind::Battlefield,
                candidates: ObjectPredicateDef::HasType(CardType::Land),
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(2)),
                chosen: crate::Binding!("sacrifices"),
                unchosen: crate::Binding!("unchosen_sacrifices"),
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    crate::Binding!("sacrifices"),
                ))),
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

// SOS 159 — Shopkeeper's Bane
pub(in crate::card::sets) static SHOPKEEPER_S_BANE: CardRecord = CardRecord::new(
    "Shopkeeper's Bane",
    "97f7fbb9-228c-4a74-975b-38d3b6cecb32",
    "Filipe Pagliuso",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Badger", "Pest"], 4, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature attacks, you gain 2 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// SOS 160 — Slumbering Trudge
// Audit: unsupported — Needs entry-value subtraction for three minus cast X and an entry replacement condition comparing that cast X with two; current entry-value and condition readers support neither composition.
pub(in crate::card::sets) static SLUMBERING_TRUDGE: CardRecord = CardRecord::new(
    "Slumbering Trudge",
    "3a925370-58ac-4181-9acc-db7b0e0abf17",
    "Tuan Duong Chu",
    CardRules::unsupported(),
);

// SOS 161 — Snarl Song
pub(in crate::card::sets) static SNARL_SONG: CardRecord = CardRecord::new(
    "Snarl Song",
    "fc4c7fa2-aebb-4636-9afd-f1010c923316",
    "Josu Hernaiz",
    CardRules::new_sorcery(mana_cost!("{5}{G}")).with_abilities(&[AbilityDef::spell(
        "Converge — Create two 0/0 green and blue Fractal creature \
         tokens. Put X +1/+1 counters on each of them and you gain X \
         life, where X is the number of colors of mana spent to cast \
         this spell.",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(
                &["Fractal"],
                &[ManaColor::Green, ManaColor::Blue],
                0,
                0,
            )
            .with_count(ValueDef::Constant(2))
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("fractals"),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "fractals"
                    ))),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::ColorsOfManaSpent,
                },
            }),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ColorsOfManaSpent,
            },
        ]),
    )]),
);

// SOS 162 — Studious First-Year // Rampant Growth
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static STUDIOUS_FIRST_YEAR: CardRecord = CardRecord::new(
    "Studious First-Year // Rampant Growth",
    "24f888dd-785c-4089-a89c-03f9080130ed",
    "Mariah Tekulve",
    CardRules::unsupported(),
);

// SOS 163 — Tenured Concocter
pub(in crate::card::sets) static TENURED_CONCOCTER: CardRecord = CardRecord::new(
    "Tenured Concocter",
    "376c8b7d-1c90-47e1-bd01-e4c67f3fc4fc",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Troll", "Druid"], 4, 5).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature becomes the target of a spell or \
             ability an opponent controls, you may draw a card.",
            TriggerEventDef::targets_selected(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Any,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                ]),
                StackTargetFilterDef::Permanent(ObjectPredicateDef::Source),
                StackTargetAggregationDef::EachMatchingTarget,
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ),
        AbilityDef::static_ability(
            "Infusion — This creature gets +2/+0 as long as you gained \
             life this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
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

// SOS 164 — Thornfist Striker
pub(in crate::card::sets) static THORNFIST_STRIKER: CardRecord = CardRecord::new(
    "Thornfist Striker",
    "f6cb838e-a06a-46ae-a30f-e5192178c1cc",
    "Diana Franco",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 3, 3).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{1}"))], "Ward {1}"),
        AbilityDef::static_ability(
            "Infusion — Creatures you control get +1/+0 and have trample \
             as long as you gained life this turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                    comparison: ComparisonDef::Greater,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::StaticApply {
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
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            },
        ),
    ]),
);

// SOS 165 — Topiary Lecturer
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana.
pub(in crate::card::sets) static TOPIARY_LECTURER: CardRecord = CardRecord::new(
    "Topiary Lecturer",
    "4f16a1c2-0a80-45e4-b025-3aa0c0b03812",
    "Josiah \"Jo\" Cameron",
    CardRules::unsupported(),
);

// SOS 166 — Vastlands Scavenger // Bind to Life
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static VASTLANDS_SCAVENGER: CardRecord = CardRecord::new(
    "Vastlands Scavenger // Bind to Life",
    "476b6a4d-cc05-4e98-8a45-a5c6582ec514",
    "Bryan Sola",
    CardRules::unsupported(),
);

// SOS 167 — Wild Hypothesis
pub(in crate::card::sets) static WILD_HYPOTHESIS: CardRecord = CardRecord::new(
    "Wild Hypothesis",
    "04fdfabc-c247-4384-a5bb-f49035f8aae0",
    "Lie Setiawan",
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_abilities(&[AbilityDef::spell(
        "Create a 0/0 green and blue Fractal creature token. Put X \
         +1/+1 counters on it.\nSurveil 2. (Look at the top two cards \
         of your library, then put any number of them into your \
         graveyard and the rest on top of your library in any order.)",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(
                &["Fractal"],
                &[ManaColor::Green, ManaColor::Blue],
                0,
                0,
            )
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("fractals"),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "fractals"
                    ))),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::ChosenX,
                },
            }),
            abilities::surveil(ValueDef::Constant(2)),
        ]),
    )]),
);

// SOS 168 — Wildgrowth Archaic
// Audit: unsupported — Needs a cast-trigger value reading the creature spell's paid colors and an entry modification attached to that particular spell until it resolves.
pub(in crate::card::sets) static WILDGROWTH_ARCHAIC: CardRecord = CardRecord::new(
    "Wildgrowth Archaic",
    "0e6e2188-7203-4d10-a838-27233f283cd5",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// SOS 169 — Zimone's Experiment
// Audit: unsupported — Needs a simultaneous move with per-card battlefield or hand destinations, including a destination choice when a revealed card is both a land and a creature; sequential collection moves do not represent that overlapping instruction.
pub(in crate::card::sets) static ZIMONE_S_EXPERIMENT: CardRecord = CardRecord::new(
    "Zimone's Experiment",
    "a6597852-4267-4ea6-a391-f927e4833be2",
    "Izzy",
    CardRules::unsupported(),
);

// SOS 170 — Abigale, Poet Laureate // Heroic Stanza
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static ABIGALE_POET_LAUREATE: CardRecord = CardRecord::new(
    "Abigale, Poet Laureate // Heroic Stanza",
    "77285d12-e658-4eb3-ba13-ff202afab9c8",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// SOS 171 — Abstract Paintmage
pub(in crate::card::sets) static ABSTRACT_PAINTMAGE: CardRecord = CardRecord::new(
    "Abstract Paintmage",
    "ea008094-d995-4740-9b39-c61049356c55",
    "David Auden Nash",
    CardRules::new_creature(mana_cost!("{U}{U/R}{R}"), &["Djinn", "Sorcerer"], 2, 2)
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of your first main phase, add {U}{R}. Spend \
             this mana only to cast instant and sorcery spells.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::PrecombatMain,
                player: PlayerRelation::You,
            },
            EffectDef::AddMana(
                AddManaEffectDef::one_of_each(ManaColor::Blue, ManaColor::Red).with_restrictions(
                    &[ManaRestrictionDef::CastSpell(ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]))],
                ),
            ),
        )]),
);

// SOS 172 — Applied Geometry
pub(in crate::card::sets) static APPLIED_GEOMETRY: CardRecord = CardRecord::new(
    "Applied Geometry",
    "f109f2eb-895b-44a6-b6b5-81bf3831ccd5",
    "Justyna Dura",
    CardRules::new_sorcery(mana_cost!("{2}{G}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Create a token that's a copy of target non-Aura permanent you \
             control, except it's a 0/0 Fractal creature in addition to \
             its other types. Put six +1/+1 counters on it.",
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
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Aura",
                        ))),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::create_token_from_copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef {
                    added_types: CardTypeSet::single(CardType::Creature),
                    added_creature_types: CreatureTypeSetDef::named(&["Fractal"]),
                    base_power_toughness: Some((0, 0)),
                    ..CopyExceptionsDef::NONE
                },
            })
            .with_created_tokens(CreatedTokensDef {
                binding: crate::Binding!("copy"),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "copy"
                    ))),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(6),
                },
            }),
        ),
    ]),
);

// SOS 173 — Ark of Hunger
// Audit: unsupported — Needs a grouped one-or-more-cards-leave-your-graveyard event and a one-turn play permission for the exact milled card in the graveyard, including lands.
pub(in crate::card::sets) static ARK_OF_HUNGER: CardRecord = CardRecord::new(
    "Ark of Hunger",
    "79d01c19-162b-4a12-9e27-18366d95eaa0",
    "Ksenia Kim",
    CardRules::unsupported(),
);

// SOS 174 — Aziza, Mage Tower Captain
pub(in crate::card::sets) static AZIZA_MAGE_TOWER_CAPTAIN: CardRecord = CardRecord::new(
    "Aziza, Mage Tower Captain",
    "6261e89a-dbf1-481a-823e-6bb00be57195",
    "Aurore Folny",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Djinn", "Sorcerer"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, you may tap \
             three untapped creatures you control. If you do, copy that \
             spell. You may choose new targets for the copy.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(3),
                }),
                then: &EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 3,
                        maximum: 3,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Sequence(&[
                            EffectDef::Tap {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                            },
                            EffectDef::CopyStackObject(&CopyStackObjectDef {
                                object: EffectRecipientDef::TriggeringObject,
                                controller: PlayerRefDef::EffectController,
                                count: ValueDef::Constant(1),
                                retarget: true,
                                colors: None,
                            }),
                        ]),
                    }),
                },
            },
        )]),
);

// SOS 175 — Berta, Wise Extrapolator
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana. Also needs a grouped one-or-more-counters-added event.
pub(in crate::card::sets) static BERTA_WISE_EXTRAPOLATOR: CardRecord = CardRecord::new(
    "Berta, Wise Extrapolator",
    "75f89c36-c81d-4580-9a5c-218fed0c5c9a",
    "Tuan Duong Chu",
    CardRules::unsupported(),
);

// SOS 176 — Blech, Loafing Pest
pub(in crate::card::sets) static BLECH_LOAFING_PEST: CardRecord = CardRecord::new(
    "Blech, Loafing Pest",
    "f588fa50-7cc5-41ba-90df-2d252eb5c785",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Pest"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever you gain life, put a +1/+1 counter on each Pest, \
             Bat, Insect, Snake, and Spider you control.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pest")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Bat")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Insect")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Snake")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]),
);

// SOS 177 — Bogwater Lumaret
pub(in crate::card::sets) static BOGWATER_LUMARET: CardRecord = CardRecord::new(
    "Bogwater Lumaret",
    "7a42f51a-3377-47bb-b6fb-c0515bf1dcfb",
    "Lie Setiawan",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Spirit", "Frog"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature or another creature you control \
             enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Source,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
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

// SOS 178 — Borrowed Knowledge
pub(in crate::card::sets) static BORROWED_KNOWLEDGE: CardRecord = CardRecord::new(
    "Borrowed Knowledge",
    "a3226e14-554d-47c9-b8b6-dfeb53cc41ba",
    "Inkognit",
    CardRules::new_sorcery(mana_cost!("{2}{R}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Discard your hand, then draw cards equal to the number of \
                 cards in target opponent's hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
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
                    abilities::draw_cards(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerRelation::Opponent,
                        ),
                    )),
                ]),
            ),
            AbilityDef::spell(
                "Discard your hand, then draw cards equal to the number of \
                 cards discarded this way.",
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Any,
                        bound: Some(crate::Binding!("discarded")),
                        effect: &abilities::draw_cards(ValueDef::BoundObjectCount(
                            crate::Binding!("discarded"),
                        )),
                    }),
                },
            ),
        ],
    )]),
);

// SOS 179 — Cauldron of Essence
pub(in crate::card::sets) static CAULDRON_OF_ESSENCE: CardRecord = CardRecord::new(
    "Cauldron of Essence",
    "b7091740-e70c-4cf2-8d3d-b8e1ac1fbbdd",
    "Craig J Spearing",
    CardRules::new_artifact(mana_cost!("{1}{B}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever a creature you control dies, each opponent loses 1 \
             life and you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
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
        AbilityDef::activated_with_targets(
            "{1}{B}{G}, {T}, Sacrifice a creature: Return target creature \
             card from your graveyard to the battlefield. Activate only as \
             a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{1}{B}{G}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
            ],
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
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// SOS 180 — Colorstorm Stallion
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static COLORSTORM_STALLION: CardRecord = CardRecord::new(
    "Colorstorm Stallion",
    "f5b54d46-2caf-4d1b-8be1-dbd9e9dce058",
    "Lorenzo Lanfranconi",
    CardRules::unsupported(),
);

// SOS 181 — Colossus of the Blood Age
// Audit: unsupported — Needs a zero-to-all discard selection with a follow-up counting the cards actually discarded after replacements; the current discard operation selects a fixed computed quantity.
pub(in crate::card::sets) static COLOSSUS_OF_THE_BLOOD_AGE: CardRecord = CardRecord::new(
    "Colossus of the Blood Age",
    "bfa7f0a4-6b65-4e53-ba00-848df260d8e3",
    "Leon Tukker",
    CardRules::unsupported(),
);

// SOS 182 — Conciliator's Duelist
pub(in crate::card::sets) static CONCILIATOR_S_DUELIST: CardRecord = CardRecord::new(
    "Conciliator's Duelist",
    "e225929b-6197-4550-969e-3c4a97206a68",
    "Andrew Mar",
    CardRules::new_creature(mana_cost!("{W}{W}{B}{B}"), &["Kor", "Warlock"], 4, 3).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, draw a card. Each player loses 1 life.",
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::EachPlayer,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::triggered_with_targets(
                "Repartee — Whenever you cast an instant or sorcery spell that \
                 targets a creature, exile up to one target creature. Return \
                 that card to the battlefield under its owner's control at the \
                 beginning of the next end step.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                            CardType::Creature,
                        )),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
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
                    then: &EffectDef::BindObjects(BindObjectsDef {
                        source: ObjectCollectionSourceDef::ObjectSet(
                            ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!("exiled")),
                        ),
                        binding: crate::Binding!("returning"),
                        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of the next end step, return that card to \
                                 the battlefield under its owner's control.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("returning"),
                                    )),
                                    ZoneKind::Battlefield,
                                    ZonePlacement::Top,
                                ),
                            ),
                        )),
                    }),
                },
            ),
        ],
    ),
);

// SOS 183 — Cuboid Colony
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana.
pub(in crate::card::sets) static CUBOID_COLONY: CardRecord = CardRecord::new(
    "Cuboid Colony",
    "6384d135-7780-4d75-9e95-71bce506948e",
    "Alexandre Honoré",
    CardRules::unsupported(),
);

// SOS 184 — Dina's Guidance
pub(in crate::card::sets) static DINA_S_GUIDANCE: CardRecord = CardRecord::new(
    "Dina's Guidance",
    "775c1e50-08a4-413f-ab0f-f1c2a79cfe94",
    "Manuel Castañón",
    CardRules::new_instant(mana_cost!("{1}{B}{G}")).with_abilities(&[AbilityDef::spell(
        "Search your library for a creature card, reveal it, put it \
         into your hand or graveyard, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: false,
            enters_tapped: false,
            attachment: None,
            binding: Some(crate::Binding!("found")),
            then: Some(&EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(crate::Binding!("found")),
                        predicate: ObjectSetPredicateDef {
                            filter: None,
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 1,
                        },
                    }),
                    then: &EffectDef::ChooseEffect {
                        player: EffectRecipientDef::Controller,
                        choices: &[
                            EffectChoiceDef {
                                label: "Hand",
                                effect: EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("found"),
                                    )),
                                    ZoneKind::Hand,
                                    ZonePlacement::Top,
                                ),
                            },
                            EffectChoiceDef {
                                label: "Graveyard",
                                effect: EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("found"),
                                    )),
                                    ZoneKind::Graveyard,
                                    ZonePlacement::Top,
                                ),
                            },
                        ],
                    },
                },
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
            ])),
        },
    )]),
);

// SOS 185 — Elemental Mascot
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts. Its exile permission also needs to expire at the end of your next turn.
pub(in crate::card::sets) static ELEMENTAL_MASCOT: CardRecord = CardRecord::new(
    "Elemental Mascot",
    "c507eb1c-48e9-4d28-bb2d-71f2a9df9ab0",
    "Justyna Dura",
    CardRules::unsupported(),
);

// SOS 186 — Embrace the Paradox
pub(in crate::card::sets) static EMBRACE_THE_PARADOX: CardRecord = CardRecord::new(
    "Embrace the Paradox",
    "c0cf5e0f-3668-46f2-850d-d91a538e8ead",
    "Julian Kok Joon Wen",
    CardRules::new_instant(mana_cost!("{3}{G}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw three cards. You may put a land card from your hand onto \
         the battlefield tapped.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(3)),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
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
        ]),
    )]),
);

// SOS 187 — Essenceknit Scholar
// Audit: unsupported — Needs per-controller creature-death history for the current turn; the existing creature-died flag records deaths globally.
pub(in crate::card::sets) static ESSENCEKNIT_SCHOLAR: CardRecord = CardRecord::new(
    "Essenceknit Scholar",
    "2a3cba55-3fae-4d45-ae03-4d662ec13718",
    "Ioannis Fiore",
    CardRules::unsupported(),
);

// SOS 188 — Fix What's Broken
pub(in crate::card::sets) static FIX_WHAT_S_BROKEN: CardRecord = CardRecord::new(
    "Fix What's Broken",
    "c0cd1d71-8e4a-4e00-80cd-83aec231fa57",
    "Chris Rallis",
    CardRules::new_sorcery(mana_cost!("{2}{W}{B}")).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay X life.\nReturn \
             each artifact and creature card with mana value X from your \
             graveyard to the battlefield.",
            &[],
            CostDef::pay_life(CostQuantityDef::ChosenX),
            EffectDef::move_to_zone(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// SOS 189 — Fractal Mascot
pub(in crate::card::sets) static FRACTAL_MASCOT: CardRecord = CardRecord::new(
    "Fractal Mascot",
    "cf5b19e3-eed1-4b36-9756-660ffb3baa08",
    "Manuel Castañón",
    CardRules::new_creature(mana_cost!("{4}{G}{U}"), &["Fractal", "Elk"], 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, tap target creature an opponent \
             controls. Put a stun counter on it. (If a permanent with a \
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
        ),
    ]),
);

// SOS 190 — Fractal Tender
// Audit: unsupported — Needs a cast-event value for the total mana actually spent, compared with this creature's power and toughness at trigger and resolution time; mana value and chosen X do not measure paid mana. Also needs a per-object, per-player counter-added-this-turn history.
pub(in crate::card::sets) static FRACTAL_TENDER: CardRecord = CardRecord::new(
    "Fractal Tender",
    "ea7f5262-4ddb-410a-be72-4bac6af9b4ec",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// SOS 191 — Geometer's Arthropod
// Audit: unsupported — Needs a predicate for an X symbol in a card or spell's mana cost, including casts with X equal to zero; chosen X and mana value do not answer that question. Also needs the triggering spell's chosen X, rather than the new triggered ability's X.
pub(in crate::card::sets) static GEOMETER_S_ARTHROPOD: CardRecord = CardRecord::new(
    "Geometer's Arthropod",
    "ec0f3613-1edc-40e8-8f26-2e5ef13be55e",
    "Joe Slucher",
    CardRules::unsupported(),
);

// SOS 192 — Grapple with Death
pub(in crate::card::sets) static GRAPPLE_WITH_DEATH: CardRecord = CardRecord::new(
    "Grapple with Death",
    "62842fb4-8bd3-4d80-b4f9-5bc3c5cebd3a",
    "Nereida",
    CardRules::new_sorcery(mana_cost!("{1}{B}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact or creature. You gain 1 life.",
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
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// SOS 193 — Growth Curve
pub(in crate::card::sets) static GROWTH_CURVE: CardRecord = CardRecord::new(
    "Growth Curve",
    "1675a445-86ae-413b-b95a-a1c254a7f252",
    "Joe Slucher",
    CardRules::new_sorcery(mana_cost!("{G}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature you control, then \
         double the number of +1/+1 counters on that creature.",
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
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::CountersOnObject(&ObjectCounterValueDef {
                    object: ObjectRefDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                }),
            },
        ]),
    )]),
);

// SOS 194 — Hardened Academic
// Audit: unsupported — Needs a grouped one-or-more-cards-leave-your-graveyard event; the current per-card zone-change event would trigger separately for cards moved together.
pub(in crate::card::sets) static HARDENED_ACADEMIC: CardRecord = CardRecord::new(
    "Hardened Academic",
    "06c9e8a7-2840-4cff-90af-c6636e598f78",
    "Vincent Christiaens",
    CardRules::unsupported(),
);

// SOS 195 — Imperious Inkmage
pub(in crate::card::sets) static IMPERIOUS_INKMAGE: CardRecord = CardRecord::new(
    "Imperious Inkmage",
    "d5df1c3f-2536-4476-b8cd-34b026c38366",
    "Nathaniel Himawan",
    CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Orc", "Warlock"], 3, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// SOS 196 — Inkling Mascot
pub(in crate::card::sets) static INKLING_MASCOT: CardRecord = CardRecord::new(
    "Inkling Mascot",
    "6d4a2f39-0e1e-4076-815a-2676a09a1aab",
    "Inkognit",
    CardRules::new_creature(mana_cost!("{W}{B}"), &["Inkling", "Cat"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, this creature gains flying until end of \
             turn. Surveil 1. (Look at the top card of your library. You \
             may put it into your graveyard.)",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                abilities::surveil(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// SOS 197 — Killian's Confidence
pub(in crate::card::sets) static KILLIAN_S_CONFIDENCE: CardRecord = CardRecord::new(
    "Killian's Confidence",
    "55ff776a-fc3b-4338-8864-d57a85b3f123",
    "Jodie Muir",
    CardRules::new_sorcery(mana_cost!("{W}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +1/+1 until end of turn. Draw a card.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
        AbilityDef::triggered(
            "Whenever one or more creatures you control deal combat damage \
             to a player, you may pay {W/B}. If you do, return this card \
             from your graveyard to your hand.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                players: PlayerRelation::Any,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{W/B}"))],
                &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// SOS 198 — Kirol, History Buff // Pack a Punch
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static KIROL_HISTORY_BUFF: CardRecord = CardRecord::new(
    "Kirol, History Buff // Pack a Punch",
    "676ba521-66e4-42cf-a315-70d03cb7334e",
    "Bryan Sola",
    CardRules::unsupported(),
);

// SOS 199 — Lluwen, Exchange Student // Pest Friend
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static LLUWEN_EXCHANGE_STUDENT: CardRecord = CardRecord::new(
    "Lluwen, Exchange Student // Pest Friend",
    "a0bcb638-c3c8-4973-9537-5c471f43f34f",
    "Alix Branwyn",
    CardRules::unsupported(),
);

// SOS 200 — Lorehold Charm
pub(in crate::card::sets) static LOREHOLD_CHARM: CardRecord = CardRecord::new(
    "Lorehold Charm",
    "5fe70295-e550-4577-a341-dab6c25aabfd",
    "Ksenia Kim",
    CardRules::new_instant(mana_cost!("{R}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Each opponent sacrifices a nontoken artifact of their choice.",
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Opponent,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
            ),
            AbilityDef::spell_with_targets(
                "Return target artifact or creature card with mana value 2 or \
                 less from your graveyard to the battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(2),
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
            AbilityDef::spell(
                "Creatures you control get +1/+1 and gain trample until end of \
                 turn.",
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
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// SOS 201 — Lorehold, the Historian
// Audit: unsupported — Needs miracle granted to matching cards in hand and recognized during the first-draw reveal/alternative-cast window; current static grants do not reach that draw procedure.
pub(in crate::card::sets) static LOREHOLD_THE_HISTORIAN: CardRecord = CardRecord::new(
    "Lorehold, the Historian",
    "71a6701f-40f1-43ef-bff5-a5907fd67cd6",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// SOS 202 — Mind into Matter
pub(in crate::card::sets) static MIND_INTO_MATTER: CardRecord = CardRecord::new(
    "Mind into Matter",
    "0a7f0fdf-1d4b-4458-a19c-274611e8a59a",
    "Joe Slucher",
    CardRules::new_sorcery(mana_cost!("{X}{G}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw X cards. Then you may put a permanent card with mana \
         value X or less from your hand onto the battlefield tapped.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::ChosenX),
            EffectDef::Choose(ChooseDef {
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
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                    ]),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Private,
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
        ]),
    )]),
);

// SOS 203 — Mind Roots
pub(in crate::card::sets) static MIND_ROOTS: CardRecord = CardRecord::new(
    "Mind Roots",
    "9d5fdbda-ebbe-45d6-a668-5ddee057a063",
    "Elliot Lang",
    CardRules::new_sorcery(mana_cost!("{1}{B}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player discards two cards. Put up to one land card \
             discarded this way onto the battlefield tapped under your \
             control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: Some(DiscardFollowUpDef {
                    counted: ObjectPredicateDef::HasType(CardType::Land),
                    bound: Some(crate::Binding!("discarded_lands")),
                    effect: &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::Binding!("discarded_lands")),
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
                                modifications: &[BattlefieldEntryModificationDef::Tapped],
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                        },
                    }),
                }),
            },
        ),
    ]),
);

// SOS 204 — Molten Note
// Audit: unsupported — Needs a resolving value for the actual mana spent to cast this spell, including flashback and cost modifications; mana value and chosen X are insufficient.
pub(in crate::card::sets) static MOLTEN_NOTE: CardRecord = CardRecord::new(
    "Molten Note",
    "506f69aa-7dc4-4dd7-990a-7371fc1762c0",
    "David Álvarez",
    CardRules::unsupported(),
);

// SOS 205 — Moment of Reckoning
pub(in crate::card::sets) static MOMENT_OF_RECKONING: CardRecord = CardRecord::new(
    "Moment of Reckoning",
    "577d9dc8-7720-4dc9-b650-64b4729b309b",
    "Néstor Ossandón Leal",
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}{B}{B}")).with_abilities(&[
        AbilityDef::modal_spell(
            "Choose one —",
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target nonland permanent.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Return target nonland permanent card from your graveyard to \
                     the battlefield.",
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
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                                    CardType::Land,
                                )),
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
            ],
        )
        .with_mode_selection(0, 4, true),
    ]),
);

// SOS 206 — Nita, Forum Conciliator
// Audit: unsupported — Needs a this-turn permission to cast the bound exiled card with both spend-mana-as-any-type and exile-instead-of-graveyard riders attached to the resulting spell.
pub(in crate::card::sets) static NITA_FORUM_CONCILIATOR: CardRecord = CardRecord::new(
    "Nita, Forum Conciliator",
    "fd80a87d-35d3-4ad1-8172-c85e93032d1d",
    "Jodie Muir",
    CardRules::unsupported(),
);

// SOS 207 — Old-Growth Educator
pub(in crate::card::sets) static OLD_GROWTH_EDUCATOR: CardRecord = CardRecord::new(
    "Old-Growth Educator",
    "eb7e858a-9b85-49b2-a379-ee656b64935a",
    "Vincent Christiaens",
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Treefolk", "Druid"], 4, 4).with_abilities(
        &[
            abilities::reach(),
            abilities::vigilance(),
            abilities::enters_trigger(
                "Infusion — When this creature enters, put two +1/+1 counters \
                 on it if you gained life this turn.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                        comparison: ComparisonDef::Greater,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(2),
                    },
                },
            ),
        ],
    ),
);

// SOS 208 — Paradox Surveyor
// Audit: unsupported — Needs a predicate for an X symbol in a card or spell's mana cost, including casts with X equal to zero; chosen X and mana value do not answer that question.
pub(in crate::card::sets) static PARADOX_SURVEYOR: CardRecord = CardRecord::new(
    "Paradox Surveyor",
    "d7cb1af2-0302-46ff-8303-ae9d07541a01",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// SOS 209 — Pest Mascot
pub(in crate::card::sets) static PEST_MASCOT: CardRecord = CardRecord::new(
    "Pest Mascot",
    "d882beb9-6766-4818-afbb-f6fd7a2d5b70",
    "Filipe Pagliuso",
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Pest", "Ape"], 2, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever you gain life, put a +1/+1 counter on this creature.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// SOS 210 — Practiced Scrollsmith
// Audit: unsupported — Needs exile-play permission expiring at the end of the specified player's next turn; the existing next-turn permission expires after the following opponent turn.
pub(in crate::card::sets) static PRACTICED_SCROLLSMITH: CardRecord = CardRecord::new(
    "Practiced Scrollsmith",
    "40075e3f-58b3-47fd-8fbe-4b301e9ce7a1",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// SOS 211 — Prismari Charm
pub(in crate::card::sets) static PRISMARI_CHARM: CardRecord = CardRecord::new(
    "Prismari Charm",
    "8f6c2a5e-fe13-407c-aadd-c9caf2884ff1",
    "Inkognit",
    CardRules::new_instant(mana_cost!("{U}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Surveil 2, then draw a card.",
                EffectDef::Sequence(&[
                    abilities::surveil(ValueDef::Constant(2)),
                    abilities::draw_cards(ValueDef::Constant(1)),
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Prismari Charm deals 1 damage to each of one or two targets.",
                &[AbilityTargetDef {
                    minimum: 1,
                    ..AbilityTargetDef::up_to(AbilityTargetPredicate::AnyTarget, 2)
                }],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Return target nonland permanent to its owner's hand.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )]),
);

// SOS 212 — Prismari, the Inspiration
// Audit: unsupported — Needs storm granted to the instant and sorcery spells as they are cast, so the copied trigger belongs to each spell; battlefield ability grants do not apply to prospective cast spells.
pub(in crate::card::sets) static PRISMARI_THE_INSPIRATION: CardRecord = CardRecord::new(
    "Prismari, the Inspiration",
    "767ff9fa-4e7f-421a-b911-45186b520ae1",
    "Justin Gerard",
    CardRules::unsupported(),
);

// SOS 213 — Proctor's Gaze
pub(in crate::card::sets) static PROCTOR_S_GAZE: CardRecord = CardRecord::new(
    "Proctor's Gaze",
    "b127d543-0a90-4af6-9410-94d5cd30389e",
    "Danny Schwartz",
    CardRules::new_instant(mana_cost!("{2}{G}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return up to one target nonland permanent to its owner's \
             hand. Search your library for a basic land card, put it onto \
             the battlefield tapped, then shuffle.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
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
            ]),
        ),
    ]),
);

// SOS 214 — Professor Dellian Fel
pub(in crate::card::sets) static PROFESSOR_DELLIAN_FEL: CardRecord = CardRecord::new(
    "Professor Dellian Fel",
    "6ff3b4d8-1271-4c5d-8834-7662244f173d",
    "Lie Setiawan",
    CardRules::new_planeswalker(mana_cost!("{2}{B}{G}"), &["Dellian"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+2: You gain 3 life.",
                &[CostDef::Loyalty(2)],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::activated(
                "0: You draw a card and lose 1 life.",
                &[CostDef::Loyalty(0)],
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "−3: Destroy target creature.",
                &[CostDef::Loyalty(-3)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::activated(
                "−6: You get an emblem with \"Whenever you gain life, target \
                 opponent loses that much life.\"",
                &[CostDef::Loyalty(-6)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new(
                        "Dellian Emblem",
                        &[AbilityDef::triggered_with_targets(
                            "Whenever you gain life, target opponent loses that much life.",
                            TriggerEventDef::LifeGained(PlayerRelation::You),
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                            )],
                            EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::TriggerEventAmount,
                            },
                        )],
                    ),
                },
            ),
        ]),
);

// SOS 215 — Pterafractyl
pub(in crate::card::sets) static PTERAFRACTYL: CardRecord = CardRecord::new(
    "Pterafractyl",
    "ecd33152-e290-4505-addd-a8d08cefdddd",
    "Andrew Mar",
    CardRules::new_creature(mana_cost!("{X}{G}{U}"), &["Dinosaur", "Fractal"], 1, 0)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::as_enters(
                "This creature enters with X +1/+1 counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCastXCounters {
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ),
            ),
            abilities::enters_trigger(
                "When this creature enters, you gain 2 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// SOS 216 — Pursue the Past
pub(in crate::card::sets) static PURSUE_THE_PAST: CardRecord = CardRecord::new(
    "Pursue the Past",
    "4584d5f7-b1f1-4c8e-80c5-ad35e44a968e",
    "Craig Elliott",
    CardRules::new_sorcery(mana_cost!("{R}{W}")).with_abilities(&[
        AbilityDef::spell(
            "You gain 2 life. You may discard a card. If you do, draw two \
             cards.",
            EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::DiscardCards(1)],
                    &abilities::draw_cards(ValueDef::Constant(2)),
                )),
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{R}{W}"))]),
    ]),
);

// SOS 217 — Quandrix Charm
pub(in crate::card::sets) static QUANDRIX_CHARM: CardRecord = CardRecord::new(
    "Quandrix Charm",
    "318486e0-f255-40f5-8150-dc272eec9d7d",
    "Matheus Graef",
    CardRules::new_instant(mana_cost!("{G}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
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
                EffectDef::PayOr(
                    PayOrDef::unless(
                        &[CostDef::Mana(mana_cost!("{2}"))],
                        &EffectDef::counter_target(TargetIndex::PRIMARY),
                    )
                    .with_payer(PlayerSetDef::One(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    ))),
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
            AbilityDef::spell_with_targets(
                "Target creature has base power and toughness 5/5 until end of \
                 turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(5),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// SOS 218 — Quandrix, the Proof
// Audit: unsupported — Needs cascade granted to matching spells cast from hand and a free-cast filter checking the chosen spell face's mana value as well as the exiled card's mana value.
pub(in crate::card::sets) static QUANDRIX_THE_PROOF: CardRecord = CardRecord::new(
    "Quandrix, the Proof",
    "015afe31-af3c-4c9b-9997-d7c33b915a33",
    "Lucas Graciano",
    CardRules::unsupported(),
);

// SOS 219 — Rapturous Moment
pub(in crate::card::sets) static RAPTUROUS_MOMENT: CardRecord = CardRecord::new(
    "Rapturous Moment",
    "21afe19d-881a-48cb-863e-22942bea5ebe",
    "Evyn Fong",
    CardRules::new_sorcery(mana_cost!("{4}{U}{R}")).with_abilities(&[AbilityDef::spell(
        "Draw three cards, then discard two cards. Add {U}{U}{R}{R}{R}.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(3)),
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue).with_amount(2)),
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(3)),
        ]),
    )]),
);

// SOS 220 — Render Speechless
pub(in crate::card::sets) static RENDER_SPEECHLESS: CardRecord = CardRecord::new(
    "Render Speechless",
    "25bbb1c7-14e8-444f-ab98-e95f50927460",
    "David Astruga",
    CardRules::new_sorcery(mana_cost!("{2}{W}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent reveals their hand. You choose a nonland card \
             from it. That player discards that card.\nPut two +1/+1 \
             counters on up to one target creature.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Opponent,
                )),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// SOS 221 — Resonating Lute
pub(in crate::card::sets) static RESONATING_LUTE: CardRecord = CardRecord::new(
    "Resonating Lute",
    "6ef168d6-28f2-4c24-9bfa-82c35663b729",
    "Edgar Sánchez Hidalgo",
    CardRules::new_artifact(mana_cost!("{2}{U}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "Lands you control have \"{T}: Add two mana of any one color. \
             Spend this mana only to cast instant and sorcery spells.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}: Add two mana of any one color. Spend this mana only to \
                     cast instant and sorcery spells.",
                    &[CostDef::TapSource],
                    EffectDef::AddMana(
                        AddManaEffectDef::any_color()
                            .with_amount(2)
                            .with_restrictions(&[ManaRestrictionDef::CastSpell(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Instant),
                                    ObjectPredicateDef::HasType(CardType::Sorcery),
                                ]),
                            )]),
                    ),
                )),
            },
        ),
        AbilityDef::activated(
            "{T}: Draw a card. Activate only if you have seven or more \
             cards in your hand.",
            &[CostDef::TapSource],
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .with_activation_condition(&TriggerConditionDef::ValueComparison(
            &ValueComparisonDef {
                left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(7),
            },
        )),
    ]),
);

// SOS 222 — Root Manipulation
pub(in crate::card::sets) static ROOT_MANIPULATION: CardRecord = CardRecord::new(
    "Root Manipulation",
    "5390a79c-bc4b-4edb-a845-0d3514986401",
    "Elizabeth Peiró",
    CardRules::new_sorcery(mana_cost!("{3}{B}{G}")).with_abilities(&[AbilityDef::spell(
        "Until end of turn, creatures you control get +2/+2 and gain \
         menace and \"Whenever this creature attacks, you gain 1 \
         life.\" (A creature with menace can't be blocked except by \
         two or more creatures.)",
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
                AppliedEffectDef::add_ability(&abilities::menace()),
                AppliedEffectDef::add_ability(&AbilityDef::triggered(
                    "Whenever this creature attacks, you gain 1 life.",
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                )),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// SOS 223 — Sanar, Unfinished Genius // Wild Idea
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SANAR_UNFINISHED_GENIUS: CardRecord = CardRecord::new(
    "Sanar, Unfinished Genius // Wild Idea",
    "173157aa-712d-44f2-89ba-dd2511a07f26",
    "Justin Gerard",
    CardRules::unsupported(),
);

// SOS 224 — Scolding Administrator
// Audit: unsupported — Needs copying all counter kinds and quantities from a dying source's last-known state onto a chosen recipient.
pub(in crate::card::sets) static SCOLDING_ADMINISTRATOR: CardRecord = CardRecord::new(
    "Scolding Administrator",
    "69757177-aefa-44a6-81db-5ae9b5d2f117",
    "Aleksi Briclot",
    CardRules::unsupported(),
);

// SOS 225 — Silverquill Charm
pub(in crate::card::sets) static SILVERQUILL_CHARM: CardRecord = CardRecord::new(
    "Silverquill Charm",
    "3eb73579-f1c6-4762-81d2-9568ab501fac",
    "Ksenia Kim",
    CardRules::new_instant(mana_cost!("{W}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
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
            AbilityDef::spell_with_targets(
                "Exile target creature with power 2 or less.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                    ]),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell(
                "Each opponent loses 3 life and you gain 3 life.",
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
        ],
    )]),
);

// SOS 226 — Silverquill, the Disputant
// Audit: unsupported — Needs casualty granted during casting, including an optional power-qualified sacrifice cost and the corresponding cast-time copy trigger tied to its paid receipt.
pub(in crate::card::sets) static SILVERQUILL_THE_DISPUTANT: CardRecord = CardRecord::new(
    "Silverquill, the Disputant",
    "1742c9cd-5ba0-4335-9999-acc7f9d4f73c",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// SOS 227 — Snooping Page
pub(in crate::card::sets) static SNOOPING_PAGE: CardRecord = CardRecord::new(
    "Snooping Page",
    "73d06987-8686-461b-b260-9a4fee6a3b32",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{1}{W}{B}"), &["Human", "Cleric"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Repartee — Whenever you cast an instant or sorcery spell that \
             targets a creature, this creature can't be blocked this turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, you \
             draw a card and lose 1 life.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
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

// SOS 228 — Social Snub
pub(in crate::card::sets) static SOCIAL_SNUB: CardRecord = CardRecord::new(
    "Social Snub",
    "a04b6900-0436-4920-a0d4-c0186d605ae3",
    "Raluca Marinescu",
    CardRules::new_sorcery(mana_cost!("{1}{W}{B}")).with_abilities(&[
        AbilityDef::triggered_if(
            "When you cast this spell while you control a creature, you \
             may copy this spell.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Source,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::Source,
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            },
        ),
        AbilityDef::spell(
            "Each player sacrifices a creature of their choice. Each \
             opponent loses 1 life and you gain 1 life.",
            EffectDef::Sequence(&[
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::EachPlayer,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifices"),
                    unchosen: crate::Binding!("unchosen_sacrifices"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifices")),
                    )),
                }),
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

// SOS 229 — Spectacular Skywhale
// Audit: unsupported — Needs a retained triggering-spell payment total to distinguish casts that spent at least five mana, including cost reductions, increases, alternate costs, and free casts.
pub(in crate::card::sets) static SPECTACULAR_SKYWHALE: CardRecord = CardRecord::new(
    "Spectacular Skywhale",
    "c90366d5-b4ba-4772-a3c5-f138bbe7f305",
    "Serena Malyon",
    CardRules::unsupported(),
);

// SOS 230 — Spirit Mascot
// Audit: unsupported — Needs a grouped one-or-more-cards-leave-your-graveyard event; the current per-card zone-change event would trigger separately for cards moved together.
pub(in crate::card::sets) static SPIRIT_MASCOT: CardRecord = CardRecord::new(
    "Spirit Mascot",
    "123f1fde-d8de-4640-baa1-bb3781713168",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// SOS 231 — Splatter Technique
pub(in crate::card::sets) static SPLATTER_TECHNIQUE: CardRecord = CardRecord::new(
    "Splatter Technique",
    "969b6657-c3b9-47e1-a42e-95bbcccf452d",
    "Tuan Duong Chu",
    CardRules::new_sorcery(mana_cost!("{1}{U}{U}{R}{R}")).with_abilities(&[
        AbilityDef::modal_spell(
            "Choose one —",
            &[
                AbilityDef::spell(
                    "Draw four cards.",
                    abilities::draw_cards(ValueDef::Constant(4)),
                ),
                AbilityDef::spell(
                    "Splatter Technique deals 4 damage to each creature and \
                     planeswalker.",
                    EffectDef::damage(
                        EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ))),
                        ValueDef::Constant(4),
                    ),
                ),
            ],
        ),
    ]),
);

// SOS 232 — Stadium Tidalmage
pub(in crate::card::sets) static STADIUM_TIDALMAGE: CardRecord = CardRecord::new(
    "Stadium Tidalmage",
    "a689289e-7141-4950-8a87-82e9bd6846fe",
    "Ioannis Fiore",
    CardRules::new_creature(mana_cost!("{2}{U}{R}"), &["Djinn", "Sorcerer"], 4, 4).with_abilities(
        &[AbilityDef::triggered(
            "Whenever this creature enters or attacks, you may draw a \
             card. If you do, discard a card.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            },
        )],
    ),
);

// SOS 233 — Startled Relic Sloth
pub(in crate::card::sets) static STARTLED_RELIC_SLOTH: CardRecord = CardRecord::new(
    "Startled Relic Sloth",
    "f143fd41-58c3-45a0-bef8-e9e4b4a502a5",
    "David Astruga",
    CardRules::new_creature(mana_cost!("{2}{R}{W}"), &["Sloth", "Beast"], 4, 4).with_abilities(&[
        abilities::trample(),
        abilities::lifelink(),
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, exile up to one \
             target card from a graveyard.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::Any),
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// SOS 234 — Stirring Honormancer
pub(in crate::card::sets) static STIRRING_HONORMANCER: CardRecord = CardRecord::new(
    "Stirring Honormancer",
    "ee84b04d-78fc-416f-9166-72e5417c3e17",
    "April Prime",
    CardRules::new_creature(mana_cost!("{2}{W}{W/B}{B}"), &["Rhino", "Bard"], 4, 5).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, look at the top X cards of your \
             library, where X is the number of creatures you control. Put \
             one of those cards into your hand and the rest into your \
             graveyard.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
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
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("rest"))),
                        ZoneKind::Graveyard,
                        ZonePlacement::Bottom,
                    ),
                ]),
            }),
        )],
    ),
);

// SOS 235 — Stress Dream
pub(in crate::card::sets) static STRESS_DREAM: CardRecord = CardRecord::new(
    "Stress Dream",
    "1ec40a1b-51e7-4a35-966c-ab2a10f21a80",
    "Edgar Sánchez Hidalgo",
    CardRules::new_instant(mana_cost!("{3}{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Stress Dream deals 5 damage to up to one target creature. \
             Look at the top two cards of your library. Put one of those \
             cards into your hand and the other on the bottom of your \
             library.",
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
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(5),
                ),
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
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
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("randomized"),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("randomized"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            ]),
        ),
    ]),
);

// SOS 236 — Suspend Aggression
// Audit: unsupported — Needs exile-play permission expiring at the end of the specified player's next turn; the existing next-turn permission expires after the following opponent turn.
pub(in crate::card::sets) static SUSPEND_AGGRESSION: CardRecord = CardRecord::new(
    "Suspend Aggression",
    "135c0696-d86d-4e48-988c-5c218de451fc",
    "Andreas Zafiratos",
    CardRules::unsupported(),
);

// SOS 237 — Tam, Observant Sequencer // Deep Sight
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static TAM_OBSERVANT_SEQUENCER: CardRecord = CardRecord::new(
    "Tam, Observant Sequencer // Deep Sight",
    "7120e71b-2976-451b-89a7-a1665dc6fb6b",
    "Jodie Muir",
    CardRules::unsupported(),
);

// SOS 238 — Teacher's Pest
pub(in crate::card::sets) static TEACHER_S_PEST: CardRecord = CardRecord::new(
    "Teacher's Pest",
    "eaa358ac-761d-4507-aa15-3d4684027207",
    "Stephanie Cheung",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Skeleton", "Pest"], 1, 1).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever this creature attacks, you gain 1 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{B}{G}: Return this card from your graveyard to the \
             battlefield tapped.",
            &[CostDef::Mana(mana_cost!("{B}{G}"))],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
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

// SOS 239 — Traumatic Critique
pub(in crate::card::sets) static TRAUMATIC_CRITIQUE: CardRecord = CardRecord::new(
    "Traumatic Critique",
    "2a812fa7-4599-4e25-97db-20ffc6bc0b26",
    "Aaron Miller",
    CardRules::new_instant(mana_cost!("{X}{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Traumatic Critique deals X damage to any target. Draw two \
             cards, then discard a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::ChosenX,
                ),
                abilities::draw_cards(ValueDef::Constant(2)),
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

// SOS 240 — Vibrant Outburst
pub(in crate::card::sets) static VIBRANT_OUTBURST: CardRecord = CardRecord::new(
    "Vibrant Outburst",
    "f9ba68ef-6efc-4249-8b74-e33f47173902",
    "Eelis Kyttanen",
    CardRules::new_instant(mana_cost!("{U}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Vibrant Outburst deals 3 damage to any target. Tap up to one \
         target creature.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex(1)),
            },
        ]),
    )]),
);

// SOS 241 — Vicious Rivalry
pub(in crate::card::sets) static VICIOUS_RIVALRY: CardRecord = CardRecord::new(
    "Vicious Rivalry",
    "6fa9cd18-3181-4373-ab65-49bf9de9487f",
    "Chris Rallis",
    CardRules::new_sorcery(mana_cost!("{2}{B}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay X life.\nDestroy all artifacts and \
             creatures with mana value X or less.",
            &[],
            CostDef::pay_life(CostQuantityDef::ChosenX),
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                then: None,
            },
        ),
    ),
);

// SOS 242 — Visionary's Dance
pub(in crate::card::sets) static VISIONARY_S_DANCE: CardRecord = CardRecord::new(
    "Visionary's Dance",
    "846a0e79-a530-429e-8f7f-4b87f1b0156e",
    "Josiah \"Jo\" Cameron",
// Seven mana is more than a limited deck usually reaches, which is what
    // the discard half is for: the card is never stranded in hand.
    CardRules::new_sorcery(mana_cost!("{5}{U}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 3/3 blue and red Elemental creature tokens with flying.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Elemental"], &[ManaColor::Blue, ManaColor::Red], 3, 3)
                        .with_abilities(&[abilities::flying()]),
                ))
                .with_amount(2),
            ),
        ),
        AbilityDef::activated(
            "{2}, Discard this card: Look at the top two cards of your library. Put one of them into your hand and the other into your graveyard.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::DiscardSource,
            ],
            abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(2),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        )
        // Activated from hand, which is the only place a card can be
        // discarded from.
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// SOS 243 — Wilt in the Heat
// Audit: unsupported — Needs self-cost reduction conditioned on the existing card-left-your-graveyard turn history; that history is currently readable by triggers but not by the self-cost evaluator.
pub(in crate::card::sets) static WILT_IN_THE_HEAT: CardRecord = CardRecord::new(
    "Wilt in the Heat",
    "f63f7209-fc0f-400c-8076-125f3131cb32",
    "Raluca Marinescu",
    CardRules::unsupported(),
);

// SOS 244 — Witherbloom Charm
pub(in crate::card::sets) static WITHERBLOOM_CHARM: CardRecord = CardRecord::new(
    "Witherbloom Charm",
    "254437f7-7a8a-4b11-9cea-e8e7ea23c59e",
    "Florian Herold",
    CardRules::new_instant(mana_cost!("{B}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "You may sacrifice a permanent. If you do, draw two cards.",
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]))],
                    &abilities::draw_cards(ValueDef::Constant(2)),
                )),
            ),
            AbilityDef::spell(
                "You gain 5 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target nonland permanent with mana value 2 or less.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(2),
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

// SOS 245 — Witherbloom, the Balancer
pub(in crate::card::sets) static WITHERBLOOM_THE_BALANCER: CardRecord = CardRecord::new(
    "Witherbloom, the Balancer",
    "ed7b2361-97c6-49e2-bf0b-4770f4ffe2f0",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{6}{B}{G}"), &["Elder", "Dragon"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Affinity for creatures (This spell costs {1} less to cast for \
                 each creature you control.)",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::flying(),
            abilities::deathtouch(),
            abilities::spell_cost_reduction(
                "Instant and sorcery spells you cast have affinity for creatures.",
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                PlayerRelation::You,
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ]),
);

// SOS 246 — Zaffai and the Tempests
// Audit: unsupported — Needs a once-per-own-turn free-cast permission for matching hand spells, with usage reserved and consumed by the successful cast.
pub(in crate::card::sets) static ZAFFAI_AND_THE_TEMPESTS: CardRecord = CardRecord::new(
    "Zaffai and the Tempests",
    "5bdbf507-6fd7-49f6-b437-8f2ce2d0eb0f",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// SOS 247 — Biblioplex Tomekeeper
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static BIBLIOPLEX_TOMEKEEPER: CardRecord = CardRecord::new(
    "Biblioplex Tomekeeper",
    "bf2efdd9-d2b4-4bea-a5b9-dbb2eee4dfba",
    "Raph Lomotan",
    CardRules::unsupported(),
);

// SOS 248 — Diary of Dreams
pub(in crate::card::sets) static DIARY_OF_DREAMS: CardRecord = CardRecord::new(
    "Diary of Dreams",
    "ee1e0a96-af80-444e-a456-5b256cf60625",
    "Genel Jumalon",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Book"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast an instant or sorcery spell, put a page \
                 counter on this artifact.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("page"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "{5}, {T}: Draw a card. This ability costs {1} less to \
                 activate for each page counter on this artifact.",
                &[CostDef::Mana(mana_cost!("{5}")), CostDef::TapSource],
                abilities::draw_cards(ValueDef::Constant(1)),
            )
            .with_activation_cost_reduction(
                ValueDef::CountersOnSource(CounterKind::named("page")),
                0,
            ),
        ]),
);

// SOS 249 — Mage Tower Referee
pub(in crate::card::sets) static MAGE_TOWER_REFEREE: CardRecord = CardRecord::new(
    "Mage Tower Referee",
    "1ceb704a-97a8-49f9-b799-30f001404144",
    "Nino Vecia",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 2, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a multicolored spell, put a +1/+1 counter \
             on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::ColorCount(0),
                    ObjectPredicateDef::ColorCount(1),
                ])),
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

// SOS 250 — Page, Loose Leaf
pub(in crate::card::sets) static PAGE_LOOSE_LEAF: CardRecord = CardRecord::new(
    "Page, Loose Leaf",
    "8c6fecfd-8241-4cf0-b1eb-19472b99e0ed",
    "Michal Ivan",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Construct"], 0, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{T}: Add {C}.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
            ),
            AbilityDef::activated(
                "Grandeur — Discard another card named Page, Loose Leaf: \
                 Reveal cards from the top of your library until you reveal an \
                 instant or sorcery card. Put that card into your hand and the \
                 rest on the bottom of your library in a random order.",
                &[CostDef::discard(ObjectPredicateDef::NameEquals(
                    CardNameDef::Literal("Page, Loose Leaf"),
                ))],
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
                        player: PlayerRefDef::EffectController,
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Reveal,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
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
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("randomized"),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("randomized"),
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

// SOS 251 — Potioner's Trove
pub(in crate::card::sets) static POTIONER_S_TROVE: CardRecord = CardRecord::new(
    "Potioner's Trove",
    "2123b349-4649-4a15-a8b5-b54414d2b1b7",
    "Alessandra Pisano",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated(
            "{T}: You gain 2 life. Activate only if you've cast an instant \
             or sorcery spell this turn.",
            &[CostDef::TapSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        )
        .with_activation_condition(&TriggerConditionDef::ValueComparison(
            &ValueComparisonDef {
                left: ValueDef::CountSpellsCastThisTurn(&SpellCastQueryDef {
                    spell: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    player: PlayerRelation::You,
                }),
                comparison: ComparisonDef::Greater,
                right: ValueDef::Constant(0),
            },
        )),
    ]),
);

// SOS 252 — Strixhaven Skycoach
pub(in crate::card::sets) static STRIXHAVEN_SKYCOACH: CardRecord = CardRecord::new(
    "Strixhaven Skycoach",
    "87741fbb-b426-4f83-a358-587b0907f081",
    "Michal Ivan",
    CardRules::new_vehicle(mana_cost!("{3}"), 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this Vehicle enters, you may search your library for a \
             basic land card, reveal it, put it into your hand, then \
             shuffle.",
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
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// SOS 253 — Deathcap Glade (reprint)
const DEATHCAP_GLADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::DEATHCAP_GLADE,
    "78897104-80e1-4d8a-9958-145b40f679e8",
    "Piotr Dura",
);

// SOS 254 — Dreamroot Cascade (reprint)
const DREAMROOT_CASCADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::DREAMROOT_CASCADE,
    "ef662b92-5a7f-48c9-bcc1-14b55e091aef",
    "Marco Gorlei",
);

// SOS 255 — Fields of Strife
pub(in crate::card::sets) static FIELDS_OF_STRIFE: CardRecord = CardRecord::new(
    "Fields of Strife",
    "3dc7a4c3-c356-4fba-bea0-e8788da3eb57",
    "Josu Solano",
    // Titan's Grave in red and white, and its sink costs both colours: the
    // surveil is only reachable in the deck the land is already fixing for.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
        AbilityDef::activated(
            "{2}{R}{W}, {T}: Surveil 1.",
            &[CostDef::Mana(mana_cost!("{2}{R}{W}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

/// The SOS cycle of tapped duals that surveil. Unlike the flat-cost tapped
/// duals elsewhere, each of these prices its surveil in its own two colours,
/// so the activation cost is a parameter beside the colours rather than
/// something the helper can derive.
///
/// The abilities are added one at a time in printed order: an array holding
/// the parameterized ones could not be given a `'static` lifetime.
const fn guildhall_surveil_land(
    mana_text: &'static str,
    colors: &'static [ManaColor],
    surveil_text: &'static str,
    surveil_cost: &'static [CostDef],
) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(abilities::enters_tapped(CardType::Land))
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated_with_targets(
            surveil_text,
            surveil_cost,
            &[],
            abilities::surveil(ValueDef::Constant(1)),
        ))
}

// SOS 256 — Forum of Amity
pub(in crate::card::sets) static FORUM_OF_AMITY: CardRecord = CardRecord::new(
    "Forum of Amity",
    "1de6c6cc-0c55-4997-8623-d7f796bd9ab8",
    "Richard Wright",
    guildhall_surveil_land(
        "{T}: Add {W} or {B}.",
        &[ManaColor::White, ManaColor::Black],
        "{2}{W}{B}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        &[CostDef::Mana(mana_cost!("{2}{W}{B}")), CostDef::TapSource],
    ),
);

// SOS 257 — Great Hall of the Biblioplex
pub(in crate::card::sets) static GREAT_HALL_OF_THE_BIBLIOPLEX: CardRecord = CardRecord::new(
    "Great Hall of the Biblioplex",
    "42d92674-2664-411c-b9c5-b04da7c845f4",
    "Constantin Marin",
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{T}, Pay 1 life: Add one mana of any color. Spend this mana \
             only to cast an instant or sorcery spell.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ])),
            ])),
        ),
        AbilityDef::activated(
            "{5}: If this land isn't a creature, it becomes a 2/4 Wizard \
             creature with \"Whenever you cast an instant or sorcery \
             spell, this creature gets +1/+0 until end of turn.\" It's \
             still a land.",
            &[CostDef::Mana(mana_cost!("{5}"))],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                },
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Wizard",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(4),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever you cast an instant or sorcery spell, this creature \
                             gets +1/+0 until end of turn.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Instant),
                                    ObjectPredicateDef::HasType(CardType::Sorcery),
                                ]),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ])),
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(0),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        )),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            },
        ),
    ]),
);

// SOS 258 — Paradox Gardens
pub(in crate::card::sets) static PARADOX_GARDENS: CardRecord = CardRecord::new(
    "Paradox Gardens",
    "dbc3447e-1329-4ea1-b4ca-b321b0ffec8f",
    "Leon Tukker",
    guildhall_surveil_land(
        "{T}: Add {G} or {U}.",
        &[ManaColor::Green, ManaColor::Blue],
        "{2}{G}{U}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        &[CostDef::Mana(mana_cost!("{2}{G}{U}")), CostDef::TapSource],
    ),
);

// SOS 259 — Petrified Hamlet
pub(in crate::card::sets) static PETRIFIED_HAMLET: CardRecord = CardRecord::new(
    "Petrified Hamlet",
    "355dd460-b0e9-41f2-a058-b7f7e39ac387",
    "Richard Wright",
CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "As this land enters, choose a land card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                binding: Binding!("petrified_hamlet_name"),
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::LandCardNames,
                ),
            },
        ),
        abilities::cannot_activate_nonmana_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            crate::card::CardNameDef::Binding(Binding!("petrified_hamlet_name")),
        ),
        AbilityDef::static_ability(
            "Lands with the chosen name have “{T}: Add {C}.”",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::NameEquals(
                            crate::card::CardNameDef::Binding(Binding!("petrified_hamlet_name")),
                        ),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::tap_for(ManaColor::Colorless)),
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
    ]),
);

// SOS 260 — Shattered Sanctum (reprint)
const SHATTERED_SANCTUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::SHATTERED_SANCTUM,
    "5aa0c810-3b7d-4661-979e-e84fb327742d",
    "Sergey Glushakov",
);

// SOS 261 — Skycoach Waypoint
// Audit: unsupported — Needs the prepared/unprepared designation and permission to cast a copy of the associated prepare spell while consuming that designation; the current face and cast-permission model does not represent prepare spells.
pub(in crate::card::sets) static SKYCOACH_WAYPOINT: CardRecord = CardRecord::new(
    "Skycoach Waypoint",
    "6747657b-5ce4-4dbd-b924-ca1f7119faf7",
    "Jorge Jacinto",
    CardRules::unsupported(),
);

// SOS 262 — Spectacle Summit
pub(in crate::card::sets) static SPECTACLE_SUMMIT: CardRecord = CardRecord::new(
    "Spectacle Summit",
    "a0a66f7b-eab4-45da-8895-c2c2c7eb05f8",
    "Andreas Zafiratos",
    guildhall_surveil_land(
        "{T}: Add {U} or {R}.",
        &[ManaColor::Blue, ManaColor::Red],
        "{2}{U}{R}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
         your graveyard.)",
        &[CostDef::Mana(mana_cost!("{2}{U}{R}")), CostDef::TapSource],
    ),
);

// SOS 263 — Stormcarved Coast (reprint)
const STORMCARVED_COAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::STORMCARVED_COAST,
    "bd3ae4fa-4c97-410a-8c0a-bd203342595d",
    "Leon Tukker",
);

// SOS 264 — Sundown Pass (reprint)
const SUNDOWN_PASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::SUNDOWN_PASS,
    "b34000e9-ff20-4fb4-9d0b-03a172a92457",
    "Sergey Glushakov",
);

// SOS 265 — Terramorphic Expanse (reprint)
const TERRAMORPHIC_EXPANSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::TERRAMORPHIC_EXPANSE,
    "9a4c5629-fadd-42b9-850f-9f8586a2ca50",
    "Leon Tukker",
);

// SOS 266 — Titan's Grave
pub(in crate::card::sets) static TITAN_S_GRAVE: CardRecord = CardRecord::new(
    "Titan's Grave",
    "a9ab41c8-3ee2-4676-9b8b-20c34d9f5f21",
    "Lorenzo Lanfranconi",
    // A tapped dual whose late-game half costs more than the land itself,
    // which is the point: it is a land first and a mana sink only when the
    // draw step has nothing better.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::activated(
            "{2}{B}{G}, {T}: Surveil 1.",
            &[CostDef::Mana(mana_cost!("{2}{B}{G}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// SOS 267 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "a845de50-4af0-4f4a-9c2a-db587973571c",
    "Joshua Raphael",
);

// SOS 268 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "937250fe-bcad-4ff8-9406-286a69db7e0a",
    "Joshua Raphael",
);

// SOS 269 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "1797d5c7-d3fa-4184-85ae-46db14ddf523",
    "Joshua Raphael",
);

// SOS 270 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "6af1f1db-eb91-4297-83f6-9318b87fd220",
    "Joshua Raphael",
);

// SOS 271 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "46196e8f-9339-4f00-b9cf-cab8f9abc80e",
    "Joshua Raphael",
);

// SOS 272 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "d85d0f25-a24a-4de0-9b8b-93fb5017bce9",
    "Leon Tukker",
);

// SOS 273 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "bac08825-ca54-4a9f-bf4f-f75708a1550b",
    "Andreas Zafiratos",
);

// SOS 274 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "77b88bb8-6bd9-4632-b937-89468fcb5e6a",
    "Sergey Glushakov",
);

// SOS 275 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "fd1b9e7c-09d0-4907-9f48-34289f3cd2cc",
    "Constantin Marin",
);

// SOS 276 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "51fe930f-2b5a-4b1e-9007-6ee74fb44715",
    "Sergey Glushakov",
);

// SOS 277 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "36af939c-11d9-43a5-bd69-c915a62e972b",
    "Leon Tukker",
);

// SOS 278 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "a642c7b1-d4d1-4125-a66d-560438e5ee51",
    "Sergey Glushakov",
);

// SOS 279 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "bce45bc0-accd-4853-a166-9dd5597526d5",
    "Florian Herold",
);

// SOS 280 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "f169dfb2-e4c8-46e9-8591-e51bb82da082",
    "Raph Lomotan",
);

// SOS 281 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "3041d539-4f15-4836-a215-afa19a5cc23f",
    "Andreas Zafiratos",
);

// SOS 282 — Ral Zarek, Guest Lecturer (alternate printing)
const RAL_ZAREK_GUEST_LECTURER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAL_ZAREK_GUEST_LECTURER,
    1,
    "130fca3b-714a-4e77-b7e9-55554fcc0df2",
    "Qistina Khalidah",
);

// SOS 283 — Professor Dellian Fel (alternate printing)
const PROFESSOR_DELLIAN_FEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PROFESSOR_DELLIAN_FEL,
    1,
    "bf2212c5-b9ec-4bc5-bcca-e7c7debeca0c",
    "Qistina Khalidah",
);

// SOS 284 — Lorehold, the Historian (alternate printing)
const LOREHOLD_THE_HISTORIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOREHOLD_THE_HISTORIAN,
    1,
    "44fe9413-99a6-40e9-85b6-f9ba9bd4de86",
    "Raymond Swanland",
);

// SOS 285 — Prismari, the Inspiration (alternate printing)
const PRISMARI_THE_INSPIRATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PRISMARI_THE_INSPIRATION,
    1,
    "472a4348-a1cc-4fb7-a801-9ac1d0f3eb05",
    "Wayne Reynolds",
);

// SOS 286 — Quandrix, the Proof (alternate printing)
const QUANDRIX_THE_PROOF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUANDRIX_THE_PROOF,
    1,
    "17dff19f-e732-44a3-907d-fa2982389653",
    "Anastasia Ovchinnikova",
);

// SOS 287 — Silverquill, the Disputant (alternate printing)
const SILVERQUILL_THE_DISPUTANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILVERQUILL_THE_DISPUTANT,
    1,
    "9477f2f4-2d82-412c-9af8-df66a3c457be",
    "Mark Zug",
);

// SOS 288 — Witherbloom, the Balancer (alternate printing)
const WITHERBLOOM_THE_BALANCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WITHERBLOOM_THE_BALANCER,
    1,
    "cc22696c-a9ef-48b7-a74a-7dd0a9c3d1bb",
    "Kev Walker",
);

// SOS 289 — The Dawning Archaic (alternate printing)
const THE_DAWNING_ARCHAIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DAWNING_ARCHAIC,
    1,
    "d3605e6f-ea75-4c86-9ffa-79355a4695e5",
    "Mintautas Šukys",
);

// SOS 290 — Restoration Seminar (alternate printing)
const RESTORATION_SEMINAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTORATION_SEMINAR,
    1,
    "83a32ec6-8322-4e4d-a4ff-5470adb05fda",
    "Rimas Valeikis",
);

// SOS 291 — Echocasting Symposium (alternate printing)
const ECHOCASTING_SYMPOSIUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ECHOCASTING_SYMPOSIUM,
    1,
    "72a21e89-5988-4b8b-8e67-a915e74a54e2",
    "Gintas Galvanauskas",
);

// SOS 292 — Decorum Dissertation (alternate printing)
const DECORUM_DISSERTATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DECORUM_DISSERTATION,
    1,
    "6040e27f-ed10-410c-8ae5-cfcba35c0d4c",
    "Michael MacRae",
);

// SOS 293 — Tragedy Feaster (alternate printing)
const TRAGEDY_FEASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRAGEDY_FEASTER,
    1,
    "ae9009fa-4b52-422d-8ff3-f699089249d8",
    "Rimas Valeikis",
);

// SOS 294 — Improvisation Capstone (alternate printing)
const IMPROVISATION_CAPSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IMPROVISATION_CAPSTONE,
    1,
    "36985a8e-dcc0-4349-93d7-071370a75615",
    "Mintautas Šukys",
);

// SOS 295 — Magmablood Archaic (alternate printing)
const MAGMABLOOD_ARCHAIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAGMABLOOD_ARCHAIC,
    1,
    "30f2d6a5-1e21-4d12-bd51-6e40683b0e72",
    "Gintas Galvanauskas",
);

// SOS 296 — Germination Practicum (alternate printing)
const GERMINATION_PRACTICUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GERMINATION_PRACTICUM,
    1,
    "df144bff-cdc8-46df-bbd2-6fdc93f04f04",
    "Julia Vasilyeva",
);

// SOS 297 — Wildgrowth Archaic (alternate printing)
const WILDGROWTH_ARCHAIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILDGROWTH_ARCHAIC,
    1,
    "d1861bc6-93d5-4b09-a5dd-94c0a0629e5f",
    "Vilhelmas Banys",
);

// SOS 298 — Blech, Loafing Pest (alternate printing)
const BLECH_LOAFING_PEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLECH_LOAFING_PEST,
    1,
    "df4fa3ab-30e3-4e82-a8b8-4fbaeab7870b",
    "Julia Vasilyeva",
);

// SOS 299 — Colorstorm Stallion (alternate printing)
const COLORSTORM_STALLION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLORSTORM_STALLION,
    1,
    "e8800c58-574f-4a77-bed9-2d99e8f5813c",
    "Eglė Mosakaitė",
);

// SOS 300 — Geometer's Arthropod (alternate printing)
const GEOMETER_S_ARTHROPOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GEOMETER_S_ARTHROPOD,
    1,
    "9278cfc3-7dcb-4b84-8646-4fb745fece9e",
    "Michael MacRae",
);

// SOS 301 — Deathcap Glade (alternate printing)
const DEATHCAP_GLADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vow::DEATHCAP_GLADE,
    1,
    "dca82673-3788-4b92-8605-bedda45e5b16",
    "Piotr Dura",
);

// SOS 302 — Dreamroot Cascade (alternate printing)
const DREAMROOT_CASCADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vow::DREAMROOT_CASCADE,
    1,
    "bb911ec9-4354-46af-8750-6920555eb1e7",
    "Piotr Dura",
);

// SOS 303 — Shattered Sanctum (alternate printing)
const SHATTERED_SANCTUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vow::SHATTERED_SANCTUM,
    1,
    "cfc3f81d-333f-4ac9-8374-082675e07ac1",
    "Piotr Dura",
);

// SOS 304 — Stormcarved Coast (alternate printing)
const STORMCARVED_COAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vow::STORMCARVED_COAST,
    1,
    "36f6c636-3539-494f-abf9-62ea00fcedfd",
    "Piotr Dura",
);

// SOS 305 — Sundown Pass (alternate printing)
const SUNDOWN_PASS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_vow::SUNDOWN_PASS,
    1,
    "ac70c9ae-5115-46db-9ef8-05c475c88154",
    "Piotr Dura",
);

// SOS 306 — Emeritus of Ideation // Ancestral Recall (alternate printing)
const EMERITUS_OF_IDEATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMERITUS_OF_IDEATION,
    1,
    "ef371352-ec8f-4da4-9085-67195068fb79",
    "Mark Poole",
);

// SOS 307 — Together as One (alternate printing)
const TOGETHER_AS_ONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOGETHER_AS_ONE,
    1,
    "82e4948e-5f2b-498f-8864-d46870910d83",
    "Néstor Ossandón Leal",
);

// SOS 308 — Antiquities on the Loose (alternate printing)
const ANTIQUITIES_ON_THE_LOOSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTIQUITIES_ON_THE_LOOSE,
    1,
    "fbeda63a-ce34-4d84-8306-cb48353a52f8",
    "Andreas Zafiratos",
);

// SOS 309 — Emeritus of Truce // Swords to Plowshares (alternate printing)
const EMERITUS_OF_TRUCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMERITUS_OF_TRUCE,
    1,
    "bfd607c0-7ed7-4a4e-abdd-508080f40ef2",
    "Aleksi Briclot",
);

// SOS 310 — Erode (alternate printing)
const ERODE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERODE,
    1,
    "67af20f7-666f-48fb-863b-15bc5b17ab09",
    "Florian Herold",
);

// SOS 311 — Informed Inkwright (alternate printing)
const INFORMED_INKWRIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INFORMED_INKWRIGHT,
    1,
    "fe5d2001-0c56-44fc-92cb-dd250669f3b9",
    "Nia Kovalevski",
);

// SOS 312 — Joined Researchers // Secret Rendezvous (alternate printing)
const JOINED_RESEARCHERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JOINED_RESEARCHERS,
    1,
    "15d99a75-939e-4584-9b4a-8afd115bab2c",
    "Ryan Pancoast",
);

// SOS 313 — Practiced Offense (alternate printing)
const PRACTICED_OFFENSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PRACTICED_OFFENSE,
    1,
    "e22c20cf-69c2-4225-8011-00cd27ac4103",
    "Raluca Marinescu",
);

// SOS 314 — Stirring Hopesinger (alternate printing)
const STIRRING_HOPESINGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STIRRING_HOPESINGER,
    1,
    "47eba7ac-d01c-4d17-a191-c6869cc5c3d2",
    "Cristi Balanescu",
);

// SOS 315 — Emeritus of Ideation // Ancestral Recall (alternate printing)
const EMERITUS_OF_IDEATION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EMERITUS_OF_IDEATION,
    2,
    "d8e1e20c-03b9-446b-84d4-466377e50310",
    "Evyn Fong",
);

// SOS 316 — Exhibition Tidecaller (alternate printing)
const EXHIBITION_TIDECALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXHIBITION_TIDECALLER,
    1,
    "44304d28-94da-4226-b713-b2c3a4b67e49",
    "Tulio Brito",
);

// SOS 317 — Harmonized Trio // Brainstorm (alternate printing)
const HARMONIZED_TRIO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARMONIZED_TRIO,
    1,
    "c8f1baae-dd81-4e10-841e-15531ea1ffaa",
    "Marie Magny",
);

// SOS 318 — Jadzi, Steward of Fate // Oracle's Gift (alternate printing)
const JADZI_STEWARD_OF_FATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JADZI_STEWARD_OF_FATE,
    1,
    "92fcf155-41ae-4275-8791-27ee95a2f7c0",
    "Martina Fačková",
);

// SOS 319 — Mana Sculpt (alternate printing)
const MANA_SCULPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MANA_SCULPT,
    1,
    "fc6f7f7b-8a1e-4fe0-8a56-22c1c537aad9",
    "Cristi Balanescu",
);

// SOS 320 — Mathemagics (alternate printing)
const MATHEMAGICS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MATHEMAGICS,
    1,
    "102d4882-9774-4370-aff3-370f7729d7dd",
    "Liiga Smilshkalne",
);

// SOS 321 — Pensive Professor (alternate printing)
const PENSIVE_PROFESSOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PENSIVE_PROFESSOR,
    1,
    "f1c1cf3f-fbd6-45d9-9f74-b811cb489fa1",
    "Billy Christian",
);

// SOS 322 — Skycoach Conductor // All Aboard (alternate printing)
const SKYCOACH_CONDUCTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYCOACH_CONDUCTOR,
    1,
    "f9092a09-78a8-421c-868b-115175f4c252",
    "Christina Kraus",
);

// SOS 323 — Wisdom of Ages (alternate printing)
const WISDOM_OF_AGES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WISDOM_OF_AGES,
    1,
    "8fcd092c-c317-4b17-8d72-5232b4802b96",
    "Jabari Weathers",
);

// SOS 324 — Emeritus of Woe // Demonic Tutor (alternate printing)
const EMERITUS_OF_WOE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMERITUS_OF_WOE,
    1,
    "26156da4-1ec3-4702-bded-9868af3026b5",
    "Jodie Muir",
);

// SOS 325 — Grave Researcher // Reanimate (alternate printing)
const GRAVE_RESEARCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRAVE_RESEARCHER,
    1,
    "1d12804c-dda0-4dae-9eb8-f670e72c17e7",
    "Izzy",
);

// SOS 326 — Moseo, Vein's New Dean (alternate printing)
const MOSEO_VEIN_S_NEW_DEAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOSEO_VEIN_S_NEW_DEAN,
    1,
    "b5f7cc28-ab95-4880-ade6-b28245cf12af",
    "Abz J Harding",
);

// SOS 327 — Postmortem Professor (alternate printing)
const POSTMORTEM_PROFESSOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POSTMORTEM_PROFESSOR,
    1,
    "96fbf6dc-f9f9-4568-a1a3-c65b7a9b455b",
    "Nino Vecia",
);

// SOS 328 — Pox Plague (alternate printing)
const POX_PLAGUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POX_PLAGUE,
    1,
    "cb66cb9b-9877-4146-901c-5f2cbfe86c73",
    "Camille Alquier",
);

// SOS 329 — Scheming Silvertongue // Sign in Blood (alternate printing)
const SCHEMING_SILVERTONGUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCHEMING_SILVERTONGUE,
    1,
    "1c4c2765-2fb0-43f2-9e50-934405d108d2",
    "Anna Steinbauer",
);

// SOS 330 — Withering Curse (alternate printing)
const WITHERING_CURSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WITHERING_CURSE,
    1,
    "e17f07de-7409-446d-8f21-09228f752bee",
    "Tuan Duong Chu",
);

// SOS 331 — Choreographed Sparks (alternate printing)
const CHOREOGRAPHED_SPARKS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHOREOGRAPHED_SPARKS,
    1,
    "bc3151db-7d49-4945-9c50-91f0e28f803d",
    "Paolo Parente",
);

// SOS 332 — Emeritus of Conflict // Lightning Bolt (alternate printing)
const EMERITUS_OF_CONFLICT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMERITUS_OF_CONFLICT,
    1,
    "f11b8d24-375d-40dc-8497-c1ee779e156a",
    "Alix Branwyn",
);

// SOS 333 — Flashback (alternate printing)
const FLASHBACK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FLASHBACK,
    1,
    "6b7eb857-6617-4672-85e4-f1f5e661db50",
    "Flavio Greco Paglia",
);

// SOS 334 — Maelstrom Artisan // Rocket Volley (alternate printing)
const MAELSTROM_ARTISAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAELSTROM_ARTISAN,
    1,
    "8463c38a-0441-404e-b183-1f871f61f250",
    "Eelis Kyttanen",
);

// SOS 335 — Molten-Core Maestro (alternate printing)
const MOLTEN_CORE_MAESTRO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOLTEN_CORE_MAESTRO,
    1,
    "cb23a7c0-126a-4ed9-b46d-11cfe371c381",
    "Aleksi Briclot",
);

// SOS 336 — Steal the Show (alternate printing)
const STEAL_THE_SHOW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STEAL_THE_SHOW,
    1,
    "eeee78ed-5cb9-4768-87d3-567f1b0ba305",
    "Pauline Voss",
);

// SOS 337 — Ambitious Augmenter (alternate printing)
const AMBITIOUS_AUGMENTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMBITIOUS_AUGMENTER,
    1,
    "0319c982-e257-42fc-bb85-79da3a6dd6c1",
    "Mariah Tekulve",
);

// SOS 338 — Comforting Counsel (alternate printing)
const COMFORTING_COUNSEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COMFORTING_COUNSEL,
    1,
    "2ffdf0cc-aa53-4ec0-bed5-e3a04bbabdc6",
    "Chris Rahn",
);

// SOS 339 — Emeritus of Abundance // Regrowth (alternate printing)
const EMERITUS_OF_ABUNDANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMERITUS_OF_ABUNDANCE,
    1,
    "2b7e8e07-b3ff-496e-923f-42d703f20a1e",
    "Justyna Dura",
);

// SOS 340 — Planar Engineering (alternate printing)
const PLANAR_ENGINEERING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PLANAR_ENGINEERING,
    1,
    "b1a2df2a-4578-46cd-8b6c-802a5af8d082",
    "Liiga Smilshkalne",
);

// SOS 341 — Slumbering Trudge (alternate printing)
const SLUMBERING_TRUDGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLUMBERING_TRUDGE,
    1,
    "5c16cb0f-9e3f-4176-9939-f418cdf4e6fb",
    "Tuan Duong Chu",
);

// SOS 342 — Vastlands Scavenger // Bind to Life (alternate printing)
const VASTLANDS_SCAVENGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VASTLANDS_SCAVENGER,
    1,
    "91db5c3d-45ab-440f-8568-a2837aee1fca",
    "Bryan Sola",
);

// SOS 343 — Applied Geometry (alternate printing)
const APPLIED_GEOMETRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &APPLIED_GEOMETRY,
    1,
    "03b07915-1d88-4f11-ad35-a0cd849de977",
    "Justyna Dura",
);

// SOS 344 — Ark of Hunger (alternate printing)
const ARK_OF_HUNGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARK_OF_HUNGER,
    1,
    "33669848-23ce-4738-9401-469b03f0fd6e",
    "Ksenia Kim",
);

// SOS 345 — Aziza, Mage Tower Captain (alternate printing)
const AZIZA_MAGE_TOWER_CAPTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AZIZA_MAGE_TOWER_CAPTAIN,
    1,
    "ddc2c9fa-c2e4-460f-b043-84e09055421b",
    "Aurore Folny",
);

// SOS 346 — Berta, Wise Extrapolator (alternate printing)
const BERTA_WISE_EXTRAPOLATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BERTA_WISE_EXTRAPOLATOR,
    1,
    "5bead52e-862e-47aa-916c-79585eb2e7ab",
    "Tuan Duong Chu",
);

// SOS 347 — Cauldron of Essence (alternate printing)
const CAULDRON_OF_ESSENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAULDRON_OF_ESSENCE,
    1,
    "96c53f07-b34c-4cc7-a054-f0bdfa48b975",
    "Craig J Spearing",
);

// SOS 348 — Conciliator's Duelist (alternate printing)
const CONCILIATOR_S_DUELIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONCILIATOR_S_DUELIST,
    1,
    "b654da8c-8e93-4d2b-9b10-4b55241bcfbc",
    "Andrew Mar",
);

// SOS 349 — Dina's Guidance (alternate printing)
const DINA_S_GUIDANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DINA_S_GUIDANCE,
    1,
    "f6d3a29c-e698-4af3-897e-5e00e0b55ef5",
    "Manuel Castañón",
);

// SOS 350 — Fix What's Broken (alternate printing)
const FIX_WHAT_S_BROKEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIX_WHAT_S_BROKEN,
    1,
    "0e91ece4-1e22-4b24-86c7-23d105df1abc",
    "Chris Rallis",
);

// SOS 351 — Hardened Academic (alternate printing)
const HARDENED_ACADEMIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARDENED_ACADEMIC,
    1,
    "da72de84-ba2d-4f75-adcd-241d9751c8e4",
    "Vincent Christiaens",
);

// SOS 352 — Mind into Matter (alternate printing)
const MIND_INTO_MATTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIND_INTO_MATTER,
    1,
    "e5886a17-e1ed-4d3f-b12f-392a7b5e7d7d",
    "Joe Slucher",
);

// SOS 353 — Moment of Reckoning (alternate printing)
const MOMENT_OF_RECKONING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOMENT_OF_RECKONING,
    1,
    "bf7fd85e-efdf-4627-997e-02c073683bac",
    "Néstor Ossandón Leal",
);

// SOS 354 — Nita, Forum Conciliator (alternate printing)
const NITA_FORUM_CONCILIATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NITA_FORUM_CONCILIATOR,
    1,
    "3e0c2dec-7649-495c-b2a7-2465c9e7fedb",
    "Jodie Muir",
);

// SOS 355 — Resonating Lute (alternate printing)
const RESONATING_LUTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESONATING_LUTE,
    1,
    "7151c183-24e4-4748-a513-0f2652fdd6ef",
    "Edgar Sánchez Hidalgo",
);

// SOS 356 — Splatter Technique (alternate printing)
const SPLATTER_TECHNIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPLATTER_TECHNIQUE,
    1,
    "266b881f-a0a8-4058-bbfe-cc46518f839d",
    "Tuan Duong Chu",
);

// SOS 357 — Suspend Aggression (alternate printing)
const SUSPEND_AGGRESSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUSPEND_AGGRESSION,
    1,
    "004fdcbe-6dc9-40ad-9467-b89b72a4a8ca",
    "Andreas Zafiratos",
);

// SOS 358 — Traumatic Critique (alternate printing)
const TRAUMATIC_CRITIQUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRAUMATIC_CRITIQUE,
    1,
    "0b79a6ac-85e7-4941-b7d6-e8a6fbbdded8",
    "Aaron Miller",
);

// SOS 359 — Vicious Rivalry (alternate printing)
const VICIOUS_RIVALRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VICIOUS_RIVALRY,
    1,
    "532abbc7-017d-4081-87fc-b0e8bfd804a1",
    "Chris Rallis",
);

// SOS 360 — Zaffai and the Tempests (alternate printing)
const ZAFFAI_AND_THE_TEMPESTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZAFFAI_AND_THE_TEMPESTS,
    1,
    "3f0fff0a-51be-4c3e-b9f5-6bc6999475d1",
    "Olivier Bernard",
);

// SOS 361 — Great Hall of the Biblioplex (alternate printing)
const GREAT_HALL_OF_THE_BIBLIOPLEX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_HALL_OF_THE_BIBLIOPLEX,
    1,
    "a4344235-8598-4a3b-89fe-bd68372b3b2b",
    "Constantin Marin",
);

// SOS 362 — Petrified Hamlet (alternate printing)
const PETRIFIED_HAMLET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PETRIFIED_HAMLET,
    1,
    "ed72d052-d287-448a-bbc5-969ebb1fcb68",
    "Richard Wright",
);

// SOS 363 — Lorehold Charm (alternate printing)
const LOREHOLD_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOREHOLD_CHARM,
    1,
    "6686e584-d8ad-430f-a654-3d74a355c1d1",
    "Ksenia Kim",
);

// SOS 364 — Prismari Charm (alternate printing)
const PRISMARI_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PRISMARI_CHARM,
    1,
    "dad9d807-316a-4707-949c-9b826b971a83",
    "Inkognit",
);

// SOS 365 — Quandrix Charm (alternate printing)
const QUANDRIX_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUANDRIX_CHARM,
    1,
    "97fe8ef0-6de1-4074-b5b5-99c23a31a11b",
    "Matheus Graef",
);

// SOS 366 — Silverquill Charm (alternate printing)
const SILVERQUILL_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILVERQUILL_CHARM,
    1,
    "16cbe8e2-a3ea-4b14-a514-39a2f2d5fe0d",
    "Ksenia Kim",
);

// SOS 367 — Witherbloom Charm (alternate printing)
const WITHERBLOOM_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WITHERBLOOM_CHARM,
    1,
    "f7cf8a5e-3bb9-4853-a6cd-5fdd92be9083",
    "Florian Herold",
);

// SOS 368 — Wisdom of Ages (alternate printing)
const WISDOM_OF_AGES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WISDOM_OF_AGES,
    2,
    "f064a5ff-7139-45e3-9012-cf666e4984c4",
    "Nathaniel Himawan",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &THE_DAWNING_ARCHAIC,
    &RANCOROUS_ARCHAIC,
    &SUNDERING_ARCHAIC,
    &TOGETHER_AS_ONE,
    &TRANSCENDENT_ARCHAIC,
    &AJANI_S_RESPONSE,
    &ANTIQUITIES_ON_THE_LOOSE,
    &ASCENDANT_DUSTSPEAKER,
    &DAYDREAM,
    &DIG_SITE_INVENTORY,
    &EAGER_GLYPHMAGE,
    &ELITE_INTERCEPTOR,
    &EMERITUS_OF_TRUCE,
    &ENNIS_DEBATE_MODERATOR,
    &ERODE,
    &GRADUATION_DAY,
    &GROUP_PROJECT,
    &HARSH_ANNOTATION,
    &HONORBOUND_PAGE,
    &INFORMED_INKWRIGHT,
    &INKSHAPE_DEMONSTRATOR,
    &INTERJECTION,
    &JOINED_RESEARCHERS,
    &OWLIN_HISTORIAN,
    &PRACTICED_OFFENSE,
    &PRIMARY_RESEARCH,
    &QUILL_BLADE_LAUREATE,
    &RAPIER_WIT,
    &REHEARSED_DEBATER,
    &RESTORATION_SEMINAR,
    &SHATTERED_ACOLYTE,
    &SOARING_STONEGLIDER,
    &SPIRITCALL_ENTHUSIAST,
    &STAND_UP_FOR_YOURSELF,
    &STIRRING_HOPESINGER,
    &STONE_DOCENT,
    &SUMMONED_DROMEDARY,
    &BANISHING_BETRAYAL,
    &BRUSH_OFF,
    &CAMPUS_COMPOSER,
    &CHASE_INSPIRATION,
    &DELUGE_VIRTUOSO,
    &DIVERGENT_EQUATION,
    &ECHOCASTING_SYMPOSIUM,
    &EMERITUS_OF_IDEATION,
    &ENCOURAGING_AVIATOR,
    &EXHIBITION_TIDECALLER,
    &FLOW_STATE,
    &FRACTAL_ANOMALY,
    &FRACTALIZE,
    &HARMONIZED_TRIO,
    &HOMESICKNESS,
    &HYDRO_CHANNELER,
    &JADZI_STEWARD_OF_FATE,
    &LANDSCAPE_PAINTER,
    &MANA_SCULPT,
    &MATHEMAGICS,
    &MATTERBENDING_MAGE,
    &MUSE_SEEKER,
    &MUSE_S_ENCOURAGEMENT,
    &ORYSA_TIDE_CHOREOGRAPHER,
    &PENSIVE_PROFESSOR,
    &PROCRASTINATE,
    &RUN_BEHIND,
    &SKYCOACH_CONDUCTOR,
    &SPELLBOOK_SEEKER,
    &TESTER_OF_THE_TANGENTIAL,
    &TEXTBOOK_TABULATOR,
    &WISDOM_OF_AGES,
    &ADVENTUROUS_EATER,
    &ARCANE_OMENS,
    &ARNYN_DEATHBLOOM_BOTANIST,
    &BURROG_BANEMAKER,
    &CHEERFUL_OSTEOMANCER,
    &COST_OF_BRILLIANCE,
    &DECORUM_DISSERTATION,
    &DISSECTION_PRACTICE,
    &EMERITUS_OF_WOE,
    &END_OF_THE_HUNT,
    &ETERNAL_STUDENT,
    &FOOLISH_FATE,
    &FORUM_NECROSCRIBE,
    &GRAVE_RESEARCHER,
    &LECTURING_SCORNMAGE,
    &LEECH_COLLECTOR,
    &MASTERFUL_FLOURISH,
    &MELANCHOLIC_POET,
    &MOSEO_VEIN_S_NEW_DEAN,
    &POISONER_S_APPRENTICE,
    &POSTMORTEM_PROFESSOR,
    &POX_PLAGUE,
    &PULL_FROM_THE_GRAVE,
    &RABID_ATTACK,
    &RAL_ZAREK_GUEST_LECTURER,
    &SCATHING_SHADELOCK,
    &SCHEMING_SILVERTONGUE,
    &SEND_IN_THE_PEST,
    &SNEERING_SHADEWRITER,
    &TRAGEDY_FEASTER,
    &ULNA_ALLEY_SHOPKEEP,
    &WANDER_OFF,
    &WITHERING_CURSE,
    &ARCHAIC_S_AGONY,
    &ARTISTIC_PROCESS,
    &BLAZING_FIRESINGER,
    &CHARGING_STRIFEKNIGHT,
    &CHOREOGRAPHED_SPARKS,
    &DUEL_TACTICS,
    &EMERITUS_OF_CONFLICT,
    &EXPRESSIVE_FIREDANCER,
    &FLASHBACK,
    &GARRISON_EXCAVATOR,
    &GOBLIN_GLASSWRIGHT,
    &HEATED_ARGUMENT,
    &IMPRACTICAL_JOKE,
    &IMPROVISATION_CAPSTONE,
    &LIVING_HISTORY,
    &MAELSTROM_ARTISAN,
    &MAGMABLOOD_ARCHAIC,
    &MICA_READER_OF_RUINS,
    &MOLTEN_CORE_MAESTRO,
    &PIGMENT_WRANGLER,
    &REARING_EMBERMARE,
    &RUBBLE_ROUSER,
    &STEAL_THE_SHOW,
    &STRIFE_SCHOLAR,
    &TABLET_OF_DISCOVERY,
    &TACKLE_ARTIST,
    &THUNDERDRUM_SOLOIST,
    &TOME_BLAST,
    &UNSUBTLE_MOCKERY,
    &ZEALOUS_LORECASTER,
    &ABERRANT_MANAWURM,
    &ADDITIVE_EVOLUTION,
    &AMBITIOUS_AUGMENTER,
    &BURROG_BARRAGE,
    &CHELONIAN_TACKLE,
    &COMFORTING_COUNSEL,
    &EFFLORESCENCE,
    &EMERITUS_OF_ABUNDANCE,
    &EMIL_VASTLANDS_ROAMER,
    &ENVIRONMENTAL_SCIENTIST,
    &FOLLOW_THE_LUMARETS,
    &GERMINATION_PRACTICUM,
    &GLORIOUS_DECAY,
    &HUNGRY_GRAFFALON,
    &INFIRMARY_HEALER,
    &LUMARET_S_FAVOR,
    &MINDFUL_BIOMANCER,
    &NOXIOUS_NEWT,
    &ORACLE_S_RESTORATION,
    &PESTBROOD_SLOTH,
    &PLANAR_ENGINEERING,
    &SHOPKEEPER_S_BANE,
    &SLUMBERING_TRUDGE,
    &SNARL_SONG,
    &STUDIOUS_FIRST_YEAR,
    &TENURED_CONCOCTER,
    &THORNFIST_STRIKER,
    &TOPIARY_LECTURER,
    &VASTLANDS_SCAVENGER,
    &WILD_HYPOTHESIS,
    &WILDGROWTH_ARCHAIC,
    &ZIMONE_S_EXPERIMENT,
    &ABIGALE_POET_LAUREATE,
    &ABSTRACT_PAINTMAGE,
    &APPLIED_GEOMETRY,
    &ARK_OF_HUNGER,
    &AZIZA_MAGE_TOWER_CAPTAIN,
    &BERTA_WISE_EXTRAPOLATOR,
    &BLECH_LOAFING_PEST,
    &BOGWATER_LUMARET,
    &BORROWED_KNOWLEDGE,
    &CAULDRON_OF_ESSENCE,
    &COLORSTORM_STALLION,
    &COLOSSUS_OF_THE_BLOOD_AGE,
    &CONCILIATOR_S_DUELIST,
    &CUBOID_COLONY,
    &DINA_S_GUIDANCE,
    &ELEMENTAL_MASCOT,
    &EMBRACE_THE_PARADOX,
    &ESSENCEKNIT_SCHOLAR,
    &FIX_WHAT_S_BROKEN,
    &FRACTAL_MASCOT,
    &FRACTAL_TENDER,
    &GEOMETER_S_ARTHROPOD,
    &GRAPPLE_WITH_DEATH,
    &GROWTH_CURVE,
    &HARDENED_ACADEMIC,
    &IMPERIOUS_INKMAGE,
    &INKLING_MASCOT,
    &KILLIAN_S_CONFIDENCE,
    &KIROL_HISTORY_BUFF,
    &LLUWEN_EXCHANGE_STUDENT,
    &LOREHOLD_CHARM,
    &LOREHOLD_THE_HISTORIAN,
    &MIND_INTO_MATTER,
    &MIND_ROOTS,
    &MOLTEN_NOTE,
    &MOMENT_OF_RECKONING,
    &NITA_FORUM_CONCILIATOR,
    &OLD_GROWTH_EDUCATOR,
    &PARADOX_SURVEYOR,
    &PEST_MASCOT,
    &PRACTICED_SCROLLSMITH,
    &PRISMARI_CHARM,
    &PRISMARI_THE_INSPIRATION,
    &PROCTOR_S_GAZE,
    &PROFESSOR_DELLIAN_FEL,
    &PTERAFRACTYL,
    &PURSUE_THE_PAST,
    &QUANDRIX_CHARM,
    &QUANDRIX_THE_PROOF,
    &RAPTUROUS_MOMENT,
    &RENDER_SPEECHLESS,
    &RESONATING_LUTE,
    &ROOT_MANIPULATION,
    &SANAR_UNFINISHED_GENIUS,
    &SCOLDING_ADMINISTRATOR,
    &SILVERQUILL_CHARM,
    &SILVERQUILL_THE_DISPUTANT,
    &SNOOPING_PAGE,
    &SOCIAL_SNUB,
    &SPECTACULAR_SKYWHALE,
    &SPIRIT_MASCOT,
    &SPLATTER_TECHNIQUE,
    &STADIUM_TIDALMAGE,
    &STARTLED_RELIC_SLOTH,
    &STIRRING_HONORMANCER,
    &STRESS_DREAM,
    &SUSPEND_AGGRESSION,
    &TAM_OBSERVANT_SEQUENCER,
    &TEACHER_S_PEST,
    &TRAUMATIC_CRITIQUE,
    &VIBRANT_OUTBURST,
    &VICIOUS_RIVALRY,
    &VISIONARY_S_DANCE,
    &WILT_IN_THE_HEAT,
    &WITHERBLOOM_CHARM,
    &WITHERBLOOM_THE_BALANCER,
    &ZAFFAI_AND_THE_TEMPESTS,
    &BIBLIOPLEX_TOMEKEEPER,
    &DIARY_OF_DREAMS,
    &MAGE_TOWER_REFEREE,
    &PAGE_LOOSE_LEAF,
    &POTIONER_S_TROVE,
    &STRIXHAVEN_SKYCOACH,
    &FIELDS_OF_STRIFE,
    &FORUM_OF_AMITY,
    &GREAT_HALL_OF_THE_BIBLIOPLEX,
    &PARADOX_GARDENS,
    &PETRIFIED_HAMLET,
    &SKYCOACH_WAYPOINT,
    &SPECTACLE_SUMMIT,
    &TITAN_S_GRAVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ESSENCE_SCATTER_REPRINT,
    QUICK_STUDY_REPRINT,
    LAST_GASP_REPRINT,
    ANCESTRAL_ANGER_REPRINT,
    SEIZE_THE_SPOILS_REPRINT,
    DEATHCAP_GLADE_REPRINT,
    DREAMROOT_CASCADE_REPRINT,
    SHATTERED_SANCTUM_REPRINT,
    STORMCARVED_COAST_REPRINT,
    SUNDOWN_PASS_REPRINT,
    TERRAMORPHIC_EXPANSE_REPRINT,
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
    RAL_ZAREK_GUEST_LECTURER_ALTERNATE_1,
    PROFESSOR_DELLIAN_FEL_ALTERNATE_1,
    LOREHOLD_THE_HISTORIAN_ALTERNATE_1,
    PRISMARI_THE_INSPIRATION_ALTERNATE_1,
    QUANDRIX_THE_PROOF_ALTERNATE_1,
    SILVERQUILL_THE_DISPUTANT_ALTERNATE_1,
    WITHERBLOOM_THE_BALANCER_ALTERNATE_1,
    THE_DAWNING_ARCHAIC_ALTERNATE_1,
    RESTORATION_SEMINAR_ALTERNATE_1,
    ECHOCASTING_SYMPOSIUM_ALTERNATE_1,
    DECORUM_DISSERTATION_ALTERNATE_1,
    TRAGEDY_FEASTER_ALTERNATE_1,
    IMPROVISATION_CAPSTONE_ALTERNATE_1,
    MAGMABLOOD_ARCHAIC_ALTERNATE_1,
    GERMINATION_PRACTICUM_ALTERNATE_1,
    WILDGROWTH_ARCHAIC_ALTERNATE_1,
    BLECH_LOAFING_PEST_ALTERNATE_1,
    COLORSTORM_STALLION_ALTERNATE_1,
    GEOMETER_S_ARTHROPOD_ALTERNATE_1,
    DEATHCAP_GLADE_ALTERNATE_1,
    DREAMROOT_CASCADE_ALTERNATE_1,
    SHATTERED_SANCTUM_ALTERNATE_1,
    STORMCARVED_COAST_ALTERNATE_1,
    SUNDOWN_PASS_ALTERNATE_1,
    EMERITUS_OF_IDEATION_ALTERNATE_1,
    TOGETHER_AS_ONE_ALTERNATE_1,
    ANTIQUITIES_ON_THE_LOOSE_ALTERNATE_1,
    EMERITUS_OF_TRUCE_ALTERNATE_1,
    ERODE_ALTERNATE_1,
    INFORMED_INKWRIGHT_ALTERNATE_1,
    JOINED_RESEARCHERS_ALTERNATE_1,
    PRACTICED_OFFENSE_ALTERNATE_1,
    STIRRING_HOPESINGER_ALTERNATE_1,
    EMERITUS_OF_IDEATION_ALTERNATE_2,
    EXHIBITION_TIDECALLER_ALTERNATE_1,
    HARMONIZED_TRIO_ALTERNATE_1,
    JADZI_STEWARD_OF_FATE_ALTERNATE_1,
    MANA_SCULPT_ALTERNATE_1,
    MATHEMAGICS_ALTERNATE_1,
    PENSIVE_PROFESSOR_ALTERNATE_1,
    SKYCOACH_CONDUCTOR_ALTERNATE_1,
    WISDOM_OF_AGES_ALTERNATE_1,
    EMERITUS_OF_WOE_ALTERNATE_1,
    GRAVE_RESEARCHER_ALTERNATE_1,
    MOSEO_VEIN_S_NEW_DEAN_ALTERNATE_1,
    POSTMORTEM_PROFESSOR_ALTERNATE_1,
    POX_PLAGUE_ALTERNATE_1,
    SCHEMING_SILVERTONGUE_ALTERNATE_1,
    WITHERING_CURSE_ALTERNATE_1,
    CHOREOGRAPHED_SPARKS_ALTERNATE_1,
    EMERITUS_OF_CONFLICT_ALTERNATE_1,
    FLASHBACK_ALTERNATE_1,
    MAELSTROM_ARTISAN_ALTERNATE_1,
    MOLTEN_CORE_MAESTRO_ALTERNATE_1,
    STEAL_THE_SHOW_ALTERNATE_1,
    AMBITIOUS_AUGMENTER_ALTERNATE_1,
    COMFORTING_COUNSEL_ALTERNATE_1,
    EMERITUS_OF_ABUNDANCE_ALTERNATE_1,
    PLANAR_ENGINEERING_ALTERNATE_1,
    SLUMBERING_TRUDGE_ALTERNATE_1,
    VASTLANDS_SCAVENGER_ALTERNATE_1,
    APPLIED_GEOMETRY_ALTERNATE_1,
    ARK_OF_HUNGER_ALTERNATE_1,
    AZIZA_MAGE_TOWER_CAPTAIN_ALTERNATE_1,
    BERTA_WISE_EXTRAPOLATOR_ALTERNATE_1,
    CAULDRON_OF_ESSENCE_ALTERNATE_1,
    CONCILIATOR_S_DUELIST_ALTERNATE_1,
    DINA_S_GUIDANCE_ALTERNATE_1,
    FIX_WHAT_S_BROKEN_ALTERNATE_1,
    HARDENED_ACADEMIC_ALTERNATE_1,
    MIND_INTO_MATTER_ALTERNATE_1,
    MOMENT_OF_RECKONING_ALTERNATE_1,
    NITA_FORUM_CONCILIATOR_ALTERNATE_1,
    RESONATING_LUTE_ALTERNATE_1,
    SPLATTER_TECHNIQUE_ALTERNATE_1,
    SUSPEND_AGGRESSION_ALTERNATE_1,
    TRAUMATIC_CRITIQUE_ALTERNATE_1,
    VICIOUS_RIVALRY_ALTERNATE_1,
    ZAFFAI_AND_THE_TEMPESTS_ALTERNATE_1,
    GREAT_HALL_OF_THE_BIBLIOPLEX_ALTERNATE_1,
    PETRIFIED_HAMLET_ALTERNATE_1,
    LOREHOLD_CHARM_ALTERNATE_1,
    PRISMARI_CHARM_ALTERNATE_1,
    QUANDRIX_CHARM_ALTERNATE_1,
    SILVERQUILL_CHARM_ALTERNATE_1,
    WITHERBLOOM_CHARM_ALTERNATE_1,
    WISDOM_OF_AGES_ALTERNATE_2,
];
