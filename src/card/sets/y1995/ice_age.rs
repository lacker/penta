//! Ice Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardRules, CardSet, CardType, DividedTotal,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef,
    PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    TargetChooserDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind,
    abilities,
};
use crate::{TargetIndex, mana_cost};

// ICE 1 — Adarkar Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADARKAR_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Adarkar Unicorn",
    "0ba7526f-dba8-4483-b925-946164fc0ae9",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 2 — Arctic Foxes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_FOXES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Arctic Foxes",
    "98f99c3e-dddc-492f-aab6-1d899346a385",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 3 — Arenson's Aura
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARENSON_S_AURA: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Arenson's Aura",
    "f94f3e87-1b39-49a8-ad0d-f18c854e298a",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 4 — Armor of Faith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMOR_OF_FAITH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Armor of Faith",
    "fccbbc47-99c6-4ba9-95c2-992d5d2a67b2",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 5 — Battle Cry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_CRY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Battle Cry",
    "c558a8c4-035c-464e-9ff8-c188c1bb619e",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 6 — Black Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_SCARAB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Black Scarab",
    "5bfd4ee1-05f9-45ae-a31d-1225b271dbe6",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 7 — Blessed Wine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_WINE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Blessed Wine",
    "6b9a92f9-9bbc-4887-9fbc-0f7212fd5e66",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 8 — Blinking Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLINKING_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Blinking Spirit",
    "14fc0683-9cfa-4439-a533-8773e7747ec4",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 9 — Blue Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLUE_SCARAB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Blue Scarab",
    "b423bb5a-eaac-4c1d-981a-1c635001fc5a",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 10 — Call to Arms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_TO_ARMS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Call to Arms",
    "a92f0d4a-23d8-47d4-b910-d142e0eefd3d",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 11 — Caribou Range
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARIBOU_RANGE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Caribou Range",
    "1e5f8041-67fc-4e00-b119-d216e5cc5a3a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 12 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "d528045d-3b80-48fd-b606-c132da052685",
    "Sandra Everingham",
);

// ICE 13 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "e0d377ec-c43c-43b9-934a-91b4d11650ab",
    "Pete Venters",
);

// ICE 14 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "487dfb1f-b3ab-4daa-bbd9-c43dc91a5fba",
    "Sandra Everingham",
);

// ICE 15 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "5790ce22-a94f-402e-bcc7-b98f71af9fe5",
    "Pete Venters",
);

// ICE 16 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "48bc4bb0-350c-424e-976e-b800915f7fb4",
    "Sandra Everingham",
);

// ICE 17 — Cold Snap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLD_SNAP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Cold Snap",
    "81b87a58-b20c-4f38-afa3-59d398195740",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 18 — Cooperation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COOPERATION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Cooperation",
    "21a815ed-c8b4-4414-8b27-ea612e2977e2",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 19 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "c7b21d29-050d-4704-a4c8-93e3b55086ac",
    "Harold McNeill",
);

// ICE 20 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "b6085d0c-ab2b-445d-bf9d-0fa0a19183a2",
    "Brian Snõddy",
);

// ICE 21 — Drought
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROUGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Drought",
    "97736696-3de3-416d-94cf-4fac792f23f0",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 22 — Elvish Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Elvish Healer",
    "00bd8485-d63a-4077-a3d1-4d0f2f4d8035",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 23 — Enduring Renewal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDURING_RENEWAL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Enduring Renewal",
    "be77edac-9a8b-4b7f-a859-27df76b10aa6",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 24 — Energy Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Energy Storm",
    "3955e358-4285-44e2-9e24-9804346a6e58",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 25 — Formation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORMATION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Formation",
    "78446ead-61b0-485f-a5a9-b3e72d8075a7",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 26 — Fylgja
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYLGJA: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fylgja",
    "3c6358a1-37f0-4b40-93d4-4f1652c38404",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 27 — General Jarkeld
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENERAL_JARKELD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "General Jarkeld",
    "6a4f5a28-0bd2-4cc4-b67f-324e89193caa",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 28 — Green Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEN_SCARAB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Green Scarab",
    "0fbf9266-c97e-4666-b0fa-1802a69a62cc",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 29 — Hallowed Ground
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALLOWED_GROUND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hallowed Ground",
    "4b35c0f4-5633-4ea9-9bda-daaf787aebdd",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 30 — Heal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Heal",
    "9e6b2704-685e-4c74-875a-25846175e5e4",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 31 — Hipparion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIPPARION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hipparion",
    "5969875a-f647-4daf-b76c-d1514d45c312",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 32 — Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUSTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Justice",
    "9a6e0c8d-0fc1-4f52-8357-e550b0ac579a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 33 — Kelsinko Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELSINKO_RANGER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kelsinko Ranger",
    "8402543e-5406-404f-95c4-800a1dce35f1",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 34 — Kjeldoran Elite Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ELITE_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Elite Guard",
    "a73bc4b6-f7d0-494c-9e60-48279c11b7b6",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 35 — Kjeldoran Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Guard",
    "bdf41f17-8f82-4a8c-adec-0f3804faff3b",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 36 — Kjeldoran Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Knight",
    "d5b9db8f-93b5-44e3-9e2b-728c80dfbb37",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ICE 37 — Kjeldoran Phalanx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_PHALANX: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Phalanx",
    "b6e91ba0-b229-4ab1-84f3-2a490dfa5051",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 38 — Kjeldoran Royal Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ROYAL_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Royal Guard",
    "66343008-c38a-48a9-b767-fd2243103690",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 39 — Kjeldoran Skycaptain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_SKYCAPTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Skycaptain",
    "cf0115e0-6192-48a9-9e58-f3ef77ef77c2",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 40 — Kjeldoran Skyknight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_SKYKNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Skyknight",
    "f794665a-8353-482a-b065-2a0777a8acda",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 41 — Kjeldoran Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_WARRIOR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Warrior",
    "ce76f38f-566e-49ff-b197-510cfa1cb51c",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 42 — Lightning Blow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_BLOW: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lightning Blow",
    "d1a4ed99-f38c-4e0f-9ff2-2e1e9126e6ef",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 43 — Lost Order of Jarkeld
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_ORDER_OF_JARKELD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lost Order of Jarkeld",
    "0f8fe1e5-69d2-401f-97cb-3cc01064bad3",
    "Andi Rusu",
    crate::card::CardRules::unsupported(),
);

// ICE 44 — Mercenaries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCENARIES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mercenaries",
    "7b28762d-1ab7-460e-b433-27f5fa858959",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 45 — Order of the Sacred Torch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_OF_THE_SACRED_TORCH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Order of the Sacred Torch",
    "ccc5cb36-c43d-4c71-8019-9b683e160a0a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 46 — Order of the White Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_OF_THE_WHITE_SHIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Order of the White Shield",
    "92e55b10-375f-4b4f-b676-3b9b8085fdd2",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 47 — Prismatic Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_WARD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Prismatic Ward",
    "6f8b50fd-3d1d-4ea8-a3c7-98ca7a8a455e",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 48 — Rally
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RALLY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Rally",
    "e1e9f80e-5d75-45b7-9c66-c0f30996f4dc",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 49 — Red Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_SCARAB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Red Scarab",
    "9a734154-5944-42f4-a02e-c426a45847f3",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 50 — Sacred Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Sacred Boon",
    "d721569d-9cf2-4c3c-b11c-4c46c258a0d2",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 51 — Seraph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERAPH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Seraph",
    "ab675291-3189-43f3-b11b-0724eca8b941",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 52 — Shield Bearer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_BEARER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Shield Bearer",
    "318ff2da-d309-469c-8e2f-fa3c7517a15a",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 53 — Snow Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_HOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow Hound",
    "084437ba-26d4-4af6-ab00-dcb145dd2cd0",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ICE 54 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWORDS_TO_PLOWSHARES,
    "375fd2cb-443b-4be4-ad60-6d1a8e74f510",
    "Kaja Foglio",
);

