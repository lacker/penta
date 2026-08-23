//! Invasion cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::exodus as catalog_exo;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2012::return_to_ravnica as catalog_rtr;
use crate::card::sets::y2013::gatecrash as catalog_gtc;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectRefDef, PlayerRelation,
    StackTargetKindDef, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

static FACT_OR_FICTION_PILE_MOVES: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveToZone {
        counters: None,
        object: abilities::CHOSEN_PILE,
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::MoveToZone {
        counters: None,
        object: abilities::UNCHOSEN_PILE,
        zone: ZoneKind::Graveyard,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
]);

// INV 1 — Alabaster Leech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c86b45d9-aba6-4c09-8605-037754ba7fd4"),
    "Alabaster Leech",
    crate::card::CardArt::new(
        "c86b45d9-aba6-4c09-8605-037754ba7fd4",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 2 — Angel of Mercy (reprint)

// INV 3 — Ardent Soldier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_SOLDIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39dce974-846f-4365-b0a5-851e38668e7d"),
    "Ardent Soldier",
    crate::card::CardArt::new("39dce974-846f-4365-b0a5-851e38668e7d", "Paolo Parente"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 4 — Atalya, Samite Master
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ATALYA_SAMITE_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90500e7a-f76d-453a-bda0-d56d3f7c7534"),
    "Atalya, Samite Master",
    crate::card::CardArt::new("90500e7a-f76d-453a-bda0-d56d3f7c7534", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 5 — Benalish Emissary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b82d56e-80d7-4be9-ac22-de3257efc458"),
    "Benalish Emissary",
    crate::card::CardArt::new("6b82d56e-80d7-4be9-ac22-de3257efc458", "Randy Gallegos"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 6 — Benalish Heralds
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_HERALDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13c6e51d-54eb-4e5b-9ec9-54521b16b8d1"),
    "Benalish Heralds",
    crate::card::CardArt::new("13c6e51d-54eb-4e5b-9ec9-54521b16b8d1", "Don Hazeltine"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 7 — Benalish Lancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_LANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a38d40a-e745-4fee-b179-f8c27e9b2fbd"),
    "Benalish Lancer",
    crate::card::CardArt::new("3a38d40a-e745-4fee-b179-f8c27e9b2fbd", "Paolo Parente"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 8 — Benalish Trapper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_TRAPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e312653d-c3e1-4c79-90d2-0963419b618c"),
    "Benalish Trapper",
    crate::card::CardArt::new("e312653d-c3e1-4c79-90d2-0963419b618c", "Ken Meyer, Jr."),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 9 — Blinding Light (reprint)

// INV 10 — Capashen Unicorn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAPASHEN_UNICORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec3e5741-88d7-4837-9b43-ba8304d9ee74"),
    "Capashen Unicorn",
    crate::card::CardArt::new("ec3e5741-88d7-4837-9b43-ba8304d9ee74", "Jerry Tiritilli"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 11 — Crimson Acolyte
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRIMSON_ACOLYTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1718028-3009-4bdd-9f6f-59c17edd1344"),
    "Crimson Acolyte",
    crate::card::CardArt::new("c1718028-3009-4bdd-9f6f-59c17edd1344", "Dany Orizio"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 12 — Crusading Knight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRUSADING_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4ab4640-1871-41dd-bd21-64741e21ba37"),
    "Crusading Knight",
    crate::card::CardArt::new(
        "a4ab4640-1871-41dd-bd21-64741e21ba37",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 13 — Death or Glory
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_OR_GLORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81f967c9-b38d-489d-96cc-44a6b1804e10"),
    "Death or Glory",
    crate::card::CardArt::new("81f967c9-b38d-489d-96cc-44a6b1804e10", "Jeff Easley"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 14 — Dismantling Blow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISMANTLING_BLOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39514d54-cb6c-4b3b-a3be-46db991be4d4"),
    "Dismantling Blow",
    crate::card::CardArt::new("39514d54-cb6c-4b3b-a3be-46db991be4d4", "Mark Tedin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 15 — Divine Presence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINE_PRESENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28cb898d-d6ce-410a-83bf-37962cca2735"),
    "Divine Presence",
    crate::card::CardArt::new("28cb898d-d6ce-410a-83bf-37962cca2735", "Ron Spears"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 16 — Fight or Flight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIGHT_OR_FLIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46bde162-3737-4b93-a27a-63b909a4183d"),
    "Fight or Flight",
    crate::card::CardArt::new("46bde162-3737-4b93-a27a-63b909a4183d", "Randy Gallegos"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 17 — Glimmering Angel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLIMMERING_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f14f55e4-eded-4a86-87f4-b8fa6f30bc0f"),
    "Glimmering Angel",
    crate::card::CardArt::new("f14f55e4-eded-4a86-87f4-b8fa6f30bc0f", "Ciruelo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 18 — Global Ruin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLOBAL_RUIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("336474b4-2cf5-44c0-b72c-f75f1a7ed928"),
    "Global Ruin",
    crate::card::CardArt::new("336474b4-2cf5-44c0-b72c-f75f1a7ed928", "Greg Staples"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 19 — Harsh Judgment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HARSH_JUDGMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34c78dee-ab45-4638-b89a-10686145b19a"),
    "Harsh Judgment",
    crate::card::CardArt::new("34c78dee-ab45-4638-b89a-10686145b19a", "Carl Critchlow"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 20 — Holy Day (reprint)

// INV 21 — Liberate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIBERATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96794470-31ea-478f-b11c-dc8342a508e2"),
    "Liberate",
    crate::card::CardArt::new("96794470-31ea-478f-b11c-dc8342a508e2", "Alan Pollack"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 22 — Obsidian Acolyte
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OBSIDIAN_ACOLYTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("868efcee-bb13-4b6f-b81b-99408685e4c4"),
    "Obsidian Acolyte",
    crate::card::CardArt::new("868efcee-bb13-4b6f-b81b-99408685e4c4", "Matthew D. Wilson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 23 — Orim's Touch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORIM_S_TOUCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("559f551e-7891-4c6d-8798-a25c0255fa3b"),
    "Orim's Touch",
    crate::card::CardArt::new("559f551e-7891-4c6d-8798-a25c0255fa3b", "Roger Raupp"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 24 — Pledge of Loyalty
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLEDGE_OF_LOYALTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6f98c26-5b30-400c-8af1-8c6c43065f63"),
    "Pledge of Loyalty",
    crate::card::CardArt::new("d6f98c26-5b30-400c-8af1-8c6c43065f63", "Franz Vohwinkel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 25 — Prison Barricade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRISON_BARRICADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("449c4800-8718-4593-a61e-03ad7f348c6d"),
    "Prison Barricade",
    crate::card::CardArt::new("449c4800-8718-4593-a61e-03ad7f348c6d", "Thomas Gianni"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 26 — Protective Sphere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PROTECTIVE_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef5ef13e-1cf0-42a9-95d0-30ade254d6a8"),
    "Protective Sphere",
    crate::card::CardArt::new("ef5ef13e-1cf0-42a9-95d0-30ade254d6a8", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 27 — Pure Reflection
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PURE_REFLECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbff85a6-a51b-424e-a86b-da52c9b3a9da"),
    "Pure Reflection",
    crate::card::CardArt::new("bbff85a6-a51b-424e-a86b-da52c9b3a9da", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 28 — Rampant Elephant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAMPANT_ELEPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("752642d2-3dad-4f58-b154-beb5982141dc"),
    "Rampant Elephant",
    crate::card::CardArt::new("752642d2-3dad-4f58-b154-beb5982141dc", "Alan Pollack"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 29 — Razorfoot Griffin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAZORFOOT_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("819e2046-9b78-4fd0-92f8-798bfac51195"),
    "Razorfoot Griffin",
    crate::card::CardArt::new("819e2046-9b78-4fd0-92f8-798bfac51195", "Ben Thompson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 30 — Restrain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RESTRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f6b5c765-619c-4db9-b509-91892fb65e8f"),
    "Restrain",
    crate::card::CardArt::new("f6b5c765-619c-4db9-b509-91892fb65e8f", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 31 — Reviving Dose
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REVIVING_DOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d44dd88-ad20-4d89-8831-d2dfa6873428"),
    "Reviving Dose",
    crate::card::CardArt::new(
        "8d44dd88-ad20-4d89-8831-d2dfa6873428",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 32 — Rewards of Diversity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REWARDS_OF_DIVERSITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04116b38-8fb1-47c6-b68d-060d0fc4a60d"),
    "Rewards of Diversity",
    crate::card::CardArt::new("04116b38-8fb1-47c6-b68d-060d0fc4a60d", "Darrell Riche"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 33 — Reya Dawnbringer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REYA_DAWNBRINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1e0e72b-e65e-4578-b610-9f529daa32d7"),
    "Reya Dawnbringer",
    crate::card::CardArt::new("e1e0e72b-e65e-4578-b610-9f529daa32d7", "Matthew D. Wilson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 34 — Rout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94bc55ed-b89b-4e22-b3f1-4ce0f8d180d7"),
    "Rout",
    crate::card::CardArt::new("94bc55ed-b89b-4e22-b3f1-4ce0f8d180d7", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 35 — Ruham Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUHAM_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a46c7718-1ecc-418c-b213-13be9de5cb7f"),
    "Ruham Djinn",
    crate::card::CardArt::new("a46c7718-1ecc-418c-b213-13be9de5cb7f", "Jeff Easley"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 36 — Samite Ministration
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_MINISTRATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1de62ed-79e6-4daf-a2ab-dc0726e1f7df"),
    "Samite Ministration",
    crate::card::CardArt::new("b1de62ed-79e6-4daf-a2ab-dc0726e1f7df", "Darrell Riche"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 37 — Shackles (reprint)

// INV 38 — Spirit of Resistance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_OF_RESISTANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fb66439-df73-4a01-a8d4-6f2334297fdf"),
    "Spirit of Resistance",
    crate::card::CardArt::new("5fb66439-df73-4a01-a8d4-6f2334297fdf", "John Avon"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 39 — Spirit Weaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90b0ef47-cb22-4146-a17e-e49a6031a7e6"),
    "Spirit Weaver",
    crate::card::CardArt::new("90b0ef47-cb22-4146-a17e-e49a6031a7e6", "Matthew D. Wilson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 40 — Strength of Unity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRENGTH_OF_UNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a9d4ff8-af35-413f-9aa2-f4c6e34fade2"),
    "Strength of Unity",
    crate::card::CardArt::new("1a9d4ff8-af35-413f-9aa2-f4c6e34fade2", "Andrew Goldhawk"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 41 — Sunscape Apprentice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9d6bd19-77c9-4a1a-a2d5-6f9737693fea"),
    "Sunscape Apprentice",
    crate::card::CardArt::new("a9d6bd19-77c9-4a1a-a2d5-6f9737693fea", "Stephanie Law"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 42 — Sunscape Master
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSCAPE_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebb7203d-529d-45d2-8e03-cd342c153f38"),
    "Sunscape Master",
    crate::card::CardArt::new("ebb7203d-529d-45d2-8e03-cd342c153f38", "Alan Rabinowitz"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 43 — Teferi's Care
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_CARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("031b1cc1-4468-4bc5-85c0-c22dce131225"),
    "Teferi's Care",
    crate::card::CardArt::new("031b1cc1-4468-4bc5-85c0-c22dce131225", "Scott Bailey"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 44 — Wayfaring Giant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WAYFARING_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57e45de5-0e8b-41d3-979b-ec5a29cac682"),
    "Wayfaring Giant",
    crate::card::CardArt::new(
        "57e45de5-0e8b-41d3-979b-ec5a29cac682",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 45 — Winnow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WINNOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d61748dd-4010-47da-8717-ca0147877057"),
    "Winnow",
    crate::card::CardArt::new("d61748dd-4010-47da-8717-ca0147877057", "Roger Raupp"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 46 — Barrin's Unmaking
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_UNMAKING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d4cecb0-12b5-4678-b5e7-8cec8fc86cef"),
    "Barrin's Unmaking",
    crate::card::CardArt::new("4d4cecb0-12b5-4678-b5e7-8cec8fc86cef", "Luca Zontini"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 47 — Blind Seer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLIND_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c54ec26-c7f1-4258-9cc9-1709987f293c"),
    "Blind Seer",
    crate::card::CardArt::new("5c54ec26-c7f1-4258-9cc9-1709987f293c", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 48 — Breaking Wave
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BREAKING_WAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b39cd77-97aa-4099-8405-366f82079758"),
    "Breaking Wave",
    crate::card::CardArt::new("1b39cd77-97aa-4099-8405-366f82079758", "Carl Critchlow"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 49 — Collective Restraint
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COLLECTIVE_RESTRAINT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d71daa57-ac02-4dd9-8c90-d38bdd45fb51"),
    "Collective Restraint",
    crate::card::CardArt::new("d71daa57-ac02-4dd9-8c90-d38bdd45fb51", "Alan Rabinowitz"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 50 — Crystal Spray
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_SPRAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8798a4f1-34bb-449d-a8cc-faf8bda8e0ab"),
    "Crystal Spray",
    crate::card::CardArt::new("8798a4f1-34bb-449d-a8cc-faf8bda8e0ab", "Jeff Miracola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 51 — Disrupt (reprint)

// INV 52 — Distorting Wake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISTORTING_WAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf48eec9-96be-4f53-9d9a-c6f02d44c995"),
    "Distorting Wake",
    crate::card::CardArt::new("cf48eec9-96be-4f53-9d9a-c6f02d44c995", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 53 — Dream Thrush
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREAM_THRUSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("258217df-ae88-4d93-895a-3fd242baacd1"),
    "Dream Thrush",
    crate::card::CardArt::new("258217df-ae88-4d93-895a-3fd242baacd1", "D. J. Cleland-Hura"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 54 — Empress Galina
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EMPRESS_GALINA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6851dbc7-f072-41e7-a899-897445d99425"),
    "Empress Galina",
    crate::card::CardArt::new("6851dbc7-f072-41e7-a899-897445d99425", "Matt Cavotta"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 55 — Essence Leak
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_LEAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9099b2e6-9ed8-4a9c-97ca-77cc47678228"),
    "Essence Leak",
    crate::card::CardArt::new("9099b2e6-9ed8-4a9c-97ca-77cc47678228", "Adam Rex"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 56 — Exclude
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXCLUDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aeb359c8-209c-455f-84b2-970e5678a9fa"),
    "Exclude",
    crate::card::CardArt::new("aeb359c8-209c-455f-84b2-970e5678a9fa", "Mark Romanoski"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 57 — Fact or Fiction
pub(in crate::card::sets) static FACT_OR_FICTION: CardRecord = CardRecord::new_with_legacy_id(
    277,
    "Fact or Fiction",
    CardArt::new(
        "7fd4d018-dcf3-4439-8445-02d66e44f7d3",
        "Terese Nielsen",
    ),
    CardSet::Invasion,
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell(
        "Reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard.",
        abilities::split_top_of_library_into_piles(
            ValueDef::Constant(5),
            &FACT_OR_FICTION_PILE_MOVES,
        ),
    )),
);

static OPT_DRAW: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(1),
};

static OPT_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Bottom,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: Some(&OPT_DRAW),
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

// INV 58 — Faerie Squadron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_SQUADRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c707c81-dbbd-43be-a79a-7bc92a584839"),
    "Faerie Squadron",
    crate::card::CardArt::new("4c707c81-dbbd-43be-a79a-7bc92a584839", "rk post"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 59 — Mana Maze
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_MAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d62cc17-8fa3-495c-a098-ffbbec89fa53"),
    "Mana Maze",
    crate::card::CardArt::new("0d62cc17-8fa3-495c-a098-ffbbec89fa53", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 60 — Manipulate Fate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANIPULATE_FATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bb52acb-dedb-4ed6-a6da-8c036f2b2958"),
    "Manipulate Fate",
    crate::card::CardArt::new("5bb52acb-dedb-4ed6-a6da-8c036f2b2958", "John Matson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 61 — Metathran Aerostat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_AEROSTAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59f34850-fb6f-4ac5-8309-4d53d770e28c"),
    "Metathran Aerostat",
    crate::card::CardArt::new("59f34850-fb6f-4ac5-8309-4d53d770e28c", "Greg Staples"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 62 — Metathran Transport
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_TRANSPORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4fa9048d-1599-44a5-b4b2-45382c5b238d"),
    "Metathran Transport",
    crate::card::CardArt::new("4fa9048d-1599-44a5-b4b2-45382c5b238d", "Glen Angus"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 63 — Metathran Zombie
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6676a0f7-8213-4547-b2ac-b904cd418073"),
    "Metathran Zombie",
    crate::card::CardArt::new("6676a0f7-8213-4547-b2ac-b904cd418073", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 64 — Opt
pub(in crate::card::sets) static OPT: CardRecord = CardRecord::new_with_legacy_id(
    312,
    "Opt",
    CardArt::new("958262ec-8e52-40cf-a9fd-a60e42643e15", "John Howe"),
    CardSet::Invasion,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Scry 1.\nDraw a card.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &OPT_SELECTION,
        },
    )),
);

/// Prohibit targets any spell and then asks how big it was, so a five-drop
/// can be named and simply survives. Both halves share the target; only the
/// ceiling moves.
static TARGET_SPELL: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Spell,
        zones: &[ZoneKind::Stack],
        controller: None,
        owner: None,
    },
)];

static COUNTER_TARGET_SPELL: EffectDef = EffectDef::Counter {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Graveyard,
};

static SMALL_ENOUGH_TO_COUNTER: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(2),
};

static BIG_ENOUGH_TO_COUNTER_KICKED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(4),
};

static PROHIBIT_UNKICKED: EffectDef = EffectDef::IfCondition {
    condition: &SMALL_ENOUGH_TO_COUNTER,
    then: &COUNTER_TARGET_SPELL,
};

static PROHIBIT_KICKED: EffectDef = EffectDef::IfCondition {
    condition: &BIG_ENOUGH_TO_COUNTER_KICKED,
    then: &COUNTER_TARGET_SPELL,
};

// INV 65 — Phantasmal Terrain (reprint)

// INV 66 — Probe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PROBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2a58d18-3d52-4178-86b2-7590d4164e76"),
    "Probe",
    crate::card::CardArt::new("a2a58d18-3d52-4178-86b2-7590d4164e76", "Eric Peterson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 67 — Prohibit
pub(in crate::card::sets) static PROHIBIT: CardRecord = CardRecord::new_with_legacy_id(
    2030,
    "Prohibit",
    CardArt::new("0daa5458-2a97-40d0-b18d-2381a7a68ee1", "Adam Rex"),
    CardSet::Invasion,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell if its mana value is 2 or less.",
            &TARGET_SPELL,
            PROHIBIT_UNKICKED,
        ),
        abilities::kicker(
            mana_cost!("{3}{U}"),
            "Counter target spell if its mana value is 4 or less.",
            &TARGET_SPELL,
            PROHIBIT_KICKED,
        ),
    ]),
);

static TARGET_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Artifact),
)];

static DESTROY_TARGET_ARTIFACT: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
};

static ARTIFACT_SMALL_ENOUGH: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(2),
};

static ARTIFACT_SMALL_ENOUGH_KICKED: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(5),
};

static OVERLOAD_UNKICKED: EffectDef = EffectDef::IfCondition {
    condition: &ARTIFACT_SMALL_ENOUGH,
    then: &DESTROY_TARGET_ARTIFACT,
};

static OVERLOAD_KICKED: EffectDef = EffectDef::IfCondition {
    condition: &ARTIFACT_SMALL_ENOUGH_KICKED,
    then: &DESTROY_TARGET_ARTIFACT,
};

/// A land you control, read off what the spell or ability already targets.
static A_LAND_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Land),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static RESPONSE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::StackObject {
        object: ObjectPredicateDef::TargetsObjectMatching(&A_LAND_YOU_CONTROL),
        controller: Some(PlayerRelation::Opponent),
        kind: StackTargetKindDef::SpellOrAbility,
    },
)];

/// The destroy follows the counter rather than preceding it: the countered
/// ability is retired with its source recorded, so the permanent is still
/// findable afterwards, and a spell -- which has no such source -- leaves
/// nothing to destroy.
static RESPONSE_EFFECT: EffectDef = EffectDef::Sequence(&[
    EffectDef::Counter {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Graveyard,
    },
    EffectDef::Destroy {
        object: EffectRecipientDef::object(ObjectRefDef::SourceOfTargetedStackObject(
            TargetIndex::PRIMARY,
        )),
        can_regenerate: true,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
]);

// INV 68 — Psychic Battle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_BATTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8758ca24-e613-43bf-be58-4cf557f82d0c"),
    "Psychic Battle",
    crate::card::CardArt::new("8758ca24-e613-43bf-be58-4cf557f82d0c", "Ray Lago"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 69 — Rainbow Crow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAINBOW_CROW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e622ad2-473f-489e-b4cf-bbdcc44d0cde"),
    "Rainbow Crow",
    crate::card::CardArt::new(
        "7e622ad2-473f-489e-b4cf-bbdcc44d0cde",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 70 — Repulse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REPULSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a04e9be-48be-440e-9825-cfffd4c2b1a4"),
    "Repulse",
    crate::card::CardArt::new("9a04e9be-48be-440e-9825-cfffd4c2b1a4", "Aaron Boyd"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 71 — Sapphire Leech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAPPHIRE_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6763ffd-9d89-4f26-871a-be24fbdef38d"),
    "Sapphire Leech",
    crate::card::CardArt::new("e6763ffd-9d89-4f26-871a-be24fbdef38d", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 72 — Shimmering Wings (reprint)

// INV 73 — Shoreline Raider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHORELINE_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d895b3b8-2acc-4c9f-8341-f651c1255b7c"),
    "Shoreline Raider",
    crate::card::CardArt::new("d895b3b8-2acc-4c9f-8341-f651c1255b7c", "Nelson DeCastro"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 74 — Sky Weaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKY_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04974146-42a8-4f10-b443-67bfeaa54d5d"),
    "Sky Weaver",
    crate::card::CardArt::new(
        "04974146-42a8-4f10-b443-67bfeaa54d5d",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 75 — Stormscape Apprentice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1eb42f39-9187-44e4-aa34-14ab31977199"),
    "Stormscape Apprentice",
    crate::card::CardArt::new(
        "1eb42f39-9187-44e4-aa34-14ab31977199",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 76 — Stormscape Master
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSCAPE_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b704165-4587-48f1-8830-c5a07ec666cc"),
    "Stormscape Master",
    crate::card::CardArt::new("9b704165-4587-48f1-8830-c5a07ec666cc", "Hannibal King"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 77 — Sway of Illusion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SWAY_OF_ILLUSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff65e386-9aec-4deb-a4ec-d9a97bd87645"),
    "Sway of Illusion",
    crate::card::CardArt::new(
        "ff65e386-9aec-4deb-a4ec-d9a97bd87645",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 78 — Teferi's Response
pub(in crate::card::sets) static TEFERIS_RESPONSE: CardRecord = CardRecord::new_with_legacy_id(
    2058,
    "Teferi's Response",
    CardArt::new("f3bb2df8-8b6e-4f7c-9e9a-6c8b0f4b8e2d", "Scott Bailey"),
    CardSet::Invasion,
    // The answer to Wasteland and Dust Bowl: the land lives, the thing that
    // came for it dies, and two cards make the exchange worth a card.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell or ability an opponent controls that targets a land you control. If a permanent's ability is countered this way, destroy that permanent.\nDraw two cards.",
        &RESPONSE_TARGET,
        RESPONSE_EFFECT,
    )),
);

// INV 79 — Temporal Distortion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORAL_DISTORTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74bd0d14-8d26-403f-9405-d0dcdecd1a49"),
    "Temporal Distortion",
    crate::card::CardArt::new("74bd0d14-8d26-403f-9405-d0dcdecd1a49", "Stephanie Law"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 80 — Tidal Visionary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_VISIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a72a3051-7f46-4b6b-b4fb-0f170d9687ab"),
    "Tidal Visionary",
    crate::card::CardArt::new("a72a3051-7f46-4b6b-b4fb-0f170d9687ab", "Glen Angus"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 81 — Tolarian Emissary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1cbc55e5-b84c-4449-a288-ec26cdd3997c"),
    "Tolarian Emissary",
    crate::card::CardArt::new("1cbc55e5-b84c-4449-a288-ec26cdd3997c", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 82 — Tower Drake (reprint)

// INV 83 — Traveler's Cloak
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRAVELER_S_CLOAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("977f0f82-0542-40c9-9a48-73077941dbd1"),
    "Traveler's Cloak",
    crate::card::CardArt::new("977f0f82-0542-40c9-9a48-73077941dbd1", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 84 — Vodalian Hypnotist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_HYPNOTIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("721fd877-0a28-4002-8b47-058bac4ac44d"),
    "Vodalian Hypnotist",
    crate::card::CardArt::new("721fd877-0a28-4002-8b47-058bac4ac44d", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 85 — Vodalian Merchant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_MERCHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1c0effa-a4b8-4166-a66a-90cf01c6ea0d"),
    "Vodalian Merchant",
    crate::card::CardArt::new("c1c0effa-a4b8-4166-a66a-90cf01c6ea0d", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 86 — Vodalian Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92adcf6c-ab14-414c-a5cb-56feae048c84"),
    "Vodalian Serpent",
    crate::card::CardArt::new(
        "92adcf6c-ab14-414c-a5cb-56feae048c84",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 87 — Wash Out
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WASH_OUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7719d043-5827-4479-825b-23d9e979ead7"),
    "Wash Out",
    crate::card::CardArt::new("7719d043-5827-4479-825b-23d9e979ead7", "Matthew D. Wilson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 88 — Well-Laid Plans
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_LAID_PLANS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c55eb8f-925a-42c1-9e48-d7f99cab3b01"),
    "Well-Laid Plans",
    crate::card::CardArt::new("1c55eb8f-925a-42c1-9e48-d7f99cab3b01", "Kev Walker"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 89 — Worldly Counsel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDLY_COUNSEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8fc66fbf-f411-4607-aece-7c35d9a07c80"),
    "Worldly Counsel",
    crate::card::CardArt::new("8fc66fbf-f411-4607-aece-7c35d9a07c80", "Gary Ruddell"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 90 — Zanam Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZANAM_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57a3c1d5-0ca8-443b-ae7a-66e0363e377b"),
    "Zanam Djinn",
    crate::card::CardArt::new("57a3c1d5-0ca8-443b-ae7a-66e0363e377b", "Eric Peterson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 91 — Addle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ADDLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8afb9d0-affa-4599-bf29-729cfe64703b"),
    "Addle",
    crate::card::CardArt::new("e8afb9d0-affa-4599-bf29-729cfe64703b", "Ron Spears"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 92 — Agonizing Demise
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AGONIZING_DEMISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("539ac5e1-4bad-4f70-abac-e70c406bebec"),
    "Agonizing Demise",
    crate::card::CardArt::new("539ac5e1-4bad-4f70-abac-e70c406bebec", "Mark Brill"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 93 — Andradite Leech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANDRADITE_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6da0d4f3-9216-406c-8f3e-b9bb0a11dc75"),
    "Andradite Leech",
    crate::card::CardArt::new("6da0d4f3-9216-406c-8f3e-b9bb0a11dc75", "Wayne England"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 94 — Annihilate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANNIHILATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a3bf039-ecf6-477e-997c-e32c55323c01"),
    "Annihilate",
    crate::card::CardArt::new("4a3bf039-ecf6-477e-997c-e32c55323c01", "Kev Walker"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 95 — Bog Initiate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOG_INITIATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8962dc3b-24ca-4c3c-ba1d-933c29cf7b73"),
    "Bog Initiate",
    crate::card::CardArt::new("8962dc3b-24ca-4c3c-ba1d-933c29cf7b73", "rk post"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 96 — Cremate (reprint)

// INV 97 — Crypt Angel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPT_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("522ddc6f-ec13-4a70-8f4c-b3c846b102fd"),
    "Crypt Angel",
    crate::card::CardArt::new("522ddc6f-ec13-4a70-8f4c-b3c846b102fd", "Todd Lockwood"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 98 — Cursed Flesh (reprint)

// INV 99 — Defiling Tears
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFILING_TEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db7cba29-9472-4874-bd54-37edf70645b2"),
    "Defiling Tears",
    crate::card::CardArt::new("db7cba29-9472-4874-bd54-37edf70645b2", "rk post"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 100 — Desperate Research
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_RESEARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a42ac7e-4a27-488c-a2e7-338b18103b02"),
    "Desperate Research",
    crate::card::CardArt::new("6a42ac7e-4a27-488c-a2e7-338b18103b02", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 101 — Devouring Strossus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOURING_STROSSUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("064f013f-e74f-419d-8d17-7748bd91885e"),
    "Devouring Strossus",
    crate::card::CardArt::new(
        "064f013f-e74f-419d-8d17-7748bd91885e",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 102 — Do or Die
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DO_OR_DIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05f63cd9-e82b-4cf8-b8ce-f0aa0157692b"),
    "Do or Die",
    crate::card::CardArt::new(
        "05f63cd9-e82b-4cf8-b8ce-f0aa0157692b",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 103 — Dredge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("68bfa3d5-0f0b-4684-9567-f1478da01df7"),
    "Dredge",
    crate::card::CardArt::new("68bfa3d5-0f0b-4684-9567-f1478da01df7", "Donato Giancola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 104 — Duskwalker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKWALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39a4a026-f44e-40e1-9942-a3d8448aca70"),
    "Duskwalker",
    crate::card::CardArt::new("39a4a026-f44e-40e1-9942-a3d8448aca70", "David Martin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 105 — Exotic Curse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXOTIC_CURSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ee35d99-9a8a-421b-bf43-74446909d87d"),
    "Exotic Curse",
    crate::card::CardArt::new("8ee35d99-9a8a-421b-bf43-74446909d87d", "Dany Orizio"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 106 — Firescreamer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIRESCREAMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("155a2213-bf6e-4a54-924b-e450b7d06f26"),
    "Firescreamer",
    crate::card::CardArt::new("155a2213-bf6e-4a54-924b-e450b7d06f26", "Alan Pollack"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 107 — Goham Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOHAM_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d67796c7-4d93-4c50-8839-bb69e075bc42"),
    "Goham Djinn",
    crate::card::CardArt::new("d67796c7-4d93-4c50-8839-bb69e075bc42", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 108 — Hate Weaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HATE_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8328e131-b44d-4dd0-9ce4-454c6afe6fa6"),
    "Hate Weaver",
    crate::card::CardArt::new("8328e131-b44d-4dd0-9ce4-454c6afe6fa6", "Roger Raupp"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 109 — Hypnotic Cloud
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HYPNOTIC_CLOUD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7502ea2-7555-449e-baee-6ecef5573a3b"),
    "Hypnotic Cloud",
    crate::card::CardArt::new("a7502ea2-7555-449e-baee-6ecef5573a3b", "Randy Gallegos"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 110 — Marauding Knight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARAUDING_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cea2a7de-c67e-4541-be8c-e5ef7b64d94a"),
    "Marauding Knight",
    crate::card::CardArt::new("cea2a7de-c67e-4541-be8c-e5ef7b64d94a", "Daren Bader"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 111 — Mourning
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOURNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4649d881-709f-4ed0-91de-744d232a82f5"),
    "Mourning",
    crate::card::CardArt::new("4649d881-709f-4ed0-91de-744d232a82f5", "Terese Nielsen"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 112 — Nightscape Apprentice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7498ca4c-614a-4776-8886-0a6ed58520f6"),
    "Nightscape Apprentice",
    crate::card::CardArt::new("7498ca4c-614a-4776-8886-0a6ed58520f6", "Andrew Goldhawk"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 113 — Nightscape Master
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSCAPE_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d86174b8-dd9e-4ece-bc23-4f9ac50bccd3"),
    "Nightscape Master",
    crate::card::CardArt::new("d86174b8-dd9e-4ece-bc23-4f9ac50bccd3", "Andrew Goldhawk"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 114 — Phyrexian Battleflies
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_BATTLEFLIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da27c489-c541-4b0d-a844-71aa65e55ceb"),
    "Phyrexian Battleflies",
    crate::card::CardArt::new("da27c489-c541-4b0d-a844-71aa65e55ceb", "Dan Frazier"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 115 — Phyrexian Delver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DELVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e66d87a5-7b67-4ec5-b5e2-518d67123118"),
    "Phyrexian Delver",
    crate::card::CardArt::new("e66d87a5-7b67-4ec5-b5e2-518d67123118", "Dana Knutson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 116 — Phyrexian Infiltrator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_INFILTRATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("224b8254-553d-4d88-8163-1f15e1244bd2"),
    "Phyrexian Infiltrator",
    crate::card::CardArt::new("224b8254-553d-4d88-8163-1f15e1244bd2", "Darrell Riche"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 117 — Phyrexian Reaper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_REAPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccdd498b-1081-43fe-8193-518337a5a3ea"),
    "Phyrexian Reaper",
    crate::card::CardArt::new("ccdd498b-1081-43fe-8193-518337a5a3ea", "Sam Wood"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 118 — Phyrexian Slayer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_SLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fa8c604-343f-4c94-ac25-439ab1845c19"),
    "Phyrexian Slayer",
    crate::card::CardArt::new("5fa8c604-343f-4c94-ac25-439ab1845c19", "Sam Wood"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 119 — Plague Spitter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_SPITTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8845e6bd-40ee-45ca-a099-53f19ff20a8a"),
    "Plague Spitter",
    crate::card::CardArt::new("8845e6bd-40ee-45ca-a099-53f19ff20a8a", "Chippy"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 120 — Ravenous Rats (reprint)

// INV 121 — Reckless Spite (reprint)

// INV 122 — Recover
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECOVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("771e695b-24e1-4c65-81e0-1624bda646e7"),
    "Recover",
    crate::card::CardArt::new("771e695b-24e1-4c65-81e0-1624bda646e7", "Nelson DeCastro"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 123 — Scavenged Weaponry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCAVENGED_WEAPONRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e8072a9-2699-4c6c-9556-67d91bd67a4b"),
    "Scavenged Weaponry",
    crate::card::CardArt::new("4e8072a9-2699-4c6c-9556-67d91bd67a4b", "Alan Pollack"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 124 — Soul Burn (reprint)

// INV 124s — Soul Burn (alternate printing)

// INV 124★ — Soul Burn (alternate printing)

// INV 125 — Spreading Plague
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPREADING_PLAGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac86055d-ce08-4b05-a92c-45e007ca0ba4"),
    "Spreading Plague",
    crate::card::CardArt::new("ac86055d-ce08-4b05-a92c-45e007ca0ba4", "Scott Bailey"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 126 — Tainted Well
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_WELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2eec00a1-7e12-42d2-8f46-de8ab7323c2c"),
    "Tainted Well",
    crate::card::CardArt::new("2eec00a1-7e12-42d2-8f46-de8ab7323c2c", "Val Mayerik"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 127 — Trench Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRENCH_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b076f85-d1bf-491a-af9d-f35b8e1bd163"),
    "Trench Wurm",
    crate::card::CardArt::new("1b076f85-d1bf-491a-af9d-f35b8e1bd163", "Wayne England"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 128 — Tsabo's Assassin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_S_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0047302d-4e3d-4327-9bb2-ecd5b00b00e3"),
    "Tsabo's Assassin",
    crate::card::CardArt::new("0047302d-4e3d-4327-9bb2-ecd5b00b00e3", "Glen Angus"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 129 — Tsabo's Decree
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_S_DECREE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c1a0ebd-1add-49e6-b5e6-5b26abb1de88"),
    "Tsabo's Decree",
    crate::card::CardArt::new("0c1a0ebd-1add-49e6-b5e6-5b26abb1de88", "Thomas M. Baxa"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 130 — Twilight's Call
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TWILIGHT_S_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c97c8a5-33b3-4f7f-a224-bb4df7b4bcc0"),
    "Twilight's Call",
    crate::card::CardArt::new("3c97c8a5-33b3-4f7f-a224-bb4df7b4bcc0", "Mark Romanoski"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 131 — Urborg Emissary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6912c71-1836-4e87-9a65-d577d903d03c"),
    "Urborg Emissary",
    crate::card::CardArt::new("e6912c71-1836-4e87-9a65-d577d903d03c", "Eric Peterson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 132 — Urborg Phantom
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_PHANTOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("397355b9-5b67-4973-972e-3505c500d116"),
    "Urborg Phantom",
    crate::card::CardArt::new("397355b9-5b67-4973-972e-3505c500d116", "Daren Bader"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 133 — Urborg Shambler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_SHAMBLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eaedd5c8-03c6-4bbb-bf83-632551830bd4"),
    "Urborg Shambler",
    crate::card::CardArt::new("eaedd5c8-03c6-4bbb-bf83-632551830bd4", "Pete Venters"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 134 — Urborg Skeleton (alternate printing)

// INV 134s — Urborg Skeleton (alternate printing)

// INV 134★ — Urborg Skeleton
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_SKELETON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("467e9486-1604-4fa2-ab1f-be0d7a036798"),
    "Urborg Skeleton",
    crate::card::CardArt::new("467e9486-1604-4fa2-ab1f-be0d7a036798", "Tom Wänerstrand"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 135 — Yawgmoth's Agenda
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAWGMOTH_S_AGENDA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50f7ea7f-4f17-4f78-b68e-693e265ca829"),
    "Yawgmoth's Agenda",
    crate::card::CardArt::new("50f7ea7f-4f17-4f78-b68e-693e265ca829", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 136 — Ancient Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8ccb5d0-735b-443f-addd-8b70f5f2c60d"),
    "Ancient Kavu",
    crate::card::CardArt::new("c8ccb5d0-735b-443f-addd-8b70f5f2c60d", "Glen Angus"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 137 — Bend or Break
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BEND_OR_BREAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b76b6660-d4b2-44de-a1a7-8d00811f90f6"),
    "Bend or Break",
    crate::card::CardArt::new("b76b6660-d4b2-44de-a1a7-8d00811f90f6", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 138 — Breath of Darigaaz
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BREATH_OF_DARIGAAZ: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("480bb7e3-df03-454d-ada0-592ef8a4a6f0"),
    "Breath of Darigaaz",
    crate::card::CardArt::new(
        "480bb7e3-df03-454d-ada0-592ef8a4a6f0",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 139 — Callous Giant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CALLOUS_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("330028c4-8e91-4fe3-a87d-1660dfd2507e"),
    "Callous Giant",
    crate::card::CardArt::new("330028c4-8e91-4fe3-a87d-1660dfd2507e", "Mark Brill"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 140 — Chaotic Strike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOTIC_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("061df8e4-6947-4bbb-9fe7-52ca4fd95d65"),
    "Chaotic Strike",
    crate::card::CardArt::new(
        "061df8e4-6947-4bbb-9fe7-52ca4fd95d65",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 141 — Collapsing Borders
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COLLAPSING_BORDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc019633-788e-4095-9610-6c0a432f7656"),
    "Collapsing Borders",
    crate::card::CardArt::new("cc019633-788e-4095-9610-6c0a432f7656", "Glen Angus"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 142 — Crown of Flames (reprint)

// INV 143 — Firebrand Ranger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIREBRAND_RANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee05211e-cf08-4dea-9740-ed06f8682153"),
    "Firebrand Ranger",
    crate::card::CardArt::new("ee05211e-cf08-4dea-9740-ed06f8682153", "Quinton Hoover"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 144 — Ghitu Fire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("78827acd-a526-411b-bd22-ab9b538c75dd"),
    "Ghitu Fire",
    crate::card::CardArt::new("78827acd-a526-411b-bd22-ab9b538c75dd", "Glen Angus"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 145 — Goblin Spy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a89a099-8805-4b26-babd-5d9f48ee406a"),
    "Goblin Spy",
    crate::card::CardArt::new("2a89a099-8805-4b26-babd-5d9f48ee406a", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 146 — Halam Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HALAM_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("369ade1f-e909-47ae-bb01-19588269ad8f"),
    "Halam Djinn",
    crate::card::CardArt::new("369ade1f-e909-47ae-bb01-19588269ad8f", "Adam Rex"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 147 — Hooded Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOODED_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5464b80a-22fe-42c7-a839-31667712fb2d"),
    "Hooded Kavu",
    crate::card::CardArt::new("5464b80a-22fe-42c7-a839-31667712fb2d", "John Howe"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 148 — Kavu Aggressor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_AGGRESSOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2832ad3-ce7f-44d2-beb2-c95d982905a6"),
    "Kavu Aggressor",
    crate::card::CardArt::new(
        "a2832ad3-ce7f-44d2-beb2-c95d982905a6",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 149 — Kavu Monarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_MONARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea63dfd5-d8d7-45b8-8219-1cc2b3de5666"),
    "Kavu Monarch",
    crate::card::CardArt::new("ea63dfd5-d8d7-45b8-8219-1cc2b3de5666", "Terese Nielsen"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 150 — Kavu Runner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_RUNNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bc1b462-4e3c-47cc-87c5-f6e29dd70c01"),
    "Kavu Runner",
    crate::card::CardArt::new("2bc1b462-4e3c-47cc-87c5-f6e29dd70c01", "Douglas Shuler"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 151 — Kavu Scout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cbc2670d-a3f4-47c2-b424-01fd379ff186"),
    "Kavu Scout",
    crate::card::CardArt::new("cbc2670d-a3f4-47c2-b424-01fd379ff186", "DiTerlizzi"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 152 — Lightning Dart
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_DART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54d05157-d154-4203-bf3e-add110cb1cee"),
    "Lightning Dart",
    crate::card::CardArt::new("54d05157-d154-4203-bf3e-add110cb1cee", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 153 — Loafing Giant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOAFING_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fab5f738-04d0-44c9-88ec-28469b668040"),
    "Loafing Giant",
    crate::card::CardArt::new(
        "fab5f738-04d0-44c9-88ec-28469b668040",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 154 — Mages' Contest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGES_CONTEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c516861c-68d9-4d02-a343-689dba0526c6"),
    "Mages' Contest",
    crate::card::CardArt::new("c516861c-68d9-4d02-a343-689dba0526c6", "Bradley Williams"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 155 — Maniacal Rage (reprint)

// INV 156 — Obliterate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OBLITERATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdabde40-2143-4677-b7b4-ea8fbf9b1f25"),
    "Obliterate",
    crate::card::CardArt::new("cdabde40-2143-4677-b7b4-ea8fbf9b1f25", "Kev Walker"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 157 — Overload
pub(in crate::card::sets) static OVERLOAD: CardRecord = CardRecord::new_with_legacy_id(
    2029,
    "Overload",
    CardArt::new("c91fca91-7296-422e-b251-d571b710ff71", "Gary Ruddell"),
    CardSet::Invasion,
    // One mana answers a Lotus Petal or a Cursed Scroll; three answers most
    // of what a Premodern deck actually plays.
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target artifact if its mana value is 2 or less.",
            &TARGET_ARTIFACT,
            OVERLOAD_UNKICKED,
        ),
        abilities::kicker(
            mana_cost!("{2}{R}"),
            "Destroy target artifact if its mana value is 5 or less.",
            &TARGET_ARTIFACT,
            OVERLOAD_KICKED,
        ),
    ]),
);

// INV 158 — Pouncing Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static POUNCING_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e6e2e49-7bde-43c1-8caf-43d237dfc052"),
    "Pouncing Kavu",
    crate::card::CardArt::new("7e6e2e49-7bde-43c1-8caf-43d237dfc052", "Adam Rex"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 159 — Rage Weaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAGE_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a654295d-b63c-4025-bf36-899023a8ba1d"),
    "Rage Weaver",
    crate::card::CardArt::new("a654295d-b63c-4025-bf36-899023a8ba1d", "John Matson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 160 — Rogue Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROGUE_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61e1a445-129d-4bb9-a8b0-3f55e3e0bc58"),
    "Rogue Kavu",
    crate::card::CardArt::new("61e1a445-129d-4bb9-a8b0-3f55e3e0bc58", "Darrell Riche"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 161 — Ruby Leech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUBY_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be621b12-4f4e-43a6-b65e-da4223e742b5"),
    "Ruby Leech",
    crate::card::CardArt::new("be621b12-4f4e-43a6-b65e-da4223e742b5", "Jacques Bredy"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 162 — Savage Offensive
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_OFFENSIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("356744f3-e444-4f4e-bf00-80bb6b2ef76f"),
    "Savage Offensive",
    crate::card::CardArt::new(
        "356744f3-e444-4f4e-bf00-80bb6b2ef76f",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 163 — Scarred Puma
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCARRED_PUMA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("067ff95e-c4dc-41bb-9677-67f51a09b05a"),
    "Scarred Puma",
    crate::card::CardArt::new("067ff95e-c4dc-41bb-9677-67f51a09b05a", "Aaron Boyd"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 164 — Scorching Lava
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHING_LAVA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a85437f-052e-494c-a9ee-265c4624a409"),
    "Scorching Lava",
    crate::card::CardArt::new("2a85437f-052e-494c-a9ee-265c4624a409", "Mark Tedin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 165 — Searing Rays
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEARING_RAYS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f66ff2d-f2d2-4a6b-bf26-b510de60c0b6"),
    "Searing Rays",
    crate::card::CardArt::new("4f66ff2d-f2d2-4a6b-bf26-b510de60c0b6", "Doug Chaffee"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 166 — Shivan Emissary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("945c596e-492e-4cf5-857c-4ddbbdd78485"),
    "Shivan Emissary",
    crate::card::CardArt::new("945c596e-492e-4cf5-857c-4ddbbdd78485", "Paolo Parente"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 167 — Shivan Harvest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47dbd765-d7ea-4181-bd22-5c749ad081af"),
    "Shivan Harvest",
    crate::card::CardArt::new("47dbd765-d7ea-4181-bd22-5c749ad081af", "Daren Bader"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 168 — Skittish Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTISH_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be806378-50a7-4416-9d99-1ea2c1f2b7cb"),
    "Skittish Kavu",
    crate::card::CardArt::new("be806378-50a7-4416-9d99-1ea2c1f2b7cb", "Pete Venters"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 169 — Skizzik
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKIZZIK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dc7732bc-e168-44d9-923a-db7e985bd6db"),
    "Skizzik",
    crate::card::CardArt::new("dc7732bc-e168-44d9-923a-db7e985bd6db", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 170 — Slimy Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLIMY_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e82044d-88cd-4ee4-8ec9-e71a0a85ed46"),
    "Slimy Kavu",
    crate::card::CardArt::new("8e82044d-88cd-4ee4-8ec9-e71a0a85ed46", "Randy Gallegos"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 171 — Stand or Fall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STAND_OR_FALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60c34970-a106-490c-ac37-6156eb7f34ce"),
    "Stand or Fall",
    crate::card::CardArt::new("60c34970-a106-490c-ac37-6156eb7f34ce", "Matt Cavotta"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 172 — Stun (reprint)

// INV 173 — Tectonic Instability
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TECTONIC_INSTABILITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0476cc6b-ecc6-44d6-9f44-a90d4ee85daa"),
    "Tectonic Instability",
    crate::card::CardArt::new("0476cc6b-ecc6-44d6-9f44-a90d4ee85daa", "Rob Alexander"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 174 — Thunderscape Apprentice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75a0b075-5414-48d3-a2b1-47dc20213e96"),
    "Thunderscape Apprentice",
    crate::card::CardArt::new(
        "75a0b075-5414-48d3-a2b1-47dc20213e96",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 175 — Thunderscape Master
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERSCAPE_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22abdc2f-bdc8-46c4-8ce2-f06befedbc32"),
    "Thunderscape Master",
    crate::card::CardArt::new("22abdc2f-bdc8-46c4-8ce2-f06befedbc32", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 176 — Tribal Flames
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIBAL_FLAMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b32531e-c759-4603-abd0-1724e8df70db"),
    "Tribal Flames",
    crate::card::CardArt::new("9b32531e-c759-4603-abd0-1724e8df70db", "Tony Szczudlo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 177 — Turf Wound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TURF_WOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91392e9f-f96a-4ac5-b1f1-c73540cf249e"),
    "Turf Wound",
    crate::card::CardArt::new("91392e9f-f96a-4ac5-b1f1-c73540cf249e", "Thomas Gianni"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 178 — Urza's Rage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61a25a35-3ae4-471e-adcd-d8baf2f77b68"),
    "Urza's Rage",
    crate::card::CardArt::new("61a25a35-3ae4-471e-adcd-d8baf2f77b68", "Matthew D. Wilson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 179 — Viashino Grappler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_GRAPPLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a94aeb4-349c-4394-848d-c1c9133856e2"),
    "Viashino Grappler",
    crate::card::CardArt::new("4a94aeb4-349c-4394-848d-c1c9133856e2", "Mark Romanoski"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 180 — Zap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7502ce01-b762-40fe-a064-c7b20b08a722"),
    "Zap",
    crate::card::CardArt::new("7502ce01-b762-40fe-a064-c7b20b08a722", "John Matson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 181 — Aggressive Urge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSIVE_URGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37e3154d-9b1c-4f93-9bc3-a39e68d59d23"),
    "Aggressive Urge",
    crate::card::CardArt::new(
        "37e3154d-9b1c-4f93-9bc3-a39e68d59d23",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 182 — Bind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfa51783-9ef8-4e51-ba0d-ce8439d83bdf"),
    "Bind",
    crate::card::CardArt::new("cfa51783-9ef8-4e51-ba0d-ce8439d83bdf", "Mark Zug"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 183 — Blurred Mongoose
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLURRED_MONGOOSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b073e3f-6a6f-495a-ab16-39d906b660f1"),
    "Blurred Mongoose",
    crate::card::CardArt::new("4b073e3f-6a6f-495a-ab16-39d906b660f1", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 184 — Canopy Surge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CANOPY_SURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e19d68e-7554-4627-a316-beb1f75fa494"),
    "Canopy Surge",
    crate::card::CardArt::new("2e19d68e-7554-4627-a316-beb1f75fa494", "Matt Cavotta"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 185 — Elfhame Sanctuary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELFHAME_SANCTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ab9a90c-5fd8-4f8c-b692-f98a2974810c"),
    "Elfhame Sanctuary",
    crate::card::CardArt::new("6ab9a90c-5fd8-4f8c-b692-f98a2974810c", "Alan Rabinowitz"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 186 — Elvish Champion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c19bb473-03b0-4e6d-a7da-0ec1e7707a68"),
    "Elvish Champion",
    crate::card::CardArt::new(
        "c19bb473-03b0-4e6d-a7da-0ec1e7707a68",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 187 — Explosive Growth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXPLOSIVE_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eabc1e77-404c-436b-bde1-be1b21d00584"),
    "Explosive Growth",
    crate::card::CardArt::new("eabc1e77-404c-436b-bde1-be1b21d00584", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 188 — Fertile Ground (reprint)

// INV 189 — Harrow (reprint)

// INV 190 — Jade Leech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JADE_LEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3392171d-ed25-46a1-91cc-a4f24537617d"),
    "Jade Leech",
    crate::card::CardArt::new("3392171d-ed25-46a1-91cc-a4f24537617d", "John Howe"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 191 — Kavu Chameleon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_CHAMELEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f726437b-a41a-4ee9-b0ee-e09327508615"),
    "Kavu Chameleon",
    crate::card::CardArt::new("f726437b-a41a-4ee9-b0ee-e09327508615", "John Howe"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 192 — Kavu Climber
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_CLIMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2063f31e-d972-411e-a265-1d409153b49c"),
    "Kavu Climber",
    crate::card::CardArt::new("2063f31e-d972-411e-a265-1d409153b49c", "Rob Alexander"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 193 — Kavu Lair
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_LAIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4581b53-23a0-4ca6-a77c-97d79e7a6570"),
    "Kavu Lair",
    crate::card::CardArt::new("f4581b53-23a0-4ca6-a77c-97d79e7a6570", "Chippy"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 194 — Kavu Titan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KAVU_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c5fb86d-1d9a-4da2-bb5b-4266faa20197"),
    "Kavu Titan",
    crate::card::CardArt::new("2c5fb86d-1d9a-4da2-bb5b-4266faa20197", "Todd Lockwood"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 195 — Llanowar Cavalry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_CAVALRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21d92191-a743-4916-bbe4-5e207e964d9b"),
    "Llanowar Cavalry",
    crate::card::CardArt::new("21d92191-a743-4916-bbe4-5e207e964d9b", "Eric Peterson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 196 — Llanowar Elite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e207863-de68-47e1-8c63-413b5fa48943"),
    "Llanowar Elite",
    crate::card::CardArt::new("3e207863-de68-47e1-8c63-413b5fa48943", "Kev Walker"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 197 — Llanowar Vanguard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_VANGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72e6ed79-bdfd-49f9-bfa4-be4196880487"),
    "Llanowar Vanguard",
    crate::card::CardArt::new(
        "72e6ed79-bdfd-49f9-bfa4-be4196880487",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 198 — Might Weaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIGHT_WEAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("032a4ec7-82ce-4ea0-b0dd-ebc40823a014"),
    "Might Weaver",
    crate::card::CardArt::new("032a4ec7-82ce-4ea0-b0dd-ebc40823a014", "Larry Elmore"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 199 — Molimo, Maro-Sorcerer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLIMO_MARO_SORCERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("750d3475-ae72-42c1-ae4d-638f8e7c6d1a"),
    "Molimo, Maro-Sorcerer",
    crate::card::CardArt::new("750d3475-ae72-42c1-ae4d-638f8e7c6d1a", "Mark Zug"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 200 — Nomadic Elf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NOMADIC_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b69e57a-5b19-450c-9cf5-c189e8505781"),
    "Nomadic Elf",
    crate::card::CardArt::new("3b69e57a-5b19-450c-9cf5-c189e8505781", "D. J. Cleland-Hura"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 201 — Pincer Spider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PINCER_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23271658-19ae-420d-beeb-4bed4fdbb891"),
    "Pincer Spider",
    crate::card::CardArt::new("23271658-19ae-420d-beeb-4bed4fdbb891", "Dan Frazier"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 202 — Pulse of Llanowar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PULSE_OF_LLANOWAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db09afe5-5f01-4f77-a239-12d7a6e59024"),
    "Pulse of Llanowar",
    crate::card::CardArt::new("db09afe5-5f01-4f77-a239-12d7a6e59024", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 203 — Quirion Elves (reprint)

// INV 204 — Quirion Sentinel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fc639ea-a925-4f1e-879f-b8fcb12bf257"),
    "Quirion Sentinel",
    crate::card::CardArt::new("2fc639ea-a925-4f1e-879f-b8fcb12bf257", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 205 — Quirion Trailblazer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUIRION_TRAILBLAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2b258c1-5fb4-4072-bb32-ad364df1874a"),
    "Quirion Trailblazer",
    crate::card::CardArt::new("c2b258c1-5fb4-4072-bb32-ad364df1874a", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 206 — Restock
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RESTOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11a013ff-7c99-445a-b9e0-0fc45036f068"),
    "Restock",
    crate::card::CardArt::new("11a013ff-7c99-445a-b9e0-0fc45036f068", "Daren Bader"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 207 — Rooting Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTING_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12c25a4c-d93a-402b-999f-0b9919123cc5"),
    "Rooting Kavu",
    crate::card::CardArt::new("12c25a4c-d93a-402b-999f-0b9919123cc5", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 208 — Saproling Infestation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_INFESTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8642e530-914c-4149-944a-c4966ee27299"),
    "Saproling Infestation",
    crate::card::CardArt::new("8642e530-914c-4149-944a-c4966ee27299", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 209 — Saproling Symbiosis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_SYMBIOSIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bb63748-5c84-43a0-8f17-a2a17f658337"),
    "Saproling Symbiosis",
    crate::card::CardArt::new("2bb63748-5c84-43a0-8f17-a2a17f658337", "Ciruelo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 210 — Scouting Trek
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUTING_TREK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b882e68-5c03-4ec6-9982-8c3b09847969"),
    "Scouting Trek",
    crate::card::CardArt::new("1b882e68-5c03-4ec6-9982-8c3b09847969", "Stephanie Law"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 211 — Serpentine Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENTINE_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("699f1fe8-02c6-4d95-9231-3f8aefe603da"),
    "Serpentine Kavu",
    crate::card::CardArt::new("699f1fe8-02c6-4d95-9231-3f8aefe603da", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 212 — Sulam Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SULAM_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7aeab16f-e104-47e7-81c7-b6e0123120d7"),
    "Sulam Djinn",
    crate::card::CardArt::new(
        "7aeab16f-e104-47e7-81c7-b6e0123120d7",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 213 — Tangle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b37e39c-8aa4-4938-a492-7dac5de98dfb"),
    "Tangle",
    crate::card::CardArt::new("6b37e39c-8aa4-4938-a492-7dac5de98dfb", "John Avon"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 214 — Thicket Elemental
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THICKET_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f80a56ed-3ebb-4e20-bf6a-e27127f762e8"),
    "Thicket Elemental",
    crate::card::CardArt::new("f80a56ed-3ebb-4e20-bf6a-e27127f762e8", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 215 — Thornscape Apprentice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_APPRENTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("505da522-73a8-4232-ae1a-d3365f3e598f"),
    "Thornscape Apprentice",
    crate::card::CardArt::new("505da522-73a8-4232-ae1a-d3365f3e598f", "Randy Gallegos"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 216 — Thornscape Master
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THORNSCAPE_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e8f164d-3782-4eaa-a4db-ab7082d45ee7"),
    "Thornscape Master",
    crate::card::CardArt::new("7e8f164d-3782-4eaa-a4db-ab7082d45ee7", "Larry Elmore"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 217 — Tranquility (reprint)

// INV 218 — Treefolk Healer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREEFOLK_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73c6f5c0-686d-4b3a-add7-487f9fff5faa"),
    "Treefolk Healer",
    crate::card::CardArt::new("73c6f5c0-686d-4b3a-add7-487f9fff5faa", "Matt Cavotta"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 219 — Utopia Tree
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UTOPIA_TREE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("720452e9-3245-4b0e-94b6-843cbcb641a5"),
    "Utopia Tree",
    crate::card::CardArt::new("720452e9-3245-4b0e-94b6-843cbcb641a5", "Gary Ruddell"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 220 — Verdeloth the Ancient
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VERDELOTH_THE_ANCIENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72d5fab1-fa20-4006-b19d-179d36238c9b"),
    "Verdeloth the Ancient",
    crate::card::CardArt::new("72d5fab1-fa20-4006-b19d-179d36238c9b", "Daren Bader"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 221 — Verduran Emissary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VERDURAN_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55f3361b-e2e7-4297-85c2-94323f90cc90"),
    "Verduran Emissary",
    crate::card::CardArt::new("55f3361b-e2e7-4297-85c2-94323f90cc90", "Alton Lawson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 222 — Vigorous Charge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIGOROUS_CHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af6f57ad-d370-4c81-8da0-c15d87725ab1"),
    "Vigorous Charge",
    crate::card::CardArt::new("af6f57ad-d370-4c81-8da0-c15d87725ab1", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 223 — Wallop
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALLOP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45ce5126-e7b1-41ab-9e56-1e12927c4d27"),
    "Wallop",
    crate::card::CardArt::new("45ce5126-e7b1-41ab-9e56-1e12927c4d27", "Mike Ploog"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 224 — Wandering Stream
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERING_STREAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6da5cb6c-253b-44f0-98f9-d75f42c6e14b"),
    "Wandering Stream",
    crate::card::CardArt::new("6da5cb6c-253b-44f0-98f9-d75f42c6e14b", "Quinton Hoover"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 225 — Whip Silk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHIP_SILK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10566804-fd15-4ef0-ad7d-cc979f4cc8c5"),
    "Whip Silk",
    crate::card::CardArt::new("10566804-fd15-4ef0-ad7d-cc979f4cc8c5", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 226 — Absorb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABSORB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d6a0f3e-457f-41f5-be26-5fb249874f1a"),
    "Absorb",
    crate::card::CardArt::new("5d6a0f3e-457f-41f5-be26-5fb249874f1a", "Andrew Goldhawk"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 227 — Aether Rift
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_RIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("692c186a-997c-4f7e-a339-bf84884e1019"),
    "Aether Rift",
    crate::card::CardArt::new("692c186a-997c-4f7e-a339-bf84884e1019", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 228 — Angelic Shield
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5aaa3e4e-4e08-4df2-9e0c-66e15a10fec4"),
    "Angelic Shield",
    crate::card::CardArt::new("5aaa3e4e-4e08-4df2-9e0c-66e15a10fec4", "Adam Rex"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 229 — Armadillo Cloak
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARMADILLO_CLOAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d816f98-6cb6-432c-b0a4-a0eed21658ac"),
    "Armadillo Cloak",
    crate::card::CardArt::new("9d816f98-6cb6-432c-b0a4-a0eed21658ac", "Paolo Parente"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 230 — Armored Guardian
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARMORED_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6de5e1bd-1d31-4f9f-b18d-d6f49bc7ef10"),
    "Armored Guardian",
    crate::card::CardArt::new("6de5e1bd-1d31-4f9f-b18d-d6f49bc7ef10", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 231 — Artifact Mutation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARTIFACT_MUTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5eef49c-a80f-4622-ba77-999f9151c841"),
    "Artifact Mutation",
    crate::card::CardArt::new("d5eef49c-a80f-4622-ba77-999f9151c841", "Greg Staples"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 232 — Aura Mutation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_MUTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38421179-615e-4aba-91a4-503bfee05403"),
    "Aura Mutation",
    crate::card::CardArt::new("38421179-615e-4aba-91a4-503bfee05403", "Pete Venters"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 233 — Aura Shards
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_SHARDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("df4039ef-af72-4267-ade9-fdb7c921279e"),
    "Aura Shards",
    crate::card::CardArt::new("df4039ef-af72-4267-ade9-fdb7c921279e", "Ron Spencer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 234 — Backlash
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BACKLASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dadf030d-5451-43fc-bf0c-c1629fdf88ec"),
    "Backlash",
    crate::card::CardArt::new("dadf030d-5451-43fc-bf0c-c1629fdf88ec", "Chippy"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 235 — Barrin's Spite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_SPITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d8ec4dc-c74a-4d49-856e-95703675fe9b"),
    "Barrin's Spite",
    crate::card::CardArt::new("6d8ec4dc-c74a-4d49-856e-95703675fe9b", "Terese Nielsen"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 236 — Blazing Specter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLAZING_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bd397be-0e61-4f41-b0cf-f0c9d2440da7"),
    "Blazing Specter",
    crate::card::CardArt::new("3bd397be-0e61-4f41-b0cf-f0c9d2440da7", "Marc Fishman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 237 — Captain Sisay
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_SISAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d24d441c-f37f-44fe-8a93-f5c89df807e4"),
    "Captain Sisay",
    crate::card::CardArt::new("d24d441c-f37f-44fe-8a93-f5c89df807e4", "Ray Lago"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 238 — Cauldron Dance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAULDRON_DANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dadcae0-f2b2-487c-bb93-0a2c073044c0"),
    "Cauldron Dance",
    crate::card::CardArt::new("8dadcae0-f2b2-487c-bb93-0a2c073044c0", "Donato Giancola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 239 — Charging Troll
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHARGING_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58956099-6b97-4c7b-ab23-9f9b4d50ef95"),
    "Charging Troll",
    crate::card::CardArt::new("58956099-6b97-4c7b-ab23-9f9b4d50ef95", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 240 — Cinder Shade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8dd933a-19ed-4d30-a94a-bfb2f66f8f13"),
    "Cinder Shade",
    crate::card::CardArt::new("b8dd933a-19ed-4d30-a94a-bfb2f66f8f13", "Nelson DeCastro"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 241 — Coalition Victory
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COALITION_VICTORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd8ad3aa-3225-45ae-8343-5991f5b52269"),
    "Coalition Victory",
    crate::card::CardArt::new("dd8ad3aa-3225-45ae-8343-5991f5b52269", "Eric Peterson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 242 — Crosis, the Purger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_THE_PURGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e5f336d8-12a4-482d-8ffd-c205858c72ba"),
    "Crosis, the Purger",
    crate::card::CardArt::new("e5f336d8-12a4-482d-8ffd-c205858c72ba", "Pete Venters"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 243 — Darigaaz, the Igniter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_THE_IGNITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54dcf5e3-4303-41a3-b54c-24a9d462ce07"),
    "Darigaaz, the Igniter",
    crate::card::CardArt::new("54dcf5e3-4303-41a3-b54c-24a9d462ce07", "Mark Zug"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 244 — Dromar, the Banisher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_THE_BANISHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cfcc3c72-fff5-454c-814c-eb952fd23ba9"),
    "Dromar, the Banisher",
    crate::card::CardArt::new("cfcc3c72-fff5-454c-814c-eb952fd23ba9", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 245 — Dueling Grounds
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DUELING_GROUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52760183-bee0-4ce0-96c0-074b88f78980"),
    "Dueling Grounds",
    crate::card::CardArt::new("52760183-bee0-4ce0-96c0-074b88f78980", "Pete Venters"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 246 — Fires of Yavimaya
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIRES_OF_YAVIMAYA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("967f1658-8777-46fc-a648-07fb19e46745"),
    "Fires of Yavimaya",
    crate::card::CardArt::new("967f1658-8777-46fc-a648-07fb19e46745", "Val Mayerik"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 247 — Frenzied Tilling (reprint)

// INV 248 — Galina's Knight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GALINA_S_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11b492d6-5e28-4f4b-942c-080d03cb0e92"),
    "Galina's Knight",
    crate::card::CardArt::new("11b492d6-5e28-4f4b-942c-080d03cb0e92", "David Martin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 249 — Hanna, Ship's Navigator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HANNA_SHIP_S_NAVIGATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("83a4e48d-6452-4245-bdad-63fe3263550e"),
    "Hanna, Ship's Navigator",
    crate::card::CardArt::new("83a4e48d-6452-4245-bdad-63fe3263550e", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 250 — Heroes' Reunion (reprint)

// INV 251 — Horned Cheetah
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HORNED_CHEETAH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a28ad983-ce91-40b6-a1ce-fe36ec7fbce8"),
    "Horned Cheetah",
    crate::card::CardArt::new("a28ad983-ce91-40b6-a1ce-fe36ec7fbce8", "John Matson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 252 — Hunting Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8943304a-89c9-48b0-97b4-3e1aa690ca4d"),
    "Hunting Kavu",
    crate::card::CardArt::new("8943304a-89c9-48b0-97b4-3e1aa690ca4d", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 253 — Kangee, Aerie Keeper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KANGEE_AERIE_KEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3afd7e8e-4fcc-4003-9791-7baf10ef1880"),
    "Kangee, Aerie Keeper",
    crate::card::CardArt::new("3afd7e8e-4fcc-4003-9791-7baf10ef1880", "Mark Romanoski"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 254 — Llanowar Knight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6c75d89-e432-49aa-a407-555b223b7eff"),
    "Llanowar Knight",
    crate::card::CardArt::new("e6c75d89-e432-49aa-a407-555b223b7eff", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 255 — Lobotomy (reprint)

// INV 256 — Meteor Storm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36489b24-f8a8-46b6-b879-0a5ce400a6dc"),
    "Meteor Storm",
    crate::card::CardArt::new("36489b24-f8a8-46b6-b879-0a5ce400a6dc", "John Avon"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 257 — Noble Panther
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_PANTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f327818-8222-4295-8cef-118757b34d17"),
    "Noble Panther",
    crate::card::CardArt::new("3f327818-8222-4295-8cef-118757b34d17", "Matt Cavotta"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 258 — Ordered Migration
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORDERED_MIGRATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04d83a07-6054-45f1-bdf9-07f2006238d2"),
    "Ordered Migration",
    crate::card::CardArt::new("04d83a07-6054-45f1-bdf9-07f2006238d2", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 259 — Overabundance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OVERABUNDANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4183e73d-609a-4292-b173-e39eb51949f3"),
    "Overabundance",
    crate::card::CardArt::new("4183e73d-609a-4292-b173-e39eb51949f3", "Ben Thompson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 260 — Plague Spores
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_SPORES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d106d56-a688-49cc-8d5d-0279a5a7c0a7"),
    "Plague Spores",
    crate::card::CardArt::new("0d106d56-a688-49cc-8d5d-0279a5a7c0a7", "Randy Gallegos"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 261 — Pyre Zombie
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYRE_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c030108-2995-4fb0-9b80-efdfdd0f11e0"),
    "Pyre Zombie",
    crate::card::CardArt::new("6c030108-2995-4fb0-9b80-efdfdd0f11e0", "Nelson DeCastro"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 262 — Raging Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9c77844-742c-48cf-9c1e-954ffe781e25"),
    "Raging Kavu",
    crate::card::CardArt::new("27573679-e9e5-4bfc-b5d5-85d4648b01b6", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 263 — Reckless Assault
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff0f568e-4d3a-40a5-b72a-63040ec5402d"),
    "Reckless Assault",
    crate::card::CardArt::new("ff0f568e-4d3a-40a5-b72a-63040ec5402d", "Jeff Easley"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 264 — Recoil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECOIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6a77be3-e3b0-40f5-a470-414bac49da60"),
    "Recoil",
    crate::card::CardArt::new("b6a77be3-e3b0-40f5-a470-414bac49da60", "Alan Pollack"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 265 — Reviving Vapors
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REVIVING_VAPORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47a23c32-e122-400b-b252-e636ea2e684b"),
    "Reviving Vapors",
    crate::card::CardArt::new("47a23c32-e122-400b-b252-e636ea2e684b", "Pete Venters"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 266 — Riptide Crab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIPTIDE_CRAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e42ae1d-62b4-4b19-aafc-f12bdd6fb8cc"),
    "Riptide Crab",
    crate::card::CardArt::new("7e42ae1d-62b4-4b19-aafc-f12bdd6fb8cc", "David Martin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 267 — Rith, the Awakener
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_THE_AWAKENER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c30be387-280d-49bd-a3d1-c1636ee931ce"),
    "Rith, the Awakener",
    crate::card::CardArt::new("c30be387-280d-49bd-a3d1-c1636ee931ce", "Carl Critchlow"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 268 — Sabertooth Nishoba
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SABERTOOTH_NISHOBA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8338c296-cf3f-41d7-b380-3fb4237cb41c"),
    "Sabertooth Nishoba",
    crate::card::CardArt::new("8338c296-cf3f-41d7-b380-3fb4237cb41c", "Gary Ruddell"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 269 — Samite Archer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07a262d7-6d0c-43d0-89b6-9f46a1a9eb69"),
    "Samite Archer",
    crate::card::CardArt::new("07a262d7-6d0c-43d0-89b6-9f46a1a9eb69", "Scott M. Fischer"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 270 — Seer's Vision
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEER_S_VISION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c94618a-808c-4b3c-8f34-45e64d0414d3"),
    "Seer's Vision",
    crate::card::CardArt::new("0c94618a-808c-4b3c-8f34-45e64d0414d3", "Rebecca Guay"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 271 — Shivan Zombie
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4c99269-f730-4d33-bbce-9e855e9ad0fc"),
    "Shivan Zombie",
    crate::card::CardArt::new("f4c99269-f730-4d33-bbce-9e855e9ad0fc", "Tony Szczudlo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 272 — Simoon (reprint)

// INV 273 — Sleeper's Robe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEPER_S_ROBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3411f0fd-8b85-4d0d-a202-701a24ffac9f"),
    "Sleeper's Robe",
    crate::card::CardArt::new("3411f0fd-8b85-4d0d-a202-701a24ffac9f", "Alan Pollack"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 274 — Slinking Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLINKING_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("070a7004-5a28-4ccb-8640-ad6b07b51ece"),
    "Slinking Serpent",
    crate::card::CardArt::new("070a7004-5a28-4ccb-8640-ad6b07b51ece", "Wayne England"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 275 — Smoldering Tar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SMOLDERING_TAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcdc55c0-c8ac-49d5-969b-9bf0ee8e696c"),
    "Smoldering Tar",
    crate::card::CardArt::new("fcdc55c0-c8ac-49d5-969b-9bf0ee8e696c", "David Day"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 276 — Spinal Embrace
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPINAL_EMBRACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("692ad1eb-62a3-4560-bf8e-35f7db73c7a3"),
    "Spinal Embrace",
    crate::card::CardArt::new("692ad1eb-62a3-4560-bf8e-35f7db73c7a3", "Donato Giancola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 277 — Stalking Assassin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STALKING_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff8cc71f-3070-497f-908f-35aa13a8a857"),
    "Stalking Assassin",
    crate::card::CardArt::new("ff8cc71f-3070-497f-908f-35aa13a8a857", "Dana Knutson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 278 — Sterling Grove
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STERLING_GROVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40b26aa3-8169-4978-9554-bd2fc8e18e3b"),
    "Sterling Grove",
    crate::card::CardArt::new("40b26aa3-8169-4978-9554-bd2fc8e18e3b", "Jeff Miracola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 279 — Teferi's Moat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_MOAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ed5845c-ef6d-4a7b-b725-b09d3e9bbc17"),
    "Teferi's Moat",
    crate::card::CardArt::new("9ed5845c-ef6d-4a7b-b725-b09d3e9bbc17", "rk post"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 280 — Treva, the Renewer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_THE_RENEWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ee67039-6cee-4a2d-b973-570f5060f550"),
    "Treva, the Renewer",
    crate::card::CardArt::new("4ee67039-6cee-4a2d-b973-570f5060f550", "Ciruelo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 281 — Tsabo Tavoc
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TSABO_TAVOC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccbe2539-7a7c-468b-a270-7ca1bdcccb1e"),
    "Tsabo Tavoc",
    crate::card::CardArt::new("ccbe2539-7a7c-468b-a270-7ca1bdcccb1e", "Michael Sutfin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 282 — Undermine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERMINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2334bc71-5f85-47ff-b393-601a1e746a4e"),
    "Undermine",
    crate::card::CardArt::new(
        "2334bc71-5f85-47ff-b393-601a1e746a4e",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 283 — Urborg Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97d1327e-bf87-423f-8a04-8124e45b9ae0"),
    "Urborg Drake",
    crate::card::CardArt::new("97d1327e-bf87-423f-8a04-8124e45b9ae0", "Sam Wood"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 284 — Vicious Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VICIOUS_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31e9e629-7c25-4d45-aa35-9ba5f95b43cb"),
    "Vicious Kavu",
    crate::card::CardArt::new("31e9e629-7c25-4d45-aa35-9ba5f95b43cb", "Kev Walker"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 285 — Vile Consumption
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VILE_CONSUMPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f7e5716-77f3-45d2-a40a-f5bf500f6ad7"),
    "Vile Consumption",
    crate::card::CardArt::new("7f7e5716-77f3-45d2-a40a-f5bf500f6ad7", "Heather Hudson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 286 — Vodalian Zombie
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_ZOMBIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f30a5a06-32ce-4d71-b71f-e3e1d8d4511a"),
    "Vodalian Zombie",
    crate::card::CardArt::new(
        "f30a5a06-32ce-4d71-b71f-e3e1d8d4511a",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 287 — Void
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62dc1df7-b9db-4f5f-a340-08287cd3d9e5"),
    "Void",
    crate::card::CardArt::new("62dc1df7-b9db-4f5f-a340-08287cd3d9e5", "Kev Walker"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 288 — Voracious Cobra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VORACIOUS_COBRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d8c5669-11a9-4d95-8431-7065037f1fb6"),
    "Voracious Cobra",
    crate::card::CardArt::new("9d8c5669-11a9-4d95-8431-7065037f1fb6", "Terese Nielsen"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 289 — Wings of Hope
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WINGS_OF_HOPE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be0d2402-f1ef-4a71-ac01-c7099c4ce54c"),
    "Wings of Hope",
    crate::card::CardArt::new("be0d2402-f1ef-4a71-ac01-c7099c4ce54c", "Wayne England"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 290 — Yavimaya Barbarian
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_BARBARIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e17377d-4dad-4144-b0ce-c849636096a2"),
    "Yavimaya Barbarian",
    crate::card::CardArt::new("8e17377d-4dad-4144-b0ce-c849636096a2", "Don Hazeltine"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 291 — Yavimaya Kavu
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1872f104-7cf1-41e3-b1b4-ca75c678e08b"),
    "Yavimaya Kavu",
    crate::card::CardArt::new("1872f104-7cf1-41e3-b1b4-ca75c678e08b", "Greg Staples"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 292 — Stand // Deliver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STAND_DELIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be8b338f-6f05-43c6-beeb-c5052cc0d6a9"),
    "Stand // Deliver",
    crate::card::CardArt::new("be8b338f-6f05-43c6-beeb-c5052cc0d6a9", "David Martin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 293 — Spite // Malice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPITE_MALICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("054f1845-196f-41c1-9682-042171cccd49"),
    "Spite // Malice",
    crate::card::CardArt::new("054f1845-196f-41c1-9682-042171cccd49", "David Martin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 294 — Pain // Suffering
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PAIN_SUFFERING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81be27d6-e16f-4158-b2b6-66a0f3315327"),
    "Pain // Suffering",
    crate::card::CardArt::new("81be27d6-e16f-4158-b2b6-66a0f3315327", "David Martin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 295 — Assault // Battery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ASSAULT_BATTERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ec6a889-c941-4898-a2f6-4d3863faf535"),
    "Assault // Battery",
    crate::card::CardArt::new("0ec6a889-c941-4898-a2f6-4d3863faf535", "Ben Thompson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 296 — Wax // Wane
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WAX_WANE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19859061-f5ec-4b7f-86a1-196f98648e0a"),
    "Wax // Wane",
    crate::card::CardArt::new("19859061-f5ec-4b7f-86a1-196f98648e0a", "Ben Thompson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 297 — Alloy Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALLOY_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1fb6d6a1-9d71-405b-9c93-1a7f06c67abd"),
    "Alloy Golem",
    crate::card::CardArt::new("1fb6d6a1-9d71-405b-9c93-1a7f06c67abd", "Greg Staples"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 298 — Bloodstone Cameo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODSTONE_CAMEO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9db32fa-64b2-4ef6-88f2-28e758d420bb"),
    "Bloodstone Cameo",
    crate::card::CardArt::new("f9db32fa-64b2-4ef6-88f2-28e758d420bb", "Tony Szczudlo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 299 — Chromatic Sphere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMATIC_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("920cd17f-9274-443e-906f-c9904f0658d5"),
    "Chromatic Sphere",
    crate::card::CardArt::new("920cd17f-9274-443e-906f-c9904f0658d5", "Luca Zontini"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 300 — Crosis's Attendant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CROSIS_S_ATTENDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45edc18c-2046-4d0e-92fe-a6cf4aaf1c6f"),
    "Crosis's Attendant",
    crate::card::CardArt::new("45edc18c-2046-4d0e-92fe-a6cf4aaf1c6f", "Arnie Swekel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 301 — Darigaaz's Attendant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARIGAAZ_S_ATTENDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f22b575-443a-4c06-8e75-d4140cbd3660"),
    "Darigaaz's Attendant",
    crate::card::CardArt::new("6f22b575-443a-4c06-8e75-d4140cbd3660", "Brom"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 302 — Drake-Skull Cameo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRAKE_SKULL_CAMEO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a3ce135-9c2f-45bd-b2db-c0e00c50c964"),
    "Drake-Skull Cameo",
    crate::card::CardArt::new("4a3ce135-9c2f-45bd-b2db-c0e00c50c964", "Dan Frazier"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 303 — Dromar's Attendant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROMAR_S_ATTENDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24936fa9-41a3-4da5-91cf-c28fa45f47c9"),
    "Dromar's Attendant",
    crate::card::CardArt::new("24936fa9-41a3-4da5-91cf-c28fa45f47c9", "Carl Critchlow"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 304 — Juntu Stakes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JUNTU_STAKES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ab7cf53-f62d-47e1-af70-ab12be0d22e2"),
    "Juntu Stakes",
    crate::card::CardArt::new("3ab7cf53-f62d-47e1-af70-ab12be0d22e2", "Mark Brill"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 305 — Lotus Guardian
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUS_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ddfc6396-5377-4ab3-9c10-8abcdeae2aa1"),
    "Lotus Guardian",
    crate::card::CardArt::new("ddfc6396-5377-4ab3-9c10-8abcdeae2aa1", "Dana Knutson"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 306 — Phyrexian Altar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_ALTAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25158cd5-749b-408c-9ab1-0f83e38730f7"),
    "Phyrexian Altar",
    crate::card::CardArt::new("25158cd5-749b-408c-9ab1-0f83e38730f7", "Ron Spears"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 307 — Phyrexian Lens
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_LENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ec9a91d-7af0-44a8-839f-fb9960be0ddd"),
    "Phyrexian Lens",
    crate::card::CardArt::new("6ec9a91d-7af0-44a8-839f-fb9960be0ddd", "Matt Cavotta"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 308 — Planar Portal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_PORTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24315eaa-ef55-4fd6-9145-e75b3de6f492"),
    "Planar Portal",
    crate::card::CardArt::new("24315eaa-ef55-4fd6-9145-e75b3de6f492", "Mark Tedin"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 309 — Power Armor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static POWER_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed1981dd-c0f3-4e9d-a1f1-8bea823326ef"),
    "Power Armor",
    crate::card::CardArt::new("ed1981dd-c0f3-4e9d-a1f1-8bea823326ef", "Doug Chaffee"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 310 — Rith's Attendant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RITH_S_ATTENDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a26e8130-7fe9-4ef4-98af-928814f5b130"),
    "Rith's Attendant",
    crate::card::CardArt::new("a26e8130-7fe9-4ef4-98af-928814f5b130", "Adam Rex"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 311 — Seashell Cameo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEASHELL_CAMEO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9efdbcad-e2e4-4f54-ade5-920b1853109e"),
    "Seashell Cameo",
    crate::card::CardArt::new("9efdbcad-e2e4-4f54-ade5-920b1853109e", "Tony Szczudlo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 312 — Sparring Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPARRING_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d829d9de-83fa-4feb-8efc-0075315163c6"),
    "Sparring Golem",
    crate::card::CardArt::new("d829d9de-83fa-4feb-8efc-0075315163c6", "Adam Rex"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 313 — Tek
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c1f38104-a699-4bb9-930a-699f7bbc338a"),
    "Tek",
    crate::card::CardArt::new("c1f38104-a699-4bb9-930a-699f7bbc338a", "Chippy"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 314 — Tigereye Cameo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIGEREYE_CAMEO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25976da8-338d-4f46-b8ea-78a0aa3daa35"),
    "Tigereye Cameo",
    crate::card::CardArt::new("25976da8-338d-4f46-b8ea-78a0aa3daa35", "Donato Giancola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 315 — Treva's Attendant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREVA_S_ATTENDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9857af81-fb95-4dc4-b048-9ce4e96d1eca"),
    "Treva's Attendant",
    crate::card::CardArt::new(
        "9857af81-fb95-4dc4-b048-9ce4e96d1eca",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 316 — Troll-Horn Cameo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TROLL_HORN_CAMEO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42b1ca6c-6ca0-4b02-885a-58cee3fa2aa8"),
    "Troll-Horn Cameo",
    crate::card::CardArt::new("42b1ca6c-6ca0-4b02-885a-58cee3fa2aa8", "Donato Giancola"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 317 — Tsabo's Web
pub(in crate::card::sets) static TSABOS_WEB: CardRecord = CardRecord::new_with_legacy_id(
    309,
    "Tsabo's Web",
    CardArt::new("0dee69f8-cceb-41b9-a0ee-6b2ac9f4bad9", "Carl Critchlow"),
    CardSet::Invasion,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "When this artifact enters, draw a card.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "Each land with an activated ability that isn't a mana ability doesn't untap during its controller's untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasNonManaActivatedAbility,
                    ]), &[ZoneKind::Battlefield], PlayerRelation::Any),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
    ]),
);

// INV 318 — Urza's Filter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_FILTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("680c75b1-e766-40be-84d7-2332047bb3de"),
    "Urza's Filter",
    crate::card::CardArt::new("680c75b1-e766-40be-84d7-2332047bb3de", "Dave Dorman"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 319 — Ancient Spring
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_SPRING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("004eefa4-947b-45fc-b45c-5263bfd763bc"),
    "Ancient Spring",
    crate::card::CardArt::new("004eefa4-947b-45fc-b45c-5263bfd763bc", "Don Hazeltine"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 320 — Archaeological Dig
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHAEOLOGICAL_DIG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35f55af0-5a46-4900-b3d0-ca796b710e07"),
    "Archaeological Dig",
    crate::card::CardArt::new("35f55af0-5a46-4900-b3d0-ca796b710e07", "Don Hazeltine"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 321 — Coastal Tower
pub(in crate::card::sets) static COASTAL_TOWER: CardRecord = CardRecord::new_with_legacy_id(
    282,
    "Coastal Tower",
    CardArt::new("d115dbff-e35b-495f-a1e3-19651895927e", "Don Hazeltine"),
    CardSet::Invasion,
    CardRules::new_land(&[]).with_abilities(&[
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

// INV 322 — Elfhame Palace
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELFHAME_PALACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65986555-a5d7-497e-876f-b8d967d6aa5b"),
    "Elfhame Palace",
    crate::card::CardArt::new("65986555-a5d7-497e-876f-b8d967d6aa5b", "Jerry Tiritilli"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 323 — Geothermal Crevice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GEOTHERMAL_CREVICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e744b593-13fe-4967-b492-ac02f5815e57"),
    "Geothermal Crevice",
    crate::card::CardArt::new("e744b593-13fe-4967-b492-ac02f5815e57", "John Avon"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 324 — Irrigation Ditch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IRRIGATION_DITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("977f1b44-166c-4faf-8a7b-d431707e90ce"),
    "Irrigation Ditch",
    crate::card::CardArt::new("977f1b44-166c-4faf-8a7b-d431707e90ce", "Rob Alexander"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 325 — Keldon Necropolis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_NECROPOLIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f0cccf6-b79b-4fff-89aa-801341598532"),
    "Keldon Necropolis",
    crate::card::CardArt::new("4f0cccf6-b79b-4fff-89aa-801341598532", "Franz Vohwinkel"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 326 — Salt Marsh
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SALT_MARSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed64934b-0e64-4b2f-97aa-c3fb7e6ce0b0"),
    "Salt Marsh",
    crate::card::CardArt::new("ed64934b-0e64-4b2f-97aa-c3fb7e6ce0b0", "Jerry Tiritilli"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 327 — Shivan Oasis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_OASIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9841f7e8-162c-44a3-96f3-af944fce15d1"),
    "Shivan Oasis",
    crate::card::CardArt::new("9841f7e8-162c-44a3-96f3-af944fce15d1", "Rob Alexander"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 328 — Sulfur Vent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUR_VENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22c66ed6-55fb-4c65-aac4-26d9cc3053b8"),
    "Sulfur Vent",
    crate::card::CardArt::new(
        "22c66ed6-55fb-4c65-aac4-26d9cc3053b8",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 329 — Tinder Farm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TINDER_FARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("989b5901-aeb0-4a48-8c53-3b0ec0e0deba"),
    "Tinder Farm",
    crate::card::CardArt::new("989b5901-aeb0-4a48-8c53-3b0ec0e0deba", "Rob Alexander"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 330 — Urborg Volcano
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_VOLCANO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c76f346c-ae34-4f5f-8e3b-6c77b0c4d530"),
    "Urborg Volcano",
    crate::card::CardArt::new("c76f346c-ae34-4f5f-8e3b-6c77b0c4d530", "Tony Szczudlo"),
    crate::card::CardSet::Invasion,
    crate::card::CardRules::unsupported(),
);

// INV 331 — Plains (reprint)

// INV 332 — Plains (alternate printing)

// INV 333 — Plains (alternate printing)

// INV 334 — Plains (alternate printing)

// INV 335 — Island (reprint)

// INV 336 — Island (alternate printing)

// INV 337 — Island (alternate printing)

// INV 338 — Island (alternate printing)

// INV 339 — Swamp (reprint)

// INV 340 — Swamp (alternate printing)

// INV 341 — Swamp (alternate printing)

// INV 342 — Swamp (alternate printing)

// INV 343 — Mountain (reprint)

// INV 344 — Mountain (alternate printing)

// INV 345 — Mountain (alternate printing)

// INV 346 — Mountain (alternate printing)

// INV 347 — Forest (reprint)

// INV 348 — Forest (alternate printing)

// INV 349 — Forest (alternate printing)

// INV 350 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ALABASTER_LEECH,
    &ARDENT_SOLDIER,
    &ATALYA_SAMITE_MASTER,
    &BENALISH_EMISSARY,
    &BENALISH_HERALDS,
    &BENALISH_LANCER,
    &BENALISH_TRAPPER,
    &CAPASHEN_UNICORN,
    &CRIMSON_ACOLYTE,
    &CRUSADING_KNIGHT,
    &DEATH_OR_GLORY,
    &DISMANTLING_BLOW,
    &DIVINE_PRESENCE,
    &FIGHT_OR_FLIGHT,
    &GLIMMERING_ANGEL,
    &GLOBAL_RUIN,
    &HARSH_JUDGMENT,
    &LIBERATE,
    &OBSIDIAN_ACOLYTE,
    &ORIM_S_TOUCH,
    &PLEDGE_OF_LOYALTY,
    &PRISON_BARRICADE,
    &PROTECTIVE_SPHERE,
    &PURE_REFLECTION,
    &RAMPANT_ELEPHANT,
    &RAZORFOOT_GRIFFIN,
    &RESTRAIN,
    &REVIVING_DOSE,
    &REWARDS_OF_DIVERSITY,
    &REYA_DAWNBRINGER,
    &ROUT,
    &RUHAM_DJINN,
    &SAMITE_MINISTRATION,
    &SPIRIT_OF_RESISTANCE,
    &SPIRIT_WEAVER,
    &STRENGTH_OF_UNITY,
    &SUNSCAPE_APPRENTICE,
    &SUNSCAPE_MASTER,
    &TEFERI_S_CARE,
    &WAYFARING_GIANT,
    &WINNOW,
    &BARRIN_S_UNMAKING,
    &BLIND_SEER,
    &BREAKING_WAVE,
    &COLLECTIVE_RESTRAINT,
    &CRYSTAL_SPRAY,
    &DISTORTING_WAKE,
    &DREAM_THRUSH,
    &EMPRESS_GALINA,
    &ESSENCE_LEAK,
    &EXCLUDE,
    &FACT_OR_FICTION,
    &FAERIE_SQUADRON,
    &MANA_MAZE,
    &MANIPULATE_FATE,
    &METATHRAN_AEROSTAT,
    &METATHRAN_TRANSPORT,
    &METATHRAN_ZOMBIE,
    &OPT,
    &PROBE,
    &PROHIBIT,
    &PSYCHIC_BATTLE,
    &RAINBOW_CROW,
    &REPULSE,
    &SAPPHIRE_LEECH,
    &SHORELINE_RAIDER,
    &SKY_WEAVER,
    &STORMSCAPE_APPRENTICE,
    &STORMSCAPE_MASTER,
    &SWAY_OF_ILLUSION,
    &TEFERIS_RESPONSE,
    &TEMPORAL_DISTORTION,
    &TIDAL_VISIONARY,
    &TOLARIAN_EMISSARY,
    &TRAVELER_S_CLOAK,
    &VODALIAN_HYPNOTIST,
    &VODALIAN_MERCHANT,
    &VODALIAN_SERPENT,
    &WASH_OUT,
    &WELL_LAID_PLANS,
    &WORLDLY_COUNSEL,
    &ZANAM_DJINN,
    &ADDLE,
    &AGONIZING_DEMISE,
    &ANDRADITE_LEECH,
    &ANNIHILATE,
    &BOG_INITIATE,
    &CRYPT_ANGEL,
    &DEFILING_TEARS,
    &DESPERATE_RESEARCH,
    &DEVOURING_STROSSUS,
    &DO_OR_DIE,
    &DREDGE,
    &DUSKWALKER,
    &EXOTIC_CURSE,
    &FIRESCREAMER,
    &GOHAM_DJINN,
    &HATE_WEAVER,
    &HYPNOTIC_CLOUD,
    &MARAUDING_KNIGHT,
    &MOURNING,
    &NIGHTSCAPE_APPRENTICE,
    &NIGHTSCAPE_MASTER,
    &PHYREXIAN_BATTLEFLIES,
    &PHYREXIAN_DELVER,
    &PHYREXIAN_INFILTRATOR,
    &PHYREXIAN_REAPER,
    &PHYREXIAN_SLAYER,
    &PLAGUE_SPITTER,
    &RECOVER,
    &SCAVENGED_WEAPONRY,
    &SPREADING_PLAGUE,
    &TAINTED_WELL,
    &TRENCH_WURM,
    &TSABO_S_ASSASSIN,
    &TSABO_S_DECREE,
    &TWILIGHT_S_CALL,
    &URBORG_EMISSARY,
    &URBORG_PHANTOM,
    &URBORG_SHAMBLER,
    &URBORG_SKELETON,
    &YAWGMOTH_S_AGENDA,
    &ANCIENT_KAVU,
    &BEND_OR_BREAK,
    &BREATH_OF_DARIGAAZ,
    &CALLOUS_GIANT,
    &CHAOTIC_STRIKE,
    &COLLAPSING_BORDERS,
    &FIREBRAND_RANGER,
    &GHITU_FIRE,
    &GOBLIN_SPY,
    &HALAM_DJINN,
    &HOODED_KAVU,
    &KAVU_AGGRESSOR,
    &KAVU_MONARCH,
    &KAVU_RUNNER,
    &KAVU_SCOUT,
    &LIGHTNING_DART,
    &LOAFING_GIANT,
    &MAGES_CONTEST,
    &OBLITERATE,
    &OVERLOAD,
    &POUNCING_KAVU,
    &RAGE_WEAVER,
    &ROGUE_KAVU,
    &RUBY_LEECH,
    &SAVAGE_OFFENSIVE,
    &SCARRED_PUMA,
    &SCORCHING_LAVA,
    &SEARING_RAYS,
    &SHIVAN_EMISSARY,
    &SHIVAN_HARVEST,
    &SKITTISH_KAVU,
    &SKIZZIK,
    &SLIMY_KAVU,
    &STAND_OR_FALL,
    &TECTONIC_INSTABILITY,
    &THUNDERSCAPE_APPRENTICE,
    &THUNDERSCAPE_MASTER,
    &TRIBAL_FLAMES,
    &TURF_WOUND,
    &URZA_S_RAGE,
    &VIASHINO_GRAPPLER,
    &ZAP,
    &AGGRESSIVE_URGE,
    &BIND,
    &BLURRED_MONGOOSE,
    &CANOPY_SURGE,
    &ELFHAME_SANCTUARY,
    &ELVISH_CHAMPION,
    &EXPLOSIVE_GROWTH,
    &JADE_LEECH,
    &KAVU_CHAMELEON,
    &KAVU_CLIMBER,
    &KAVU_LAIR,
    &KAVU_TITAN,
    &LLANOWAR_CAVALRY,
    &LLANOWAR_ELITE,
    &LLANOWAR_VANGUARD,
    &MIGHT_WEAVER,
    &MOLIMO_MARO_SORCERER,
    &NOMADIC_ELF,
    &PINCER_SPIDER,
    &PULSE_OF_LLANOWAR,
    &QUIRION_SENTINEL,
    &QUIRION_TRAILBLAZER,
    &RESTOCK,
    &ROOTING_KAVU,
    &SAPROLING_INFESTATION,
    &SAPROLING_SYMBIOSIS,
    &SCOUTING_TREK,
    &SERPENTINE_KAVU,
    &SULAM_DJINN,
    &TANGLE,
    &THICKET_ELEMENTAL,
    &THORNSCAPE_APPRENTICE,
    &THORNSCAPE_MASTER,
    &TREEFOLK_HEALER,
    &UTOPIA_TREE,
    &VERDELOTH_THE_ANCIENT,
    &VERDURAN_EMISSARY,
    &VIGOROUS_CHARGE,
    &WALLOP,
    &WANDERING_STREAM,
    &WHIP_SILK,
    &ABSORB,
    &AETHER_RIFT,
    &ANGELIC_SHIELD,
    &ARMADILLO_CLOAK,
    &ARMORED_GUARDIAN,
    &ARTIFACT_MUTATION,
    &AURA_MUTATION,
    &AURA_SHARDS,
    &BACKLASH,
    &BARRIN_S_SPITE,
    &BLAZING_SPECTER,
    &CAPTAIN_SISAY,
    &CAULDRON_DANCE,
    &CHARGING_TROLL,
    &CINDER_SHADE,
    &COALITION_VICTORY,
    &CROSIS_THE_PURGER,
    &DARIGAAZ_THE_IGNITER,
    &DROMAR_THE_BANISHER,
    &DUELING_GROUNDS,
    &FIRES_OF_YAVIMAYA,
    &GALINA_S_KNIGHT,
    &HANNA_SHIP_S_NAVIGATOR,
    &HORNED_CHEETAH,
    &HUNTING_KAVU,
    &KANGEE_AERIE_KEEPER,
    &LLANOWAR_KNIGHT,
    &METEOR_STORM,
    &NOBLE_PANTHER,
    &ORDERED_MIGRATION,
    &OVERABUNDANCE,
    &PLAGUE_SPORES,
    &PYRE_ZOMBIE,
    &RAGING_KAVU,
    &RECKLESS_ASSAULT,
    &RECOIL,
    &REVIVING_VAPORS,
    &RIPTIDE_CRAB,
    &RITH_THE_AWAKENER,
    &SABERTOOTH_NISHOBA,
    &SAMITE_ARCHER,
    &SEER_S_VISION,
    &SHIVAN_ZOMBIE,
    &SLEEPER_S_ROBE,
    &SLINKING_SERPENT,
    &SMOLDERING_TAR,
    &SPINAL_EMBRACE,
    &STALKING_ASSASSIN,
    &STERLING_GROVE,
    &TEFERI_S_MOAT,
    &TREVA_THE_RENEWER,
    &TSABO_TAVOC,
    &UNDERMINE,
    &URBORG_DRAKE,
    &VICIOUS_KAVU,
    &VILE_CONSUMPTION,
    &VODALIAN_ZOMBIE,
    &VOID,
    &VORACIOUS_COBRA,
    &WINGS_OF_HOPE,
    &YAVIMAYA_BARBARIAN,
    &YAVIMAYA_KAVU,
    &STAND_DELIVER,
    &SPITE_MALICE,
    &PAIN_SUFFERING,
    &ASSAULT_BATTERY,
    &WAX_WANE,
    &ALLOY_GOLEM,
    &BLOODSTONE_CAMEO,
    &CHROMATIC_SPHERE,
    &CROSIS_S_ATTENDANT,
    &DARIGAAZ_S_ATTENDANT,
    &DRAKE_SKULL_CAMEO,
    &DROMAR_S_ATTENDANT,
    &JUNTU_STAKES,
    &LOTUS_GUARDIAN,
    &PHYREXIAN_ALTAR,
    &PHYREXIAN_LENS,
    &PLANAR_PORTAL,
    &POWER_ARMOR,
    &RITH_S_ATTENDANT,
    &SEASHELL_CAMEO,
    &SPARRING_GOLEM,
    &TEK,
    &TIGEREYE_CAMEO,
    &TREVA_S_ATTENDANT,
    &TROLL_HORN_CAMEO,
    &TSABOS_WEB,
    &URZA_S_FILTER,
    &ANCIENT_SPRING,
    &ARCHAEOLOGICAL_DIG,
    &COASTAL_TOWER,
    &ELFHAME_PALACE,
    &GEOTHERMAL_CREVICE,
    &IRRIGATION_DITCH,
    &KELDON_NECROPOLIS,
    &SALT_MARSH,
    &SHIVAN_OASIS,
    &SULFUR_VENT,
    &TINDER_FARM,
    &URBORG_VOLCANO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_p02::ANGEL_OF_MERCY), // INV 2
    PrintingRecord::reprint(&catalog_mir::BLINDING_LIGHT), // INV 9
    PrintingRecord::reprint(&catalog_leg::HOLY_DAY),       // INV 20
    PrintingRecord::reprint(&catalog_exo::SHACKLES),       // INV 37
    PrintingRecord::reprint(&catalog_wth::DISRUPT),        // INV 51
    PrintingRecord::reprint(&catalog_lea::PHANTASMAL_TERRAIN), // INV 65
    PrintingRecord::reprint(&catalog_tmp::SHIMMERING_WINGS), // INV 72
    PrintingRecord::reprint(&catalog_rtr::TOWER_DRAKE),    // INV 82
    PrintingRecord::reprint(&catalog_rtr::CREMATE),        // INV 96
    PrintingRecord::reprint(&catalog_exo::CURSED_FLESH),   // INV 98
    PrintingRecord::reprint(&catalog_m13::RAVENOUS_RATS),  // INV 120
    PrintingRecord::reprint(&catalog_tmp::RECKLESS_SPITE), // INV 121
    PrintingRecord::reprint(&catalog_ice::SOUL_BURN),      // INV 124
    PrintingRecord::alternate(&catalog_ice::SOUL_BURN, 1), // INV 124s
    PrintingRecord::alternate(&catalog_ice::SOUL_BURN, 2), // INV 124★
    PrintingRecord::alternate(&URBORG_SKELETON, 1),        // INV 134
    PrintingRecord::alternate(&URBORG_SKELETON, 2),        // INV 134s
    PrintingRecord::reprint(&catalog_tmp::CROWN_OF_FLAMES), // INV 142
    PrintingRecord::reprint(&catalog_exo::MANIACAL_RAGE),  // INV 155
    PrintingRecord::reprint(&catalog_tmp::STUN),           // INV 172
    PrintingRecord::reprint(&catalog_usg::FERTILE_GROUND), // INV 188
    PrintingRecord::reprint(&catalog_tmp::HARROW),         // INV 189
    PrintingRecord::reprint(&catalog_mir::QUIRION_ELVES),  // INV 203
    PrintingRecord::reprint(&catalog_lea::TRANQUILITY),    // INV 217
    PrintingRecord::reprint(&catalog_gtc::FRENZIED_TILLING), // INV 247
    PrintingRecord::reprint(&catalog_rtr::HEROES_REUNION), // INV 250
    PrintingRecord::reprint(&catalog_tmp::LOBOTOMY),       // INV 255
    PrintingRecord::reprint(&catalog_vis::SIMOON),         // INV 272
    PrintingRecord::reprint(&catalog_lea::PLAINS),         // INV 331
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),    // INV 332
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),    // INV 333
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3),    // INV 334
    PrintingRecord::reprint(&catalog_lea::ISLAND),         // INV 335
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),    // INV 336
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),    // INV 337
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3),    // INV 338
    PrintingRecord::reprint(&catalog_lea::SWAMP),          // INV 339
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),     // INV 340
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),     // INV 341
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3),     // INV 342
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),       // INV 343
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),  // INV 344
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2),  // INV 345
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3),  // INV 346
    PrintingRecord::reprint(&catalog_lea::FOREST),         // INV 347
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),    // INV 348
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),    // INV 349
    PrintingRecord::alternate(&catalog_lea::FOREST, 3),    // INV 350
];
