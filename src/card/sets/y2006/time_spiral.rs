//! Time Spiral (TSP): complete paper-printing inventory and declarative rules audit.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardNameDef;
use crate::card::CardNameSetDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ChooseGroupDef;
use crate::card::ChooseObjectOrderDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CounterKindDef;
use crate::card::CounterOperationDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageFollowUpDef;
use crate::card::DamageKindDef;
use crate::card::DamageLimitDef;
use crate::card::DamagePreventionDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceGroupDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::HalvedValueDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PartitionGroupDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerObjectCountAggregateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::PregameConditionDef;
use crate::card::PrintedManaCost;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::RoundingDef;
use crate::card::SacrificedAmountDef;
use crate::card::ScaledValueDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::SuspendAbilityDef;
use crate::card::TargetChooserDef;
use crate::card::TokenCountersDef;
use crate::card::TokenStatsDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneChangeEventMatcherDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::actions;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TSP",
    slug: "time-spiral",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// TSP 1 — Amrou Scout
pub(in crate::card::sets) static AMROU_SCOUT: CardRecord = CardRecord::new(
    "Amrou Scout",
    "ea3e05e5-1340-4010-b39b-3571a5829840",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kithkin", "Rebel", "Scout"], 2, 1)
        .with_abilities(&[AbilityDef::activated(
            "{4}, {T}: Search your library for a Rebel permanent card with mana value 3 or less, put it onto the battlefield, then shuffle.",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rebel")),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
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
        )]),
);

// TSP 2 — Amrou Seekers
pub(in crate::card::sets) static AMROU_SEEKERS: CardRecord = CardRecord::new(
    "Amrou Seekers",
    "8bc2a06d-54f7-436f-b32a-4a8daa199fb8",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Kithkin", "Rebel"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked except by artifact creatures and/or white creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::Color(ManaColor::White),
                    ])),
                )),
            },
        ),
    ]),
);

// TSP 3 — Angel's Grace
// Audit: unsupported — Needs a temporary prohibition on losing the game and on opponents winning; the life-floor damage limiter alone is insufficient.
pub(in crate::card::sets) static ANGEL_S_GRACE: CardRecord = CardRecord::new(
    "Angel's Grace",
    "580cb5be-fa59-4eb9-8808-7d2943fb6413",
    "Mark Zug",
    CardRules::unsupported(),
);

// TSP 4 — Benalish Cavalry
pub(in crate::card::sets) static BENALISH_CAVALRY: CardRecord = CardRecord::new(
    "Benalish Cavalry",
    "1013ca9c-1d29-42f6-8665-92f98d076ff8",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 2)
        .with_ability(abilities::flanking()),
);

// TSP 5 — Castle Raptors
pub(in crate::card::sets) static CASTLE_RAPTORS: CardRecord = CardRecord::new(
    "Castle Raptors",
    "d4a3ed1e-f07e-486d-8a44-b2fbfaffa547",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird", "Soldier"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "As long as this creature is untapped, it gets +0/+2.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceUntapped,
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ]),
);

// TSP 6 — Cavalry Master
pub(in crate::card::sets) static CAVALRY_MASTER: CardRecord = CardRecord::new(
    "Cavalry Master",
    "f7b19194-87bf-432c-8d34-91dd9520cbd2",
    "Thomas M. Baxa",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 3, 3).with_abilities(&[
        abilities::flanking(),
        AbilityDef::static_ability(
            "Other creatures you control with flanking have flanking.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flanking),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::flanking()),
            },
        ),
    ]),
);

// TSP 7 — Celestial Crusader
pub(in crate::card::sets) static CELESTIAL_CRUSADER: CardRecord = CardRecord::new(
    "Celestial Crusader",
    "7f5624d6-2c27-4c7a-8001-73edb2512ffb",
    "Jim Murray",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flash(),
        abilities::split_second(),
        abilities::flying(),
        AbilityDef::static_ability(
            "Other white creatures get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::White),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
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

// TSP 8 — Children of Korlis
// Audit: unsupported — Needs the total life lost this turn, including payments and nondamage losses; damage taken and net life change are not equivalent.
pub(in crate::card::sets) static CHILDREN_OF_KORLIS: CardRecord = CardRecord::new(
    "Children of Korlis",
    "221a5895-bc21-4f10-b5fc-a4980fde843e",
    "Quinton Hoover",
    CardRules::unsupported(),
);

// TSP 9 — Chronosavant
// Audit: unsupported — Needs a resolving effect that schedules skipping the controller's next turn.
pub(in crate::card::sets) static CHRONOSAVANT: CardRecord = CardRecord::new(
    "Chronosavant",
    "d6264d4f-adf1-4d7b-b17a-fd7122e9b2cd",
    "Pete Venters",
    CardRules::unsupported(),
);

// TSP 10 — Cloudchaser Kestrel
pub(in crate::card::sets) static CLOUDCHASER_KESTREL: CardRecord = CardRecord::new(
    "Cloudchaser Kestrel",
    "87003295-fd7c-489a-b724-b76eef187b57",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, destroy target enchantment.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        AbilityDef::activated_with_targets(
            "{W}: Target permanent becomes white until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 11 — D'Avenant Healer
pub(in crate::card::sets) static D_AVENANT_HEALER: CardRecord = CardRecord::new(
    "D'Avenant Healer",
    "deac6492-ce39-4137-8418-6169d3b1b632",
    "Michael Sutfin",
    CardRules::new_creature(
        mana_cost!("{1}{W}{W}"),
        &["Human", "Cleric", "Archer"],
        1,
        2,
    )
    .with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target attacking or blocking creature.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::AttackingOrBlocking,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
        AbilityDef::activated_with_targets(
            "{T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[CostDef::TapSource],
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

// TSP 12 — Detainment Spell
pub(in crate::card::sets) static DETAINMENT_SPELL: CardRecord = CardRecord::new(
    "Detainment Spell",
    "c467446b-0168-4c7d-9ab6-57ad8b664877",
    "Darrell Riche",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature's activated abilities can't be activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Any),
                },
            ),
            AbilityDef::activated_with_targets(
                "{1}{W}: Attach this Aura to target creature.",
                &[CostDef::Mana(mana_cost!("{1}{W}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// TSP 13 — Divine Congregation
pub(in crate::card::sets) static DIVINE_CONGREGATION: CardRecord = CardRecord::new(
    "Divine Congregation",
    "9f781071-e9bc-49ee-ac94-23490134b909",
    "Jeremy Jarvis",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "You gain 2 life for each creature target player controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Scaled(&ScaledValueDef::new(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                    2,
                )),
            },
        ),
        abilities::suspend(
            "Suspend 5—{1}{W}",
            &SuspendAbilityDef::fixed(5, &[CostDef::Mana(mana_cost!("{1}{W}"))]),
        ),
    ]),
);

// TSP 14 — Duskrider Peregrine
pub(in crate::card::sets) static DUSKRIDER_PEREGRINE: CardRecord = CardRecord::new(
    "Duskrider Peregrine",
    "bfad8c3e-e459-477c-b602-34df2dda1efe",
    "Una Fricker",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Bird"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Black),
        abilities::suspend(
            "Suspend 3—{1}{W}",
            &SuspendAbilityDef::fixed(3, &[CostDef::Mana(mana_cost!("{1}{W}"))]),
        ),
    ]),
);

// TSP 15 — Errant Doomsayers
pub(in crate::card::sets) static ERRANT_DOOMSAYERS: CardRecord = CardRecord::new(
    "Errant Doomsayers",
    "61a2daf3-4d66-43f8-beda-285d85eefb57",
    "Liz Danforth",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Rebel"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Tap target creature with toughness 2 or less.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
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

// TSP 16 — Evangelize
pub(in crate::card::sets) static EVANGELIZE: CardRecord = CardRecord::new(
    "Evangelize",
    "0f58e1aa-587c-4f89-9552-1128c8c2da1a",
    "Randy Elliott",
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{2}{W}{W}"))]),
        AbilityDef::spell_with_targets(
            "Gain control of target creature of an opponent's choice they control.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                })
                .chosen_by_opponent(),
            ],
            EffectDef::gain_control(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                PlayerRefDef::EffectController,
                ControlDurationDef::Indefinitely,
            ),
        ),
    ]),
);

// TSP 17 — Flickering Spirit
pub(in crate::card::sets) static FLICKERING_SPIRIT: CardRecord = CardRecord::new(
    "Flickering Spirit",
    "64ef9188-737e-4cc6-b3a5-95aafddcf665",
    "Alex Horley-Orlandelli",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{W}: Exile this creature, then return it to the battlefield under its owner's control.",
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Source,
                    face_down: false,
                    then: None,
                },
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ]),
        ),
    ]),
);

// TSP 18 — Foriysian Interceptor
pub(in crate::card::sets) static FORIYSIAN_INTERCEPTOR: CardRecord = CardRecord::new(
    "Foriysian Interceptor",
    "833a3622-9f24-47a3-816f-1732c0e077d0",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 0, 5).with_abilities(&[
        abilities::flash(),
        abilities::defender(),
        AbilityDef::static_ability(
            "This creature can block an additional creature each combat.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
            },
        ),
    ]),
);

// TSP 19 — Fortify
pub(in crate::card::sets) static FORTIFY: CardRecord = CardRecord::new(
    "Fortify",
    "fd063dc7-a35c-44c9-9f8d-b7bb2dc95bec",
    "Christopher Moeller",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Creatures you control get +0/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// TSP 20 — Gaze of Justice
pub(in crate::card::sets) static GAZE_OF_JUSTICE: CardRecord = CardRecord::new(
    "Gaze of Justice",
    "24d565ec-541d-429e-ab45-58db16c2f41d",
    "John Avon",
    CardRules::new_sorcery(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        )
        .with_spell_additional_cost(&CostDef::TapPermanents {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Color(ManaColor::White),
            ]),
            controller: PlayerRelation::You,
            count: 3,
        }),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{W}"))]),
    ]),
);

// TSP 21 — Griffin Guide
pub(in crate::card::sets) static GRIFFIN_GUIDE: CardRecord = CardRecord::new(
    "Griffin Guide",
    "e19a15d9-5899-4b84-9c00-345b19df53ae",
    "Jim Nelson",
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&const { abilities::flying() }),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature dies, create a 2/2 white Griffin creature token with flying.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::create_creature_token(&["Griffin"], &[ManaColor::White], 2, 2)
                    .with_abilities(&[abilities::flying()]),
            ),
        ]),
);

// TSP 22 — Gustcloak Cavalier
pub(in crate::card::sets) static GUSTCLOAK_CAVALIER: CardRecord = CardRecord::new(
    "Gustcloak Cavalier",
    "c36a5462-3a21-4d64-af92-b541b425cdf5",
    "Stuart Griffin",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::flanking(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may tap target creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, you may untap this creature and remove it from combat.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::RemoveFromCombat {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            },
        ),
    ]),
);

// TSP 23 — Icatian Crier
pub(in crate::card::sets) static ICATIAN_CRIER: CardRecord = CardRecord::new(
    "Icatian Crier",
    "523ab784-c77e-4b78-99fc-b5d7ed985d76",
    "Michael Phillippi",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Spellshaper"], 1, 1).with_abilities(
        &[AbilityDef::activated(
            "{1}{W}, {T}, Discard a card: Create two 1/1 white Citizen creature tokens.",
            &[
                CostDef::Mana(mana_cost!("{1}{W}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            EffectDef::create_creature_token(&["Citizen"], &[ManaColor::White], 1, 1)
                .with_count(ValueDef::Constant(2)),
        )],
    ),
);

// TSP 24 — Ivory Giant
pub(in crate::card::sets) static IVORY_GIANT: CardRecord = CardRecord::new(
    "Ivory Giant",
    "72eb30d9-a826-4f29-ae2a-6873997674a7",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Giant"], 3, 4).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, tap all nonwhite creatures.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            },
        ),
        abilities::suspend(
            "Suspend 5—{W}",
            &SuspendAbilityDef::fixed(5, &[CostDef::Mana(mana_cost!("{W}"))]),
        ),
    ]),
);

// TSP 25 — Jedit's Dragoons
pub(in crate::card::sets) static JEDIT_S_DRAGOONS: CardRecord = CardRecord::new(
    "Jedit's Dragoons",
    "e29cc9e5-29b7-4e3c-a0cd-46265b0f74ac",
    "John Matson",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Cat", "Soldier"], 2, 5).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "When this creature enters, you gain 4 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// TSP 26 — Knight of the Holy Nimbus
pub(in crate::card::sets) static KNIGHT_OF_THE_HOLY_NIMBUS: CardRecord = CardRecord::new(
    "Knight of the Holy Nimbus",
    "8fdeb716-4632-4895-b771-0ebd59c868d5",
    "Wayne England",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Rebel", "Knight"], 2, 2)
        .with_abilities(&[
            abilities::flanking(),
            abilities::regenerates_if_destroyed("If this creature would be destroyed, regenerate it. (Tap it, remove it from combat, and heal all damage on it.)"),
            AbilityDef::activated(
                "{2}: This creature can't be regenerated this turn. Only your opponents may activate this ability.",
                &[CostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )
            .only_opponents_may_activate(),
        ]),
);

// TSP 27 — Magus of the Disk
pub(in crate::card::sets) static MAGUS_OF_THE_DISK: CardRecord = CardRecord::new(
    "Magus of the Disk",
    "febbea40-74c7-431f-a88f-c15d4fdda89d",
    "Jeremy Jarvis",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Wizard"], 2, 4).with_abilities(&[
        abilities::enters_tapped(CardType::Creature),
        AbilityDef::activated(
            "{1}, {T}: Destroy all artifacts, creatures, and enchantments.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
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
                then: None,
            },
        ),
    ]),
);

// TSP 28 — Mangara of Corondor
pub(in crate::card::sets) static MANGARA_OF_CORONDOR: CardRecord = CardRecord::new(
    "Mangara of Corondor",
    "a785b66c-c5a4-4520-a9fa-aa199f7f56e5",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Wizard"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::activated_with_targets(
            "{T}: Exile Mangara and target permanent.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::objects(ObjectSetDef::Union(&[
                    ObjectSetDef::One(ObjectRefDef::Source),
                    ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                ])),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        )]),
);

// TSP 29 — Momentary Blink
pub(in crate::card::sets) static MOMENTARY_BLINK: CardRecord = CardRecord::new(
    "Momentary Blink",
    "032e072a-0630-472b-9106-5df554dff785",
    "Anthony S. Waters",
    // Two enters triggers for four mana across two casts, and the blue
    // flashback is why a white deck splashes for it at all.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature you control, then return it to the battlefield under its \
             owner's control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            // Exiled linked to this spell and returned by the same
            // resolution, so it comes back as a new object with its enters
            // triggers armed.
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// TSP 30 — Opal Guardian
pub(in crate::card::sets) static OPAL_GUARDIAN: CardRecord = CardRecord::new(
    "Opal Guardian",
    "905ad286-e70b-46cd-87fd-eec13ed2d43b",
    "Christopher Rush",
    CardRules::new_enchantment(mana_cost!("{W}{W}{W}")).with_abilities(&[
        AbilityDef::triggered_if(
            "When an opponent casts a creature spell, if this permanent is an enchantment, this enchantment becomes a 3/4 Gargoyle creature with flying and protection from red.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ])),
            &TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Gargoyle"])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::flying() }),
                    AppliedEffectDef::add_ability(
                        &const { abilities::protection_from_color(ManaColor::Red) },
                    ),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ]),
);

// TSP 31 — Outrider en-Kor
// Audit: unsupported — Needs a one-point damage-redirection shield from this creature to a chosen creature; existing redirection is unbounded or player-scoped.
pub(in crate::card::sets) static OUTRIDER_EN_KOR: CardRecord = CardRecord::new(
    "Outrider en-Kor",
    "e3e0d923-d94f-4f12-a025-86587b18878f",
    "D. Alexander Gregory",
    CardRules::unsupported(),
);

// TSP 32 — Pentarch Paladin
pub(in crate::card::sets) static PENTARCH_PALADIN: CardRecord = CardRecord::new(
    "Pentarch Paladin",
    "ced01039-e19f-466b-bbc6-9bcb8d3ae845",
    "Jim Murray",
    CardRules::new_creature(mana_cost!("{2}{W}{W}{W}"), &["Human", "Knight"], 3, 3).with_abilities(
        &[
            abilities::flanking(),
            AbilityDef::as_enters(
                "As this permanent enters, choose a color.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ),
            AbilityDef::activated_with_targets(
                "{W}{W}, {T}: Destroy target permanent of the chosen color.",
                &[CostDef::Mana(mana_cost!("{W}{W}")), CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasSourcesChosenScalar(
                            BattlefieldEntryChoiceDestinationDef::Color,
                        ),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
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

// TSP 33 — Pentarch Ward
// Audit: unsupported — Granted protection cannot read the Aura grantor's chosen color; HasSourcesChosenScalar reads the enchanted creature instead.
pub(in crate::card::sets) static PENTARCH_WARD: CardRecord = CardRecord::new(
    "Pentarch Ward",
    "10c70894-dd4a-4cbb-b3c1-57135cc151a9",
    "Dany Orizio",
    CardRules::unsupported(),
);

// TSP 34 — Plated Pegasus
// Audit: unsupported — Needs a static prevention rule that prevents one point from each spell damage event, rather than a consumable shield or unlimited prevention.
pub(in crate::card::sets) static PLATED_PEGASUS: CardRecord = CardRecord::new(
    "Plated Pegasus",
    "873fddf3-40e7-41ee-9519-98d0a5268fd1",
    "Greg Hildebrandt",
    CardRules::unsupported(),
);

// TSP 35 — Pull from Eternity
// Audit: unsupported — Needs a target predicate distinguishing face-up from face-down cards in exile.
pub(in crate::card::sets) static PULL_FROM_ETERNITY: CardRecord = CardRecord::new(
    "Pull from Eternity",
    "3d218091-d218-41ad-b666-c8ab3de7160a",
    "Ron Spears",
    CardRules::unsupported(),
);

// TSP 36 — Pulmonic Sliver
// Audit: unsupported — Needs a controller choice for an optional battlefield-to-library replacement; shared battlefield-exit replacements must currently be mandatory.
pub(in crate::card::sets) static PULMONIC_SLIVER: CardRecord = CardRecord::new(
    "Pulmonic Sliver",
    "31c19d42-aaaf-417c-a303-74262f82e365",
    "Jeff Easley",
    CardRules::unsupported(),
);

// TSP 37 — Quilled Sliver
pub(in crate::card::sets) static QUILLED_SLIVER: CardRecord = CardRecord::new(
    "Quilled Sliver",
    "72486240-eabb-4b37-99cc-ab13413683fa",
    "John Matson",
    CardRules::new_creature(mana_cost!("{1}{W}"), &const { ["Sliver"] }, 1, 1).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"{T}: This permanent deals 1 damage to target attacking or blocking creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_with_targets(
                                "{T}: This permanent deals 1 damage to target attacking or blocking creature.",
                                &const { [CostDef::TapSource] },
                                &const {
                                    [AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::Object {
                                            object: ObjectPredicateDef::All(
                                                &const {
                                                    [
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Creature,
                                                        ),
                                                        ObjectPredicateDef::AnyOf(
                                                            &const {
                                                                [
                                                                    ObjectPredicateDef::Attacking,
                                                                    ObjectPredicateDef::Blocking,
                                                                ]
                                                            },
                                                        ),
                                                    ]
                                                },
                                            ),
                                            zones: &const { [ZoneKind::Battlefield] },
                                            controller: None,
                                            owner: None,
                                        },
                                    )]
                                },
                                EffectDef::damage(
                                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    ValueDef::Constant(1),
                                ),
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 38 — Restore Balance
pub(in crate::card::sets) static RESTORE_BALANCE: CardRecord = CardRecord::new(
    "Restore Balance",
    "c3be9788-2360-4ff0-b7f6-0bfbefa03ce0",
    "Mark Poole",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .printed_colors(&[ManaColor::White])
    .with_abilities(&[
        AbilityDef::spell(
            "Each player chooses a number of lands they control equal to the number of lands controlled by the player who controls the fewest, then sacrifices the rest. Players sacrifice creatures and discard cards the same way.",
            EffectDef::Sequence(&[
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::EachPlayer,
                    candidates: ObjectPredicateDef::HasType(CardType::Land),
                    zone: ZoneKind::Battlefield,
                    selection: PerPlayerSelectionDef::Count(ValueDef::AggregatePlayerObjectCounts(
                        &PlayerObjectCountAggregateDef {
                            players: PlayerSetDef::All,
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Land),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            operation: AggregateOperationDef::Minimum,
                        },
                    )),
                    visibility: ChoiceVisibilityDef::Public,
                    chosen: Binding!("tsp_lands"),
                    unchosen: Binding!("tsp_lands_rest"),
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(Binding!("tsp_lands_rest")),
                    )),
                }),
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::EachPlayer,
                    candidates: ObjectPredicateDef::HasType(CardType::Creature),
                    zone: ZoneKind::Battlefield,
                    selection: PerPlayerSelectionDef::Count(ValueDef::AggregatePlayerObjectCounts(
                        &PlayerObjectCountAggregateDef {
                            players: PlayerSetDef::All,
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            operation: AggregateOperationDef::Minimum,
                        },
                    )),
                    visibility: ChoiceVisibilityDef::Public,
                    chosen: Binding!("tsp_creatures"),
                    unchosen: Binding!("tsp_creatures_rest"),
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(Binding!("tsp_creatures_rest")),
                    )),
                }),
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::EachPlayer,
                    candidates: ObjectPredicateDef::Any,
                    zone: ZoneKind::Hand,
                    selection: PerPlayerSelectionDef::Count(ValueDef::AggregatePlayerObjectCounts(
                        &PlayerObjectCountAggregateDef {
                            players: PlayerSetDef::All,
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerRelation::You,
                            ),
                            operation: AggregateOperationDef::Minimum,
                        },
                    )),
                    visibility: ChoiceVisibilityDef::Private,
                    chosen: Binding!("tsp_hand"),
                    unchosen: Binding!("tsp_hand_rest"),
                    then: &EffectDef::discard_cards(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(Binding!("tsp_hand_rest")),
                    )),
                }),
            ]),
        ),
        abilities::suspend(
            "Suspend 6—{W}",
            &SuspendAbilityDef::fixed(6, &[CostDef::Mana(mana_cost!("{W}"))]),
        ),
    ]),
);