// ICE 55 — Warning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARNING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Warning",
    "cca5b4a7-df11-4635-a147-df12cd13a67c",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ICE 56 — White Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHITE_SCARAB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "White Scarab",
    "c57726b5-dfdd-4e47-bc52-ebf6eedbf3bd",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 57 — Arnjlot's Ascent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARNJLOT_S_ASCENT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Arnjlot's Ascent",
    "2307fb16-8b77-45b5-8a02-51a13214791d",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 58 — Balduvian Conjurer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_CONJURER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Balduvian Conjurer",
    "5b616963-fac0-451c-8df4-2cacc9466b17",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 59 — Balduvian Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Balduvian Shaman",
    "74859723-8ddf-4ee6-a0a7-87192c84e8ad",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 60 — Binding Grasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BINDING_GRASP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Binding Grasp",
    "6b086186-5fbf-4ba7-af0d-ee3ad61d27bb",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 61 — Brainstorm
pub(in crate::card::sets) static BRAINSTORM: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Brainstorm",
    "8d42d7aa-7f53-4cfc-842a-086aab2448d1",
    "Christopher Rush",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREATH_OF_DREAMS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Breath of Dreams",
    "e40c9657-fab4-489d-8eb0-960ba2605add",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 63 — Clairvoyance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLAIRVOYANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Clairvoyance",
    "46740353-e2ba-4d80-a97d-1368bc67bf30",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 64 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "aedbcbaa-40f0-485f-8427-778edc2d2ec0",
    "Allen Williams",
);

// ICE 65 — Deflection
pub(in crate::card::sets) static DEFLECTION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Deflection",
    "1005a00a-6a0e-44cb-abea-37e2e53125e2",
    "Mike Raabe",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Change the target of target spell with a single target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::DeclaredTargetCount {
                        minimum: 1,
                        maximum: 1,
                    },
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            chooser: PlayerRefDef::EffectController,
            change: crate::card::StackTargetChangeDef::ChooseNew {
                optional: false,
                restriction: None,
            },
        }),
    )),
);

// ICE 66 — Dreams of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAMS_OF_THE_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Dreams of the Dead",
    "93372854-57e7-4db7-a1a6-376c9f49a514",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 67 — Enervate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERVATE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Enervate",
    "c4fdfc5b-c2ab-4c4d-b120-301e17f3d9c6",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 68 — Errant Minion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRANT_MINION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Errant Minion",
    "61648ddb-6efb-43d0-b2b1-418cc957854c",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 69 — Essence Flare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FLARE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Essence Flare",
    "13ebb5dd-d7f1-4b06-8585-7004045be542",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 70 — Force Void
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORCE_VOID: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Force Void",
    "226555ba-22af-45f1-a3f4-d265f8685dd5",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 71 — Glacial Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Glacial Wall",
    "07b71bc1-d9a2-4e99-a8fa-cd696925328d",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 72 — Hydroblast
pub(in crate::card::sets) static HYDROBLAST: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Hydroblast",
    "f62716f0-fde2-49ef-b8a4-c1b03f451194",
    "Kaja Foglio",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's red.",
                &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Color(ManaColor::Red),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                }),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's red.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(ManaColor::Red)),
                true,
            ),
        ],
    )),
);

// ICE 73 — Iceberg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICEBERG: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Iceberg",
    "a2f70e49-17fa-4033-bd45-63374f7f5ec5",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 74 — Icy Prison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICY_PRISON: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Icy Prison",
    "39a7e496-8d2e-49db-b298-475d9017537a",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 75 — Illusionary Forces
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_FORCES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Illusionary Forces",
    "ab02268e-01cf-4729-95ca-5773afd40b56",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// ICE 76 — Illusionary Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_PRESENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Illusionary Presence",
    "aa31efed-4a11-4f59-a623-bac45d20091d",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 77 — Illusionary Terrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_TERRAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Illusionary Terrain",
    "691f4a1b-4706-41aa-82da-ae920739f036",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 78 — Illusionary Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Illusionary Wall",
    "6430e8e2-fee3-4744-820e-d6e16cb992bd",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 79 — Illusions of Grandeur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONS_OF_GRANDEUR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Illusions of Grandeur",
    "17eeeef2-2ced-42b8-a5e0-1095c9e13b02",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 80 — Infuse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFUSE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Infuse",
    "223287b6-224c-4e00-946c-e7ac5539bd45",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 81 — Krovikan Sorcerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_SORCERER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Krovikan Sorcerer",
    "9c5fc053-7b0b-4e76-bf87-ccdb1e8752ed",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ICE 82 — Magus of the Unseen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGUS_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Magus of the Unseen",
    "86da04e9-b94d-42af-add3-02baf772bd33",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 83 — Mesmeric Trance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MESMERIC_TRANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mesmeric Trance",
    "ae3df593-e9d5-479d-9a9a-1c7262dd9c6c",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 84 — Mistfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mistfolk",
    "4f3f4d4e-ca4a-4fba-b9fd-cd1d9457cfa1",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 85 — Musician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUSICIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Musician",
    "9f8d2247-a10e-413a-b497-2add3918f991",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 86 — Mystic Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_MIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mystic Might",
    "e35d7f08-0687-41bd-8c53-31a49adabb11",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 87 — Mystic Remora
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_REMORA: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mystic Remora",
    "58e93dff-b774-4765-b7bd-d3957e42ff4a",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 88 — Phantasmal Mount
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_MOUNT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Phantasmal Mount",
    "75afdbe6-a3f9-49cf-b4ef-f370e518e960",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 89 — Polar Kraken
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLAR_KRAKEN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Polar Kraken",
    "aee01e9c-0445-4228-a73a-3e5744844ed3",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 90 — Portent
pub(in crate::card::sets) static PORTENT: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Portent",
    "e040be83-3fb5-4da5-ba7a-4923b8854b74",
    "Liz Danforth",
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Look at the top three cards of target player's library, then put them back in any order. You may have that player shuffle.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
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
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "85cbec45-81b4-40cc-b356-d6713a6a9b2b",
    "Mark Poole",
);

// ICE 92 — Ray of Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_OF_COMMAND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ray of Command",
    "638abe5f-2a8a-42ca-bcdf-a52a3df66946",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 93 — Ray of Erasure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_OF_ERASURE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ray of Erasure",
    "5a09fc0b-7b9c-4283-8336-f2607f5ffaf5",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 94 — Reality Twist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REALITY_TWIST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Reality Twist",
    "1b7e955c-3de2-430c-93b9-0b39ccea5420",
    "James Ernest",
    crate::card::CardRules::unsupported(),
);

// ICE 95 — Sea Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Sea Spirit",
    "f2d93d05-98bc-4504-9045-dedb925895ae",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 96 — Shyft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHYFT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Shyft",
    "99a60c33-b641-42c4-870d-95d07bc975dc",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 97 — Sibilant Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIBILANT_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Sibilant Spirit",
    "47364ad2-5ce9-4b19-a9d2-f6a33188b882",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ICE 98 — Silver Erne
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_ERNE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Silver Erne",
    "685076cc-098c-4f98-918c-0ad825eda10f",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 99 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "93dc9f02-11ad-4c4a-8199-9d20c23d31a7",
    "Nicola Leonard",
);

