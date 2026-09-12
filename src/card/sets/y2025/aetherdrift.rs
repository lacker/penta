//! Aetherdrift card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::AdditionalCostObjectIndex;
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
use crate::card::BasicLandType;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureStats;
use crate::card::CreatureTypeSetDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ManaTypeDef;
use crate::card::MoveToZoneCostDef;
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
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
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
use crate::card::sets::y2009::zendikar as catalog_zen;
use crate::card::sets::y2013::theros as catalog_ths;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2020::zendikar_rising as catalog_znr;

pub const EXHAUST: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:exhaust");

/// Exhaust labels an ordinary ability restricted to one activation per object.
///
/// # Panics
///
/// Panics if the clause is not an activated ability or activated mana ability.
#[must_use]
pub const fn exhaust(ability: AbilityDef) -> AbilityDef {
    ability.once_per_object().labeled(EXHAUST)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DFT",
    slug: "aetherdrift",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("ba7638ef-114b-4055-9855-390f82b7d5c5", "Racrufi"),
);

const THOPTER_TOKEN: TokenCharacteristics =
    TokenCharacteristics::artifact_creature(&["Thopter"], &[], 1, 1)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "d38fc294-ad86-441e-96fe-4ca286a11218",
            "Kev Fang",
        ));
const ELEPHANT_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Elephant"], &[ManaColor::Green], 3, 3).with_art(
        CardArt::new("6ecb6655-2aa0-4622-ae9a-21dfffa7625e", "Milivoj Ćeran"),
    );
const THOPTER_TOKEN_2: TokenCharacteristics =
    TokenCharacteristics::artifact_creature(&["Thopter"], &[], 1, 1)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "d38fc294-ad86-441e-96fe-4ca286a11218",
            "Kev Fang",
        ));

// DFT 1 — Air Response Unit
pub(in crate::card::sets) static AIR_RESPONSE_UNIT: CardRecord = CardRecord::new(
    "Air Response Unit",
    "d77c8e29-de24-4664-baf8-959608dd99ca",
    "Brock Grossman",
    CardRules::new_vehicle(mana_cost!("{2}{W}"), 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 2 — Alacrian Armory
pub(in crate::card::sets) static ALACRIAN_ARMORY: CardRecord = CardRecord::new(
    "Alacrian Armory",
    "d39f7f98-ad5e-4e5e-9f7b-abe0984ffe17",
    "Artur Nakhodkin",
    CardRules::new_artifact(mana_cost!("{3}{W}")).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures you control get +0/+1 and have vigilance.",
            EffectDef::StaticApply {
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
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
            },
        ),
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, choose up to one \
             target Mount or Vehicle you control. Until end of turn, that \
             permanent becomes saddled if it's a Mount and becomes an \
             artifact creature if it's a Vehicle.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                    },
                    then: &EffectDef::Saddle {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// DFT 3 — Basri, Tomorrow's Champion
pub(in crate::card::sets) static BASRI_TOMORROW_S_CHAMPION: CardRecord = CardRecord::new(
    "Basri, Tomorrow's Champion",
    "991270fa-a391-4c2e-bd9a-19151386fb67",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Knight"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "{W}, {T}, Exert Basri: Create a 1/1 white Cat creature token \
                 with lifelink. (An exerted creature won't untap during your \
                 next untap step.)",
                &[
                    CostDef::Mana(mana_cost!("{W}")),
                    CostDef::TapSource,
                    CostDef::ExertSource,
                ],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Cat"], &[ManaColor::White], 1, 1)
                        .with_abilities(&[abilities::lifelink()]),
                ))),
            ),
            abilities::cycling!(
                "Cycling {2}{W} ({2}{W}, Discard this card: Draw a card.)",
                &[CostDef::Mana(mana_cost!("{2}{W}"))]
            ),
            AbilityDef::triggered(
                "When you cycle this card, Cats you control gain hexproof and \
                 indestructible until end of turn.",
                TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
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

// DFT 4 — Brightfield Glider
pub(in crate::card::sets) static BRIGHTFIELD_GLIDER: CardRecord = CardRecord::new(
    "Brightfield Glider",
    "7eb819eb-ba5c-4449-87b5-3894380558bc",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{W}"), &["Possum", "Mount"], 1, 1).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, it gets +1/+2 \
             and gains flying until end of turn.",
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
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
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

// DFT 5 — Brightfield Mustang
pub(in crate::card::sets) static BRIGHTFIELD_MUSTANG: CardRecord = CardRecord::new(
    "Brightfield Mustang",
    "b2c7cacc-f15e-46c1-9c25-b567bb3e8680",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Horse", "Mount"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, untap it and \
             put a +1/+1 counter on it.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 1 }],
            "Saddle 1 (Tap any number of other creatures you control with \
             total power 1 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// DFT 6 — Broadcast Rambler
pub(in crate::card::sets) static BROADCAST_RAMBLER: CardRecord = CardRecord::new(
    "Broadcast Rambler",
    "89ce2385-e33d-47b3-96c8-5a4672d9df7c",
    "Ioannis Fiore",
    CardRules::new_vehicle(mana_cost!("{4}{W}"), 5, 4).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, create a 1/1 colorless Thopter \
             artifact creature token with flying.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(THOPTER_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 7 — Bulwark Ox
pub(in crate::card::sets) static BULWARK_OX: CardRecord = CardRecord::new(
    "Bulwark Ox",
    "106944b2-f3ae-4350-be33-61b9f92fc92f",
    "Brent Hollowell",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Ox", "Mount"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks while saddled, put a +1/+1 \
             counter on target creature.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
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
        AbilityDef::activated(
            "Sacrifice this creature: Creatures you control with counters \
             on them gain hexproof and indestructible until end of turn.",
            &[CostDef::SacrificeSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasAnyCounter,
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
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 1 }],
            "Saddle 1 (Tap any number of other creatures you control with \
             total power 1 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// DFT 8 — Canyon Vaulter
// Audit: unsupported — Needs a committed crew/saddle contribution event naming the paying creature and the Mount or Vehicle during the main phase; the current total-power tap payment only emits ordinary tapped events and does not retain that relationship.
pub(in crate::card::sets) static CANYON_VAULTER: CardRecord = CardRecord::new(
    "Canyon Vaulter",
    "cc0b15da-a45c-42f5-aafc-20ad9e38bf24",
    "David Astruga",
    CardRules::unsupported(),
);

// DFT 9 — Cloudspire Captain
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static CLOUDSPIRE_CAPTAIN: CardRecord = CardRecord::new(
    "Cloudspire Captain",
    "3380d87a-c460-409c-8d47-9b2fc5ddd2ea",
    "Manny Edeko",
    CardRules::unsupported(),
);

// DFT 10 — Collision Course
pub(in crate::card::sets) static COLLISION_COURSE: CardRecord = CardRecord::new(
    "Collision Course",
    "6b60da34-b622-42de-a249-79545bcbf30d",
    "Konstantin Porubov",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Collision Course deals X damage to target creature, where X \
                 is the number of permanents you control that are creatures \
                 and/or Vehicles.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
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

// DFT 11 — Daring Mechanic
pub(in crate::card::sets) static DARING_MECHANIC: CardRecord = CardRecord::new(
    "Daring Mechanic",
    "3382552c-2740-409a-83a1-80b60627beb8",
    "Elizabeth Peiró",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Artificer"], 3, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}{W}: Put a +1/+1 counter on target Mount or Vehicle.",
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
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

// DFT 12 — Detention Chariot
// Audit: unsupported — Needs an exile-until-source-leaves duration that returns the card immediately when the duration ends (CR 610.3); an ordinary leaves trigger would return it later through the stack.
pub(in crate::card::sets) static DETENTION_CHARIOT: CardRecord = CardRecord::new(
    "Detention Chariot",
    "75d5e64f-7af2-4cb4-abd1-23992e346bee",
    "Adrián Rodríguez Pérez",
    CardRules::unsupported(),
);

// DFT 13 — Gallant Strike
pub(in crate::card::sets) static GALLANT_STRIKE: CardRecord = CardRecord::new(
    "Gallant Strike",
    "9bdb58b7-e1ef-496b-b8dd-d1fabf3d2e7a",
    "Brent Hollowell",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature with toughness 4 or greater.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ToughnessGreaterThan(ValueDef::Constant(3)),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 14 — Gloryheath Lynx
pub(in crate::card::sets) static GLORYHEATH_LYNX: CardRecord = CardRecord::new(
    "Gloryheath Lynx",
    "ea3ac678-8b74-4865-a896-c42692c02341",
    "Deruchenko Alexander",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Mount"], 2, 3).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, search your \
             library for a basic Plains card, reveal it, put it into your \
             hand, then shuffle.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains")),
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
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 2 }],
            "Saddle 2 (Tap any number of other creatures you control with \
             total power 2 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// DFT 15 — Guardian Sunmare
pub(in crate::card::sets) static GUARDIAN_SUNMARE: CardRecord = CardRecord::new(
    "Guardian Sunmare",
    "7c274595-94e2-4587-9cef-b38639d6429a",
    "Christina Kraus",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Horse", "Mount"], 5, 5).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, search your \
             library for a nonland permanent card with mana value 3 or \
             less, put it onto the battlefield, then shuffle.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
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
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 4 }],
            "Saddle 4",
        ),
    ]),
);

// DFT 16 — Guidelight Synergist
pub(in crate::card::sets) static GUIDELIGHT_SYNERGIST: CardRecord = CardRecord::new(
    "Guidelight Synergist",
    "fdaeea2c-d8aa-416f-95d6-6af888591fdf",
    "Camille Alquier",
    CardRules::new_artifact_creature(mana_cost!("{3}{W}"), &["Robot", "Artificer"], 0, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "This creature gets +1/+0 for each artifact you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// DFT 17 — Interface Ace
// Audit: unsupported — Needs a toughness-based contribution rule for both crew and saddle payments; current total-power tap payments read power plus Vehicle-only power bonuses.
pub(in crate::card::sets) static INTERFACE_ACE: CardRecord = CardRecord::new(
    "Interface Ace",
    "fcfd487a-a9e6-44e3-80af-bc384316106f",
    "Wonchun Choi",
    CardRules::unsupported(),
);

// DFT 18 — Leonin Surveyor
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static LEONIN_SURVEYOR: CardRecord = CardRecord::new(
    "Leonin Surveyor",
    "e08e4107-213f-491b-a032-8e3367009ba8",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// DFT 19 — Lightshield Parry
pub(in crate::card::sets) static LIGHTSHIELD_PARRY: CardRecord = CardRecord::new(
    "Lightshield Parry",
    "dcf6a0e7-1fd4-425f-b634-c93236daea35",
    "Leanna Crossan",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+2 until end of turn.",
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
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 20 — Lightwheel Enhancements
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static LIGHTWHEEL_ENHANCEMENTS: CardRecord = CardRecord::new(
    "Lightwheel Enhancements",
    "9ab169c1-4e25-4a5d-8961-4f06298c3781",
    "Yeong-Hao Han",
    CardRules::unsupported(),
);

// DFT 21 — Lotusguard Disciple
pub(in crate::card::sets) static LOTUSGUARD_DISCIPLE: CardRecord = CardRecord::new(
    "Lotusguard Disciple",
    "80645651-3804-481f-8f8f-ade762a011e1",
    "Josiah \"Jo\" Cameron",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Bird", "Cleric"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature or Vehicle gains \
             lifelink and indestructible until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DFT 22 — Nesting Bot
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static NESTING_BOT: CardRecord = CardRecord::new(
    "Nesting Bot",
    "7829c0ae-f72f-4195-ad43-775d7218565c",
    "Racrufi",
    CardRules::unsupported(),
);

// DFT 23 — Perilous Snare
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed. Also needs the exile-until-source-leaves duration returning its exiled card immediately when that duration ends.
pub(in crate::card::sets) static PERILOUS_SNARE: CardRecord = CardRecord::new(
    "Perilous Snare",
    "47f7e468-2196-4960-a612-37ab326e2a17",
    "Chris Seaman",
    CardRules::unsupported(),
);

// DFT 24 — Pride of the Road
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static PRIDE_OF_THE_ROAD: CardRecord = CardRecord::new(
    "Pride of the Road",
    "4172222f-d871-4354-9a02-7af0001d8956",
    "Alfonso Santano",
    CardRules::unsupported(),
);

// DFT 25 — Ride's End
// Audit: unsupported — Needs a self spell-cost condition inspecting whether any declared target is tapped; existing spell-cost predicates only test targeting the external cost source.
pub(in crate::card::sets) static RIDE_S_END: CardRecord = CardRecord::new(
    "Ride's End",
    "2f96b33b-c952-45ac-9626-40169b2bd4ef",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// DFT 26 — Roadside Assistance
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static ROADSIDE_ASSISTANCE: CardRecord = CardRecord::new(
    "Roadside Assistance",
    "8f2a9154-7b43-4b8d-9d81-d11cfda5d597",
    "Artur Nakhodkin",
    CardRules::unsupported(),
);

// DFT 27 — Salvation Engine
pub(in crate::card::sets) static SALVATION_ENGINE: CardRecord = CardRecord::new(
    "Salvation Engine",
    "34ed1bf2-0f3c-4528-b570-e5bdcd7ffda9",
    "Ben Wootten",
    CardRules::new_vehicle(mana_cost!("{4}{W}"), 6, 10).with_abilities(&[
        AbilityDef::static_ability(
            "Other artifact creatures you control get +2/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            },
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this Vehicle attacks, return up to one target \
             artifact card from your graveyard to the battlefield.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
        abilities::crew("Crew 6", 6),
    ]),
);

// DFT 28 — Skyseer's Chariot
// Audit: unsupported — Needs a generic activation-cost increase applying to matching named sources in every zone and to mana abilities; existing cost modification structures do not cover that activation family.
pub(in crate::card::sets) static SKYSEER_S_CHARIOT: CardRecord = CardRecord::new(
    "Skyseer's Chariot",
    "96ed5b66-8e74-4a90-ad4e-c39d15993994",
    "Carl Critchlow",
    CardRules::unsupported(),
);

// DFT 29 — Spectacular Pileup
pub(in crate::card::sets) static SPECTACULAR_PILEUP: CardRecord = CardRecord::new(
    "Spectacular Pileup",
    "a24a6309-0e69-45f0-a9ff-44d4997e7e4d",
    "Zezhou Chen",
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "All creatures and Vehicles lose indestructible until end of \
             turn, then destroy all creatures and Vehicles.",
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Indestructible,
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    then: None,
                },
            ]),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 30 — Spotcycle Scouter
pub(in crate::card::sets) static SPOTCYCLE_SCOUTER: CardRecord = CardRecord::new(
    "Spotcycle Scouter",
    "f0489108-75b7-441c-888d-12987c0c1080",
    "Josiah \"Jo\" Cameron",
    CardRules::new_vehicle(mana_cost!("{1}{W}"), 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, scry 2. (Look at the top two cards \
             of your library, then put any number of them on the bottom \
             and the rest on top in any order.)",
            abilities::scry(ValueDef::Constant(2)),
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 31 — Sundial, Dawn Tyrant
pub(in crate::card::sets) static SUNDIAL_DAWN_TYRANT: CardRecord = CardRecord::new(
    "Sundial, Dawn Tyrant",
    "b2e5435c-52f3-42d7-bcee-5aa13afd6626",
    "Bruce Brenneise",
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Construct"], 3, 3)
        .with_supertype(CardSupertype::Legendary),
);

// DFT 32 — Swiftwing Assailant
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static SWIFTWING_ASSAILANT: CardRecord = CardRecord::new(
    "Swiftwing Assailant",
    "72db9bb9-d930-40e5-b144-01ebfd377996",
    "Pig Hands",
    CardRules::unsupported(),
);

// DFT 33 — Tune Up
pub(in crate::card::sets) static TUNE_UP: CardRecord = CardRecord::new(
    "Tune Up",
    "f8bddc5f-8f25-4313-b5bb-e5eae2923878",
    "Chris Rallis",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target artifact card from your graveyard to the \
         battlefield. If it's a Vehicle, it becomes an artifact \
         creature.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::WithZoneMoveResult {
            effect: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            binding: crate::Binding!("returned"),
            then: &EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Matching {
                    objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                        "returned"
                    )),
                    object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Subtype(
                        SubtypeDef::Literal("Vehicle"),
                    )),
                }),
                effect: AppliedEffectDef::add_card_types(
                    CardTypeSet::single(CardType::Artifact)
                        .union(CardTypeSet::single(CardType::Creature)),
                ),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        },
    )]),
);

// DFT 34 — Unswerving Sloth
pub(in crate::card::sets) static UNSWERVING_SLOTH: CardRecord = CardRecord::new(
    "Unswerving Sloth",
    "12296a74-5d60-4ee3-aa53-2289f84da776",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Sloth", "Mount"], 5, 5).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, it gains \
             indestructible until end of turn. Untap all creatures you \
             control.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                },
            ]),
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 4 }],
            "Saddle 4 (Tap any number of other creatures you control with \
             total power 4 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// DFT 35 — Valor's Flagship
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static VALOR_S_FLAGSHIP: CardRecord = CardRecord::new(
    "Valor's Flagship",
    "8af1dddf-6c95-448b-acc8-df5a99202e9a",
    "Stephan Martiniere",
    CardRules::unsupported(),
);

// DFT 36 — Voyager Glidecar
pub(in crate::card::sets) static VOYAGER_GLIDECAR: CardRecord = CardRecord::new(
    "Voyager Glidecar",
    "13eb445a-dd41-4760-8299-9ba5d6de6aaf",
    "Eduardo Francisco",
    CardRules::new_vehicle(mana_cost!("{W}"), 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, scry 1.",
            abilities::scry(ValueDef::Constant(1)),
        ),
        AbilityDef::activated(
            "Tap three other untapped creatures you control: Until end of \
             turn, this Vehicle becomes an artifact creature and gains \
             flying. Put a +1/+1 counter on it.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
                count: 3,
            }],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        abilities::crew("Crew 1", 1),
    ]),
);

// DFT 37 — Voyager Quickwelder
pub(in crate::card::sets) static VOYAGER_QUICKWELDER: CardRecord = CardRecord::new(
    "Voyager Quickwelder",
    "f6dcdc8c-fba1-4ea1-bf93-65072d10f0da",
    "Kenn Yap",
    CardRules::new_artifact_creature(mana_cost!("{2}{W}"), &["Robot", "Artificer"], 2, 4)
        .with_abilities(&[AbilityDef::static_ability(
            "Artifact spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::HasType(CardType::Artifact),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        )]),
);

// DFT 38 — Aether Syphon
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static AETHER_SYPHON: CardRecord = CardRecord::new(
    "Aether Syphon",
    "d7033739-4cd8-4727-b9b5-099fb597006b",
    "Martin de Diego Sádaba",
    CardRules::unsupported(),
);

// DFT 39 — Bounce Off
pub(in crate::card::sets) static BOUNCE_OFF: CardRecord = CardRecord::new(
    "Bounce Off",
    "7b3c8dda-2405-4879-8dd1-e790a833c42d",
    "Deruchenko Alexander",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature or Vehicle to its owner's hand.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
            ]),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )]),
);

// DFT 40 — Caelorna, Coral Tyrant
pub(in crate::card::sets) static CAELORNA_CORAL_TYRANT: CardRecord = CardRecord::new(
    "Caelorna, Coral Tyrant",
    "e8654e38-4230-4094-b815-778bfb5d06f2",
    "Deruchenko Alexander",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Octopus"], 0, 8)
        .with_supertype(CardSupertype::Legendary),
);

// DFT 41 — Diversion Unit
pub(in crate::card::sets) static DIVERSION_UNIT: CardRecord = CardRecord::new(
    "Diversion Unit",
    "e04d4fa6-1fa3-4bfd-a462-47c23ccf9124",
    "Xabi Gaztelua",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}"), &["Robot"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{U}, Sacrifice this creature: Counter target instant or \
             sorcery spell unless its controller pays {3}.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{3}"))]),
        ),
    ]),
);

// DFT 42 — Flood the Engine
pub(in crate::card::sets) static FLOOD_THE_ENGINE: CardRecord = CardRecord::new(
    "Flood the Engine",
    "57402f7c-5d4c-4f1e-8bce-a2328a297111",
    "Eric Wilkerson",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature or Vehicle",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                )],
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
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted permanent.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// DFT 43 — Gearseeker Serpent (reprint)
const GEARSEEKER_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::GEARSEEKER_SERPENT,
    "3dca0007-42d3-4ee2-8e88-361d80a7103c",
    "J.P. Targete",
);

// DFT 44 — Glitch Ghost Surveyor
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static GLITCH_GHOST_SURVEYOR: CardRecord = CardRecord::new(
    "Glitch Ghost Surveyor",
    "b9bb89b9-50dd-4b36-aa10-aba585e50246",
    "Johan Grenier",
    CardRules::unsupported(),
);

// DFT 45 — Guidelight Optimizer
// Audit: unsupported — Needs a union of mana spending permissions: cast an artifact spell or activate any ability; current mana restrictions allow one use family, while CannotCastSpell would also allow unrelated resolving payments.
pub(in crate::card::sets) static GUIDELIGHT_OPTIMIZER: CardRecord = CardRecord::new(
    "Guidelight Optimizer",
    "e9fc07dd-05b1-49ed-a3ee-46c31b8e0a3d",
    "Mirko Failoni",
    CardRules::unsupported(),
);

// DFT 46 — Howler's Heavy
pub(in crate::card::sets) static HOWLER_S_HEAVY: CardRecord = CardRecord::new(
    "Howler's Heavy",
    "8bbd7758-59c7-4ae1-9af8-c3580f4aa958",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Seal", "Pirate"], 3, 4).with_abilities(&[
        abilities::cycling!(
            "Cycling {1}{U} ({1}{U}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{1}{U}"))]
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, target creature or Vehicle an \
             opponent controls gets -3/-0 until end of turn.",
            TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
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

// DFT 47 — Hulldrifter
pub(in crate::card::sets) static HULLDRIFTER: CardRecord = CardRecord::new(
    "Hulldrifter",
    "402666f8-c9cc-4e8f-abaa-6c38be90cdd2",
    "Alexandre Honoré",
    CardRules::new_vehicle(mana_cost!("{3}{U}{U}"), 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this Vehicle enters, draw two cards.",
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
        abilities::crew(
            "Crew 3 (Tap any number of creatures you control with total \
             power 3 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            3,
        ),
    ]),
);

// DFT 48 — Keen Buccaneer
pub(in crate::card::sets) static KEEN_BUCCANEER: CardRecord = CardRecord::new(
    "Keen Buccaneer",
    "0ed90a63-5aca-470a-8e1e-518d8aeb6d91",
    "Mirko Failoni",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Octopus", "Pirate"], 2, 3).with_abilities(&[
        abilities::vigilance(),
        exhaust(AbilityDef::activated(
            "Exhaust — {1}{U}: Draw a card, then discard a card. Put a \
             +1/+1 counter on this creature. (Activate each exhaust \
             ability only once.)",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )),
    ]),
);

// DFT 49 — Memory Guardian
pub(in crate::card::sets) static MEMORY_GUARDIAN: CardRecord = CardRecord::new(
    "Memory Guardian",
    "6b199ce2-0ea0-47e5-a36c-36373be53fec",
    "Hardy Fowler",
    CardRules::new_artifact_creature(mana_cost!("{4}{U}"), &["Robot", "Artificer"], 3, 4)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Affinity for artifacts (This spell costs {1} less to cast for \
                 each artifact you control.)",
                EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            )
            .with_source_zones(&[ZoneKind::Hand]),
            abilities::flying(),
        ]),
);

// DFT 50 — Midnight Mangler
pub(in crate::card::sets) static MIDNIGHT_MANGLER: CardRecord = CardRecord::new(
    "Midnight Mangler",
    "237568e3-7331-4bbb-a091-a766723134fc",
    "Villarrte",
    CardRules::new_vehicle(mana_cost!("{1}{U}"), 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "During turns other than yours, this Vehicle is an artifact \
             creature.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                },
            },
        ),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
    ]),
);

// DFT 51 — Mindspring Merfolk
pub(in crate::card::sets) static MINDSPRING_MERFOLK: CardRecord = CardRecord::new(
    "Mindspring Merfolk",
    "b6250b8b-1943-445f-ada9-30b41eb6d29b",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk", "Wizard"], 1, 1).with_abilities(&[
        exhaust(AbilityDef::activated(
            "Exhaust — {X}{U}{U}, {T}: Draw X cards. Put a +1/+1 counter \
             on each Merfolk creature you control. (Activate each exhaust \
             ability only once.)",
            &[CostDef::Mana(mana_cost!("{X}{U}{U}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::ChosenX),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )),
    ]),
);

// DFT 52 — Mu Yanling, Wind Rider
// Audit: unsupported — Needs a single trigger per damaged player for a simultaneous batch from one or more controlled flying creatures; current damage grouping coalesces only an unfiltered Any-source matcher and cannot group this source predicate.
pub(in crate::card::sets) static MU_YANLING_WIND_RIDER: CardRecord = CardRecord::new(
    "Mu Yanling, Wind Rider",
    "76423446-d62f-4cc5-a23a-3175be88bd73",
    "Justyna Dura",
    CardRules::unsupported(),
);

// DFT 53 — Nimble Thopterist
pub(in crate::card::sets) static NIMBLE_THOPTERIST: CardRecord = CardRecord::new(
    "Nimble Thopterist",
    "47717312-f6c6-4e86-ba6a-a30698962430",
    "Ioannis Fiore",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Vedalken", "Artificer"], 3, 2).with_abilities(
        &[abilities::enters_trigger(
            "When this creature enters, create a 1/1 colorless Thopter \
             artifact creature token with flying.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(THOPTER_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        )],
    ),
);

// DFT 54 — Possession Engine
// Audit: unsupported — Needs control and attack/block restriction durations ending when this player stops controlling the source Vehicle; current continuous duration vocabulary tracks source existence or tapping, not that control relationship.
pub(in crate::card::sets) static POSSESSION_ENGINE: CardRecord = CardRecord::new(
    "Possession Engine",
    "f206b0a1-50d8-4d53-850d-fb15fd328267",
    "Leroy Steinmann",
    CardRules::unsupported(),
);

// DFT 55 — Rangers' Refueler
// Audit: unsupported — Needs a committed activation event filtered by the exhaust mechanic label, including mana abilities; current trigger events do not observe labeled ability activations.
pub(in crate::card::sets) static RANGERS_REFUELER: CardRecord = CardRecord::new(
    "Rangers' Refueler",
    "67d2d713-8acb-4e3d-bd1d-0416fe9b9ef6",
    "Samuel Perin",
    CardRules::unsupported(),
);

// DFT 56 — Repurposing Bay
pub(in crate::card::sets) static REPURPOSING_BAY: CardRecord = CardRecord::new(
    "Repurposing Bay",
    "0cf1ace1-b7f5-4bd9-a494-ee7cb6c1f854",
    "William Tempest",
    CardRules::new_artifact(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::activated(
        "{2}, {T}, Sacrifice another artifact: Search your library for \
         an artifact card with mana value equal to 1 plus the \
         sacrificed artifact's mana value, put that card onto the \
         battlefield, then shuffle. Activate only as a sorcery.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::TapSource,
            CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ])),
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::ManaValueEqualTo(ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(1),
                    ValueDef::SacrificedManaValue,
                ))),
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
    )
    .with_activation_timing(ActivationTimingDef::SorcerySpeed)]),
);

// DFT 57 — Riverchurn Monument
pub(in crate::card::sets) static RIVERCHURN_MONUMENT: CardRecord = CardRecord::new(
    "Riverchurn Monument",
    "e66ff696-fd39-49ad-9ee5-c0868167df37",
    "Anthony Devine",
    CardRules::new_artifact(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Any number of target players each mill two cards. \
             (Each of them puts the top two cards of their library into \
             their graveyard.)",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            &[
                AbilityTargetDef::up_to(AbilityTargetPredicate::Player(PlayerRelation::Any), 1),
                AbilityTargetDef {
                    another: true,
                    ..AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                        1,
                    )
                },
            ],
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex(0)),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex(1)),
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
        exhaust(AbilityDef::activated_with_targets(
            "Exhaust — {2}{U}{U}, {T}: Any number of target players each \
             mill cards equal to the number of cards in their graveyard. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{2}{U}{U}")), CostDef::TapSource],
            &[
                AbilityTargetDef::up_to(AbilityTargetPredicate::Player(PlayerRelation::Any), 1),
                AbilityTargetDef {
                    another: true,
                    ..AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                        1,
                    )
                },
            ],
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex(0)),
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex(0))),
                    )),
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex(1)),
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex(1))),
                    )),
                },
            ]),
        )),
    ]),
);