// TSP 39 — Return to Dust
// Audit: unsupported — Needs cast-time own-main-phase provenance for the optional second exile; sorcery-speed timing is not equivalent.
pub(in crate::card::sets) static RETURN_TO_DUST: CardRecord = CardRecord::new(
    "Return to Dust",
    "48185c8f-ac41-46e2-85b1-760abac914ac",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// TSP 40 — Serra Avenger
// Audit: unsupported — Needs cast restrictions based on how many turns this player has taken, including extra turns.
pub(in crate::card::sets) static SERRA_AVENGER: CardRecord = CardRecord::new(
    "Serra Avenger",
    "9e9d7c1c-3bfd-4705-9bc2-5ca3f84cc32a",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// TSP 41 — Sidewinder Sliver
pub(in crate::card::sets) static SIDEWINDER_SLIVER: CardRecord = CardRecord::new(
    "Sidewinder Sliver",
    "b073b8f2-b42d-40f2-b5fb-e78ffb1733ea",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{W}"), &["Sliver"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures have flanking.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::flanking() }),
            },
        ),
    ]),
);

// TSP 42 — Spirit Loop
pub(in crate::card::sets) static SPIRIT_LOOP: CardRecord = CardRecord::new(
    "Spirit Loop",
    "31ffb2e6-b1bc-41c5-9e52-294c060b1d45",
    "Wayne Reynolds",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature_you_control(),
            AbilityDef::triggered(
                "Whenever enchanted creature deals damage, you gain that much life.",
                TriggerEventDef::damage_dealt_by(ObjectPredicateDef::AttachedToSource),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ),
            AbilityDef::triggered(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TSP 43 — Temporal Isolation
pub(in crate::card::sets) static TEMPORAL_ISOLATION: CardRecord = CardRecord::new(
    "Temporal Isolation",
    "eb406292-1879-4aa9-a369-082822dae1d7",
    "Stephen Tappin",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has shadow.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&const { abilities::shadow() }),
                },
            ),
            AbilityDef::static_ability(
                "Prevent all damage that would be dealt by enchanted creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
                        DamageEventMatcherDef {
                            source: DamageSourceMatcherDef::AffectedObject,
                            ..DamageEventMatcherDef::ANY
                        },
                    )),
                },
            ),
        ]),
);

// TSP 44 — Tivadar of Thorn
pub(in crate::card::sets) static TIVADAR_OF_THORN: CardRecord = CardRecord::new(
    "Tivadar of Thorn",
    "f34d3a74-496e-462d-afa6-370dc2b8347d",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Knight"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::protection_from_color(ManaColor::Red),
            AbilityDef::triggered_with_targets(
                "When Tivadar enters, destroy target Goblin.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ]),
);

// TSP 45 — Watcher Sliver
pub(in crate::card::sets) static WATCHER_SLIVER: CardRecord = CardRecord::new(
    "Watcher Sliver",
    "7d72a950-82ed-4e7b-9e18-b8231a2ebea7",
    "Liz Danforth",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Sliver"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures get +0/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
            },
        ),
    ]),
);

// TSP 46 — Weathered Bodyguards
pub(in crate::card::sets) static WEATHERED_BODYGUARDS: CardRecord = CardRecord::new(
    "Weathered Bodyguards",
    "991a18e5-a183-4b84-b157-5a3609b3f910",
    "Wayne Reynolds",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Human", "Soldier"], 2, 5)
        .with_morph(&[CostDef::Mana(mana_cost!("{3}{W}"))])
        .with_abilities(&[
            AbilityDef::static_ability(
                "As long as this creature is untapped, all combat damage that would be dealt to you by unblocked creatures is dealt to this creature instead.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceUntapped,
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::RedirectPlayerDamageToThis(
                            DamageSourceGroupDef::UnblockedCreatures,
                        )),
                    },
                },
            ),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                Some("Morph {3}{W}"),
                EffectDef::None,
            ),
        ]),
);

// TSP 47 — Zealot il-Vec
pub(in crate::card::sets) static ZEALOT_IL_VEC: CardRecord = CardRecord::new(
    "Zealot il-Vec",
    "3a2a2740-68cb-48c9-9716-081d369075e0",
    "Chippy",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Rebel"], 1, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may have it deal 1 damage to target creature. If you do, prevent all combat damage this creature would deal this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                    EffectDef::PreventDamage {
                        prevention: DamagePreventionDef::unlimited(
                            DamageEventMatcherDef::combat_from(ObjectRefDef::Source),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            },
        ),
    ]),
);

// TSP 48 — Ancestral Vision
pub(in crate::card::sets) static ANCESTRAL_VISION: CardRecord = CardRecord::new(
    "Ancestral Vision",
    "bccedc4d-38c7-4bf3-9ca7-4febd6c49d3d",
    "Mark Poole",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .with_abilities(&[
        abilities::suspend(
            "Suspend 4—{U}",
            &crate::card::SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{U}"))]),
        ),
        AbilityDef::spell_with_targets(
            "Target player draws three cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// TSP 49 — Bewilder
pub(in crate::card::sets) static BEWILDER: CardRecord = CardRecord::new(
    "Bewilder",
    "7be63098-6446-4e64-85b0-9af914f5e075",
    "Ralph Horsley",
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -3/-0 until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
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
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// TSP 50 — Brine Elemental
// Audit: unsupported — Needs a turned-face-up trigger and a player-scoped skip-next-untap-step effect; morph payment alone is insufficient.
pub(in crate::card::sets) static BRINE_ELEMENTAL: CardRecord = CardRecord::new(
    "Brine Elemental",
    "8dab2c73-501b-46c9-bc76-134b50d35bb8",
    "Stephen Tappin",
    CardRules::unsupported(),
);

// TSP 51 — Cancel
pub(in crate::card::sets) static CANCEL: CardRecord = CardRecord::new(
    "Cancel",
    "b4e175f7-f649-451b-9ee5-ad1140b2e8a7",
    "Mark Poole",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Spell,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// TSP 52 — Careful Consideration
// Audit: unsupported — Needs cast-time own-main-phase provenance; SourceCastAtInstantSpeed also includes responses during that main phase.
pub(in crate::card::sets) static CAREFUL_CONSIDERATION: CardRecord = CardRecord::new(
    "Careful Consideration",
    "bcf71574-5b1e-4483-9a8e-c3212148ce9c",
    "Janine Johnston",
    CardRules::unsupported(),
);

// TSP 53 — Clockspinning
pub(in crate::card::sets) static CLOCKSPINNING: CardRecord = CardRecord::new(
    "Clockspinning",
    "1323d548-e2fe-47c5-8df3-f181aed537c5",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Choose a counter on target permanent or suspended card. Remove that counter from that permanent or card or put another of those counters on it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyOf(&[
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasAnyCounter,
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    AbilityTargetPredicate::Object {
                        object: abilities::SUSPENDED_CARD,
                        zones: &[ZoneKind::Exile],
                        controller: None,
                        owner: None,
                    },
                ]),
            )],
            EffectDef::ChooseCounterKind {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Remove the chosen counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Chosen,
                                operation: CounterOperationDef::Remove,
                                amount: ValueDef::Constant(1),
                            },
                        },
                        EffectChoiceDef {
                            label: "Put another of the chosen counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Chosen,
                                operation: CounterOperationDef::Add,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    ],
                },
            },
        ),
    ]),
);

// TSP 54 — Coral Trickster
// Audit: unsupported — Needs a turned-face-up trigger for the optional tap-or-untap instruction.
pub(in crate::card::sets) static CORAL_TRICKSTER: CardRecord = CardRecord::new(
    "Coral Trickster",
    "ffcdcfa1-f499-4d95-9d79-3b3996cd75ed",
    "D. Alexander Gregory",
    CardRules::unsupported(),
);

// TSP 55 — Crookclaw Transmuter
pub(in crate::card::sets) static CROOKCLAW_TRANSMUTER: CardRecord = CardRecord::new(
    "Crookclaw Transmuter",
    "d2266726-1832-4b7b-92e1-9bb0c43b7d5d",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Wizard"], 3, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, switch target creature's power and toughness until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::switch_power_toughness(),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 56 — Deep-Sea Kraken
pub(in crate::card::sets) static DEEP_SEA_KRAKEN: CardRecord = CardRecord::new(
    "Deep-Sea Kraken",
    "8e050532-e245-4eea-90a5-03e3e410dcbe",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{7}{U}{U}{U}"), &["Kraken"], 6, 6).with_abilities(&[
        abilities::cannot_be_blocked(),
        abilities::suspend(
            "Suspend 9—{2}{U}",
            &crate::card::SuspendAbilityDef::fixed(9, &[CostDef::Mana(mana_cost!("{2}{U}"))]),
        ),
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell, if this card is suspended, remove a time counter from it.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &abilities::SUSPEND_SOURCE_IS_SUSPENDED,
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("time"),
                amount: ValueDef::Constant(1),
            },
        )
        .with_source_zones(&[ZoneKind::Exile]),
    ]),
);

// TSP 57 — Draining Whelk
pub(in crate::card::sets) static DRAINING_WHELK: CardRecord = CardRecord::new(
    "Draining Whelk",
    "c559d326-b97b-43d9-b7c9-c09e1a0e9db6",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Illusion"], 1, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, counter target spell. Put X +1/+1 counters on this creature, where X is that spell's mana value.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
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
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ]),
);

// TSP 58 — Dream Stalker
pub(in crate::card::sets) static DREAM_STALKER: CardRecord = CardRecord::new(
    "Dream Stalker",
    "a0e8fc8e-3b23-4275-b19b-43aa6a307619",
    "Brian Despain",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Illusion"], 1, 5).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, return a permanent you control to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 1,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            }),
        ),
    ]),
);

// TSP 59 — Drifter il-Dal
pub(in crate::card::sets) static DRIFTER_IL_DAL: CardRecord = CardRecord::new(
    "Drifter il-Dal",
    "4c3de909-b47e-45d7-9922-8d5e08a76ec9",
    "Justin Sweet",
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 2, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {U}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                &[CostDef::Mana(mana_cost!("{U}"))],
                &EffectDef::sacrifice_yours(EffectRecipientDef::Source),
            )),
        ),
    ]),
);

// TSP 60 — Errant Ephemeron
pub(in crate::card::sets) static ERRANT_EPHEMERON: CardRecord = CardRecord::new(
    "Errant Ephemeron",
    "398c26cc-cd55-42c6-a744-aaefd7018960",
    "Luca Zontini",
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Illusion"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::suspend(
            "Suspend 4—{1}{U}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{1}{U}"))]),
        ),
    ]),
);

// TSP 61 — Eternity Snare
pub(in crate::card::sets) static ETERNITY_SNARE: CardRecord = CardRecord::new(
    "Eternity Snare",
    "de753839-cf75-48d0-98c3-5765779678c0",
    "Drew Tucker",
    CardRules::new_enchantment(mana_cost!("{5}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When this Aura enters, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// TSP 62 — Fathom Seer
// Audit: unsupported — Needs a turned-face-up trigger for drawing cards; the return-Islands morph payment alone is insufficient.
pub(in crate::card::sets) static FATHOM_SEER: CardRecord = CardRecord::new(
    "Fathom Seer",
    "20de275a-2e11-4452-8037-bc397dd53a8c",
    "Ralph Horsley",
    CardRules::unsupported(),
);

// TSP 63 — Fledgling Mawcor
pub(in crate::card::sets) static FLEDGLING_MAWCOR: CardRecord = CardRecord::new(
    "Fledgling Mawcor",
    "c464923e-ae6e-4c1d-9315-0ddb86c07b40",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Beast"], 2, 2)
        .with_morph(&[CostDef::Mana(mana_cost!("{U}{U}"))])
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{T}: This creature deals 1 damage to any target.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
            ),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                Some("Morph {U}{U}"),
                EffectDef::None,
            ),
        ]),
);

// TSP 64 — Fool's Demise
pub(in crate::card::sets) static FOOL_S_DEMISE: CardRecord = CardRecord::new(
    "Fool's Demise",
    "3cbc7ac4-c28e-4155-b3c8-60946653b9b4",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_enchantment(mana_cost!("{4}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature dies, return that card to the battlefield under your control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::TriggeringZoneChangeResult,
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            AbilityDef::triggered(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TSP 65 — Ixidron
// Audit: unsupported — Needs to turn existing battlefield objects face down and count their face-down state without changing zones.
pub(in crate::card::sets) static IXIDRON: CardRecord = CardRecord::new(
    "Ixidron",
    "a2f5c613-80ad-487f-995a-6a869c137798",
    "Terese Nielsen",
    CardRules::unsupported(),
);

// TSP 66 — Looter il-Kor
pub(in crate::card::sets) static LOOTER_IL_KOR: CardRecord = CardRecord::new(
    "Looter il-Kor",
    "368ee06f-9021-4b65-9f53-9c326bf3a27f",
    "Mike Dringenberg",
    // Shadow makes the trigger unconditional in practice, which is why a
    // 1/1 that loots every turn is worth two mana.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Kor", "Rogue"], 1, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, draw a card, then discard a \
             card.",
            // Any damage rather than combat damage, so a pump that pings
            // still loots.
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::Sequence(&[
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
        ),
    ]),
);

// TSP 67 — Magus of the Jar
pub(in crate::card::sets) static MAGUS_OF_THE_JAR: CardRecord = CardRecord::new(
    "Magus of the Jar",
    "31961041-a9e6-4d96-bd4a-5c5f88a79e74",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Human", "Wizard"], 3, 3).with_abilities(&[
        AbilityDef::activated(
            "{T}, Sacrifice this creature: Each player exiles all cards from their hand face down and draws seven cards. At the beginning of the next end step, each player discards their hand and returns to their hand each card they exiled this way.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::Any,
                ),
                face_down: true,
                then: Some(&EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::EachPlayer,
                        amount: ValueDef::Constant(7),
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, each player discards their hand and returns to their hand each card they exiled this way.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Sequence(&[
                            EffectDef::discard_cards(EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Hand],
                                PlayerRelation::Any,
                            )),
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                counters: None,
                                zone: ZoneKind::Hand,
                                grant: None,
                                controller: None,
                                transformed: false,
                            },
                        ]),
                    ))),
                ])),
            },
        ),
    ]),
);

// TSP 68 — Moonlace
// Audit: unsupported — Needs continuous color changes on stack objects that persist through spell resolution when the spell becomes a permanent.
pub(in crate::card::sets) static MOONLACE: CardRecord = CardRecord::new(
    "Moonlace",
    "c781cc80-5f57-4677-a9da-3523191cc7c6",
    "Mike Dringenberg",
    CardRules::unsupported(),
);

// TSP 69 — Mystical Teachings
pub(in crate::card::sets) static MYSTICAL_TEACHINGS: CardRecord = CardRecord::new(
    "Mystical Teachings",
    "a057e5f3-b6bd-4995-b990-714201f36989",
    "Ron Spears",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Search your library for an instant card or a card with flash, reveal it, put it into your hand, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flash),
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
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{B}"))]),
    ]),
);

// TSP 70 — Ophidian Eye
pub(in crate::card::sets) static OPHIDIAN_EYE: CardRecord = CardRecord::new(
    "Ophidian Eye",
    "26836ff5-b3c3-4b10-af1e-df3658781cb2",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever enchanted creature deals damage to an opponent, you may draw a card.",
                TriggerEventDef::damage_to_player(
                    ObjectPredicateDef::AttachedToSource,
                    PlayerRelation::Opponent,
                ),
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

// TSP 71 — Paradox Haze
// Audit: unsupported — Needs insertion of an additional upkeep step and identification of the enchanted player's first upkeep each turn.
pub(in crate::card::sets) static PARADOX_HAZE: CardRecord = CardRecord::new(
    "Paradox Haze",
    "a7c74def-83e5-4420-989d-2304bf4743ae",
    "Greg Staples",
    CardRules::unsupported(),
);

// TSP 72 — Psionic Sliver
pub(in crate::card::sets) static PSIONIC_SLIVER: CardRecord = CardRecord::new(
    "Psionic Sliver",
    "2559ca03-4442-47c5-bd6e-84e71cfa6ca5",
    "Wayne England",
    CardRules::new_creature(mana_cost!("{4}{U}"), &const { ["Sliver"] }, 2, 2).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Sliver creatures have \"{T}: This creature deals 2 damage to any target and 3 damage to itself.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(
                            &const {
                                [
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                ]
                            },
                        ),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_with_targets(
                                "{T}: This creature deals 2 damage to any target and 3 damage to itself.",
                                &const { [CostDef::TapSource] },
                                &const {
                                    [AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::AnyTarget,
                                    )]
                                },
                                EffectDef::damage_simultaneously(
                                    &const {
                                        [
                                            DamageAssignmentDef::from_effect(
                                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                                ValueDef::Constant(2),
                                            ),
                                            DamageAssignmentDef::from_effect(
                                                EffectRecipientDef::Source,
                                                ValueDef::Constant(3),
                                            ),
                                        ]
                                    },
                                ),
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 73 — Riftwing Cloudskate
pub(in crate::card::sets) static RIFTWING_CLOUDSKATE: CardRecord = CardRecord::new(
    "Riftwing Cloudskate",
    "78ad6c86-8cfb-4f24-b8e8-ecbeffc949b8",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Illusion"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, return target permanent to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
        abilities::suspend(
            "Suspend 3—{1}{U}",
            &SuspendAbilityDef::fixed(3, &[CostDef::Mana(mana_cost!("{1}{U}"))]),
        ),
    ]),
);

// TSP 74 — Sage of Epityr
pub(in crate::card::sets) static SAGE_OF_EPITYR: CardRecord = CardRecord::new(
    "Sage of Epityr",
    "8e2ea578-069e-4020-a762-d108a3e14861",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, look at the top four cards of your library, then put them back in any order.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::EffectController,
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// TSP 75 — Screeching Sliver
pub(in crate::card::sets) static SCREECHING_SLIVER: CardRecord = CardRecord::new(
    "Screeching Sliver",
    "313e71da-ce72-4976-8286-f4495ea56485",
    "Stuart Griffin",
    CardRules::new_creature(mana_cost!("{U}"), &const { ["Sliver"] }, 1, 1).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"{T}: Target player mills a card.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_with_targets(
                                "{T}: Target player mills a card.",
                                &const { [CostDef::TapSource] },
                                &const {
                                    [AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::Player(PlayerRelation::Any),
                                    )]
                                },
                                EffectDef::Mill {
                                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    amount: ValueDef::Constant(1),
                                },
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 76 — Shadow Sliver
pub(in crate::card::sets) static SHADOW_SLIVER: CardRecord = CardRecord::new(
    "Shadow Sliver",
    "fb725914-1b0f-4efc-808b-9fe2eaa7f17d",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Sliver"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures have shadow.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::shadow() }),
            },
        ),
    ]),
);

// TSP 77 — Slipstream Serpent
pub(in crate::card::sets) static SLIPSTREAM_SERPENT: CardRecord = CardRecord::new(
    "Slipstream Serpent",
    "b8dc6ee4-4b8a-4746-9d3f-4f66b3809670",
    "Darrell Riche",
    CardRules::new_creature(mana_cost!("{7}{U}"), &["Serpent"], 6, 6)
        .with_morph(&[CostDef::Mana(mana_cost!("{5}{U}"))])
        .with_abilities(&[
            AbilityDef::static_ability(
                "This creature can't attack unless defending player controls an Island.",
                EffectDef::CannotAttackUnless(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                )),
            ),
            AbilityDef::triggered_if(
                "When you control no Islands, sacrifice this creature.",
                TriggerEventDef::StateCondition,
                &TriggerConditionDef::Not(&TriggerConditionDef::controls_basic_land_type(
                    PlayerRelation::You,
                    BasicLandType::Island,
                )),
                EffectDef::sacrifice_yours(EffectRecipientDef::Source),
            ),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                Some("Morph {5}{U}"),
                EffectDef::None,
            ),
        ]),
);

