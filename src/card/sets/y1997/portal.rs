//! Portal card records.

use super::{CardRecord, PrintingRecord};
use crate::AbilityDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::AppliedEffectDef;
use crate::AppliedRuleDef;
use crate::CardRules;
use crate::CardSet;
use crate::CardType;
use crate::DiscardSelectionDef;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ObjectPredicateDef;
use crate::PlayerRelation;
use crate::TargetIndex;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::abilities;
use crate::{BasicLandType, ObjectRefDef, PlayerRefDef, ResolvedEffectDurationDef};

use crate::mana_cost;

// POR 1 — Alabaster Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Alabaster Dragon",
    "1edc6ec1-3b34-45e0-8573-39eba1d10efa",
    "Ted Naifeh",
    crate::card::CardRules::unsupported(),
);

// POR 2 — Angelic Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Angelic Blessing",
    "31dda640-2a00-437e-855f-173c487e7395",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// POR 4 — Ardent Militia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_MILITIA: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Ardent Militia",
    "543f8c6a-bcf1-4400-82e5-83d36cb60464",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// POR 6 — Armored Pegasus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMORED_PEGASUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Armored Pegasus",
    "a81b61af-cdb7-468f-9ff0-db82aa084023",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// POR 7 — Blessed Reversal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_REVERSAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Blessed Reversal",
    "899ecc19-8106-4e5a-bb25-aaea9684ba0e",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// POR 10 — Breath of Life
pub(in crate::card::sets) static BREATH_OF_LIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Breath of Life",
    "bcea5e09-6385-41df-970b-ac26c9b46127",
    "DiTerlizzi",
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

// POR 11 — Charging Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Charging Paladin",
    "29db1bbf-a6cf-460c-bec8-dbd682157af4",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// POR 20 — Knight Errant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_ERRANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Knight Errant",
    "9c31b4b4-18fc-4a6e-8d74-fd5340964320",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// POR 21 — Path of Peace
pub(in crate::card::sets) static PATH_OF_PEACE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Path of Peace",
    "a1f3e1c9-bfad-49a1-b171-6fa344ef2eef",
    "Pete Venters",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature. Its owner gains 4 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
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

// POR 22 — Regal Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REGAL_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Regal Unicorn",
    "daa1fb8c-12fa-4e9c-979f-55e89356acaf",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// POR 25 — Sacred Nectar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_NECTAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Sacred Nectar",
    "484d1b31-5363-49ef-9b13-2005568636c1",
    "Janine Johnston",
    crate::card::CardRules::unsupported(),
);

// POR 26 — Seasoned Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASONED_MARSHAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Seasoned Marshal",
    "17db0060-3667-4c8c-ae9b-d62dceac64e3",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// POR 29 — Starlight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Starlight",
    "f6992524-6921-473b-8301-cb63fe502600",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// POR 35 — Venerable Monk
pub(in crate::card::sets) static VENERABLE_MONK: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Venerable Monk",
    "72322032-c287-4a9e-9d61-a452f6c45bfb",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Monk", "Cleric"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// POR 36 — Vengeance
pub(in crate::card::sets) static VENGEANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Vengeance",
    "c91c249b-157c-4f1d-8171-29d1e75b1c9f",
    "Andrew Robinson",
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target tapped creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Tapped,
        ])),
        true,
    )),
);

// POR 42 — Baleful Stare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALEFUL_STARE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Baleful Stare",
    "49fb46c8-30ae-4457-a726-6fe1ddd183d5",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// POR 47 — Cloud Spirit
pub(in crate::card::sets) static CLOUD_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Cloud Spirit",
    "cc7547aa-fcf7-4b6e-955d-cc5ebc40cd7d",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spirit"], 3, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// POR 54 — Exhaustion
pub(in crate::card::sets) static EXHAUSTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Exhaustion",
    "9d6a5c33-cf74-4cec-a4f4-1aac9e7b8f79",
    "DiTerlizzi",
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

// POR 55 — Flux
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLUX: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Flux",
    "3c26bf66-8fa8-4f69-9556-c9fcc56a7f33",
    "Ted Naifeh",
    crate::card::CardRules::unsupported(),
);

