//! Chronicles has no unique card definitions.
//!
//! It reprinted cards from the sets before it, which is what brings some of
//! them inside the Premodern window.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1994::the_dark as catalog_drk;
use crate::card::sets::y1994::the_dark;

// CHR 1 — Abu Ja'far (reprint)
const ABU_JAFAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ABU_JAFAR,
    "023b5e6f-10de-422d-8431-11f1fdeca246",
    "Ken Meyer, Jr.",
);

// CHR 2 — Akron Legionnaire (reprint)
const AKRON_LEGIONNAIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AKRON_LEGIONNAIRE,
    "359e49df-aef4-4bce-abd6-a61f821faedf",
    "Mark Poole",
);

// CHR 3 — Angelic Voices (reprint)
const ANGELIC_VOICES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ANGELIC_VOICES,
    "1e9a66b2-7392-4030-a3a8-8f3697307a39",
    "Julie Baroh",
);

// CHR 4 — Blood of the Martyr (reprint)
const BLOOD_OF_THE_MARTYR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BLOOD_OF_THE_MARTYR,
    "823b637b-e089-479a-88c0-be014516429a",
    "Christopher Rush",
);

// CHR 5 — D'Avenant Archer (reprint)
const DAVENANT_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DAVENANT_ARCHER,
    "6868c8aa-1784-4944-bef2-7909de64a98d",
    "Douglas Shuler",
);

// CHR 6 — Divine Offering (reprint)
const DIVINE_OFFERING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DIVINE_OFFERING,
    "9371c4aa-424f-4b89-9f73-239a9e4f8a9d",
    "Jeff A. Menges",
);

// CHR 7 — Indestructible Aura (reprint)
const INDESTRUCTIBLE_AURA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::INDESTRUCTIBLE_AURA,
    "0397a4f3-6d7c-43d1-9fc2-c0eaf780ecb0",
    "Mark Poole",
);

// CHR 8 — Ivory Guardians (reprint)
const IVORY_GUARDIANS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::IVORY_GUARDIANS,
    "b7033d2f-45be-42bc-830a-dc3b5b27c366",
    "Melissa A. Benson",
);

// CHR 9 — Keepers of the Faith (reprint)
const KEEPERS_OF_THE_FAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KEEPERS_OF_THE_FAITH,
    "a3d1ad50-c60c-46a9-b2dc-5cd2680d7263",
    "Daniel Gelon",
);

// CHR 10 — Petra Sphinx (reprint)
const PETRA_SPHINX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PETRA_SPHINX,
    "d2b3574a-e294-47b9-a543-5a65358ebcdc",
    "Sandra Everingham",
);

// CHR 11 — Repentant Blacksmith (reprint)
const REPENTANT_BLACKSMITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::REPENTANT_BLACKSMITH,
    "98270418-5d24-46d3-a235-8956a3a34d21",
    "Drew Tucker",
);

// CHR 12 — Shield Wall (reprint)
const SHIELD_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SHIELD_WALL,
    "b97ea2e6-9bc7-4eb8-a23b-4b23d507865e",
    "Douglas Shuler",
);

// CHR 13 — War Elephant (reprint)
const WAR_ELEPHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::WAR_ELEPHANT,
    "709cd715-28ef-42e8-bf8c-fc51a1ca0c6a",
    "Kristen Bishop",
);

// CHR 14 — Witch Hunter (reprint)
const WITCH_HUNTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::WITCH_HUNTER,
    "4dc64378-3495-42d7-bf14-faa969f62583",
    "Jesper Myrfors",
);

// CHR 15 — Azure Drake (reprint)
const AZURE_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AZURE_DRAKE,
    "c511ae41-05c0-4fb2-b74b-f27f2a4475be",
    "Dan Frazier",
);

// CHR 16 — Boomerang (reprint)
const BOOMERANG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BOOMERANG,
    "5b27431c-e301-4581-b67d-9213edee6ebb",
    "Brian Snõddy",
);

// CHR 17 — Dance of Many (reprint)
const DANCE_OF_MANY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::DANCE_OF_MANY,
    "ab4bf154-a611-49f1-9d1c-f247ab12c52f",
    "Sandra Everingham",
);

// CHR 18 — Dandân (reprint)
const DANDAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DANDAN,
    "bfc43585-55ac-4d58-9e80-b19a7c8c8662",
    "Drew Tucker",
);

