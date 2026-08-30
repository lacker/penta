//! New Phyrexia cards used to exercise Phyrexian mana.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2010::scars_of_mirrodin::METALCRAFT;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AggregateOperationDef, AlternativeCastKindDef, AppliedEffectDef,
    AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, BasicLandType,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CardTypeSet, ControlDurationDef, CopyExceptionsDef, CounterKind, DiscardSelectionDef,
    EffectDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, ManaColor,
    ManaRestrictionDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    ObjectValueAggregateDef, ObjectValueDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    SpellAdditionalCostDef, SumValueDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::AdditionalCostObjectIndex;
use crate::{TargetIndex, mana_cost};

// NPH 1 — Karn Liberated
// Audit: metadata-only — Needs a restart-game procedure that preserves the non-Aura permanent cards linked in exile and puts them onto the restarted battlefield.
pub(in crate::card::sets) static KARN_LIBERATED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9287151-95df-4f5a-b32a-4b0aea825452"),
    "Karn Liberated",
    crate::card::CardArt::new("f9287151-95df-4f5a-b32a-4b0aea825452", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 2 — Apostle's Blessing
// Audit: metadata-only — Needs one resolution choice whose alternatives are protection from artifacts or protection from a dynamically chosen color; the color-choice effect cannot express the artifact branch.
pub(in crate::card::sets) static APOSTLE_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f7c3571-925d-486e-80dd-bac47aa48283"),
    "Apostle's Blessing",
    crate::card::CardArt::new("9f7c3571-925d-486e-80dd-bac47aa48283", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 3 — Auriok Survivors
// Audit: metadata-only — MoveToZone can force an arriving Equipment to attach, but it cannot bind that successor into a second, independently optional attach choice after the optional return succeeds.
pub(in crate::card::sets) static AURIOK_SURVIVORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("deffb601-6a53-4d88-a6af-686ce97eb4f0"),
    "Auriok Survivors",
    crate::card::CardArt::new("deffb601-6a53-4d88-a6af-686ce97eb4f0", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 4 — Blade Splicer
pub(in crate::card::sets) static BLADE_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8e56a28-713b-4a13-a601-1128cf117539"),
    "Blade Splicer",
    crate::card::CardArt::new("b8e56a28-713b-4a13-a601-1128cf117539", "Greg Staples"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Phyrexian", "Human", "Artificer"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.", EffectDef::create_artifact_creature_token(&["Phyrexian", "Golem"], &[], 3, 3)),
        AbilityDef::static_ability(
            "Golems you control have first strike.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Subtype("Golem"), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            },
        ),
    ]),
);

// NPH 5 — Cathedral Membrane
// Audit: metadata-only — Needs a dies-during-combat event plus last-known access to every creature the source blocked; the blocker relation alone cannot gate the death trigger.
pub(in crate::card::sets) static CATHEDRAL_MEMBRANE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07592731-68be-4218-bb2c-c2523c5a27f1"),
    "Cathedral Membrane",
    crate::card::CardArt::new("07592731-68be-4218-bb2c-c2523c5a27f1", "Richard Whitters"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 6 — Chancellor of the Annex
pub(in crate::card::sets) static CHANCELLOR_OF_THE_ANNEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be1b482a-badb-4b9a-ab63-2e7944826aa0"),
    "Chancellor of the Annex",
    crate::card::CardArt::new("be1b482a-badb-4b9a-ab63-2e7944826aa0", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{W}{W}{W}"), &["Phyrexian", "Angel"], 5, 6)
        .with_abilities(&[
            AbilityDef::opening_hand_reveal(
                "You may reveal this card from your opening hand. If you do, when each opponent casts their first spell of the game, counter that spell unless that player pays {1}.",
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "When each opponent casts their first spell of the game, counter that spell unless that player pays {1}.",
                    TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
                    EffectDef::PayOr(PayOrDef::unless(
                        EffectPaymentDef::mana(
                            PlayerSetDef::One(PlayerRefDef::EventPlayer),
                            mana_cost!("{1}"),
                        ),
                        &EffectDef::Counter {
                            object: EffectRecipientDef::TriggeringObject,
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                        },
                    )),
                ))),
            ),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever an opponent casts a spell, counter it unless that player pays {1}.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::Opponent,
                )),
                EffectDef::PayOr(PayOrDef::unless(
                    EffectPaymentDef::mana(
                        PlayerSetDef::One(PlayerRefDef::EventPlayer),
                        mana_cost!("{1}"),
                    ),
                    &EffectDef::Counter {
                        object: EffectRecipientDef::TriggeringObject,
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                    },
                )),
            ),
        ]),
);

// NPH 7 — Dispatch
pub(in crate::card::sets) static DISPATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("496634f9-1271-4be7-bad5-364bb87a6962"),
    "Dispatch",
    crate::card::CardArt::new("496634f9-1271-4be7-bad5-364bb87a6962", "Erica Yang"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target creature.\nMetalcraft — If you control three or more artifacts, exile that creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::IfCondition {
                condition: &METALCRAFT,
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            },
        ]),
    )),
);

// NPH 8 — Due Respect
// Audit: metadata-only — Needs a turn-scoped global battlefield-entry replacement that makes every permanent enter tapped; temporary effects cannot currently install entry replacements.
pub(in crate::card::sets) static DUE_RESPECT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7159850-964b-4f12-957f-614eb0570544"),
    "Due Respect",
    crate::card::CardArt::new("a7159850-964b-4f12-957f-614eb0570544", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 9 — Elesh Norn, Grand Cenobite
pub(in crate::card::sets) static ELESH_NORN_GRAND_CENOBITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b66390d6-1649-4bfa-92d3-77664650d552"),
    "Elesh Norn, Grand Cenobite",
    crate::card::CardArt::new("b66390d6-1649-4bfa-92d3-77664650d552", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Phyrexian", "Praetor"], 4, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Other creatures you control get +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "Creatures your opponents control get -2/-2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Opponent,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                },
            ),
        ]),
);

// NPH 10 — Exclusion Ritual
// Audit: metadata-only — Needs a cast prohibition whose name predicate is read from the permanent card linked in exile rather than from a chosen name or the source's own name.
pub(in crate::card::sets) static EXCLUSION_RITUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e3b826a-7349-45ae-89bf-675fea7ce8e3"),
    "Exclusion Ritual",
    crate::card::CardArt::new("9e3b826a-7349-45ae-89bf-675fea7ce8e3", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 11 — Forced Worship
pub(in crate::card::sets) static FORCED_WORSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e050701d-4609-470d-85ff-4b7638893c6a"),
    "Forced Worship",
    crate::card::CardArt::new("e050701d-4609-470d-85ff-4b7638893c6a", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                },
            ),
            AbilityDef::activated(
                "{2}{W}: Return this Aura to its owner's hand.",
                &[AbilityCostDef::Mana(mana_cost!("{2}{W}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// NPH 12 — Inquisitor Exarch
pub(in crate::card::sets) static INQUISITOR_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49e241a0-a027-494b-8187-6ecb006d1d33"),
    "Inquisitor Exarch",
    crate::card::CardArt::new("49e241a0-a027-494b-8187-6ecb006d1d33", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Phyrexian", "Cleric"], 2, 2).with_ability(
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —\n• You gain 2 life.\n• Target opponent loses 2 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell(
                    "You gain 2 life.",
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target opponent loses 2 life.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                    )],
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                ),
            ],
        ),
    ),
);

// NPH 13 — Lost Leonin
pub(in crate::card::sets) static LOST_LEONIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8209fa5d-2c0e-4827-813b-fff123533f16"),
    "Lost Leonin",
    crate::card::CardArt::new("8209fa5d-2c0e-4827-813b-fff123533f16", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Phyrexian", "Cat", "Soldier"], 2, 1)
        .with_abilities(&[abilities::infect()]),
);

// NPH 14 — Loxodon Convert
pub(in crate::card::sets) static LOXODON_CONVERT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00c050c3-4f50-4bb6-8477-6737887ca10d"),
    "Loxodon Convert",
    crate::card::CardArt::new("00c050c3-4f50-4bb6-8477-6737887ca10d", "Adrian Smith"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(
        mana_cost!("{3}{W}"),
        &["Phyrexian", "Elephant", "Soldier"],
        4,
        2,
    ),
);

// NPH 15 — Marrow Shards
pub(in crate::card::sets) static MARROW_SHARDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53ca60ee-e54b-4a28-b6a6-7bf3503c35b4"),
    "Marrow Shards",
    crate::card::CardArt::new("53ca60ee-e54b-4a28-b6a6-7bf3503c35b4", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{W/P}")).with_ability(AbilityDef::spell(
        "This spell deals 1 damage to each attacking creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(1),
        },
    )),
);

// NPH 16 — Master Splicer
pub(in crate::card::sets) static MASTER_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("859d2b91-63af-4700-8ca5-b1756aa6639b"),
    "Master Splicer",
    crate::card::CardArt::new("859d2b91-63af-4700-8ca5-b1756aa6639b", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Phyrexian", "Human", "Artificer"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.", EffectDef::create_artifact_creature_token(&["Phyrexian", "Golem"], &[], 3, 3)),
        AbilityDef::static_ability(
            "Golems you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Subtype("Golem"), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
            },
        ),
    ]),
);

// NPH 17 — Norn's Annex
pub(in crate::card::sets) static NORN_S_ANNEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a64073f2-99f5-4dc7-9403-e7cb94ce0e60"),
    "Norn's Annex",
    crate::card::CardArt::new("a64073f2-99f5-4dc7-9403-e7cb94ce0e60", "James Paick"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}{W/P}{W/P}")).with_ability(
        AbilityDef::static_ability(
            "Creatures can't attack you or planeswalkers you control unless their controller pays {W/P} for each of those creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(
                    AttackRestrictionDef::unless_paid(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker,
                        mana_cost!("{W/P}"),
                    ),
                )),
            },
        ),
    ),
);

// NPH 18 — Phyrexian Unlife
// Audit: metadata-only — Needs suppression of the state-based loss at 0 life plus a conditional damage replacement that gives damage infect only while that player's life is nonpositive.
pub(in crate::card::sets) static PHYREXIAN_UNLIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4a1e16a-39f0-47ab-aba8-73e82ba9ab18"),
    "Phyrexian Unlife",
    crate::card::CardArt::new("b4a1e16a-39f0-47ab-aba8-73e82ba9ab18", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 19 — Porcelain Legionnaire
pub(in crate::card::sets) static PORCELAIN_LEGIONNAIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2616aa0e-8413-4e63-877c-bffd5263f552"),
    "Porcelain Legionnaire",
    crate::card::CardArt::new("2616aa0e-8413-4e63-877c-bffd5263f552", "Eric Deschamps"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{2}{W/P}"), &["Phyrexian", "Soldier"], 3, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// NPH 20 — Puresteel Paladin
pub(in crate::card::sets) static PURESTEEL_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca100248-fcd6-41ed-8d75-bcb473845edd"),
    "Puresteel Paladin",
    crate::card::CardArt::new("ca100248-fcd6-41ed-8d75-bcb473845edd", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an Equipment you control enters, you may draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Equipment"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
        AbilityDef::static_ability(
            "Metalcraft — Equipment you control have equip {0} as long as you control three or more artifacts.",
            EffectDef::IfCondition {
                condition: &METALCRAFT,
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype("Equipment"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::equip(
                        &[],
                        "Equip {0} ({0}: Attach to target creature you control. Equip only as a sorcery.)",
                    )),
                },
            },
        ),
    ]),
);

// NPH 21 — Remember the Fallen
pub(in crate::card::sets) static REMEMBER_THE_FALLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d9b8325-2a28-4312-b778-40087f8ea778"),
    "Remember the Fallen",
    crate::card::CardArt::new("6d9b8325-2a28-4312-b778-40087f8ea778", "Eric Deschamps"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one or both —\n• Return target creature card from your graveyard to your hand.\n• Return target artifact card from your graveyard to your hand.",
        &[
            AbilityDef::spell_with_targets(
                "Return target creature card from your graveyard to your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
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
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
        1,
        2,
        false,
    )),
);

