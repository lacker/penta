//! Return to Ravnica card records used by the built-in ISD–M14 Standard deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::{y1993::alpha, y1999::mercadian_masques as mmq, y2012::magic_2013};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef,
    BindObjectsDef, CardArt, CardRules, CardSet, CardSupertype, CardType, CardTypeSet,
    CastTimingPermissionDef, ChoiceVisibilityDef, ChooseCardsFromCollectionDef, ChooseGroupDef,
    ChooseObjectOrderDef, ClassifyObjectsDef, CollectionInspectionDef, ColorSet, ComparisonDef,
    ConditionalStaticEffectDef, ControlDurationDef, CopyExceptionsDef, CostModificationDef,
    CostQuantityDef, CounterKind, CreatureTypeSetDef, DamageAssignmentDef, DamageEventMatcherDef,
    DamagePreventionDef, DiscardSelectionDef, EffectDef, EffectPaymentCostDef, EffectPaymentDef,
    EffectRecipientDef, FreePlayDef, FreePlayDurationDef, HalvedValueDef, IfNoObjectsDef,
    InstalledTriggerDef, KeywordAbility, ManaColor, MillUntilDef, MoveObjectsDef,
    ObjectCollectionSourceDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetCountConditionDef, ObjectSetDef, PartitionGroupDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ReplacementEffectDef, ReplacementEventDef,
    ResolvedEffectDurationDef, RevealObjectsDef, RoundingDef, SacrificedAmountDef,
    SpellAdditionalCostDef, SpellResolutionDestinationDef, StaticApplyDef, TokenStatsDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneChangeEventMatcherDef,
    ZoneKind, ZoneMoveCauseDef, ZonePlacement, abilities,
};
use crate::ids::{Binding, ParentBinding, TargetIndex};
use crate::mana_cost;

#[allow(clippy::too_many_arguments)]
const fn vanilla_creature(
    legacy_id: u64,
    name: &'static str,
    scryfall_id: &'static str,
    artist: &'static str,
    mana_cost: crate::card::ManaCost,
    subtypes: &'static [&'static str],
    power: i16,
    toughness: i16,
) -> CardRecord {
    CardRecord::new_with_legacy_id(
        legacy_id,
        name,
        CardArt::new(scryfall_id, artist),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost, subtypes, power, toughness),
    )
}

#[allow(clippy::large_types_passed_by_value, clippy::too_many_arguments)]
const fn keyword_creature(
    legacy_id: u64,
    name: &'static str,
    scryfall_id: &'static str,
    artist: &'static str,
    mana_cost: crate::card::ManaCost,
    subtypes: &'static [&'static str],
    power: i16,
    toughness: i16,
    ability: AbilityDef,
) -> CardRecord {
    CardRecord::new_with_legacy_id(
        legacy_id,
        name,
        CardArt::new(scryfall_id, artist),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost, subtypes, power, toughness).with_ability(ability),
    )
}

static MULTICOLORED_SPELL: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::ColorCount(2),
    ObjectPredicateDef::ColorCount(3),
    ObjectPredicateDef::ColorCount(4),
    ObjectPredicateDef::ColorCount(5),
]);

const fn keyrune_animation(
    power: i32,
    toughness: i32,
    creature_types: &'static [&'static str],
    colors: ColorSet,
) -> [AppliedEffectDef; 4] {
    [
        AppliedEffectDef::add_card_types(
            CardTypeSet::single(CardType::Creature).with(CardType::Artifact),
        ),
        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(creature_types)),
        AppliedEffectDef::set_colors(colors),
        AppliedEffectDef::set_base_power_toughness(
            ValueDef::Constant(power),
            ValueDef::Constant(toughness),
        ),
    ]
}

// RTR 1 — Angel of Serenity
pub(in crate::card::sets) static ANGEL_OF_SERENITY: CardRecord = CardRecord::new_with_legacy_id(
    131,
    "Angel of Serenity",
    CardArt::new("f10d82f7-7759-457e-a9bb-f9a5bd968f82", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{4}{W}{W}{W}"),
        &["Angel"],
        5,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, you may exile up to three other target creatures from the battlefield and/or creature cards from graveyards.", &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                zones: &[ZoneKind::Battlefield, ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
            3,
        )], EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
},
            }),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, return the exiled cards to their owners' hands.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                zone: ZoneKind::Hand,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// RTR 2 — Armory Guard
pub(in crate::card::sets) static ARMORY_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c03498c1-b7f7-41fb-8e2a-1c087d4e9990"),
    "Armory Guard",
    crate::card::CardArt::new("c03498c1-b7f7-41fb-8e2a-1c087d4e9990", "Karl Kopinski"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Giant", "Soldier"], 2, 5).with_ability(
        AbilityDef::static_ability(
            "This creature has vigilance as long as you control a Gate.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Gate"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    filter: None,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            }),
        ),
    ),
);

// RTR 3 — Arrest (reprint)

// RTR 4 — Avenging Arrow
pub(in crate::card::sets) static AVENGING_ARROW: CardRecord = CardRecord::new_with_legacy_id(
    1905,
    "Avenging Arrow",
    CardArt::new("696678ff-44dc-4fe4-bf17-024e86cd0220", "James Ryman"),
    CardSet::ReturnToRavnica,
    // The revenge is for damage to anything, so a creature that traded in
    // combat is as legal a target as one that connected.
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature that dealt damage this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::DealtDamageThisTurn,
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// RTR 5 — Azorius Arrester
pub(in crate::card::sets) static AZORIUS_ARRESTER: CardRecord = CardRecord::new_with_legacy_id(
    1529,
    "Azorius Arrester",
    CardArt::new("199f7563-563e-483c-8317-5380a83db955", "Wayne Reynolds"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, detain target creature an opponent controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 6 — Azorius Justiciar
pub(in crate::card::sets) static AZORIUS_JUSTICIAR: CardRecord = CardRecord::new_with_legacy_id(
    1530,
    "Azorius Justiciar",
    CardArt::new("9f56272e-c05e-446b-8871-e3783dd29a8b", "Chris Rahn"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, detain up to two target creatures your opponents control.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                2,
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// RTR 7 — Bazaar Krovod
pub(in crate::card::sets) static BAZAAR_KROVOD: CardRecord = CardRecord::new_with_legacy_id(
    1240,
    "Bazaar Krovod",
    CardArt::new("b07bb2fe-3a9b-47d0-864b-99a662d9544b", "Lars Grant-West"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Beast"], 2, 5).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target attacking creature gets +0/+2 until end of turn. Untap that creature.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(2)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ),
);

// RTR 8 — Concordia Pegasus
pub(in crate::card::sets) static CONCORDIA_PEGASUS: CardRecord = keyword_creature(
    1241,
    "Concordia Pegasus",
    "f0333d0b-ae42-48aa-83d8-a4f2c7483a46",
    "Winona Nelson",
    mana_cost!("{1}{W}"),
    &["Pegasus"],
    1,
    3,
    abilities::flying(),
);

// RTR 9 — Ethereal Armor
/// The Armor is itself an enchantment you control, so it always counts at
/// least one -- and every other Aura and enchantment adds to it live.
static ETHEREAL_ARMOR_ENCHANTMENTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Enchantment),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static ETHEREAL_ARMOR: CardRecord = CardRecord::new_with_legacy_id(
    1956,
    "Ethereal Armor",
    CardArt::new("76960e65-e5c7-4414-b9a5-37d7b2ded4a0", "Daarken"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each enchantment you control and has first \
                 strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::CountMatchingObjects(&ETHEREAL_ARMOR_ENCHANTMENTS),
                            ValueDef::CountMatchingObjects(&ETHEREAL_ARMOR_ENCHANTMENTS),
                        ),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                },
            ),
        ]),
);

// RTR 10 — Eyes in the Skies
pub(in crate::card::sets) static EYES_IN_THE_SKIES: CardRecord = CardRecord::new_with_legacy_id(
    1615,
    "Eyes in the Skies",
    CardArt::new("befef095-3429-4dd7-aa01-2f7f619675d4", "James Ryman"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Create a 1/1 white Bird creature token with flying, then populate.",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(&["Bird"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "05b4dbe1-12ac-404f-a1fe-96e0b620533e",
                    "James Ryman",
                )),
            abilities::populate(),
        ]),
    )),
);

// RTR 11 — Fencing Ace
pub(in crate::card::sets) static FENCING_ACE: CardRecord = keyword_creature(
    1242,
    "Fencing Ace",
    "a42d3066-f4ec-4d28-83ab-e48141206c72",
    "David Rapoza",
    mana_cost!("{1}{W}"),
    &["Human", "Soldier"],
    1,
    1,
    abilities::double_strike(),
);

// RTR 12 — Keening Apparition
pub(in crate::card::sets) static KEENING_APPARITION: CardRecord = CardRecord::new_with_legacy_id(
    1243,
    "Keening Apparition",
    CardArt::new("657b242c-46cb-44d1-86fd-fb2485144a5b", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Destroy target enchantment.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// RTR 13 — Knightly Valor
pub(in crate::card::sets) static KNIGHTLY_VALOR: CardRecord = CardRecord::new_with_legacy_id(
    1244,
    "Knightly Valor",
    CardArt::new("122d821f-c8dd-4a3c-a6d7-b42fe5491f02", "Matt Stewart"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{4}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, create a 2/2 white Knight creature token with vigilance.",
                EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2)
                    .with_abilities(&[abilities::vigilance()])
                    .with_art(CardArt::new(
                        "67d3d039-248a-4eb8-be5c-12959b458fea",
                        "Matt Stewart",
                    )),
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
        ]),
);

// RTR 14 — Martial Law
pub(in crate::card::sets) static MARTIAL_LAW: CardRecord = CardRecord::new_with_legacy_id(
    1531,
    "Martial Law",
    CardArt::new("21078b6f-a39d-4ec8-879e-ad10d97c3ff6", "Tyler Jacobson"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, detain target creature an opponent controls.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// RTR 15 — Palisade Giant
// Audit: unsupported — Needs a damage-redirection replacement covering you and every other permanent you control.
pub(in crate::card::sets) static PALISADE_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fdfd37ca-e4c9-4674-a75f-15d8ebcce72b"),
    "Palisade Giant",
    crate::card::CardArt::new("fdfd37ca-e4c9-4674-a75f-15d8ebcce72b", "Greg Staples"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 16 — Phantom General
pub(in crate::card::sets) static PHANTOM_GENERAL: CardRecord = CardRecord::new_with_legacy_id(
    1890,
    "Phantom General",
    CardArt::new(
        "11f42791-070a-4e3a-91c8-b801980abb76",
        "Christopher Moeller",
    ),
    CardSet::ReturnToRavnica,
    // The General is not itself a token, so its anthem never reaches it.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit", "Soldier"], 2, 3).with_ability(
        AbilityDef::static_ability(
            "Creature tokens you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Token,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ),
);

// RTR 17 — Precinct Captain
pub(in crate::card::sets) static PRECINCT_CAPTAIN: CardRecord = CardRecord::new_with_legacy_id(
    1245,
    "Precinct Captain",
    CardArt::new("5f1f6178-4071-401f-bd0d-cac0c5967661", "Steve Prescott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, create a 1/1 white Soldier creature token.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1).with_art(CardArt::new("944a40e8-5469-4d8b-b044-67ff3382ec92", "Steve Prescott")),
            ),
        ]),
);

// RTR 18 — Rest in Peace
pub(in crate::card::sets) static REST_IN_PEACE: CardRecord = CardRecord::new_with_legacy_id(
    202,
    "Rest in Peace",
    CardArt::new("37c2b1d1-faa0-40fd-82f4-216604ce7635", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, exile all graveyards.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::replacement_for(
            "If a card or token would be put into a graveyard from anywhere, exile it instead.",
            ReplacementEventDef::AnyObjectWouldMove {
                to: ZoneKind::Graveyard,
                owner: PlayerRelation::Any,
                tokens: true,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

// RTR 19 — Rootborn Defenses
pub(in crate::card::sets) static ROOTBORN_DEFENSES: CardRecord = CardRecord::new_with_legacy_id(
    1616,
    "Rootborn Defenses",
    CardArt::new("deccfa48-b8df-4dcc-ba1b-920f8352def7", "Mark Zug"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Populate. Creatures you control gain indestructible until end of turn.",
        EffectDef::Sequence(&[
            abilities::populate(),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// RTR 20 — Security Blockade
pub(in crate::card::sets) static SECURITY_BLOCKADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6250b31-8592-48b2-a877-3637c9ee7d49"),
    "Security Blockade",
    crate::card::CardArt::new("e6250b31-8592-48b2-a877-3637c9ee7d49", "James Ryman"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            abilities::enters_trigger(
                "When this Aura enters, create a 2/2 white Knight creature token with vigilance.",
                EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2)
                    .with_abilities(&[abilities::vigilance()]),
            ),
            AbilityDef::static_ability(
                "Enchanted land has “{T}: Prevent the next 1 damage that would be dealt to you this turn.”",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{T}: Prevent the next 1 damage that would be dealt to you this turn.",
                        &[AbilityCostDef::TapSource],
                        EffectDef::PreventDamage {
                            prevention: DamagePreventionDef::amount(
                                DamageEventMatcherDef::to(EffectRecipientDef::Controller),
                                ValueDef::Constant(1),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    )),
                },
            ),
        ]),
);

// RTR 21 — Selesnya Sentry
pub(in crate::card::sets) static SELESNYA_SENTRY: CardRecord = CardRecord::new_with_legacy_id(
    1490,
    "Selesnya Sentry",
    CardArt::new("9c34c1f5-d509-4c66-ba41-c7958ef5ee44", "Wesley Burt"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Elephant", "Soldier"], 3, 2).with_ability(
        abilities::regenerate_self(
            "{5}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{5}{G}"))],
        ),
    ),
);

// RTR 22 — Seller of Songbirds
pub(in crate::card::sets) static SELLER_OF_SONGBIRDS: CardRecord = CardRecord::new_with_legacy_id(
    1246,
    "Seller of Songbirds",
    CardArt::new(
        "2a41edbe-4c5a-4535-a082-235dc3ffe60a",
        "Christopher Moeller",
    ),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Bird creature token with flying.",
            EffectDef::create_creature_token(&["Bird"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "05b4dbe1-12ac-404f-a1fe-96e0b620533e",
                    "James Ryman",
                )),
        ),
    ),
);

// RTR 23 — Soul Tithe
pub(in crate::card::sets) static SOUL_TITHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77e546ce-f498-4136-9015-bb262c301716"),
    "Soul Tithe",
    crate::card::CardArt::new("77e546ce-f498-4136-9015-bb262c301716", "Dave Kendall"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant nonland permanent",
                &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
                    &ObjectPredicateDef::HasType(CardType::Land),
                ))],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of the upkeep of enchanted permanent's controller, that player sacrifices it unless they pay {X}, where X is its mana value.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::ControllerOfAttachedPermanent,
                },
                EffectDef::PayOr(PayOrDef::unless(
                    EffectPaymentDef {
                        payer: PlayerSetDef::Related(PlayerRelation::ControllerOfAttachedPermanent),
                        cost: EffectPaymentCostDef::GenericMana(ValueDef::ObjectManaValue(
                            ObjectRefDef::AttachedToSource,
                        )),
                    },
                    &EffectDef::Sacrifice {
                        object: EffectRecipientDef::AttachedPermanent,
                    },
                )),
            ),
        ]),
);

// RTR 24 — Sphere of Safety
// Audit: unsupported — Needs a per-attacker combat tax whose amount dynamically counts enchantments you control.
pub(in crate::card::sets) static SPHERE_OF_SAFETY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce3bd0d8-7e44-4cf2-9012-8ff0bc39417f"),
    "Sphere of Safety",
    crate::card::CardArt::new("ce3bd0d8-7e44-4cf2-9012-8ff0bc39417f", "Slawomir Maniak"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 25 — Sunspire Griffin
pub(in crate::card::sets) static SUNSPIRE_GRIFFIN: CardRecord = keyword_creature(
    1247,
    "Sunspire Griffin",
    "1388ce6e-8199-46c1-8ee3-71266b0929bf",
    "Johannes Voss",
    mana_cost!("{1}{W}{W}"),
    &["Griffin"],
    2,
    3,
    abilities::flying(),
);

// RTR 26 — Swift Justice
pub(in crate::card::sets) static SWIFT_JUSTICE: CardRecord = CardRecord::new_with_legacy_id(
    1248,
    "Swift Justice",
    CardArt::new("a94801ba-0295-4611-abda-4c6508d69cc3", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target creature gets +1/+0 and gains first strike and lifelink.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
                AppliedEffectDef::add_ability(&abilities::lifelink()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// RTR 27 — Trained Caracal
pub(in crate::card::sets) static TRAINED_CARACAL: CardRecord = keyword_creature(
    1249,
    "Trained Caracal",
    "797e45d1-d17d-40c0-bfdf-ec533784e676",
    "James Ryman",
    mana_cost!("{W}"),
    &["Cat"],
    1,
    1,
    abilities::lifelink(),
);

// RTR 28 — Trostani's Judgment
pub(in crate::card::sets) static TROSTANIS_JUDGMENT: CardRecord = CardRecord::new_with_legacy_id(
    1618,
    "Trostani's Judgment",
    CardArt::new(
        "d707bdb1-1f8c-4cc0-be01-496f3f03b878",
        "Christopher Moeller",
    ),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{5}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature, then populate.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            abilities::populate(),
        ]),
    )),
);

// RTR 29 — Aquus Steed
pub(in crate::card::sets) static AQUUS_STEED: CardRecord = CardRecord::new_with_legacy_id(
    1250,
    "Aquus Steed",
    CardArt::new("af643949-7a9b-4195-8ab8-d43b1928b85a", "Warren Mahy"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Beast"], 1, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{U}, {T}: Target creature gets -2/-0 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 30 — Blustersquall
pub(in crate::card::sets) static BLUSTERSQUALL: CardRecord = CardRecord::new_with_legacy_id(
    1251,
    "Blustersquall",
    CardArt::new("d998847b-323d-406a-b08e-0da66edcc7b3", "Willian Murai"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tap target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::overload(
            mana_cost!("{3}{U}"),
            "Tap each creature you don't control.",
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
            },
        ),
    ]),
);

// RTR 31 — Cancel
pub(in crate::card::sets) static CANCEL: CardRecord = CardRecord::new_with_legacy_id(
    1252,
    "Cancel",
    CardArt::new("fd994a26-65ff-43be-8d52-476e887d3ed2", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
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

// RTR 32 — Chronic Flooding
pub(in crate::card::sets) static CHRONIC_FLOODING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a757425-3cf2-4aca-b415-5ec2d5f753fe"),
    "Chronic Flooding",
    crate::card::CardArt::new("1a757425-3cf2-4aca-b415-5ec2d5f753fe", "Scott Chou"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::triggered(
                "Whenever enchanted land becomes tapped, its controller mills three cards.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::Mill {
                    player: EffectRecipientDef::ControllerOfTriggeringObject,
                    amount: ValueDef::Constant(3),
                },
            ),
        ]),
);