// CHR 19 — Enchantment Alteration (reprint)
const ENCHANTMENT_ALTERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ENCHANTMENT_ALTERATION,
    "f94c7a9a-4b7e-438e-9f8f-b22a9e757f4f",
    "Brian Snõddy",
);

// CHR 20 — Fishliver Oil (reprint)
const FISHLIVER_OIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::FISHLIVER_OIL,
    "cf25fe38-e50d-423a-8781-96e506c29d52",
    "Anson Maddocks",
);

// CHR 21 — Flash Flood (reprint)
const FLASH_FLOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FLASH_FLOOD,
    "721aa268-be3b-4d67-a715-c5b86c3b414e",
    "Tom Wänerstrand",
);

// CHR 22 — Juxtapose (reprint)
const JUXTAPOSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::JUXTAPOSE,
    "fe9dc8dd-6212-465b-9633-f99cebf492f9",
    "Justin Hampton",
);

// CHR 23 — Puppet Master (reprint)
const PUPPET_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PUPPET_MASTER,
    "b401d253-4e51-42db-91cc-ac8cc3d06ae6",
    "Sandra Everingham",
);

// CHR 24 — Recall (reprint)
const RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RECALL,
    "84b8de7a-c055-4031-a359-24526b6e6354",
    "Brian Snõddy",
);

// CHR 25 — Remove Soul (reprint)
const REMOVE_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::REMOVE_SOUL,
    "e0fa83df-57e8-4e4e-95af-9557869a34a3",
    "Brian Snõddy",
);

// CHR 26 — Teleport (reprint)
const TELEPORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TELEPORT,
    "39c65d23-4bc5-4f79-a0e5-0cd4b2660f96",
    "Douglas Shuler",
);

// CHR 27 — Wall of Vapor (reprint)
const WALL_OF_VAPOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_VAPOR,
    "309c1b2a-0230-4b66-84a0-32b8cd6d31eb",
    "Richard Thomas",
);

// CHR 28 — Wall of Wonder (reprint)
const WALL_OF_WONDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_WONDER,
    "14c65a4e-ea5a-4cad-9542-1cfd18eb3c25",
    "Richard Thomas",
);

// CHR 29 — Banshee (reprint)
const BANSHEE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BANSHEE,
    "70a68d3f-fd2a-475f-8d17-6a62e773a312",
    "Jesper Myrfors",
);

// CHR 30 — Bog Rats (reprint)
const BOG_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_RATS,
    "94c57803-d350-4b26-a867-4c2484da303a",
    "Ron Spencer",
);

// CHR 31 — Cuombajj Witches (reprint)
const CUOMBAJJ_WITCHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::CUOMBAJJ_WITCHES,
    "6f32b7f8-989d-4ffe-8279-57c6b6848a32",
    "Kaja Foglio",
);

// CHR 32 — Fallen Angel (reprint)
const FALLEN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FALLEN_ANGEL,
    "47e79d10-5561-41f7-b6d7-ab7560a9ff23",
    "Anson Maddocks",
);

// CHR 33 — Giant Slug (reprint)
const GIANT_SLUG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GIANT_SLUG,
    "d78999ab-2ccc-41ec-b808-18e40702d1c3",
    "Anson Maddocks",
);

// CHR 34 — Hasran Ogress (reprint)
const HASRAN_OGRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::HASRAN_OGRESS,
    "b41923d3-1420-49fc-873e-6290b1a80248",
    "Dan Frazier",
);

// CHR 35 — Hell's Caretaker (reprint)
const HELLS_CARETAKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::HELLS_CARETAKER,
    "ce4004fe-e8ed-4806-878b-34ee3ed82016",
    "Sandra Everingham",
);

// CHR 36 — Shimian Night Stalker (reprint)
const SHIMIAN_NIGHT_STALKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SHIMIAN_NIGHT_STALKER,
    "9caf87f7-36d5-478b-9836-52043833290f",
    "Jesper Myrfors",
);

// CHR 37 — Takklemaggot (reprint)
const TAKKLEMAGGOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TAKKLEMAGGOT,
    "a596069b-864b-48fb-a097-bf5e6cafbcce",
    "Daniel Gelon",
);

// CHR 38 — The Fallen (reprint)
const THE_FALLEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::THE_FALLEN,
    "262053e7-09b7-4cfc-9959-f8ab7c8149d8",
    "Jesper Myrfors",
);