// ICE 100 — Snow Devil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_DEVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow Devil",
    "2be3a9a5-2ac5-4ea4-915d-8cff35c0e72f",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 101 — Snowfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWFALL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snowfall",
    "788ed793-3993-4a63-b9f9-9ac3947c3108",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 102 — Soldevi Machinist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_MACHINIST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Soldevi Machinist",
    "1f0999df-2f94-499e-b9af-fe377d515400",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 103 — Soul Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_BARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Soul Barrier",
    "9ad7fac7-db4d-45b2-aba6-16f4fd1a586f",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 104 — Thunder Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Thunder Wall",
    "4fc5d510-c4f7-4a09-bf86-83c3fa3f8928",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 105 — Updraft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UPDRAFT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Updraft",
    "d1bd4e16-27fe-4c7b-ae25-78ed77d8e8e7",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 106 — Wind Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIND_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wind Spirit",
    "4d882447-9594-4aab-b1a7-8bb275f250cf",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 107 — Winter's Chill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_S_CHILL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Winter's Chill",
    "a779aca7-ff2c-48d8-9484-6ad04b2c6bcb",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 108 — Word of Undoing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORD_OF_UNDOING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Word of Undoing",
    "22b04476-5a5d-4843-a948-82db209c4218",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 109 — Wrath of Marit Lage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRATH_OF_MARIT_LAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wrath of Marit Lage",
    "1d512f5c-0327-4d49-8a26-672574a49102",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 110 — Zur's Weirding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZUR_S_WEIRDING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Zur's Weirding",
    "e1f8531f-19ca-48a2-baf2-c5dc6f18d79c",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 111 — Zuran Enchanter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZURAN_ENCHANTER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Zuran Enchanter",
    "721edcef-f40a-4d43-9d80-26161dc425cb",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 112 — Zuran Spellcaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZURAN_SPELLCASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Zuran Spellcaster",
    "152a72b1-a7b7-4e5c-8558-fab97465f549",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 113 — Abyssal Specter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABYSSAL_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Abyssal Specter",
    "fc26f19c-bcf7-4bd8-af42-4757dbe47fb1",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 114 — Ashen Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHEN_GHOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ashen Ghoul",
    "6bb83301-5662-4628-b536-6a3ee0296f2e",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ICE 115 — Brine Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRINE_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Brine Shaman",
    "f445962c-44a1-4f3f-88d4-17048f8ca9dc",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 116 — Burnt Offering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNT_OFFERING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Burnt Offering",
    "1dae52a2-3af7-4b97-9d2e-2448b7c413fb",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 117 — Cloak of Confusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOAK_OF_CONFUSION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Cloak of Confusion",
    "dc45d103-0fca-4431-a5c0-869f0f9be93e",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 118 — Dance of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DANCE_OF_THE_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Dance of the Dead",
    "e7c53ba4-9956-4cd6-85ca-2d6b61a5127c",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 119 — Dark Banishing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_BANISHING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Dark Banishing",
    "f7dc2716-ed62-4797-ad2b-227eca5408d0",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 120 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "4ebcd681-1871-4914-bcd7-6bd95829f6e0",
    "Justin Hampton",
);

// ICE 121 — Demonic Consultation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMONIC_CONSULTATION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Demonic Consultation",
    "8d727b9b-6114-414d-9172-16b6e1db41cc",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 122 — Dread Wight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_WIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Dread Wight",
    "65d332e2-4b2d-4131-84f7-862cb138c477",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 123 — Drift of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIFT_OF_THE_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Drift of the Dead",
    "d8b65656-9f8c-4179-81aa-4b15d8280baa",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 124 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "5709398f-0744-4780-a1d2-eead96c8f348",
    "Rick Emond",
);

// ICE 125 — Flow of Maggots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOW_OF_MAGGOTS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Flow of Maggots",
    "6880a4d3-5cbc-4a01-9190-3565617efcc9",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ICE 126 — Foul Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUL_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Foul Familiar",
    "8bad3541-8e40-4a2f-ac9d-f7b61f3d75a1",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 127 — Gangrenous Zombies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANGRENOUS_ZOMBIES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Gangrenous Zombies",
    "08be4d83-99be-4360-90f1-104dee1c3c2f",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 128 — Gaze of Pain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAZE_OF_PAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Gaze of Pain",
    "48401643-ec4b-444a-8f9a-1a5ea471ff4a",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 129 — Gravebind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEBIND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Gravebind",
    "4782fd4f-2474-4d0d-8301-e0b52af93746",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 130 — Hecatomb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HECATOMB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hecatomb",
    "8f59620f-ff9e-44d8-9c4e-be9de1a919e8",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 131 — Hoar Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOAR_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hoar Shade",
    "72242dff-15ca-4da0-b3ae-9984d037b31f",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 132 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "ca9d0d6b-056e-4b94-8de5-a325768f67b6",
    "Mark Poole",
);