// DFT 58 — Roadside Blowout
// Audit: unsupported — Needs a self spell-cost condition inspecting whether a declared target has mana value 1; existing spell-cost predicates only test targeting the external cost source.
pub(in crate::card::sets) static ROADSIDE_BLOWOUT: CardRecord = CardRecord::new(
    "Roadside Blowout",
    "d6153a76-56f7-46ee-bba5-b62c0143388a",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// DFT 59 — Sabotage Strategist
// Audit: unsupported — Needs a grouped attack declaration event carrying exactly the creatures attacking this player, with a bound set retained for resolution; current attack events do not publish that filtered attacker group.
pub(in crate::card::sets) static SABOTAGE_STRATEGIST: CardRecord = CardRecord::new(
    "Sabotage Strategist",
    "c8bb15e2-e1ad-4645-aab0-df4a1a68563d",
    "Darren Tan",
    CardRules::unsupported(),
);

// DFT 60 — Scrounging Skyray
pub(in crate::card::sets) static SCROUNGING_SKYRAY: CardRecord = CardRecord::new(
    "Scrounging Skyray",
    "d60bece3-6f63-4d9e-bca0-cef2d38f1472",
    "Ron Spears",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Fish", "Pirate"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you discard one or more cards, put that many +1/+1 \
             counters on this creature.",
            TriggerEventDef::DiscardedCards(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 61 — Skystreak Engineer
pub(in crate::card::sets) static SKYSTREAK_ENGINEER: CardRecord = CardRecord::new(
    "Skystreak Engineer",
    "5bc9c501-098d-4560-9826-329b05689e0f",
    "Elizabeth Peiró",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Pilot"], 1, 3).with_abilities(&[
        abilities::flying(),
        exhaust(AbilityDef::activated(
            "Exhaust — {4}{U}: Put two +1/+1 counters on this creature. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{4}{U}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )),
    ]),
);

// DFT 62 — Slick Imitator
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static SLICK_IMITATOR: CardRecord = CardRecord::new(
    "Slick Imitator",
    "3e86ef50-4939-4e7c-853d-438f0f3e0411",
    "Xabi Gaztelua",
    CardRules::unsupported(),
);

// DFT 63 — Spectral Interference
pub(in crate::card::sets) static SPECTRAL_INTERFERENCE: CardRecord = CardRecord::new(
    "Spectral Interference",
    "860cb8af-a5f6-47e7-a34b-7b9f11ddc8c6",
    "Steve Ellis",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target artifact or creature spell unless its \
         controller pays {4}.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{4}"))]),
    )]),
);

// DFT 64 — Spell Pierce (reprint)
const SPELL_PIERCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::SPELL_PIERCE,
    "8dd4374f-0301-4b2e-bc99-2cd19568cb3b",
    "Maxime Minard",
);

// DFT 65 — Spikeshell Harrier
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static SPIKESHELL_HARRIER: CardRecord = CardRecord::new(
    "Spikeshell Harrier",
    "8f1ece22-ca32-45bb-b5f4-480f9b366cb5",
    "Alfonso Santano",
    CardRules::unsupported(),
);

// DFT 66 — Stall Out
pub(in crate::card::sets) static STALL_OUT: CardRecord = CardRecord::new(
    "Stall Out",
    "4ea0e0d3-833f-4353-b648-57b0b657cc1c",
    "Inkognit",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tap target creature or Vehicle, then put three stun counters \
             on it. (If a permanent with a stun counter would become \
             untapped, remove one from it instead.)",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(3),
                },
            ]),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 67 — Stock Up
pub(in crate::card::sets) static STOCK_UP: CardRecord = CardRecord::new(
    "Stock Up",
    "0a786855-6eb4-42c0-a528-4842db46809d",
    "Izzy",
// Two cards for three mana at sorcery speed is unremarkable; seeing five
    // to find them is what puts it in a deck built around one or two cards.
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Look at the top five cards of your library. Put two of them into your hand and the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_choose_to_hand_rest_bottom(
            ValueDef::Constant(5),
            ObjectPredicateDef::Any,
            2,
            2,
        ),
    )),
);

// DFT 68 — Thopter Fabricator
pub(in crate::card::sets) static THOPTER_FABRICATOR: CardRecord = CardRecord::new(
    "Thopter Fabricator",
    "8924b785-b140-4212-a1eb-a10340e09fea",
    "Racrufi",
    CardRules::new_vehicle(mana_cost!("{2}{U}"), 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, create a 1/1 \
             colorless Thopter artifact creature token with flying.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(THOPTER_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// DFT 69 — Trade the Helm
pub(in crate::card::sets) static TRADE_THE_HELM: CardRecord = CardRecord::new(
    "Trade the Helm",
    "eb0b5c09-6c21-4080-81c4-a8376deb729f",
    "Lius Lasahido",
    CardRules::new_sorcery(mana_cost!("{4}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exchange control of target artifact or creature you control \
             and target artifact or creature an opponent controls.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
            ],
            EffectDef::ExchangeControl {
                first: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                second: EffectRecipientDef::Target(TargetIndex(1)),
                otherwise: None,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 70 — Transit Mage
pub(in crate::card::sets) static TRANSIT_MAGE: CardRecord = CardRecord::new(
    "Transit Mage",
    "6727169f-c33a-4ca5-889d-a63bcfc5a3f0",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you may search your library for an \
             artifact card with mana value 4 or 5, reveal it, put it into \
             your hand, then shuffle.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(4)),
                            ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(5)),
                        ]),
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

// DFT 71 — Trip Up
pub(in crate::card::sets) static TRIP_UP: CardRecord = CardRecord::new(
    "Trip Up",
    "273061f3-7aa7-4cb0-afd6-616252b88948",
    "Josiah \"Jo\" Cameron",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target nonland permanent's owner puts it on their choice of \
             the top or bottom of their library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
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
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 72 — Unstoppable Plan
pub(in crate::card::sets) static UNSTOPPABLE_PLAN: CardRecord = CardRecord::new(
    "Unstoppable Plan",
    "aaeb5981-7e6a-4ffd-bb02-4757b2e92f08",
    "Borja Pindado",
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::triggered(
        "At the beginning of your end step, untap all nonland \
         permanents you control.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::Untap {
            object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
        },
    )]),
);

// DFT 73 — Vnwxt, Verbose Host
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static VNWXT_VERBOSE_HOST: CardRecord = CardRecord::new(
    "Vnwxt, Verbose Host",
    "893254c7-64cc-4cb9-b79f-2c41a8935ea0",
    "Izzy",
    CardRules::unsupported(),
);

// DFT 74 — Waxen Shapethief
pub(in crate::card::sets) static WAXEN_SHAPETHIEF: CardRecord = CardRecord::new(
    "Waxen Shapethief",
    "412aaa30-b9cd-4cf8-beb8-1c1229667b31",
    "Helge C. Balzer",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Shapeshifter"], 0, 0).with_abilities(&[
        abilities::flash(),
        AbilityDef::replacement(
            "You may have this creature enter as a copy of an artifact or \
             creature you control.",
            ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                exceptions: CopyExceptionsDef::NONE,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 75 — Ancient Vendetta
// Audit: unsupported — Needs one bounded search selecting up to four named cards across a target opponent's graveyard, hand, and library; current zone searches do not share one combined selection limit across those zones.
pub(in crate::card::sets) static ANCIENT_VENDETTA: CardRecord = CardRecord::new(
    "Ancient Vendetta",
    "230301f2-f288-4b13-9f62-e649ad8357bb",
    "Tianxing Xu",
    CardRules::unsupported(),
);

// DFT 76 — Back on Track
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static BACK_ON_TRACK: CardRecord = CardRecord::new(
    "Back on Track",
    "884c0032-9c62-4028-a55f-6a3da2545654",
    "Raoul Vitale",
    CardRules::unsupported(),
);

// DFT 77 — Bloodghast (reprint)
const BLOODGHAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::BLOODGHAST,
    "fdceefe6-f083-4955-b53a-8e6f8aeb2083",
    "Francisco Badilla",
);

// DFT 78 — Carrion Cruiser
pub(in crate::card::sets) static CARRION_CRUISER: CardRecord = CardRecord::new(
    "Carrion Cruiser",
    "dc00fdee-3d24-4360-a4d2-ffd3c08a462d",
    "Mathias Kollros",
    CardRules::new_vehicle(mana_cost!("{2}{B}"), 3, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, mill two cards. Then return a \
             creature or Vehicle card from your graveyard to your hand. \
             (To mill two cards, put the top two cards of your library \
             into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
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
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 79 — Chitin Gravestalker
pub(in crate::card::sets) static CHITIN_GRAVESTALKER: CardRecord = CardRecord::new(
    "Chitin Gravestalker",
    "903b4141-04a3-44c4-9d3e-aa2a773d9883",
    "Slawomir Maniak",
// Cycling is what makes the discount reachable: the card fills the
    // graveyard it later reads, including with copies of itself.
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Insect", "Warrior"], 5, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each artifact and/or creature card in your graveyard.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
            )),
        )
        // Read from hand, where the cost is paid.
        .with_source_zones(&[ZoneKind::Hand]),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// DFT 80 — Cryptcaller Chariot
pub(in crate::card::sets) static CRYPTCALLER_CHARIOT: CardRecord = CardRecord::new(
    "Cryptcaller Chariot",
    "a0c8259c-055e-4bff-b945-c0ecb057a8f0",
    "Aaron Miller",
    CardRules::new_vehicle(mana_cost!("{3}{B}"), 5, 5).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever you discard one or more cards, create that many \
             tapped 2/2 black Zombie creature tokens.",
            TriggerEventDef::DiscardedCards(PlayerRelation::You),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Zombie"],
                    &[ManaColor::Black],
                    2,
                    2,
                )))
                .with_count(ValueDef::TriggerEventAmount)
                .entering_tapped(),
            ),
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// DFT 81 — Cursecloth Wrappings
// Audit: unsupported — Needs granting a graveyard card an embalm activation with costs derived from its mana cost and the complete embalm copy exceptions; current static ability grants do not expose that computed-cost graveyard activation.
pub(in crate::card::sets) static CURSECLOTH_WRAPPINGS: CardRecord = CardRecord::new(
    "Cursecloth Wrappings",
    "d5803b32-4a81-46c2-9b10-3198a709611d",
    "Dominik Mayer",
    CardRules::unsupported(),
);

// DFT 82 — Deathless Pilot
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static DEATHLESS_PILOT: CardRecord = CardRecord::new(
    "Deathless Pilot",
    "e704fb95-17b7-432a-831c-18abe7d9cc73",
    "Justin Cornell",
    CardRules::unsupported(),
);

// DFT 83 — Demonic Junker
// Audit: unsupported — Needs a destruction result retaining which destroyed creatures this player controlled, including destruction whose graveyard move is replaced; WithZoneMoveResult does not wrap destruction, and DestroyFollowUp binds only resulting graveyard cards without their former controllers.
pub(in crate::card::sets) static DEMONIC_JUNKER: CardRecord = CardRecord::new(
    "Demonic Junker",
    "4aad569e-4acb-4416-9d4f-64e6991de3ed",
    "Stephan Martiniere",
    CardRules::unsupported(),
);

// DFT 84 — Engine Rat
pub(in crate::card::sets) static ENGINE_RAT: CardRecord = CardRecord::new(
    "Engine Rat",
    "949d6137-98d3-4f46-ab1e-08d7492af307",
    "Camille Alquier",
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie", "Rat"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::activated(
            "{5}{B}: Each opponent loses 2 life.",
            &[CostDef::Mana(mana_cost!("{5}{B}"))],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// DFT 85 — Gas Guzzler
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static GAS_GUZZLER: CardRecord = CardRecord::new(
    "Gas Guzzler",
    "4db3a28c-e4b4-4b18-8d56-e3842184d105",
    "Yohann Schepacz",
    CardRules::unsupported(),
);

// DFT 86 — Gastal Raider
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static GASTAL_RAIDER: CardRecord = CardRecord::new(
    "Gastal Raider",
    "6e4877b5-4ce5-466a-810f-6501f2a0f217",
    "Lorenzo Mastroianni",
    CardRules::unsupported(),
);

// DFT 87 — Gonti, Night Minister
// Audit: unsupported — Needs hidden exile-play permission assigned to the damaging creature's controller with mana of any type and lasting while the card remains exiled; current exile grants assume the resolving ability's controller or a bounded turn duration.
pub(in crate::card::sets) static GONTI_NIGHT_MINISTER: CardRecord = CardRecord::new(
    "Gonti, Night Minister",
    "d79ca40a-e5c0-4956-8df0-ecbd2a25656f",
    "Scott M. Fischer",
    CardRules::unsupported(),
);

// DFT 88 — Grim Bauble
pub(in crate::card::sets) static GRIM_BAUBLE: CardRecord = CardRecord::new(
    "Grim Bauble",
    "9bfdf60a-6f67-4872-8961-d63776b192c3",
    "Wero Gallo",
    // One mana kills an early creature and the artifact stays behind, which
    // is what makes the four-mana surveil a bonus rather than the plan.
    CardRules::new_artifact(mana_cost!("{B}")).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this artifact enters, target creature an opponent controls gets -2/-2 until \
             end of turn.",
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
        AbilityDef::activated(
            "{2}{B}, {T}, Sacrifice this artifact: Surveil 2. (Look at the top two cards of your \
             library, then put any number of them into your graveyard and the rest on top of \
             your library in any order.)",
            &[
                CostDef::Mana(mana_cost!("{2}{B}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// DFT 89 — Grim Javelineer
// Audit: unsupported — Needs a delayed death trigger tied to the chosen object that survives losing its abilities and expires this turn; installed event predicates cannot refer to an object binding, and granting a dies ability is not equivalent.
pub(in crate::card::sets) static GRIM_JAVELINEER: CardRecord = CardRecord::new(
    "Grim Javelineer",
    "87154116-e306-4e15-bd5a-dcdb5ddbcd36",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// DFT 90 — Hellish Sideswipe
pub(in crate::card::sets) static HELLISH_SIDESWIPE: CardRecord = CardRecord::new(
    "Hellish Sideswipe",
    "7a9db650-47f9-46d7-ac17-8d19fef6d6b0",
    "Diana Franco",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "As an additional cost to cast this spell, sacrifice an \
         artifact or creature.\nDestroy target creature or Vehicle. If \
         the sacrificed permanent was a Vehicle, draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(
                        AdditionalCostObjectIndex::PRIMARY,
                    )),
                    predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::Subtype(
                        SubtypeDef::Literal("Vehicle"),
                    )),
                }),
                then: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        ]),
    )
    .with_spell_additional_cost(&CostDef::Sacrifice {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::HasType(CardType::Creature),
        ]),
        quantity: CostQuantityDef::Fixed(1),
    })]),
);

// DFT 91 — Hour of Victory
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static HOUR_OF_VICTORY: CardRecord = CardRecord::new(
    "Hour of Victory",
    "9192abc8-05a3-4e72-a634-fc5acbe97b26",
    "Aaron Miller",
    CardRules::unsupported(),
);

// DFT 92 — Intimidation Tactics
pub(in crate::card::sets) static INTIMIDATION_TACTICS: CardRecord = CardRecord::new(
    "Intimidation Tactics",
    "9b4e6022-44d2-4dfe-8f7a-51581e298f23",
    "Cristi Balanescu",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent reveals their hand. You choose an artifact or \
             creature card from it. Exile that card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&abilities::reveal_hand_and_exile_chosen_card(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )),
        ),
        abilities::cycling!(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{3}"))]
        ),
    ]),
);

// DFT 93 — Kalakscion, Hunger Tyrant
pub(in crate::card::sets) static KALAKSCION_HUNGER_TYRANT: CardRecord = CardRecord::new(
    "Kalakscion, Hunger Tyrant",
    "1214fc6d-ae47-418d-88cc-58633ec2ac7a",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Crocodile"], 7, 2)
        .with_supertype(CardSupertype::Legendary),
);

// DFT 94 — The Last Ride
// Audit: unsupported — Needs a continuous power/toughness modifier reading the controller's current life total; LifeTotal is available to resolving effects but the static value evaluator cannot supply it.
pub(in crate::card::sets) static THE_LAST_RIDE: CardRecord = CardRecord::new(
    "The Last Ride",
    "9cbb7b4e-bd32-44a0-9396-16738c5e4381",
    "Michele Giorgi",
    CardRules::unsupported(),
);

// DFT 95 — Locust Spray
pub(in crate::card::sets) static LOCUST_SPRAY: CardRecord = CardRecord::new(
    "Locust Spray",
    "54b3a547-6f74-4cb4-ad98-7e1b75f1a120",
    "Caio Monteiro",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[
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
        abilities::cycling!(
            "Cycling {B} ({B}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{B}"))]
        ),
    ]),
);

// DFT 96 — Maximum Overdrive
pub(in crate::card::sets) static MAXIMUM_OVERDRIVE: CardRecord = CardRecord::new(
    "Maximum Overdrive",
    "5f6e5bb2-cfe9-48e5-86f9-e21f3d328327",
    "Javier Charro",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature. It gains deathtouch \
         and indestructible until end of turn.",
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
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// DFT 97 — Momentum Breaker
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static MOMENTUM_BREAKER: CardRecord = CardRecord::new(
    "Momentum Breaker",
    "38513b53-384f-45e7-9905-80dd2c3c4918",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// DFT 98 — Mutant Surveyor
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static MUTANT_SURVEYOR: CardRecord = CardRecord::new(
    "Mutant Surveyor",
    "7cec5105-3907-40d2-8e46-95acfaaaa0cc",
    "Nicholas Gregory",
    CardRules::unsupported(),
);

// DFT 99 — Pactdoll Terror
pub(in crate::card::sets) static PACTDOLL_TERROR: CardRecord = CardRecord::new(
    "Pactdoll Terror",
    "70226354-47b7-4f9d-a5a8-559d17f07720",
    "David Astruga",
    CardRules::new_artifact_creature(mana_cost!("{3}{B}"), &["Toy"], 3, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature or another artifact you control \
             enters, each opponent loses 1 life and you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Source,
                        ObjectPredicateDef::HasType(CardType::Artifact),
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

// DFT 100 — Quag Feast
pub(in crate::card::sets) static QUAG_FEAST: CardRecord = CardRecord::new(
    "Quag Feast",
    "dd8b6033-63e6-484e-8efb-a4eb9ca59fbf",
    "Loïc Canavaggia",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Choose target creature, planeswalker, or Vehicle. Mill two \
         cards, then destroy the chosen permanent if its mana value is \
         less than or equal to the number of cards in your graveyard.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMostValue(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                    ),
                },
                then: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ]),
    )]),
);

// DFT 101 — Ripclaw Wrangler
pub(in crate::card::sets) static RIPCLAW_WRANGLER: CardRecord = CardRecord::new(
    "Ripclaw Wrangler",
    "d4981a4a-6eca-4f84-8715-8e2672507b59",
    "John Tedrick",
    CardRules::new_vehicle(mana_cost!("{3}{B}"), 4, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
    ]),
);

// DFT 102 — Risen Necroregent
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static RISEN_NECROREGENT: CardRecord = CardRecord::new(
    "Risen Necroregent",
    "5a68482a-401d-48e7-854e-46e3db07ff35",
    "Inkognit",
    CardRules::unsupported(),
);

// DFT 103 — Risky Shortcut
pub(in crate::card::sets) static RISKY_SHORTCUT: CardRecord = CardRecord::new(
    "Risky Shortcut",
    "c80aa587-4445-43d7-abc0-654d94ff4cda",
    "Ignatius Budi",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell(
        "Draw two cards. Each player loses 2 life.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// DFT 104 — Shefet Archfiend
pub(in crate::card::sets) static SHEFET_ARCHFIEND: CardRecord = CardRecord::new(
    "Shefet Archfiend",
    "079dc8d2-0de3-415e-8af8-b8dec669368a",
    "GodMachine",
    CardRules::new_creature(mana_cost!("{5}{B}{B}"), &["Demon"], 5, 5).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, all other creatures get -2/-2 \
             until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
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
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 105 — The Speed Demon
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static THE_SPEED_DEMON: CardRecord = CardRecord::new(
    "The Speed Demon",
    "62242a80-0444-4a0e-a868-97eabcc77648",
    "Helge C. Balzer",
    CardRules::unsupported(),
);

// DFT 106 — Spin Out
pub(in crate::card::sets) static SPIN_OUT: CardRecord = CardRecord::new(
    "Spin Out",
    "be722ac5-e8c4-4180-aed0-7c28895afc0d",
    "Adrián Rodríguez Pérez",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature or Vehicle.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// DFT 107 — Streaking Oilgorger
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static STREAKING_OILGORGER: CardRecord = CardRecord::new(
    "Streaking Oilgorger",
    "6ff120a2-e2bb-42a2-bcb7-a48eb7a6d9b2",
    "Campbell White",
    CardRules::unsupported(),
);

// DFT 108 — Syphon Fuel
pub(in crate::card::sets) static SYPHON_FUEL: CardRecord = CardRecord::new(
    "Syphon Fuel",
    "4af17ae0-1035-4cb2-8974-98b377bfaa48",
    "Mathias Kollros",
    CardRules::new_instant(mana_cost!("{4}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -6/-6 until end of turn. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-6),
                    ValueDef::Constant(-6),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// DFT 109 — Wickerfolk Indomitable
// Audit: unsupported — Needs a graveyard cast permission with required additional life and sacrifice costs that also compose with other alternative costs; encoding a new complete alternative cost would incorrectly replace those other casting choices.
pub(in crate::card::sets) static WICKERFOLK_INDOMITABLE: CardRecord = CardRecord::new(
    "Wickerfolk Indomitable",
    "ba78e076-8962-4b3f-b86f-04400b062951",
    "Sergio Cosmai",
    CardRules::unsupported(),
);

// DFT 110 — Wreckage Wickerfolk
pub(in crate::card::sets) static WRECKAGE_WICKERFOLK: CardRecord = CardRecord::new(
    "Wreckage Wickerfolk",
    "aeff3db7-81ac-4c51-9954-bc1dbcb8c4e3",
    "Johan Grenier",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}"), &["Scarecrow"], 1, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
    ]),
);

// DFT 111 — Wretched Doll
pub(in crate::card::sets) static WRETCHED_DOLL: CardRecord = CardRecord::new(
    "Wretched Doll",
    "4983177d-fbf4-47fa-997f-9d08294870f2",
    "Loïc Canavaggia",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}"), &["Toy"], 3, 1).with_abilities(&[
        AbilityDef::activated(
            "{B}, {T}: Surveil 1. (Look at the top card of your library. \
             You may put that card into your graveyard.)",
            &[CostDef::Mana(mana_cost!("{B}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// DFT 112 — Adrenaline Jockey
// Audit: unsupported — Needs a committed activation event filtered by the exhaust mechanic label, including mana abilities; current trigger events do not observe labeled ability activations.
pub(in crate::card::sets) static ADRENALINE_JOCKEY: CardRecord = CardRecord::new(
    "Adrenaline Jockey",
    "c8655373-320d-440d-b700-d03413f743fd",
    "Alfonso Santano",
    CardRules::unsupported(),
);

// DFT 113 — Boommobile
pub(in crate::card::sets) static BOOMMOBILE: CardRecord = CardRecord::new(
    "Boommobile",
    "930c8289-4043-401a-8a7f-22349b7148b4",
    "Alexandr Leskinen",
    CardRules::new_vehicle(mana_cost!("{2}{R}{R}"), 5, 5).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, add four mana of any one color. \
             Spend this mana only to activate abilities.",
            EffectDef::AddMana(
                AddManaEffectDef::any_color()
                    .with_amount(4)
                    .with_restrictions(&[ManaRestrictionDef::ActivateAbility(
                        ObjectPredicateDef::Any,
                    )]),
            ),
        ),
        exhaust(AbilityDef::activated_with_targets(
            "Exhaust — {X}{2}{R}: This Vehicle deals X damage to any \
             target. Put a +1/+1 counter on this Vehicle. (Activate each \
             exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{X}{2}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::ChosenX,
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )),
        abilities::crew("Crew 2", 2),
    ]),
);

// DFT 114 — Burner Rocket
pub(in crate::card::sets) static BURNER_ROCKET: CardRecord = CardRecord::new(
    "Burner Rocket",
    "ecee6509-1103-4ae5-a2e7-0441f5d4a872",
    "José Parodi",
    CardRules::new_vehicle(mana_cost!("{1}{R}"), 3, 1).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this Vehicle enters, target creature you control gets \
             +2/+0 and gains trample until end of turn.",
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
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 115 — Burnout Bashtronaut
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static BURNOUT_BASHTRONAUT: CardRecord = CardRecord::new(
    "Burnout Bashtronaut",
    "4db66e7b-cb7a-4d86-a563-d570946aeb0d",
    "Andrea Piparo",
    CardRules::unsupported(),
);

// DFT 116 — Chandra, Spark Hunter
pub(in crate::card::sets) static CHANDRA_SPARK_HUNTER: CardRecord = CardRecord::new(
    "Chandra, Spark Hunter",
    "11f9b98e-48a1-491e-bdab-6e94e4ec747a",
    "Devin Elle Kurtz",
    CardRules::new_planeswalker(mana_cost!("{3}{R}"), &["Chandra"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "At the beginning of combat on your turn, choose up to one \
                 target Vehicle you control. Until end of turn, it becomes an \
                 artifact creature and gains haste.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_card_types(
                            CardTypeSet::single(CardType::Artifact)
                                .union(CardTypeSet::single(CardType::Creature)),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::activated(
                "+2: You may sacrifice an artifact or discard a card. If you \
                 do, draw a card.",
                &[CostDef::Loyalty(2)],
                EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Sacrifice an artifact",
                            effect: EffectDef::PayOr(PayOrDef::optional(
                                &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                                    CardType::Artifact,
                                ))],
                                &abilities::draw_cards(ValueDef::Constant(1)),
                            )),
                        },
                        EffectChoiceDef {
                            label: "Discard a card",
                            effect: EffectDef::PayOr(PayOrDef::optional(
                                &[CostDef::discard(ObjectPredicateDef::Any)],
                                &abilities::draw_cards(ValueDef::Constant(1)),
                            )),
                        },
                        EffectChoiceDef {
                            label: "Do neither",
                            effect: EffectDef::None,
                        },
                    ],
                },
            ),
            AbilityDef::activated(
                "0: Create a 3/2 colorless Vehicle artifact token with crew 1.",
                &[CostDef::Loyalty(0)],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::new(
                        CardTypeSet::single(CardType::Artifact),
                        &["Vehicle"],
                        &[],
                        Some(CreatureStats {
                            power: 3,
                            toughness: 2,
                        }),
                    )
                    .with_abilities(&[abilities::crew("Crew 1", 1)]),
                ))),
            ),
            AbilityDef::activated(
                "−7: You get an emblem with \"Whenever an artifact you control \
                 enters, this emblem deals 3 damage to any target.\"",
                &[CostDef::Loyalty(-7)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new(
                        "Chandra Emblem",
                        &[AbilityDef::triggered_with_targets(
                            "Whenever an artifact you control enters, this emblem deals 3 \
                             damage to any target.",
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ]),
                                None,
                                Some(ZoneKind::Battlefield),
                            ),
                            &[AbilityTargetDef::exactly_one(
                                AbilityTargetPredicate::AnyTarget,
                            )],
                            EffectDef::damage(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                ValueDef::Constant(3),
                            ),
                        )],
                    ),
                },
            ),
        ]),
);

// DFT 117 — Clamorous Ironclad
pub(in crate::card::sets) static CLAMOROUS_IRONCLAD: CardRecord = CardRecord::new(
    "Clamorous Ironclad",
    "4701da52-b9c8-4ce9-9d57-53e10899f19d",
    "Svetlin Velinov",
    CardRules::new_vehicle(mana_cost!("{3}{R}"), 6, 3).with_abilities(&[
        abilities::menace(),
        abilities::crew(
            "Crew 3 (Tap any number of creatures you control with total \
             power 3 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            3,
        ),
        abilities::cycling!(
            "Cycling {R} ({R}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{R}"))]
        ),
    ]),
);

// DFT 118 — Count on Luck
pub(in crate::card::sets) static COUNT_ON_LUCK: CardRecord = CardRecord::new(
    "Count on Luck",
    "8f31beae-9b8f-4c32-af84-9f3ee767ba1d",
    "Michal Ivan",
    CardRules::new_enchantment(mana_cost!("{R}{R}{R}")).with_abilities(&[AbilityDef::triggered(
        "At the beginning of your upkeep, exile the top card of your \
         library. You may play that card this turn.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::BindObjects(BindObjectsDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
            },
            binding: crate::Binding!("top"),
            then: &EffectDef::ExileGrantingControllerPlayThisTurn {
                object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("top"))),
            },
        }),
    )]),
);

