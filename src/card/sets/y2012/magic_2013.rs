//! Magic 2013 card records used by the built-in ISD–M14 Standard deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord, avacyn_restored, dark_ascension};
use crate::card::CostQuantityDef;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1998::stronghold as catalog_sth;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2001::apocalypse as catalog_apc;
use crate::card::sets::y2001::odyssey as catalog_ody;
use crate::card::sets::y2004::fifth_dawn as catalog_5dn;
use crate::card::sets::y2006::time_spiral as catalog_tsp;
use crate::card::sets::y2007::future_sight as catalog_fut;
use crate::card::sets::y2010::magic_2011 as catalog_m11;
use crate::card::sets::y2012::dark_ascension as catalog_dka;
use crate::card::sets::{
    y1993::alpha, y1994::the_dark, y1999::urzas_legacy, y2001::planeshift, y2002::onslaught,
    y2011::innistrad,
};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ChoiceVisibilityDef, ChooseGroupDef, ColorSet, ComparisonDef, ControlDurationDef,
    CostModificationDef, CounterKind, CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef,
    DamagePreventionDef, DamageRecipientMatcherDef, DamageSourceMatcherDef, DiscardFollowUpDef,
    DiscardSelectionDef, DividedTotal, EffectDef, EffectRecipientDef, KeywordAbility, ManaColor,
    MoveObjectsDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PartitionGroupDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, RevealObjectsDef, SacrificedAmountDef,
    SpellAdditionalCostDef, TargetChooserDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ObjectSetBindingIndex, TargetIndex};
use crate::mana_cost;

// M13 1 — Ajani, Caller of the Pride
pub(in crate::card::sets) static AJANI_CALLER_OF_THE_PRIDE: CardRecord =
    CardRecord::new_with_legacy_id(
        2010,
        "Ajani, Caller of the Pride",
        CardArt::new(
            "5e7f410a-7934-48ae-a90b-ffd096aed43d",
            "D. Alexander Gregory",
        ),
        CardSet::Magic2013,
        CardRules::new_planeswalker(mana_cost!("{1}{W}{W}"), &["Ajani"], 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::activated_with_targets(
                    "+1: Put a +1/+1 counter on up to one target creature.",
                    &[AbilityCostDef::Loyalty(1)],
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
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::activated_with_targets(
                    "−3: Target creature gains flying and double strike until end of turn.",
                    &[AbilityCostDef::Loyalty(-3)],
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::flying()),
                            AppliedEffectDef::add_ability(&abilities::double_strike()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
                AbilityDef::activated(
                    "−8: Create X 2/2 white Cat creature tokens, where X is your life total.",
                    &[AbilityCostDef::Loyalty(-8)],
                    EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 2, 2)
                        .with_art(CardArt::new(
                            "f97868f6-a9ce-4ce9-bc3f-b535f3202602",
                            "Jesper Ejsing",
                        ))
                        .with_count(ValueDef::LifeTotal(PlayerRelation::You)),
                ),
            ]),
    );

// M13 2 — Ajani's Sunstriker
pub(in crate::card::sets) static AJANIS_SUNSTRIKER: CardRecord = CardRecord::new_with_legacy_id(
    965,
    "Ajani's Sunstriker",
    CardArt::new("3570c4d9-cd42-4aca-9421-ac44e057a785", "Matt Stewart"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Cat", "Cleric"], 2, 2)
        .with_abilities(&[abilities::lifelink()]),
);

// M13 3 — Angel's Mercy (reprint)

// M13 4 — Angelic Benediction
pub(in crate::card::sets) static ANGELIC_BENEDICTION: CardRecord = CardRecord::new_with_legacy_id(
    1501,
    "Angelic Benediction",
    CardArt::new("22125507-31e3-424c-9527-d994e4525d75", "Michael Komarck"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_abilities(&[
        abilities::exalted(),
        AbilityDef::triggered_with_targets(
            "Whenever a creature you control attacks alone, you may tap target creature.",
            TriggerEventDef::attacks_in_declaration(
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                1,
                Some(1),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
        ),
    ]),
);

// M13 5 — Attended Knight
pub(in crate::card::sets) static ATTENDED_KNIGHT: CardRecord = CardRecord::new_with_legacy_id(
    966,
    "Attended Knight",
    CardArt::new("c0f5cb3f-c27d-4b35-930f-00d806393796", "Seb McKinnon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Soldier creature token.",
            EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1).with_art(
                CardArt::new("86272c08-c5f2-413f-87ea-b135aca2d9c5", "Greg Staples"),
            ),
        ),
    ]),
);

// M13 6 — Aven Squire
pub(in crate::card::sets) static AVEN_SQUIRE: CardRecord = CardRecord::new_with_legacy_id(
    1502,
    "Aven Squire",
    CardArt::new("e60a0c43-9f47-404a-8acf-508173e7062f", "David Palumbo"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird", "Soldier"], 1, 1)
        .with_abilities(&[abilities::flying(), abilities::exalted()]),
);

// M13 7 — Battleflight Eagle
pub(in crate::card::sets) static BATTLEFLIGHT_EAGLE: CardRecord = CardRecord::new_with_legacy_id(
    967,
    "Battleflight Eagle",
    CardArt::new("4182dbd5-8eae-4f4b-86aa-2bfc24481800", "Kev Walker"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, target creature gets +2/+2 and gains flying until end of turn.", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
    ]),
);

// M13 8 — Captain of the Watch
pub(in crate::card::sets) static CAPTAIN_OF_THE_WATCH: CardRecord = CardRecord::new_with_legacy_id(
    968,
    "Captain of the Watch",
    CardArt::new("8e3c18f5-89cd-4d33-8d5b-12dacad9f9b3", "Greg Staples"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Human", "Soldier"], 3, 3).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Other Soldier creatures you control get +1/+1 and have vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Soldier"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            ),
            abilities::enters_trigger(
                "When this creature enters, create three 1/1 white Soldier creature tokens.",
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
                    .with_art(CardArt::new(
                        "86272c08-c5f2-413f-87ea-b135aca2d9c5",
                        "Greg Staples",
                    ))
                    .with_amount(3),
            ),
        ],
    ),
);

// M13 9 — Captain's Call
pub(in crate::card::sets) static CAPTAINS_CALL: CardRecord = CardRecord::new_with_legacy_id(
    969,
    "Captain's Call",
    CardArt::new("79258432-ea35-4f2a-9e4a-4abb53f335c6", "Greg Staples"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Create three 1/1 white Soldier creature tokens.",
        EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
            .with_art(CardArt::new(
                "86272c08-c5f2-413f-87ea-b135aca2d9c5",
                "Greg Staples",
            ))
            .with_amount(3),
    )),
);

// M13 10 — Crusader of Odric
static CRUSADER_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static CRUSADER_OF_ODRIC: CardRecord = CardRecord::new_with_legacy_id(
    970,
    "Crusader of Odric",
    CardArt::new("295096bb-1857-4224-bc7b-307b38cfd338", "Michael Komarck"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Crusader of Odric's power and toughness are each equal to the number of creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CRUSADER_CREATURES), ValueDef::CountMatchingObjects(&CRUSADER_CREATURES)),
            },
        ),
    ),
);

// M13 11 — Divine Favor
pub(in crate::card::sets) static DIVINE_FAVOR: CardRecord = CardRecord::new_with_legacy_id(
    971,
    "Divine Favor",
    CardArt::new("b713c1f7-9346-4f4e-8fcd-5ada5b3f95c0", "Allen Williams"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
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
                "When this Aura enters, you gain 3 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+3.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(3),
                    ),
                },
            ),
        ]),
);

// M13 12 — Divine Verdict
pub(in crate::card::sets) static DIVINE_VERDICT: CardRecord = CardRecord::new_with_legacy_id(
    972,
    "Divine Verdict",
    CardArt::new("cc52c269-d44f-449c-af59-4c425aa10bbf", "Kev Walker"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target attacking or blocking creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::AttackingOrBlocking,
        ])),
        true,
    )),
);

// M13 13 — Erase
pub(in crate::card::sets) static ERASE: CardRecord = CardRecord::new_with_legacy_id(
    973,
    "Erase",
    CardArt::new("8618b737-faa0-4a0c-a3f2-bee685c00580", "Richard Wright"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target enchantment.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
        },
    )),
);