// ICE 133 — Hyalopterous Lemure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYALOPTEROUS_LEMURE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hyalopterous Lemure",
    "d2c9e037-f4d5-46fd-b439-56bee6fb2ad3",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 134 — Icequake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICEQUAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Icequake",
    "14b4dd4d-c617-4603-8a87-761ec6fc6883",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 135 — Infernal Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_DARKNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Infernal Darkness",
    "f3475eb3-909d-450b-9597-b241b259b425",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 136 — Infernal Denizen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_DENIZEN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Infernal Denizen",
    "b63ac9a6-aaa5-4659-97d1-c5f6b0d5ccfe",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 137 — Kjeldoran Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Dead",
    "d3f7b614-6075-4b7c-acc7-ab63185b570b",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 138 — Knight of Stromgald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_STROMGALD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Knight of Stromgald",
    "2b87069b-ebaf-4705-b5da-446932af9b73",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 139 — Krovikan Elementalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_ELEMENTALIST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Krovikan Elementalist",
    "bbedca18-a074-4441-b0a9-7b14fdb07412",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 140 — Krovikan Fetish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_FETISH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Krovikan Fetish",
    "844e73e6-b201-4b2e-b46a-b719484fba0e",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 141 — Krovikan Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_VAMPIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Krovikan Vampire",
    "717c5dda-8e38-4c76-b241-685198402284",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 142 — Legions of Lim-Dûl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGIONS_OF_LIM_DUL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Legions of Lim-Dûl",
    "75b67eb2-b60e-46b4-9d48-11c284957bec",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 143 — Leshrac's Rite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESHRAC_S_RITE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Leshrac's Rite",
    "4e0a6b4e-95b4-40f6-bb19-568dbd908a2b",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 144 — Leshrac's Sigil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESHRAC_S_SIGIL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Leshrac's Sigil",
    "ad5ba7ee-d6df-4b62-a8a1-c81e6fca392a",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 145 — Lim-Dûl's Cohort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_COHORT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lim-Dûl's Cohort",
    "3d0006f6-2f96-453d-9145-eaefa588efbc",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 146 — Lim-Dûl's Hex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_HEX: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lim-Dûl's Hex",
    "af976f42-3d56-4e32-8294-970a276a4bf3",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 147 — Mind Ravel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_RAVEL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mind Ravel",
    "61cf3ac5-985d-4b48-b230-d5ae4ab1ace8",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 148 — Mind Warp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_WARP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mind Warp",
    "de150cd6-0bbc-47f7-a781-cd1aa10eabc6",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 149 — Mind Whip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_WHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mind Whip",
    "3f3ff5fb-4126-4a18-b540-2beaae382e59",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 150 — Minion of Leshrac
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_LESHRAC: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Minion of Leshrac",
    "61278908-a1b4-4b4c-84f5-498ca41fc6b6",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 151 — Minion of Tevesh Szat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_TEVESH_SZAT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Minion of Tevesh Szat",
    "ea9f3ab5-6a31-47db-b8bf-4c56a7ff19d1",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ICE 152 — Mole Worms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLE_WORMS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mole Worms",
    "4914f6fc-e3e7-426b-8688-12157c7df9e7",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 153 — Moor Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOOR_FIEND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Moor Fiend",
    "57089dd4-e30d-498d-9341-43c104c6f3f9",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 154 — Necropotence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECROPOTENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Necropotence",
    "54d7a0c1-efb4-4a8d-ad92-a96d43835052",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 155 — Norritt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NORRITT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Norritt",
    "35abefe6-c39b-4fe5-b2e3-d213f0c4f447",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 156 — Oath of Lim-Dûl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_LIM_DUL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Oath of Lim-Dûl",
    "f16df768-06de-43a0-b548-44fb0887490b",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 157 — Pestilence Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PESTILENCE_RATS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pestilence Rats",
    "bff7f6a6-0e90-4eb4-b76e-d98454975fb6",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 158 — Pox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POX: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pox",
    "a914138c-a593-414c-bbcb-83d3c1bc4f6f",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 159 — Seizures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZURES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Seizures",
    "da369c86-7e17-43d8-b626-b6842e3d2d50",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ICE 160 — Songs of the Damned
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONGS_OF_THE_DAMNED: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Songs of the Damned",
    "6cff3547-8c72-439a-91fe-ebe729dab748",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 161 — Soul Burn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_BURN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Soul Burn",
    "eb8e00d2-2381-4d45-bed8-c9bf738a9419",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 162 — Soul Kiss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_KISS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Soul Kiss",
    "42fbf6a5-86fe-41a3-891e-f72f11ad0aee",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 163 — Spoils of Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPOILS_OF_EVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Spoils of Evil",
    "fd368eb6-72f0-42d4-afa5-3daa7de949ff",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 164 — Spoils of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPOILS_OF_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Spoils of War",
    "b38af8bd-d927-46d0-a1b1-fb437ea9ea66",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 165 — Stench of Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STENCH_OF_EVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stench of Evil",
    "4c7065a2-f819-4cbe-b453-a55e904f0461",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 166 — Stromgald Cabal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STROMGALD_CABAL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stromgald Cabal",
    "6ac6fa0c-753e-4fbc-8a70-0f956503cf4e",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 167 — Touch of Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCH_OF_DEATH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Touch of Death",
    "a49c658f-e657-490b-af1f-e67e48d0046e",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 168 — Withering Wisps
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITHERING_WISPS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Withering Wisps",
    "ad1e6ae5-c972-42c0-ae78-f203873aeeb1",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 169 — Aggression
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Aggression",
    "f3f26060-0c24-496c-b8e2-4dac7ea6166b",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 170 — Anarchy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANARCHY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Anarchy",
    "28d941da-b5cb-4b7e-84f2-ece883f89af3",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 171 — Avalanche
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVALANCHE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Avalanche",
    "d3a925e5-0d0a-42ec-b1c6-9793b8e11625",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 172 — Balduvian Barbarians
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_BARBARIANS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Balduvian Barbarians",
    "efeabe8e-8107-4d19-8a43-362aa79cdd92",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 173 — Balduvian Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_HYDRA: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Balduvian Hydra",
    "c3a3b37f-daa6-4502-bb12-c72afe3df035",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 174 — Barbarian Guides
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBARIAN_GUIDES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Barbarian Guides",
    "fe65a045-dacb-4392-bcb6-843394ef98c9",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 175 — Battle Frenzy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_FRENZY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Battle Frenzy",
    "a85ae675-56ca-4a00-83d2-ee035f33d6d1",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 176 — Bone Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Bone Shaman",
    "0a5e3d54-4dc4-482b-8ecc-bb819ba03d2c",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 177 — Brand of Ill Omen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAND_OF_ILL_OMEN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Brand of Ill Omen",
    "ceeb7bbc-2d41-4709-95be-1ceb952ed1fb",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 178 — Chaos Lord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_LORD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Chaos Lord",
    "ee245922-b380-4b2e-a43f-ab1ba8078943",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 179 — Chaos Moon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_MOON: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Chaos Moon",
    "aae0543f-7f8b-4327-b735-ac21244e9936",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 180 — Conquer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONQUER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Conquer",
    "ae610e66-7bcb-40ec-bed5-86dcfd098654",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 181 — Curse of Marit Lage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURSE_OF_MARIT_LAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Curse of Marit Lage",
    "69b381c1-aa71-4d40-a320-70f58a440d51",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 182 — Dwarven Armory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_ARMORY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Dwarven Armory",
    "7d14a430-6e08-40cf-970a-cae84bba6ef7",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 183 — Errantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRANTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Errantry",
    "8346e741-61f8-4283-be51-f5f80e9595a5",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 184 — Flame Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Flame Spirit",
    "add2b82a-9aa5-4d5c-a1c2-e313541f12c8",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// ICE 185 — Flare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLARE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Flare",
    "d5350236-7bd2-462d-9768-50087626c764",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 186 — Game of Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAME_OF_CHAOS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Game of Chaos",
    "08265332-2c0e-4c42-8c51-83ac20462eed",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 187 — Glacial Crevasses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_CREVASSES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Glacial Crevasses",
    "2726b192-f239-470b-8ad6-69887405e7f9",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 188 — Goblin Mutant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MUTANT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Goblin Mutant",
    "6db54f95-6652-45a3-b960-c2fc118beca1",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 189 — Goblin Sappers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SAPPERS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Goblin Sappers",
    "de839540-a7b9-4f91-91df-3fd4f5c0bc4e",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 190 — Goblin Ski Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SKI_PATROL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Goblin Ski Patrol",
    "fde1c8b5-1e01-4920-8d02-bf80d5b238c5",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 191 — Goblin Snowman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SNOWMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Goblin Snowman",
    "5bbb260a-6763-4d1c-a009-4e34cd572519",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 192 — Grizzled Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIZZLED_WOLVERINE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Grizzled Wolverine",
    "95bb17b9-55c4-4cc1-83f6-75490b9a97d0",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 193 — Imposing Visage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPOSING_VISAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Imposing Visage",
    "cca42b74-9b42-482b-b12a-79cafdcd087e",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 194 — Incinerate
pub(in crate::card::sets) static INCINERATE: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Incinerate",
    "9c3f00af-010d-4485-b8b7-47400d99c496",
    "Mark Poole",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOKULHAUPS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Jokulhaups",
    "3bf0d325-5928-4593-8faa-64ffa414cb48",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 196 — Karplusan Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARPLUSAN_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Karplusan Giant",
    "c524ac2a-294c-4b19-b00b-999e370a3b95",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 197 — Karplusan Yeti
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARPLUSAN_YETI: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Karplusan Yeti",
    "7dd9b214-d9fe-4c2e-b45b-7145ad98c408",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 198 — Lava Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_BURST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lava Burst",
    "79dc0e20-5790-4927-8432-cf0e9b7381d4",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 199 — Márton Stromgald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARTON_STROMGALD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Márton Stromgald",
    "7880e815-53e7-43e0-befd-e368f00a75d8",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 200 — Melee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELEE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Melee",
    "b13a064d-bff4-4a48-a158-1b61951b0ac3",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 201 — Melting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELTING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Melting",
    "8d90065e-2c7e-44e5-9f59-015d468214bf",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 202 — Meteor Shower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_SHOWER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Meteor Shower",
    "50b4851e-677b-468e-9baa-e47a3b4b8339",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 203 — Mountain Goat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNTAIN_GOAT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mountain Goat",
    "ccf70276-a40c-4d25-b584-4c8a07a00602",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 204 — Mudslide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUDSLIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mudslide",
    "65acce56-8674-471e-9d5e-91b7e3f672c1",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 205 — Orcish Cannoneers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_CANNONEERS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Orcish Cannoneers",
    "a4309a2f-27f5-4652-b0b4-6a6119436f75",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 206 — Orcish Conscripts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_CONSCRIPTS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Orcish Conscripts",
    "e71394f8-3038-4cad-adea-a704f004777f",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 207 — Orcish Farmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_FARMER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Orcish Farmer",
    "efa5beef-d609-4809-a813-621b0b4cff7f",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 208 — Orcish Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Orcish Healer",
    "7ff511f3-416e-4919-acd6-fd8183bf5c60",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 209 — Orcish Librarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_LIBRARIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Orcish Librarian",
    "8ed908d6-6d06-4ccb-9577-37ef2d01c1a5",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 210 — Orcish Lumberjack
