//! Urza's Destiny cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, ArrivalAttachmentDef, BasicLandType, BattlefieldArrivalDef,
    BattlefieldEntryChoiceDestinationDef, BattlefieldEntryScalarChoiceDef, BlockRestrictionDef,
    BlockRestrictionMatchDef, BlockRestrictionSubjectDef, CardArt, CardChoiceSourceDef, CardRules,
    CardSet, CardSupertype, CardType, CardTypeSet, CharacteristicOperationDef, ChoiceVisibilityDef,
    ChooseDef, ChooseExactDef, ComparisonDef, ControlDurationDef, CostModificationDef,
    CostQuantityDef, CounterKind, DamageEventMatcherDef, DamagePreventionDef, DestroyFollowUpDef,
    EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef,
    ManaColor, MillUntilDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef, PayOrDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, PowerToughnessOperationDef, QuantifierDef, ReplacementChoiceDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, RevealObjectsDef, ScaledValueDef,
    SetOperationDef, SpellAdditionalCostDef, TargetChooserDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

// UDS 1 — Academy Rector
pub(in crate::card::sets) static ACADEMY_RECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4367bc78-0912-4abd-8edd-bc792558d01a"),
    "Academy Rector",
    crate::card::CardArt::new("4367bc78-0912-4abd-8edd-bc792558d01a", "Heather Hudson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, you may exile it. If you do, search your library for an enchantment card, put that card onto the battlefield, then shuffle.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                    binding: ParentBinding,
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(ParentBinding),
                                filter: None,
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ),
                        then: &EffectDef::SearchZone {
                            player: EffectRecipientDef::Controller,
                            source: ZoneKind::Library,
                            object: ObjectPredicateDef::HasType(CardType::Enchantment),
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
                    },
                },
            },
        ),
    ),
);

// UDS 2 — Archery Training
// Audit: metadata-only — The granted tap ability must read the number of arrow counters on the Aura granting it; granted abilities can currently name their own source, but not that granting permanent.
pub(in crate::card::sets) static ARCHERY_TRAINING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("151232e6-68cc-4cac-a532-9ade8e925961"),
    "Archery Training",
    crate::card::CardArt::new("151232e6-68cc-4cac-a532-9ade8e925961", "Mark Brill"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 3 — Capashen Knight (reprint)

// UDS 4 — Capashen Standard
pub(in crate::card::sets) static CAPASHEN_STANDARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16665386-405e-48c9-8c69-c21b03931c2f"),
    "Capashen Standard",
    crate::card::CardArt::new("16665386-405e-48c9-8c69-c21b03931c2f", "Todd Lockwood"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this Aura: Draw a card.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// UDS 5 — Capashen Templar
pub(in crate::card::sets) static CAPASHEN_TEMPLAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0976a193-463a-4bcb-a951-ca73347a5572"),
    "Capashen Templar",
    crate::card::CardArt::new("0976a193-463a-4bcb-a951-ca73347a5572", "Todd Lockwood"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2).with_ability(
        AbilityDef::activated(
            "{W}: This creature gets +0/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// UDS 6 — False Prophet
pub(in crate::card::sets) static FALSE_PROPHET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3853fde8-a7cf-402d-abe1-526d0f92dc32"),
    "False Prophet",
    crate::card::CardArt::new("5fcb46d3-1ddf-4e3b-9ac7-a3fee49f04c6", "Eric Peterson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Cleric"], 2, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, exile all creatures.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// UDS 7 — Fend Off
pub(in crate::card::sets) static FEND_OFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a64d7a33-986d-45ad-8662-7bca80d3628d"),
    "Fend Off",
    crate::card::CardArt::new("a64d7a33-986d-45ad-8662-7bca80d3628d", "Paolo Parente"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Prevent all combat damage that would be dealt by target creature this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// UDS 8 — Field Surgeon
pub(in crate::card::sets) static FIELD_SURGEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb830403-0832-47f7-b4b4-4f241f1b9112"),
    "Field Surgeon",
    crate::card::CardArt::new("bb830403-0832-47f7-b4b4-4f241f1b9112", "Heather Hudson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Tap an untapped creature you control: Prevent the next 1 damage that would be dealt to target creature this turn.",
            &[AbilityCostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 1,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// UDS 9 — Flicker
pub(in crate::card::sets) static FLICKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f55e7ec5-6488-483f-8020-b48e1a951f09"),
    "Flicker",
    crate::card::CardArt::new("f55e7ec5-6488-483f-8020-b48e1a951f09", "Douglas Shuler"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Exile target nontoken permanent, then return it to the battlefield under its owner's control.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
            )],
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
    ),
);

// UDS 10 — Jasmine Seer
pub(in crate::card::sets) static JASMINE_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6641dd2-5b9c-4089-8a71-a3a1a9c29f8b"),
    "Jasmine Seer",
    crate::card::CardArt::new("a6641dd2-5b9c-4089-8a71-a3a1a9c29f8b", "Donato Giancola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "{2}{W}, {T}: Reveal any number of white cards in your hand. You gain 2 life for each card revealed this way.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{W}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::White),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::BoundObjectCount(ParentBinding),
                            factor: 2,
                        }),
                    },
                ]),
            }),
        ),
    ),
);

// UDS 11 — Mask of Law and Grace
pub(in crate::card::sets) static MASK_OF_LAW_AND_GRACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6bd97150-b405-4bb5-b5a8-fceda4a45ebb"),
    "Mask of Law and Grace",
    crate::card::CardArt::new("6bd97150-b405-4bb5-b5a8-fceda4a45ebb", "Kev Walker"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has protection from black and from red.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::protection_from_color(
                            ManaColor::Black,
                        )),
                        AppliedEffectDef::add_ability(&abilities::protection_from_color(
                            ManaColor::Red,
                        )),
                    ]),
                },
            ),
        ]),
);

// UDS 12 — Master Healer
pub(in crate::card::sets) static MASTER_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e21a342c-ad80-43e4-8b2d-1c48241c52b1"),
    "Master Healer",
    crate::card::CardArt::new("e21a342c-ad80-43e4-8b2d-1c48241c52b1", "Adam Rex"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Cleric"], 1, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Prevent the next 4 damage that would be dealt to any target this turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// UDS 13 — Opalescence
pub(in crate::card::sets) static OPALESCENCE: CardRecord = CardRecord::new_with_legacy_id(
    2080,
    "Opalescence",
    CardArt::new("c8b66a4d-4ee1-40ba-993a-a56a5cbd2c3c", "John Avon"),
    CardSet::UrzasDestiny,
    // The deck's whole win condition: the enchantments it already wanted to
    // resolve stand up and attack.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Each other non-Aura enchantment is a creature in addition to its other types and has base power and base toughness each equal to its mana value.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                // Every other non-Aura enchantment. An Aura is left alone because a
                // creature Aura would fall off whatever it was attached to, and the
                // enchantment doing the animating is not one of the things it animates.
                ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Aura")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            )),
            // A creature in addition to its other types, with a body its own cost
            // decides: the number is read off each affected enchantment rather than off
            // the Opalescence.
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
                    CardTypeSet::single(CardType::Creature),
                ))),
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::SetBase {
                        power: ValueDef::AffectedManaValue,
                        toughness: ValueDef::AffectedManaValue,
                    },
                )),
            ]),
        },
    )),
);

// UDS 14 — Reliquary Monk
pub(in crate::card::sets) static RELIQUARY_MONK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("243e9386-2a7f-406a-9ed3-77d4bf1b50fd"),
    "Reliquary Monk",
    crate::card::CardArt::new("243e9386-2a7f-406a-9ed3-77d4bf1b50fd", "Thomas M. Baxa"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Monk", "Cleric"], 2, 2).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, destroy target artifact or enchantment.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// UDS 15 — Replenish
pub(in crate::card::sets) static REPLENISH: CardRecord = CardRecord::new_with_legacy_id(
    2077,
    "Replenish",
    CardArt::new("c922d401-7916-42d3-9185-9de6219f9c38", "Jim Nelson"),
    CardSet::UrzasDestiny,
    // The deck is built to fill its own graveyard first, so this is not
    // recursion so much as the whole board arriving on one turn.
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Return all enchantment cards from your graveyard to the battlefield.",
        EffectDef::MoveToZone {
            // Every enchantment card the graveyard holds, all at once. The printed
            // reminder about Auras is the ordinary rule for an Aura arriving with
            // nothing to enchant, not a clause of its own.
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Enchantment),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            ),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// UDS 16 — Sanctimony
pub(in crate::card::sets) static SANCTIMONY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfc6b744-92c7-4839-9b27-833bedb92bba"),
    "Sanctimony",
    crate::card::CardArt::new("cfc6b744-92c7-4839-9b27-833bedb92bba", "Mark Brill"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent taps a Mountain for mana, you may gain 1 life.",
        TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// UDS 17 — Scent of Jasmine
pub(in crate::card::sets) static SCENT_OF_JASMINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0dae0cf-9696-498c-8d28-dc8c239faec7"),
    "Scent of Jasmine",
    crate::card::CardArt::new("c0dae0cf-9696-498c-8d28-dc8c239faec7", "Douglas Shuler"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{W}"))
        .with_ability(AbilityDef::spell(
            "Reveal any number of white cards in your hand. You gain 2 life for each card revealed this way.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::White),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Scaled(&ScaledValueDef {
                            value: ValueDef::BoundObjectCount(ParentBinding),
                            factor: 2,
                        }),
                    },
                ]),
            }),
        )),
);