// DFT 119 — Crash and Burn
pub(in crate::card::sets) static CRASH_AND_BURN: CardRecord = CardRecord::new(
    "Crash and Burn",
    "339a41a1-b36f-4b81-b74c-220d279c0e26",
    "Anthony Devine",
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target Vehicle.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Crash and Burn deals 6 damage to target creature or planeswalker.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(6),
                ),
            ),
        ],
    )]),
);

// DFT 120 — Daretti, Rocketeer Engineer
pub(in crate::card::sets) static DARETTI_ROCKETEER_ENGINEER: CardRecord = CardRecord::new(
    "Daretti, Rocketeer Engineer",
    "be626e2f-1075-4497-bd5b-bd805777afd3",
    "Borja Pindado",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Goblin", "Artificer"], 0, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Daretti's power is equal to the greatest mana value among \
                 artifacts you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::define_power(ValueDef::AggregateObjectValues(
                        &ObjectValueAggregateDef {
                            objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            select: ObjectValueDef::ManaValue,
                            operation: AggregateOperationDef::Maximum,
                        },
                    )),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever Daretti enters or attacks, choose target artifact \
                 card in your graveyard. You may sacrifice an artifact. If you \
                 do, return the chosen card to the battlefield.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Artifact),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                        CardType::Artifact,
                    ))],
                    &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                )),
            ),
        ]),
);

// DFT 121 — Draconautics Engineer
pub(in crate::card::sets) static DRACONAUTICS_ENGINEER: CardRecord = CardRecord::new(
    "Draconautics Engineer",
    "c44440be-e611-4e70-9919-b08e2354b7ad",
    "Artur Nakhodkin",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Artificer"], 2, 2).with_abilities(
        &[
            exhaust(AbilityDef::activated(
                "Exhaust — {R}: Other creatures you control gain haste until \
                 end of turn. Put a +1/+1 counter on this creature. (Activate \
                 each exhaust ability only once.)",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Sequence(&[
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
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            )),
            exhaust(AbilityDef::activated(
                "Exhaust — {3}{R}: Create a 4/4 red Dinosaur Dragon creature \
                 token with flying.",
                &[CostDef::Mana(mana_cost!("{3}{R}"))],
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(
                        &["Dinosaur", "Dragon"],
                        &[ManaColor::Red],
                        4,
                        4,
                    )
                    .with_abilities(&[abilities::flying()]),
                ))),
            )),
        ],
    ),
);

// DFT 122 — Dracosaur Auxiliary
pub(in crate::card::sets) static DRACOSAUR_AUXILIARY: CardRecord = CardRecord::new(
    "Dracosaur Auxiliary",
    "cf800b8c-d08e-4644-8ed2-11b839153861",
    "Brian Valeza",
    CardRules::new_creature(
        mana_cost!("{4}{R}{R}"),
        &["Dinosaur", "Dragon", "Mount"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks while saddled, it deals 2 \
             damage to any target.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 3 }],
            "Saddle 3 (Tap any number of other creatures you control with \
             total power 3 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// DFT 123 — Dynamite Diver
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static DYNAMITE_DIVER: CardRecord = CardRecord::new(
    "Dynamite Diver",
    "5a4f96f6-dd91-4357-ae19-1c35db2c2bcb",
    "Pete Venters",
    CardRules::unsupported(),
);

// DFT 124 — Endrider Catalyzer
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static ENDRIDER_CATALYZER: CardRecord = CardRecord::new(
    "Endrider Catalyzer",
    "55a5a67b-d969-4ba4-9dcc-32d0c2e5c04a",
    "Karl Kopinski",
    CardRules::unsupported(),
);

// DFT 125 — Endrider Spikespitter
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static ENDRIDER_SPIKESPITTER: CardRecord = CardRecord::new(
    "Endrider Spikespitter",
    "4e58cb18-f216-4248-8f0d-65b0263c5c28",
    "Mila Pesic",
    CardRules::unsupported(),
);

// DFT 126 — Fuel the Flames
pub(in crate::card::sets) static FUEL_THE_FLAMES: CardRecord = CardRecord::new(
    "Fuel the Flames",
    "624335fb-8a0b-4fa9-aefb-60ac641a7934",
    "Nicholas Gregory",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Fuel the Flames deals 2 damage to each creature.",
            EffectDef::damage(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ))),
                ValueDef::Constant(2),
            ),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 127 — Full Throttle
// Audit: unsupported — Needs scheduling two additional combats after a main phase while installing an untap trigger at every combat start this turn; the current phase schedule accepts extra phases but does not condition insertion on resolving during a main phase.
pub(in crate::card::sets) static FULL_THROTTLE: CardRecord = CardRecord::new(
    "Full Throttle",
    "d91f7cad-89e8-45cb-a78e-b35b0ee64783",
    "Benjamin Ee",
    CardRules::unsupported(),
);

// DFT 128 — Gastal Blockbuster
// Audit: unsupported — Needs a resolution-created reflexive trigger that chooses its target after the sacrifice and survives the source leaving; OptionalEffectTaken watches battlefield listeners and does not represent this payment-specific continuation.
pub(in crate::card::sets) static GASTAL_BLOCKBUSTER: CardRecord = CardRecord::new(
    "Gastal Blockbuster",
    "dca41ec6-8f8f-42ef-abac-cc645c6440b7",
    "Bryan Sola",
    CardRules::unsupported(),
);

// DFT 129 — Gastal Thrillroller
// Audit: unsupported — Needs an ordinary activation from the graveyard that pays a chosen hand-card discard in addition to mana; current nonbattlefield activation enumeration does not support that discard cost.
pub(in crate::card::sets) static GASTAL_THRILLROLLER: CardRecord = CardRecord::new(
    "Gastal Thrillroller",
    "d8b1762b-ff03-4312-afcc-cb6b5f280ade",
    "Caio Monteiro",
    CardRules::unsupported(),
);

// DFT 130 — Gilded Ghoda
pub(in crate::card::sets) static GILDED_GHODA: CardRecord = CardRecord::new(
    "Gilded Ghoda",
    "55685602-a18c-43a4-ac60-13b039672fa4",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Horse", "Mount"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, create a \
             Treasure token. (It's an artifact with \"{T}, Sacrifice this \
             token: Add one mana of any color.\")",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 1 }],
            "Saddle 1 (Tap any number of other creatures you control with \
             total power 1 or more: This Mount becomes saddled until end \
             of turn. Saddle only as a sorcery.)",
        ),
    ]),
);

// DFT 131 — Goblin Surveyor
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static GOBLIN_SURVEYOR: CardRecord = CardRecord::new(
    "Goblin Surveyor",
    "e1efffe9-00f8-4177-a9e6-4ad62887d32f",
    "Pete Venters",
    CardRules::unsupported(),
);

// DFT 132 — Greasewrench Goblin
pub(in crate::card::sets) static GREASEWRENCH_GOBLIN: CardRecord = CardRecord::new(
    "Greasewrench Goblin",
    "c8f0b123-fdb0-4f3e-ba78-fa155c227e20",
    "Alexandre Honoré",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Artificer"], 2, 1).with_abilities(&[
        exhaust(AbilityDef::activated(
            "Exhaust — {2}{R}: Discard up to two cards, then draw that \
             many cards. Put a +1/+1 counter on this creature. (Activate \
             each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{2}{R}"))],
            EffectDef::Sequence(&[
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
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )),
    ]),
);

// DFT 133 — Hazoret, Godseeker
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static HAZORET_GODSEEKER: CardRecord = CardRecord::new(
    "Hazoret, Godseeker",
    "e2f66043-0872-4334-a91f-0e9bbbdddf66",
    "Chris Rallis",
    CardRules::unsupported(),
);

// DFT 134 — Howlsquad Heavy
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static HOWLSQUAD_HEAVY: CardRecord = CardRecord::new(
    "Howlsquad Heavy",
    "df582f80-7b9a-4f71-95a9-70548ec7d2d7",
    "Leonardo Santanna",
    CardRules::unsupported(),
);

// DFT 135 — Kickoff Celebrations
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static KICKOFF_CELEBRATIONS: CardRecord = CardRecord::new(
    "Kickoff Celebrations",
    "4e5590e1-0ac0-4bdd-815b-136bf24ced03",
    "Evyn Fong",
    CardRules::unsupported(),
);

// DFT 136 — Lightning Strike (reprint)
const LIGHTNING_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::LIGHTNING_STRIKE,
    "30077b49-b825-4dbb-a0c7-f3992f647df0",
    "Steve Ellis",
);

// DFT 137 — Magmakin Artillerist
pub(in crate::card::sets) static MAGMAKIN_ARTILLERIST: CardRecord = CardRecord::new(
    "Magmakin Artillerist",
    "2ac26341-a100-424d-be84-33fbbf1b4078",
    "Madeline Boni",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Elemental", "Pirate"], 1, 4).with_abilities(
        &[
            AbilityDef::triggered(
                "Whenever you discard one or more cards, this creature deals \
                 that much damage to each opponent.",
                TriggerEventDef::DiscardedCards(PlayerRelation::You),
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::TriggerEventAmount),
            ),
            abilities::cycling!(
                "Cycling {1}{R} ({1}{R}, Discard this card: Draw a card.)",
                &[CostDef::Mana(mana_cost!("{1}{R}"))]
            ),
            AbilityDef::triggered(
                "When you cycle this card, it deals 1 damage to each opponent.",
                TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
            ),
        ],
    ),
);

// DFT 138 — Marauding Mako
pub(in crate::card::sets) static MARAUDING_MAKO: CardRecord = CardRecord::new(
    "Marauding Mako",
    "9efbfd67-e0f5-43e0-9fff-1eb4a2bed0d8",
    "Alix Branwyn",
    CardRules::new_creature(mana_cost!("{R}"), &["Shark", "Pirate"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you discard one or more cards, put that many +1/+1 \
             counters on this creature.",
            TriggerEventDef::DiscardedCards(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 139 — Outpace Oblivion
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static OUTPACE_OBLIVION: CardRecord = CardRecord::new(
    "Outpace Oblivion",
    "c22e415f-636f-4394-9b21-600ab720ac98",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// DFT 140 — Pacesetter Paragon
pub(in crate::card::sets) static PACESETTER_PARAGON: CardRecord = CardRecord::new(
    "Pacesetter Paragon",
    "7364b4dc-8cce-498d-a62e-eabc612b062a",
    "Wonchun Choi",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Pilot"], 2, 3).with_abilities(&[
        exhaust(AbilityDef::activated(
            "Exhaust — {2}{R}: Put a +1/+1 counter on this creature. It \
             gains double strike until end of turn. (Activate each exhaust \
             ability only once.)",
            &[CostDef::Mana(mana_cost!("{2}{R}"))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        )),
    ]),
);

// DFT 141 — Pedal to the Metal
pub(in crate::card::sets) static PEDAL_TO_THE_METAL: CardRecord = CardRecord::new(
    "Pedal to the Metal",
    "75d07295-78b7-4f90-82c7-e7a639db9993",
    "Anthony Devine",
    CardRules::new_instant(mana_cost!("{X}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +X/+0 and gains first strike until end \
         of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(ValueDef::ChosenX, ValueDef::Constant(0)),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DFT 142 — Prowcatcher Specialist
pub(in crate::card::sets) static PROWCATCHER_SPECIALIST: CardRecord = CardRecord::new(
    "Prowcatcher Specialist",
    "affbc8db-4ff7-4a86-954a-910493e5a2ef",
    "Konstantin Porubov",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 1).with_abilities(&[
        abilities::haste(),
        exhaust(AbilityDef::activated(
            "Exhaust — {3}{R}: Put two +1/+1 counters on this creature. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{3}{R}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        )),
    ]),
);

// DFT 143 — Push the Limit
pub(in crate::card::sets) static PUSH_THE_LIMIT: CardRecord = CardRecord::new(
    "Push the Limit",
    "21de84a2-2654-4e1a-a569-bf385bb43685",
    "Alexander Mokhov",
    CardRules::new_sorcery(mana_cost!("{5}{R}{R}")).with_abilities(&[AbilityDef::spell(
        "Return all Mount and Vehicle cards from your graveyard to the \
         battlefield. Sacrifice them at the beginning of the next end \
         step.\nVehicles you control become artifact creatures until \
         end of turn. Creatures you control gain haste until end of \
         turn.",
        EffectDef::Sequence(&[
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ))),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                binding: crate::Binding!("returned"),
                then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                    &AbilityDef::triggered(
                        "At the beginning of the next end step, sacrifice this permanent.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::sacrifice(EffectRecipientDef::objects(
                            ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                "returned"
                            )),
                        )),
                    ),
                )),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_card_types(
                    CardTypeSet::single(CardType::Artifact)
                        .union(CardTypeSet::single(CardType::Creature)),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// DFT 144 — Reckless Velocitaur
// Audit: unsupported — Needs a committed crew/saddle contribution event naming the paying creature and the Mount or Vehicle during the main phase; the current total-power tap payment only emits ordinary tapped events and does not retain that relationship.
pub(in crate::card::sets) static RECKLESS_VELOCITAUR: CardRecord = CardRecord::new(
    "Reckless Velocitaur",
    "8edd18be-3861-4510-ba6d-38ccba60bb5b",
    "Inkognit",
    CardRules::unsupported(),
);

// DFT 145 — Road Rage
pub(in crate::card::sets) static ROAD_RAGE: CardRecord = CardRecord::new(
    "Road Rage",
    "fca022de-0c7b-48ea-b3b5-af28987a5070",
    "Javier Charro",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Road Rage deals X damage to target creature or planeswalker, \
         where X is 2 plus the number of Mounts and Vehicles you \
         control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Sum(&SumValueDef::new(
                ValueDef::Constant(2),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        ),
    )]),
);

// DFT 146 — Skycrash
pub(in crate::card::sets) static SKYCRASH: CardRecord = CardRecord::new(
    "Skycrash",
    "87e82be6-d2b4-4c95-8466-477e8c6832e9",
    "Nicholas Gregory",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
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
        abilities::cycling!(
            "Cycling {R} ({R}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{R}"))]
        ),
    ]),
);

// DFT 147 — Spire Mechcycle
pub(in crate::card::sets) static SPIRE_MECHCYCLE: CardRecord = CardRecord::new(
    "Spire Mechcycle",
    "f483debe-9c54-4323-aec5-226587ece2c9",
    "Adam Volker",
    CardRules::new_vehicle(mana_cost!("{4}{R}"), 5, 4).with_abilities(&[
        abilities::haste(),
        exhaust(AbilityDef::activated(
            "Exhaust — Tap another untapped Mount or Vehicle you control: \
             This Vehicle becomes an artifact creature. Put a +1/+1 \
             counter on it for each Mount and/or Vehicle you control other \
             than this Vehicle. (Activate each exhaust ability only once.)",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
                count: 1,
            }],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ]),
        )),
        abilities::crew("Crew 2", 2),
    ]),
);

// DFT 148 — Thunderhead Gunner
pub(in crate::card::sets) static THUNDERHEAD_GUNNER: CardRecord = CardRecord::new(
    "Thunderhead Gunner",
    "ef537868-b2cb-4bbd-a935-b7f73f19bd06",
    "Mirko Failoni",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Shark", "Pirate"], 4, 5).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated(
            "Discard a card: Draw a card. Activate only as a sorcery and \
             only once each turn.",
            &[CostDef::discard(ObjectPredicateDef::Any)],
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .activations_each_turn(1),
    ]),
);

// DFT 149 — Tyrox, Saurid Tyrant
pub(in crate::card::sets) static TYROX_SAURID_TYRANT: CardRecord = CardRecord::new(
    "Tyrox, Saurid Tyrant",
    "159dc5a8-5cba-4c20-8c07-28a3d86c8411",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dinosaur", "Warrior"], 4, 1)
        .with_supertype(CardSupertype::Legendary),
);

// DFT 150 — Afterburner Expert
// Audit: unsupported — Needs a committed activation event filtered by the exhaust mechanic label, including mana abilities; current trigger events do not observe labeled ability activations.
pub(in crate::card::sets) static AFTERBURNER_EXPERT: CardRecord = CardRecord::new(
    "Afterburner Expert",
    "555e1bfc-6d07-4979-a914-b2bd1fb031f2",
    "April Prime",
    CardRules::unsupported(),
);

// DFT 151 — Agonasaur Rex
pub(in crate::card::sets) static AGONASAUR_REX: CardRecord = CardRecord::new(
    "Agonasaur Rex",
    "5b11ec26-9e07-45e1-bcaa-5d44d1231586",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Dinosaur"], 8, 8).with_abilities(&[
        abilities::trample(),
        abilities::cycling!(
            "Cycling {2}{G} ({2}{G}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}{G}"))]
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, put two +1/+1 counters on up to one \
             target creature or Vehicle. It gains trample and \
             indestructible until end of turn.",
            TriggerEventDef::DiscardedToActivate(abilities::CYCLING),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
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
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// DFT 152 — Alacrian Jaguar
pub(in crate::card::sets) static ALACRIAN_JAGUAR: CardRecord = CardRecord::new(
    "Alacrian Jaguar",
    "2bd40bca-aa54-4e52-8d48-b3709a11e633",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Cat", "Mount"], 4, 4).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, it gets +2/+2 \
             until end of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
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

// DFT 153 — Autarch Mammoth
pub(in crate::card::sets) static AUTARCH_MAMMOTH: CardRecord = CardRecord::new(
    "Autarch Mammoth",
    "4d313ea7-2456-48f3-8b12-97d8e8c2a5b3",
    "Leonardo Santanna",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elephant", "Mount"], 5, 5).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters and whenever it attacks while \
                 saddled, create a 3/3 green Elephant creature token.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELEPHANT_TOKEN))),
            ),
            AbilityDef::triggered(
                "When this creature enters and whenever it attacks while \
                 saddled, create a 3/3 green Elephant creature token.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Saddled,
                    },
                },
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELEPHANT_TOKEN))),
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 5 }],
                "Saddle 5 (Tap any number of other creatures you control with \
                 total power 5 or more: This Mount becomes saddled until end \
                 of turn. Saddle only as a sorcery.)",
            ),
        ],
    ),
);

// DFT 154 — Beastrider Vanguard
pub(in crate::card::sets) static BEASTRIDER_VANGUARD: CardRecord = CardRecord::new(
    "Beastrider Vanguard",
    "f4b99a61-49f1-46a2-9eb7-b6c88c236215",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{4}{G}: Look at the top three cards of your library. You may \
             reveal a permanent card from among them and put it into your \
             hand. Put the rest on the bottom of your library in any \
             order.",
            &[CostDef::Mana(mana_cost!("{4}{G}"))],
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_bottom(
                ValueDef::Constant(3),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                0,
                1,
            ),
        ),
    ]),
);

// DFT 155 — Bestow Greatness
pub(in crate::card::sets) static BESTOW_GREATNESS: CardRecord = CardRecord::new(
    "Bestow Greatness",
    "b6a5adee-3482-4c5b-9260-58efa8fec5ba",
    "Jeff Carpenter",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +4/+4 and gains trample until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(4),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DFT 156 — Broken Wings (reprint)
const BROKEN_WINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::BROKEN_WINGS,
    "1d7d5b71-7c1b-4fc2-a5ec-7285e17ffe0f",
    "Nils Hamm",
);

// DFT 157 — Defend the Rider
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static DEFEND_THE_RIDER: CardRecord = CardRecord::new(
    "Defend the Rider",
    "59ed23a2-6153-47b2-ab73-062195cafb74",
    "Raph Lomotan",
    CardRules::unsupported(),
);

// DFT 158 — District Mascot
pub(in crate::card::sets) static DISTRICT_MASCOT: CardRecord = CardRecord::new(
    "District Mascot",
    "3171b16f-cd67-416d-be5e-5d9cccb8d0a0",
    "Liiga Smilshkalne",
    CardRules::new_creature(mana_cost!("{G}"), &["Dog", "Mount"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{1}{G}, Remove two +1/+1 counters from this creature: Destroy \
             target artifact.",
            &[
                CostDef::Mana(mana_cost!("{1}{G}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, put a +1/+1 \
             counter on it.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::saddle(
            &[CostDef::TapCreaturesWithTotalPower { minimum: 1 }],
            "Saddle 1",
        ),
    ]),
);

// DFT 159 — Dredger's Insight
// Audit: unsupported — Needs one trigger for a simultaneous group of artifact or creature cards leaving this player's graveyard; current zone-change matchers emit a trigger for each card and do not coalesce the matching group.
pub(in crate::card::sets) static DREDGER_S_INSIGHT: CardRecord = CardRecord::new(
    "Dredger's Insight",
    "148400a0-7819-4551-9815-9357eed1db4d",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// DFT 160 — Earthrumbler
pub(in crate::card::sets) static EARTHRUMBLER: CardRecord = CardRecord::new(
    "Earthrumbler",
    "ee386cd0-934a-4b33-9db3-0a9033ab577e",
    "J.P. Targete",
    CardRules::new_vehicle(mana_cost!("{4}{G}"), 7, 6).with_abilities(&[
        abilities::vigilance(),
        abilities::trample(),
        AbilityDef::activated(
            "Exile an artifact or creature card from your graveyard: This \
             Vehicle becomes an artifact creature until end of turn.",
            &[CostDef::MoveToZone(MoveToZoneCostDef::new(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                ZoneKind::Graveyard,
                ZoneKind::Exile,
                1,
            ))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_card_types(
                    CardTypeSet::single(CardType::Artifact)
                        .union(CardTypeSet::single(CardType::Creature)),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::crew(
            "Crew 3 (Tap any number of creatures you control with total \
             power 3 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            3,
        ),
    ]),
);

// DFT 161 — Elvish Refueler
// Audit: unsupported — Needs a player-wide exception to once-per-object activation history for exhaust plus an exhaust-activation count for the current turn; neither is represented by existing activation permissions.
pub(in crate::card::sets) static ELVISH_REFUELER: CardRecord = CardRecord::new(
    "Elvish Refueler",
    "25dfbcb6-9b67-4151-b10f-dde70c5fd16d",
    "Carly Milligan",
    CardRules::unsupported(),
);

// DFT 162 — Fang Guardian
pub(in crate::card::sets) static FANG_GUARDIAN: CardRecord = CardRecord::new(
    "Fang Guardian",
    "6db483a7-c9f0-449d-be97-77dbd6d3a27a",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Ape", "Druid"], 4, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, another target creature or Vehicle \
             you control gets +2/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
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
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DFT 163 — Fang-Druid Summoner
// Audit: unsupported — Needs one optional quality-constrained search across library and/or graveyard with a single-card combined limit and shuffling only when the library is searched; ordinary Choose over a library would bypass search restrictions, while separate searches permit two cards.
pub(in crate::card::sets) static FANG_DRUID_SUMMONER: CardRecord = CardRecord::new(
    "Fang-Druid Summoner",
    "496442f6-48c7-464e-bcf3-4c14f49fa065",
    "Nino Is",
    CardRules::unsupported(),
);

// DFT 164 — Greenbelt Guardian
pub(in crate::card::sets) static GREENBELT_GUARDIAN: CardRecord = CardRecord::new(
    "Greenbelt Guardian",
    "2f6b0000-5fd3-483b-bac5-f9b888d8755b",
    "Tianxing Xu",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Ranger"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{G}: Target creature gains trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        exhaust(AbilityDef::activated(
            "Exhaust — {3}{G}: Put three +1/+1 counters on this creature. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(3),
            },
        )),
    ]),
);

// DFT 165 — Hazard of the Dunes
pub(in crate::card::sets) static HAZARD_OF_THE_DUNES: CardRecord = CardRecord::new(
    "Hazard of the Dunes",
    "c3fbee7a-7e73-43cf-bc39-ccb1c63bf837",
    "Brent Hollowell",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Wurm"], 4, 4).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        exhaust(AbilityDef::activated(
            "Exhaust — {6}{G}: Put three +1/+1 counters on this creature. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{6}{G}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(3),
            },
        )),
    ]),
);

// DFT 166 — Jibbirik Omnivore
pub(in crate::card::sets) static JIBBIRIK_OMNIVORE: CardRecord = CardRecord::new(
    "Jibbirik Omnivore",
    "68a0569b-65c8-49ce-91ac-e639b8faf939",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Beast"], 3, 2),
);

// DFT 167 — Loxodon Surveyor
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static LOXODON_SURVEYOR: CardRecord = CardRecord::new(
    "Loxodon Surveyor",
    "cd1cecb1-6776-4495-be56-b7dde65453f1",
    "J.P. Targete",
    CardRules::unsupported(),
);

// DFT 168 — Lumbering Worldwagon
pub(in crate::card::sets) static LUMBERING_WORLDWAGON: CardRecord = CardRecord::new(
    "Lumbering Worldwagon",
    "9f989c59-7b2a-4036-9ea2-cd0c7e85c15b",
    "Raph Lomotan",
    CardRules::new_vehicle(mana_cost!("{2}{G}"), 0, 4).with_abilities(&[
        AbilityDef::static_ability(
            "This Vehicle's power is equal to the number of lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            },
        ),
        AbilityDef::triggered(
            "Whenever this Vehicle enters or attacks, you may search your \
             library for a basic land card, put it onto the battlefield \
             tapped, then shuffle.",
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
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
        abilities::crew("Crew 4", 4),
    ]),
);

// DFT 169 — March of the World Ooze
pub(in crate::card::sets) static MARCH_OF_THE_WORLD_OOZE: CardRecord = CardRecord::new(
    "March of the World Ooze",
    "b1964ec5-0dd1-4b54-917c-cbeab05aba79",
    "Helge C. Balzer",
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Creatures you control have base power and toughness 6/6 and \
             are Oozes in addition to their other types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(6),
                        ValueDef::Constant(6),
                    ),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Ooze"])),
                ]),
            },
        ),
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell, if it's not their turn, \
             you create a 3/3 green Elephant creature token.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Any,
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ])),
            &TriggerConditionDef::Not(&TriggerConditionDef::ActivePlayer(
                PlayerRelation::EventPlayer,
            )),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELEPHANT_TOKEN))),
        ),
    ]),
);

// DFT 170 — Migrating Ketradon
pub(in crate::card::sets) static MIGRATING_KETRADON: CardRecord = CardRecord::new(
    "Migrating Ketradon",
    "a9ba2219-8184-4141-8708-845cb0957299",
    "Izzy",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Dinosaur"], 6, 6).with_abilities(&[
        abilities::reach(),
        abilities::enters_trigger(
            "When this creature enters, you gain 4 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 171 — Molt Tender
// Audit: unsupported — Needs a mana activation cost choosing and exiling a card from the controller's graveyard; the mana planner does not enumerate graveyard MoveToZone costs, although ordinary stack activations do.
pub(in crate::card::sets) static MOLT_TENDER: CardRecord = CardRecord::new(
    "Molt Tender",
    "f800bf4e-4bfb-45b6-950b-c76952f52bb1",
    "Filip Burburan",
    CardRules::unsupported(),
);

// DFT 172 — Ooze Patrol
pub(in crate::card::sets) static OOZE_PATROL: CardRecord = CardRecord::new(
    "Ooze Patrol",
    "101d22c6-830d-4908-9003-6b206f694eba",
    "Forrest Schehl",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Ooze"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill two cards, then put a +1/+1 \
             counter on this creature for each artifact and/or creature \
             card in your graveyard. (To mill two cards, put the top two \
             cards of your library into your graveyard.)",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                },
            ]),
        ),
    ]),
);

// DFT 173 — Oviya, Automech Artisan
pub(in crate::card::sets) static OVIYA_AUTOMECH_ARTISAN: CardRecord = CardRecord::new(
    "Oviya, Automech Artisan",
    "ee5f504c-33fc-4a91-b69b-8ef555987c79",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Human", "Artificer"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Each creature that's attacking one of your opponents has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Attacking,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            ),
            AbilityDef::activated(
                "{G}, {T}: You may put a creature or Vehicle card from your \
                 hand onto the battlefield. If you put an artifact onto the \
                 battlefield this way, put two +1/+1 counters on it.",
                &[CostDef::Mana(mana_cost!("{G}")), CostDef::TapSource],
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
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
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("arrived"),
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Matching {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("arrived"),
                                ),
                                object: ObjectSetFilterDef::Predicate(
                                    &ObjectPredicateDef::HasType(CardType::Artifact),
                                ),
                            }),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(2),
                        },
                    },
                }),
            ),
        ]),
);

