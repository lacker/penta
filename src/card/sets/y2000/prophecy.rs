//! Prophecy card records.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardRules, CardSet, CardType, ComparisonDef,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, PlayerRelation,
    TriggerConditionDef, ZoneKind,
};
use crate::mana_cost;

static YOU_CONTROL_AN_UNTAPPED_LAND: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static OPPONENT_CONTROLS_AN_UNTAPPED_LAND: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::Opponent,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static BRAWLER_RESTRICTIONS: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "This creature can't attack if defending player controls an untapped land.",
        EffectDef::IfCondition {
            condition: &OPPONENT_CONTROLS_AN_UNTAPPED_LAND,
            then: &EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        },
    ),
    AbilityDef::static_ability(
        "This creature can't block if you control an untapped land.",
        EffectDef::IfCondition {
            condition: &YOU_CONTROL_AN_UNTAPPED_LAND,
            then: &EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        },
    ),
];

// PCY 1 — Abolish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABOLISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Abolish",
    "3c81ae90-5abd-4c79-b14a-d5f3a1daff38",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PCY 2 — Aura Fracture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_FRACTURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Aura Fracture",
    "de8d3e36-977f-4169-8f2a-a4057b912ccb",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// PCY 3 — Avatar of Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_HOPE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Avatar of Hope",
    "7eec03a2-c62b-4e55-ae9d-edc30a9ad5f4",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PCY 4 — Blessed Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Blessed Wind",
    "3cb624d6-9aec-498c-8df9-6fd025c74487",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// PCY 5 — Celestial Convergence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_CONVERGENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Celestial Convergence",
    "e8e5c9ca-b453-488b-8702-fc74907a8335",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// PCY 6 — Diving Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVING_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Diving Griffin",
    "ec9f72b2-e3d0-4b24-9a73-b95d54695fa4",
    "John Howe",
    crate::card::CardRules::unsupported(),
);

// PCY 7 — Entangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENTANGLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Entangler",
    "ecc20785-4512-4ef6-8f62-928482cb585f",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// PCY 8 — Excise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Excise",
    "8d4f97dd-434b-4156-8e9d-253a943784e3",
    "Joel Biske",
    crate::card::CardRules::unsupported(),
);

// PCY 9 — Flowering Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWERING_FIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Flowering Field",
    "c241fd76-f52d-48fc-864c-57caffa700f6",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// PCY 10 — Glittering Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLITTERING_LION: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Glittering Lion",
    "ab4be296-33a6-46b1-9748-5b0d335f40ee",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// PCY 11 — Glittering Lynx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLITTERING_LYNX: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Glittering Lynx",
    "a3f26c7e-c525-4191-a542-b81343ae95bb",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// PCY 12 — Jeweled Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEWELED_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Jeweled Spirit",
    "b0d3e681-bd4b-41e9-8db4-083172f3caad",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// PCY 13 — Mageta the Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGETA_THE_LION: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mageta the Lion",
    "5861dffc-5afa-44a3-a3fa-9fd440093377",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// PCY 14 — Mageta's Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGETA_S_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mageta's Boon",
    "22db8a3b-413d-4f4d-b103-f50fc0415e9b",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// PCY 15 — Mercenary Informer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCENARY_INFORMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mercenary Informer",
    "98ee3f50-09d7-4960-8214-680a7299fa20",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// PCY 16 — Mine Bearer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINE_BEARER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mine Bearer",
    "a8151510-2445-4244-b851-ab332b908170",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// PCY 17 — Mirror Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRROR_STRIKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mirror Strike",
    "148fbe36-b22d-44e6-9341-7f707baca49d",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// PCY 18 — Reveille Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVEILLE_SQUAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Reveille Squad",
    "8f6385bb-18f9-461b-b541-3c2a5e59189b",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// PCY 19 — Rhystic Circle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_CIRCLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Circle",
    "4a76711e-b508-4bb7-a87c-911a11905af3",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// PCY 20 — Rhystic Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_SHIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Shield",
    "49af7b3f-f56a-4102-b398-5c215dd4fa11",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PCY 21 — Samite Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_SANCTUARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Samite Sanctuary",
    "022ebeba-b61a-497a-a698-e75b130c468c",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// PCY 22 — Sheltering Prayers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHELTERING_PRAYERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Sheltering Prayers",
    "a30803e6-7f0e-4832-b121-b18480c6465c",
    "Nelson DeCastro",
    crate::card::CardRules::unsupported(),
);