pub(in crate::card::sets) static ORCISH_LUMBERJACK: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Orcish Lumberjack",
    "21ef13e3-658c-43a3-a290-4c5dde8e8b55",
    "Dan Frazier",
    // One mana for a 1/1 that turns a land into three mana of either colour:
    // the land is gone and the body is nothing, and the deck playing it only
    // needs the turn it buys.
    CardRules::new_creature(mana_cost!("{R}"), &["Orc"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}, Sacrifice a Forest: Add three mana in any combination of {R} and/or {G}.",
            // "Sacrifice a Forest" reads the land type rather than the card name, so a
            // dual land with the type counts and a Forest somebody enchanted still
            // does. Which one is spent is chosen as the ability is activated.
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddMana(AddManaEffectDef::combination(
                &[ManaColor::Red, ManaColor::Green],
                3,
            )),
        ),
    ),
);

// ICE 211 — Orcish Squatters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_SQUATTERS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Orcish Squatters",
    "f3ee7bd5-612b-4916-a914-1294805b8f64",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 212 — Panic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANIC: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Panic",
    "a9ab85ac-311c-4e36-943a-817e43a3c8a8",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// ICE 213 — Pyroblast
pub(in crate::card::sets) static PYROBLAST: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Pyroblast",
    "c342cac5-08ae-4428-9c2c-f6c5904e54d2",
    "Kaja Foglio",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's blue.",
                &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                }),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's blue.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Blue,
                )),
                true,
            ),
        ],
    )),
);

// ICE 214 — Pyroclasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYROCLASM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pyroclasm",
    "88040748-ad76-4b9a-bd4e-87e5980e9816",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ICE 215 — Sabretooth Tiger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SABRETOOTH_TIGER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Sabretooth Tiger",
    "6914c5a8-2114-41c5-a471-ca97524d622f",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 216 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "7eb18d53-20de-43d7-86f7-97a6d14d54b8",
    "Bryon Wackwitz",
);

// ICE 217 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "5a002e6d-ea59-4694-b3e5-075d6020b0d9",
    "Kaja Foglio",
);

// ICE 218 — Stone Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stone Spirit",
    "789dfae7-fe23-4e2e-9f5f-304535d22a78",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 219 — Stonehands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONEHANDS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stonehands",
    "d23fa1af-78e5-4d23-bbf6-cd62bc54b4e9",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 220 — Tor Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOR_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Tor Giant",
    "7ef8f279-1a10-4685-99d6-bc971a7f922b",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 221 — Total War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOTAL_WAR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Total War",
    "6107388b-ec1e-401e-a407-a821c908ed8d",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 222 — Vertigo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERTIGO: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Vertigo",
    "3067e7af-7bbd-48c1-9f1d-df2a91a0ec54",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 223 — Wall of Lava
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_LAVA: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wall of Lava",
    "b99d6d11-b3f7-4d73-967c-3049af82a9d8",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 224 — Word of Blasting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORD_OF_BLASTING: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Word of Blasting",
    "46b383c8-d604-4131-a869-9e9d13e30b94",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 225 — Aurochs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUROCHS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Aurochs",
    "7e973a84-7f7d-4524-9f2f-ec9a014d52ee",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 226 — Balduvian Bears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_BEARS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Balduvian Bears",
    "ef5297cb-e763-4871-9cd3-0e2dbcc52095",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 227 — Blizzard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIZZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Blizzard",
    "c369e4f9-0f2b-446c-9e2d-d3eefab0586d",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 228 — Brown Ouphe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROWN_OUPHE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Brown Ouphe",
    "e26ce35b-ba65-451d-a5ed-e1db6f1d0c6f",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 229 — Chub Toad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHUB_TOAD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Chub Toad",
    "b6ebcc1d-0c5c-4bc2-ade7-41944f69162e",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 230 — Dire Wolves
pub(in crate::card::sets) static DIRE_WOLVES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Dire Wolves",
    "a602c93d-e00f-4b4f-a7ff-95316b7e7641",
    "Ron Spencer",
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHLORE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Earthlore",
    "319d252e-7c43-47d6-8873-f69b0e063256",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 232 — Elder Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELDER_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Elder Druid",
    "210f6fab-62f0-42ab-bd01-00d647bd25e7",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 233 — Essence Filter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FILTER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Essence Filter",
    "9b610103-dafd-4248-9d79-ce57f84b9e03",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 234 — Fanatical Fever
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FANATICAL_FEVER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fanatical Fever",
    "2abba7f1-5d07-4137-88a2-5967396a3e42",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ICE 235 — Folk of the Pines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOLK_OF_THE_PINES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Folk of the Pines",
    "0c13311d-db83-483f-ba2b-4f54ceb8b026",
    "NéNé Thomas & Catherine Buck",
    crate::card::CardRules::unsupported(),
);

// ICE 236 — Forbidden Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDEN_LORE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Forbidden Lore",
    "5fc225cf-4fe2-4a5b-828e-ffcb99e404e8",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 237 — Forgotten Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_LORE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Forgotten Lore",
    "fb01dd39-a957-4c1a-86cf-f31a699a154a",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 238 — Foxfire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOXFIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Foxfire",
    "88db9685-6a2f-4548-b6c4-669918d653b4",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 239 — Freyalise Supplicant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_SUPPLICANT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Freyalise Supplicant",
    "5b1e718a-882a-4bdc-9d62-4dda88da0ba0",
    "Liz Danforth & Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 240 — Freyalise's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_S_CHARM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Freyalise's Charm",
    "3e147ac1-d221-49c7-966e-5e665ddeab6b",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 241 — Freyalise's Winds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_S_WINDS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Freyalise's Winds",
    "b11cd2e0-9419-4267-807e-5b73915c748a",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 242 — Fyndhorn Brownie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_BROWNIE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fyndhorn Brownie",
    "06204e82-9dfd-4334-a23a-f8240fc37772",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 243 — Fyndhorn Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fyndhorn Elder",
    "fca8aa11-f7cb-4f88-a041-30098579f1d2",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 244 — Fyndhorn Elves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_ELVES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fyndhorn Elves",
    "3ba95ffa-990a-4013-98b7-5d8c0b34e9c4",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// ICE 245 — Fyndhorn Pollen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_POLLEN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fyndhorn Pollen",
    "3efbe59d-bebc-40b1-85ac-2e4c1ff3731e",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 246 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "431c9749-fd7b-4960-a910-8d41d3704e6c",
    "Allen Williams",
);

// ICE 247 — Gorilla Pack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_PACK: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Gorilla Pack",
    "046f6b76-5f17-4728-aa34-72b7eff1d4c9",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 248 — Hot Springs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOT_SPRINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hot Springs",
    "1d4fe072-81a7-424e-8d21-aaca010d5b1d",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 249 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "a8cc6db7-1f40-40e3-a7ea-92f1d05e2e3d",
    "Cornelius Brudi",
);