// TSP 78 — Snapback
pub(in crate::card::sets) static SNAPBACK: CardRecord = CardRecord::new(
    "Snapback",
    "de9135ba-9b6a-4443-a1c4-882f9bfae626",
    "Alan Pollack",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Exile {
                object: ObjectPredicateDef::Color(ManaColor::Blue),
                from: ZoneKind::Hand,
                quantity: CostQuantityDef::Fixed(1),
            }],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may exile a blue card from your hand rather than pay this spell's mana cost.",
            ),
            EffectDef::None,
        ),
        AbilityDef::spell_with_targets(
            "Return target creature to its owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
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

// TSP 79 — Spell Burst
pub(in crate::card::sets) static SPELL_BURST: CardRecord = CardRecord::new(
    "Spell Burst",
    "f95c8015-fd7d-4329-ab23-aec37a824083",
    "Terese Nielsen",
    CardRules::new_instant(mana_cost!("{X}{U}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Counter target spell with mana value X.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
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

// TSP 80 — Spiketail Drakeling
pub(in crate::card::sets) static SPIKETAIL_DRAKELING: CardRecord = CardRecord::new(
    "Spiketail Drakeling",
    "c752a0b9-cc87-4c3e-a1e8-0162715e011a",
    "Dave Dorman",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Drake"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Counter target spell unless its controller pays {2}.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(&[CostDef::Mana(mana_cost!("{2}"))]),
        ),
    ]),
);

// TSP 81 — Sprite Noble
pub(in crate::card::sets) static SPRITE_NOBLE: CardRecord = CardRecord::new(
    "Sprite Noble",
    "0de52e53-8a45-4c0a-bb7d-6dc0d659a7c0",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Faerie", "Noble"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other creatures you control with flying get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated(
            "{T}: Other creatures you control with flying get +1/+0 until end of turn.",
            &[CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 82 — Stormcloud Djinn
pub(in crate::card::sets) static STORMCLOUD_DJINN: CardRecord = CardRecord::new(
    "Stormcloud Djinn",
    "ea1b0791-618b-4060-b4d9-3bd7a778098e",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Djinn"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
        AbilityDef::activated(
            "{R}{R}: This creature gets +2/+0 until end of turn and deals 1 damage to you.",
            &[CostDef::Mana(mana_cost!("{R}{R}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// TSP 83 — Teferi, Mage of Zhalfir
// Audit: unsupported — Needs an opponent-scoped sorcery-timing restriction that overrides flash and permission to cast during resolution.
pub(in crate::card::sets) static TEFERI_MAGE_OF_ZHALFIR: CardRecord = CardRecord::new(
    "Teferi, Mage of Zhalfir",
    "3c0145f6-4e27-49e7-9ef6-bac6fa3de26d",
    "D. Alexander Gregory & Jeremy Jarvis",
    CardRules::unsupported(),
);

// TSP 84 — Telekinetic Sliver
pub(in crate::card::sets) static TELEKINETIC_SLIVER: CardRecord = CardRecord::new(
    "Telekinetic Sliver",
    "61b934ad-4858-4680-924a-53ea4f250f9e",
    "Randy Elliott",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &const { ["Sliver"] }, 2, 2).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"{T}: Tap target permanent.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_with_targets(
                                "{T}: Tap target permanent.",
                                &const { [CostDef::TapSource] },
                                &const {
                                    [AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::Object {
                                            object: ObjectPredicateDef::Any,
                                            zones: &const { [ZoneKind::Battlefield] },
                                            controller: None,
                                            owner: None,
                                        },
                                    )]
                                },
                                EffectDef::Tap {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                },
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 85 — Temporal Eddy
pub(in crate::card::sets) static TEMPORAL_EDDY: CardRecord = CardRecord::new(
    "Temporal Eddy",
    "3cab2147-d496-489b-aaf0-b354e31a6b45",
    "Wayne England",
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target creature or land on top of its owner's library.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Library,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// TSP 86 — Think Twice
pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new(
    "Think Twice",
    "352d99db-de6d-4405-90ec-b144abbaa5a4",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell("Draw a card.", abilities::draw_cards(ValueDef::Constant(1))),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
    ]),
);

// TSP 87 — Tolarian Sentinel
pub(in crate::card::sets) static TOLARIAN_SENTINEL: CardRecord = CardRecord::new(
    "Tolarian Sentinel",
    "2e97ec8b-6163-41ff-9e6f-af091a7c529e",
    "Thomas M. Baxa",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Spellshaper"], 1, 3).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{U}, {T}, Discard a card: Return target permanent you control to its owner's hand.",
                &[
                    CostDef::Mana(mana_cost!("{U}")),
                    CostDef::TapSource,
                    CostDef::discard(ObjectPredicateDef::Any),
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
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
);

// TSP 88 — Trickbind
// Audit: unsupported — Needs a successful-counter continuation before disabling the countered ability's originating permanent; unconditional sequencing cannot express "countered this way".
pub(in crate::card::sets) static TRICKBIND: CardRecord = CardRecord::new(
    "Trickbind",
    "f2e58ff2-dea3-42b3-8c22-3e6202a7d433",
    "John Zeleznik",
    CardRules::unsupported(),
);

// TSP 89 — Truth or Tale
pub(in crate::card::sets) static TRUTH_OR_TALE: CardRecord = CardRecord::new(
    "Truth or Tale",
    "5b3e301a-02a5-4d92-9cdd-5ab877bf8ed3",
    "Michael Phillippi",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell(
        "Reveal the top five cards of your library and separate them into two piles. An opponent chooses one of those piles. Put a card from the chosen pile into your hand, then put all other cards revealed this way on the bottom of your library in any order.",
        abilities::bind_top_cards_then(
            PlayerRefDef::EffectController,
            ValueDef::Constant(5),
            &EffectDef::Sequence(&[
                EffectDef::RevealObjects(RevealObjectsDef {
                    input: ObjectSetDef::Binding(ParentBinding),
                    then: &EffectDef::None,
                }),
                EffectDef::PartitionGroup(PartitionGroupDef {
                    actor: PlayerRefDef::EffectController,
                    input: ObjectSetDef::Binding(ParentBinding),
                    first: Binding!("tsp_pile_one"),
                    second: Binding!("tsp_pile_two"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::ChooseGroup(ChooseGroupDef {
                        actor: PlayerRefDef::Opponent,
                        first: ObjectSetDef::Binding(Binding!("tsp_pile_one")),
                        second: ObjectSetDef::Binding(Binding!("tsp_pile_two")),
                        chosen: Binding!("tsp_pile_chosen"),
                        unchosen: Binding!("tsp_pile_rest"),
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                            unchosen: Some(Binding!("tsp_remaining")),
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Binding(Binding!("tsp_pile_chosen")),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::Sequence(&[
                                EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                    ZoneKind::Hand,
                                    ZonePlacement::Top,
                                ),
                                EffectDef::ChooseObjectOrder(ChooseObjectOrderDef {
                                    actor: PlayerRefDef::EffectController,
                                    input: ObjectSetDef::Union(&[
                                        ObjectSetDef::Binding(Binding!("tsp_pile_rest")),
                                        ObjectSetDef::Binding(Binding!("tsp_remaining")),
                                    ]),
                                    ordered: ParentBinding,
                                    placement: ZonePlacement::Bottom,
                                    visibility: ChoiceVisibilityDef::Public,
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
                        }),
                    }),
                }),
            ]),
        ),
    )]),
);

// TSP 90 — Vesuvan Shapeshifter
// Audit: unsupported — Needs turned-face-up replacement choices, face-down actions, and a copy duration ending on turning face down.
pub(in crate::card::sets) static VESUVAN_SHAPESHIFTER: CardRecord = CardRecord::new(
    "Vesuvan Shapeshifter",
    "458031cc-238d-45f0-9a81-b52e82286e0f",
    "Quinton Hoover",
    CardRules::unsupported(),
);

// TSP 91 — Viscerid Deepwalker
pub(in crate::card::sets) static VISCERID_DEEPWALKER: CardRecord = CardRecord::new(
    "Viscerid Deepwalker",
    "cdc9f54e-7fba-4f03-83ca-6d293dffc07a",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Homarid", "Warrior"], 2, 3).with_abilities(&[
        abilities::apply_to_self_until_end_of_turn(
            "{U}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
        ),
        abilities::suspend(
            "Suspend 4—{U}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{U}"))]),
        ),
    ]),
);

// TSP 92 — Voidmage Husher
pub(in crate::card::sets) static VOIDMAGE_HUSHER: CardRecord = CardRecord::new(
    "Voidmage Husher",
    "c9765c83-fe52-43c5-9466-7318809bbfe0",
    "Chippy",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, counter target activated ability.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::ActivatedAbility,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
        AbilityDef::triggered(
            "Whenever you cast a spell, you may return this creature to its owner's hand.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
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

// TSP 93 — Walk the Aeons
pub(in crate::card::sets) static WALK_THE_AEONS: CardRecord = CardRecord::new(
    "Walk the Aeons",
    "9db567cf-3d26-4216-932b-53ca4cfeb5b7",
    "Jeremy Jarvis",
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        abilities::buyback(&[CostDef::sacrifice(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            CostQuantityDef::Fixed(3),
        )]),
        AbilityDef::spell_with_targets(
            "Target player takes an extra turn after this one.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// TSP 94 — Wipe Away
pub(in crate::card::sets) static WIPE_AWAY: CardRecord = CardRecord::new(
    "Wipe Away",
    "37eb43af-367c-4778-a7cc-7ee3b3103379",
    "Jeff Miracola",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Return target permanent to its owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
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

// TSP 95 — Assassinate
pub(in crate::card::sets) static ASSASSINATE: CardRecord = CardRecord::new(
    "Assassinate",
    "40b67839-622d-41c1-b9c7-1a26b021ec78",
    "Kev Walker",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target tapped creature.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Tapped,
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )]),
);

// TSP 96 — Basal Sliver
pub(in crate::card::sets) static BASAL_SLIVER: CardRecord = CardRecord::new(
    "Basal Sliver",
    "4564e9df-bfa3-48e5-a12e-f7e96a504cb1",
    "Drew Tucker",
    CardRules::new_creature(mana_cost!("{2}{B}"), &const { ["Sliver"] }, 2, 2).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"Sacrifice this permanent: Add {B}{B}.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_mana(
                                "Sacrifice this permanent: Add {B}{B}.",
                                &const { [CostDef::SacrificeSource] },
                                EffectDef::AddMana(
                                    AddManaEffectDef::one(ManaColor::Black).with_amount(2),
                                ),
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 97 — Call to the Netherworld
// Audit: unsupported — Needs madness's discard-to-exile replacement and optional cast trigger.
pub(in crate::card::sets) static CALL_TO_THE_NETHERWORLD: CardRecord = CardRecord::new(
    "Call to the Netherworld",
    "fa7322e2-dbea-46e1-ba29-a86a13e5d33e",
    "Vance Kovacs",
    CardRules::unsupported(),
);

// TSP 98 — Corpulent Corpse
pub(in crate::card::sets) static CORPULENT_CORPSE: CardRecord = CardRecord::new(
    "Corpulent Corpse",
    "a58b842a-a4c0-475d-a8b3-62d4e5bb2eaf",
    "Doug Chaffee",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Zombie"], 3, 3).with_abilities(&[
        abilities::fear(),
        abilities::suspend(
            "Suspend 5—{B}",
            &SuspendAbilityDef::fixed(5, &[CostDef::Mana(mana_cost!("{B}"))]),
        ),
    ]),
);

// TSP 99 — Curse of the Cabal
pub(in crate::card::sets) static CURSE_OF_THE_CABAL: CardRecord = CardRecord::new(
    "Curse of the Cabal",
    "d4323aa6-a1fb-42ee-9634-3747129fde3b",
    "John Avon",
    CardRules::new_sorcery(mana_cost!("{9}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player sacrifices half the permanents they control of their choice, rounded down.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                object: ObjectPredicateDef::Any,
                count: ValueDef::Halved(&HalvedValueDef {
                    value: ValueDef::CountMatchingObjects(&ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    )),
                    rounding: RoundingDef::Down,
                }),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
        abilities::suspend(
            "Suspend 2—{2}{B}{B}",
            &SuspendAbilityDef::fixed(2, &[CostDef::Mana(mana_cost!("{2}{B}{B}"))]),
        ),
        AbilityDef::triggered_if(
            "At the beginning of each player's upkeep, if this card is suspended, that player may sacrifice a permanent of their choice. If the player does, put two time counters on this card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            &abilities::SUSPEND_SOURCE_IS_SUSPENDED,
            EffectDef::PayOr(
                PayOrDef::optional(
                    &[CostDef::SacrificePermanent {
                        object: ObjectPredicateDef::Any,
                        controller: PlayerRelation::You,
                    }],
                    &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("time"),
                        amount: ValueDef::Constant(2),
                    },
                )
                .with_payer(PlayerSetDef::One(PlayerRefDef::EventPlayer)),
            ),
        )
        .with_source_zones(&[ZoneKind::Exile]),
    ]),
);

// TSP 100 — Cyclopean Giant
pub(in crate::card::sets) static CYCLOPEAN_GIANT: CardRecord = CardRecord::new(
    "Cyclopean Giant",
    "fab6bc89-492a-46a4-90dc-95ae2e2bc483",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Zombie", "Giant"], 4, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature dies, target land becomes a Swamp. Exile this card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::set_basic_land_types(&[BasicLandType::Swamp]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ]),
        ),
    ]),
);

// TSP 101 — Dark Withering
// Audit: unsupported — Needs madness's discard-to-exile replacement and optional cast trigger.
pub(in crate::card::sets) static DARK_WITHERING: CardRecord = CardRecord::new(
    "Dark Withering",
    "3da58e0d-5877-43c4-b129-993e154b6087",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// TSP 102 — Deathspore Thallid
pub(in crate::card::sets) static DEATHSPORE_THALLID: CardRecord = CardRecord::new(
    "Deathspore Thallid",
    "44ee5ee3-11f8-4a4e-bfa3-10ff45ed6d1b",
    "Randy Elliott",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Fungus"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice a Saproling: Target creature gets -1/-1 until end of turn.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saproling")),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
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
    ]),
);

// TSP 103 — Demonic Collusion
pub(in crate::card::sets) static DEMONIC_COLLUSION: CardRecord = CardRecord::new(
    "Demonic Collusion",
    "f1a68a28-05fa-4c6e-896d-ddb9aea3a38d",
    "Jim Nelson",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_abilities(&[
        abilities::buyback(&[CostDef::DiscardCards(2)]),
        AbilityDef::spell(
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
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// TSP 104 — Dread Return
pub(in crate::card::sets) static DREAD_RETURN: CardRecord = CardRecord::new(
    "Dread Return",
    "d7e304fc-0ace-459e-8d2f-376f1899639c",
    "Kev Walker",
    // The flashback costs no mana at all, which is why the card is about
    // having three expendable bodies rather than about having four lands.
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
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
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
        AbilityDef::alternative_cast(
            &[CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(3),
            )],
            AlternativeCastKindDef::Flashback,
            Some("Flashback—Sacrifice three creatures."),
            EffectDef::None,
        ),
    ]),
);

// TSP 105 — Drudge Reavers
pub(in crate::card::sets) static DRUDGE_REAVERS: CardRecord = CardRecord::new(
    "Drudge Reavers",
    "03c07d9a-afed-4028-9fa1-ec439b60f08f",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Skeleton"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// TSP 106 — Endrek Sahr, Master Breeder
pub(in crate::card::sets) static ENDREK_SAHR_MASTER_BREEDER: CardRecord = CardRecord::new(
    "Endrek Sahr, Master Breeder",
    "dac6faba-59f2-436f-addb-c53db2b0cc17",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Human", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a creature spell, create X 1/1 black Thrull creature tokens, where X is that spell's mana value.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::create_creature_token(&["Thrull"], &[ManaColor::Black], 1, 1)
                    .with_count(ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::One(ObjectRefDef::TriggeringObject),
                        select: ObjectValueDef::ManaValue,
                        operation: AggregateOperationDef::Sum,
                    })),
            ),
            AbilityDef::triggered_if(
                "When you control seven or more Thrulls, sacrifice Endrek Sahr.",
                TriggerEventDef::StateCondition,
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Thrull")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                EffectDef::sacrifice_yours(EffectRecipientDef::Source),
            ),
        ]),
);

// TSP 107 — Evil Eye of Urborg
pub(in crate::card::sets) static EVIL_EYE_OF_URBORG: CardRecord = CardRecord::new(
    "Evil Eye of Urborg",
    "fc17fe31-11f1-48e0-9b0d-7c7dcda417a6",
    "Clint Langley",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Eye"], 6, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Non-Eye creatures you control can't attack.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Eye",
                        ))),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a creature, destroy that creature.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::Any,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::TriggeringObject,
                then: None,
            },
        ),
    ]),
);

// TSP 108 — Faceless Devourer
pub(in crate::card::sets) static FACELESS_DEVOURER: CardRecord = CardRecord::new(
    "Faceless Devourer",
    "d17146f8-d517-4941-b01e-09ecf00ec8e4",
    "Chippy",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Nightmare", "Horror"], 2, 1).with_abilities(
        &[
            abilities::shadow(),
            AbilityDef::triggered_with_targets(
                "When this creature enters, exile another target creature with shadow.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
            AbilityDef::triggered(
                "When this creature leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ),
        ],
    ),
);

// TSP 109 — Fallen Ideal
pub(in crate::card::sets) static FALLEN_IDEAL: CardRecord = CardRecord::new(
"Fallen Ideal",
"6b2d24b0-f0d3-40f0-a51f-074f8e95e6af",
"Anson Maddocks",
CardRules::new_enchantment(mana_cost!("{2}{B}")).with_subtypes(&const { ["Aura"] }).with_abilities(&const { [abilities::enchant_creature(),AbilityDef::static_ability("Enchanted creature has flying and \"Sacrifice a creature: This creature gets +2/+1 until end of turn.\"",EffectDef::StaticApply{recipient:EffectRecipientDef::AttachedPermanent,effect:AppliedEffectDef::Composite(&const { [AppliedEffectDef::add_ability(&const {abilities::flying()}),AppliedEffectDef::add_ability(&const {AbilityDef::activated("Sacrifice a creature: This creature gets +2/+1 until end of turn.",&const { [CostDef::SacrificePermanent{object:ObjectPredicateDef::HasType(CardType::Creature),controller:PlayerRelation::You}] },EffectDef::Apply{recipient:EffectRecipientDef::Source,effect:AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2),ValueDef::Constant(1)),duration:ResolvedEffectDurationDef::UntilEndOfTurn})})] })}),AbilityDef::triggered("When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",TriggerEventDef::zone_changed(ObjectPredicateDef::Source,Some(ZoneKind::Battlefield),Some(ZoneKind::Graveyard)),EffectDef::move_to_zone(EffectRecipientDef::Source,ZoneKind::Hand,ZonePlacement::Top))] }),
);

// TSP 110 — Feebleness
pub(in crate::card::sets) static FEEBLENESS: CardRecord = CardRecord::new(
    "Feebleness",
    "1ba2660d-b661-4266-b7e5-07bb8b72bce6",
    "Kev Walker",
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
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

// TSP 111 — Gorgon Recluse
// Audit: unsupported — Needs madness's discard-to-exile replacement and optional cast trigger in addition to the delayed destruction.
pub(in crate::card::sets) static GORGON_RECLUSE: CardRecord = CardRecord::new(
    "Gorgon Recluse",
    "abdeb7c9-98bd-4143-87e4-1c78c0ce627e",
    "Darrell Riche",
    CardRules::unsupported(),
);

// TSP 112 — Haunting Hymn
// Audit: unsupported — Needs cast-time own-main-phase provenance to choose the discard amount.
pub(in crate::card::sets) static HAUNTING_HYMN: CardRecord = CardRecord::new(
    "Haunting Hymn",
    "e8163a9d-5a1b-4d36-b20a-b220207a3b94",
    "rk post",
    CardRules::unsupported(),
);

// TSP 113 — Liege of the Pit
pub(in crate::card::sets) static LIEGE_OF_THE_PIT: CardRecord = CardRecord::new(
    "Liege of the Pit",
    "52e7c608-d224-472b-8736-273874211f24",
    "Jeremy Jarvis",
    CardRules::new_creature(mana_cost!("{5}{B}{B}{B}"), &["Demon"], 7, 7)
        .with_morph(&[CostDef::Mana(mana_cost!("{B}{B}{B}{B}"))])
        .with_abilities(&[
            abilities::flying(),
            abilities::trample(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, sacrifice a creature other than this creature. If you can't, this creature deals 7 damage to you.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: Some(&EffectDef::damage(
                        EffectRecipientDef::Controller,
                        ValueDef::Constant(7),
                    )),
                    optional: false,
                },
            ),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                Some("Morph {B}{B}{B}{B}"),
                EffectDef::None,
            ),
        ]),
);

// TSP 114 — Lim-Dûl the Necromancer
pub(in crate::card::sets) static LIM_D_L_THE_NECROMANCER: CardRecord = CardRecord::new(
    "Lim-Dûl the Necromancer",
    "b8a3cdfe-0289-474b-b9c4-07e8c6588ec5",
    "Matt Cavotta",
    CardRules::new_creature(mana_cost!("{5}{B}{B}"), &["Human", "Wizard"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever a creature an opponent controls dies, you may pay {1}{B}. If you do, return that card to the battlefield under your control. If it's a creature, it's a Zombie in addition to its other creature types.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::Mana(mana_cost!("{1}{B}"))],
                    &EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::WithBattlefieldArrival {
                            arrival: BattlefieldArrivalDef {
                                controller: Some(PlayerRelation::You),
                                ..BattlefieldArrivalDef::DEFAULT
                            },
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::TriggeringZoneChangeResult,
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                        },
                        binding: ParentBinding,
                        then: &EffectDef::IfCondition {
                            condition: &TriggerConditionDef::ObjectSetCount(
                                &ObjectSetCountConditionDef {
                                    objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                        ParentBinding,
                                    ),
                                    predicate: ObjectSetPredicateDef::contains(
                                        &ObjectPredicateDef::HasType(CardType::Creature),
                                    ),
                                },
                            ),
                            then: &EffectDef::Apply {
                                recipient: EffectRecipientDef::objects(
                                    ObjectSetDef::ZoneChangeSuccessorsOfBinding(ParentBinding),
                                ),
                                effect: AppliedEffectDef::add_creature_types(
                                    CreatureTypeSetDef::named(&["Zombie"]),
                                ),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        },
                    },
                )),
            ),
            AbilityDef::activated_with_targets(
                "{1}{B}: Regenerate target Zombie.",
                &[CostDef::Mana(mana_cost!("{1}{B}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// TSP 115 — Living End
pub(in crate::card::sets) static LIVING_END: CardRecord = CardRecord::new(
    "Living End",
    "3be0ff69-d9f3-4b81-b02f-1360e4064aff",
    "Greg Staples",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .printed_colors(&[ManaColor::Black])
    .with_abilities(&[
        AbilityDef::spell(
            "Each player exiles all creature cards from their graveyard, then sacrifices all creatures they control, then puts all cards they exiled this way onto the battlefield.",
            EffectDef::MoveObjects(MoveObjectsDef {
                input: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Any,
                )),
                from: Some(ZoneKind::Graveyard),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                moved: Some(ParentBinding),
                then: &EffectDef::Sequence(&[
                    EffectDef::sacrifice(EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                ]),
            }),
        ),
        abilities::suspend(
            "Suspend 3—{2}{B}{B}",
            &SuspendAbilityDef::fixed(3, &[CostDef::Mana(mana_cost!("{2}{B}{B}"))]),
        ),
    ]),
);

// TSP 116 — Magus of the Mirror
// Audit: unsupported — Needs atomic exchange of life totals with failure semantics; sequential SetLifeTotal effects are not an exchange.
pub(in crate::card::sets) static MAGUS_OF_THE_MIRROR: CardRecord = CardRecord::new(
    "Magus of the Mirror",
    "4a6e8ff6-323d-4cc3-b7b6-8607bf89597e",
    "Christopher Moeller",
    CardRules::unsupported(),
);

// TSP 117 — Mana Skimmer
pub(in crate::card::sets) static MANA_SKIMMER: CardRecord = CardRecord::new(
    "Mana Skimmer",
    "8a39ed44-bc0c-40e2-ba53-cd586c2d5877",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Leech"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature deals damage to a player, tap target land that player controls. That land doesn't untap during its controller's next untap step.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::EventPlayer),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    count: 1,
                },
            ]),
        ),
    ]),
);