// M13 14 — Faith's Reward
// Audit: metadata-only — Needs turn-history provenance for permanent cards put into your graveyard from the battlefield and a simultaneous mass return.
pub(in crate::card::sets) static FAITH_S_REWARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("799ed076-4724-47bb-94a0-11b42a9826eb"),
    "Faith's Reward",
    crate::card::CardArt::new("799ed076-4724-47bb-94a0-11b42a9826eb", "Raymond Swanland"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 15 — Glorious Charge
pub(in crate::card::sets) static GLORIOUS_CHARGE: CardRecord = CardRecord::new_with_legacy_id(
    974,
    "Glorious Charge",
    CardArt::new("f8672cfd-e34b-4587-9e24-015e03c7574d", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 16 — Griffin Protector
pub(in crate::card::sets) static GRIFFIN_PROTECTOR: CardRecord = CardRecord::new_with_legacy_id(
    975,
    "Griffin Protector",
    CardArt::new("ddae4f7a-525c-4306-81b5-b0991840a11e", "Christopher Moeller"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature gets +1/+1 until end of turn.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), None, Some(ZoneKind::Battlefield)),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 17 — Guardian Lions
pub(in crate::card::sets) static GUARDIAN_LIONS: CardRecord = CardRecord::new_with_legacy_id(
    976,
    "Guardian Lions",
    CardArt::new("3defc506-537e-4659-815d-5dab15fbf199", "Johannes Voss"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Cat"], 1, 6)
        .with_abilities(&[abilities::vigilance()]),
);

// M13 18 — Guardians of Akrasa
pub(in crate::card::sets) static GUARDIANS_OF_AKRASA: CardRecord = CardRecord::new_with_legacy_id(
    1503,
    "Guardians of Akrasa",
    CardArt::new("383c9aa5-30ad-4a2a-8b64-65d4b333c613", "Alan Pollack"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 0, 4)
        .with_abilities(&[abilities::defender(), abilities::exalted()]),
);

// M13 19 — Healer of the Pride
pub(in crate::card::sets) static HEALER_OF_THE_PRIDE: CardRecord = CardRecord::new_with_legacy_id(
    977,
    "Healer of the Pride",
    CardArt::new(
        "35716e37-1bb2-41e2-bb55-e65126b01ce3",
        "Christopher Moeller",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Cat", "Cleric"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 2 life.",
            TriggerEventDef::zone_changed(
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
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// M13 20 — Intrepid Hero (reprint)

// M13 21 — Knight of Glory
pub(in crate::card::sets) static KNIGHT_OF_GLORY: CardRecord = CardRecord::new_with_legacy_id(
    1589,
    "Knight of Glory",
    CardArt::new("1646cb67-e0ac-4f2d-af21-618ff3613d69", "Peter Mohrbacher"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        abilities::exalted(),
    ]),
);

// M13 22 — Oblivion Ring
pub(in crate::card::sets) static OBLIVION_RING: CardRecord = CardRecord::new_with_legacy_id(
    192,
    "Oblivion Ring",
    CardArt::new("1e2a73ec-39be-4d23-8c25-17d7c174dcee", "Franz Vohwinkel"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        abilities::enters_trigger_with_targets("When this enchantment enters, exile another target nonland permanent.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
}),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
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

// M13 23 — Odric, Master Tactician
// Audit: metadata-only — Needs an attacking-group threshold and a combat procedure that lets its controller assign every blocker.
pub(in crate::card::sets) static ODRIC_MASTER_TACTICIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb1552a8-27b4-4a95-9022-6fdd59aca28f"),
    "Odric, Master Tactician",
    crate::card::CardArt::new("bb1552a8-27b4-4a95-9022-6fdd59aca28f", "Michael Komarck"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 24 — Pacifism
pub(in crate::card::sets) static PACIFISM: CardRecord = CardRecord::new_with_legacy_id(
    1750,
    "Pacifism",
    CardArt::new("258e9351-2108-4dbe-97a8-3eeb9c7b502a", "Robert Bliss"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature can't attack or block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Two prohibitions rather than one: nothing in the vocabulary bars combat
                    // wholesale, and nothing needs to -- attacking and blocking are separate
                    // declarations, so barring each is barring both.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
        ]),
);

// M13 25 — Pillarfield Ox
pub(in crate::card::sets) static PILLARFIELD_OX: CardRecord = CardRecord::new_with_legacy_id(
    978,
    "Pillarfield Ox",
    CardArt::new("33e2f3ae-bf92-478b-9c63-acc3f175f02a", "Andrew Robinson"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Ox"], 2, 4),
);

// M13 26 — Planar Cleansing
pub(in crate::card::sets) static PLANAR_CLEANSING: CardRecord = CardRecord::new_with_legacy_id(
    979,
    "Planar Cleansing",
    CardArt::new("b5047b71-2359-4d9a-a168-a8eec43c5f1b", "Michael Komarck"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all nonland permanents.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: None,
        },
    )),
);

// M13 27 — Prized Elephant
static PRIZED_ELEPHANT_FORESTS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static PRIZED_ELEPHANT: CardRecord = CardRecord::new_with_legacy_id(
    1352,
    "Prized Elephant",
    CardArt::new("01597ede-94e7-44a4-93c2-7fd1db11e92a", "Ioan Dumitrescu"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Elephant"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Forest.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&PRIZED_ELEPHANT_FORESTS),
                    ValueDef::AnyMatchingObject(&PRIZED_ELEPHANT_FORESTS),
                ),
            },
        ),
        AbilityDef::activated(
            "{G}: This creature gains trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 28 — Rain of Blades
pub(in crate::card::sets) static RAIN_OF_BLADES: CardRecord = CardRecord::new_with_legacy_id(
    980,
    "Rain of Blades",
    CardArt::new("f3bd6ca4-c4ed-41c3-834c-23e0c1741b72", "Rob Alexander"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Rain of Blades deals 1 damage to each attacking creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(1),
        },
    )),
);

// M13 29 — Rhox Faithmender
pub(in crate::card::sets) static RHOX_FAITHMENDER: CardRecord = CardRecord::new_with_legacy_id(
    204,
    "Rhox Faithmender",
    CardArt::new("85ea185a-7b38-49f3-be73-be8180fb6295", "Wesley Burt"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Rhino", "Monk"], 1, 5).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::replacement_for(
            "If you would gain life, you gain twice that much life instead.",
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            ReplacementEffectDef::MultiplyEventAmount(2),
        ),
    ]),
);

// M13 30 — Safe Passage
pub(in crate::card::sets) static SAFE_PASSAGE: CardRecord = CardRecord::new_with_legacy_id(
    1498,
    "Safe Passage",
    CardArt::new(
        "9fc65c3f-ad29-4368-bf45-8345a7ec6f31",
        "Christopher Moeller",
    ),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Prevent all damage that would be dealt to you and creatures you control this turn.",
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(
                DamageEventMatcherDef::to_player_and_creatures_controlled_by(
                    PlayerRefDef::EffectController,
                ),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 31 — Serra Angel (reprint)

// M13 32 — Serra Avatar (reprint)

// M13 33 — Serra Avenger (reprint)

// M13 34 — Show of Valor
pub(in crate::card::sets) static SHOW_OF_VALOR: CardRecord = CardRecord::new_with_legacy_id(
    981,
    "Show of Valor",
    CardArt::new("abe4d19d-1c9f-4b05-bde2-a9290b52c28d", "Anthony Palumbo"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
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
    )),
);

// M13 35 — Silvercoat Lion
pub(in crate::card::sets) static SILVERCOAT_LION: CardRecord = CardRecord::new_with_legacy_id(
    982,
    "Silvercoat Lion",
    CardArt::new("9d33e866-cfd8-44e6-8070-df8df1ce965d", "Terese Nielsen"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat"], 2, 2),
);

// M13 36 — Sublime Archangel
pub(in crate::card::sets) static SUBLIME_ARCHANGEL: CardRecord = CardRecord::new_with_legacy_id(
    1895,
    "Sublime Archangel",
    CardArt::new("f5cc38bc-55a4-446a-b054-48fb90216946", "Cynthia Sheppard"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 4, 3).with_abilities(&[
        abilities::flying(),
        abilities::exalted(),
        AbilityDef::static_ability(
            "Other creatures you control have exalted.",
            // Each granted copy is its own instance, so attacking alone into a board of
            // three other creatures is four separate triggers.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::exalted()),
            },
        ),
    ]),
);

// M13 37 — Touch of the Eternal
// Audit: metadata-only — No effect can set a player's life total from a battlefield permanent count.
pub(in crate::card::sets) static TOUCH_OF_THE_ETERNAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55c5f0c2-99e6-42b7-aa16-61d5815d060d"),
    "Touch of the Eternal",
    crate::card::CardArt::new(
        "55c5f0c2-99e6-42b7-aa16-61d5815d060d",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 38 — War Falcon
// Audit: metadata-only — Combat restrictions cannot condition this source's attack permission on controlling a Knight or Soldier.
pub(in crate::card::sets) static WAR_FALCON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e092a0d-c031-4a76-86c1-7f83878a06e8"),
    "War Falcon",
    crate::card::CardArt::new("7e092a0d-c031-4a76-86c1-7f83878a06e8", "Volkan Baǵa"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 39 — War Priest of Thune
pub(in crate::card::sets) static WAR_PRIEST_OF_THUNE: CardRecord = CardRecord::new_with_legacy_id(
    241,
    "War Priest of Thune",
    CardArt::new("d28eb320-aea7-466e-8718-de8652a2b191", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target enchantment.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                // "You may" is an optional target: declining to choose one is how the
                // trigger does nothing, so the minimum is zero rather than one.
                minimum: 0,
                maximum: 1,
                exact_count: None,
                divided_total: None,
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);
// M13 40 — Warclamp Mastiff
pub(in crate::card::sets) static WARCLAMP_MASTIFF: CardRecord = CardRecord::new_with_legacy_id(
    983,
    "Warclamp Mastiff",
    CardArt::new("102e48e0-8a5f-499d-ac62-005d3c075ef3", "David Palumbo"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{W}"), &["Dog"], 1, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// M13 41 — Archaeomancer
pub(in crate::card::sets) static ARCHAEOMANCER: CardRecord = CardRecord::new_with_legacy_id(
    984,
    "Archaeomancer",
    CardArt::new("73c6d1be-55ad-4ee4-b044-88438e9b78cc", "Zoltan Boros"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 1, 2).with_ability(
        abilities::enters_trigger_with_targets("When this creature enters, return target instant or sorcery card from your graveyard to your hand.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })], EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
}),
    ),
);

// M13 42 — Arctic Aven
static PLAINS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static ARCTIC_AVEN: CardRecord = CardRecord::new_with_legacy_id(
    985,
    "Arctic Aven",
    CardArt::new("06f6aab1-c400-4d87-b68e-f36552e7417f", "Igor Kieryluk"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Bird", "Wizard"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Plains.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&PLAINS_YOU_CONTROL),
                    ValueDef::AnyMatchingObject(&PLAINS_YOU_CONTROL),
                ),
            },
        ),
        AbilityDef::activated(
            "{W}: This creature gains lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 43 — Augur of Bolas
pub(in crate::card::sets) static AUGUR_OF_BOLAS: CardRecord = CardRecord::new_with_legacy_id(
    135,
    "Augur of Bolas",
    CardArt::new("2e6ec8a6-ad88-45c9-ab4b-dd7de2418bb7", "Slawomir Maniak"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Merfolk", "Wizard"],
        1,
        3,
    )
    .with_ability(abilities::enters_trigger(
        "When this creature enters, look at the top three cards of your library. You may reveal an instant or sorcery card from among them and put it into your hand. Put the rest on the bottom of your library in any order.",
        abilities::look_at_top_cards_reveal_choice_to_hand_rest_bottom(
            ValueDef::Constant(3),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            0,
            1,
        ),
    )),
);

// M13 44 — Battle of Wits
pub(in crate::card::sets) static BATTLE_OF_WITS: CardRecord = CardRecord::new_with_legacy_id(
    1353,
    "Battle of Wits",
    CardArt::new("b4be15a4-693f-4e22-a46c-38bb440c073c", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{3}{U}{U}")).with_ability(
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if you have 200 or more cards in your library, you win the game.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Library],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 200,
            },
            EffectDef::LoseTheGame {
                player: EffectRecipientDef::Opponent,
            },
        ),
    ),
);

// M13 45 — Clone (reprint)

// M13 46 — Courtly Provocateur
// Audit: metadata-only — No turn-long effects require a target creature to attack or block if able.
pub(in crate::card::sets) static COURTLY_PROVOCATEUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba912207-a8bf-4ffb-9967-34029cb09f7f"),
    "Courtly Provocateur",
    crate::card::CardArt::new("ba912207-a8bf-4ffb-9967-34029cb09f7f", "James Ryman"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 47 — Divination (reprint)

// M13 48 — Downpour
pub(in crate::card::sets) static DOWNPOUR: CardRecord = CardRecord::new_with_legacy_id(
    986,
    "Downpour",
    CardArt::new("f220afb1-8638-4b54-b6af-0043b4cc1cef", "Eytan Zana"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to three target creatures.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            3,
        )],
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// M13 49 — Encrust
pub(in crate::card::sets) static ENCRUST: CardRecord = CardRecord::new_with_legacy_id(
    1953,
    "Encrust",
    CardArt::new("dfd05474-5cec-4c71-85e7-79cf25958525", "Jason Felix"),
    CardSet::Magic2013,
    // It answers an artifact as readily as a creature, which is the whole
    // reason to play it over an ordinary creature Aura.
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}"))
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
                "Enchanted permanent doesn't untap during its controller's untap step and its \
                 activated abilities can't be activated.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // Both halves for the same duration, so the Aura leaving returns the untap
                    // and the activations together.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                        AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
                    ]),
                },
            ),
        ]),
);

// M13 50 — Essence Scatter
pub(in crate::card::sets) static ESSENCE_SCATTER: CardRecord = CardRecord::new_with_legacy_id(
    162,
    "Essence Scatter",
    CardArt::new("fcd965f9-bdaa-4434-a9c8-53fc57e997db", "Jon Foster"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target creature spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Creature)),
    )),
);

// M13 51 — Faerie Invaders
pub(in crate::card::sets) static FAERIE_INVADERS: CardRecord = CardRecord::new_with_legacy_id(
    987,
    "Faerie Invaders",
    CardArt::new("fcbc71b3-544b-4b81-8922-52744892989b", "Ryan Pancoast"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Faerie", "Rogue"], 3, 3)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// M13 52 — Fog Bank
pub(in crate::card::sets) static FOG_BANK: CardRecord = CardRecord::new_with_legacy_id(
    1912,
    "Fog Bank",
    CardArt::new("8a5a69dc-c6f3-459b-9dcd-b3363c26ca34", "Howard Lyon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Wall"], 0, 2).with_abilities(&[
        abilities::defender(),
        abilities::flying(),
        AbilityDef::static_ability(
            "Prevent all combat damage that would be dealt to and dealt by this creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // Both directions of the same clause: nothing it deals lands and nothing
                // dealt to it lands, so it blocks anything and survives, and kills nothing.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
                        kind: DamageKindDef::Combat,
                        source: DamageSourceMatcherDef::Any,
                        recipient: DamageRecipientMatcherDef::AffectedObject,
                    })),
                    AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
                        kind: DamageKindDef::Combat,
                        source: DamageSourceMatcherDef::AffectedObject,
                        recipient: DamageRecipientMatcherDef::Any,
                    })),
                ]),
            },
        ),
    ]),
);

// M13 53 — Harbor Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HARBOR_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa10b43f-eb63-4999-92a0-56826031b686"),
    "Harbor Serpent",
    crate::card::CardArt::new("af0f7357-08b0-403e-8913-8965662a905e", "Daarken"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 54 — Hydrosurge
pub(in crate::card::sets) static HYDROSURGE: CardRecord = CardRecord::new_with_legacy_id(
    988,
    "Hydrosurge",
    CardArt::new("1a22f992-ef16-45be-8bac-bd7418ed068f", "Steve Prescott"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -5/-0 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-5),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 55 — Index (reprint)

// M13 56 — Jace, Memory Adept
pub(in crate::card::sets) static JACE_MEMORY_ADEPT: CardRecord = CardRecord::new_with_legacy_id(
    181,
    "Jace, Memory Adept",
    CardArt::new(
        "96b2a335-2f01-4ba7-a037-453dbb1045e9",
        "D. Alexander Gregory",
    ),
    CardSet::Magic2013,
    CardRules::new_planeswalker(mana_cost!("{3}{U}{U}"), &["Jace"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+1: Draw a card. Target player mills a card.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Mill {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "0: Target player mills ten cards.",
                &[AbilityCostDef::Loyalty(0)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Mill {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(10),
                },
            ),
            AbilityDef::activated_with_targets(
                "−7: Any number of target players each draw twenty cards.",
                &[AbilityCostDef::Loyalty(-7)],
                // Two players means "any number" is up to two.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    2,
                )],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(20),
                },
            ),
        ]),
);

// M13 57 — Jace's Phantasm
// Audit: metadata-only — Conditional static bonuses cannot test whether an opponent's graveyard contains at least ten cards.
pub(in crate::card::sets) static JACE_S_PHANTASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16829504-385c-4154-8e6d-f3fbaf273890"),
    "Jace's Phantasm",
    crate::card::CardArt::new("16829504-385c-4154-8e6d-f3fbaf273890", "Johann Bodin"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 58 — Kraken Hatchling
pub(in crate::card::sets) static KRAKEN_HATCHLING: CardRecord = CardRecord::new_with_legacy_id(
    989,
    "Kraken Hatchling",
    CardArt::new("59a50590-9091-4632-bf8c-792e1e0a75a8", "Jason Felix"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{U}"), &["Kraken"], 0, 4),
);

// M13 59 — Master of the Pearl Trident
pub(in crate::card::sets) static MASTER_OF_THE_PEARL_TRIDENT: CardRecord =
    CardRecord::new_with_legacy_id(
        1859,
        "Master of the Pearl Trident",
        CardArt::new("e7decbd3-c754-451c-8d63-4f31f81412d2", "Ryan Pancoast"),
        CardSet::Magic2013,
        CardRules::new_creature(mana_cost!("{U}{U}"), &["Merfolk"], 2, 2).with_ability(
            AbilityDef::static_ability(
                "Other Merfolk creatures you control get +1/+1 and have islandwalk.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        // "Other Merfolk creatures you control": narrower than Lord of Atlantis,
                        // which reaches every Merfolk on the battlefield including the opponent's.
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Merfolk"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::landwalk(BasicLandType::Island)),
                    ]),
                },
            ),
        ),
    );

// M13 60 — Merfolk of the Pearl Trident (reprint)

// M13 61 — Mind Sculpt
pub(in crate::card::sets) static MIND_SCULPT: CardRecord = CardRecord::new_with_legacy_id(
    990,
    "Mind Sculpt",
    CardArt::new("5870d18e-0303-4722-b7f2-a751f8e372be", "Michael C. Hayes"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent mills seven cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(7),
        },
    )),
);