// NPH 22 — Sensor Splicer
pub(in crate::card::sets) static SENSOR_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79076264-d71c-4b30-aac9-702a4d229933"),
    "Sensor Splicer",
    crate::card::CardArt::new("79076264-d71c-4b30-aac9-702a4d229933", "Izzy"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Phyrexian", "Artificer"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.", EffectDef::create_artifact_creature_token(&["Phyrexian", "Golem"], &[], 3, 3)),
        AbilityDef::static_ability(
            "Golem creatures you control have vigilance.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Subtype("Golem"), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
            },
        ),
    ]),
);

// NPH 23 — Shattered Angel
pub(in crate::card::sets) static SHATTERED_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("012f94e9-91cd-48da-873f-2da2b03a4965"),
    "Shattered Angel",
    crate::card::CardArt::new("012f94e9-91cd-48da-873f-2da2b03a4965", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Phyrexian", "Angel"], 3, 3).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever a land an opponent controls enters, you may gain 3 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                },
            ),
        ],
    ),
);

// NPH 24 — Shriek Raptor
pub(in crate::card::sets) static SHRIEK_RAPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73071a3b-9329-418e-9285-fa4765463d1f"),
    "Shriek Raptor",
    crate::card::CardArt::new("73071a3b-9329-418e-9285-fa4765463d1f", "Efrem Palacios"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Phyrexian", "Bird"], 2, 3)
        .with_abilities(&[abilities::flying(), abilities::infect()]),
);

// NPH 25 — Suture Priest
pub(in crate::card::sets) static SUTURE_PRIEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31432e98-86cd-42ea-ad37-eb4383dc6a81"),
    "Suture Priest",
    crate::card::CardArt::new("31432e98-86cd-42ea-ad37-eb4383dc6a81", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Phyrexian", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, you may gain 1 life.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), None, Some(ZoneKind::Battlefield)),
            EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::GainLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1) } },
        ),
        AbilityDef::triggered(
            "Whenever a creature an opponent controls enters, you may have that player lose 1 life.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)]), None, Some(ZoneKind::Battlefield)),
            EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::LoseLife { recipient: EffectRecipientDef::ControllerOfTriggeringObject, amount: ValueDef::Constant(1) } },
        ),
    ]),
);

// NPH 26 — War Report
pub(in crate::card::sets) static WAR_REPORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d837262-cd5d-4fc9-96dd-39ed04166883"),
    "War Report",
    crate::card::CardArt::new("6d837262-cd5d-4fc9-96dd-39ed04166883", "Mike Bierek"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "You gain life equal to the number of creatures on the battlefield plus the number of artifacts on the battlefield.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Sum(&SumValueDef::new(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                )),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                )),
            )),
        },
    )),
);

// NPH 27 — Argent Mutation
pub(in crate::card::sets) static ARGENT_MUTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("507fa5fd-2aa5-4721-a059-2c8c3056a4ca"),
    "Argent Mutation",
    crate::card::CardArt::new(
        "507fa5fd-2aa5-4721-a059-2c8c3056a4ca",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target permanent becomes an artifact in addition to its other types until end of turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Artifact)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 28 — Arm with Aether
pub(in crate::card::sets) static ARM_WITH_AETHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0878b20-315d-49fa-a4d7-232ba1ed6b0d"),
    "Arm with Aether",
    crate::card::CardArt::new("a0878b20-315d-49fa-a4d7-232ba1ed6b0d", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Until end of turn, creatures you control gain \"Whenever this creature deals damage to an opponent, you may return target creature that player controls to its owner's hand.\"",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets(
                "Whenever this creature deals damage to an opponent, you may return target creature that player controls to its owner's hand.",
                TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::EventPlayer),
                        owner: None,
                    },
                )],
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                },
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NPH 29 — Blighted Agent
pub(in crate::card::sets) static BLIGHTED_AGENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cddaebde-a060-4510-8c97-68432d931987"),
    "Blighted Agent",
    crate::card::CardArt::new("cddaebde-a060-4510-8c97-68432d931987", "Anthony Francisco"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Phyrexian", "Human", "Rogue"], 1, 1)
        .with_abilities(&[
            abilities::infect(),
            abilities::cannot_be_blocked("This creature can't be blocked."),
        ]),
);

// NPH 30 — Chained Throatseeker
// Audit: metadata-only — Needs an attack restriction that reads poison counters on each prospective defending player; attack predicates cannot currently inspect player counters.
pub(in crate::card::sets) static CHAINED_THROATSEEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a7bb447-c2b0-429e-bf82-02d6a966fe73"),
    "Chained Throatseeker",
    crate::card::CardArt::new("3a7bb447-c2b0-429e-bf82-02d6a966fe73", "Stephan Martiniere"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 31 — Chancellor of the Spires
pub(in crate::card::sets) static CHANCELLOR_OF_THE_SPIRES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1e06e16-96fa-4611-b4a9-512eeeeddd3c"),
    "Chancellor of the Spires",
    crate::card::CardArt::new("b1e06e16-96fa-4611-b4a9-512eeeeddd3c", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{U}{U}{U}"), &["Phyrexian", "Sphinx"], 5, 7)
        .with_abilities(&[
            AbilityDef::opening_hand_reveal(
                "You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, each opponent mills seven cards.",
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the first upkeep, each opponent mills seven cards.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::Mill {
                        player: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
                        amount: ValueDef::Constant(7),
                    },
                ))),
            ),
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, you may cast target instant or sorcery card from an opponent's graveyard without paying its mana cost.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::Opponent),
                    },
                )],
                EffectDef::MayCastTargetWithoutPaying {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ability: &AbilityDef::alternative_cast(
                        mana_cost!("{0}"),
                        AlternativeCastKindDef::WithoutPayingManaCost,
                        Some("Cast without paying its mana cost."),
                        EffectDef::None,
                    ),
                },
            ),
        ]),
);

// NPH 32 — Corrupted Resolve
// Audit: metadata-only — Needs a counterspell condition that reads poison counters on the targeted spell's controller; spell predicates cannot currently inspect controller counters.
pub(in crate::card::sets) static CORRUPTED_RESOLVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28432161-023b-4a98-b92a-55dc6d936cd1"),
    "Corrupted Resolve",
    crate::card::CardArt::new("28432161-023b-4a98-b92a-55dc6d936cd1", "Greg Staples"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 33 — Deceiver Exarch
pub(in crate::card::sets) static DECEIVER_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f123ad6-fe84-4fed-9c0f-6b41921e9c26"),
    "Deceiver Exarch",
    crate::card::CardArt::new("1f123ad6-fe84-4fed-9c0f-6b41921e9c26", "Izzy"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Phyrexian", "Cleric"], 1, 4)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::modal_triggered(
                "When this creature enters, choose one —\n• Untap target permanent you control.\n• Tap target permanent an opponent controls.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                &[
                    AbilityDef::spell_with_targets(
                        "Untap target permanent you control.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Any,
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::You),
                                owner: None,
                            },
                        )],
                        EffectDef::Untap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Tap target permanent an opponent controls.",
                        &[AbilityTargetDef::exactly_one(
                            AbilityTargetPredicate::Object {
                                object: ObjectPredicateDef::Any,
                                zones: &[ZoneKind::Battlefield],
                                controller: Some(PlayerRelation::Opponent),
                                owner: None,
                            },
                        )],
                        EffectDef::Tap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                    ),
                ],
            ),
        ]),
);

// NPH 34 — Defensive Stance
pub(in crate::card::sets) static DEFENSIVE_STANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0670653-d4fe-4fac-b769-d19ca4698c97"),
    "Defensive Stance",
    crate::card::CardArt::new("d0670653-d4fe-4fac-b769-d19ca4698c97", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// NPH 35 — Gitaxian Probe
pub(in crate::card::sets) static GITAXIAN_PROBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("995486ce-58bb-4753-a812-0ca73ef1a235"),
    "Gitaxian Probe",
    CardArt::new("995486ce-58bb-4753-a812-0ca73ef1a235", "Chippy"),
    CardSet::NewPhyrexia,
    // Two life for a cantrip and a look at their hand -- and in a deck that
    // counts spells cast, the look is beside the point.
    CardRules::new_sorcery(mana_cost!("{U/P}")).with_ability(AbilityDef::spell_with_targets(
        "Look at target player's hand.\nDraw a card.",
        // Any player, including yourself: looking at your own hand does nothing,
        // but the clause does not stop you, and a Probe with no opponent worth
        // reading is still a free card.
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::LookAtHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 36 — Impaler Shrike
pub(in crate::card::sets) static IMPALER_SHRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91e1f8b5-4792-457d-b3de-1d4874ddf72e"),
    "Impaler Shrike",
    crate::card::CardArt::new("91e1f8b5-4792-457d-b3de-1d4874ddf72e", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Phyrexian", "Bird"], 3, 1)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, you may sacrifice it. If you do, draw three cards.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::Source,
                    count: ValueDef::Constant(1),
                    then: Some(&EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    }),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: true,
                },
            ),
        ]),
);

// NPH 37 — Jin-Gitaxias, Core Augur
// Audit: partial — Needs a numeric maximum-hand-size modifier; the player-rule vocabulary only supports removing the maximum entirely.
pub(in crate::card::sets) static JIN_GITAXIAS_CORE_AUGUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd46fc9f-5b92-44d7-8940-2f39b0962b8f"),
    "Jin-Gitaxias, Core Augur",
    crate::card::CardArt::new("bd46fc9f-5b92-44d7-8940-2f39b0962b8f", "Eric Deschamps"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{8}{U}{U}"), &["Phyrexian", "Praetor"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::triggered(
                "At the beginning of your end step, draw seven cards.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(7),
                },
            ),
            AbilityDef::not_implemented(
                "Each opponent's maximum hand size is reduced by seven.",
                "Needs a numeric maximum-hand-size modifier.",
            ),
        ]),
);

// NPH 38 — Mental Misstep
pub(in crate::card::sets) static MENTAL_MISSTEP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61e9c6df-1c84-4eab-9076-a4feb6347c10"),
    "Mental Misstep",
    crate::card::CardArt::new("61e9c6df-1c84-4eab-9076-a4feb6347c10", "Erica Yang"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{U/P}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell with mana value 1.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(1)),
        )],
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
            placement: ZonePlacement::Top,
        },
    )),
);

// NPH 39 — Mindculling
pub(in crate::card::sets) static MINDCULLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6faf4372-6fb5-48aa-9b94-b0e77c867116"),
    "Mindculling",
    crate::card::CardArt::new("6faf4372-6fb5-48aa-9b94-b0e77c867116", "Cos Koniotis"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{5}{U}")).with_ability(AbilityDef::spell_with_targets(
        "You draw two cards and target opponent discards two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: crate::card::DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// NPH 40 — Numbing Dose
pub(in crate::card::sets) static NUMBING_DOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f28a0f4-43e1-46df-8b6a-d588c5cceb88"),
    "Numbing Dose",
    crate::card::CardArt::new("8f28a0f4-43e1-46df-8b6a-d588c5cceb88", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact or creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
            ),
            AbilityDef::static_ability(
                "Enchanted permanent doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(
                        AppliedRuleDef::DoesNotUntapDuringUntapStep,
                    ),
                },
            ),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted permanent's controller, that player loses 1 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// NPH 41 — Phyrexian Ingester
pub(in crate::card::sets) static PHYREXIAN_INGESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("376e9829-23eb-4b43-9ec7-246cb3156e95"),
    "Phyrexian Ingester",
    crate::card::CardArt::new("376e9829-23eb-4b43-9ec7-246cb3156e95", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{6}{U}"), &["Phyrexian", "Beast"], 3, 3)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "Imprint — When this creature enters, you may exile target nontoken creature.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
            AbilityDef::static_ability(
                "This creature gets +X/+Y, where X is the exiled creature card's power and Y is its toughness.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::LinkedExiles,
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Sum,
                        }),
                        ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::LinkedExiles,
                            select: ObjectValueDef::Toughness,
                            operation: AggregateOperationDef::Sum,
                        }),
                    ),
                },
            ),
        ]),
);