// TSP 118 — Mindlash Sliver
pub(in crate::card::sets) static MINDLASH_SLIVER: CardRecord =
    CardRecord::new(
        "Mindlash Sliver",
        "f8c54575-dc1d-491c-a4f6-41f76eba2a2d",
        "Jeff Miracola",
        CardRules::new_creature(mana_cost!("{B}"), &const { ["Sliver"] }, 1, 1).with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "All Slivers have \"{1}, Sacrifice this permanent: Each player discards a card.\"",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                            &const { [ZoneKind::Battlefield] },
                            PlayerRelation::Any,
                        ),
                        effect: AppliedEffectDef::add_ability(
                            &const {
                                AbilityDef::activated(
                                    "{1}, Sacrifice this permanent: Each player discards a card.",
                                    &const {
                                        [CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource]
                                    },
                                    EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                                        player: EffectRecipientDef::EachPlayer,
                                        candidates: ObjectPredicateDef::Any,
                                        zone: ZoneKind::Hand,
                                        selection: PerPlayerSelectionDef::Count(
                                            ValueDef::Constant(1),
                                        ),
                                        visibility: ChoiceVisibilityDef::Private,
                                        chosen: Binding!("tsp_hand"),
                                        unchosen: Binding!("tsp_hand_rest"),
                                        then: &const {
                                            EffectDef::discard_cards(EffectRecipientDef::objects(
                                                ObjectSetDef::Binding(Binding!("tsp_hand")),
                                            ))
                                        },
                                    }),
                                )
                            },
                        ),
                    },
                )]
            },
        ),
    );

// TSP 119 — Mindstab
pub(in crate::card::sets) static MINDSTAB: CardRecord = CardRecord::new(
    "Mindstab",
    "3efd28ef-77db-4e7b-a69d-5a089e016737",
    "Mark Tedin",
    CardRules::new_sorcery(mana_cost!("{5}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player discards three cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
        abilities::suspend(
            "Suspend 4—{B}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{B}"))]),
        ),
    ]),
);

// TSP 120 — Nether Traitor
pub(in crate::card::sets) static NETHER_TRAITOR: CardRecord = CardRecord::new(
    "Nether Traitor",
    "911d2271-8998-4193-9d1a-1768c5dfaaad",
    "Vance Kovacs",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Spirit"], 1, 1).with_abilities(&[
        abilities::haste(),
        abilities::shadow(),
        AbilityDef::triggered(
            "Whenever another creature is put into your graveyard from the battlefield, you may pay {B}. If you do, return this card from your graveyard to the battlefield.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
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

// TSP 121 — Nightshade Assassin
// Audit: unsupported — Needs madness and a variable reveal payment of black cards that binds X to the enters trigger.
pub(in crate::card::sets) static NIGHTSHADE_ASSASSIN: CardRecord = CardRecord::new(
    "Nightshade Assassin",
    "2e57492c-ca26-4f75-a27e-5c54967534ea",
    "Alan Pollack",
    CardRules::unsupported(),
);

// TSP 122 — Phthisis
pub(in crate::card::sets) static PHTHISIS: CardRecord = CardRecord::new(
    "Phthisis",
    "9ba55f16-a37c-4caa-9417-227a06cf4061",
    "Carl Critchlow",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature. Its controller loses life equal to its power plus its toughness.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    amount: ValueDef::Sum(&SumValueDef::new(
                        ValueDef::TargetPower(TargetIndex::PRIMARY),
                        ValueDef::TargetToughness(TargetIndex::PRIMARY),
                    )),
                },
            ]),
        ),
        abilities::suspend(
            "Suspend 5—{1}{B}",
            &SuspendAbilityDef::fixed(5, &[CostDef::Mana(mana_cost!("{1}{B}"))]),
        ),
    ]),
);

// TSP 123 — Pit Keeper
pub(in crate::card::sets) static PIT_KEEPER: CardRecord = CardRecord::new(
    "Pit Keeper",
    "8936e767-0e48-4adb-93e9-790fe0cc19f2",
    "Anthony S. Waters",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Wizard"], 2, 1).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if you have four or more creature cards in your graveyard, you may return target creature card from your graveyard to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 4,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            },
        ),
    ]),
);

// TSP 124 — Plague Sliver
pub(in crate::card::sets) static PLAGUE_SLIVER: CardRecord = CardRecord::new(
    "Plague Sliver",
    "703d491e-b25f-44fa-8b76-b955a4e75ba5",
    "Dave Allsop",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Sliver"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "All Slivers have \"At the beginning of your upkeep, this permanent deals 1 damage to you.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(
                    &const {
                        AbilityDef::triggered(
                            "At the beginning of your upkeep, this permanent deals 1 damage to you.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::Upkeep,
                                player: PlayerRelation::You,
                            },
                            EffectDef::damage(
                                EffectRecipientDef::Controller,
                                ValueDef::Constant(1),
                            ),
                        )
                    },
                ),
            },
        ),
    ]),
);

// TSP 125 — Premature Burial
// Audit: unsupported — Needs per-object entry history measured from the end of this controller's previous turn.
pub(in crate::card::sets) static PREMATURE_BURIAL: CardRecord = CardRecord::new(
    "Premature Burial",
    "e96cea6a-fea6-4a6b-84b2-7b57237be96a",
    "Clint Langley",
    CardRules::unsupported(),
);

// TSP 126 — Psychotic Episode
// Audit: unsupported — Needs madness's discard-to-exile replacement and optional cast trigger.
pub(in crate::card::sets) static PSYCHOTIC_EPISODE: CardRecord = CardRecord::new(
    "Psychotic Episode",
    "5e61fbcd-b673-4d1d-bc38-a791a56a9aac",
    "Drew Tucker",
    CardRules::unsupported(),
);

// TSP 127 — Sangrophage
pub(in crate::card::sets) static SANGROPHAGE: CardRecord = CardRecord::new(
    "Sangrophage",
    "ab9abc52-76f6-4b1f-8602-2e5abdd23b6b",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Zombie"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, tap this creature unless you pay 2 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                &[CostDef::PayLife(2)],
                &EffectDef::Tap {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// TSP 128 — Sengir Nosferatu
pub(in crate::card::sets) static SENGIR_NOSFERATU: CardRecord = CardRecord::new(
    "Sengir Nosferatu",
    "3cd9cc4c-708b-41cb-9cac-fd6d8ce9922d",
    "Scott M. Fischer",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Vampire"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{B}, Exile this creature: Create a 1/2 black Bat creature token with flying. It has \"{1}{B}, Sacrifice this token: Return an exiled card named Sengir Nosferatu to the battlefield under its owner's control.\"",
            &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::ExileSource],
            EffectDef::create_creature_token(&["Bat"], &[ManaColor::Black], 1, 2).with_abilities(
                &[
                    abilities::flying(),
                    AbilityDef::activated(
                        "{1}{B}, Sacrifice this token: Return an exiled card named Sengir Nosferatu to the battlefield under its owner's control.",
                        &[
                            CostDef::Mana(mana_cost!("{1}{B}")),
                            CostDef::SacrificeSource,
                        ],
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::NameEquals(CardNameDef::Literal(
                                    "Sengir Nosferatu",
                                )),
                                &[ZoneKind::Exile],
                                PlayerRelation::Any,
                            )),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                        }),
                    ),
                ],
            ),
        ),
    ]),
);

// TSP 129 — Skittering Monstrosity
pub(in crate::card::sets) static SKITTERING_MONSTROSITY: CardRecord = CardRecord::new(
    "Skittering Monstrosity",
    "d0d21e39-b1dd-476d-86dc-4317858e4f8e",
    "rk post",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Horror"], 5, 5).with_abilities(&[
        AbilityDef::triggered(
            "When you cast a creature spell, sacrifice this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::sacrifice_yours(EffectRecipientDef::Source),
        ),
    ]),
);

// TSP 130 — Skulking Knight
pub(in crate::card::sets) static SKULKING_KNIGHT: CardRecord = CardRecord::new(
    "Skulking Knight",
    "a7f7927b-64ae-4448-9540-8d7bbe88c9cc",
    "rk post",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie", "Knight"], 3, 3).with_abilities(&[
        abilities::flanking(),
        AbilityDef::triggered(
            "When this creature becomes the target of a spell or ability, sacrifice it.",
            TriggerEventDef::becomes_targeted(ObjectPredicateDef::Any),
            EffectDef::sacrifice_yours(EffectRecipientDef::Source),
        ),
    ]),
);

// TSP 131 — Smallpox
pub(in crate::card::sets) static SMALLPOX: CardRecord = CardRecord::new(
    "Smallpox",
    "175d5a88-2597-4e85-aed6-7a65c0595fb4",
    "Janine Johnston",
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell(
        "Each player loses 1 life, discards a card, sacrifices a creature of their choice, then sacrifices a land of their choice.",
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(1),
            },
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                candidates: ObjectPredicateDef::Any,
                zone: ZoneKind::Hand,
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                visibility: ChoiceVisibilityDef::Private,
                chosen: Binding!("tsp_hand"),
                unchosen: Binding!("tsp_hand_rest"),
                then: &EffectDef::discard_cards(EffectRecipientDef::objects(
                    ObjectSetDef::Binding(Binding!("tsp_hand")),
                )),
            }),
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                candidates: ObjectPredicateDef::HasType(CardType::Creature),
                zone: ZoneKind::Battlefield,
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                visibility: ChoiceVisibilityDef::Public,
                chosen: Binding!("tsp_creatures"),
                unchosen: Binding!("tsp_creatures_rest"),
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    Binding!("tsp_creatures"),
                ))),
            }),
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                candidates: ObjectPredicateDef::HasType(CardType::Land),
                zone: ZoneKind::Battlefield,
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                visibility: ChoiceVisibilityDef::Public,
                chosen: Binding!("tsp_lands"),
                unchosen: Binding!("tsp_lands_rest"),
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    Binding!("tsp_lands"),
                ))),
            }),
        ]),
    )]),
);

// TSP 132 — Strangling Soot
pub(in crate::card::sets) static STRANGLING_SOOT: CardRecord = CardRecord::new(
    "Strangling Soot",
    "6723e552-baf5-4b6a-8af6-843fd8597f6c",
    "Jim Murray",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature with toughness 3 or less.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(4)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{R}"))]),
    ]),
);

// TSP 133 — Stronghold Overseer
pub(in crate::card::sets) static STRONGHOLD_OVERSEER: CardRecord = CardRecord::new(
    "Stronghold Overseer",
    "e0722fa2-53df-4217-a592-fcaf239f717a",
    "Puddnhead",
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Demon"], 5, 5).with_abilities(&[
        abilities::flying(),
        abilities::shadow(),
        AbilityDef::activated(
            "{B}{B}: Creatures with shadow get +1/+0 until end of turn and creatures without shadow get -1/-0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}{B}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Shadow),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                KeywordAbility::Shadow,
                            )),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// TSP 134 — Sudden Death
pub(in crate::card::sets) static SUDDEN_DEATH: CardRecord = CardRecord::new(
    "Sudden Death",
    "16e889c4-105b-4f4e-8a3d-ace753a24a4e",
    "Dave Allsop",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Target creature gets -4/-4 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-4),
                    ValueDef::Constant(-4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 135 — Sudden Spoiling
pub(in crate::card::sets) static SUDDEN_SPOILING: CardRecord = CardRecord::new(
    "Sudden Spoiling",
    "9f89f1e4-4d55-48b8-bf81-54cfd34f4aa0",
    "Alan Pollack",
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Until end of turn, creatures target player controls lose all abilities and have base power and toughness 0/2.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects_controlled_by_target(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    TargetIndex::PRIMARY,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 136 — Tendrils of Corruption
pub(in crate::card::sets) static TENDRILS_OF_CORRUPTION: CardRecord = CardRecord::new(
    "Tendrils of Corruption",
    "7f61db9e-ef88-4dc8-b90c-1f8b2d7e9bb9",
    "Mike Dringenberg",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tendrils of Corruption deals X damage to target creature and you gain X life, where X is the number of Swamps you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        ]),
    )]),
);

// TSP 137 — Traitor's Clutch
pub(in crate::card::sets) static TRAITOR_S_CLUTCH: CardRecord = CardRecord::new(
    "Traitor's Clutch",
    "6313a601-5d26-487b-a70c-2c7184b7cc91",
    "Dave Allsop",
    CardRules::new_instant(mana_cost!("{4}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +1/+0, becomes black, and gains shadow until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
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
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Black])),
                    AppliedEffectDef::add_ability(&const { abilities::shadow() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{1}{B}"))]),
    ]),
);

// TSP 138 — Trespasser il-Vec
pub(in crate::card::sets) static TRESPASSER_IL_VEC: CardRecord = CardRecord::new(
    "Trespasser il-Vec",
    "4903171e-76b2-437d-8965-e871e47a3482",
    "Jim Murray",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 3, 1).with_abilities(&[
        AbilityDef::activated(
            "Discard a card: This creature gains shadow until end of turn.",
            &[CostDef::discard(ObjectPredicateDef::Any)],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::shadow() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 139 — Urborg Syphon-Mage
// Audit: unsupported — Needs a captured actual life-loss amount for the gain-life continuation, including effects that prohibit life changes.
pub(in crate::card::sets) static URBORG_SYPHON_MAGE: CardRecord = CardRecord::new(
    "Urborg Syphon-Mage",
    "0e8376f6-5c26-4b87-b226-960466e16cb0",
    "Greg Staples",
    CardRules::unsupported(),
);

// TSP 140 — Vampiric Sliver
pub(in crate::card::sets) static VAMPIRIC_SLIVER: CardRecord = CardRecord::new(
    "Vampiric Sliver",
    "28c56db1-bb2d-4383-90aa-72d00fe476b2",
    "Thomas M. Baxa",
    CardRules::new_creature(mana_cost!("{3}{B}"), &const { ["Sliver"] }, 3, 3).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Sliver creatures have \"Whenever a creature dealt damage by this creature this turn dies, put a +1/+1 counter on this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(
                            &const {
                                [
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                ]
                            },
                        ),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::triggered(
                                "Whenever a creature dealt damage by this creature this turn dies, put a +1/+1 counter on this creature.",
                                TriggerEventDef::ZoneChanged(
                                    ZoneChangeEventMatcherDef::new(
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        Some(ZoneKind::Battlefield),
                                        Some(ZoneKind::Graveyard),
                                    )
                                    .previously_damaged_by(ObjectRefDef::Source),
                                ),
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::Source,
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(1),
                                },
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 141 — Viscid Lemures
pub(in crate::card::sets) static VISCID_LEMURES: CardRecord = CardRecord::new(
    "Viscid Lemures",
    "863f04d7-da34-41e3-9153-97891a428889",
    "Drew Tucker",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Spirit"], 4, 3).with_abilities(&[
        AbilityDef::activated(
            "{0}: This creature gets -1/-0 and gains swampwalk until end of turn.",
            &[CostDef::Mana(mana_cost!("{0}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(
                        &const { abilities::landwalk(BasicLandType::Swamp) },
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 142 — Aetherflame Wall
// Audit: unsupported — Needs a blocking permission that ignores an attacker's shadow without granting shadow to the blocker.
pub(in crate::card::sets) static AETHERFLAME_WALL: CardRecord = CardRecord::new(
    "Aetherflame Wall",
    "93b14f08-dd93-4a9f-a616-8f8d0b26b966",
    "Justin Sweet",
    CardRules::unsupported(),
);

// TSP 143 — Ancient Grudge
pub(in crate::card::sets) static ANCIENT_GRUDGE: CardRecord = CardRecord::new(
    "Ancient Grudge",
    "89cbad1f-4f16-4d5f-a485-5bf950565216",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{G}"))]),
    ]),
);

// TSP 144 — Barbed Shocker
pub(in crate::card::sets) static BARBED_SHOCKER: CardRecord = CardRecord::new(
    "Barbed Shocker",
    "41b6f45a-7219-4b59-bf0a-9e374599ae5e",
    "Tony Szczudlo",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature deals damage to a player, that player discards all the cards in their hand, then draws that many cards.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
            EffectDef::Discard {
                recipient: EffectRecipientDef::player(PlayerRefDef::EventPlayer),
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::EventPlayer,
                )),
                selection: DiscardSelectionDef::RecipientChooses,
                then: Some(DiscardFollowUpDef {
                    counted: ObjectPredicateDef::Any,
                    bound: Some(ParentBinding),
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::player(PlayerRefDef::EventPlayer),
                        amount: ValueDef::CountObjects(&ObjectSetDef::Binding(ParentBinding)),
                    },
                }),
            },
        ),
    ]),
);

// TSP 145 — Basalt Gargoyle
pub(in crate::card::sets) static BASALT_GARGOYLE: CardRecord = CardRecord::new(
    "Basalt Gargoyle",
    "f34945e1-b982-48a0-8c03-9b97c13cc73d",
    "Clint Langley",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Gargoyle"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::echo("Echo {2}{R}", &[CostDef::Mana(mana_cost!("{2}{R}"))]),
        abilities::apply_to_self_until_end_of_turn(
            "{R}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
        ),
    ]),
);

// TSP 146 — Blazing Blade Askari
pub(in crate::card::sets) static BLAZING_BLADE_ASKARI: CardRecord = CardRecord::new(
    "Blazing Blade Askari",
    "cabf35d5-de8a-4d9d-be59-7ad7039873c6",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::flanking(),
        AbilityDef::activated(
            "{2}: This creature becomes colorless until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_colors(ColorSet::empty()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 147 — Bogardan Hellkite
pub(in crate::card::sets) static BOGARDAN_HELLKITE: CardRecord = CardRecord::new(
    "Bogardan Hellkite",
    "5852f528-92aa-45cf-8436-5774691676d3",
    "Scott M. Fischer",
    CardRules::new_creature(mana_cost!("{6}{R}{R}"), &["Dragon"], 5, 5).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, it deals 5 damage divided as you choose among any number of targets.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                minimum: 0,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::Fixed(5)),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::DividedAmongTargets,
            ),
        ),
    ]),
);

// TSP 148 — Bogardan Rager
pub(in crate::card::sets) static BOGARDAN_RAGER: CardRecord = CardRecord::new(
    "Bogardan Rager",
    "ec53d26b-ad3e-474f-a374-05fcdc00e49c",
    "Clint Langley",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Elemental"], 3, 4).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gets +4/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
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
        ),
    ]),
);

// TSP 149 — Bonesplitter Sliver
pub(in crate::card::sets) static BONESPLITTER_SLIVER: CardRecord = CardRecord::new(
    "Bonesplitter Sliver",
    "705e29c5-2d9b-44ad-a04c-9a62dd74eb12",
    "Dany Orizio",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Sliver"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures get +2/+0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ]),
);

// TSP 150 — Coal Stoker
pub(in crate::card::sets) static COAL_STOKER: CardRecord = CardRecord::new(
    "Coal Stoker",
    "3a6bdc34-1ef6-4de3-a9a9-b5dd503e02c0",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 3, 3).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if you cast it from your hand, add {R}{R}{R}.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourceCastFrom(ZoneKind::Hand),
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(3)),
        ),
    ]),
);

// TSP 151 — Conflagrate
pub(in crate::card::sets) static CONFLAGRATE: CardRecord = CardRecord::new(
    "Conflagrate",
    "efc5f7fb-b7a6-433d-a0bb-240c7aa0a720",
    "Warren Mahy",
    CardRules::new_sorcery(mana_cost!("{X}{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Conflagrate deals X damage divided as you choose among any number of targets.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                minimum: 0,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::ChosenX),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::DividedAmongTargets,
            ),
        ),
        abilities::flashback(&[
            CostDef::Mana(mana_cost!("{R}{R}")),
            CostDef::discard(ObjectPredicateDef::Any).with_quantity(CostQuantityDef::ChosenX),
        ]),
    ]),
);

// TSP 152 — Empty the Warrens
pub(in crate::card::sets) static EMPTY_THE_WARRENS: CardRecord = CardRecord::new(
    "Empty the Warrens",
    "952bb27c-c58a-478a-b637-eb4f7e1e0ab4",
    "Mark Brill",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 1/1 red Goblin creature tokens.",
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
                .with_count(ValueDef::Constant(2)),
        ),
        abilities::storm(),
    ]),
);