// CHR 39 — The Wretched (reprint)
const THE_WRETCHED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::THE_WRETCHED,
    "016d00a2-a75f-4394-b70d-8affa7691674",
    "Christopher Rush",
);

// CHR 40 — Transmutation (reprint)
const TRANSMUTATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TRANSMUTATION,
    "e2d90519-68f8-43ab-902a-0fed0f526488",
    "Susan Van Camp",
);

// CHR 41 — Wall of Shadows (reprint)
const WALL_OF_SHADOWS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_SHADOWS,
    "69c6e076-d7bf-435b-ba79-84aa9f073130",
    "Pete Venters",
);

// CHR 42 — Yawgmoth Demon (reprint)
const YAWGMOTH_DEMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::YAWGMOTH_DEMON,
    "db9b9a95-57f5-46cb-a429-f1b4e5cdbdc4",
    "Sandra Everingham",
);

// CHR 43 — Active Volcano (reprint)
const ACTIVE_VOLCANO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ACTIVE_VOLCANO,
    "56adb184-f261-40e1-bdf3-e85d7d13faa2",
    "Justin Hampton",
);

// CHR 44 — Aladdin (reprint)
const ALADDIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDIN,
    "f1681ad7-c5ce-4f11-af53-cf2882637ba7",
    "Julie Baroh",
);

// CHR 45 — Beasts of Bogardan (reprint)
const BEASTS_OF_BOGARDAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BEASTS_OF_BOGARDAN,
    "cec0fe2c-e7e6-42d1-8128-58d70a7f1177",
    "Daniel Gelon",
);

// CHR 46 — Blood Moon (reprint)
const BLOOD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BLOOD_MOON,
    "49673567-c358-4de8-99eb-1a61b7198d46",
    "Tom Wänerstrand",
);

// CHR 47 — Fire Drake (reprint)
const FIRE_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FIRE_DRAKE,
    "69b0b840-bef0-47dc-aacf-aab59a3e3632",
    "Christopher Rush",
);

// CHR 48 — Goblin Artisans (reprint)
const GOBLIN_ARTISANS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::GOBLIN_ARTISANS,
    "b4e8d779-ef6f-4b09-870e-f1fdeac83e32",
    "Julie Baroh",
);

// CHR 49 — Goblin Digging Team (reprint)
const GOBLIN_DIGGING_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_DIGGING_TEAM,
    "bc8fdbaa-bf48-4ed9-b3fd-c2c5d497a0be",
    "Ron Spencer",
);

// CHR 50 — Goblin Shrine (reprint)
const GOBLIN_SHRINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_SHRINE,
    "12711fa3-455d-498a-a1e9-881c62e9bb4f",
    "Ron Spencer",
);

// CHR 51 — Goblins of the Flarg (reprint)
const GOBLINS_OF_THE_FLARG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLINS_OF_THE_FLARG,
    "d885f124-2c0c-4321-9434-19dcdcdb907a",
    "Tom Wänerstrand",
);

// CHR 52 — Land's Edge (reprint)
const LAND_S_EDGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::LAND_S_EDGE,
    "41798dd9-8ce8-4642-89c2-7356ea129d4e",
    "Brian Snõddy",
);

// CHR 53 — Mountain Yeti (reprint)
const MOUNTAIN_YETI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::MOUNTAIN_YETI,
    "fe91cca2-e44d-4b62-80d5-248c126ad635",
    "Dan Frazier",
);

// CHR 54 — Primordial Ooze (reprint)
const PRIMORDIAL_OOZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PRIMORDIAL_OOZE,
    "dab748e8-5917-4c7f-8d5f-18dadcb82abe",
    "Sandra Everingham",
);

// CHR 55 — Wall of Heat (reprint)
const WALL_OF_HEAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_HEAT,
    "ff6b2307-2c56-4f63-900e-88a3ac6f0b32",
    "Richard Thomas",
);

// CHR 56 — Wall of Opposition (reprint)
const WALL_OF_OPPOSITION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_OPPOSITION,
    "23243752-25c5-407e-9ce4-4bc1f02d01c1",
    "Harold McNeill",
);

// CHR 57 — Argothian Pixies (reprint)
const ARGOTHIAN_PIXIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ARGOTHIAN_PIXIES,
    "78276d53-7c01-45b4-a136-a9f3674e1f26",
    "Amy Weber",
);