// RTR 33 — Conjured Currency
pub(in crate::card::sets) static CONJURED_CURRENCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7c2ea6e-7d29-4526-b135-9bbb1eed9d4a"),
    "Conjured Currency",
    crate::card::CardArt::new("a7c2ea6e-7d29-4526-b135-9bbb1eed9d4a", "Steve Argyle"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{5}{U}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, you may exchange control of this enchantment and target permanent you neither own nor control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: Some(PlayerRelation::NotYou),
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ExchangeControl {
                    first: EffectRecipientDef::Source,
                    second: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    otherwise: None,
                },
            },
        ),
    ),
);

// RTR 34 — Crosstown Courier
pub(in crate::card::sets) static CROSSTOWN_COURIER: CardRecord = CardRecord::new_with_legacy_id(
    1253,
    "Crosstown Courier",
    CardArt::new("8c8875a3-9f56-4947-9655-aa5d95f06de0", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Vedalken"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player mills that many cards.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Mill {
                player: EffectRecipientDef::EventPlayer,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// RTR 35 — Cyclonic Rift
pub(in crate::card::sets) static CYCLONIC_RIFT: CardRecord = CardRecord::new_with_legacy_id(
    1254,
    "Cyclonic Rift",
    CardArt::new("205c4689-8b02-4d40-9274-3c1fcafa8b82", "Chris Rahn"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target nonland permanent you don't control to its owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::overload(
            mana_cost!("{6}{U}"),
            "Return each nonland permanent you don't control to its owner's hand.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// RTR 36 — Dispel
pub(in crate::card::sets) static DISPEL: CardRecord = CardRecord::new_with_legacy_id(
    155,
    "Dispel",
    CardArt::new("08d4a8d7-c136-472f-8146-a1100701ca4f", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::HasType(CardType::Instant),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// RTR 37 — Doorkeeper
pub(in crate::card::sets) static DOORKEEPER: CardRecord = CardRecord::new_with_legacy_id(
    1255,
    "Doorkeeper",
    CardArt::new("5c31221f-3753-4d5c-905a-6b558ab648ae", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Homunculus"], 0, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated_with_targets(
            "{2}{U}, {T}: Target player mills X where X is the number of creatures you control with defender.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Defender),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        ),
    ]),
);

// RTR 38 — Downsize
pub(in crate::card::sets) static DOWNSIZE: CardRecord = CardRecord::new_with_legacy_id(
    1256,
    "Downsize",
    CardArt::new("e8408a52-d34a-4e03-9312-700dec75d844", "Ryan Pancoast"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you don't control gets -4/-0 until end of turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-4),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{2}{U}"),
            "Each creature you don't control gets -4/-0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-4),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 39 — Faerie Impostor
pub(in crate::card::sets) static FAERIE_IMPOSTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7dfddc9d-85f6-4b37-9973-14c69f6818ec"),
    "Faerie Impostor",
    crate::card::CardArt::new("7dfddc9d-85f6-4b37-9973-14c69f6818ec", "Johann Bodin"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U}"), &["Faerie", "Rogue"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, sacrifice it unless you return another creature you control to its owner's hand.",
            EffectDef::PayOr(PayOrDef::unless(
                EffectPaymentDef {
                    payer: PlayerSetDef::Related(PlayerRelation::You),
                    cost: EffectPaymentCostDef::MovePermanentMatching {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zone: ZoneKind::Hand,
                    },
                },
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ]),
);

// RTR 40 — Hover Barrier
pub(in crate::card::sets) static HOVER_BARRIER: CardRecord = CardRecord::new_with_legacy_id(
    1257,
    "Hover Barrier",
    CardArt::new("884afdb3-0d5f-45a1-b57e-6c3760aa0031", "Mathias Kollros"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Illusion", "Wall"], 0, 6)
        .with_abilities(&[abilities::defender(), abilities::flying()]),
);

// RTR 41 — Inaction Injunction
pub(in crate::card::sets) static INACTION_INJUNCTION: CardRecord = CardRecord::new_with_legacy_id(
    1532,
    "Inaction Injunction",
    CardArt::new("5342ec3c-9d26-474a-9df5-c21ac90bb233", "Wayne Reynolds"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Detain target creature an opponent controls. Draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// RTR 42 — Inspiration
pub(in crate::card::sets) static INSPIRATION: CardRecord = CardRecord::new_with_legacy_id(
    1258,
    "Inspiration",
    CardArt::new("e3cf9dc0-0a12-459c-88e2-97ed94653058", "Izzy"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// RTR 43 — Isperia's Skywatch
pub(in crate::card::sets) static ISPERIAS_SKYWATCH: CardRecord = CardRecord::new_with_legacy_id(
    1533,
    "Isperia's Skywatch",
    CardArt::new("019ba84d-d236-45c5-a8ad-75def7736d0c", "Chris Rahn"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Vedalken", "Knight"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, detain target creature an opponent controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 44 — Jace, Architect of Thought
// Audit: unsupported — Needs controller-driven searches of every player's library followed by grouped free casts.
pub(in crate::card::sets) static JACE_ARCHITECT_OF_THOUGHT: CardRecord =
    CardRecord::new_with_legacy_id(
        180,
        "Jace, Architect of Thought",
        CardArt::new("d4df3a38-678e-42dc-a3fd-d1d399368f07", "Jaime Jones"),
        CardSet::ReturnToRavnica,
        CardRules::unsupported(),
    );

// RTR 45 — Mizzium Skin
static MIZZIUM_SKIN_EFFECT: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(0), ValueDef::Constant(1)),
    AppliedEffectDef::add_ability(&abilities::hexproof()),
]);

pub(in crate::card::sets) static MIZZIUM_SKIN: CardRecord = CardRecord::new_with_legacy_id(
    1259,
    "Mizzium Skin",
    CardArt::new("d9859344-4efc-4b87-a3fb-147e496cee68", "Scott Chou"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control gets +0/+1 and gains hexproof until end of turn.",
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
                effect: MIZZIUM_SKIN_EFFECT,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{1}{U}"),
            "Each creature you control gets +0/+1 and gains hexproof until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: MIZZIUM_SKIN_EFFECT,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 46 — Paralyzing Grasp
pub(in crate::card::sets) static PARALYZING_GRASP: CardRecord = CardRecord::new_with_legacy_id(
    1260,
    "Paralyzing Grasp",
    CardArt::new("3dfd97b3-d83e-406f-af45-40eec6347462", "Scott Chou"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
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

// RTR 47 — Psychic Spiral
pub(in crate::card::sets) static PSYCHIC_SPIRAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59b09c16-f611-4c90-a990-e22bf46bd0e2"),
    "Psychic Spiral",
    crate::card::CardArt::new("59b09c16-f611-4c90-a990-e22bf46bd0e2", "Ryan Pancoast"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{4}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Shuffle all cards from your graveyard into your library. Target player mills that many cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::MoveObjects(MoveObjectsDef {
            input: ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::Any,
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            )),
            from: Some(ZoneKind::Graveyard),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
            moved: Some(ParentBinding),
            then: &EffectDef::Sequence(&[
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::BoundObjectCount(ParentBinding),
                },
            ]),
        }),
    )),
);

// RTR 48 — Runewing
pub(in crate::card::sets) static RUNEWING: CardRecord = CardRecord::new_with_legacy_id(
    1261,
    "Runewing",
    CardArt::new("749961e6-b135-4629-ae9d-124de0d70db9", "Martina Pilcerova"),
    CardSet::ReturnToRavnica,
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

// RTR 49 — Search the City
// Audit: unsupported — Needs source-linked top-card exile, name matching against those and the conditional extra-turn continuation.
pub(in crate::card::sets) static SEARCH_THE_CITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cd77575-fedc-45b1-a53b-dfed0f34c875"),
    "Search the City",
    crate::card::CardArt::new("9cd77575-fedc-45b1-a53b-dfed0f34c875", "Jack Wang"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 50 — Skyline Predator
pub(in crate::card::sets) static SKYLINE_PREDATOR: CardRecord = CardRecord::new_with_legacy_id(
    1262,
    "Skyline Predator",
    CardArt::new("5839556c-6635-44c4-96ed-666e4466b929", "Wesley Burt"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Drake"], 3, 4)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// RTR 51 — Soulsworn Spirit
pub(in crate::card::sets) static SOULSWORN_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    1542,
    "Soulsworn Spirit",
    CardArt::new("32602ed9-c0a7-4498-a333-235eaae628df", "James Ryman"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::cannot_be_blocked("This creature can't be blocked."),
        abilities::enters_trigger_with_targets(
            "When this creature enters, detain target creature an opponent controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 52 — Sphinx of the Chimes
// Audit: unsupported — Needs choosing two same-named nonland cards from hand as a single activation cost.
pub(in crate::card::sets) static SPHINX_OF_THE_CHIMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31a0cae5-925f-4d00-a4da-1db8bae5511b"),
    "Sphinx of the Chimes",
    crate::card::CardArt::new("31a0cae5-925f-4d00-a4da-1db8bae5511b", "Greg Staples"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 53 — Stealer of Secrets
pub(in crate::card::sets) static STEALER_OF_SECRETS: CardRecord = CardRecord::new_with_legacy_id(
    1263,
    "Stealer of Secrets",
    CardArt::new("30ae7001-4d0f-4160-b41c-2fcb83fdb60b", "Michael C. Hayes"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// RTR 54 — Syncopate
pub(in crate::card::sets) static SYNCOPATE: CardRecord = CardRecord::new_with_legacy_id(
    223,
    "Syncopate",
    CardArt::new("ba6f218f-83b0-4b68-a00f-0327cd79f32a", "Clint Cearley"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(
        AbilityDef::spell_with_targets("Counter target spell unless its controller pays {X}. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )], abilities::counter_target_to_exile_unless_paid(ValueDef::ChosenX)),
    ),
);

// RTR 55 — Tower Drake
pub(in crate::card::sets) static TOWER_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1264,
    "Tower Drake",
    CardArt::new("5d759d6f-daf0-47f4-8a35-81c9d6437495", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 1).with_abilities(&[
        abilities::flying(),
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
    ]),
);

// RTR 56 — Voidwielder
pub(in crate::card::sets) static VOIDWIELDER: CardRecord = CardRecord::new_with_legacy_id(
    1265,
    "Voidwielder",
    CardArt::new("23723bc7-a68e-4810-bc87-60df916cbb8a", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Human", "Wizard"], 1, 4).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may return target creature to its owner's hand.",
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
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            },
        ),
    ),
);

// RTR 57 — Assassin's Strike
pub(in crate::card::sets) static ASSASSINS_STRIKE: CardRecord = CardRecord::new_with_legacy_id(
    1266,
    "Assassin's Strike",
    CardArt::new("f796e320-9898-45d4-9d7a-6d35de53c9ab", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. Its controller discards a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// RTR 58 — Catacomb Slug
pub(in crate::card::sets) static CATACOMB_SLUG: CardRecord = vanilla_creature(
    1267,
    "Catacomb Slug",
    "53b36fba-6a0e-4f03-8bee-03919062537f",
    "Nils Hamm",
    mana_cost!("{4}{B}"),
    &["Slug"],
    2,
    6,
);

// RTR 59 — Cremate
pub(in crate::card::sets) static CREMATE: CardRecord = CardRecord::new_with_legacy_id(
    1268,
    "Cremate",
    CardArt::new("013d5260-f906-4f6a-97ed-725197743b60", "Cynthia Sheppard"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target card from a graveyard. Draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// RTR 60 — Daggerdrome Imp
pub(in crate::card::sets) static DAGGERDROME_IMP: CardRecord = CardRecord::new_with_legacy_id(
    1269,
    "Daggerdrome Imp",
    CardArt::new("70639887-bdba-4879-a3f8-c716f97fc325", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Imp"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::lifelink()]),
);

// RTR 61 — Dark Revenant
pub(in crate::card::sets) static DARK_REVENANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2167cc6d-ddb5-4f13-8905-a0c5123b852a"),
    "Dark Revenant",
    crate::card::CardArt::new("2167cc6d-ddb5-4f13-8905-a0c5123b852a", "Daarken"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, put it on top of its owner's library.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::TriggeringZoneChangeResultSuccessor,
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// RTR 62 — Dead Reveler
pub(in crate::card::sets) static DEAD_REVELER: CardRecord = CardRecord::new_with_legacy_id(
    1521,
    "Dead Reveler",
    CardArt::new("909a0a38-2c22-4b49-8938-1a8162c077e6", "David Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 3)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

// RTR 63 — Desecration Demon
pub(in crate::card::sets) static DESECRATION_DEMON: CardRecord = CardRecord::new_with_legacy_id(
    152,
    "Desecration Demon",
    CardArt::new("8242fade-754c-4404-b3fb-f3cccf84b3b6", "Jason Chan"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Demon"],
        6,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of each combat, any opponent may sacrifice a creature of their choice. If a player does, tap this creature and put a +1/+1 counter on it.",
            // Each combat, so on either player's turn.
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::Any,
            },
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                // What the Demon takes when an opponent feeds it: it stays home for the turn
                // and grows permanently.
                then: Some(&EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Source,
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ])),
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: true,
            },
        ),
    ]),
);

// RTR 64 — Destroy the Evidence
pub(in crate::card::sets) static DESTROY_THE_EVIDENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bca53097-108d-457e-831c-e3d6cb499a41"),
    "Destroy the Evidence",
    crate::card::CardArt::new("bca53097-108d-457e-831c-e3d6cb499a41", "Clint Cearley"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Its controller reveals cards from the top of their library until they reveal a land card, then puts those cards into their graveyard.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::MillUntil(&MillUntilDef {
                player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                object: ObjectPredicateDef::HasType(CardType::Land),
                matched_zone: ZoneKind::Graveyard,
            }),
        ]),
    )),
);

// RTR 65 — Deviant Glee
pub(in crate::card::sets) static DEVIANT_GLEE: CardRecord = CardRecord::new_with_legacy_id(
    1270,
    "Deviant Glee",
    CardArt::new("e150896e-8745-42ac-894b-8f42a92bd7a7", "Michael C. Hayes"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1 and has \"{R}: This creature gains trample until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(1)),
                        AppliedEffectDef::add_ability(&AbilityDef::activated(
                            "{R}: This creature gains trample until end of turn.",
                            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )),
                    ]),
                },
            ),
        ]),
);

// RTR 66 — Drainpipe Vermin
pub(in crate::card::sets) static DRAINPIPE_VERMIN: CardRecord = CardRecord::new_with_legacy_id(
    1271,
    "Drainpipe Vermin",
    CardArt::new("4d7251f3-df66-4611-a84c-1897f74431f7", "Trevor Claxton"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, you may pay {B}. If you do, target player discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{B}"),
                ),
                &EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            )),
        ),
    ),
);

// RTR 67 — Grave Betrayal
const GRAVE_BETRAYAL_CARD: Binding = Binding!("grave_betrayal_card");

pub(in crate::card::sets) static GRAVE_BETRAYAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47b38c68-8e72-4afc-bb5e-0b40880fdda9"),
    "Grave Betrayal",
    crate::card::CardArt::new("47b38c68-8e72-4afc-bb5e-0b40880fdda9", "Lucas Graciano"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{5}{B}{B}")).with_ability(
        abilities::dies_trigger_matching(
            "Whenever a creature you don't control dies, return it to the battlefield under your control with an additional +1/+1 counter on it at the beginning of the next end step. That creature is a black Zombie in addition to its other colors and types.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
            ]),
            EffectDef::BindObjects(BindObjectsDef {
                source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(
                    ObjectRefDef::ZoneChangeResultOfTriggeringObject,
                )),
                binding: GRAVE_BETRAYAL_CARD,
                then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                    &AbilityDef::triggered(
                        "At the beginning of the next end step, return that card to the battlefield under your control with an additional +1/+1 counter on it. That creature is a black Zombie in addition to its other colors and types.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::WithZoneMoveResult {
                            effect: &EffectDef::WithBattlefieldArrival {
                                effect: &EffectDef::MoveToZone {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        GRAVE_BETRAYAL_CARD,
                                    )),
                                    zone: ZoneKind::Battlefield,
                                    placement: ZonePlacement::Top,
                                },
                                arrival: crate::card::BattlefieldArrivalDef {
                                    controller: Some(PlayerRelation::You),
                                    modifications: &[
                                        BattlefieldEntryModificationDef::AddCounters {
                                            kind: CounterKind::PlusOnePlusOne,
                                            amount: 1,
                                        },
                                    ],
                                    ..crate::card::BattlefieldArrivalDef::DEFAULT
                                },
                            },
                            binding: ParentBinding,
                            then: &EffectDef::Apply {
                                recipient: EffectRecipientDef::binding_zone_change_successors(
                                    ParentBinding,
                                ),
                                effect: AppliedEffectDef::Composite(&[
                                    AppliedEffectDef::add_colors(ColorSet::from_colors(&[
                                        ManaColor::Black,
                                    ])),
                                    AppliedEffectDef::add_creature_types(
                                        CreatureTypeSetDef::named(&["Zombie"]),
                                    ),
                                ]),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        },
                    ),
                )),
            }),
        ),
    ),
);

// RTR 68 — Grim Roustabout
pub(in crate::card::sets) static GRIM_ROUSTABOUT: CardRecord = CardRecord::new_with_legacy_id(
    1621,
    "Grim Roustabout",
    CardArt::new("1a5ae3f5-5466-4058-a2cd-1a036cb38a8e", "Steven Belledin"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton", "Warrior"], 1, 1).with_abilities(
        &[
            abilities::unleash(),
            abilities::unleash_counter(),
            AbilityDef::activated(
                "{1}{B}: Regenerate this creature.",
                &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::Source,
                },
            ),
        ],
    ),
);

// RTR 69 — Launch Party
pub(in crate::card::sets) static LAUNCH_PARTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53f29821-902e-41bc-97a2-6fc7a710cbdb"),
    "Launch Party",
    crate::card::CardArt::new("53f29821-902e-41bc-97a2-6fc7a710cbdb", "Lucas Graciano"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nDestroy target creature. Its controller loses 2 life.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ),
);

// RTR 70 — Mind Rot (reprint)

// RTR 71 — Necropolis Regent
pub(in crate::card::sets) static NECROPOLIS_REGENT: CardRecord = CardRecord::new_with_legacy_id(
    1272,
    "Necropolis Regent",
    CardArt::new("b421dcc9-0299-416d-86bc-c70ef49bcf98", "Winona Nelson"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Vampire"], 6, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a creature you control deals combat damage to a player, put that many +1/+1 counters on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::TriggeringObject,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ]),
);

// RTR 72 — Ogre Jailbreaker
pub(in crate::card::sets) static OGRE_JAILBREAKER: CardRecord = CardRecord::new_with_legacy_id(
    1957,
    "Ogre Jailbreaker",
    CardArt::new("9a96c83d-96d9-4f8f-8020-77990130ad81", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Ogre", "Rogue"], 4, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "This creature can attack as though it didn't have defender as long as you control a \
             Gate.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype("Gate"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    filter: None,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                // A permission rather than an ability removal: the Ogre keeps defender, so
                // anything reading "a creature with defender" still finds one.
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                },
            }),
        ),
    ]),
);

// RTR 73 — Pack Rat
static RATS_YOU_CONTROL: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Rat"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
));