// NPH 42 — Phyrexian Metamorph
pub(in crate::card::sets) static PHYREXIAN_METAMORPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8903546d-4f9a-4e90-8dd8-5ab068d40907"),
    "Phyrexian Metamorph",
    CardArt::new(
        "d2e27911-87cb-49a0-a34f-6afe4bddd592",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::NewPhyrexia,
    // A 0/0 that is never a 0/0: it copies something or it dies, and the
    // artifact it adds to its own types is what a copy of a creature keeps
    // afterwards.
    CardRules::new_artifact_creature(mana_cost!("{3}{U/P}"), &["Phyrexian", "Shapeshifter"], 0, 0)
        .with_ability(AbilityDef::replacement(
            "You may have this creature enter as a copy of any artifact or creature on the \
         battlefield, except it's an artifact in addition to its other types.",
            ReplacementEffectDef::CopyEntering {
                // "Any artifact or creature", which is wider than either Clone or Copy
                // Artifact: what it copies may be the other player's best creature or your
                // own best artifact, and the four mana is really three and two life.
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                exceptions: CopyExceptionsDef::NONE
                    .with_added_types(CardTypeSet::single(CardType::Artifact)),
            },
        )),
);

// NPH 43 — Psychic Barrier
pub(in crate::card::sets) static PSYCHIC_BARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1cba7d67-5c6c-4738-8907-7cce503e3180"),
    "Psychic Barrier",
    crate::card::CardArt::new("1cba7d67-5c6c-4738-8907-7cce503e3180", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target creature spell. Its controller loses 1 life.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 44 — Psychic Surgery
// Audit: metadata-only — Needs a library-shuffled trigger event plus an optional exile stage and an arrangement stage over the unchosen cards of another player's library.
pub(in crate::card::sets) static PSYCHIC_SURGERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51ea9a6d-d6ca-48cb-adac-958ad0e7440c"),
    "Psychic Surgery",
    crate::card::CardArt::new("51ea9a6d-d6ca-48cb-adac-958ad0e7440c", "Anthony Francisco"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 45 — Spined Thopter
pub(in crate::card::sets) static SPINED_THOPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd27f71a-cd22-4b5e-9536-3e160111875a"),
    "Spined Thopter",
    crate::card::CardArt::new("bd27f71a-cd22-4b5e-9536-3e160111875a", "Pete Venters"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{2}{U/P}"), &["Phyrexian", "Thopter"], 2, 1)
        .with_abilities(&[abilities::flying()]),
);

// NPH 46 — Spire Monitor
pub(in crate::card::sets) static SPIRE_MONITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("189f83aa-264b-4d09-b45f-099597a789d4"),
    "Spire Monitor",
    crate::card::CardArt::new("189f83aa-264b-4d09-b45f-099597a789d4", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Phyrexian", "Drake"], 3, 3)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// NPH 47 — Tezzeret's Gambit
pub(in crate::card::sets) static TEZZERET_S_GAMBIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fff5a09e-9276-44b8-b374-4b84aebd47cc"),
    "Tezzeret's Gambit",
    crate::card::CardArt::new("fff5a09e-9276-44b8-b374-4b84aebd47cc", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{3}{U/P}")).with_ability(AbilityDef::spell(
        "Draw two cards, then proliferate.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Proliferate,
        ]),
    )),
);

// NPH 48 — Vapor Snag
pub(in crate::card::sets) static VAPOR_SNAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70305148-23bd-41dd-9de5-13cf5ae591ae"),
    "Vapor Snag",
    crate::card::CardArt::new("70305148-23bd-41dd-9de5-13cf5ae591ae", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand. Its controller loses 1 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 49 — Viral Drake
pub(in crate::card::sets) static VIRAL_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d89b312b-cc90-4f08-ae2e-043a79e51156"),
    "Viral Drake",
    crate::card::CardArt::new("d89b312b-cc90-4f08-ae2e-043a79e51156", "Lars Grant-West"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Phyrexian", "Drake"], 1, 4).with_abilities(&[
        abilities::flying(),
        abilities::infect(),
        AbilityDef::activated(
            "{3}{U}: Proliferate.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
            EffectDef::Proliferate,
        ),
    ]),
);

// NPH 50 — Wing Splicer
pub(in crate::card::sets) static WING_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2dbfb1b-092c-44a3-932d-a8b27be0a72b"),
    "Wing Splicer",
    crate::card::CardArt::new("e2dbfb1b-092c-44a3-932d-a8b27be0a72b", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Phyrexian", "Human", "Artificer"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.", EffectDef::create_artifact_creature_token(&["Phyrexian", "Golem"], &[], 3, 3)),
        AbilityDef::static_ability(
            "Golem creatures you control have flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Subtype("Golem"), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
            },
        ),
    ]),
);

// NPH 51 — Xenograft
// Audit: metadata-only — Needs a persistent creature-type choice made as this enters and a continuous type effect that consumes that stored choice.
pub(in crate::card::sets) static XENOGRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f52f08e1-b234-42e4-8f1f-485a4f6edb3b"),
    "Xenograft",
    crate::card::CardArt::new("f52f08e1-b234-42e4-8f1f-485a4f6edb3b", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 52 — Blind Zealot
pub(in crate::card::sets) static BLIND_ZEALOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bd04df1-5131-455d-b497-fcce4f9af552"),
    "Blind Zealot",
    crate::card::CardArt::new(
        "9bd04df1-5131-455d-b497-fcce4f9af552",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Phyrexian", "Human", "Cleric"], 2, 2)
        .with_abilities(&[
            abilities::intimidate(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature deals combat damage to a player, you may sacrifice it. If you do, destroy target creature that player controls.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::EventPlayer),
                        owner: None,
                    },
                )],
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::Source,
                    count: ValueDef::Constant(1),
                    then: Some(&EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            can_regenerate: true,
                            then: None,
                        }),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: true,
                },
            ),
        ]),
);

// NPH 53 — Caress of Phyrexia
pub(in crate::card::sets) static CARESS_OF_PHYREXIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ef987ad-a3dc-4ef5-90ec-9a8cfa95965b"),
    "Caress of Phyrexia",
    crate::card::CardArt::new("5ef987ad-a3dc-4ef5-90ec-9a8cfa95965b", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws three cards, loses 3 life, and gets three poison counters.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Poison,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// NPH 54 — Chancellor of the Dross
pub(in crate::card::sets) static CHANCELLOR_OF_THE_DROSS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eec6d85e-6263-44b4-a91f-d51585c561c2"),
    "Chancellor of the Dross",
    crate::card::CardArt::new("eec6d85e-6263-44b4-a91f-d51585c561c2", "Stephan Martiniere"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{B}{B}{B}"), &["Phyrexian", "Vampire"], 6, 6)
        .with_abilities(&[
            AbilityDef::opening_hand_reveal(
                "You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, each opponent loses 3 life, then you gain life equal to the life lost this way.",
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the first upkeep, each opponent loses 3 life, then you gain life equal to the life lost this way.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::Sequence(&[
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
                            amount: ValueDef::Constant(3),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(3),
                        },
                    ]),
                ))),
            ),
            abilities::flying(),
            abilities::lifelink(),
        ]),
);

// NPH 55 — Dementia Bat
pub(in crate::card::sets) static DEMENTIA_BAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72ae22c3-2dea-463e-894a-188657849909"),
    "Dementia Bat",
    crate::card::CardArt::new("72ae22c3-2dea-463e-894a-188657849909", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Phyrexian", "Bat"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{4}{B}, Sacrifice this creature: Target player discards two cards.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}{B}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// NPH 56 — Despise
pub(in crate::card::sets) static DESPISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee7bfcd3-9f2b-41f5-93b4-8c1ee6ba4d88"),
    "Despise",
    crate::card::CardArt::new("ee7bfcd3-9f2b-41f5-93b4-8c1ee6ba4d88", "Terese Nielsen"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a creature or planeswalker card from it. That player discards that card.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )),
    )),
);

// NPH 57 — Dismember
pub(in crate::card::sets) static DISMEMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("064dfdeb-485f-473e-9fa0-8fdb7638cdc6"),
    "Dismember",
    crate::card::CardArt::new("064dfdeb-485f-473e-9fa0-8fdb7638cdc6", "Terese Nielsen"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{1}{B/P}{B/P}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets -5/-5 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-5),
                    ValueDef::Constant(-5),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NPH 58 — Enslave
pub(in crate::card::sets) static ENSLAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c6283e1-e4f1-4ff6-be01-b66ab623e0ac"),
    "Enslave",
    crate::card::CardArt::new("17c2f5f0-1f37-4f51-9c10-c02e2ef7d4ee", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{4}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
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
            AbilityDef::triggered(
                "At the beginning of your upkeep, enchanted creature deals 1 damage to its owner.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::DealDamageFrom {
                    source: ObjectRefDef::AttachedToSource,
                    recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// NPH 59 — Entomber Exarch
pub(in crate::card::sets) static ENTOMBER_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f58020e-6d4d-474d-8d4b-cfb7d5a5e9a8"),
    "Entomber Exarch",
    crate::card::CardArt::new("7f58020e-6d4d-474d-8d4b-cfb7d5a5e9a8", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Cleric"], 2, 2)
        .with_ability(AbilityDef::modal_triggered(
            "When this creature enters, choose one —\n• Return target creature card from your graveyard to your hand.\n• Target opponent reveals their hand. You choose a noncreature card from it. That player discards that card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Return target creature card from your graveyard to your hand.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target opponent reveals their hand. You choose a noncreature card from it. That player discards that card.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                    )],
                    EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    )),
                ),
            ],
        )),
);

// NPH 60 — Evil Presence (reprint)

// NPH 61 — Geth's Verdict
pub(in crate::card::sets) static GETH_S_VERDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a20b5a2-8613-49ed-b5cc-7cae9d0e0850"),
    "Geth's Verdict",
    crate::card::CardArt::new("7a20b5a2-8613-49ed-b5cc-7cae9d0e0850", "Whit Brachna"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player sacrifices a creature and loses 1 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                object: ObjectPredicateDef::HasType(CardType::Creature),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 62 — Glistening Oil
pub(in crate::card::sets) static GLISTENING_OIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("483e99fd-7e48-400d-9817-451089089e0c"),
    "Glistening Oil",
    crate::card::CardArt::new("483e99fd-7e48-400d-9817-451089089e0c", "Steven Belledin"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has infect.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::infect()),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a -1/-1 counter on enchanted creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::MinusOneMinusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::dies_trigger(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::TriggeringZoneChangeResult,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// NPH 63 — Grim Affliction
pub(in crate::card::sets) static GRIM_AFFLICTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d5c8ba8-d9f4-440c-8e0b-93699df6343e"),
    "Grim Affliction",
    crate::card::CardArt::new("9d5c8ba8-d9f4-440c-8e0b-93699df6343e", "Erica Yang"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put a -1/-1 counter on target creature, then proliferate.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Proliferate,
        ]),
    )),
);

// NPH 64 — Ichor Explosion
static NEGATIVE_ADDITIONAL_COST_POWER: ValueDef = ValueDef::Negate(&ValueDef::ObjectPower(
    ObjectRefDef::AdditionalCostObject(AdditionalCostObjectIndex::PRIMARY),
));

pub(in crate::card::sets) static ICHOR_EXPLOSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b207e2f-4604-43c5-bb35-a877e35ddd81"),
    "Ichor Explosion",
    crate::card::CardArt::new("0b207e2f-4604-43c5-bb35-a877e35ddd81", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{5}{B}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nAll creatures get -X/-X until end of turn, where X is the sacrificed creature's power.",
            &[],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    NEGATIVE_ADDITIONAL_COST_POWER,
                    NEGATIVE_ADDITIONAL_COST_POWER,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NPH 65 — Life's Finale
pub(in crate::card::sets) static LIFE_S_FINALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ffd3fbd2-87c7-4f08-baaa-91d61c1114da"),
    "Life's Finale",
    crate::card::CardArt::new("ffd3fbd2-87c7-4f08-baaa-91d61c1114da", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{4}{B}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Destroy all creatures, then search target opponent's library for up to three creature cards and put them into their graveyard. Then that player shuffles.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Opponent,
            ))],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    can_regenerate: true,
                    then: None,
                },
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    minimum: 0,
                    maximum: ValueDef::Constant(3),
                    reveal: false,
                    destination: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ]),
        ),
    ),
);