// CHR 58 — Cat Warriors (reprint)
const CAT_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CAT_WARRIORS,
    "d927c073-0198-4a32-b21a-ead7d8ac116f",
    "Melissa A. Benson",
);

// CHR 59 — Cocoon (reprint)
const COCOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::COCOON,
    "897de61e-440f-4eaf-aef8-0dd1c6117288",
    "Mark Tedin",
);

// CHR 60 — Concordant Crossroads (reprint)
const CONCORDANT_CROSSROADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CONCORDANT_CROSSROADS,
    "13cfca5f-e25a-432e-8679-54704eeeecc8",
    "Amy Weber",
);

// CHR 61 — Craw Giant (reprint)
const CRAW_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CRAW_GIANT,
    "cee9ec10-d96a-4c8f-a56a-3435f3f385d6",
    "Christopher Rush",
);

// CHR 62 — Cyclone (reprint)
const CYCLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::CYCLONE,
    "ff67fbf5-7935-41e2-a196-7e2917a63d09",
    "Mark Tedin",
);

// CHR 63 — Emerald Dragonfly (reprint)
const EMERALD_DRAGONFLY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::EMERALD_DRAGONFLY,
    "931f2093-4d43-449e-82e8-beb97d25a0e8",
    "Quinton Hoover",
);

// CHR 64 — Erhnam Djinn (reprint)
const ERHNAM_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ERHNAM_DJINN,
    "d2d5bfa6-243f-4178-871b-f5a19b4024c1",
    "Ken Meyer, Jr.",
);

// CHR 65 — Ghazbán Ogre (reprint)
const GHAZBAN_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::GHAZBAN_OGRE,
    "85bcd723-780b-45ca-9476-d28270350013",
    "Jesper Myrfors",
);

// CHR 66 — Metamorphosis (reprint)
const METAMORPHOSIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::METAMORPHOSIS,
    "fc73bd94-6e14-4798-b9ff-163ba7bdd663",
    "Christopher Rush",
);

// CHR 67 — Rabid Wombat (reprint)
const RABID_WOMBAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RABID_WOMBAT,
    "1f29a24b-0c45-4192-a855-4c414c016bb8",
    "Kaja Foglio",
);

// CHR 68 — Revelation (reprint)
const REVELATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::REVELATION,
    "d467517a-1e6f-4c1f-adb5-bf60df1284e2",
    "Kaja Foglio",
);

// CHR 69 — Scavenger Folk (reprint)
const SCAVENGER_FOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::SCAVENGER_FOLK,
    "8a3ebda6-7eea-4a65-9fc1-9fc453ff39bb",
    "Dennis Detwiller",
);

// CHR 70 — Storm Seeker (reprint)
const STORM_SEEKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::STORM_SEEKER,
    "e6166e9b-f119-4b38-aba3-a9e8007704e6",
    "Mark Poole",
);

// CHR 71 — Arcades Sabboth (reprint)
const ARCADES_SABBOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ARCADES_SABBOTH,
    "d20a0b5a-2a04-4cce-83e4-2d65ef9399df",
    "Edward P. Beard, Jr.",
);

// CHR 72 — Axelrod Gunnarson (reprint)
const AXELROD_GUNNARSON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AXELROD_GUNNARSON,
    "a9c5dbc6-b0c5-4ae2-8c23-a26524fee543",
    "Scott Kirschner",
);

// CHR 73 — Ayesha Tanaka (reprint)
const AYESHA_TANAKA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AYESHA_TANAKA,
    "8ce912d9-406b-4eba-97be-3bf1d425ee05",
    "Bryon Wackwitz",
);

// CHR 74 — Chromium (reprint)
const CHROMIUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CHROMIUM,
    "e85e54f3-012b-460f-a6f9-d0242a174adc",
    "Edward P. Beard, Jr.",
);

// CHR 75 — Dakkon Blackblade (reprint)
const DAKKON_BLACKBLADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DAKKON_BLACKBLADE,
    "4874388e-0227-4b89-a986-d86c14482c81",
    "Richard Kane Ferguson",
);

// CHR 76 — Gabriel Angelfire (reprint)
const GABRIEL_ANGELFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GABRIEL_ANGELFIRE,
    "f2a26496-b4c9-4a29-9a85-26e217deafa2",
    "Daniel Gelon",
);

// CHR 77 — Johan (reprint)
const JOHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::JOHAN,
    "2f2f3b3e-63f3-4cab-aa95-030990157ed5",
    "Mark Tedin",
);