// TSP 153 — Firemaw Kavu
pub(in crate::card::sets) static FIREMAW_KAVU: CardRecord = CardRecord::new(
    "Firemaw Kavu",
    "d592bc85-41ca-48a4-b35a-4d86c8e76d54",
    "Greg Hildebrandt",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Kavu"], 4, 2).with_abilities(&[
        abilities::echo("Echo {5}{R}", &[CostDef::Mana(mana_cost!("{5}{R}"))]),
        AbilityDef::triggered_with_targets(
            "When this creature enters, it deals 2 damage to target creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
        AbilityDef::triggered_with_targets(
            "When this creature leaves the battlefield, it deals 4 damage to target creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// TSP 154 — Flamecore Elemental
pub(in crate::card::sets) static FLAMECORE_ELEMENTAL: CardRecord = CardRecord::new(
    "Flamecore Elemental",
    "940bfe4f-bc18-4506-b6f4-13d339fd3e55",
    "Dave Dorman",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Elemental"], 5, 4).with_abilities(&[
        abilities::echo("Echo {2}{R}{R}", &[CostDef::Mana(mana_cost!("{2}{R}{R}"))]),
    ]),
);

// TSP 155 — Flowstone Channeler
pub(in crate::card::sets) static FLOWSTONE_CHANNELER: CardRecord = CardRecord::new(
    "Flowstone Channeler",
    "9f10fa1c-2c7b-49ba-87b6-3b652b65704d",
    "Alan Pollack",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Spellshaper"], 2, 2).with_abilities(
        &[AbilityDef::activated_with_targets(
            "{1}{R}, {T}, Discard a card: Target creature gets +1/-1 and gains haste until end of turn.",
            &[
                CostDef::Mana(mana_cost!("{1}{R}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(-1),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::haste() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )],
    ),
);

// TSP 156 — Fortune Thief
pub(in crate::card::sets) static FORTUNE_THIEF: CardRecord = CardRecord::new(
    "Fortune Thief",
    "c893bbb7-0589-4544-a488-f84f9aa1d058",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Rogue"], 0, 1)
        .with_morph(&[CostDef::Mana(mana_cost!("{R}{R}"))])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Damage that would reduce your life total to less than 1 reduces it to 1 instead.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage {
                        matcher: DamageEventMatcherDef::ANY,
                        limit: DamageLimitDef::LeaveAtLeastLife(1),
                    }),
                },
            ),
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                Some("Morph {R}{R}"),
                EffectDef::None,
            ),
        ]),
);

// TSP 157 — Fury Sliver
pub(in crate::card::sets) static FURY_SLIVER: CardRecord = CardRecord::new(
    "Fury Sliver",
    "0000579f-7b35-4ed3-b44c-db2a538066fe",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Sliver"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures have double strike.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::double_strike() }),
            },
        ),
    ]),
);

// TSP 158 — Ghitu Firebreathing
pub(in crate::card::sets) static GHITU_FIREBREATHING: CardRecord = CardRecord::new(
    "Ghitu Firebreathing",
    "9098b989-072a-4e5b-8972-22c61645c5bb",
    "Jim Nelson",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/+0 until end of turn.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{R}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TSP 159 — Goblin Skycutter
pub(in crate::card::sets) static GOBLIN_SKYCUTTER: CardRecord = CardRecord::new(
    "Goblin Skycutter",
    "c45406cd-a036-45d5-a0fb-3c195d40d746",
    "Clint Langley",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: It deals 2 damage to target creature with flying. That creature loses flying until end of turn.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Flying,
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// TSP 160 — Grapeshot
pub(in crate::card::sets) static GRAPESHOT: CardRecord = CardRecord::new(
    "Grapeshot",
    "4ee33cb6-768e-44a0-b6f4-b8638aa84330",
    "Pete Venters",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Grapeshot deals 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
        abilities::storm(),
    ]),
);

// TSP 161 — Greater Gargadon
pub(in crate::card::sets) static GREATER_GARGADON: CardRecord = CardRecord::new(
    "Greater Gargadon",
    "653ddfa0-2088-4503-a3ab-b0f1d55d8351",
    "Rob Alexander",
    CardRules::new_creature(mana_cost!("{9}{R}"), &["Beast"], 9, 7).with_abilities(&[
        abilities::suspend(
            "Suspend 10—{R}",
            &crate::card::SuspendAbilityDef::fixed(10, &[CostDef::Mana(mana_cost!("{R}"))]),
        ),
        AbilityDef::activated(
            "Sacrifice an artifact, creature, or land: Remove a time counter from this card. Activate only if this card is suspended.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("time"),
                amount: ValueDef::Constant(1),
            },
        )
        .with_source_zones(&[ZoneKind::Exile])
        .with_activation_condition(&abilities::SUSPEND_SOURCE_IS_SUSPENDED),
    ]),
);

// TSP 162 — Ground Rift
pub(in crate::card::sets) static GROUND_RIFT: CardRecord = CardRecord::new(
    "Ground Rift",
    "62333783-6a18-4461-88ce-1c37eaf64e2b",
    "Thomas M. Baxa",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature without flying can't block this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::storm(),
    ]),
);

// TSP 163 — Ib Halfheart, Goblin Tactician
// Audit: unsupported — Needs sacrifice-success continuation retaining the complete blocking group and the sacrificed attacker's damage-source identity.
pub(in crate::card::sets) static IB_HALFHEART_GOBLIN_TACTICIAN: CardRecord = CardRecord::new(
    "Ib Halfheart, Goblin Tactician",
    "38134389-b471-4f58-a9ae-26bf9dc1557a",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// TSP 164 — Ignite Memories
pub(in crate::card::sets) static IGNITE_MEMORIES: CardRecord = CardRecord::new(
    "Ignite Memories",
    "2f7b7831-27a1-4c0a-8ed1-6dddf2754d65",
    "Justin Sweet",
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player reveals a card at random from their hand. Ignite Memories deals damage to that player equal to that card's mana value.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    binding: Binding!("tsp_revealed"),
                    effect: &EffectDef::RevealAtRandomFromHand {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                },
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                        objects: ObjectSetDef::Binding(Binding!("tsp_revealed")),
                        select: ObjectValueDef::ManaValue,
                        operation: AggregateOperationDef::Sum,
                    }),
                ),
            ]),
        ),
        abilities::storm(),
    ]),
);

// TSP 165 — Ironclaw Buzzardiers
pub(in crate::card::sets) static IRONCLAW_BUZZARDIERS: CardRecord = CardRecord::new(
    "Ironclaw Buzzardiers",
    "b07e37ea-aa8d-4443-9af6-4351994bc00c",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Orc", "Scout"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block creatures with power 2 or greater.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(2)),
                )),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 166 — Jaya Ballard, Task Mage
pub(in crate::card::sets) static JAYA_BALLARD_TASK_MAGE: CardRecord = CardRecord::new(
    "Jaya Ballard, Task Mage",
    "a1f85b2a-7836-4058-87a5-859c4311a7e4",
    "Matt Cavotta",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Spellshaper"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{R}, {T}, Discard a card: Destroy target blue permanent.",
                &[
                    CostDef::Mana(mana_cost!("{R}")),
                    CostDef::TapSource,
                    CostDef::discard(ObjectPredicateDef::Any),
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Color(ManaColor::Blue),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::activated_with_targets(
                "{1}{R}, {T}, Discard a card: Jaya Ballard deals 3 damage to any target. A creature dealt damage this way can't be regenerated this turn.",
                &[
                    CostDef::Mana(mana_cost!("{1}{R}")),
                    CostDef::TapSource,
                    CostDef::discard(ObjectPredicateDef::Any),
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::DealDamage(
                    DamageDef::new(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(3),
                    )
                    .with_follow_up(DamageFollowUpDef::ApplyToDamaged {
                        effect: &AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    }),
                ),
            ),
            AbilityDef::activated(
                "{5}{R}{R}, {T}, Discard a card: Jaya Ballard deals 6 damage to each creature and each player.",
                &[
                    CostDef::Mana(mana_cost!("{5}{R}{R}")),
                    CostDef::TapSource,
                    CostDef::discard(ObjectPredicateDef::Any),
                ],
                EffectDef::damage_simultaneously(&[
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        ValueDef::Constant(6),
                    ),
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::EachPlayer,
                        ValueDef::Constant(6),
                    ),
                ]),
            ),
        ]),
);

// TSP 167 — Keldon Halberdier
pub(in crate::card::sets) static KELDON_HALBERDIER: CardRecord = CardRecord::new(
    "Keldon Halberdier",
    "a90723e0-fbb3-4976-9463-0373f8ed337c",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Warrior"], 4, 1).with_abilities(&[
        abilities::first_strike(),
        abilities::suspend(
            "Suspend 4—{R}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{R}"))]),
        ),
    ]),
);

// TSP 168 — Lightning Axe
pub(in crate::card::sets) static LIGHTNING_AXE: CardRecord = CardRecord::new(
    "Lightning Axe",
    "1b748290-04b0-48a9-81aa-ab43182cf339",
    "Dan Murayama Scott",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "As an additional cost to cast this spell, discard a card or pay {5}.\nLightning Axe deals 5 damage to target creature.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
        )
        .with_spell_additional_cost(&CostDef::choice(&[
            CostDef::DiscardCards(1),
            CostDef::Mana(mana_cost!("{5}")),
        ])),
    ),
);

// TSP 169 — Magus of the Scroll
pub(in crate::card::sets) static MAGUS_OF_THE_SCROLL: CardRecord = CardRecord::new(
    "Magus of the Scroll",
    "6060cada-52bf-4ef2-8a26-8ba04dded458",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Wizard"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}, {T}: Choose a card name, then reveal a card at random from your hand. If that card has the chosen name, this creature deals 2 damage to any target.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    binding: Binding!("tsp_named"),
                    effect: &EffectDef::ChooseCardName {
                        chooser: PlayerRefDef::EffectController,
                        names: CardNameSetDef::AllCardNames,
                    },
                },
                EffectDef::BindOutput {
                    binding: Binding!("tsp_revealed"),
                    effect: &EffectDef::RevealAtRandomFromHand {
                        player: EffectRecipientDef::Controller,
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::Binding(Binding!("tsp_revealed")),
                        predicate: ObjectSetPredicateDef::contains(
                            &ObjectPredicateDef::NameEquals(CardNameDef::Binding(Binding!(
                                "tsp_named"
                            ))),
                        ),
                    }),
                    then: &EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(2),
                    ),
                },
            ]),
        ),
    ]),
);

// TSP 170 — Mogg War Marshal
pub(in crate::card::sets) static MOGG_WAR_MARSHAL: CardRecord = CardRecord::new(
    "Mogg War Marshal",
    "8b9e0bdb-b615-447a-b80d-d7244c25c56e",
    "Wayne England",
    // Letting the echo go unpaid is the normal line: three bodies for two
    // mana, and the last one arrives because the Marshal died.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        abilities::echo("Echo {1}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)", &[CostDef::Mana(mana_cost!("{1}{R}"))]),
        AbilityDef::triggered(
            "When this creature enters or dies, create a 1/1 red Goblin creature token.",
            // One printed sentence with two ways in, so it is one ability
            // watching both zone changes rather than two abilities.
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
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1),
        ),
    ]),
);

// TSP 171 — Norin the Wary
pub(in crate::card::sets) static NORIN_THE_WARY: CardRecord = CardRecord::new(
    "Norin the Wary",
    "f61ea59a-1db0-4e6b-bcde-19787c76a49b",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Warrior"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "When a player casts a spell or a creature attacks, exile Norin. Return it to the battlefield under its owner's control at the beginning of the next end step.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::spell_cast(ObjectPredicateDef::Any),
                TriggerEventDef::attacks(ObjectPredicateDef::Any),
            ]),
            abilities::exile_until_next_end_step(EffectRecipientDef::Source),
        )]),
);

// TSP 172 — Orcish Cannonade
pub(in crate::card::sets) static ORCISH_CANNONADE: CardRecord = CardRecord::new(
    "Orcish Cannonade",
    "4e40f99c-9608-4463-8c6f-c6e142f0d716",
    "Pete Venters",
    CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Orcish Cannonade deals 2 damage to any target and 3 damage to you.\nDraw a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage_simultaneously(&[
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(2),
                    ),
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::Controller,
                        ValueDef::Constant(3),
                    ),
                ]),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// TSP 173 — Pardic Dragon
pub(in crate::card::sets) static PARDIC_DRAGON: CardRecord = CardRecord::new(
    "Pardic Dragon",
    "ad47d954-dda8-412a-b69b-a213bbd2f309",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::apply_to_self_until_end_of_turn(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
        ),
        abilities::suspend(
            "Suspend 2—{R}{R}",
            &SuspendAbilityDef::fixed(2, &[CostDef::Mana(mana_cost!("{R}{R}"))]),
        ),
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell, if this card is suspended, that player may put a time counter on this card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &abilities::SUSPEND_SOURCE_IS_SUSPENDED,
            EffectDef::May {
                player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::TriggeringObject,
                )),
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("time"),
                    amount: ValueDef::Constant(1),
                },
            },
        )
        .with_source_zones(&[ZoneKind::Exile]),
    ]),
);

// TSP 174 — Plunder
pub(in crate::card::sets) static PLUNDER: CardRecord = CardRecord::new(
    "Plunder",
    "79cff220-b1a6-4da6-b765-25a583b33de2",
    "Thomas M. Baxa",
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact or land.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
        abilities::suspend(
            "Suspend 4—{1}{R}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{1}{R}"))]),
        ),
    ]),
);

// TSP 175 — Reiterate
pub(in crate::card::sets) static REITERATE: CardRecord = CardRecord::new(
    "Reiterate",
    "4bc2151b-acec-4237-8f6f-ed97055f3bb9",
    "Dan Murayama Scott",
    CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{3}"))]),
        AbilityDef::spell_with_targets(
            "Copy target instant or sorcery spell. You may choose new targets for the copy.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
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

// TSP 176 — Rift Bolt
pub(in crate::card::sets) static RIFT_BOLT: CardRecord = CardRecord::new(
    "Rift Bolt",
    "88dde96e-6824-4d26-9fb5-86b9f3c50959",
    "Michael Sutfin",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        abilities::suspend(
            "Suspend 1—{R}",
            &crate::card::SuspendAbilityDef::fixed(1, &[CostDef::Mana(mana_cost!("{R}"))]),
        ),
        AbilityDef::spell_with_targets(
            "Rift Bolt deals 3 damage to any target.",
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

// TSP 177 — Sedge Sliver
// Audit: unsupported — Grant validation cannot attach executable static abilities to Slivers; the Swamp condition must be evaluated for each granted ability's controller.
pub(in crate::card::sets) static SEDGE_SLIVER: CardRecord = CardRecord::new(
    "Sedge Sliver",
    "18dc8d7e-745c-46ba-842c-526f1beb89a7",
    "Richard Kane Ferguson",
    CardRules::unsupported(),
);

// TSP 178 — Subterranean Shambler
pub(in crate::card::sets) static SUBTERRANEAN_SHAMBLER: CardRecord = CardRecord::new(
    "Subterranean Shambler",
    "f9429ec0-ff23-494d-ab1c-b88f3a0dee1b",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Elemental"], 2, 3).with_abilities(&[
        abilities::echo("Echo {3}{R}", &[CostDef::Mana(mana_cost!("{3}{R}"))]),
        AbilityDef::triggered(
            "When this creature enters or leaves the battlefield, it deals 1 damage to each creature without flying.",
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
            EffectDef::damage(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// TSP 179 — Sudden Shock
pub(in crate::card::sets) static SUDDEN_SHOCK: CardRecord = CardRecord::new(
    "Sudden Shock",
    "7d2691ee-8eec-437d-9bc1-113264525fb8",
    "Vance Kovacs",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Sudden Shock deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// TSP 180 — Sulfurous Blast
// Audit: unsupported — Needs a "cast during your main phase" condition. The closest is SourceCastAtInstantSpeed, and its negation is not the same question: a spell cast in your own main phase in response to something was cast when a sorcery could not have been, so the negation would report the smaller amount where the printed card gives the larger one.
pub(in crate::card::sets) static SULFUROUS_BLAST: CardRecord = CardRecord::new(
    "Sulfurous Blast",
    "67511e0e-be09-4f4e-9949-b9ecbdc7f536",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// TSP 181 — Tectonic Fiend
pub(in crate::card::sets) static TECTONIC_FIEND: CardRecord = CardRecord::new(
    "Tectonic Fiend",
    "9c8aaf09-312f-4c82-9e0a-bd797e7f26c3",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Elemental"], 7, 7).with_abilities(&[
        abilities::echo("Echo {4}{R}{R}", &[CostDef::Mana(mana_cost!("{4}{R}{R}"))]),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// TSP 182 — Thick-Skinned Goblin
// Audit: unsupported — Needs a player-scoped substitution of {0} for another permanent's labeled echo payment.
pub(in crate::card::sets) static THICK_SKINNED_GOBLIN: CardRecord = CardRecord::new(
    "Thick-Skinned Goblin",
    "5b1a82da-ae5c-4f3d-8aa0-4bd3de7bf0b6",
    "Ralph Horsley",
    CardRules::unsupported(),
);

// TSP 183 — Two-Headed Sliver
pub(in crate::card::sets) static TWO_HEADED_SLIVER: CardRecord = CardRecord::new(
    "Two-Headed Sliver",
    "2f89fb3b-0238-4d76-a46d-7d6fa4a74620",
    "Dany Orizio",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Sliver"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures have menace.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::menace() }),
            },
        ),
    ]),
);

// TSP 184 — Undying Rage
pub(in crate::card::sets) static UNDYING_RAGE: CardRecord = CardRecord::new(
    "Undying Rage",
    "4f7ff7d2-a8f1-4ce4-acad-828ef36afe07",
    "Scott M. Fischer",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TSP 185 — Viashino Bladescout
pub(in crate::card::sets) static VIASHINO_BLADESCOUT: CardRecord = CardRecord::new(
    "Viashino Bladescout",
    "41fce1b3-0961-4672-845f-e1c6ce101c1b",
    "Dany Orizio",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Lizard", "Scout"], 2, 1).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gains first strike until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 186 — Volcanic Awakening
pub(in crate::card::sets) static VOLCANIC_AWAKENING: CardRecord = CardRecord::new(
    "Volcanic Awakening",
    "aebd5c57-cfc8-4a3c-b4a2-0cd64a5e3575",
    "Dan Murayama Scott",
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
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
                then: None,
            },
        ),
        abilities::storm(),
    ]),
);

// TSP 187 — Wheel of Fate
pub(in crate::card::sets) static WHEEL_OF_FATE: CardRecord = CardRecord::new(
    "Wheel of Fate",
    "0d9eb9fc-0a69-480d-be17-3e80e34c453b",
    "Kev Walker",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .printed_colors(&[ManaColor::Red])
    .with_abilities(&[
        abilities::suspend(
            "Suspend 4—{1}{R}",
            &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{1}{R}"))]),
        ),
        AbilityDef::spell(
            "Each player discards their hand, then draws seven cards.",
            EffectDef::Sequence(&[
                EffectDef::discard_cards(EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::Any,
                )),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(7),
                },
            ]),
        ),
    ]),
);

// TSP 188 — Word of Seizing
pub(in crate::card::sets) static WORD_OF_SEIZING: CardRecord = CardRecord::new(
    "Word of Seizing",
    "d6db3c98-ec84-4686-bc21-c8572a736e62",
    "Vance Kovacs",
    CardRules::new_instant(mana_cost!("{3}{R}{R}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Untap target permanent and gain control of it until end of turn. It gains haste until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::gain_control(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    PlayerRefDef::EffectController,
                    ControlDurationDef::UntilEndOfTurn,
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// TSP 189 — Aether Web
// Audit: unsupported — Needs a blocking permission that ignores an attacker's shadow without granting shadow to the enchanted creature.
pub(in crate::card::sets) static AETHER_WEB: CardRecord = CardRecord::new(
    "Aether Web",
    "ce9a6af9-1b39-41e1-bd71-210700b7608b",
    "Justin Sweet",
    CardRules::unsupported(),
);

// TSP 190 — Ashcoat Bear
pub(in crate::card::sets) static ASHCOAT_BEAR: CardRecord = CardRecord::new(
    "Ashcoat Bear",
    "9b7a6ab5-8a8f-492c-8484-3089354ce8cf",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2)
        .with_abilities(&[abilities::flash()]),
);

// TSP 191 — Aspect of Mongoose
pub(in crate::card::sets) static ASPECT_OF_MONGOOSE: CardRecord = CardRecord::new(
    "Aspect of Mongoose",
    "b3eb61f4-627b-4f42-85aa-eb676a035fbd",
    "Dave Dorman",
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has shroud.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&const { abilities::shroud() }),
                },
            ),
            AbilityDef::triggered(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TSP 192 — Chameleon Blur
pub(in crate::card::sets) static CHAMELEON_BLUR: CardRecord = CardRecord::new(
    "Chameleon Blur",
    "6d506b1c-f329-4e07-8926-a87b4abf16a7",
    "Anthony S. Waters",
    CardRules::new_instant(mana_cost!("{3}{G}")).with_abilities(&[AbilityDef::spell(
        "Prevent all damage that creatures would deal to players this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Matching(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                recipient: DamageRecipientMatcherDef::Recipients(EffectRecipientDef::EachPlayer),
            }),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// TSP 193 — Durkwood Baloth
pub(in crate::card::sets) static DURKWOOD_BALOTH: CardRecord = CardRecord::new(
    "Durkwood Baloth",
    "670521c3-df02-487d-a299-49419e41889f",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Beast"], 5, 5).with_ability(
        abilities::suspend(
            "Suspend 5—{G}",
            &crate::card::SuspendAbilityDef::fixed(5, &[CostDef::Mana(mana_cost!("{G}"))]),
        ),
    ),
);

// TSP 194 — Durkwood Tracker
pub(in crate::card::sets) static DURKWOOD_TRACKER: CardRecord = CardRecord::new(
    "Durkwood Tracker",
    "b49f5835-c933-4ba7-a3c9-34f4eb01f00d",
    "Michael Phillippi",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Giant"], 4, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{G}, {T}: If this creature is on the battlefield, it deals damage equal to its power to target attacking creature. That creature deals damage equal to its power to this creature.",
            &[CostDef::Mana(mana_cost!("{1}{G}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceOnBattlefield,
                then: &EffectDef::Sequence(&[
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::SourcePower,
                    ),
                    EffectDef::damage_from(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                        EffectRecipientDef::Source,
                        ValueDef::TargetPower(TargetIndex::PRIMARY),
                    ),
                ]),
            },
        ),
    ]),
);

// TSP 195 — Fungus Sliver
pub(in crate::card::sets) static FUNGUS_SLIVER: CardRecord = CardRecord::new(
    "Fungus Sliver",
    "c4c0486a-288d-443a-bb07-62dc1dddaed7",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{3}{G}"), &const { ["Fungus", "Sliver"] }, 2, 2)
        .with_abilities(
            &const {
                [AbilityDef::static_ability(
                    "All Sliver creatures have \"Whenever this creature is dealt damage, put a +1/+1 counter on it.\"",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::All(
                                &const {
                                    [
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                    ]
                                },
                            ),
                            &const { [ZoneKind::Battlefield] },
                            PlayerRelation::Any,
                        ),
                        effect: AppliedEffectDef::add_ability(
                            &const {
                                AbilityDef::triggered(
                                    "Whenever this creature is dealt damage, put a +1/+1 counter on it.",
                                    TriggerEventDef::damage_to_source(),
                                    EffectDef::AddCounters {
                                        object: EffectRecipientDef::Source,
                                        kind: CounterKind::PlusOnePlusOne,
                                        amount: ValueDef::Constant(1),
                                    },
                                )
                            },
                        ),
                    },
                )]
            },
        ),
);