// ICE 250 — Johtull Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOHTULL_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Johtull Wurm",
    "64a22e88-f7b1-48c8-a199-e57edcd50654",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 251 — Juniper Order Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNIPER_ORDER_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Juniper Order Druid",
    "cb211704-ff8e-498b-b7bb-f8384f198ffd",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 252 — Lhurgoyf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LHURGOYF: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lhurgoyf",
    "fee6d385-d44b-4f1a-beb1-13aeebde063e",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 253 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "87af69ee-c2bb-46ea-8d36-d484d04a3c8a",
    "Phil Foglio",
);

// ICE 254 — Maddening Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADDENING_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Maddening Wind",
    "5277656c-70f5-4660-bd58-7d9261d53fb5",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 255 — Nature's Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_LORE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Nature's Lore",
    "668d2969-b6b7-4507-bdd4-20bbaa68035a",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 256 — Pale Bears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALE_BEARS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pale Bears",
    "7f19c2a3-6403-4a78-bf45-6e339578d673",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 257 — Pygmy Allosaurus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_ALLOSAURUS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pygmy Allosaurus",
    "88a68767-9822-4f15-895e-32164e2159be",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 258 — Pyknite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYKNITE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pyknite",
    "6ffc64e4-ae3c-49f9-8ed6-518dd497bfe6",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 259 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "1dacfaec-6b61-450d-a134-2087c38a298a",
    "Justin Hampton",
);

// ICE 260 — Rime Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIME_DRYAD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Rime Dryad",
    "7a93e6ce-1295-41f8-b454-2dfe321481a6",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 261 — Ritual of Subdual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITUAL_OF_SUBDUAL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ritual of Subdual",
    "5c5c01e7-8116-45fc-afc3-d52a31a635cb",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// ICE 262 — Scaled Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALED_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Scaled Wurm",
    "499cd7fa-c86c-4a5f-b36d-8160e8a6af1f",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 263 — Shambling Strider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAMBLING_STRIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Shambling Strider",
    "8886ba2d-b25a-4b74-9299-911c509ae864",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 264 — Snowblind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWBLIND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snowblind",
    "5f62c376-487a-42bc-bd85-ab8b0480f7dc",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 265 — Stampede
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stampede",
    "bc8265a1-4621-4d25-8f7f-f0179951a694",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 266 — Stunted Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUNTED_GROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stunted Growth",
    "4c9b7393-eb35-4c99-bbf5-bcf924aa8ff3",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 267 — Tarpan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TARPAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Tarpan",
    "b1420ec5-367c-4514-86c5-3993bf339e37",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 268 — Thermokarst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THERMOKARST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Thermokarst",
    "00ae906b-2c4d-48e9-9f2d-217777e22292",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 269 — Thoughtleech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHTLEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Thoughtleech",
    "d8fe7f9d-644f-48d0-93fa-d9a536f1f755",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 270 — Tinder Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TINDER_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Tinder Wall",
    "2a7c6489-21e9-4b86-a54a-b1e2f1fce318",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 271 — Touch of Vitae
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCH_OF_VITAE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Touch of Vitae",
    "48d2cd18-a24d-40e0-a654-777d9e623ae2",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 272 — Trailblazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAILBLAZER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Trailblazer",
    "9194c69d-c849-4c4a-976c-d1382bd5cf32",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ICE 273 — Venomous Breath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_BREATH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Venomous Breath",
    "8eeb9e02-1d26-4959-a878-2ef8db2358bc",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 274 — Wall of Pine Needles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_PINE_NEEDLES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wall of Pine Needles",
    "5d879923-55fc-46ab-9306-5e1f10441c89",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 275 — Whiteout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHITEOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Whiteout",
    "a8645e4f-eaa8-4420-a6a3-eb53c311fab1",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 276 — Wiitigo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIITIGO: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wiitigo",
    "9ee86bf2-6c54-4c6e-8394-eb39f98d5a85",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 277 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "f8047ab9-a0fc-4933-bcbc-e761aa0f622b",
    "Mike Raabe",
);

// ICE 278 — Woolly Mammoths
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOOLLY_MAMMOTHS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Woolly Mammoths",
    "eaca1216-99c8-4ad5-a51a-3c4ff3b82097",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 279 — Woolly Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOOLLY_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Woolly Spider",
    "e10520b2-b5a7-4328-84c8-20443b6f588a",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 280 — Yavimaya Gnats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_GNATS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Yavimaya Gnats",
    "9d8b7020-ca8f-4867-bc51-13d824daf154",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 281 — Altar of Bone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALTAR_OF_BONE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Altar of Bone",
    "75d5b014-8675-4d91-a539-ac5c31d44b35",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 282 — Centaur Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_ARCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Centaur Archer",
    "e275c295-72da-4a86-82c6-cfd75b38b19c",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 283 — Chromatic Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMATIC_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Chromatic Armor",
    "2657e85b-8f77-41fa-9df2-233443efef43",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 284 — Diabolic Vision
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIABOLIC_VISION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Diabolic Vision",
    "1ea01324-1cfb-498c-8299-f690373864bd",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 285 — Earthlink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHLINK: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Earthlink",
    "a83cb1c4-7c5b-4a5e-b15e-138d644f5cdb",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 286 — Elemental Augury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEMENTAL_AUGURY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Elemental Augury",
    "62bbff2a-5109-400a-961b-eacffb9aed67",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 287 — Essence Vortex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_VORTEX: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Essence Vortex",
    "fe07e496-5070-4116-a91a-a3bbe19c12af",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 288 — Fiery Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERY_JUSTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fiery Justice",
    "8965ce61-0522-4f77-a82d-89441d1ba867",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 289 — Fire Covenant
pub(in crate::card::sets) static FIRE_COVENANT: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Fire Covenant",
    "6a0139c2-ad86-4c71-ab6d-4840c37d5d20",
    "Dan Frazier",
    // The life is paid as it is cast, so it is spent whether or not the
    // spell resolves -- and it is life, so nothing about the board caps how
    // much damage three mana can deal.
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "As an additional cost to cast this spell, pay X life. This spell deals X damage \
             divided as you choose among any number of target creatures.",
            // "Any number of target creatures" is however many shares X splits into,
            // and X is the life its caster was willing to spend rather than anything in
            // the mana cost -- three mana kills a board if you have the life for it.
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 0,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::ChosenX),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        )
        .with_spell_additional_cost(&SpellAdditionalCostDef::pay_life(CostQuantityDef::ChosenX)),
    ),
);