// CHR 78 — Kei Takahashi (reprint)
const KEI_TAKAHASHI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KEI_TAKAHASHI,
    "d282cac6-2cc6-4384-a4b9-06763eb4a706",
    "Scott Kirschner",
);

// CHR 79 — Marhault Elsdragon (reprint)
const MARHAULT_ELSDRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::MARHAULT_ELSDRAGON,
    "b2617094-1216-4541-8057-34568defe3a1",
    "Mark Poole",
);

// CHR 80 — Nebuchadnezzar (reprint)
const NEBUCHADNEZZAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::NEBUCHADNEZZAR,
    "76a0b7cc-0889-4980-a23e-6f5088b374f8",
    "Richard Kane Ferguson",
);

// CHR 81 — Nicol Bolas (reprint)
const NICOL_BOLAS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::NICOL_BOLAS,
    "05362cde-3b0b-4a8d-9875-675cfac52e4a",
    "Edward P. Beard, Jr.",
);

// CHR 82 — Palladia-Mors (reprint)
const PALLADIA_MORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PALLADIA_MORS,
    "0ea81883-7cd6-4443-9733-39d25cc64328",
    "Edward P. Beard, Jr.",
);

// CHR 83 — Rubinia Soulsinger (reprint)
const RUBINIA_SOULSINGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RUBINIA_SOULSINGER,
    "c87cecd6-ea9d-4515-890c-262bb15e9f37",
    "Rob Alexander",
);

// CHR 84 — Sivitri Scarzam (reprint)
const SIVITRI_SCARZAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SIVITRI_SCARZAM,
    "58ab11d1-5536-4a4d-a324-bce236358f58",
    "NéNé Thomas",
);

// CHR 85 — Sol'kanar the Swamp King (reprint)
const SOLKANAR_THE_SWAMP_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SOLKANAR_THE_SWAMP_KING,
    "c7ea7454-c715-4d6d-8e46-cf5acaf397f5",
    "Richard Kane Ferguson",
);

// CHR 86 — Stangg (reprint)
const STANGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::STANGG,
    "4e9df135-4449-42ff-868b-56b1f702aca6",
    "Mark Poole",
);

// CHR 87 — Tobias Andrion (reprint)
const TOBIAS_ANDRION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TOBIAS_ANDRION,
    "e298291d-8e96-4676-8b6f-daf6fd290b57",
    "Andi Rusu",
);

// CHR 88 — Tor Wauki (reprint)
const TOR_WAUKI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TOR_WAUKI,
    "eef9d043-c9ab-4215-acfc-9065250513d1",
    "Randy Asplund-Faith",
);

// CHR 89 — Vaevictis Asmadi (reprint)
const VAEVICTIS_ASMADI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::VAEVICTIS_ASMADI,
    "ac77e867-939e-49fb-ae1a-9c1dd5e54a9a",
    "Andi Rusu",
);

// CHR 90 — Xira Arien (reprint)
const XIRA_ARIEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::XIRA_ARIEN,
    "21fa636b-8ce8-40b6-a4d0-3191a664bd92",
    "Melissa A. Benson",
);

// CHR 91 — Arena of the Ancients (reprint)
const ARENA_OF_THE_ANCIENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ARENA_OF_THE_ANCIENTS,
    "52bf69ca-b974-4620-8483-b0ba33db26a5",
    "Tom Wänerstrand",
);

// CHR 92 — Ashnod's Altar (reprint)
const ASHNOD_S_ALTAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ASHNOD_S_ALTAR,
    "d5e1a75e-1369-4f28-bb7d-e7ea2e6087e9",
    "Anson Maddocks",
);

// CHR 93 — Ashnod's Transmogrant (reprint)
const ASHNODS_TRANSMOGRANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ASHNODS_TRANSMOGRANT,
    "c820cde8-ee7e-4654-afb3-cd0ee05f2635",
    "Mark Tedin",
);

// CHR 94 — Barl's Cage (reprint)
const BARLS_CAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BARLS_CAGE,
    "913d734e-8788-42fd-849f-ac46b40ffa10",
    "Tom Wänerstrand",
);

// CHR 95 — Book of Rass (reprint)
const BOOK_OF_RASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOOK_OF_RASS,
    "820a9c79-5e0b-4959-9ff0-0dc6f2f632c5",
    "Sandra Everingham",
);