// DFT 174 — Plow Through
pub(in crate::card::sets) static PLOW_THROUGH: CardRecord = CardRecord::new(
    "Plow Through",
    "a311d4b3-ab2a-43c2-8480-6c5daac41178",
    "Brian Valeza",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target creature you control fights target creature an \
                 opponent controls. (Each deals damage equal to its power to \
                 the other.)",
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
            AbilityDef::spell_with_targets(
                "Destroy target Vehicle.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )]),
);

// DFT 175 — Point the Way
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static POINT_THE_WAY: CardRecord = CardRecord::new(
    "Point the Way",
    "8fd4c73d-0e9a-4ffe-8062-f2f4d0e601fe",
    "Izzy",
    CardRules::unsupported(),
);

// DFT 176 — Pothole Mole
pub(in crate::card::sets) static POTHOLE_MOLE: CardRecord = CardRecord::new(
    "Pothole Mole",
    "21c7b59f-fdae-4a11-9784-497a8165d75a",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Mole"], 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, mill three cards, then you may \
             return a land card from your graveyard to your hand. (To mill \
             three cards, put the top three cards of your library into \
             your graveyard.)",
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
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
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

// DFT 177 — Regal Imperiosaur
pub(in crate::card::sets) static REGAL_IMPERIOSAUR: CardRecord = CardRecord::new(
    "Regal Imperiosaur",
    "36f569cc-ed09-4c27-b753-18b22ad7f425",
    "Stephanie Cheung",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Dinosaur"], 5, 4).with_abilities(&[
        AbilityDef::static_ability(
            "Other Dinosaurs you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
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

// DFT 178 — Rise from the Wreck
pub(in crate::card::sets) static RISE_FROM_THE_WRECK: CardRecord = CardRecord::new(
    "Rise from the Wreck",
    "43e6ac32-a7a4-4c15-81b0-8485d6a0e7ca",
    "Nino Is",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return up to one target creature card, up to one target Mount \
         card, up to one target Vehicle card, and up to one target \
         creature card with no abilities from your graveyard to your \
         hand.",
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
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            ),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            ),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasAbility(
                            AbilityPredicateDef::Any,
                        )),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
                1,
            ),
        ],
        EffectDef::move_to_zone(
            EffectRecipientDef::objects(ObjectSetDef::Union(&[
                ObjectSetDef::LegalTargets(TargetIndex(0)),
                ObjectSetDef::LegalTargets(TargetIndex(1)),
                ObjectSetDef::LegalTargets(TargetIndex(2)),
                ObjectSetDef::LegalTargets(TargetIndex(3)),
            ])),
            ZoneKind::Hand,
            ZonePlacement::Top,
        ),
    )]),
);

// DFT 179 — Run Over
// Audit: unsupported — Needs a self spell-cost condition inspecting whether a declared target is a controlled Mount or Vehicle; existing spell-cost predicates only test targeting the external cost source.
pub(in crate::card::sets) static RUN_OVER: CardRecord = CardRecord::new(
    "Run Over",
    "642f8fdd-6c58-49a3-904c-27f717cae980",
    "Tuan Duong Chu",
    CardRules::unsupported(),
);

// DFT 180 — Silken Strength
pub(in crate::card::sets) static SILKEN_STRENGTH: CardRecord = CardRecord::new(
    "Silken Strength",
    "ce0e1ded-9b00-4d7b-884c-70a429783b1f",
    "Olivier Bernard",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::aura_spell(
                "Enchant creature or Vehicle",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                )],
            ),
            AbilityDef::static_ability(
                "Enchanted permanent gets +1/+2 and has reach.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::reach()),
                    ]),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, untap enchanted permanent.",
                EffectDef::Untap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// DFT 181 — Stampeding Scurryfoot
pub(in crate::card::sets) static STAMPEDING_SCURRYFOOT: CardRecord = CardRecord::new(
    "Stampeding Scurryfoot",
    "7fc713d7-4a7e-4f1f-b461-e89bb1457d7d",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{G}"), &["Mouse"], 1, 1).with_abilities(&[exhaust(
        AbilityDef::activated(
            "Exhaust — {3}{G}: Put a +1/+1 counter on this creature. \
             Create a 3/3 green Elephant creature token. (Activate each \
             exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELEPHANT_TOKEN))),
            ]),
        ),
    )]),
);

// DFT 182 — Terrian, World Tyrant
pub(in crate::card::sets) static TERRIAN_WORLD_TYRANT: CardRecord = CardRecord::new(
    "Terrian, World Tyrant",
    "b44255cf-5264-4ead-9de0-20cc0f7cac6f",
    "Nicholas Gregory",
    CardRules::new_creature(mana_cost!("{2}{G}{G}{G}"), &["Dinosaur", "Ooze"], 9, 7)
        .with_supertype(CardSupertype::Legendary),
);

// DFT 183 — Thunderous Velocipede
pub(in crate::card::sets) static THUNDEROUS_VELOCIPEDE: CardRecord = CardRecord::new(
    "Thunderous Velocipede",
    "98a79557-8ed6-4d9a-b4e1-cece05664984",
    "Adrián Rodríguez Pérez",
    CardRules::new_vehicle(mana_cost!("{1}{G}{G}"), 5, 5).with_abilities(&[
        abilities::trample(),
        AbilityDef::replacement_for(
            "Each other Vehicle and creature you control enters with an \
             additional +1/+1 counter on it if its mana value is 4 or \
             less. Otherwise, it enters with three additional +1/+1 \
             counters on it.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::ManaValueAtMost(4),
                ]),
                controller: PlayerRelation::You,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::replacement_for(
            "Each other Vehicle and creature you control enters with an \
             additional +1/+1 counter on it if its mana value is 4 or \
             less. Otherwise, it enters with three additional +1/+1 \
             counters on it.",
            ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(4)),
                ]),
                controller: PlayerRelation::You,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        abilities::crew("Crew 3", 3),
    ]),
);

// DFT 184 — Veloheart Bike
pub(in crate::card::sets) static VELOHEART_BIKE: CardRecord = CardRecord::new(
    "Veloheart Bike",
    "85edde1a-f05c-4ce8-bedd-88359647aa0d",
    "Anthony Devine",
    CardRules::new_vehicle(mana_cost!("{2}{G}"), 4, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add one mana of any color.",
            AddManaEffectDef::any_color(),
        ),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
    ]),
);

// DFT 185 — Venomsac Lagac
pub(in crate::card::sets) static VENOMSAC_LAGAC: CardRecord = CardRecord::new(
    "Venomsac Lagac",
    "9142b1c9-09d7-42e4-8138-6c15d31f2470",
    "Andrey Kuzinskiy",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Lizard", "Mount"], 2, 1).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature attacks while saddled, it gets +0/+3 \
             until end of turn.",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Saddled,
                },
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
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

// DFT 186 — Webstrike Elite
// Audit: unsupported — Needs the cycling activation's chosen X captured in its discard-trigger context and target validation; current DiscardedToActivate capture resets X to zero.
pub(in crate::card::sets) static WEBSTRIKE_ELITE: CardRecord = CardRecord::new(
    "Webstrike Elite",
    "064cc22d-d424-4bc9-b8f0-88b170fd6c28",
    "Andrew Mar",
    CardRules::unsupported(),
);

// DFT 187 — Aatchik, Emerald Radian
pub(in crate::card::sets) static AATCHIK_EMERALD_RADIAN: CardRecord = CardRecord::new(
    "Aatchik, Emerald Radian",
    "fbdaa29b-85ff-4a06-b27e-fcdbdfd4a3fe",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{3}{B}{B}{G}"), &["Insect", "Druid"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Aatchik enters, create a 1/1 green Insect creature token \
                 for each artifact and/or creature card in your graveyard.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                        &["Insect"],
                        &[ManaColor::Green],
                        1,
                        1,
                    )))
                    .with_count(ValueDef::CountMatchingObjects(
                        &ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                    )),
                ),
            ),
            AbilityDef::triggered(
                "Whenever another Insect you control dies, put a +1/+1 counter \
                 on Aatchik. Each opponent loses 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Insect")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// DFT 188 — Apocalypse Runner
pub(in crate::card::sets) static APOCALYPSE_RUNNER: CardRecord = CardRecord::new(
    "Apocalypse Runner",
    "971286f5-77ea-4824-91de-f0ac88c46238",
    "Aaron J. Riley",
    CardRules::new_vehicle(mana_cost!("{2}{B}{R}"), 6, 5).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control with power 2 or less gains \
             lifelink until end of turn and can't be blocked this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
        abilities::crew(
            "Crew 3 (Tap any number of creatures you control with total \
             power 3 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            3,
        ),
    ]),
);

// DFT 189 — Boom Scholar
// Audit: unsupported — Needs a generic activation-cost reduction filtered by the exhaust mechanic label; existing spell cost reductions do not alter costs of activated or mana abilities.
pub(in crate::card::sets) static BOOM_SCHOLAR: CardRecord = CardRecord::new(
    "Boom Scholar",
    "f2b84684-10c9-4635-a922-620f04809bb1",
    "Brian Valeza",
    CardRules::unsupported(),
);

// DFT 190 — Boosted Sloop
pub(in crate::card::sets) static BOOSTED_SLOOP: CardRecord = CardRecord::new(
    "Boosted Sloop",
    "3563cb48-9dcb-4b92-af3a-4793adf03125",
    "José Parodi",
    CardRules::new_vehicle(mana_cost!("{1}{U}{R}"), 3, 3).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered(
            "Whenever you attack, draw a card, then discard a card.",
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
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 191 — Brightglass Gearhulk
pub(in crate::card::sets) static BRIGHTGLASS_GEARHULK: CardRecord = CardRecord::new(
    "Brightglass Gearhulk",
    "3dea5b45-925c-4732-8e9d-fa8232792736",
    "José Parodi",
// A 4/4 first striker with trample that also finds the two one-drops the
    // deck is built around, which is what four coloured pips buy.
    CardRules::new_artifact_creature(mana_cost!("{G}{G}{W}{W}"), &["Construct"], 4, 4)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::trample(),
            // "You may" on top of a search that already allows none: declining and
            // finding nothing look the same from the outside, and the card offers
            // both because a library nobody wants to shuffle is a real answer.
            abilities::enters_trigger(
                "When this creature enters, you may search your library for up to two artifact, creature, \
                 and/or enchantment cards with mana value 1 or less, reveal them, put them into your \
                 hand, then shuffle.",
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // "Up to two" and revealed: a minimum of none, and everything taken is
                    // shown, which is what stops the search being private information.
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        // "Artifact, creature, and/or enchantment cards with mana value 1 or less."
                        // The three types are alternatives and the mana value applies to all of
                        // them, so the bound is outside the choice rather than inside it.
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(1),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(2),
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

// DFT 192 — Broadside Barrage
pub(in crate::card::sets) static BROADSIDE_BARRAGE: CardRecord = CardRecord::new(
    "Broadside Barrage",
    "d086e4e5-98fa-437e-85aa-d8849c94ce94",
    "Javier Charro",
    CardRules::new_instant(mana_cost!("{1}{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Broadside Barrage deals 5 damage to target creature or \
             planeswalker. Draw a card, then discard a card.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(5),
                ),
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

// DFT 193 — Broodheart Engine
pub(in crate::card::sets) static BROODHEART_ENGINE: CardRecord = CardRecord::new(
    "Broodheart Engine",
    "ed09139b-a66e-4cd1-b45a-23a4648d3401",
    "Rémi Jacquot",
    CardRules::new_artifact(mana_cost!("{B}{G}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, surveil 1.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            abilities::surveil(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_with_targets(
            "{2}{B}{G}, {T}, Sacrifice this artifact: Return target \
             creature or Vehicle card from your graveyard to the \
             battlefield. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}{B}{G}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
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

// DFT 194 — Captain Howler, Sea Scourge
// Audit: unsupported — Needs a turn-long delayed damage trigger bound to the chosen creature and surviving ability removal; installed triggers cannot match a bound object identity, while granting an ability changes that behavior.
pub(in crate::card::sets) static CAPTAIN_HOWLER_SEA_SCOURGE: CardRecord = CardRecord::new(
    "Captain Howler, Sea Scourge",
    "0957c90f-e10d-40f8-a4be-9e9ef623dd43",
    "Mirko Failoni",
    CardRules::unsupported(),
);

// DFT 195 — Caradora, Heart of Alacria
// Audit: unsupported — Needs a prospective +1/+1 counter placement replacement that adds one to each qualifying counter batch, including entry counters; existing counter operations modify counters as effects rather than replacing their placement.
pub(in crate::card::sets) static CARADORA_HEART_OF_ALACRIA: CardRecord = CardRecord::new(
    "Caradora, Heart of Alacria",
    "1256d22d-a2a9-41fb-b669-0661ba230bc7",
    "Mirko Failoni",
    CardRules::unsupported(),
);

// DFT 196 — Cloudspire Coordinator
// Audit: unsupported — Needs a committed entry count for Mounts and Vehicles controlled by this player this turn, including departed objects, plus a saddle power bonus for its Pilot tokens.
pub(in crate::card::sets) static CLOUDSPIRE_COORDINATOR: CardRecord = CardRecord::new(
    "Cloudspire Coordinator",
    "eef16cda-9150-4e7d-8490-d9f287b81b62",
    "Eduardo Francisco",
    CardRules::unsupported(),
);

// DFT 197 — Cloudspire Skycycle
pub(in crate::card::sets) static CLOUDSPIRE_SKYCYCLE: CardRecord = CardRecord::new(
    "Cloudspire Skycycle",
    "881f01a4-a923-4d56-bacb-984a389296fa",
    "Hardy Fowler",
    CardRules::new_vehicle(mana_cost!("{2}{R}{W}"), 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this Vehicle enters, distribute two +1/+1 counters among \
             one or two other target Vehicles and/or creatures you \
             control.",
            &[AbilityTargetDef {
                minimum: 1,
                maximum: 2,
                divided_total: Some(crate::card::DividedTotal::Fixed(2)),
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })
            }],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::DividedAmongTargets,
            },
        ),
        abilities::crew("Crew 1", 1),
    ]),
);

// DFT 198 — Coalstoke Gearhulk
pub(in crate::card::sets) static COALSTOKE_GEARHULK: CardRecord = CardRecord::new(
    "Coalstoke Gearhulk",
    "73431628-b9b0-41e6-8e9b-8a090939b0c1",
    "Nino Vecia",
    CardRules::new_artifact_creature(mana_cost!("{1}{B}{B}{R}{R}"), &["Construct"], 5, 4)
        .with_abilities(&[
            abilities::menace(),
            abilities::deathtouch(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, put target creature card with mana \
                 value 4 or less from a graveyard onto the battlefield under \
                 your control with a finality counter on it. That creature \
                 gains menace, deathtouch, and haste. At the beginning of your \
                 next end step, exile that creature.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueAtMost(4),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Any),
                    },
                )],
                EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::WithBattlefieldArrival {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        arrival: BattlefieldArrivalDef {
                            controller: Some(PlayerRelation::You),
                            modifications: &[BattlefieldEntryModificationDef::AddCounters {
                                kind: CounterKind::Finality,
                                amount: 1,
                            }],
                            ..BattlefieldArrivalDef::DEFAULT
                        },
                    },
                    binding: crate::Binding!("returned"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(
                                ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                    "returned"
                                )),
                            ),
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_ability(&abilities::menace()),
                                AppliedEffectDef::add_ability(&abilities::deathtouch()),
                                AppliedEffectDef::add_ability(&abilities::haste()),
                            ]),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                        EffectDef::InstallTrigger(InstalledTriggerDef::once(
                            &AbilityDef::triggered(
                                "At the beginning of your next end step, exile that creature.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::End,
                                    player: PlayerRelation::You,
                                },
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(
                                        ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                            crate::Binding!("returned"),
                                        ),
                                    ),
                                    ZoneKind::Exile,
                                    ZonePlacement::Top,
                                ),
                            ),
                        )),
                    ]),
                },
            ),
        ]),
);

// DFT 199 — Debris Beetle
pub(in crate::card::sets) static DEBRIS_BEETLE: CardRecord = CardRecord::new(
    "Debris Beetle",
    "405e7900-f6df-4033-b70b-b1d2a8b6d7c8",
    "Julie Dillon",
    CardRules::new_vehicle(mana_cost!("{2}{B}{G}"), 6, 6).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this Vehicle enters, each opponent loses 3 life and you \
             gain 3 life.",
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
        abilities::crew("Crew 2", 2),
    ]),
);

// DFT 200 — Dune Drifter
pub(in crate::card::sets) static DUNE_DRIFTER: CardRecord = CardRecord::new(
    "Dune Drifter",
    "36185de4-55c2-4e5d-9bcc-ea12b7052728",
    "Simon Dominic",
    CardRules::new_vehicle(mana_cost!("{X}{W}{B}"), 3, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this Vehicle enters, return target artifact or creature \
             card with mana value X or less from your graveyard to the \
             battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourceCastX),
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
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
    ]),
);

// DFT 201 — Embalmed Ascendant
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static EMBALMED_ASCENDANT: CardRecord = CardRecord::new(
    "Embalmed Ascendant",
    "5cf181ae-daa7-42f9-b667-5e679d80cf34",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// DFT 202 — Explosive Getaway
pub(in crate::card::sets) static EXPLOSIVE_GETAWAY: CardRecord = CardRecord::new(
    "Explosive Getaway",
    "a876edac-b8c8-4994-94f5-548e0fe70fe4",
    "Caio Monteiro",
    CardRules::new_sorcery(mana_cost!("{3}{R}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile up to one target artifact or creature. Return it to the \
             battlefield under its owner's control at the beginning of the \
             next end step.\nExplosive Getaway deals 4 damage to each \
             creature.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                abilities::exile_until_next_end_step(EffectRecipientDef::Target(
                    TargetIndex::PRIMARY,
                )),
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
    ]),
);

// DFT 203 — Far Fortune, End Boss
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed. Also needs a prospective damage replacement that adds 1 to each qualifying damage event rather than modifying life afterward.
pub(in crate::card::sets) static FAR_FORTUNE_END_BOSS: CardRecord = CardRecord::new(
    "Far Fortune, End Boss",
    "f523e96d-9df1-4854-accb-9876aef787e5",
    "Javier Charro",
    CardRules::unsupported(),
);

// DFT 204 — Fearless Swashbuckler
// Audit: unsupported — Needs a predicate over this combat's complete attacker declaration, requiring both a Pirate and a Vehicle, including attackers that already left; current conditions count attackers or individual predicates without that stored declaration query.
pub(in crate::card::sets) static FEARLESS_SWASHBUCKLER: CardRecord = CardRecord::new(
    "Fearless Swashbuckler",
    "0d86d66b-481f-44ec-86d8-6fc91b52ef38",
    "Konstantin Porubov",
    CardRules::unsupported(),
);

// DFT 205 — Gastal Thrillseeker
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static GASTAL_THRILLSEEKER: CardRecord = CardRecord::new(
    "Gastal Thrillseeker",
    "a8e5205d-d734-4292-a3c8-70cf5f131289",
    "Olivier Bernard",
    CardRules::unsupported(),
);

// DFT 206 — Guidelight Pathmaker
pub(in crate::card::sets) static GUIDELIGHT_PATHMAKER: CardRecord = CardRecord::new(
    "Guidelight Pathmaker",
    "b2e00cd7-925e-4bab-b064-c96aa2935f8c",
    "Stephan Martiniere",
    CardRules::new_vehicle(mana_cost!("{4}{W}{U}"), 6, 5).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this Vehicle enters, you may search your library for an \
             artifact card and reveal it. Put it onto the battlefield if \
             its mana value is 2 or less. Otherwise, put it into your \
             hand. If you search your library this way, shuffle.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: Some(crate::Binding!("found")),
                    then: Some(&EffectDef::IfElseCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("found")),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::ManaValueAtMost(2),
                                ),
                            },
                        ),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "found"
                            ))),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        otherwise: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "found"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                },
            },
        ),
        abilities::crew("Crew 2", 2),
    ]),
);

// DFT 207 — Haunt the Network
pub(in crate::card::sets) static HAUNT_THE_NETWORK: CardRecord = CardRecord::new(
    "Haunt the Network",
    "478d236c-9778-435a-ac21-bd0017a17d5b",
    "Jeff Carpenter",
    CardRules::new_sorcery(mana_cost!("{3}{U}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Choose target opponent. Create two 1/1 colorless Thopter \
             artifact creature tokens with flying. Then the chosen player \
             loses X life and you gain X life, where X is the number of \
             artifacts you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(THOPTER_TOKEN_2))
                        .with_count(ValueDef::Constant(2)),
                ),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ]),
        ),
    ]),
);

// DFT 208 — Haunted Hellride
pub(in crate::card::sets) static HAUNTED_HELLRIDE: CardRecord = CardRecord::new(
    "Haunted Hellride",
    "e2ce0d37-d5d7-49dd-885e-9998bb8abede",
    "Olivier Bernard",
    CardRules::new_vehicle(mana_cost!("{1}{U}{B}"), 3, 3).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever you attack, target creature you control gets +1/+0 \
             and gains deathtouch until end of turn. Untap it.",
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
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
        abilities::crew(
            "Crew 1 (Tap any number of creatures you control with total \
             power 1 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            1,
        ),
    ]),
);

// DFT 209 — Ketramose, the New Dawn
// Audit: unsupported — Needs one trigger per simultaneous exile batch across both battlefield and graveyards during this player's turn; existing zone-change triggers observe individual transitions and cannot merge that group.
pub(in crate::card::sets) static KETRAMOSE_THE_NEW_DAWN: CardRecord = CardRecord::new(
    "Ketramose, the New Dawn",
    "cffae8d0-7b4e-42ed-8124-24a86b38f490",
    "Maaz Ali Khan",
    CardRules::unsupported(),
);

// DFT 210 — Kolodin, Triumph Caster
pub(in crate::card::sets) static KOLODIN_TRIUMPH_CASTER: CardRecord = CardRecord::new(
    "Kolodin, Triumph Caster",
    "36bcd476-a236-4434-b191-5c8b6fa7be7b",
    "Michal Ivan",
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Pilot"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Mounts and Vehicles you control have haste.",
                EffectDef::StaticApply {
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
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::triggered(
                "Whenever a Mount you control enters, it becomes saddled until \
                 end of turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Saddle {
                    object: EffectRecipientDef::TriggeringObject,
                },
            ),
            AbilityDef::triggered(
                "Whenever a Vehicle you control enters, it becomes an artifact \
                 creature until end of turn.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::TriggeringObject,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// DFT 211 — Lagorin, Soul of Alacria
pub(in crate::card::sets) static LAGORIN_SOUL_OF_ALACRIA: CardRecord = CardRecord::new(
    "Lagorin, Soul of Alacria",
    "7b04e3f4-103b-4845-b3f0-5417971c7666",
    "Mirko Failoni",
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Beast", "Mount"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever Lagorin attacks while saddled, put a +1/+1 counter \
                 on each of up to two target Mounts and/or Vehicles.",
                TriggerEventDef::While {
                    event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Saddled,
                    },
                },
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
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
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 1 }],
                "Saddle 1 (Tap any number of other creatures you control with \
                 total power 1 or more: This Mount becomes saddled until end \
                 of turn. Saddle only as a sorcery.)",
            ),
        ]),
);

// DFT 212 — Loot, the Pathfinder
pub(in crate::card::sets) static LOOT_THE_PATHFINDER: CardRecord = CardRecord::new(
    "Loot, the Pathfinder",
    "33c59c04-4c0b-4a60-826e-3a7757d0b2a2",
    "Ernanda Souza",
// Five mana for a hasty double striker that also unloads three cards,
    // three mana, or three damage -- once each, and never twice, because
    // every one of them taps it.
    CardRules::new_creature(mana_cost!("{2}{G}{U}{R}"), &["Beast", "Noble"], 2, 4)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_abilities(&[
            abilities::double_strike(),
            abilities::vigilance(),
            abilities::haste(),
            exhaust(AbilityDef::activated_mana(
                "Exhaust — {G}, {T}: Add three mana of any one color. (Activate each exhaust ability \
                 only once.)",
                &[
                    CostDef::Mana(mana_cost!("{G}")),
                    CostDef::TapSource,
                ],
                EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
            )),
            exhaust(AbilityDef::activated(
                "Exhaust — {U}, {T}: Draw three cards.",
                &[
                    CostDef::Mana(mana_cost!("{U}")),
                    CostDef::TapSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            )),
            exhaust(AbilityDef::activated_with_targets(
                "Exhaust — {R}, {T}: This creature deals 3 damage to any target.",
                &[
                    CostDef::Mana(mana_cost!("{R}")),
                    CostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            )),
        ]),
);

// DFT 213 — Mendicant Core, Guidelight
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static MENDICANT_CORE_GUIDELIGHT: CardRecord = CardRecord::new(
    "Mendicant Core, Guidelight",
    "f434b103-490f-424e-a0a1-efb1b931c8e6",
    "Zezhou Chen",
    CardRules::unsupported(),
);

// DFT 214 — Mimeoplasm, Revered One
// Audit: unsupported — Needs an as-entry choice exiling up to X creature cards, linking those exiles and deriving entry counters from the completed exile group; current entry replacements cannot execute that variable linked-exile selection.
pub(in crate::card::sets) static MIMEOPLASM_REVERED_ONE: CardRecord = CardRecord::new(
    "Mimeoplasm, Revered One",
    "34e4c342-dc22-4e9c-81fc-a691ae9e21c1",
    "Ron Spencer",
    CardRules::unsupported(),
);

// DFT 215 — Oildeep Gearhulk
pub(in crate::card::sets) static OILDEEP_GEARHULK: CardRecord = CardRecord::new(
    "Oildeep Gearhulk",
    "a5e2d09e-b9f5-4a0d-96d3-984b5c2c387d",
    "Artur Nakhodkin",
    CardRules::new_artifact_creature(mana_cost!("{U}{U}{B}{B}"), &["Construct"], 4, 4)
        .with_abilities(&[
            abilities::lifelink(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{1}"))], "Ward {1}"),
            abilities::enters_trigger_with_targets(
                "When this creature enters, look at target player's hand. You \
                 may choose a card from it. If you do, that player discards \
                 that card, then draws a card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                        ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Hand],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        ),
                    )),
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::Any,
                    minimum: 0,
                    maximum: 1,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CountObjects(&ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            comparison: ComparisonDef::Greater,
                            right: ValueDef::Constant(0),
                        }),
                        then: &EffectDef::Sequence(&[
                            EffectDef::discard_cards(EffectRecipientDef::objects(
                                ObjectSetDef::Binding(crate::Binding!("chosen")),
                            )),
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(1),
                            },
                        ]),
                    },
                }),
            ),
        ]),
);

// DFT 216 — Pyrewood Gearhulk
pub(in crate::card::sets) static PYREWOOD_GEARHULK: CardRecord = CardRecord::new(
    "Pyrewood Gearhulk",
    "3d8493ee-bd06-4583-9908-a0dbcc5be6d7",
    "Martin de Diego Sádaba",
    CardRules::new_artifact_creature(mana_cost!("{2}{R}{R}{G}{G}"), &["Construct"], 7, 7)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::menace(),
            abilities::enters_trigger(
                "When this creature enters, other creatures you control get \
                 +2/+2 and gain vigilance and menace until end of turn. Damage \
                 can't be prevented this turn.",
                EffectDef::Sequence(&[
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
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                            AppliedEffectDef::add_ability(&abilities::vigilance()),
                            AppliedEffectDef::add_ability(&abilities::menace()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::DamageCannotBePreventedThisTurn,
                ]),
            ),
        ]),
);

// DFT 217 — Rangers' Aetherhive
// Audit: unsupported — Needs a committed activation event filtered by the exhaust mechanic label, including mana abilities; current trigger events do not observe labeled ability activations.
pub(in crate::card::sets) static RANGERS_AETHERHIVE: CardRecord = CardRecord::new(
    "Rangers' Aetherhive",
    "1b238d2c-d10f-496d-aa34-5a1536e056b5",
    "Josiah \"Jo\" Cameron",
    CardRules::unsupported(),
);

// DFT 218 — Redshift, Rocketeer Chief
pub(in crate::card::sets) static REDSHIFT_ROCKETEER_CHIEF: CardRecord = CardRecord::new(
    "Redshift, Rocketeer Chief",
    "5fba3820-32ba-47e9-9fa8-f985ff471b3f",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Goblin", "Pilot"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::activated_mana(
                "{T}: Add X mana of any one color, where X is Redshift's \
                 power. Spend this mana only to activate abilities.",
                &[CostDef::TapSource],
                EffectDef::AddMana(
                    AddManaEffectDef::any_color()
                        .with_variable_amount(ValueDef::SourcePower)
                        .with_restrictions(&[ManaRestrictionDef::ActivateAbility(
                            ObjectPredicateDef::Any,
                        )]),
                ),
            ),
            exhaust(AbilityDef::activated(
                "Exhaust — {10}{R}{G}: Put any number of permanent cards from \
                 your hand onto the battlefield. (Activate each exhaust \
                 ability only once.)",
                &[CostDef::Mana(mana_cost!("{10}{R}{G}"))],
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
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 255,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                }),
            )),
        ]),
);

