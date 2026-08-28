//! Ice Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules, CardSet, CardType,
    DividedTotal, EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor,
    ObjectPredicateDef, PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef, SpellLifeCostDef,
    TargetChooserDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// ICE 1 — Adarkar Unicorn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ADARKAR_UNICORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ba7526f-dba8-4483-b925-946164fc0ae9"),
    "Adarkar Unicorn",
    crate::card::CardArt::new("0ba7526f-dba8-4483-b925-946164fc0ae9", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 2 — Arctic Foxes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_FOXES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98f99c3e-dddc-492f-aab6-1d899346a385"),
    "Arctic Foxes",
    crate::card::CardArt::new("98f99c3e-dddc-492f-aab6-1d899346a385", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 3 — Arenson's Aura
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARENSON_S_AURA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f94f3e87-1b39-49a8-ad0d-f18c854e298a"),
    "Arenson's Aura",
    crate::card::CardArt::new("f94f3e87-1b39-49a8-ad0d-f18c854e298a", "Nicola Leonard"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 4 — Armor of Faith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARMOR_OF_FAITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fccbbc47-99c6-4ba9-95c2-992d5d2a67b2"),
    "Armor of Faith",
    crate::card::CardArt::new("fccbbc47-99c6-4ba9-95c2-992d5d2a67b2", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 5 — Battle Cry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_CRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c558a8c4-035c-464e-9ff8-c188c1bb619e"),
    "Battle Cry",
    crate::card::CardArt::new("c558a8c4-035c-464e-9ff8-c188c1bb619e", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 6 — Black Scarab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_SCARAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bfd4ee1-05f9-45ae-a31d-1225b271dbe6"),
    "Black Scarab",
    crate::card::CardArt::new("5bfd4ee1-05f9-45ae-a31d-1225b271dbe6", "Kaja Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 7 — Blessed Wine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_WINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b9a92f9-9bbc-4887-9fbc-0f7212fd5e66"),
    "Blessed Wine",
    crate::card::CardArt::new("6b9a92f9-9bbc-4887-9fbc-0f7212fd5e66", "Kaja Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 8 — Blinking Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLINKING_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14fc0683-9cfa-4439-a533-8773e7747ec4"),
    "Blinking Spirit",
    crate::card::CardArt::new("14fc0683-9cfa-4439-a533-8773e7747ec4", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 9 — Blue Scarab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLUE_SCARAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b423bb5a-eaac-4c1d-981a-1c635001fc5a"),
    "Blue Scarab",
    crate::card::CardArt::new("b423bb5a-eaac-4c1d-981a-1c635001fc5a", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 10 — Call to Arms
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_TO_ARMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a92f0d4a-23d8-47d4-b910-d142e0eefd3d"),
    "Call to Arms",
    crate::card::CardArt::new("a92f0d4a-23d8-47d4-b910-d142e0eefd3d", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 11 — Caribou Range
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARIBOU_RANGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e5f8041-67fc-4e00-b119-d216e5cc5a3a"),
    "Caribou Range",
    crate::card::CardArt::new("1e5f8041-67fc-4e00-b119-d216e5cc5a3a", "Ruth Thompson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 12 — Circle of Protection: Black (reprint)

// ICE 13 — Circle of Protection: Blue (reprint)

// ICE 14 — Circle of Protection: Green (reprint)

// ICE 15 — Circle of Protection: Red (reprint)

// ICE 16 — Circle of Protection: White (reprint)

// ICE 17 — Cold Snap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COLD_SNAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81b87a58-b20c-4f38-afa3-59d398195740"),
    "Cold Snap",
    crate::card::CardArt::new("81b87a58-b20c-4f38-afa3-59d398195740", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 18 — Cooperation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COOPERATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21a815ed-c8b4-4414-8b27-ea612e2977e2"),
    "Cooperation",
    crate::card::CardArt::new("21a815ed-c8b4-4414-8b27-ea612e2977e2", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 19 — Death Ward (reprint)

// ICE 20 — Disenchant (reprint)

// ICE 21 — Drought
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROUGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97736696-3de3-416d-94cf-4fac792f23f0"),
    "Drought",
    crate::card::CardArt::new("97736696-3de3-416d-94cf-4fac792f23f0", "NéNé Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 22 — Elvish Healer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00bd8485-d63a-4077-a3d1-4d0f2f4d8035"),
    "Elvish Healer",
    crate::card::CardArt::new("00bd8485-d63a-4077-a3d1-4d0f2f4d8035", "Rick Emond"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 23 — Enduring Renewal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENDURING_RENEWAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be77edac-9a8b-4b7f-a859-27df76b10aa6"),
    "Enduring Renewal",
    crate::card::CardArt::new("be77edac-9a8b-4b7f-a859-27df76b10aa6", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 24 — Energy Storm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3955e358-4285-44e2-9e24-9804346a6e58"),
    "Energy Storm",
    crate::card::CardArt::new("3955e358-4285-44e2-9e24-9804346a6e58", "Sandra Everingham"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 25 — Formation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORMATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("78446ead-61b0-485f-a5a9-b3e72d8075a7"),
    "Formation",
    crate::card::CardArt::new("78446ead-61b0-485f-a5a9-b3e72d8075a7", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 26 — Fylgja
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FYLGJA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c6358a1-37f0-4b40-93d4-4f1652c38404"),
    "Fylgja",
    crate::card::CardArt::new(
        "3c6358a1-37f0-4b40-93d4-4f1652c38404",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 27 — General Jarkeld
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GENERAL_JARKELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a4f5a28-0bd2-4cc4-b67f-324e89193caa"),
    "General Jarkeld",
    crate::card::CardArt::new("6a4f5a28-0bd2-4cc4-b67f-324e89193caa", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 28 — Green Scarab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GREEN_SCARAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fbf9266-c97e-4666-b0fa-1802a69a62cc"),
    "Green Scarab",
    crate::card::CardArt::new("0fbf9266-c97e-4666-b0fa-1802a69a62cc", "Nicola Leonard"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 29 — Hallowed Ground
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HALLOWED_GROUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b35c0f4-5633-4ea9-9bda-daaf787aebdd"),
    "Hallowed Ground",
    crate::card::CardArt::new("4b35c0f4-5633-4ea9-9bda-daaf787aebdd", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 30 — Heal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e6b2704-685e-4c74-875a-25846175e5e4"),
    "Heal",
    crate::card::CardArt::new("9e6b2704-685e-4c74-875a-25846175e5e4", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 31 — Hipparion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HIPPARION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5969875a-f647-4daf-b76c-d1514d45c312"),
    "Hipparion",
    crate::card::CardArt::new("5969875a-f647-4daf-b76c-d1514d45c312", "Dameon Willich"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 32 — Justice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a6e0c8d-0fc1-4f52-8357-e550b0ac579a"),
    "Justice",
    crate::card::CardArt::new("9a6e0c8d-0fc1-4f52-8357-e550b0ac579a", "Ruth Thompson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 33 — Kelsinko Ranger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KELSINKO_RANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8402543e-5406-404f-95c4-800a1dce35f1"),
    "Kelsinko Ranger",
    crate::card::CardArt::new("8402543e-5406-404f-95c4-800a1dce35f1", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 34 — Kjeldoran Elite Guard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ELITE_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a73bc4b6-f7d0-494c-9e60-48279c11b7b6"),
    "Kjeldoran Elite Guard",
    crate::card::CardArt::new("a73bc4b6-f7d0-494c-9e60-48279c11b7b6", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 35 — Kjeldoran Guard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdf41f17-8f82-4a8c-adec-0f3804faff3b"),
    "Kjeldoran Guard",
    crate::card::CardArt::new("bdf41f17-8f82-4a8c-adec-0f3804faff3b", "Anthony S. Waters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 36 — Kjeldoran Knight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5b9db8f-93b5-44e3-9e2b-728c80dfbb37"),
    "Kjeldoran Knight",
    crate::card::CardArt::new("d5b9db8f-93b5-44e3-9e2b-728c80dfbb37", "Ron Spencer"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 37 — Kjeldoran Phalanx
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_PHALANX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6e91ba0-b229-4ab1-84f3-2a490dfa5051"),
    "Kjeldoran Phalanx",
    crate::card::CardArt::new(
        "b6e91ba0-b229-4ab1-84f3-2a490dfa5051",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 38 — Kjeldoran Royal Guard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ROYAL_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66343008-c38a-48a9-b767-fd2243103690"),
    "Kjeldoran Royal Guard",
    crate::card::CardArt::new("66343008-c38a-48a9-b767-fd2243103690", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 39 — Kjeldoran Skycaptain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_SKYCAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf0115e0-6192-48a9-9e58-f3ef77ef77c2"),
    "Kjeldoran Skycaptain",
    crate::card::CardArt::new("cf0115e0-6192-48a9-9e58-f3ef77ef77c2", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 40 — Kjeldoran Skyknight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_SKYKNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f794665a-8353-482a-b065-2a0777a8acda"),
    "Kjeldoran Skyknight",
    crate::card::CardArt::new("f794665a-8353-482a-b065-2a0777a8acda", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 41 — Kjeldoran Warrior
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce76f38f-566e-49ff-b197-510cfa1cb51c"),
    "Kjeldoran Warrior",
    crate::card::CardArt::new("ce76f38f-566e-49ff-b197-510cfa1cb51c", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 42 — Lightning Blow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_BLOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1a4ed99-f38c-4e0f-9ff2-2e1e9126e6ef"),
    "Lightning Blow",
    crate::card::CardArt::new("d1a4ed99-f38c-4e0f-9ff2-2e1e9126e6ef", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 43 — Lost Order of Jarkeld
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_ORDER_OF_JARKELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f8fe1e5-69d2-401f-97cb-3cc01064bad3"),
    "Lost Order of Jarkeld",
    crate::card::CardArt::new("0f8fe1e5-69d2-401f-97cb-3cc01064bad3", "Andi Rusu"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 44 — Mercenaries
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MERCENARIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b28762d-1ab7-460e-b433-27f5fa858959"),
    "Mercenaries",
    crate::card::CardArt::new("7b28762d-1ab7-460e-b433-27f5fa858959", "Cornelius Brudi"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 45 — Order of the Sacred Torch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_OF_THE_SACRED_TORCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccc5cb36-c43d-4c71-8019-9b683e160a0a"),
    "Order of the Sacred Torch",
    crate::card::CardArt::new("ccc5cb36-c43d-4c71-8019-9b683e160a0a", "Ruth Thompson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 46 — Order of the White Shield
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_OF_THE_WHITE_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("92e55b10-375f-4b4f-b676-3b9b8085fdd2"),
    "Order of the White Shield",
    crate::card::CardArt::new("92e55b10-375f-4b4f-b676-3b9b8085fdd2", "Ruth Thompson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 47 — Prismatic Ward
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_WARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f8b50fd-3d1d-4ea8-a3c7-98ca7a8a455e"),
    "Prismatic Ward",
    crate::card::CardArt::new("6f8b50fd-3d1d-4ea8-a3c7-98ca7a8a455e", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 48 — Rally
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RALLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1e9f80e-5d75-45b7-9c66-c0f30996f4dc"),
    "Rally",
    crate::card::CardArt::new("e1e9f80e-5d75-45b7-9c66-c0f30996f4dc", "Heather Hudson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 49 — Red Scarab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RED_SCARAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a734154-5944-42f4-a02e-c426a45847f3"),
    "Red Scarab",
    crate::card::CardArt::new("9a734154-5944-42f4-a02e-c426a45847f3", "Sandra Everingham"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 50 — Sacred Boon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_BOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d721569d-9cf2-4c3c-b11c-4c46c258a0d2"),
    "Sacred Boon",
    crate::card::CardArt::new("d721569d-9cf2-4c3c-b11c-4c46c258a0d2", "Mike Raabe"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 51 — Seraph
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERAPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab675291-3189-43f3-b11b-0724eca8b941"),
    "Seraph",
    crate::card::CardArt::new("ab675291-3189-43f3-b11b-0724eca8b941", "Christopher Rush"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 52 — Shield Bearer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_BEARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("318ff2da-d309-469c-8e2f-fa3c7517a15a"),
    "Shield Bearer",
    crate::card::CardArt::new("318ff2da-d309-469c-8e2f-fa3c7517a15a", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 53 — Snow Hound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("084437ba-26d4-4af6-ab00-dcb145dd2cd0"),
    "Snow Hound",
    crate::card::CardArt::new("084437ba-26d4-4af6-ab00-dcb145dd2cd0", "Pat Lewis"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 54 — Swords to Plowshares (reprint)

// ICE 55 — Warning
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WARNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cca5b4a7-df11-4635-a147-df12cd13a67c"),
    "Warning",
    crate::card::CardArt::new("cca5b4a7-df11-4635-a147-df12cd13a67c", "Pat Lewis"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 56 — White Scarab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHITE_SCARAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c57726b5-dfdd-4e47-bc52-ebf6eedbf3bd"),
    "White Scarab",
    crate::card::CardArt::new("c57726b5-dfdd-4e47-bc52-ebf6eedbf3bd", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 57 — Arnjlot's Ascent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARNJLOT_S_ASCENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2307fb16-8b77-45b5-8a02-51a13214791d"),
    "Arnjlot's Ascent",
    crate::card::CardArt::new("2307fb16-8b77-45b5-8a02-51a13214791d", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 58 — Balduvian Conjurer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_CONJURER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b616963-fac0-451c-8df4-2cacc9466b17"),
    "Balduvian Conjurer",
    crate::card::CardArt::new("5b616963-fac0-451c-8df4-2cacc9466b17", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 59 — Balduvian Shaman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74859723-8ddf-4ee6-a0a7-87192c84e8ad"),
    "Balduvian Shaman",
    crate::card::CardArt::new("74859723-8ddf-4ee6-a0a7-87192c84e8ad", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 60 — Binding Grasp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BINDING_GRASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b086186-5fbf-4ba7-af0d-ee3ad61d27bb"),
    "Binding Grasp",
    crate::card::CardArt::new("6b086186-5fbf-4ba7-af0d-ee3ad61d27bb", "Ruth Thompson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 61 — Brainstorm
pub(in crate::card::sets) static BRAINSTORM: CardRecord = CardRecord::new_with_legacy_id(
    2254,
    "Brainstorm",
    CardArt::new("8d42d7aa-7f53-4cfc-842a-086aab2448d1", "Christopher Rush"),
    CardSet::IceAge,
    // One mana, no card advantage, and the best blue card in the format:
    // what it buys is the top of the library, and a fetchland turns the two
    // cards put back into two cards nobody has to draw.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Draw three cards, then put two cards from your hand on top of your library in any \
             order.",
        abilities::brainstorm(),
    )),
);

// ICE 62 — Breath of Dreams
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BREATH_OF_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e40c9657-fab4-489d-8eb0-960ba2605add"),
    "Breath of Dreams",
    crate::card::CardArt::new("e40c9657-fab4-489d-8eb0-960ba2605add", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 63 — Clairvoyance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLAIRVOYANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46740353-e2ba-4d80-a97d-1368bc67bf30"),
    "Clairvoyance",
    crate::card::CardArt::new("46740353-e2ba-4d80-a97d-1368bc67bf30", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 64 — Counterspell (reprint)

// ICE 65 — Deflection
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFLECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1005a00a-6a0e-44cb-abea-37e2e53125e2"),
    "Deflection",
    crate::card::CardArt::new("1005a00a-6a0e-44cb-abea-37e2e53125e2", "Mike Raabe"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 66 — Dreams of the Dead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREAMS_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93372854-57e7-4db7-a1a6-376c9f49a514"),
    "Dreams of the Dead",
    crate::card::CardArt::new("93372854-57e7-4db7-a1a6-376c9f49a514", "Heather Hudson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 67 — Enervate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENERVATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4fdfc5b-c2ab-4c4d-b120-301e17f3d9c6"),
    "Enervate",
    crate::card::CardArt::new("c4fdfc5b-c2ab-4c4d-b120-301e17f3d9c6", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 68 — Errant Minion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERRANT_MINION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61648ddb-6efb-43d0-b2b1-418cc957854c"),
    "Errant Minion",
    crate::card::CardArt::new("61648ddb-6efb-43d0-b2b1-418cc957854c", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 69 — Essence Flare
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13ebb5dd-d7f1-4b06-8585-7004045be542"),
    "Essence Flare",
    crate::card::CardArt::new(
        "13ebb5dd-d7f1-4b06-8585-7004045be542",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 70 — Force Void
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORCE_VOID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("226555ba-22af-45f1-a3f4-d265f8685dd5"),
    "Force Void",
    crate::card::CardArt::new("226555ba-22af-45f1-a3f4-d265f8685dd5", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 71 — Glacial Wall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07b71bc1-d9a2-4e99-a8fa-cd696925328d"),
    "Glacial Wall",
    crate::card::CardArt::new("07b71bc1-d9a2-4e99-a8fa-cd696925328d", "Dameon Willich"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 72 — Hydroblast
pub(in crate::card::sets) static HYDROBLAST: CardRecord = CardRecord::new_with_legacy_id(
    264,
    "Hydroblast",
    CardArt::new("f62716f0-fde2-49ef-b8a4-c1b03f451194", "Kaja Foglio"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target spell if it's red.\n• Destroy target permanent if it's red.",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's red",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Red)),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's red",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Red,
                )),
                true,
            ),
        ],
    )),
);

// ICE 73 — Iceberg
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICEBERG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2f70e49-17fa-4033-bd45-63374f7f5ec5"),
    "Iceberg",
    crate::card::CardArt::new("a2f70e49-17fa-4033-bd45-63374f7f5ec5", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 74 — Icy Prison
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICY_PRISON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39a7e496-8d2e-49db-b298-475d9017537a"),
    "Icy Prison",
    crate::card::CardArt::new("39a7e496-8d2e-49db-b298-475d9017537a", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 75 — Illusionary Forces
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_FORCES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab02268e-01cf-4729-95ca-5773afd40b56"),
    "Illusionary Forces",
    crate::card::CardArt::new("ab02268e-01cf-4729-95ca-5773afd40b56", "Justin Hampton"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 76 — Illusionary Presence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_PRESENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa31efed-4a11-4f59-a623-bac45d20091d"),
    "Illusionary Presence",
    crate::card::CardArt::new("aa31efed-4a11-4f59-a623-bac45d20091d", "Kaja Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 77 — Illusionary Terrain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_TERRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("691f4a1b-4706-41aa-82da-ae920739f036"),
    "Illusionary Terrain",
    crate::card::CardArt::new("691f4a1b-4706-41aa-82da-ae920739f036", "Rob Alexander"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 78 — Illusionary Wall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6430e8e2-fee3-4744-820e-d6e16cb992bd"),
    "Illusionary Wall",
    crate::card::CardArt::new("6430e8e2-fee3-4744-820e-d6e16cb992bd", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 79 — Illusions of Grandeur
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONS_OF_GRANDEUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17eeeef2-2ced-42b8-a5e0-1095c9e13b02"),
    "Illusions of Grandeur",
    crate::card::CardArt::new("17eeeef2-2ced-42b8-a5e0-1095c9e13b02", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 80 — Infuse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("223287b6-224c-4e00-946c-e7ac5539bd45"),
    "Infuse",
    crate::card::CardArt::new("223287b6-224c-4e00-946c-e7ac5539bd45", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 81 — Krovikan Sorcerer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_SORCERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c5fc053-7b0b-4e76-bf87-ccdb1e8752ed"),
    "Krovikan Sorcerer",
    crate::card::CardArt::new("9c5fc053-7b0b-4e76-bf87-ccdb1e8752ed", "Pat Lewis"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 82 — Magus of the Unseen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGUS_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86da04e9-b94d-42af-add3-02baf772bd33"),
    "Magus of the Unseen",
    crate::card::CardArt::new("86da04e9-b94d-42af-add3-02baf772bd33", "Kaja Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 83 — Mesmeric Trance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MESMERIC_TRANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae3df593-e9d5-479d-9a9a-1c7262dd9c6c"),
    "Mesmeric Trance",
    crate::card::CardArt::new("ae3df593-e9d5-479d-9a9a-1c7262dd9c6c", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 84 — Mistfolk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f3f4d4e-ca4a-4fba-b9fd-cd1d9457cfa1"),
    "Mistfolk",
    crate::card::CardArt::new("4f3f4d4e-ca4a-4fba-b9fd-cd1d9457cfa1", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 85 — Musician
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MUSICIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f8d2247-a10e-413a-b497-2add3918f991"),
    "Musician",
    crate::card::CardArt::new("9f8d2247-a10e-413a-b497-2add3918f991", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 86 — Mystic Might
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_MIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e35d7f08-0687-41bd-8c53-31a49adabb11"),
    "Mystic Might",
    crate::card::CardArt::new("e35d7f08-0687-41bd-8c53-31a49adabb11", "Nicola Leonard"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 87 — Mystic Remora
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_REMORA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58e93dff-b774-4765-b7bd-d3957e42ff4a"),
    "Mystic Remora",
    crate::card::CardArt::new("58e93dff-b774-4765-b7bd-d3957e42ff4a", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 88 — Phantasmal Mount
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_MOUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75afdbe6-a3f9-49cf-b4ef-f370e518e960"),
    "Phantasmal Mount",
    crate::card::CardArt::new("75afdbe6-a3f9-49cf-b4ef-f370e518e960", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 89 — Polar Kraken
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static POLAR_KRAKEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aee01e9c-0445-4228-a73a-3e5744844ed3"),
    "Polar Kraken",
    crate::card::CardArt::new("aee01e9c-0445-4228-a73a-3e5744844ed3", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 90 — Portent
static PORTENT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

pub(in crate::card::sets) static PORTENT: CardRecord = CardRecord::new_with_legacy_id(
    2051,
    "Portent",
    CardArt::new("e040be83-3fb5-4da5-ba7a-4923b8854b74", "Liz Danforth"),
    CardSet::IceAge,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Look at the top three cards of target player's library, then put them back in any order. You may have that player shuffle.\nDraw a card at the beginning of the next turn's upkeep.",
        &PORTENT_TARGET,
        EffectDef::Sequence(&[
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next turn's upkeep, draw a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Any,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ))),
        ]),
    )),
);

// ICE 91 — Power Sink (reprint)

// ICE 92 — Ray of Command
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_OF_COMMAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("638abe5f-2a8a-42ca-bcdf-a52a3df66946"),
    "Ray of Command",
    crate::card::CardArt::new("638abe5f-2a8a-42ca-bcdf-a52a3df66946", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 93 — Ray of Erasure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_OF_ERASURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a09fc0b-7b9c-4283-8336-f2607f5ffaf5"),
    "Ray of Erasure",
    crate::card::CardArt::new("5a09fc0b-7b9c-4283-8336-f2607f5ffaf5", "Mike Raabe"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 94 — Reality Twist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REALITY_TWIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b7e955c-3de2-430c-93b9-0b39ccea5420"),
    "Reality Twist",
    crate::card::CardArt::new("1b7e955c-3de2-430c-93b9-0b39ccea5420", "James Ernest"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 95 — Sea Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2d93d05-98bc-4504-9045-dedb925895ae"),
    "Sea Spirit",
    crate::card::CardArt::new("f2d93d05-98bc-4504-9045-dedb925895ae", "Rob Alexander"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 96 — Shyft
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHYFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99a60c33-b641-42c4-870d-95d07bc975dc"),
    "Shyft",
    crate::card::CardArt::new("99a60c33-b641-42c4-870d-95d07bc975dc", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 97 — Sibilant Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SIBILANT_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47364ad2-5ce9-4b19-a9d2-f6a33188b882"),
    "Sibilant Spirit",
    crate::card::CardArt::new("47364ad2-5ce9-4b19-a9d2-f6a33188b882", "Ron Spencer"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 98 — Silver Erne
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_ERNE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("685076cc-098c-4f98-918c-0ad825eda10f"),
    "Silver Erne",
    crate::card::CardArt::new("685076cc-098c-4f98-918c-0ad825eda10f", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 99 — Sleight of Mind (reprint)

// ICE 100 — Snow Devil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_DEVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2be3a9a5-2ac5-4ea4-915d-8cff35c0e72f"),
    "Snow Devil",
    crate::card::CardArt::new("2be3a9a5-2ac5-4ea4-915d-8cff35c0e72f", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 101 — Snowfall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWFALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("788ed793-3993-4a63-b9f9-9ac3947c3108"),
    "Snowfall",
    crate::card::CardArt::new("788ed793-3993-4a63-b9f9-9ac3947c3108", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 102 — Soldevi Machinist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_MACHINIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f0999df-2f94-499e-b9af-fe377d515400"),
    "Soldevi Machinist",
    crate::card::CardArt::new("1f0999df-2f94-499e-b9af-fe377d515400", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 103 — Soul Barrier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_BARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ad7fac7-db4d-45b2-aba6-16f4fd1a586f"),
    "Soul Barrier",
    crate::card::CardArt::new("9ad7fac7-db4d-45b2-aba6-16f4fd1a586f", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 104 — Thunder Wall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4fc5d510-c4f7-4a09-bf86-83c3fa3f8928"),
    "Thunder Wall",
    crate::card::CardArt::new("4fc5d510-c4f7-4a09-bf86-83c3fa3f8928", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 105 — Updraft
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UPDRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1bd4e16-27fe-4c7b-ae25-78ed77d8e8e7"),
    "Updraft",
    crate::card::CardArt::new("d1bd4e16-27fe-4c7b-ae25-78ed77d8e8e7", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 106 — Wind Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WIND_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d882447-9594-4aab-b1a7-8bb275f250cf"),
    "Wind Spirit",
    crate::card::CardArt::new("4d882447-9594-4aab-b1a7-8bb275f250cf", "Kaja Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 107 — Winter's Chill
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_S_CHILL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a779aca7-ff2c-48d8-9484-6ad04b2c6bcb"),
    "Winter's Chill",
    crate::card::CardArt::new(
        "a779aca7-ff2c-48d8-9484-6ad04b2c6bcb",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 108 — Word of Undoing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WORD_OF_UNDOING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22b04476-5a5d-4843-a948-82db209c4218"),
    "Word of Undoing",
    crate::card::CardArt::new("22b04476-5a5d-4843-a948-82db209c4218", "Christopher Rush"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 109 — Wrath of Marit Lage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WRATH_OF_MARIT_LAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d512f5c-0327-4d49-8a26-672574a49102"),
    "Wrath of Marit Lage",
    crate::card::CardArt::new("1d512f5c-0327-4d49-8a26-672574a49102", "Mike Raabe"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 110 — Zur's Weirding
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZUR_S_WEIRDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1f8531f-19ca-48a2-baf2-c5dc6f18d79c"),
    "Zur's Weirding",
    crate::card::CardArt::new("e1f8531f-19ca-48a2-baf2-c5dc6f18d79c", "Liz Danforth"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 111 — Zuran Enchanter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZURAN_ENCHANTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("721edcef-f40a-4d43-9d80-26161dc425cb"),
    "Zuran Enchanter",
    crate::card::CardArt::new("721edcef-f40a-4d43-9d80-26161dc425cb", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 112 — Zuran Spellcaster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZURAN_SPELLCASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("152a72b1-a7b7-4e5c-8558-fab97465f549"),
    "Zuran Spellcaster",
    crate::card::CardArt::new(
        "152a72b1-a7b7-4e5c-8558-fab97465f549",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 113 — Abyssal Specter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABYSSAL_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc26f19c-bcf7-4bd8-af42-4757dbe47fb1"),
    "Abyssal Specter",
    crate::card::CardArt::new("fc26f19c-bcf7-4bd8-af42-4757dbe47fb1", "Ruth Thompson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 114 — Ashen Ghoul
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ASHEN_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6bb83301-5662-4628-b536-6a3ee0296f2e"),
    "Ashen Ghoul",
    crate::card::CardArt::new("6bb83301-5662-4628-b536-6a3ee0296f2e", "Ron Spencer"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 115 — Brine Shaman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRINE_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f445962c-44a1-4f3f-88d4-17048f8ca9dc"),
    "Brine Shaman",
    crate::card::CardArt::new("f445962c-44a1-4f3f-88d4-17048f8ca9dc", "Cornelius Brudi"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 116 — Burnt Offering
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BURNT_OFFERING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1dae52a2-3af7-4b97-9d2e-2448b7c413fb"),
    "Burnt Offering",
    crate::card::CardArt::new("1dae52a2-3af7-4b97-9d2e-2448b7c413fb", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 117 — Cloak of Confusion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLOAK_OF_CONFUSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dc45d103-0fca-4431-a5c0-869f0f9be93e"),
    "Cloak of Confusion",
    crate::card::CardArt::new(
        "dc45d103-0fca-4431-a5c0-869f0f9be93e",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 118 — Dance of the Dead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DANCE_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7c53ba4-9956-4cd6-85ca-2d6b61a5127c"),
    "Dance of the Dead",
    crate::card::CardArt::new("e7c53ba4-9956-4cd6-85ca-2d6b61a5127c", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 119 — Dark Banishing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_BANISHING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7dc2716-ed62-4797-ad2b-227eca5408d0"),
    "Dark Banishing",
    crate::card::CardArt::new("f7dc2716-ed62-4797-ad2b-227eca5408d0", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 120 — Dark Ritual (reprint)

// ICE 121 — Demonic Consultation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEMONIC_CONSULTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d727b9b-6114-414d-9172-16b6e1db41cc"),
    "Demonic Consultation",
    crate::card::CardArt::new("8d727b9b-6114-414d-9172-16b6e1db41cc", "Rob Alexander"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 122 — Dread Wight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_WIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65d332e2-4b2d-4131-84f7-862cb138c477"),
    "Dread Wight",
    crate::card::CardArt::new("65d332e2-4b2d-4131-84f7-862cb138c477", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 123 — Drift of the Dead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRIFT_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8b65656-9f8c-4179-81aa-4b15d8280baa"),
    "Drift of the Dead",
    crate::card::CardArt::new("d8b65656-9f8c-4179-81aa-4b15d8280baa", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 124 — Fear (reprint)

// ICE 125 — Flow of Maggots
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLOW_OF_MAGGOTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6880a4d3-5cbc-4a01-9190-3565617efcc9"),
    "Flow of Maggots",
    crate::card::CardArt::new("6880a4d3-5cbc-4a01-9190-3565617efcc9", "Ron Spencer"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 126 — Foul Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOUL_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bad3541-8e40-4a2f-ac9d-f7b61f3d75a1"),
    "Foul Familiar",
    crate::card::CardArt::new("8bad3541-8e40-4a2f-ac9d-f7b61f3d75a1", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 127 — Gangrenous Zombies
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GANGRENOUS_ZOMBIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08be4d83-99be-4360-90f1-104dee1c3c2f"),
    "Gangrenous Zombies",
    crate::card::CardArt::new("08be4d83-99be-4360-90f1-104dee1c3c2f", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 128 — Gaze of Pain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GAZE_OF_PAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48401643-ec4b-444a-8f9a-1a5ea471ff4a"),
    "Gaze of Pain",
    crate::card::CardArt::new("48401643-ec4b-444a-8f9a-1a5ea471ff4a", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 129 — Gravebind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEBIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4782fd4f-2474-4d0d-8301-e0b52af93746"),
    "Gravebind",
    crate::card::CardArt::new("4782fd4f-2474-4d0d-8301-e0b52af93746", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 130 — Hecatomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HECATOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f59620f-ff9e-44d8-9c4e-be9de1a919e8"),
    "Hecatomb",
    crate::card::CardArt::new("8f59620f-ff9e-44d8-9c4e-be9de1a919e8", "NéNé Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 131 — Hoar Shade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOAR_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72242dff-15ca-4da0-b3ae-9984d037b31f"),
    "Hoar Shade",
    crate::card::CardArt::new("72242dff-15ca-4da0-b3ae-9984d037b31f", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 132 — Howl from Beyond (reprint)

// ICE 133 — Hyalopterous Lemure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HYALOPTEROUS_LEMURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2c9e037-f4d5-46fd-b439-56bee6fb2ad3"),
    "Hyalopterous Lemure",
    crate::card::CardArt::new("d2c9e037-f4d5-46fd-b439-56bee6fb2ad3", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 134 — Icequake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICEQUAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14b4dd4d-c617-4603-8a87-761ec6fc6883"),
    "Icequake",
    crate::card::CardArt::new(
        "14b4dd4d-c617-4603-8a87-761ec6fc6883",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 135 — Infernal Darkness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_DARKNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3475eb3-909d-450b-9597-b241b259b425"),
    "Infernal Darkness",
    crate::card::CardArt::new("f3475eb3-909d-450b-9597-b241b259b425", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 136 — Infernal Denizen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_DENIZEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b63ac9a6-aaa5-4659-97d1-c5f6b0d5ccfe"),
    "Infernal Denizen",
    crate::card::CardArt::new("b63ac9a6-aaa5-4659-97d1-c5f6b0d5ccfe", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 137 — Kjeldoran Dead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3f7b614-6075-4b7c-acc7-ab63185b570b"),
    "Kjeldoran Dead",
    crate::card::CardArt::new("d3f7b614-6075-4b7c-acc7-ab63185b570b", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 138 — Knight of Stromgald
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_STROMGALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b87069b-ebaf-4705-b5da-446932af9b73"),
    "Knight of Stromgald",
    crate::card::CardArt::new("2b87069b-ebaf-4705-b5da-446932af9b73", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 139 — Krovikan Elementalist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_ELEMENTALIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbedca18-a074-4441-b0a9-7b14fdb07412"),
    "Krovikan Elementalist",
    crate::card::CardArt::new("bbedca18-a074-4441-b0a9-7b14fdb07412", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 140 — Krovikan Fetish
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_FETISH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("844e73e6-b201-4b2e-b46a-b719484fba0e"),
    "Krovikan Fetish",
    crate::card::CardArt::new("844e73e6-b201-4b2e-b46a-b719484fba0e", "Heather Hudson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 141 — Krovikan Vampire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_VAMPIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("717c5dda-8e38-4c76-b241-685198402284"),
    "Krovikan Vampire",
    crate::card::CardArt::new("717c5dda-8e38-4c76-b241-685198402284", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 142 — Legions of Lim-Dûl
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEGIONS_OF_LIM_DUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75b67eb2-b60e-46b4-9d48-11c284957bec"),
    "Legions of Lim-Dûl",
    crate::card::CardArt::new("75b67eb2-b60e-46b4-9d48-11c284957bec", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 143 — Leshrac's Rite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LESHRAC_S_RITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e0a6b4e-95b4-40f6-bb19-568dbd908a2b"),
    "Leshrac's Rite",
    crate::card::CardArt::new("4e0a6b4e-95b4-40f6-bb19-568dbd908a2b", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 144 — Leshrac's Sigil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LESHRAC_S_SIGIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad5ba7ee-d6df-4b62-a8a1-c81e6fca392a"),
    "Leshrac's Sigil",
    crate::card::CardArt::new("ad5ba7ee-d6df-4b62-a8a1-c81e6fca392a", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 145 — Lim-Dûl's Cohort
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_COHORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d0006f6-2f96-453d-9145-eaefa588efbc"),
    "Lim-Dûl's Cohort",
    crate::card::CardArt::new("3d0006f6-2f96-453d-9145-eaefa588efbc", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 146 — Lim-Dûl's Hex
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_HEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af976f42-3d56-4e32-8294-970a276a4bf3"),
    "Lim-Dûl's Hex",
    crate::card::CardArt::new("af976f42-3d56-4e32-8294-970a276a4bf3", "Liz Danforth"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 147 — Mind Ravel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_RAVEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61cf3ac5-985d-4b48-b230-d5ae4ab1ace8"),
    "Mind Ravel",
    crate::card::CardArt::new("61cf3ac5-985d-4b48-b230-d5ae4ab1ace8", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 148 — Mind Warp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_WARP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de150cd6-0bbc-47f7-a781-cd1aa10eabc6"),
    "Mind Warp",
    crate::card::CardArt::new("de150cd6-0bbc-47f7-a781-cd1aa10eabc6", "Liz Danforth"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 149 — Mind Whip
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_WHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f3ff5fb-4126-4a18-b540-2beaae382e59"),
    "Mind Whip",
    crate::card::CardArt::new("3f3ff5fb-4126-4a18-b540-2beaae382e59", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 150 — Minion of Leshrac
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_LESHRAC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61278908-a1b4-4b4c-84f5-498ca41fc6b6"),
    "Minion of Leshrac",
    crate::card::CardArt::new("61278908-a1b4-4b4c-84f5-498ca41fc6b6", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 151 — Minion of Tevesh Szat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_TEVESH_SZAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea9f3ab5-6a31-47db-b8bf-4c56a7ff19d1"),
    "Minion of Tevesh Szat",
    crate::card::CardArt::new("ea9f3ab5-6a31-47db-b8bf-4c56a7ff19d1", "Julie Baroh"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 152 — Mole Worms
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLE_WORMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4914f6fc-e3e7-426b-8688-12157c7df9e7"),
    "Mole Worms",
    crate::card::CardArt::new("4914f6fc-e3e7-426b-8688-12157c7df9e7", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 153 — Moor Fiend
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOOR_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57089dd4-e30d-498d-9341-43c104c6f3f9"),
    "Moor Fiend",
    crate::card::CardArt::new("57089dd4-e30d-498d-9341-43c104c6f3f9", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 154 — Necropotence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECROPOTENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("54d7a0c1-efb4-4a8d-ad92-a96d43835052"),
    "Necropotence",
    crate::card::CardArt::new("54d7a0c1-efb4-4a8d-ad92-a96d43835052", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 155 — Norritt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NORRITT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35abefe6-c39b-4fe5-b2e3-d213f0c4f447"),
    "Norritt",
    crate::card::CardArt::new("35abefe6-c39b-4fe5-b2e3-d213f0c4f447", "Mike Raabe"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 156 — Oath of Lim-Dûl
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_LIM_DUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f16df768-06de-43a0-b548-44fb0887490b"),
    "Oath of Lim-Dûl",
    crate::card::CardArt::new("f16df768-06de-43a0-b548-44fb0887490b", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 157 — Pestilence Rats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PESTILENCE_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bff7f6a6-0e90-4eb4-b76e-d98454975fb6"),
    "Pestilence Rats",
    crate::card::CardArt::new("bff7f6a6-0e90-4eb4-b76e-d98454975fb6", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 158 — Pox
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static POX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a914138c-a593-414c-bbcb-83d3c1bc4f6f"),
    "Pox",
    crate::card::CardArt::new("a914138c-a593-414c-bbcb-83d3c1bc4f6f", "Cornelius Brudi"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 159 — Seizures
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZURES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da369c86-7e17-43d8-b626-b6842e3d2d50"),
    "Seizures",
    crate::card::CardArt::new("da369c86-7e17-43d8-b626-b6842e3d2d50", "Julie Baroh"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 160 — Songs of the Damned
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SONGS_OF_THE_DAMNED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6cff3547-8c72-439a-91fe-ebe729dab748"),
    "Songs of the Damned",
    crate::card::CardArt::new("6cff3547-8c72-439a-91fe-ebe729dab748", "Pete Venters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 161 — Soul Burn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_BURN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb8e00d2-2381-4d45-bed8-c9bf738a9419"),
    "Soul Burn",
    crate::card::CardArt::new("eb8e00d2-2381-4d45-bed8-c9bf738a9419", "Rob Alexander"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 162 — Soul Kiss
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_KISS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42fbf6a5-86fe-41a3-891e-f72f11ad0aee"),
    "Soul Kiss",
    crate::card::CardArt::new("42fbf6a5-86fe-41a3-891e-f72f11ad0aee", "Nicola Leonard"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 163 — Spoils of Evil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPOILS_OF_EVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd368eb6-72f0-42d4-afa5-3daa7de949ff"),
    "Spoils of Evil",
    crate::card::CardArt::new("fd368eb6-72f0-42d4-afa5-3daa7de949ff", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 164 — Spoils of War
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPOILS_OF_WAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b38af8bd-d927-46d0-a1b1-fb437ea9ea66"),
    "Spoils of War",
    crate::card::CardArt::new("b38af8bd-d927-46d0-a1b1-fb437ea9ea66", "Pete Venters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 165 — Stench of Evil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STENCH_OF_EVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c7065a2-f819-4cbe-b453-a55e904f0461"),
    "Stench of Evil",
    crate::card::CardArt::new("4c7065a2-f819-4cbe-b453-a55e904f0461", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 166 — Stromgald Cabal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STROMGALD_CABAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ac6fa0c-753e-4fbc-8a70-0f956503cf4e"),
    "Stromgald Cabal",
    crate::card::CardArt::new("6ac6fa0c-753e-4fbc-8a70-0f956503cf4e", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 167 — Touch of Death
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCH_OF_DEATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a49c658f-e657-490b-af1f-e67e48d0046e"),
    "Touch of Death",
    crate::card::CardArt::new("a49c658f-e657-490b-af1f-e67e48d0046e", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 168 — Withering Wisps
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WITHERING_WISPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad1e6ae5-c972-42c0-ae78-f203873aeeb1"),
    "Withering Wisps",
    crate::card::CardArt::new("ad1e6ae5-c972-42c0-ae78-f203873aeeb1", "NéNé Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 169 — Aggression
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3f26060-0c24-496c-b8e2-4dac7ea6166b"),
    "Aggression",
    crate::card::CardArt::new("f3f26060-0c24-496c-b8e2-4dac7ea6166b", "Rick Emond"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 170 — Anarchy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANARCHY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28d941da-b5cb-4b7e-84f2-ece883f89af3"),
    "Anarchy",
    crate::card::CardArt::new("28d941da-b5cb-4b7e-84f2-ece883f89af3", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 171 — Avalanche
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AVALANCHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3a925e5-0d0a-42ec-b1c6-9793b8e11625"),
    "Avalanche",
    crate::card::CardArt::new("d3a925e5-0d0a-42ec-b1c6-9793b8e11625", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 172 — Balduvian Barbarians
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_BARBARIANS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efeabe8e-8107-4d19-8a43-362aa79cdd92"),
    "Balduvian Barbarians",
    crate::card::CardArt::new("efeabe8e-8107-4d19-8a43-362aa79cdd92", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 173 — Balduvian Hydra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3a3b37f-daa6-4502-bb12-c72afe3df035"),
    "Balduvian Hydra",
    crate::card::CardArt::new("c3a3b37f-daa6-4502-bb12-c72afe3df035", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 174 — Barbarian Guides
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARBARIAN_GUIDES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe65a045-dacb-4392-bcb6-843394ef98c9"),
    "Barbarian Guides",
    crate::card::CardArt::new("fe65a045-dacb-4392-bcb6-843394ef98c9", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 175 — Battle Frenzy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_FRENZY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a85ae675-56ca-4a00-83d2-ee035f33d6d1"),
    "Battle Frenzy",
    crate::card::CardArt::new("a85ae675-56ca-4a00-83d2-ee035f33d6d1", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 176 — Bone Shaman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a5e3d54-4dc4-482b-8ecc-bb819ba03d2c"),
    "Bone Shaman",
    crate::card::CardArt::new("0a5e3d54-4dc4-482b-8ecc-bb819ba03d2c", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 177 — Brand of Ill Omen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRAND_OF_ILL_OMEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ceeb7bbc-2d41-4709-95be-1ceb952ed1fb"),
    "Brand of Ill Omen",
    crate::card::CardArt::new("ceeb7bbc-2d41-4709-95be-1ceb952ed1fb", "Rob Alexander"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 178 — Chaos Lord
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_LORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee245922-b380-4b2e-a43f-ab1ba8078943"),
    "Chaos Lord",
    crate::card::CardArt::new("ee245922-b380-4b2e-a43f-ab1ba8078943", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 179 — Chaos Moon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_MOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aae0543f-7f8b-4327-b735-ac21244e9936"),
    "Chaos Moon",
    crate::card::CardArt::new("aae0543f-7f8b-4327-b735-ac21244e9936", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 180 — Conquer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONQUER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae610e66-7bcb-40ec-bed5-86dcfd098654"),
    "Conquer",
    crate::card::CardArt::new("ae610e66-7bcb-40ec-bed5-86dcfd098654", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 181 — Curse of Marit Lage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CURSE_OF_MARIT_LAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69b381c1-aa71-4d40-a320-70f58a440d51"),
    "Curse of Marit Lage",
    crate::card::CardArt::new("69b381c1-aa71-4d40-a320-70f58a440d51", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 182 — Dwarven Armory
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_ARMORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d14a430-6e08-40cf-970a-cae84bba6ef7"),
    "Dwarven Armory",
    crate::card::CardArt::new("7d14a430-6e08-40cf-970a-cae84bba6ef7", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 183 — Errantry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERRANTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8346e741-61f8-4283-be51-f5f80e9595a5"),
    "Errantry",
    crate::card::CardArt::new("8346e741-61f8-4283-be51-f5f80e9595a5", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 184 — Flame Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("add2b82a-9aa5-4d5c-a1c2-e313541f12c8"),
    "Flame Spirit",
    crate::card::CardArt::new("add2b82a-9aa5-4d5c-a1c2-e313541f12c8", "Justin Hampton"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 185 — Flare
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5350236-7bd2-462d-9768-50087626c764"),
    "Flare",
    crate::card::CardArt::new("d5350236-7bd2-462d-9768-50087626c764", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 186 — Game of Chaos
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GAME_OF_CHAOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08265332-2c0e-4c42-8c51-83ac20462eed"),
    "Game of Chaos",
    crate::card::CardArt::new("08265332-2c0e-4c42-8c51-83ac20462eed", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 187 — Glacial Crevasses
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_CREVASSES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2726b192-f239-470b-8ad6-69887405e7f9"),
    "Glacial Crevasses",
    crate::card::CardArt::new("2726b192-f239-470b-8ad6-69887405e7f9", "Mike Raabe"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 188 — Goblin Mutant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MUTANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6db54f95-6652-45a3-b960-c2fc118beca1"),
    "Goblin Mutant",
    crate::card::CardArt::new("6db54f95-6652-45a3-b960-c2fc118beca1", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 189 — Goblin Sappers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SAPPERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de839540-a7b9-4f91-91df-3fd4f5c0bc4e"),
    "Goblin Sappers",
    crate::card::CardArt::new("de839540-a7b9-4f91-91df-3fd4f5c0bc4e", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 190 — Goblin Ski Patrol
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SKI_PATROL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fde1c8b5-1e01-4920-8d02-bf80d5b238c5"),
    "Goblin Ski Patrol",
    crate::card::CardArt::new("fde1c8b5-1e01-4920-8d02-bf80d5b238c5", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 191 — Goblin Snowman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SNOWMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bbb260a-6763-4d1c-a009-4e34cd572519"),
    "Goblin Snowman",
    crate::card::CardArt::new("5bbb260a-6763-4d1c-a009-4e34cd572519", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 192 — Grizzled Wolverine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRIZZLED_WOLVERINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95bb17b9-55c4-4cc1-83f6-75490b9a97d0"),
    "Grizzled Wolverine",
    crate::card::CardArt::new("95bb17b9-55c4-4cc1-83f6-75490b9a97d0", "Cornelius Brudi"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 193 — Imposing Visage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMPOSING_VISAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cca42b74-9b42-482b-b12a-79cafdcd087e"),
    "Imposing Visage",
    crate::card::CardArt::new("cca42b74-9b42-482b-b12a-79cafdcd087e", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 194 — Incinerate
pub(in crate::card::sets) static INCINERATE: CardRecord = CardRecord::new_with_legacy_id(
    265,
    "Incinerate",
    CardArt::new("9c3f00af-010d-4485-b8b7-47400d99c496", "Mark Poole"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Incinerate deals 3 damage to any target. A creature dealt damage this way can't be regenerated this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamageAndApply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
                applied: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 195 — Jokulhaups
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JOKULHAUPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bf0d325-5928-4593-8faa-64ffa414cb48"),
    "Jokulhaups",
    crate::card::CardArt::new("3bf0d325-5928-4593-8faa-64ffa414cb48", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 196 — Karplusan Giant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KARPLUSAN_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c524ac2a-294c-4b19-b00b-999e370a3b95"),
    "Karplusan Giant",
    crate::card::CardArt::new("c524ac2a-294c-4b19-b00b-999e370a3b95", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 197 — Karplusan Yeti
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KARPLUSAN_YETI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7dd9b214-d9fe-4c2e-b45b-7145ad98c408"),
    "Karplusan Yeti",
    crate::card::CardArt::new("7dd9b214-d9fe-4c2e-b45b-7145ad98c408", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 198 — Lava Burst
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79dc0e20-5790-4927-8432-cf0e9b7381d4"),
    "Lava Burst",
    crate::card::CardArt::new("79dc0e20-5790-4927-8432-cf0e9b7381d4", "Tom Wänerstrand"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 199 — Márton Stromgald
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARTON_STROMGALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7880e815-53e7-43e0-befd-e368f00a75d8"),
    "Márton Stromgald",
    crate::card::CardArt::new("7880e815-53e7-43e0-befd-e368f00a75d8", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 200 — Melee
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MELEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b13a064d-bff4-4a48-a158-1b61951b0ac3"),
    "Melee",
    crate::card::CardArt::new("b13a064d-bff4-4a48-a158-1b61951b0ac3", "Dameon Willich"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 201 — Melting
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MELTING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d90065e-2c7e-44e5-9f59-015d468214bf"),
    "Melting",
    crate::card::CardArt::new("8d90065e-2c7e-44e5-9f59-015d468214bf", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 202 — Meteor Shower
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_SHOWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50b4851e-677b-468e-9baa-e47a3b4b8339"),
    "Meteor Shower",
    crate::card::CardArt::new("50b4851e-677b-468e-9baa-e47a3b4b8339", "Rick Emond"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 203 — Mountain Goat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNTAIN_GOAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccf70276-a40c-4d25-b584-4c8a07a00602"),
    "Mountain Goat",
    crate::card::CardArt::new("ccf70276-a40c-4d25-b584-4c8a07a00602", "Cornelius Brudi"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 204 — Mudslide
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MUDSLIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65acce56-8674-471e-9d5e-91b7e3f672c1"),
    "Mudslide",
    crate::card::CardArt::new("65acce56-8674-471e-9d5e-91b7e3f672c1", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 205 — Orcish Cannoneers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_CANNONEERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4309a2f-27f5-4652-b0b4-6a6119436f75"),
    "Orcish Cannoneers",
    crate::card::CardArt::new("a4309a2f-27f5-4652-b0b4-6a6119436f75", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 206 — Orcish Conscripts
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_CONSCRIPTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e71394f8-3038-4cad-adea-a704f004777f"),
    "Orcish Conscripts",
    crate::card::CardArt::new("e71394f8-3038-4cad-adea-a704f004777f", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 207 — Orcish Farmer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_FARMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efa5beef-d609-4809-a813-621b0b4cff7f"),
    "Orcish Farmer",
    crate::card::CardArt::new("efa5beef-d609-4809-a813-621b0b4cff7f", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 208 — Orcish Healer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_HEALER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ff511f3-416e-4919-acd6-fd8183bf5c60"),
    "Orcish Healer",
    crate::card::CardArt::new("7ff511f3-416e-4919-acd6-fd8183bf5c60", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 209 — Orcish Librarian
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_LIBRARIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ed908d6-6d06-4ccb-9577-37ef2d01c1a5"),
    "Orcish Librarian",
    crate::card::CardArt::new("8ed908d6-6d06-4ccb-9577-37ef2d01c1a5", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 210 — Orcish Lumberjack
/// "Sacrifice a Forest" reads the land type rather than the card name, so a
/// dual land with the type counts and a Forest somebody enchanted still
/// does. Which one is spent is chosen as the ability is activated.
static LUMBERJACK_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificePermanent {
        object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
        controller: PlayerRelation::You,
    },
];

static LUMBERJACK_COLORS: [ManaColor; 2] = [ManaColor::Red, ManaColor::Green];

pub(in crate::card::sets) static ORCISH_LUMBERJACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21ef13e3-658c-43a3-a290-4c5dde8e8b55"),
    "Orcish Lumberjack",
    CardArt::new("21ef13e3-658c-43a3-a290-4c5dde8e8b55", "Dan Frazier"),
    CardSet::IceAge,
    // One mana for a 1/1 that turns a land into three mana of either colour:
    // the land is gone and the body is nothing, and the deck playing it only
    // needs the turn it buys.
    CardRules::new_creature(mana_cost!("{R}"), &["Orc"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}, Sacrifice a Forest: Add three mana in any combination of {R} and/or {G}.",
            &LUMBERJACK_COST,
            EffectDef::AddMana(AddManaEffectDef::combination(&LUMBERJACK_COLORS, 3)),
        ),
    ),
);

// ICE 211 — Orcish Squatters
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_SQUATTERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f3ee7bd5-612b-4916-a914-1294805b8f64"),
    "Orcish Squatters",
    crate::card::CardArt::new(
        "f3ee7bd5-612b-4916-a914-1294805b8f64",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 212 — Panic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PANIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9ab85ac-311c-4e36-943a-817e43a3c8a8"),
    "Panic",
    crate::card::CardArt::new("a9ab85ac-311c-4e36-943a-817e43a3c8a8", "Mike Kimble"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 213 — Pyroblast
pub(in crate::card::sets) static PYROBLAST: CardRecord = CardRecord::new_with_legacy_id(
    266,
    "Pyroblast",
    CardArt::new("c342cac5-08ae-4428-9c2c-f6c5904e54d2", "Kaja Foglio"),
    CardSet::IceAge,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Counter target spell if it's blue.\n• Destroy target permanent if it's blue.",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's blue",
                &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Color(ManaColor::Blue)),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's blue",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Blue,
                )),
                true,
            ),
        ],
    )),
);

// ICE 214 — Pyroclasm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYROCLASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88040748-ad76-4b9a-bd4e-87e5980e9816"),
    "Pyroclasm",
    crate::card::CardArt::new("88040748-ad76-4b9a-bd4e-87e5980e9816", "Pat Lewis"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 215 — Sabretooth Tiger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SABRETOOTH_TIGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6914c5a8-2114-41c5-a471-ca97524d622f"),
    "Sabretooth Tiger",
    crate::card::CardArt::new("6914c5a8-2114-41c5-a471-ca97524d622f", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 216 — Shatter (reprint)

// ICE 217 — Stone Rain (reprint)

// ICE 218 — Stone Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("789dfae7-fe23-4e2e-9f5f-304535d22a78"),
    "Stone Spirit",
    crate::card::CardArt::new("789dfae7-fe23-4e2e-9f5f-304535d22a78", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 219 — Stonehands
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STONEHANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d23fa1af-78e5-4d23-bbf6-cd62bc54b4e9"),
    "Stonehands",
    crate::card::CardArt::new("d23fa1af-78e5-4d23-bbf6-cd62bc54b4e9", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 220 — Tor Giant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOR_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ef8f279-1a10-4685-99d6-bc971a7f922b"),
    "Tor Giant",
    crate::card::CardArt::new("7ef8f279-1a10-4685-99d6-bc971a7f922b", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 221 — Total War
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOTAL_WAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6107388b-ec1e-401e-a407-a821c908ed8d"),
    "Total War",
    crate::card::CardArt::new("6107388b-ec1e-401e-a407-a821c908ed8d", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 222 — Vertigo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VERTIGO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3067e7af-7bbd-48c1-9f1d-df2a91a0ec54"),
    "Vertigo",
    crate::card::CardArt::new("3067e7af-7bbd-48c1-9f1d-df2a91a0ec54", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 223 — Wall of Lava
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_LAVA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b99d6d11-b3f7-4d73-967c-3049af82a9d8"),
    "Wall of Lava",
    crate::card::CardArt::new("b99d6d11-b3f7-4d73-967c-3049af82a9d8", "Pete Venters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 224 — Word of Blasting
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WORD_OF_BLASTING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46b383c8-d604-4131-a869-9e9d13e30b94"),
    "Word of Blasting",
    crate::card::CardArt::new("46b383c8-d604-4131-a869-9e9d13e30b94", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 225 — Aurochs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AUROCHS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e973a84-7f7d-4524-9f2f-ec9a014d52ee"),
    "Aurochs",
    crate::card::CardArt::new("7e973a84-7f7d-4524-9f2f-ec9a014d52ee", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 226 — Balduvian Bears
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_BEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef5297cb-e763-4871-9cd3-0e2dbcc52095"),
    "Balduvian Bears",
    crate::card::CardArt::new("ef5297cb-e763-4871-9cd3-0e2dbcc52095", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 227 — Blizzard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLIZZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c369e4f9-0f2b-446c-9e2d-d3eefab0586d"),
    "Blizzard",
    crate::card::CardArt::new("c369e4f9-0f2b-446c-9e2d-d3eefab0586d", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 228 — Brown Ouphe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BROWN_OUPHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e26ce35b-ba65-451d-a5ed-e1db6f1d0c6f"),
    "Brown Ouphe",
    crate::card::CardArt::new("e26ce35b-ba65-451d-a5ed-e1db6f1d0c6f", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 229 — Chub Toad
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHUB_TOAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6ebcc1d-0c5c-4bc2-ade7-41944f69162e"),
    "Chub Toad",
    crate::card::CardArt::new("b6ebcc1d-0c5c-4bc2-ade7-41944f69162e", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 230 — Dire Wolves
pub(in crate::card::sets) static DIRE_WOLVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a602c93d-e00f-4b4f-a7ff-95316b7e7641"),
    "Dire Wolves",
    crate::card::CardArt::new("a602c93d-e00f-4b4f-a7ff-95316b7e7641", "Ron Spencer"),
    crate::card::CardSet::IceAge,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wolf"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature has banding as long as you control a Plains.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::controls_basic_land_type(
                    PlayerRelation::You,
                    BasicLandType::Plains,
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::banding()),
                },
            },
        ),
    ),
);

// ICE 231 — Earthlore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHLORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("319d252e-7c43-47d6-8873-f69b0e063256"),
    "Earthlore",
    crate::card::CardArt::new("319d252e-7c43-47d6-8873-f69b0e063256", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 232 — Elder Druid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELDER_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("210f6fab-62f0-42ab-bd01-00d647bd25e7"),
    "Elder Druid",
    crate::card::CardArt::new(
        "210f6fab-62f0-42ab-bd01-00d647bd25e7",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 233 — Essence Filter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FILTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b610103-dafd-4248-9d79-ce57f84b9e03"),
    "Essence Filter",
    crate::card::CardArt::new("9b610103-dafd-4248-9d79-ce57f84b9e03", "Rick Emond"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 234 — Fanatical Fever
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FANATICAL_FEVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2abba7f1-5d07-4137-88a2-5967396a3e42"),
    "Fanatical Fever",
    crate::card::CardArt::new("2abba7f1-5d07-4137-88a2-5967396a3e42", "Julie Baroh"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 235 — Folk of the Pines
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOLK_OF_THE_PINES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c13311d-db83-483f-ba2b-4f54ceb8b026"),
    "Folk of the Pines",
    crate::card::CardArt::new(
        "0c13311d-db83-483f-ba2b-4f54ceb8b026",
        "NéNé Thomas & Catherine Buck",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 236 — Forbidden Lore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_LORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fc225cf-4fe2-4a5b-828e-ffcb99e404e8"),
    "Forbidden Lore",
    crate::card::CardArt::new("5fc225cf-4fe2-4a5b-828e-ffcb99e404e8", "Christopher Rush"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 237 — Forgotten Lore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_LORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb01dd39-a957-4c1a-86cf-f31a699a154a"),
    "Forgotten Lore",
    crate::card::CardArt::new("fb01dd39-a957-4c1a-86cf-f31a699a154a", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 238 — Foxfire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOXFIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88db9685-6a2f-4548-b6c4-669918d653b4"),
    "Foxfire",
    crate::card::CardArt::new(
        "88db9685-6a2f-4548-b6c4-669918d653b4",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 239 — Freyalise Supplicant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_SUPPLICANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b1e718a-882a-4bdc-9d62-4dda88da0ba0"),
    "Freyalise Supplicant",
    crate::card::CardArt::new(
        "5b1e718a-882a-4bdc-9d62-4dda88da0ba0",
        "Liz Danforth & Douglas Shuler",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 240 — Freyalise's Charm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_S_CHARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e147ac1-d221-49c7-966e-5e665ddeab6b"),
    "Freyalise's Charm",
    crate::card::CardArt::new(
        "3e147ac1-d221-49c7-966e-5e665ddeab6b",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 241 — Freyalise's Winds
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_S_WINDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b11cd2e0-9419-4267-807e-5b73915c748a"),
    "Freyalise's Winds",
    crate::card::CardArt::new("b11cd2e0-9419-4267-807e-5b73915c748a", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 242 — Fyndhorn Brownie
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_BROWNIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06204e82-9dfd-4334-a23a-f8240fc37772"),
    "Fyndhorn Brownie",
    crate::card::CardArt::new("06204e82-9dfd-4334-a23a-f8240fc37772", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 243 — Fyndhorn Elder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fca8aa11-f7cb-4f88-a041-30098579f1d2"),
    "Fyndhorn Elder",
    crate::card::CardArt::new("fca8aa11-f7cb-4f88-a041-30098579f1d2", "Christopher Rush"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 244 — Fyndhorn Elves
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_ELVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ba95ffa-990a-4013-98b7-5d8c0b34e9c4"),
    "Fyndhorn Elves",
    crate::card::CardArt::new("3ba95ffa-990a-4013-98b7-5d8c0b34e9c4", "Justin Hampton"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 245 — Fyndhorn Pollen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_POLLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3efbe59d-bebc-40b1-85ac-2e4c1ff3731e"),
    "Fyndhorn Pollen",
    crate::card::CardArt::new("3efbe59d-bebc-40b1-85ac-2e4c1ff3731e", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 246 — Giant Growth (reprint)

// ICE 247 — Gorilla Pack
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_PACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("046f6b76-5f17-4728-aa34-72b7eff1d4c9"),
    "Gorilla Pack",
    crate::card::CardArt::new("046f6b76-5f17-4728-aa34-72b7eff1d4c9", "Anthony S. Waters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 248 — Hot Springs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOT_SPRINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d4fe072-81a7-424e-8d21-aaca010d5b1d"),
    "Hot Springs",
    crate::card::CardArt::new("1d4fe072-81a7-424e-8d21-aaca010d5b1d", "Nicola Leonard"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 249 — Hurricane (reprint)

// ICE 250 — Johtull Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JOHTULL_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64a22e88-f7b1-48c8-a199-e57edcd50654"),
    "Johtull Wurm",
    crate::card::CardArt::new("64a22e88-f7b1-48c8-a199-e57edcd50654", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 251 — Juniper Order Druid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JUNIPER_ORDER_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb211704-ff8e-498b-b7bb-f8384f198ffd"),
    "Juniper Order Druid",
    crate::card::CardArt::new("cb211704-ff8e-498b-b7bb-f8384f198ffd", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 252 — Lhurgoyf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LHURGOYF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fee6d385-d44b-4f1a-beb1-13aeebde063e"),
    "Lhurgoyf",
    crate::card::CardArt::new("fee6d385-d44b-4f1a-beb1-13aeebde063e", "Pete Venters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 253 — Lure (reprint)

// ICE 254 — Maddening Wind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MADDENING_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5277656c-70f5-4660-bd58-7d9261d53fb5"),
    "Maddening Wind",
    crate::card::CardArt::new("5277656c-70f5-4660-bd58-7d9261d53fb5", "Dameon Willich"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 255 — Nature's Lore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_LORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("668d2969-b6b7-4507-bdd4-20bbaa68035a"),
    "Nature's Lore",
    crate::card::CardArt::new("668d2969-b6b7-4507-bdd4-20bbaa68035a", "Rick Emond"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 256 — Pale Bears
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PALE_BEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f19c2a3-6403-4a78-bf45-6e339578d673"),
    "Pale Bears",
    crate::card::CardArt::new("7f19c2a3-6403-4a78-bf45-6e339578d673", "Anthony S. Waters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 257 — Pygmy Allosaurus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_ALLOSAURUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88a68767-9822-4f15-895e-32164e2159be"),
    "Pygmy Allosaurus",
    crate::card::CardArt::new("88a68767-9822-4f15-895e-32164e2159be", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 258 — Pyknite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYKNITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ffc64e4-ae3c-49f9-8ed6-518dd497bfe6"),
    "Pyknite",
    crate::card::CardArt::new(
        "6ffc64e4-ae3c-49f9-8ed6-518dd497bfe6",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 259 — Regeneration (reprint)

// ICE 260 — Rime Dryad
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIME_DRYAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a93e6ce-1295-41f8-b454-2dfe321481a6"),
    "Rime Dryad",
    crate::card::CardArt::new("7a93e6ce-1295-41f8-b454-2dfe321481a6", "Heather Hudson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 261 — Ritual of Subdual
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RITUAL_OF_SUBDUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c5c01e7-8116-45fc-afc3-d52a31a635cb"),
    "Ritual of Subdual",
    crate::card::CardArt::new("5c5c01e7-8116-45fc-afc3-d52a31a635cb", "Justin Hampton"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 262 — Scaled Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCALED_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("499cd7fa-c86c-4a5f-b36d-8160e8a6af1f"),
    "Scaled Wurm",
    crate::card::CardArt::new("499cd7fa-c86c-4a5f-b36d-8160e8a6af1f", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 263 — Shambling Strider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHAMBLING_STRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8886ba2d-b25a-4b74-9299-911c509ae864"),
    "Shambling Strider",
    crate::card::CardArt::new("8886ba2d-b25a-4b74-9299-911c509ae864", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 264 — Snowblind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWBLIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f62c376-487a-42bc-bd85-ab8b0480f7dc"),
    "Snowblind",
    crate::card::CardArt::new("5f62c376-487a-42bc-bd85-ab8b0480f7dc", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 265 — Stampede
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc8265a1-4621-4d25-8f7f-f0179951a694"),
    "Stampede",
    crate::card::CardArt::new("bc8265a1-4621-4d25-8f7f-f0179951a694", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 266 — Stunted Growth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STUNTED_GROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c9b7393-eb35-4c99-bbf5-bcf924aa8ff3"),
    "Stunted Growth",
    crate::card::CardArt::new("4c9b7393-eb35-4c99-bbf5-bcf924aa8ff3", "NéNé Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 267 — Tarpan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TARPAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b1420ec5-367c-4514-86c5-3993bf339e37"),
    "Tarpan",
    crate::card::CardArt::new(
        "b1420ec5-367c-4514-86c5-3993bf339e37",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 268 — Thermokarst
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THERMOKARST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00ae906b-2c4d-48e9-9f2d-217777e22292"),
    "Thermokarst",
    crate::card::CardArt::new("00ae906b-2c4d-48e9-9f2d-217777e22292", "Ken Meyer, Jr."),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 269 — Thoughtleech
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTLEECH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d8fe7f9d-644f-48d0-93fa-d9a536f1f755"),
    "Thoughtleech",
    crate::card::CardArt::new("d8fe7f9d-644f-48d0-93fa-d9a536f1f755", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 270 — Tinder Wall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TINDER_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a7c6489-21e9-4b86-a54a-b1e2f1fce318"),
    "Tinder Wall",
    crate::card::CardArt::new("2a7c6489-21e9-4b86-a54a-b1e2f1fce318", "Rick Emond"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 271 — Touch of Vitae
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCH_OF_VITAE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48d2cd18-a24d-40e0-a654-777d9e623ae2"),
    "Touch of Vitae",
    crate::card::CardArt::new("48d2cd18-a24d-40e0-a654-777d9e623ae2", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 272 — Trailblazer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRAILBLAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9194c69d-c849-4c4a-976c-d1382bd5cf32"),
    "Trailblazer",
    crate::card::CardArt::new("9194c69d-c849-4c4a-976c-d1382bd5cf32", "Julie Baroh"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 273 — Venomous Breath
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_BREATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8eeb9e02-1d26-4959-a878-2ef8db2358bc"),
    "Venomous Breath",
    crate::card::CardArt::new("8eeb9e02-1d26-4959-a878-2ef8db2358bc", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 274 — Wall of Pine Needles
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_PINE_NEEDLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d879923-55fc-46ab-9306-5e1f10441c89"),
    "Wall of Pine Needles",
    crate::card::CardArt::new("5d879923-55fc-46ab-9306-5e1f10441c89", "Brian Snõddy"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 275 — Whiteout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHITEOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8645e4f-eaa8-4420-a6a3-eb53c311fab1"),
    "Whiteout",
    crate::card::CardArt::new("a8645e4f-eaa8-4420-a6a3-eb53c311fab1", "NéNé Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 276 — Wiitigo
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WIITIGO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ee86bf2-6c54-4c6e-8394-eb39f98d5a85"),
    "Wiitigo",
    crate::card::CardArt::new("9ee86bf2-6c54-4c6e-8394-eb39f98d5a85", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 277 — Wild Growth (reprint)

// ICE 278 — Woolly Mammoths
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WOOLLY_MAMMOTHS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eaca1216-99c8-4ad5-a51a-3c4ff3b82097"),
    "Woolly Mammoths",
    crate::card::CardArt::new("eaca1216-99c8-4ad5-a51a-3c4ff3b82097", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 279 — Woolly Spider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WOOLLY_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e10520b2-b5a7-4328-84c8-20443b6f588a"),
    "Woolly Spider",
    crate::card::CardArt::new("e10520b2-b5a7-4328-84c8-20443b6f588a", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 280 — Yavimaya Gnats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_GNATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d8b7020-ca8f-4867-bc51-13d824daf154"),
    "Yavimaya Gnats",
    crate::card::CardArt::new("9d8b7020-ca8f-4867-bc51-13d824daf154", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 281 — Altar of Bone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALTAR_OF_BONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75d5b014-8675-4d91-a539-ac5c31d44b35"),
    "Altar of Bone",
    crate::card::CardArt::new("75d5b014-8675-4d91-a539-ac5c31d44b35", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 282 — Centaur Archer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_ARCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e275c295-72da-4a86-82c6-cfd75b38b19c"),
    "Centaur Archer",
    crate::card::CardArt::new("e275c295-72da-4a86-82c6-cfd75b38b19c", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 283 — Chromatic Armor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMATIC_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2657e85b-8f77-41fa-9df2-233443efef43"),
    "Chromatic Armor",
    crate::card::CardArt::new("2657e85b-8f77-41fa-9df2-233443efef43", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 284 — Diabolic Vision
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DIABOLIC_VISION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ea01324-1cfb-498c-8299-f690373864bd"),
    "Diabolic Vision",
    crate::card::CardArt::new("1ea01324-1cfb-498c-8299-f690373864bd", "Anthony S. Waters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 285 — Earthlink
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHLINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a83cb1c4-7c5b-4a5e-b15e-138d644f5cdb"),
    "Earthlink",
    crate::card::CardArt::new(
        "a83cb1c4-7c5b-4a5e-b15e-138d644f5cdb",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 286 — Elemental Augury
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELEMENTAL_AUGURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62bbff2a-5109-400a-961b-eacffb9aed67"),
    "Elemental Augury",
    crate::card::CardArt::new("62bbff2a-5109-400a-961b-eacffb9aed67", "Anthony S. Waters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 287 — Essence Vortex
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_VORTEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe07e496-5070-4116-a91a-a3bbe19c12af"),
    "Essence Vortex",
    crate::card::CardArt::new(
        "fe07e496-5070-4116-a91a-a3bbe19c12af",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 288 — Fiery Justice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIERY_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8965ce61-0522-4f77-a82d-89441d1ba867"),
    "Fiery Justice",
    crate::card::CardArt::new("8965ce61-0522-4f77-a82d-89441d1ba867", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 289 — Fire Covenant
/// "Any number of target creatures" is however many shares X splits into,
/// and X is the life its caster was willing to spend rather than anything in
/// the mana cost -- three mana kills a board if you have the life for it.
static FIRE_COVENANT_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
    predicate: AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    minimum: 0,
    maximum: AbilityTargetDef::UNLIMITED,
    divided_total: Some(DividedTotal::ChosenX),
    another: false,
    excludes_source: false,
    chooser: TargetChooserDef::Controller,
}];

pub(in crate::card::sets) static FIRE_COVENANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a0139c2-ad86-4c71-ab6d-4840c37d5d20"),
    "Fire Covenant",
    CardArt::new("6a0139c2-ad86-4c71-ab6d-4840c37d5d20", "Dan Frazier"),
    CardSet::IceAge,
    // The life is paid as it is cast, so it is spent whether or not the
    // spell resolves -- and it is life, so nothing about the board caps how
    // much damage three mana can deal.
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "As an additional cost to cast this spell, pay X life. This spell deals X damage \
             divided as you choose among any number of target creatures.",
            &FIRE_COVENANT_TARGETS,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        )
        .with_spell_life_cost(SpellLifeCostDef::variable()),
    ),
);

// ICE 290 — Flooded Woodlands
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODED_WOODLANDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de89e9e1-485b-42e5-9728-5d6f948999e1"),
    "Flooded Woodlands",
    crate::card::CardArt::new("de89e9e1-485b-42e5-9728-5d6f948999e1", "Kaja Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 291 — Fumarole
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FUMAROLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efa53e9a-0d7c-4d17-b2be-56930edfa2c2"),
    "Fumarole",
    crate::card::CardArt::new("efa53e9a-0d7c-4d17-b2be-56930edfa2c2", "Drew Tucker"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 292 — Ghostly Flame
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTLY_FLAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6314344b-6493-4142-9c76-da9b90b8d3e1"),
    "Ghostly Flame",
    crate::card::CardArt::new("6314344b-6493-4142-9c76-da9b90b8d3e1", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 293 — Giant Trap Door Spider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_TRAP_DOOR_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8965dfa8-dc90-4cf2-a93b-72bf88b58936"),
    "Giant Trap Door Spider",
    crate::card::CardArt::new("8965dfa8-dc90-4cf2-a93b-72bf88b58936", "Heather Hudson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 294 — Glaciers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b86e159b-ecf1-4b4a-9041-4e97fdf935e5"),
    "Glaciers",
    crate::card::CardArt::new("b86e159b-ecf1-4b4a-9041-4e97fdf935e5", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 295 — Hymn of Rebirth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HYMN_OF_REBIRTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61d0f2f2-f6e2-4b8a-8418-10b17c5e0ea9"),
    "Hymn of Rebirth",
    crate::card::CardArt::new(
        "61d0f2f2-f6e2-4b8a-8418-10b17c5e0ea9",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 296 — Kjeldoran Frostbeast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_FROSTBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fccb1d0-b324-4780-bb9e-4533240da06d"),
    "Kjeldoran Frostbeast",
    crate::card::CardArt::new("2fccb1d0-b324-4780-bb9e-4533240da06d", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 297 — Merieke Ri Berit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MERIEKE_RI_BERIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bf47c0a-5c17-47d0-b663-becff62fbdf8"),
    "Merieke Ri Berit",
    crate::card::CardArt::new("3bf47c0a-5c17-47d0-b663-becff62fbdf8", "Heather Hudson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 298 — Monsoon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MONSOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("254fcc50-79a5-40cd-b028-e78dde3f8480"),
    "Monsoon",
    crate::card::CardArt::new("254fcc50-79a5-40cd-b028-e78dde3f8480", "NéNé Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 299 — Mountain Titan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNTAIN_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcc1d589-02a2-4896-a283-9d0385534667"),
    "Mountain Titan",
    crate::card::CardArt::new("bcc1d589-02a2-4896-a283-9d0385534667", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 300 — Reclamation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RECLAMATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca335f4f-d345-4eb9-9bc6-74595c501078"),
    "Reclamation",
    crate::card::CardArt::new("ca335f4f-d345-4eb9-9bc6-74595c501078", "Dameon Willich"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 301 — Skeleton Ship
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKELETON_SHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("271c8a7c-0f71-4f9d-ab0e-ca7c8c4aca50"),
    "Skeleton Ship",
    crate::card::CardArt::new(
        "271c8a7c-0f71-4f9d-ab0e-ca7c8c4aca50",
        "Amy Weber & Tom Wänerstrand",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 302 — Spectral Shield
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fe0a783-d086-4dc8-ae4a-59f3c2daaca0"),
    "Spectral Shield",
    crate::card::CardArt::new(
        "7fe0a783-d086-4dc8-ae4a-59f3c2daaca0",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 303 — Storm Spirit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a383a5f-4814-4b92-aa80-2a6440a719bc"),
    "Storm Spirit",
    crate::card::CardArt::new("7a383a5f-4814-4b92-aa80-2a6440a719bc", "Pete Venters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 304 — Stormbind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMBIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2d5d91b-aeb4-4d7e-b748-77f9960da55f"),
    "Stormbind",
    crate::card::CardArt::new(
        "c2d5d91b-aeb4-4d7e-b748-77f9960da55f",
        "NéNé Thomas & Phillip Mosness",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 305 — Wings of Aesthir
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WINGS_OF_AESTHIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eeb0282d-ccec-4556-8b70-b6f665077afe"),
    "Wings of Aesthir",
    crate::card::CardArt::new(
        "eeb0282d-ccec-4556-8b70-b6f665077afe",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 306 — Adarkar Sentinel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ADARKAR_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff62754b-f4f0-4731-8dd7-327a820f60a8"),
    "Adarkar Sentinel",
    crate::card::CardArt::new("ff62754b-f4f0-4731-8dd7-327a820f60a8", "Melissa A. Benson"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 307 — Aegis of the Meek
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AEGIS_OF_THE_MEEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d272051-f442-4f6e-8c64-df28b398d2e8"),
    "Aegis of the Meek",
    crate::card::CardArt::new("5d272051-f442-4f6e-8c64-df28b398d2e8", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 308 — Amulet of Quoz
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AMULET_OF_QUOZ: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("764ec6a8-a878-446c-b7e4-6026c2a3e9a4"),
    "Amulet of Quoz",
    crate::card::CardArt::new("764ec6a8-a878-446c-b7e4-6026c2a3e9a4", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 309 — Arcum's Sleigh
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_SLEIGH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9780ce2-756c-48e5-9936-45f6a224f61d"),
    "Arcum's Sleigh",
    crate::card::CardArt::new("e9780ce2-756c-48e5-9936-45f6a224f61d", "Tom Wänerstrand"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 310 — Arcum's Weathervane
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_WEATHERVANE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e142435-6930-4596-bc3b-60abde1229df"),
    "Arcum's Weathervane",
    crate::card::CardArt::new("9e142435-6930-4596-bc3b-60abde1229df", "Tom Wänerstrand"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 311 — Arcum's Whistle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_WHISTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73c07c87-0e44-4a5a-92b7-728350cd02de"),
    "Arcum's Whistle",
    crate::card::CardArt::new("73c07c87-0e44-4a5a-92b7-728350cd02de", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 312 — Barbed Sextant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_SEXTANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edb82654-de12-4dce-8c6b-f28d68f0fbe1"),
    "Barbed Sextant",
    crate::card::CardArt::new("edb82654-de12-4dce-8c6b-f28d68f0fbe1", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 313 — Baton of Morale
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BATON_OF_MORALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bc29872-b1a2-4851-9eca-f3e67ae6e14c"),
    "Baton of Morale",
    crate::card::CardArt::new("8bc29872-b1a2-4851-9eca-f3e67ae6e14c", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 314 — Celestial Sword
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_SWORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bc0e8d3-633b-4281-863f-c51c69eed0b6"),
    "Celestial Sword",
    crate::card::CardArt::new("2bc0e8d3-633b-4281-863f-c51c69eed0b6", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 315 — Crown of the Ages
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_THE_AGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fce2991f-48e1-4cfe-af0a-18b6d9400493"),
    "Crown of the Ages",
    crate::card::CardArt::new("fce2991f-48e1-4cfe-af0a-18b6d9400493", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 316 — Despotic Scepter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DESPOTIC_SCEPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53e381a4-810e-4b75-aed3-c16cf0eb06fa"),
    "Despotic Scepter",
    crate::card::CardArt::new("53e381a4-810e-4b75-aed3-c16cf0eb06fa", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 317 — Elkin Bottle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELKIN_BOTTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49301c19-55a0-4146-9474-0b86cd320e31"),
    "Elkin Bottle",
    crate::card::CardArt::new("49301c19-55a0-4146-9474-0b86cd320e31", "Quinton Hoover"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 318 — Fyndhorn Bow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_BOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65dd0a41-cc51-4728-b597-fdb2510accd8"),
    "Fyndhorn Bow",
    crate::card::CardArt::new("65dd0a41-cc51-4728-b597-fdb2510accd8", "Rob Alexander"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 319 — Goblin Lyre
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_LYRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("951114fb-5ae5-4eb0-8e03-6e39b0b634b5"),
    "Goblin Lyre",
    crate::card::CardArt::new("951114fb-5ae5-4eb0-8e03-6e39b0b634b5", "Mike Kimble"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 320 — Hematite Talisman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEMATITE_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("83585337-56a9-44d2-9ed1-8a959bcfb010"),
    "Hematite Talisman",
    crate::card::CardArt::new("83585337-56a9-44d2-9ed1-8a959bcfb010", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 321 — Ice Cauldron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CAULDRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a3e095a-7056-4df3-bf7d-9c217d591446"),
    "Ice Cauldron",
    crate::card::CardArt::new("1a3e095a-7056-4df3-bf7d-9c217d591446", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 322 — Icy Manipulator (reprint)

// ICE 323 — Infinite Hourglass
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFINITE_HOURGLASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9a42152-32c0-47ff-aaac-8deaf01873ca"),
    "Infinite Hourglass",
    crate::card::CardArt::new("f9a42152-32c0-47ff-aaac-8deaf01873ca", "Harold McNeill"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 324 — Jester's Cap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JESTER_S_CAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47ac44d0-8090-4e7b-ac47-c567294f185e"),
    "Jester's Cap",
    crate::card::CardArt::new("47ac44d0-8090-4e7b-ac47-c567294f185e", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 325 — Jester's Mask
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JESTER_S_MASK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("daa1ba0c-cb89-4bb2-8a35-6a4a4eecccf7"),
    "Jester's Mask",
    crate::card::CardArt::new("daa1ba0c-cb89-4bb2-8a35-6a4a4eecccf7", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 326 — Jeweled Amulet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JEWELED_AMULET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34f7bad2-d28f-42d2-9246-fe3545ef49a7"),
    "Jeweled Amulet",
    crate::card::CardArt::new("34f7bad2-d28f-42d2-9246-fe3545ef49a7", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 327 — Lapis Lazuli Talisman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAPIS_LAZULI_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce00bb19-983e-427d-be54-ae6daf0ccdde"),
    "Lapis Lazuli Talisman",
    crate::card::CardArt::new("ce00bb19-983e-427d-be54-ae6daf0ccdde", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 328 — Malachite Talisman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MALACHITE_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63fb8a24-ce53-4a69-be2a-55c6dbba5ee7"),
    "Malachite Talisman",
    crate::card::CardArt::new("63fb8a24-ce53-4a69-be2a-55c6dbba5ee7", "Christopher Rush"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 329 — Nacre Talisman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NACRE_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06912236-8225-4eb0-8086-c6a163c69892"),
    "Nacre Talisman",
    crate::card::CardArt::new("06912236-8225-4eb0-8086-c6a163c69892", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 330 — Naked Singularity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NAKED_SINGULARITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cabadfb2-93cd-4c7a-b901-59c3dd1a7c3c"),
    "Naked Singularity",
    crate::card::CardArt::new("cabadfb2-93cd-4c7a-b901-59c3dd1a7c3c", "Mark Tedin"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 331 — Onyx Talisman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ONYX_TALISMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a89b2368-1180-4821-bcb8-8161c18e5538"),
    "Onyx Talisman",
    crate::card::CardArt::new("a89b2368-1180-4821-bcb8-8161c18e5538", "Sandra Everingham"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 332 — Pentagram of the Ages
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PENTAGRAM_OF_THE_AGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8d889a5-f6c7-410d-97f9-acf08b9091c8"),
    "Pentagram of the Ages",
    crate::card::CardArt::new("b8d889a5-f6c7-410d-97f9-acf08b9091c8", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 333 — Pit Trap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PIT_TRAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c588fe7f-945d-4459-904c-67442f88b4e1"),
    "Pit Trap",
    crate::card::CardArt::new("c588fe7f-945d-4459-904c-67442f88b4e1", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 334 — Runed Arch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUNED_ARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca02861b-9639-480d-8e54-e024f0c70158"),
    "Runed Arch",
    crate::card::CardArt::new("ca02861b-9639-480d-8e54-e024f0c70158", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 335 — Shield of the Ages
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_OF_THE_AGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7411ab40-47f6-44d1-8e33-9ff5301dcd9b"),
    "Shield of the Ages",
    crate::card::CardArt::new("7411ab40-47f6-44d1-8e33-9ff5301dcd9b", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 336 — Skull Catapult
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKULL_CATAPULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb92a3e6-dc30-4a08-baba-e125290cadc5"),
    "Skull Catapult",
    crate::card::CardArt::new("eb92a3e6-dc30-4a08-baba-e125290cadc5", "Bryon Wackwitz"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 337 — Snow Fortress
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_FORTRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c480e07-fb26-4760-865f-47985f7447bb"),
    "Snow Fortress",
    crate::card::CardArt::new("1c480e07-fb26-4760-865f-47985f7447bb", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 338 — Soldevi Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64d35e88-81d3-4a54-aa79-190615abc616"),
    "Soldevi Golem",
    crate::card::CardArt::new("64d35e88-81d3-4a54-aa79-190615abc616", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 339 — Soldevi Simulacrum
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_SIMULACRUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fabc7b6-e766-4e3c-816e-04cfeceaff09"),
    "Soldevi Simulacrum",
    crate::card::CardArt::new("9fabc7b6-e766-4e3c-816e-04cfeceaff09", "Dan Frazier"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 340 — Staff of the Ages
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STAFF_OF_THE_AGES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c709836-55b6-4de9-b190-b5f66dc53c87"),
    "Staff of the Ages",
    crate::card::CardArt::new("5c709836-55b6-4de9-b190-b5f66dc53c87", "Daniel Gelon"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 341 — Sunstone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c1c67fa-ff88-4a61-b8a5-8a872b3dc44f"),
    "Sunstone",
    crate::card::CardArt::new("3c1c67fa-ff88-4a61-b8a5-8a872b3dc44f", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 342 — Time Bomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_BOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("092ec691-4729-46d3-a4e2-0cfc5df42a31"),
    "Time Bomb",
    crate::card::CardArt::new("092ec691-4729-46d3-a4e2-0cfc5df42a31", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 343 — Urza's Bauble
/// "You draw a card at the beginning of the next turn's upkeep": a delayed
/// draw rather than a cantrip, which is what makes the Bauble free to play
/// and slow to pay.
static BAUBLE_DRAWS: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next turn's upkeep, you draw a card.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::Upkeep,
        player: PlayerRelation::Any,
    },
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);

static BAUBLE_PEEKS: [EffectDef; 2] = [
    EffectDef::LookAtRandomCardInHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&BAUBLE_DRAWS)),
];

static BAUBLE_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource];

static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

pub(in crate::card::sets) static URZAS_BAUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58c9e9a7-e170-4361-b7d5-22fc0771c489"),
    "Urza's Bauble",
    CardArt::new("58c9e9a7-e170-4361-b7d5-22fc0771c489", "Christopher Rush"),
    CardSet::IceAge,
    // A free artifact that replaces itself a turn later, which is why the
    // decks that count artifacts or graveyard cards play it for no other
    // reason.
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Look at a card at random in target player's hand. You draw \
         a card at the beginning of the next turn's upkeep.",
        &BAUBLE_COST,
        &A_PLAYER,
        EffectDef::Sequence(&BAUBLE_PEEKS),
    )),
);

// ICE 344 — Vexing Arcanix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VEXING_ARCANIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c9ea118-6a19-4e1b-aa5a-9b2729efc096"),
    "Vexing Arcanix",
    crate::card::CardArt::new("0c9ea118-6a19-4e1b-aa5a-9b2729efc096", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 345 — Vibrating Sphere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIBRATING_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48f93ded-ecf6-4a70-8ca3-a9c0c3201c21"),
    "Vibrating Sphere",
    crate::card::CardArt::new("48f93ded-ecf6-4a70-8ca3-a9c0c3201c21", "Richard Thomas"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 346 — Walking Wall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cba1238c-1969-452d-8112-124cbbd49417"),
    "Walking Wall",
    crate::card::CardArt::new("cba1238c-1969-452d-8112-124cbbd49417", "Anthony S. Waters"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 347 — Wall of Shields
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_SHIELDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6376c7c4-aaca-4625-83d4-a49f01aec535"),
    "Wall of Shields",
    crate::card::CardArt::new("6376c7c4-aaca-4625-83d4-a49f01aec535", "Randy Gallegos"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 348 — War Chariot
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_CHARIOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0ea0c6c-aa76-4b16-bc99-2ff46dc56d4e"),
    "War Chariot",
    crate::card::CardArt::new("d0ea0c6c-aa76-4b16-bc99-2ff46dc56d4e", "Dameon Willich"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 349 — Whalebone Glider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHALEBONE_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b75adf0-9501-4776-a213-456c2b821070"),
    "Whalebone Glider",
    crate::card::CardArt::new("4b75adf0-9501-4776-a213-456c2b821070", "Amy Weber"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 350 — Zuran Orb
pub(in crate::card::sets) static ZURAN_ORB: CardRecord = CardRecord::new_with_legacy_id(
    2106,
    "Zuran Orb",
    CardArt::new("3a9d1082-a862-45d4-9e5e-392e879fead6", "Sandra Everingham"),
    CardSet::IceAge,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated(
        "Sacrifice a land: You gain 2 life.",
        &[AbilityCostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Land),
            controller: PlayerRelation::You,
        }],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// ICE 351 — Adarkar Wastes
pub(in crate::card::sets) static ADARKAR_WASTES: CardRecord = CardRecord::new_with_legacy_id(
    294,
    "Adarkar Wastes",
    CardArt::new("09dd9023-f7ee-4e99-8821-7059deb83730", "Mike Raabe"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {U}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Blue],
    )),
);

// ICE 352 — Brushland
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRUSHLAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("170e5ccd-54bf-4c6d-86b4-0359ca8f36e8"),
    "Brushland",
    crate::card::CardArt::new("170e5ccd-54bf-4c6d-86b4-0359ca8f36e8", "Bryon Wackwitz"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 353 — Glacial Chasm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_CHASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d23f800-7a6f-40e3-b242-9f5955e47a75"),
    "Glacial Chasm",
    crate::card::CardArt::new("3d23f800-7a6f-40e3-b242-9f5955e47a75", "Liz Danforth"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 354 — Halls of Mist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HALLS_OF_MIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b926a189-90b6-47bb-b5d6-b033e57007b4"),
    "Halls of Mist",
    crate::card::CardArt::new("b926a189-90b6-47bb-b5d6-b033e57007b4", "Mark Poole"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 355 — Ice Floe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_FLOE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85ce04fb-e687-41e0-ae9a-16a51df5d943"),
    "Ice Floe",
    crate::card::CardArt::new("85ce04fb-e687-41e0-ae9a-16a51df5d943", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 356 — Karplusan Forest
pub(in crate::card::sets) static KARPLUSAN_FOREST: CardRecord = CardRecord::new_with_legacy_id(
    295,
    "Karplusan Forest",
    CardArt::new("ba6f1263-d598-49fb-b5f8-09f11822ebd0", "Nicola Leonard"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {R} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Red, ManaColor::Green],
    )),
);

// ICE 357 — Land Cap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAND_CAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4806c02-7a4d-42e3-affd-0338084bd3ab"),
    "Land Cap",
    crate::card::CardArt::new("c4806c02-7a4d-42e3-affd-0338084bd3ab", "Allen Williams"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 358 — Lava Tubes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_TUBES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e7c2cf6-f36f-451b-bba5-19a82c659c4c"),
    "Lava Tubes",
    crate::card::CardArt::new("5e7c2cf6-f36f-451b-bba5-19a82c659c4c", "Bryon Wackwitz"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 359 — River Delta
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIVER_DELTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ea335fc0-0591-4acd-9ae8-7858222770da"),
    "River Delta",
    crate::card::CardArt::new("ea335fc0-0591-4acd-9ae8-7858222770da", "Sandra Everingham"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 360 — Sulfurous Springs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUROUS_SPRINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fdeab50-b45f-412b-85a3-c6cf009ce567"),
    "Sulfurous Springs",
    crate::card::CardArt::new("2fdeab50-b45f-412b-85a3-c6cf009ce567", "Phil Foglio"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 361 — Timberline Ridge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIMBERLINE_RIDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87cc2fc9-0a24-4ac1-afcc-9317b90c7178"),
    "Timberline Ridge",
    crate::card::CardArt::new("87cc2fc9-0a24-4ac1-afcc-9317b90c7178", "Jeff A. Menges"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 362 — Underground River
pub(in crate::card::sets) static UNDERGROUND_RIVER: CardRecord = CardRecord::new_with_legacy_id(
    296,
    "Underground River",
    CardArt::new("92369d7e-5e5a-46f9-bb31-c57d62410283", "NéNé Thomas"),
    CardSet::IceAge,
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {U} or {B}. This land deals 1 damage to you.",
        &[ManaColor::Blue, ManaColor::Black],
    )),
);

// ICE 363 — Veldt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VELDT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("987534fb-74a9-46a3-805f-fe2fe2df4a90"),
    "Veldt",
    crate::card::CardArt::new("987534fb-74a9-46a3-805f-fe2fe2df4a90", "Bryon Wackwitz"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 364 — Plains (reprint)

// ICE 365 — Plains (alternate printing)

// ICE 366 — Plains (alternate printing)

// ICE 367 — Snow-Covered Plains
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_PLAINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb3ac778-fb45-4fd3-a9af-8a0791f833e8"),
    "Snow-Covered Plains",
    crate::card::CardArt::new("cb3ac778-fb45-4fd3-a9af-8a0791f833e8", "Christopher Rush"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 368 — Island (reprint)

// ICE 369 — Island (alternate printing)

// ICE 370 — Island (alternate printing)

// ICE 371 — Snow-Covered Island
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_ISLAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad8b77cf-b53e-4da3-9c27-3851b7b25a98"),
    "Snow-Covered Island",
    crate::card::CardArt::new("ad8b77cf-b53e-4da3-9c27-3851b7b25a98", "Anson Maddocks"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 372 — Snow-Covered Swamp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_SWAMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("65a3c27f-6b15-49b6-ac89-36cfb79b3b54"),
    "Snow-Covered Swamp",
    crate::card::CardArt::new("65a3c27f-6b15-49b6-ac89-36cfb79b3b54", "Douglas Shuler"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 373 — Swamp (reprint)

// ICE 374 — Swamp (alternate printing)

// ICE 375 — Swamp (alternate printing)

// ICE 376 — Mountain (reprint)

// ICE 377 — Mountain (alternate printing)

// ICE 378 — Mountain (alternate printing)

// ICE 379 — Snow-Covered Mountain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_MOUNTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccd3afb3-5574-4f2d-adbe-969a428f1c63"),
    "Snow-Covered Mountain",
    crate::card::CardArt::new("ccd3afb3-5574-4f2d-adbe-969a428f1c63", "Tom Wänerstrand"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

// ICE 380 — Forest (reprint)

// ICE 381 — Forest (alternate printing)

// ICE 382 — Forest (alternate printing)

// ICE 383 — Snow-Covered Forest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_FOREST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c0ad95c-d62c-4138-ada0-fa39a63a449e"),
    "Snow-Covered Forest",
    crate::card::CardArt::new("4c0ad95c-d62c-4138-ada0-fa39a63a449e", "Pat Lewis"),
    crate::card::CardSet::IceAge,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADARKAR_UNICORN,
    &ARCTIC_FOXES,
    &ARENSON_S_AURA,
    &ARMOR_OF_FAITH,
    &BATTLE_CRY,
    &BLACK_SCARAB,
    &BLESSED_WINE,
    &BLINKING_SPIRIT,
    &BLUE_SCARAB,
    &CALL_TO_ARMS,
    &CARIBOU_RANGE,
    &COLD_SNAP,
    &COOPERATION,
    &DROUGHT,
    &ELVISH_HEALER,
    &ENDURING_RENEWAL,
    &ENERGY_STORM,
    &FORMATION,
    &FYLGJA,
    &GENERAL_JARKELD,
    &GREEN_SCARAB,
    &HALLOWED_GROUND,
    &HEAL,
    &HIPPARION,
    &JUSTICE,
    &KELSINKO_RANGER,
    &KJELDORAN_ELITE_GUARD,
    &KJELDORAN_GUARD,
    &KJELDORAN_KNIGHT,
    &KJELDORAN_PHALANX,
    &KJELDORAN_ROYAL_GUARD,
    &KJELDORAN_SKYCAPTAIN,
    &KJELDORAN_SKYKNIGHT,
    &KJELDORAN_WARRIOR,
    &LIGHTNING_BLOW,
    &LOST_ORDER_OF_JARKELD,
    &MERCENARIES,
    &ORDER_OF_THE_SACRED_TORCH,
    &ORDER_OF_THE_WHITE_SHIELD,
    &PRISMATIC_WARD,
    &RALLY,
    &RED_SCARAB,
    &SACRED_BOON,
    &SERAPH,
    &SHIELD_BEARER,
    &SNOW_HOUND,
    &WARNING,
    &WHITE_SCARAB,
    &ARNJLOT_S_ASCENT,
    &BALDUVIAN_CONJURER,
    &BALDUVIAN_SHAMAN,
    &BINDING_GRASP,
    &BRAINSTORM,
    &BREATH_OF_DREAMS,
    &CLAIRVOYANCE,
    &DEFLECTION,
    &DREAMS_OF_THE_DEAD,
    &ENERVATE,
    &ERRANT_MINION,
    &ESSENCE_FLARE,
    &FORCE_VOID,
    &GLACIAL_WALL,
    &HYDROBLAST,
    &ICEBERG,
    &ICY_PRISON,
    &ILLUSIONARY_FORCES,
    &ILLUSIONARY_PRESENCE,
    &ILLUSIONARY_TERRAIN,
    &ILLUSIONARY_WALL,
    &ILLUSIONS_OF_GRANDEUR,
    &INFUSE,
    &KROVIKAN_SORCERER,
    &MAGUS_OF_THE_UNSEEN,
    &MESMERIC_TRANCE,
    &MISTFOLK,
    &MUSICIAN,
    &MYSTIC_MIGHT,
    &MYSTIC_REMORA,
    &PHANTASMAL_MOUNT,
    &POLAR_KRAKEN,
    &PORTENT,
    &RAY_OF_COMMAND,
    &RAY_OF_ERASURE,
    &REALITY_TWIST,
    &SEA_SPIRIT,
    &SHYFT,
    &SIBILANT_SPIRIT,
    &SILVER_ERNE,
    &SNOW_DEVIL,
    &SNOWFALL,
    &SOLDEVI_MACHINIST,
    &SOUL_BARRIER,
    &THUNDER_WALL,
    &UPDRAFT,
    &WIND_SPIRIT,
    &WINTER_S_CHILL,
    &WORD_OF_UNDOING,
    &WRATH_OF_MARIT_LAGE,
    &ZUR_S_WEIRDING,
    &ZURAN_ENCHANTER,
    &ZURAN_SPELLCASTER,
    &ABYSSAL_SPECTER,
    &ASHEN_GHOUL,
    &BRINE_SHAMAN,
    &BURNT_OFFERING,
    &CLOAK_OF_CONFUSION,
    &DANCE_OF_THE_DEAD,
    &DARK_BANISHING,
    &DEMONIC_CONSULTATION,
    &DREAD_WIGHT,
    &DRIFT_OF_THE_DEAD,
    &FLOW_OF_MAGGOTS,
    &FOUL_FAMILIAR,
    &GANGRENOUS_ZOMBIES,
    &GAZE_OF_PAIN,
    &GRAVEBIND,
    &HECATOMB,
    &HOAR_SHADE,
    &HYALOPTEROUS_LEMURE,
    &ICEQUAKE,
    &INFERNAL_DARKNESS,
    &INFERNAL_DENIZEN,
    &KJELDORAN_DEAD,
    &KNIGHT_OF_STROMGALD,
    &KROVIKAN_ELEMENTALIST,
    &KROVIKAN_FETISH,
    &KROVIKAN_VAMPIRE,
    &LEGIONS_OF_LIM_DUL,
    &LESHRAC_S_RITE,
    &LESHRAC_S_SIGIL,
    &LIM_DUL_S_COHORT,
    &LIM_DUL_S_HEX,
    &MIND_RAVEL,
    &MIND_WARP,
    &MIND_WHIP,
    &MINION_OF_LESHRAC,
    &MINION_OF_TEVESH_SZAT,
    &MOLE_WORMS,
    &MOOR_FIEND,
    &NECROPOTENCE,
    &NORRITT,
    &OATH_OF_LIM_DUL,
    &PESTILENCE_RATS,
    &POX,
    &SEIZURES,
    &SONGS_OF_THE_DAMNED,
    &SOUL_BURN,
    &SOUL_KISS,
    &SPOILS_OF_EVIL,
    &SPOILS_OF_WAR,
    &STENCH_OF_EVIL,
    &STROMGALD_CABAL,
    &TOUCH_OF_DEATH,
    &WITHERING_WISPS,
    &AGGRESSION,
    &ANARCHY,
    &AVALANCHE,
    &BALDUVIAN_BARBARIANS,
    &BALDUVIAN_HYDRA,
    &BARBARIAN_GUIDES,
    &BATTLE_FRENZY,
    &BONE_SHAMAN,
    &BRAND_OF_ILL_OMEN,
    &CHAOS_LORD,
    &CHAOS_MOON,
    &CONQUER,
    &CURSE_OF_MARIT_LAGE,
    &DWARVEN_ARMORY,
    &ERRANTRY,
    &FLAME_SPIRIT,
    &FLARE,
    &GAME_OF_CHAOS,
    &GLACIAL_CREVASSES,
    &GOBLIN_MUTANT,
    &GOBLIN_SAPPERS,
    &GOBLIN_SKI_PATROL,
    &GOBLIN_SNOWMAN,
    &GRIZZLED_WOLVERINE,
    &IMPOSING_VISAGE,
    &INCINERATE,
    &JOKULHAUPS,
    &KARPLUSAN_GIANT,
    &KARPLUSAN_YETI,
    &LAVA_BURST,
    &MARTON_STROMGALD,
    &MELEE,
    &MELTING,
    &METEOR_SHOWER,
    &MOUNTAIN_GOAT,
    &MUDSLIDE,
    &ORCISH_CANNONEERS,
    &ORCISH_CONSCRIPTS,
    &ORCISH_FARMER,
    &ORCISH_HEALER,
    &ORCISH_LIBRARIAN,
    &ORCISH_LUMBERJACK,
    &ORCISH_SQUATTERS,
    &PANIC,
    &PYROBLAST,
    &PYROCLASM,
    &SABRETOOTH_TIGER,
    &STONE_SPIRIT,
    &STONEHANDS,
    &TOR_GIANT,
    &TOTAL_WAR,
    &VERTIGO,
    &WALL_OF_LAVA,
    &WORD_OF_BLASTING,
    &AUROCHS,
    &BALDUVIAN_BEARS,
    &BLIZZARD,
    &BROWN_OUPHE,
    &CHUB_TOAD,
    &DIRE_WOLVES,
    &EARTHLORE,
    &ELDER_DRUID,
    &ESSENCE_FILTER,
    &FANATICAL_FEVER,
    &FOLK_OF_THE_PINES,
    &FORBIDDEN_LORE,
    &FORGOTTEN_LORE,
    &FOXFIRE,
    &FREYALISE_SUPPLICANT,
    &FREYALISE_S_CHARM,
    &FREYALISE_S_WINDS,
    &FYNDHORN_BROWNIE,
    &FYNDHORN_ELDER,
    &FYNDHORN_ELVES,
    &FYNDHORN_POLLEN,
    &GORILLA_PACK,
    &HOT_SPRINGS,
    &JOHTULL_WURM,
    &JUNIPER_ORDER_DRUID,
    &LHURGOYF,
    &MADDENING_WIND,
    &NATURE_S_LORE,
    &PALE_BEARS,
    &PYGMY_ALLOSAURUS,
    &PYKNITE,
    &RIME_DRYAD,
    &RITUAL_OF_SUBDUAL,
    &SCALED_WURM,
    &SHAMBLING_STRIDER,
    &SNOWBLIND,
    &STAMPEDE,
    &STUNTED_GROWTH,
    &TARPAN,
    &THERMOKARST,
    &THOUGHTLEECH,
    &TINDER_WALL,
    &TOUCH_OF_VITAE,
    &TRAILBLAZER,
    &VENOMOUS_BREATH,
    &WALL_OF_PINE_NEEDLES,
    &WHITEOUT,
    &WIITIGO,
    &WOOLLY_MAMMOTHS,
    &WOOLLY_SPIDER,
    &YAVIMAYA_GNATS,
    &ALTAR_OF_BONE,
    &CENTAUR_ARCHER,
    &CHROMATIC_ARMOR,
    &DIABOLIC_VISION,
    &EARTHLINK,
    &ELEMENTAL_AUGURY,
    &ESSENCE_VORTEX,
    &FIERY_JUSTICE,
    &FIRE_COVENANT,
    &FLOODED_WOODLANDS,
    &FUMAROLE,
    &GHOSTLY_FLAME,
    &GIANT_TRAP_DOOR_SPIDER,
    &GLACIERS,
    &HYMN_OF_REBIRTH,
    &KJELDORAN_FROSTBEAST,
    &MERIEKE_RI_BERIT,
    &MONSOON,
    &MOUNTAIN_TITAN,
    &RECLAMATION,
    &SKELETON_SHIP,
    &SPECTRAL_SHIELD,
    &STORM_SPIRIT,
    &STORMBIND,
    &WINGS_OF_AESTHIR,
    &ADARKAR_SENTINEL,
    &AEGIS_OF_THE_MEEK,
    &AMULET_OF_QUOZ,
    &ARCUM_S_SLEIGH,
    &ARCUM_S_WEATHERVANE,
    &ARCUM_S_WHISTLE,
    &BARBED_SEXTANT,
    &BATON_OF_MORALE,
    &CELESTIAL_SWORD,
    &CROWN_OF_THE_AGES,
    &DESPOTIC_SCEPTER,
    &ELKIN_BOTTLE,
    &FYNDHORN_BOW,
    &GOBLIN_LYRE,
    &HEMATITE_TALISMAN,
    &ICE_CAULDRON,
    &INFINITE_HOURGLASS,
    &JESTER_S_CAP,
    &JESTER_S_MASK,
    &JEWELED_AMULET,
    &LAPIS_LAZULI_TALISMAN,
    &MALACHITE_TALISMAN,
    &NACRE_TALISMAN,
    &NAKED_SINGULARITY,
    &ONYX_TALISMAN,
    &PENTAGRAM_OF_THE_AGES,
    &PIT_TRAP,
    &RUNED_ARCH,
    &SHIELD_OF_THE_AGES,
    &SKULL_CATAPULT,
    &SNOW_FORTRESS,
    &SOLDEVI_GOLEM,
    &SOLDEVI_SIMULACRUM,
    &STAFF_OF_THE_AGES,
    &SUNSTONE,
    &TIME_BOMB,
    &URZAS_BAUBLE,
    &VEXING_ARCANIX,
    &VIBRATING_SPHERE,
    &WALKING_WALL,
    &WALL_OF_SHIELDS,
    &WAR_CHARIOT,
    &WHALEBONE_GLIDER,
    &ZURAN_ORB,
    &ADARKAR_WASTES,
    &BRUSHLAND,
    &GLACIAL_CHASM,
    &HALLS_OF_MIST,
    &ICE_FLOE,
    &KARPLUSAN_FOREST,
    &LAND_CAP,
    &LAVA_TUBES,
    &RIVER_DELTA,
    &SULFUROUS_SPRINGS,
    &TIMBERLINE_RIDGE,
    &UNDERGROUND_RIVER,
    &VELDT,
    &SNOW_COVERED_PLAINS,
    &SNOW_COVERED_ISLAND,
    &SNOW_COVERED_SWAMP,
    &SNOW_COVERED_MOUNTAIN,
    &SNOW_COVERED_FOREST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_leb::CIRCLE_OF_PROTECTION_BLACK), // ICE 12
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_BLUE),  // ICE 13
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_GREEN), // ICE 14
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_RED),   // ICE 15
    PrintingRecord::reprint(&catalog_lea::CIRCLE_OF_PROTECTION_WHITE), // ICE 16
    PrintingRecord::reprint(&catalog_lea::DEATH_WARD),                 // ICE 19
    PrintingRecord::reprint(&catalog_lea::DISENCHANT),                 // ICE 20
    PrintingRecord::reprint(&catalog_lea::SWORDS_TO_PLOWSHARES),       // ICE 54
    PrintingRecord::reprint(&catalog_lea::COUNTERSPELL),               // ICE 64
    PrintingRecord::reprint(&catalog_lea::POWER_SINK),                 // ICE 91
    PrintingRecord::reprint(&catalog_lea::SLEIGHT_OF_MIND),            // ICE 99
    PrintingRecord::reprint(&catalog_lea::DARK_RITUAL),                // ICE 120
    PrintingRecord::reprint(&catalog_lea::FEAR),                       // ICE 124
    PrintingRecord::reprint(&catalog_lea::HOWL_FROM_BEYOND),           // ICE 132
    PrintingRecord::reprint(&catalog_lea::SHATTER),                    // ICE 216
    PrintingRecord::reprint(&catalog_lea::STONE_RAIN),                 // ICE 217
    PrintingRecord::reprint(&catalog_lea::GIANT_GROWTH),               // ICE 246
    PrintingRecord::reprint(&catalog_lea::HURRICANE),                  // ICE 249
    PrintingRecord::reprint(&catalog_lea::LURE),                       // ICE 253
    PrintingRecord::reprint(&catalog_lea::REGENERATION),               // ICE 259
    PrintingRecord::reprint(&catalog_lea::WILD_GROWTH),                // ICE 277
    PrintingRecord::reprint(&catalog_lea::ICY_MANIPULATOR),            // ICE 322
    PrintingRecord::reprint(&catalog_lea::PLAINS),                     // ICE 364
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),                // ICE 365
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),                // ICE 366
    PrintingRecord::reprint(&catalog_lea::ISLAND),                     // ICE 368
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),                // ICE 369
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),                // ICE 370
    PrintingRecord::reprint(&catalog_lea::SWAMP),                      // ICE 373
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),                 // ICE 374
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),                 // ICE 375
    PrintingRecord::reprint(&catalog_lea::MOUNTAIN),                   // ICE 376
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),              // ICE 377
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2),              // ICE 378
    PrintingRecord::reprint(&catalog_lea::FOREST),                     // ICE 380
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),                // ICE 381
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),                // ICE 382
];