// CHR 96 — Bronze Horse (reprint)
const BRONZE_HORSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BRONZE_HORSE,
    "e52122a1-e204-4299-b889-609df93b36fa",
    "Mark Poole",
);

// CHR 97 — Feldon's Cane (reprint)
const FELDONS_CANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::FELDONS_CANE,
    "38ae4abb-f16e-4433-ac1a-e4be57dc652b",
    "Mark Tedin",
);

// CHR 98 — Fountain of Youth (reprint)
const FOUNTAIN_OF_YOUTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FOUNTAIN_OF_YOUTH,
    "813df533-934f-416d-b99b-4951431a8e1d",
    "Daniel Gelon",
);

// CHR 99 — Gauntlets of Chaos (reprint)
const GAUNTLETS_OF_CHAOS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GAUNTLETS_OF_CHAOS,
    "6279176a-3b27-4be7-878c-2745941e12d6",
    "Dan Frazier",
);

// CHR 100 — Horn of Deafening (reprint)
const HORN_OF_DEAFENING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::HORN_OF_DEAFENING,
    "412848e0-dcbb-4b6e-87cf-64886bd5e456",
    "Dan Frazier",
);

// CHR 101 — Jalum Tome (reprint)
const JALUM_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::JALUM_TOME,
    "627ecfc9-faea-4d52-b0fb-ec0895001fd4",
    "Tom Wänerstrand",
);

// CHR 102 — Jeweled Bird (reprint)
const JEWELED_BIRD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JEWELED_BIRD,
    "d78e61e5-9313-4c83-af38-98849aea4e42",
    "Amy Weber",
);

// CHR 103 — Living Armor (reprint)
const LIVING_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::LIVING_ARMOR,
    "b9da41f7-c1d2-4d5c-b6a5-09f6cc05cd4a",
    "Anson Maddocks",
);

// CHR 104 — Obelisk of Undoing (reprint)
const OBELISK_OF_UNDOING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::OBELISK_OF_UNDOING,
    "73159a98-4400-4ae8-ba7c-26f534de8b84",
    "Tom Wänerstrand",
);

// CHR 105 — Rakalite (reprint)
const RAKALITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::RAKALITE,
    "d436239d-ea38-497b-bcdd-9bfa41cb21e5",
    "Christopher Rush",
);

// CHR 106 — Runesword (reprint)
const RUNESWORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::RUNESWORD,
    "a11ec2f6-a0c7-4164-a562-fab7aab7d211",
    "Christopher Rush",
);

// CHR 107 — Sentinel (reprint)
const SENTINEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SENTINEL,
    "ffd4921f-cda5-4318-837d-a3fe4f0d9362",
    "Randy Asplund-Faith",
);

// CHR 108 — Serpent Generator (reprint)
const SERPENT_GENERATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SERPENT_GENERATOR,
    "3fe9b765-babe-433a-8c45-d73c12bd3329",
    "Mark Tedin",
);

// CHR 109 — Tormod's Crypt (reprint)
const TORMODS_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &the_dark::TORMODS_CRYPT,
    "55709270-74b2-4a3f-947d-29ac7c309b0f",
    "Christopher Rush",
);

// CHR 110 — Triassic Egg (reprint)
const TRIASSIC_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TRIASSIC_EGG,
    "3f43d810-7756-4a55-93e8-f8faa7af0ca8",
    "Dan Frazier",
);

// CHR 111 — Voodoo Doll (reprint)
const VOODOO_DOLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::VOODOO_DOLL,
    "089e12c0-e60f-4b60-a2eb-b6c1d088ac50",
    "Sandra Everingham",
);

// CHR 112 — City of Brass (reprint)
const CITY_OF_BRASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::CITY_OF_BRASS,
    "61e9c770-f05b-40df-a01f-82b734db1733",
    "Mark Tedin",
);

// CHR 113 — Safe Haven (reprint)
const SAFE_HAVEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::SAFE_HAVEN,
    "031ad642-9c2c-4462-802c-bb35c124e0bb",
    "Christopher Rush",
);

// CHR 114a — Urza's Mine (reprint)
const URZA_S_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_MINE,
    "2ee60a99-762b-4f82-9a26-3d3b5682f46b",
    "Anson Maddocks",
);

// CHR 114b — Urza's Mine (alternate printing)
const URZA_S_MINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_MINE,
    1,
    "897a79db-ec7a-450c-98ec-53337c564baa",
    "Anson Maddocks",
);