pub(in crate::card::sets) static PACK_RAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("170693f5-13db-4191-99b1-e527ffb5b88e"),
    "Pack Rat",
    crate::card::CardArt::new("170693f5-13db-4191-99b1-e527ffb5b88e", "Kev Walker"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "This creature's power and toughness are each equal to the number of Rats you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::define_power_toughness(
                    RATS_YOU_CONTROL,
                    RATS_YOU_CONTROL,
                ),
            },
        ),
        AbilityDef::activated(
            "{2}{B}, Discard a card: Create a token that's a copy of this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}")),
                AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::Source,
                exceptions: CopyExceptionsDef::NONE,
            }),
        ),
    ]),
);

// RTR 74 — Perilous Shadow
pub(in crate::card::sets) static PERILOUS_SHADOW: CardRecord = CardRecord::new_with_legacy_id(
    1273,
    "Perilous Shadow",
    CardArt::new("2c101171-a988-4c1d-9954-634e2f1c6f01", "Clint Cearley"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Insect", "Shade"], 0, 4).with_ability(
        AbilityDef::activated(
            "{1}{B}: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 75 — Sewer Shambler
pub(in crate::card::sets) static SEWER_SHAMBLER: CardRecord = CardRecord::new_with_legacy_id(
    1866,
    "Sewer Shambler",
    CardArt::new("0ae7ba14-c901-4266-ae31-812e001916d3", "Nils Hamm"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 1).with_abilities(&[
        abilities::landwalk(BasicLandType::Swamp),
        abilities::scavenge(
            mana_cost!("{2}{B}"),
            "Scavenge {2}{B} ({2}{B}, Exile this card from your graveyard: Put a number of \
             +1/+1 counters equal to this card's power on target creature. Scavenge only as a \
             sorcery.)",
        ),
    ]),
);

// RTR 76 — Shrieking Affliction
pub(in crate::card::sets) static SHRIEKING_AFFLICTION: CardRecord = CardRecord::new_with_legacy_id(
    1274,
    "Shrieking Affliction",
    CardArt::new("dfd08894-2534-4114-9365-40809ba95eb2", "Johann Bodin"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{B}")).with_ability(AbilityDef::triggered_if(
        "At the beginning of each opponent's upkeep, if that player has one or fewer cards in hand, they lose 3 life.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Opponent,
        },
        &TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::Any,
                &[ZoneKind::Hand],
                PlayerRelation::EventPlayer,
            ),
            comparison: ComparisonDef::LessOrEqual,
            amount: 1,
        },
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(3),
        },
    )),
);

// RTR 77 — Slum Reaper
pub(in crate::card::sets) static SLUM_REAPER: CardRecord = CardRecord::new_with_legacy_id(
    1275,
    "Slum Reaper",
    CardArt::new("6f0fea13-63cf-4574-8752-3c357eee4524", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Horror"], 4, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each player sacrifices a creature of their choice.",
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// RTR 78 — Stab Wound
pub(in crate::card::sets) static STAB_WOUND: CardRecord = CardRecord::new_with_legacy_id(
    1903,
    "Stab Wound",
    CardArt::new("7b562269-e6ec-4f8d-844e-26b272248d9d", "Scott Chou"),
    CardSet::ReturnToRavnica,
    // The drain follows the creature: gaining control of it means taking the
    // two a turn as well.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                },
            ),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, that player \
                 loses 2 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(2),
                },
            ),
        ]),
);

// RTR 79 — Tavern Swindler
pub(in crate::card::sets) static TAVERN_SWINDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47240ed3-f256-45fb-ab38-7b07e672d2ed"),
    "Tavern Swindler",
    crate::card::CardArt::new("47240ed3-f256-45fb-ab38-7b07e672d2ed", "Cynthia Sheppard"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Rogue"], 2, 2).with_ability(
        AbilityDef::activated(
            "{T}, Pay 3 life: Flip a coin. If you win the flip, you gain 6 life.",
            &[AbilityCostDef::TapSource, AbilityCostDef::PayLife(3)],
            EffectDef::Randomized {
                likelihood: crate::card::LikelihoodDef::new(0.5),
                on_success: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(6),
                },
                on_failure: &EffectDef::None,
            },
        ),
    ),
);

// RTR 80 — Terrus Wurm
pub(in crate::card::sets) static TERRUS_WURM: CardRecord = CardRecord::new_with_legacy_id(
    1545,
    "Terrus Wurm",
    CardArt::new("1998135c-7b7f-402b-a8a5-4f4af131b1bc", "Cliff Childs"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{6}{B}"), &["Zombie", "Wurm"], 5, 5).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{6}{B}"),
            "Scavenge {6}{B} ({6}{B}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 81 — Thrill-Kill Assassin
pub(in crate::card::sets) static THRILL_KILL_ASSASSIN: CardRecord = CardRecord::new_with_legacy_id(
    1522,
    "Thrill-Kill Assassin",
    CardArt::new("a9f32204-eda7-4184-92e5-9da8b15b2359", "Tyler Jacobson"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Assassin"], 1, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

// RTR 82 — Ultimate Price
pub(in crate::card::sets) static ULTIMATE_PRICE: CardRecord = CardRecord::new_with_legacy_id(
    231,
    "Ultimate Price",
    CardArt::new("d2b4912a-83a2-4870-8fac-81fa79da2830", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target monocolored creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ColorCount(1),
        ])),
        true,
    )),
);

// RTR 83 — Underworld Connections
pub(in crate::card::sets) static UNDERWORLD_CONNECTIONS: CardRecord =
    CardRecord::new_with_legacy_id(
        233,
        "Underworld Connections",
        CardArt::new("19c52e3b-b3b8-4243-96fe-fa4c8eea7c59", "Yeong-Hao Han"),
        CardSet::ReturnToRavnica,
        CardRules::new_enchantment(mana_cost!("{1}{B}{B}"))
            .with_subtypes(&["Aura"])
            .with_abilities(&[
                AbilityDef::spell_with_targets(
                    "Enchant land",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Land),
                            zones: &[ZoneKind::Battlefield],
                            controller: None,
                            owner: None,
                        },
                    )],
                    EffectDef::Attach {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ),
                AbilityDef::static_ability(
                    "Enchanted land has \"{T}, Pay 1 life: Draw a card.\"",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                            "{T}, Pay 1 life: Draw a card.",
                            &[AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)],
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    },
                ),
            ]),
    );

// RTR 84 — Zanikev Locust
pub(in crate::card::sets) static ZANIKEV_LOCUST: CardRecord = CardRecord::new_with_legacy_id(
    1546,
    "Zanikev Locust",
    CardArt::new("adcbd7ee-8958-46fe-abb0-e899e7d2e654", "Cliff Childs"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Insect"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::scavenge(
            mana_cost!("{2}{B}{B}"),
            "Scavenge {2}{B}{B} ({2}{B}{B}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 85 — Annihilating Fire
pub(in crate::card::sets) static ANNIHILATING_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae12fd10-c13e-4777-a233-96204ec75ac1"),
    "Annihilating Fire",
    crate::card::CardArt::new("ae12fd10-c13e-4777-a233-96204ec75ac1", "Clint Cearley"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "This spell deals 3 damage to any target. If a creature dealt damage this way would die this turn, exile it instead.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::DealDamageAndApply {
                amount: ValueDef::Constant(3),
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                applied: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 86 — Ash Zealot
pub(in crate::card::sets) static ASH_ZEALOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f61b1a3-4b3b-4490-a9dc-17aac258cbda"),
    "Ash Zealot",
    crate::card::CardArt::new("1f61b1a3-4b3b-4490-a9dc-17aac258cbda", "Eric Deschamps"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{R}{R}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever a player casts a spell from a graveyard, this creature deals 3 damage to that player.",
            TriggerEventDef::spell_cast_from(ObjectPredicateDef::Any, ZoneKind::Graveyard),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::ControllerOfTriggeringObject,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// RTR 87 — Batterhorn
pub(in crate::card::sets) static BATTERHORN: CardRecord = CardRecord::new_with_legacy_id(
    1276,
    "Batterhorn",
    CardArt::new("a7b40f74-893f-4bfc-87b2-7f8df4c912d8", "Dave Kendall"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Beast"], 4, 3).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target artifact.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            },
        ),
    ),
);

// RTR 88 — Bellows Lizard
pub(in crate::card::sets) static BELLOWS_LIZARD: CardRecord = CardRecord::new_with_legacy_id(
    1277,
    "Bellows Lizard",
    CardArt::new("5da4a644-9809-4591-9007-6b70b5f9d923", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{R}"), &["Lizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
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

// RTR 89 — Bloodfray Giant
pub(in crate::card::sets) static BLOODFRAY_GIANT: CardRecord = CardRecord::new_with_legacy_id(
    1523,
    "Bloodfray Giant",
    CardArt::new("a8c468cd-1255-4257-9a69-c6b40a27c427", "Steve Argyle"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Giant"], 4, 3).with_abilities(&[
        abilities::trample(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

// RTR 90 — Chaos Imps
pub(in crate::card::sets) static CHAOS_IMPS: CardRecord = CardRecord::new_with_legacy_id(
    1622,
    "Chaos Imps",
    CardArt::new("c70a702a-c9be-4bee-9087-22b5905f783a", "Tyler Jacobson"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Imp"], 6, 5).with_abilities(&[
        abilities::flying(),
        abilities::unleash(),
        abilities::unleash_counter(),
        AbilityDef::static_ability(
            "This creature has trample as long as it has a +1/+1 counter on it.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            },
        ),
    ]),
);

// RTR 91 — Cobblebrute
pub(in crate::card::sets) static COBBLEBRUTE: CardRecord = vanilla_creature(
    1278,
    "Cobblebrute",
    "4e038376-801f-454e-a635-0e2d58ccbf7c",
    "Eytan Zana",
    mana_cost!("{3}{R}"),
    &["Elemental"],
    5,
    2,
);

// RTR 92 — Dynacharge
pub(in crate::card::sets) static DYNACHARGE: CardRecord = CardRecord::new_with_legacy_id(
    1279,
    "Dynacharge",
    CardArt::new("e612a032-39be-44cd-a78c-29f89dc384b0", "Matt Stewart"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
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
        abilities::overload(
            mana_cost!("{2}{R}"),
            "Each creature you control gets +2/+0 until end of turn.",
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
    ]),
);

// RTR 93 — Electrickery
pub(in crate::card::sets) static ELECTRICKERY: CardRecord = CardRecord::new_with_legacy_id(
    1280,
    "Electrickery",
    CardArt::new("5ed81ee8-d5e4-4127-876e-9bff81f9c726", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Electrickery deals 1 damage to target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::overload(
            mana_cost!("{1}{R}"),
            "Electrickery deals 1 damage to each creature you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// RTR 94 — Explosive Impact
pub(in crate::card::sets) static EXPLOSIVE_IMPACT: CardRecord = CardRecord::new_with_legacy_id(
    1281,
    "Explosive Impact",
    CardArt::new("3a3e2b45-b086-4ffd-aa1a-1d03046e0d61", "Steve Argyle"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{5}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Explosive Impact deals 5 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// RTR 95 — Goblin Rally
pub(in crate::card::sets) static GOBLIN_RALLY: CardRecord = CardRecord::new_with_legacy_id(
    1282,
    "Goblin Rally",
    CardArt::new("e4ec8ada-09a6-449a-ac4a-7d3acbd08014", "Nic Klein"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_ability(AbilityDef::spell(
        "Create four 1/1 red Goblin creature tokens.",
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
            .with_art(CardArt::new(
                "577c2e32-deb6-40d9-a050-c2acb5bfc05f",
                "Christopher Moeller",
            ))
            .with_amount(4),
    )),
);

// RTR 96 — Gore-House Chainwalker
pub(in crate::card::sets) static GORE_HOUSE_CHAINWALKER: CardRecord =
    CardRecord::new_with_legacy_id(
        1524,
        "Gore-House Chainwalker",
        CardArt::new("56ba132f-95fc-4b99-a1dc-ebe6f622bb41", "Dan Murayama Scott"),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Warrior"], 2, 1)
            .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
    );

// RTR 97 — Guild Feud
const GUILD_FEUD_OPPONENT_CHOSEN: Binding = Binding!("guild_feud_opponent_chosen");
const GUILD_FEUD_OPPONENT_REST: Binding = Binding!("guild_feud_opponent_rest");
const GUILD_FEUD_OPPONENT_ENTERED: Binding = Binding!("guild_feud_opponent_entered");
const GUILD_FEUD_CONTROLLER_CHOSEN: Binding = Binding!("guild_feud_controller_chosen");
const GUILD_FEUD_CONTROLLER_REST: Binding = Binding!("guild_feud_controller_rest");
const GUILD_FEUD_CONTROLLER_ENTERED: Binding = Binding!("guild_feud_controller_entered");
const GUILD_FEUD_OPPONENT_FIGHTER: Binding = Binding!("guild_feud_opponent_fighter");

pub(in crate::card::sets) static GUILD_FEUD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e622878-0aea-4401-873e-d34bf05ee98d"),
    "Guild Feud",
    crate::card::CardArt::new("8e622878-0aea-4401-873e-d34bf05ee98d", "Karl Kopinski"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{5}{R}")).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, target opponent reveals the top three cards of their library, may put a creature card from among them onto the battlefield, then puts the rest into their graveyard. You do the same with the top three cards of your library. If two creatures are put onto the battlefield this way, those creatures fight each other.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::Target(TargetIndex::PRIMARY),
                    count: ValueDef::Constant(3),
                },
                actor: PlayerRefDef::Target(TargetIndex::PRIMARY),
                inspection: CollectionInspectionDef::Reveal,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                minimum: 0,
                maximum: 1,
                chosen: GUILD_FEUD_OPPONENT_CHOSEN,
                remainder: GUILD_FEUD_OPPONENT_REST,
                then: &EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding(GUILD_FEUD_OPPONENT_CHOSEN),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    moved: Some(GUILD_FEUD_OPPONENT_ENTERED),
                    then: &EffectDef::Sequence(&[
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(GUILD_FEUD_OPPONENT_REST),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                            moved: None,
                            then: &EffectDef::None,
                        }),
                        EffectDef::ChooseCardsFromCollection(
                            ChooseCardsFromCollectionDef {
                                source: ObjectCollectionSourceDef::TopCards {
                                    player: PlayerRefDef::EffectController,
                                    count: ValueDef::Constant(3),
                                },
                                actor: PlayerRefDef::EffectController,
                                inspection: CollectionInspectionDef::Reveal,
                                object: ObjectPredicateDef::HasType(CardType::Creature),
                                minimum: 0,
                                maximum: 1,
                                chosen: GUILD_FEUD_CONTROLLER_CHOSEN,
                                remainder: GUILD_FEUD_CONTROLLER_REST,
                                then: &EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(GUILD_FEUD_CONTROLLER_CHOSEN),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Battlefield,
                                    placement: ZonePlacement::Top,
                                    moved: Some(GUILD_FEUD_CONTROLLER_ENTERED),
                                    then: &EffectDef::Sequence(&[
                                        EffectDef::MoveObjects(MoveObjectsDef {
                                            input: ObjectSetDef::Binding(
                                                GUILD_FEUD_CONTROLLER_REST,
                                            ),
                                            from: Some(ZoneKind::Library),
                                            zone: ZoneKind::Graveyard,
                                            placement: ZonePlacement::Top,
                                            moved: None,
                                            then: &EffectDef::None,
                                        }),
                                        EffectDef::ForEachInBinding {
                                            objects: GUILD_FEUD_OPPONENT_ENTERED,
                                            binding: GUILD_FEUD_OPPONENT_FIGHTER,
                                            effect: &EffectDef::ForEachInBinding {
                                                objects: GUILD_FEUD_CONTROLLER_ENTERED,
                                                binding: ParentBinding,
                                                effect: &EffectDef::Fight {
                                                    first: ObjectRefDef::Binding(
                                                        GUILD_FEUD_OPPONENT_FIGHTER,
                                                    ),
                                                    second: ObjectRefDef::Binding(
                                                        ParentBinding,
                                                    ),
                                                    excess: None,
                                                },
                                            },
                                        },
                                    ]),
                                }),
                            },
                        ),
                    ]),
                }),
            }),
        ),
    ),
);

// RTR 98 — Guttersnipe
pub(in crate::card::sets) static GUTTERSNIPE: CardRecord = CardRecord::new_with_legacy_id(
    1283,
    "Guttersnipe",
    CardArt::new("9d8590ea-512c-4e09-97cc-7f07d0706f2b", "Steve Prescott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, this creature deals 2 damage to each opponent.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// RTR 99 — Lobber Crew
pub(in crate::card::sets) static LOBBER_CREW: CardRecord = CardRecord::new_with_legacy_id(
    1284,
    "Lobber Crew",
    CardArt::new("b9d4aa15-a3c2-42a3-a87a-443e7dd20c04", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 0, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{T}: This creature deals 1 damage to each opponent.",
            &[AbilityCostDef::TapSource],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered(
            "Whenever you cast a multicolored spell, untap this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                MULTICOLORED_SPELL,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// RTR 100 — Minotaur Aggressor
pub(in crate::card::sets) static MINOTAUR_AGGRESSOR: CardRecord = CardRecord::new_with_legacy_id(
    1285,
    "Minotaur Aggressor",
    CardArt::new("e22959dc-8759-454e-80b9-623a799af354", "Lucas Graciano"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Minotaur", "Berserker"], 6, 2)
        .with_abilities(&[abilities::first_strike(), abilities::haste()]),
);

// RTR 101 — Mizzium Mortars
pub(in crate::card::sets) static MIZZIUM_MORTARS: CardRecord = CardRecord::new_with_legacy_id(
    186,
    "Mizzium Mortars",
    CardArt::new("d4ded88d-2688-4f5e-a8b2-16216cf9c792", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Mizzium Mortars deals 4 damage to target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        abilities::overload(
            mana_cost!("{3}{R}{R}{R}"),
            "Mizzium Mortars deals 4 damage to each creature you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// RTR 102 — Pursuit of Flight
pub(in crate::card::sets) static PURSUIT_OF_FLIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1286,
    "Pursuit of Flight",
    CardArt::new("37a6290c-a0a8-4032-972b-84a7eef04dae", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has \"{U}: This creature gains flying until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                        AppliedEffectDef::add_ability(&AbilityDef::activated(
                            "{U}: This creature gains flying until end of turn.",
                            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                            },
                        )),
                    ]),
                },
            ),
        ]),
);

// RTR 103 — Pyroconvergence
pub(in crate::card::sets) static PYROCONVERGENCE: CardRecord = CardRecord::new_with_legacy_id(
    1287,
    "Pyroconvergence",
    CardArt::new("6cff95b7-79eb-4796-9a01-31ff355681ab", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{4}{R}"))
        .with_ability(AbilityDef::triggered_with_targets(
        "Whenever you cast a multicolored spell, this enchantment deals 2 damage to any target.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            MULTICOLORED_SPELL,
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// RTR 104 — Racecourse Fury
pub(in crate::card::sets) static RACECOURSE_FURY: CardRecord = CardRecord::new_with_legacy_id(
    1288,
    "Racecourse Fury",
    CardArt::new("15d13d35-b5f7-4d3d-a1fa-84178b28acae", "Sam Burley"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Land),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Target creature gains haste until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                        "{T}: Target creature gains haste until end of turn.",
                        &[AbilityCostDef::TapSource],
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    )),
                },
            ),
        ]),
);