// DFT 219 — Riptide Gearhulk
// Audit: unsupported — Needs a zone placement inserting a card third from the top of its owner's library, or at the bottom when fewer than three cards exist; current placements only expose top and bottom.
pub(in crate::card::sets) static RIPTIDE_GEARHULK: CardRecord = CardRecord::new(
    "Riptide Gearhulk",
    "44bfb0f7-18ca-4f6e-ba64-92120010456e",
    "Artur Nakhodkin",
    CardRules::unsupported(),
);

// DFT 220 — Rocketeer Boostbuggy
pub(in crate::card::sets) static ROCKETEER_BOOSTBUGGY: CardRecord = CardRecord::new(
    "Rocketeer Boostbuggy",
    "4c80c91e-dd3d-4c7b-89e4-bfb253eeaee2",
    "Chris Seaman",
    CardRules::new_vehicle(mana_cost!("{R}{G}"), 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this Vehicle attacks, create a Treasure token. (It's \
             an artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
        exhaust(AbilityDef::activated(
            "Exhaust — {3}: This Vehicle becomes an artifact creature. Put \
             a +1/+1 counter on it. (Activate each exhaust ability only \
             once.)",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )),
        abilities::crew("Crew 1", 1),
    ]),
);

// DFT 221 — Sab-Sunen, Luxa Embodied
// Audit: unsupported — Needs parity of the total number of counters across all counter kinds, both in static attack/block restrictions and resolution conditions; existing values select particular kinds and do not provide odd/even predicates.
pub(in crate::card::sets) static SAB_SUNEN_LUXA_EMBODIED: CardRecord = CardRecord::new(
    "Sab-Sunen, Luxa Embodied",
    "2ef555b1-666d-4386-8983-0e88f9b6cdec",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// DFT 222 — Samut, the Driving Force
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static SAMUT_THE_DRIVING_FORCE: CardRecord = CardRecord::new(
    "Samut, the Driving Force",
    "8efd8222-5c37-46d8-a2ec-1d7aae25320b",
    "Chris Rallis",
    CardRules::unsupported(),
);

// DFT 223 — Sita Varma, Masked Racer
pub(in crate::card::sets) static SITA_VARMA_MASKED_RACER: CardRecord = CardRecord::new(
    "Sita Varma, Masked Racer",
    "eb523c06-6f3e-4e19-b777-19aefc4a4e02",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{G}{U}"), &["Human", "Rogue"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[exhaust(AbilityDef::activated(
            "Exhaust — {X}{G}{G}{U}: Put X +1/+1 counters on Sita Varma. \
             Then you may have the base power and toughness of each other \
             creature you control become equal to Sita Varma's power until \
             end of turn. (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{X}{G}{G}{U}"))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::ChosenX,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Apply {
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
                        effect: AppliedEffectDef::set_base_power_toughness(
                            ValueDef::SourcePower,
                            ValueDef::SourcePower,
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ))]),
);

// DFT 224 — Skyserpent Seeker
// Audit: unsupported — Needs revealing one library collection through its second matching land before moving both lands together and randomizing the rest; current collection sources stop at the first match or a fixed count.
pub(in crate::card::sets) static SKYSERPENT_SEEKER: CardRecord = CardRecord::new(
    "Skyserpent Seeker",
    "8dbd9fc0-c0df-4119-a0d3-2e1790998c21",
    "Johan Grenier",
    CardRules::unsupported(),
);

// DFT 225 — Thundering Broodwagon
pub(in crate::card::sets) static THUNDERING_BROODWAGON: CardRecord = CardRecord::new(
    "Thundering Broodwagon",
    "2d1576d4-15a3-4e91-84ef-e71e258185e7",
    "Bartek Fedyczak",
    CardRules::new_vehicle(mana_cost!("{2}{B}{B}{G}{G}"), 6, 5).with_abilities(&[
        abilities::reach(),
        abilities::menace(),
        abilities::enters_trigger_with_targets(
            "When this Vehicle enters, destroy target nonland permanent an \
             opponent controls with mana value 4 or less.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(4),
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
        abilities::crew("Crew 3", 3),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 226 — Veteran Beastrider
pub(in crate::card::sets) static VETERAN_BEASTRIDER: CardRecord = CardRecord::new(
    "Veteran Beastrider",
    "cab38f35-b60c-464c-a0b5-9d5f2dc6de7f",
    "Andreia Ugrai",
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Human", "Knight"], 3, 4).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your end step, untap each creature you \
             control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
            },
        ),
        AbilityDef::activated(
            "{2}{G}{W}: Creatures you control get +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}{G}{W}"))],
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

// DFT 227 — Voyage Home
pub(in crate::card::sets) static VOYAGE_HOME: CardRecord = CardRecord::new(
    "Voyage Home",
    "4ba835da-0247-4716-9079-1b605297f6d5",
    "Hardy Fowler",
    CardRules::new_sorcery(mana_cost!("{5}{W}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for artifacts (This spell costs {1} less to cast for \
             each artifact you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "You draw three cards and gain 3 life.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(3)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ]),
        ),
    ]),
);

// DFT 228 — Winter, Cursed Rider
// Audit: unsupported — Needs an activated cost exiling an arbitrary chosen number of artifact cards from the graveyard and carrying that paid count as X; the ordinary activation planner currently supports only fixed single-card exile selections.
pub(in crate::card::sets) static WINTER_CURSED_RIDER: CardRecord = CardRecord::new(
    "Winter, Cursed Rider",
    "02d46d0e-3161-45b5-a49e-5cd592c67ddd",
    "Daren Bader",
    CardRules::unsupported(),
);

// DFT 229 — Zahur, Glory's Past
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static ZAHUR_GLORY_S_PAST: CardRecord = CardRecord::new(
    "Zahur, Glory's Past",
    "31944ea5-045d-481d-9aff-3c7ed663813a",
    "Leroy Steinmann",
    CardRules::unsupported(),
);

// DFT 230 — Aetherjacket
pub(in crate::card::sets) static AETHERJACKET: CardRecord = CardRecord::new(
    "Aetherjacket",
    "0a29c8a5-fd98-422c-9518-2db0993495e5",
    "Domenico Cava",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Thopter"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        AbilityDef::activated_with_targets(
            "{2}, {T}, Sacrifice this creature: Destroy another target \
             artifact. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// DFT 231 — The Aetherspark
// Audit: unsupported — Needs a planeswalker attack prohibition that applies while it is attached to a creature, plus legal Equipment planeswalker attachment state; ordinary creature attack/block restrictions do not prevent attacks against this permanent.
pub(in crate::card::sets) static THE_AETHERSPARK: CardRecord = CardRecord::new(
    "The Aetherspark",
    "05690d52-06c4-40b1-8360-380418a83250",
    "Donato Giancola",
    CardRules::unsupported(),
);

// DFT 232 — Camera Launcher
pub(in crate::card::sets) static CAMERA_LAUNCHER: CardRecord = CardRecord::new(
    "Camera Launcher",
    "968651db-92fb-46cd-acda-9e097668b7c9",
    "Ben Wootten",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 2, 2).with_abilities(&[
        exhaust(AbilityDef::activated(
            "Exhaust — {3}: Put a +1/+1 counter on this creature. Create a \
             1/1 colorless Thopter artifact creature token with flying. \
             (Activate each exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(THOPTER_TOKEN_2))
                        .with_count(ValueDef::Constant(1)),
                ),
            ]),
        )),
    ]),
);

// DFT 233 — Guidelight Matrix
pub(in crate::card::sets) static GUIDELIGHT_MATRIX: CardRecord = CardRecord::new(
    "Guidelight Matrix",
    "cccf7fb5-c043-4a1f-ad2f-edb280cb5037",
    "Eli Minaya",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::enters_trigger(
            "When this artifact enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target Mount you control becomes saddled until end \
             of turn. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mount")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Saddle {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target Vehicle you control becomes an artifact \
             creature until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_card_types(
                    CardTypeSet::single(CardType::Artifact)
                        .union(CardTypeSet::single(CardType::Creature)),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DFT 234 — Lifecraft Engine
pub(in crate::card::sets) static LIFECRAFT_ENGINE: CardRecord = CardRecord::new(
    "Lifecraft Engine",
    "40c92203-17df-4f10-92c6-ebcc79f01357",
    "Mirko Failoni",
    CardRules::new_vehicle(mana_cost!("{3}"), 4, 4).with_abilities(&[
        AbilityDef::as_enters(
            "As this permanent enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        AbilityDef::static_ability(
            "Vehicle creatures you control are the chosen creature type in \
             addition to their other types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_chosen_creature_type(),
            },
        ),
        AbilityDef::static_ability(
            "Each creature you control of the chosen type other than this \
             Vehicle gets +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::HasSourcesChosenScalar(
                                BattlefieldEntryChoiceDestinationDef::CreatureType,
                            ),
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
        abilities::crew("Crew 3", 3),
    ]),
);

// DFT 235 — Marketback Walker
pub(in crate::card::sets) static MARKETBACK_WALKER: CardRecord = CardRecord::new(
    "Marketback Walker",
    "c2152030-6007-493f-a616-545723a00249",
    "Svetlin Velinov",
    CardRules::new_artifact_creature(mana_cost!("{X}{X}"), &["Construct"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        AbilityDef::activated(
            "{4}: Put a +1/+1 counter on this creature.",
            &[CostDef::Mana(mana_cost!("{4}"))],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::dies_trigger(
            "When this creature dies, draw a card for each +1/+1 counter \
             on it.",
            abilities::draw_cards(ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne)),
        ),
    ]),
);

// DFT 236 — Marshals' Pathcruiser
pub(in crate::card::sets) static MARSHALS_PATHCRUISER: CardRecord = CardRecord::new(
    "Marshals' Pathcruiser",
    "8f920f83-6380-4e4f-be68-6bf9df3110d8",
    "Javier Charro",
    CardRules::new_vehicle(mana_cost!("{3}"), 6, 5).with_abilities(&[
        abilities::enters_trigger(
            "When this Vehicle enters, search your library for a basic \
             land card, reveal it, put it into your hand, then shuffle.",
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
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        exhaust(AbilityDef::activated(
            "Exhaust — {W}{U}{B}{R}{G}: This Vehicle becomes an artifact \
             creature. Put two +1/+1 counters on it. (Activate each \
             exhaust ability only once.)",
            &[CostDef::Mana(mana_cost!("{W}{U}{B}{R}{G}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_card_types(
                        CardTypeSet::single(CardType::Artifact)
                            .union(CardTypeSet::single(CardType::Creature)),
                    ),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            ]),
        )),
        abilities::crew("Crew 5", 5),
    ]),
);

// DFT 237 — Monument to Endurance
// Audit: unsupported — Needs a per-source, per-turn history of modes chosen, enforced while choosing triggered modes; existing modal triggers can constrain mode count but cannot exclude previously chosen modes.
pub(in crate::card::sets) static MONUMENT_TO_ENDURANCE: CardRecord = CardRecord::new(
    "Monument to Endurance",
    "d21433ba-0a14-42bc-ad0b-a4ef823a3295",
    "Victor Sales",
    CardRules::unsupported(),
);

// DFT 238 — Pit Automaton
// Audit: unsupported — Needs an event identifying a nonmana exhaust activation and a delayed trigger consumed by the next such event before end of turn; neither the activation event nor that combined lifetime is represented.
pub(in crate::card::sets) static PIT_AUTOMATON: CardRecord = CardRecord::new(
    "Pit Automaton",
    "c72527ef-ac05-44c8-8c76-10532ce3da6e",
    "Villarrte",
    CardRules::unsupported(),
);

// DFT 239 — Racers' Scoreboard
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static RACERS_SCOREBOARD: CardRecord = CardRecord::new(
    "Racers' Scoreboard",
    "50bae2ba-a6a0-4a6a-96e9-0e0372e55108",
    "Konstantin Porubov",
    CardRules::unsupported(),
);

// DFT 240 — Radiant Lotus
// Audit: unsupported — Needs a targeted mana-producing activation with one-or-more artifact sacrifice selection and output scaled by the completed payment; ordinary sacrifice activations support fixed counts, not this chosen group.
pub(in crate::card::sets) static RADIANT_LOTUS: CardRecord = CardRecord::new(
    "Radiant Lotus",
    "be6dac83-39c2-40dc-a322-76a3ea4e7aee",
    "Bruce Brenneise",
    CardRules::unsupported(),
);

// DFT 241 — Rover Blades
pub(in crate::card::sets) static ROVER_BLADES: CardRecord = CardRecord::new(
    "Rover Blades",
    "8855a999-83f6-4c0d-9444-3ff292f77d58",
    "Nathaniel Himawan",
    CardRules::new_vehicle(mana_cost!("{3}"), 2, 2).with_abilities(&[
        abilities::double_strike(),
        AbilityDef::static_ability(
            "Equipped creature has double strike.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
            },
        ),
        abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn. Creatures can't be attached to other \
             permanents.)",
            2,
        ),
    ]),
);

// DFT 242 — Scrap Compactor
pub(in crate::card::sets) static SCRAP_COMPACTOR: CardRecord = CardRecord::new(
    "Scrap Compactor",
    "12ccd555-60d4-49a1-b022-1e119344172a",
    "Viko Menezes",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}, {T}, Sacrifice this artifact: It deals 3 damage to \
             target creature.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
        AbilityDef::activated_with_targets(
            "{6}, {T}, Sacrifice this artifact: Destroy target creature or \
             Vehicle.",
            &[
                CostDef::Mana(mana_cost!("{6}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vehicle")),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// DFT 243 — Skybox Ferry
pub(in crate::card::sets) static SKYBOX_FERRY: CardRecord = CardRecord::new(
    "Skybox Ferry",
    "75d9ef34-ae95-4465-a30c-372e231bd733",
    "Borja Pindado",
    CardRules::new_vehicle(mana_cost!("{5}"), 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::crew(
            "Crew 2 (Tap any number of creatures you control with total \
             power 2 or more: This Vehicle becomes an artifact creature \
             until end of turn.)",
            2,
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 244 — Starting Column
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static STARTING_COLUMN: CardRecord = CardRecord::new(
    "Starting Column",
    "0530b343-98c2-440a-b32e-d1566d318c3b",
    "Jakub Kasper",
    CardRules::unsupported(),
);

// DFT 245 — Ticket Tortoise
pub(in crate::card::sets) static TICKET_TORTOISE: CardRecord = CardRecord::new(
    "Ticket Tortoise",
    "fa178ed7-8f3a-45f0-817d-5fbc7993b04a",
    "Brian Valeza",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Turtle"], 3, 1).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered_if(
            "When this creature enters, if an opponent controls more lands \
             than you, you create a Treasure token. (It's an artifact with \
             \"{T}, Sacrifice this token: Add one mana of any color.\")",
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
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// DFT 246 — Walking Sarcophagus
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static WALKING_SARCOPHAGUS: CardRecord = CardRecord::new(
    "Walking Sarcophagus",
    "89ccbd73-9414-48a3-bdcf-e838fcffc08f",
    "Julia Metzger",
    CardRules::unsupported(),
);

// DFT 247 — Wreck Remover
pub(in crate::card::sets) static WRECK_REMOVER: CardRecord = CardRecord::new(
    "Wreck Remover",
    "e3151960-cc0c-47b5-b476-295d7a17ae14",
    "Villarrte",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 3, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature enters or attacks, exile up to one \
             target card from a graveyard. You gain 1 life.",
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
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))]
        ),
    ]),
);

// DFT 248 — Amonkhet Raceway
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static AMONKHET_RACEWAY: CardRecord = CardRecord::new(
    "Amonkhet Raceway",
    "4f312807-ea2a-4385-8774-4e23b4a5d4a6",
    "Brian Valeza",
    CardRules::unsupported(),
);

// DFT 249 — Avishkar Raceway
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static AVISHKAR_RACEWAY: CardRecord = CardRecord::new(
    "Avishkar Raceway",
    "08a6b378-c7fa-4226-a310-4ee7e550b4d6",
    "Julian Kok Joon Wen",
    CardRules::unsupported(),
);

// DFT 250 — Bleachbone Verge
pub(in crate::card::sets) static BLEACHBONE_VERGE: CardRecord = CardRecord::new(
    "Bleachbone Verge",
    "52dcdabd-a186-45fe-9fee-6c0f1afeaf16",
    "Mark Tedin",
    // Untapped and free either way: the black is unconditional, and the
    // white is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {W}. Activate only if you control a Plains or a Swamp.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Orzhov colours. Either type answers
                // it, so a Godless Shrine is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Plains,
                        BasicLandType::Swamp,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ]),
);

// DFT 251 — Bloodfell Caves (reprint)
const BLOODFELL_CAVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOODFELL_CAVES,
    "49195d90-de0c-4290-aa1c-9f4d948b5521",
    "Ron Spencer",
);

// DFT 252 — Blossoming Sands (reprint)
const BLOSSOMING_SANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOSSOMING_SANDS,
    "69949863-2510-4fe2-a815-0682beeb08a3",
    "Valera Lutfullina",
);

// DFT 253 — Country Roads
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static COUNTRY_ROADS: CardRecord = CardRecord::new(
    "Country Roads",
    "897acd91-12ba-4fa8-a26e-c09f009167a8",
    "Mark Poole",
    CardRules::unsupported(),
);

// DFT 254 — Dismal Backwater (reprint)
const DISMAL_BACKWATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::DISMAL_BACKWATER,
    "f20aef3f-79f6-4357-8631-1d141f437def",
    "Wayne Wu",
);

// DFT 255 — Foul Roads
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static FOUL_ROADS: CardRecord = CardRecord::new(
    "Foul Roads",
    "a37c026a-c89f-41f3-8812-424124dd3760",
    "Borja Pindado",
    CardRules::unsupported(),
);

// DFT 256 — Jungle Hollow (reprint)
const JUNGLE_HOLLOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::JUNGLE_HOLLOW,
    "7dd24a1a-08db-4fa7-8939-f5541677327e",
    "Eddie Mendoza",
);

// DFT 257 — Muraganda Raceway
// Audit: unsupported — Needs the speed player designation: starting at 1, increasing once during each of that player's turns when an opponent loses life, and capping at 4; current player state and value predicates do not represent speed or max speed.
pub(in crate::card::sets) static MURAGANDA_RACEWAY: CardRecord = CardRecord::new(
    "Muraganda Raceway",
    "5041ae16-29ff-4ad5-8a37-4736e9409294",
    "Brian Valeza",
    CardRules::unsupported(),
);

// DFT 258 — Night Market
pub(in crate::card::sets) static NIGHT_MARKET: CardRecord = CardRecord::new(
    "Night Market",
    "a8c1dce3-6136-4294-9d2b-5ef8527d733b",
    "David Álvarez",
    // A tapped land that fixes one colour and cycles away once the mana is
    // there, so it is never the draw that loses the game.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::as_enters(
            "As this land enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
        abilities::cycling!(
            "Cycling {3} ({3}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{3}"))],
        ),
    ]),
);

// DFT 259 — Reef Roads
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static REEF_ROADS: CardRecord = CardRecord::new(
    "Reef Roads",
    "73a8171a-2629-4356-ae86-b4d03aa14bd3",
    "David Álvarez",
    CardRules::unsupported(),
);

// DFT 260 — Riverpyre Verge
pub(in crate::card::sets) static RIVERPYRE_VERGE: CardRecord = CardRecord::new(
    "Riverpyre Verge",
    "57a93a71-d77c-417f-85d0-cd420f573331",
    "Titus Lunter",
    // Untapped and free either way: the red is unconditional, and the blue
    // is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {U}. Activate only if you control an Island or a Mountain.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The same verge condition in this cycle's other pair of colours: either
                // type answers it, so a Volcanic Island is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Island,
                        BasicLandType::Mountain,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// DFT 261 — Rocky Roads
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static ROCKY_ROADS: CardRecord = CardRecord::new(
    "Rocky Roads",
    "6f0d4e8a-aab5-458e-bc0e-2b6b0054646a",
    "Arthur Yuan",
    CardRules::unsupported(),
);

// DFT 262 — Rugged Highlands (reprint)
const RUGGED_HIGHLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::RUGGED_HIGHLANDS,
    "73b7484f-923c-46c5-95bf-c2c6706c0d48",
    "Florian de Gesincourt",
);

// DFT 263 — Scoured Barrens (reprint)
const SCOURED_BARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SCOURED_BARRENS,
    "ac52be0c-49f4-4956-820e-bdd7d2744d2f",
    "Eddie Mendoza",
);

// DFT 264 — Sunbillow Verge
pub(in crate::card::sets) static SUNBILLOW_VERGE: CardRecord = CardRecord::new(
    "Sunbillow Verge",
    "94ed132f-b818-4dbf-9b4a-e5acb067e0a4",
    "Pete Venters",
    // Untapped and free either way: the white is unconditional, and the red
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {R}. Activate only if you control a Mountain or a Plains.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Boros colours. Either type answers
                // it, so a Plateau is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Plains,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DFT 265 — Swiftwater Cliffs (reprint)
const SWIFTWATER_CLIFFS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SWIFTWATER_CLIFFS,
    "949bf6bb-1e97-48a3-8547-0a780664c275",
    "Mark Poole",
);

// DFT 266 — Thornwood Falls (reprint)
const THORNWOOD_FALLS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::THORNWOOD_FALLS,
    "4723a303-7f20-4399-8bc6-6a27a61a3532",
    "Eddie Mendoza",
);

// DFT 267 — Tranquil Cove (reprint)
const TRANQUIL_COVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::TRANQUIL_COVE,
    "70688800-7675-4e75-bb3c-9e05b95a685c",
    "Wayne Wu",
);

// DFT 268 — Wastewood Verge
pub(in crate::card::sets) static WASTEWOOD_VERGE: CardRecord = CardRecord::new(
    "Wastewood Verge",
    "5ceacc7d-d407-4f82-af58-9bdf8426924e",
    "Bartek Fedyczak",
    // Untapped and free either way: the green is unconditional, and the
    // black is what the second land in the deck is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {B}. Activate only if you control a Swamp or a Forest.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition: any land you control with either type answers it,
                // so a Bayou is both halves at once and a land whose types were changed
                // counts for what it is now rather than what it was printed as.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Swamp,
                        BasicLandType::Forest,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// DFT 269 — Wild Roads
// Audit: unsupported — Needs the printed power contribution bonus for both saddling Mounts and crewing Vehicles; CrewsAsThoughPowerGreater is deliberately limited to Vehicles and does not increase saddle payment contributions.
pub(in crate::card::sets) static WILD_ROADS: CardRecord = CardRecord::new(
    "Wild Roads",
    "4d3d48d1-a98e-40af-b04c-c40b9d52e9ee",
    "Leanna Crossan",
    CardRules::unsupported(),
);

// DFT 270 — Willowrush Verge
pub(in crate::card::sets) static WILLOWRUSH_VERGE: CardRecord = CardRecord::new(
    "Willowrush Verge",
    "758d93d5-3f66-4395-a928-000485396c87",
    "Aaron Miller",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Blue),
        AbilityDef::activated_mana_if(
            "{T}: Add {G}. Activate only if you control a Forest or an Island.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// DFT 271 — Wind-Scarred Crag (reprint)
const WIND_SCARRED_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::WIND_SCARRED_CRAG,
    "882969d8-7e3e-4f32-858a-00da20c67b83",
    "Svetlin Velinov",
);

// DFT 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "2d2da8b9-797f-46e8-a15d-a86c7c56dc84",
    "Adam Paquette",
);

// DFT 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "f53ed206-8ec4-46a6-8603-c819682e1433",
    "Maxime Minard",
);

// DFT 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "e969e168-e74c-4608-9fa0-a5660bf7fa02",
    "Jonas De Ro",
);

// DFT 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "50748a4c-43a0-4274-9a84-046d76027166",
    "Chris Ostrowski",
);

// DFT 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "777e868d-cb88-4b8e-99d1-12ff67f5eae2",
    "Andreas Rocha",
);

// DFT 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "464723b3-0723-45f2-a258-32d098c39039",
    "Samuele Bandini",
);

// DFT 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "4b97d2f5-498c-42db-95d9-f9002d4dfcc5",
    "Titus Lunter",
);

// DFT 279 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "a1cf2f81-3041-4349-9b37-f215c4e38c5b",
    "Leon Tukker",
);

// DFT 280 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "5a8e9b9e-4947-4b6a-b21b-6fe009760fc5",
    "Samuele Bandini",
);

// DFT 281 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "f9369d0a-87c0-4c29-b43c-cd45c007f8d6",
    "Mark Poole",
);

// DFT 282 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "7a3c4a9d-48db-474f-aeb6-8b4fa08ce6fa",
    "Leon Tukker",
);

// DFT 283 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "040d064e-b023-4750-afeb-1f58e36bc4ab",
    "Samuele Bandini",
);

// DFT 284 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "73e124b6-3af7-41c0-b829-ff85aa0c3782",
    "Ron Spencer",
);

// DFT 285 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "85288987-6f3f-4b7b-9ecc-9393dd37f115",
    "Leon Tukker",
);

// DFT 286 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "fb76cb6c-2f4d-4ca2-876f-d49ea529e59b",
    "Samuele Bandini",
);

// DFT 287 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "c872e70c-626f-4d2c-b26b-d94f815fceea",
    "Florian de Gesincourt",
);

// DFT 288 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "653e2393-30a7-4e0c-bd20-f05ca0ba3cb6",
    "Leon Tukker",
);

// DFT 289 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "b69adddd-3626-45d6-8100-26cf1f314d00",
    "Samuele Bandini",
);

// DFT 290 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "6472e2a6-287f-4e7c-ad85-1454e40d4a46",
    "Yeong-Hao Han",
);

// DFT 291 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "562b16ae-8ae7-4b2d-9098-bf3ff1429f7b",
    "Leon Tukker",
);

// DFT 292 — Air Response Unit (alternate printing)
const AIR_RESPONSE_UNIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AIR_RESPONSE_UNIT,
    1,
    "89e66d63-56be-46f5-b0df-e1350106ba45",
    "SchmandrewART",
);

// DFT 293 — Broadcast Rambler (alternate printing)
const BROADCAST_RAMBLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BROADCAST_RAMBLER,
    1,
    "b6118bb1-9530-487b-a211-7cc56cef3fbb",
    "Francisco Badilla",
);

// DFT 294 — Detention Chariot (alternate printing)
const DETENTION_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DETENTION_CHARIOT,
    1,
    "0e45f00e-2156-4e8c-813c-0e36ab34982e",
    "Arik Roper",
);

// DFT 295 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    1,
    "37c2e68f-3f40-45b1-9afe-cc1c5fb41203",
    "Adam Volker",
);

// DFT 296 — Skyseer's Chariot (alternate printing)
const SKYSEER_S_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYSEER_S_CHARIOT,
    1,
    "ec0eb86e-371c-46fd-a261-384b2a603b36",
    "Douglas P. Lobo",
);

// DFT 297 — Spotcycle Scouter (alternate printing)
const SPOTCYCLE_SCOUTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPOTCYCLE_SCOUTER,
    1,
    "c0303505-d6f1-4626-a932-0e577d9572fe",
    "Neo.G",
);

// DFT 298 — Valor's Flagship (alternate printing)
const VALOR_S_FLAGSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALOR_S_FLAGSHIP,
    1,
    "f142d6c1-90f6-49d6-861e-36e9a6fac958",
    "William Tempest",
);

// DFT 299 — Voyager Glidecar (alternate printing)
const VOYAGER_GLIDECAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VOYAGER_GLIDECAR,
    1,
    "e7c7da86-902a-4d40-a230-4d22d4b2df59",
    "Andrew Griffith",
);

// DFT 300 — Hulldrifter (alternate printing)
const HULLDRIFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HULLDRIFTER,
    1,
    "0279171d-1623-438c-b796-d294c6675604",
    "Juan Marquez",
);

// DFT 301 — Midnight Mangler (alternate printing)
const MIDNIGHT_MANGLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIDNIGHT_MANGLER,
    1,
    "4aeb6fa4-569e-4ede-94c5-a8f3c28f55e4",
    "Boneface",
);

// DFT 302 — Possession Engine (alternate printing)
const POSSESSION_ENGINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POSSESSION_ENGINE,
    1,
    "821045bd-c8e4-40fb-babd-5d13b64a71ec",
    "Douglas P. Lobo",
);

// DFT 303 — Rangers' Refueler (alternate printing)
const RANGERS_REFUELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RANGERS_REFUELER,
    1,
    "530159a6-0672-485a-867c-aa0bac961765",
    "Francisco Badilla",
);