// UDS 18 — Scour
// Audit: metadata-only — SharingNameWith can find every copy, but the spell's controller must choose cards from another player's public graveyard and private hand/library; SearchZone gives hidden-zone choices to that zone's owner.
pub(in crate::card::sets) static SCOUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cac5162e-39ea-4f01-92eb-182fe23c1608"),
    "Scour",
    crate::card::CardArt::new("cac5162e-39ea-4f01-92eb-182fe23c1608", "Eric Peterson"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 19 — Serra Advocate
pub(in crate::card::sets) static SERRA_ADVOCATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cac540c7-8b3a-4e28-96a2-d414ff613640"),
    "Serra Advocate",
    crate::card::CardArt::new("cac540c7-8b3a-4e28-96a2-d414ff613640", "Scott Hampton"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: Target attacking or blocking creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
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

// UDS 20 — Solidarity
pub(in crate::card::sets) static SOLIDARITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5e1589a-ec4f-47a4-b758-ada7f49ffb8f"),
    "Solidarity",
    crate::card::CardArt::new("c5e1589a-ec4f-47a4-b758-ada7f49ffb8f", "John Zeleznik"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +0/+5 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(0),
                ValueDef::Constant(5),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// UDS 21 — Tethered Griffin
pub(in crate::card::sets) static TETHERED_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bed19fa-e497-48de-8459-030e60fdc9a8"),
    "Tethered Griffin",
    crate::card::CardArt::new("2bed19fa-e497-48de-8459-030e60fdc9a8", "Matthew D. Wilson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{W}"), &["Griffin"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if(
            "When you control no enchantments, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// UDS 22 — Tormented Angel
pub(in crate::card::sets) static TORMENTED_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00d4d751-50df-4d8f-a6d9-4e76797c429a"),
    "Tormented Angel",
    crate::card::CardArt::new(
        "00d4d751-50df-4d8f-a6d9-4e76797c429a",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 1, 5)
        .with_ability(abilities::flying()),
);

// UDS 23 — Voice of Duty
pub(in crate::card::sets) static VOICE_OF_DUTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c648e59-c872-4e04-b45f-2729b42410af"),
    "Voice of Duty",
    crate::card::CardArt::new("1c648e59-c872-4e04-b45f-2729b42410af", "Mark Zug"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Green),
    ]),
);

// UDS 24 — Voice of Reason
pub(in crate::card::sets) static VOICE_OF_REASON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed3d5a10-6d4b-4383-b400-7323f2b4670e"),
    "Voice of Reason",
    crate::card::CardArt::new("ed3d5a10-6d4b-4383-b400-7323f2b4670e", "Ray Lago"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Blue),
    ]),
);

// UDS 25 — Wall of Glare
pub(in crate::card::sets) static WALL_OF_GLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f159193-fcf8-437c-b14a-06718a446a5c"),
    "Wall of Glare",
    crate::card::CardArt::new("5f159193-fcf8-437c-b14a-06718a446a5c", "Patrick Ho"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Wall"], 0, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "This creature can block any number of creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayBlockAdditionalCreatures(
                    u8::MAX,
                )),
            },
        ),
    ]),
);

// UDS 26 — Aura Thief
pub(in crate::card::sets) static AURA_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ae591d2-b9d3-4bc5-bcec-5d3d79a13b41"),
    "Aura Thief",
    crate::card::CardArt::new("8ae591d2-b9d3-4bc5-bcec-5d3d79a13b41", "Ron Spears"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Illusion"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, you gain control of all enchantments.",
            EffectDef::GainControl {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                controller: PlayerRefDef::EffectController,
                duration: ControlDurationDef::Indefinitely,
            },
        ),
    ]),
);

// UDS 27 — Blizzard Elemental
pub(in crate::card::sets) static BLIZZARD_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5949c5a7-9656-466a-add8-1800973fefee"),
    "Blizzard Elemental",
    crate::card::CardArt::new("5949c5a7-9656-466a-add8-1800973fefee", "Thomas M. Baxa"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Elemental"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{U}: Untap this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// UDS 28 — Brine Seer
pub(in crate::card::sets) static BRINE_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f6e5575-b004-417f-9366-6ba7840a79e7"),
    "Brine Seer",
    crate::card::CardArt::new("2f6e5575-b004-417f-9366-6ba7840a79e7", "Donato Giancola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{U}, {T}: Reveal any number of blue cards in your hand. Counter target spell unless its controller pays {1} for each card revealed this way.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::PayOr(PayOrDef {
                        payment: EffectPaymentDef::generic_mana(
                            PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                                TargetIndex::PRIMARY,
                            ))),
                            ValueDef::BoundObjectCount(ParentBinding),
                        ),
                        if_paid: None,
                        otherwise: Some(&EffectDef::Counter {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                        }),
                        visibility: ChoiceVisibilityDef::Private,
                        condition: None,
                    }),
                ]),
            }),
        ),
    ),
);

// UDS 29 — Bubbling Beebles
pub(in crate::card::sets) static BUBBLING_BEEBLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("002cf7d8-3fc2-48eb-a727-a1ce5a049665"),
    "Bubbling Beebles",
    crate::card::CardArt::new("002cf7d8-3fc2-48eb-a727-a1ce5a049665", "Jeff Miracola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Beeble"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked as long as defending player controls an enchantment.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::DefendingPlayer,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                },
            },
        ),
    ),
);

// UDS 30 — Disappear
pub(in crate::card::sets) static DISAPPEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdf280f4-74a1-4e6f-aec6-1852f04204e4"),
    "Disappear",
    crate::card::CardArt::new(
        "bdf280f4-74a1-4e6f-aec6-1852f04204e4",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::activated(
                "{U}: Return enchanted creature and this Aura to their owners' hands.",
                &[AbilityCostDef::Mana(mana_cost!("{U}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Source,
                                ObjectPredicateDef::AttachedToSource,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// UDS 31 — Donate
pub(in crate::card::sets) static DONATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f6d8ce9-f8c8-45ad-b74c-97fba0e2982e"),
    "Donate",
    crate::card::CardArt::new("7f6d8ce9-f8c8-45ad-b74c-97fba0e2982e", "Jeff Miracola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains control of target permanent you control.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any)),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
        ],
        EffectDef::GainControl {
            object: EffectRecipientDef::Target(TargetIndex(1)),
            controller: PlayerRefDef::Target(TargetIndex::PRIMARY),
            duration: ControlDurationDef::Indefinitely,
        },
    )),
);

// UDS 32 — Fatigue
// Audit: metadata-only — Needs a player-scoped effect that skips that player's next draw step; only permanent untap-step skipping is currently modeled.
pub(in crate::card::sets) static FATIGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("660fb109-dd65-4410-99b9-a2a14f8ea202"),
    "Fatigue",
    crate::card::CardArt::new("660fb109-dd65-4410-99b9-a2a14f8ea202", "Jeff Miracola"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 33 — Fledgling Osprey
pub(in crate::card::sets) static FLEDGLING_OSPREY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8cd46bfa-ca09-422f-9891-db9399fa2d3a"),
    "Fledgling Osprey",
    crate::card::CardArt::new("8cd46bfa-ca09-422f-9891-db9399fa2d3a", "Heather Hudson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{U}"), &["Bird"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature has flying as long as it's enchanted.",
            EffectDef::IfCondition {
                condition: &crate::card::TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Enchanted,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            },
        ),
    ),
);

// UDS 34 — Illuminated Wings
pub(in crate::card::sets) static ILLUMINATED_WINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f98e703-a0d3-497f-840a-aa026b02d47f"),
    "Illuminated Wings",
    crate::card::CardArt::new("7f98e703-a0d3-497f-840a-aa026b02d47f", "Jim Nelson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this Aura: Draw a card.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// UDS 35 — Iridescent Drake
pub(in crate::card::sets) static IRIDESCENT_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70cbc36d-3391-4086-9b81-fb1ef0b83046"),
    "Iridescent Drake",
    crate::card::CardArt::new("70cbc36d-3391-4086-9b81-fb1ef0b83046", "Jim Nelson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Drake"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put target Aura card from a graveyard onto the battlefield under your control attached to this creature.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Subtype("Aura"),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                arrival: BattlefieldArrivalDef {
                    controller: Some(PlayerRelation::You),
                    attachment: Some(ArrivalAttachmentDef::ArrivalToHost(ObjectRefDef::Source)),
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        ),
    ]),
);

// UDS 36 — Kingfisher
pub(in crate::card::sets) static KINGFISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("442bc3ba-00b3-4616-a5b2-55524ff8a736"),
    "Kingfisher",
    crate::card::CardArt::new(
        "442bc3ba-00b3-4616-a5b2-55524ff8a736",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// UDS 37 — Mental Discipline
// Audit: metadata-only — DiscardCards can describe the activation cost, but the shared activation runtime cannot yet ask the controller to choose and discard a card atomically while paying that cost.
pub(in crate::card::sets) static MENTAL_DISCIPLINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e9ffd83-b5c9-46b4-bc5a-172ca34ddc79"),
    "Mental Discipline",
    crate::card::CardArt::new(
        "5e9ffd83-b5c9-46b4-bc5a-172ca34ddc79",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::unsupported(),
);

// UDS 38 — Metathran Elite
pub(in crate::card::sets) static METATHRAN_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa941f17-1b81-4017-90ae-4466eba8da2f"),
    "Metathran Elite",
    crate::card::CardArt::new("aa941f17-1b81-4017-90ae-4466eba8da2f", "Jim Nelson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Metathran", "Soldier"], 2, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked as long as it's enchanted.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Enchanted,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                },
            },
        ),
    ),
);

// UDS 39 — Metathran Soldier
pub(in crate::card::sets) static METATHRAN_SOLDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("650d40d0-78ec-4b6e-8ea0-28d43ce175d5"),
    "Metathran Soldier",
    crate::card::CardArt::new("650d40d0-78ec-4b6e-8ea0-28d43ce175d5", "Paolo Parente"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Metathran", "Soldier"], 1, 1).with_ability(
        abilities::cannot_be_blocked("This creature can't be blocked."),
    ),
);