// NPH 66 — Mortis Dogs
pub(in crate::card::sets) static MORTIS_DOGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cae1f40-0e43-41d8-bc5c-aa9873f7d7d5"),
    "Mortis Dogs",
    crate::card::CardArt::new("3cae1f40-0e43-41d8-bc5c-aa9873f7d7d5", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Phyrexian", "Dog"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::dies_trigger_with_targets(
            "When this creature dies, target player loses life equal to its power.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
        ),
    ]),
);

// NPH 67 — Parasitic Implant
pub(in crate::card::sets) static PARASITIC_IMPLANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e34f1bf3-9f3a-47f0-9761-8b2356328a39"),
    "Parasitic Implant",
    crate::card::CardArt::new("e34f1bf3-9f3a-47f0-9761-8b2356328a39", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::triggered(
                "At the beginning of your upkeep, enchanted creature's controller sacrifices it and you create a 1/1 colorless Phyrexian Myr artifact creature token.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    object: ObjectPredicateDef::AttachedToSource,
                    count: ValueDef::Constant(1),
                    then: Some(&EffectDef::create_artifact_creature_token(&["Phyrexian", "Myr"], &[], 1, 1)),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
        ]),
);

// NPH 68 — Phyrexian Obliterator
pub(in crate::card::sets) static PHYREXIAN_OBLITERATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44c4476d-58f9-420d-9545-f5d580c589de"),
    "Phyrexian Obliterator",
    crate::card::CardArt::new("44c4476d-58f9-420d-9545-f5d580c589de", "Todd Lockwood"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{B}{B}{B}{B}"), &["Phyrexian", "Horror"], 5, 5)
        .with_ability(AbilityDef::triggered(
            "Whenever a source deals damage to this creature, that source's controller sacrifices that many permanents.",
            TriggerEventDef::damage_to_source(),
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::ControllerOfTriggeringObject,
                object: ObjectPredicateDef::Any,
                count: ValueDef::TriggerEventAmount,
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        )),
);

// NPH 69 — Pith Driller
pub(in crate::card::sets) static PITH_DRILLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28e960c6-6da0-4679-87eb-55bac890e0c6"),
    "Pith Driller",
    crate::card::CardArt::new("28e960c6-6da0-4679-87eb-55bac890e0c6", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{4}{B/P}"), &["Phyrexian", "Horror"], 2, 4)
        .with_ability(abilities::enters_trigger_with_targets(
            "When this creature enters, put a -1/-1 counter on target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        )),
);

// NPH 70 — Postmortem Lunge
// Audit: metadata-only — Chosen-X targeting and an arrival haste effect are supported, but MoveToZone cannot bind the returned successor into a one-shot delayed exile at the beginning of the next end step.
pub(in crate::card::sets) static POSTMORTEM_LUNGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5f8b46e-1ad3-4c6e-aa63-376f2d222d46"),
    "Postmortem Lunge",
    crate::card::CardArt::new("d5f8b46e-1ad3-4c6e-aa63-376f2d222d46", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 71 — Praetor's Grasp
// Audit: metadata-only — SearchZone cannot compose its hidden-library choice with face-down exile; the face-down linked-exile and durable exile-play permissions are separate operations and cannot grant this spell's controller private access to the searched successor.
pub(in crate::card::sets) static PRAETOR_S_GRASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9588be49-d9b5-4491-a5a0-10bcadc9f8b3"),
    "Praetor's Grasp",
    crate::card::CardArt::new("9588be49-d9b5-4491-a5a0-10bcadc9f8b3", "Steve Argyle"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 72 — Reaper of Sheoldred
pub(in crate::card::sets) static REAPER_OF_SHEOLDRED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a300a645-aec6-4cda-8c11-1e8a6af056ff"),
    "Reaper of Sheoldred",
    crate::card::CardArt::new("a300a645-aec6-4cda-8c11-1e8a6af056ff", "Stephan Martiniere"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Phyrexian", "Horror"], 2, 5)
        .with_abilities(&[
            abilities::infect(),
            AbilityDef::triggered(
                "Whenever a source deals damage to this creature, that source's controller gets a poison counter.",
                TriggerEventDef::damage_to_source(),
                EffectDef::AddPlayerCounters {
                    recipient: EffectRecipientDef::ControllerOfTriggeringObject,
                    kind: CounterKind::Poison,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// NPH 73 — Sheoldred, Whispering One
pub(in crate::card::sets) static SHEOLDRED_WHISPERING_ONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72ddbbac-9914-44ff-b4d4-60989031744e"),
    "Sheoldred, Whispering One",
    crate::card::CardArt::new(
        "3bb8347b-8663-40b8-bdfb-411236d2efc8",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{5}{B}{B}"), &["Phyrexian", "Praetor"], 6, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::landwalk(BasicLandType::Swamp),
            AbilityDef::triggered_with_targets(
                "At the beginning of your upkeep, return target creature card from your graveyard to the battlefield.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                })],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
            ),
            AbilityDef::triggered(
                "At the beginning of each opponent's upkeep, that player sacrifices a creature.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Opponent,
                },
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::EventPlayer,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
        ]),
);

// NPH 74 — Surgical Extraction
// Audit: metadata-only — SharingNameWithBinding can find every copy, but this needs the spell's controller to choose any number across the target owner's public graveyard and private hand/library before the final shuffle; SearchZone gives the hidden-zone decision to that zone's owner.
pub(in crate::card::sets) static SURGICAL_EXTRACTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("114834d8-4da5-48b9-9ac7-5e3e4b7ddf2d"),
    "Surgical Extraction",
    crate::card::CardArt::new("dca7e072-edb5-4f7e-bdec-a3a393053c80", "Steven Belledin"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 75 — Toxic Nim
pub(in crate::card::sets) static TOXIC_NIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5823990c-8d40-4352-8d34-74332934adb2"),
    "Toxic Nim",
    crate::card::CardArt::new("5823990c-8d40-4352-8d34-74332934adb2", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Phyrexian", "Zombie"], 4, 1)
        .with_abilities(&[
            abilities::infect(),
            abilities::regenerate_self(
                "{B}: Regenerate this creature.",
                &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            ),
        ]),
);

// NPH 76 — Vault Skirge
pub(in crate::card::sets) static VAULT_SKIRGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f254239c-c07a-4c41-98f7-8f4de539c73e"),
    "Vault Skirge",
    crate::card::CardArt::new("f254239c-c07a-4c41-98f7-8f4de539c73e", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{1}{B/P}"), &["Phyrexian", "Imp"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

// NPH 77 — Whispering Specter
pub(in crate::card::sets) static WHISPERING_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcb1b486-e336-4e88-b635-b6ff18cb4841"),
    "Whispering Specter",
    crate::card::CardArt::new("bcb1b486-e336-4e88-b635-b6ff18cb4841", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Phyrexian", "Specter"], 1, 1)
        .with_abilities(&[
            abilities::flying(),
            abilities::infect(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, you may sacrifice it. If you do, that player discards a card for each poison counter they have.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::Source,
                    count: ValueDef::Constant(1),
                    then: Some(&EffectDef::Discard {
                            recipient: EffectRecipientDef::EventPlayer,
                            amount: ValueDef::PlayerCounters {
                                player: PlayerRelation::EventPlayer,
                                kind: CounterKind::Poison,
                            },
                            selection: DiscardSelectionDef::RecipientChooses,
                            then: None,
                        }),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: true,
                },
            ),
        ]),
);

// NPH 78 — Act of Aggression
pub(in crate::card::sets) static ACT_OF_AGGRESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61a9f30b-d154-49a4-ad6b-f05601992de3"),
    "Act of Aggression",
    crate::card::CardArt::new("61a9f30b-d154-49a4-ad6b-f05601992de3", "Whit Brachna"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{3}{R/P}{R/P}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature an opponent controls until end of turn. Untap that creature. It gains haste until end of turn.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// NPH 79 — Artillerize
pub(in crate::card::sets) static ARTILLERIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("034522ae-f531-44d9-b186-ada046ce0abc"),
    "Artillerize",
    crate::card::CardArt::new("034522ae-f531-44d9-b186-ada046ce0abc", "Johann Bodin"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice an artifact or creature.\nThis spell deals 5 damage to any target.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ),
);

// NPH 80 — Bludgeon Brawl
// Audit: metadata-only — Needs per-object dynamic equip abilities whose equip cost and granted power bonus both read each affected artifact's mana value.
pub(in crate::card::sets) static BLUDGEON_BRAWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a30fa96d-64d1-423e-a62e-d43453ea838d"),
    "Bludgeon Brawl",
    crate::card::CardArt::new("a30fa96d-64d1-423e-a62e-d43453ea838d", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 81 — Chancellor of the Forge
pub(in crate::card::sets) static CHANCELLOR_OF_THE_FORGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd3520a7-a55f-4c00-b4f1-c1c154adfc8f"),
    "Chancellor of the Forge",
    crate::card::CardArt::new("dd3520a7-a55f-4c00-b4f1-c1c154adfc8f", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{R}{R}{R}"), &["Phyrexian", "Giant"], 5, 5)
        .with_abilities(&[
            AbilityDef::opening_hand_reveal(
                "You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, create a 1/1 red Phyrexian Goblin creature token with haste.",
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the first upkeep, create a 1/1 red Phyrexian Goblin creature token with haste.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::create_creature_token(&["Phyrexian", "Goblin"], &[ManaColor::Red], 1, 1)
                        .with_abilities(&[abilities::haste()]),
                ))),
            ),
            abilities::enters_trigger(
                "When this creature enters, create X 1/1 red Phyrexian Goblin creature tokens with haste, where X is the number of creatures you control.",
                EffectDef::create_creature_token(
                    &["Phyrexian", "Goblin"],
                    &[ManaColor::Red],
                    1,
                    1,
                )
                .with_abilities(&[abilities::haste()])
                .with_count(ValueDef::CountMatchingObjects(
                    &ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
            ),
        ]),
);

// NPH 82 — Fallen Ferromancer
pub(in crate::card::sets) static FALLEN_FERROMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b200986-f553-4156-8f5e-37678db09687"),
    "Fallen Ferromancer",
    crate::card::CardArt::new("7b200986-f553-4156-8f5e-37678db09687", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(
        mana_cost!("{3}{R}"),
        &["Phyrexian", "Human", "Shaman"],
        1,
        1,
    )
    .with_abilities(&[
        abilities::infect(),
        AbilityDef::activated_with_targets(
            "{1}{R}, {T}: This creature deals 1 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// NPH 83 — Flameborn Viron
pub(in crate::card::sets) static FLAMEBORN_VIRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9601ea62-a609-4bc5-a2f0-f7615b4dd5fa"),
    "Flameborn Viron",
    crate::card::CardArt::new("9601ea62-a609-4bc5-a2f0-f7615b4dd5fa", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Phyrexian", "Insect"], 6, 4),
);

// NPH 84 — Furnace Scamp
pub(in crate::card::sets) static FURNACE_SCAMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97538294-058c-47d4-b7a8-4db3753a6628"),
    "Furnace Scamp",
    crate::card::CardArt::new("97538294-058c-47d4-b7a8-4db3753a6628", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{R}"), &["Phyrexian", "Beast"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, you may sacrifice it. If you do, it deals 3 damage to that player.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::Source,
                count: ValueDef::Constant(1),
                then: Some(&EffectDef::DealDamageFrom {
                        source: ObjectRefDef::Source,
                        recipient: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::Constant(3),
                    }),
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: true,
            },
        ),
    ),
);

// NPH 85 — Geosurge
pub(in crate::card::sets) static GEOSURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("118b7aa3-bb05-4691-978e-51486435bf05"),
    "Geosurge",
    crate::card::CardArt::new("118b7aa3-bb05-4691-978e-51486435bf05", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{R}{R}{R}{R}")).with_ability(AbilityDef::spell(
        "Add {R}{R}{R}{R}{R}{R}{R}. Spend this mana only to cast artifact or creature spells.",
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Red)
                .with_amount(7)
                .with_restrictions(&[ManaRestrictionDef::CastSpell(ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]))]),
        ),
    )),
);

// NPH 86 — Gut Shot
pub(in crate::card::sets) static GUT_SHOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a54a2a30-b96a-49c7-9151-1f4b0d4a4413"),
    "Gut Shot",
    CardArt::new("a54a2a30-b96a-49c7-9151-1f4b0d4a4413", "Greg Staples"),
    CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{R/P}")).with_ability(AbilityDef::spell_with_targets(
        "Gut Shot deals 1 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    )),
);

// NPH 87 — Invader Parasite
// Audit: metadata-only — Needs a later land-entry trigger to compare the entering land's name with the land card linked in exile by the enters ability.
pub(in crate::card::sets) static INVADER_PARASITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89a8c53f-2cb0-41ea-8391-c32667f17c30"),
    "Invader Parasite",
    crate::card::CardArt::new("89a8c53f-2cb0-41ea-8391-c32667f17c30", "Volkan Baǵa"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 88 — Moltensteel Dragon
pub(in crate::card::sets) static MOLTENSTEEL_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13b78018-bfbe-43fa-809f-9b52a155e11c"),
    "Moltensteel Dragon",
    crate::card::CardArt::new("13b78018-bfbe-43fa-809f-9b52a155e11c", "James Ryman"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{4}{R/P}{R/P}"), &["Phyrexian", "Dragon"], 4, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated(
                "{R/P}: This creature gets +1/+0 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R/P}"))],
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

// NPH 89 — Ogre Menial
pub(in crate::card::sets) static OGRE_MENIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6271c5c1-5f39-4908-b838-0f34c74e912e"),
    "Ogre Menial",
    crate::card::CardArt::new("6271c5c1-5f39-4908-b838-0f34c74e912e", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Phyrexian", "Ogre"], 0, 4).with_abilities(&[
        abilities::infect(),
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

// NPH 90 — Priest of Urabrask
pub(in crate::card::sets) static PRIEST_OF_URABRASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0a9f49c-f15c-4b2d-b6a5-8efc3c430d87"),
    "Priest of Urabrask",
    crate::card::CardArt::new("d0a9f49c-f15c-4b2d-b6a5-8efc3c430d87", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(
        mana_cost!("{2}{R}"),
        &["Phyrexian", "Human", "Cleric"],
        2,
        1,
    )
    .with_ability(abilities::enters_trigger(
        "When this creature enters, add {R}{R}{R}.",
        EffectDef::AddMana(AddManaEffectDef::combination(&[ManaColor::Red], 3)),
    )),
);

// NPH 91 — Rage Extractor
// Audit: metadata-only — Needs a spell predicate that detects Phyrexian mana symbols in the triggering spell's printed mana cost.
pub(in crate::card::sets) static RAGE_EXTRACTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8cebc2c-a46b-4459-b62b-7fce1a744b11"),
    "Rage Extractor",
    crate::card::CardArt::new("d8cebc2c-a46b-4459-b62b-7fce1a744b11", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 92 — Razor Swine
pub(in crate::card::sets) static RAZOR_SWINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fb022a3-5f9e-491e-8340-087e33f927d6"),
    "Razor Swine",
    crate::card::CardArt::new("2fb022a3-5f9e-491e-8340-087e33f927d6", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Phyrexian", "Boar"], 2, 1)
        .with_abilities(&[abilities::first_strike(), abilities::infect()]),
);

// NPH 93 — Ruthless Invasion
pub(in crate::card::sets) static RUTHLESS_INVASION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc2bbff9-af57-4858-9351-d148b8c4bc3a"),
    "Ruthless Invasion",
    crate::card::CardArt::new("bc2bbff9-af57-4858-9351-d148b8c4bc3a", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{3}{R/P}")).with_ability(AbilityDef::spell(
        "Nonartifact creatures can't block this turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NPH 94 — Scrapyard Salvo
pub(in crate::card::sets) static SCRAPYARD_SALVO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a4874eb-635b-47f0-bbee-6bd8b26e2f10"),
    "Scrapyard Salvo",
    crate::card::CardArt::new("3a4874eb-635b-47f0-bbee-6bd8b26e2f10", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "This spell deals damage to target player or planeswalker equal to the number of artifact cards in your graveyard.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
            },
        ),
    ),
);

// NPH 95 — Slag Fiend
static ARTIFACT_CARDS_IN_ALL_GRAVEYARDS: ObjectQueryDef = ObjectQueryDef::new(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Graveyard],
);