// TSP 196 — Gemhide Sliver
pub(in crate::card::sets) static GEMHIDE_SLIVER: CardRecord = CardRecord::new(
    "Gemhide Sliver",
    "f09135b0-fd57-4205-aa74-c9869946c264",
    "John Matson",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Sliver"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "All Slivers have \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(
                    &const {
                        abilities::tap_for_mana(
                            "{T}: Add one mana of any color.",
                            AddManaEffectDef::any_color(),
                        )
                    },
                ),
            },
        ),
    ]),
);

// TSP 197 — Glass Asp
// Audit: unsupported — Needs a temporary payment window lasting until the damaged player's next draw step, with a delayed unpaid life loss.
pub(in crate::card::sets) static GLASS_ASP: CardRecord = CardRecord::new(
    "Glass Asp",
    "118253e9-f33a-455d-b785-dd9df657e7cf",
    "Richard Kane Ferguson",
    CardRules::unsupported(),
);

// TSP 198 — Greenseeker
pub(in crate::card::sets) static GREENSEEKER: CardRecord = CardRecord::new(
    "Greenseeker",
    "f2dfa231-349a-4dce-bed6-c82a136fe0a1",
    "Rebecca Guay",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Spellshaper"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{G}, {T}, Discard a card: Search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
            &[
                CostDef::Mana(mana_cost!("{G}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
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

// TSP 199 — Havenwood Wurm
pub(in crate::card::sets) static HAVENWOOD_WURM: CardRecord = CardRecord::new(
    "Havenwood Wurm",
    "561a4bb5-285a-4d52-b372-d165e442cff3",
    "Stuart Griffin",
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Wurm"], 5, 6)
        .with_abilities(&[abilities::flash(), abilities::trample()]),
);

// TSP 200 — Herd Gnarr
pub(in crate::card::sets) static HERD_GNARR: CardRecord = CardRecord::new(
    "Herd Gnarr",
    "9cf4fd75-34b1-4afa-b8cd-777dfc9e6376",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature gets +2/+2 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 201 — Hypergenesis
// Audit: unsupported — Needs a repeating APNAP sequence of optional hand-to-battlefield choices that stops only after every player declines in one round.
pub(in crate::card::sets) static HYPERGENESIS: CardRecord = CardRecord::new(
    "Hypergenesis",
    "da4d6969-e579-4b28-a76b-5a99c7488f15",
    "Ron Spears",
    CardRules::unsupported(),
);

// TSP 202 — Krosan Grip
pub(in crate::card::sets) static KROSAN_GRIP: CardRecord = CardRecord::new(
    "Krosan Grip",
    "59b524bb-5a18-4dda-9c45-26ce9ff0f0c1",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
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
                then: None,
            },
        ),
    ]),
);

// TSP 203 — Magus of the Candelabra
pub(in crate::card::sets) static MAGUS_OF_THE_CANDELABRA: CardRecord = CardRecord::new(
    "Magus of the Candelabra",
    "960ba1a0-eb35-4622-a7c5-6f9eb1357e23",
    "Terese Nielsen",
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{X}, {T}: Untap X target lands.",
            &[CostDef::Mana(mana_cost!("{X}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_chosen_x(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// TSP 204 — Might of Old Krosa
// Audit: unsupported — Needs cast-time own-main-phase provenance for the larger bonus; sorcery-speed timing is not equivalent.
pub(in crate::card::sets) static MIGHT_OF_OLD_KROSA: CardRecord = CardRecord::new(
    "Might of Old Krosa",
    "7b7b2c47-2123-45c1-993c-e7a2c9438855",
    "Una Fricker",
    CardRules::unsupported(),
);

// TSP 205 — Might Sliver
pub(in crate::card::sets) static MIGHT_SLIVER: CardRecord = CardRecord::new(
    "Might Sliver",
    "8bb8c52f-e608-4710-872f-8a8ae2b5c00c",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Sliver"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures get +2/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            },
        ),
    ]),
);

// TSP 206 — Molder
pub(in crate::card::sets) static MOLDER: CardRecord = CardRecord::new(
    "Molder",
    "8c0eb32c-89b1-40be-a77c-62c114a73ccc",
    "Greg Hildebrandt",
    CardRules::new_instant(mana_cost!("{X}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment with mana value X. It can't be regenerated. You gain X life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ChosenX,
            },
        ]),
    )]),
);

// TSP 207 — Mwonvuli Acid-Moss
pub(in crate::card::sets) static MWONVULI_ACID_MOSS: CardRecord = CardRecord::new(
    "Mwonvuli Acid-Moss",
    "6841dbf6-5023-4612-bbd7-182fd35b05c8",
    "Randy Gallegos",
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target land. Search your library for a Forest card, put that card onto the battlefield tapped, then shuffle.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
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

// TSP 208 — Nantuko Shaman
pub(in crate::card::sets) static NANTUKO_SHAMAN: CardRecord = CardRecord::new(
    "Nantuko Shaman",
    "8e75970f-b41e-406a-9c22-f5df871e70e6",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect", "Shaman"], 3, 2).with_abilities(&[
        AbilityDef::triggered_if(
            "When this creature enters, if you control no tapped lands, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Tapped,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::suspend(
            "Suspend 1—{2}{G}{G}",
            &SuspendAbilityDef::fixed(1, &[CostDef::Mana(mana_cost!("{2}{G}{G}"))]),
        ),
    ]),
);

// TSP 209 — Pendelhaven Elder
pub(in crate::card::sets) static PENDELHAVEN_ELDER: CardRecord = CardRecord::new(
    "Pendelhaven Elder",
    "63d759b5-1739-414e-9616-723625883ebf",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Shaman"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{T}: Each 1/1 creature you control gets +1/+2 until end of turn.",
            &[CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerExactly(1),
                        ObjectPredicateDef::ToughnessExactly(1),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 210 — Penumbra Spider
pub(in crate::card::sets) static PENUMBRA_SPIDER: CardRecord = CardRecord::new(
    "Penumbra Spider",
    "6a989ac1-df69-45e7-98bf-564cc7c38973",
    "Jeff Easley",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Spider"], 2, 4).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "When this creature dies, create a 2/4 black Spider creature token with reach.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::create_creature_token(&["Spider"], &[ManaColor::Black], 2, 4)
                .with_abilities(&[abilities::reach()]),
        ),
    ]),
);

// TSP 211 — Phantom Wurm
// Audit: unsupported — Needs a damage-prevention follow-up removing a counter; prevention alone would make the creature permanently immune.
pub(in crate::card::sets) static PHANTOM_WURM: CardRecord = CardRecord::new(
    "Phantom Wurm",
    "c94722ba-adb5-4613-b8f0-da1c34591c10",
    "Alan Pollack",
    CardRules::unsupported(),
);

// TSP 212 — Primal Forcemage
pub(in crate::card::sets) static PRIMAL_FORCEMAGE: CardRecord = CardRecord::new(
    "Primal Forcemage",
    "98fa7f5c-8a10-4037-9a94-42e0559b4b72",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, that creature gets +3/+3 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 213 — Savage Thallid
pub(in crate::card::sets) static SAVAGE_THALLID: CardRecord = CardRecord::new(
    "Savage Thallid",
    "a87cbbdb-3bbc-48da-b5e3-fcdbddebec81",
    "Luca Zontini",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Fungus"], 5, 2).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice a Saproling: Regenerate target Fungus.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saproling")),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Fungus")),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// TSP 214 — Scarwood Treefolk
pub(in crate::card::sets) static SCARWOOD_TREEFOLK: CardRecord = CardRecord::new(
    "Scarwood Treefolk",
    "ed6dece3-058c-4738-a22f-8893345ddd1c",
    "Stuart Griffin",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Treefolk"], 3, 5)
        .with_abilities(&[abilities::enters_tapped(CardType::Creature)]),
);

// TSP 215 — Scryb Ranger
// Audit: unsupported — Activated-cost enumeration cannot return a chosen Forest to hand as a cost; ReturnToHand currently supports casting costs only.
pub(in crate::card::sets) static SCRYB_RANGER: CardRecord = CardRecord::new(
    "Scryb Ranger",
    "3aacabde-f5ec-4519-895d-17f5e48746ee",
    "Rebecca Guay",
    CardRules::unsupported(),
);

// TSP 216 — Search for Tomorrow
pub(in crate::card::sets) static SEARCH_FOR_TOMORROW: CardRecord = CardRecord::new(
    "Search for Tomorrow",
    "56f739e8-b4ba-426a-a159-0e0d5a0ebb6f",
    "Randy Gallegos",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Search your library for a basic land card, put it onto the battlefield, then shuffle.",
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
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        abilities::suspend(
            "Suspend 2—{G}",
            &SuspendAbilityDef::fixed(2, &[CostDef::Mana(mana_cost!("{G}"))]),
        ),
    ]),
);

// TSP 217 — Spectral Force
pub(in crate::card::sets) static SPECTRAL_FORCE: CardRecord = CardRecord::new(
    "Spectral Force",
    "d7dff1c4-6297-46f6-9c70-544c67319dd0",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental", "Spirit"], 8, 8)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered_if(
                "Whenever this creature attacks, if defending player controls no black permanents, it doesn't untap during your next untap step.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Color(ManaColor::Black),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::DefendingPlayer,
                    ),
                    comparison: ComparisonDef::Equal,
                    amount: 0,
                },
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::Source,
                    count: 1,
                },
            ),
        ]),
);

// TSP 218 — Spike Tiller
pub(in crate::card::sets) static SPIKE_TILLER: CardRecord = CardRecord::new(
    "Spike Tiller",
    "caf3643a-502f-4736-8063-5b567bdfbcc4",
    "Tony Szczudlo",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Spike"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{2}, Remove a +1/+1 counter from this creature: Put a +1/+1 counter on target creature.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}, Remove a +1/+1 counter from this creature: Target land becomes a 2/2 creature that's still a land. Put a +1/+1 counter on it.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// TSP 219 — Spinneret Sliver
pub(in crate::card::sets) static SPINNERET_SLIVER: CardRecord = CardRecord::new(
    "Spinneret Sliver",
    "da698c63-f167-4129-a650-b50c080a24b5",
    "Michael Sutfin",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Sliver"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All Sliver creatures have reach.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&const { abilities::reach() }),
            },
        ),
    ]),
);

// TSP 220 — Sporesower Thallid
pub(in crate::card::sets) static SPORESOWER_THALLID: CardRecord = CardRecord::new(
    "Sporesower Thallid",
    "2f834b1f-9f05-4553-a9b5-828a8348ed30",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Fungus"], 4, 4).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on each Fungus you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Fungus")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ]),
);

// TSP 221 — Sprout
pub(in crate::card::sets) static SPROUT: CardRecord = CardRecord::new(
    "Sprout",
    "6967363f-1e05-4484-8790-47f7be68455c",
    "Anthony S. Waters",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell(
        "Create a 1/1 green Saproling creature token.",
        EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
    )]),
);

// TSP 222 — Squall Line
pub(in crate::card::sets) static SQUALL_LINE: CardRecord = CardRecord::new(
    "Squall Line",
    "3f368729-a6f2-4bf7-8b06-39c551f0b24a",
    "Lars Grant-West",
    CardRules::new_instant(mana_cost!("{X}{G}{G}")).with_abilities(&[AbilityDef::spell(
        "Squall Line deals X damage to each creature with flying and each player.",
        EffectDef::damage_simultaneously(&[
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::ChosenX,
            ),
            DamageAssignmentDef::from_effect(EffectRecipientDef::EachPlayer, ValueDef::ChosenX),
        ]),
    )]),
);

// TSP 223 — Stonewood Invocation
pub(in crate::card::sets) static STONEWOOD_INVOCATION: CardRecord = CardRecord::new(
    "Stonewood Invocation",
    "1b300e67-04d2-4c69-b516-c5cc0a6ff2e7",
    "Pete Venters",
    CardRules::new_instant(mana_cost!("{3}{G}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Target creature gets +5/+5 and gains shroud until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(5),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::shroud() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 224 — Strength in Numbers
pub(in crate::card::sets) static STRENGTH_IN_NUMBERS: CardRecord = CardRecord::new(
    "Strength in Numbers",
    "ca5a571a-f323-473a-b5ec-f881bea82ce1",
    "Ron Spencer",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature gains trample and gets +X/+X, where X is the number of attacking creatures.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_ability(&const { abilities::trample() }),
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                ),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// TSP 225 — Thallid Germinator
pub(in crate::card::sets) static THALLID_GERMINATOR: CardRecord = CardRecord::new(
    "Thallid Germinator",
    "ac526fb0-e6d0-4ee0-9888-15098c1704df",
    "Tom Wänerstrand",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Fungus"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice a Saproling: Target creature gets +1/+1 until end of turn.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saproling")),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 226 — Thallid Shell-Dweller
pub(in crate::card::sets) static THALLID_SHELL_DWELLER: CardRecord = CardRecord::new(
    "Thallid Shell-Dweller",
    "4ee36256-022f-49b3-8914-9b1c2f4dc506",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Fungus"], 0, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a spore counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("spore"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("spore"),
                amount: 3,
            }],
            EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1),
        ),
    ]),
);

// TSP 227 — Thelon of Havenwood
// Audit: unsupported — Needs static bonuses based on each affected creature's own spore counters, rather than the granting source's counters.
pub(in crate::card::sets) static THELON_OF_HAVENWOOD: CardRecord = CardRecord::new(
    "Thelon of Havenwood",
    "92b0be6d-9183-4938-b7a1-ae7f04ba78a0",
    "Kev Walker",
    CardRules::unsupported(),
);

// TSP 228 — Thelonite Hermit
// Audit: unsupported — Needs a turned-face-up trigger for Saproling creation.
pub(in crate::card::sets) static THELONITE_HERMIT: CardRecord = CardRecord::new(
    "Thelonite Hermit",
    "be95b6b0-ff20-405f-81ae-87f5cce45fb2",
    "Chippy",
    CardRules::unsupported(),
);

// TSP 229 — Thrill of the Hunt
pub(in crate::card::sets) static THRILL_OF_THE_HUNT: CardRecord = CardRecord::new(
    "Thrill of the Hunt",
    "e8fe0c8e-f361-4eac-8c2c-ca6602dad352",
    "Stephen Tappin",
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +1/+2 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{W}"))]),
    ]),
);

// TSP 230 — Tromp the Domains
pub(in crate::card::sets) static TROMP_THE_DOMAINS: CardRecord = CardRecord::new(
    "Tromp the Domains",
    "832b67f4-60ec-44ad-9bcc-c3d247c22705",
    "Mike Dringenberg",
    CardRules::new_sorcery(mana_cost!("{5}{G}")).with_abilities(&[AbilityDef::spell(
        "Domain — Until end of turn, creatures you control gain trample and get +1/+1 for each basic land type among lands you control.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_ability(&const { abilities::trample() }),
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                ),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// TSP 231 — Unyaro Bees
pub(in crate::card::sets) static UNYARO_BEES: CardRecord = CardRecord::new(
    "Unyaro Bees",
    "76356bc9-2285-44a7-815e-a27ad4e07afc",
    "Tom Wänerstrand",
    CardRules::new_creature(mana_cost!("{G}{G}{G}"), &["Insect"], 0, 1).with_abilities(&[
        abilities::flying(),
        abilities::apply_to_self_until_end_of_turn(
            "{G}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
        ),
        AbilityDef::activated_with_targets(
            "{3}{G}, Sacrifice this creature: It deals 2 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{3}{G}")),
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// TSP 232 — Verdant Embrace
pub(in crate::card::sets) static VERDANT_EMBRACE: CardRecord = CardRecord::new(
    "Verdant Embrace",
    "3dd1d130-98e7-4898-9f13-2bb58fa4777b",
    "Stephen Tappin",
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+3 and has \"At the beginning of each upkeep, create a 1/1 green Saproling creature token.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(3),
                        ),
                        AppliedEffectDef::add_ability(
                            &const {
                                AbilityDef::triggered(
                                    "At the beginning of each upkeep, create a 1/1 green Saproling creature token.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::Upkeep,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::create_creature_token(
                                        &["Saproling"],
                                        &[ManaColor::Green],
                                        1,
                                        1,
                                    ),
                                )
                            },
                        ),
                    ]),
                },
            ),
        ]),
);

// TSP 233 — Wormwood Dryad
pub(in crate::card::sets) static WORMWOOD_DRYAD: CardRecord = CardRecord::new(
    "Wormwood Dryad",
    "d35ad370-1b7e-4926-9580-7acf919787f2",
    "Warren Mahy",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Dryad"], 3, 1).with_abilities(&[
        AbilityDef::activated(
            "{G}: This creature gains forestwalk until end of turn and deals 1 damage to you.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(
                        &const { abilities::landwalk(BasicLandType::Forest) },
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
            ]),
        ),
        AbilityDef::activated(
            "{B}: This creature gains swampwalk until end of turn and deals 1 damage to you.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(
                        &const { abilities::landwalk(BasicLandType::Swamp) },
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// TSP 234 — Wurmcalling
pub(in crate::card::sets) static WURMCALLING: CardRecord = CardRecord::new(
    "Wurmcalling",
    "85402aef-3fb1-4e78-9103-e2e47e5f3c65",
    "Jeff Easley",
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_abilities(&[
        abilities::buyback(&[CostDef::Mana(mana_cost!("{2}{G}"))]),
        AbilityDef::spell(
            "Create an X/X green Wurm creature token.",
            EffectDef::create_creature_token_with_stats(
                &["Wurm"],
                &[ManaColor::Green],
                &TokenStatsDef {
                    power: ValueDef::ChosenX,
                    toughness: ValueDef::ChosenX,
                },
            ),
        ),
    ]),
);

// TSP 235 — Yavimaya Dryad
// Audit: unsupported — SearchZone and battlefield-arrival modifiers cannot put a found card under a targeted player's control; arrival controllers are relation-based.
pub(in crate::card::sets) static YAVIMAYA_DRYAD: CardRecord = CardRecord::new(
    "Yavimaya Dryad",
    "f1b56d65-b1ac-4aa3-aee6-433770c3dfbc",
    "Rebecca Guay",
    CardRules::unsupported(),
);

// TSP 236 — Dementia Sliver
pub(in crate::card::sets) static DEMENTIA_SLIVER: CardRecord = CardRecord::new(
    "Dementia Sliver",
    "955028d2-33df-4295-8ee3-572e5979b04c",
    "Una Fricker",
    CardRules::new_creature(mana_cost!("{3}{U}{B}"), &const { ["Sliver"] }, 3, 3).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"{T}: Choose a card name. Target opponent reveals a card at random from their hand. If that card has the chosen name, that player discards it. Activate only during your turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_with_targets(
                                "{T}: Choose a card name. Target opponent reveals a card at random from their hand. If that card has the chosen name, that player discards it. Activate only during your turn.",
                                &const { [CostDef::TapSource] },
                                &const {
                                    [AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                                    )]
                                },
                                EffectDef::Sequence(
                                    &const {
                                        [
                                            EffectDef::BindOutput {
                                                binding: Binding!("tsp_named"),
                                                effect: &EffectDef::ChooseCardName {
                                                    chooser: PlayerRefDef::EffectController,
                                                    names: CardNameSetDef::AllCardNames,
                                                },
                                            },
                                            EffectDef::BindOutput {
                                                binding: Binding!("tsp_revealed"),
                                                effect: &EffectDef::RevealAtRandomFromHand {
                                                    player: EffectRecipientDef::Target(
                                                        TargetIndex::PRIMARY,
                                                    ),
                                                },
                                            },
                                            EffectDef::IfCondition {
                                                condition: &TriggerConditionDef::ObjectSetCount(
                                                    &ObjectSetCountConditionDef {
                                                        objects: &ObjectSetDef::Binding(Binding!(
                                                            "tsp_revealed"
                                                        )),
                                                        predicate: ObjectSetPredicateDef::contains(
                                                            &const {
                                                                ObjectPredicateDef::NameEquals(
                                                                    CardNameDef::Binding(Binding!(
                                                                        "tsp_named"
                                                                    )),
                                                                )
                                                            },
                                                        ),
                                                    },
                                                ),
                                                then: &EffectDef::discard_cards(
                                                    EffectRecipientDef::objects(
                                                        ObjectSetDef::Binding(Binding!(
                                                            "tsp_revealed"
                                                        )),
                                                    ),
                                                ),
                                            },
                                        ]
                                    },
                                ),
                            )
                            .with_activation_condition(
                                &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 237 — Dralnu, Lich Lord
// Audit: unsupported — Needs replacement of a damage event with a quantity of mandatory sacrifices; ordinary damage prevention is not equivalent.
pub(in crate::card::sets) static DRALNU_LICH_LORD: CardRecord = CardRecord::new(
    "Dralnu, Lich Lord",
    "b3828efe-cbe1-473c-8dde-60ea9aaf23a2",
    "Greg Staples",
    CardRules::unsupported(),
);

// TSP 238 — Firewake Sliver
pub(in crate::card::sets) static FIREWAKE_SLIVER: CardRecord = CardRecord::new(
    "Firewake Sliver",
    "5d3f5a0d-029e-44fd-b3e6-c70176b5b4ac",
    "Anthony S. Waters",
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &const { ["Sliver"] }, 1, 1).with_abilities(
        &const {
            [
                AbilityDef::static_ability(
                    "All Sliver creatures have haste.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::All(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                                    ]
                                },
                            ),
                            &const { [ZoneKind::Battlefield] },
                            PlayerRelation::Any,
                        ),
                        effect: AppliedEffectDef::add_ability(&const { abilities::haste() }),
                    },
                ),
                AbilityDef::static_ability(
                    "All Slivers have \"{1}, Sacrifice this permanent: Target Sliver creature gets +2/+2 until end of turn.\"",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                            &const { [ZoneKind::Battlefield] },
                            PlayerRelation::Any,
                        ),
                        effect: AppliedEffectDef::add_ability(
                            &const {
                                AbilityDef::activated_with_targets(
                                    "{1}, Sacrifice this permanent: Target Sliver creature gets +2/+2 until end of turn.",
                                    &const {
                                        [CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource]
                                    },
                                    &const {
                                        [AbilityTargetDef::exactly_one(
                                            AbilityTargetPredicate::Object {
                                                object: ObjectPredicateDef::All(
                                                    &const {
                                                        [
                                                            ObjectPredicateDef::HasType(
                                                                CardType::Creature,
                                                            ),
                                                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                                                        ]
                                                    },
                                                ),
                                                zones: &const { [ZoneKind::Battlefield] },
                                                controller: None,
                                                owner: None,
                                            },
                                        )]
                                    },
                                    EffectDef::Apply {
                                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        effect: AppliedEffectDef::modify_power_toughness(
                                            ValueDef::Constant(2),
                                            ValueDef::Constant(2),
                                        ),
                                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                                    },
                                )
                            },
                        ),
                    },
                ),
            ]
        },
    ),
);