// RTR 105 — Splatter Thug
pub(in crate::card::sets) static SPLATTER_THUG: CardRecord = CardRecord::new_with_legacy_id(
    1525,
    "Splatter Thug",
    CardArt::new("7c511805-3392-4033-8679-811711a0aaca", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::unleash(),
        abilities::unleash_counter(),
    ]),
);

// RTR 106 — Street Spasm
/// "Creature without flying you don't control", shared by both halves: the
/// overload changes only whether it is one of them or all of them.
static STREET_SPASM_GROUNDED: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(KeywordAbility::Flying)),
]);

pub(in crate::card::sets) static STREET_SPASM: CardRecord = CardRecord::new_with_legacy_id(
    1846,
    "Street Spasm",
    CardArt::new("9a19d3b8-80d0-480f-8900-47be527d0e53", "Raymond Swanland"),
    CardSet::ReturnToRavnica,
    // The overload cost doubles X, so the same X costs two more mana and
    // reaches every grounded creature instead of one.
    CardRules::new_instant(mana_cost!("{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Street Spasm deals X damage to target creature without flying you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: STREET_SPASM_GROUNDED,
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
        abilities::overload(
            mana_cost!("{X}{X}{R}{R}"),
            "Street Spasm deals X damage to each creature without flying you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    STREET_SPASM_GROUNDED,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

// RTR 107 — Survey the Wreckage
pub(in crate::card::sets) static SURVEY_THE_WRECKAGE: CardRecord = CardRecord::new_with_legacy_id(
    1289,
    "Survey the Wreckage",
    CardArt::new("a6e750f9-ad86-4d60-98a3-78d11cd52cd1", "Warren Mahy"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Create a 1/1 red Goblin creature token.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_art(
                CardArt::new(
                    "577c2e32-deb6-40d9-a050-c2acb5bfc05f",
                    "Christopher Moeller",
                ),
            ),
        ]),
    )),
);

// RTR 108 — Tenement Crasher
pub(in crate::card::sets) static TENEMENT_CRASHER: CardRecord = keyword_creature(
    1290,
    "Tenement Crasher",
    "44af9170-bd99-4fde-b673-62d988312b2d",
    "Warren Mahy",
    mana_cost!("{5}{R}"),
    &["Beast"],
    5,
    4,
    abilities::haste(),
);

// RTR 109 — Traitorous Instinct
pub(in crate::card::sets) static TRAITOROUS_INSTINCT: CardRecord = CardRecord::new_with_legacy_id(
    1291,
    "Traitorous Instinct",
    CardArt::new("d4456951-844a-4847-b933-c32cfafbfef0", "Daarken"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap that creature. Until end of turn, it gets +2/+0 and gains haste.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                    controller: PlayerRefDef::EffectController,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// RTR 110 — Utvara Hellkite
pub(in crate::card::sets) static UTVARA_HELLKITE: CardRecord = CardRecord::new_with_legacy_id(
    1292,
    "Utvara Hellkite",
    CardArt::new("f17c6478-dd80-4854-9560-bfc5ef597872", "Mark Zug"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{6}{R}{R}"), &["Dragon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a Dragon you control attacks, create a 6/6 red Dragon creature token with flying.",
            TriggerEventDef::attacks(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Subtype("Dragon"),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::create_creature_token(&["Dragon"], &[ManaColor::Red], 6, 6).with_abilities(&[abilities::flying()]).with_art(CardArt::new("84310f84-3e5f-4db8-bff1-16bef64de1a0", "Mark Zug")),
        ),
    ]),
);

// RTR 111 — Vandalblast
pub(in crate::card::sets) static VANDALBLAST: CardRecord = CardRecord::new_with_legacy_id(
    1293,
    "Vandalblast",
    CardArt::new("5925c559-3e3c-481b-ba95-20a405cbffce", "Seb McKinnon"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
        abilities::overload(
            mana_cost!("{4}{R}"),
            "Destroy each artifact you don't control.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::NotYou,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// RTR 112 — Viashino Racketeer
pub(in crate::card::sets) static VIASHINO_RACKETEER: CardRecord = CardRecord::new_with_legacy_id(
    1294,
    "Viashino Racketeer",
    CardArt::new("bf4c2d22-9c36-42cc-854d-f96410bb5cf1", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard", "Rogue"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may discard a card. If you do, draw a card.",
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::discard(PlayerSetDef::Related(PlayerRelation::You), 1),
                &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            )),
        ),
    ),
);

// RTR 113 — Aerial Predation
pub(in crate::card::sets) static AERIAL_PREDATION: CardRecord = CardRecord::new_with_legacy_id(
    1295,
    "Aerial Predation",
    CardArt::new("ec3c023c-037e-495a-b7df-32be42a75f36", "BD"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with flying. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// RTR 114 — Archweaver
pub(in crate::card::sets) static ARCHWEAVER: CardRecord = CardRecord::new_with_legacy_id(
    1296,
    "Archweaver",
    CardArt::new("f99dc8ff-932c-4d56-9253-99ce9e145306", "Jason Felix"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Spider"], 5, 5)
        .with_abilities(&[abilities::reach(), abilities::trample()]),
);

// RTR 115 — Axebane Guardian
pub(in crate::card::sets) static AXEBANE_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("725584fe-9e97-4020-89b1-5e5b45a5beb2"),
    "Axebane Guardian",
    crate::card::CardArt::new("725584fe-9e97-4020-89b1-5e5b45a5beb2", "Slawomir Maniak"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Druid"], 0, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated_mana(
            "{T}: Add X mana in any combination of colors, where X is the number of creatures you control with defender.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::combination(&ManaColor::COLORS, 0).with_variable_amount(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Defender),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            ),
        ),
    ]),
);

// RTR 116 — Axebane Stag
pub(in crate::card::sets) static AXEBANE_STAG: CardRecord = vanilla_creature(
    1297,
    "Axebane Stag",
    "bfce7c02-ccc3-44cd-8087-627eaa6a072e",
    "Martina Pilcerova",
    mana_cost!("{6}{G}"),
    &["Elk"],
    6,
    7,
);

// RTR 117 — Brushstrider
pub(in crate::card::sets) static BRUSHSTRIDER: CardRecord = keyword_creature(
    1298,
    "Brushstrider",
    "59bd1534-52d1-4946-b430-d26f039a9067",
    "Raoul Vitale",
    mana_cost!("{1}{G}"),
    &["Beast"],
    3,
    1,
    abilities::vigilance(),
);

// RTR 118 — Centaur's Herald
pub(in crate::card::sets) static CENTAURS_HERALD: CardRecord = CardRecord::new_with_legacy_id(
    1299,
    "Centaur's Herald",
    CardArt::new("08598b2b-6fd2-4a1d-8d74-7ca6d93ad382", "Howard Lyon"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 0, 1).with_ability(
        AbilityDef::activated(
            "{2}{G}, Sacrifice this creature: Create a 3/3 green Centaur creature token.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::create_creature_token(&["Centaur"], &[ManaColor::Green], 3, 3).with_art(
                CardArt::new("880d5dc1-ceec-4c5f-93c2-c88b7dbfcac2", "Slawomir Maniak"),
            ),
        ),
    ),
);

// RTR 119 — Chorus of Might
static CHORUS_OF_MIGHT_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static CHORUS_OF_MIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1300,
    "Chorus of Might",
    CardArt::new("214dc9c1-154d-4c35-845d-dd2928f1e142", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Until end of turn, target creature gets +1/+1 for each creature you control and gains trample.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CHORUS_OF_MIGHT_CREATURES), ValueDef::CountMatchingObjects(&CHORUS_OF_MIGHT_CREATURES)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 120 — Deadbridge Goliath
pub(in crate::card::sets) static DEADBRIDGE_GOLIATH: CardRecord = CardRecord::new_with_legacy_id(
    1547,
    "Deadbridge Goliath",
    CardArt::new("6ad03e99-25d3-4a09-819b-9192dfd8c9d2", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect"], 5, 5).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{4}{G}{G}"),
            "Scavenge {4}{G}{G} ({4}{G}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 121 — Death's Presence
pub(in crate::card::sets) static DEATHS_PRESENCE: CardRecord = CardRecord::new_with_legacy_id(
    2009,
    "Death's Presence",
    CardArt::new("fa82c57d-4bb9-407c-b973-7abc793b6f47", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    // Nothing you lose is wasted: the body moves its power onto whatever is
    // left standing.
    CardRules::new_enchantment(mana_cost!("{5}{G}")).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a creature you control dies, put X +1/+1 counters on target creature you control, where X is the power of the creature that died.",
            TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            )),
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
                // Last known: the creature is in a graveyard by now, which is
                // the only time the reading is interesting.
                amount: ValueDef::TriggeringObjectPower,
            },
        ),
    ),
);

// RTR 122 — Drudge Beetle
pub(in crate::card::sets) static DRUDGE_BEETLE: CardRecord = CardRecord::new_with_legacy_id(
    1548,
    "Drudge Beetle",
    CardArt::new("e4812e81-beca-4afc-b2f2-24d5ab27abff", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{5}{G}"),
            "Scavenge {5}{G} ({5}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 123 — Druid's Deliverance
pub(in crate::card::sets) static DRUIDS_DELIVERANCE: CardRecord = CardRecord::new_with_legacy_id(
    1864,
    "Druid's Deliverance",
    CardArt::new("83b35961-f4d5-4e14-a793-335147110627", "Dan Murayama Scott"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Prevent all combat damage that would be dealt to you this turn. Populate.",
        // "Dealt to you", so the shield is scoped to its controller rather than
        // covering the whole combat the way a Fog does.
        EffectDef::Sequence(&[
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_to(
                    EffectRecipientDef::Controller,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::populate(),
        ]),
    )),
);

// RTR 124 — Gatecreeper Vine
pub(in crate::card::sets) static GATECREEPER_VINE: CardRecord = CardRecord::new_with_legacy_id(
    1301,
    "Gatecreeper Vine",
    CardArt::new("5dabcc2f-7536-44e3-a495-bbfc526fdc5d", "Trevor Claxton"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant"], 0, 2).with_abilities(&[
        abilities::defender(),
        abilities::enters_trigger("When this creature enters, you may search your library for a basic land card or a Gate card, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ]),
                        ObjectPredicateDef::Subtype("Gate"),
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
            }),
    ]),
);

// RTR 125 — Giant Growth (reprint)

// RTR 126 — Gobbling Ooze
pub(in crate::card::sets) static GOBBLING_OOZE: CardRecord = CardRecord::new_with_legacy_id(
    1302,
    "Gobbling Ooze",
    CardArt::new("465d8a63-0ced-4aec-be34-2098b72c8af6", "Johann Bodin"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Ooze"], 3, 3).with_ability(
        AbilityDef::activated(
            "{G}, Sacrifice another creature: Put a +1/+1 counter on this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// RTR 127 — Golgari Decoy
pub(in crate::card::sets) static GOLGARI_DECOY: CardRecord = CardRecord::new_with_legacy_id(
    1867,
    "Golgari Decoy",
    CardArt::new("511a42a8-71ce-476f-98fa-fc0dc822edcf", "Marco Nelor"),
    CardSet::ReturnToRavnica,
    // A requirement, not a permission: it takes away the alternatives rather
    // than letting anything block that could not already.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "All creatures able to block this creature do so.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )),
            },
        ),
        abilities::scavenge(
            mana_cost!("{3}{G}{G}"),
            "Scavenge {3}{G}{G} ({3}{G}{G}, Exile this card from your graveyard: Put a number \
             of +1/+1 counters equal to this card's power on target creature. Scavenge only as \
             a sorcery.)",
        ),
    ]),
);

// RTR 128 — Horncaller's Chant
pub(in crate::card::sets) static HORNCALLERS_CHANT: CardRecord = CardRecord::new_with_legacy_id(
    1619,
    "Horncaller's Chant",
    CardArt::new("7b8d33ed-9ca2-41d1-ba35-fdeb5b88ad44", "Eric Velhagen"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{7}{G}")).with_ability(AbilityDef::spell(
        "Create a 4/4 green Rhino creature token with trample, then populate.",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(&["Rhino"], &[ManaColor::Green], 4, 4)
                .with_abilities(&[abilities::trample()])
                .with_art(CardArt::new(
                    "1331008a-ae86-4640-b823-a73be766ac16",
                    "Tomasz Jedruszek",
                )),
            abilities::populate(),
        ]),
    )),
);

// RTR 129 — Korozda Monitor
pub(in crate::card::sets) static KOROZDA_MONITOR: CardRecord = CardRecord::new_with_legacy_id(
    1549,
    "Korozda Monitor",
    CardArt::new("2f319d57-7a54-4b10-86d4-58d7b3994844", "Lars Grant-West"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Lizard"], 3, 3).with_abilities(&[
        abilities::trample(),
        abilities::scavenge(
            mana_cost!("{5}{G}{G}"),
            "Scavenge {5}{G}{G} ({5}{G}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 130 — Mana Bloom
const CHARGE_COUNTER: CounterKind = CounterKind::named("charge");

pub(in crate::card::sets) static MANA_BLOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7592d88-64e8-4a31-b00a-f65d4b1867fc"),
    "Mana Bloom",
    crate::card::CardArt::new("d7592d88-64e8-4a31-b00a-f65d4b1867fc", "Mike Bierek"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{X}{G}")).with_abilities(&[
        AbilityDef::as_enters(
            "This enchantment enters with X charge counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CHARGE_COUNTER,
                },
            ),
        ),
        AbilityDef::activated_mana(
            "Remove a charge counter from this enchantment: Add one mana of any color. Activate only once each turn.",
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CHARGE_COUNTER,
                amount: 1,
            }],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        )
        .activations_each_turn(1),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this enchantment has no charge counters on it, return it to its owner's hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceCounters {
                kind: CHARGE_COUNTER,
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// RTR 131 — Oak Street Innkeeper
pub(in crate::card::sets) static OAK_STREET_INNKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08274d1b-52b7-46c1-8f93-7631d2e21def"),
    "Oak Street Innkeeper",
    crate::card::CardArt::new("08274d1b-52b7-46c1-8f93-7631d2e21def", "Svetlin Velinov"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "During turns other than yours, tapped creatures you control have hexproof.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Tapped,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                },
            },
        ),
    ),
);

// RTR 132 — Rubbleback Rhino
pub(in crate::card::sets) static RUBBLEBACK_RHINO: CardRecord = keyword_creature(
    1303,
    "Rubbleback Rhino",
    "51daaf9b-d8a8-49a6-94e1-0c8be2c6188b",
    "Johann Bodin",
    mana_cost!("{4}{G}"),
    &["Rhino"],
    3,
    4,
    abilities::hexproof(),
);