// M13 62 — Negate
pub(in crate::card::sets) static NEGATE: CardRecord = CardRecord::new_with_legacy_id(
    191,
    "Negate",
    CardArt::new("8da17a86-3666-46b8-932e-daafd6a0cd69", "Jeremy Jarvis"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target noncreature spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Not(
            &ObjectPredicateDef::HasType(CardType::Creature),
        )),
    )),
);

// M13 63 — Omniscience
pub(in crate::card::sets) static OMNISCIENCE: CardRecord = CardRecord::new_with_legacy_id(
    1695,
    "Omniscience",
    CardArt::new("1088f33e-cb5f-4248-ae8e-280c4e41f291", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{7}{U}{U}{U}")).with_ability(
        AbilityDef::static_ability(
            "You may cast spells from your hand without paying their mana costs.",
            EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                spell: ObjectPredicateDef::Any,
                caster: PlayerRelation::You,
                zones: &[ZoneKind::Hand],
                cost: mana_cost!("{0}"),
            }),
        ),
    ),
);

// M13 64 — Redirect
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REDIRECT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60bae44b-c6f2-40bf-a427-aee5cfbdfea9"),
    "Redirect",
    crate::card::CardArt::new("0eef8431-f63c-44e0-940c-e1a38c338214", "Izzy"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 65 — Rewind (reprint)

// M13 66 — Scroll Thief
pub(in crate::card::sets) static SCROLL_THIEF: CardRecord = CardRecord::new_with_legacy_id(
    991,
    "Scroll Thief",
    CardArt::new(
        "dc201a82-fb48-4bb4-b072-e206e6872aa5",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Rogue"], 1, 3).with_ability(
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

// M13 67 — Sleep
/// Both clauses reach the same set, so the skip lands on exactly the
/// creatures the tap found rather than on whatever is tapped later.
static SLEEP_THEIR_CREATURES: EffectRecipientDef = EffectRecipientDef::objects_controlled_by_target(
    ObjectPredicateDef::HasType(CardType::Creature),
    TargetIndex::PRIMARY,
);

pub(in crate::card::sets) static SLEEP: CardRecord = CardRecord::new_with_legacy_id(
    1860,
    "Sleep",
    CardArt::new("1e352497-1454-4917-b38c-4cc45424d876", "Chris Rahn"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap all creatures target player controls. Those creatures don't untap during that \
         player's next untap step.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: SLEEP_THEIR_CREATURES,
            },
            EffectDef::SkipNextUntapSteps {
                object: SLEEP_THEIR_CREATURES,
                count: 1,
            },
        ]),
    )),
);

// M13 68 — Spelltwine
// Audit: metadata-only — Needs linked graveyard choices, spell copies, and permission to cast both copies without paying their costs.
pub(in crate::card::sets) static SPELLTWINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e4d2f5ab-c6be-4661-843c-51b4977a9bea"),
    "Spelltwine",
    crate::card::CardArt::new("e4d2f5ab-c6be-4661-843c-51b4977a9bea", "Noah Bradley"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 69 — Sphinx of Uthuun
const SPHINX_INSPECTED: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
const SPHINX_FIRST: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);
const SPHINX_SECOND: ObjectSetBindingIndex = ObjectSetBindingIndex::new(2);
const SPHINX_CHOSEN: ObjectSetBindingIndex = ObjectSetBindingIndex::new(3);
const SPHINX_UNCHOSEN: ObjectSetBindingIndex = ObjectSetBindingIndex::new(4);

pub(in crate::card::sets) static SPHINX_OF_UTHUUN: CardRecord = CardRecord::new_with_legacy_id(
    1354,
    "Sphinx of Uthuun",
    CardArt::new("4462978c-0076-466b-a64b-0f54d09d4f27", "Kekai Kotaki"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Sphinx"], 5, 6).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard.",
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(5),
                SPHINX_INSPECTED,
                &const { EffectDef::RevealObjects(RevealObjectsDef {
                    input: ObjectSetDef::Binding(SPHINX_INSPECTED),
                    then: &const { EffectDef::PartitionGroup(PartitionGroupDef {
                        actor: PlayerRefDef::Opponent,
                        input: ObjectSetDef::Binding(SPHINX_INSPECTED),
                        first: SPHINX_FIRST,
                        second: SPHINX_SECOND,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &const { EffectDef::ChooseGroup(ChooseGroupDef {
                            actor: PlayerRefDef::EffectController,
                            first: ObjectSetDef::Binding(SPHINX_FIRST),
                            second: ObjectSetDef::Binding(SPHINX_SECOND),
                            chosen: SPHINX_CHOSEN,
                            unchosen: SPHINX_UNCHOSEN,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &const { EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(SPHINX_CHOSEN),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Hand,
                                placement: ZonePlacement::Top,
                                moved: None,
                                then: &const { EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(SPHINX_UNCHOSEN),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }) },
                            }) },
                        }) },
                    }) },
                }) },
            ),
        ),
    ]),
);

// M13 70 — Stormtide Leviathan (reprint)

// M13 71 — Switcheroo
// Audit: metadata-only — Indefinite control exchange between two targets is unavailable.
pub(in crate::card::sets) static SWITCHEROO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d62aaf3-0fd4-44ba-8eeb-18ac759dfe84"),
    "Switcheroo",
    crate::card::CardArt::new("7d62aaf3-0fd4-44ba-8eeb-18ac759dfe84", "Kev Walker"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 72 — Talrand, Sky Summoner
pub(in crate::card::sets) static TALRAND_SKY_SUMMONER: CardRecord = CardRecord::new_with_legacy_id(
    992,
    "Talrand, Sky Summoner",
    CardArt::new("bc1a6867-921d-4912-afae-c3c445ad81e7", "Svetlin Velinov"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{2}{U}{U}"),
        &["Merfolk", "Wizard"],
        2,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_ability(AbilityDef::triggered(
        "Whenever you cast an instant or sorcery spell, create a 2/2 blue Drake creature token with flying.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
        ])),
        EffectDef::create_creature_token(&["Drake"], &[ManaColor::Blue], 2, 2).with_abilities(&[abilities::flying()]).with_art(CardArt::new("93679bb9-ee1c-4eea-bcdd-72785d5788af", "Svetlin Velinov")),
    )),
);

// M13 73 — Talrand's Invocation
pub(in crate::card::sets) static TALRANDS_INVOCATION: CardRecord = CardRecord::new_with_legacy_id(
    993,
    "Talrand's Invocation",
    CardArt::new("c2cd809c-557a-42a5-950b-56b5b47b325b", "Svetlin Velinov"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell(
        "Create two 2/2 blue Drake creature tokens with flying.",
        EffectDef::create_creature_token(&["Drake"], &[ManaColor::Blue], 2, 2)
            .with_abilities(&[abilities::flying()])
            .with_art(CardArt::new(
                "93679bb9-ee1c-4eea-bcdd-72785d5788af",
                "Svetlin Velinov",
            ))
            .with_amount(2),
    )),
);

// M13 74 — Tricks of the Trade
pub(in crate::card::sets) static TRICKS_OF_THE_TRADE: CardRecord = CardRecord::new_with_legacy_id(
    1355,
    "Tricks of the Trade",
    CardArt::new("8c796ef9-4061-4f82-9ee9-3bc446804ee9", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{3}{U}"))
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
                "Enchanted creature gets +2/+0 and can't be blocked.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                            ObjectPredicateDef::Any,
                        )),
                    ]),
                },
            ),
        ]),
);

// M13 75 — Unsummon (reprint)