// UDS 40 — Opposition
pub(in crate::card::sets) static OPPOSITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95be2701-af7c-483e-8165-e8bd4b2774ed"),
    "Opposition",
    crate::card::CardArt::new("95be2701-af7c-483e-8165-e8bd4b2774ed", "Todd Lockwood"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_ability(
        AbilityDef::activated_with_targets(
            "Tap an untapped creature you control: Tap target artifact, creature, or land.",
            &[AbilityCostDef::TapPermanents {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
                count: 1,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// UDS 41 — Private Research
pub(in crate::card::sets) static PRIVATE_RESEARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f7f9849-a4bd-4501-9473-79345c751701"),
    "Private Research",
    crate::card::CardArt::new("6f7f9849-a4bd-4501-9473-79345c751701", "Scott M. Fischer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, you may put a page counter on this Aura.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("page"),
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature dies, draw a card for each page counter on this Aura.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountersOnSource(CounterKind::named("page")),
                },
            ),
        ]),
);

// UDS 42 — Quash
// Audit: metadata-only — Counter can handle the target spell and SharingNameWith can find its copies, but the spell's controller must choose cards from another player's public graveyard and private hand/library; SearchZone gives hidden-zone choices to that zone's owner.
pub(in crate::card::sets) static QUASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62019ac4-a5a1-4a8c-bfb4-96e818949bbe"),
    "Quash",
    crate::card::CardArt::new("62019ac4-a5a1-4a8c-bfb4-96e818949bbe", "Don Hazeltine"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 43 — Rayne, Academy Chancellor
pub(in crate::card::sets) static RAYNE_ACADEMY_CHANCELLOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ee6480c-7697-47e2-893b-ca88c0ab3376"),
    "Rayne, Academy Chancellor",
    crate::card::CardArt::new("8ee6480c-7697-47e2-893b-ca88c0ab3376", "Matthew D. Wilson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::triggered(
            "Whenever you or a permanent you control becomes the target of a spell or ability an opponent controls, you may draw a card. You may draw an additional card if Rayne is enchanted.",
            TriggerEventDef::YouOrYourPermanentBecomesTarget(
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ),
            EffectDef::Sequence(&[
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Enchanted,
                    },
                    then: &EffectDef::May {
                        player: EffectRecipientDef::Controller,
                        effect: &EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    },
                },
            ]),
        )),
);

// UDS 44 — Rescue
pub(in crate::card::sets) static RESCUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63fc979e-7758-4310-9259-659e9ced2c7f"),
    "Rescue",
    crate::card::CardArt::new("63fc979e-7758-4310-9259-659e9ced2c7f", "Greg Staples"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target permanent you control to its owner's hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// UDS 45 — Scent of Brine
pub(in crate::card::sets) static SCENT_OF_BRINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d117bf8d-23ec-4f9d-99d0-3a990c5f7075"),
    "Scent of Brine",
    crate::card::CardArt::new("d117bf8d-23ec-4f9d-99d0-3a990c5f7075", "Greg Staples"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{1}{U}"))
        .with_ability(AbilityDef::spell_with_targets(
            "Reveal any number of blue cards in your hand. Counter target spell unless its controller pays {1} for each card revealed this way.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::PayOr(PayOrDef {
                        payment: EffectPaymentDef::generic_mana(
                            PlayerSetDef::One(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                                TargetIndex::PRIMARY,
                            ))),
                            ValueDef::BoundObjectCount(ParentBinding),
                        ),
                        if_paid: None,
                        otherwise: Some(&EffectDef::Counter {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                        }),
                        visibility: ChoiceVisibilityDef::Private,
                        condition: None,
                    }),
                ]),
            }),
        )),
);

// UDS 46 — Sigil of Sleep
pub(in crate::card::sets) static SIGIL_OF_SLEEP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a31f3c70-e2c3-479e-8d22-2fd1429e9857"),
    "Sigil of Sleep",
    crate::card::CardArt::new(
        "a31f3c70-e2c3-479e-8d22-2fd1429e9857",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered_with_targets(
                "Whenever enchanted creature deals damage to a player, return target creature that player controls to its owner's hand.",
                TriggerEventDef::damage_to_player(
                    ObjectPredicateDef::AttachedToSource,
                    PlayerRelation::Any,
                ),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::EventPlayer),
                    owner: None,
                })],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// UDS 47 — Telepathic Spies
pub(in crate::card::sets) static TELEPATHIC_SPIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("769e7f64-e32c-4242-aae9-45d50b89ff1f"),
    "Telepathic Spies",
    crate::card::CardArt::new("769e7f64-e32c-4242-aae9-45d50b89ff1f", "Thomas M. Baxa"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, look at target opponent's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LookAtHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// UDS 48 — Temporal Adept
pub(in crate::card::sets) static TEMPORAL_ADEPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07bb9695-a8b0-47b4-9a03-11d559412f33"),
    "Temporal Adept",
    crate::card::CardArt::new("07bb9695-a8b0-47b4-9a03-11d559412f33", "Heather Hudson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{U}{U}{U}, {T}: Return target permanent to its owner's hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}{U}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// UDS 49 — Thieving Magpie
pub(in crate::card::sets) static THIEVING_MAGPIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b6b23b9-4569-40ff-988f-ad1d5d3fe573"),
    "Thieving Magpie",
    crate::card::CardArt::new("2b6b23b9-4569-40ff-988f-ad1d5d3fe573", "Una Fricker"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Bird"], 1, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, draw a card.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// UDS 50 — Treachery
pub(in crate::card::sets) static TREACHERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("613694aa-b169-400d-8063-2b83d8303611"),
    "Treachery",
    crate::card::CardArt::new("613694aa-b169-400d-8063-2b83d8303611", "Matthew D. Wilson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "You control enchanted creature.",
                EffectDef::GainControl {
                    object: EffectRecipientDef::AttachedPermanent,
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, untap up to five lands.",
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 5,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Untap {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    },
                }),
            ),
        ]),
);

// UDS 51 — Apprentice Necromancer
pub(in crate::card::sets) static APPRENTICE_NECROMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d7cc1f6-9897-4de4-8e94-40cbe2d962a2"),
    "Apprentice Necromancer",
    crate::card::CardArt::new("6d7cc1f6-9897-4de4-8e94-40cbe2d962a2", "Pete Venters"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, {T}, Sacrifice this creature: Return target creature card from your graveyard to the battlefield. That creature gains haste. At the beginning of the next end step, sacrifice it.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    arrival: BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
                binding: ParentBinding,
                then: &EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::binding_zone_change_successors(
                            ParentBinding,
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::haste()),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "Sacrifice the creature at the beginning of the next end step.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::SacrificeYours {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                        },
                    ))),
                ]),
            },
        ),
    ),
);

// UDS 52 — Attrition
pub(in crate::card::sets) static ATTRITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3eb615b-249d-433f-a521-8310e8784b5d"),
    "Attrition",
    crate::card::CardArt::new("e3eb615b-249d-433f-a521-8310e8784b5d", "Scott M. Fischer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, Sacrifice a creature: Destroy target nonblack creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// UDS 53 — Body Snatcher
pub(in crate::card::sets) static BODY_SNATCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7d4c858-5a11-485d-a514-12a6d80459f0"),
    "Body Snatcher",
    crate::card::CardArt::new("c7d4c858-5a11-485d-a514-12a6d80459f0", "Mark Zug"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Minion"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, exile it unless you discard a creature card.",
                EffectDef::PayOr(PayOrDef {
                    payment: EffectPaymentDef {
                        payer: PlayerSetDef::Related(PlayerRelation::You),
                        cost: EffectPaymentCostDef::DiscardMatching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ),
                    },
                    if_paid: None,
                    otherwise: Some(&EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    }),
                    visibility: ChoiceVisibilityDef::Private,
                    condition: None,
                }),
            ),
            abilities::dies_trigger_with_targets(
                "When this creature dies, exile it and return target creature card from your graveyard to the battlefield.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                })],
                EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::WithBattlefieldArrival {
                        effect: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Battlefield,
                            placement: ZonePlacement::Top,
                        },
                        arrival: BattlefieldArrivalDef {
                            controller: Some(PlayerRelation::You),
                            ..BattlefieldArrivalDef::DEFAULT
                        },
                    },
                ]),
            ),
        ]),
);

// UDS 54 — Bubbling Muck
// Audit: metadata-only — InstallTrigger can create an ordinary delayed trigger, but the immediate-resolution mana path rejects an installed triggered mana ability.
pub(in crate::card::sets) static BUBBLING_MUCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ca76614-78a1-4535-9162-70469d1e8a13"),
    "Bubbling Muck",
    crate::card::CardArt::new(
        "6ca76614-78a1-4535-9162-70469d1e8a13",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::unsupported(),
);

// UDS 55 — Carnival of Souls
pub(in crate::card::sets) static CARNIVAL_OF_SOULS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("847340fb-9251-4439-b33b-f86bff507dcd"),
    "Carnival of Souls",
    crate::card::CardArt::new("847340fb-9251-4439-b33b-f86bff507dcd", "Brian Snõddy"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a creature enters, you lose 1 life and add {B}.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::HasType(CardType::Creature),
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            EffectDef::AddManaEqualTo {
                color: ManaColor::Black,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// UDS 56 — Chime of Night
pub(in crate::card::sets) static CHIME_OF_NIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ec7c917-b254-4643-afc2-b6387f267469"),
    "Chime of Night",
    crate::card::CardArt::new("1ec7c917-b254-4643-afc2-b6387f267469", "Pete Venters"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::dies_trigger_with_targets(
                "When this Aura is put into a graveyard from the battlefield, destroy target nonblack creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]),
                )],
                EffectDef::destroy_target(TargetIndex::PRIMARY, true),
            ),
        ]),
);