// RTR 133 — Savage Surge
pub(in crate::card::sets) static SAVAGE_SURGE: CardRecord = CardRecord::new_with_legacy_id(
    1304,
    "Savage Surge",
    CardArt::new("0fa74aae-e857-410c-8836-953c8623d0b0", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 until end of turn. Untap that creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
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
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// RTR 134 — Seek the Horizon
pub(in crate::card::sets) static SEEK_THE_HORIZON: CardRecord = CardRecord::new_with_legacy_id(
    1367,
    "Seek the Horizon",
    CardArt::new(
        "b6f52ac7-933f-4b31-8576-338f5dcf4285",
        "Howard Lyon",
    ),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to three basic land reveal them, put them into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(3),
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

// RTR 135 — Slime Molding
pub(in crate::card::sets) static SLIME_MOLDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44a439e8-d586-4995-abfc-3dee5c860968"),
    "Slime Molding",
    crate::card::CardArt::new("44a439e8-d586-4995-abfc-3dee5c860968", "Marco Nelor"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_ability(AbilityDef::spell(
        "Create an X/X green Ooze creature token.",
        EffectDef::create_creature_token(&["Ooze"], &[ManaColor::Green], 0, 0)
            .with_variable_token_stats(&TokenStatsDef {
                power: ValueDef::ChosenX,
                toughness: ValueDef::ChosenX,
            }),
    )),
);

// RTR 136 — Stonefare Crocodile
pub(in crate::card::sets) static STONEFARE_CROCODILE: CardRecord = CardRecord::new_with_legacy_id(
    1305,
    "Stonefare Crocodile",
    CardArt::new("a2517d74-0589-49dc-88f1-1fc02b27bc9d", "Tomasz Jedruszek"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Crocodile"], 3, 2).with_ability(
        AbilityDef::activated(
            "{2}{B}: This creature gains lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 137 — Towering Indrik
pub(in crate::card::sets) static TOWERING_INDRIK: CardRecord = keyword_creature(
    1306,
    "Towering Indrik",
    "c6049e92-6c52-44be-a3c7-aa8e8bf9c10a",
    "Lars Grant-West",
    mana_cost!("{3}{G}"),
    &["Beast"],
    2,
    4,
    abilities::reach(),
);

// RTR 138 — Urban Burgeoning
// Audit: unsupported — Needs an Aura-granted untap action during each other player's untap step.
pub(in crate::card::sets) static URBAN_BURGEONING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("393c230f-5bc3-4b71-b5ac-81d5ce227df5"),
    "Urban Burgeoning",
    crate::card::CardArt::new("393c230f-5bc3-4b71-b5ac-81d5ce227df5", "Nic Klein"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 139 — Wild Beastmaster
pub(in crate::card::sets) static WILD_BEASTMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4d4ef98-949b-49db-b4f9-a070f8b4ff47"),
    "Wild Beastmaster",
    crate::card::CardArt::new("a4d4ef98-949b-49db-b4f9-a070f8b4ff47", "Kev Walker"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Shaman"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, each other creature you control gets +X/+X until end of turn, where X is this creature's power.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::SourcePower,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 140 — Worldspine Wurm
pub(in crate::card::sets) static WORLDSPINE_WURM: CardRecord = CardRecord::new_with_legacy_id(
    2259,
    "Worldspine Wurm",
    CardArt::new("543d55cb-3a6b-4620-af25-10ae74ed32c4", "Richard Wright"),
    CardSet::ReturnToRavnica,
    // Eleven mana nobody pays: it is reanimated or put onto the battlefield
    // some other way, and the shuffle is what stops that from being repeatable.
    CardRules::new_creature(mana_cost!("{8}{G}{G}{G}"), &["Wurm"], 15, 15)
        .with_abilities(&[
            abilities::trample(),
            // A separate ability from the shuffle below, and it only watches the
            // battlefield: a Wurm milled out of a library makes nothing.
            abilities::dies_trigger(
                "When this creature dies, create three 5/5 green Wurm creature tokens with trample.",
                EffectDef::create_creature_token(&["Wurm"], &[ManaColor::Green], 5, 5)
                    .with_abilities(&[abilities::trample()])
                    .with_art(CardArt::new(
                        "33ee3f6c-5df6-4271-b2f9-86b9afffab7b",
                        "Anthony Palumbo",
                    ))
                    .with_amount(3),
            ),
            // A trigger rather than a replacement. The ability belongs to the new
            // graveyard object, even when the previous object was a permanent; the
            // separate dies ability above is the one that looks back at the battlefield.
            AbilityDef::triggered(
                "When Worldspine Wurm is put into a graveyard from anywhere, shuffle it into its owner's \
                 library.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Graveyard)),
                EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::players(PlayerSetDef::One(PlayerRefDef::OwnerOf(
                            ObjectRefDef::Source,
                        ))),
                    },
                ]),
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// RTR 141 — Abrupt Decay
pub(in crate::card::sets) static ABRUPT_DECAY: CardRecord = CardRecord::new_with_legacy_id(
    129,
    "Abrupt Decay",
    CardArt::new("3b1e92b4-6e53-4dba-a572-c67e01965ac5", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Destroy target nonland permanent with mana value 3 or less.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// RTR 142 — Archon of the Triumvirate
pub(in crate::card::sets) static ARCHON_OF_THE_TRIUMVIRATE: CardRecord =
    CardRecord::new_with_legacy_id(
        1534,
        "Archon of the Triumvirate",
        CardArt::new("bf91d847-4a87-4a65-8d6d-e20d538c5cec", "David Rapoza"),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost!("{5}{W}{U}"), &["Archon"], 4, 5).with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature attacks, detain up to two target nonland permanents your \
             opponents control.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
                            CardType::Land,
                        )),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                    2,
                )],
                EffectDef::Detain {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
    );

// RTR 143 — Armada Wurm
pub(in crate::card::sets) static ARMADA_WURM: CardRecord = CardRecord::new_with_legacy_id(
    1307,
    "Armada Wurm",
    CardArt::new("50cb4bf3-70d1-4acc-a1fb-49f4ea74ca16", "Volkan Baǵa"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{G}{G}{W}{W}"), &["Wurm"], 5, 5).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, create a 5/5 green Wurm creature token with trample.",
            EffectDef::create_creature_token(&["Wurm"], &[ManaColor::Green], 5, 5)
                .with_abilities(&[abilities::trample()])
                .with_art(CardArt::new(
                    "33ee3f6c-5df6-4271-b2f9-86b9afffab7b",
                    "Anthony Palumbo",
                )),
        ),
    ]),
);

// RTR 144 — Auger Spree
pub(in crate::card::sets) static AUGER_SPREE: CardRecord = CardRecord::new_with_legacy_id(
    1308,
    "Auger Spree",
    CardArt::new("9580a40b-b413-4f0d-9b38-13903a9d367d", "Raymond Swanland"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +4/-4 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(4),
                ValueDef::Constant(-4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// RTR 145 — Azorius Charm
pub(in crate::card::sets) static AZORIUS_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    139,
    "Azorius Charm",
    CardArt::new("26adc211-d089-4102-91e5-225bbeb5f382", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control gain lifelink until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::spell_with_targets(
                "Put target attacking or blocking creature on top of its owner's library.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::AttackingOrBlocking,
                    ]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
    )),
);

// RTR 146 — Call of the Conclave
pub(in crate::card::sets) static CALL_OF_THE_CONCLAVE: CardRecord = CardRecord::new_with_legacy_id(
    1309,
    "Call of the Conclave",
    CardArt::new("c6df8f4d-a07a-4664-878d-efec8b2affb9", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{G}{W}")).with_ability(AbilityDef::spell(
        "Create a 3/3 green Centaur creature token.",
        EffectDef::create_creature_token(&["Centaur"], &[ManaColor::Green], 3, 3).with_art(
            CardArt::new("880d5dc1-ceec-4c5f-93c2-c88b7dbfcac2", "Slawomir Maniak"),
        ),
    )),
);

// RTR 147 — Carnival Hellsteed
pub(in crate::card::sets) static CARNIVAL_HELLSTEED: CardRecord = CardRecord::new_with_legacy_id(
    1526,
    "Carnival Hellsteed",
    CardArt::new("d8ada7ce-c693-48f0-a6e3-766f61d93370", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{4}{B}{R}"), &["Nightmare", "Horse"], 5, 4).with_abilities(
        &[
            abilities::first_strike(),
            abilities::haste(),
            abilities::unleash(),
            abilities::unleash_counter(),
        ],
    ),
);

// RTR 148 — Centaur Healer
pub(in crate::card::sets) static CENTAUR_HEALER: CardRecord = CardRecord::new_with_legacy_id(
    1310,
    "Centaur Healer",
    CardArt::new("833835d1-9beb-4ad8-b675-7adebdbd7d82", "Mark Zug"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Centaur", "Cleric"], 3, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you gain 3 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// RTR 149 — Chemister's Trick
static CHEMISTERS_TRICK_ATTACK: AbilityDef =
    abilities::attacks_each_combat_if_able("This creature attacks this turn if able.");

pub(in crate::card::sets) static CHEMISTERS_TRICK: CardRecord = CardRecord::new_with_legacy_id(
    1311,
    "Chemister's Trick",
    CardArt::new("dbfc2748-351f-4b5d-8a7e-bc51851578bb", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you don't control gets -2/-0 until end of turn and attacks this turn if able.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&CHEMISTERS_TRICK_ATTACK),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{3}{U}{R}"),
            "Each creature you don't control gets -2/-0 until end of turn and attacks this turn if able.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::NotYou),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-2), ValueDef::Constant(0)),
                    AppliedEffectDef::add_ability(&CHEMISTERS_TRICK_ATTACK),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 150 — Collective Blessing
pub(in crate::card::sets) static COLLECTIVE_BLESSING: CardRecord = CardRecord::new_with_legacy_id(
    1312,
    "Collective Blessing",
    CardArt::new("53c84c4d-e6d6-4eac-9d14-5b6cba914c3d", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{3}{G}{G}{W}")).with_ability(
        AbilityDef::static_ability(
            "Creatures you control get +3/+3.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
            },
        ),
    ),
);

// RTR 151 — Common Bond
pub(in crate::card::sets) static COMMON_BOND: CardRecord = CardRecord::new_with_legacy_id(
    1313,
    "Common Bond",
    CardArt::new("59965953-1522-4103-9a6c-5534205d34d9", "Raymond Swanland"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature. Put a +1/+1 counter on target creature.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
        ],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex(1)),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// RTR 152 — Corpsejack Menace
// Audit: unsupported — Needs a replacement effect that doubles +1/+1 counters placed on creatures you control.
pub(in crate::card::sets) static CORPSEJACK_MENACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b35a8efe-2a3e-4060-9134-d4150e4bdf28"),
    "Corpsejack Menace",
    crate::card::CardArt::new("b35a8efe-2a3e-4060-9134-d4150e4bdf28", "Chris Rahn"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 153 — Counterflux
pub(in crate::card::sets) static COUNTERFLUX: CardRecord = CardRecord::new_with_legacy_id(
    150,
    "Counterflux",
    CardArt::new("94e4b773-40a4-4272-85dd-f728ada22748", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{U}{R}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Counter target spell you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::overload(
            mana_cost!("{1}{U}{U}{R}"),
            "Counter each spell you don't control.",
            EffectDef::Counter {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Spell,
                    &[ZoneKind::Stack],
                    PlayerRelation::NotYou,
                ),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// RTR 154 — Coursers' Accord
pub(in crate::card::sets) static COURSERS_ACCORD: CardRecord = CardRecord::new_with_legacy_id(
    1620,
    "Coursers' Accord",
    CardArt::new("f027ceb0-5d2b-4cf6-87ad-e6b9b1e20634", "Nils Hamm"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{G}{W}")).with_ability(AbilityDef::spell(
        "Create a 3/3 green Centaur creature token, then populate.",
        EffectDef::Sequence(&[
            EffectDef::create_creature_token(&["Centaur"], &[ManaColor::Green], 3, 3).with_art(
                CardArt::new("880d5dc1-ceec-4c5f-93c2-c88b7dbfcac2", "Slawomir Maniak"),
            ),
            abilities::populate(),
        ]),
    )),
);

// RTR 155 — Detention Sphere
pub(in crate::card::sets) static DETENTION_SPHERE: CardRecord = CardRecord::new_with_legacy_id(
    153,
    "Detention Sphere",
    CardArt::new("afee5464-83b7-4d7a-b407-9ee7de21535b", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}{U}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this enchantment enters, you may exile target nonland permanent not named Detention Sphere and all other permanents with the same name as that permanent.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    // By name rather than by identity, so a second Sphere is
                    // no more a legal target than this one.
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasName(ObjectRefDef::Source)),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
face_down: false,
then: None,
},
            }),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled cards to the battlefield under their owner's control.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                counters: None,
                zone: ZoneKind::Battlefield,
                grant: None,
                controller: None,
                transformed: false,
            },
        ),
    ]),
);

// RTR 156 — Dramatic Rescue
pub(in crate::card::sets) static DRAMATIC_RESCUE: CardRecord = CardRecord::new_with_legacy_id(
    1314,
    "Dramatic Rescue",
    CardArt::new("041afd23-1ecc-4cca-9244-fe42203ad689", "Ryan Pancoast"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// RTR 157 — Dreadbore
pub(in crate::card::sets) static DREADBORE: CardRecord = CardRecord::new_with_legacy_id(
    1315,
    "Dreadbore",
    CardArt::new("a83945c6-4dc6-4d9a-9bc2-2d4a264e5422", "Wayne Reynolds"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{B}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target creature or planeswalker.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ])),
        true,
    )),
);

// RTR 158 — Dreg Mangler
pub(in crate::card::sets) static DREG_MANGLER: CardRecord = CardRecord::new_with_legacy_id(
    1550,
    "Dreg Mangler",
    CardArt::new("28d42d6a-e9a0-449e-9b31-436c09b7c1ba", "Peter Mohrbacher"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Plant", "Zombie"], 3, 3).with_abilities(&[
        abilities::haste(),
        abilities::scavenge(
            mana_cost!("{3}{B}{G}"),
            "Scavenge {3}{B}{G} ({3}{B}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 159 — Epic Experiment
const EPIC_EXPERIMENT_CASTABLE: Binding = Binding!("epic_experiment_castable");
const EPIC_EXPERIMENT_EXILED: Binding = Binding!("epic_experiment_exiled");

pub(in crate::card::sets) static EPIC_EXPERIMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42f0b68a-de4b-4c0c-98ac-a812017f88a7"),
    "Epic Experiment",
    crate::card::CardArt::new("42f0b68a-de4b-4c0c-98ac-a812017f88a7", "Dan Murayama Scott"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{X}{U}{R}")).with_ability(AbilityDef::spell(
        "Exile the top X cards of your library. You may cast instant and sorcery spells with mana value X or less from among them without paying their mana costs. Then put all cards exiled this way that weren't cast into your graveyard.",
        abilities::bind_top_cards_then(
            PlayerRefDef::EffectController,
            ValueDef::ChosenX,
            &const {
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding(ParentBinding),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    moved: Some(EPIC_EXPERIMENT_EXILED),
                    then: &EffectDef::ClassifyObjects(ClassifyObjectsDef {
                        input: ObjectSetDef::Binding(EPIC_EXPERIMENT_EXILED),
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                            ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                        ]),
                        matching: EPIC_EXPERIMENT_CASTABLE,
                        remainder: Binding!("epic_experiment_rest"),
                        then: &EffectDef::Sequence(&[
                            EffectDef::MayPlayWithoutPaying(FreePlayDef {
                                objects: ObjectSetDef::Binding(EPIC_EXPERIMENT_CASTABLE),
                                duration: FreePlayDurationDef::WhileResolving,
                                mandatory: false,
                                grants_haste: false,
                            }),
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(EPIC_EXPERIMENT_EXILED),
                                from: Some(ZoneKind::Exile),
                                zone: ZoneKind::Graveyard,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                        ]),
                    }),
                })
            },
        ),
    )),
);

// RTR 160 — Essence Backlash
pub(in crate::card::sets) static ESSENCE_BACKLASH: CardRecord = CardRecord::new_with_legacy_id(
    1316,
    "Essence Backlash",
    CardArt::new("a98609dc-ea90-4c7e-a191-5e5d0ba16847", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{2}{U}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target creature spell. Essence Backlash deals damage equal to that spell's power to its controller.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                    placement: ZonePlacement::Top,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                },
            ]),
        ),
    ),
);

// RTR 161 — Fall of the Gavel
pub(in crate::card::sets) static FALL_OF_THE_GAVEL: CardRecord = CardRecord::new_with_legacy_id(
    1317,
    "Fall of the Gavel",
    CardArt::new("64f42848-963b-4b16-aeec-66d0f349758b", "Matt Stewart"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. You gain 5 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ]),
    )),
);

// RTR 162 — Firemind's Foresight
pub(in crate::card::sets) static FIREMIND_S_FORESIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cb5cb63-e7ec-4fc3-a389-4d8b5a4b96b9"),
    "Firemind's Foresight",
    crate::card::CardArt::new("9cb5cb63-e7ec-4fc3-a389-4d8b5a4b96b9", "Dan Murayama Scott"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{5}{U}{R}")).with_ability(AbilityDef::spell(
        "Search your library for an instant card with mana value 3, reveal it, and put it into your hand. Then repeat this process for instant cards with mana values 2 and 1. Then shuffle.",
        EffectDef::Sequence(&[
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(3)),
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
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(2)),
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
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(1)),
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
        ]),
    )),
);

// RTR 163 — Goblin Electromancer
pub(in crate::card::sets) static GOBLIN_ELECTROMANCER: CardRecord = CardRecord::new_with_legacy_id(
    1760,
    "Goblin Electromancer",
    CardArt::new("725b112d-2637-45c1-aec8-e89981ba5fa3", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Goblin", "Wizard"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "Instant and sorcery spells you cast cost {1} less to cast.",
            EffectDef::ModifyCost(CostModificationDef::reduce_spell(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                PlayerRelation::You,
                ValueDef::Constant(1),
            )),
        ),
    ),
);

// RTR 164 — Golgari Charm
pub(in crate::card::sets) static GOLGARI_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    1491,
    "Golgari Charm",
    CardArt::new("48fce388-eefc-4234-8dd9-1260c1ba97eb", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "All creatures get -1/-1 until end of turn.",
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
            AbilityDef::destroy_target(
                "Destroy target enchantment.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Enchantment,
                )),
                true,
            ),
            AbilityDef::spell(
                "Regenerate each creature you control.",
                EffectDef::Regenerate {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                },
            ),
        ],
    )),
);

// RTR 165 — Grisly Salvage
pub(in crate::card::sets) static GRISLY_SALVAGE: CardRecord = CardRecord::new_with_legacy_id(
    173,
    "Grisly Salvage",
    CardArt::new("dcb5eb2a-ae7a-4416-970c-6e9306689c88", "Dave Kendall"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_ability(AbilityDef::spell(
        "Reveal the top five cards of your library. You may put a creature or land card from among them into your hand. Put the rest into your graveyard.",
        abilities::reveal_top_cards_choose_to_hand_rest_graveyard(
            ValueDef::Constant(5),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            0,
            1,
        ),
    )),
);

// RTR 166 — Havoc Festival
pub(in crate::card::sets) static HAVOC_FESTIVAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04560623-c768-4273-a40d-7e3f39e832cf"),
    "Havoc Festival",
    crate::card::CardArt::new("04560623-c768-4273-a40d-7e3f39e832cf", "Johannes Voss"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{4}{B}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "Players can't gain life.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of each player's upkeep, that player loses half their life, rounded up.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Halved(&HalvedValueDef::new(
                    ValueDef::LifeTotal(PlayerRelation::EventPlayer),
                    RoundingDef::Up,
                )),
            },
        ),
    ]),
);

// RTR 167 — Hellhole Flailer
pub(in crate::card::sets) static HELLHOLE_FLAILER: CardRecord = CardRecord::new_with_legacy_id(
    1623,
    "Hellhole Flailer",
    CardArt::new("4984a089-84af-4387-9a0d-819b119b5565", "Steve Prescott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Ogre", "Warrior"], 3, 2).with_abilities(&[
        abilities::unleash(),
        abilities::unleash_counter(),
        AbilityDef::activated_with_targets(
            "{2}{B}{R}, Sacrifice this creature: It deals damage equal to its power to target \
             player or planeswalker.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{B}{R}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
        ),
    ]),
);

// RTR 168 — Heroes' Reunion
pub(in crate::card::sets) static HEROES_REUNION: CardRecord = CardRecord::new_with_legacy_id(
    1318,
    "Heroes' Reunion",
    CardArt::new("99b56515-f688-495c-b721-2b9abc6628c2", "Howard Lyon"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 7 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(7),
        },
    )),
);

// RTR 169 — Hussar Patrol
pub(in crate::card::sets) static HUSSAR_PATROL: CardRecord = CardRecord::new_with_legacy_id(
    1319,
    "Hussar Patrol",
    CardArt::new("dd775231-e1e0-41e2-ad9a-0726624f57f9", "Seb McKinnon"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Human", "Knight"], 2, 4)
        .with_abilities(&[abilities::flash(), abilities::vigilance()]),
);

// RTR 170 — Hypersonic Dragon
pub(in crate::card::sets) static HYPERSONIC_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f024b19a-923a-4313-be06-e743d3fbab46"),
    "Hypersonic Dragon",
    crate::card::CardArt::new("f024b19a-923a-4313-be06-e743d3fbab46", "Dan Murayama Scott"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::static_ability(
            "You may cast sorcery spells as though they had flash.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::HasType(CardType::Sorcery)),
                )),
            },
        ),
    ]),
);

// RTR 171 — Isperia, Supreme Judge
pub(in crate::card::sets) static ISPERIA_SUPREME_JUDGE: CardRecord = CardRecord::new_with_legacy_id(
    1320,
    "Isperia, Supreme Judge",
    CardArt::new("b2cce2d4-3944-4ff0-98e8-80f19697f108", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{W}{U}{U}"), &["Sphinx"], 6, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever a creature attacks you or a planeswalker you control, you may draw a card.",
                TriggerEventDef::attacks(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::Opponent,
                )),
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

// RTR 172 — Izzet Charm
pub(in crate::card::sets) static IZZET_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    178,
    "Izzet Charm",
    CardArt::new("1e3a5af6-5423-442b-a207-364e97a871d8", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Counter target noncreature spell unless its controller pays {2}.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::NoncreatureSpell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                abilities::counter_target_unless_paid(ValueDef::Constant(2)),
            ),
            AbilityDef::spell_with_targets(
                "Izzet Charm deals 2 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::spell(
                "Draw two cards, then discard two cards.",
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
        ],
    )),
);

// RTR 173 — Izzet Staticaster
pub(in crate::card::sets) static IZZET_STATICASTER: CardRecord = CardRecord::new_with_legacy_id(
    179,
    "Izzet Staticaster",
    CardArt::new("190ac2fe-532d-4d7e-9d74-07ae6850aac8", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{1}{U}{R}"),
        &["Human", "Wizard"],
        0,
        3,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::haste(),
        AbilityDef::activated_with_targets("{T}: This creature deals 1 damage to target creature and each other creature with the same name as that creature.", &[AbilityCostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )], // The target and every other creature sharing its name are one
            // set, so the two printed halves are a single sweep.
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            }),
    ]),
);