pub(in crate::card::sets) static SLAG_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0d1ee33-e247-4ada-bb01-518611cd7d00"),
    "Slag Fiend",
    crate::card::CardArt::new("c0d1ee33-e247-4ada-bb01-518611cd7d00", "Mike Bierek"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{R}"), &["Phyrexian", "Construct"], 0, 0)
        .with_ability(AbilityDef::static_ability(
            "This creature's power and toughness are each equal to the number of artifact cards in all graveyards.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_base_power_toughness(
                    ValueDef::CountMatchingObjects(&ARTIFACT_CARDS_IN_ALL_GRAVEYARDS),
                    ValueDef::CountMatchingObjects(&ARTIFACT_CARDS_IN_ALL_GRAVEYARDS),
                ),
            },
        )),
);

// NPH 96 — Slash Panther
pub(in crate::card::sets) static SLASH_PANTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f510946-34de-4c12-8998-f61887d1a0e1"),
    "Slash Panther",
    crate::card::CardArt::new("2f510946-34de-4c12-8998-f61887d1a0e1", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{4}{R/P}"), &["Phyrexian", "Cat"], 4, 2)
        .with_abilities(&[abilities::haste()]),
);

// NPH 97 — Tormentor Exarch
pub(in crate::card::sets) static TORMENTOR_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4886eb6a-0f6a-4ea7-8e85-4a27d1a6f03b"),
    "Tormentor Exarch",
    crate::card::CardArt::new("4886eb6a-0f6a-4ea7-8e85-4a27d1a6f03b", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Phyrexian", "Cleric"], 2, 2).with_ability(
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —\n• Target creature gets +2/+0 until end of turn.\n• Target creature gets -0/-2 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Target creature gets +2/+0 until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
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
                    "Target creature gets -0/-2 until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(-2),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ],
        ),
    ),
);

// NPH 98 — Urabrask the Hidden
pub(in crate::card::sets) static URABRASK_THE_HIDDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b06fcab2-891e-4fa3-8583-068ba56c2e27"),
    "Urabrask the Hidden",
    crate::card::CardArt::new("b06fcab2-891e-4fa3-8583-068ba56c2e27", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Phyrexian", "Praetor"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Creatures you control have haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::replacement_for(
                "Creatures your opponents control enter tapped.",
                ReplacementEventDef::ObjectEntersBattlefield {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::Opponent,
                    cast: None,
                },
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
            ),
        ]),
);

// NPH 99 — Victorious Destruction
pub(in crate::card::sets) static VICTORIOUS_DESTRUCTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b81cb30-e9f8-41f3-a10b-26e0ba2503aa"),
    "Victorious Destruction",
    crate::card::CardArt::new("7b81cb30-e9f8-41f3-a10b-26e0ba2503aa", "Jung Park"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or land. Its controller loses 1 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 100 — Volt Charge
pub(in crate::card::sets) static VOLT_CHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa88011c-a19d-4faa-8da6-86b9980cd571"),
    "Volt Charge",
    crate::card::CardArt::new(
        "aa88011c-a19d-4faa-8da6-86b9980cd571",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals 3 damage to any target. Proliferate.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::Proliferate,
        ]),
    )),
);

// NPH 101 — Vulshok Refugee
pub(in crate::card::sets) static VULSHOK_REFUGEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b1615ec-21b3-4575-8b02-fd2bccb930ba"),
    "Vulshok Refugee",
    crate::card::CardArt::new("0b1615ec-21b3-4575-8b02-fd2bccb930ba", "Wayne Reynolds"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Warrior"], 3, 2)
        .with_ability(abilities::protection_from_color(ManaColor::Red)),
);

// NPH 102 — Whipflare
pub(in crate::card::sets) static WHIPFLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a7e6c10-d066-4967-932f-5b6c8d74568b"),
    "Whipflare",
    crate::card::CardArt::new("5a7e6c10-d066-4967-932f-5b6c8d74568b", "Johann Bodin"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "This spell deals 2 damage to each nonartifact creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(2),
        },
    )),
);

// NPH 103 — Beast Within
pub(in crate::card::sets) static BEAST_WITHIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce5b6d19-22e3-4f57-8f4d-a17e982286c7"),
    "Beast Within",
    crate::card::CardArt::new("ce5b6d19-22e3-4f57-8f4d-a17e982286c7", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target permanent. Its controller creates a 3/3 green Beast creature token.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::create_creature_token(&["Beast"], &[ManaColor::Green], 3, 3)
                .with_controller(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
        ]),
    )),
);

// NPH 104 — Birthing Pod
pub(in crate::card::sets) static BIRTHING_POD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b768efa2-e56b-4a7e-ace8-d673f10e0714"),
    "Birthing Pod",
    crate::card::CardArt::new("b768efa2-e56b-4a7e-ace8-d673f10e0714", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}{G/P}")).with_ability(
        AbilityDef::activated(
            "{1}{G/P}, {T}, Sacrifice a creature: Search your library for a creature card with mana value equal to 1 plus the sacrificed creature's mana value, put that card onto the battlefield, then shuffle. Activate only as a sorcery.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{G/P}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::Sum(&SumValueDef::new(ValueDef::SacrificedManaValue, ValueDef::Constant(1)))),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// NPH 105 — Brutalizer Exarch
pub(in crate::card::sets) static BRUTALIZER_EXARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ddfa4ed-70fb-4e25-875d-df0f973f7294"),
    "Brutalizer Exarch",
    crate::card::CardArt::new("9ddfa4ed-70fb-4e25-875d-df0f973f7294", "Mark Zug"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Phyrexian", "Cleric"], 3, 3).with_ability(
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —\n• Search your library for a creature card, reveal it, then shuffle and put that card on top.\n• Put target noncreature permanent on the bottom of its owner's library.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell(
                    "Search your library for a creature card, reveal it, then shuffle and put that card on top.",
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::HasType(CardType::Creature),
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
                AbilityDef::spell_with_targets(
                    "Put target noncreature permanent on the bottom of its owner's library.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Bottom,
                    },
                ),
            ],
        ),
    ),
);

// NPH 106 — Chancellor of the Tangle
pub(in crate::card::sets) static CHANCELLOR_OF_THE_TANGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d129aa8-b637-451e-8123-5221e08cc2cc"),
    "Chancellor of the Tangle",
    crate::card::CardArt::new("6d129aa8-b637-451e-8123-5221e08cc2cc", "Steve Prescott"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{G}{G}{G}"), &["Phyrexian", "Beast"], 6, 7)
        .with_abilities(&[
            AbilityDef::opening_hand_reveal(
                "You may reveal this card from your opening hand. If you do, at the beginning of your first precombat main phase, add {G}.",
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of your first precombat main phase, add {G}.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::PrecombatMain,
                        player: PlayerRelation::You,
                    },
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
                ))),
            ),
            abilities::vigilance(),
            abilities::reach(),
        ]),
);

// NPH 107 — Corrosive Gale
pub(in crate::card::sets) static CORROSIVE_GALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04a13825-ab9b-4ffd-9b59-6198181891b9"),
    "Corrosive Gale",
    crate::card::CardArt::new("04a13825-ab9b-4ffd-9b59-6198181891b9", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{X}{G/P}")).with_ability(AbilityDef::spell(
        "This spell deals X damage to each creature with flying.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::ChosenX,
        },
    )),
);