// UDS 57 — Disease Carriers
pub(in crate::card::sets) static DISEASE_CARRIERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49125cfc-dbae-4543-9d2d-4cc78f45ce9a"),
    "Disease Carriers",
    crate::card::CardArt::new(
        "49125cfc-dbae-4543-9d2d-4cc78f45ce9a",
        "Chippy & Matthew D. Wilson",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Rat"], 2, 2).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, target creature gets -2/-2 until end of turn.",
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
    ),
);

// UDS 58 — Dying Wail
pub(in crate::card::sets) static DYING_WAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a25a472-495e-4062-b66f-c37f148b494f"),
    "Dying Wail",
    crate::card::CardArt::new("2a25a472-495e-4062-b66f-c37f148b494f", "Brian Snõddy"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered_with_targets(
                "When enchanted creature dies, target player discards two cards.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                    selection: crate::card::DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
        ]),
);

// UDS 59 — Encroach
pub(in crate::card::sets) static ENCROACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fbd48dac-0a1a-49c4-8daf-11972b990454"),
    "Encroach",
    crate::card::CardArt::new(
        "fbd48dac-0a1a-49c4-8daf-11972b990454",
        "rk post & Mark Tedin",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{B}"))
        .with_ability(AbilityDef::spell_with_targets(
            "Target player reveals their hand. You choose a nonbasic land card from it. That player discards that card.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                        CardSupertype::Basic,
                    )),
                ]),
            )),
        )),
);

// UDS 60 — Eradicate
// Audit: metadata-only — SharingNameWith can find every copy, but the spell's controller must choose cards from another player's public graveyard and private hand/library; SearchZone gives hidden-zone choices to that zone's owner.
pub(in crate::card::sets) static ERADICATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fad4607-c11d-4407-b5fa-bd34f74e41b3"),
    "Eradicate",
    crate::card::CardArt::new("0fad4607-c11d-4407-b5fa-bd34f74e41b3", "Kev Walker"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 61 — Festering Wound
pub(in crate::card::sets) static FESTERING_WOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("927eed13-510b-4b06-811d-91a6a069cb8c"),
    "Festering Wound",
    crate::card::CardArt::new(
        "927eed13-510b-4b06-811d-91a6a069cb8c",
        "Chippy & Matthew D. Wilson",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, you may put an infection counter on this Aura.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("infection"),
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
            AbilityDef::triggered(
                "At the beginning of the upkeep of enchanted creature's controller, this Aura deals X damage to that player, where X is the number of infection counters on this Aura.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::ControllerOfAttachedPermanent,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::CountersOnSource(CounterKind::named("infection")),
                },
            ),
        ]),
);

// UDS 62 — Lurking Jackals
pub(in crate::card::sets) static LURKING_JACKALS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97d082d8-f401-47ad-845c-77776ee647ba"),
    "Lurking Jackals",
    crate::card::CardArt::new("97d082d8-f401-47ad-845c-77776ee647ba", "Greg Staples"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_ability(AbilityDef::triggered_if(
            "When an opponent has 10 or less life, if this permanent is an enchantment, it becomes a 3/2 Jackal creature.",
            TriggerEventDef::StateCondition,
            &TriggerConditionDef::All(&[
                TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeTotal(PlayerRelation::Opponent),
                    comparison: ComparisonDef::LessOrEqual,
                    right: ValueDef::Constant(10),
                }),
                TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                },
            ]),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
                        SetOperationDef::Set(CardTypeSet::single(CardType::Creature)),
                    )),
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        SetOperationDef::Set(&["Jackal"]),
                    )),
                    AppliedEffectDef::Characteristic(
                        CharacteristicOperationDef::PowerToughness(
                            PowerToughnessOperationDef::SetBase {
                                power: ValueDef::Constant(3),
                                toughness: ValueDef::Constant(2),
                            },
                        ),
                    ),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        )),
);

// UDS 63 — Nightshade Seer
pub(in crate::card::sets) static NIGHTSHADE_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2262467-1354-4aec-84a2-21916c44b9ef"),
    "Nightshade Seer",
    crate::card::CardArt::new("e2262467-1354-4aec-84a2-21916c44b9ef", "Donato Giancola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{B}, {T}: Reveal any number of black cards in your hand. Target creature gets -X/-X until end of turn, where X is the number of cards revealed this way.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Black),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Negate(&ValueDef::BoundObjectCount(ParentBinding)),
                            ValueDef::Negate(&ValueDef::BoundObjectCount(ParentBinding)),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            }),
        ),
    ),
);

// UDS 64 — Phyrexian Monitor
pub(in crate::card::sets) static PHYREXIAN_MONITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("571058b0-7e10-4259-8db9-5c8b78c1e13d"),
    "Phyrexian Monitor",
    crate::card::CardArt::new("571058b0-7e10-4259-8db9-5c8b78c1e13d", "Carl Critchlow"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Phyrexian", "Skeleton"], 2, 2).with_ability(
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ),
);

// UDS 65 — Phyrexian Negator
pub(in crate::card::sets) static PHYREXIAN_NEGATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45a02d67-5931-49ae-a28e-57aa6f9c7f83"),
    "Phyrexian Negator",
    crate::card::CardArt::new("45a02d67-5931-49ae-a28e-57aa6f9c7f83", "John Zeleznik"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Horror"], 5, 5).with_abilities(
        &[
            abilities::trample(),
            AbilityDef::triggered(
                "Whenever this creature is dealt damage, sacrifice that many permanents.",
                TriggerEventDef::damage_to_source(),
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::Any,
                    count: ValueDef::TriggerEventAmount,
                    then: None,
                    amount: crate::card::SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
        ],
    ),
);

// UDS 66 — Plague Dogs
pub(in crate::card::sets) static PLAGUE_DOGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b9cebd8-aa3f-4e22-8d15-d4b7bad355e4"),
    "Plague Dogs",
    crate::card::CardArt::new(
        "6b9cebd8-aa3f-4e22-8d15-d4b7bad355e4",
        "Chippy & Matthew D. Wilson",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Phyrexian", "Zombie", "Dog"], 3, 3)
        .with_abilities(&[
            abilities::dies_trigger(
                "When this creature dies, all creatures get -1/-1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this creature: Draw a card.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// UDS 67 — Rapid Decay
// Audit: metadata-only — Multi-card targets can express “up to three,” but cannot require every chosen card to come from one shared graveyard chosen by the caster.
pub(in crate::card::sets) static RAPID_DECAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1678d911-1456-4631-a2f4-d7de4906644b"),
    "Rapid Decay",
    crate::card::CardArt::new("1678d911-1456-4631-a2f4-d7de4906644b", "Chippy"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 68 — Ravenous Rats (reprint)

// UDS 69 — Scent of Nightshade
pub(in crate::card::sets) static SCENT_OF_NIGHTSHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("582468a6-0ea9-411e-a694-13977d47c877"),
    "Scent of Nightshade",
    crate::card::CardArt::new("582468a6-0ea9-411e-a694-13977d47c877", "John Avon"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{1}{B}"))
        .with_ability(AbilityDef::spell_with_targets(
            "Reveal any number of black cards in your hand. Target creature gets -X/-X until end of turn, where X is the number of cards revealed this way.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Black),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Negate(&ValueDef::BoundObjectCount(ParentBinding)),
                            ValueDef::Negate(&ValueDef::BoundObjectCount(ParentBinding)),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            }),
        )),
);

// UDS 70 — Skittering Horror
pub(in crate::card::sets) static SKITTERING_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80cd2771-9681-4b4a-8c2c-a2ffd7361c35"),
    "Skittering Horror",
    crate::card::CardArt::new("80cd2771-9681-4b4a-8c2c-a2ffd7361c35", "Mark Zug"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Horror"], 4, 3).with_ability(
        AbilityDef::triggered(
            "When you cast a creature spell, sacrifice this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// UDS 71 — Slinking Skirge
pub(in crate::card::sets) static SLINKING_SKIRGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00522c4b-4e64-4403-96b1-df41afbe255f"),
    "Slinking Skirge",
    crate::card::CardArt::new("00522c4b-4e64-4403-96b1-df41afbe255f", "Ron Spencer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Phyrexian", "Imp"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{2}, Sacrifice this creature: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// UDS 72 — Soul Feast
pub(in crate::card::sets) static SOUL_FEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9417a9db-5101-4fe1-84b7-283ca1fd42e5"),
    "Soul Feast",
    crate::card::CardArt::new("9417a9db-5101-4fe1-84b7-283ca1fd42e5", "Ray Lago"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player loses 4 life and you gain 4 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// UDS 73 — Squirming Mass
pub(in crate::card::sets) static SQUIRMING_MASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e47793a3-9a98-4733-8d6a-2fb1a67b15c9"),
    "Squirming Mass",
    crate::card::CardArt::new("e47793a3-9a98-4733-8d6a-2fb1a67b15c9", "Ron Spencer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Horror"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "Fear (This creature can't be blocked except by artifact creatures and/or black creatures.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::prohibit(
                        BlockRestrictionSubjectDef::Attacker,
                        BlockRestrictionMatchDef::Except(ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::Color(ManaColor::Black),
                        ])),
                    ),
                )),
            },
        ),
    ),
);

// UDS 74 — Twisted Experiment
pub(in crate::card::sets) static TWISTED_EXPERIMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64e37889-7dc0-476b-8b99-8f06881d352c"),
    "Twisted Experiment",
    crate::card::CardArt::new("64e37889-7dc0-476b-8b99-8f06881d352c", "rk post"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
        ]),
);