// POR 56 — Giant Octopus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_OCTOPUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Giant Octopus",
    "4528edca-cc36-4f63-9615-24ca315d672c",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// POR 57 — Horned Turtle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_TURTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Horned Turtle",
    "a7d25497-36b4-48b9-ba01-f24f6222d6be",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// POR 65 — Phantom Warrior
pub(in crate::card::sets) static PHANTOM_WARRIOR: CardRecord = CardRecord::new(
    CardSet::Portal,
    "Phantom Warrior",
    "6dbcb0df-d1cc-4718-ba1e-b590852cce20",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Illusion", "Warrior"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ),
);

// POR 72 — Theft of Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THEFT_OF_DREAMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Theft of Dreams",
    "29019e28-4ef8-4732-9972-0a47305fe303",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// POR 74 — Tidal Surge
pub(in crate::card::sets) static TIDAL_SURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Tidal Surge",
    "a027c31d-c662-4ce1-a0d1-a32e62f6a724",
    "Douglas Shuler",
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap up to three target creatures without flying.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
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

// POR 75 — Time Ebb
pub(in crate::card::sets) static TIME_EBB: CardRecord = CardRecord::new(
    CardSet::Portal,
    "Time Ebb",
    "e5fd26ca-dc7d-453d-8653-7f967e8f6dc7",
    "Alan Rabinowitz",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature on top of its owner's library.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// POR 77 — Wind Drake
pub(in crate::card::sets) static WIND_DRAKE: CardRecord = CardRecord::new(
    CardSet::Portal,
    "Wind Drake",
    "5486d2dc-9a5d-4f58-a5ec-d94de54b852f",
    "Zina Saunders",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Drake"], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

// POR 82 — Bog Raiders
pub(in crate::card::sets) static BOG_RAIDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Bog Raiders",
    "eb7bbb7a-b59a-4a01-b1cb-66eef881ffcd",
    "Steve Luke",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Zombie"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// POR 95 — Gravedigger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEDIGGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Gravedigger",
    "b979d70e-d514-420f-886c-f60e2bb1861f",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// POR 101 — Mind Rot
pub(in crate::card::sets) static MIND_ROT: CardRecord = CardRecord::new(
    CardSet::Portal,
    "Mind Rot",
    "b91d355d-8409-4f0b-87ce-7590a8b9ebc0",
    "Steve Luke",
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

// POR 106 — Rain of Tears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAIN_OF_TEARS: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Rain of Tears",
    "803ba4ef-24ed-4f45-aed8-f9442322e31e",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// POR 109 — Serpent Warrior
pub(in crate::card::sets) static SERPENT_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Serpent Warrior",
    "c364fd06-64c5-45f6-8ed5-64f44a1e8bda",
    "Roger Raupp",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Snake", "Warrior"], 3, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you lose 3 life.",
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// POR 118 — Blaze
pub(in crate::card::sets) static BLAZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Blaze",
    "f175c959-3b5d-46a3-9194-fad2359bbff9",
    "Gerry Grace",
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

// POR 121 — Craven Giant
pub(in crate::card::sets) static CRAVEN_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Craven Giant",
    "4a2e1c12-f848-43b4-9505-851c66a509f1",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Giant"], 4, 1).with_ability(
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
    ),
);

// POR 137 — Lava Axe
pub(in crate::card::sets) static LAVA_AXE: CardRecord = CardRecord::new(
    CardSet::Portal,
    "Lava Axe",
    "f2bebbad-76aa-4388-891a-583e8af9509d",
    "Adrian Smith",
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Lava Axe deals 5 damage to target player or planeswalker.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(5),
        },
    )),
);

// POR 145 — Raging Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_GOBLIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Raging Goblin",
    "fed57a17-7847-4e60-bc40-4452880f12a3",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// POR 147 — Rain of Salt
pub(in crate::card::sets) static RAIN_OF_SALT: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Rain of Salt",
    "661ffab2-9cf5-492d-874f-de73d7a13e2b",
    "Charles Gillespie",
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy two target lands.",
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )),
);

// POR 152 — Thundermare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERMARE: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Thundermare",
    "59a9f3f5-c80f-47a4-bf84-b7262437017f",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// POR 154 — Volcanic Hammer