// PCY 23 — Shield Dancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_DANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Shield Dancer",
    "1d885360-1ce1-4b80-8928-29437731993f",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// PCY 24 — Soul Charmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_CHARMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Soul Charmer",
    "1656bd2a-e7ce-48b1-8fa1-5f470fe6058e",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// PCY 25 — Sword Dancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWORD_DANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Sword Dancer",
    "a06f00e8-3e58-4ba7-9542-ce6b17fd4005",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// PCY 26 — Trenching Steed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRENCHING_STEED: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Trenching Steed",
    "9a359837-2e41-4ddc-9299-89a783d62014",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// PCY 27 — Troubled Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROUBLED_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Troubled Healer",
    "54407ba7-6671-42a9-acbe-8a1104c7166c",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// PCY 28 — Alexi, Zephyr Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALEXI_ZEPHYR_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Alexi, Zephyr Mage",
    "6f8fc0b0-4a23-47ed-b61b-a4505fcfc5d2",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// PCY 29 — Alexi's Cloak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALEXI_S_CLOAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Alexi's Cloak",
    "457a5613-d1d4-4112-8484-f40120079b7b",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// PCY 30 — Avatar of Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_WILL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Avatar of Will",
    "bf65efc7-6ab5-4116-b003-1f028af80939",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PCY 31 — Coastal Hornclaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COASTAL_HORNCLAW: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Coastal Hornclaw",
    "a5b91ddc-8630-4214-8dce-215f28ccc685",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// PCY 32 — Denying Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DENYING_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Denying Wind",
    "15f236ce-41ad-4a49-a6f9-7853a2395a84",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// PCY 33 — Excavation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCAVATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Excavation",
    "1a4b87ff-42a9-4ea0-a79e-1208ca35ffb2",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// PCY 34 — Foil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Foil",
    "870fb793-3107-4cb2-ba78-34fbf5c9da2f",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// PCY 35 — Gulf Squid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GULF_SQUID: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Gulf Squid",
    "bf424982-a0ab-4db9-8889-f3cef10966c6",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// PCY 36 — Hazy Homunculus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZY_HOMUNCULUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Hazy Homunculus",
    "f87489f2-82b7-4be6-80ae-3d5955d5ed92",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// PCY 37 — Heightened Awareness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEIGHTENED_AWARENESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Heightened Awareness",
    "2765be4f-23bf-49c1-9546-11a7916156be",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// PCY 38 — Mana Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_VAPORS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mana Vapors",
    "0b6dfe49-9fd6-4fa0-b73e-e6470d8e7ca7",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// PCY 39 — Overburden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERBURDEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Overburden",
    "7e6cf6b6-b4c1-4742-9be4-b3b15fbb0202",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// PCY 40 — Psychic Theft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_THEFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Psychic Theft",
    "f6f86c92-c19e-4f2c-96d3-d3b05623cb00",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// PCY 41 — Quicksilver Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Quicksilver Wall",
    "a680aeaf-8c6e-45b5-8814-7fd04e963220",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PCY 42 — Rethink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETHINK: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rethink",
    "915ae03f-22f3-4ecc-a875-5226d8dec384",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PCY 43 — Rhystic Deluge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_DELUGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Deluge",
    "0a3dd540-7f54-46fe-b1e8-7b07f57e71d0",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// PCY 44 — Rhystic Scrying
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_SCRYING: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Scrying",
    "81a59737-f06f-49b7-a490-3dc1115b47b7",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// PCY 45 — Rhystic Study
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_STUDY: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Study",
    "3394cefd-a3c6-4917-8f46-234e441ecfb6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// PCY 46 — Ribbon Snake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIBBON_SNAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Ribbon Snake",
    "eb5135dc-4fc1-48a1-8405-44b2f93a3c21",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// PCY 47 — Shrouded Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHROUDED_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Shrouded Serpent",
    "d3d9035b-b6ec-479f-b697-3e5c3110ef10",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// PCY 48 — Spiketail Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKETAIL_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Spiketail Drake",
    "4db398ca-6b0b-4225-baaa-c4b1c243b2bd",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// PCY 49 — Spiketail Hatchling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKETAIL_HATCHLING: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Spiketail Hatchling",
    "9988f0fe-a7d4-44f9-b37c-fa30014ea215",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// PCY 50 — Stormwatch Eagle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMWATCH_EAGLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Stormwatch Eagle",
    "21c3bb62-63b4-4b53-9e4d-edfc7487494b",
    "Aaron Boyd",
    crate::card::CardRules::unsupported(),
);