// TSP 239 — Ghostflame Sliver
pub(in crate::card::sets) static GHOSTFLAME_SLIVER: CardRecord = CardRecord::new(
    "Ghostflame Sliver",
    "64eb10d1-22c6-4848-a7f4-c6ccd2c66dae",
    "Luca Zontini",
    CardRules::new_creature(mana_cost!("{B}{R}"), &["Sliver"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All Slivers are colorless.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::set_colors(ColorSet::empty()),
            },
        ),
    ]),
);

// TSP 240 — Harmonic Sliver
pub(in crate::card::sets) static HARMONIC_SLIVER: CardRecord = CardRecord::new(
    "Harmonic Sliver",
    "f7904997-a857-44f0-91d8-b78651bb4e83",
    "Luca Zontini",
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &const { ["Sliver"] }, 1, 1).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"When this permanent enters, destroy target artifact or enchantment.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::triggered_with_targets(
                                "When this permanent enters, destroy target artifact or enchantment.",
                                TriggerEventDef::zone_changed(
                                    ObjectPredicateDef::Source,
                                    None,
                                    Some(ZoneKind::Battlefield),
                                ),
                                &const {
                                    [AbilityTargetDef::exactly_one(
                                        AbilityTargetPredicate::Object {
                                            object: ObjectPredicateDef::AnyOf(
                                                &const {
                                                    [
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Artifact,
                                                        ),
                                                        ObjectPredicateDef::HasType(
                                                            CardType::Enchantment,
                                                        ),
                                                    ]
                                                },
                                            ),
                                            zones: &const { [ZoneKind::Battlefield] },
                                            controller: None,
                                            owner: None,
                                        },
                                    )]
                                },
                                EffectDef::Destroy {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    then: None,
                                },
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 241 — Ith, High Arcanist
pub(in crate::card::sets) static ITH_HIGH_ARCANIST: CardRecord = CardRecord::new(
    "Ith, High Arcanist",
    "8dd98dea-8012-49bb-84cf-dd8ed5c032cf",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_creature(mana_cost!("{5}{W}{U}"), &["Human", "Wizard"], 3, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::activated_with_targets(
                "{T}: Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Attacking,
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::PreventDamage {
                        prevention: DamagePreventionDef::unlimited(
                            DamageEventMatcherDef::combat_from(ObjectRefDef::Target(
                                TargetIndex::PRIMARY,
                            )),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::PreventDamage {
                        prevention: DamagePreventionDef::unlimited(
                            DamageEventMatcherDef::combat_to(EffectRecipientDef::Target(
                                TargetIndex::PRIMARY,
                            )),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            abilities::suspend(
                "Suspend 4—{W}{U}",
                &SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{W}{U}"))]),
            ),
        ]),
);

// TSP 242 — Kaervek the Merciless
pub(in crate::card::sets) static KAERVEK_THE_MERCILESS: CardRecord = CardRecord::new(
    "Kaervek the Merciless",
    "444fa5a5-b5b0-4555-bd69-67c1c0cbf317",
    "rk post",
    CardRules::new_creature(mana_cost!("{5}{B}{R}"), &["Human", "Shaman"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever an opponent casts a spell, Kaervek deals damage equal to that spell's mana value to any target.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                    objects: ObjectSetDef::One(ObjectRefDef::TriggeringObject),
                    select: ObjectValueDef::ManaValue,
                    operation: AggregateOperationDef::Sum,
                }),
            ),
        )]),
);

// TSP 243 — Mishra, Artificer Prodigy
// Audit: unsupported — Needs one optional search spanning hand, graveyard, and library, recording whether the library was searched before deciding to shuffle.
pub(in crate::card::sets) static MISHRA_ARTIFICER_PRODIGY: CardRecord = CardRecord::new(
    "Mishra, Artificer Prodigy",
    "d5deaec5-499d-4e19-b879-8bcd1dc35f3e",
    "Scott M. Fischer",
    CardRules::unsupported(),
);

// TSP 244 — Opaline Sliver
pub(in crate::card::sets) static OPALINE_SLIVER: CardRecord = CardRecord::new(
    "Opaline Sliver",
    "75af1cc6-cb40-48c8-818b-91e64bdbe691",
    "Dave Dorman",
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &const { ["Sliver"] }, 2, 2).with_abilities(
        &const {
            [AbilityDef::static_ability(
                "All Slivers have \"Whenever this permanent becomes the target of a spell an opponent controls, you may draw a card.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                        &const { [ZoneKind::Battlefield] },
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::triggered(
                                "Whenever this permanent becomes the target of a spell an opponent controls, you may draw a card.",
                                TriggerEventDef::becomes_targeted(ObjectPredicateDef::All(
                                    &const {
                                        [
                                            ObjectPredicateDef::Spell,
                                            ObjectPredicateDef::ControlledBy(
                                                PlayerRelation::Opponent,
                                            ),
                                        ]
                                    },
                                )),
                                EffectDef::May {
                                    player: EffectRecipientDef::Controller,
                                    effect: &EffectDef::DrawCards {
                                        recipient: EffectRecipientDef::Controller,
                                        amount: ValueDef::Constant(1),
                                    },
                                },
                            )
                        },
                    ),
                },
            )]
        },
    ),
);

// TSP 245 — Saffi Eriksdotter
// Audit: unsupported — Needs a delayed dies trigger matching one saved target object independently of that creature's own abilities; granting a dies ability is not equivalent.
pub(in crate::card::sets) static SAFFI_ERIKSDOTTER: CardRecord = CardRecord::new(
    "Saffi Eriksdotter",
    "c34bb91e-138b-4491-bb2d-5df2cd272bf0",
    "Christopher Moeller",
    CardRules::unsupported(),
);

// TSP 246 — Scion of the Ur-Dragon
pub(in crate::card::sets) static SCION_OF_THE_UR_DRAGON: CardRecord = CardRecord::new(
    "Scion of the Ur-Dragon",
    "efb6f85b-60e9-4191-b557-ec952b7f22fc",
    "Jim Murray",
    CardRules::new_creature(mana_cost!("{W}{U}{B}{R}{G}"), &["Dragon", "Avatar"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated(
                "{2}: Search your library for a Dragon permanent card and put it into your graveyard. If you do, Scion of the Ur-Dragon becomes a copy of that card until end of turn. Then shuffle.",
                &[CostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Sequence(&[
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                        ]),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: true,
                        destination: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                        shuffle: false,
                        enters_tapped: false,
                        attachment: None,
                        binding: Some(ParentBinding),
                        then: Some(&EffectDef::BecomeCopyOf {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                            copier: None,
                            exceptions: CopyExceptionsDef::NONE,
                            duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
                        }),
                    },
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                ]),
            ),
        ]),
);

// TSP 247 — Stonebrow, Krosan Hero
pub(in crate::card::sets) static STONEBROW_KROSAN_HERO: CardRecord = CardRecord::new(
    "Stonebrow, Krosan Hero",
    "95c62102-0562-4b09-a05d-986aa4212f44",
    "Ron Spears",
    CardRules::new_creature(mana_cost!("{3}{R}{G}"), &["Centaur", "Warrior"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "Whenever a creature you control with trample attacks, it gets +2/+2 until end of turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Trample),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::TriggeringObject,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// TSP 248 — Assembly-Worker
pub(in crate::card::sets) static ASSEMBLY_WORKER: CardRecord = CardRecord::new(
    "Assembly-Worker",
    "e086d662-3740-4704-9184-c42c6a16d829",
    "Chippy",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Assembly-Worker"], 2, 2).with_abilities(
        &[AbilityDef::activated_with_targets(
            "{T}: Target Assembly-Worker creature gets +1/+1 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Assembly-Worker")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )],
    ),
);

// TSP 249 — Brass Gnat
pub(in crate::card::sets) static BRASS_GNAT: CardRecord = CardRecord::new(
    "Brass Gnat",
    "386ae7c6-347c-4b29-b7a9-3ca3bb050396",
    "Martina Pilcerova",
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may pay {1}. If you do, untap this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{1}"))],
                &EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// TSP 250 — Candles of Leng
pub(in crate::card::sets) static CANDLES_OF_LENG: CardRecord = CardRecord::new(
    "Candles of Leng",
    "ee874779-e654-48f0-8f2d-cc98ef150d75",
    "Daniel Gelon",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::activated(
        "{4}, {T}: Reveal the top card of your library. If it has the same name as a card in your graveyard, put it into your graveyard. Otherwise, draw a card.",
        &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
        abilities::bind_top_cards_then(
            PlayerRefDef::EffectController,
            ValueDef::Constant(1),
            &EffectDef::Sequence(&[
                EffectDef::RevealObjects(RevealObjectsDef {
                    input: ObjectSetDef::Binding(ParentBinding),
                    then: &EffectDef::None,
                }),
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::NameIn(&CardNameSetDef::NamesOf(
                                &ObjectSetDef::Binding(ParentBinding),
                            )),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                        ZoneKind::Graveyard,
                        ZonePlacement::Top,
                    ),
                    otherwise: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
    )]),
);

// TSP 251 — Chromatic Star
pub(in crate::card::sets) static CHROMATIC_STAR: CardRecord = CardRecord::new(
    "Chromatic Star",
    "1d7a1357-debd-49b0-9fd5-560d5b3f589e",
    "Alex Horley-Orlandelli",
    // A card that fixes one mana and replaces itself, and does the second
    // half however it dies rather than only when it is spent.
    // The draw is a separate trigger rather than part of the mana ability,
    // which is the whole difference from Chromatic Sphere: the mana arrives at
    // once and the card waits on the stack, so anything that answers the Star
    // after it has been sacrificed is already too late.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{1}, {T}, Sacrifice this artifact: Add one mana of any color.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::triggered(
            "When this artifact is put into a graveyard from the battlefield, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TSP 252 — Chronatog Totem
// Audit: unsupported — Needs a resolving effect that schedules skipping the controller's next turn.
pub(in crate::card::sets) static CHRONATOG_TOTEM: CardRecord = CardRecord::new(
    "Chronatog Totem",
    "5acd482c-9815-4ff5-99ef-fb1a014251bb",
    "Christopher Rush",
    CardRules::unsupported(),
);

// TSP 253 — Clockwork Hydra
pub(in crate::card::sets) static CLOCKWORK_HYDRA: CardRecord = CardRecord::new(
    "Clockwork Hydra",
    "54522c50-5333-47ac-ba3e-d87c599404c4",
    "Daren Bader",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Hydra"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with four +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 4,
                },
            ),
        ),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks or blocks, remove a +1/+1 counter from it. If you do, it deals 1 damage to any target.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                TriggerEventDef::Blocks {
                    blocked: ObjectPredicateDef::Any,
                },
            ]),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::All(&[
                    TriggerConditionDef::SourceOnBattlefield,
                    TriggerConditionDef::SourceCounters {
                        kind: CounterKind::PlusOnePlusOne,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                ]),
                then: &EffectDef::Sequence(&[
                    EffectDef::RemoveCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::damage(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(1),
                    ),
                ]),
            },
        ),
        AbilityDef::activated(
            "{T}: Put a +1/+1 counter on this creature.",
            &[CostDef::TapSource],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TSP 254 — Foriysian Totem
pub(in crate::card::sets) static FORIYSIAN_TOTEM: CardRecord = CardRecord::new(
    "Foriysian Totem",
    "45cba87d-3c8c-4f38-bdd8-51072ba6365b",
    "Anson Maddocks",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for(ManaColor::Red),
        AbilityDef::activated(
            "{4}{R}: This artifact becomes a 4/4 red Giant artifact creature with trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{4}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Giant"])),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(4),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::trample() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::static_ability(
            "As long as this artifact is a creature, it can block an additional creature each combat.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(1)),
                },
            },
        ),
    ]),
);

// TSP 255 — Gauntlet of Power
// Audit: unsupported — Needs a tapped-for-mana trigger comparing actual produced mana against the source's chosen color.
pub(in crate::card::sets) static GAUNTLET_OF_POWER: CardRecord = CardRecord::new(
    "Gauntlet of Power",
    "109f93ac-86e9-42f1-9fba-fec8bcd521b0",
    "Greg Hildebrandt",
    CardRules::unsupported(),
);

// TSP 256 — Hivestone
pub(in crate::card::sets) static HIVESTONE: CardRecord = CardRecord::new(
    "Hivestone",
    "685fc2f8-4cad-46a1-b9d7-d94c13990994",
    "Dave Allsop",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures you control are Slivers in addition to their other creature types.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Sliver"])),
        },
    )]),
);

// TSP 257 — Jhoira's Timebug
pub(in crate::card::sets) static JHOIRAS_TIMEBUG: CardRecord = CardRecord::new(
    "Jhoira's Timebug",
    "9ce2c6d7-505b-490b-9c6f-b5166c9ff71d",
    "Dan Frazier",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Insect"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Choose target permanent you control or suspended card you own. If it has a time counter on it, you may remove a time counter from it or put another time counter on it.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyOf(&[
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    AbilityTargetPredicate::Object {
                        object: abilities::SUSPENDED_CARD,
                        zones: &[ZoneKind::Exile],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                ]),
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::HasCounter(CounterKind::named("time")),
                },
                then: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Do nothing",
                            effect: EffectDef::None,
                        },
                        EffectChoiceDef {
                            label: "Remove a time counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Fixed(CounterKind::named("time")),
                                operation: CounterOperationDef::Remove,
                                amount: ValueDef::Constant(1),
                            },
                        },
                        EffectChoiceDef {
                            label: "Put another time counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Fixed(CounterKind::named("time")),
                                operation: CounterOperationDef::Add,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    ],
                },
            },
        ),
    ),
);

// TSP 258 — Locket of Yesterdays
// Audit: unsupported — Needs spell-cost evaluation that compares graveyard card names to the spell being costed, rather than the cost-modifying permanent.
pub(in crate::card::sets) static LOCKET_OF_YESTERDAYS: CardRecord = CardRecord::new(
    "Locket of Yesterdays",
    "b316f198-4465-4fb5-a7a6-a92bf8eae8c1",
    "Dany Orizio",
    CardRules::unsupported(),
);

// TSP 259 — Lotus Bloom
pub(in crate::card::sets) static LOTUS_BLOOM: CardRecord = CardRecord::new(
    "Lotus Bloom",
    "73127ee0-9a0c-48fa-9c60-b4c600ace8f7",
    "Mark Zug",
    CardRules::new_artifact_without_mana_cost(&[]).with_abilities(&[
        abilities::suspend(
            "Suspend 3—{0}",
            &SuspendAbilityDef::fixed(3, &[CostDef::Mana(mana_cost!("{0}"))]),
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this artifact: Add three mana of any one color.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
        ),
    ]),
);

// TSP 260 — Paradise Plume
pub(in crate::card::sets) static PARADISE_PLUME: CardRecord = CardRecord::new(
    "Paradise Plume",
    "1205ac7e-4d4e-4849-b3ae-1e4e0558fd96",
    "Wayne England",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this permanent enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::triggered(
            "Whenever a player casts a spell of the chosen color, you may gain 1 life.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::HasSourcesChosenScalar(
                BattlefieldEntryChoiceDestinationDef::Color,
            )),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
        abilities::tap_for_mana(
            "{T}: Add one mana of the chosen color.",
            AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor),
        ),
    ]),
);

// TSP 261 — Phyrexian Totem
pub(in crate::card::sets) static PHYREXIAN_TOTEM: CardRecord = CardRecord::new(
    "Phyrexian Totem",
    "368b9fee-88ce-4725-82a5-335d8645aca2",
    "John Zeleznik",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for(ManaColor::Black),
        AbilityDef::activated(
            "{2}{B}: This artifact becomes a 5/5 black Phyrexian Horror artifact creature with trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                        "Phyrexian",
                        "Horror",
                    ])),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Black])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(5),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::trample() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered_if(
            "Whenever this permanent is dealt damage, if it's a creature, sacrifice that many permanents.",
            TriggerEventDef::damage_to_source(),
            &TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::HasType(CardType::Creature),
            },
            actions::choose_sacrifice(1)
                .with_amount(ValueDef::TriggerEventAmount)
                .as_effect(),
        ),
    ]),
);

// TSP 262 — Prismatic Lens
pub(in crate::card::sets) static PRISMATIC_LENS: CardRecord = CardRecord::new(
    "Prismatic Lens",
    "50f058a7-c3c7-4bdf-a66c-a2636a8bd9db",
    "Alan Pollack",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// TSP 263 — Sarpadian Empires, Vol. VII
// Audit: unsupported — Needs one entry choice binding a paired color and creature type for later token creation.
pub(in crate::card::sets) static SARPADIAN_EMPIRES_VOL_VII: CardRecord = CardRecord::new(
    "Sarpadian Empires, Vol. VII",
    "b37a6cf7-f239-4bca-b4c3-a48932ef56b5",
    "Doug Chaffee",
    CardRules::unsupported(),
);

// TSP 264 — Stuffy Doll
pub(in crate::card::sets) static STUFFY_DOLL: CardRecord = CardRecord::new(
    "Stuffy Doll",
    "14ca7425-a499-4864-b955-369ef2577849",
    "Dave Allsop",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Construct"], 0, 1).with_abilities(&[
        abilities::indestructible(),
        AbilityDef::as_enters(
            "As this creature enters, choose a player.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::PLAYER,
            )),
        ),
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, it deals that much damage to the chosen player.",
            TriggerEventDef::damage_to_source(),
            EffectDef::damage(
                EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::ChosenPlayer)),
                ValueDef::TriggerEventAmount,
            ),
        ),
        AbilityDef::activated(
            "{T}: This creature deals 1 damage to itself.",
            &[CostDef::TapSource],
            EffectDef::damage(EffectRecipientDef::Source, ValueDef::Constant(1)),
        ),
    ]),
);

// TSP 265 — Thunder Totem
pub(in crate::card::sets) static THUNDER_TOTEM: CardRecord = CardRecord::new(
    "Thunder Totem",
    "5fc332da-e93f-45b1-912c-06af41b412e9",
    "Randy Gallegos",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for(ManaColor::White),
        AbilityDef::activated(
            "{1}{W}{W}: This artifact becomes a 2/2 white Spirit artifact creature with flying and first strike until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Spirit"])),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::flying() }),
                    AppliedEffectDef::add_ability(&const { abilities::first_strike() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 266 — Triskelavus
pub(in crate::card::sets) static TRISKELAVUS: CardRecord = CardRecord::new(
    "Triskelavus",
    "978d11e7-cfb7-41bf-bb98-deb79549a074",
    "Mark Zug",
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Construct"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::as_enters(
            "This creature enters with three +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated(
            "{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless Triskelavite artifact creature token with flying. It has \"Sacrifice this token: This token deals 1 damage to any target.\"",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Triskelavite"], &[], 1, 1).with_abilities(
                &[
                    abilities::flying(),
                    AbilityDef::activated_with_targets(
                        "Sacrifice this token: This token deals 1 damage to any target.",
                        &[CostDef::SacrificeSource],
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::AnyTarget,
                        )],
                        EffectDef::damage(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ValueDef::Constant(1),
                        ),
                    ),
                ],
            ),
        ),
    ]),
);

// TSP 267 — Venser's Sliver
pub(in crate::card::sets) static VENSER_S_SLIVER: CardRecord = CardRecord::new(
    "Venser's Sliver",
    "1e3c5a64-453b-4477-853a-9514ba326f16",
    "Carl Critchlow",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Sliver"], 3, 3),
);