// NPH 108 — Death-Hood Cobra
pub(in crate::card::sets) static DEATH_HOOD_COBRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5279ac25-8175-44ad-ab7b-dfa17e359a10"),
    "Death-Hood Cobra",
    crate::card::CardArt::new("5279ac25-8175-44ad-ab7b-dfa17e359a10", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Phyrexian", "Snake"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{G}: This creature gains reach until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::reach()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{1}{G}: This creature gains deathtouch until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// NPH 109 — Fresh Meat
// Audit: metadata-only — Needs controller-scoped creature-death history; CreaturesDiedThisTurn is global and would count creatures put into other players' graveyards.
pub(in crate::card::sets) static FRESH_MEAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("000ce65b-5347-4a88-81af-be9053e4d3f3"),
    "Fresh Meat",
    crate::card::CardArt::new("000ce65b-5347-4a88-81af-be9053e4d3f3", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 110 — Glissa's Scorn
pub(in crate::card::sets) static GLISSA_S_SCORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f11187c1-de35-4e85-87c3-656f978b2d7e"),
    "Glissa's Scorn",
    crate::card::CardArt::new("f11187c1-de35-4e85-87c3-656f978b2d7e", "Nils Hamm"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact. Its controller loses 1 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                    ObjectRefDef::Target(TargetIndex::PRIMARY),
                )),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// NPH 111 — Glistener Elf
pub(in crate::card::sets) static GLISTENER_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b94f4c6-b518-43b3-be52-e889d1f3ea38"),
    "Glistener Elf",
    crate::card::CardArt::new("8b94f4c6-b518-43b3-be52-e889d1f3ea38", "Steve Argyle"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{G}"), &["Phyrexian", "Elf", "Warrior"], 1, 1)
        .with_abilities(&[abilities::infect()]),
);

// NPH 112 — Greenhilt Trainee
pub(in crate::card::sets) static GREENHILT_TRAINEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("370f8ef5-c809-43cc-903a-077fad33cd30"),
    "Greenhilt Trainee",
    crate::card::CardArt::new("370f8ef5-c809-43cc-903a-077fad33cd30", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Warrior"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gets +4/+4 until end of turn. Activate only if this creature's power is 4 or greater.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .with_activation_condition(&TriggerConditionDef::SourceMatches {
            object: ObjectPredicateDef::PowerAtLeast(4),
        }),
    ),
);

// NPH 113 — Leeching Bite
pub(in crate::card::sets) static LEECHING_BITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c3bdbeb-c376-42bd-af2a-251cd7ac704c"),
    "Leeching Bite",
    crate::card::CardArt::new("1c3bdbeb-c376-42bd-af2a-251cd7ac704c", "Cos Koniotis"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 until end of turn. Another target creature gets -1/-1 until end of turn.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// NPH 114 — Maul Splicer
pub(in crate::card::sets) static MAUL_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d2c6a6d-5b59-47d7-b290-df3640d9555f"),
    "Maul Splicer",
    crate::card::CardArt::new("2d2c6a6d-5b59-47d7-b290-df3640d9555f", "Jason Chan"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{6}{G}"), &["Phyrexian", "Human", "Artificer"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, create two 3/3 colorless Phyrexian Golem artifact creature tokens.", EffectDef::create_artifact_creature_token(&["Phyrexian", "Golem"], &[], 3, 3).with_amount(2)),
        AbilityDef::static_ability(
            "Golem creatures you control have trample.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::Subtype("Golem"), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
            },
        ),
    ]),
);

// NPH 115 — Melira, Sylvok Outcast
// Audit: metadata-only — Needs player poison-counter placement prevention, creature -1/-1-counter placement prevention, and a continuous removal of infect from creatures opponents control.
pub(in crate::card::sets) static MELIRA_SYLVOK_OUTCAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e83851a1-e4e8-49ec-af5c-4efe86fa51ad"),
    "Melira, Sylvok Outcast",
    crate::card::CardArt::new("e83851a1-e4e8-49ec-af5c-4efe86fa51ad", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 116 — Mutagenic Growth
pub(in crate::card::sets) static MUTAGENIC_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af2d23da-70a1-49ba-91bf-c110cc4bbedc"),
    "Mutagenic Growth",
    crate::card::CardArt::new("af2d23da-70a1-49ba-91bf-c110cc4bbedc", "Dave Kendall"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{G/P}")).with_ability(AbilityDef::spell_with_targets(
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
    )),
);

// NPH 117 — Mycosynth Fiend
// Audit: metadata-only — Needs a continuous power/toughness value that reads poison counters across opponents; player-counter values are not supported in static effects.
pub(in crate::card::sets) static MYCOSYNTH_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdcd1b8e-9f1f-48a3-b7a1-43a32cc03bb1"),
    "Mycosynth Fiend",
    crate::card::CardArt::new("bdcd1b8e-9f1f-48a3-b7a1-43a32cc03bb1", "Kev Walker"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 118 — Noxious Revival
pub(in crate::card::sets) static NOXIOUS_REVIVAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1bdd1243-1d14-496a-9b7a-0c5b34461361"),
    "Noxious Revival",
    crate::card::CardArt::new("1bdd1243-1d14-496a-9b7a-0c5b34461361", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_instant(mana_cost!("{G/P}")).with_ability(AbilityDef::spell_with_targets(
        "Put target card from a graveyard on top of its owner's library.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// NPH 119 — Phyrexian Swarmlord
pub(in crate::card::sets) static PHYREXIAN_SWARMLORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a91dea7-9792-4714-82b0-ba2c06cef304"),
    "Phyrexian Swarmlord",
    crate::card::CardArt::new("8a91dea7-9792-4714-82b0-ba2c06cef304", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(
        mana_cost!("{4}{G}{G}"),
        &["Phyrexian", "Insect", "Horror"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::infect(),
        AbilityDef::triggered(
            "At the beginning of your upkeep, create a 1/1 green Phyrexian Insect creature token with infect for each poison counter your opponents have.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::create_creature_token(
                &["Phyrexian", "Insect"],
                &[ManaColor::Green],
                1,
                1,
            )
            .with_abilities(&[abilities::infect()])
            .with_count(ValueDef::PlayerCounters {
                player: PlayerRelation::Opponent,
                kind: CounterKind::Poison,
            }),
        ),
    ]),
);

// NPH 120 — Rotted Hystrix
pub(in crate::card::sets) static ROTTED_HYSTRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bcae97d-468a-4e16-bfed-d2946f64784c"),
    "Rotted Hystrix",
    crate::card::CardArt::new("7bcae97d-468a-4e16-bfed-d2946f64784c", "Dave Allsop"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Phyrexian", "Beast"], 3, 6),
);

// NPH 121 — Spinebiter
// Audit: metadata-only — Needs a combat-damage assignment option that lets the attacker assign damage as though it were unblocked without actually becoming unblocked.
pub(in crate::card::sets) static SPINEBITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfc79ac6-ffc6-4506-9dea-e20176f960ea"),
    "Spinebiter",
    crate::card::CardArt::new("cfc79ac6-ffc6-4506-9dea-e20176f960ea", "Jaime Jones"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 122 — Thundering Tanadon
pub(in crate::card::sets) static THUNDERING_TANADON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2fab443-0f4b-45ea-8a6d-435b93803409"),
    "Thundering Tanadon",
    crate::card::CardArt::new("e2fab443-0f4b-45ea-8a6d-435b93803409", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{4}{G/P}{G/P}"), &["Phyrexian", "Beast"], 5, 4)
        .with_abilities(&[abilities::trample()]),
);

// NPH 123 — Triumph of the Hordes
pub(in crate::card::sets) static TRIUMPH_OF_THE_HORDES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c16b90ff-d256-4ac6-b687-3430b8c80dd7"),
    "Triumph of the Hordes",
    crate::card::CardArt::new("c16b90ff-d256-4ac6-b687-3430b8c80dd7", "Izzy"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell(
        "Until end of turn, creatures you control get +1/+1 and gain trample and infect.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
                AppliedEffectDef::add_ability(&abilities::infect()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NPH 124 — Viridian Betrayers
// Audit: metadata-only — Player-counter comparisons work for resolving and intervening-if conditions, but the static-condition runtime rejects PlayerCounters because its live layer walk has no supported player-counter value path.
pub(in crate::card::sets) static VIRIDIAN_BETRAYERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc6ea52f-4b24-45ff-99e1-4d0e1bd42875"),
    "Viridian Betrayers",
    crate::card::CardArt::new("cc6ea52f-4b24-45ff-99e1-4d0e1bd42875", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 125 — Viridian Harvest
pub(in crate::card::sets) static VIRIDIAN_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("666eb9a5-b105-45c1-be3e-7ac5cc650338"),
    "Viridian Harvest",
    crate::card::CardArt::new("666eb9a5-b105-45c1-be3e-7ac5cc650338", "Johann Bodin"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant artifact", &abilities::ENCHANT_ARTIFACT_TARGET),
            AbilityDef::triggered(
                "When enchanted artifact is put into a graveyard, you gain 6 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(6),
                },
            ),
        ]),
);

// NPH 126 — Vital Splicer
pub(in crate::card::sets) static VITAL_SPLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("273b982d-bca2-4418-8618-c711d28fc901"),
    "Vital Splicer",
    crate::card::CardArt::new("273b982d-bca2-4418-8618-c711d28fc901", "Daarken"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Phyrexian", "Human", "Artificer"], 1, 1)
        .with_abilities(&[
            abilities::enters_trigger(
                "When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.",
                EffectDef::create_artifact_creature_token(
                    &["Phyrexian", "Golem"],
                    &[],
                    3,
                    3,
                ),
            ),
            AbilityDef::activated_with_targets(
                "{1}: Regenerate target Golem you control.",
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype("Golem"),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// NPH 127 — Vorinclex, Voice of Hunger
// Audit: metadata-only — Needs a mana trigger that reproduces the exact type of mana the triggering land produced plus a turn-based rule that skips that land's controller's next untap of it.
pub(in crate::card::sets) static VORINCLEX_VOICE_OF_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0806adab-6a08-411b-b249-e1c58ade354b"),
    "Vorinclex, Voice of Hunger",
    crate::card::CardArt::new("0806adab-6a08-411b-b249-e1c58ade354b", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 128 — Jor Kadeen, the Prevailer
pub(in crate::card::sets) static JOR_KADEEN_THE_PREVAILER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfd8d7de-a2e1-4f83-85f9-7057eebf0c37"),
    "Jor Kadeen, the Prevailer",
    crate::card::CardArt::new("bfd8d7de-a2e1-4f83-85f9-7057eebf0c37", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_creature(mana_cost!("{3}{R}{W}"), &["Human", "Warrior"], 5, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::static_ability(
                "Metalcraft — Creatures you control get +3/+0 as long as you control three or more artifacts.",
                EffectDef::IfCondition {
                    condition: &METALCRAFT,
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(0)),
                    },
                },
            ),
        ]),
);

// NPH 129 — Alloy Myr
pub(in crate::card::sets) static ALLOY_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abd3350b-89fb-40b4-a942-28e0c8c274aa"),
    "Alloy Myr",
    crate::card::CardArt::new("abd3350b-89fb-40b4-a942-28e0c8c274aa", "Matt Cavotta"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Myr"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ),
);

// NPH 130 — Batterskull
pub(in crate::card::sets) static BATTERSKULL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd114ec3-d286-4c70-a122-3043bc53cc88"),
    "Batterskull",
    crate::card::CardArt::new("cd114ec3-d286-4c70-a122-3043bc53cc88", "Mark Zug"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +4/+4 and has vigilance and lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
            AbilityDef::activated(
                "{3}: Return this Equipment to its owner's hand.",
                &[AbilityCostDef::Mana(mana_cost!("{3}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{5}"))], "Equip {5}"),
        ]),
);

// NPH 131 — Blinding Souleater
pub(in crate::card::sets) static BLINDING_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2220e9dd-f1d8-4a69-9df9-1322e4a5cdc7"),
    "Blinding Souleater",
    crate::card::CardArt::new("2220e9dd-f1d8-4a69-9df9-1322e4a5cdc7", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Cleric"], 1, 3)
        .with_ability(AbilityDef::activated_with_targets(
            "{W/P}, {T}: Tap target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W/P}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )),
);

// NPH 132 — Caged Sun
// Audit: metadata-only — Needs one persistent color choice consumed by both a creature-color continuous effect and a land-mana trigger that reproduces the chosen color.
pub(in crate::card::sets) static CAGED_SUN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("506597cc-48f9-4098-a229-2b3b3c0de944"),
    "Caged Sun",
    crate::card::CardArt::new("506597cc-48f9-4098-a229-2b3b3c0de944", "Scott Chou"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 133 — Conversion Chamber
pub(in crate::card::sets) static CONVERSION_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14d5a8f3-05b6-4bb7-bbe1-e753e22cbb50"),
    "Conversion Chamber",
    crate::card::CardArt::new("14d5a8f3-05b6-4bb7-bbe1-e753e22cbb50", "Anthony Francisco"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}, {T}: Exile target artifact card from a graveyard. Put a charge counter on this artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        AbilityDef::activated(
            "{2}, {T}, Remove a charge counter from this artifact: Create a 3/3 colorless Phyrexian Golem artifact creature token.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                },
            ],
            EffectDef::create_artifact_creature_token(&["Phyrexian", "Golem"], &[], 3, 3),
        ),
    ]),
);

// NPH 134 — Darksteel Relic
pub(in crate::card::sets) static DARKSTEEL_RELIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fd8c918-62d9-41be-a3e1-32ddac71b7e7"),
    "Darksteel Relic",
    crate::card::CardArt::new("0fd8c918-62d9-41be-a3e1-32ddac71b7e7", "Daniel Ljunggren"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(abilities::indestructible()),
);

// NPH 135 — Etched Monstrosity
pub(in crate::card::sets) static ETCHED_MONSTROSITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff9c4451-dd17-4859-a31d-62ed2430c63c"),
    "Etched Monstrosity",
    crate::card::CardArt::new("ff9c4451-dd17-4859-a31d-62ed2430c63c", "Steven Belledin"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Phyrexian", "Golem"], 10, 10)
        .with_abilities(&[
            AbilityDef::as_enters(
                "This creature enters with five -1/-1 counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::MinusOneMinusOne,
                        amount: 5,
                    },
                ),
            ),
            AbilityDef::activated_with_targets(
                "{W}{U}{B}{R}{G}, Remove five -1/-1 counters from this creature: Target player draws three cards.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{W}{U}{B}{R}{G}")),
                    AbilityCostDef::RemoveCountersFromSource {
                        kind: CounterKind::MinusOneMinusOne,
                        amount: 5,
                    },
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Any,
                ))],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// NPH 136 — Gremlin Mine
pub(in crate::card::sets) static GREMLIN_MINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccde7ebb-90de-4174-a1c5-75fc9384deaa"),
    "Gremlin Mine",
    crate::card::CardArt::new("ccde7ebb-90de-4174-a1c5-75fc9384deaa", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this artifact: It deals 4 damage to target artifact creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )],
            EffectDef::DealDamageFrom {
                source: ObjectRefDef::Source,
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}, {T}, Sacrifice this artifact: Remove up to four charge counters from target noncreature artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ]),
            )],
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// NPH 137 — Hex Parasite
// Audit: metadata-only — Counter-count predicates and fixed-kind removal exist, but this needs a variable payment tied to an independently chosen number, a choice across arbitrary counter kinds, and a pump based on the number actually removed.
pub(in crate::card::sets) static HEX_PARASITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43502078-5349-4e29-8e7d-277654a9a71e"),
    "Hex Parasite",
    crate::card::CardArt::new("43502078-5349-4e29-8e7d-277654a9a71e", "Raymond Swanland"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 138 — Hovermyr
pub(in crate::card::sets) static HOVERMYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95e4e445-8333-4cb4-b4fb-80957fae0b97"),
    "Hovermyr",
    crate::card::CardArt::new("95e4e445-8333-4cb4-b4fb-80957fae0b97", "Dan Murayama Scott"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 2)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// NPH 139 — Immolating Souleater
pub(in crate::card::sets) static IMMOLATING_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abbaf916-067d-4834-a55c-b400fe0d8c1f"),
    "Immolating Souleater",
    crate::card::CardArt::new("abbaf916-067d-4834-a55c-b400fe0d8c1f", "Austin Hsu"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Phyrexian", "Dog"], 1, 1).with_ability(
        AbilityDef::activated(
            "{R/P}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R/P}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NPH 140 — Insatiable Souleater
pub(in crate::card::sets) static INSATIABLE_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("171d5213-5bb4-4f5b-9ddd-e2a7ac092ec6"),
    "Insatiable Souleater",
    crate::card::CardArt::new("171d5213-5bb4-4f5b-9ddd-e2a7ac092ec6", "Dave Kendall"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Phyrexian", "Beast"], 5, 1)
        .with_ability(AbilityDef::activated(
            "{G/P}: This creature gains trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G/P}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// NPH 141 — Isolation Cell
pub(in crate::card::sets) static ISOLATION_CELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5e72c64-cb0e-4a04-97d0-3537bb0420cd"),
    "Isolation Cell",
    crate::card::CardArt::new("c5e72c64-cb0e-4a04-97d0-3537bb0420cd", "Adrian Smith"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a creature spell, that player loses 2 life unless they pay {2}.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::PayOr(PayOrDef::unless(
            EffectPaymentDef::mana(
                PlayerSetDef::One(PlayerRefDef::EventPlayer),
                mana_cost!("{2}"),
            ),
            &EffectDef::LoseLife {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        )),
    )),
);

// NPH 142 — Kiln Walker
pub(in crate::card::sets) static KILN_WALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91678632-ebe6-41b6-9250-cd3ffd63663b"),
    "Kiln Walker",
    crate::card::CardArt::new("91678632-ebe6-41b6-9250-cd3ffd63663b", "Volkan Baǵa"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Construct"], 0, 3)
        .with_ability(AbilityDef::triggered(
            "Whenever this creature attacks, it gets +3/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// NPH 143 — Lashwrithe
static SWAMPS_YOU_CONTROL: ObjectQueryDef =
    ObjectQueryDef::controlled_basic_land_type(PlayerRelation::You, BasicLandType::Swamp);

pub(in crate::card::sets) static LASHWRITHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c418159-b5d1-48e9-9a31-707f49d6733b"),
    "Lashwrithe",
    crate::card::CardArt::new("8c418159-b5d1-48e9-9a31-707f49d6733b", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each Swamp you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&SWAMPS_YOU_CONTROL),
                        ValueDef::CountMatchingObjects(&SWAMPS_YOU_CONTROL),
                    ),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{B/P}{B/P}"))],
                "Equip {B/P}{B/P}",
            ),
        ]),
);

// NPH 144 — Mindcrank
// Audit: metadata-only — Needs a life-loss trigger event carrying the exact amount lost for both damage and non-damage life loss.
pub(in crate::card::sets) static MINDCRANK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d13a5ae0-d76a-4430-98c1-47a19e615e2c"),
    "Mindcrank",
    crate::card::CardArt::new("d13a5ae0-d76a-4430-98c1-47a19e615e2c", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 145 — Mycosynth Wellspring
pub(in crate::card::sets) static MYCOSYNTH_WELLSPRING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("097f7ab8-01fa-4699-943a-32075aecebc2"),
    "Mycosynth Wellspring",
    crate::card::CardArt::new("097f7ab8-01fa-4699-943a-32075aecebc2", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "When this artifact enters or is put into a graveyard from the battlefield, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        TriggerEventDef::AnyOf(&[
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
        ]),
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller, source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Supertype(CardSupertype::Basic)]),
            minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top,
            shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None,
        },
    )),
);