// PCY 51 — Sunken Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNKEN_FIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Sunken Field",
    "9211ee02-e854-4414-92b1-65a7af29f0b9",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// PCY 52 — Troublesome Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROUBLESOME_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Troublesome Spirit",
    "23d7e856-6852-4b97-ae0e-a4becdfc8166",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// PCY 53 — Windscouter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINDSCOUTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Windscouter",
    "bb70925a-4bef-4067-a1d7-79114aff5847",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// PCY 54 — Withdraw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITHDRAW: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Withdraw",
    "f1a3a52f-0ccd-4935-b3ca-9c69cba283cc",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// PCY 55 — Agent of Shauku
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_OF_SHAUKU: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Agent of Shauku",
    "d8316804-6f8b-423e-a2c3-fa476c095544",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// PCY 56 — Avatar of Woe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_WOE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Avatar of Woe",
    "0f695405-7238-48fb-9ea2-1b1613a0afda",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PCY 57 — Bog Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Bog Elemental",
    "75191915-352e-4de7-b216-63f0ff588ba5",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// PCY 58 — Bog Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_GLIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Bog Glider",
    "086a5620-704b-47a9-9a5d-73e28631d6f8",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// PCY 59 — Chilling Apparition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHILLING_APPARITION: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Chilling Apparition",
    "c20edb71-aa1d-437b-bcfb-953efbe45150",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// PCY 60 — Coffin Puppets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COFFIN_PUPPETS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Coffin Puppets",
    "afcda8e4-d3dc-44f8-b277-b61fa261666b",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PCY 60s — Coffin Puppets (alternate printing)
const COFFIN_PUPPETS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COFFIN_PUPPETS,
    1,
    "c34019cb-5d87-4451-a102-b751ea3a97f8",
    "Glen Angus",
);

// PCY 61 — Death Charmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_CHARMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Death Charmer",
    "e58a303a-9f7a-43e7-bcba-c58b378a53ce",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PCY 62 — Despoil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPOIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Despoil",
    "06bb6ff7-2cd6-430e-a618-0b83d9c1d044",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// PCY 63 — Endbringer's Revel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDBRINGER_S_REVEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Endbringer's Revel",
    "b76843b0-0e71-473b-8c9b-6a8bc30255da",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// PCY 64 — Fen Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEN_STALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Fen Stalker",
    "8e7d1125-7eb0-4065-bc2c-764689380fa8",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// PCY 65 — Flay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Flay",
    "a6fe155f-bfb2-49d8-83f0-ab1047a961d1",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// PCY 66 — Greel, Mind Raker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEL_MIND_RAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Greel, Mind Raker",
    "e9d1f317-efd1-4595-92e2-44815a2b8147",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// PCY 67 — Greel's Caress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEL_S_CARESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Greel's Caress",
    "5b25ce3f-fab3-40f8-8a16-fe580f3d97a5",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// PCY 68 — Infernal Genesis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_GENESIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Infernal Genesis",
    "1a63d16e-319d-46f4-a28c-895b36605ee6",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// PCY 69 — Nakaya Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAKAYA_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Nakaya Shade",
    "eefd9315-9b7c-4c6b-8a15-a6af873dab6f",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// PCY 70 — Noxious Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOXIOUS_FIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Noxious Field",
    "10c84d09-555c-472b-b445-5dd5a44cd555",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// PCY 71 — Outbreak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTBREAK: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Outbreak",
    "f43c30d9-23a5-4872-925d-3427f5f57995",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// PCY 72 — Pit Raptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIT_RAPTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Pit Raptor",
    "e37cd150-1064-43b7-919b-8922d8a18f21",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// PCY 73 — Plague Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_FIEND: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Plague Fiend",
    "11f077f5-c0b0-4e94-8599-e2122bc87238",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// PCY 74 — Plague Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Plague Wind",
    "b0d4bd20-7422-45ed-aa76-3ef055c556e7",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// PCY 75 — Rebel Informer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REBEL_INFORMER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rebel Informer",
    "c98a71a8-291f-4d94-ada0-5f50f354cca7",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// PCY 76 — Rhystic Syphon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_SYPHON: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Syphon",
    "750c5df2-e299-4bf3-8018-725893702314",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// PCY 77 — Rhystic Tutor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_TUTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Tutor",
    "e02c1609-9cac-460f-8504-a84e28c340c1",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// PCY 78 — Soul Strings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_STRINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Soul Strings",
    "e34f9d1b-a89a-439f-8aa9-b96a1bf892eb",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// PCY 79 — Steal Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAL_STRENGTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Steal Strength",
    "5470b3bb-5061-4beb-9f44-b56c3b2fd816",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// PCY 80 — Wall of Vipers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_VIPERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Wall of Vipers",
    "00042443-4d4e-4087-b4e5-5e781e7cc5fa",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// PCY 81 — Whipstitched Zombie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPSTITCHED_ZOMBIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Whipstitched Zombie",
    "9cd00b0b-2ac1-4926-a735-215f402ba1c4",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// PCY 82 — Avatar of Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_FURY: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Avatar of Fury",
    "528293b4-ce3b-4623-8ced-496701d7265b",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PCY 83 — Barbed Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_FIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Barbed Field",
    "1c76db48-6f05-49c3-a49c-587c0a8a3613",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// PCY 84 — Branded Brawlers