// UDS 75 — Yawgmoth's Bargain
// Audit: metadata-only — Needs a replacement effect that skips the controller's draw step; the activated life payment and draw are otherwise supported.
pub(in crate::card::sets) static YAWGMOTH_S_BARGAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86901bf2-7722-43f8-b879-7a30630371fa"),
    "Yawgmoth's Bargain",
    crate::card::CardArt::new("86901bf2-7722-43f8-b879-7a30630371fa", "Michael Sutfin"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 76 — Aether Sting
pub(in crate::card::sets) static AETHER_STING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66a09917-50ce-4b51-a5cf-e28e88a45762"),
    "Aether Sting",
    crate::card::CardArt::new("66a09917-50ce-4b51-a5cf-e28e88a45762", "Pete Venters"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a creature spell, this enchantment deals 1 damage to that player.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// UDS 77 — Bloodshot Cyclops
// Audit: metadata-only — SacrificePermanent can pay the activation cost, but it does not expose the sacrificed creature's power to the resolving damage amount.
pub(in crate::card::sets) static BLOODSHOT_CYCLOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9320f3d8-0e51-43d0-aedb-bfed771101e9"),
    "Bloodshot Cyclops",
    crate::card::CardArt::new("9320f3d8-0e51-43d0-aedb-bfed771101e9", "Ray Lago"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 78 — Cinder Seer
pub(in crate::card::sets) static CINDER_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d96e7522-e0bc-4e23-8e4b-40a0c28ea986"),
    "Cinder Seer",
    crate::card::CardArt::new("d96e7522-e0bc-4e23-8e4b-40a0c28ea986", "Donato Giancola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{R}, {T}: Reveal any number of red cards in your hand. This creature deals X damage to any target, where X is the number of cards revealed this way.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Red),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::BoundObjectCount(ParentBinding),
                    },
                ]),
            }),
        ),
    ),
);

// UDS 79 — Colos Yearling
pub(in crate::card::sets) static COLOS_YEARLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d68eb62-9f86-4c85-8696-46a248c744ff"),
    "Colos Yearling",
    crate::card::CardArt::new("1d68eb62-9f86-4c85-8696-46a248c744ff", "Patrick Ho"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goat", "Beast"], 1, 1).with_abilities(&[
        abilities::mountainwalk(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
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

// UDS 80 — Covetous Dragon
pub(in crate::card::sets) static COVETOUS_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5f37e36-c004-4b89-a668-5cd984c59019"),
    "Covetous Dragon",
    crate::card::CardArt::new("c5f37e36-c004-4b89-a668-5cd984c59019", "rk post"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Dragon"], 6, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if(
            "When you control no artifacts, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// UDS 81 — Flame Jet
pub(in crate::card::sets) static FLAME_JET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a511f9df-b53b-4fea-87cd-9f18f6833f92"),
    "Flame Jet",
    crate::card::CardArt::new("a511f9df-b53b-4fea-87cd-9f18f6833f92", "John Avon"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Flame Jet deals 3 damage to target player or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// UDS 82 — Goblin Berserker
pub(in crate::card::sets) static GOBLIN_BERSERKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3c7635d-98b2-4505-9153-d7e9e53ea16d"),
    "Goblin Berserker",
    crate::card::CardArt::new("a3c7635d-98b2-4505-9153-d7e9e53ea16d", "Christopher Rush"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Berserker"], 2, 2)
        .with_abilities(&[abilities::first_strike(), abilities::haste()]),
);

// UDS 83 — Goblin Festival
// Audit: metadata-only — Randomized can model the coin flip, but a failed flip requires its controller to choose one opponent to gain control in multiplayer rather than using a fixed opponent relation.
pub(in crate::card::sets) static GOBLIN_FESTIVAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac067eb8-427f-4bfa-b392-0bb41ac8370e"),
    "Goblin Festival",
    crate::card::CardArt::new("ac067eb8-427f-4bfa-b392-0bb41ac8370e", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 84 — Goblin Gardener
pub(in crate::card::sets) static GOBLIN_GARDENER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7eab0544-9c0b-4365-86bb-bc0c3e9d87ce"),
    "Goblin Gardener",
    crate::card::CardArt::new("7eab0544-9c0b-4365-86bb-bc0c3e9d87ce", "Dan Frazier"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, destroy target land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// UDS 85 — Goblin Marshal
pub(in crate::card::sets) static GOBLIN_MARSHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a85b2f9-c12c-46dd-ae04-470ebf5ec6d9"),
    "Goblin Marshal",
    crate::card::CardArt::new("6a85b2f9-c12c-46dd-ae04-470ebf5ec6d9", "DiTerlizzi"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Goblin", "Warrior"], 3, 3)
        .with_abilities(&[
            abilities::echo(
                "Echo {4}{R}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
                mana_cost!("{4}{R}{R}"),
            ),
            AbilityDef::triggered(
                "When this creature enters or dies, create two 1/1 red Goblin creature tokens.",
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
                EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
                    .with_count(ValueDef::Constant(2)),
            ),
        ]),
);

// UDS 86 — Goblin Masons
pub(in crate::card::sets) static GOBLIN_MASONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("124070d9-c362-4053-a405-9438b1cfac02"),
    "Goblin Masons",
    crate::card::CardArt::new("124070d9-c362-4053-a405-9438b1cfac02", "DiTerlizzi"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 2, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, destroy target Wall.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Subtype("Wall"),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// UDS 87 — Hulking Ogre
pub(in crate::card::sets) static HULKING_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0676d39e-229f-480b-874e-ff0cb8e335d8"),
    "Hulking Ogre",
    crate::card::CardArt::new(
        "0676d39e-229f-480b-874e-ff0cb8e335d8",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ogre"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ),
);

// UDS 88 — Impatience
pub(in crate::card::sets) static IMPATIENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d39c8166-9e63-4b02-af7b-4caf14ca73ac"),
    "Impatience",
    crate::card::CardArt::new("d39c8166-9e63-4b02-af7b-4caf14ca73ac", "Mark Brill"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_ability(AbilityDef::triggered_if(
            "At the beginning of each player's end step, if that player didn't cast a spell this turn, this enchantment deals 2 damage to that player.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &TriggerConditionDef::SpellsCastThisTurn {
                quantifier: QuantifierDef::Any,
                player: PlayerRelation::EventPlayer,
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        )),
);

// UDS 89 — Incendiary
pub(in crate::card::sets) static INCENDIARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("854f4775-29b1-4ed1-94d9-db5930a35157"),
    "Incendiary",
    crate::card::CardArt::new("854f4775-29b1-4ed1-94d9-db5930a35157", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, you may put a fuse counter on this Aura.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("fuse"),
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
            AbilityDef::triggered_with_targets(
                "When enchanted creature dies, this Aura deals X damage to any target, where X is the number of fuse counters on this Aura.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::CountersOnSource(CounterKind::named("fuse")),
                },
            ),
        ]),
);

// UDS 90 — Keldon Champion
pub(in crate::card::sets) static KELDON_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1eee4d2-fe28-418e-a81f-73a66e831b05"),
    "Keldon Champion",
    crate::card::CardArt::new("b1eee4d2-fe28-418e-a81f-73a66e831b05", "Mark Tedin"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Barbarian"], 3, 2)
        .with_abilities(&[
            abilities::haste(),
            abilities::echo(
                "Echo {2}{R}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
                mana_cost!("{2}{R}{R}"),
            ),
            abilities::enters_trigger_with_targets(
                "When this creature enters, it deals 3 damage to target player or planeswalker.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// UDS 91 — Keldon Vandals
pub(in crate::card::sets) static KELDON_VANDALS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f18cdf4d-42ce-4f2d-8b8f-8cf52a1b8db4"),
    "Keldon Vandals",
    crate::card::CardArt::new("f18cdf4d-42ce-4f2d-8b8f-8cf52a1b8db4", "Greg Staples"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Rogue"], 4, 1).with_abilities(&[
        abilities::echo(
            "Echo {2}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{2}{R}"),
        ),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target artifact.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// UDS 92 — Landslide
pub(in crate::card::sets) static LANDSLIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0ddc0dc-8783-4659-bbbd-db6698843b47"),
    "Landslide",
    crate::card::CardArt::new("c0ddc0dc-8783-4659-bbbd-db6698843b47", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{R}"))
        .with_ability(AbilityDef::spell_with_additional_cost(
            "Sacrifice any number of Mountains. Landslide deals that much damage to target player or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                CostQuantityDef::ChosenX,
            ),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        )),
);

// UDS 93 — Mark of Fury
pub(in crate::card::sets) static MARK_OF_FURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b21b2f4-a05d-477b-8a39-632f7ff7f5f5"),
    "Mark of Fury",
    crate::card::CardArt::new("5b21b2f4-a05d-477b-8a39-632f7ff7f5f5", "Thomas M. Baxa"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of the end step, return this Aura to its owner's hand.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// UDS 94 — Reckless Abandon
pub(in crate::card::sets) static RECKLESS_ABANDON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f335d43-cacb-40ad-93c1-9a861e9f66c7"),
    "Reckless Abandon",
    crate::card::CardArt::new("8f335d43-cacb-40ad-93c1-9a861e9f66c7", "Ron Spears"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nReckless Abandon deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// UDS 95 — Repercussion
// Audit: metadata-only — Damage triggers can capture one explicit damaged object, but the runtime does not support a trigger matcher over every creature recipient while preserving a separate trigger and damage amount for each creature dealt damage.
pub(in crate::card::sets) static REPERCUSSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0f3c78e-16c0-4fbc-8ef4-fbf610f9d464"),
    "Repercussion",
    crate::card::CardArt::new("d0f3c78e-16c0-4fbc-8ef4-fbf610f9d464", "Michael Sutfin"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::unsupported(),
);

// UDS 96 — Scent of Cinder
pub(in crate::card::sets) static SCENT_OF_CINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c030eca0-bc5f-403b-8600-1f295fc85fee"),
    "Scent of Cinder",
    crate::card::CardArt::new("c030eca0-bc5f-403b-8600-1f295fc85fee", "Marc Fishman"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{1}{R}"))
        .with_ability(AbilityDef::spell_with_targets(
            "Reveal any number of red cards in your hand. Scent of Cinder deals X damage to any target, where X is the number of cards revealed this way.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Red),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::BoundObjectCount(ParentBinding),
                    },
                ]),
            }),
        )),
);

// UDS 97 — Sowing Salt
// Audit: metadata-only — SharingNameWith can find every copy, but the spell's controller must choose cards from another player's public graveyard and private hand/library; SearchZone gives hidden-zone choices to that zone's owner.
pub(in crate::card::sets) static SOWING_SALT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de2f7251-f71a-47d2-a779-c898d94e807c"),
    "Sowing Salt",
    crate::card::CardArt::new("de2f7251-f71a-47d2-a779-c898d94e807c", "Todd Lockwood"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 98 — Trumpet Blast (reprint)

// UDS 99 — Wake of Destruction
pub(in crate::card::sets) static WAKE_OF_DESTRUCTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c070f12-0342-48d5-ab0e-4fc4701c3669"),
    "Wake of Destruction",
    crate::card::CardArt::new("0c070f12-0342-48d5-ab0e-4fc4701c3669", "Todd Lockwood"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Destroy target land and all other lands with the same name as that land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// UDS 100 — Wild Colos
pub(in crate::card::sets) static WILD_COLOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d39f746-7b82-476a-9774-3375debb47bd"),
    "Wild Colos",
    crate::card::CardArt::new("2d39f746-7b82-476a-9774-3375debb47bd", "Marc Fishman"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goat", "Beast"], 2, 2)
        .with_ability(abilities::haste()),
);

// UDS 101 — Ancient Silverback
pub(in crate::card::sets) static ANCIENT_SILVERBACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49651dd4-a489-42d3-b4eb-51f5353b334e"),
    "Ancient Silverback",
    crate::card::CardArt::new("49651dd4-a489-42d3-b4eb-51f5353b334e", "Paolo Parente"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Ape"], 6, 5).with_ability(
        abilities::regenerate_self(
            "{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
        ),
    ),
);

// UDS 102 — Compost
pub(in crate::card::sets) static COMPOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2523c403-0025-48c7-8ff1-e66ca27ee585"),
    "Compost",
    crate::card::CardArt::new("2523c403-0025-48c7-8ff1-e66ca27ee585", "Douglas Shuler"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_ability(AbilityDef::triggered(
            "Whenever a black card is put into an opponent's graveyard from anywhere, you may draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                ]),
                None,
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        )),
);

// UDS 103 — Elvish Lookout
pub(in crate::card::sets) static ELVISH_LOOKOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9a8a0e2-311a-4627-8a48-43df045c3112"),
    "Elvish Lookout",
    crate::card::CardArt::new(
        "d9a8a0e2-311a-4627-8a48-43df045c3112",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(abilities::shroud()),
);

// UDS 104 — Elvish Piper
pub(in crate::card::sets) static ELVISH_PIPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55e76333-0959-4572-a1ca-d77f76da1279"),
    "Elvish Piper",
    crate::card::CardArt::new("55e76333-0959-4572-a1ca-d77f76da1279", "Scott M. Fischer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Shaman"], 1, 1).with_ability(
        AbilityDef::activated(
            "{G}, {T}: You may put a creature card from your hand onto the battlefield.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::HasType(CardType::Creature),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// UDS 105 — Emperor Crocodile
pub(in crate::card::sets) static EMPEROR_CROCODILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ccba208-1e24-45bb-a556-a3eb936efb10"),
    "Emperor Crocodile",
    crate::card::CardArt::new("9ccba208-1e24-45bb-a556-a3eb936efb10", "Kev Walker"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Crocodile"], 5, 5).with_ability(
        AbilityDef::triggered_if(
            "When you control no other creatures, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// UDS 106 — Gamekeeper
pub(in crate::card::sets) static GAMEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6006c21-28b5-4550-8b4e-ac631f39cdf7"),
    "Gamekeeper",
    crate::card::CardArt::new("b6006c21-28b5-4550-8b4e-ac631f39cdf7", "Scott Hampton"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf"], 2, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, you may exile it. If you do, reveal cards from the top of your library until you reveal a creature card. Put that card onto the battlefield and put all other cards revealed this way into your graveyard.",
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                    binding: ParentBinding,
                    then: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(ParentBinding),
                                filter: None,
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        ),
                        then: &EffectDef::MillUntil(&MillUntilDef {
                            player: EffectRecipientDef::Controller,
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            matched_zone: ZoneKind::Battlefield,
                        }),
                    },
                },
            },
        ),
    ),
);

// UDS 107 — Goliath Beetle
pub(in crate::card::sets) static GOLIATH_BEETLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f83d8765-f654-4837-9b06-739610188415"),
    "Goliath Beetle",
    crate::card::CardArt::new("f83d8765-f654-4837-9b06-739610188415", "Don Hazeltine"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect"], 3, 1)
        .with_ability(abilities::trample()),
);

// UDS 108 — Heart Warden
pub(in crate::card::sets) static HEART_WARDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96e42dbe-3eeb-4367-bb6d-0f5c71f5da80"),
    "Heart Warden",
    crate::card::CardArt::new("96e42dbe-3eeb-4367-bb6d-0f5c71f5da80", "Adam Rex"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Druid"], 1, 1).with_abilities(&[
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated(
            "{2}, Sacrifice this creature: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// UDS 109 — Hunting Moa
pub(in crate::card::sets) static HUNTING_MOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("926cefa1-3c5c-4bd6-859b-de620a3ee777"),
    "Hunting Moa",
    crate::card::CardArt::new("926cefa1-3c5c-4bd6-859b-de620a3ee777", "DiTerlizzi"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bird", "Beast"], 3, 2).with_abilities(&[
        abilities::echo(
            "Echo {2}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{2}{G}"),
        ),
        AbilityDef::triggered_with_targets(
            "When this creature enters or dies, put a +1/+1 counter on target creature.",
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

// UDS 110 — Ivy Seer
pub(in crate::card::sets) static IVY_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("018ad11c-1351-4eff-94ac-3926037d7247"),
    "Ivy Seer",
    crate::card::CardArt::new("018ad11c-1351-4eff-94ac-3926037d7247", "Donato Giancola"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{G}, {T}: Reveal any number of green cards in your hand. Target creature gets +X/+X until end of turn, where X is the number of cards revealed this way.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Green),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::BoundObjectCount(ParentBinding),
                            ValueDef::BoundObjectCount(ParentBinding),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            }),
        ),
    ),
);