// RTR 174 — Jarad, Golgari Lich Lord
// Audit: unsupported — Needs a dynamic creature-card graveyard bonus and sacrifice costs whose chosen object's power and land subtypes drive linked effects.
pub(in crate::card::sets) static JARAD_GOLGARI_LICH_LORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6a2f10c-721c-40c7-8fe1-b4877a40fe96"),
    "Jarad, Golgari Lich Lord",
    crate::card::CardArt::new("02ef18d1-fd05-4dbc-9fa7-a383799b34e9", "Eric Deschamps"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 175 — Jarad's Orders
const JARAD_ORDERS_GRAVEYARD: Binding = Binding!("jarad_orders_graveyard");

pub(in crate::card::sets) static JARAD_S_ORDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c59171ce-7dc6-4dd9-a124-3c2c3028d93d"),
    "Jarad's Orders",
    crate::card::CardArt::new("c59171ce-7dc6-4dd9-a124-3c2c3028d93d", "Svetlin Velinov"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{2}{B}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to two creature cards and reveal them. Put one into your hand and the other into your graveyard. Then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: ValueDef::Constant(2),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: false,
            enters_tapped: false,
            attachment: None,
            binding: Some(ParentBinding),
            then: Some(&EffectDef::IfNoObjects(IfNoObjectsDef {
                input: ObjectSetDef::Binding(ParentBinding),
                if_empty: &EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Controller,
                },
                otherwise: &EffectDef::ChooseCardsFromCollection(
                    ChooseCardsFromCollectionDef {
                        source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Binding(
                            ParentBinding,
                        )),
                        actor: PlayerRefDef::EffectController,
                        inspection: CollectionInspectionDef::Reveal,
                        object: ObjectPredicateDef::Any,
                        minimum: 1,
                        maximum: 1,
                        chosen: ParentBinding,
                        remainder: JARAD_ORDERS_GRAVEYARD,
                        then: &EffectDef::Sequence(&[
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(JARAD_ORDERS_GRAVEYARD),
                                from: Some(ZoneKind::Hand),
                                zone: ZoneKind::Graveyard,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                            EffectDef::ShuffleLibrary {
                                player: EffectRecipientDef::Controller,
                            },
                        ]),
                    },
                ),
            })),
        },
    )),
);

// RTR 176 — Korozda Guildmage
pub(in crate::card::sets) static KOROZDA_GUILDMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1977,
    "Korozda Guildmage",
    CardArt::new("761c16aa-d4a7-492d-9275-98d0e07de45a", "Ryan Pancoast"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Elf", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}{B}{G}: Target creature gets +1/+1 and gains intimidate until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::intimidate()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{2}{B}{G}, Sacrifice a nontoken creature: Create X 1/1 green Saproling creature tokens, where X is the sacrificed creature's toughness.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{B}{G}"))],
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Controller,
                // Nontoken, so the Saprolings it makes cannot be fed back in.
                // Nontoken, so the Saprolings it makes cannot be fed back in.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                // One Saproling per point of toughness the sacrifice had.
                then: Some(&EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1)
                        .with_art(CardArt::new(
                            "e6544989-91b4-4db7-ad44-f1355f1d6e6b",
                            "Raoul Vitale",
                        ))
                        .with_count(ValueDef::TriggerEventAmount)),
                amount: SacrificedAmountDef::Toughness,
                otherwise: None,
                optional: false,
            },
        ),
    ]),
);

// RTR 177 — Lotleth Troll
pub(in crate::card::sets) static LOTLETH_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b628197-f26c-457a-b9a4-c1f1d3e02f3d"),
    "Lotleth Troll",
    crate::card::CardArt::new("3b628197-f26c-457a-b9a4-c1f1d3e02f3d", "Vincent Proce"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Zombie", "Troll"], 2, 1).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "Discard a creature card: Put a +1/+1 counter on this creature.",
            &[AbilityCostDef::DiscardCardMatching(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// RTR 178 — Loxodon Smiter
pub(in crate::card::sets) static LOXODON_SMITER: CardRecord = CardRecord::new_with_legacy_id(
    185,
    "Loxodon Smiter",
    CardArt::new("69247168-2bfb-4cce-a2a6-61459a0fbce4", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{1}{G}{W}"),
        &["Elephant", "Soldier"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::replacement_for(
            "If a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield instead of putting it into your graveyard.",
            ReplacementEventDef::WouldMove {
                from: Some(ZoneKind::Hand),
                to: ZoneKind::Graveyard,
                cause: ZoneMoveCauseDef::EffectControlledBy(PlayerRelation::Opponent),
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Battlefield),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// RTR 179 — Lyev Skyknight
pub(in crate::card::sets) static LYEV_SKYKNIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1535,
    "Lyev Skyknight",
    CardArt::new("11cbeb3b-1579-4318-a024-4a2c06896eaf", "Johannes Voss"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Human", "Knight"], 3, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, detain target nonland permanent an opponent controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 180 — Mercurial Chemister
const MERCURIAL_CHEMISTER_DISCARDED: Binding = Binding!("mercurial_chemister_discarded");

pub(in crate::card::sets) static MERCURIAL_CHEMISTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("881728ce-4b18-410e-9cdb-4d439ce0b21d"),
    "Mercurial Chemister",
    crate::card::CardArt::new("881728ce-4b18-410e-9cdb-4d439ce0b21d", "Wesley Burt"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{U}{R}"), &["Human", "Wizard"], 2, 3).with_abilities(
        &[
            AbilityDef::activated(
                "{U}, {T}: Draw two cards.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{U}")),
                    AbilityCostDef::TapSource,
                ],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::activated_with_targets(
                "{R}, {T}, Discard a card: This creature deals damage to any target equal to that card's mana value.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{R}")),
                    AbilityCostDef::TapSource,
                    AbilityCostDef::MoveToZone(
                        crate::card::MoveToZoneCostDef::new(
                            ObjectPredicateDef::Any,
                            ZoneKind::Hand,
                            ZoneKind::Graveyard,
                            1,
                        )
                        .binding(MERCURIAL_CHEMISTER_DISCARDED),
                    ),
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ObjectManaValue(ObjectRefDef::Binding(
                        MERCURIAL_CHEMISTER_DISCARDED,
                    )),
                },
            ),
        ],
    ),
);

// RTR 181 — New Prahv Guildmage
pub(in crate::card::sets) static NEW_PRAHV_GUILDMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1544,
    "New Prahv Guildmage",
    CardArt::new("698b47d1-c72e-4dc3-b28b-7421e0163f22", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{W}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{W}{U}: Target creature gains flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}{W}{U}: Detain target nonland permanent an opponent controls.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}{U}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Detain {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// RTR 182 — Nivix Guildmage
pub(in crate::card::sets) static NIVIX_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d3fde47-8d9e-4a84-b8a5-dcfe0c1d443c"),
    "Nivix Guildmage",
    crate::card::CardArt::new("9d3fde47-8d9e-4a84-b8a5-dcfe0c1d443c", "Scott M. Fischer"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{U}{R}: Draw a card, then discard a card.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}{R}"))],
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
        AbilityDef::activated_with_targets(
            "{2}{U}{R}: Copy target instant or sorcery spell you control. You may choose new targets for the copy.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{U}{R}"))],
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
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        ),
    ]),
);

// RTR 183 — Niv-Mizzet, Dracogenius
pub(in crate::card::sets) static NIV_MIZZET_DRACOGENIUS: CardRecord =
    CardRecord::new_with_legacy_id(
        1321,
        "Niv-Mizzet, Dracogenius",
        CardArt::new("c345e475-8095-41b5-90b4-771fcf80b939", "Todd Lockwood"),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost!("{2}{U}{U}{R}{R}"), &["Dragon", "Wizard"], 5, 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::flying(),
                AbilityDef::triggered(
                    "Whenever Niv-Mizzet deals damage to a player, you may draw a card.",
                    TriggerEventDef::damage_to_player(
                        ObjectPredicateDef::Source,
                        PlayerRelation::Any,
                    ),
                    EffectDef::May {
                        player: EffectRecipientDef::Controller,
                        effect: &EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    },
                ),
                AbilityDef::activated_with_targets(
                    "{U}{R}: Niv-Mizzet deals 1 damage to any target.",
                    &[AbilityCostDef::Mana(mana_cost!("{U}{R}"))],
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

// RTR 184 — Rakdos Charm
pub(in crate::card::sets) static RAKDOS_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fcd4394-d22d-4eec-ad73-ffaf10ad60de"),
    "Rakdos Charm",
    crate::card::CardArt::new("0fcd4394-d22d-4eec-ad73-ffaf10ad60de", "Zoltan Boros"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Exile target player's graveyard.\n• Destroy target artifact.\n• Each creature deals 1 damage to its controller.",
        &[
            AbilityDef::spell_with_targets(
                "Exile target player's graveyard.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::cards_owned_by_target(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        TargetIndex::PRIMARY,
                    ),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::spell(
                "Each creature deals 1 damage to its controller.",
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                        ObjectQueryDef::new(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                        ),
                    )),
                    binding: ParentBinding,
                    then: &EffectDef::ForEachInBinding {
                        objects: ParentBinding,
                        binding: ParentBinding,
                        effect: &EffectDef::DealDamageFrom {
                            source: ObjectRefDef::Binding(ParentBinding),
                            recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                                ObjectRefDef::Binding(ParentBinding),
                            )),
                            amount: ValueDef::Constant(1),
                        },
                    },
                }),
            ),
        ],
    )),
);

// RTR 185 — Rakdos Ragemutt
pub(in crate::card::sets) static RAKDOS_RAGEMUTT: CardRecord = CardRecord::new_with_legacy_id(
    1322,
    "Rakdos Ragemutt",
    CardArt::new("bb36840a-3f85-4fca-87ab-379dfce8e542", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}{R}"), &["Elemental", "Dog"], 3, 3)
        .with_abilities(&[abilities::lifelink(), abilities::haste()]),
);

// RTR 186 — Rakdos Ringleader
pub(in crate::card::sets) static RAKDOS_RINGLEADER: CardRecord = CardRecord::new_with_legacy_id(
    1492,
    "Rakdos Ringleader",
    CardArt::new("6b54fbe8-324a-4066-bed1-dda1dca319fc", "Jason Felix"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{4}{B}{R}"),
        &["Skeleton", "Warrior"],
        3,
        1,
    )
    .with_abilities(&[
        abilities::first_strike(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards a card at random.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        ),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// RTR 187 — Rakdos, Lord of Riots
// Audit: unsupported — Needs a life-lost-this-turn cast restriction and a global creature-spell cost reduction derived from opponents' life loss.
pub(in crate::card::sets) static RAKDOS_LORD_OF_RIOTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04f3db71-802f-488c-b40d-ac90df2d660a"),
    "Rakdos, Lord of Riots",
    crate::card::CardArt::new("04f3db71-802f-488c-b40d-ac90df2d660a", "Vincent Proce"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 188 — Rakdos's Return
pub(in crate::card::sets) static RAKDOS_S_RETURN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d72981c0-1632-4d64-9341-2a76047d9b36"),
    "Rakdos's Return",
    crate::card::CardArt::new("d72981c0-1632-4d64-9341-2a76047d9b36", "Daarken"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{X}{B}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "This spell deals X damage to target opponent or planeswalker. That player or that planeswalker's controller discards X cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    amount: ValueDef::ChosenX,
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        ),
    ),
);

// RTR 189 — Righteous Authority
/// "Its controller's hand", not the Aura controller's, so gifting the
/// creature away moves the bonus and the extra draw with it.
static CARDS_IN_THE_ENCHANTED_CONTROLLERS_HAND: ValueDef = ValueDef::CardsInHandAbove {
    player: PlayerRelation::ControllerOfAttachedPermanent,
    threshold: 0,
};

pub(in crate::card::sets) static RIGHTEOUS_AUTHORITY: CardRecord = CardRecord::new_with_legacy_id(
    1969,
    "Righteous Authority",
    CardArt::new("6695e5bb-56a9-49ab-8940-72336e845875", "Scott Chou"),
    CardSet::ReturnToRavnica,
    // The extra draw feeds the bonus: one more card in hand is one more
    // power on the creature holding it.
    CardRules::new_enchantment(mana_cost!("{3}{W}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 for each card in its controller's hand.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        CARDS_IN_THE_ENCHANTED_CONTROLLERS_HAND,
                        CARDS_IN_THE_ENCHANTED_CONTROLLERS_HAND,
                    ),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of the draw step of enchanted creature's controller, that \
                 player draws an additional card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Draw,
                    player: PlayerRelation::ControllerOfAttachedPermanent,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// RTR 190 — Risen Sanctuary
pub(in crate::card::sets) static RISEN_SANCTUARY: CardRecord = keyword_creature(
    1323,
    "Risen Sanctuary",
    "a0b6c136-2bbe-48c1-ac53-2a8221b96936",
    "Chase Stone",
    mana_cost!("{5}{G}{W}"),
    &["Elemental"],
    8,
    8,
    abilities::vigilance(),
);

// RTR 191 — Rites of Reaping
pub(in crate::card::sets) static RITES_OF_REAPING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("115d504f-3aec-4374-8cd8-732d56c448f2"),
    "Rites of Reaping",
    crate::card::CardArt::new("115d504f-3aec-4374-8cd8-732d56c448f2", "David Rapoza"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{4}{B}{G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +3/+3 until end of turn. Another target creature gets -3/-3 until end of turn.",
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))
                .another(),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex(1)),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-3),
                        ValueDef::Constant(-3),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// RTR 192 — Rix Maadi Guildmage
// Audit: unsupported — Its second activation needs a target-player predicate for a player who lost life this turn.
pub(in crate::card::sets) static RIX_MAADI_GUILDMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb737465-5de8-4015-befe-2bf386da2a89"),
    "Rix Maadi Guildmage",
    crate::card::CardArt::new("eb737465-5de8-4015-befe-2bf386da2a89", "Karl Kopinski"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 193 — Search Warrant
pub(in crate::card::sets) static SEARCH_WARRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d55f625a-6e9e-40ba-ae46-cf6bafc0a41b"),
    "Search Warrant",
    CardArt::new("d55f625a-6e9e-40ba-ae46-cf6bafc0a41b", "Steven Belledin"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{W}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player reveals their hand. You gain life equal to the number of cards in that player's hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::RevealHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountObjects(&ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                ))),
            },
        ]),
    )),
);

// RTR 194 — Selesnya Charm
pub(in crate::card::sets) static SELESNYA_CHARM: CardRecord = CardRecord::new_with_legacy_id(
    209,
    "Selesnya Charm",
    CardArt::new("a9848eab-1d3a-4ab0-adf6-c20858aa3afb", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target creature gets +2/+2 and gains trample until end of turn.",
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
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Exile target creature with power 5 or greater.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(5),
                    ]),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            ),
            AbilityDef::spell(
                "Create a 2/2 white Knight creature token with vigilance.",
                EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 2, 2)
                    .with_abilities(&[abilities::vigilance()])
                    .with_art(CardArt::new(
                        "67d3d039-248a-4eb8-be5c-12959b458fea",
                        "Matt Stewart",
                    )),
            ),
        ],
    )),
);

// RTR 195 — Skull Rend
pub(in crate::card::sets) static SKULL_REND: CardRecord = CardRecord::new_with_legacy_id(
    1324,
    "Skull Rend",
    CardArt::new("1c8efb23-bac0-41d2-b4ee-27a6b1fe3134", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{3}{B}{R}")).with_ability(AbilityDef::spell(
        "Skull Rend deals 2 damage to each opponent. Those players each discard two cards at random.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::Random,
                then: None,
            },
        ]),
    )),
);

// RTR 196 — Skymark Roc
pub(in crate::card::sets) static SKYMARK_ROC: CardRecord = CardRecord::new_with_legacy_id(
    1325,
    "Skymark Roc",
    CardArt::new("60601296-2229-4c48-94cc-1903926750ce", "Christopher Moeller"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Bird"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may return target creature defending player controls with toughness 2 or less to its owner's hand.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ToughnessLessThan(ValueDef::Constant(3)),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
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
        ),
    ]),
);

// RTR 197 — Slaughter Games
// Audit: unsupported — Needs a nonland card-name choice and a name-linked search across an opponent's graveyard, hand, and library.
pub(in crate::card::sets) static SLAUGHTER_GAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf37391d-db35-40a7-908a-abb53895793c"),
    "Slaughter Games",
    crate::card::CardArt::new("bf37391d-db35-40a7-908a-abb53895793c", "Steve Prescott"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 198 — Sluiceway Scorpion
pub(in crate::card::sets) static SLUICEWAY_SCORPION: CardRecord = CardRecord::new_with_legacy_id(
    1551,
    "Sluiceway Scorpion",
    CardArt::new("7b6dbadf-a6f7-4876-9c3f-44e4a33b2bee", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{2}{B}{G}"), &["Scorpion"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::scavenge(
            mana_cost!("{1}{B}{G}"),
            "Scavenge {1}{B}{G} ({1}{B}{G}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 199 — Spawn of Rix Maadi
pub(in crate::card::sets) static SPAWN_OF_RIX_MAADI: CardRecord = CardRecord::new_with_legacy_id(
    1527,
    "Spawn of Rix Maadi",
    CardArt::new("6196e702-7f76-49db-8ee4-bd343359d498", "Min Yum"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{B}{R}"), &["Horror"], 5, 3)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

// RTR 200 — Sphinx's Revelation
pub(in crate::card::sets) static SPHINXS_REVELATION: CardRecord = CardRecord::new_with_legacy_id(
    216,
    "Sphinx's Revelation",
    CardArt::new("404d9413-ef57-4b6e-8584-48a1dc7fe6f1", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{X}{W}{U}{U}")).with_ability(AbilityDef::spell(
        "You gain X life and draw X cards.",
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::EffectController),
                amount: ValueDef::ChosenX,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::player(PlayerRefDef::EffectController),
                amount: ValueDef::ChosenX,
            },
        ]),
    )),
);