// DFT 304 — Thopter Fabricator (alternate printing)
const THOPTER_FABRICATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THOPTER_FABRICATOR,
    1,
    "1dd26a5a-ca58-4cb6-b599-7c84d601dc07",
    "William Tempest",
);

// DFT 305 — Carrion Cruiser (alternate printing)
const CARRION_CRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARRION_CRUISER,
    1,
    "818931d1-258d-4dda-9d0a-2015ec330bab",
    "Ian Jepson",
);

// DFT 306 — Cryptcaller Chariot (alternate printing)
const CRYPTCALLER_CHARIOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYPTCALLER_CHARIOT,
    1,
    "914f4e3c-2729-488b-883f-7657deea39c8",
    "Oliver Barrett",
);

// DFT 307 — Demonic Junker (alternate printing)
const DEMONIC_JUNKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_JUNKER,
    1,
    "c322936f-3b56-454c-a89e-fa53faab6a33",
    "Deathburger",
);

// DFT 308 — The Last Ride (alternate printing)
const THE_LAST_RIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RIDE,
    1,
    "f2d563b1-f03b-4ce3-9de0-05dab257b67c",
    "Deathburger",
);

// DFT 309 — Ripclaw Wrangler (alternate printing)
const RIPCLAW_WRANGLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIPCLAW_WRANGLER,
    1,
    "6ba1b096-7424-4ce9-8dfd-56a86fe612e4",
    "Andrew Griffith",
);

// DFT 310 — Boommobile (alternate printing)
const BOOMMOBILE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOOMMOBILE,
    1,
    "3501b938-7a25-43c7-b1c5-03cd9b690930",
    "Ian Jepson",
);

// DFT 311 — Burner Rocket (alternate printing)
const BURNER_ROCKET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BURNER_ROCKET,
    1,
    "267c93bd-1313-4c7b-bd6f-91e329932dea",
    "Carl Critchlow",
);

// DFT 312 — Clamorous Ironclad (alternate printing)
const CLAMOROUS_IRONCLAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLAMOROUS_IRONCLAD,
    1,
    "79d3c041-976c-41e2-9ce6-4b292e499526",
    "Cosmin Podar",
);

// DFT 313 — Gastal Thrillroller (alternate printing)
const GASTAL_THRILLROLLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_THRILLROLLER,
    1,
    "03a07256-3305-4178-bd4f-d1583d23106d",
    "Dan Mumford",
);

// DFT 314 — Spire Mechcycle (alternate printing)
const SPIRE_MECHCYCLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIRE_MECHCYCLE,
    1,
    "5f348c0a-41f3-4d88-be1d-19c0821f158e",
    "Neo.G",
);

// DFT 315 — Earthrumbler (alternate printing)
const EARTHRUMBLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTHRUMBLER,
    1,
    "0d95a5f3-7f14-4d59-8329-559ed4ff0491",
    "SchmandrewART",
);

// DFT 316 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    1,
    "11c196dd-61a9-4e75-bd43-9531ba3dc961",
    "Francisco Badilla",
);

// DFT 317 — Thunderous Velocipede (alternate printing)
const THUNDEROUS_VELOCIPEDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_VELOCIPEDE,
    1,
    "3e5f6a1c-7b34-4c23-8172-5b869c96eb65",
    "Eduardo Francisco",
);

// DFT 318 — Veloheart Bike (alternate printing)
const VELOHEART_BIKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VELOHEART_BIKE,
    1,
    "cfdffb27-5035-49c2-b214-abe46a17433d",
    "Juan Marquez",
);

// DFT 319 — Apocalypse Runner (alternate printing)
const APOCALYPSE_RUNNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &APOCALYPSE_RUNNER,
    1,
    "ea55a6ec-634c-4590-9875-db4e5b81795d",
    "Arik Roper",
);

// DFT 320 — Boosted Sloop (alternate printing)
const BOOSTED_SLOOP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BOOSTED_SLOOP,
    1,
    "85bb2217-d3e1-4638-9a14-bb72bfedd18c",
    "Gabriel Rubio",
);

// DFT 321 — Cloudspire Skycycle (alternate printing)
const CLOUDSPIRE_SKYCYCLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOUDSPIRE_SKYCYCLE,
    1,
    "73918759-b7b6-4a09-baa1-7f660ecef42e",
    "Yuko Shimizu",
);

// DFT 322 — Debris Beetle (alternate printing)
const DEBRIS_BEETLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEBRIS_BEETLE,
    1,
    "b8fe7293-2d85-42dd-b6a1-21e98487cbfa",
    "Michal Ivan",
);

// DFT 323 — Dune Drifter (alternate printing)
const DUNE_DRIFTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DUNE_DRIFTER,
    1,
    "71586675-e9f5-4bec-984d-8683b2494b35",
    "Arik Roper",
);

// DFT 324 — Guidelight Pathmaker (alternate printing)
const GUIDELIGHT_PATHMAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUIDELIGHT_PATHMAKER,
    1,
    "83566cd0-2253-477e-ae99-4c0c4902ad6e",
    "Dan Mumford",
);

// DFT 325 — Haunted Hellride (alternate printing)
const HAUNTED_HELLRIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAUNTED_HELLRIDE,
    1,
    "9d68c20a-f377-4613-8771-cdda1745cdff",
    "Deathburger",
);

// DFT 326 — Rangers' Aetherhive (alternate printing)
const RANGERS_AETHERHIVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RANGERS_AETHERHIVE,
    1,
    "3f0d5c5c-d3d6-48ae-9775-fcd2cd9a0c65",
    "Francisco Badilla",
);

// DFT 327 — Rocketeer Boostbuggy (alternate printing)
const ROCKETEER_BOOSTBUGGY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROCKETEER_BOOSTBUGGY,
    1,
    "2db27c67-0462-474c-844b-4f69d8fe6734",
    "Adam Volker",
);

// DFT 328 — Thundering Broodwagon (alternate printing)
const THUNDERING_BROODWAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDERING_BROODWAGON,
    1,
    "b4adfba1-c73a-43b4-8400-e11617958f7f",
    "Villarrte",
);

// DFT 329 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    1,
    "1f0aa565-1ef0-41f5-a86d-0701e357cde9",
    "Eduardo Francisco",
);

// DFT 330 — Marshals' Pathcruiser (alternate printing)
const MARSHALS_PATHCRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARSHALS_PATHCRUISER,
    1,
    "5c4e88ae-7f5c-4dea-add0-a53a6a9bd8e4",
    "Michal Ivan",
);

// DFT 331 — Rover Blades (alternate printing)
const ROVER_BLADES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROVER_BLADES,
    1,
    "5a3b07b7-8d51-428e-a399-61da45529c8f",
    "Eduardo Francisco",
);

// DFT 332 — Skybox Ferry (alternate printing)
const SKYBOX_FERRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYBOX_FERRY,
    1,
    "cd9e294e-cdb2-423d-a034-f68f673b5973",
    "SchmandrewART",
);

// DFT 333 — Bulwark Ox (alternate printing)
const BULWARK_OX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BULWARK_OX,
    1,
    "27736f40-a87b-45e4-a0da-accf9433c1fc",
    "Jon Vermilyea",
);

// DFT 334 — Guardian Sunmare (alternate printing)
const GUARDIAN_SUNMARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUARDIAN_SUNMARE,
    1,
    "309072be-b813-42bd-adaf-7d45053029c0",
    "Wojtek Łebski",
);

// DFT 335 — Mindspring Merfolk (alternate printing)
const MINDSPRING_MERFOLK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MINDSPRING_MERFOLK,
    1,
    "bcda94fc-43f0-4fa6-b38b-bb8c4c26d9a1",
    "CatDirty",
);

// DFT 336 — Waxen Shapethief (alternate printing)
const WAXEN_SHAPETHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WAXEN_SHAPETHIEF,
    1,
    "7fc820fb-eec2-4873-a272-68e8379470fc",
    "CatDirty",
);

// DFT 337 — Bloodghast (alternate printing)
const BLOODGHAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::BLOODGHAST,
    1,
    "4560bd1f-eb63-45bc-a8c7-b5de200facc9",
    "Wojtek Łebski",
);

// DFT 338 — Gas Guzzler (alternate printing)
const GAS_GUZZLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GAS_GUZZLER,
    1,
    "3150199e-9e4f-486d-bfb0-fc2a9feb8536",
    "Sam McKenzie",
);

// DFT 339 — The Speed Demon (alternate printing)
const THE_SPEED_DEMON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SPEED_DEMON,
    1,
    "6c4045e7-6d2c-4ef9-922e-3681fa820c4b",
    "CatDirty",
);

// DFT 340 — Burnout Bashtronaut (alternate printing)
const BURNOUT_BASHTRONAUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BURNOUT_BASHTRONAUT,
    1,
    "eb1ffa8e-2794-4f11-8351-42f1a0f26831",
    "Jon Vermilyea",
);

// DFT 341 — Draconautics Engineer (alternate printing)
const DRACONAUTICS_ENGINEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRACONAUTICS_ENGINEER,
    1,
    "4712279a-ae45-488c-870a-0187201c860f",
    "Ryan Roadkill",
);

// DFT 342 — Howlsquad Heavy (alternate printing)
const HOWLSQUAD_HEAVY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOWLSQUAD_HEAVY,
    1,
    "6a758d38-adfe-4894-bf27-d988a228e944",
    "Ryan Roadkill",
);

// DFT 343 — Agonasaur Rex (alternate printing)
const AGONASAUR_REX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGONASAUR_REX,
    1,
    "328d60a4-af43-4a2e-98b5-1098478eeb02",
    "Jon Vermilyea",
);

// DFT 344 — District Mascot (alternate printing)
const DISTRICT_MASCOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DISTRICT_MASCOT,
    1,
    "15f978b9-d464-42f7-ba8b-0c0795d54cb5",
    "Ryan Roadkill",
);

// DFT 345 — Webstrike Elite (alternate printing)
const WEBSTRIKE_ELITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEBSTRIKE_ELITE,
    1,
    "14e8e016-2b68-43d1-b25a-553bad97e431",
    "Sam McKenzie",
);

// DFT 346 — Fearless Swashbuckler (alternate printing)
const FEARLESS_SWASHBUCKLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEARLESS_SWASHBUCKLER,
    1,
    "2e8aff7b-bcf4-46a2-a8f5-dc00394a84da",
    "CatDirty",
);

// DFT 347 — Hazoret, Godseeker (alternate printing)
const HAZORET_GODSEEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAZORET_GODSEEKER,
    1,
    "e5e6cbde-ce5b-4c25-9a27-715fe7abe4e4",
    "Massiveface",
);

// DFT 348 — Brightglass Gearhulk (alternate printing)
const BRIGHTGLASS_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRIGHTGLASS_GEARHULK,
    1,
    "82000701-d4a8-4f9f-a4b0-236858c84954",
    "Jorge Gutierrez Garcia",
);

// DFT 349 — Coalstoke Gearhulk (alternate printing)
const COALSTOKE_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COALSTOKE_GEARHULK,
    1,
    "b4574869-9598-4295-bd43-ce3633e0d8e6",
    "Ivan Shavrin",
);

// DFT 350 — Ketramose, the New Dawn (alternate printing)
const KETRAMOSE_THE_NEW_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KETRAMOSE_THE_NEW_DAWN,
    1,
    "3c8a81bf-f63c-40ca-a4da-db68df3f3e1a",
    "Florian Bertmer",
);

// DFT 351 — Oildeep Gearhulk (alternate printing)
const OILDEEP_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OILDEEP_GEARHULK,
    1,
    "099ead92-1d35-495c-8ee8-14d73e9f1437",
    "Chun Lo",
);

// DFT 352 — Pyrewood Gearhulk (alternate printing)
const PYREWOOD_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PYREWOOD_GEARHULK,
    1,
    "e0aa0bc1-3c31-4886-bfd2-f1ab6e048559",
    "William Tempest",
);

// DFT 353 — Riptide Gearhulk (alternate printing)
const RIPTIDE_GEARHULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIPTIDE_GEARHULK,
    1,
    "b6cdbb98-a5b8-4b9e-aaaa-457eb9d4041e",
    "Kudaman",
);

// DFT 354 — Sab-Sunen, Luxa Embodied (alternate printing)
const SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAB_SUNEN_LUXA_EMBODIED,
    1,
    "43655ecc-af27-494f-bee7-a5542cae9b54",
    "Benjamin Ee",
);

// DFT 355 — Basri, Tomorrow's Champion (alternate printing)
const BASRI_TOMORROW_S_CHAMPION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BASRI_TOMORROW_S_CHAMPION,
    1,
    "440766a6-5820-4248-ad91-4d946235e136",
    "Justyna Dura",
);

// DFT 356 — Vnwxt, Verbose Host (alternate printing)
const VNWXT_VERBOSE_HOST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VNWXT_VERBOSE_HOST,
    1,
    "a7d0d849-d400-43fc-a42a-0eeb27131d1b",
    "Chris Seaman",
);

// DFT 357 — Gonti, Night Minister (alternate printing)
const GONTI_NIGHT_MINISTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GONTI_NIGHT_MINISTER,
    1,
    "38a867af-cf5d-4e3d-bba1-361e3cfeca82",
    "Richard Kane Ferguson",
);

// DFT 358 — Daretti, Rocketeer Engineer (alternate printing)
const DARETTI_ROCKETEER_ENGINEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DARETTI_ROCKETEER_ENGINEER,
    1,
    "3993ac84-304a-4e42-aa39-b72be30471ea",
    "Samuel Perin",
);

// DFT 359 — Oviya, Automech Artisan (alternate printing)
const OVIYA_AUTOMECH_ARTISAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVIYA_AUTOMECH_ARTISAN,
    1,
    "c9b47651-5607-40b0-9e73-ad356b10acc9",
    "Ron Spears",
);

// DFT 360 — Aatchik, Emerald Radian (alternate printing)
const AATCHIK_EMERALD_RADIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AATCHIK_EMERALD_RADIAN,
    1,
    "e789df76-d658-47a4-9efb-74da6bd8821c",
    "Erica Williams",
);

// DFT 361 — Captain Howler, Sea Scourge (alternate printing)
const CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_HOWLER_SEA_SCOURGE,
    1,
    "a8f0f937-5519-4d38-9fc3-46460669bb86",
    "Mark Zug",
);

// DFT 362 — Caradora, Heart of Alacria (alternate printing)
const CARADORA_HEART_OF_ALACRIA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARADORA_HEART_OF_ALACRIA,
    1,
    "a7bca478-48c0-4c39-95ae-ba8afb207153",
    "Yuko Shimizu",
);

// DFT 363 — Far Fortune, End Boss (alternate printing)
const FAR_FORTUNE_END_BOSS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAR_FORTUNE_END_BOSS,
    1,
    "ae0bd239-25ae-496e-9be2-c2d968dc9346",
    "Justine Jones",
);

// DFT 364 — Kolodin, Triumph Caster (alternate printing)
const KOLODIN_TRIUMPH_CASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOLODIN_TRIUMPH_CASTER,
    1,
    "33887384-a2bb-4daf-b932-a4259c93b808",
    "John Stanko",
);

// DFT 365 — Mendicant Core, Guidelight (alternate printing)
const MENDICANT_CORE_GUIDELIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MENDICANT_CORE_GUIDELIGHT,
    1,
    "6f11d7a7-72e9-406e-a701-5c3a556a43e0",
    "Dan Mumford",
);

// DFT 366 — Redshift, Rocketeer Chief (alternate printing)
const REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REDSHIFT_ROCKETEER_CHIEF,
    1,
    "c56bc66f-12d1-4eae-8de7-2fae34976d6b",
    "Xavier Ribeiro",
);

// DFT 367 — Samut, the Driving Force (alternate printing)
const SAMUT_THE_DRIVING_FORCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAMUT_THE_DRIVING_FORCE,
    1,
    "227036a1-7570-4657-9c0a-635dd6d889e9",
    "Mark Poole",
);

// DFT 368 — Sita Varma, Masked Racer (alternate printing)
const SITA_VARMA_MASKED_RACER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SITA_VARMA_MASKED_RACER,
    1,
    "119a39ce-80f6-48b3-b159-e5d919d3b617",
    "rk post",
);

// DFT 369 — Winter, Cursed Rider (alternate printing)
const WINTER_CURSED_RIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINTER_CURSED_RIDER,
    1,
    "61aa759b-a940-4071-9373-13e6aef62fe4",
    "Ovidio Cartagena",
);

// DFT 370 — Zahur, Glory's Past (alternate printing)
const ZAHUR_GLORY_S_PAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZAHUR_GLORY_S_PAST,
    1,
    "e1004f64-5052-4b1e-8fd8-eda463fbd9b7",
    "Alex Stone",
);

// DFT 371 — Bleachbone Verge (alternate printing)
const BLEACHBONE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLEACHBONE_VERGE,
    1,
    "6ba2edc7-dbc0-4948-9f37-a27a91bdf7b9",
    "Daren Bader",
);

// DFT 372 — Riverpyre Verge (alternate printing)
const RIVERPYRE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIVERPYRE_VERGE,
    1,
    "70efc6f3-2fdc-4629-84b9-f3707b799460",
    "Justin Sweet",
);

// DFT 373 — Sunbillow Verge (alternate printing)
const SUNBILLOW_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNBILLOW_VERGE,
    1,
    "c92a7ca5-a25b-4888-a319-6e176ab4d0ce",
    "Darrell Riche",
);

// DFT 374 — Wastewood Verge (alternate printing)
const WASTEWOOD_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WASTEWOOD_VERGE,
    1,
    "78ab2fc7-7945-4b49-8ad0-223815d0d2c2",
    "Kev Walker",
);

// DFT 375 — Willowrush Verge (alternate printing)
const WILLOWRUSH_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILLOWRUSH_VERGE,
    1,
    "38218d0f-4b70-45eb-a753-cdb44f94271c",
    "Carl Critchlow",
);

// DFT 376 — The Aetherspark (alternate printing)
const THE_AETHERSPARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_AETHERSPARK,
    1,
    "934cc1ca-2ce8-4062-b1b7-1976e76da8b6",
    "Dominik Mayer",
);

// DFT 377 — Perilous Snare (alternate printing)
const PERILOUS_SNARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PERILOUS_SNARE,
    1,
    "d1edff87-8563-4cb5-ab9b-7696e920346a",
    "Chris Seaman",
);

// DFT 378 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    1,
    "b513dfeb-d7c8-4e4d-b419-f67b45608b4b",
    "Zezhou Chen",
);

// DFT 379 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    1,
    "c4797dcb-507f-4219-95ff-994e2c0d251b",
    "Justyna Dura",
);

// DFT 380 — Repurposing Bay (alternate printing)
const REPURPOSING_BAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REPURPOSING_BAY,
    1,
    "231c4f52-5a12-407f-9917-d2ca91ba4706",
    "William Tempest",
);

// DFT 381 — Riverchurn Monument (alternate printing)
const RIVERCHURN_MONUMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIVERCHURN_MONUMENT,
    1,
    "131df43b-2a72-4ca3-8aae-c0dca2eca8fc",
    "Anthony Devine",
);

// DFT 382 — Unstoppable Plan (alternate printing)
const UNSTOPPABLE_PLAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNSTOPPABLE_PLAN,
    1,
    "ce40ff0e-04ae-4786-ac0c-54a66ffb48a6",
    "Borja Pindado",
);

// DFT 383 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    1,
    "c0a57479-6409-49e1-8896-2bd3e88d554d",
    "Dominik Mayer",
);

// DFT 384 — Quag Feast (alternate printing)
const QUAG_FEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUAG_FEAST,
    1,
    "5d6498f0-5ebd-441a-a851-461b0460e7d5",
    "Loïc Canavaggia",
);

// DFT 385 — Count on Luck (alternate printing)
const COUNT_ON_LUCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COUNT_ON_LUCK,
    1,
    "5d2abab5-c4f0-41ec-ae6f-e42721ca2cfd",
    "Michal Ivan",
);

// DFT 386 — Full Throttle (alternate printing)
const FULL_THROTTLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FULL_THROTTLE,
    1,
    "e7967282-ae00-4fdd-85c6-bcc8bc9b790b",
    "Benjamin Ee",
);

// DFT 387 — Afterburner Expert (alternate printing)
const AFTERBURNER_EXPERT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AFTERBURNER_EXPERT,
    1,
    "aafe531a-f1f9-46d4-aff4-2b0df789be66",
    "April Prime",
);

// DFT 388 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    1,
    "615e3ebb-c41a-497a-b71b-6549366e5b07",
    "Helge C. Balzer",
);

// DFT 389 — Regal Imperiosaur (alternate printing)
const REGAL_IMPERIOSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REGAL_IMPERIOSAUR,
    1,
    "3a198715-4f82-409d-add0-e0200a88d708",
    "Stephanie Cheung",
);

// DFT 390 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    1,
    "a258dcf4-b481-4ad5-96ef-2a42de8c9c2a",
    "Caio Monteiro",
);

// DFT 391 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    1,
    "2528d058-5ca3-4671-8bf1-c8161872cd36",
    "Ernanda Souza",
);

// DFT 392 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    1,
    "115088f3-9fca-4120-8ca5-c124c3d32975",
    "Ron Spencer",
);

// DFT 393 — Marketback Walker (alternate printing)
const MARKETBACK_WALKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARKETBACK_WALKER,
    1,
    "93211140-5aa7-4700-804d-aa46a8b141d7",
    "Svetlin Velinov",
);

// DFT 394 — Monument to Endurance (alternate printing)
const MONUMENT_TO_ENDURANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MONUMENT_TO_ENDURANCE,
    1,
    "c66cbdac-1d15-40f5-b76f-6f21b917abbc",
    "Victor Sales",
);

// DFT 395 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    1,
    "8ef84601-6ace-4847-abeb-16dcfdba5c89",
    "Bruce Brenneise",
);

// DFT 396 — Muraganda Raceway (alternate printing)
const MURAGANDA_RACEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MURAGANDA_RACEWAY,
    1,
    "55b87e12-bd95-450e-a528-a44c76dbfae5",
    "Brian Valeza",
);

// DFT 397 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    2,
    "d56cb1b9-38b7-42ad-b176-dc265bc8abe0",
    "Mai Minamiura",
);

// DFT 398 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    2,
    "ec93ed6d-a840-477d-a16c-ffd15a52c941",
    "JIN-E YAMAMOTO",
);

// DFT 399 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    2,
    "7fe19f52-ae23-4a23-800d-f3cf186279f2",
    "D-suzuki",
);

// DFT 400 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    2,
    "c5280941-bcda-4ac7-97d1-db02c7a18ef9",
    "BARON UEDA",
);

// DFT 401 — Chandra, Spark Hunter (alternate printing)
const CHANDRA_SPARK_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_SPARK_HUNTER,
    1,
    "a1864b99-6893-4e4d-94de-641837c05da3",
    "jbstyle.",
);

// DFT 402 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    2,
    "455027f1-26a4-464b-a456-271a6ec88669",
    "Raimaru",
);

// DFT 403 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    2,
    "14a81f86-e80f-47f7-81cd-0c4b9330e646",
    "TSCR",
);

// DFT 404 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    2,
    "4f5b04ab-4c61-47f5-88e4-c5c2f93edc11",
    "Tetsu Kurosawa",
);

// DFT 405 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    2,
    "d5a8caa7-714e-4987-8d44-04fbe5a77c6d",
    "SH11NA",
);

// DFT 406 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    2,
    "8eb85966-66a2-4fb7-b89d-05db7f4e2cb8",
    "Tomoyuki Mizufune",
);

// DFT 407 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    3,
    "0e73d4ed-def7-4456-a750-74fb2bf10f9f",
    "Mai Minamiura",
);

// DFT 408 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    3,
    "22fd4f1b-afbd-41a5-a76a-f9788e0cec9d",
    "JIN-E YAMAMOTO",
);

// DFT 409 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    3,
    "8327b4b6-058e-448b-99ca-7d2de52d3b4c",
    "D-suzuki",
);

// DFT 410 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    3,
    "3c15a04e-b0a7-4560-a838-94213dbb2336",
    "BARON UEDA",
);

// DFT 411 — Chandra, Spark Hunter (alternate printing)
const CHANDRA_SPARK_HUNTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_SPARK_HUNTER,
    2,
    "d6a40fa6-4378-413b-bed8-7eb0d61b70ad",
    "jbstyle.",
);

// DFT 412 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    3,
    "1b9fbbfc-8920-46cd-95cd-2372feb4617a",
    "Raimaru",
);

// DFT 413 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    3,
    "b3dd66b9-f292-4b10-b753-2df112f6f714",
    "TSCR",
);

// DFT 414 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    3,
    "89fba04a-20ab-40eb-9edf-89fdd03bfb7e",
    "Tetsu Kurosawa",
);

// DFT 415 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    3,
    "6b243165-79ad-4193-af89-2ad5e34fc415",
    "SH11NA",
);

// DFT 416 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    3,
    "656ba742-b00f-4fce-8418-987226c25a81",
    "Tomoyuki Mizufune",
);

// DFT 417 — Tune Up (alternate printing)
const TUNE_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TUNE_UP,
    1,
    "6b832719-ac31-470f-9d50-0b514ecc6845",
    "Chris Rallis",
);

// DFT 418 — Gastal Raider (alternate printing)
const GASTAL_RAIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_RAIDER,
    1,
    "4a8b12dd-2ce3-48a6-9ea7-7d5f155164c9",
    "Lorenzo Mastroianni",
);

// DFT 419 — Marauding Mako (alternate printing)
const MARAUDING_MAKO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARAUDING_MAKO,
    1,
    "194ebb23-fecd-4aa5-96b7-447c9768794e",
    "Alix Branwyn",
);

// DFT 420 — Skyserpent Seeker (alternate printing)
const SKYSERPENT_SEEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYSERPENT_SEEKER,
    1,
    "b7de8c7e-7270-4a4f-af03-9f088b04c05b",
    "Johan Grenier",
);

// DFT 421 — Voyage Home (alternate printing)
const VOYAGE_HOME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VOYAGE_HOME,
    1,
    "30789c9b-7b4b-43e7-a161-aa6595ea9dee",
    "Hardy Fowler",
);

// DFT 422 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    2,
    "06a85032-0c45-40f7-901a-3fd77ff212a4",
    "Adam Volker",
);

// DFT 423 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    2,
    "23eda600-af1e-4444-a822-830037133c0d",
    "José Parodi",
);

// DFT 424 — Amonkhet Raceway (alternate printing)
const AMONKHET_RACEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMONKHET_RACEWAY,
    1,
    "fb37d984-5a9c-45e3-8213-4af93872d512",
    "Titus Lunter",
);

// DFT 425 — Avishkar Raceway (alternate printing)
const AVISHKAR_RACEWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVISHKAR_RACEWAY,
    1,
    "21e9b236-f0e5-4e9d-a9c9-7d625e8412f2",
    "Titus Lunter",
);

// DFT 426 — Muraganda Raceway (alternate printing)
const MURAGANDA_RACEWAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MURAGANDA_RACEWAY,
    2,
    "9db48a2f-5110-431e-8d72-b70bd2e9d887",
    "Titus Lunter",
);

// DFT 427 — Basri, Tomorrow's Champion (alternate printing)
const BASRI_TOMORROW_S_CHAMPION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BASRI_TOMORROW_S_CHAMPION,
    2,
    "932d4e0c-2653-4353-8e70-b7cf0f6808ed",
    "Kai Carpenter",
);

// DFT 428 — Bulwark Ox (alternate printing)
const BULWARK_OX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BULWARK_OX,
    2,
    "146670b1-0b6d-4d9b-b931-137c508309a9",
    "Brent Hollowell",
);

// DFT 429 — Guardian Sunmare (alternate printing)
const GUARDIAN_SUNMARE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GUARDIAN_SUNMARE,
    2,
    "9a6b0d3e-0d3e-44a7-99e3-4ccba6c832e1",
    "Christina Kraus",
);

// DFT 430 — Perilous Snare (alternate printing)
const PERILOUS_SNARE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PERILOUS_SNARE,
    2,
    "87181867-f63d-4642-875f-001c1ed7912f",
    "Chris Seaman",
);

// DFT 431 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    4,
    "7f338af2-f173-45a0-bbea-02517d41bef2",
    "Ben Wootten",
);

// DFT 432 — Skyseer's Chariot (alternate printing)
const SKYSEER_S_CHARIOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SKYSEER_S_CHARIOT,
    2,
    "3a062cbd-b769-49d3-a9fd-7aeb414f44d6",
    "Carl Critchlow",
);

// DFT 433 — Spectacular Pileup (alternate printing)
const SPECTACULAR_PILEUP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_PILEUP,
    4,
    "9d21406e-0c00-4501-a93e-ccd8172314e8",
    "Zezhou Chen",
);

