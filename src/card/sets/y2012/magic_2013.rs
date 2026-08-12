//! Magic 2013 card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord, avacyn_restored, dark_ascension};
use crate::card::sets::{
    y1993::alpha, y1994::the_dark, y2001::planeshift, y2002::onslaught, y2011::innistrad,
};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, BasicLandType, CardArt, CardBehavior, CardRules, CardSet,
    CardSupertype, CardType, ComparisonDef, CounterKind, DiscardSelectionDef, DividedTotal,
    EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef, KeywordAbility,
    ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayerRelation, ReplacementEventDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// M13 1 — Ajani, Caller of the Pride
// Audit: blocked — The token count needs the controller's current life total, which no token-count value exposes.

// M13 2 — Ajani's Sunstriker
pub(in crate::card::sets) static AJANIS_SUNSTRIKER: CardRecord = CardRecord::new(
    cards::AJANIS_SUNSTRIKER,
    "Ajani's Sunstriker",
    CardArt::new("3570c4d9-cd42-4aca-9421-ac44e057a785", "Matt Stewart"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Cat", "Cleric"], 2, 2)
        .with_abilities(&[abilities::lifelink()]),
);

// M13 4 — Angelic Benediction
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 5 — Attended Knight
pub(in crate::card::sets) static ATTENDED_KNIGHT: CardRecord = CardRecord::new(
    cards::ATTENDED_KNIGHT,
    "Attended Knight",
    CardArt::new("c0f5cb3f-c27d-4b35-930f-00d806393796", "Seb McKinnon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::triggered(
            "When this creature enters, create a 1/1 white Soldier creature token.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::CreateToken {
                token: cards::SOLDIER_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M13 6 — Aven Squire
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 7 — Battleflight Eagle
pub(in crate::card::sets) static BATTLEFLIGHT_EAGLE: CardRecord = CardRecord::new(
    cards::BATTLEFLIGHT_EAGLE,
    "Battleflight Eagle",
    CardArt::new("4182dbd5-8eae-4f4b-86aa-2bfc24481800", "Kev Walker"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gets +2/+2 and gains flying until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(2),
                    },
                    AppliedEffectDef::GrantAbility(&abilities::flying()),
                ]),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 8 — Captain of the Watch
pub(in crate::card::sets) static CAPTAIN_OF_THE_WATCH: CardRecord = CardRecord::new(
    cards::CAPTAIN_OF_THE_WATCH,
    "Captain of the Watch",
    CardArt::new("8e3c18f5-89cd-4d33-8d5b-12dacad9f9b3", "Greg Staples"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Human", "Soldier"], 3, 3).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Other Soldier creatures you control get +1/+1 and have vigilance.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Soldier"),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(1),
                            toughness: ValueDef::Constant(1),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::vigilance()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
            AbilityDef::triggered(
                "When this creature enters, create three 1/1 white Soldier creature tokens.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                EffectDef::CreateToken {
                    token: cards::SOLDIER_TOKEN_1_1_WHITE,
                    count: ValueDef::Constant(3),
                },
            ),
        ],
    ),
);

// M13 9 — Captain's Call
pub(in crate::card::sets) static CAPTAINS_CALL: CardRecord = CardRecord::new(
    cards::CAPTAINS_CALL,
    "Captain's Call",
    CardArt::new("79258432-ea35-4f2a-9e4a-4abb53f335c6", "Greg Staples"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Create three 1/1 white Soldier creature tokens.",
        EffectDef::CreateToken {
            token: cards::SOLDIER_TOKEN_1_1_WHITE,
            count: ValueDef::Constant(3),
        },
    )),
);