// TSP 268 — Weatherseed Totem
pub(in crate::card::sets) static WEATHERSEED_TOTEM: CardRecord = CardRecord::new(
    "Weatherseed Totem",
    "3275df49-c3f1-4310-87dd-25e3e0ee7fca",
    "Heather Hudson",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated(
            "{2}{G}{G}{G}: This artifact becomes a 5/3 green Treefolk artifact creature with trample until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}{G}{G}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Treefolk"])),
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(5),
                        ValueDef::Constant(3),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::trample() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "When this artifact is put into a graveyard from the battlefield, if it was a creature, return this card to its owner's hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Source,
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// TSP 269 — Academy Ruins
pub(in crate::card::sets) static ACADEMY_RUINS: CardRecord = CardRecord::new(
    "Academy Ruins",
    "af09af65-0a3b-42df-b0fd-372e2158beac",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated_with_targets(
                "{1}{U}, {T}: Put target artifact card from your graveyard on top of your library.",
                &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::TapSource],
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
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// TSP 270 — Calciform Pools
// Audit: unsupported — The mana planner cannot pay generic mana together with variable counter removal unless the ability also taps or sacrifices a permanent; this activation does neither.
pub(in crate::card::sets) static CALCIFORM_POOLS: CardRecord = CardRecord::new(
    "Calciform Pools",
    "f6cead86-b8ff-4975-90b2-3badcc452fdb",
    "Darrell Riche",
    CardRules::unsupported(),
);

// TSP 271 — Dreadship Reef
// Audit: unsupported — The mana planner cannot pay generic mana together with variable counter removal unless the ability also taps or sacrifices a permanent; this activation does neither.
pub(in crate::card::sets) static DREADSHIP_REEF: CardRecord = CardRecord::new(
    "Dreadship Reef",
    "b4a83915-07b7-4912-bdf2-089b05796378",
    "Lars Grant-West",
    CardRules::unsupported(),
);

// TSP 272 — Flagstones of Trokair
pub(in crate::card::sets) static FLAGSTONES_OF_TROKAIR: CardRecord = CardRecord::new(
    "Flagstones of Trokair",
    "ad23cd0e-6871-4ba6-b5bf-cfd14f8f71b2",
    "Rob Alexander",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::White),
            AbilityDef::triggered(
                "When Flagstones of Trokair is put into a graveyard from the battlefield, you may search your library for a Plains card, put it onto the battlefield tapped, then shuffle.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
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
        ]),
);

// TSP 273 — Fungal Reaches
// Audit: unsupported — The mana planner cannot pay generic mana together with variable counter removal unless the ability also taps or sacrifices a permanent; this activation does neither.
pub(in crate::card::sets) static FUNGAL_REACHES: CardRecord = CardRecord::new(
    "Fungal Reaches",
    "9b4521b3-a3b0-4a1d-a623-cc630058eee2",
    "Martina Pilcerova",
    CardRules::unsupported(),
);

// TSP 274 — Gemstone Caverns
pub(in crate::card::sets) static GEMSTONE_CAVERNS: CardRecord = CardRecord::new(
    "Gemstone Caverns",
    "94d74254-4750-4fb3-9e53-473a5f98b315",
    "Martina Pilcerova",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::opening_hand_with(
                "If this card is in your opening hand and you're not the starting player, you may begin the game with Gemstone Caverns on the battlefield with a luck counter on it. If you do, exile a card from your hand.",
                PregameConditionDef::NotStartingPlayer,
                &[CostDef::ExileCardFromHand(ObjectPredicateDef::Any)],
                EffectDef::WithBattlefieldArrival {
                    effect: &const {
                        EffectDef::move_to_zone(
                            EffectRecipientDef::Source,
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        )
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        counters: Some(TokenCountersDef {
                            kind: CounterKind::named("luck"),
                            amount: ValueDef::Constant(1),
                        }),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            AbilityDef::activated_mana_if(
                "{T}: Add {C}.",
                &[CostDef::TapSource],
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("luck"),
                    comparison: ComparisonDef::LessOrEqual,
                    amount: 0,
                },
                EffectDef::AddMana(AddManaEffectDef::one(crate::card::ManaColor::Colorless)),
            ),
            AbilityDef::activated_mana_if(
                "{T}: If this land has a luck counter on it, add one mana of any color instead.",
                &[CostDef::TapSource],
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("luck"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                EffectDef::AddMana(AddManaEffectDef::any_color()),
            ),
        ]),
);

// TSP 275 — Kher Keep
pub(in crate::card::sets) static KHER_KEEP: CardRecord = CardRecord::new(
    "Kher Keep",
    "dde3b332-5bc3-47bc-ba24-5437f04c21a0",
    "Paolo Parente",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated(
                "{1}{R}, {T}: Create a 0/1 red Kobold creature token named Kobolds of Kher Keep.",
                &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::TapSource],
                EffectDef::create_creature_token(&["Kobold"], &[ManaColor::Red], 0, 1)
                    .with_name("Kobolds of Kher Keep"),
            ),
        ]),
);

// TSP 276 — Molten Slagheap
// Audit: unsupported — The mana planner cannot pay generic mana together with variable counter removal unless the ability also taps or sacrifices a permanent; this activation does neither.
pub(in crate::card::sets) static MOLTEN_SLAGHEAP: CardRecord = CardRecord::new(
    "Molten Slagheap",
    "d090288c-48f3-49fe-87c4-3766eb7bb240",
    "Daren Bader",
    CardRules::unsupported(),
);

// TSP 277 — Saltcrusted Steppe
// Audit: unsupported — The mana planner cannot pay generic mana together with variable counter removal unless the ability also taps or sacrifices a permanent; this activation does neither.
pub(in crate::card::sets) static SALTCRUSTED_STEPPE: CardRecord = CardRecord::new(
    "Saltcrusted Steppe",
    "2adc67a6-9b05-4480-be0c-70b860075d70",
    "Greg Staples",
    CardRules::unsupported(),
);

// TSP 278 — Swarmyard
pub(in crate::card::sets) static SWARMYARD: CardRecord = CardRecord::new(
    "Swarmyard",
    "25bfe849-a508-4fa8-8999-909fe8ec34fa",
    "Thomas M. Baxa",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}: Regenerate target Insect, Rat, Spider, or Squirrel.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Insect")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Rat")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Squirrel")),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// TSP 279 — Terramorphic Expanse
pub(in crate::card::sets) static TERRAMORPHIC_EXPANSE: CardRecord = CardRecord::new(
    "Terramorphic Expanse",
    "fd097ea2-0a1c-44a0-824b-d5373df513fb",
    "Dan Murayama Scott",
    CardRules::new_land(&[]).with_abilities(&[AbilityDef::activated(
        "{T}, Sacrifice this land: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.",
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

// TSP 280 — Urza's Factory
pub(in crate::card::sets) static URZA_S_FACTORY: CardRecord = CardRecord::new(
    "Urza's Factory",
    "b22d97f3-08cf-4c19-bbe2-5a36096187cd",
    "Mark Tedin",
    CardRules::new_land(&["Urza's"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{7}, {T}: Create a 2/2 colorless Assembly-Worker artifact creature token.",
            &[CostDef::Mana(mana_cost!("{7}")), CostDef::TapSource],
            EffectDef::create_artifact_creature_token(&["Assembly-Worker"], &[], 2, 2),
        ),
    ]),
);

// TSP 281 — Vesuva
// Audit: unsupported — Needs optional copy entry coupled to tapped arrival only when copying is chosen; independent copy and tapped replacements also tap a declined copy.
pub(in crate::card::sets) static VESUVA: CardRecord = CardRecord::new(
    "Vesuva",
    "82fc9498-7397-4857-87fe-7c9010944ed8",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::unsupported(),
);

// TSP 282 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &super::super::y1993::alpha::PLAINS,
    "cd9a8337-e4ca-458d-8fcd-672d8d6f1c0d",
    "Rob Alexander",
);

// TSP 283 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::PLAINS,
    1,
    "97e0aaf3-eba0-445e-b560-f889b800e6b7",
    "Craig Mullins",
);

// TSP 284 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::PLAINS,
    2,
    "d056a94a-a07d-4494-a75c-b3f6f59457b3",
    "Justin Sweet",
);

// TSP 285 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::PLAINS,
    3,
    "fa21cca2-59cb-4223-b67b-1a8fc54aadd8",
    "Richard Wright",
);

// TSP 286 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &super::super::y1993::alpha::ISLAND,
    "e5fdeb2c-afb8-4850-bcdc-86283ffa3486",
    "Rob Alexander",
);

// TSP 287 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::ISLAND,
    1,
    "e08915c9-78ff-450b-a47f-0c9678d16bbd",
    "Jeremy Jarvis",
);

// TSP 288 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::ISLAND,
    2,
    "89f8c7cf-6705-4ff7-8aa8-113c1400ffda",
    "Craig Mullins",
);

// TSP 289 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::ISLAND,
    3,
    "ac1a989a-38d4-4225-a41f-37d54f2f42ae",
    "Richard Wright",
);

// TSP 290 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &super::super::y1993::alpha::SWAMP,
    "3fecc759-61fa-4040-9b46-710512baa4bf",
    "John Avon",
);

// TSP 291 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::SWAMP,
    1,
    "b53dbcc0-169d-4b5c-b4e5-d82716c1539d",
    "Vance Kovacs",
);

// TSP 292 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::SWAMP,
    2,
    "a7dc4523-075d-425e-9b53-77921d26c173",
    "Craig Mullins",
);

// TSP 293 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::SWAMP,
    3,
    "dd634a88-c114-469a-8133-59cfc9bf3215",
    "Richard Wright",
);

// TSP 294 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &super::super::y1993::alpha::MOUNTAIN,
    "e34229e7-5dc4-47b3-9ba2-609e5b3ca6c2",
    "John Avon",
);

// TSP 295 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::MOUNTAIN,
    1,
    "5c71906a-f885-49ba-a889-8baf4e74cce8",
    "D. Alexander Gregory",
);

// TSP 296 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::MOUNTAIN,
    2,
    "d3e3961c-d092-4a15-b5db-d7b5c70e4fc1",
    "Craig Mullins",
);

// TSP 297 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::MOUNTAIN,
    3,
    "dae3e69d-ec38-43e2-b4a2-3a9064835a97",
    "Greg Staples",
);

// TSP 298 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &super::super::y1993::alpha::FOREST,
    "0d5bd3e1-45ef-43b7-8796-6085842278df",
    "Rob Alexander",
);

// TSP 299 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::FOREST,
    1,
    "2a10ed4f-2c9a-424d-bcca-730af7727854",
    "Vance Kovacs",
);

// TSP 300 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::FOREST,
    2,
    "2b176d24-a4d6-489e-af69-4547693e7de1",
    "Craig Mullins",
);

// TSP 301 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &super::super::y1993::alpha::FOREST,
    3,
    "393d66f7-7dbf-4a92-912f-377f99c8f164",
    "Stephen Tappin",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AMROU_SCOUT,
    &AMROU_SEEKERS,
    &ANGEL_S_GRACE,
    &BENALISH_CAVALRY,
    &CASTLE_RAPTORS,
    &CAVALRY_MASTER,
    &CELESTIAL_CRUSADER,
    &CHILDREN_OF_KORLIS,
    &CHRONOSAVANT,
    &CLOUDCHASER_KESTREL,
    &D_AVENANT_HEALER,
    &DETAINMENT_SPELL,
    &DIVINE_CONGREGATION,
    &DUSKRIDER_PEREGRINE,
    &ERRANT_DOOMSAYERS,
    &EVANGELIZE,
    &FLICKERING_SPIRIT,
    &FORIYSIAN_INTERCEPTOR,
    &FORTIFY,
    &GAZE_OF_JUSTICE,
    &GRIFFIN_GUIDE,
    &GUSTCLOAK_CAVALIER,
    &ICATIAN_CRIER,
    &IVORY_GIANT,
    &JEDIT_S_DRAGOONS,
    &KNIGHT_OF_THE_HOLY_NIMBUS,
    &MAGUS_OF_THE_DISK,
    &MANGARA_OF_CORONDOR,
    &MOMENTARY_BLINK,
    &OPAL_GUARDIAN,
    &OUTRIDER_EN_KOR,
    &PENTARCH_PALADIN,
    &PENTARCH_WARD,
    &PLATED_PEGASUS,
    &PULL_FROM_ETERNITY,
    &PULMONIC_SLIVER,
    &QUILLED_SLIVER,
    &RESTORE_BALANCE,
    &RETURN_TO_DUST,
    &SERRA_AVENGER,
    &SIDEWINDER_SLIVER,
    &SPIRIT_LOOP,
    &TEMPORAL_ISOLATION,
    &TIVADAR_OF_THORN,
    &WATCHER_SLIVER,
    &WEATHERED_BODYGUARDS,
    &ZEALOT_IL_VEC,
    &ANCESTRAL_VISION,
    &BEWILDER,
    &BRINE_ELEMENTAL,
    &CANCEL,
    &CAREFUL_CONSIDERATION,
    &CLOCKSPINNING,
    &CORAL_TRICKSTER,
    &CROOKCLAW_TRANSMUTER,
    &DEEP_SEA_KRAKEN,
    &DRAINING_WHELK,
    &DREAM_STALKER,
    &DRIFTER_IL_DAL,
    &ERRANT_EPHEMERON,
    &ETERNITY_SNARE,
    &FATHOM_SEER,
    &FLEDGLING_MAWCOR,
    &FOOL_S_DEMISE,
    &IXIDRON,
    &LOOTER_IL_KOR,
    &MAGUS_OF_THE_JAR,
    &MOONLACE,
    &MYSTICAL_TEACHINGS,
    &OPHIDIAN_EYE,
    &PARADOX_HAZE,
    &PSIONIC_SLIVER,
    &RIFTWING_CLOUDSKATE,
    &SAGE_OF_EPITYR,
    &SCREECHING_SLIVER,
    &SHADOW_SLIVER,
    &SLIPSTREAM_SERPENT,
    &SNAPBACK,
    &SPELL_BURST,
    &SPIKETAIL_DRAKELING,
    &SPRITE_NOBLE,
    &STORMCLOUD_DJINN,
    &TEFERI_MAGE_OF_ZHALFIR,
    &TELEKINETIC_SLIVER,
    &TEMPORAL_EDDY,
    &THINK_TWICE,
    &TOLARIAN_SENTINEL,
    &TRICKBIND,
    &TRUTH_OR_TALE,
    &VESUVAN_SHAPESHIFTER,
    &VISCERID_DEEPWALKER,
    &VOIDMAGE_HUSHER,
    &WALK_THE_AEONS,
    &WIPE_AWAY,
    &ASSASSINATE,
    &BASAL_SLIVER,
    &CALL_TO_THE_NETHERWORLD,
    &CORPULENT_CORPSE,
    &CURSE_OF_THE_CABAL,
    &CYCLOPEAN_GIANT,
    &DARK_WITHERING,
    &DEATHSPORE_THALLID,
    &DEMONIC_COLLUSION,
    &DREAD_RETURN,
    &DRUDGE_REAVERS,
    &ENDREK_SAHR_MASTER_BREEDER,
    &EVIL_EYE_OF_URBORG,
    &FACELESS_DEVOURER,
    &FALLEN_IDEAL,
    &FEEBLENESS,
    &GORGON_RECLUSE,
    &HAUNTING_HYMN,
    &LIEGE_OF_THE_PIT,
    &LIM_D_L_THE_NECROMANCER,
    &LIVING_END,
    &MAGUS_OF_THE_MIRROR,
    &MANA_SKIMMER,
    &MINDLASH_SLIVER,
    &MINDSTAB,
    &NETHER_TRAITOR,
    &NIGHTSHADE_ASSASSIN,
    &PHTHISIS,
    &PIT_KEEPER,
    &PLAGUE_SLIVER,
    &PREMATURE_BURIAL,
    &PSYCHOTIC_EPISODE,
    &SANGROPHAGE,
    &SENGIR_NOSFERATU,
    &SKITTERING_MONSTROSITY,
    &SKULKING_KNIGHT,
    &SMALLPOX,
    &STRANGLING_SOOT,
    &STRONGHOLD_OVERSEER,
    &SUDDEN_DEATH,
    &SUDDEN_SPOILING,
    &TENDRILS_OF_CORRUPTION,
    &TRAITOR_S_CLUTCH,
    &TRESPASSER_IL_VEC,
    &URBORG_SYPHON_MAGE,
    &VAMPIRIC_SLIVER,
    &VISCID_LEMURES,
    &AETHERFLAME_WALL,
    &ANCIENT_GRUDGE,
    &BARBED_SHOCKER,
    &BASALT_GARGOYLE,
    &BLAZING_BLADE_ASKARI,
    &BOGARDAN_HELLKITE,
    &BOGARDAN_RAGER,
    &BONESPLITTER_SLIVER,
    &COAL_STOKER,
    &CONFLAGRATE,
    &EMPTY_THE_WARRENS,
    &FIREMAW_KAVU,
    &FLAMECORE_ELEMENTAL,
    &FLOWSTONE_CHANNELER,
    &FORTUNE_THIEF,
    &FURY_SLIVER,
    &GHITU_FIREBREATHING,
    &GOBLIN_SKYCUTTER,
    &GRAPESHOT,
    &GREATER_GARGADON,
    &GROUND_RIFT,
    &IB_HALFHEART_GOBLIN_TACTICIAN,
    &IGNITE_MEMORIES,
    &IRONCLAW_BUZZARDIERS,
    &JAYA_BALLARD_TASK_MAGE,
    &KELDON_HALBERDIER,
    &LIGHTNING_AXE,
    &MAGUS_OF_THE_SCROLL,
    &MOGG_WAR_MARSHAL,
    &NORIN_THE_WARY,
    &ORCISH_CANNONADE,
    &PARDIC_DRAGON,
    &PLUNDER,
    &REITERATE,
    &RIFT_BOLT,
    &SEDGE_SLIVER,
    &SUBTERRANEAN_SHAMBLER,
    &SUDDEN_SHOCK,
    &SULFUROUS_BLAST,
    &TECTONIC_FIEND,
    &THICK_SKINNED_GOBLIN,
    &TWO_HEADED_SLIVER,
    &UNDYING_RAGE,
    &VIASHINO_BLADESCOUT,
    &VOLCANIC_AWAKENING,
    &WHEEL_OF_FATE,
    &WORD_OF_SEIZING,
    &AETHER_WEB,
    &ASHCOAT_BEAR,
    &ASPECT_OF_MONGOOSE,
    &CHAMELEON_BLUR,
    &DURKWOOD_BALOTH,
    &DURKWOOD_TRACKER,
    &FUNGUS_SLIVER,
    &GEMHIDE_SLIVER,
    &GLASS_ASP,
    &GREENSEEKER,
    &HAVENWOOD_WURM,
    &HERD_GNARR,
    &HYPERGENESIS,
    &KROSAN_GRIP,
    &MAGUS_OF_THE_CANDELABRA,
    &MIGHT_OF_OLD_KROSA,
    &MIGHT_SLIVER,
    &MOLDER,
    &MWONVULI_ACID_MOSS,
    &NANTUKO_SHAMAN,
    &PENDELHAVEN_ELDER,
    &PENUMBRA_SPIDER,
    &PHANTOM_WURM,
    &PRIMAL_FORCEMAGE,
    &SAVAGE_THALLID,
    &SCARWOOD_TREEFOLK,
    &SCRYB_RANGER,
    &SEARCH_FOR_TOMORROW,
    &SPECTRAL_FORCE,
    &SPIKE_TILLER,
    &SPINNERET_SLIVER,
    &SPORESOWER_THALLID,
    &SPROUT,
    &SQUALL_LINE,
    &STONEWOOD_INVOCATION,
    &STRENGTH_IN_NUMBERS,
    &THALLID_GERMINATOR,
    &THALLID_SHELL_DWELLER,
    &THELON_OF_HAVENWOOD,
    &THELONITE_HERMIT,
    &THRILL_OF_THE_HUNT,
    &TROMP_THE_DOMAINS,
    &UNYARO_BEES,
    &VERDANT_EMBRACE,
    &WORMWOOD_DRYAD,
    &WURMCALLING,
    &YAVIMAYA_DRYAD,
    &DEMENTIA_SLIVER,
    &DRALNU_LICH_LORD,
    &FIREWAKE_SLIVER,
    &GHOSTFLAME_SLIVER,
    &HARMONIC_SLIVER,
    &ITH_HIGH_ARCANIST,
    &KAERVEK_THE_MERCILESS,
    &MISHRA_ARTIFICER_PRODIGY,
    &OPALINE_SLIVER,
    &SAFFI_ERIKSDOTTER,
    &SCION_OF_THE_UR_DRAGON,
    &STONEBROW_KROSAN_HERO,
    &ASSEMBLY_WORKER,
    &BRASS_GNAT,
    &CANDLES_OF_LENG,
    &CHROMATIC_STAR,
    &CHRONATOG_TOTEM,
    &CLOCKWORK_HYDRA,
    &FORIYSIAN_TOTEM,
    &GAUNTLET_OF_POWER,
    &HIVESTONE,
    &JHOIRAS_TIMEBUG,
    &LOCKET_OF_YESTERDAYS,
    &LOTUS_BLOOM,
    &PARADISE_PLUME,
    &PHYREXIAN_TOTEM,
    &PRISMATIC_LENS,
    &SARPADIAN_EMPIRES_VOL_VII,
    &STUFFY_DOLL,
    &THUNDER_TOTEM,
    &TRISKELAVUS,
    &VENSER_S_SLIVER,
    &WEATHERSEED_TOTEM,
    &ACADEMY_RUINS,
    &CALCIFORM_POOLS,
    &DREADSHIP_REEF,
    &FLAGSTONES_OF_TROKAIR,
    &FUNGAL_REACHES,
    &GEMSTONE_CAVERNS,
    &KHER_KEEP,
    &MOLTEN_SLAGHEAP,
    &SALTCRUSTED_STEPPE,
    &SWARMYARD,
    &TERRAMORPHIC_EXPANSE,
    &URZA_S_FACTORY,
    &VESUVA,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