// CHR 114c — Urza's Mine (alternate printing)
const URZA_S_MINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_MINE,
    2,
    "21079a99-690a-459a-b803-19585d8cf5fb",
    "Anson Maddocks",
);

// CHR 114d — Urza's Mine (alternate printing)
const URZA_S_MINE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_MINE,
    3,
    "e4e89152-99a3-45cf-a61a-5635568a0401",
    "Anson Maddocks",
);

// CHR 115a — Urza's Power Plant (reprint)
const URZA_S_POWER_PLANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_POWER_PLANT,
    "df2a9344-38b5-4def-bb6e-87d837e6da54",
    "Mark Tedin",
);

// CHR 115b — Urza's Power Plant (alternate printing)
const URZA_S_POWER_PLANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_POWER_PLANT,
    1,
    "18074937-9ad1-4d96-b4c0-96af06cedac4",
    "Mark Tedin",
);

// CHR 115c — Urza's Power Plant (alternate printing)
const URZA_S_POWER_PLANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_POWER_PLANT,
    2,
    "e01bf216-3e4c-4373-8021-26a7110b5421",
    "Mark Tedin",
);

// CHR 115d — Urza's Power Plant (alternate printing)
const URZA_S_POWER_PLANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_POWER_PLANT,
    3,
    "222dc48a-335d-48e2-ba3d-048ae4c5c925",
    "Mark Tedin",
);

// CHR 116a — Urza's Tower (reprint)
const URZA_S_TOWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_TOWER,
    "530d6fd5-57f2-497d-8144-b3ce64a6c5cb",
    "Mark Poole",
);

// CHR 116b — Urza's Tower (alternate printing)
const URZA_S_TOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_TOWER,
    1,
    "e2424487-fffb-4af6-8f0f-b99ebf6360d6",
    "Mark Poole",
);

// CHR 116c — Urza's Tower (alternate printing)
const URZA_S_TOWER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_TOWER,
    2,
    "52263b78-9829-4778-b7a9-8c4a97d1c6df",
    "Mark Poole",
);