// M13 76 — Vedalken Entrancer
pub(in crate::card::sets) static VEDALKEN_ENTRANCER: CardRecord = CardRecord::new_with_legacy_id(
    994,
    "Vedalken Entrancer",
    CardArt::new("dc4bbd25-5ddd-4502-b582-b7d89c9f97a5", "Dan Murayama Scott"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Vedalken", "Wizard"], 1, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{U}, {T}: Target player mills two cards.",
            &[
                AbilityCostDef::Mana(mana_cost!("{U}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// M13 77 — Void Stalker
// Audit: metadata-only — Needs simultaneous source-and-target library moves followed by shuffling both affected owners' libraries.
pub(in crate::card::sets) static VOID_STALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fc30e31-4796-4e98-992c-a56cd51ad3c9"),
    "Void Stalker",
    crate::card::CardArt::new("7fc30e31-4796-4e98-992c-a56cd51ad3c9", "Marco Nelor"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 78 — Watercourser
pub(in crate::card::sets) static WATERCOURSER: CardRecord = CardRecord::new_with_legacy_id(
    995,
    "Watercourser",
    CardArt::new("a27c441a-b31d-4214-8fc5-054003e257dc", "Mathias Kollros"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental"], 2, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
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

// M13 79 — Welkin Tern
pub(in crate::card::sets) static WELKIN_TERN: CardRecord = CardRecord::new_with_legacy_id(
    1748,
    "Welkin Tern",
    CardArt::new("ddfd4f37-3630-4770-bfad-83623c11be19", "Austin Hsu"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 2, 1).with_abilities(&[
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
    ]),
);

// M13 80 — Wind Drake
pub(in crate::card::sets) static WIND_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    618,
    "Wind Drake",
    CardArt::new("c9dcb8d2-0da9-40fc-b0c0-2c76b3d277bc", "Steve Prescott"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

// M13 81 — Blood Reckoning
pub(in crate::card::sets) static BLOOD_RECKONING: CardRecord = CardRecord::new_with_legacy_id(
    1356,
    "Blood Reckoning",
    CardArt::new("24577bb2-61b0-4675-84e6-5d675b28fc0e", "Wayne Reynolds"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a creature attacks you or a planeswalker you control, that creature's controller loses 1 life.",
        TriggerEventDef::attacks(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// M13 82 — Bloodhunter Bat
pub(in crate::card::sets) static BLOODHUNTER_BAT: CardRecord = CardRecord::new_with_legacy_id(
    996,
    "Bloodhunter Bat",
    CardArt::new("99c10705-6e0e-46f6-a64c-0095b2796aaf", "Tomasz Jedruszek"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Bat"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target player loses 2 life and you gain 2 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
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

// M13 83 — Bloodthrone Vampire
pub(in crate::card::sets) static BLOODTHRONE_VAMPIRE: CardRecord = CardRecord::new_with_legacy_id(
    997,
    "Bloodthrone Vampire",
    CardArt::new("7c0b87e0-d5e4-44f2-8220-325443ee9f31", "Steve Argyle"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 1, 1).with_ability(
        AbilityDef::activated(
            "Sacrifice a creature: This creature gets +2/+2 until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
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

// M13 84 — Cower in Fear
pub(in crate::card::sets) static COWER_IN_FEAR: CardRecord = CardRecord::new_with_legacy_id(
    998,
    "Cower in Fear",
    CardArt::new("bf2d53b8-7847-4b94-9711-eca29facccba", "Nils Hamm"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell(
        "Creatures your opponents control get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 85 — Crippling Blight
pub(in crate::card::sets) static CRIPPLING_BLIGHT: CardRecord = CardRecord::new_with_legacy_id(
    1751,
    "Crippling Blight",
    CardArt::new("eeed276f-40b5-40a7-9005-94021fa49aa2", "Lucas Graciano"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/-1 and can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-1),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            ),
        ]),
);

// M13 86 — Dark Favor
pub(in crate::card::sets) static DARK_FAVOR: CardRecord = CardRecord::new_with_legacy_id(
    999,
    "Dark Favor",
    CardArt::new("5aae919b-7da6-42b1-84b4-fbc2971dad1e", "Allen Williams"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
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
                "When this Aura enters, you lose 1 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// M13 87 — Diabolic Revelation
pub(in crate::card::sets) static DIABOLIC_REVELATION: CardRecord = CardRecord::new_with_legacy_id(
    1981,
    "Diabolic Revelation",
    CardArt::new("145d6d7b-1e87-47b7-baf3-d201458ad996", "Raymond Swanland"),
    CardSet::Magic2013,
    // "Up to X" with no qualifier on the cards: whatever X buys, it buys
    // exactly, and a smaller library is its own ceiling.
    CardRules::new_sorcery(mana_cost!("{X}{3}{B}{B}")).with_ability(AbilityDef::spell(
        "Search your library for up to X put those cards into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 0,
            maximum: ValueDef::ChosenX,
            reveal: false,
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

// M13 88 — Disciple of Bolas
pub(in crate::card::sets) static DISCIPLE_OF_BOLAS: CardRecord = CardRecord::new_with_legacy_id(
    154,
    "Disciple of Bolas",
    CardArt::new("c4dd57f8-27bc-4ad9-a79e-48a68af33b02", "Slawomir Maniak"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{3}{B}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_ability(abilities::enters_trigger("When this creature enters, sacrifice another creature. You gain X life and draw X where X is that creature's power.", EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                // "Another" creature, so the Disciple cannot eat itself.
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            // X is read off the sacrificed creature, so both halves take the power the
            // sacrifice recorded rather than counting anything on the board.
            then: Some(&EffectDef::Sequence(&[
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::TriggerEventAmount,
                },
            ])),
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        })),
);

// M13 89 — Disentomb
pub(in crate::card::sets) static DISENTOMB: CardRecord = CardRecord::new_with_legacy_id(
    1000,
    "Disentomb",
    CardArt::new(
        "ce7473bb-d092-4d76-b3c3-5036222dbdf7",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
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
    )),
);

// M13 90 — Duress
pub(in crate::card::sets) static DURESS: CardRecord = CardRecord::new_with_legacy_id(
    159,
    "Duress",
    CardArt::new("f7201d43-ae2e-4faa-a508-8555079c3bc7", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a noncreature, nonland card from it. That player discards that card.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            ]),
        )),
    )),
);

// M13 91 — Duskmantle Prowler
pub(in crate::card::sets) static DUSKMANTLE_PROWLER: CardRecord = CardRecord::new_with_legacy_id(
    1504,
    "Duskmantle Prowler",
    CardArt::new("bcb031da-d41a-496a-b78e-0773f6504303", "Johannes Voss"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Rogue"], 2, 2)
        .with_abilities(&[abilities::haste(), abilities::exalted()]),
);

// M13 92 — Duty-Bound Dead
pub(in crate::card::sets) static DUTY_BOUND_DEAD: CardRecord = CardRecord::new_with_legacy_id(
    1870,
    "Duty-Bound Dead",
    CardArt::new("5150aa90-1284-4261-8625-2528139f0015", "Johannes Voss"),
    CardSet::Magic2013,
    // A 0/2 that regenerates: exalted is what gives it something to do on
    // the attack, since alone it is the creature being pumped.
    CardRules::new_creature(mana_cost!("{B}"), &["Skeleton"], 0, 2).with_abilities(&[
        abilities::exalted(),
        abilities::regenerate_self(
            "{3}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{B}"))],
        ),
    ]),
);

// M13 93 — Essence Drain
pub(in crate::card::sets) static ESSENCE_DRAIN: CardRecord = CardRecord::new_with_legacy_id(
    1001,
    "Essence Drain",
    CardArt::new("58df0c6d-3fd2-4d87-81e2-6640e6e75985", "Jim Nelson"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Essence Drain deals 3 damage to any target and you gain 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )),
);

// M13 94 — Giant Scorpion
pub(in crate::card::sets) static GIANT_SCORPION: CardRecord = CardRecord::new_with_legacy_id(
    1002,
    "Giant Scorpion",
    CardArt::new("4097d5dc-46d3-4054-818f-a4ad8d7effe2", "Raymond Swanland"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Scorpion"], 1, 3)
        .with_abilities(&[abilities::deathtouch()]),
);

// M13 95 — Harbor Bandit
static HARBOR_BANDIT_ISLANDS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static HARBOR_BANDIT: CardRecord = CardRecord::new_with_legacy_id(
    1357,
    "Harbor Bandit",
    CardArt::new("8422e109-de8d-46ea-a7f8-d5ccb6340497", "Jesper Ejsing"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control an Island.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&HARBOR_BANDIT_ISLANDS),
                    ValueDef::AnyMatchingObject(&HARBOR_BANDIT_ISLANDS),
                ),
            },
        ),
        AbilityDef::activated(
            "{1}{U}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 96 — Knight of Infamy
pub(in crate::card::sets) static KNIGHT_OF_INFAMY: CardRecord = CardRecord::new_with_legacy_id(
    1590,
    "Knight of Infamy",
    CardArt::new("9e339853-5b6b-47b7-8d88-e9d3befb803f", "Peter Mohrbacher"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::White),
        abilities::exalted(),
    ]),
);

// M13 97 — Liliana of the Dark Realms
// Audit: metadata-only — Needs a choice between dynamic Swamp-count pump or shrink and an emblem that multiplies mana from Swamps.
pub(in crate::card::sets) static LILIANA_OF_THE_DARK_REALMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2cd2d81e-1388-4f34-9917-2289971cf8da"),
    "Liliana of the Dark Realms",
    crate::card::CardArt::new(
        "2cd2d81e-1388-4f34-9917-2289971cf8da",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 98 — Liliana's Shade
pub(in crate::card::sets) static LILIANAS_SHADE: CardRecord = CardRecord::new_with_legacy_id(
    1366,
    "Liliana's Shade",
    CardArt::new(
        "1cf0c01d-a4a0-43fb-970d-e428e9ac63d7",
        "Eric Deschamps",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Shade"], 1, 1).with_abilities(&[
        abilities::enters_trigger("When this creature enters, you may search your library for a Swamp card, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
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
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 99 — Mark of the Vampire
pub(in crate::card::sets) static MARK_OF_THE_VAMPIRE: CardRecord = CardRecord::new_with_legacy_id(
    1003,
    "Mark of the Vampire",
    CardArt::new("90484815-2529-4a81-9f1b-f0f7382e4b66", "Winona Nelson"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
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
                "Enchanted creature gets +2/+2 and has lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
        ]),
);

// M13 100 — Mind Rot
pub(in crate::card::sets) static MIND_ROT: CardRecord = CardRecord::new_with_legacy_id(
    1004,
    "Mind Rot",
    CardArt::new("ab454fb8-347f-4d4d-84bb-195c9d51b06b", "Steve Luke"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )),
);

// M13 101 — Murder
pub(in crate::card::sets) static MURDER: CardRecord = CardRecord::new_with_legacy_id(
    1005,
    "Murder",
    CardArt::new("c8676f02-cf1e-4d40-a0c5-6e5a97417898", "Allen Williams"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
        true,
    )),
);

// M13 102 — Mutilate
/// Mutilate scales with your Swamps, and reads the same count twice.
static SWAMPS_YOU_CONTROL: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Swamp"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
));

pub(in crate::card::sets) static MUTILATE: CardRecord = CardRecord::new_with_legacy_id(
    190,
    "Mutilate",
    CardArt::new("c48bc86b-df0a-4a9c-8aad-c3ffb742a5ff", "Tyler Jacobson"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -1/-1 until end of turn for each Swamp you control.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Negate(&SWAMPS_YOU_CONTROL),
                ValueDef::Negate(&SWAMPS_YOU_CONTROL),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// M13 103 — Nefarox, Overlord of Grixis
// Audit: metadata-only — Needs exalted's attacks-alone subject and the captured defending player for the sacrifice choice.
pub(in crate::card::sets) static NEFAROX_OVERLORD_OF_GRIXIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abc382f3-fdb9-4987-acf4-bf1ac4fd2ef7"),
    "Nefarox, Overlord of Grixis",
    crate::card::CardArt::new("abc382f3-fdb9-4987-acf4-bf1ac4fd2ef7", "Aleksi Briclot"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 104 — Phylactery Lich (reprint)

// M13 105 — Public Execution
// Audit: metadata-only — A target-relative creature sweep cannot exclude the destroyed target when destruction is prevented or replaced.
pub(in crate::card::sets) static PUBLIC_EXECUTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48188942-d0ba-4503-bd75-c7a5329bb7c8"),
    "Public Execution",
    crate::card::CardArt::new("48188942-d0ba-4503-bd75-c7a5329bb7c8", "Anthony Palumbo"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 106 — Ravenous Rats
pub(in crate::card::sets) static RAVENOUS_RATS: CardRecord = CardRecord::new_with_legacy_id(
    1006,
    "Ravenous Rats",
    CardArt::new("0642111c-f668-4acb-9df5-f0b920352407", "Carl Critchlow"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 1, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ),
);

// M13 107 — Rise from the Grave
pub(in crate::card::sets) static RISE_FROM_THE_GRAVE: CardRecord = CardRecord::new_with_legacy_id(
    2002,
    "Rise from the Grave",
    CardArt::new("5d2b187e-c489-4652-a638-390fc9ecef0e", "Vance Kovacs"),
    CardSet::Magic2013,
    // Any graveyard, so it steals as readily as it recurs.
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature card from a graveyard onto the battlefield under your control. That creature is a black Zombie in addition to its other colors and types.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::WithZoneMoveResult {
            effect: &EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                },
                arrival: crate::card::BattlefieldArrivalDef {
                    controller: Some(PlayerRelation::You),
                    ..crate::card::BattlefieldArrivalDef::DEFAULT
                },
            },
            binding: crate::ObjectSetBindingIndex::PRIMARY,
            then: &EffectDef::Apply {
                recipient: EffectRecipientDef::binding_zone_change_successors(
                    crate::ObjectSetBindingIndex::PRIMARY,
                ),
                // "In addition to its other colors and types", so both leaves add rather
                // than set.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::Black])),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Zombie"])),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        },
    )),
);

// M13 108 — Servant of Nefarox
pub(in crate::card::sets) static SERVANT_OF_NEFAROX: CardRecord = CardRecord::new_with_legacy_id(
    1505,
    "Servant of Nefarox",
    CardArt::new("e00a2b22-a473-44ae-919f-29bc8be05543", "Igor Kieryluk"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Cleric"], 3, 1)
        .with_ability(abilities::exalted()),
);

// M13 109 — Shimian Specter (reprint)

// M13 110 — Sign in Blood
pub(in crate::card::sets) static SIGN_IN_BLOOD: CardRecord = CardRecord::new_with_legacy_id(
    213,
    "Sign in Blood",
    CardArt::new("64f6600b-36c4-43bd-8c01-cfbca402ecd6", "Howard Lyon"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player draws two cards and loses 2 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// M13 111 — Tormented Soul
pub(in crate::card::sets) static TORMENTED_SOUL: CardRecord = CardRecord::new_with_legacy_id(
    1752,
    "Tormented Soul",
    CardArt::new("e7a27749-350e-4c8a-8ff3-52539a5ec418", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{B}"), &["Spirit"], 1, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't block and can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // The two halves point opposite ways: one keeps it out of blocks it would
                // join, the other out of blocks it would be caught by.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                ]),
            },
        ),
    ),
);

// M13 112 — Vampire Nighthawk
pub(in crate::card::sets) static VAMPIRE_NIGHTHAWK: CardRecord = CardRecord::new_with_legacy_id(
    236,
    "Vampire Nighthawk",
    CardArt::new("9ba96d96-8d9e-47c8-ab39-17479564aadf", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Vampire", "Shaman"], 2, 3).with_abilities(
        &[
            abilities::flying(),
            abilities::deathtouch(),
            abilities::lifelink(),
        ],
    ),
);

// M13 113 — Vampire Nocturnus
// Audit: metadata-only — Needs persistent top-library revelation and a top-card-color-conditioned Vampire mass bonus and flying grant.
pub(in crate::card::sets) static VAMPIRE_NOCTURNUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3194ae81-90fb-49e9-90de-9d161e296770"),
    "Vampire Nocturnus",
    crate::card::CardArt::new("8daccbbb-6600-4467-810f-277f01a11771", "Raymond Swanland"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 114 — Veilborn Ghoul
// Audit: metadata-only — Needs a land-entry trigger functioning from this card's graveyard and a self return from that zone.
pub(in crate::card::sets) static VEILBORN_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3f49232-2853-427f-8c20-322e09a3ccde"),
    "Veilborn Ghoul",
    crate::card::CardArt::new("d3f49232-2853-427f-8c20-322e09a3ccde", "Dan Murayama Scott"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 115 — Vile Rebirth
pub(in crate::card::sets) static VILE_REBIRTH: CardRecord = CardRecord::new_with_legacy_id(
    1007,
    "Vile Rebirth",
    CardArt::new("965b5a48-d0ff-47ce-b44e-a1611fab1876", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature card from a graveyard. Create a 2/2 black Zombie creature token.",
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
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                CardArt::new("1966d7e6-cd4a-47ff-bc3e-f8e0db8a3439", "Lucas Graciano"),
            ),
        ]),
    )),
);

// M13 116 — Walking Corpse (reprint)

// M13 117 — Wit's End
pub(in crate::card::sets) static WITS_END: CardRecord = CardRecord::new_with_legacy_id(
    1358,
    "Wit's End",
    CardArt::new("71298c75-533e-4ccd-a1f5-875f63a1e89b", "Chris Rahn"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{5}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards their hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(i32::MAX),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )),
);

// M13 118 — Xathrid Gorgon
// Audit: metadata-only — Needs a resolving counter choice plus type, color, ability-removal, and defender changes keyed to that counter.
pub(in crate::card::sets) static XATHRID_GORGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abfeac83-84ef-41ab-b6ee-cc2bab6aa06d"),
    "Xathrid Gorgon",
    crate::card::CardArt::new("e07524e0-303d-465d-b112-ca605b9b27fc", "Chase Stone"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 119 — Zombie Goliath
pub(in crate::card::sets) static ZOMBIE_GOLIATH: CardRecord = CardRecord::new_with_legacy_id(
    1008,
    "Zombie Goliath",
    CardArt::new("8638edec-ddcd-4f50-9c2f-2e1668e3d175", "E. M. Gist"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie", "Giant"], 4, 3),
);

// M13 120 — Arms Dealer
pub(in crate::card::sets) static ARMS_DEALER: CardRecord = CardRecord::new_with_legacy_id(
    1359,
    "Arms Dealer",
    CardArt::new("910d3c33-8cda-487b-8b44-87a9d06d6749", "Wayne Reynolds"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{R}, Sacrifice a Goblin: This creature deals 4 damage to target creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Goblin"),
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
        ),
    ),
);

// M13 121 — Bladetusk Boar
pub(in crate::card::sets) static BLADETUSK_BOAR: CardRecord = CardRecord::new_with_legacy_id(
    1009,
    "Bladetusk Boar",
    CardArt::new("d28442f9-06cf-4273-80a3-2b054f5881a4", "Paul Bonner"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Boar"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// M13 122 — Canyon Minotaur
pub(in crate::card::sets) static CANYON_MINOTAUR: CardRecord = CardRecord::new_with_legacy_id(
    1010,
    "Canyon Minotaur",
    CardArt::new("f8dc0efb-5847-4061-b386-9b4099361a58", "Steve Prescott"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur", "Warrior"], 3, 3),
);

// M13 123 — Chandra, the Firebrand
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHANDRA_THE_FIREBRAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efb37556-186f-4660-8b75-c52ef16a6d8f"),
    "Chandra, the Firebrand",
    crate::card::CardArt::new(
        "beb039db-7367-4af1-8d85-4951f58e2732",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 124 — Chandra's Fury
pub(in crate::card::sets) static CHANDRAS_FURY: CardRecord = CardRecord::new_with_legacy_id(
    1011,
    "Chandra's Fury",
    CardArt::new("25335fee-d320-4622-bcf4-292400dee52b", "Volkan Baǵa"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{4}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Chandra's Fury deals 4 damage to target player or planeswalker and 1 damage to each creature that player or that planeswalker's controller controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::objects_controlled_by_target(ObjectPredicateDef::HasType(CardType::Creature), TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// M13 125 — Cleaver Riot
pub(in crate::card::sets) static CLEAVER_RIOT: CardRecord = CardRecord::new_with_legacy_id(
    1012,
    "Cleaver Riot",
    CardArt::new("6761eacf-03fc-4ccd-a4a6-eca5357b5c5b", "Dave Kendall"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell(
        "Creatures you control gain double strike until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 126 — Craterize
pub(in crate::card::sets) static CRATERIZE: CardRecord = CardRecord::new_with_legacy_id(
    1013,
    "Craterize",
    CardArt::new("e5459409-5103-4a97-a6fb-3e3ab896eb66", "Eytan Zana"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target land.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land)),
        true,
    )),
);

// M13 127 — Crimson Muckwader
static CRIMSON_MUCKWADER_SWAMPS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static CRIMSON_MUCKWADER: CardRecord = CardRecord::new_with_legacy_id(
    1489,
    "Crimson Muckwader",
    CardArt::new("a0811f91-ed92-4a8e-badd-ae5054e7707d", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Lizard"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Swamp.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&CRIMSON_MUCKWADER_SWAMPS),
                    ValueDef::AnyMatchingObject(&CRIMSON_MUCKWADER_SWAMPS),
                ),
            },
        ),
        abilities::regenerate_self(
            "{2}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{B}"))],
        ),
    ]),
);

// M13 128 — Dragon Hatchling
pub(in crate::card::sets) static DRAGON_HATCHLING: CardRecord = CardRecord::new_with_legacy_id(
    1014,
    "Dragon Hatchling",
    CardArt::new("ed599d52-f2d9-4913-ad88-70f8aa4af7b9", "David Palumbo"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dragon"], 0, 1).with_abilities(&[
        abilities::flying(),
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

// M13 129 — Fervor
pub(in crate::card::sets) static FERVOR: CardRecord = CardRecord::new_with_legacy_id(
    1015,
    "Fervor",
    CardArt::new("a88515c2-4b4f-4d16-9f50-149ef012e961", "Wayne England"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have haste.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::haste()),
        },
    )),
);

// M13 130 — Fire Elemental (reprint)

// M13 131 — Firewing Phoenix
// Audit: metadata-only — Effect recipients cannot identify an activated ability's own source card while it is in a graveyard.
pub(in crate::card::sets) static FIREWING_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8824674-ced2-448e-9bf0-03c1c43a5315"),
    "Firewing Phoenix",
    crate::card::CardArt::new("b8824674-ced2-448e-9bf0-03c1c43a5315", "James Paick"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 132 — Flames of the Firebrand
pub(in crate::card::sets) static FLAMES_OF_THE_FIREBRAND: CardRecord = CardRecord::new_with_legacy_id(
    163,
    "Flames of the Firebrand",
    CardArt::new("aca215b1-7b98-49ce-afae-eeb61058125a", "Steve Argyle"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Flames of the Firebrand deals 3 damage divided as you choose among one, two, or three targets.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::AnyTarget,
                // One, two, or three targets is not a separate rule: three damage
                // split with every share at least one says the same thing.
                minimum: 1,
                maximum: 3,
                exact_count: None,
                divided_total: Some(DividedTotal::Fixed(3)),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ),
);

// M13 133 — Furnace Whelp
pub(in crate::card::sets) static FURNACE_WHELP: CardRecord = CardRecord::new_with_legacy_id(
    1016,
    "Furnace Whelp",
    CardArt::new("41e73d9c-8c17-4c3c-b535-e21f03e577bc", "Matt Cavotta"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 2, 2).with_abilities(&[
        abilities::flying(),
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

// M13 134 — Goblin Arsonist
pub(in crate::card::sets) static GOBLIN_ARSONIST: CardRecord = CardRecord::new_with_legacy_id(
    1017,
    "Goblin Arsonist",
    CardArt::new("4d131369-db00-4a11-bd47-4401188b0f35", "Wayne Reynolds"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Shaman"], 1, 1).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, you may have it deal 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ),
);

// M13 135 — Goblin Battle Jester
pub(in crate::card::sets) static GOBLIN_BATTLE_JESTER: CardRecord = CardRecord::new_with_legacy_id(
    1873,
    "Goblin Battle Jester",
    CardArt::new("c13e56b0-becc-4bc2-9ba3-23b3ca8bfe58", "Steve Prescott"),
    CardSet::Magic2013,
    // The Jester itself is a red spell, but the trigger watches casts rather
    // than arrivals, so casting it does not fire its own ability.
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever you cast a red spell, target creature can't block this turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Color(ManaColor::Red),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M13 136 — Hamletback Goliath
pub(in crate::card::sets) static HAMLETBACK_GOLIATH: CardRecord = CardRecord::new_with_legacy_id(
    1871,
    "Hamletback Goliath",
    CardArt::new(
        "01ddeef1-f6f9-48c0-a93c-7bb3877c0e59",
        "Paolo Parente & Brian Snõddy",
    ),
    CardSet::Magic2013,
    // "Another creature", with no controller clause: the opponent's arrivals
    // feed it too, which is what makes it worth its cost.
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Giant", "Warrior"], 6, 6).with_ability(
        AbilityDef::triggered(
            "Whenever another creature enters, you may put X +1/+1 counters on this creature, \
             where X is that creature's power.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::TriggeringObjectPower,
                },
            },
        ),
    ),
);

// M13 137 — Kindled Fury
pub(in crate::card::sets) static KINDLED_FURY: CardRecord = CardRecord::new_with_legacy_id(
    1018,
    "Kindled Fury",
    CardArt::new("35494897-b72b-46c4-8b36-b3b8865559bd", "Wayne Reynolds"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains first strike until end of turn.",
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
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 138 — Krenko, Mob Boss
pub(in crate::card::sets) static KRENKO_MOB_BOSS: CardRecord = CardRecord::new_with_legacy_id(
    1019,
    "Krenko, Mob Boss",
    CardArt::new("aa078518-0ce2-4c6f-9061-aa7e22ed7493", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{2}{R}{R}"),
        &["Goblin", "Warrior"],
        3,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_ability(AbilityDef::activated(
        "{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.",
        &[AbilityCostDef::TapSource],
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).with_art(CardArt::new("0e67efea-8a80-42ec-8e77-07d387d933d4", "Karl Kopinski")).with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype("Goblin"),
            ]),
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ))),
    )),
);

// M13 139 — Krenko's Command
pub(in crate::card::sets) static KRENKOS_COMMAND: CardRecord = CardRecord::new_with_legacy_id(
    1020,
    "Krenko's Command",
    CardArt::new("84df41e9-e973-4441-b17f-434517134d46", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Create two 1/1 red Goblin creature tokens.",
        EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
            .with_art(CardArt::new(
                "0e67efea-8a80-42ec-8e77-07d387d933d4",
                "Karl Kopinski",
            ))
            .with_amount(2),
    )),
);

// M13 140 — Magmaquake
pub(in crate::card::sets) static MAGMAQUAKE: CardRecord = CardRecord::new_with_legacy_id(
    1642,
    "Magmaquake",
    CardArt::new("ac85679e-17c7-4525-8eed-979d04feb8f1", "Gabor Szikszai"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{X}{R}{R}")).with_ability(AbilityDef::spell(
        "Magmaquake deals X damage to each creature without flying and each planeswalker.",
        // One sweep rather than two, so a permanent that is both a nonflying
        // creature and a planeswalker is dealt X once.
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M13 141 — Mark of Mutiny
pub(in crate::card::sets) static MARK_OF_MUTINY: CardRecord = CardRecord::new_with_legacy_id(
    1021,
    "Mark of Mutiny",
    CardArt::new("0b7c6e09-3a14-4cc4-ba6b-f1f45e7d9f2a", "Mike Bierek"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Put a +1/+1 counter on it and untap it. That creature gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::UntilEndOfTurn,
                controller: PlayerRefDef::EffectController,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
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
    )),
);

// M13 142 — Mindclaw Shaman
// Audit: metadata-only — Needs an opponent-hand reveal and choice followed by permission to cast the chosen card without paying its cost.
pub(in crate::card::sets) static MINDCLAW_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f342fe9-aa73-4222-b908-d4035b5746be"),
    "Mindclaw Shaman",
    crate::card::CardArt::new("0f342fe9-aa73-4222-b908-d4035b5746be", "Slawomir Maniak"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 143 — Mogg Flunkies (reprint)

// M13 144 — Reckless Brute
pub(in crate::card::sets) static RECKLESS_BRUTE: CardRecord = CardRecord::new_with_legacy_id(
    1022,
    "Reckless Brute",
    CardArt::new("5fd32a9e-1d39-4792-9657-69d17e5e0134", "Johann Bodin"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ogre", "Warrior"], 3, 1).with_abilities(&[
        abilities::haste(),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    ]),
);

// M13 145 — Reverberate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REVERBERATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd435013-0ab9-42f4-985c-66ea2b3760e9"),
    "Reverberate",
    crate::card::CardArt::new("5996feb4-02ac-45e8-a7f2-966cf74391dc", "jD"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 146 — Rummaging Goblin
pub(in crate::card::sets) static RUMMAGING_GOBLIN: CardRecord = CardRecord::new_with_legacy_id(
    1023,
    "Rummaging Goblin",
    CardArt::new("cc5b622c-83a4-477e-a99c-2674e2bd6bb9", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}, Discard a card: Draw a card.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 147 — Searing Spear
pub(in crate::card::sets) static SEARING_SPEAR: CardRecord = CardRecord::new_with_legacy_id(
    1024,
    "Searing Spear",
    CardArt::new("11a94b7c-0216-473c-87a6-71e5a64d7799", "Chris Rahn"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Searing Spear deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )),
);

// M13 148 — Slumbering Dragon
// Audit: metadata-only — Needs attack and block permission based on counter count plus an attack-at-you event that adds counters.
pub(in crate::card::sets) static SLUMBERING_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("277cbd0d-c8da-4a37-965c-6a60771df2f7"),
    "Slumbering Dragon",
    crate::card::CardArt::new("277cbd0d-c8da-4a37-965c-6a60771df2f7", "Chris Rahn"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 149 — Smelt
pub(in crate::card::sets) static SMELT: CardRecord = CardRecord::new_with_legacy_id(
    1025,
    "Smelt",
    CardArt::new("723cb7e3-3f48-41fa-aa08-bdc59225e44f", "Zoltan Boros"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::destroy_target(
        "Destroy target artifact.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Artifact)),
        true,
    )),
);

// M13 150 — Thundermaw Hellkite
/// The damage and the tap name the same creatures, so both clauses ask the
/// same question.
const OPPOSING_FLIERS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
]);

pub(in crate::card::sets) static THUNDERMAW_HELLKITE: CardRecord = CardRecord::new_with_legacy_id(
    228,
    "Thundermaw Hellkite",
    CardArt::new("d0476e0f-61df-46a6-aaf1-8ee79c701160", "Svetlin Velinov"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{3}{R}{R}"),
        &["Dragon"],
        5,
        5,
    )
    .with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        abilities::enters_trigger("When this creature enters, it deals 1 damage to each creature with flying your opponents control. Tap those creatures.", EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::matching_objects(OPPOSING_FLIERS, &[ZoneKind::Battlefield], PlayerRelation::Opponent),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::matching_objects(OPPOSING_FLIERS, &[ZoneKind::Battlefield], PlayerRelation::Opponent),
                },
            ])),
    ]),
);

// M13 151 — Torch Fiend (reprint)

// M13 152 — Trumpet Blast
pub(in crate::card::sets) static TRUMPET_BLAST: CardRecord = CardRecord::new_with_legacy_id(
    1026,
    "Trumpet Blast",
    CardArt::new("4ac9f745-236a-4302-acf2-21c14c6e6eab", "Carl Critchlow"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Attacking creatures get +2/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 153 — Turn to Slag
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TURN_TO_SLAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66fd5b49-b4f2-40da-94d5-6d6fc69506f6"),
    "Turn to Slag",
    crate::card::CardArt::new(
        "7275ede4-22d6-41db-91e9-3b0295abb8a9",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 154 — Volcanic Geyser
pub(in crate::card::sets) static VOLCANIC_GEYSER: CardRecord = CardRecord::new_with_legacy_id(
    1027,
    "Volcanic Geyser",
    CardArt::new("df5bab70-3c28-48db-9ed3-64706f64f4fa", "Clint Cearley"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{X}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Volcanic Geyser deals X damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M13 155 — Volcanic Strength
pub(in crate::card::sets) static VOLCANIC_STRENGTH: CardRecord = CardRecord::new_with_legacy_id(
    239,
    "Volcanic Strength",
    CardArt::new("f1963f08-1765-4f3e-92be-479773de47a0", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
        AbilityDef::spell_with_targets("Enchant creature", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has mountainwalk. (It can't be blocked as long as defending player controls a Mountain.)",
            EffectDef::Sequence(&[
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                },
            ]),
        ),
    ]),
);

// M13 156 — Wall of Fire (reprint)

// M13 157 — Wild Guess
pub(in crate::card::sets) static WILD_GUESS: CardRecord = CardRecord::new_with_legacy_id(
    1608,
    "Wild Guess",
    CardArt::new("a4e513b8-25c2-4645-abcc-a6e9d5f51e09", "Lucas Graciano"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{R}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a card.\nDraw two cards.",
            &[],
            SpellAdditionalCostDef::discard(ObjectPredicateDef::Any, CostQuantityDef::Fixed(1)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// M13 158 — Worldfire
pub(in crate::card::sets) static WORLDFIRE: CardRecord = CardRecord::new_with_legacy_id(
    1696,
    "Worldfire",
    CardArt::new("2ef3d4b5-0453-4bf0-b018-23b0c3b9ae11", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{6}{R}{R}{R}")).with_ability(AbilityDef::spell(
        "Exile all permanents. Exile all cards from all hands and graveyards. Each player's life total becomes 1.",
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand, ZoneKind::Graveyard],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::SetLifeTotal {
                recipient: EffectRecipientDef::players(PlayerSetDef::All),
                total: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M13 159 — Acidic Slime
pub(in crate::card::sets) static ACIDIC_SLIME: CardRecord = CardRecord::new_with_legacy_id(
    1028,
    "Acidic Slime",
    CardArt::new("bd7bef5a-e0ab-46d3-a802-620bf2a7546f", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Ooze"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target artifact, enchantment, or land.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// M13 160 — Arbor Elf
pub(in crate::card::sets) static ARBOR_ELF: CardRecord = CardRecord::new_with_legacy_id(
    132,
    "Arbor Elf",
    CardArt::new("b7d6b117-0c14-4455-92fc-29555ee75d97", "rk post"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Untap target Forest.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype("Forest"),
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

// M13 161 — Bond Beetle
pub(in crate::card::sets) static BOND_BEETLE: CardRecord = CardRecord::new_with_legacy_id(
    1029,
    "Bond Beetle",
    CardArt::new("f341ed2c-353b-49a3-b200-94ae43cb8e24", "John Avon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{G}"), &["Insect"], 0, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 162 — Boundless Realms
pub(in crate::card::sets) static BOUNDLESS_REALMS: CardRecord = CardRecord::new_with_legacy_id(
    1982,
    "Boundless Realms",
    CardArt::new("e3c3cf16-ba81-4558-b1a6-79942a02f629", "Cliff Childs"),
    CardSet::Magic2013,
    // It doubles the lands you have, so the seven it costs is measured
    // against a board that is already large.
    CardRules::new_sorcery(mana_cost!("{6}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to X basic land where X is the number of lands you control, put them onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            // The lands you already control, which is what the search is sized by.
            maximum: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
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
);

// M13 163 — Bountiful Harvest
pub(in crate::card::sets) static BOUNTIFUL_HARVEST: CardRecord = CardRecord::new_with_legacy_id(
    1030,
    "Bountiful Harvest",
    CardArt::new("8d7a4494-2ced-4405-9204-d2617961a1d6", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{4}{G}")).with_ability(AbilityDef::spell(
        "You gain 1 life for each land you control.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// M13 164 — Centaur Courser
pub(in crate::card::sets) static CENTAUR_COURSER: CardRecord = CardRecord::new_with_legacy_id(
    1031,
    "Centaur Courser",
    CardArt::new("44a5f7db-ea4e-4af5-9d4a-0335db6ea0e9", "Vance Kovacs"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Centaur", "Warrior"], 3, 3),
);

// M13 165 — Deadly Recluse
pub(in crate::card::sets) static DEADLY_RECLUSE: CardRecord = CardRecord::new_with_legacy_id(
    1032,
    "Deadly Recluse",
    CardArt::new("a32a5f77-7c1f-4da4-9ae6-3947504a8dea", "Warren Mahy"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider"], 1, 2)
        .with_abilities(&[abilities::reach(), abilities::deathtouch()]),
);

// M13 166 — Duskdale Wurm
pub(in crate::card::sets) static DUSKDALE_WURM: CardRecord = CardRecord::new_with_legacy_id(
    1033,
    "Duskdale Wurm",
    CardArt::new("7d1a2d9a-e14c-4c44-8cf1-a2ce09bdae27", "Dan Dos Santos"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Wurm"], 7, 7)
        .with_abilities(&[abilities::trample()]),
);

// M13 167 — Elderscale Wurm
// Audit: metadata-only — Needs conditional life-total setting on entry and a damage replacement that enforces a life floor.
pub(in crate::card::sets) static ELDERSCALE_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20f3f63d-0f04-4945-9895-940c916a2547"),
    "Elderscale Wurm",
    crate::card::CardArt::new("20f3f63d-0f04-4945-9895-940c916a2547", "Richard Wright"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 168 — Elvish Archdruid
pub(in crate::card::sets) static ELVISH_ARCHDRUID: CardRecord = CardRecord::new_with_legacy_id(
    1872,
    "Elvish Archdruid",
    CardArt::new("bf8eba57-8c51-490b-995f-53eeb7ad574f", "Karl Kopinski"),
    CardSet::Magic2013,
    // The count includes the Archdruid itself, which is an Elf: a lone one
    // taps for a single green rather than none.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elf", "Druid"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "Other Elf creatures you control get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Elf"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
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
        AbilityDef::activated_mana(
            "{T}: Add {G} for each Elf you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Green,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype("Elf"),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        ),
    ]),
);

// M13 169 — Elvish Visionary
pub(in crate::card::sets) static ELVISH_VISIONARY: CardRecord = CardRecord::new_with_legacy_id(
    1034,
    "Elvish Visionary",
    CardArt::new(
        "65ea2998-ed91-43b8-bd81-b01a6c24a5b0",
        "D. Alexander Gregory",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Shaman"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 170 — Farseek
pub(in crate::card::sets) static FARSEEK: CardRecord = CardRecord::new_with_legacy_id(
    1697,
    "Farseek",
    CardArt::new("f9b69d33-96dd-4844-aefa-27a885cb2ffc", "Martina Pilcerova"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Search your library for a Plains, Island, Swamp, or Mountain card, put it onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Plains,
                BasicLandType::Island,
                BasicLandType::Swamp,
                BasicLandType::Mountain,
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M13 171 — Flinthoof Boar
/// A second Mountain does not make the bonus bigger, so this is asked as a
/// condition rather than counted.
static MOUNTAIN_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Mountain"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static FLINTHOOF_BOAR: CardRecord = CardRecord::new_with_legacy_id(
    164,
    "Flinthoof Boar",
    CardArt::new("7e380b99-0173-4083-a4a2-222ad98b904a", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Boar"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Mountain.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                    ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                ),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gains haste until end of turn. (It can attack and {T} this turn.)",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 172 — Fog (reprint)

// M13 173 — Fungal Sprouting
pub(in crate::card::sets) static FUNGAL_SPROUTING: CardRecord = CardRecord::new_with_legacy_id(
    1979,
    "Fungal Sprouting",
    CardArt::new("97413ae3-037e-4786-85a3-e92604acd771", "Brad Rigney"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Create X 1/1 green Saproling creature tokens, where X is the greatest power among creatures you control.",
        EffectDef::create_creature_token(&["Saproling"], &[ManaColor::Green], 1, 1).with_art(CardArt::new("dd67de8a-3879-4d03-a716-6e907d597b25", "Brad Rigney")).with_count(abilities::greatest_power_you_control()),
    )),
);

// M13 174 — Garruk, Primal Hunter
pub(in crate::card::sets) static GARRUK_PRIMAL_HUNTER: CardRecord = CardRecord::new_with_legacy_id(
    1698,
    "Garruk, Primal Hunter",
    CardArt::new(
        "9945307b-d49d-4d21-bba0-2aebba68d57a",
        "D. Alexander Gregory",
    ),
    CardSet::Magic2013,
    CardRules::new_planeswalker(mana_cost!("{2}{G}{G}{G}"), &["Garruk"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Create a 3/3 green Beast creature token.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::create_creature_token(&["Beast"], &[ManaColor::Green], 3, 3).with_art(
                    CardArt::new("c94010f1-cd4b-4f65-8a0e-2df6eec058ec", "John Donahue"),
                ),
            ),
            AbilityDef::activated(
                "−3: Draw cards equal to the greatest power among creatures you control.",
                &[AbilityCostDef::Loyalty(-3)],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: abilities::greatest_power_you_control(),
                },
            ),
            AbilityDef::activated(
                "−6: Create a 6/6 green Wurm creature token for each land you control.",
                &[AbilityCostDef::Loyalty(-6)],
                EffectDef::create_creature_token(&["Wurm"], &[ManaColor::Green], 6, 6)
                    .with_art(CardArt::new(
                        "a4d87f38-c342-4186-8768-c3f1aceb680a",
                        "Anthony Francisco",
                    ))
                    .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ))),
            ),
        ]),
);

// M13 175 — Garruk's Packleader (reprint)

// M13 176 — Ground Seal (reprint)

// M13 177 — Mwonvuli Beast Tracker
pub(in crate::card::sets) static MWONVULI_BEAST_TRACKER: CardRecord = CardRecord::new_with_legacy_id(
    1643,
    "Mwonvuli Beast Tracker",
    CardArt::new("0034d32c-cc82-48d7-a913-d58cc3d3afeb", "Zoltan Boros"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human", "Scout"], 2, 1).with_ability(
        abilities::enters_trigger("When this creature enters, search your library for a creature card with deathtouch, hexproof, reach, or trample, reveal it, then shuffle your library and put that card on top of it.", EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Deathtouch),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Hexproof),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Reach),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Trample),
                    ]),
                ]),
                minimum: 1,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Library,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            }),
    ),
);

// M13 178 — Naturalize (reprint)

// M13 179 — Plummet
pub(in crate::card::sets) static PLUMMET: CardRecord = CardRecord::new_with_legacy_id(
    1644,
    "Plummet",
    CardArt::new("a96d7d96-5a86-45ef-a30b-b11ece22f060", "Pete Venters"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with flying.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// M13 180 — Predatory Rampage
pub(in crate::card::sets) static PREDATORY_RAMPAGE: CardRecord = CardRecord::new_with_legacy_id(
    1874,
    "Predatory Rampage",
    CardArt::new("3e054ea5-3657-4198-9715-6acc0e362da3", "Wayne England"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{G}{G}")).with_ability(AbilityDef::spell(
        "Creatures you control get +3/+3 until end of turn. Each creature your opponents \
         control blocks this turn if able.",
        // The two clauses point at opposite boards, which is the whole card: yours
        // get bigger and theirs are forced in front of them.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBlockEachAttackerIfAble),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// M13 181 — Prey Upon (reprint)

// M13 182 — Primal Huntbeast
pub(in crate::card::sets) static PRIMAL_HUNTBEAST: CardRecord = CardRecord::new_with_legacy_id(
    1035,
    "Primal Huntbeast",
    CardArt::new("eb77f6a8-a9d6-4fdd-996e-70877199ebab", "Chris Rahn"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 3, 3)
        .with_abilities(&[abilities::hexproof()]),
);

// M13 183 — Primordial Hydra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMORDIAL_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3dcc5521-df8f-4992-b93e-e430d8cc7715"),
    "Primordial Hydra",
    crate::card::CardArt::new("937deb52-8888-4298-9ae5-0361c6fdbba2", "Aleksi Briclot"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 184 — Quirion Dryad (reprint)

// M13 185 — Rancor (reprint)

// M13 186 — Ranger's Path
pub(in crate::card::sets) static RANGERS_PATH: CardRecord = CardRecord::new_with_legacy_id(
    1699,
    "Ranger's Path",
    CardArt::new("26858a53-1054-407a-b2a2-34a7c4ae0f10", "Tomasz Jedruszek"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to two Forest cards, put them onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            minimum: 0,
            maximum: ValueDef::Constant(2),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: true,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// M13 187 — Revive
pub(in crate::card::sets) static REVIVE: CardRecord = CardRecord::new_with_legacy_id(
    1036,
    "Revive",
    CardArt::new("3a9aae03-f29b-4da6-a0cb-edd67bb111f5", "Matthew D. Wilson"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Return target green card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Color(ManaColor::Green),
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
    )),
);

// M13 188 — Roaring Primadox
// Audit: metadata-only — No resolving choice selects a nontarget creature the controller owns for a mandatory return.
pub(in crate::card::sets) static ROARING_PRIMADOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19529b2f-03f0-469d-92d4-e2a2a933d5dc"),
    "Roaring Primadox",
    crate::card::CardArt::new("19529b2f-03f0-469d-92d4-e2a2a933d5dc", "James Ryman"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 189 — Sentinel Spider
pub(in crate::card::sets) static SENTINEL_SPIDER: CardRecord = CardRecord::new_with_legacy_id(
    1037,
    "Sentinel Spider",
    CardArt::new("5f55ff4b-f0e1-498b-982b-e6ec01d30d95", "Vincent Proce"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Spider"], 4, 4)
        .with_abilities(&[abilities::reach(), abilities::vigilance()]),
);

// M13 190 — Serpent's Gift
pub(in crate::card::sets) static SERPENTS_GIFT: CardRecord = CardRecord::new_with_legacy_id(
    1038,
    "Serpent's Gift",
    CardArt::new("0e27503e-059e-4c44-a817-678e67254111", "Steve Argyle"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains deathtouch until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 191 — Silklash Spider
pub(in crate::card::sets) static SILKLASH_SPIDER: CardRecord = CardRecord::new_with_legacy_id(
    1645,
    "Silklash Spider",
    CardArt::new("359d1bb9-dbfd-4094-bda0-9a19817ce4bc", "Iain McCaig"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Spider"], 2, 7).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated(
            "{X}{G}{G}: This creature deals X damage to each creature with flying.",
            &[AbilityCostDef::Mana(mana_cost!("{X}{G}{G}"))],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::ChosenX,
            },
        ),
    ]),
);

// M13 192 — Spiked Baloth
pub(in crate::card::sets) static SPIKED_BALOTH: CardRecord = CardRecord::new_with_legacy_id(
    1039,
    "Spiked Baloth",
    CardArt::new("522777b1-a89f-4969-a962-0137018ec86c", "Daarken"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 4, 2)
        .with_abilities(&[abilities::trample()]),
);

// M13 193 — Thragtusk
pub(in crate::card::sets) static THRAGTUSK: CardRecord = CardRecord::new_with_legacy_id(
    227,
    "Thragtusk",
    CardArt::new("28667c8b-d02c-4e57-a050-1549207b65d1", "Nils Hamm"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 5, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, you gain 5 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, create a 3/3 green Beast creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::create_creature_token(&["Beast"], &[ManaColor::Green], 3, 3).with_art(
                CardArt::new("c94010f1-cd4b-4f65-8a0e-2df6eec058ec", "John Donahue"),
            ),
        ),
    ]),
);

// M13 194 — Timberpack Wolf
static OTHER_TIMBERPACK_WOLVES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasName(ObjectRefDef::Source),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static TIMBERPACK_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    1040,
    "Timberpack Wolf",
    CardArt::new("d16928c9-0470-46ec-b92d-0d6ff9f23ef7", "John Avon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each other creature you control named Timberpack Wolf.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&OTHER_TIMBERPACK_WOLVES),
                    ValueDef::CountMatchingObjects(&OTHER_TIMBERPACK_WOLVES),
                ),
            },
        ),
    ),
);

// M13 195 — Titanic Growth
pub(in crate::card::sets) static TITANIC_GROWTH: CardRecord = CardRecord::new_with_legacy_id(
    1041,
    "Titanic Growth",
    CardArt::new("5f1fb9f8-c070-40c9-89cd-c74eb8dbbf1a", "Ryan Pancoast"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +4/+4 until end of turn.",
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
    )),
);

// M13 196 — Vastwood Gorger
pub(in crate::card::sets) static VASTWOOD_GORGER: CardRecord = CardRecord::new_with_legacy_id(
    1042,
    "Vastwood Gorger",
    CardArt::new("70fc4a5f-1c59-4139-a506-72baebb1168f", "Kieran Yanner"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Wurm"], 5, 6),
);

// M13 197 — Yeva, Nature's Herald
// Audit: metadata-only — No static permission grants flash-like casting timing to green creature cards.
pub(in crate::card::sets) static YEVA_NATURE_S_HERALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80acb6dc-a9bd-4f12-9025-623416bdfc32"),
    "Yeva, Nature's Herald",
    crate::card::CardArt::new("80acb6dc-a9bd-4f12-9025-623416bdfc32", "Eric Deschamps"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 198 — Yeva's Forcemage
pub(in crate::card::sets) static YEVAS_FORCEMAGE: CardRecord = CardRecord::new_with_legacy_id(
    1043,
    "Yeva's Forcemage",
    CardArt::new("3f9ebf02-56b3-492e-88fb-2e95f13f5764", "Eric Deschamps"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature gets +2/+2 until end of turn.",
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
    ),
);

// M13 199 — Nicol Bolas, Planeswalker
pub(in crate::card::sets) static NICOL_BOLAS_PLANESWALKER: CardRecord = CardRecord::new_with_legacy_id(
    1700,
    "Nicol Bolas, Planeswalker",
    CardArt::new("0e3b1fea-5c2c-4848-8109-548f56b99d49", "D. Alexander Gregory"),
    CardSet::Magic2013,
    CardRules::new_planeswalker(mana_cost!("{4}{U}{B}{B}{R}"), &["Bolas"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "+3: Destroy target noncreature permanent.",
                &[AbilityCostDef::Loyalty(3)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::activated_with_targets(
                "−2: Gain control of target creature.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))],
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::Indefinitely,
                },
            ),
            AbilityDef::activated_with_targets(
                "−9: Nicol Bolas deals 7 damage to target player or planeswalker. That player or that planeswalker's controller discards seven then sacrifices seven permanents of their choice.",
                &[AbilityCostDef::Loyalty(-9)],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any))],
                EffectDef::Sequence(&[
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(7),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(7),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Any,
                            bound: None,
                            effect: &EffectDef::SacrificeOfChoice {
                                player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                                object: ObjectPredicateDef::Any,
                                count: ValueDef::Constant(7),
                                then: None,
                                amount: SacrificedAmountDef::Power,
                                otherwise: None,
                                optional: false,
                            },
                        }),
                    },
                ]),
            ),
        ]),
);

// M13 200 — Akroma's Memorial
pub(in crate::card::sets) static AKROMAS_MEMORIAL: CardRecord = CardRecord::new_with_legacy_id(
    1360,
    "Akroma's Memorial",
    CardArt::new(
        "d00d63c3-85a5-4c2d-bdba-6213527b5e9a",
        "Dan Murayama Scott",
    ),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Creatures you control have flying, first strike, vigilance, trample, haste, and protection from black and from red.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::flying()),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                    AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Black)),
                    AppliedEffectDef::add_ability(&abilities::protection_from_color(ManaColor::Red)),
                ]),
            },
        )),
);

// M13 201 — Chronomaton
pub(in crate::card::sets) static CHRONOMATON: CardRecord = CardRecord::new_with_legacy_id(
    1044,
    "Chronomaton",
    CardArt::new("aac35e28-dd0e-4dc8-b8e6-4a1e33706214", "Vincent Proce"),
    CardSet::Magic2013,
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Golem"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}, {T}: Put a +1/+1 counter on this creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 202 — Clock of Omens (reprint)

// M13 203 — Door to Nothingness
pub(in crate::card::sets) static DOOR_TO_NOTHINGNESS: CardRecord = CardRecord::new_with_legacy_id(
    1045,
    "Door to Nothingness",
    CardArt::new("57877b1c-e91d-4941-81bd-008dff1272ed", "Svetlin Velinov"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{5}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        AbilityDef::activated_with_targets(
            "{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}, {T}, Sacrifice this artifact: Target player loses the game.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}{W}{U}{U}{B}{B}{R}{R}{G}{G}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::LoseTheGame {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// M13 204 — Elixir of Immortality
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELIXIR_OF_IMMORTALITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99bd4740-9b1f-40a6-a14d-2c0d642b848b"),
    "Elixir of Immortality",
    crate::card::CardArt::new(
        "813d6a95-719d-474d-942a-b4c5156af7ba",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 205 — Gem of Becoming
// Audit: metadata-only — Needs three separate library searches for cards with three different names.
pub(in crate::card::sets) static GEM_OF_BECOMING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e07bc36-2207-48f4-a151-4ccb0c6d851d"),
    "Gem of Becoming",
    crate::card::CardArt::new("0e07bc36-2207-48f4-a151-4ccb0c6d851d", "Jack Wang"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 206 — Gilded Lotus
pub(in crate::card::sets) static GILDED_LOTUS: CardRecord = CardRecord::new_with_legacy_id(
    1046,
    "Gilded Lotus",
    CardArt::new("33704052-aeb1-4798-a64d-778e1879eeb9", "Martina Pilcerova"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add three mana of any one color.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )),
);

// M13 207 — Jayemdae Tome (reprint)

// M13 208 — Kitesail
pub(in crate::card::sets) static KITESAIL: CardRecord = CardRecord::new_with_legacy_id(
    1923,
    "Kitesail",
    CardArt::new(
        "2f95cf4c-1845-4260-8571-91c03d582da3",
        "Cyril Van Der Haegen",
    ),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// M13 209 — Phyrexian Hulk
pub(in crate::card::sets) static PHYREXIAN_HULK: CardRecord = CardRecord::new_with_legacy_id(
    1047,
    "Phyrexian Hulk",
    CardArt::new("a761426e-2138-438e-8f3b-024486165260", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Phyrexian", "Golem"], 5, 4),
);

// M13 210 — Primal Clay (reprint)

// M13 211 — Ring of Evos Isle
pub(in crate::card::sets) static RING_OF_EVOS_ISLE: CardRecord = CardRecord::new_with_legacy_id(
    1686,
    "Ring of Evos Isle",
    CardArt::new("a7c740a8-1bbc-4ec8-a72c-01aee9e48f3d", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::activated(
                "{2}: Equipped creature gains hexproof until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's blue.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Blue),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 212 — Ring of Kalonia
pub(in crate::card::sets) static RING_OF_KALONIA: CardRecord = CardRecord::new_with_legacy_id(
    1687,
    "Ring of Kalonia",
    CardArt::new("2082e04f-f972-424e-a724-7a5975215538", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's green.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Green),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 213 — Ring of Thune
pub(in crate::card::sets) static RING_OF_THUNE: CardRecord = CardRecord::new_with_legacy_id(
    1688,
    "Ring of Thune",
    CardArt::new("1ee2e94f-5b06-4df0-ba87-4499b1ee4dba", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has vigilance.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's white.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::White),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 214 — Ring of Valkas
pub(in crate::card::sets) static RING_OF_VALKAS: CardRecord = CardRecord::new_with_legacy_id(
    1689,
    "Ring of Valkas",
    CardArt::new("546e9fc1-03ff-4ae5-9488-51bf2e627486", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's red.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Red),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 215 — Ring of Xathrid
pub(in crate::card::sets) static RING_OF_XATHRID: CardRecord = CardRecord::new_with_legacy_id(
    1690,
    "Ring of Xathrid",
    CardArt::new("47e2aa59-63dc-4e28-8cdc-2ca868ff8f59", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::activated(
                "{2}: Regenerate equipped creature.",
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::triggered_if(
                "At the beginning of your upkeep, put a +1/+1 counter on equipped creature if \
                 it's black.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                &TriggerConditionDef::AttachedPermanentMatches {
                    object: ObjectPredicateDef::Color(ManaColor::Black),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// M13 216 — Sands of Delirium
pub(in crate::card::sets) static SANDS_OF_DELIRIUM: CardRecord = CardRecord::new_with_legacy_id(
    1361,
    "Sands of Delirium",
    CardArt::new("78c9d3bf-c858-42f4-bb61-3292f9a7141b", "Charles Urbach"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{X}, {T}: Target player mills X cards.",
        &[
            AbilityCostDef::Mana(mana_cost!("{X}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Mill {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// M13 217 — Staff of Nin
pub(in crate::card::sets) static STAFF_OF_NIN: CardRecord = CardRecord::new_with_legacy_id(
    1048,
    "Staff of Nin",
    CardArt::new("69b7381a-ec4a-4f1b-b81c-bdf9f9d64f31", "Dan Murayama Scott"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{6}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, draw a card.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}: This artifact deals 1 damage to any target.",
            &[AbilityCostDef::TapSource],
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

// M13 218 — Stuffy Doll (reprint)

// M13 219 — Tormod's Crypt (reprint)

// M13 220 — Trading Post
// Audit: metadata-only — Needs four modes whose costs include unsupported discard or separately chosen sacrifice costs and their linked continuations.
pub(in crate::card::sets) static TRADING_POST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20604b28-d096-40f8-a30c-3bc89e708676"),
    "Trading Post",
    crate::card::CardArt::new("20604b28-d096-40f8-a30c-3bc89e708676", "Adam Paquette"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 221 — Cathedral of War
pub(in crate::card::sets) static CATHEDRAL_OF_WAR: CardRecord = CardRecord::new_with_legacy_id(
    1506,
    "Cathedral of War",
    CardArt::new("dd222c07-0b28-41cb-9237-ad7991ab078f", "Kekai Kotaki"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]),
);

// M13 222 — Dragonskull Summit
pub(in crate::card::sets) static DRAGONSKULL_SUMMIT: CardRecord = CardRecord::new_with_legacy_id(
    1049,
    "Dragonskull Summit",
    CardArt::new("5e49c561-570c-43dd-a369-48bc7ad7edac", "Jon Foster"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Swamp or a Mountain.",
            &[BasicLandType::Swamp, BasicLandType::Mountain],
        ),
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

// M13 223 — Drowned Catacomb
pub(in crate::card::sets) static DROWNED_CATACOMB: CardRecord = CardRecord::new_with_legacy_id(
    1050,
    "Drowned Catacomb",
    CardArt::new("8b41b86b-58e1-4601-b8ed-0ad31f03a78d", "Dave Kendall"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control an Island or a Swamp.",
            &[BasicLandType::Island, BasicLandType::Swamp],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// M13 224 — Evolving Wilds (reprint)

// M13 225 — Glacial Fortress
pub(in crate::card::sets) static GLACIAL_FORTRESS: CardRecord = CardRecord::new_with_legacy_id(
    170,
    "Glacial Fortress",
    CardArt::new("bc9d29ee-1a21-4c3e-99c1-f815d40e8f19", "Franz Vohwinkel"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Plains or an Island.",
            &[BasicLandType::Plains, BasicLandType::Island],
        ),
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

// M13 226 — Hellion Crucible
// Audit: metadata-only — The counter vocabulary has no pressure counter, so the add and remove costs cannot use the printed counter kind.
pub(in crate::card::sets) static HELLION_CRUCIBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad8274ef-a46a-4f5f-8ad1-6ce828f24210"),
    "Hellion Crucible",
    crate::card::CardArt::new("ad8274ef-a46a-4f5f-8ad1-6ce828f24210", "Trevor Claxton"),
    crate::card::CardSet::Magic2013,
    crate::card::CardRules::unsupported(),
);

// M13 227 — Reliquary Tower
pub(in crate::card::sets) static RELIQUARY_TOWER: CardRecord = CardRecord::new_with_legacy_id(
    1950,
    "Reliquary Tower",
    CardArt::new("f92583e4-9749-4c11-9d32-fb81260c5b63", "Jesper Ejsing"),
    CardSet::Magic2013,
    // "You", so it does nothing for the opponent, and it is read at cleanup
    // rather than captured -- losing the Tower on your own turn puts the
    // limit straight back.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::static_ability(
            "You have no maximum hand size.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::NoMaximumHandSize),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ]),
);

// M13 228 — Rootbound Crag
pub(in crate::card::sets) static ROOTBOUND_CRAG: CardRecord = CardRecord::new_with_legacy_id(
    205,
    "Rootbound Crag",
    CardArt::new("76364643-bfcb-4c50-9224-bf9e35648ddf", "Matt Stewart"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain or a Forest.",
            &[BasicLandType::Mountain, BasicLandType::Forest],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// M13 229 — Sunpetal Grove
pub(in crate::card::sets) static SUNPETAL_GROVE: CardRecord = CardRecord::new_with_legacy_id(
    221,
    "Sunpetal Grove",
    CardArt::new("15663129-9deb-4c34-84a0-f94cf1a723f0", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Forest or a Plains.",
            &[BasicLandType::Forest, BasicLandType::Plains],
        ),
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

// M13 230 — Plains (reprint)

// M13 231 — Plains (alternate printing)

// M13 232 — Plains (alternate printing)

// M13 233 — Plains (alternate printing)

// M13 234 — Island (reprint)

// M13 235 — Island (alternate printing)

// M13 236 — Island (alternate printing)

// M13 237 — Island (alternate printing)

// M13 238 — Swamp (reprint)

// M13 239 — Swamp (alternate printing)

// M13 240 — Swamp (alternate printing)

// M13 241 — Swamp (alternate printing)

// M13 242 — Mountain (reprint)

// M13 243 — Mountain (alternate printing)

// M13 244 — Mountain (alternate printing)

// M13 245 — Mountain (alternate printing)

// M13 246 — Forest (reprint)

// M13 247 — Forest (alternate printing)

// M13 248 — Forest (alternate printing)

// M13 249 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AJANI_CALLER_OF_THE_PRIDE,
    &AJANIS_SUNSTRIKER,
    &ANGELIC_BENEDICTION,
    &ATTENDED_KNIGHT,
    &AVEN_SQUIRE,
    &BATTLEFLIGHT_EAGLE,
    &CAPTAIN_OF_THE_WATCH,
    &CAPTAINS_CALL,
    &CRUSADER_OF_ODRIC,
    &DIVINE_FAVOR,
    &DIVINE_VERDICT,
    &ERASE,
    &FAITH_S_REWARD,
    &GLORIOUS_CHARGE,
    &GRIFFIN_PROTECTOR,
    &GUARDIAN_LIONS,
    &GUARDIANS_OF_AKRASA,
    &HEALER_OF_THE_PRIDE,
    &KNIGHT_OF_GLORY,
    &OBLIVION_RING,
    &ODRIC_MASTER_TACTICIAN,
    &PACIFISM,
    &PILLARFIELD_OX,
    &PLANAR_CLEANSING,
    &PRIZED_ELEPHANT,
    &RAIN_OF_BLADES,
    &RHOX_FAITHMENDER,
    &SAFE_PASSAGE,
    &SHOW_OF_VALOR,
    &SILVERCOAT_LION,
    &SUBLIME_ARCHANGEL,
    &TOUCH_OF_THE_ETERNAL,
    &WAR_FALCON,
    &WAR_PRIEST_OF_THUNE,
    &WARCLAMP_MASTIFF,
    &ARCHAEOMANCER,
    &ARCTIC_AVEN,
    &AUGUR_OF_BOLAS,
    &BATTLE_OF_WITS,
    &COURTLY_PROVOCATEUR,
    &DOWNPOUR,
    &ENCRUST,
    &ESSENCE_SCATTER,
    &FAERIE_INVADERS,
    &FOG_BANK,
    &HARBOR_SERPENT,
    &HYDROSURGE,
    &JACE_MEMORY_ADEPT,
    &JACE_S_PHANTASM,
    &KRAKEN_HATCHLING,
    &MASTER_OF_THE_PEARL_TRIDENT,
    &MIND_SCULPT,
    &NEGATE,
    &OMNISCIENCE,
    &REDIRECT,
    &SCROLL_THIEF,
    &SLEEP,
    &SPELLTWINE,
    &SPHINX_OF_UTHUUN,
    &SWITCHEROO,
    &TALRAND_SKY_SUMMONER,
    &TALRANDS_INVOCATION,
    &TRICKS_OF_THE_TRADE,
    &VEDALKEN_ENTRANCER,
    &VOID_STALKER,
    &WATERCOURSER,
    &WELKIN_TERN,
    &WIND_DRAKE,
    &BLOOD_RECKONING,
    &BLOODHUNTER_BAT,
    &BLOODTHRONE_VAMPIRE,
    &COWER_IN_FEAR,
    &CRIPPLING_BLIGHT,
    &DARK_FAVOR,
    &DIABOLIC_REVELATION,
    &DISCIPLE_OF_BOLAS,
    &DISENTOMB,
    &DURESS,
    &DUSKMANTLE_PROWLER,
    &DUTY_BOUND_DEAD,
    &ESSENCE_DRAIN,
    &GIANT_SCORPION,
    &HARBOR_BANDIT,
    &KNIGHT_OF_INFAMY,
    &LILIANA_OF_THE_DARK_REALMS,
    &LILIANAS_SHADE,
    &MARK_OF_THE_VAMPIRE,
    &MIND_ROT,
    &MURDER,
    &MUTILATE,
    &NEFAROX_OVERLORD_OF_GRIXIS,
    &PUBLIC_EXECUTION,
    &RAVENOUS_RATS,
    &RISE_FROM_THE_GRAVE,
    &SERVANT_OF_NEFAROX,
    &SIGN_IN_BLOOD,
    &TORMENTED_SOUL,
    &VAMPIRE_NIGHTHAWK,
    &VAMPIRE_NOCTURNUS,
    &VEILBORN_GHOUL,
    &VILE_REBIRTH,
    &WITS_END,
    &XATHRID_GORGON,
    &ZOMBIE_GOLIATH,
    &ARMS_DEALER,
    &BLADETUSK_BOAR,
    &CANYON_MINOTAUR,
    &CHANDRA_THE_FIREBRAND,
    &CHANDRAS_FURY,
    &CLEAVER_RIOT,
    &CRATERIZE,
    &CRIMSON_MUCKWADER,
    &DRAGON_HATCHLING,
    &FERVOR,
    &FIREWING_PHOENIX,
    &FLAMES_OF_THE_FIREBRAND,
    &FURNACE_WHELP,
    &GOBLIN_ARSONIST,
    &GOBLIN_BATTLE_JESTER,
    &HAMLETBACK_GOLIATH,
    &KINDLED_FURY,
    &KRENKO_MOB_BOSS,
    &KRENKOS_COMMAND,
    &MAGMAQUAKE,
    &MARK_OF_MUTINY,
    &MINDCLAW_SHAMAN,
    &RECKLESS_BRUTE,
    &REVERBERATE,
    &RUMMAGING_GOBLIN,
    &SEARING_SPEAR,
    &SLUMBERING_DRAGON,
    &SMELT,
    &THUNDERMAW_HELLKITE,
    &TRUMPET_BLAST,
    &TURN_TO_SLAG,
    &VOLCANIC_GEYSER,
    &VOLCANIC_STRENGTH,
    &WILD_GUESS,
    &WORLDFIRE,
    &ACIDIC_SLIME,
    &ARBOR_ELF,
    &BOND_BEETLE,
    &BOUNDLESS_REALMS,
    &BOUNTIFUL_HARVEST,
    &CENTAUR_COURSER,
    &DEADLY_RECLUSE,
    &DUSKDALE_WURM,
    &ELDERSCALE_WURM,
    &ELVISH_ARCHDRUID,
    &ELVISH_VISIONARY,
    &FARSEEK,
    &FLINTHOOF_BOAR,
    &FUNGAL_SPROUTING,
    &GARRUK_PRIMAL_HUNTER,
    &MWONVULI_BEAST_TRACKER,
    &PLUMMET,
    &PREDATORY_RAMPAGE,
    &PRIMAL_HUNTBEAST,
    &PRIMORDIAL_HYDRA,
    &RANGERS_PATH,
    &REVIVE,
    &ROARING_PRIMADOX,
    &SENTINEL_SPIDER,
    &SERPENTS_GIFT,
    &SILKLASH_SPIDER,
    &SPIKED_BALOTH,
    &THRAGTUSK,
    &TIMBERPACK_WOLF,
    &TITANIC_GROWTH,
    &VASTWOOD_GORGER,
    &YEVA_NATURE_S_HERALD,
    &YEVAS_FORCEMAGE,
    &NICOL_BOLAS_PLANESWALKER,
    &AKROMAS_MEMORIAL,
    &CHRONOMATON,
    &DOOR_TO_NOTHINGNESS,
    &ELIXIR_OF_IMMORTALITY,
    &GEM_OF_BECOMING,
    &GILDED_LOTUS,
    &KITESAIL,
    &PHYREXIAN_HULK,
    &RING_OF_EVOS_ISLE,
    &RING_OF_KALONIA,
    &RING_OF_THUNE,
    &RING_OF_VALKAS,
    &RING_OF_XATHRID,
    &SANDS_OF_DELIRIUM,
    &STAFF_OF_NIN,
    &TRADING_POST,
    &CATHEDRAL_OF_WAR,
    &DRAGONSKULL_SUMMIT,
    &DROWNED_CATACOMB,
    &GLACIAL_FORTRESS,
    &HELLION_CRUCIBLE,
    &RELIQUARY_TOWER,
    &ROOTBOUND_CRAG,
    &SUNPETAL_GROVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&avacyn_restored::ANGELS_MERCY), // M13 3
    PrintingRecord::reprint(&catalog_usg::INTREPID_HERO),    // M13 20
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),            // M13 31
    PrintingRecord::reprint(&catalog_usg::SERRA_AVATAR),     // M13 32
    PrintingRecord::reprint(&catalog_tsp::SERRA_AVENGER),    // M13 33
    PrintingRecord::reprint(&alpha::CLONE),                  // M13 45
    PrintingRecord::reprint(&dark_ascension::DIVINATION),    // M13 47
    PrintingRecord::reprint(&catalog_apc::INDEX),            // M13 55
    PrintingRecord::reprint(&alpha::MERFOLK_OF_THE_PEARL_TRIDENT), // M13 60
    PrintingRecord::reprint(&catalog_usg::REWIND),           // M13 65
    PrintingRecord::reprint(&catalog_m11::STORMTIDE_LEVIATHAN), // M13 70
    PrintingRecord::reprint(&alpha::UNSUMMON),               // M13 75
    PrintingRecord::reprint(&catalog_m11::PHYLACTERY_LICH),  // M13 104
    PrintingRecord::reprint(&catalog_fut::SHIMIAN_SPECTER),  // M13 109
    PrintingRecord::reprint(&innistrad::WALKING_CORPSE),     // M13 116
    PrintingRecord::reprint(&alpha::FIRE_ELEMENTAL),         // M13 130
    PrintingRecord::reprint(&catalog_sth::MOGG_FLUNKIES),    // M13 143
    PrintingRecord::reprint(&dark_ascension::TORCH_FIEND),   // M13 151
    PrintingRecord::reprint(&alpha::WALL_OF_FIRE),           // M13 156
    PrintingRecord::reprint(&alpha::FOG),                    // M13 172
    PrintingRecord::reprint(&catalog_m11::GARRUK_S_PACKLEADER), // M13 175
    PrintingRecord::reprint(&catalog_ody::GROUND_SEAL),      // M13 176
    PrintingRecord::reprint(&onslaught::NATURALIZE),         // M13 178
    PrintingRecord::reprint(&innistrad::PREY_UPON),          // M13 181
    PrintingRecord::reprint(&planeshift::QUIRION_DRYAD),     // M13 184
    PrintingRecord::reprint(&urzas_legacy::RANCOR),          // M13 185
    PrintingRecord::reprint(&catalog_5dn::CLOCK_OF_OMENS),   // M13 202
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),          // M13 207
    PrintingRecord::reprint(&catalog_atq::PRIMAL_CLAY),      // M13 210
    PrintingRecord::reprint(&catalog_tsp::STUFFY_DOLL),      // M13 218
    PrintingRecord::reprint(&the_dark::TORMODS_CRYPT),       // M13 219
    PrintingRecord::reprint(&catalog_dka::EVOLVING_WILDS),   // M13 224
    PrintingRecord::reprint(&alpha::PLAINS),                 // M13 230
    PrintingRecord::alternate(&alpha::PLAINS, 1),            // M13 231
    PrintingRecord::alternate(&alpha::PLAINS, 2),            // M13 232
    PrintingRecord::alternate(&alpha::PLAINS, 3),            // M13 233
    PrintingRecord::reprint(&alpha::ISLAND),                 // M13 234
    PrintingRecord::alternate(&alpha::ISLAND, 1),            // M13 235
    PrintingRecord::alternate(&alpha::ISLAND, 2),            // M13 236
    PrintingRecord::alternate(&alpha::ISLAND, 3),            // M13 237
    PrintingRecord::reprint(&alpha::SWAMP),                  // M13 238
    PrintingRecord::alternate(&alpha::SWAMP, 1),             // M13 239
    PrintingRecord::alternate(&alpha::SWAMP, 2),             // M13 240
    PrintingRecord::alternate(&alpha::SWAMP, 3),             // M13 241
    PrintingRecord::reprint(&alpha::MOUNTAIN),               // M13 242
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),          // M13 243
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),          // M13 244
    PrintingRecord::alternate(&alpha::MOUNTAIN, 3),          // M13 245
    PrintingRecord::reprint(&alpha::FOREST),                 // M13 246
    PrintingRecord::alternate(&alpha::FOREST, 1),            // M13 247
    PrintingRecord::alternate(&alpha::FOREST, 2),            // M13 248
    PrintingRecord::alternate(&alpha::FOREST, 3),            // M13 249
];