// DFT 434 — Valor's Flagship (alternate printing)
const VALOR_S_FLAGSHIP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VALOR_S_FLAGSHIP,
    2,
    "1f73b484-1503-4899-8b6a-076e8b087810",
    "Stephan Martiniere",
);

// DFT 435 — Voyager Glidecar (alternate printing)
const VOYAGER_GLIDECAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VOYAGER_GLIDECAR,
    2,
    "559dd83b-f24a-43e7-9195-9fcbe6c0c64b",
    "Eduardo Francisco",
);

// DFT 436 — Mindspring Merfolk (alternate printing)
const MINDSPRING_MERFOLK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MINDSPRING_MERFOLK,
    2,
    "09e15402-8cc6-4ad2-8088-4df92fa20d8e",
    "Andreia Ugrai",
);

// DFT 437 — Mu Yanling, Wind Rider (alternate printing)
const MU_YANLING_WIND_RIDER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MU_YANLING_WIND_RIDER,
    4,
    "40d45912-7b8d-416e-893b-1d103492a5a1",
    "Justyna Dura",
);

// DFT 438 — Possession Engine (alternate printing)
const POSSESSION_ENGINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &POSSESSION_ENGINE,
    2,
    "858dc258-965a-4d1a-aab5-447e0fecbf81",
    "Leroy Steinmann",
);

// DFT 439 — Repurposing Bay (alternate printing)
const REPURPOSING_BAY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REPURPOSING_BAY,
    2,
    "5277eeef-50af-4654-9216-caf0c37c27d7",
    "William Tempest",
);

// DFT 440 — Riverchurn Monument (alternate printing)
const RIVERCHURN_MONUMENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RIVERCHURN_MONUMENT,
    2,
    "06d22866-07ac-4aef-b43e-29a37f3db1c0",
    "Anthony Devine",
);

// DFT 441 — Thopter Fabricator (alternate printing)
const THOPTER_FABRICATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THOPTER_FABRICATOR,
    2,
    "a98ff182-4a84-4d69-83ce-57a7e5b18ebb",
    "Racrufi",
);

// DFT 442 — Unstoppable Plan (alternate printing)
const UNSTOPPABLE_PLAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &UNSTOPPABLE_PLAN,
    2,
    "62a95aeb-b1b0-42cd-ad67-fd13717d3c6c",
    "Borja Pindado",
);

// DFT 443 — Vnwxt, Verbose Host (alternate printing)
const VNWXT_VERBOSE_HOST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VNWXT_VERBOSE_HOST,
    2,
    "7642f9db-3eee-471d-ada6-60e990ec3fab",
    "Izzy",
);

// DFT 444 — Waxen Shapethief (alternate printing)
const WAXEN_SHAPETHIEF_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WAXEN_SHAPETHIEF,
    2,
    "cb17c37c-907d-4416-b8c1-d6a6f4e312f3",
    "Helge C. Balzer",
);

// DFT 445 — Bloodghast (alternate printing)
const BLOODGHAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::BLOODGHAST,
    2,
    "77534750-fa92-42b4-8f32-be437eca64ad",
    "Francisco Badilla",
);

// DFT 446 — Cryptcaller Chariot (alternate printing)
const CRYPTCALLER_CHARIOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CRYPTCALLER_CHARIOT,
    2,
    "de65f4cd-3ba4-4e1a-8810-e43cf26b38a4",
    "Aaron Miller",
);

// DFT 447 — Cursecloth Wrappings (alternate printing)
const CURSECLOTH_WRAPPINGS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &CURSECLOTH_WRAPPINGS,
    4,
    "2b283c18-abe7-4b73-8532-dd76a85fabd6",
    "Dominik Mayer",
);

// DFT 448 — Demonic Junker (alternate printing)
const DEMONIC_JUNKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_JUNKER,
    2,
    "ad0eb3de-796d-4da3-aab3-08c2b5b6943d",
    "Stephan Martiniere",
);

// DFT 449 — Gas Guzzler (alternate printing)
const GAS_GUZZLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GAS_GUZZLER,
    2,
    "e7e8e6d0-d155-49da-ac5b-47fb408202e6",
    "Yohann Schepacz",
);

// DFT 450 — Gonti, Night Minister (alternate printing)
const GONTI_NIGHT_MINISTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GONTI_NIGHT_MINISTER,
    2,
    "21ed5dd3-e67b-4425-bb98-6b07db1e9d1f",
    "Scott M. Fischer",
);

// DFT 451 — The Last Ride (alternate printing)
const THE_LAST_RIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RIDE,
    2,
    "a901a616-b8bb-4e10-9886-ee9c6fdc6435",
    "Michele Giorgi",
);

// DFT 452 — Quag Feast (alternate printing)
const QUAG_FEAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &QUAG_FEAST,
    2,
    "8684d080-070c-4419-9cc8-777500573f55",
    "Loïc Canavaggia",
);

// DFT 453 — The Speed Demon (alternate printing)
const THE_SPEED_DEMON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_SPEED_DEMON,
    2,
    "da8a7c1f-9d05-4a4c-bdf2-cd9ee7ed18a5",
    "Helge C. Balzer",
);

// DFT 454 — Boommobile (alternate printing)
const BOOMMOBILE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BOOMMOBILE,
    2,
    "3b1bff20-0b7a-49f9-b967-773f3043be13",
    "Alexandr Leskinen",
);

// DFT 455 — Burnout Bashtronaut (alternate printing)
const BURNOUT_BASHTRONAUT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BURNOUT_BASHTRONAUT,
    2,
    "60c9b613-1494-4df0-9a38-cc0a69fd3945",
    "Andrea Piparo",
);

// DFT 456 — Chandra, Spark Hunter (alternate printing)
const CHANDRA_SPARK_HUNTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_SPARK_HUNTER,
    3,
    "625b175a-b63f-46d1-a7e1-95dfc7c03e3f",
    "Devin Elle Kurtz",
);

// DFT 457 — Count on Luck (alternate printing)
const COUNT_ON_LUCK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COUNT_ON_LUCK,
    2,
    "eee38fa7-d5c8-4f76-92f8-4c5cb5bae5e7",
    "Michal Ivan",
);

// DFT 458 — Daretti, Rocketeer Engineer (alternate printing)
const DARETTI_ROCKETEER_ENGINEER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DARETTI_ROCKETEER_ENGINEER,
    2,
    "0a17ef59-177c-4c39-8cbb-efa8ab7ac24a",
    "Borja Pindado",
);

// DFT 459 — Draconautics Engineer (alternate printing)
const DRACONAUTICS_ENGINEER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DRACONAUTICS_ENGINEER,
    2,
    "7758b89f-9629-45f9-ad37-6435dba37997",
    "Artur Nakhodkin",
);

// DFT 460 — Full Throttle (alternate printing)
const FULL_THROTTLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FULL_THROTTLE,
    2,
    "76bf1b29-5a8a-4ac1-aa17-58fcbda969a2",
    "Benjamin Ee",
);

// DFT 461 — Gastal Thrillroller (alternate printing)
const GASTAL_THRILLROLLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_THRILLROLLER,
    2,
    "5a80bb9d-da0d-4f98-8946-43513db4aef6",
    "Caio Monteiro",
);

// DFT 462 — Hazoret, Godseeker (alternate printing)
const HAZORET_GODSEEKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HAZORET_GODSEEKER,
    2,
    "05bbbcb2-822f-4729-872b-20638e7ad155",
    "Chris Rallis",
);

// DFT 463 — Howlsquad Heavy (alternate printing)
const HOWLSQUAD_HEAVY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOWLSQUAD_HEAVY,
    2,
    "a28b514a-29a1-4d85-8eaa-279f3d7e09c1",
    "Leonardo Santanna",
);

// DFT 464 — Afterburner Expert (alternate printing)
const AFTERBURNER_EXPERT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AFTERBURNER_EXPERT,
    2,
    "3985ee93-3aeb-4f0d-a6af-686efbcadda4",
    "April Prime",
);

// DFT 465 — Agonasaur Rex (alternate printing)
const AGONASAUR_REX_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AGONASAUR_REX,
    2,
    "54fc56fc-8c82-4d74-8119-5d260c691b36",
    "Lucas Graciano",
);

// DFT 466 — District Mascot (alternate printing)
const DISTRICT_MASCOT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DISTRICT_MASCOT,
    2,
    "3e7926c8-1a46-4edb-950b-2c8bac91b476",
    "Liiga Smilshkalne",
);

// DFT 467 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    3,
    "8bb97f9f-e7be-4794-a3a0-a0d73445c8fe",
    "Raph Lomotan",
);

// DFT 468 — March of the World Ooze (alternate printing)
const MARCH_OF_THE_WORLD_OOZE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MARCH_OF_THE_WORLD_OOZE,
    4,
    "f89e403b-9653-493f-a3ac-e01aa6f006e2",
    "Helge C. Balzer",
);

// DFT 469 — Oviya, Automech Artisan (alternate printing)
const OVIYA_AUTOMECH_ARTISAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVIYA_AUTOMECH_ARTISAN,
    2,
    "6d6eae8b-73de-4d79-bbe5-2921db1987b2",
    "Julia Metzger",
);

// DFT 470 — Regal Imperiosaur (alternate printing)
const REGAL_IMPERIOSAUR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REGAL_IMPERIOSAUR,
    2,
    "d72328b4-19d1-4855-aa0d-331321eb9945",
    "Stephanie Cheung",
);

// DFT 471 — Thunderous Velocipede (alternate printing)
const THUNDEROUS_VELOCIPEDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_VELOCIPEDE,
    2,
    "f63dc8cd-34a7-43ea-a422-3ced3c11f5be",
    "Adrián Rodríguez Pérez",
);

// DFT 472 — Webstrike Elite (alternate printing)
const WEBSTRIKE_ELITE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WEBSTRIKE_ELITE,
    2,
    "10c14636-3ff0-4fbf-9df1-1b6d3d52e29e",
    "Andrew Mar",
);

// DFT 473 — Aatchik, Emerald Radian (alternate printing)
const AATCHIK_EMERALD_RADIAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AATCHIK_EMERALD_RADIAN,
    2,
    "71074080-d83d-4b4a-b385-f918186530b4",
    "Loïc Canavaggia",
);

// DFT 474 — Brightglass Gearhulk (alternate printing)
const BRIGHTGLASS_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BRIGHTGLASS_GEARHULK,
    2,
    "c0e29a88-34b6-4c3d-9956-b7eb8d5ef591",
    "José Parodi",
);

// DFT 475 — Captain Howler, Sea Scourge (alternate printing)
const CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_HOWLER_SEA_SCOURGE,
    2,
    "db48629b-dc1b-4609-aea2-e4833f66d894",
    "Mirko Failoni",
);

// DFT 476 — Caradora, Heart of Alacria (alternate printing)
const CARADORA_HEART_OF_ALACRIA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CARADORA_HEART_OF_ALACRIA,
    2,
    "61bc2ca6-427f-4f07-88b0-cf57a1fe74ab",
    "Mirko Failoni",
);

// DFT 477 — Coalstoke Gearhulk (alternate printing)
const COALSTOKE_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COALSTOKE_GEARHULK,
    2,
    "07cf7eba-b917-4f14-8bf4-420bb919b7e5",
    "Nino Vecia",
);

// DFT 478 — Debris Beetle (alternate printing)
const DEBRIS_BEETLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEBRIS_BEETLE,
    2,
    "92766c53-703d-41a2-ad13-8c366dcf25ec",
    "Julie Dillon",
);

// DFT 479 — Explosive Getaway (alternate printing)
const EXPLOSIVE_GETAWAY_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &EXPLOSIVE_GETAWAY,
    4,
    "a556367e-7db7-47ad-9f10-37d277d80e77",
    "Caio Monteiro",
);

// DFT 480 — Far Fortune, End Boss (alternate printing)
const FAR_FORTUNE_END_BOSS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FAR_FORTUNE_END_BOSS,
    2,
    "3655e0a5-af3e-4720-b4cf-c93ae9c10cac",
    "Javier Charro",
);

// DFT 481 — Fearless Swashbuckler (alternate printing)
const FEARLESS_SWASHBUCKLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FEARLESS_SWASHBUCKLER,
    2,
    "e3bb615c-e142-4a6e-8d7a-f0a7a3b30568",
    "Konstantin Porubov",
);

// DFT 482 — Ketramose, the New Dawn (alternate printing)
const KETRAMOSE_THE_NEW_DAWN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KETRAMOSE_THE_NEW_DAWN,
    2,
    "9a86678f-e4b4-4c40-9a99-83029fd97730",
    "Maaz Ali Khan",
);

// DFT 483 — Kolodin, Triumph Caster (alternate printing)
const KOLODIN_TRIUMPH_CASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KOLODIN_TRIUMPH_CASTER,
    2,
    "46164400-0557-4c99-83d5-194dc814b46a",
    "Michal Ivan",
);

// DFT 484 — Loot, the Pathfinder (alternate printing)
const LOOT_THE_PATHFINDER_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LOOT_THE_PATHFINDER,
    4,
    "542fe4ca-d3d8-469c-b5cb-4d0824ba514b",
    "Ernanda Souza",
);

// DFT 485 — Mendicant Core, Guidelight (alternate printing)
const MENDICANT_CORE_GUIDELIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MENDICANT_CORE_GUIDELIGHT,
    2,
    "ec057f26-8d14-463b-84ce-cce82b95b08e",
    "Zezhou Chen",
);

// DFT 486 — Mimeoplasm, Revered One (alternate printing)
const MIMEOPLASM_REVERED_ONE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &MIMEOPLASM_REVERED_ONE,
    4,
    "2976e7e4-c53f-47d5-9890-cd30acad4f68",
    "Ron Spencer",
);

// DFT 487 — Oildeep Gearhulk (alternate printing)
const OILDEEP_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OILDEEP_GEARHULK,
    2,
    "a749306b-26ce-4c71-8586-a1ef9788c286",
    "Artur Nakhodkin",
);

// DFT 488 — Pyrewood Gearhulk (alternate printing)
const PYREWOOD_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PYREWOOD_GEARHULK,
    2,
    "794b7a29-eb42-4c52-a4d9-3976611efdd7",
    "Martin de Diego Sádaba",
);

// DFT 489 — Redshift, Rocketeer Chief (alternate printing)
const REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &REDSHIFT_ROCKETEER_CHIEF,
    2,
    "63ea12cb-c927-44f4-b304-f24919b1104e",
    "Wayne Reynolds",
);

// DFT 490 — Riptide Gearhulk (alternate printing)
const RIPTIDE_GEARHULK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RIPTIDE_GEARHULK,
    2,
    "81d5d0a0-b0ae-4a5c-ace6-77d87fcc8563",
    "Artur Nakhodkin",
);

// DFT 491 — Sab-Sunen, Luxa Embodied (alternate printing)
const SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SAB_SUNEN_LUXA_EMBODIED,
    2,
    "36581aa3-8970-40cc-bd1c-e78787ebe441",
    "Valera Lutfullina",
);

// DFT 492 — Samut, the Driving Force (alternate printing)
const SAMUT_THE_DRIVING_FORCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SAMUT_THE_DRIVING_FORCE,
    2,
    "c1bd0f7d-1686-42ae-9bea-66b96309f322",
    "Chris Rallis",
);

// DFT 493 — Sita Varma, Masked Racer (alternate printing)
const SITA_VARMA_MASKED_RACER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SITA_VARMA_MASKED_RACER,
    2,
    "321d595f-7efe-4004-9bbc-390b5a6eb734",
    "Kai Carpenter",
);

// DFT 494 — Winter, Cursed Rider (alternate printing)
const WINTER_CURSED_RIDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WINTER_CURSED_RIDER,
    2,
    "5f93e48d-4681-4354-aa85-493a48287233",
    "Daren Bader",
);

// DFT 495 — Zahur, Glory's Past (alternate printing)
const ZAHUR_GLORY_S_PAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZAHUR_GLORY_S_PAST,
    2,
    "f10ee92a-85cc-48cb-b0b5-8e5185781fb2",
    "Leroy Steinmann",
);

// DFT 496 — The Aetherspark (alternate printing)
const THE_AETHERSPARK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_AETHERSPARK,
    2,
    "c0ea3cfc-64f1-4289-ac6c-82d55bab65de",
    "Donato Giancola",
);

// DFT 497 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    3,
    "8c14d4cc-79c3-4934-b7a8-25e1973ffea3",
    "Mirko Failoni",
);

// DFT 498 — Marketback Walker (alternate printing)
const MARKETBACK_WALKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MARKETBACK_WALKER,
    2,
    "20419718-d922-4f3c-aeb2-30ffb2ff7cbb",
    "Svetlin Velinov",
);

// DFT 499 — Monument to Endurance (alternate printing)
const MONUMENT_TO_ENDURANCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MONUMENT_TO_ENDURANCE,
    2,
    "c75f3b27-7c08-4e92-ae9a-6208aa9aec0c",
    "Victor Sales",
);

// DFT 500 — Radiant Lotus (alternate printing)
const RADIANT_LOTUS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &RADIANT_LOTUS,
    4,
    "333039ef-b328-4082-85ae-162caed5612e",
    "Bruce Brenneise",
);

// DFT 501 — Bleachbone Verge (alternate printing)
const BLEACHBONE_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLEACHBONE_VERGE,
    2,
    "712aab40-5452-4d62-8ad3-fec96d03db98",
    "Mark Tedin",
);

// DFT 502 — Muraganda Raceway (alternate printing)
const MURAGANDA_RACEWAY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MURAGANDA_RACEWAY,
    3,
    "b6457d47-8e68-42db-85c9-e0f981c53aa4",
    "Brian Valeza",
);

// DFT 503 — Riverpyre Verge (alternate printing)
const RIVERPYRE_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RIVERPYRE_VERGE,
    2,
    "c21462ee-76d7-4910-9a0c-b470f96b8b51",
    "Titus Lunter",
);

// DFT 504 — Sunbillow Verge (alternate printing)
const SUNBILLOW_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SUNBILLOW_VERGE,
    2,
    "cff03662-3879-4ca8-b73c-34918194203b",
    "Pete Venters",
);

// DFT 505 — Wastewood Verge (alternate printing)
const WASTEWOOD_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WASTEWOOD_VERGE,
    2,
    "ba0c9044-13ae-465e-a9d1-7dc63827540d",
    "Bartek Fedyczak",
);

// DFT 506 — Willowrush Verge (alternate printing)
const WILLOWRUSH_VERGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &WILLOWRUSH_VERGE,
    2,
    "8e928cc4-be33-4c99-b0d0-c808c225e3ab",
    "Aaron Miller",
);

// DFT 507 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "4e196d67-1923-459d-98cf-646337914d3b",
    "Adam Paquette",
);

// DFT 508 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    4,
    "9b33841e-7006-41d4-98b0-313ab151c9ec",
    "Maxime Minard",
);

// DFT 509 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    4,
    "ba042d60-b1e8-44ad-9103-f40588a0b29c",
    "Jonas De Ro",
);

// DFT 510 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    4,
    "c7533cbb-ab73-4a0c-9ea0-baa97c4665b8",
    "Chris Ostrowski",
);

// DFT 511 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    4,
    "fd1b6c5c-4585-47c1-82eb-a324189f3ebd",
    "Andreas Rocha",
);

// DFT 512 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "5dd76e71-df9b-4fd2-b534-d3f724a56a91",
    "Calder Moore",
);

// DFT 513 — Island (alternate printing)
const ISLAND_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    5,
    "d13bb767-1123-48d2-960f-b90b6db3f6ed",
    "Calder Moore",
);

// DFT 514 — Swamp (alternate printing)
const SWAMP_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    5,
    "f4d198b3-2ff5-484a-905f-c272de7c139d",
    "Calder Moore",
);

// DFT 515 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    5,
    "5e4952a8-4e1f-4a74-8a06-c9d633b25dcd",
    "Calder Moore",
);

// DFT 516 — Forest (alternate printing)
const FOREST_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    5,
    "2a7aba60-d300-4c33-9e1e-846d6167dc50",
    "Calder Moore",
);

// DFT 517 — Salvation Engine (alternate printing)
const SALVATION_ENGINE_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_ENGINE,
    5,
    "f2c7805b-c952-4847-b321-e0567923c80e",
    "Adam Volker",
);

// DFT 518 — Skyseer's Chariot (alternate printing)
const SKYSEER_S_CHARIOT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SKYSEER_S_CHARIOT,
    3,
    "d18d5057-39f7-42aa-9dd5-ecc52f861366",
    "Douglas P. Lobo",
);

// DFT 519 — Valor's Flagship (alternate printing)
const VALOR_S_FLAGSHIP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VALOR_S_FLAGSHIP,
    3,
    "58d4d35a-1f7e-4c5e-a999-09ad42f8b4e2",
    "William Tempest",
);

// DFT 520 — Voyager Glidecar (alternate printing)
const VOYAGER_GLIDECAR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VOYAGER_GLIDECAR,
    3,
    "b3666b4e-d6a4-41be-a529-4a577e75c86d",
    "Andrew Griffith",
);

// DFT 521 — Possession Engine (alternate printing)
const POSSESSION_ENGINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &POSSESSION_ENGINE,
    3,
    "96f0c1eb-603e-4e3e-b40b-65662c22d72d",
    "Douglas P. Lobo",
);

// DFT 522 — Thopter Fabricator (alternate printing)
const THOPTER_FABRICATOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THOPTER_FABRICATOR,
    3,
    "66f1f362-fa4a-49e6-baa5-6797a9408cad",
    "William Tempest",
);

// DFT 523 — Cryptcaller Chariot (alternate printing)
const CRYPTCALLER_CHARIOT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CRYPTCALLER_CHARIOT,
    3,
    "fa56ae43-4a6b-4af6-9707-679068bd0934",
    "Oliver Barrett",
);

// DFT 524 — Demonic Junker (alternate printing)
const DEMONIC_JUNKER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_JUNKER,
    3,
    "a9747c8c-024f-417f-86ad-ee7c6f756aa2",
    "Deathburger",
);

// DFT 525 — The Last Ride (alternate printing)
const THE_LAST_RIDE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_LAST_RIDE,
    3,
    "f85dac5a-c6fa-4a86-9fe3-7a17db06f07c",
    "Deathburger",
);

// DFT 526 — Boommobile (alternate printing)
const BOOMMOBILE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BOOMMOBILE,
    3,
    "d62d070e-52b7-4d7f-bd98-13622d859a7b",
    "Ian Jepson",
);

// DFT 527 — Gastal Thrillroller (alternate printing)
const GASTAL_THRILLROLLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GASTAL_THRILLROLLER,
    3,
    "a02b39bc-8eda-4c5a-8705-5416cdb24f0d",
    "Dan Mumford",
);

// DFT 528 — Lumbering Worldwagon (alternate printing)
const LUMBERING_WORLDWAGON_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LUMBERING_WORLDWAGON,
    4,
    "83725fcd-d5ec-4b99-b3f6-e7f5325c766e",
    "Francisco Badilla",
);

// DFT 529 — Thunderous Velocipede (alternate printing)
const THUNDEROUS_VELOCIPEDE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THUNDEROUS_VELOCIPEDE,
    3,
    "ac4ff9fc-5c34-4a07-a1f3-35c21a3323b0",
    "Eduardo Francisco",
);

// DFT 530 — Debris Beetle (alternate printing)
const DEBRIS_BEETLE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEBRIS_BEETLE,
    3,
    "4efa1ade-00f9-4736-8397-11d4faf65b7b",
    "Michal Ivan",
);

// DFT 531 — Lifecraft Engine (alternate printing)
const LIFECRAFT_ENGINE_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &LIFECRAFT_ENGINE,
    4,
    "91496c82-44bc-4bb7-9e2a-5f4a3a199f4c",
    "Eduardo Francisco",
);

// DFT 532 — Bulwark Ox (alternate printing)
const BULWARK_OX_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BULWARK_OX,
    3,
    "4d4e8762-0aed-4009-86f0-7935a1d12b52",
    "Jon Vermilyea",
);

// DFT 533 — Guardian Sunmare (alternate printing)
const GUARDIAN_SUNMARE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GUARDIAN_SUNMARE,
    3,
    "72eb0c5b-ef3f-42db-9f40-0463cacd548d",
    "Wojtek Łebski",
);

// DFT 534 — Mindspring Merfolk (alternate printing)
const MINDSPRING_MERFOLK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MINDSPRING_MERFOLK,
    3,
    "f1191c3a-a6eb-4765-bb79-2f0d920899f9",
    "CatDirty",
);

// DFT 535 — Waxen Shapethief (alternate printing)
const WAXEN_SHAPETHIEF_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WAXEN_SHAPETHIEF,
    3,
    "ec7ee49c-1e5d-4b3a-8e0a-59a0d258e804",
    "CatDirty",
);

// DFT 536 — Bloodghast (alternate printing)
const BLOODGHAST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::BLOODGHAST,
    3,
    "3489de46-e90c-483e-ac2b-547497f64e82",
    "Wojtek Łebski",
);

// DFT 537 — Gas Guzzler (alternate printing)
const GAS_GUZZLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &GAS_GUZZLER,
    3,
    "48b75e83-2ed5-48c9-881a-da3d77bdef99",
    "Sam McKenzie",
);

// DFT 538 — The Speed Demon (alternate printing)
const THE_SPEED_DEMON_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_SPEED_DEMON,
    3,
    "36bd51bd-e451-4f1d-af19-bb777ec1a85c",
    "CatDirty",
);

// DFT 539 — Burnout Bashtronaut (alternate printing)
const BURNOUT_BASHTRONAUT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BURNOUT_BASHTRONAUT,
    3,
    "b2151639-5621-4ffa-9374-3a1c48a635d9",
    "Jon Vermilyea",
);

// DFT 540 — Draconautics Engineer (alternate printing)
const DRACONAUTICS_ENGINEER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DRACONAUTICS_ENGINEER,
    3,
    "9419e00b-062b-4bb2-9203-f2f86b502371",
    "Ryan Roadkill",
);

// DFT 541 — Howlsquad Heavy (alternate printing)
const HOWLSQUAD_HEAVY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HOWLSQUAD_HEAVY,
    3,
    "bf709cb6-8953-40ce-9fcc-396b6c0ab37a",
    "Ryan Roadkill",
);

// DFT 542 — Agonasaur Rex (alternate printing)
const AGONASAUR_REX_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &AGONASAUR_REX,
    3,
    "8d815a94-cdf0-4a19-ad06-4292efbb7105",
    "Jon Vermilyea",
);

// DFT 543 — District Mascot (alternate printing)
const DISTRICT_MASCOT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DISTRICT_MASCOT,
    3,
    "3d8ad303-505d-4e2a-9380-80ab9f91a0a3",
    "Ryan Roadkill",
);

// DFT 544 — Webstrike Elite (alternate printing)
const WEBSTRIKE_ELITE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &WEBSTRIKE_ELITE,
    3,
    "ecf3bcd2-fdc6-4490-858d-3334437a3148",
    "Sam McKenzie",
);

// DFT 545 — Fearless Swashbuckler (alternate printing)
const FEARLESS_SWASHBUCKLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &FEARLESS_SWASHBUCKLER,
    3,
    "47c8f0f0-f2d0-44fa-8163-41a8e35952fd",
    "CatDirty",
);

// DFT 546 — Hazoret, Godseeker (alternate printing)
const HAZORET_GODSEEKER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HAZORET_GODSEEKER,
    3,
    "c5e0ead2-933a-41ad-b1c3-0aa79cc41e93",
    "Massiveface",
);

// DFT 547 — Brightglass Gearhulk (alternate printing)
const BRIGHTGLASS_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BRIGHTGLASS_GEARHULK,
    3,
    "0b787e28-a6bf-489c-affb-4043845c9769",
    "Jorge Gutierrez Garcia",
);

// DFT 548 — Coalstoke Gearhulk (alternate printing)
const COALSTOKE_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &COALSTOKE_GEARHULK,
    3,
    "4363ec0d-a473-4afc-9ae5-4feed762a630",
    "Ivan Shavrin",
);

// DFT 549 — Ketramose, the New Dawn (alternate printing)
const KETRAMOSE_THE_NEW_DAWN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KETRAMOSE_THE_NEW_DAWN,
    3,
    "1688041b-a654-493e-bfac-247fdca68b22",
    "Florian Bertmer",
);

// DFT 550 — Oildeep Gearhulk (alternate printing)
const OILDEEP_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OILDEEP_GEARHULK,
    3,
    "41dd7467-4e77-4fad-b8e9-b258fa6254f3",
    "Chun Lo",
);

// DFT 551 — Pyrewood Gearhulk (alternate printing)
const PYREWOOD_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &PYREWOOD_GEARHULK,
    3,
    "ec50050d-1da8-4f37-a975-f81d5684a619",
    "William Tempest",
);