// UDS 111 — Magnify
pub(in crate::card::sets) static MAGNIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b9bb2c6-f1a6-42c3-a7cb-3a1a46854c9b"),
    "Magnify",
    crate::card::CardArt::new("7b9bb2c6-f1a6-42c3-a7cb-3a1a46854c9b", "Michael Sutfin"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "All creatures get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// UDS 112 — Marker Beetles
pub(in crate::card::sets) static MARKER_BEETLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cbd3c78-197a-40b9-94d1-bbb1ec1e64b1"),
    "Marker Beetles",
    crate::card::CardArt::new("5cbd3c78-197a-40b9-94d1-bbb1ec1e64b1", "Ron Spencer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Insect"], 2, 3).with_abilities(&[
        abilities::dies_trigger_with_targets(
            "When this creature dies, target creature gets +1/+1 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
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
        AbilityDef::activated(
            "{2}, Sacrifice this creature: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// UDS 113 — Momentum
pub(in crate::card::sets) static MOMENTUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10bd11a2-7cab-4d3f-b52b-f5bb66fbbec6"),
    "Momentum",
    crate::card::CardArt::new("10bd11a2-7cab-4d3f-b52b-f5bb66fbbec6", "Carl Critchlow"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "At the beginning of your upkeep, you may put a growth counter on this Aura.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("growth"),
                        amount: ValueDef::Constant(1),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each growth counter on this Aura.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::named("growth")),
                        ValueDef::CountersOnSource(CounterKind::named("growth")),
                    ),
                },
            ),
        ]),
);

// UDS 114 — Multani's Decree
pub(in crate::card::sets) static MULTANI_S_DECREE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58b4d5c8-23fc-4fb8-99d6-bb64e66cc4db"),
    "Multani's Decree",
    crate::card::CardArt::new("58b4d5c8-23fc-4fb8-99d6-bb64e66cc4db", "Eric Peterson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Destroy all enchantments. You gain 2 life for each enchantment destroyed this way.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Enchantment),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: Some(DestroyFollowUpDef {
                binding: ParentBinding,
                effect: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::BoundObjectCount(ParentBinding),
                        factor: 2,
                    }),
                },
            }),
        },
    )),
);

// UDS 115 — Pattern of Rebirth
pub(in crate::card::sets) static PATTERN_OF_REBIRTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f23c4f4-a191-4225-a3b7-dab5b1462922"),
    "Pattern of Rebirth",
    crate::card::CardArt::new("9f23c4f4-a191-4225-a3b7-dab5b1462922", "Mark Brill"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature dies, that creature's controller may search their library for a creature card, put that card onto the battlefield, then shuffle.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTriggeringObject,
                    effect: &EffectDef::SearchZone {
                        player: EffectRecipientDef::ControllerOfTriggeringObject,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::HasType(CardType::Creature),
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
                },
            ),
        ]),
);

