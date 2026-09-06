//! Prophecy card records.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, AppliedRuleDef,
    BasicLandType, CardArt, CardRules, CardSet, CardType, ComparisonDef, CostDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, SacrificedAmountDef,
    TriggerConditionDef, TriggerEventDef, ValueComparisonDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, TurnStepDef, mana_cost};

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
    PrintingAnchor::scryfall("3c81ae90-5abd-4c79-b14a-d5f3a1daff38"),
    "Abolish",
    crate::card::CardArt::new("3c81ae90-5abd-4c79-b14a-d5f3a1daff38", "Kev Walker"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 2 — Aura Fracture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_FRACTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de8d3e36-977f-4169-8f2a-a4057b912ccb"),
    "Aura Fracture",
    crate::card::CardArt::new("de8d3e36-977f-4169-8f2a-a4057b912ccb", "Rebecca Guay"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 3 — Avatar of Hope
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_HOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7246341-5a22-4857-b7ab-331843db0915"),
    "Avatar of Hope",
    crate::card::CardArt::new("7eec03a2-c62b-4e55-ae9d-edc30a9ad5f4", "rk post"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 4 — Blessed Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cb624d6-9aec-498c-8df9-6fd025c74487"),
    "Blessed Wind",
    crate::card::CardArt::new("3cb624d6-9aec-498c-8df9-6fd025c74487", "Anthony S. Waters"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 5 — Celestial Convergence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_CONVERGENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8e5c9ca-b453-488b-8702-fc74907a8335"),
    "Celestial Convergence",
    crate::card::CardArt::new("e8e5c9ca-b453-488b-8702-fc74907a8335", "Ray Lago"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 6 — Diving Griffin
pub(in crate::card::sets) static DIVING_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec9f72b2-e3d0-4b24-9a73-b95d54695fa4"),
    "Diving Griffin",
    CardArt::new("ec9f72b2-e3d0-4b24-9a73-b95d54695fa4", "John Howe"),
    CardSet::Prophecy,
    // A 2/2 flier that attacks without giving up the block, which is what
    // the third mana is buying.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Griffin"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// PCY 7 — Entangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENTANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ecc20785-4512-4ef6-8f62-928482cb585f"),
    "Entangler",
    crate::card::CardArt::new(
        "ecc20785-4512-4ef6-8f62-928482cb585f",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 8 — Excise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d4f97dd-434b-4156-8e9d-253a943784e3"),
    "Excise",
    crate::card::CardArt::new("8d4f97dd-434b-4156-8e9d-253a943784e3", "Joel Biske"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 9 — Flowering Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWERING_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c241fd76-f52d-48fc-864c-57caffa700f6"),
    "Flowering Field",
    crate::card::CardArt::new("c241fd76-f52d-48fc-864c-57caffa700f6", "Jeff Miracola"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 10 — Glittering Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLITTERING_LION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab4be296-33a6-46b1-9748-5b0d335f40ee"),
    "Glittering Lion",
    crate::card::CardArt::new("ab4be296-33a6-46b1-9748-5b0d335f40ee", "Don Hazeltine"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 11 — Glittering Lynx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLITTERING_LYNX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3f26c7e-c525-4191-a542-b81343ae95bb"),
    "Glittering Lynx",
    crate::card::CardArt::new("a3f26c7e-c525-4191-a542-b81343ae95bb", "Dan Frazier"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 12 — Jeweled Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEWELED_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0d3e681-bd4b-41e9-8db4-083172f3caad"),
    "Jeweled Spirit",
    crate::card::CardArt::new(
        "b0d3e681-bd4b-41e9-8db4-083172f3caad",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 13 — Mageta the Lion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGETA_THE_LION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5861dffc-5afa-44a3-a3fa-9fd440093377"),
    "Mageta the Lion",
    crate::card::CardArt::new("5861dffc-5afa-44a3-a3fa-9fd440093377", "Brom"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 14 — Mageta's Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGETA_S_BOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22db8a3b-413d-4f4d-b103-f50fc0415e9b"),
    "Mageta's Boon",
    crate::card::CardArt::new("22db8a3b-413d-4f4d-b103-f50fc0415e9b", "Bradley Williams"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 15 — Mercenary Informer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCENARY_INFORMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98ee3f50-09d7-4960-8214-680a7299fa20"),
    "Mercenary Informer",
    crate::card::CardArt::new("98ee3f50-09d7-4960-8214-680a7299fa20", "Nelson DeCastro"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 16 — Mine Bearer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINE_BEARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8151510-2445-4244-b851-ab332b908170"),
    "Mine Bearer",
    crate::card::CardArt::new(
        "a8151510-2445-4244-b851-ab332b908170",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 17 — Mirror Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRROR_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("148fbe36-b22d-44e6-9341-7f707baca49d"),
    "Mirror Strike",
    crate::card::CardArt::new("148fbe36-b22d-44e6-9341-7f707baca49d", "Dave Dorman"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 18 — Reveille Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVEILLE_SQUAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f6385bb-18f9-461b-b541-3c2a5e59189b"),
    "Reveille Squad",
    crate::card::CardArt::new(
        "8f6385bb-18f9-461b-b541-3c2a5e59189b",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 19 — Rhystic Circle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_CIRCLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a76711e-b508-4bb7-a87c-911a11905af3"),
    "Rhystic Circle",
    crate::card::CardArt::new("4a76711e-b508-4bb7-a87c-911a11905af3", "Alan Pollack"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 20 — Rhystic Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49af7b3f-f56a-4102-b398-5c215dd4fa11"),
    "Rhystic Shield",
    crate::card::CardArt::new("49af7b3f-f56a-4102-b398-5c215dd4fa11", "Kev Walker"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 21 — Samite Sanctuary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("022ebeba-b61a-497a-a698-e75b130c468c"),
    "Samite Sanctuary",
    crate::card::CardArt::new("022ebeba-b61a-497a-a698-e75b130c468c", "Ben Thompson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 22 — Sheltering Prayers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHELTERING_PRAYERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a30803e6-7f0e-4832-b121-b18480c6465c"),
    "Sheltering Prayers",
    crate::card::CardArt::new("a30803e6-7f0e-4832-b121-b18480c6465c", "Nelson DeCastro"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 23 — Shield Dancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_DANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d885360-1ce1-4b80-8928-29437731993f"),
    "Shield Dancer",
    crate::card::CardArt::new("1d885360-1ce1-4b80-8928-29437731993f", "Mike Ploog"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 24 — Soul Charmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_CHARMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1656bd2a-e7ce-48b1-8fa1-5f470fe6058e"),
    "Soul Charmer",
    crate::card::CardArt::new("1656bd2a-e7ce-48b1-8fa1-5f470fe6058e", "Glen Angus"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 25 — Sword Dancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWORD_DANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a06f00e8-3e58-4ba7-9542-ce6b17fd4005"),
    "Sword Dancer",
    crate::card::CardArt::new("a06f00e8-3e58-4ba7-9542-ce6b17fd4005", "Roger Raupp"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 26 — Trenching Steed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRENCHING_STEED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a359837-2e41-4ddc-9299-89a783d62014"),
    "Trenching Steed",
    crate::card::CardArt::new(
        "9a359837-2e41-4ddc-9299-89a783d62014",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 27 — Troubled Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROUBLED_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54407ba7-6671-42a9-acbe-8a1104c7166c"),
    "Troubled Healer",
    crate::card::CardArt::new("54407ba7-6671-42a9-acbe-8a1104c7166c", "Terese Nielsen"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 28 — Alexi, Zephyr Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALEXI_ZEPHYR_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f8fc0b0-4a23-47ed-b61b-a4505fcfc5d2"),
    "Alexi, Zephyr Mage",
    crate::card::CardArt::new("6f8fc0b0-4a23-47ed-b61b-a4505fcfc5d2", "Mark Zug"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 29 — Alexi's Cloak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALEXI_S_CLOAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("457a5613-d1d4-4112-8484-f40120079b7b"),
    "Alexi's Cloak",
    crate::card::CardArt::new("457a5613-d1d4-4112-8484-f40120079b7b", "Alan Rabinowitz"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 30 — Avatar of Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_WILL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf65efc7-6ab5-4116-b003-1f028af80939"),
    "Avatar of Will",
    crate::card::CardArt::new("bf65efc7-6ab5-4116-b003-1f028af80939", "rk post"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 31 — Coastal Hornclaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COASTAL_HORNCLAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a5b91ddc-8630-4214-8dce-215f28ccc685"),
    "Coastal Hornclaw",
    crate::card::CardArt::new("a5b91ddc-8630-4214-8dce-215f28ccc685", "DiTerlizzi"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 32 — Denying Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DENYING_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15f236ce-41ad-4a49-a6f9-7853a2395a84"),
    "Denying Wind",
    crate::card::CardArt::new("15f236ce-41ad-4a49-a6f9-7853a2395a84", "Tony Szczudlo"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 33 — Excavation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCAVATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a4b87ff-42a9-4ea0-a79e-1208ca35ffb2"),
    "Excavation",
    crate::card::CardArt::new("1a4b87ff-42a9-4ea0-a79e-1208ca35ffb2", "Terese Nielsen"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 34 — Foil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("870fb793-3107-4cb2-ba78-34fbf5c9da2f"),
    "Foil",
    crate::card::CardArt::new("870fb793-3107-4cb2-ba78-34fbf5c9da2f", "Bradley Williams"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 35 — Gulf Squid
pub(in crate::card::sets) static GULF_SQUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf424982-a0ab-4db9-8889-f3cef10966c6"),
    "Gulf Squid",
    CardArt::new("bf424982-a0ab-4db9-8889-f3cef10966c6", "Wayne England"),
    CardSet::Prophecy,
    // Four mana to take their whole turn's mana, once. It is tempo bought
    // with a card, which is what Prophecy sold.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Squid", "Beast"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, tap all lands target player controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::controlled_by(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                    ),
                )),
            },
        ),
    ),
);

// PCY 36 — Hazy Homunculus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZY_HOMUNCULUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f87489f2-82b7-4be6-80ae-3d5955d5ed92"),
    "Hazy Homunculus",
    crate::card::CardArt::new("f87489f2-82b7-4be6-80ae-3d5955d5ed92", "Anthony S. Waters"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 37 — Heightened Awareness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEIGHTENED_AWARENESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2765be4f-23bf-49c1-9546-11a7916156be"),
    "Heightened Awareness",
    crate::card::CardArt::new("2765be4f-23bf-49c1-9546-11a7916156be", "Pete Venters"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 38 — Mana Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_VAPORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b6dfe49-9fd6-4fa0-b73e-e6470d8e7ca7"),
    "Mana Vapors",
    crate::card::CardArt::new("0b6dfe49-9fd6-4fa0-b73e-e6470d8e7ca7", "Mark Romanoski"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 39 — Overburden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERBURDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e6cf6b6-b4c1-4742-9be4-b3b15fbb0202"),
    "Overburden",
    crate::card::CardArt::new("7e6cf6b6-b4c1-4742-9be4-b3b15fbb0202", "John Matson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 40 — Psychic Theft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_THEFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6f86c92-c19e-4f2c-96d3-d3b05623cb00"),
    "Psychic Theft",
    crate::card::CardArt::new("f6f86c92-c19e-4f2c-96d3-d3b05623cb00", "Don Hazeltine"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 41 — Quicksilver Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a680aeaf-8c6e-45b5-8814-7fd04e963220"),
    "Quicksilver Wall",
    crate::card::CardArt::new("a680aeaf-8c6e-45b5-8814-7fd04e963220", "Matt Cavotta"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 42 — Rethink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETHINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("915ae03f-22f3-4ecc-a875-5226d8dec384"),
    "Rethink",
    crate::card::CardArt::new("915ae03f-22f3-4ecc-a875-5226d8dec384", "Matt Cavotta"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 43 — Rhystic Deluge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_DELUGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a3dd540-7f54-46fe-b1e8-7b07f57e71d0"),
    "Rhystic Deluge",
    crate::card::CardArt::new("0a3dd540-7f54-46fe-b1e8-7b07f57e71d0", "Pete Venters"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 44 — Rhystic Scrying
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_SCRYING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81a59737-f06f-49b7-a490-3dc1115b47b7"),
    "Rhystic Scrying",
    crate::card::CardArt::new("81a59737-f06f-49b7-a490-3dc1115b47b7", "Roger Raupp"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 45 — Rhystic Study
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_STUDY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3394cefd-a3c6-4917-8f46-234e441ecfb6"),
    "Rhystic Study",
    crate::card::CardArt::new("3394cefd-a3c6-4917-8f46-234e441ecfb6", "Terese Nielsen"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 46 — Ribbon Snake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIBBON_SNAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb5135dc-4fc1-48a1-8405-44b2f93a3c21"),
    "Ribbon Snake",
    crate::card::CardArt::new("eb5135dc-4fc1-48a1-8405-44b2f93a3c21", "Mark Zug"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 47 — Shrouded Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHROUDED_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3d9035b-b6ec-479f-b697-3e5c3110ef10"),
    "Shrouded Serpent",
    crate::card::CardArt::new("d3d9035b-b6ec-479f-b697-3e5c3110ef10", "Dana Knutson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 48 — Spiketail Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKETAIL_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4db398ca-6b0b-4225-baaa-c4b1c243b2bd"),
    "Spiketail Drake",
    crate::card::CardArt::new("4db398ca-6b0b-4225-baaa-c4b1c243b2bd", "Michael Sutfin"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 49 — Spiketail Hatchling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIKETAIL_HATCHLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9988f0fe-a7d4-44f9-b37c-fa30014ea215"),
    "Spiketail Hatchling",
    crate::card::CardArt::new("9988f0fe-a7d4-44f9-b37c-fa30014ea215", "Greg Staples"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 50 — Stormwatch Eagle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMWATCH_EAGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21c3bb62-63b4-4b53-9e4d-edfc7487494b"),
    "Stormwatch Eagle",
    crate::card::CardArt::new("21c3bb62-63b4-4b53-9e4d-edfc7487494b", "Aaron Boyd"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 51 — Sunken Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNKEN_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9211ee02-e854-4414-92b1-65a7af29f0b9"),
    "Sunken Field",
    crate::card::CardArt::new("9211ee02-e854-4414-92b1-65a7af29f0b9", "Donato Giancola"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 52 — Troublesome Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TROUBLESOME_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d7e856-6852-4b97-ae0e-a4becdfc8166"),
    "Troublesome Spirit",
    crate::card::CardArt::new("23d7e856-6852-4b97-ae0e-a4becdfc8166", "Adam Rex"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 53 — Windscouter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINDSCOUTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb70925a-4bef-4067-a1d7-79114aff5847"),
    "Windscouter",
    crate::card::CardArt::new("bb70925a-4bef-4067-a1d7-79114aff5847", "Brian Snõddy"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 54 — Withdraw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITHDRAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1a3a52f-0ccd-4935-b3ca-9c69cba283cc"),
    "Withdraw",
    crate::card::CardArt::new("f1a3a52f-0ccd-4935-b3ca-9c69cba283cc", "Adam Rex"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 55 — Agent of Shauku
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_OF_SHAUKU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8316804-6f8b-423e-a2c3-fa476c095544"),
    "Agent of Shauku",
    crate::card::CardArt::new("d8316804-6f8b-423e-a2c3-fa476c095544", "Donato Giancola"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 56 — Avatar of Woe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_WOE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f695405-7238-48fb-9ea2-1b1613a0afda"),
    "Avatar of Woe",
    crate::card::CardArt::new("0f695405-7238-48fb-9ea2-1b1613a0afda", "rk post"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 57 — Bog Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75191915-352e-4de7-b216-63f0ff588ba5"),
    "Bog Elemental",
    crate::card::CardArt::new("75191915-352e-4de7-b216-63f0ff588ba5", "Glen Angus"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 58 — Bog Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("086a5620-704b-47a9-9a5d-73e28631d6f8"),
    "Bog Glider",
    crate::card::CardArt::new("086a5620-704b-47a9-9a5d-73e28631d6f8", "Brian Snõddy"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 59 — Chilling Apparition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHILLING_APPARITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c20edb71-aa1d-437b-bcfb-953efbe45150"),
    "Chilling Apparition",
    crate::card::CardArt::new("c20edb71-aa1d-437b-bcfb-953efbe45150", "Ron Spears"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 60 — Coffin Puppets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COFFIN_PUPPETS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afcda8e4-d3dc-44f8-b277-b61fa261666b"),
    "Coffin Puppets",
    crate::card::CardArt::new("afcda8e4-d3dc-44f8-b277-b61fa261666b", "Arnie Swekel"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 60s — Coffin Puppets (alternate printing)

// PCY 61 — Death Charmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_CHARMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e58a303a-9f7a-43e7-bcba-c58b378a53ce"),
    "Death Charmer",
    crate::card::CardArt::new("e58a303a-9f7a-43e7-bcba-c58b378a53ce", "David Martin"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 62 — Despoil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPOIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06bb6ff7-2cd6-430e-a618-0b83d9c1d044"),
    "Despoil",
    crate::card::CardArt::new("06bb6ff7-2cd6-430e-a618-0b83d9c1d044", "Scott M. Fischer"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 63 — Endbringer's Revel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDBRINGER_S_REVEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b76843b0-0e71-473b-8c9b-6a8bc30255da"),
    "Endbringer's Revel",
    crate::card::CardArt::new("b76843b0-0e71-473b-8c9b-6a8bc30255da", "Pete Venters"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 64 — Fen Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEN_STALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e7d1125-7eb0-4065-bc2c-764689380fa8"),
    "Fen Stalker",
    crate::card::CardArt::new(
        "8e7d1125-7eb0-4065-bc2c-764689380fa8",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 65 — Flay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6fe155f-bfb2-49d8-83f0-ab1047a961d1"),
    "Flay",
    crate::card::CardArt::new("a6fe155f-bfb2-49d8-83f0-ab1047a961d1", "Matthew D. Wilson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 66 — Greel, Mind Raker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEL_MIND_RAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9d1f317-efd1-4595-92e2-44815a2b8147"),
    "Greel, Mind Raker",
    crate::card::CardArt::new("e9d1f317-efd1-4595-92e2-44815a2b8147", "Brom"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 67 — Greel's Caress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEL_S_CARESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b25ce3f-fab3-40f8-8a16-fe580f3d97a5"),
    "Greel's Caress",
    crate::card::CardArt::new("5b25ce3f-fab3-40f8-8a16-fe580f3d97a5", "Chippy"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 68 — Infernal Genesis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_GENESIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a63d16e-319d-46f4-a28c-895b36605ee6"),
    "Infernal Genesis",
    crate::card::CardArt::new("1a63d16e-319d-46f4-a28c-895b36605ee6", "Ron Spencer"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 69 — Nakaya Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAKAYA_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eefd9315-9b7c-4c6b-8a15-a6af873dab6f"),
    "Nakaya Shade",
    crate::card::CardArt::new("eefd9315-9b7c-4c6b-8a15-a6af873dab6f", "Ray Lago"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 70 — Noxious Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOXIOUS_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10c84d09-555c-472b-b445-5dd5a44cd555"),
    "Noxious Field",
    crate::card::CardArt::new("10c84d09-555c-472b-b445-5dd5a44cd555", "Eric Peterson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 71 — Outbreak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTBREAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f43c30d9-23a5-4872-925d-3427f5f57995"),
    "Outbreak",
    crate::card::CardArt::new("f43c30d9-23a5-4872-925d-3427f5f57995", "Quinton Hoover"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 72 — Pit Raptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIT_RAPTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e37cd150-1064-43b7-919b-8922d8a18f21"),
    "Pit Raptor",
    crate::card::CardArt::new("e37cd150-1064-43b7-919b-8922d8a18f21", "Thomas Gianni"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 73 — Plague Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11f077f5-c0b0-4e94-8599-e2122bc87238"),
    "Plague Fiend",
    crate::card::CardArt::new("11f077f5-c0b0-4e94-8599-e2122bc87238", "David Martin"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 74 — Plague Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0d4bd20-7422-45ed-aa76-3ef055c556e7"),
    "Plague Wind",
    crate::card::CardArt::new("b0d4bd20-7422-45ed-aa76-3ef055c556e7", "Alan Pollack"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 75 — Rebel Informer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REBEL_INFORMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c98a71a8-291f-4d94-ada0-5f50f354cca7"),
    "Rebel Informer",
    crate::card::CardArt::new("c98a71a8-291f-4d94-ada0-5f50f354cca7", "Scott M. Fischer"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 76 — Rhystic Syphon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_SYPHON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("750c5df2-e299-4bf3-8018-725893702314"),
    "Rhystic Syphon",
    crate::card::CardArt::new("750c5df2-e299-4bf3-8018-725893702314", "Ron Spencer"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 77 — Rhystic Tutor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_TUTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e02c1609-9cac-460f-8504-a84e28c340c1"),
    "Rhystic Tutor",
    crate::card::CardArt::new("e02c1609-9cac-460f-8504-a84e28c340c1", "Dan Frazier"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 78 — Soul Strings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_STRINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e34f9d1b-a89a-439f-8aa9-b96a1bf892eb"),
    "Soul Strings",
    crate::card::CardArt::new("e34f9d1b-a89a-439f-8aa9-b96a1bf892eb", "Daren Bader"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 79 — Steal Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAL_STRENGTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5470b3bb-5061-4beb-9f44-b56c3b2fd816"),
    "Steal Strength",
    crate::card::CardArt::new(
        "5470b3bb-5061-4beb-9f44-b56c3b2fd816",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 80 — Wall of Vipers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_VIPERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00042443-4d4e-4087-b4e5-5e781e7cc5fa"),
    "Wall of Vipers",
    crate::card::CardArt::new("00042443-4d4e-4087-b4e5-5e781e7cc5fa", "Marc Fishman"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 81 — Whipstitched Zombie
pub(in crate::card::sets) static WHIPSTITCHED_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cd00b0b-2ac1-4926-a735-215f402ba1c4"),
    "Whipstitched Zombie",
    CardArt::new("9cd00b0b-2ac1-4926-a735-215f402ba1c4", "Mark Tedin"),
    CardSet::Prophecy,
    // The cheap end of the same deal: a 2/2 for two that keeps costing one
    // every turn it survives.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 2).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {B}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{B}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ),
);

// PCY 82 — Avatar of Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_FURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("528293b4-ce3b-4623-8ced-496701d7265b"),
    "Avatar of Fury",
    crate::card::CardArt::new("528293b4-ce3b-4623-8ced-496701d7265b", "rk post"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 83 — Barbed Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c76db48-6f05-49c3-a49c-587c0a8a3613"),
    "Barbed Field",
    crate::card::CardArt::new("1c76db48-6f05-49c3-a49c-587c0a8a3613", "Carl Critchlow"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 84 — Branded Brawlers
pub(in crate::card::sets) static BRANDED_BRAWLERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90a48065-fbf1-4f2a-993e-7061057a4c45"),
    "Branded Brawlers",
    crate::card::CardArt::new("90a48065-fbf1-4f2a-993e-7061057a4c45", "Scott M. Fischer"),
    CardSet::Prophecy,
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&BRAWLER_RESTRICTIONS),
);

// PCY 85 — Brutal Suppression
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRUTAL_SUPPRESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17b3725b-924d-4137-9078-1a28f06c84fa"),
    "Brutal Suppression",
    crate::card::CardArt::new("17b3725b-924d-4137-9078-1a28f06c84fa", "Val Mayerik"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 86 — Citadel of Pain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITADEL_OF_PAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66585109-77cb-42f1-9c14-3dac1d493b71"),
    "Citadel of Pain",
    crate::card::CardArt::new("66585109-77cb-42f1-9c14-3dac1d493b71", "Darrell Riche"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 87 — Devastate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVASTATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bfe7c990-a34b-475e-a612-447c22f998d3"),
    "Devastate",
    crate::card::CardArt::new("bfe7c990-a34b-475e-a612-447c22f998d3", "Greg Staples"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 88 — Fault Riders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAULT_RIDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c9d3579-3fc1-434e-8f26-d5dbd6344429"),
    "Fault Riders",
    crate::card::CardArt::new("1c9d3579-3fc1-434e-8f26-d5dbd6344429", "Dave Dorman"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 89 — Fickle Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FICKLE_EFREET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca6d047a-f3dc-4c34-9679-fb76037e4044"),
    "Fickle Efreet",
    crate::card::CardArt::new("ca6d047a-f3dc-4c34-9679-fb76037e4044", "Dave Dorman"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 90 — Flameshot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMESHOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f7b61a8-a1ff-4e2a-bc24-8990c61a5e5b"),
    "Flameshot",
    crate::card::CardArt::new("3f7b61a8-a1ff-4e2a-bc24-8990c61a5e5b", "Mark Brill"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 91 — Inflame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFLAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd7bc4c0-9bfd-444b-b22c-f1b7e1426807"),
    "Inflame",
    crate::card::CardArt::new("cd7bc4c0-9bfd-444b-b22c-f1b7e1426807", "Eric Peterson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 92 — Keldon Arsonist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_ARSONIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("113f58b1-d8d7-4544-8363-e2b96e9d2623"),
    "Keldon Arsonist",
    crate::card::CardArt::new("113f58b1-d8d7-4544-8363-e2b96e9d2623", "Paolo Parente"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 93 — Keldon Berserker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_BERSERKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("daa3f9c0-66ff-4b94-b0c3-e1c65d2040b9"),
    "Keldon Berserker",
    crate::card::CardArt::new("daa3f9c0-66ff-4b94-b0c3-e1c65d2040b9", "Paolo Parente"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 94 — Keldon Firebombers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_FIREBOMBERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3fc78b5-c259-4c67-810c-99655e72c2da"),
    "Keldon Firebombers",
    crate::card::CardArt::new("d3fc78b5-c259-4c67-810c-99655e72c2da", "Randy Gallegos"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 95 — Latulla, Keldon Overseer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LATULLA_KELDON_OVERSEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd3c7b0c-98bd-4c63-bbb0-80484a5ab26f"),
    "Latulla, Keldon Overseer",
    crate::card::CardArt::new("fd3c7b0c-98bd-4c63-bbb0-80484a5ab26f", "Brom"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 96 — Latulla's Orders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LATULLA_S_ORDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a56cd728-5c3c-4fd6-bb01-0bf0875508c7"),
    "Latulla's Orders",
    crate::card::CardArt::new("a56cd728-5c3c-4fd6-bb01-0bf0875508c7", "Ben Thompson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 97 — Lesser Gargadon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESSER_GARGADON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63ed7aec-a513-418e-9cef-e0c51203055b"),
    "Lesser Gargadon",
    crate::card::CardArt::new("63ed7aec-a513-418e-9cef-e0c51203055b", "Rob Alexander"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 98 — Panic Attack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANIC_ATTACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89ec751c-08b5-4afb-bc08-8b2735b24f59"),
    "Panic Attack",
    crate::card::CardArt::new("89ec751c-08b5-4afb-bc08-8b2735b24f59", "Mike Ploog"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 99 — Rhystic Lightning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21ce365e-3002-42e9-aeb5-1b845408271e"),
    "Rhystic Lightning",
    crate::card::CardArt::new("21ce365e-3002-42e9-aeb5-1b845408271e", "Roger Raupp"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 100 — Ridgeline Rager
pub(in crate::card::sets) static RIDGELINE_RAGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f663a4a-592a-4a3b-bbaf-e9c5c3049021"),
    "Ridgeline Rager",
    CardArt::new("5f663a4a-592a-4a3b-bbaf-e9c5c3049021", "Chippy"),
    CardSet::Prophecy,
    // Firebreathing on a 1/2: the body is a place to put mana rather than a
    // threat on its own.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Beast"], 1, 2).with_ability(
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
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

// PCY 101 — Scoria Cat
pub(in crate::card::sets) static SCORIA_CAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7274f791-c9f1-49a1-9002-10e94caee96e"),
    "Scoria Cat",
    CardArt::new("7274f791-c9f1-49a1-9002-10e94caee96e", "Andrew Goldhawk"),
    CardSet::Prophecy,
    // A 6/6 for exactly as long as the mana is all spent, which is the
    // turn the attack happens and no other.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Cat"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature gets +3/+3 as long as you control no untapped lands.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::Equal,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                },
            },
        ),
    ),
);

// PCY 102 — Search for Survivors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEARCH_FOR_SURVIVORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f19a1b5-48ba-44a9-b91f-2f628b223ffb"),
    "Search for Survivors",
    crate::card::CardArt::new("2f19a1b5-48ba-44a9-b91f-2f628b223ffb", "Mark Romanoski"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 103 — Searing Wind
pub(in crate::card::sets) static SEARING_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b761f97-3690-497a-b6ab-c71f61b8e841"),
    "Searing Wind",
    CardArt::new("7b761f97-3690-497a-b6ab-c71f61b8e841", "John Matson"),
    CardSet::Prophecy,
    // Nine mana to end the game from ten life, which is the whole reason
    // the card exists.
    CardRules::new_instant(mana_cost!("{8}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Searing Wind deals 10 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(10),
        },
    )]),
);

// PCY 104 — Spur Grappler
pub(in crate::card::sets) static SPUR_GRAPPLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50bf91a7-4d04-437c-a290-6adb52f25312"),
    "Spur Grappler",
    CardArt::new("50bf91a7-4d04-437c-a290-6adb52f25312", "Randy Gallegos"),
    CardSet::Prophecy,
    // The cheap version of the same deal: tapping out is the cost of the
    // bonus, and holding up a trick turns it off.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Beast"], 2, 1).with_ability(
        AbilityDef::static_ability(
            "This creature gets +2/+1 as long as you control no untapped lands.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::Equal,
                    right: ValueDef::Constant(0),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
    ),
);

// PCY 105 — Task Mage Assembly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASK_MAGE_ASSEMBLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("258a9cdd-c626-404e-b82d-01091f11f107"),
    "Task Mage Assembly",
    crate::card::CardArt::new("258a9cdd-c626-404e-b82d-01091f11f107", "Val Mayerik"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 106 — Veteran Brawlers
pub(in crate::card::sets) static VETERAN_BRAWLERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee4d3acb-68be-409d-beb7-92a7cbc0402f"),
    "Veteran Brawlers",
    crate::card::CardArt::new("ee4d3acb-68be-409d-beb7-92a7cbc0402f", "Paolo Parente"),
    CardSet::Prophecy,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Soldier"], 4, 4)
        .with_abilities(&BRAWLER_RESTRICTIONS),
);

// PCY 107 — Whip Sergeant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIP_SERGEANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e6b3f38-87c9-4cea-b9e5-b8fb42e64794"),
    "Whip Sergeant",
    crate::card::CardArt::new("2e6b3f38-87c9-4cea-b9e5-b8fb42e64794", "Paolo Parente"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 108 — Zerapa Minotaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZERAPA_MINOTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55a8e1ce-e394-48d6-938a-aa76c0273abe"),
    "Zerapa Minotaur",
    crate::card::CardArt::new("55a8e1ce-e394-48d6-938a-aa76c0273abe", "Mark Zug"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 109 — Avatar of Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVATAR_OF_MIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c97614db-167e-4ede-96bd-77ed90b57d4e"),
    "Avatar of Might",
    crate::card::CardArt::new("c97614db-167e-4ede-96bd-77ed90b57d4e", "rk post"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 110 — Calming Verse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALMING_VERSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec38c856-dc21-450d-9aa6-da16c91a489a"),
    "Calming Verse",
    crate::card::CardArt::new("ec38c856-dc21-450d-9aa6-da16c91a489a", "Rebecca Guay"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 111 — Darba
pub(in crate::card::sets) static DARBA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d82636dc-4b3e-44a8-bc72-dab1275dfb6d"),
    "Darba",
    CardArt::new("d82636dc-4b3e-44a8-bc72-dab1275dfb6d", "Heather Hudson"),
    CardSet::Prophecy,
    // Five power for four mana, rented at two green a turn, which green
    // can usually afford and rarely wants to.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Bird", "Beast"], 5, 4).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {G}{G}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{G}{G}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ),
);

// PCY 112 — Dual Nature
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUAL_NATURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6890e414-d7e1-4320-924c-083e65a2ae72"),
    "Dual Nature",
    crate::card::CardArt::new("6890e414-d7e1-4320-924c-083e65a2ae72", "Arnie Swekel"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 113 — Elephant Resurgence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEPHANT_RESURGENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22147f72-7ff8-40c4-9bdd-df41dce17dad"),
    "Elephant Resurgence",
    crate::card::CardArt::new("22147f72-7ff8-40c4-9bdd-df41dce17dad", "DiTerlizzi"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 114 — Forgotten Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fefbace-03cb-43db-a221-0be2b8784357"),
    "Forgotten Harvest",
    crate::card::CardArt::new("9fefbace-03cb-43db-a221-0be2b8784357", "DiTerlizzi"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 115 — Jolrael, Empress of Beasts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLRAEL_EMPRESS_OF_BEASTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad0ea1c3-e920-467b-a3c8-a6b1097c3e8d"),
    "Jolrael, Empress of Beasts",
    crate::card::CardArt::new("ad0ea1c3-e920-467b-a3c8-a6b1097c3e8d", "Matthew D. Wilson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 116 — Jolrael's Favor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLRAEL_S_FAVOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8275ecd7-f119-4cca-bef1-626a3272dd2c"),
    "Jolrael's Favor",
    crate::card::CardArt::new("8275ecd7-f119-4cca-bef1-626a3272dd2c", "Daren Bader"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 117 — Living Terrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_TERRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64558128-7990-470c-88c9-d47d622e44db"),
    "Living Terrain",
    crate::card::CardArt::new("64558128-7990-470c-88c9-d47d622e44db", "Andrew Goldhawk"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 118 — Marsh Boa
pub(in crate::card::sets) static MARSH_BOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87c99bb6-a483-4beb-b98c-eb641fe3d50a"),
    "Marsh Boa",
    CardArt::new("87c99bb6-a483-4beb-b98c-eb641fe3d50a", "Heather Hudson"),
    CardSet::Prophecy,
    // One green mana for a creature black cannot block, which is the whole
    // of Prophecy's off-colour hosing.
    CardRules::new_creature(mana_cost!("{G}"), &["Snake"], 1, 1)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// PCY 119 — Mungha Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUNGHA_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6addc915-2997-4b81-a026-97d4421cf17d"),
    "Mungha Wurm",
    crate::card::CardArt::new("6addc915-2997-4b81-a026-97d4421cf17d", "Greg Staples"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 120 — Pygmy Razorback
pub(in crate::card::sets) static PYGMY_RAZORBACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ad9744f-797a-4dd3-8617-192773be995c"),
    "Pygmy Razorback",
    CardArt::new("0ad9744f-797a-4dd3-8617-192773be995c", "Matt Cavotta"),
    CardSet::Prophecy,
    // Trample on a 2/1 for two: a point gets through the chump block, and
    // that is the whole card.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Boar"], 2, 1)
        .with_abilities(&[abilities::trample()]),
);

// PCY 121 — Rib Cage Spider
pub(in crate::card::sets) static RIB_CAGE_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d71bebea-1634-4d9a-b3ad-2e01ecacad7e"),
    "Rib Cage Spider",
    CardArt::new("d71bebea-1634-4d9a-b3ad-2e01ecacad7e", "Dana Knutson"),
    CardSet::Prophecy,
    // A 1/4 reach body: it stops fliers all game and never threatens
    // anything itself.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spider"], 1, 4)
        .with_ability(abilities::reach()),
);

// PCY 122 — Root Cage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_CAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db109497-7674-4067-a12b-dfb5c317a358"),
    "Root Cage",
    crate::card::CardArt::new("db109497-7674-4067-a12b-dfb5c317a358", "Glen Angus"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 123 — Silt Crawler
pub(in crate::card::sets) static SILT_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f334e864-4e62-4bc3-9470-661be3d879e2"),
    "Silt Crawler",
    CardArt::new("f334e864-4e62-4bc3-9470-661be3d879e2", "Arnie Swekel"),
    CardSet::Prophecy,
    // A 3/3 for three that costs the rest of the turn's mana, which is a
    // real drawback on the turn it lands.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Beast"], 3, 3).with_ability(
        abilities::enters_trigger(
            "When this creature enters, tap all lands you control.",
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            },
        ),
    ),
);

// PCY 124 — Snag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7401df8b-23e7-4485-9ed4-70118a66feed"),
    "Snag",
    crate::card::CardArt::new("7401df8b-23e7-4485-9ed4-70118a66feed", "Ron Spencer"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 125 — Spitting Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITTING_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f220fada-5cbb-4266-bca3-a44d51773d63"),
    "Spitting Spider",
    crate::card::CardArt::new(
        "f220fada-5cbb-4266-bca3-a44d51773d63",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 126 — Spore Frog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPORE_FROG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f752339-003d-4ded-b2bf-e4200fc8d5d6"),
    "Spore Frog",
    crate::card::CardArt::new("2f752339-003d-4ded-b2bf-e4200fc8d5d6", "Donato Giancola"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 127 — Squirrel Wrangler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRREL_WRANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7094be2a-454e-4e4d-a540-c5c80e37468a"),
    "Squirrel Wrangler",
    crate::card::CardArt::new("7094be2a-454e-4e4d-a540-c5c80e37468a", "Carl Critchlow"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 128 — Thresher Beast
pub(in crate::card::sets) static THRESHER_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57996732-c9e4-4271-9d5f-2a8c77f8d177"),
    "Thresher Beast",
    CardArt::new("57996732-c9e4-4271-9d5f-2a8c77f8d177", "Jeff Easley"),
    CardSet::Prophecy,
    // Four mana of land destruction for anyone who blocks it, which is
    // why the sensible answer is to take four.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Beast"], 4, 4).with_ability(AbilityDef::triggered(
        "Whenever this creature becomes blocked, defending player sacrifices a land of their choice.",
        TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::players(PlayerSetDef::Related(
                PlayerRelation::DefendingPlayer,
            )),
            object: ObjectPredicateDef::HasType(CardType::Land),
            count: ValueDef::Constant(1),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )),
);

// PCY 129 — Thrive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cb20099-fc53-4fdf-86f4-d7d8155c2af1"),
    "Thrive",
    crate::card::CardArt::new("9cb20099-fc53-4fdf-86f4-d7d8155c2af1", "Mike Ploog"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 130 — Verdant Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERDANT_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d123da53-9fd3-492b-beb7-76d1c0f5e4f6"),
    "Verdant Field",
    crate::card::CardArt::new("d123da53-9fd3-492b-beb7-76d1c0f5e4f6", "Ron Spears"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 131 — Vintara Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINTARA_ELEPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f99f7aef-7398-4e10-9f4e-68e7c294c101"),
    "Vintara Elephant",
    crate::card::CardArt::new("f99f7aef-7398-4e10-9f4e-68e7c294c101", "Tony Szczudlo"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 132 — Vintara Snapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINTARA_SNAPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("897edf01-fc6f-4835-b025-c137d921ce09"),
    "Vintara Snapper",
    crate::card::CardArt::new("897edf01-fc6f-4835-b025-c137d921ce09", "Joel Biske"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 133 — Vitalizing Wind
pub(in crate::card::sets) static VITALIZING_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fbd7c20-d527-4d97-9630-896d5e7bf1de"),
    "Vitalizing Wind",
    CardArt::new("0fbd7c20-d527-4d97-9630-896d5e7bf1de", "Jeff Easley"),
    CardSet::Prophecy,
    // Nine mana. If it resolves with any creatures out the game is over,
    // which is the only argument for the cost.
    CardRules::new_instant(mana_cost!("{8}{G}")).with_ability(AbilityDef::spell(
        "Creatures you control get +7/+7 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(7),
                ValueDef::Constant(7),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// PCY 134 — Wild Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_MIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a8c0be2-ba90-4e7a-b1a6-c68be550e33d"),
    "Wild Might",
    crate::card::CardArt::new("7a8c0be2-ba90-4e7a-b1a6-c68be550e33d", "Carl Critchlow"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 135 — Wing Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WING_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59e66be1-f18a-433f-8504-aa1e85e22023"),
    "Wing Storm",
    crate::card::CardArt::new("59e66be1-f18a-433f-8504-aa1e85e22023", "Heather Hudson"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 136 — Chimeric Idol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIMERIC_IDOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd0fb30e-e4d0-4271-a712-db40fa7650c3"),
    "Chimeric Idol",
    crate::card::CardArt::new("fd0fb30e-e4d0-4271-a712-db40fa7650c3", "Mark Tedin"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 137 — Copper-Leaf Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COPPER_LEAF_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7be413dd-d6e0-4bd3-8c14-4dbe44e8ee41"),
    "Copper-Leaf Angel",
    crate::card::CardArt::new(
        "7be413dd-d6e0-4bd3-8c14-4dbe44e8ee41",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 138 — Hollow Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLLOW_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("163add05-9ee9-4a8f-8838-3c4143ddc2f5"),
    "Hollow Warrior",
    crate::card::CardArt::new("163add05-9ee9-4a8f-8838-3c4143ddc2f5", "Adam Rex"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 139 — Keldon Battlewagon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_BATTLEWAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c810aa1-e367-4102-a5bd-6dc02d3023e8"),
    "Keldon Battlewagon",
    crate::card::CardArt::new("5c810aa1-e367-4102-a5bd-6dc02d3023e8", "Kev Walker"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 140 — Well of Discovery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_OF_DISCOVERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82331a8a-c0f1-4d89-87f8-1b1d0fccabb8"),
    "Well of Discovery",
    crate::card::CardArt::new("82331a8a-c0f1-4d89-87f8-1b1d0fccabb8", "Alan Rabinowitz"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 141 — Well of Life
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_OF_LIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf3f4fc3-3819-470d-92d9-98cb390f89b9"),
    "Well of Life",
    crate::card::CardArt::new("bf3f4fc3-3819-470d-92d9-98cb390f89b9", "Tom Wänerstrand"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 142 — Rhystic Cave
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHYSTIC_CAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ae74463-4426-4ad4-b7a2-324694854245"),
    "Rhystic Cave",
    crate::card::CardArt::new("4ae74463-4426-4ad4-b7a2-324694854245", "Rob Alexander"),
    crate::card::CardSet::Prophecy,
    crate::card::CardRules::unsupported(),
);

// PCY 143 — Wintermoon Mesa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTERMOON_MESA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f07144a6-6e47-4315-8353-f8958f014f41"),
    "Wintermoon Mesa",
    crate::card::CardArt::new("f07144a6-6e47-4315-8353-f8958f014f41", "Tom Wänerstrand"),
    crate::card::CardSet::Prophecy,
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
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&COFFIN_PUPPETS, 1), // PCY 60s
];