static CRUSADER_CREATURES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 10 — Crusader of Odric
pub(in crate::card::sets) static CRUSADER_OF_ODRIC: CardRecord = CardRecord::new(
    cards::CRUSADER_OF_ODRIC,
    "Crusader of Odric",
    CardArt::new("295096bb-1857-4224-bc7b-307b38cfd338", "Michael Komarck"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Crusader of Odric's power and toughness are each equal to the number of creatures you control.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::CountMatchingObjects(&CRUSADER_CREATURES),
                    toughness: ValueDef::CountMatchingObjects(&CRUSADER_CREATURES),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// M13 11 — Divine Favor
pub(in crate::card::sets) static DIVINE_FAVOR: CardRecord = CardRecord::new(
    cards::DIVINE_FAVOR,
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
            AbilityDef::triggered(
                "When this Aura enters, you gain 3 life.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+3.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(3),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// M13 12 — Divine Verdict
pub(in crate::card::sets) static DIVINE_VERDICT: CardRecord = CardRecord::new(
    cards::DIVINE_VERDICT,
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
pub(in crate::card::sets) static ERASE: CardRecord = CardRecord::new(
    cards::ERASE,
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
            controller: None,
        },
    )),
);

// M13 14 — Faith's Reward
// Audit: blocked — Needs turn-history provenance for permanent cards put into your graveyard from the battlefield and a simultaneous mass return.

// M13 15 — Glorious Charge
pub(in crate::card::sets) static GLORIOUS_CHARGE: CardRecord = CardRecord::new(
    cards::GLORIOUS_CHARGE,
    "Glorious Charge",
    CardArt::new("f8672cfd-e34b-4587-9e24-015e03c7574d", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(1),
                toughness: ValueDef::Constant(1),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 16 — Griffin Protector
pub(in crate::card::sets) static GRIFFIN_PROTECTOR: CardRecord = CardRecord::new(
    cards::GRIFFIN_PROTECTOR,
    "Griffin Protector",
    CardArt::new("ddae4f7a-525c-4306-81b5-b0991840a11e", "Christopher Moeller"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature gets +1/+1 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 17 — Guardian Lions
pub(in crate::card::sets) static GUARDIAN_LIONS: CardRecord = CardRecord::new(
    cards::GUARDIAN_LIONS,
    "Guardian Lions",
    CardArt::new("3defc506-537e-4659-815d-5dab15fbf199", "Johannes Voss"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Cat"], 1, 6)
        .with_abilities(&[abilities::vigilance()]),
);

// M13 18 — Guardians of Akrasa
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 19 — Healer of the Pride
pub(in crate::card::sets) static HEALER_OF_THE_PRIDE: CardRecord = CardRecord::new(
    cards::HEALER_OF_THE_PRIDE,
    "Healer of the Pride",
    CardArt::new(
        "35716e37-1bb2-41e2-bb55-e65126b01ce3",
        "Christopher Moeller",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Cat", "Cleric"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 2 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// M13 20 — Intrepid Hero
// Audit: blocked — Power predicates omit continuous static bonuses, so they cannot enforce the target's full current power.

// M13 21 — Knight of Glory
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 22 — Oblivion Ring
pub(in crate::card::sets) static OBLIVION_RING: CardRecord = CardRecord::new(
    cards::OBLIVION_RING,
    "Oblivion Ring",
    CardArt::new("1e2a73ec-39be-4d23-8c25-17d7c174dcee", "Franz Vohwinkel"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::triggered_with_targets("When this enchantment enters, exile another target nonland permanent.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
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
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: None,
            },
            EffectDef::ReturnLinkedExiles {
                zone: ZoneKind::Battlefield,
                grant: None,
            },
        ),
    ]),
);

// M13 23 — Odric, Master Tactician
// Audit: blocked — Needs an attacking-group threshold and a combat procedure that lets its controller assign every blocker.

// M13 24 — Pacifism
// Audit: blocked — Continuous effects cannot prohibit both attacking and blocking.

// M13 25 — Pillarfield Ox
pub(in crate::card::sets) static PILLARFIELD_OX: CardRecord = CardRecord::new(
    cards::PILLARFIELD_OX,
    "Pillarfield Ox",
    CardArt::new("33e2f3ae-bf92-478b-9c63-acc3f175f02a", "Andrew Robinson"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Ox"], 2, 4),
);

// M13 26 — Planar Cleansing
pub(in crate::card::sets) static PLANAR_CLEANSING: CardRecord = CardRecord::new(
    cards::PLANAR_CLEANSING,
    "Planar Cleansing",
    CardArt::new("b5047b71-2359-4d9a-a168-a8eec43c5f1b", "Michael Komarck"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all nonland permanents.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )),
);

static PRIZED_ELEPHANT_FORESTS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 27 — Prized Elephant
pub(in crate::card::sets) static PRIZED_ELEPHANT: CardRecord = CardRecord::new(
    cards::PRIZED_ELEPHANT,
    "Prized Elephant",
    CardArt::new("01597ede-94e7-44a4-93c2-7fd1db11e92a", "Ioan Dumitrescu"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Elephant"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Forest.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::AnyMatchingObject(&PRIZED_ELEPHANT_FORESTS),
                    toughness: ValueDef::AnyMatchingObject(&PRIZED_ELEPHANT_FORESTS),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::activated(
            "{G}: This creature gains trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::trample()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 28 — Rain of Blades
pub(in crate::card::sets) static RAIN_OF_BLADES: CardRecord = CardRecord::new(
    cards::RAIN_OF_BLADES,
    "Rain of Blades",
    CardArt::new("f3bd6ca4-c4ed-41c3-834c-23e0c1741b72", "Rob Alexander"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Rain of Blades deals 1 damage to each attacking creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            amount: ValueDef::Constant(1),
        },
    )),
);

// M13 29 — Rhox Faithmender
pub(in crate::card::sets) static RHOX_FAITHMENDER: CardRecord = CardRecord::new(
    cards::RHOX_FAITHMENDER,
    "Rhox Faithmender",
    CardArt::new("85ea185a-7b38-49f3-be73-be8180fb6295", "Wesley Burt"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Rhino", "Monk"], 1, 5).with_abilities(&[
        abilities::lifelink(),
        AbilityDef::replacement_for(
            "If you would gain life, you gain twice that much life instead.",
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            EffectDef::MultiplyEventAmount(2),
        ),
    ]),
);

// M13 30 — Safe Passage
// Audit: blocked — The prevention vocabulary cannot cover all damage to a player and every creature they control for the turn.

// M13 32 — Serra Avatar
// Audit: blocked — Needs a life-total characteristic value in all zones and a self graveyard-to-library replacement.

// M13 33 — Serra Avenger
// Audit: blocked — Casting restrictions cannot inspect how many turns its controller has taken.

// M13 34 — Show of Valor
pub(in crate::card::sets) static SHOW_OF_VALOR: CardRecord = CardRecord::new(
    cards::SHOW_OF_VALOR,
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
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(2),
                toughness: ValueDef::Constant(4),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 35 — Silvercoat Lion
pub(in crate::card::sets) static SILVERCOAT_LION: CardRecord = CardRecord::new(
    cards::SILVERCOAT_LION,
    "Silvercoat Lion",
    CardArt::new("9d33e866-cfd8-44e6-8070-df8df1ce965d", "Terese Nielsen"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat"], 2, 2),
);

// M13 36 — Sublime Archangel
// Audit: blocked — Needs executable exalted and a static grant of separate exalted instances to other creatures.

// M13 37 — Touch of the Eternal
// Audit: blocked — No effect can set a player's life total from a battlefield permanent count.

// M13 38 — War Falcon
// Audit: blocked — Combat restrictions cannot condition this source's attack permission on controlling a Knight or Soldier.

// M13 39 — War Priest of Thune
pub(in crate::card::sets) static WAR_PRIEST_OF_THUNE: CardRecord = CardRecord::new(
    cards::WAR_PRIEST_OF_THUNE,
    "War Priest of Thune",
    CardArt::new("d28eb320-aea7-466e-8718-de8652a2b191", "Izzy"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may destroy target enchantment.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
                divided_total: None,
            }],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);
// M13 40 — Warclamp Mastiff
pub(in crate::card::sets) static WARCLAMP_MASTIFF: CardRecord = CardRecord::new(
    cards::WARCLAMP_MASTIFF,
    "Warclamp Mastiff",
    CardArt::new("102e48e0-8a5f-499d-ac62-005d3c075ef3", "David Palumbo"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{W}"), &["Dog"], 1, 1)
        .with_abilities(&[abilities::first_strike()]),
);

// M13 41 — Archaeomancer
pub(in crate::card::sets) static ARCHAEOMANCER: CardRecord = CardRecord::new(
    cards::ARCHAEOMANCER,
    "Archaeomancer",
    CardArt::new("73c6d1be-55ad-4ee4-b044-88438e9b78cc", "Zoltan Boros"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 1, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, return target instant or sorcery card from your graveyard to your hand.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
            },
        ),
    ),
);

static PLAINS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 42 — Arctic Aven
pub(in crate::card::sets) static ARCTIC_AVEN: CardRecord = CardRecord::new(
    cards::ARCTIC_AVEN,
    "Arctic Aven",
    CardArt::new("06f6aab1-c400-4d87-b68e-f36552e7417f", "Igor Kieryluk"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Bird", "Wizard"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Plains.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::AnyMatchingObject(&PLAINS_YOU_CONTROL),
                    toughness: ValueDef::AnyMatchingObject(&PLAINS_YOU_CONTROL),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::activated(
            "{W}: This creature gains lifelink until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 43 — Augur of Bolas
pub(in crate::card::sets) static AUGUR_OF_BOLAS: CardRecord = CardRecord::new(
    cards::AUGUR_OF_BOLAS,
    "Augur of Bolas",
    CardArt::new("2e6ec8a6-ad88-45c9-ab4b-dd7de2418bb7", "Slawomir Maniak"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Merfolk", "Wizard"],
        1,
        3,
    )
    .with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, look at the top three cards of your library. You may reveal an instant or sorcery card from among them and put it into your hand. Put the rest on the bottom of your library in any order.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::None,
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::AugurOfBolas))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The trigger uses the shared stack and a card-local library-selection resolver.",
        )),
    ]),
);

static BATTLE_OF_WITS_CONDITION: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Library],
        controller: PlayerRelation::You,
    },
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 200,
};

// M13 44 — Battle of Wits
pub(in crate::card::sets) static BATTLE_OF_WITS: CardRecord = CardRecord::new(
    cards::BATTLE_OF_WITS,
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
            &BATTLE_OF_WITS_CONDITION,
            EffectDef::LoseTheGame {
                player: EffectRecipientDef::Opponent,
            },
        ),
    ),
);

// M13 46 — Courtly Provocateur
// Audit: blocked — No turn-long effects require a target creature to attack or block if able.