pub(in crate::card::sets) static BRANDED_BRAWLERS: CardRecord = CardRecord::new(
    CardSet::Prophecy,
    "Branded Brawlers",
    "90a48065-fbf1-4f2a-993e-7061057a4c45",
    "Scott M. Fischer",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&BRAWLER_RESTRICTIONS),
);

// PCY 85 — Brutal Suppression
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRUTAL_SUPPRESSION: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Brutal Suppression",
    "17b3725b-924d-4137-9078-1a28f06c84fa",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// PCY 86 — Citadel of Pain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITADEL_OF_PAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Citadel of Pain",
    "66585109-77cb-42f1-9c14-3dac1d493b71",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// PCY 87 — Devastate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVASTATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Devastate",
    "bfe7c990-a34b-475e-a612-447c22f998d3",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// PCY 88 — Fault Riders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAULT_RIDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Fault Riders",
    "1c9d3579-3fc1-434e-8f26-d5dbd6344429",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// PCY 89 — Fickle Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FICKLE_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Fickle Efreet",
    "ca6d047a-f3dc-4c34-9679-fb76037e4044",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// PCY 90 — Flameshot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMESHOT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Flameshot",
    "3f7b61a8-a1ff-4e2a-bc24-8990c61a5e5b",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// PCY 91 — Inflame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFLAME: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Inflame",
    "cd7bc4c0-9bfd-444b-b22c-f1b7e1426807",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// PCY 92 — Keldon Arsonist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_ARSONIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Keldon Arsonist",
    "113f58b1-d8d7-4544-8363-e2b96e9d2623",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PCY 93 — Keldon Berserker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_BERSERKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Keldon Berserker",
    "daa3f9c0-66ff-4b94-b0c3-e1c65d2040b9",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PCY 94 — Keldon Firebombers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_FIREBOMBERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Keldon Firebombers",
    "d3fc78b5-c259-4c67-810c-99655e72c2da",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// PCY 95 — Latulla, Keldon Overseer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LATULLA_KELDON_OVERSEER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Latulla, Keldon Overseer",
    "fd3c7b0c-98bd-4c63-bbb0-80484a5ab26f",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// PCY 96 — Latulla's Orders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LATULLA_S_ORDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Latulla's Orders",
    "a56cd728-5c3c-4fd6-bb01-0bf0875508c7",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// PCY 97 — Lesser Gargadon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESSER_GARGADON: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Lesser Gargadon",
    "63ed7aec-a513-418e-9cef-e0c51203055b",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// PCY 98 — Panic Attack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANIC_ATTACK: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Panic Attack",
    "89ec751c-08b5-4afb-bc08-8b2735b24f59",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// PCY 99 — Rhystic Lightning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_LIGHTNING: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Lightning",
    "21ce365e-3002-42e9-aeb5-1b845408271e",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// PCY 100 — Ridgeline Rager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIDGELINE_RAGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Ridgeline Rager",
    "5f663a4a-592a-4a3b-bbaf-e9c5c3049021",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// PCY 101 — Scoria Cat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORIA_CAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Scoria Cat",
    "7274f791-c9f1-49a1-9002-10e94caee96e",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// PCY 102 — Search for Survivors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARCH_FOR_SURVIVORS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Search for Survivors",
    "2f19a1b5-48ba-44a9-b91f-2f628b223ffb",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// PCY 103 — Searing Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Searing Wind",
    "7b761f97-3690-497a-b6ab-c71f61b8e841",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// PCY 104 — Spur Grappler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPUR_GRAPPLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Spur Grappler",
    "50bf91a7-4d04-437c-a290-6adb52f25312",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// PCY 105 — Task Mage Assembly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASK_MAGE_ASSEMBLY: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Task Mage Assembly",
    "258a9cdd-c626-404e-b82d-01091f11f107",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// PCY 106 — Veteran Brawlers