// RTR 201 — Supreme Verdict
pub(in crate::card::sets) static SUPREME_VERDICT: CardRecord = CardRecord::new_with_legacy_id(
    222,
    "Supreme Verdict",
    CardArt::new("4e9648f9-7a67-4717-bca1-861d1f7fed43", "Sam Burley"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}{U}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell(
            "Destroy all creatures.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// RTR 202 — Teleportal
pub(in crate::card::sets) static TELEPORTAL: CardRecord = CardRecord::new_with_legacy_id(
    1326,
    "Teleportal",
    CardArt::new("e438c718-ac18-45d5-824e-5b697c5f0692", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{U}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature you control gets +1/+0 until end of turn and can't be blocked this turn.",
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
                    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Any,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::overload(
            mana_cost!("{3}{U}{R}"),
            "Each creature you control gets +1/+0 until end of turn and can't be blocked this turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Any,
                    )),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 203 — Thoughtflare
pub(in crate::card::sets) static THOUGHTFLARE: CardRecord = CardRecord::new_with_legacy_id(
    1327,
    "Thoughtflare",
    CardArt::new("d90514aa-e356-4502-9e0e-76ab7644a07a", "David Rapoza"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{3}{U}{R}")).with_ability(AbilityDef::spell(
        "Draw four cards, then discard two cards.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// RTR 204 — Treasured Find
pub(in crate::card::sets) static TREASURED_FIND: CardRecord = CardRecord::new_with_legacy_id(
    1328,
    "Treasured Find",
    CardArt::new("a2c0e00b-2290-493f-a3fc-3b9bff2830cc", "Jason Chan"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{B}{G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target card from your graveyard to your hand. Exile Treasured Find.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
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
        )
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ),
);

// RTR 205 — Trestle Troll
pub(in crate::card::sets) static TRESTLE_TROLL: CardRecord = CardRecord::new_with_legacy_id(
    1493,
    "Trestle Troll",
    CardArt::new("6d224279-83f3-4a29-9fd9-86b72407b87a", "Peter Mohrbacher"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{B}{G}"), &["Troll"], 1, 4).with_abilities(&[
        abilities::defender(),
        abilities::reach(),
        abilities::regenerate_self(
            "{1}{B}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}{G}"))],
        ),
    ]),
);

// RTR 206 — Trostani, Selesnya's Voice
pub(in crate::card::sets) static TROSTANI_SELESNYAS_VOICE: CardRecord =
    CardRecord::new_with_legacy_id(
        1857,
        "Trostani, Selesnya's Voice",
        CardArt::new("9d1d9d86-5666-4e59-9766-137657b4e040", "Chippy"),
        CardSet::ReturnToRavnica,
        CardRules::new_creature(mana_cost!("{G}{G}{W}{W}"), &["Dryad"], 2, 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                // The toughness read is the entering creature's, so a token
                // copied by the ability below feeds this one on the way in.
                AbilityDef::triggered(
                    "Whenever another creature you control enters, you gain life equal to that \
                 creature's toughness.",
                    TriggerEventDef::zone_changed(
                        // "Another creature you control", which is three conditions rather than one:
                        // a creature, yours, and not Trostani herself.
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::TriggeringObjectToughness,
                    },
                ),
                AbilityDef::activated(
                    "{1}{G}{W}, {T}: Populate.",
                    &[
                        AbilityCostDef::Mana(mana_cost!("{1}{G}{W}")),
                        AbilityCostDef::TapSource,
                    ],
                    abilities::populate(),
                ),
            ]),
    );

// RTR 207 — Vitu-Ghazi Guildmage
pub(in crate::card::sets) static VITU_GHAZI_GUILDMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1858,
    "Vitu-Ghazi Guildmage",
    CardArt::new("e54f8e61-550f-4493-b8ba-65f81b2457d3", "Jason Chan"),
    CardSet::ReturnToRavnica,
    // Two abilities rather than one modal ability: each has its own cost, so
    // there is nothing to choose between at activation time.
    CardRules::new_creature(mana_cost!("{G}{W}"), &["Dryad", "Shaman"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{4}{G}{W}: Create a 3/3 green Centaur creature token.",
            &[AbilityCostDef::Mana(mana_cost!("{4}{G}{W}"))],
            EffectDef::create_creature_token(&["Centaur"], &[ManaColor::Green], 3, 3).with_art(
                CardArt::new("880d5dc1-ceec-4c5f-93c2-c88b7dbfcac2", "Slawomir Maniak"),
            ),
        ),
        AbilityDef::activated(
            "{2}{G}{W}: Populate.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}{W}"))],
            abilities::populate(),
        ),
    ]),
);

// RTR 208 — Vraska the Unseen
pub(in crate::card::sets) static VRASKA_THE_UNSEEN: CardRecord = CardRecord::new_with_legacy_id(
    240,
    "Vraska the Unseen",
    CardArt::new("8971938c-cd26-4b83-96d7-1408cd0b0de6", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
    CardRules::new_planeswalker(mana_cost!("{3}{B}{G}"), &["Vraska"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Until your next turn, whenever a creature deals combat damage to Vraska, destroy that creature.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    // The delayed trigger Vraska's +1 hangs on herself. It reads damage arriving
                    // at the planeswalker, which only became reachable once a creature could
                    // attack one.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::triggered(
                        "Whenever a creature deals combat damage to Vraska, destroy that creature.",
                        TriggerEventDef::combat_damage_to_source(ObjectPredicateDef::HasType(CardType::Creature)),
                        EffectDef::Destroy {
                            object: EffectRecipientDef::TriggeringObject,
                            can_regenerate: true,
                            then: None,
                        },
                    )),
                    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                },
            ),
            AbilityDef::activated_with_targets(
                "−3: Destroy target nonland permanent.",
                &[AbilityCostDef::Loyalty(-3)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "−7: Create three 1/1 black Assassin creature tokens with \"Whenever this token deals combat damage to a player, that player loses the game.\"",
                &[AbilityCostDef::Loyalty(-7)],
                EffectDef::create_creature_token(&["Assassin"], &[ManaColor::Black], 1, 1)
                    .with_abilities(&[AbilityDef::triggered(
                        "Whenever this token deals combat damage to a player, that player loses the game.",
                        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                        EffectDef::LoseTheGame {
                            player: EffectRecipientDef::EventPlayer,
                        },
                    )])
                    .with_art(CardArt::new(
                        "89eb9f92-d189-4438-b6fe-cb253055d63e",
                        "Svetlin Velinov",
                    ))
                    .with_amount(3),
            ),
        ]),
);

// RTR 209 — Wayfaring Temple
static WAYFARING_TEMPLE_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static WAYFARING_TEMPLE: CardRecord = CardRecord::new_with_legacy_id(
    1855,
    "Wayfaring Temple",
    CardArt::new("2125e6aa-f916-4dfa-a9fe-82bb546012af", "Peter Mohrbacher"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{1}{G}{W}"), &["Elemental"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "Wayfaring Temple's power and toughness are each equal to the number of creatures \
             you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // Counted live and including itself, so a lone Temple is a 1/1.
                effect: AppliedEffectDef::set_base_power_toughness(
                    ValueDef::CountMatchingObjects(&WAYFARING_TEMPLE_CREATURES),
                    ValueDef::CountMatchingObjects(&WAYFARING_TEMPLE_CREATURES),
                ),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, populate.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            abilities::populate(),
        ),
    ]),
);

// RTR 210 — Azor's Elocutors
const FILIBUSTER_COUNTER: CounterKind = CounterKind::named("filibuster");

pub(in crate::card::sets) static AZOR_S_ELOCUTORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61e82934-546b-4734-a715-b22ace4c5a9b"),
    "Azor's Elocutors",
    crate::card::CardArt::new("61e82934-546b-4734-a715-b22ace4c5a9b", "Johannes Voss"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{3}{W/U}{W/U}"), &["Human", "Advisor"], 3, 5)
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, put a filibuster counter on this creature. Then if this creature has five or more filibuster counters on it, you win the game.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: FILIBUSTER_COUNTER,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SourceCounters {
                            kind: FILIBUSTER_COUNTER,
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 5,
                        },
                        then: &EffectDef::WinTheGame {
                            player: EffectRecipientDef::Controller,
                        },
                    },
                ]),
            ),
            AbilityDef::triggered(
                "Whenever a source deals damage to you, remove a filibuster counter from this creature.",
                TriggerEventDef::damage_to_player(ObjectPredicateDef::Any, PlayerRelation::You),
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: FILIBUSTER_COUNTER,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// RTR 211 — Blistercoil Weird
pub(in crate::card::sets) static BLISTERCOIL_WEIRD: CardRecord = CardRecord::new_with_legacy_id(
    1329,
    "Blistercoil Weird",
    CardArt::new("d2a8e716-ea33-4ae2-9ff8-5e78b0e50459", "Dan Murayama Scott"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U/R}"), &["Weird"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, this creature gets +1/+1 until end of turn. Untap it.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
    ),
);

// RTR 212 — Cryptborn Horror
// Audit: unsupported — Needs the total life lost by all opponents this turn as a dynamic enters-with-counter replacement value.
pub(in crate::card::sets) static CRYPTBORN_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ffd77d7-3fe6-4493-96eb-f62183c0358d"),
    "Cryptborn Horror",
    crate::card::CardArt::new("8ffd77d7-3fe6-4493-96eb-f62183c0358d", "Richard Wright"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 213 — Deathrite Shaman
pub(in crate::card::sets) static DEATHRITE_SHAMAN: CardRecord = CardRecord::new_with_legacy_id(
    1330,
    "Deathrite Shaman",
    CardArt::new("70496f16-c4c0-4c03-beef-454eb4824cd1", "Steve Argyle"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B/G}"), &["Elf", "Shaman"], 1, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Exile target land card from a graveyard. Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            // "Add one mana of any color", named as the ability resolves. It is not a
            // mana ability at all: it targets, and a targeted ability uses the stack
            // however much mana it makes (CR 605.1a).
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                EffectDef::AddMana(AddManaEffectDef::choice(&ManaColor::COLORS)),
            ]),
        ),
        AbilityDef::activated_with_targets(
            "{B}, {T}: Exile target instant or sorcery card from a graveyard. Each opponent loses 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
},
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
        AbilityDef::activated_with_targets(
            "{G}, {T}: Exile target creature card from a graveyard. You gain 2 life.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
},
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// RTR 214 — Dryad Militant
// Audit: unsupported — The graveyard replacement event cannot filter the moving object to instant or sorcery cards.
pub(in crate::card::sets) static DRYAD_MILITANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bb8cb8c-0d03-4cbf-b7f2-a97324817698"),
    "Dryad Militant",
    crate::card::CardArt::new("2bb8cb8c-0d03-4cbf-b7f2-a97324817698", "Terese Nielsen"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 215 — Frostburn Weird
pub(in crate::card::sets) static FROSTBURN_WEIRD: CardRecord = CardRecord::new_with_legacy_id(
    1331,
    "Frostburn Weird",
    CardArt::new("ba5a68d3-6bc9-4de8-bc06-e1106cf9b3d4", "Mike Bierek"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{U/R}{U/R}"), &["Weird"], 1, 4).with_ability(
        AbilityDef::activated(
            "{U/R}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U/R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// RTR 216 — Golgari Longlegs
pub(in crate::card::sets) static GOLGARI_LONGLEGS: CardRecord = vanilla_creature(
    1332,
    "Golgari Longlegs",
    "d44058ba-3419-4777-8d59-05dea5e864e1",
    "Volkan Baǵa",
    mana_cost!("{3}{B/G}{B/G}"),
    &["Insect"],
    5,
    4,
);

// RTR 217 — Growing Ranks
pub(in crate::card::sets) static GROWING_RANKS: CardRecord = CardRecord::new_with_legacy_id(
    1617,
    "Growing Ranks",
    CardArt::new("12f31616-1249-4964-b81a-4435405a2449", "Seb McKinnon"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{2}{G/W}{G/W}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, populate.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        abilities::populate(),
    )),
);

// RTR 218 — Judge's Familiar
pub(in crate::card::sets) static JUDGES_FAMILIAR: CardRecord = CardRecord::new_with_legacy_id(
    1333,
    "Judge's Familiar",
    CardArt::new("0fc51899-3970-416b-b7de-fadbc9678955", "Jack Wang"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{W/U}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Counter target instant or sorcery spell unless its controller pays {1}.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
ObjectPredicateDef::Spell,
ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ])
]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
        ),
    ]),
);

// RTR 219 — Nivmagus Elemental
// Audit: unsupported — Needs exiling a chosen instant or sorcery spell you control from the stack as an activation cost.
pub(in crate::card::sets) static NIVMAGUS_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1892003-2e4c-43bd-8a37-3a97a76f113a"),
    "Nivmagus Elemental",
    crate::card::CardArt::new("b1892003-2e4c-43bd-8a37-3a97a76f113a", "Mike Bierek"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 220 — Rakdos Cackler
pub(in crate::card::sets) static RAKDOS_CACKLER: CardRecord = CardRecord::new_with_legacy_id(
    1528,
    "Rakdos Cackler",
    CardArt::new("5f873c0b-e779-4f09-8e9c-94a1765eb5da", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B/R}"), &["Devil"], 1, 1)
        .with_abilities(&[abilities::unleash(), abilities::unleash_counter()]),
);

// RTR 221 — Rakdos Shred-Freak
pub(in crate::card::sets) static RAKDOS_SHRED_FREAK: CardRecord = keyword_creature(
    1334,
    "Rakdos Shred-Freak",
    "06899549-5534-4d11-86c1-afd1796e18b1",
    "Wayne Reynolds",
    mana_cost!("{B/R}{B/R}"),
    &["Human", "Berserker"],
    2,
    1,
    abilities::haste(),
);

// RTR 222 — Slitherhead
pub(in crate::card::sets) static SLITHERHEAD: CardRecord = CardRecord::new_with_legacy_id(
    1552,
    "Slitherhead",
    CardArt::new("d9327905-a254-4885-8310-69fc153ec52f", "Greg Staples"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(mana_cost!("{B/G}"), &["Plant", "Zombie"], 1, 1).with_abilities(&[
        abilities::scavenge(
            mana_cost!("{0}"),
            "Scavenge {0} ({0}, Exile this card from your graveyard: Put a number of +1/+1 counters equal to this card's power on target creature. Scavenge only as a sorcery.)",
        ),
    ]),
);

// RTR 223 — Sundering Growth
pub(in crate::card::sets) static SUNDERING_GROWTH: CardRecord = CardRecord::new_with_legacy_id(
    1856,
    "Sundering Growth",
    CardArt::new("14d5048e-cb76-48c4-8a95-70dcc14775f6", "David Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G/W}{G/W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment, then populate.",
        // "Artifact or enchantment", which is one target slot rather than two.
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        // The destruction comes first, so a token the destroyed permanent was
        // keeping alive is already gone when the copy is chosen.
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            abilities::populate(),
        ]),
    )),
);

// RTR 224 — Vassal Soul
pub(in crate::card::sets) static VASSAL_SOUL: CardRecord = keyword_creature(
    1335,
    "Vassal Soul",
    "dfc61748-029f-4bae-a7ec-e08b7059226d",
    "Dan Murayama Scott",
    mana_cost!("{1}{W/U}{W/U}"),
    &["Spirit"],
    2,
    2,
    abilities::flying(),
);

// RTR 225 — Azorius Keyrune
pub(in crate::card::sets) static AZORIUS_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1336,
    "Azorius Keyrune",
    CardArt::new("23a05db7-dcae-4180-8ad1-60ba6fb30816", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{W}{U}: This artifact becomes a 2/2 white and blue Bird artifact creature with flying until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&keyrune_animation(
                        2,
                        2,
                        &["Bird"],
                        ColorSet::from_colors(&[ManaColor::White, ManaColor::Blue]),
                    )),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 226 — Chromatic Lantern
static CHROMATIC_LANTERN_MANA: AbilityDef = AbilityDef::activated_mana(
    "{T}: Add one mana of any color.",
    &[AbilityCostDef::TapSource],
    EffectDef::AddMana(AddManaEffectDef::choice(&[
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ])),
);

pub(in crate::card::sets) static CHROMATIC_LANTERN: CardRecord = CardRecord::new_with_legacy_id(
    1337,
    "Chromatic Lantern",
    CardArt::new("57f4e0f0-13d1-43ed-8d95-13cff94a26e7", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::static_ability(
            "Lands you control have \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&CHROMATIC_LANTERN_MANA),
            },
        ),
        CHROMATIC_LANTERN_MANA,
    ]),
);

// RTR 227 — Civic Saber
pub(in crate::card::sets) static CIVIC_SABER: CardRecord = CardRecord::new_with_legacy_id(
    2311,
    "Civic Saber",
    CardArt::new("29c9247e-05f4-44bb-86e3-90a60e880374", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 for each of its colors.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::AffectedColorCount,
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// RTR 228 — Codex Shredder
pub(in crate::card::sets) static CODEX_SHREDDER: CardRecord = CardRecord::new_with_legacy_id(
    1338,
    "Codex Shredder",
    CardArt::new("8f7b632b-ee20-4082-8376-1dae53f91b70", "Jason Felix"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Target player mills a card.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{5}, {T}, Sacrifice this artifact: Return target card from your graveyard to your hand.",
            &[
                AbilityCostDef::Mana(mana_cost!("{5}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
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
    ]),
);

// RTR 229 — Golgari Keyrune
pub(in crate::card::sets) static GOLGARI_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1339,
    "Golgari Keyrune",
    CardArt::new("913b803f-ba82-4660-86be-49677d1e32c9", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
        AbilityDef::activated(
            "{B}{G}: This artifact becomes a 2/2 black and green Insect artifact creature with deathtouch until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&keyrune_animation(
                        2,
                        2,
                        &["Insect"],
                        ColorSet::from_colors(&[ManaColor::Black, ManaColor::Green]),
                    )),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 230 — Izzet Keyrune
pub(in crate::card::sets) static IZZET_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1340,
    "Izzet Keyrune",
    CardArt::new("83e4e83b-cbb8-4efd-b6c4-4459a29177ac", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Red,
            ])),
        ),
        AbilityDef::activated(
            "{U}{R}: Until end of turn, this artifact becomes a 2/1 blue and red Elemental artifact creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&keyrune_animation(
                    2,
                    1,
                    &["Elemental"],
                    ColorSet::from_colors(&[ManaColor::Blue, ManaColor::Red]),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::triggered(
            "Whenever this artifact deals combat damage to a player, you may draw a card. If you do, discard a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
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
        ),
    ]),
);

// RTR 231 — Pithing Needle
pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new_with_legacy_id(
    196,
    "Pithing Needle",
    CardArt::new("786c1e91-9d75-46a3-9e0d-56d29fcb01a7", "Anthony Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::choose_card_name_as_enters(
            "As this artifact enters, choose a card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::CARD_NAME,
        ),
        abilities::cannot_activate_nonmana_abilities_with_chosen_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
        ),
    ]),
);

// RTR 232 — Rakdos Keyrune
pub(in crate::card::sets) static RAKDOS_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1341,
    "Rakdos Keyrune",
    CardArt::new("f6124b4c-49a1-42e3-a6ff-62de231c7823", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
        AbilityDef::activated(
            "{B}{R}: This artifact becomes a 3/1 black and red Devil artifact creature with first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Composite(&keyrune_animation(
                        3,
                        1,
                        &["Devil"],
                        ColorSet::from_colors(&[ManaColor::Black, ManaColor::Red]),
                    )),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 233 — Selesnya Keyrune
