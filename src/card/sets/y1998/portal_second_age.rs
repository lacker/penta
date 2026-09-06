//! Portal Second Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::abilities;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, AppliedRuleDef,
    CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef,
    SacrificedAmountDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

// P02 8 — Angel of Mercy
pub(in crate::card::sets) static ANGEL_OF_MERCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dac5c913-4eb5-4cfb-9c24-223f14f07064"),
    "Angel of Mercy",
    CardArt::new("dac5c913-4eb5-4cfb-9c24-223f14f07064", "Melissa A. Benson"),
    CardSet::PortalSecondAge,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, you gain 3 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// P02 15 — Breath of Life
pub(in crate::card::sets) static BREATH_OF_LIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcea5e09-6385-41df-970b-ac26c9b46127"),
    "Breath of Life",
    CardArt::new("a10f24f7-f82e-413e-824f-384607c7d858", "Lubov"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the battlefield.",
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
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// P02 18 — Path of Peace
pub(in crate::card::sets) static PATH_OF_PEACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1f3e1c9-bfad-49a1-b171-6fa344ef2eef"),
    "Path of Peace",
    CardArt::new("cb14d3f4-09f3-4113-bdc3-0fd753137f7c", "David A. Cherry"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. Its owner gains 4 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// P02 27 — Vengeance
pub(in crate::card::sets) static VENGEANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c91c249b-157c-4f1d-8171-29d1e75b1c9f"),
    "Vengeance",
    CardArt::new("3209ee48-4485-44fc-b71d-cd6241674e64", "Keith Parkinson"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target tapped creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Tapped,
        ])),
    )),
);

// P02 37 — Exhaustion
pub(in crate::card::sets) static EXHAUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d6a5c33-cf74-4cec-a4f4-1aac9e7b8f79"),
    "Exhaustion",
    CardArt::new("fcc103a6-7888-4e35-b35b-a796a48caf70", "Kaja Foglio"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Creatures and lands target opponent controls don't untap during their next untap step.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Opponent),
        )],
        EffectDef::SkipNextUntapSteps {
            object: EffectRecipientDef::objects_controlled_by_target(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                TargetIndex::PRIMARY,
            ),
            count: 1,
        },
    )),
);

// P02 46 — Sleight of Hand
pub(in crate::card::sets) static SLEIGHT_OF_HAND: CardRecord = CardRecord::new_with_legacy_id(
    311,
    "Sleight of Hand",
    CardArt::new("f3405184-dcda-4bb6-ade6-c2a87bc3296d", "Phil Foglio"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Look at the top two cards of your library. Put one of them into your hand and the other on the bottom of your library.",
        abilities::look_at_top_cards_choose_to_hand_rest_bottom(
            ValueDef::Constant(2),
            ObjectPredicateDef::Any,
            1,
            1,
        ),
    )),
);

// P02 91 — Blaze
pub(in crate::card::sets) static BLAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04095ad2-7308-4e26-b9ef-070a5755d066"),
    "Blaze",
    CardArt::new("3940d0ca-0ca2-4446-9330-a554c3e89824", "David A. Cherry"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Blaze deals X damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// P02 98 — Goblin Glider
pub(in crate::card::sets) static GOBLIN_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c29491b-dec1-429d-9950-062582f8164f"),
    "Goblin Glider",
    CardArt::new("9c29491b-dec1-429d-9950-062582f8164f", "Pete Venters"),
    CardSet::PortalSecondAge,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ]),
);

// P02 103 — Goblin Raider
pub(in crate::card::sets) static GOBLIN_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("68fe9691-d788-42cb-8d13-005724939b62"),
    "Goblin Raider",
    CardArt::new("68fe9691-d788-42cb-8d13-005724939b62", "Matt Stawicki"),
    CardSet::PortalSecondAge,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ),
);

// P02 105 — Goblin War Strike
pub(in crate::card::sets) static GOBLIN_WAR_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("738fecfd-1119-4dcb-acd6-ec9715d9c074"),
    "Goblin War Strike",
    CardArt::new("738fecfd-1119-4dcb-acd6-ec9715d9c074", "Michael Weaver"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Goblin War Strike deals damage to target player or planeswalker equal to the number of Goblins you control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::Subtype("Goblin"),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        },
    )),
);

// P02 106 — Jagged Lightning
pub(in crate::card::sets) static JAGGED_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("148e6704-9cf0-45cf-9bab-db318c016593"),
    "Jagged Lightning",
    CardArt::new("148e6704-9cf0-45cf-9bab-db318c016593", "Michael Weaver"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{3}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Jagged Lightning deals 3 damage to each of two target creatures.",
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )),
);

// P02 112 — Ogre Taskmaster
pub(in crate::card::sets) static OGRE_TASKMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d674a92e-b268-48f7-b082-f8ca2e63d43b"),
    "Ogre Taskmaster",
    CardArt::new("d674a92e-b268-48f7-b082-f8ca2e63d43b", "Dan Frazier"),
    CardSet::PortalSecondAge,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Ogre"], 4, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ),
);

// P02 119 — Volcanic Hammer
pub(in crate::card::sets) static VOLCANIC_HAMMER: CardRecord = CardRecord::new_with_legacy_id(
    273,
    "Volcanic Hammer",
    CardArt::new(
        "58c0489d-b073-4ad4-b044-447fcc865b6c",
        "Edward P. Beard, Jr.",
    ),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Volcanic Hammer deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )),
);

// P02 120 — Wildfire
pub(in crate::card::sets) static WILDFIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b69cfcb0-db68-4494-a3e1-7c2ca279fcf5"),
    "Wildfire",
    CardArt::new("b69cfcb0-db68-4494-a3e1-7c2ca279fcf5", "Rob Alexander"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(AbilityDef::spell(
        "Each player sacrifices four lands of their choice. Wildfire deals 4 damage to each creature.",
        EffectDef::Sequence(&[
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EachPlayer,
                object: ObjectPredicateDef::HasType(CardType::Land),
                count: ValueDef::Constant(4),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// P02 131 — Lone Wolf
pub(in crate::card::sets) static LONE_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ff4d831-7388-4321-a636-79cf7bde25bb"),
    "Lone Wolf",
    CardArt::new("7ff4d831-7388-4321-a636-79cf7bde25bb", "Michael Weaver"),
    CardSet::PortalSecondAge,
    // Blocking it stops nothing, so it is really a 2/2 that the defender
    // can only trade with, never wall off.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wolf"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "You may have this creature assign its combat damage as though it weren't blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(
                    AppliedRuleDef::MayAssignCombatDamageAsThoughUnblocked,
                ),
            },
        ),
    ]),
);

// P02 133 — Monstrous Growth
pub(in crate::card::sets) static MONSTROUS_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0523c816-dddf-4b63-8db8-5e41dc673e5f"),
    "Monstrous Growth",
    CardArt::new("3816da20-4434-4bf7-a9dd-3eb3bb735f08", "Una Fricker"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGEL_OF_MERCY,
    &BREATH_OF_LIFE,
    &PATH_OF_PEACE,
    &VENGEANCE,
    &EXHAUSTION,
    &SLEIGHT_OF_HAND,
    &BLAZE,
    &GOBLIN_GLIDER,
    &GOBLIN_RAIDER,
    &GOBLIN_WAR_STRIKE,
    &JAGGED_LIGHTNING,
    &OGRE_TASKMASTER,
    &VOLCANIC_HAMMER,
    &WILDFIRE,
    &LONE_WOLF,
    &MONSTROUS_GROWTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