pub(in crate::card::sets) static VETERAN_BRAWLERS: CardRecord = CardRecord::new(
    CardSet::Prophecy,
    "Veteran Brawlers",
    "ee4d3acb-68be-409d-beb7-92a7cbc0402f",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Soldier"], 4, 4)
        .with_abilities(&BRAWLER_RESTRICTIONS),
);

// PCY 107 — Whip Sergeant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIP_SERGEANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Whip Sergeant",
    "2e6b3f38-87c9-4cea-b9e5-b8fb42e64794",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// PCY 108 — Zerapa Minotaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZERAPA_MINOTAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Zerapa Minotaur",
    "55a8e1ce-e394-48d6-938a-aa76c0273abe",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// PCY 109 — Avatar of Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_MIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Avatar of Might",
    "c97614db-167e-4ede-96bd-77ed90b57d4e",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// PCY 110 — Calming Verse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALMING_VERSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Calming Verse",
    "ec38c856-dc21-450d-9aa6-da16c91a489a",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// PCY 111 — Darba
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARBA: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Darba",
    "d82636dc-4b3e-44a8-bc72-dab1275dfb6d",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PCY 112 — Dual Nature
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUAL_NATURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Dual Nature",
    "6890e414-d7e1-4320-924c-083e65a2ae72",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PCY 113 — Elephant Resurgence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEPHANT_RESURGENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Elephant Resurgence",
    "22147f72-7ff8-40c4-9bdd-df41dce17dad",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// PCY 114 — Forgotten Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Forgotten Harvest",
    "9fefbace-03cb-43db-a221-0be2b8784357",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// PCY 115 — Jolrael, Empress of Beasts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLRAEL_EMPRESS_OF_BEASTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Jolrael, Empress of Beasts",
    "ad0ea1c3-e920-467b-a3c8-a6b1097c3e8d",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// PCY 116 — Jolrael's Favor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLRAEL_S_FAVOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Jolrael's Favor",
    "8275ecd7-f119-4cca-bef1-626a3272dd2c",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// PCY 117 — Living Terrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_TERRAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Living Terrain",
    "64558128-7990-470c-88c9-d47d622e44db",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// PCY 118 — Marsh Boa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARSH_BOA: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Marsh Boa",
    "87c99bb6-a483-4beb-b98c-eb641fe3d50a",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PCY 119 — Mungha Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUNGHA_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Mungha Wurm",
    "6addc915-2997-4b81-a026-97d4421cf17d",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// PCY 120 — Pygmy Razorback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_RAZORBACK: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Pygmy Razorback",
    "0ad9744f-797a-4dd3-8617-192773be995c",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// PCY 121 — Rib Cage Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIB_CAGE_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rib Cage Spider",
    "d71bebea-1634-4d9a-b3ad-2e01ecacad7e",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// PCY 122 — Root Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_CAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Root Cage",
    "db109497-7674-4067-a12b-dfb5c317a358",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// PCY 123 — Silt Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILT_CRAWLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Silt Crawler",
    "f334e864-4e62-4bc3-9470-661be3d879e2",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// PCY 124 — Snag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAG: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Snag",
    "7401df8b-23e7-4485-9ed4-70118a66feed",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// PCY 125 — Spitting Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITTING_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Spitting Spider",
    "f220fada-5cbb-4266-bca3-a44d51773d63",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// PCY 126 — Spore Frog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPORE_FROG: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Spore Frog",
    "2f752339-003d-4ded-b2bf-e4200fc8d5d6",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// PCY 127 — Squirrel Wrangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRREL_WRANGLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Squirrel Wrangler",
    "7094be2a-454e-4e4d-a540-c5c80e37468a",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// PCY 128 — Thresher Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRESHER_BEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Thresher Beast",
    "57996732-c9e4-4271-9d5f-2a8c77f8d177",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// PCY 129 — Thrive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Thrive",
    "9cb20099-fc53-4fdf-86f4-d7d8155c2af1",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// PCY 130 — Verdant Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDANT_FIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Verdant Field",
    "d123da53-9fd3-492b-beb7-76d1c0f5e4f6",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// PCY 131 — Vintara Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINTARA_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Vintara Elephant",
    "f99f7aef-7398-4e10-9f4e-68e7c294c101",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// PCY 132 — Vintara Snapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINTARA_SNAPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Vintara Snapper",
    "897edf01-fc6f-4835-b025-c137d921ce09",
    "Joel Biske",
    crate::card::CardRules::unsupported(),
);