// CHR 116d — Urza's Tower (alternate printing)
const URZA_S_TOWER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_atq::URZA_S_TOWER,
    3,
    "0fc0146c-3f85-4ed5-b37c-f1eeea04143d",
    "Mark Poole",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ABU_JAFAR_REPRINT,
    AKRON_LEGIONNAIRE_REPRINT,
    ANGELIC_VOICES_REPRINT,
    BLOOD_OF_THE_MARTYR_REPRINT,
    DAVENANT_ARCHER_REPRINT,
    DIVINE_OFFERING_REPRINT,
    INDESTRUCTIBLE_AURA_REPRINT,
    IVORY_GUARDIANS_REPRINT,
    KEEPERS_OF_THE_FAITH_REPRINT,
    PETRA_SPHINX_REPRINT,
    REPENTANT_BLACKSMITH_REPRINT,
    SHIELD_WALL_REPRINT,
    WAR_ELEPHANT_REPRINT,
    WITCH_HUNTER_REPRINT,
    AZURE_DRAKE_REPRINT,
    BOOMERANG_REPRINT,
    DANCE_OF_MANY_REPRINT,
    DANDAN_REPRINT,
    ENCHANTMENT_ALTERATION_REPRINT,
    FISHLIVER_OIL_REPRINT,
    FLASH_FLOOD_REPRINT,
    JUXTAPOSE_REPRINT,
    PUPPET_MASTER_REPRINT,
    RECALL_REPRINT,
    REMOVE_SOUL_REPRINT,
    TELEPORT_REPRINT,
    WALL_OF_VAPOR_REPRINT,
    WALL_OF_WONDER_REPRINT,
    BANSHEE_REPRINT,
    BOG_RATS_REPRINT,
    CUOMBAJJ_WITCHES_REPRINT,
    FALLEN_ANGEL_REPRINT,
    GIANT_SLUG_REPRINT,
    HASRAN_OGRESS_REPRINT,
    HELLS_CARETAKER_REPRINT,
    SHIMIAN_NIGHT_STALKER_REPRINT,
    TAKKLEMAGGOT_REPRINT,
    THE_FALLEN_REPRINT,
    THE_WRETCHED_REPRINT,
    TRANSMUTATION_REPRINT,
    WALL_OF_SHADOWS_REPRINT,
    YAWGMOTH_DEMON_REPRINT,
    ACTIVE_VOLCANO_REPRINT,
    ALADDIN_REPRINT,
    BEASTS_OF_BOGARDAN_REPRINT,
    BLOOD_MOON_REPRINT,
    FIRE_DRAKE_REPRINT,
    GOBLIN_ARTISANS_REPRINT,
    GOBLIN_DIGGING_TEAM_REPRINT,
    GOBLIN_SHRINE_REPRINT,
    GOBLINS_OF_THE_FLARG_REPRINT,
    LAND_S_EDGE_REPRINT,
    MOUNTAIN_YETI_REPRINT,
    PRIMORDIAL_OOZE_REPRINT,
    WALL_OF_HEAT_REPRINT,
    WALL_OF_OPPOSITION_REPRINT,
    ARGOTHIAN_PIXIES_REPRINT,
    CAT_WARRIORS_REPRINT,
    COCOON_REPRINT,
    CONCORDANT_CROSSROADS_REPRINT,
    CRAW_GIANT_REPRINT,
    CYCLONE_REPRINT,
    EMERALD_DRAGONFLY_REPRINT,
    ERHNAM_DJINN_REPRINT,
    GHAZBAN_OGRE_REPRINT,
    METAMORPHOSIS_REPRINT,
    RABID_WOMBAT_REPRINT,
    REVELATION_REPRINT,
    SCAVENGER_FOLK_REPRINT,
    STORM_SEEKER_REPRINT,
    ARCADES_SABBOTH_REPRINT,
    AXELROD_GUNNARSON_REPRINT,
    AYESHA_TANAKA_REPRINT,
    CHROMIUM_REPRINT,
    DAKKON_BLACKBLADE_REPRINT,
    GABRIEL_ANGELFIRE_REPRINT,
    JOHAN_REPRINT,
    KEI_TAKAHASHI_REPRINT,
    MARHAULT_ELSDRAGON_REPRINT,
    NEBUCHADNEZZAR_REPRINT,
    NICOL_BOLAS_REPRINT,
    PALLADIA_MORS_REPRINT,
    RUBINIA_SOULSINGER_REPRINT,
    SIVITRI_SCARZAM_REPRINT,
    SOLKANAR_THE_SWAMP_KING_REPRINT,
    STANGG_REPRINT,
    TOBIAS_ANDRION_REPRINT,
    TOR_WAUKI_REPRINT,
    VAEVICTIS_ASMADI_REPRINT,
    XIRA_ARIEN_REPRINT,
    ARENA_OF_THE_ANCIENTS_REPRINT,
    ASHNOD_S_ALTAR_REPRINT,
    ASHNODS_TRANSMOGRANT_REPRINT,
    BARLS_CAGE_REPRINT,
    BOOK_OF_RASS_REPRINT,
    BRONZE_HORSE_REPRINT,
    FELDONS_CANE_REPRINT,
    FOUNTAIN_OF_YOUTH_REPRINT,
    GAUNTLETS_OF_CHAOS_REPRINT,
    HORN_OF_DEAFENING_REPRINT,
    JALUM_TOME_REPRINT,
    JEWELED_BIRD_REPRINT,
    LIVING_ARMOR_REPRINT,
    OBELISK_OF_UNDOING_REPRINT,
    RAKALITE_REPRINT,
    RUNESWORD_REPRINT,
    SENTINEL_REPRINT,
    SERPENT_GENERATOR_REPRINT,
    TORMODS_CRYPT_REPRINT,
    TRIASSIC_EGG_REPRINT,
    VOODOO_DOLL_REPRINT,
    CITY_OF_BRASS_REPRINT,
    SAFE_HAVEN_REPRINT,
    URZA_S_MINE_REPRINT,
    URZA_S_MINE_ALTERNATE_1,
    URZA_S_MINE_ALTERNATE_2,
    URZA_S_MINE_ALTERNATE_3,
    URZA_S_POWER_PLANT_REPRINT,
    URZA_S_POWER_PLANT_ALTERNATE_1,
    URZA_S_POWER_PLANT_ALTERNATE_2,
    URZA_S_POWER_PLANT_ALTERNATE_3,
    URZA_S_TOWER_REPRINT,
    URZA_S_TOWER_ALTERNATE_1,
    URZA_S_TOWER_ALTERNATE_2,
    URZA_S_TOWER_ALTERNATE_3,
];