// ICE 290 — Flooded Woodlands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODED_WOODLANDS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Flooded Woodlands",
    "de89e9e1-485b-42e5-9728-5d6f948999e1",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 291 — Fumarole
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUMAROLE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fumarole",
    "efa53e9a-0d7c-4d17-b2be-56930edfa2c2",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 292 — Ghostly Flame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTLY_FLAME: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ghostly Flame",
    "6314344b-6493-4142-9c76-da9b90b8d3e1",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 293 — Giant Trap Door Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_TRAP_DOOR_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Giant Trap Door Spider",
    "8965dfa8-dc90-4cf2-a93b-72bf88b58936",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 294 — Glaciers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIERS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Glaciers",
    "b86e159b-ecf1-4b4a-9041-4e97fdf935e5",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 295 — Hymn of Rebirth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYMN_OF_REBIRTH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hymn of Rebirth",
    "61d0f2f2-f6e2-4b8a-8418-10b17c5e0ea9",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 296 — Kjeldoran Frostbeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_FROSTBEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Kjeldoran Frostbeast",
    "2fccb1d0-b324-4780-bb9e-4533240da06d",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 297 — Merieke Ri Berit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERIEKE_RI_BERIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Merieke Ri Berit",
    "3bf47c0a-5c17-47d0-b663-becff62fbdf8",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 298 — Monsoon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONSOON: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Monsoon",
    "254fcc50-79a5-40cd-b028-e78dde3f8480",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 299 — Mountain Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNTAIN_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Mountain Titan",
    "bcc1d589-02a2-4896-a283-9d0385534667",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 300 — Reclamation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECLAMATION: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Reclamation",
    "ca335f4f-d345-4eb9-9bc6-74595c501078",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 301 — Skeleton Ship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKELETON_SHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Skeleton Ship",
    "271c8a7c-0f71-4f9d-ab0e-ca7c8c4aca50",
    "Amy Weber & Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 302 — Spectral Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_SHIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Spectral Shield",
    "7fe0a783-d086-4dc8-ae4a-59f3c2daaca0",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 303 — Storm Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Storm Spirit",
    "7a383a5f-4814-4b92-aa80-2a6440a719bc",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 304 — Stormbind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMBIND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Stormbind",
    "c2d5d91b-aeb4-4d7e-b748-77f9960da55f",
    "NéNé Thomas & Phillip Mosness",
    crate::card::CardRules::unsupported(),
);

// ICE 305 — Wings of Aesthir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINGS_OF_AESTHIR: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wings of Aesthir",
    "eeb0282d-ccec-4556-8b70-b6f665077afe",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 306 — Adarkar Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADARKAR_SENTINEL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Adarkar Sentinel",
    "ff62754b-f4f0-4731-8dd7-327a820f60a8",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 307 — Aegis of the Meek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AEGIS_OF_THE_MEEK: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Aegis of the Meek",
    "5d272051-f442-4f6e-8c64-df28b398d2e8",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 308 — Amulet of Quoz
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMULET_OF_QUOZ: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Amulet of Quoz",
    "764ec6a8-a878-446c-b7e4-6026c2a3e9a4",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 309 — Arcum's Sleigh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_SLEIGH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Arcum's Sleigh",
    "e9780ce2-756c-48e5-9936-45f6a224f61d",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 310 — Arcum's Weathervane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_WEATHERVANE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Arcum's Weathervane",
    "9e142435-6930-4596-bc3b-60abde1229df",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 311 — Arcum's Whistle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_WHISTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Arcum's Whistle",
    "73c07c87-0e44-4a5a-92b7-728350cd02de",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 312 — Barbed Sextant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBED_SEXTANT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Barbed Sextant",
    "edb82654-de12-4dce-8c6b-f28d68f0fbe1",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 313 — Baton of Morale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATON_OF_MORALE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Baton of Morale",
    "8bc29872-b1a2-4851-9eca-f3e67ae6e14c",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 314 — Celestial Sword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_SWORD: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Celestial Sword",
    "2bc0e8d3-633b-4281-863f-c51c69eed0b6",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 315 — Crown of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_THE_AGES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Crown of the Ages",
    "fce2991f-48e1-4cfe-af0a-18b6d9400493",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 316 — Despotic Scepter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPOTIC_SCEPTER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Despotic Scepter",
    "53e381a4-810e-4b75-aed3-c16cf0eb06fa",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 317 — Elkin Bottle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELKIN_BOTTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Elkin Bottle",
    "49301c19-55a0-4146-9474-0b86cd320e31",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 318 — Fyndhorn Bow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_BOW: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Fyndhorn Bow",
    "65dd0a41-cc51-4728-b597-fdb2510accd8",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 319 — Goblin Lyre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_LYRE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Goblin Lyre",
    "951114fb-5ae5-4eb0-8e03-6e39b0b634b5",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// ICE 320 — Hematite Talisman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEMATITE_TALISMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Hematite Talisman",
    "83585337-56a9-44d2-9ed1-8a959bcfb010",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 321 — Ice Cauldron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CAULDRON: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ice Cauldron",
    "1a3e095a-7056-4df3-bf7d-9c217d591446",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 322 — Icy Manipulator (reprint)
const ICY_MANIPULATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ICY_MANIPULATOR,
    "1eda936f-7691-4440-9b83-eb0c6035b109",
    "Amy Weber",
);

// ICE 323 — Infinite Hourglass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFINITE_HOURGLASS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Infinite Hourglass",
    "f9a42152-32c0-47ff-aaac-8deaf01873ca",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 324 — Jester's Cap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESTER_S_CAP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Jester's Cap",
    "47ac44d0-8090-4e7b-ac47-c567294f185e",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 325 — Jester's Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESTER_S_MASK: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Jester's Mask",
    "daa1ba0c-cb89-4bb2-8a35-6a4a4eecccf7",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 326 — Jeweled Amulet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEWELED_AMULET: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Jeweled Amulet",
    "34f7bad2-d28f-42d2-9246-fe3545ef49a7",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 327 — Lapis Lazuli Talisman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAPIS_LAZULI_TALISMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lapis Lazuli Talisman",
    "ce00bb19-983e-427d-be54-ae6daf0ccdde",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 328 — Malachite Talisman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALACHITE_TALISMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Malachite Talisman",
    "63fb8a24-ce53-4a69-be2a-55c6dbba5ee7",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 329 — Nacre Talisman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NACRE_TALISMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Nacre Talisman",
    "06912236-8225-4eb0-8086-c6a163c69892",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 330 — Naked Singularity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAKED_SINGULARITY: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Naked Singularity",
    "cabadfb2-93cd-4c7a-b901-59c3dd1a7c3c",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 331 — Onyx Talisman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ONYX_TALISMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Onyx Talisman",
    "a89b2368-1180-4821-bcb8-8161c18e5538",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 332 — Pentagram of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PENTAGRAM_OF_THE_AGES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pentagram of the Ages",
    "b8d889a5-f6c7-410d-97f9-acf08b9091c8",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 333 — Pit Trap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PIT_TRAP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Pit Trap",
    "c588fe7f-945d-4459-904c-67442f88b4e1",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 334 — Runed Arch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNED_ARCH: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Runed Arch",
    "ca02861b-9639-480d-8e54-e024f0c70158",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 335 — Shield of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_OF_THE_AGES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Shield of the Ages",
    "7411ab40-47f6-44d1-8e33-9ff5301dcd9b",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 336 — Skull Catapult
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULL_CATAPULT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Skull Catapult",
    "eb92a3e6-dc30-4a08-baba-e125290cadc5",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ICE 337 — Snow Fortress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_FORTRESS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow Fortress",
    "1c480e07-fb26-4760-865f-47985f7447bb",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 338 — Soldevi Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Soldevi Golem",
    "64d35e88-81d3-4a54-aa79-190615abc616",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 339 — Soldevi Simulacrum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_SIMULACRUM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Soldevi Simulacrum",
    "9fabc7b6-e766-4e3c-816e-04cfeceaff09",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 340 — Staff of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAFF_OF_THE_AGES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Staff of the Ages",
    "5c709836-55b6-4de9-b190-b5f66dc53c87",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 341 — Sunstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSTONE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Sunstone",
    "3c1c67fa-ff88-4a61-b8a5-8a872b3dc44f",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 342 — Time Bomb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_BOMB: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Time Bomb",
    "092ec691-4729-46d3-a4e2-0cfc5df42a31",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 343 — Urza's Bauble