pub(in crate::card::sets) static VOLCANIC_HAMMER: CardRecord = CardRecord::new(
    CardSet::Portal,
    "Volcanic Hammer",
    "9563d7c1-4ed1-4919-b0b8-ea1ec9d4bbf6",
    "Christopher Rush",
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

// POR 158 — Anaconda
pub(in crate::card::sets) static ANACONDA: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Anaconda",
    "0a2012ad-6425-4935-83af-fc7309ec2ece",
    "Andrew Robinson",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Snake"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// POR 160 — Bull Hippo
pub(in crate::card::sets) static BULL_HIPPO: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Bull Hippo",
    "30dd236b-94fc-4c56-aeae-215c71a009ea",
    "Roger Raupp",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Hippo"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// POR 161 — Charging Rhino
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_RHINO: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Charging Rhino",
    "49e47248-051c-4ee6-aad2-352ebd1f38ca",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// POR 168 — Gorilla Warrior
pub(in crate::card::sets) static GORILLA_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Gorilla Warrior",
    "38f9c3f3-0d4d-4eec-bd14-9be3233178dc",
    "John Matson",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Ape", "Warrior"], 3, 2),
);

// POR 173 — Monstrous Growth
pub(in crate::card::sets) static MONSTROUS_GROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Monstrous Growth",
    "1fd2edb9-0b53-432e-bb3b-171d2a85439d",
    "Dan Frazier",
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

// POR 176 — Natural Spring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURAL_SPRING: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Natural Spring",
    "8ddfc1cc-5c13-443c-a0ae-0bcc931923e7",
    "Janine Johnston",
    crate::card::CardRules::unsupported(),
);

// POR 179 — Needle Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEEDLE_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Needle Storm",
    "29a44e44-94b1-4bd2-8e00-6bd2ec07ee4c",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// POR 183 — Redwood Treefolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDWOOD_TREEFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Redwood Treefolk",
    "e9399667-ae2a-4b64-84dd-8f97f3e5fe79",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// POR 185 — Spined Wurm
pub(in crate::card::sets) static SPINED_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Spined Wurm",
    "0053bd00-90fd-48c2-8f79-952d5d1e3e74",
    "Colin MacNeil",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Wurm"], 5, 4),
);

// POR 194 — Winter's Grasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_S_GRASP: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Winter's Grasp",
    "b2215de4-da49-4270-aec7-5e16a938bae4",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// POR 195 — Wood Elves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOOD_ELVES: CardRecord = CardRecord::new(
    crate::card::CardSet::Portal,
    "Wood Elves",
    "b7f1fb90-5c85-46a5-802d-248cc0250921",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALABASTER_DRAGON,
    &ANGELIC_BLESSING,
    &ARDENT_MILITIA,
    &ARMORED_PEGASUS,
    &BLESSED_REVERSAL,
    &BREATH_OF_LIFE,
    &CHARGING_PALADIN,
    &KNIGHT_ERRANT,
    &PATH_OF_PEACE,
    &REGAL_UNICORN,
    &SACRED_NECTAR,
    &SEASONED_MARSHAL,
    &STARLIGHT,
    &VENERABLE_MONK,
    &VENGEANCE,
    &BALEFUL_STARE,
    &CLOUD_SPIRIT,
    &EXHAUSTION,
    &FLUX,
    &GIANT_OCTOPUS,
    &HORNED_TURTLE,
    &PHANTOM_WARRIOR,
    &THEFT_OF_DREAMS,
    &TIDAL_SURGE,
    &TIME_EBB,
    &WIND_DRAKE,
    &BOG_RAIDERS,
    &GRAVEDIGGER,
    &MIND_ROT,
    &RAIN_OF_TEARS,
    &SERPENT_WARRIOR,
    &BLAZE,
    &CRAVEN_GIANT,
    &LAVA_AXE,
    &RAGING_GOBLIN,
    &RAIN_OF_SALT,
    &THUNDERMARE,
    &VOLCANIC_HAMMER,
    &ANACONDA,
    &BULL_HIPPO,
    &CHARGING_RHINO,
    &GORILLA_WARRIOR,
    &MONSTROUS_GROWTH,
    &NATURAL_SPRING,
    &NEEDLE_STORM,
    &REDWOOD_TREEFOLK,
    &SPINED_WURM,
    &WINTER_S_GRASP,
    &WOOD_ELVES,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