// UDS 116 — Plated Spider
pub(in crate::card::sets) static PLATED_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3529f49b-7e5e-4fa8-a03d-a94877761525"),
    "Plated Spider",
    crate::card::CardArt::new("3529f49b-7e5e-4fa8-a03d-a94877761525", "Ron Spencer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spider"], 4, 4)
        .with_ability(abilities::reach()),
);

// UDS 117 — Plow Under
pub(in crate::card::sets) static PLOW_UNDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a30735c4-7f12-4db9-972b-9b7568a8ada8"),
    "Plow Under",
    crate::card::CardArt::new(
        "a30735c4-7f12-4db9-972b-9b7568a8ada8",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Put two target lands on top of their owners' libraries.",
        &[AbilityTargetDef {
            predicate: AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            chooser: TargetChooserDef::Controller,
            minimum: 2,
            maximum: 2,
            exact_count: None,
            divided_total: None,
            another: false,
            excludes_source: false,
        }],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// UDS 118 — Rofellos, Llanowar Emissary
pub(in crate::card::sets) static ROFELLOS_LLANOWAR_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6aa5cc65-f8f1-4f6f-8b4e-2fedccbda684"),
    "Rofellos, Llanowar Emissary",
    crate::card::CardArt::new("6aa5cc65-f8f1-4f6f-8b4e-2fedccbda684", "Michael Sutfin"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Elf", "Druid"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {G} for each Forest you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Green).with_variable_amount(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::controlled_basic_land_type(
                        PlayerRelation::You,
                        BasicLandType::Forest,
                    )),
                ),
            ),
        )),
);

// UDS 119 — Rofellos's Gift
pub(in crate::card::sets) static ROFELLOS_S_GIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a41347ba-b2e3-4d7e-8018-e6fd30243559"),
    "Rofellos's Gift",
    crate::card::CardArt::new("a41347ba-b2e3-4d7e-8018-e6fd30243559", "Pete Venters"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_sorcery(mana_cost!("{G}"))
        .with_ability(AbilityDef::spell(
            "Reveal any number of green cards in your hand. Return an enchantment card from your graveyard to your hand for each card revealed this way.",
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Green),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::ChooseExact(ChooseExactDef {
                        binding: ParentBinding,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        amount: ValueDef::BoundObjectCount(ParentBinding),
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                ParentBinding,
                            )),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    }),
                ]),
            }),
        )),
);

// UDS 120 — Scent of Ivy
pub(in crate::card::sets) static SCENT_OF_IVY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a56b4894-b959-4d00-b631-95d26eb85a4e"),
    "Scent of Ivy",
    crate::card::CardArt::new("a56b4894-b959-4d00-b631-95d26eb85a4e", "John Avon"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_instant(mana_cost!("{1}{G}"))
        .with_ability(AbilityDef::spell_with_targets(
            "Reveal any number of green cards in your hand. Target creature gets +X/+X until end of turn, where X is the number of cards revealed this way.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::Color(ManaColor::Green),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: usize::MAX,
                visibility: ChoiceVisibilityDef::Private,
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        then: &EffectDef::None,
                    }),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::BoundObjectCount(ParentBinding),
                            ValueDef::BoundObjectCount(ParentBinding),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            }),
        )),
);

// UDS 121 — Splinter
// Audit: metadata-only — SharingNameWith can find every copy, but the spell's controller must choose cards from another player's public graveyard and private hand/library; SearchZone gives hidden-zone choices to that zone's owner.
pub(in crate::card::sets) static SPLINTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb32175c-f2e6-460b-b4bf-dd85cac3eb4f"),
    "Splinter",
    crate::card::CardArt::new("eb32175c-f2e6-460b-b4bf-dd85cac3eb4f", "Daren Bader"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 122 — Taunting Elf
pub(in crate::card::sets) static TAUNTING_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85bfa6b9-c898-4bb6-a444-6cf336bfb260"),
    "Taunting Elf",
    crate::card::CardArt::new("85bfa6b9-c898-4bb6-a444-6cf336bfb260", "Scott M. Fischer"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 0, 1).with_ability(
        AbilityDef::static_ability(
            "All creatures able to block this creature do so.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )),
            },
        ),
    ),
);

// UDS 123 — Thorn Elemental
// Audit: metadata-only — Needs the combat-damage assignment option to assign this creature's damage as though it were unblocked.
pub(in crate::card::sets) static THORN_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("971d4b0d-fe3e-46f5-86df-3fbac6b900b0"),
    "Thorn Elemental",
    crate::card::CardArt::new("971d4b0d-fe3e-46f5-86df-3fbac6b900b0", "rk post"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 124 — Yavimaya Elder
pub(in crate::card::sets) static YAVIMAYA_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("325d9372-01c9-4e99-a966-13c8f8566e2e"),
    "Yavimaya Elder",
    crate::card::CardArt::new("325d9372-01c9-4e99-a966-13c8f8566e2e", "Ray Lago"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Druid"], 2, 1)
        .with_abilities(&[
            abilities::dies_trigger(
                "When this creature dies, you may search your library for up to two basic land cards, reveal them, put them into your hand, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
            ),
            AbilityDef::activated(
                "{2}, Sacrifice this creature: Draw a card.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::SacrificeSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// UDS 125 — Yavimaya Enchantress
pub(in crate::card::sets) static YAVIMAYA_ENCHANTRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9e3934e-6169-416e-92bb-359e41900c3b"),
    "Yavimaya Enchantress",
    crate::card::CardArt::new("c9e3934e-6169-416e-92bb-359e41900c3b", "Matthew D. Wilson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each enchantment on the battlefield.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        &[ZoneKind::Battlefield],
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        &[ZoneKind::Battlefield],
                    )),
                ),
            },
        ),
    ),
);

// UDS 126 — Braidwood Cup
pub(in crate::card::sets) static BRAIDWOOD_CUP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2e783b7-9bd1-4f82-bf20-5d201413f5e8"),
    "Braidwood Cup",
    crate::card::CardArt::new(
        "c2e783b7-9bd1-4f82-bf20-5d201413f5e8",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated(
        "{T}: You gain 1 life.",
        &[AbilityCostDef::TapSource],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// UDS 127 — Braidwood Sextant
pub(in crate::card::sets) static BRAIDWOOD_SEXTANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16dc7634-8ef5-4a03-8276-7e1dae4244c2"),
    "Braidwood Sextant",
    crate::card::CardArt::new("16dc7634-8ef5-4a03-8276-7e1dae4244c2", "Don Hazeltine"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{2}, {T}, Sacrifice this artifact: Search your library for a basic land card, reveal that card, put it into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificeSource,
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
    )),
);