pub(in crate::card::sets) static SELESNYA_KEYRUNE: CardRecord = CardRecord::new_with_legacy_id(
    1342,
    "Selesnya Keyrune",
    CardArt::new("1645eafd-cd17-463a-9d7e-42f5f7b5196f", "Daniel Ljunggren"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
        AbilityDef::activated(
            "{G}{W}: This artifact becomes a 3/3 green and white Wolf artifact creature until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&keyrune_animation(
                    3,
                    3,
                    &["Wolf"],
                    ColorSet::from_colors(&[ManaColor::Green, ManaColor::White]),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 234 — Street Sweeper
// Audit: unsupported — Needs selecting and destroying every Aura attached to the targeted land through an attachment-relationship predicate.
pub(in crate::card::sets) static STREET_SWEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a62827a-183f-4bef-b6ce-20a4577f6d30"),
    "Street Sweeper",
    crate::card::CardArt::new("4a62827a-183f-4bef-b6ce-20a4577f6d30", "Izzy"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 235 — Tablet of the Guilds
// Audit: unsupported — Needs choosing and storing two colors, matching cast spells against both, and counting how many chosen colors match.
pub(in crate::card::sets) static TABLET_OF_THE_GUILDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b006384-6fb6-4129-b1d2-7674d1141f8f"),
    "Tablet of the Guilds",
    crate::card::CardArt::new("6b006384-6fb6-4129-b1d2-7674d1141f8f", "Nic Klein"),
    crate::card::CardSet::ReturnToRavnica,
    crate::card::CardRules::unsupported(),
);

// RTR 236 — Volatile Rig
pub(in crate::card::sets) static VOLATILE_RIG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5055ed18-b234-4702-92eb-4d483431ff47"),
    "Volatile Rig",
    crate::card::CardArt::new("5055ed18-b234-4702-92eb-4d483431ff47", "Mathias Kollros"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 4, 4).with_abilities(&[
        abilities::trample(),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, flip a coin. If you lose the flip, sacrifice this creature.",
            TriggerEventDef::damage_to_source(),
            EffectDef::Randomized {
                likelihood: crate::card::LikelihoodDef::new(0.5),
                on_success: &EffectDef::None,
                on_failure: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
        abilities::dies_trigger(
            "When this creature dies, flip a coin. If you lose the flip, it deals 4 damage to each creature and each player.",
            EffectDef::Randomized {
                likelihood: crate::card::LikelihoodDef::new(0.5),
                on_success: &EffectDef::None,
                on_failure: &EffectDef::DealDamageSimultaneously(&[
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        ValueDef::Constant(4),
                    ),
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::EachPlayer,
                        ValueDef::Constant(4),
                    ),
                ]),
            },
        ),
    ]),
);

// RTR 237 — Azorius Guildgate
pub(in crate::card::sets) static AZORIUS_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1343,
    "Azorius Guildgate",
    CardArt::new("984e37df-0734-493a-a958-f519a0c98580", "Drew Baker"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// RTR 238 — Blood Crypt
pub(in crate::card::sets) static BLOOD_CRYPT: CardRecord = CardRecord::new_with_legacy_id(
    1344,
    "Blood Crypt",
    CardArt::new("8bd5828b-8dcd-4ce6-b834-ebe9cbaa12d1", "Vincent Proce"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Swamp", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// RTR 239 — Golgari Guildgate
pub(in crate::card::sets) static GOLGARI_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    172,
    "Golgari Guildgate",
    CardArt::new("8fe2fd1a-f7d3-48b4-bad8-be5ee45d6121", "Eytan Zana"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// RTR 240 — Grove of the Guardian
// Audit: metadata-only — The activation planner cannot yet reserve two chosen
// untapped creatures jointly with the ability's mana payment.
pub(in crate::card::sets) static GROVE_OF_THE_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cf60ca0-e01f-499c-8d04-d59050f38c33"),
    "Grove of the Guardian",
    crate::card::CardArt::new("3cf60ca0-e01f-499c-8d04-d59050f38c33", "Christine Choi"),
    crate::card::CardSet::ReturnToRavnica,
    CardRules::unsupported(),
);

// RTR 241 — Hallowed Fountain
pub(in crate::card::sets) static HALLOWED_FOUNTAIN: CardRecord = CardRecord::new_with_legacy_id(
    174,
    "Hallowed Fountain",
    CardArt::new("af7091c9-5f98-4078-a42b-c9e057346d9b", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Plains", "Island"]).with_ability(abilities::shock_land_enters()),
);

// RTR 242 — Izzet Guildgate
pub(in crate::card::sets) static IZZET_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1345,
    "Izzet Guildgate",
    CardArt::new("6951d84f-2d3c-4203-8d31-e08f4bc707f0", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// RTR 243 — Overgrown Tomb
pub(in crate::card::sets) static OVERGROWN_TOMB: CardRecord = CardRecord::new_with_legacy_id(
    194,
    "Overgrown Tomb",
    CardArt::new("1c7d50d6-b63a-4d8c-88fa-1d78ae693a45", "Steven Belledin"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Swamp", "Forest"]).with_ability(abilities::shock_land_enters()),
);

// RTR 244 — Rakdos Guildgate
pub(in crate::card::sets) static RAKDOS_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1346,
    "Rakdos Guildgate",
    CardArt::new("207048f5-268b-4cdb-b4e7-c8282cac1b28", "Eytan Zana"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// RTR 245 — Rogue's Passage
pub(in crate::card::sets) static ROGUES_PASSAGE: CardRecord = CardRecord::new_with_legacy_id(
    1347,
    "Rogue's Passage",
    CardArt::new("f416e36a-15cb-43c8-b27f-82f65a95ddef", "Christine Choi"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{4}, {T}: Target creature can't be blocked this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{4}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// RTR 246 — Selesnya Guildgate
pub(in crate::card::sets) static SELESNYA_GUILDGATE: CardRecord = CardRecord::new_with_legacy_id(
    1348,
    "Selesnya Guildgate",
    CardArt::new("ff61d4e4-3c8c-48f7-a994-ec2317bbd9a0", "Howard Lyon"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::White,
            ])),
        ),
    ]),
);

// RTR 247 — Steam Vents
pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new_with_legacy_id(
    217,
    "Steam Vents",
    CardArt::new("de911c88-f5c8-4955-9fa5-1f28a9b17236", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Island", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// RTR 248 — Temple Garden
pub(in crate::card::sets) static TEMPLE_GARDEN: CardRecord = CardRecord::new_with_legacy_id(
    224,
    "Temple Garden",
    CardArt::new("b821e604-f9fd-47a4-b5ff-bfb5022834c2", "Volkan Baǵa"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Forest", "Plains"]).with_ability(abilities::shock_land_enters()),
);

// RTR 249 — Transguild Promenade
pub(in crate::card::sets) static TRANSGUILD_PROMENADE: CardRecord = CardRecord::new_with_legacy_id(
    1349,
    "Transguild Promenade",
    CardArt::new("90ce8115-41fe-44c2-8719-741ba87bcb17", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::enters_trigger(
            "When this land enters, sacrifice it unless you pay {1}.",
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{1}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// RTR 250 — Plains (reprint)

// RTR 251 — Plains (alternate printing)

// RTR 252 — Plains (alternate printing)

// RTR 253 — Plains (alternate printing)

// RTR 254 — Plains (alternate printing)

// RTR 255 — Island (reprint)

// RTR 256 — Island (alternate printing)

// RTR 257 — Island (alternate printing)

// RTR 258 — Island (alternate printing)

// RTR 259 — Island (alternate printing)

// RTR 260 — Swamp (reprint)

// RTR 261 — Swamp (alternate printing)

// RTR 262 — Swamp (alternate printing)

// RTR 263 — Swamp (alternate printing)

// RTR 264 — Swamp (alternate printing)

// RTR 265 — Mountain (reprint)

// RTR 266 — Mountain (alternate printing)

// RTR 267 — Mountain (alternate printing)

// RTR 268 — Mountain (alternate printing)

// RTR 269 — Mountain (alternate printing)

// RTR 270 — Forest (reprint)

// RTR 271 — Forest (alternate printing)

// RTR 272 — Forest (alternate printing)

// RTR 273 — Forest (alternate printing)

// RTR 274 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGEL_OF_SERENITY,
    &ARMORY_GUARD,
    &AVENGING_ARROW,
    &AZORIUS_ARRESTER,
    &AZORIUS_JUSTICIAR,
    &BAZAAR_KROVOD,
    &CONCORDIA_PEGASUS,
    &ETHEREAL_ARMOR,
    &EYES_IN_THE_SKIES,
    &FENCING_ACE,
    &KEENING_APPARITION,
    &KNIGHTLY_VALOR,
    &MARTIAL_LAW,
    &PALISADE_GIANT,
    &PHANTOM_GENERAL,
    &PRECINCT_CAPTAIN,
    &REST_IN_PEACE,
    &ROOTBORN_DEFENSES,
    &SECURITY_BLOCKADE,
    &SELESNYA_SENTRY,
    &SELLER_OF_SONGBIRDS,
    &SOUL_TITHE,
    &SPHERE_OF_SAFETY,
    &SUNSPIRE_GRIFFIN,
    &SWIFT_JUSTICE,
    &TRAINED_CARACAL,
    &TROSTANIS_JUDGMENT,
    &AQUUS_STEED,
    &BLUSTERSQUALL,
    &CANCEL,
    &CHRONIC_FLOODING,
    &CONJURED_CURRENCY,
    &CROSSTOWN_COURIER,
    &CYCLONIC_RIFT,
    &DISPEL,
    &DOORKEEPER,
    &DOWNSIZE,
    &FAERIE_IMPOSTOR,
    &HOVER_BARRIER,
    &INACTION_INJUNCTION,
    &INSPIRATION,
    &ISPERIAS_SKYWATCH,
    &JACE_ARCHITECT_OF_THOUGHT,
    &MIZZIUM_SKIN,
    &PARALYZING_GRASP,
    &PSYCHIC_SPIRAL,
    &RUNEWING,
    &SEARCH_THE_CITY,
    &SKYLINE_PREDATOR,
    &SOULSWORN_SPIRIT,
    &SPHINX_OF_THE_CHIMES,
    &STEALER_OF_SECRETS,
    &SYNCOPATE,
    &TOWER_DRAKE,
    &VOIDWIELDER,
    &ASSASSINS_STRIKE,
    &CATACOMB_SLUG,
    &CREMATE,
    &DAGGERDROME_IMP,
    &DARK_REVENANT,
    &DEAD_REVELER,
    &DESECRATION_DEMON,
    &DESTROY_THE_EVIDENCE,
    &DEVIANT_GLEE,
    &DRAINPIPE_VERMIN,
    &GRAVE_BETRAYAL,
    &GRIM_ROUSTABOUT,
    &LAUNCH_PARTY,
    &NECROPOLIS_REGENT,
    &OGRE_JAILBREAKER,
    &PACK_RAT,
    &PERILOUS_SHADOW,
    &SEWER_SHAMBLER,
    &SHRIEKING_AFFLICTION,
    &SLUM_REAPER,
    &STAB_WOUND,
    &TAVERN_SWINDLER,
    &TERRUS_WURM,
    &THRILL_KILL_ASSASSIN,
    &ULTIMATE_PRICE,
    &UNDERWORLD_CONNECTIONS,
    &ZANIKEV_LOCUST,
    &ANNIHILATING_FIRE,
    &ASH_ZEALOT,
    &BATTERHORN,
    &BELLOWS_LIZARD,
    &BLOODFRAY_GIANT,
    &CHAOS_IMPS,
    &COBBLEBRUTE,
    &DYNACHARGE,
    &ELECTRICKERY,
    &EXPLOSIVE_IMPACT,
    &GOBLIN_RALLY,
    &GORE_HOUSE_CHAINWALKER,
    &GUILD_FEUD,
    &GUTTERSNIPE,
    &LOBBER_CREW,
    &MINOTAUR_AGGRESSOR,
    &MIZZIUM_MORTARS,
    &PURSUIT_OF_FLIGHT,
    &PYROCONVERGENCE,
    &RACECOURSE_FURY,
    &SPLATTER_THUG,
    &STREET_SPASM,
    &SURVEY_THE_WRECKAGE,
    &TENEMENT_CRASHER,
    &TRAITOROUS_INSTINCT,
    &UTVARA_HELLKITE,
    &VANDALBLAST,
    &VIASHINO_RACKETEER,
    &AERIAL_PREDATION,
    &ARCHWEAVER,
    &AXEBANE_GUARDIAN,
    &AXEBANE_STAG,
    &BRUSHSTRIDER,
    &CENTAURS_HERALD,
    &CHORUS_OF_MIGHT,
    &DEADBRIDGE_GOLIATH,
    &DEATHS_PRESENCE,
    &DRUDGE_BEETLE,
    &DRUIDS_DELIVERANCE,
    &GATECREEPER_VINE,
    &GOBBLING_OOZE,
    &GOLGARI_DECOY,
    &HORNCALLERS_CHANT,
    &KOROZDA_MONITOR,
    &MANA_BLOOM,
    &OAK_STREET_INNKEEPER,
    &RUBBLEBACK_RHINO,
    &SAVAGE_SURGE,
    &SEEK_THE_HORIZON,
    &SLIME_MOLDING,
    &STONEFARE_CROCODILE,
    &TOWERING_INDRIK,
    &URBAN_BURGEONING,
    &WILD_BEASTMASTER,
    &WORLDSPINE_WURM,
    &ABRUPT_DECAY,
    &ARCHON_OF_THE_TRIUMVIRATE,
    &ARMADA_WURM,
    &AUGER_SPREE,
    &AZORIUS_CHARM,
    &CALL_OF_THE_CONCLAVE,
    &CARNIVAL_HELLSTEED,
    &CENTAUR_HEALER,
    &CHEMISTERS_TRICK,
    &COLLECTIVE_BLESSING,
    &COMMON_BOND,
    &CORPSEJACK_MENACE,
    &COUNTERFLUX,
    &COURSERS_ACCORD,
    &DETENTION_SPHERE,
    &DRAMATIC_RESCUE,
    &DREADBORE,
    &DREG_MANGLER,
    &EPIC_EXPERIMENT,
    &ESSENCE_BACKLASH,
    &FALL_OF_THE_GAVEL,
    &FIREMIND_S_FORESIGHT,
    &GOBLIN_ELECTROMANCER,
    &GOLGARI_CHARM,
    &GRISLY_SALVAGE,
    &HAVOC_FESTIVAL,
    &HELLHOLE_FLAILER,
    &HEROES_REUNION,
    &HUSSAR_PATROL,
    &HYPERSONIC_DRAGON,
    &ISPERIA_SUPREME_JUDGE,
    &IZZET_CHARM,
    &IZZET_STATICASTER,
    &JARAD_GOLGARI_LICH_LORD,
    &JARAD_S_ORDERS,
    &KOROZDA_GUILDMAGE,
    &LOTLETH_TROLL,
    &LOXODON_SMITER,
    &LYEV_SKYKNIGHT,
    &MERCURIAL_CHEMISTER,
    &NEW_PRAHV_GUILDMAGE,
    &NIVIX_GUILDMAGE,
    &NIV_MIZZET_DRACOGENIUS,
    &RAKDOS_CHARM,
    &RAKDOS_RAGEMUTT,
    &RAKDOS_RINGLEADER,
    &RAKDOS_LORD_OF_RIOTS,
    &RAKDOS_S_RETURN,
    &RIGHTEOUS_AUTHORITY,
    &RISEN_SANCTUARY,
    &RITES_OF_REAPING,
    &RIX_MAADI_GUILDMAGE,
    &SEARCH_WARRANT,
    &SELESNYA_CHARM,
    &SKULL_REND,
    &SKYMARK_ROC,
    &SLAUGHTER_GAMES,
    &SLUICEWAY_SCORPION,
    &SPAWN_OF_RIX_MAADI,
    &SPHINXS_REVELATION,
    &SUPREME_VERDICT,
    &TELEPORTAL,
    &THOUGHTFLARE,
    &TREASURED_FIND,
    &TRESTLE_TROLL,
    &TROSTANI_SELESNYAS_VOICE,
    &VITU_GHAZI_GUILDMAGE,
    &VRASKA_THE_UNSEEN,
    &WAYFARING_TEMPLE,
    &AZOR_S_ELOCUTORS,
    &BLISTERCOIL_WEIRD,
    &CRYPTBORN_HORROR,
    &DEATHRITE_SHAMAN,
    &DRYAD_MILITANT,
    &FROSTBURN_WEIRD,
    &GOLGARI_LONGLEGS,
    &GROWING_RANKS,
    &JUDGES_FAMILIAR,
    &NIVMAGUS_ELEMENTAL,
    &RAKDOS_CACKLER,
    &RAKDOS_SHRED_FREAK,
    &SLITHERHEAD,
    &SUNDERING_GROWTH,
    &VASSAL_SOUL,
    &AZORIUS_KEYRUNE,
    &CHROMATIC_LANTERN,
    &CIVIC_SABER,
    &CODEX_SHREDDER,
    &GOLGARI_KEYRUNE,
    &IZZET_KEYRUNE,
    &PITHING_NEEDLE,
    &RAKDOS_KEYRUNE,
    &SELESNYA_KEYRUNE,
    &STREET_SWEEPER,
    &TABLET_OF_THE_GUILDS,
    &VOLATILE_RIG,
    &AZORIUS_GUILDGATE,
    &BLOOD_CRYPT,
    &GOLGARI_GUILDGATE,
    &GROVE_OF_THE_GUARDIAN,
    &HALLOWED_FOUNTAIN,
    &IZZET_GUILDGATE,
    &OVERGROWN_TOMB,
    &RAKDOS_GUILDGATE,
    &ROGUES_PASSAGE,
    &SELESNYA_GUILDGATE,
    &STEAM_VENTS,
    &TEMPLE_GARDEN,
    &TRANSGUILD_PROMENADE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&mmq::ARREST),          // RTR 3
    PrintingRecord::reprint(&magic_2013::MIND_ROT), // RTR 70
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),  // RTR 125
    PrintingRecord::reprint(&alpha::PLAINS),        // RTR 250
    PrintingRecord::alternate(&alpha::PLAINS, 1),   // RTR 251
    PrintingRecord::alternate(&alpha::PLAINS, 2),   // RTR 252
    PrintingRecord::alternate(&alpha::PLAINS, 3),   // RTR 253
    PrintingRecord::alternate(&alpha::PLAINS, 4),   // RTR 254
    PrintingRecord::reprint(&alpha::ISLAND),        // RTR 255
    PrintingRecord::alternate(&alpha::ISLAND, 1),   // RTR 256
    PrintingRecord::alternate(&alpha::ISLAND, 2),   // RTR 257
    PrintingRecord::alternate(&alpha::ISLAND, 3),   // RTR 258
    PrintingRecord::alternate(&alpha::ISLAND, 4),   // RTR 259
    PrintingRecord::reprint(&alpha::SWAMP),         // RTR 260
    PrintingRecord::alternate(&alpha::SWAMP, 1),    // RTR 261
    PrintingRecord::alternate(&alpha::SWAMP, 2),    // RTR 262
    PrintingRecord::alternate(&alpha::SWAMP, 3),    // RTR 263
    PrintingRecord::alternate(&alpha::SWAMP, 4),    // RTR 264
    PrintingRecord::reprint(&alpha::MOUNTAIN),      // RTR 265
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1), // RTR 266
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2), // RTR 267
    PrintingRecord::alternate(&alpha::MOUNTAIN, 3), // RTR 268
    PrintingRecord::alternate(&alpha::MOUNTAIN, 4), // RTR 269
    PrintingRecord::reprint(&alpha::FOREST),        // RTR 270
    PrintingRecord::alternate(&alpha::FOREST, 1),   // RTR 271
    PrintingRecord::alternate(&alpha::FOREST, 2),   // RTR 272
    PrintingRecord::alternate(&alpha::FOREST, 3),   // RTR 273
    PrintingRecord::alternate(&alpha::FOREST, 4),   // RTR 274
];