// NPH 146 — Myr Superion
// Audit: metadata-only — Needs casting-payment legality that restricts accepted mana by the producing permanent's creature type; mana restrictions currently constrain spend purpose instead.
pub(in crate::card::sets) static MYR_SUPERION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("290c6036-02a3-43fa-b0d4-af3818794c3c"),
    "Myr Superion",
    crate::card::CardArt::new(
        "290c6036-02a3-43fa-b0d4-af3818794c3c",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 147 — Necropouncer
pub(in crate::card::sets) static NECROPOUNCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ed51dbc-bbec-4c78-a71e-26322a8d2439"),
    "Necropouncer",
    crate::card::CardArt::new("4ed51dbc-bbec-4c78-a71e-26322a8d2439", "Cos Koniotis"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{6}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +3/+1 and has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(3),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// NPH 148 — Omen Machine
// Audit: metadata-only — Needs a draw prohibition plus a draw-step replacement that branches on the exiled top card's type and grants a resolution-time free-cast permission.
pub(in crate::card::sets) static OMEN_MACHINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ff4e35f-2a82-4d3c-86c5-ae05a5abc4d7"),
    "Omen Machine",
    crate::card::CardArt::new("0ff4e35f-2a82-4d3c-86c5-ae05a5abc4d7", "David Rapoza"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 149 — Pestilent Souleater
pub(in crate::card::sets) static PESTILENT_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a069cc07-55eb-4ddb-a548-cbf463d078d3"),
    "Pestilent Souleater",
    crate::card::CardArt::new("a069cc07-55eb-4ddb-a548-cbf463d078d3", "Matt Stewart"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Phyrexian", "Insect"], 3, 3)
        .with_ability(AbilityDef::activated(
            "{B/P}: This creature gains infect until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B/P}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::infect()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// NPH 150 — Phyrexian Hulk (reprint)

// NPH 151 — Pristine Talisman
// Audit: metadata-only — Shared mana-ability execution cannot sequence the printed life gain after adding mana.
pub(in crate::card::sets) static PRISTINE_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e30e622d-1e82-4954-8f7d-ee256d5606bf"),
    "Pristine Talisman",
    crate::card::CardArt::new("b31d96cf-7276-46c4-ad17-d6a5c85f1315", "Matt Cavotta"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::unsupported(),
);

// NPH 152 — Shrine of Boundless Growth
pub(in crate::card::sets) static SHRINE_OF_BOUNDLESS_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2ec7b95-667f-43ed-b310-b657befd55a2"),
    "Shrine of Boundless Growth",
    crate::card::CardArt::new("b2ec7b95-667f-43ed-b310-b657befd55a2", "Karl Kopinski"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep and whenever you cast a green spell, put a charge counter on this artifact.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}, Sacrifice this artifact: Add {C} for each charge counter on this artifact.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Colorless,
                amount: ValueDef::CountersOnSource(CounterKind::named("charge")),
            },
        ),
    ]),
);

// NPH 153 — Shrine of Burning Rage
pub(in crate::card::sets) static SHRINE_OF_BURNING_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1a8afef-fa50-4aeb-94de-a4d90b1e5631"),
    "Shrine of Burning Rage",
    crate::card::CardArt::new("d1a8afef-fa50-4aeb-94de-a4d90b1e5631", "Dave Kendall"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep and whenever you cast a red spell, put a charge counter on this artifact.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::Red),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}, Sacrifice this artifact: It deals damage equal to the number of charge counters on it to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::DealDamageFrom {
                source: ObjectRefDef::Source,
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountersOnSource(CounterKind::named("charge")),
            },
        ),
    ]),
);

// NPH 154 — Shrine of Limitless Power
pub(in crate::card::sets) static SHRINE_OF_LIMITLESS_POWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61873223-f378-4478-9cf3-f1326eb76834"),
    "Shrine of Limitless Power",
    crate::card::CardArt::new("61873223-f378-4478-9cf3-f1326eb76834", "Min Yum"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep and whenever you cast a black spell, put a charge counter on this artifact.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{4}, {T}, Sacrifice this artifact: Target player discards a card for each charge counter on this artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountersOnSource(CounterKind::named("charge")),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// NPH 155 — Shrine of Loyal Legions
pub(in crate::card::sets) static SHRINE_OF_LOYAL_LEGIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d13563c7-abe0-4760-9b4c-841de47dbc46"),
    "Shrine of Loyal Legions",
    crate::card::CardArt::new("d13563c7-abe0-4760-9b4c-841de47dbc46", "Igor Kieryluk"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep and whenever you cast a white spell, put a charge counter on this artifact.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::White),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{3}, {T}, Sacrifice this artifact: Create a 1/1 colorless Phyrexian Myr artifact creature token for each charge counter on this artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::create_artifact_creature_token(
                &["Phyrexian", "Myr"],
                &[],
                1,
                1,
            )
            .with_count(ValueDef::CountersOnSource(CounterKind::named("charge"))),
        ),
    ]),
);

// NPH 156 — Shrine of Piercing Vision
pub(in crate::card::sets) static SHRINE_OF_PIERCING_VISION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b150924-f83c-410e-aaab-ff2d06c9d356"),
    "Shrine of Piercing Vision",
    crate::card::CardArt::new(
        "9b150924-f83c-410e-aaab-ff2d06c9d356",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep and whenever you cast a blue spell, put a charge counter on this artifact.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Color(ManaColor::Blue),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            ]),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Look at the top X cards of your library, where X is the number of charge counters on this artifact. Put one of those cards into your hand and the rest on the bottom of your library in any order.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            abilities::look_at_top_cards_choose_to_hand_rest_bottom(
                ValueDef::CountersOnSource(CounterKind::named("charge")),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
    ]),
);