// UDS 128 — Brass Secretary
pub(in crate::card::sets) static BRASS_SECRETARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5685cff-b607-4a81-aa47-6676ab1a5782"),
    "Brass Secretary",
    crate::card::CardArt::new("c5685cff-b607-4a81-aa47-6676ab1a5782", "DiTerlizzi"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 2, 1).with_ability(
        AbilityDef::activated(
            "{2}, Sacrifice this creature: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// UDS 129 — Caltrops
pub(in crate::card::sets) static CALTROPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9cf74e4-31d2-4cd2-8f3a-b2141301f686"),
    "Caltrops",
    crate::card::CardArt::new("a9cf74e4-31d2-4cd2-8f3a-b2141301f686", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::triggered(
        "Whenever a creature attacks, this artifact deals 1 damage to it.",
        TriggerEventDef::attacks(ObjectPredicateDef::HasType(CardType::Creature)),
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::TriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// UDS 130 — Extruder
pub(in crate::card::sets) static EXTRUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fc2f0d0-273d-428f-9a8a-c582f4d16394"),
    "Extruder",
    crate::card::CardArt::new("2fc2f0d0-273d-428f-9a8a-c582f4d16394", "Mark Tedin"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Juggernaut"], 4, 3)
        .with_abilities(&[
            abilities::echo(
                "Echo {4} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
                mana_cost!("{4}"),
            ),
            AbilityDef::activated_with_targets(
                "Sacrifice an artifact: Put a +1/+1 counter on target creature.",
                &[AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
                }],
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

// UDS 131 — Fodder Cannon
pub(in crate::card::sets) static FODDER_CANNON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("229ba320-69c9-4400-a0d7-f0f79e8d9856"),
    "Fodder Cannon",
    crate::card::CardArt::new("229ba320-69c9-4400-a0d7-f0f79e8d9856", "DiTerlizzi"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{4}, {T}, Sacrifice a creature: This artifact deals 4 damage to target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{4}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// UDS 132 — Junk Diver
pub(in crate::card::sets) static JUNK_DIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4f5a9d8-80b9-4765-adb2-10d53baaacb0"),
    "Junk Diver",
    crate::card::CardArt::new("c4f5a9d8-80b9-4765-adb2-10d53baaacb0", "Eric Peterson"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger_with_targets(
            "When this creature dies, return another target artifact card from your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })
            .another()],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// UDS 133 — Mantis Engine
pub(in crate::card::sets) static MANTIS_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97f7ebfb-f955-4849-a0f5-6806ff6ae891"),
    "Mantis Engine",
    crate::card::CardArt::new("97f7ebfb-f955-4849-a0f5-6806ff6ae891", "John Zeleznik"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Insect"], 3, 3).with_abilities(&[
        AbilityDef::activated(
            "{2}: This creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// UDS 134 — Masticore
pub(in crate::card::sets) static MASTICORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("908a2215-7231-43a4-8fec-5d1e4233c028"),
    "Masticore",
    crate::card::CardArt::new("908a2215-7231-43a4-8fec-5d1e4233c028", "Paolo Parente"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Masticore"], 4, 4).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you discard a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef::discard(PlayerSetDef::Related(PlayerRelation::You), 1),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::activated_with_targets(
            "{2}: This creature deals 1 damage to target creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::regenerate_self(
            "{2}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// UDS 135 — Metalworker
// Audit: metadata-only — The immediate mana-ability path must enumerate a complete activation before resolution and cannot capture an arbitrary hidden hand subset to reveal and count for mana.
pub(in crate::card::sets) static METALWORKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2050d414-71c7-4c42-a1ff-4c04068ba7f2"),
    "Metalworker",
    crate::card::CardArt::new("2050d414-71c7-4c42-a1ff-4c04068ba7f2", "Don Hazeltine"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 136 — Powder Keg
pub(in crate::card::sets) static POWDER_KEG: CardRecord = CardRecord::new_with_legacy_id(
    2053,
    "Powder Keg",
    CardArt::new("4d9715c2-9036-4ae2-a5b4-1b190d50c963", "Dan Frazier"),
    CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may put a fuse counter on this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // The counter is optional, so the Keg can be held at whatever size the board
            // calls for rather than ticking past it.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("fuse"),
                    amount: ValueDef::Constant(1),
                },
            },
        ),
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Destroy each artifact and creature with mana value equal to the number of fuse counters on this artifact.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    // Everything the fuse counters name. A Keg with no counters on it destroys
                    // every nothing-cost permanent, which is the mode that answers a board of
                    // tokens.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(CounterKind::named("fuse"))),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// UDS 137 — Scrying Glass
// Audit: metadata-only — Needs a positive integer choice at resolution and a way to compare it with the count of a separately chosen color in the revealed hand.
pub(in crate::card::sets) static SCRYING_GLASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7286819f-6c57-4503-898c-528786ad86e9"),
    "Scrying Glass",
    crate::card::CardArt::new("7286819f-6c57-4503-898c-528786ad86e9", "Patrick Ho"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 138 — Storage Matrix
// Audit: metadata-only — Needs each player to choose artifact, creature, or land during their untap step and a player-scoped untap restriction derived from that choice for the rest of the step.
pub(in crate::card::sets) static STORAGE_MATRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77378279-024c-4c36-b5bf-6294fe5c32f5"),
    "Storage Matrix",
    crate::card::CardArt::new("77378279-024c-4c36-b5bf-6294fe5c32f5", "Patrick Ho"),
    crate::card::CardSet::UrzasDestiny,
    crate::card::CardRules::unsupported(),
);

// UDS 139 — Thran Dynamo
pub(in crate::card::sets) static THRAN_DYNAMO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f0c9dee-fab5-4522-9821-343f84b0c8ab"),
    "Thran Dynamo",
    crate::card::CardArt::new("3f0c9dee-fab5-4522-9821-343f84b0c8ab", "Ron Spears"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}{C}{C}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
    )),
);

// UDS 140 — Thran Foundry
pub(in crate::card::sets) static THRAN_FOUNDRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cdc0d42-d96f-490f-87cb-3577dfdce807"),
    "Thran Foundry",
    crate::card::CardArt::new("9cdc0d42-d96f-490f-87cb-3577dfdce807", "John Zeleznik"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}, Exile this artifact: Target player shuffles their graveyard into their library.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::ExileSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                ))),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// UDS 141 — Thran Golem (reprint)

// UDS 142 — Urza's Incubator
pub(in crate::card::sets) static URZA_S_INCUBATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdf96c2c-b3d6-4d84-9572-fb115a795bed"),
    "Urza's Incubator",
    crate::card::CardArt::new("bdf96c2c-b3d6-4d84-9572-fb115a795bed", "Pete Venters"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::replacement(
            "As this artifact enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        AbilityDef::static_ability(
            "Creature spells of the chosen type cost {2} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasSourcesChosenScalar(
                        BattlefieldEntryChoiceDestinationDef::CreatureType,
                    ),
                ]),
                PlayerRelation::Any,
                ValueDef::Constant(2),
            )),
        ),
    ]),
);

// UDS 143 — Yavimaya Hollow
pub(in crate::card::sets) static YAVIMAYA_HOLLOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47dd5c4b-5972-43e1-ae2a-ebf275006458"),
    "Yavimaya Hollow",
    crate::card::CardArt::new("47dd5c4b-5972-43e1-ae2a-ebf275006458", "Douglas Shuler"),
    crate::card::CardSet::UrzasDestiny,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated_with_targets(
                "{G}, {T}: Regenerate target creature.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{G}")),
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ACADEMY_RECTOR,
    &ARCHERY_TRAINING,
    &CAPASHEN_STANDARD,
    &CAPASHEN_TEMPLAR,
    &FALSE_PROPHET,
    &FEND_OFF,
    &FIELD_SURGEON,
    &FLICKER,
    &JASMINE_SEER,
    &MASK_OF_LAW_AND_GRACE,
    &MASTER_HEALER,
    &OPALESCENCE,
    &RELIQUARY_MONK,
    &REPLENISH,
    &SANCTIMONY,
    &SCENT_OF_JASMINE,
    &SCOUR,
    &SERRA_ADVOCATE,
    &SOLIDARITY,
    &TETHERED_GRIFFIN,
    &TORMENTED_ANGEL,
    &VOICE_OF_DUTY,
    &VOICE_OF_REASON,
    &WALL_OF_GLARE,
    &AURA_THIEF,
    &BLIZZARD_ELEMENTAL,
    &BRINE_SEER,
    &BUBBLING_BEEBLES,
    &DISAPPEAR,
    &DONATE,
    &FATIGUE,
    &FLEDGLING_OSPREY,
    &ILLUMINATED_WINGS,
    &IRIDESCENT_DRAKE,
    &KINGFISHER,
    &MENTAL_DISCIPLINE,
    &METATHRAN_ELITE,
    &METATHRAN_SOLDIER,
    &OPPOSITION,
    &PRIVATE_RESEARCH,
    &QUASH,
    &RAYNE_ACADEMY_CHANCELLOR,
    &RESCUE,
    &SCENT_OF_BRINE,
    &SIGIL_OF_SLEEP,
    &TELEPATHIC_SPIES,
    &TEMPORAL_ADEPT,
    &THIEVING_MAGPIE,
    &TREACHERY,
    &APPRENTICE_NECROMANCER,
    &ATTRITION,
    &BODY_SNATCHER,
    &BUBBLING_MUCK,
    &CARNIVAL_OF_SOULS,
    &CHIME_OF_NIGHT,
    &DISEASE_CARRIERS,
    &DYING_WAIL,
    &ENCROACH,
    &ERADICATE,
    &FESTERING_WOUND,
    &LURKING_JACKALS,
    &NIGHTSHADE_SEER,
    &PHYREXIAN_MONITOR,
    &PHYREXIAN_NEGATOR,
    &PLAGUE_DOGS,
    &RAPID_DECAY,
    &SCENT_OF_NIGHTSHADE,
    &SKITTERING_HORROR,
    &SLINKING_SKIRGE,
    &SOUL_FEAST,
    &SQUIRMING_MASS,
    &TWISTED_EXPERIMENT,
    &YAWGMOTH_S_BARGAIN,
    &AETHER_STING,
    &BLOODSHOT_CYCLOPS,
    &CINDER_SEER,
    &COLOS_YEARLING,
    &COVETOUS_DRAGON,
    &FLAME_JET,
    &GOBLIN_BERSERKER,
    &GOBLIN_FESTIVAL,
    &GOBLIN_GARDENER,
    &GOBLIN_MARSHAL,
    &GOBLIN_MASONS,
    &HULKING_OGRE,
    &IMPATIENCE,
    &INCENDIARY,
    &KELDON_CHAMPION,
    &KELDON_VANDALS,
    &LANDSLIDE,
    &MARK_OF_FURY,
    &RECKLESS_ABANDON,
    &REPERCUSSION,
    &SCENT_OF_CINDER,
    &SOWING_SALT,
    &WAKE_OF_DESTRUCTION,
    &WILD_COLOS,
    &ANCIENT_SILVERBACK,
    &COMPOST,
    &ELVISH_LOOKOUT,
    &ELVISH_PIPER,
    &EMPEROR_CROCODILE,
    &GAMEKEEPER,
    &GOLIATH_BEETLE,
    &HEART_WARDEN,
    &HUNTING_MOA,
    &IVY_SEER,
    &MAGNIFY,
    &MARKER_BEETLES,
    &MOMENTUM,
    &MULTANI_S_DECREE,
    &PATTERN_OF_REBIRTH,
    &PLATED_SPIDER,
    &PLOW_UNDER,
    &ROFELLOS_LLANOWAR_EMISSARY,
    &ROFELLOS_S_GIFT,
    &SCENT_OF_IVY,
    &SPLINTER,
    &TAUNTING_ELF,
    &THORN_ELEMENTAL,
    &YAVIMAYA_ELDER,
    &YAVIMAYA_ENCHANTRESS,
    &BRAIDWOOD_CUP,
    &BRAIDWOOD_SEXTANT,
    &BRASS_SECRETARY,
    &CALTROPS,
    &EXTRUDER,
    &FODDER_CANNON,
    &JUNK_DIVER,
    &MANTIS_ENGINE,
    &MASTICORE,
    &METALWORKER,
    &POWDER_KEG,
    &SCRYING_GLASS,
    &STORAGE_MATRIX,
    &THRAN_DYNAMO,
    &THRAN_FOUNDRY,
    &URZA_S_INCUBATOR,
    &YAVIMAYA_HOLLOW,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_m14::CAPASHEN_KNIGHT), // UDS 3
    PrintingRecord::reprint(&catalog_m13::RAVENOUS_RATS),   // UDS 68
    PrintingRecord::reprint(&catalog_m13::TRUMPET_BLAST),   // UDS 98
    PrintingRecord::reprint(&catalog_m12::THRAN_GOLEM),     // UDS 141
];