pub(in crate::card::sets) static URZAS_BAUBLE: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Urza's Bauble",
    "58c9e9a7-e170-4361-b7d5-22fc0771c489",
    "Christopher Rush",
    // A free artifact that replaces itself a turn later, which is why the
    // decks that count artifacts or graveyard cards play it for no other
    // reason.
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Look at a card at random in target player's hand. You draw \
         a card at the beginning of the next turn's upkeep.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::LookAtRandomCardInHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            // "You draw a card at the beginning of the next turn's upkeep": a delayed
            // draw rather than a cantrip, which is what makes the Bauble free to play
            // and slow to pay.
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next turn's upkeep, you draw a card.",
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

// ICE 344 — Vexing Arcanix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEXING_ARCANIX: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Vexing Arcanix",
    "0c9ea118-6a19-4e1b-aa5a-9b2729efc096",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 345 — Vibrating Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIBRATING_SPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Vibrating Sphere",
    "48f93ded-ecf6-4a70-8ca3-a9c0c3201c21",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 346 — Walking Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Walking Wall",
    "cba1238c-1969-452d-8112-124cbbd49417",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 347 — Wall of Shields
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_SHIELDS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Wall of Shields",
    "6376c7c4-aaca-4625-83d4-a49f01aec535",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 348 — War Chariot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_CHARIOT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "War Chariot",
    "d0ea0c6c-aa76-4b16-bc99-2ff46dc56d4e",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 349 — Whalebone Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHALEBONE_GLIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Whalebone Glider",
    "4b75adf0-9501-4776-a213-456c2b821070",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 350 — Zuran Orb
pub(in crate::card::sets) static ZURAN_ORB: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Zuran Orb",
    "3a9d1082-a862-45d4-9e5e-392e879fead6",
    "Sandra Everingham",
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
pub(in crate::card::sets) static ADARKAR_WASTES: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Adarkar Wastes",
    "09dd9023-f7ee-4e99-8821-7059deb83730",
    "Mike Raabe",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {U}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Blue],
    )),
);

// ICE 352 — Brushland
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRUSHLAND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Brushland",
    "170e5ccd-54bf-4c6d-86b4-0359ca8f36e8",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ICE 353 — Glacial Chasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_CHASM: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Glacial Chasm",
    "3d23f800-7a6f-40e3-b242-9f5955e47a75",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 354 — Halls of Mist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALLS_OF_MIST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Halls of Mist",
    "b926a189-90b6-47bb-b5d6-b033e57007b4",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 355 — Ice Floe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_FLOE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Ice Floe",
    "85ce04fb-e687-41e0-ae9a-16a51df5d943",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 356 — Karplusan Forest
pub(in crate::card::sets) static KARPLUSAN_FOREST: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Karplusan Forest",
    "ba6f1263-d598-49fb-b5f8-09f11822ebd0",
    "Nicola Leonard",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {R} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Red, ManaColor::Green],
    )),
);

// ICE 357 — Land Cap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAND_CAP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Land Cap",
    "c4806c02-7a4d-42e3-affd-0338084bd3ab",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 358 — Lava Tubes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_TUBES: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Lava Tubes",
    "5e7c2cf6-f36f-451b-bba5-19a82c659c4c",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ICE 359 — River Delta
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVER_DELTA: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "River Delta",
    "ea335fc0-0591-4acd-9ae8-7858222770da",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 360 — Sulfurous Springs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUROUS_SPRINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Sulfurous Springs",
    "2fdeab50-b45f-412b-85a3-c6cf009ce567",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 361 — Timberline Ridge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMBERLINE_RIDGE: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Timberline Ridge",
    "87cc2fc9-0a24-4ac1-afcc-9317b90c7178",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 362 — Underground River
pub(in crate::card::sets) static UNDERGROUND_RIVER: CardRecord = CardRecord::new(
    CardSet::IceAge,
    "Underground River",
    "92369d7e-5e5a-46f9-bb31-c57d62410283",
    "NéNé Thomas",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {U} or {B}. This land deals 1 damage to you.",
        &[ManaColor::Blue, ManaColor::Black],
    )),
);

// ICE 363 — Veldt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VELDT: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Veldt",
    "987534fb-74a9-46a3-805f-fe2fe2df4a90",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ICE 364 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "7b68bdb0-41cc-48f6-905e-7da1ff4ba5e0",
    "Christopher Rush",
);

// ICE 365 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "df3e94f7-9f97-4652-a1f1-381feb15f688",
    "Christopher Rush",
);

// ICE 366 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "27ac1fc7-0698-4a94-8353-cc4c13bd6ffa",
    "Christopher Rush",
);

// ICE 367 — Snow-Covered Plains
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_PLAINS: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow-Covered Plains",
    "cb3ac778-fb45-4fd3-a9af-8a0791f833e8",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 368 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "ef2d6fc9-ddad-4dd2-b218-afa1a5449b7e",
    "Anson Maddocks",
);

// ICE 369 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "61a467ab-4460-4e5e-94c1-8150bfe0c954",
    "Anson Maddocks",
);

// ICE 370 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "82f11c42-9d67-4833-9519-e165e6a7e9c4",
    "Anson Maddocks",
);

// ICE 371 — Snow-Covered Island
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_ISLAND: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow-Covered Island",
    "ad8b77cf-b53e-4da3-9c27-3851b7b25a98",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 372 — Snow-Covered Swamp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_SWAMP: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow-Covered Swamp",
    "65a3c27f-6b15-49b6-ac89-36cfb79b3b54",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 373 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "4695653a-5c4c-4ff3-b80c-f4b6c685f370",
    "Douglas Shuler",
);

// ICE 374 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "6a90b49f-53b3-4ce0-92c1-bcd76d6981ea",
    "Douglas Shuler",
);

// ICE 375 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "ddca7e2e-bb0a-47ed-ade3-31900da992dc",
    "Douglas Shuler",
);

// ICE 376 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "4ecf39c3-3b5f-4263-a7b5-9881bded3494",
    "Tom Wänerstrand",
);

// ICE 377 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "2eb15b42-be2a-4663-b064-aad6c7cb2714",
    "Tom Wänerstrand",
);

// ICE 378 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "17ac61e4-b543-4c37-9bfa-43f0c928152d",
    "Tom Wänerstrand",
);

// ICE 379 — Snow-Covered Mountain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_MOUNTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow-Covered Mountain",
    "ccd3afb3-5574-4f2d-adbe-969a428f1c63",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 380 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "fbdcbd97-90a9-45ea-94f6-2a1c6faaf965",
    "Pat Lewis",
);

// ICE 381 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "b346b784-7bde-49d0-bfa9-56236cbe19d9",
    "Pat Lewis",
);

// ICE 382 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "768c4d8f-5700-4f0a-9ff2-58422aeb1dac",
    "Pat Lewis",
);

// ICE 383 — Snow-Covered Forest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_COVERED_FOREST: CardRecord = CardRecord::new(
    crate::card::CardSet::IceAge,
    "Snow-Covered Forest",
    "4c0ad95c-d62c-4138-ada0-fa39a63a449e",
    "Pat Lewis",
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
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    DEATH_WARD_REPRINT,
    DISENCHANT_REPRINT,
    SWORDS_TO_PLOWSHARES_REPRINT,
    COUNTERSPELL_REPRINT,
    POWER_SINK_REPRINT,
    SLEIGHT_OF_MIND_REPRINT,
    DARK_RITUAL_REPRINT,
    FEAR_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    SHATTER_REPRINT,
    STONE_RAIN_REPRINT,
    GIANT_GROWTH_REPRINT,
    HURRICANE_REPRINT,
    LURE_REPRINT,
    REGENERATION_REPRINT,
    WILD_GROWTH_REPRINT,
    ICY_MANIPULATOR_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
];