// DFT 552 — Riptide Gearhulk (alternate printing)
const RIPTIDE_GEARHULK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RIPTIDE_GEARHULK,
    3,
    "6127cd9e-5785-4ba5-89d0-5eeb0d8aaa0e",
    "Kudaman",
);

// DFT 553 — Sab-Sunen, Luxa Embodied (alternate printing)
const SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SAB_SUNEN_LUXA_EMBODIED,
    3,
    "fb5f9907-e187-4c01-964c-24b3d27948b5",
    "Benjamin Ee",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AIR_RESPONSE_UNIT,
    &ALACRIAN_ARMORY,
    &BASRI_TOMORROW_S_CHAMPION,
    &BRIGHTFIELD_GLIDER,
    &BRIGHTFIELD_MUSTANG,
    &BROADCAST_RAMBLER,
    &BULWARK_OX,
    &CANYON_VAULTER,
    &CLOUDSPIRE_CAPTAIN,
    &COLLISION_COURSE,
    &DARING_MECHANIC,
    &DETENTION_CHARIOT,
    &GALLANT_STRIKE,
    &GLORYHEATH_LYNX,
    &GUARDIAN_SUNMARE,
    &GUIDELIGHT_SYNERGIST,
    &INTERFACE_ACE,
    &LEONIN_SURVEYOR,
    &LIGHTSHIELD_PARRY,
    &LIGHTWHEEL_ENHANCEMENTS,
    &LOTUSGUARD_DISCIPLE,
    &NESTING_BOT,
    &PERILOUS_SNARE,
    &PRIDE_OF_THE_ROAD,
    &RIDE_S_END,
    &ROADSIDE_ASSISTANCE,
    &SALVATION_ENGINE,
    &SKYSEER_S_CHARIOT,
    &SPECTACULAR_PILEUP,
    &SPOTCYCLE_SCOUTER,
    &SUNDIAL_DAWN_TYRANT,
    &SWIFTWING_ASSAILANT,
    &TUNE_UP,
    &UNSWERVING_SLOTH,
    &VALOR_S_FLAGSHIP,
    &VOYAGER_GLIDECAR,
    &VOYAGER_QUICKWELDER,
    &AETHER_SYPHON,
    &BOUNCE_OFF,
    &CAELORNA_CORAL_TYRANT,
    &DIVERSION_UNIT,
    &FLOOD_THE_ENGINE,
    &GLITCH_GHOST_SURVEYOR,
    &GUIDELIGHT_OPTIMIZER,
    &HOWLER_S_HEAVY,
    &HULLDRIFTER,
    &KEEN_BUCCANEER,
    &MEMORY_GUARDIAN,
    &MIDNIGHT_MANGLER,
    &MINDSPRING_MERFOLK,
    &MU_YANLING_WIND_RIDER,
    &NIMBLE_THOPTERIST,
    &POSSESSION_ENGINE,
    &RANGERS_REFUELER,
    &REPURPOSING_BAY,
    &RIVERCHURN_MONUMENT,
    &ROADSIDE_BLOWOUT,
    &SABOTAGE_STRATEGIST,
    &SCROUNGING_SKYRAY,
    &SKYSTREAK_ENGINEER,
    &SLICK_IMITATOR,
    &SPECTRAL_INTERFERENCE,
    &SPIKESHELL_HARRIER,
    &STALL_OUT,
    &STOCK_UP,
    &THOPTER_FABRICATOR,
    &TRADE_THE_HELM,
    &TRANSIT_MAGE,
    &TRIP_UP,
    &UNSTOPPABLE_PLAN,
    &VNWXT_VERBOSE_HOST,
    &WAXEN_SHAPETHIEF,
    &ANCIENT_VENDETTA,
    &BACK_ON_TRACK,
    &CARRION_CRUISER,
    &CHITIN_GRAVESTALKER,
    &CRYPTCALLER_CHARIOT,
    &CURSECLOTH_WRAPPINGS,
    &DEATHLESS_PILOT,
    &DEMONIC_JUNKER,
    &ENGINE_RAT,
    &GAS_GUZZLER,
    &GASTAL_RAIDER,
    &GONTI_NIGHT_MINISTER,
    &GRIM_BAUBLE,
    &GRIM_JAVELINEER,
    &HELLISH_SIDESWIPE,
    &HOUR_OF_VICTORY,
    &INTIMIDATION_TACTICS,
    &KALAKSCION_HUNGER_TYRANT,
    &THE_LAST_RIDE,
    &LOCUST_SPRAY,
    &MAXIMUM_OVERDRIVE,
    &MOMENTUM_BREAKER,
    &MUTANT_SURVEYOR,
    &PACTDOLL_TERROR,
    &QUAG_FEAST,
    &RIPCLAW_WRANGLER,
    &RISEN_NECROREGENT,
    &RISKY_SHORTCUT,
    &SHEFET_ARCHFIEND,
    &THE_SPEED_DEMON,
    &SPIN_OUT,
    &STREAKING_OILGORGER,
    &SYPHON_FUEL,
    &WICKERFOLK_INDOMITABLE,
    &WRECKAGE_WICKERFOLK,
    &WRETCHED_DOLL,
    &ADRENALINE_JOCKEY,
    &BOOMMOBILE,
    &BURNER_ROCKET,
    &BURNOUT_BASHTRONAUT,
    &CHANDRA_SPARK_HUNTER,
    &CLAMOROUS_IRONCLAD,
    &COUNT_ON_LUCK,
    &CRASH_AND_BURN,
    &DARETTI_ROCKETEER_ENGINEER,
    &DRACONAUTICS_ENGINEER,
    &DRACOSAUR_AUXILIARY,
    &DYNAMITE_DIVER,
    &ENDRIDER_CATALYZER,
    &ENDRIDER_SPIKESPITTER,
    &FUEL_THE_FLAMES,
    &FULL_THROTTLE,
    &GASTAL_BLOCKBUSTER,
    &GASTAL_THRILLROLLER,
    &GILDED_GHODA,
    &GOBLIN_SURVEYOR,
    &GREASEWRENCH_GOBLIN,
    &HAZORET_GODSEEKER,
    &HOWLSQUAD_HEAVY,
    &KICKOFF_CELEBRATIONS,
    &MAGMAKIN_ARTILLERIST,
    &MARAUDING_MAKO,
    &OUTPACE_OBLIVION,
    &PACESETTER_PARAGON,
    &PEDAL_TO_THE_METAL,
    &PROWCATCHER_SPECIALIST,
    &PUSH_THE_LIMIT,
    &RECKLESS_VELOCITAUR,
    &ROAD_RAGE,
    &SKYCRASH,
    &SPIRE_MECHCYCLE,
    &THUNDERHEAD_GUNNER,
    &TYROX_SAURID_TYRANT,
    &AFTERBURNER_EXPERT,
    &AGONASAUR_REX,
    &ALACRIAN_JAGUAR,
    &AUTARCH_MAMMOTH,
    &BEASTRIDER_VANGUARD,
    &BESTOW_GREATNESS,
    &DEFEND_THE_RIDER,
    &DISTRICT_MASCOT,
    &DREDGER_S_INSIGHT,
    &EARTHRUMBLER,
    &ELVISH_REFUELER,
    &FANG_GUARDIAN,
    &FANG_DRUID_SUMMONER,
    &GREENBELT_GUARDIAN,
    &HAZARD_OF_THE_DUNES,
    &JIBBIRIK_OMNIVORE,
    &LOXODON_SURVEYOR,
    &LUMBERING_WORLDWAGON,
    &MARCH_OF_THE_WORLD_OOZE,
    &MIGRATING_KETRADON,
    &MOLT_TENDER,
    &OOZE_PATROL,
    &OVIYA_AUTOMECH_ARTISAN,
    &PLOW_THROUGH,
    &POINT_THE_WAY,
    &POTHOLE_MOLE,
    &REGAL_IMPERIOSAUR,
    &RISE_FROM_THE_WRECK,
    &RUN_OVER,
    &SILKEN_STRENGTH,
    &STAMPEDING_SCURRYFOOT,
    &TERRIAN_WORLD_TYRANT,
    &THUNDEROUS_VELOCIPEDE,
    &VELOHEART_BIKE,
    &VENOMSAC_LAGAC,
    &WEBSTRIKE_ELITE,
    &AATCHIK_EMERALD_RADIAN,
    &APOCALYPSE_RUNNER,
    &BOOM_SCHOLAR,
    &BOOSTED_SLOOP,
    &BRIGHTGLASS_GEARHULK,
    &BROADSIDE_BARRAGE,
    &BROODHEART_ENGINE,
    &CAPTAIN_HOWLER_SEA_SCOURGE,
    &CARADORA_HEART_OF_ALACRIA,
    &CLOUDSPIRE_COORDINATOR,
    &CLOUDSPIRE_SKYCYCLE,
    &COALSTOKE_GEARHULK,
    &DEBRIS_BEETLE,
    &DUNE_DRIFTER,
    &EMBALMED_ASCENDANT,
    &EXPLOSIVE_GETAWAY,
    &FAR_FORTUNE_END_BOSS,
    &FEARLESS_SWASHBUCKLER,
    &GASTAL_THRILLSEEKER,
    &GUIDELIGHT_PATHMAKER,
    &HAUNT_THE_NETWORK,
    &HAUNTED_HELLRIDE,
    &KETRAMOSE_THE_NEW_DAWN,
    &KOLODIN_TRIUMPH_CASTER,
    &LAGORIN_SOUL_OF_ALACRIA,
    &LOOT_THE_PATHFINDER,
    &MENDICANT_CORE_GUIDELIGHT,
    &MIMEOPLASM_REVERED_ONE,
    &OILDEEP_GEARHULK,
    &PYREWOOD_GEARHULK,
    &RANGERS_AETHERHIVE,
    &REDSHIFT_ROCKETEER_CHIEF,
    &RIPTIDE_GEARHULK,
    &ROCKETEER_BOOSTBUGGY,
    &SAB_SUNEN_LUXA_EMBODIED,
    &SAMUT_THE_DRIVING_FORCE,
    &SITA_VARMA_MASKED_RACER,
    &SKYSERPENT_SEEKER,
    &THUNDERING_BROODWAGON,
    &VETERAN_BEASTRIDER,
    &VOYAGE_HOME,
    &WINTER_CURSED_RIDER,
    &ZAHUR_GLORY_S_PAST,
    &AETHERJACKET,
    &THE_AETHERSPARK,
    &CAMERA_LAUNCHER,
    &GUIDELIGHT_MATRIX,
    &LIFECRAFT_ENGINE,
    &MARKETBACK_WALKER,
    &MARSHALS_PATHCRUISER,
    &MONUMENT_TO_ENDURANCE,
    &PIT_AUTOMATON,
    &RACERS_SCOREBOARD,
    &RADIANT_LOTUS,
    &ROVER_BLADES,
    &SCRAP_COMPACTOR,
    &SKYBOX_FERRY,
    &STARTING_COLUMN,
    &TICKET_TORTOISE,
    &WALKING_SARCOPHAGUS,
    &WRECK_REMOVER,
    &AMONKHET_RACEWAY,
    &AVISHKAR_RACEWAY,
    &BLEACHBONE_VERGE,
    &COUNTRY_ROADS,
    &FOUL_ROADS,
    &MURAGANDA_RACEWAY,
    &NIGHT_MARKET,
    &REEF_ROADS,
    &RIVERPYRE_VERGE,
    &ROCKY_ROADS,
    &SUNBILLOW_VERGE,
    &WASTEWOOD_VERGE,
    &WILD_ROADS,
    &WILLOWRUSH_VERGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    GEARSEEKER_SERPENT_REPRINT,
    SPELL_PIERCE_REPRINT,
    BLOODGHAST_REPRINT,
    LIGHTNING_STRIKE_REPRINT,
    BROKEN_WINGS_REPRINT,
    BLOODFELL_CAVES_REPRINT,
    BLOSSOMING_SANDS_REPRINT,
    DISMAL_BACKWATER_REPRINT,
    JUNGLE_HOLLOW_REPRINT,
    RUGGED_HIGHLANDS_REPRINT,
    SCOURED_BARRENS_REPRINT,
    SWIFTWATER_CLIFFS_REPRINT,
    THORNWOOD_FALLS_REPRINT,
    TRANQUIL_COVE_REPRINT,
    WIND_SCARRED_CRAG_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    AIR_RESPONSE_UNIT_ALTERNATE_1,
    BROADCAST_RAMBLER_ALTERNATE_1,
    DETENTION_CHARIOT_ALTERNATE_1,
    SALVATION_ENGINE_ALTERNATE_1,
    SKYSEER_S_CHARIOT_ALTERNATE_1,
    SPOTCYCLE_SCOUTER_ALTERNATE_1,
    VALOR_S_FLAGSHIP_ALTERNATE_1,
    VOYAGER_GLIDECAR_ALTERNATE_1,
    HULLDRIFTER_ALTERNATE_1,
    MIDNIGHT_MANGLER_ALTERNATE_1,
    POSSESSION_ENGINE_ALTERNATE_1,
    RANGERS_REFUELER_ALTERNATE_1,
    THOPTER_FABRICATOR_ALTERNATE_1,
    CARRION_CRUISER_ALTERNATE_1,
    CRYPTCALLER_CHARIOT_ALTERNATE_1,
    DEMONIC_JUNKER_ALTERNATE_1,
    THE_LAST_RIDE_ALTERNATE_1,
    RIPCLAW_WRANGLER_ALTERNATE_1,
    BOOMMOBILE_ALTERNATE_1,
    BURNER_ROCKET_ALTERNATE_1,
    CLAMOROUS_IRONCLAD_ALTERNATE_1,
    GASTAL_THRILLROLLER_ALTERNATE_1,
    SPIRE_MECHCYCLE_ALTERNATE_1,
    EARTHRUMBLER_ALTERNATE_1,
    LUMBERING_WORLDWAGON_ALTERNATE_1,
    THUNDEROUS_VELOCIPEDE_ALTERNATE_1,
    VELOHEART_BIKE_ALTERNATE_1,
    APOCALYPSE_RUNNER_ALTERNATE_1,
    BOOSTED_SLOOP_ALTERNATE_1,
    CLOUDSPIRE_SKYCYCLE_ALTERNATE_1,
    DEBRIS_BEETLE_ALTERNATE_1,
    DUNE_DRIFTER_ALTERNATE_1,
    GUIDELIGHT_PATHMAKER_ALTERNATE_1,
    HAUNTED_HELLRIDE_ALTERNATE_1,
    RANGERS_AETHERHIVE_ALTERNATE_1,
    ROCKETEER_BOOSTBUGGY_ALTERNATE_1,
    THUNDERING_BROODWAGON_ALTERNATE_1,
    LIFECRAFT_ENGINE_ALTERNATE_1,
    MARSHALS_PATHCRUISER_ALTERNATE_1,
    ROVER_BLADES_ALTERNATE_1,
    SKYBOX_FERRY_ALTERNATE_1,
    BULWARK_OX_ALTERNATE_1,
    GUARDIAN_SUNMARE_ALTERNATE_1,
    MINDSPRING_MERFOLK_ALTERNATE_1,
    WAXEN_SHAPETHIEF_ALTERNATE_1,
    BLOODGHAST_ALTERNATE_1,
    GAS_GUZZLER_ALTERNATE_1,
    THE_SPEED_DEMON_ALTERNATE_1,
    BURNOUT_BASHTRONAUT_ALTERNATE_1,
    DRACONAUTICS_ENGINEER_ALTERNATE_1,
    HOWLSQUAD_HEAVY_ALTERNATE_1,
    AGONASAUR_REX_ALTERNATE_1,
    DISTRICT_MASCOT_ALTERNATE_1,
    WEBSTRIKE_ELITE_ALTERNATE_1,
    FEARLESS_SWASHBUCKLER_ALTERNATE_1,
    HAZORET_GODSEEKER_ALTERNATE_1,
    BRIGHTGLASS_GEARHULK_ALTERNATE_1,
    COALSTOKE_GEARHULK_ALTERNATE_1,
    KETRAMOSE_THE_NEW_DAWN_ALTERNATE_1,
    OILDEEP_GEARHULK_ALTERNATE_1,
    PYREWOOD_GEARHULK_ALTERNATE_1,
    RIPTIDE_GEARHULK_ALTERNATE_1,
    SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_1,
    BASRI_TOMORROW_S_CHAMPION_ALTERNATE_1,
    VNWXT_VERBOSE_HOST_ALTERNATE_1,
    GONTI_NIGHT_MINISTER_ALTERNATE_1,
    DARETTI_ROCKETEER_ENGINEER_ALTERNATE_1,
    OVIYA_AUTOMECH_ARTISAN_ALTERNATE_1,
    AATCHIK_EMERALD_RADIAN_ALTERNATE_1,
    CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_1,
    CARADORA_HEART_OF_ALACRIA_ALTERNATE_1,
    FAR_FORTUNE_END_BOSS_ALTERNATE_1,
    KOLODIN_TRIUMPH_CASTER_ALTERNATE_1,
    MENDICANT_CORE_GUIDELIGHT_ALTERNATE_1,
    REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_1,
    SAMUT_THE_DRIVING_FORCE_ALTERNATE_1,
    SITA_VARMA_MASKED_RACER_ALTERNATE_1,
    WINTER_CURSED_RIDER_ALTERNATE_1,
    ZAHUR_GLORY_S_PAST_ALTERNATE_1,
    BLEACHBONE_VERGE_ALTERNATE_1,
    RIVERPYRE_VERGE_ALTERNATE_1,
    SUNBILLOW_VERGE_ALTERNATE_1,
    WASTEWOOD_VERGE_ALTERNATE_1,
    WILLOWRUSH_VERGE_ALTERNATE_1,
    THE_AETHERSPARK_ALTERNATE_1,
    PERILOUS_SNARE_ALTERNATE_1,
    SPECTACULAR_PILEUP_ALTERNATE_1,
    MU_YANLING_WIND_RIDER_ALTERNATE_1,
    REPURPOSING_BAY_ALTERNATE_1,
    RIVERCHURN_MONUMENT_ALTERNATE_1,
    UNSTOPPABLE_PLAN_ALTERNATE_1,
    CURSECLOTH_WRAPPINGS_ALTERNATE_1,
    QUAG_FEAST_ALTERNATE_1,
    COUNT_ON_LUCK_ALTERNATE_1,
    FULL_THROTTLE_ALTERNATE_1,
    AFTERBURNER_EXPERT_ALTERNATE_1,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_1,
    REGAL_IMPERIOSAUR_ALTERNATE_1,
    EXPLOSIVE_GETAWAY_ALTERNATE_1,
    LOOT_THE_PATHFINDER_ALTERNATE_1,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_1,
    MARKETBACK_WALKER_ALTERNATE_1,
    MONUMENT_TO_ENDURANCE_ALTERNATE_1,
    RADIANT_LOTUS_ALTERNATE_1,
    MURAGANDA_RACEWAY_ALTERNATE_1,
    SALVATION_ENGINE_ALTERNATE_2,
    SPECTACULAR_PILEUP_ALTERNATE_2,
    MU_YANLING_WIND_RIDER_ALTERNATE_2,
    CURSECLOTH_WRAPPINGS_ALTERNATE_2,
    CHANDRA_SPARK_HUNTER_ALTERNATE_1,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_2,
    EXPLOSIVE_GETAWAY_ALTERNATE_2,
    LOOT_THE_PATHFINDER_ALTERNATE_2,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_2,
    RADIANT_LOTUS_ALTERNATE_2,
    SALVATION_ENGINE_ALTERNATE_3,
    SPECTACULAR_PILEUP_ALTERNATE_3,
    MU_YANLING_WIND_RIDER_ALTERNATE_3,
    CURSECLOTH_WRAPPINGS_ALTERNATE_3,
    CHANDRA_SPARK_HUNTER_ALTERNATE_2,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_3,
    EXPLOSIVE_GETAWAY_ALTERNATE_3,
    LOOT_THE_PATHFINDER_ALTERNATE_3,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_3,
    RADIANT_LOTUS_ALTERNATE_3,
    TUNE_UP_ALTERNATE_1,
    GASTAL_RAIDER_ALTERNATE_1,
    MARAUDING_MAKO_ALTERNATE_1,
    SKYSERPENT_SEEKER_ALTERNATE_1,
    VOYAGE_HOME_ALTERNATE_1,
    LUMBERING_WORLDWAGON_ALTERNATE_2,
    LIFECRAFT_ENGINE_ALTERNATE_2,
    AMONKHET_RACEWAY_ALTERNATE_1,
    AVISHKAR_RACEWAY_ALTERNATE_1,
    MURAGANDA_RACEWAY_ALTERNATE_2,
    BASRI_TOMORROW_S_CHAMPION_ALTERNATE_2,
    BULWARK_OX_ALTERNATE_2,
    GUARDIAN_SUNMARE_ALTERNATE_2,
    PERILOUS_SNARE_ALTERNATE_2,
    SALVATION_ENGINE_ALTERNATE_4,
    SKYSEER_S_CHARIOT_ALTERNATE_2,
    SPECTACULAR_PILEUP_ALTERNATE_4,
    VALOR_S_FLAGSHIP_ALTERNATE_2,
    VOYAGER_GLIDECAR_ALTERNATE_2,
    MINDSPRING_MERFOLK_ALTERNATE_2,
    MU_YANLING_WIND_RIDER_ALTERNATE_4,
    POSSESSION_ENGINE_ALTERNATE_2,
    REPURPOSING_BAY_ALTERNATE_2,
    RIVERCHURN_MONUMENT_ALTERNATE_2,
    THOPTER_FABRICATOR_ALTERNATE_2,
    UNSTOPPABLE_PLAN_ALTERNATE_2,
    VNWXT_VERBOSE_HOST_ALTERNATE_2,
    WAXEN_SHAPETHIEF_ALTERNATE_2,
    BLOODGHAST_ALTERNATE_2,
    CRYPTCALLER_CHARIOT_ALTERNATE_2,
    CURSECLOTH_WRAPPINGS_ALTERNATE_4,
    DEMONIC_JUNKER_ALTERNATE_2,
    GAS_GUZZLER_ALTERNATE_2,
    GONTI_NIGHT_MINISTER_ALTERNATE_2,
    THE_LAST_RIDE_ALTERNATE_2,
    QUAG_FEAST_ALTERNATE_2,
    THE_SPEED_DEMON_ALTERNATE_2,
    BOOMMOBILE_ALTERNATE_2,
    BURNOUT_BASHTRONAUT_ALTERNATE_2,
    CHANDRA_SPARK_HUNTER_ALTERNATE_3,
    COUNT_ON_LUCK_ALTERNATE_2,
    DARETTI_ROCKETEER_ENGINEER_ALTERNATE_2,
    DRACONAUTICS_ENGINEER_ALTERNATE_2,
    FULL_THROTTLE_ALTERNATE_2,
    GASTAL_THRILLROLLER_ALTERNATE_2,
    HAZORET_GODSEEKER_ALTERNATE_2,
    HOWLSQUAD_HEAVY_ALTERNATE_2,
    AFTERBURNER_EXPERT_ALTERNATE_2,
    AGONASAUR_REX_ALTERNATE_2,
    DISTRICT_MASCOT_ALTERNATE_2,
    LUMBERING_WORLDWAGON_ALTERNATE_3,
    MARCH_OF_THE_WORLD_OOZE_ALTERNATE_4,
    OVIYA_AUTOMECH_ARTISAN_ALTERNATE_2,
    REGAL_IMPERIOSAUR_ALTERNATE_2,
    THUNDEROUS_VELOCIPEDE_ALTERNATE_2,
    WEBSTRIKE_ELITE_ALTERNATE_2,
    AATCHIK_EMERALD_RADIAN_ALTERNATE_2,
    BRIGHTGLASS_GEARHULK_ALTERNATE_2,
    CAPTAIN_HOWLER_SEA_SCOURGE_ALTERNATE_2,
    CARADORA_HEART_OF_ALACRIA_ALTERNATE_2,
    COALSTOKE_GEARHULK_ALTERNATE_2,
    DEBRIS_BEETLE_ALTERNATE_2,
    EXPLOSIVE_GETAWAY_ALTERNATE_4,
    FAR_FORTUNE_END_BOSS_ALTERNATE_2,
    FEARLESS_SWASHBUCKLER_ALTERNATE_2,
    KETRAMOSE_THE_NEW_DAWN_ALTERNATE_2,
    KOLODIN_TRIUMPH_CASTER_ALTERNATE_2,
    LOOT_THE_PATHFINDER_ALTERNATE_4,
    MENDICANT_CORE_GUIDELIGHT_ALTERNATE_2,
    MIMEOPLASM_REVERED_ONE_ALTERNATE_4,
    OILDEEP_GEARHULK_ALTERNATE_2,
    PYREWOOD_GEARHULK_ALTERNATE_2,
    REDSHIFT_ROCKETEER_CHIEF_ALTERNATE_2,
    RIPTIDE_GEARHULK_ALTERNATE_2,
    SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_2,
    SAMUT_THE_DRIVING_FORCE_ALTERNATE_2,
    SITA_VARMA_MASKED_RACER_ALTERNATE_2,
    WINTER_CURSED_RIDER_ALTERNATE_2,
    ZAHUR_GLORY_S_PAST_ALTERNATE_2,
    THE_AETHERSPARK_ALTERNATE_2,
    LIFECRAFT_ENGINE_ALTERNATE_3,
    MARKETBACK_WALKER_ALTERNATE_2,
    MONUMENT_TO_ENDURANCE_ALTERNATE_2,
    RADIANT_LOTUS_ALTERNATE_4,
    BLEACHBONE_VERGE_ALTERNATE_2,
    MURAGANDA_RACEWAY_ALTERNATE_3,
    RIVERPYRE_VERGE_ALTERNATE_2,
    SUNBILLOW_VERGE_ALTERNATE_2,
    WASTEWOOD_VERGE_ALTERNATE_2,
    WILLOWRUSH_VERGE_ALTERNATE_2,
    PLAINS_ALTERNATE_4,
    ISLAND_ALTERNATE_4,
    SWAMP_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_4,
    FOREST_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    ISLAND_ALTERNATE_5,
    SWAMP_ALTERNATE_5,
    MOUNTAIN_ALTERNATE_5,
    FOREST_ALTERNATE_5,
    SALVATION_ENGINE_ALTERNATE_5,
    SKYSEER_S_CHARIOT_ALTERNATE_3,
    VALOR_S_FLAGSHIP_ALTERNATE_3,
    VOYAGER_GLIDECAR_ALTERNATE_3,
    POSSESSION_ENGINE_ALTERNATE_3,
    THOPTER_FABRICATOR_ALTERNATE_3,
    CRYPTCALLER_CHARIOT_ALTERNATE_3,
    DEMONIC_JUNKER_ALTERNATE_3,
    THE_LAST_RIDE_ALTERNATE_3,
    BOOMMOBILE_ALTERNATE_3,
    GASTAL_THRILLROLLER_ALTERNATE_3,
    LUMBERING_WORLDWAGON_ALTERNATE_4,
    THUNDEROUS_VELOCIPEDE_ALTERNATE_3,
    DEBRIS_BEETLE_ALTERNATE_3,
    LIFECRAFT_ENGINE_ALTERNATE_4,
    BULWARK_OX_ALTERNATE_3,
    GUARDIAN_SUNMARE_ALTERNATE_3,
    MINDSPRING_MERFOLK_ALTERNATE_3,
    WAXEN_SHAPETHIEF_ALTERNATE_3,
    BLOODGHAST_ALTERNATE_3,
    GAS_GUZZLER_ALTERNATE_3,
    THE_SPEED_DEMON_ALTERNATE_3,
    BURNOUT_BASHTRONAUT_ALTERNATE_3,
    DRACONAUTICS_ENGINEER_ALTERNATE_3,
    HOWLSQUAD_HEAVY_ALTERNATE_3,
    AGONASAUR_REX_ALTERNATE_3,
    DISTRICT_MASCOT_ALTERNATE_3,
    WEBSTRIKE_ELITE_ALTERNATE_3,
    FEARLESS_SWASHBUCKLER_ALTERNATE_3,
    HAZORET_GODSEEKER_ALTERNATE_3,
    BRIGHTGLASS_GEARHULK_ALTERNATE_3,
    COALSTOKE_GEARHULK_ALTERNATE_3,
    KETRAMOSE_THE_NEW_DAWN_ALTERNATE_3,
    OILDEEP_GEARHULK_ALTERNATE_3,
    PYREWOOD_GEARHULK_ALTERNATE_3,
    RIPTIDE_GEARHULK_ALTERNATE_3,
    SAB_SUNEN_LUXA_EMBODIED_ALTERNATE_3,
];