// M13 48 — Downpour
pub(in crate::card::sets) static DOWNPOUR: CardRecord = CardRecord::new(
    cards::DOWNPOUR,
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
// Audit: blocked — Static effects cannot prohibit activation of the attached permanent's activated abilities.

// M13 50 — Essence Scatter
pub(in crate::card::sets) static ESSENCE_SCATTER: CardRecord = CardRecord::new(
    cards::ESSENCE_SCATTER,
    "Essence Scatter",
    CardArt::new("fcd965f9-bdaa-4434-a9c8-53fc57e997db", "Jon Foster"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::custom_full(
        "Counter target creature spell.",
        CardBehavior::EssenceScatter,
        "Implemented by the named card-local special behavior.",
    )),
);

// M13 51 — Faerie Invaders
pub(in crate::card::sets) static FAERIE_INVADERS: CardRecord = CardRecord::new(
    cards::FAERIE_INVADERS,
    "Faerie Invaders",
    CardArt::new("fcbc71b3-544b-4b81-8922-52744892989b", "Ryan Pancoast"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Faerie", "Rogue"], 3, 3)
        .with_abilities(&[abilities::flash(), abilities::flying()]),
);

// M13 52 — Fog Bank
// Audit: blocked — No static prevention effect suppresses all combat damage both to and from the source.

// M13 53 — Harbor Serpent
// Audit: blocked — Islandwalk and an attack restriction based on the total Island count are unavailable.

// M13 54 — Hydrosurge
pub(in crate::card::sets) static HYDROSURGE: CardRecord = CardRecord::new(
    cards::HYDROSURGE,
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
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-5),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 55 — Index
// Audit: blocked — Top-card selection cannot return all five cards in an arbitrary chosen order.

// M13 56 — Jace, Memory Adept
pub(in crate::card::sets) static JACE_MEMORY_ADEPT: CardRecord = CardRecord::new(
    cards::JACE_MEMORY_ADEPT,
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
// Audit: blocked — Conditional static bonuses cannot test whether an opponent's graveyard contains at least ten cards.

// M13 58 — Kraken Hatchling
pub(in crate::card::sets) static KRAKEN_HATCHLING: CardRecord = CardRecord::new(
    cards::KRAKEN_HATCHLING,
    "Kraken Hatchling",
    CardArt::new("59a50590-9091-4632-bf8c-792e1e0a75a8", "Jason Felix"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{U}"), &["Kraken"], 0, 4),
);

// M13 59 — Master of the Pearl Trident
// Audit: blocked — Needs the printed islandwalk keyword and its defending-player land/blocking semantics.

// M13 61 — Mind Sculpt
pub(in crate::card::sets) static MIND_SCULPT: CardRecord = CardRecord::new(
    cards::MIND_SCULPT,
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
pub(in crate::card::sets) static NEGATE: CardRecord = CardRecord::new(
    cards::NEGATE,
    "Negate",
    CardArt::new("8da17a86-3666-46b8-932e-daafd6a0cd69", "Jeremy Jarvis"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::custom_full(
        "Counter target noncreature spell.",
        CardBehavior::Negate,
        "Implemented by the named card-local special behavior.",
    )),
);

/// X is read off the sacrificed creature, so both halves take the power the
/// sacrifice recorded rather than counting anything on the board.
static DISCIPLE_OF_BOLAS_PAYOFF: EffectDef = EffectDef::Sequence(&[
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggerEventAmount,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::TriggerEventAmount,
    },
]);

// M13 63 — Omniscience
// Audit: blocked — No static permission waives mana costs for spells cast from hand.

// M13 64 — Redirect
// Audit: blocked — No effect can retarget a spell on the stack.

// M13 65 — Rewind
// Audit: blocked — Needs a non-target choice of up to four lands made during resolution after the spell is countered.

// M13 66 — Scroll Thief
pub(in crate::card::sets) static SCROLL_THIEF: CardRecord = CardRecord::new(
    cards::SCROLL_THIEF,
    "Scroll Thief",
    CardArt::new(
        "dc201a82-fb48-4bb4-b072-e206e6872aa5",
        "Alex Horley-Orlandelli",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Rogue"], 1, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 67 — Sleep
// Audit: blocked — Needs simultaneous mass tapping plus a next-untap-step skip attached to exactly those creatures.

// M13 68 — Spelltwine
// Audit: blocked — Needs linked graveyard choices, spell copies, and permission to cast both copies without paying their costs.

// M13 69 — Sphinx of Uthuun
pub(in crate::card::sets) static SPHINX_OF_UTHUUN: CardRecord = CardRecord::new(
    cards::SPHINX_OF_UTHUUN,
    "Sphinx of Uthuun",
    CardArt::new("4462978c-0076-466b-a64b-0f54d09d4f27", "Kekai Kotaki"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Sphinx"], 5, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::RevealAndSplitIntoPiles {
                count: ValueDef::Constant(5),
                rest: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// M13 70 — Stormtide Leviathan
// Audit: blocked — Islandwalk, global Island type addition, and attack restrictions based on flying or islandwalk cannot all be expressed.

// M13 71 — Switcheroo
// Audit: blocked — Indefinite control exchange between two targets is unavailable.

// M13 72 — Talrand, Sky Summoner
pub(in crate::card::sets) static TALRAND_SKY_SUMMONER: CardRecord = CardRecord::new(
    cards::TALRAND_SKY_SUMMONER,
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
        TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
        ])),
        EffectDef::CreateToken {
            token: cards::DRAKE_TOKEN_2_2_BLUE,
            count: ValueDef::Constant(1),
        },
    )),
);

// M13 73 — Talrand's Invocation
pub(in crate::card::sets) static TALRANDS_INVOCATION: CardRecord = CardRecord::new(
    cards::TALRANDS_INVOCATION,
    "Talrand's Invocation",
    CardArt::new("c2cd809c-557a-42a5-950b-56b5b47b325b", "Svetlin Velinov"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::spell(
        "Create two 2/2 blue Drake creature tokens with flying.",
        EffectDef::CreateToken {
            token: cards::DRAKE_TOKEN_2_2_BLUE,
            count: ValueDef::Constant(2),
        },
    )),
);

// M13 74 — Tricks of the Trade
pub(in crate::card::sets) static TRICKS_OF_THE_TRADE: CardRecord = CardRecord::new(
    cards::TRICKS_OF_THE_TRADE,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(2),
                            toughness: ValueDef::Constant(0),
                        },
                        AppliedEffectDef::CannotBeBlockedBy(ObjectPredicateDef::Any),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// M13 76 — Vedalken Entrancer
pub(in crate::card::sets) static VEDALKEN_ENTRANCER: CardRecord = CardRecord::new(
    cards::VEDALKEN_ENTRANCER,
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
// Audit: blocked — Needs simultaneous source-and-target library moves followed by shuffling both affected owners' libraries.

// M13 78 — Watercourser
pub(in crate::card::sets) static WATERCOURSER: CardRecord = CardRecord::new(
    cards::WATERCOURSER,
    "Watercourser",
    CardArt::new("a27c441a-b31d-4214-8fc5-054003e257dc", "Mathias Kollros"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental"], 2, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(-1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M13 79 — Welkin Tern
// Audit: blocked — No combat restriction limits this source to blocking creatures with flying.

// M13 80 — Wind Drake
pub(in crate::card::sets) static WIND_DRAKE: CardRecord = CardRecord::new(
    cards::WIND_DRAKE,
    "Wind Drake",
    CardArt::new("c9dcb8d2-0da9-40fc-b0c0-2c76b3d277bc", "Steve Prescott"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

// M13 81 — Blood Reckoning
pub(in crate::card::sets) static BLOOD_RECKONING: CardRecord = CardRecord::new(
    cards::BLOOD_RECKONING,
    "Blood Reckoning",
    CardArt::new("24577bb2-61b0-4675-84e6-5d675b28fc0e", "Wayne Reynolds"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{3}{B}")).with_ability(AbilityDef::triggered(
        "Whenever a creature attacks you or a planeswalker you control, that creature's controller loses 1 life.",
        TriggerEventDef::Attacks(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::ControllerOfTriggeringObject,
            amount: ValueDef::Constant(1),
        },
    )),
);

// M13 82 — Bloodhunter Bat
pub(in crate::card::sets) static BLOODHUNTER_BAT: CardRecord = CardRecord::new(
    cards::BLOODHUNTER_BAT,
    "Bloodhunter Bat",
    CardArt::new("99c10705-6e0e-46f6-a64c-0095b2796aaf", "Tomasz Jedruszek"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Bat"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, target player loses 2 life and you gain 2 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
pub(in crate::card::sets) static BLOODTHRONE_VAMPIRE: CardRecord = CardRecord::new(
    cards::BLOODTHRONE_VAMPIRE,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M13 84 — Cower in Fear
pub(in crate::card::sets) static COWER_IN_FEAR: CardRecord = CardRecord::new(
    cards::COWER_IN_FEAR,
    "Cower in Fear",
    CardArt::new("bf2d53b8-7847-4b94-9711-eca29facccba", "Nils Hamm"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell(
        "Creatures your opponents control get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Opponent,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(-1),
                toughness: ValueDef::Constant(-1),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 85 — Crippling Blight
// Audit: blocked — Continuous combat restrictions cannot make the enchanted creature unable to block.

// M13 86 — Dark Favor
pub(in crate::card::sets) static DARK_FAVOR: CardRecord = CardRecord::new(
    cards::DARK_FAVOR,
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
            AbilityDef::triggered(
                "When this Aura enters, you lose 1 life.",
                TriggerEventDef::ZoneChanged {
                    object: ObjectPredicateDef::Source,
                    from: None,
                    to: Some(ZoneKind::Battlefield),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +3/+1.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(3),
                        toughness: ValueDef::Constant(1),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// M13 87 — Diabolic Revelation
// Audit: blocked — SearchZone has a static maximum and cannot select up to the chosen X cards.

// M13 88 — Disciple of Bolas
pub(in crate::card::sets) static DISCIPLE_OF_BOLAS: CardRecord = CardRecord::new(
    cards::DISCIPLE_OF_BOLAS,
    "Disciple of Bolas",
    CardArt::new("c4dd57f8-27bc-4ad9-a79e-48a68af33b02", "Slawomir Maniak"),
    CardSet::Magic2013,
    CardRules::new_creature(
        mana_cost!("{3}{B}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_ability(AbilityDef::triggered(
        "When this creature enters, sacrifice another creature. You gain X life and draw X cards, where X is that creature's power.",
        TriggerEventDef::ZoneChanged {
            object: ObjectPredicateDef::Source,
            from: None,
            to: Some(ZoneKind::Battlefield),
        },
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                // "Another" creature, so the Disciple cannot eat itself.
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            then: Some(&DISCIPLE_OF_BOLAS_PAYOFF),
            optional: false,
        },
    )),
);

// M13 89 — Disentomb
pub(in crate::card::sets) static DISENTOMB: CardRecord = CardRecord::new(
    cards::DISENTOMB,
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
            controller: None,
        },
    )),
);

// M13 90 — Duress
pub(in crate::card::sets) static DURESS: CardRecord = CardRecord::new(
    cards::DURESS,
    "Duress",
    CardArt::new("f7201d43-ae2e-4faa-a508-8555079c3bc7", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(
        AbilityDef::custom_full(
            "Target opponent reveals their hand. You choose a noncreature, nonland card from it. That player discards that card.",
            CardBehavior::Duress,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

/// Mutilate scales with your Swamps, and reads the same count twice.
static SWAMPS_YOU_CONTROL: ValueDef = ValueDef::CountMatchingObjects(&ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Swamp"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
});

// M13 91 — Duskmantle Prowler
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 92 — Duty-Bound Dead
// Audit: blocked — Needs executable exalted plus a regeneration action.

// M13 93 — Essence Drain
pub(in crate::card::sets) static ESSENCE_DRAIN: CardRecord = CardRecord::new(
    cards::ESSENCE_DRAIN,
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
pub(in crate::card::sets) static GIANT_SCORPION: CardRecord = CardRecord::new(
    cards::GIANT_SCORPION,
    "Giant Scorpion",
    CardArt::new("4097d5dc-46d3-4054-818f-a4ad8d7effe2", "Raymond Swanland"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Scorpion"], 1, 3)
        .with_abilities(&[abilities::deathtouch()]),
);

static HARBOR_BANDIT_ISLANDS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 95 — Harbor Bandit
pub(in crate::card::sets) static HARBOR_BANDIT: CardRecord = CardRecord::new(
    cards::HARBOR_BANDIT,
    "Harbor Bandit",
    CardArt::new("8422e109-de8d-46ea-a7f8-d5ccb6340497", "Jesper Ejsing"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control an Island.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::AnyMatchingObject(&HARBOR_BANDIT_ISLANDS),
                    toughness: ValueDef::AnyMatchingObject(&HARBOR_BANDIT_ISLANDS),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::activated(
            "{1}{U}: This creature can't be blocked this turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::MakeUnblockableThisTurn {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// M13 96 — Knight of Infamy
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 97 — Liliana of the Dark Realms
// Audit: blocked — Needs a choice between dynamic Swamp-count pump or shrink and an emblem that multiplies mana from Swamps.

// M13 98 — Liliana's Shade
pub(in crate::card::sets) static LILIANAS_SHADE: CardRecord = CardRecord::new(
    cards::LILIANAS_SHADE,
    "Liliana's Shade",
    CardArt::new(
        "1cf0c01d-a4a0-43fb-970d-e428e9ac63d7",
        "Eric Deschamps",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Shade"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, you may search your library for a Swamp card, reveal it, put it into your hand, then shuffle.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Swamp]),
                    minimum: 0,
                    maximum: 1,
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                },
            },
        ),
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(1),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 99 — Mark of the Vampire
pub(in crate::card::sets) static MARK_OF_THE_VAMPIRE: CardRecord = CardRecord::new(
    cards::MARK_OF_THE_VAMPIRE,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(2),
                            toughness: ValueDef::Constant(2),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                    ]),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// M13 100 — Mind Rot
pub(in crate::card::sets) static MIND_ROT: CardRecord = CardRecord::new(
    cards::MIND_ROT,
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
        },
    )),
);

// M13 101 — Murder
pub(in crate::card::sets) static MURDER: CardRecord = CardRecord::new(
    cards::MURDER,
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
pub(in crate::card::sets) static MUTILATE: CardRecord = CardRecord::new(
    cards::MUTILATE,
    "Mutilate",
    CardArt::new("c48bc86b-df0a-4a9c-8aad-c3ffb742a5ff", "Tyler Jacobson"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[AbilityDef::spell(
        "All creatures get -1/-1 until end of turn for each Swamp you control.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Negate(&SWAMPS_YOU_CONTROL),
                toughness: ValueDef::Negate(&SWAMPS_YOU_CONTROL),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// M13 103 — Nefarox, Overlord of Grixis
// Audit: blocked — Needs exalted's attacks-alone subject and the captured defending player for the sacrifice choice.

// M13 104 — Phylactery Lich
// Audit: blocked — Needs an as-enters artifact choice that receives a phylactery counter and a state trigger for controlling none.

// M13 105 — Public Execution
// Audit: blocked — A target-relative creature sweep cannot exclude the destroyed target when destruction is prevented or replaced.

// M13 106 — Ravenous Rats
pub(in crate::card::sets) static RAVENOUS_RATS: CardRecord = CardRecord::new(
    cards::RAVENOUS_RATS,
    "Ravenous Rats",
    CardArt::new("0642111c-f668-4acb-9df5-f0b920352407", "Carl Critchlow"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target opponent discards a card.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
            },
        ),
    ),
);

// M13 107 — Rise from the Grave
// Audit: blocked — Continuous effects cannot add black color and the Zombie subtype to the reanimated target indefinitely.

// M13 108 — Servant of Nefarox
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 109 — Shimian Specter
// Audit: blocked — Needs a combat-damage hand reveal and choice, then same-name searches across graveyard, hand, and library.

// M13 110 — Sign in Blood
pub(in crate::card::sets) static SIGN_IN_BLOOD: CardRecord = CardRecord::new(
    cards::SIGN_IN_BLOOD,
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
// Audit: blocked — Continuous combat restrictions cannot make this creature unable to block.

// M13 112 — Vampire Nighthawk
pub(in crate::card::sets) static VAMPIRE_NIGHTHAWK: CardRecord = CardRecord::new(
    cards::VAMPIRE_NIGHTHAWK,
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
// Audit: blocked — Needs persistent top-library revelation and a top-card-color-conditioned Vampire mass bonus and flying grant.

// M13 114 — Veilborn Ghoul
// Audit: blocked — Needs a land-entry trigger functioning from this card's graveyard and a self return from that zone.

// M13 115 — Vile Rebirth
pub(in crate::card::sets) static VILE_REBIRTH: CardRecord = CardRecord::new(
    cards::VILE_REBIRTH,
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
                controller: None,
            },
            EffectDef::CreateToken {
                token: cards::ZOMBIE_TOKEN_2_2_BLACK,
                count: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M13 117 — Wit's End
pub(in crate::card::sets) static WITS_END: CardRecord = CardRecord::new(
    cards::WITS_END,
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
        },
    )),
);

// M13 118 — Xathrid Gorgon
// Audit: blocked — Needs a resolving counter choice plus type, color, ability-removal, and defender changes keyed to that counter.

// M13 119 — Zombie Goliath
pub(in crate::card::sets) static ZOMBIE_GOLIATH: CardRecord = CardRecord::new(
    cards::ZOMBIE_GOLIATH,
    "Zombie Goliath",
    CardArt::new("8638edec-ddcd-4f50-9c2f-2e1668e3d175", "E. M. Gist"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Zombie", "Giant"], 4, 3),
);

// M13 120 — Arms Dealer
pub(in crate::card::sets) static ARMS_DEALER: CardRecord = CardRecord::new(
    cards::ARMS_DEALER,
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
pub(in crate::card::sets) static BLADETUSK_BOAR: CardRecord = CardRecord::new(
    cards::BLADETUSK_BOAR,
    "Bladetusk Boar",
    CardArt::new("d28442f9-06cf-4273-80a3-2b054f5881a4", "Paul Bonner"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Boar"], 3, 2)
        .with_abilities(&[abilities::intimidate()]),
);

// M13 122 — Canyon Minotaur
pub(in crate::card::sets) static CANYON_MINOTAUR: CardRecord = CardRecord::new(
    cards::CANYON_MINOTAUR,
    "Canyon Minotaur",
    CardArt::new("f8dc0efb-5847-4061-b386-9b4099361a58", "Steve Prescott"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur", "Warrior"], 3, 3),
);

// M13 123 — Chandra, the Firebrand
// Audit: blocked — Needs a delayed next-instant-or-sorcery trigger that copies the spell and supports retargeting.

// M13 124 — Chandra's Fury
pub(in crate::card::sets) static CHANDRAS_FURY: CardRecord = CardRecord::new(
    cards::CHANDRAS_FURY,
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
                    recipient: EffectRecipientDef::ObjectsControlledByTarget {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        slot: TargetIndex::PRIMARY,
                    },
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// M13 125 — Cleaver Riot
pub(in crate::card::sets) static CLEAVER_RIOT: CardRecord = CardRecord::new(
    cards::CLEAVER_RIOT,
    "Cleaver Riot",
    CardArt::new("6761eacf-03fc-4ccd-a4a6-eca5357b5c5b", "Dave Kendall"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell(
        "Creatures you control gain double strike until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::GrantAbility(&abilities::double_strike()),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 126 — Craterize
pub(in crate::card::sets) static CRATERIZE: CardRecord = CardRecord::new(
    cards::CRATERIZE,
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
// Audit: blocked — No resolving effect performs regeneration.

// M13 128 — Dragon Hatchling
pub(in crate::card::sets) static DRAGON_HATCHLING: CardRecord = CardRecord::new(
    cards::DRAGON_HATCHLING,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 129 — Fervor
pub(in crate::card::sets) static FERVOR: CardRecord = CardRecord::new(
    cards::FERVOR,
    "Fervor",
    CardArt::new("a88515c2-4b4f-4d16-9f50-149ef012e961", "Wayne England"),
    CardSet::Magic2013,
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have haste.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )),
);

// M13 131 — Firewing Phoenix
// Audit: blocked — Effect recipients cannot identify an activated ability's own source card while it is in a graveyard.

// M13 132 — Flames of the Firebrand
pub(in crate::card::sets) static FLAMES_OF_THE_FIREBRAND: CardRecord = CardRecord::new(
    cards::FLAMES_OF_THE_FIREBRAND,
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
                divided_total: Some(DividedTotal::Fixed(3)),
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ),
);

/// The damage and the tap name the same creatures, so both clauses ask the
/// same question.
const OPPOSING_FLIERS: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
]);

// M13 133 — Furnace Whelp
pub(in crate::card::sets) static FURNACE_WHELP: CardRecord = CardRecord::new(
    cards::FURNACE_WHELP,
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
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 134 — Goblin Arsonist
pub(in crate::card::sets) static GOBLIN_ARSONIST: CardRecord = CardRecord::new(
    cards::GOBLIN_ARSONIST,
    "Goblin Arsonist",
    CardArt::new("4d131369-db00-4a11-bd47-4401188b0f35", "Wayne Reynolds"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Shaman"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, you may have it deal 1 damage to any target.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: Some(ZoneKind::Graveyard),
            },
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
// Audit: blocked — No turn-long effect can make a target creature unable to block.

// M13 136 — Hamletback Goliath
// Audit: blocked — Trigger values cannot read the entering creature's power for the counter amount.

// M13 137 — Kindled Fury
pub(in crate::card::sets) static KINDLED_FURY: CardRecord = CardRecord::new(
    cards::KINDLED_FURY,
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
                AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                AppliedEffectDef::GrantAbility(&abilities::first_strike()),
            ]),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

static KRENKO_GOBLINS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Subtype("Goblin"),
    ]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 138 — Krenko, Mob Boss
pub(in crate::card::sets) static KRENKO_MOB_BOSS: CardRecord = CardRecord::new(
    cards::KRENKO_MOB_BOSS,
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
        EffectDef::CreateToken {
            token: cards::GOBLIN_TOKEN_1_1_RED,
            count: ValueDef::CountMatchingObjects(&KRENKO_GOBLINS),
        },
    )),
);

// M13 139 — Krenko's Command
pub(in crate::card::sets) static KRENKOS_COMMAND: CardRecord = CardRecord::new(
    cards::KRENKOS_COMMAND,
    "Krenko's Command",
    CardArt::new("84df41e9-e973-4441-b17f-434517134d46", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Create two 1/1 red Goblin creature tokens.",
        EffectDef::CreateToken {
            token: cards::GOBLIN_TOKEN_1_1_RED,
            count: ValueDef::Constant(2),
        },
    )),
);

// M13 140 — Magmaquake
// Audit: blocked — Flying predicates omit continuous static grants, so the creature sweep cannot use full current characteristics.

// M13 141 — Mark of Mutiny
pub(in crate::card::sets) static MARK_OF_MUTINY: CardRecord = CardRecord::new(
    cards::MARK_OF_MUTINY,
    "Mark of Mutiny",
    CardArt::new("0b7c6e09-3a14-4cc4-ba6b-f1f45e7d9f2a", "Mike Bierek"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Put a +1/+1 counter on it and untap it. That creature gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::GainControlThisTurn {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
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
                effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// M13 142 — Mindclaw Shaman
// Audit: blocked — Needs an opponent-hand reveal and choice followed by permission to cast the chosen card without paying its cost.

// M13 143 — Mogg Flunkies
// Audit: blocked — Combat restrictions cannot require another creature to attack or block alongside this source.

// M13 144 — Reckless Brute
pub(in crate::card::sets) static RECKLESS_BRUTE: CardRecord = CardRecord::new(
    cards::RECKLESS_BRUTE,
    "Reckless Brute",
    CardArt::new("5fd32a9e-1d39-4792-9657-69d17e5e0134", "Johann Bodin"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Ogre", "Warrior"], 3, 1).with_abilities(&[
        abilities::haste(),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    ]),
);

// M13 145 — Reverberate
// Audit: blocked — No effect can copy and optionally retarget an instant or sorcery spell.

// M13 146 — Rummaging Goblin
// Audit: partial — Discarding a card is not supported as an activated-ability cost.
pub(in crate::card::sets) static RUMMAGING_GOBLIN: CardRecord = CardRecord::new(
    cards::RUMMAGING_GOBLIN,
    "Rummaging Goblin",
    CardArt::new("cc5b622c-83a4-477e-a99c-2674e2bd6bb9", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::not_implemented(
            "{T}, Discard a card: Draw a card.",
            "Discarding a card is not supported as an activated-ability cost.",
        ),
    ),
);

// M13 147 — Searing Spear
pub(in crate::card::sets) static SEARING_SPEAR: CardRecord = CardRecord::new(
    cards::SEARING_SPEAR,
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
// Audit: blocked — Needs attack and block permission based on counter count plus an attack-at-you event that adds counters.

// M13 149 — Smelt
pub(in crate::card::sets) static SMELT: CardRecord = CardRecord::new(
    cards::SMELT,
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
// Audit: partial — Creature matching by flying ignores abilities granted or removed by static effects.
pub(in crate::card::sets) static THUNDERMAW_HELLKITE: CardRecord = CardRecord::new(
    cards::THUNDERMAW_HELLKITE,
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
        AbilityDef::triggered(
            "When this creature enters, it deals 1 damage to each creature with flying your opponents control. Tap those creatures.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: OPPOSING_FLIERS,
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Opponent,
                    },
                    amount: ValueDef::Constant(1),
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::MatchingObjects {
                        object: OPPOSING_FLIERS,
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::Opponent,
                    },
                },
            ]),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "Creature matching by flying ignores abilities granted or removed by static effects.",
        )),
    ]),
);

// M13 152 — Trumpet Blast
pub(in crate::card::sets) static TRUMPET_BLAST: CardRecord = CardRecord::new(
    cards::TRUMPET_BLAST,
    "Trumpet Blast",
    CardArt::new("4ac9f745-236a-4302-acf2-21c14c6e6eab", "Carl Critchlow"),
    CardSet::Magic2013,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Attacking creatures get +2/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::Attacking,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(2),
                toughness: ValueDef::Constant(0),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 153 — Turn to Slag
// Audit: blocked — No effect can discover and destroy every Equipment attached to the damaged creature.

// M13 154 — Volcanic Geyser
pub(in crate::card::sets) static VOLCANIC_GEYSER: CardRecord = CardRecord::new(
    cards::VOLCANIC_GEYSER,
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
pub(in crate::card::sets) static VOLCANIC_STRENGTH: CardRecord = CardRecord::new(
    cards::VOLCANIC_STRENGTH,
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
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(2),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&abilities::mountainwalk()),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ]),
        ),
    ]),
);

// M13 157 — Wild Guess
// Audit: blocked — Spell definitions cannot require discarding a card as an additional casting cost.

// M13 158 — Worldfire
// Audit: blocked — Needs simultaneous mass exile across zones plus a life-total setter.

// M13 159 — Acidic Slime
pub(in crate::card::sets) static ACIDIC_SLIME: CardRecord = CardRecord::new(
    cards::ACIDIC_SLIME,
    "Acidic Slime",
    CardArt::new("bd7bef5a-e0ab-46d3-a802-620bf2a7546f", "Karl Kopinski"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Ooze"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered_with_targets(
            "When this creature enters, destroy target artifact, enchantment, or land.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
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
            },
        ),
    ]),
);

// M13 160 — Arbor Elf
pub(in crate::card::sets) static ARBOR_ELF: CardRecord = CardRecord::new(
    cards::ARBOR_ELF,
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

/// A second Mountain does not make the bonus bigger, so this is asked as a
/// condition rather than counted.
static MOUNTAIN_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::Subtype("Mountain"),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 161 — Bond Beetle
pub(in crate::card::sets) static BOND_BEETLE: CardRecord = CardRecord::new(
    cards::BOND_BEETLE,
    "Bond Beetle",
    CardArt::new("f341ed2c-353b-49a3-b200-94ae43cb8e24", "John Avon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{G}"), &["Insect"], 0, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, put a +1/+1 counter on target creature.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
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
    ),
);

// M13 162 — Boundless Realms
// Audit: blocked — SearchZone has a static maximum and cannot make the selected lands enter tapped.

static BOUNTIFUL_HARVEST_LANDS: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Land),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 163 — Bountiful Harvest
pub(in crate::card::sets) static BOUNTIFUL_HARVEST: CardRecord = CardRecord::new(
    cards::BOUNTIFUL_HARVEST,
    "Bountiful Harvest",
    CardArt::new("8d7a4494-2ced-4405-9204-d2617961a1d6", "Jason Chan"),
    CardSet::Magic2013,
    CardRules::new_sorcery(mana_cost!("{4}{G}")).with_ability(AbilityDef::spell(
        "You gain 1 life for each land you control.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::CountMatchingObjects(&BOUNTIFUL_HARVEST_LANDS),
        },
    )),
);

// M13 164 — Centaur Courser
pub(in crate::card::sets) static CENTAUR_COURSER: CardRecord = CardRecord::new(
    cards::CENTAUR_COURSER,
    "Centaur Courser",
    CardArt::new("44a5f7db-ea4e-4af5-9d4a-0335db6ea0e9", "Vance Kovacs"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Centaur", "Warrior"], 3, 3),
);

// M13 165 — Deadly Recluse
pub(in crate::card::sets) static DEADLY_RECLUSE: CardRecord = CardRecord::new(
    cards::DEADLY_RECLUSE,
    "Deadly Recluse",
    CardArt::new("a32a5f77-7c1f-4da4-9ae6-3947504a8dea", "Warren Mahy"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Spider"], 1, 2)
        .with_abilities(&[abilities::reach(), abilities::deathtouch()]),
);

// M13 166 — Duskdale Wurm
pub(in crate::card::sets) static DUSKDALE_WURM: CardRecord = CardRecord::new(
    cards::DUSKDALE_WURM,
    "Duskdale Wurm",
    CardArt::new("7d1a2d9a-e14c-4c44-8cf1-a2ce09bdae27", "Dan Dos Santos"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Wurm"], 7, 7)
        .with_abilities(&[abilities::trample()]),
);

// M13 167 — Elderscale Wurm
// Audit: blocked — Needs conditional life-total setting on entry and a damage replacement that enforces a life floor.

// M13 168 — Elvish Archdruid
// Audit: blocked — Activated mana abilities cannot produce an amount derived from the controller's Elf count.

// M13 169 — Elvish Visionary
pub(in crate::card::sets) static ELVISH_VISIONARY: CardRecord = CardRecord::new(
    cards::ELVISH_VISIONARY,
    "Elvish Visionary",
    CardArt::new(
        "65ea2998-ed91-43b8-bd81-b01a6c24a5b0",
        "D. Alexander Gregory",
    ),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Shaman"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, draw a card.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// M13 170 — Farseek
// Audit: blocked — Search-to-battlefield cannot make the found land enter tapped.

// M13 171 — Flinthoof Boar
pub(in crate::card::sets) static FLINTHOOF_BOAR: CardRecord = CardRecord::new(
    cards::FLINTHOOF_BOAR,
    "Flinthoof Boar",
    CardArt::new("7e380b99-0173-4083-a4a2-222ad98b904a", "Erica Yang"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Boar"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +1/+1 as long as you control a Mountain.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                    toughness: ValueDef::AnyMatchingObject(&MOUNTAIN_YOU_CONTROL),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gains haste until end of turn. (It can attack and {T} this turn.)",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::haste()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M13 172 — Fog
// Audit: blocked — Per-object combat prevention cannot also cover creatures that enter after the spell resolves.

// M13 173 — Fungal Sprouting
// Audit: blocked — No value expression computes the greatest power among creatures the controller controls.

// M13 174 — Garruk, Primal Hunter
// Audit: blocked — Needs greatest-power evaluation and a token count derived from that value.

// M13 175 — Garruk's Packleader
// Audit: blocked — Power predicates omit continuous static bonuses, so the entry trigger cannot test full current power.

// M13 176 — Ground Seal
// Audit: blocked — No static effect prohibits targeting cards in graveyards.

// M13 177 — Mwonvuli Beast Tracker
// Audit: blocked — Keyword predicates omit continuous static grants, so the search cannot test full current abilities.

// M13 179 — Plummet
// Audit: blocked — Flying predicates omit continuous static grants, so target legality cannot use full current characteristics.

// M13 180 — Predatory Rampage
// Audit: blocked — No turn-long effect requires creatures to block if able.

// M13 182 — Primal Huntbeast
pub(in crate::card::sets) static PRIMAL_HUNTBEAST: CardRecord = CardRecord::new(
    cards::PRIMAL_HUNTBEAST,
    "Primal Huntbeast",
    CardArt::new("eb77f6a8-a9d6-4fdd-996e-70877199ebab", "Chris Rahn"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 3, 3)
        .with_abilities(&[abilities::hexproof()]),
);

// M13 183 — Primordial Hydra
// Audit: blocked — Needs X entry counters, counter doubling each upkeep, and a counter-threshold trample grant.

// M13 185 — Rancor
// Audit: blocked — A leave-the-battlefield trigger cannot address its source card after it becomes a new graveyard object.

// M13 186 — Ranger's Path
// Audit: blocked — Multi-card battlefield searches and tapped entry are outside SearchZone's current runtime boundary.

// M13 187 — Revive
pub(in crate::card::sets) static REVIVE: CardRecord = CardRecord::new(
    cards::REVIVE,
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
            controller: None,
        },
    )),
);

// M13 188 — Roaring Primadox
// Audit: blocked — No resolving choice selects a nontarget creature the controller owns for a mandatory return.

// M13 189 — Sentinel Spider
pub(in crate::card::sets) static SENTINEL_SPIDER: CardRecord = CardRecord::new(
    cards::SENTINEL_SPIDER,
    "Sentinel Spider",
    CardArt::new("5f55ff4b-f0e1-498b-982b-e6ec01d30d95", "Vincent Proce"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Spider"], 4, 4)
        .with_abilities(&[abilities::reach(), abilities::vigilance()]),
);

// M13 190 — Serpent's Gift
pub(in crate::card::sets) static SERPENTS_GIFT: CardRecord = CardRecord::new(
    cards::SERPENTS_GIFT,
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
            effect: AppliedEffectDef::GrantAbility(&abilities::deathtouch()),
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 191 — Silklash Spider
// Audit: blocked — Flying predicates omit continuous static grants, so the X-damage sweep cannot use full current characteristics.

// M13 192 — Spiked Baloth
pub(in crate::card::sets) static SPIKED_BALOTH: CardRecord = CardRecord::new(
    cards::SPIKED_BALOTH,
    "Spiked Baloth",
    CardArt::new("522777b1-a89f-4969-a962-0137018ec86c", "Daarken"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Beast"], 4, 2)
        .with_abilities(&[abilities::trample()]),
);

// M13 193 — Thragtusk
pub(in crate::card::sets) static THRAGTUSK: CardRecord = CardRecord::new(
    cards::THRAGTUSK,
    "Thragtusk",
    CardArt::new("28667c8b-d02c-4e57-a050-1549207b65d1", "Nils Hamm"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 5, 3).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, you gain 5 life.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, create a 3/3 green Beast creature token.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: None,
            },
            EffectDef::CreateToken {
                token: cards::BEAST_TOKEN_3_3_GREEN,
                count: ValueDef::Constant(1),
            },
        ),
    ]),
);

static OTHER_TIMBERPACK_WOLVES: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::SharesNameWithSource,
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

// M13 194 — Timberpack Wolf
pub(in crate::card::sets) static TIMBERPACK_WOLF: CardRecord = CardRecord::new(
    cards::TIMBERPACK_WOLF,
    "Timberpack Wolf",
    CardArt::new("d16928c9-0470-46ec-b92d-0d6ff9f23ef7", "John Avon"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each other creature you control named Timberpack Wolf.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::CountMatchingObjects(&OTHER_TIMBERPACK_WOLVES),
                    toughness: ValueDef::CountMatchingObjects(&OTHER_TIMBERPACK_WOLVES),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        ),
    ),
);

// M13 195 — Titanic Growth
pub(in crate::card::sets) static TITANIC_GROWTH: CardRecord = CardRecord::new(
    cards::TITANIC_GROWTH,
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
            effect: AppliedEffectDef::ModifyPowerToughness {
                power: ValueDef::Constant(4),
                toughness: ValueDef::Constant(4),
            },
            duration: EffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M13 196 — Vastwood Gorger
pub(in crate::card::sets) static VASTWOOD_GORGER: CardRecord = CardRecord::new(
    cards::VASTWOOD_GORGER,
    "Vastwood Gorger",
    CardArt::new("70fc4a5f-1c59-4139-a506-72baebb1168f", "Kieran Yanner"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Wurm"], 5, 6),
);

// M13 197 — Yeva, Nature's Herald
// Audit: blocked — No static permission grants flash-like casting timing to green creature cards.

// M13 198 — Yeva's Forcemage
pub(in crate::card::sets) static YEVAS_FORCEMAGE: CardRecord = CardRecord::new(
    cards::YEVAS_FORCEMAGE,
    "Yeva's Forcemage",
    CardArt::new("3f9ebf02-56b3-492e-88fb-2e95f13f5764", "Eric Deschamps"),
    CardSet::Magic2013,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gets +2/+2 until end of turn.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(2),
                    toughness: ValueDef::Constant(2),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M13 199 — Nicol Bolas, Planeswalker
// Audit: blocked — Needs indefinite control change plus linked seven-card discard and seven-permanent sacrifice choices.

// M13 200 — Akroma's Memorial
pub(in crate::card::sets) static AKROMAS_MEMORIAL: CardRecord = CardRecord::new(
    cards::AKROMAS_MEMORIAL,
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
            EffectDef::Apply {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::You,
                },
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::GrantAbility(&abilities::flying()),
                    AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    AppliedEffectDef::GrantAbility(&abilities::vigilance()),
                    AppliedEffectDef::GrantAbility(&abilities::trample()),
                    AppliedEffectDef::GrantAbility(&abilities::haste()),
                    AppliedEffectDef::GrantAbility(&abilities::protection_from(ManaColor::Black)),
                    AppliedEffectDef::GrantAbility(&abilities::protection_from(ManaColor::Red)),
                ]),
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )),
);

// M13 201 — Chronomaton
pub(in crate::card::sets) static CHRONOMATON: CardRecord = CardRecord::new(
    cards::CHRONOMATON,
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

// M13 202 — Clock of Omens
// Audit: blocked — Ability costs cannot tap two separately chosen untapped artifacts.

// M13 203 — Door to Nothingness
pub(in crate::card::sets) static DOOR_TO_NOTHINGNESS: CardRecord = CardRecord::new(
    cards::DOOR_TO_NOTHINGNESS,
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
// Audit: blocked — Needs a simultaneous source-and-graveyard library move followed by one shuffle.

// M13 205 — Gem of Becoming
// Audit: blocked — Needs three separate library searches for cards with three different names.

// M13 206 — Gilded Lotus
pub(in crate::card::sets) static GILDED_LOTUS: CardRecord = CardRecord::new(
    cards::GILDED_LOTUS,
    "Gilded Lotus",
    CardArt::new("33704052-aeb1-4798-a64d-778e1879eeb9", "Martina Pilcerova"),
    CardSet::Magic2013,
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated_mana(
        "{T}: Add three mana of any one color.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )),
);

// M13 208 — Kitesail
// Audit: blocked — Equipment attachment and equip costs are not declaratively supported.

// M13 209 — Phyrexian Hulk
pub(in crate::card::sets) static PHYREXIAN_HULK: CardRecord = CardRecord::new(
    cards::PHYREXIAN_HULK,
    "Phyrexian Hulk",
    CardArt::new("a761426e-2138-438e-8f3b-024486165260", "Steven Belledin"),
    CardSet::Magic2013,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Phyrexian", "Golem"], 5, 4),
);

// M13 210 — Primal Clay
// Audit: blocked — Needs an as-enters modal choice that sets one of three characteristic and keyword packages.

// M13 211 — Ring of Evos Isle
// Audit: blocked — Needs Equipment attachment plus an upkeep bonus conditioned on the attached creature's color.

// M13 212 — Ring of Kalonia
// Audit: blocked — Needs Equipment attachment plus an upkeep bonus conditioned on the attached creature's color.

// M13 213 — Ring of Thune
// Audit: blocked — Needs Equipment attachment plus an upkeep bonus conditioned on the attached creature's color.

// M13 214 — Ring of Valkas
// Audit: blocked — Needs Equipment attachment plus an upkeep bonus conditioned on the attached creature's color.

// M13 215 — Ring of Xathrid
// Audit: blocked — Needs Equipment attachment, a color-conditioned upkeep bonus, and regeneration.

// M13 216 — Sands of Delirium
pub(in crate::card::sets) static SANDS_OF_DELIRIUM: CardRecord = CardRecord::new(
    cards::SANDS_OF_DELIRIUM,
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
pub(in crate::card::sets) static STAFF_OF_NIN: CardRecord = CardRecord::new(
    cards::STAFF_OF_NIN,
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

// M13 218 — Stuffy Doll
// Audit: blocked — Effects cannot route triggered damage to the player chosen as this permanent entered.

// M13 220 — Trading Post
// Audit: blocked — Needs four modes whose costs include unsupported discard or separately chosen sacrifice costs and their linked continuations.

// M13 221 — Cathedral of War
// Audit: blocked — Exalted needs an attacks-alone event and access to the lone attacking creature.

// M13 222 — Dragonskull Summit
pub(in crate::card::sets) static DRAGONSKULL_SUMMIT: CardRecord = CardRecord::new(
    cards::DRAGONSKULL_SUMMIT,
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
pub(in crate::card::sets) static DROWNED_CATACOMB: CardRecord = CardRecord::new(
    cards::DROWNED_CATACOMB,
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

// M13 225 — Glacial Fortress
pub(in crate::card::sets) static GLACIAL_FORTRESS: CardRecord = CardRecord::new(
    cards::GLACIAL_FORTRESS,
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
// Audit: blocked — The counter vocabulary has no pressure counter, so the add and remove costs cannot use the printed counter kind.

// M13 227 — Reliquary Tower
// Audit: blocked — No static effect removes a player's maximum hand size.

// M13 228 — Rootbound Crag
pub(in crate::card::sets) static ROOTBOUND_CRAG: CardRecord = CardRecord::new(
    cards::ROOTBOUND_CRAG,
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
pub(in crate::card::sets) static SUNPETAL_GROVE: CardRecord = CardRecord::new(
    cards::SUNPETAL_GROVE,
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AJANIS_SUNSTRIKER,
    &ATTENDED_KNIGHT,
    &BATTLEFLIGHT_EAGLE,
    &CAPTAIN_OF_THE_WATCH,
    &CAPTAINS_CALL,
    &CRUSADER_OF_ODRIC,
    &DIVINE_FAVOR,
    &DIVINE_VERDICT,
    &ERASE,
    &GLORIOUS_CHARGE,
    &GRIFFIN_PROTECTOR,
    &GUARDIAN_LIONS,
    &HEALER_OF_THE_PRIDE,
    &OBLIVION_RING,
    &PILLARFIELD_OX,
    &PLANAR_CLEANSING,
    &PRIZED_ELEPHANT,
    &RAIN_OF_BLADES,
    &RHOX_FAITHMENDER,
    &SHOW_OF_VALOR,
    &SILVERCOAT_LION,
    &WAR_PRIEST_OF_THUNE,
    &WARCLAMP_MASTIFF,
    &ARCHAEOMANCER,
    &ARCTIC_AVEN,
    &AUGUR_OF_BOLAS,
    &BATTLE_OF_WITS,
    &DOWNPOUR,
    &ESSENCE_SCATTER,
    &FAERIE_INVADERS,
    &HYDROSURGE,
    &JACE_MEMORY_ADEPT,
    &KRAKEN_HATCHLING,
    &MIND_SCULPT,
    &NEGATE,
    &SCROLL_THIEF,
    &SPHINX_OF_UTHUUN,
    &TALRAND_SKY_SUMMONER,
    &TALRANDS_INVOCATION,
    &TRICKS_OF_THE_TRADE,
    &VEDALKEN_ENTRANCER,
    &WATERCOURSER,
    &WIND_DRAKE,
    &BLOOD_RECKONING,
    &BLOODHUNTER_BAT,
    &BLOODTHRONE_VAMPIRE,
    &COWER_IN_FEAR,
    &DARK_FAVOR,
    &DISCIPLE_OF_BOLAS,
    &DISENTOMB,
    &DURESS,
    &ESSENCE_DRAIN,
    &GIANT_SCORPION,
    &HARBOR_BANDIT,
    &LILIANAS_SHADE,
    &MARK_OF_THE_VAMPIRE,
    &MIND_ROT,
    &MURDER,
    &MUTILATE,
    &RAVENOUS_RATS,
    &SIGN_IN_BLOOD,
    &VAMPIRE_NIGHTHAWK,
    &VILE_REBIRTH,
    &WITS_END,
    &ZOMBIE_GOLIATH,
    &ARMS_DEALER,
    &BLADETUSK_BOAR,
    &CANYON_MINOTAUR,
    &CHANDRAS_FURY,
    &CLEAVER_RIOT,
    &CRATERIZE,
    &DRAGON_HATCHLING,
    &FERVOR,
    &FLAMES_OF_THE_FIREBRAND,
    &FURNACE_WHELP,
    &GOBLIN_ARSONIST,
    &KINDLED_FURY,
    &KRENKO_MOB_BOSS,
    &KRENKOS_COMMAND,
    &MARK_OF_MUTINY,
    &RECKLESS_BRUTE,
    &RUMMAGING_GOBLIN,
    &SEARING_SPEAR,
    &SMELT,
    &THUNDERMAW_HELLKITE,
    &TRUMPET_BLAST,
    &VOLCANIC_GEYSER,
    &VOLCANIC_STRENGTH,
    &ACIDIC_SLIME,
    &ARBOR_ELF,
    &BOND_BEETLE,
    &BOUNTIFUL_HARVEST,
    &CENTAUR_COURSER,
    &DEADLY_RECLUSE,
    &DUSKDALE_WURM,
    &ELVISH_VISIONARY,
    &FLINTHOOF_BOAR,
    &PRIMAL_HUNTBEAST,
    &REVIVE,
    &SENTINEL_SPIDER,
    &SERPENTS_GIFT,
    &SPIKED_BALOTH,
    &THRAGTUSK,
    &TIMBERPACK_WOLF,
    &TITANIC_GROWTH,
    &VASTWOOD_GORGER,
    &YEVAS_FORCEMAGE,
    &AKROMAS_MEMORIAL,
    &CHRONOMATON,
    &DOOR_TO_NOTHINGNESS,
    &GILDED_LOTUS,
    &PHYREXIAN_HULK,
    &SANDS_OF_DELIRIUM,
    &STAFF_OF_NIN,
    &DRAGONSKULL_SUMMIT,
    &DROWNED_CATACOMB,
    &GLACIAL_FORTRESS,
    &ROOTBOUND_CRAG,
    &SUNPETAL_GROVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&avacyn_restored::ANGELS_MERCY), // M13 3
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),            // M13 31
    PrintingRecord::reprint(&alpha::CLONE),                  // M13 45
    PrintingRecord::reprint(&dark_ascension::DIVINATION),    // M13 47
    PrintingRecord::reprint(&alpha::MERFOLK_OF_THE_PEARL_TRIDENT), // M13 60
    PrintingRecord::reprint(&alpha::UNSUMMON),               // M13 75
    PrintingRecord::reprint(&innistrad::WALKING_CORPSE),     // M13 116
    PrintingRecord::reprint(&alpha::FIRE_ELEMENTAL),         // M13 130
    PrintingRecord::reprint(&dark_ascension::TORCH_FIEND),   // M13 151
    PrintingRecord::reprint(&alpha::WALL_OF_FIRE),           // M13 156
    PrintingRecord::reprint(&onslaught::NATURALIZE),         // M13 178
    PrintingRecord::reprint(&innistrad::PREY_UPON),          // M13 181
    PrintingRecord::reprint(&planeshift::QUIRION_DRYAD),     // M13 184
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),          // M13 207
    PrintingRecord::reprint(&the_dark::TORMODS_CRYPT),       // M13 219
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