// PCY 133 — Vitalizing Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VITALIZING_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Vitalizing Wind",
    "0fbd7c20-d527-4d97-9630-896d5e7bf1de",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// PCY 134 — Wild Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_MIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Wild Might",
    "7a8c0be2-ba90-4e7a-b1a6-c68be550e33d",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// PCY 135 — Wing Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WING_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Wing Storm",
    "59e66be1-f18a-433f-8504-aa1e85e22023",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// PCY 136 — Chimeric Idol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIMERIC_IDOL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Chimeric Idol",
    "fd0fb30e-e4d0-4271-a712-db40fa7650c3",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// PCY 137 — Copper-Leaf Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COPPER_LEAF_ANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Copper-Leaf Angel",
    "7be413dd-d6e0-4bd3-8c14-4dbe44e8ee41",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// PCY 138 — Hollow Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLLOW_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Hollow Warrior",
    "163add05-9ee9-4a8f-8838-3c4143ddc2f5",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// PCY 139 — Keldon Battlewagon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_BATTLEWAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Keldon Battlewagon",
    "5c810aa1-e367-4102-a5bd-6dc02d3023e8",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// PCY 140 — Well of Discovery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_OF_DISCOVERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Well of Discovery",
    "82331a8a-c0f1-4d89-87f8-1b1d0fccabb8",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// PCY 141 — Well of Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_OF_LIFE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Well of Life",
    "bf3f4fc3-3819-470d-92d9-98cb390f89b9",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// PCY 142 — Rhystic Cave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_CAVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Rhystic Cave",
    "4ae74463-4426-4ad4-b7a2-324694854245",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// PCY 143 — Wintermoon Mesa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTERMOON_MESA: CardRecord = CardRecord::new(
    crate::card::CardSet::Prophecy,
    "Wintermoon Mesa",
    "f07144a6-6e47-4315-8353-f8958f014f41",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABOLISH,
    &AURA_FRACTURE,
    &AVATAR_OF_HOPE,
    &BLESSED_WIND,
    &CELESTIAL_CONVERGENCE,
    &DIVING_GRIFFIN,
    &ENTANGLER,
    &EXCISE,
    &FLOWERING_FIELD,
    &GLITTERING_LION,
    &GLITTERING_LYNX,
    &JEWELED_SPIRIT,
    &MAGETA_THE_LION,
    &MAGETA_S_BOON,
    &MERCENARY_INFORMER,
    &MINE_BEARER,
    &MIRROR_STRIKE,
    &REVEILLE_SQUAD,
    &RHYSTIC_CIRCLE,
    &RHYSTIC_SHIELD,
    &SAMITE_SANCTUARY,
    &SHELTERING_PRAYERS,
    &SHIELD_DANCER,
    &SOUL_CHARMER,
    &SWORD_DANCER,
    &TRENCHING_STEED,
    &TROUBLED_HEALER,
    &ALEXI_ZEPHYR_MAGE,
    &ALEXI_S_CLOAK,
    &AVATAR_OF_WILL,
    &COASTAL_HORNCLAW,
    &DENYING_WIND,
    &EXCAVATION,
    &FOIL,
    &GULF_SQUID,
    &HAZY_HOMUNCULUS,
    &HEIGHTENED_AWARENESS,
    &MANA_VAPORS,
    &OVERBURDEN,
    &PSYCHIC_THEFT,
    &QUICKSILVER_WALL,
    &RETHINK,
    &RHYSTIC_DELUGE,
    &RHYSTIC_SCRYING,
    &RHYSTIC_STUDY,
    &RIBBON_SNAKE,
    &SHROUDED_SERPENT,
    &SPIKETAIL_DRAKE,
    &SPIKETAIL_HATCHLING,
    &STORMWATCH_EAGLE,
    &SUNKEN_FIELD,
    &TROUBLESOME_SPIRIT,
    &WINDSCOUTER,
    &WITHDRAW,
    &AGENT_OF_SHAUKU,
    &AVATAR_OF_WOE,
    &BOG_ELEMENTAL,
    &BOG_GLIDER,
    &CHILLING_APPARITION,
    &COFFIN_PUPPETS,
    &DEATH_CHARMER,
    &DESPOIL,
    &ENDBRINGER_S_REVEL,
    &FEN_STALKER,
    &FLAY,
    &GREEL_MIND_RAKER,
    &GREEL_S_CARESS,
    &INFERNAL_GENESIS,
    &NAKAYA_SHADE,
    &NOXIOUS_FIELD,
    &OUTBREAK,
    &PIT_RAPTOR,
    &PLAGUE_FIEND,
    &PLAGUE_WIND,
    &REBEL_INFORMER,
    &RHYSTIC_SYPHON,
    &RHYSTIC_TUTOR,
    &SOUL_STRINGS,
    &STEAL_STRENGTH,
    &WALL_OF_VIPERS,
    &WHIPSTITCHED_ZOMBIE,
    &AVATAR_OF_FURY,
    &BARBED_FIELD,
    &BRANDED_BRAWLERS,
    &BRUTAL_SUPPRESSION,
    &CITADEL_OF_PAIN,
    &DEVASTATE,
    &FAULT_RIDERS,
    &FICKLE_EFREET,
    &FLAMESHOT,
    &INFLAME,
    &KELDON_ARSONIST,
    &KELDON_BERSERKER,
    &KELDON_FIREBOMBERS,
    &LATULLA_KELDON_OVERSEER,
    &LATULLA_S_ORDERS,
    &LESSER_GARGADON,
    &PANIC_ATTACK,
    &RHYSTIC_LIGHTNING,
    &RIDGELINE_RAGER,
    &SCORIA_CAT,
    &SEARCH_FOR_SURVIVORS,
    &SEARING_WIND,
    &SPUR_GRAPPLER,
    &TASK_MAGE_ASSEMBLY,
    &VETERAN_BRAWLERS,
    &WHIP_SERGEANT,
    &ZERAPA_MINOTAUR,
    &AVATAR_OF_MIGHT,
    &CALMING_VERSE,
    &DARBA,
    &DUAL_NATURE,
    &ELEPHANT_RESURGENCE,
    &FORGOTTEN_HARVEST,
    &JOLRAEL_EMPRESS_OF_BEASTS,
    &JOLRAEL_S_FAVOR,
    &LIVING_TERRAIN,
    &MARSH_BOA,
    &MUNGHA_WURM,
    &PYGMY_RAZORBACK,
    &RIB_CAGE_SPIDER,
    &ROOT_CAGE,
    &SILT_CRAWLER,
    &SNAG,
    &SPITTING_SPIDER,
    &SPORE_FROG,
    &SQUIRREL_WRANGLER,
    &THRESHER_BEAST,
    &THRIVE,
    &VERDANT_FIELD,
    &VINTARA_ELEPHANT,
    &VINTARA_SNAPPER,
    &VITALIZING_WIND,
    &WILD_MIGHT,
    &WING_STORM,
    &CHIMERIC_IDOL,
    &COPPER_LEAF_ANGEL,
    &HOLLOW_WARRIOR,
    &KELDON_BATTLEWAGON,
    &WELL_OF_DISCOVERY,
    &WELL_OF_LIFE,
    &RHYSTIC_CAVE,
    &WINTERMOON_MESA,
];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[COFFIN_PUPPETS_ALTERNATE_1];