// NPH 157 — Sickleslicer
pub(in crate::card::sets) static SICKLESLICER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d44746d5-3d34-4480-b4cd-c66de72f0622"),
    "Sickleslicer",
    crate::card::CardArt::new("d44746d5-3d34-4480-b4cd-c66de72f0622", "Jason Felix"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        ]),
);

// NPH 158 — Soul Conduit
// Audit: metadata-only — Needs a simultaneous exchange operation that sets two targeted players' life totals to each other's captured values.
pub(in crate::card::sets) static SOUL_CONDUIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa7e4989-cba7-4e0c-bb9d-140af6c006c3"),
    "Soul Conduit",
    crate::card::CardArt::new("aa7e4989-cba7-4e0c-bb9d-140af6c006c3", "Brad Rigney"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 159 — Spellskite
// Audit: metadata-only — Needs a retarget operation that revalidates this permanent as a legal replacement target for the chosen spell or ability.
pub(in crate::card::sets) static SPELLSKITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a84bada-ed6a-4e97-8a0c-05b7cb32d66f"),
    "Spellskite",
    crate::card::CardArt::new("1a84bada-ed6a-4e97-8a0c-05b7cb32d66f", "Chippy"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 160 — Surge Node
pub(in crate::card::sets) static SURGE_NODE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12414fc0-bb24-4244-baf4-adad0125376e"),
    "Surge Node",
    crate::card::CardArt::new("12414fc0-bb24-4244-baf4-adad0125376e", "Lars Grant-West"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters with six charge counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("charge"),
                    amount: 6,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{1}, {T}, Remove a charge counter from this artifact: Put a charge counter on target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::named("charge"),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// NPH 161 — Sword of War and Peace
pub(in crate::card::sets) static SWORD_OF_WAR_AND_PEACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fab5bc6c-8943-4078-866a-5d02f9be0eef"),
    "Sword of War and Peace",
    crate::card::CardArt::new("fab5bc6c-8943-4078-866a-5d02f9be0eef", "Chris Rahn"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2 and has protection from red and from white.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::protection_from_color(
                            ManaColor::Red,
                        )),
                        AppliedEffectDef::add_ability(&abilities::protection_from_color(
                            ManaColor::White,
                        )),
                    ]),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage to a player, this Equipment deals damage to that player equal to the number of cards in their hand and you gain 1 life for each card in your hand.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::AttachedToSource),
                EffectDef::Sequence(&[
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::CardsInHandAbove {
                            player: PlayerRelation::EventPlayer,
                            threshold: 0,
                        },
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::CardsInHandAbove {
                            player: PlayerRelation::You,
                            threshold: 0,
                        },
                    },
                ]),
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// NPH 162 — Torpor Orb
// Audit: metadata-only — Needs a player rule that suppresses triggered abilities caused by creatures entering the battlefield without suppressing other entry events.
pub(in crate::card::sets) static TORPOR_ORB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("953610f6-ea96-4e71-969f-50ecac09c091"),
    "Torpor Orb",
    crate::card::CardArt::new("953610f6-ea96-4e71-969f-50ecac09c091", "Svetlin Velinov"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 163 — Trespassing Souleater
pub(in crate::card::sets) static TRESPASSING_SOULEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5263269-9f64-43de-9e82-408644dbc628"),
    "Trespassing Souleater",
    crate::card::CardArt::new("b5263269-9f64-43de-9e82-408644dbc628", "Scott Chou"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Construct"], 2, 2)
        .with_ability(AbilityDef::activated(
            "{U/P}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U/P}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// NPH 164 — Unwinding Clock
// Audit: metadata-only — Needs a turn-based untap rule that untaps the controller's artifacts during each other player's untap step.
pub(in crate::card::sets) static UNWINDING_CLOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("495d520b-7560-4ecb-ae62-143eeec5682f"),
    "Unwinding Clock",
    crate::card::CardArt::new("495d520b-7560-4ecb-ae62-143eeec5682f", "Mike Bierek"),
    crate::card::CardSet::NewPhyrexia,
    crate::card::CardRules::unsupported(),
);

// NPH 165 — Phyrexia's Core
pub(in crate::card::sets) static PHYREXIA_S_CORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db36c8e7-0c13-4f0d-9947-68cb0e9ea239"),
    "Phyrexia's Core",
    crate::card::CardArt::new("db36c8e7-0c13-4f0d-9947-68cb0e9ea239", "Franz Vohwinkel"),
    crate::card::CardSet::NewPhyrexia,
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice an artifact: You gain 1 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// NPH 166 — Plains (reprint)

// NPH 167 — Plains (alternate printing)

// NPH 168 — Island (reprint)

// NPH 169 — Island (alternate printing)

// NPH 170 — Swamp (reprint)

// NPH 171 — Swamp (alternate printing)

// NPH 172 — Mountain (reprint)

// NPH 173 — Mountain (alternate printing)

// NPH 174 — Forest (reprint)

// NPH 175 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KARN_LIBERATED,
    &APOSTLE_S_BLESSING,
    &AURIOK_SURVIVORS,
    &BLADE_SPLICER,
    &CATHEDRAL_MEMBRANE,
    &CHANCELLOR_OF_THE_ANNEX,
    &DISPATCH,
    &DUE_RESPECT,
    &ELESH_NORN_GRAND_CENOBITE,
    &EXCLUSION_RITUAL,
    &FORCED_WORSHIP,
    &INQUISITOR_EXARCH,
    &LOST_LEONIN,
    &LOXODON_CONVERT,
    &MARROW_SHARDS,
    &MASTER_SPLICER,
    &NORN_S_ANNEX,
    &PHYREXIAN_UNLIFE,
    &PORCELAIN_LEGIONNAIRE,
    &PURESTEEL_PALADIN,
    &REMEMBER_THE_FALLEN,
    &SENSOR_SPLICER,
    &SHATTERED_ANGEL,
    &SHRIEK_RAPTOR,
    &SUTURE_PRIEST,
    &WAR_REPORT,
    &ARGENT_MUTATION,
    &ARM_WITH_AETHER,
    &BLIGHTED_AGENT,
    &CHAINED_THROATSEEKER,
    &CHANCELLOR_OF_THE_SPIRES,
    &CORRUPTED_RESOLVE,
    &DECEIVER_EXARCH,
    &DEFENSIVE_STANCE,
    &GITAXIAN_PROBE,
    &IMPALER_SHRIKE,
    &JIN_GITAXIAS_CORE_AUGUR,
    &MENTAL_MISSTEP,
    &MINDCULLING,
    &NUMBING_DOSE,
    &PHYREXIAN_INGESTER,
    &PHYREXIAN_METAMORPH,
    &PSYCHIC_BARRIER,
    &PSYCHIC_SURGERY,
    &SPINED_THOPTER,
    &SPIRE_MONITOR,
    &TEZZERET_S_GAMBIT,
    &VAPOR_SNAG,
    &VIRAL_DRAKE,
    &WING_SPLICER,
    &XENOGRAFT,
    &BLIND_ZEALOT,
    &CARESS_OF_PHYREXIA,
    &CHANCELLOR_OF_THE_DROSS,
    &DEMENTIA_BAT,
    &DESPISE,
    &DISMEMBER,
    &ENSLAVE,
    &ENTOMBER_EXARCH,
    &GETH_S_VERDICT,
    &GLISTENING_OIL,
    &GRIM_AFFLICTION,
    &ICHOR_EXPLOSION,
    &LIFE_S_FINALE,
    &MORTIS_DOGS,
    &PARASITIC_IMPLANT,
    &PHYREXIAN_OBLITERATOR,
    &PITH_DRILLER,
    &POSTMORTEM_LUNGE,
    &PRAETOR_S_GRASP,
    &REAPER_OF_SHEOLDRED,
    &SHEOLDRED_WHISPERING_ONE,
    &SURGICAL_EXTRACTION,
    &TOXIC_NIM,
    &VAULT_SKIRGE,
    &WHISPERING_SPECTER,
    &ACT_OF_AGGRESSION,
    &ARTILLERIZE,
    &BLUDGEON_BRAWL,
    &CHANCELLOR_OF_THE_FORGE,
    &FALLEN_FERROMANCER,
    &FLAMEBORN_VIRON,
    &FURNACE_SCAMP,
    &GEOSURGE,
    &GUT_SHOT,
    &INVADER_PARASITE,
    &MOLTENSTEEL_DRAGON,
    &OGRE_MENIAL,
    &PRIEST_OF_URABRASK,
    &RAGE_EXTRACTOR,
    &RAZOR_SWINE,
    &RUTHLESS_INVASION,
    &SCRAPYARD_SALVO,
    &SLAG_FIEND,
    &SLASH_PANTHER,
    &TORMENTOR_EXARCH,
    &URABRASK_THE_HIDDEN,
    &VICTORIOUS_DESTRUCTION,
    &VOLT_CHARGE,
    &VULSHOK_REFUGEE,
    &WHIPFLARE,
    &BEAST_WITHIN,
    &BIRTHING_POD,
    &BRUTALIZER_EXARCH,
    &CHANCELLOR_OF_THE_TANGLE,
    &CORROSIVE_GALE,
    &DEATH_HOOD_COBRA,
    &FRESH_MEAT,
    &GLISSA_S_SCORN,
    &GLISTENER_ELF,
    &GREENHILT_TRAINEE,
    &LEECHING_BITE,
    &MAUL_SPLICER,
    &MELIRA_SYLVOK_OUTCAST,
    &MUTAGENIC_GROWTH,
    &MYCOSYNTH_FIEND,
    &NOXIOUS_REVIVAL,
    &PHYREXIAN_SWARMLORD,
    &ROTTED_HYSTRIX,
    &SPINEBITER,
    &THUNDERING_TANADON,
    &TRIUMPH_OF_THE_HORDES,
    &VIRIDIAN_BETRAYERS,
    &VIRIDIAN_HARVEST,
    &VITAL_SPLICER,
    &VORINCLEX_VOICE_OF_HUNGER,
    &JOR_KADEEN_THE_PREVAILER,
    &ALLOY_MYR,
    &BATTERSKULL,
    &BLINDING_SOULEATER,
    &CAGED_SUN,
    &CONVERSION_CHAMBER,
    &DARKSTEEL_RELIC,
    &ETCHED_MONSTROSITY,
    &GREMLIN_MINE,
    &HEX_PARASITE,
    &HOVERMYR,
    &IMMOLATING_SOULEATER,
    &INSATIABLE_SOULEATER,
    &ISOLATION_CELL,
    &KILN_WALKER,
    &LASHWRITHE,
    &MINDCRANK,
    &MYCOSYNTH_WELLSPRING,
    &MYR_SUPERION,
    &NECROPOUNCER,
    &OMEN_MACHINE,
    &PESTILENT_SOULEATER,
    &PRISTINE_TALISMAN,
    &SHRINE_OF_BOUNDLESS_GROWTH,
    &SHRINE_OF_BURNING_RAGE,
    &SHRINE_OF_LIMITLESS_POWER,
    &SHRINE_OF_LOYAL_LEGIONS,
    &SHRINE_OF_PIERCING_VISION,
    &SICKLESLICER,
    &SOUL_CONDUIT,
    &SPELLSKITE,
    &SURGE_NODE,
    &SWORD_OF_WAR_AND_PEACE,
    &TORPOR_ORB,
    &TRESPASSING_SOULEATER,
    &UNWINDING_CLOCK,
    &PHYREXIA_S_CORE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::EVIL_PRESENCE), // NPH 60
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::PHYREXIAN_HULK), // NPH 150
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::PLAINS),        // NPH 166
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),                       // NPH 167
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::ISLAND),        // NPH 168
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),                       // NPH 169
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SWAMP),         // NPH 170
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),                        // NPH 171
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::MOUNTAIN),      // NPH 172
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),                     // NPH 173
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FOREST),        // NPH 174
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),                       // NPH 175
];
