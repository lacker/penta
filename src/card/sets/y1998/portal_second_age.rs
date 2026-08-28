//! Portal Second Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::abilities;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ValueDef,
};
use crate::{TargetIndex, mana_cost};

// P02 8 — Angel of Mercy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGEL_OF_MERCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dac5c913-4eb5-4cfb-9c24-223f14f07064"),
    "Angel of Mercy",
    crate::card::CardArt::new("dac5c913-4eb5-4cfb-9c24-223f14f07064", "Melissa A. Benson"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 15 — Breath of Life
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BREATH_OF_LIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcea5e09-6385-41df-970b-ac26c9b46127"),
    "Breath of Life",
    crate::card::CardArt::new("a10f24f7-f82e-413e-824f-384607c7d858", "Lubov"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 18 — Path of Peace
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PATH_OF_PEACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1f3e1c9-bfad-49a1-b171-6fa344ef2eef"),
    "Path of Peace",
    crate::card::CardArt::new("cb14d3f4-09f3-4113-bdc3-0fd753137f7c", "David A. Cherry"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 27 — Vengeance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c91c249b-157c-4f1d-8171-29d1e75b1c9f"),
    "Vengeance",
    crate::card::CardArt::new("3209ee48-4485-44fc-b71d-cd6241674e64", "Keith Parkinson"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 37 — Exhaustion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXHAUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d6a5c33-cf74-4cec-a4f4-1aac9e7b8f79"),
    "Exhaustion",
    crate::card::CardArt::new("fcc103a6-7888-4e35-b35b-a796a48caf70", "Kaja Foglio"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04095ad2-7308-4e26-b9ef-070a5755d066"),
    "Blaze",
    crate::card::CardArt::new("3940d0ca-0ca2-4446-9330-a554c3e89824", "David A. Cherry"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 98 — Goblin Glider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c29491b-dec1-429d-9950-062582f8164f"),
    "Goblin Glider",
    crate::card::CardArt::new("9c29491b-dec1-429d-9950-062582f8164f", "Pete Venters"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 103 — Goblin Raider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("68fe9691-d788-42cb-8d13-005724939b62"),
    "Goblin Raider",
    crate::card::CardArt::new("68fe9691-d788-42cb-8d13-005724939b62", "Matt Stawicki"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 105 — Goblin War Strike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_WAR_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("738fecfd-1119-4dcb-acd6-ec9715d9c074"),
    "Goblin War Strike",
    crate::card::CardArt::new("738fecfd-1119-4dcb-acd6-ec9715d9c074", "Michael Weaver"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 106 — Jagged Lightning
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JAGGED_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("148e6704-9cf0-45cf-9bab-db318c016593"),
    "Jagged Lightning",
    crate::card::CardArt::new("148e6704-9cf0-45cf-9bab-db318c016593", "Michael Weaver"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 112 — Ogre Taskmaster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_TASKMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d674a92e-b268-48f7-b082-f8ca2e63d43b"),
    "Ogre Taskmaster",
    crate::card::CardArt::new("d674a92e-b268-48f7-b082-f8ca2e63d43b", "Dan Frazier"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WILDFIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b69cfcb0-db68-4494-a3e1-7c2ca279fcf5"),
    "Wildfire",
    crate::card::CardArt::new("b69cfcb0-db68-4494-a3e1-7c2ca279fcf5", "Rob Alexander"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 131 — Lone Wolf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LONE_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ff4d831-7388-4321-a636-79cf7bde25bb"),
    "Lone Wolf",
    crate::card::CardArt::new("7ff4d831-7388-4321-a636-79cf7bde25bb", "Michael Weaver"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
);

// P02 133 — Monstrous Growth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MONSTROUS_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0523c816-dddf-4b63-8db8-5e41dc673e5f"),
    "Monstrous Growth",
    crate::card::CardArt::new("3816da20-4434-4bf7-a9dd-3eb3bb735f08", "Una Fricker"),
    crate::card::CardSet::PortalSecondAge,
    crate::card::CardRules::unsupported(),
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
