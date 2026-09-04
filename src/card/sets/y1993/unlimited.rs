//! Unlimited Edition has no unique card definitions.
//!
//! Every card in the built-in Unlimited catalog points to its first printing.

use super::{CardRecord, PrintingRecord, alpha, beta};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;

// 2ED 1 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "05d7bed4-950a-4a4e-b79a-50e4aa416fe9",
    "Dan Frazier",
);

// 2ED 2 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ARMAGEDDON,
    "df2c5d5c-f1c9-4639-bf72-3f6bde554864",
    "Jesper Myrfors",
);

// 2ED 3 — Balance (reprint)
const BALANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BALANCE,
    "8352e8b6-c947-49f3-a653-a6af65d3e9c3",
    "Mark Poole",
);

// 2ED 4 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "9404e779-2065-4c4f-95d1-6997c7fea156",
    "Douglas Shuler",
);

// 2ED 5 — Black Ward (reprint)
const BLACK_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_WARD,
    "f0cd79e9-1b61-4ad3-8f6d-cb5d3f60ef8e",
    "Dan Frazier",
);

// 2ED 6 — Blaze of Glory (reprint)
const BLAZE_OF_GLORY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLAZE_OF_GLORY,
    "2d636573-287d-4f6f-93b0-12ddd8f3e6d1",
    "Richard Thomas",
);

// 2ED 7 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLESSING,
    "402e84fb-7c77-4491-9ece-c2d9b8506ece",
    "Julie Baroh",
);

// 2ED 8 — Blue Ward (reprint)
const BLUE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_WARD,
    "1704d11c-569c-4b4e-bbe0-df42af98c4fc",
    "Dan Frazier",
);

// 2ED 9 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "2ea3db44-85c5-4201-a5c9-ec14a9d244d6",
    "Dameon Willich",
);

// 2ED 10 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "1eea1199-6b07-430c-b100-c5825a23d8b0",
    "Jesper Myrfors",
);

// 2ED 11 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "c19c60f7-92b7-4f84-b2c3-64e3d00dcb63",
    "Dameon Willich",
);

// 2ED 12 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "108ce265-1b3a-484a-9b0c-cab1094d1521",
    "Sandra Everingham",
);

// 2ED 13 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "4cc60529-401b-481a-b65c-ad791153afd7",
    "Mark Tedin",
);

// 2ED 14 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "98a1c689-cd8b-4a80-ad6d-e4ff5933f5e7",
    "Douglas Shuler",
);

// 2ED 15 — Consecrate Land (reprint)
const CONSECRATE_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSECRATE_LAND,
    "9efb29d2-550f-4ede-b024-7b0e15c2e986",
    "Jeff A. Menges",
);

// 2ED 16 — Conversion (reprint)
const CONVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONVERSION,
    "45bf4297-ccf4-4fa0-b7ce-5aaebca50813",
    "Jesper Myrfors",
);

// 2ED 17 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CRUSADE,
    "4b9933e3-2267-4534-a1c6-c463e767480a",
    "Mark Poole",
);

// 2ED 18 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "d7604388-752a-463d-95cd-486752a4bd04",
    "Mark Poole",
);

// 2ED 19 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DISENCHANT,
    "73636b95-103d-43c8-bc96-63fad0da34dd",
    "Amy Weber",
);

// 2ED 20 — Farmstead (reprint)
const FARMSTEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FARMSTEAD,
    "3d79940b-9384-4009-8b74-3d56a2c5a8a5",
    "Mark Poole",
);

// 2ED 21 — Green Ward (reprint)
const GREEN_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GREEN_WARD,
    "73f6058a-9292-4474-a794-7161ec9a99f0",
    "Dan Frazier",
);

// 2ED 22 — Guardian Angel (reprint)
const GUARDIAN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GUARDIAN_ANGEL,
    "c2b47221-c468-4b77-89c5-79a06443ef81",
    "Anson Maddocks",
);

// 2ED 23 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "a38b2f1c-a69b-467a-a749-d7fbc1fb6dbb",
    "Dan Frazier",
);

// 2ED 24 — Holy Armor (reprint)
const HOLY_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_ARMOR,
    "9a7d92de-d663-4919-a23f-38389ba5593e",
    "Melissa A. Benson",
);

// 2ED 25 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "f25cea1b-22c0-4323-8119-0ca627426aa7",
    "Anson Maddocks",
);

// 2ED 26 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "d5726f8d-4467-4ab9-8931-432c0cefcbf4",
    "Mark Poole",
);

// 2ED 27 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "c9aa32e2-aeb0-4104-8603-a56bd8fc0953",
    "Richard Thomas",
);

// 2ED 28 — Lance (reprint)
const LANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LANCE,
    "e7e9714d-072b-4237-8371-5ce2709c878f",
    "Rob Alexander",
);

// 2ED 29 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "7ff95a24-86e9-4302-bd90-89ca96164032",
    "Melissa A. Benson",
);

// 2ED 30 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "309cd081-13bb-428b-b561-60b7a81c0f1d",
    "Douglas Shuler",
);

// 2ED 31 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "9254b0be-d350-41c1-8ed9-41a22525adf9",
    "Cornelius Brudi",
);

// 2ED 32 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "19272824-a0a4-4352-8904-a185516c95e1",
    "Kev Brockschmidt",
);

// 2ED 33 — Purelace (reprint)
const PURELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PURELACE,
    "bd89c79a-668e-4f3c-b248-6f067e6fca65",
    "Sandra Everingham",
);

// 2ED 34 — Red Ward (reprint)
const RED_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_WARD,
    "03d818b4-4722-4035-a2bc-ebc4c8c90ec0",
    "Dan Frazier",
);

// 2ED 35 — Resurrection (reprint)
const RESURRECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RESURRECTION,
    "609f6e06-daaa-4b15-a167-dc3ed6ce33cc",
    "Dan Frazier",
);

// 2ED 36 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "7f83f4fa-9c22-4bcb-8de0-f40f208128d1",
    "Dameon Willich",
);

// 2ED 37 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "ddb92543-e601-4575-8e17-84ec0b1edd66",
    "Douglas Shuler",
);

// 2ED 38 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "7281e17d-a6e0-4e0e-8ee6-c6d9dec54231",
    "Tom Wänerstrand",
);

// 2ED 39 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH_LIONS,
    "3da61fc1-6201-4823-975f-2d4d9f7f3193",
    "Daniel Gelon",
);

// 2ED 40 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SERRA_ANGEL,
    "1941cf19-b1f6-4148-a1de-6d03531f2f1c",
    "Douglas Shuler",
);

// 2ED 41 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWORDS_TO_PLOWSHARES,
    "50fc5b10-6215-48a9-8993-b61681f61186",
    "Jeff A. Menges",
);

// 2ED 42 — Veteran Bodyguard (reprint)
const VETERAN_BODYGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VETERAN_BODYGUARD,
    "8d693da0-039d-462b-a5cb-d2bb179df65e",
    "Douglas Shuler",
);

// 2ED 43 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "0437a9e4-df29-4fbb-8c99-05e5d30a18e3",
    "Mark Tedin",
);

// 2ED 44 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHITE_KNIGHT,
    "8e4c578c-1c36-4c29-86a5-7a664ffe34d0",
    "Daniel Gelon",
);

// 2ED 45 — White Ward (reprint)
const WHITE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_WARD,
    "77cbc0fa-d5b8-412a-bdca-7f62d8d1ce1e",
    "Dan Frazier",
);

// 2ED 46 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WRATH_OF_GOD,
    "e57404bc-44ba-4909-87da-f4a71673168d",
    "Quinton Hoover",
);

// 2ED 47 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "ef5b8140-a157-4c20-a428-fa7250ab34e1",
    "Richard Thomas",
);

// 2ED 48 — Ancestral Recall (reprint)
const ANCESTRAL_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANCESTRAL_RECALL,
    "2dd41293-d7c8-4422-9f0c-b3e96350f5c9",
    "Mark Poole",
);

// 2ED 49 — Animate Artifact (reprint)
const ANIMATE_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANIMATE_ARTIFACT,
    "caf1ee51-2852-44c0-a5d4-d0e415381738",
    "Douglas Shuler",
);

// 2ED 50 — Blue Elemental Blast (reprint)
const BLUE_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLUE_ELEMENTAL_BLAST,
    "42d1579e-a587-4397-bd9a-cda52fcf6a1b",
    "Richard Thomas",
);

// 2ED 51 — Braingeyser (reprint)
const BRAINGEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BRAINGEYSER,
    "3dbeef5c-f973-480b-a148-28de397b610f",
    "Mark Tedin",
);

// 2ED 52 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLONE,
    "bcf09714-89cf-4feb-b941-74f791bbdf6e",
    "Julie Baroh",
);

// 2ED 53 — Control Magic (reprint)
const CONTROL_MAGIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTROL_MAGIC,
    "076d132a-fa3d-464b-b5f9-a12e46c9f2df",
    "Dameon Willich",
);

// 2ED 54 — Copy Artifact (reprint)
const COPY_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPY_ARTIFACT,
    "dde40c1f-5ccc-435b-ac35-62eb58ffeea2",
    "Amy Weber",
);

// 2ED 55 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COUNTERSPELL,
    "7c666b4b-c4ff-40ca-9d16-c76aafebaa83",
    "Mark Poole",
);

// 2ED 56 — Creature Bond (reprint)
const CREATURE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CREATURE_BOND,
    "1f9e4aa8-4ca7-4893-81d7-98205246f357",
    "Anson Maddocks",
);

// 2ED 57 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "6123f833-236d-4c61-b543-4ac662759336",
    "Douglas Shuler",
);

// 2ED 58 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "5083317e-8536-41e3-a441-8e6be4d63d50",
    "Quinton Hoover",
);

// 2ED 59 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "5460051e-07fc-4818-82fd-7c424334b7bf",
    "Anson Maddocks",
);

// 2ED 60 — Invisibility (reprint)
const INVISIBILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INVISIBILITY,
    "de833d23-2abd-42c3-a38f-f16813aaee4e",
    "Anson Maddocks",
);

// 2ED 61 — Jump (reprint)
const JUMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUMP,
    "24b4c4c9-84c1-484c-9f67-1f460585d45c",
    "Mark Poole",
);

// 2ED 62 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "64641b90-c72e-4eab-9b99-330786739ab9",
    "Anson Maddocks",
);

// 2ED 63 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "fa161987-2dd1-4efe-b934-acbd93653169",
    "Melissa A. Benson",
);

// 2ED 64 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "7abc2b06-3613-4e42-bf51-d340d1e70a78",
    "Julie Baroh",
);

// 2ED 65 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "66d5effc-dc31-485c-91c0-e9a8e2b098af",
    "Dan Frazier",
);

// 2ED 66 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_SHORT,
    "743e0f1e-55ab-429a-b9f1-769b008ad06a",
    "Dameon Willich",
);

// 2ED 67 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "ab8019a6-0d62-4145-8a4d-87205d3cb9d6",
    "Jeff A. Menges",
);

// 2ED 68 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "e8244a80-3d9a-4392-ac62-739b3e330638",
    "Mark Poole",
);

// 2ED 69 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "c521f86e-f1bb-4e63-ab12-5ecebba2701b",
    "Dameon Willich",
);

// 2ED 70 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "cd480428-de3e-4e98-8483-684f0572c400",
    "Jesper Myrfors",
);

// 2ED 71 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "d6119988-4797-4993-a75f-e7015c2c6354",
    "Tom Wänerstrand",
);

// 2ED 72 — Power Leak (reprint)
const POWER_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_LEAK,
    "436fd628-c545-4cbf-8100-4e6aa8475868",
    "Drew Tucker",
);

// 2ED 73 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "f0313c44-d4ca-4021-866a-3d5cf58b0e76",
    "Richard Thomas",
);

// 2ED 74 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "4cfb5638-4502-44ed-b54c-27276d45d1ad",
    "Douglas Shuler",
);

// 2ED 75 — Psionic Blast (reprint)
const PSIONIC_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PSIONIC_BLAST,
    "8a1dff82-de5c-4b1d-b87f-6ddb4551f820",
    "Douglas Shuler",
);

// 2ED 76 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "b36e0fba-f6a4-4400-b685-3178431c292f",
    "Brian Snõddy",
);

// 2ED 77 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "af430730-2ce8-45c3-b1da-9745fc792d71",
    "Jeff A. Menges",
);

// 2ED 78 — Siren's Call (reprint)
const SIREN_S_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIREN_S_CALL,
    "0c907ef4-a2cf-4e7a-acf6-f187308ff303",
    "Anson Maddocks",
);

// 2ED 79 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "d1349af8-a709-4535-b532-eb769289906d",
    "Mark Poole",
);

// 2ED 80 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "9ec03950-80f7-4783-9b65-f2538436c9be",
    "Brian Snõddy",
);

// 2ED 81 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STASIS,
    "5902c2aa-c77c-4c6a-9a1e-77cb9bb53aa1",
    "Fay Jones",
);

// 2ED 82 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "04a1a6f4-a73b-4593-b14e-8c87f94debc1",
    "Amy Weber",
);

// 2ED 83 — Thoughtlace (reprint)
const THOUGHTLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THOUGHTLACE,
    "b8859cf0-e4c3-4044-9674-d0703646d72e",
    "Mark Poole",
);

// 2ED 84 — Time Walk (reprint)
const TIME_WALK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_WALK,
    "ade7d00d-4e7b-46e9-ace1-63f628a589fc",
    "Amy Weber",
);

// 2ED 85 — Timetwister (reprint)
const TIMETWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIMETWISTER,
    "01bda3d7-122a-48a0-bab3-676c4a557b74",
    "Mark Tedin",
);

// 2ED 86 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "ba01195b-05a0-4de7-807e-934e71feb8c7",
    "Rob Alexander",
);

// 2ED 87 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "0a681487-951d-4ff1-ab08-bc173ea022e8",
    "Douglas Shuler",
);

// 2ED 88 — Vesuvan Doppelganger (reprint)
const VESUVAN_DOPPELGANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VESUVAN_DOPPELGANGER,
    "408ec348-183b-43de-abac-7ae9e3843c10",
    "Quinton Hoover",
);

// 2ED 89 — Volcanic Eruption (reprint)
const VOLCANIC_ERUPTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VOLCANIC_ERUPTION,
    "6d7c78a4-e3db-42bf-8365-d7a08c26f4a9",
    "Douglas Shuler",
);

// 2ED 90 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "d672107f-e274-4a0e-888a-c2aa59a2fab5",
    "Richard Thomas",
);

// 2ED 91 — Wall of Water (reprint)
const WALL_OF_WATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WATER,
    "f97f5b6e-7997-498a-9b27-ac2873f425dd",
    "Richard Thomas",
);

// 2ED 92 — Water Elemental (reprint)
const WATER_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WATER_ELEMENTAL,
    "c498c898-1671-4632-b69a-0e1e9b8d05b8",
    "Jeff A. Menges",
);

// 2ED 93 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "0fc3ed63-96ee-420c-bde1-e0c904059931",
    "Anson Maddocks",
);

// 2ED 94 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "882fe528-1a84-4d34-bd15-330963b684ff",
    "Jesper Myrfors",
);

// 2ED 95 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_KNIGHT,
    "36b94d0d-fbe5-4f32-af02-bbe3ab2e234a",
    "Jeff A. Menges",
);

// 2ED 96 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "94345aab-b9f2-463e-91ab-acd8b99a7ec0",
    "Jeff A. Menges",
);

// 2ED 97 — Contract from Below (reprint)
const CONTRACT_FROM_BELOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTRACT_FROM_BELOW,
    "530be94d-7f6a-4c23-948f-22c5bdbceb4f",
    "Douglas Shuler",
);

// 2ED 98 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "69f37a32-dc03-49fd-b28b-d091563d3690",
    "Jesper Myrfors",
);

// 2ED 99 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DARK_RITUAL,
    "c4d24ff3-315d-44cd-8c27-d8ad6972e027",
    "Sandra Everingham",
);

// 2ED 100 — Darkpact (reprint)
const DARKPACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARKPACT,
    "9350762e-cb5f-4cea-a880-6e731685bf67",
    "Quinton Hoover",
);

// 2ED 101 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "fe9210db-2ab3-42e6-be04-790917092317",
    "Anson Maddocks",
);

// 2ED 102 — Deathlace (reprint)
const DEATHLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHLACE,
    "c3e02432-b8bd-4091-a520-6895313ff141",
    "Sandra Everingham",
);

// 2ED 103 — Demonic Attorney (reprint)
const DEMONIC_ATTORNEY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_ATTORNEY,
    "4fbb25bc-7071-40eb-bea6-16387db164a7",
    "Daniel Gelon",
);

// 2ED 104 — Demonic Hordes (reprint)
const DEMONIC_HORDES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_HORDES,
    "812a0a10-0765-499f-8581-c4d7e0e81299",
    "Jesper Myrfors",
);

// 2ED 105 — Demonic Tutor (reprint)
const DEMONIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DEMONIC_TUTOR,
    "c8d5d6a5-6807-4a80-9460-7633dc430ee9",
    "Douglas Shuler",
);

// 2ED 106 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAIN_LIFE,
    "d5f7044e-3b91-42ac-91ec-56e17cd72274",
    "Douglas Shuler",
);

// 2ED 107 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "4eb88d79-048b-4f7c-9ca0-4d9066af805e",
    "Sandra Everingham",
);

// 2ED 108 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "19d85c34-2057-4572-a881-29dd35c1ee30",
    "Sandra Everingham",
);

// 2ED 109 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "e48c7fd2-860e-4266-b8c0-f6d48f52b851",
    "Mark Poole",
);

// 2ED 110 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "485421e0-ee1c-425b-abe0-ec5a7e2c0042",
    "Douglas Shuler",
);

// 2ED 111 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "f463412c-ac10-476c-bba1-27724c041d68",
    "Dan Frazier",
);

// 2ED 112 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "78694fa9-85dc-4671-87e9-a2bccdc9fcce",
    "Mark Poole",
);

// 2ED 113 — Hypnotic Specter (reprint)
const HYPNOTIC_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::HYPNOTIC_SPECTER,
    "e12847f4-4ace-4116-bc96-f3e5336eb35f",
    "Douglas Shuler",
);

// 2ED 114 — Lich (reprint)
const LICH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LICH,
    "5bded615-62bc-40f6-9a54-7c9d0d551d4c",
    "Daniel Gelon",
);

// 2ED 115 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "3ac3a8d8-47a7-4e47-a16c-109aeccd8d1f",
    "Mark Tedin",
);

// 2ED 116 — Mind Twist (reprint)
const MIND_TWIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MIND_TWIST,
    "f3d7381b-9075-4c9b-adf5-a0d1c26fab67",
    "Julie Baroh",
);

// 2ED 117 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "18e057ae-8e60-478c-b047-605dab356835",
    "Christopher Rush",
);

// 2ED 118 — Nettling Imp (reprint)
const NETTLING_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETTLING_IMP,
    "96706002-176d-41f7-9788-3d0f7962ea03",
    "Quinton Hoover",
);

// 2ED 119 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "747d4c99-0287-4138-af13-6244f33d2e57",
    "Melissa A. Benson",
);

// 2ED 120 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "e21b04cd-2d43-4d64-a1c2-46a9f02508d6",
    "Anson Maddocks",
);

// 2ED 121 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "2be5a75e-2fef-4205-bdec-5ea0d1dd0733",
    "Jesper Myrfors",
);

// 2ED 122 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "f2a5bd30-a11f-4218-aca6-3183d82d02b9",
    "Anson Maddocks",
);

// 2ED 123 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "990dc823-881d-40ea-9731-d3f19c41aadc",
    "Jeff A. Menges",
);

// 2ED 124 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROYAL_ASSASSIN,
    "5cceb11b-0f70-4749-8a8c-d698cd01cd6e",
    "Tom Wänerstrand",
);

// 2ED 125 — Sacrifice (reprint)
const SACRIFICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SACRIFICE,
    "288323c1-13f1-481e-940e-5e4ecebb404e",
    "Dan Frazier",
);

// 2ED 126 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "08e060d5-85f2-46a7-9f05-8a8c713ea999",
    "Jesper Myrfors",
);

// 2ED 127 — Scavenging Ghoul (reprint)
const SCAVENGING_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCAVENGING_GHOUL,
    "12459e80-2878-4a76-b45a-478ee3b0f7a4",
    "Jeff A. Menges",
);

// 2ED 128 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SENGIR_VAMPIRE,
    "ffd7ca8e-6437-4b85-81dd-7173200dcec7",
    "Anson Maddocks",
);

// 2ED 129 — Simulacrum (reprint)
const SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIMULACRUM,
    "a80e1e4c-4b53-41d2-b038-2f9135d8455d",
    "Mark Poole",
);

// 2ED 130 — Sinkhole (reprint)
const SINKHOLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SINKHOLE,
    "485cef94-d7aa-4bb3-b2e6-61d0ccf8007e",
    "Sandra Everingham",
);

// 2ED 131 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TERROR,
    "df3c25cc-5705-4deb-be61-07a8a2716c86",
    "Ron Spencer",
);

// 2ED 132 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "7150245a-4fed-47cd-b13f-24507e89449d",
    "Douglas Shuler",
);

// 2ED 133 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "ed63a624-dc31-4461-9cda-589a84dc5a40",
    "Anson Maddocks",
);

// 2ED 134 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "d1320d4a-ecfc-4cd5-bc6b-445f63c17b27",
    "Amy Weber",
);

// 2ED 135 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "b1646d85-2396-445c-9bbb-65bf65b0a63c",
    "Douglas Shuler",
);

// 2ED 136 — Will-o'-the-Wisp (reprint)
const WILL_O_THE_WISP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILL_O_THE_WISP,
    "73a2a070-464e-4749-87f1-2df5c8b2a93b",
    "Jesper Myrfors",
);

// 2ED 137 — Word of Command (reprint)
const WORD_OF_COMMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WORD_OF_COMMAND,
    "239c8547-207b-41d1-a2be-8825bfc6ef7f",
    "Jesper Myrfors",
);

// 2ED 138 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "d9b2accc-11e8-4bfd-97fc-d2f6bcd94c26",
    "Jeff A. Menges",
);

// 2ED 139 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "08c109d4-6dd1-42a5-90ed-f8a71b6a0ca5",
    "Mark Poole",
);

// 2ED 140 — Chaoslace (reprint)
const CHAOSLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHAOSLACE,
    "f2776675-8720-4a4d-8d7b-96de9ad14533",
    "Dameon Willich",
);

// 2ED 141 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "f94878cc-4c0f-42e4-a49f-02a2b269ef06",
    "Anson Maddocks",
);

// 2ED 142 — Dragon Whelp (reprint)
const DRAGON_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAGON_WHELP,
    "7ad8ab3d-8a77-4fd3-8d5a-ac1e8a09e3bc",
    "Amy Weber",
);

// 2ED 143 — Dwarven Demolition Team (reprint)
const DWARVEN_DEMOLITION_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_DEMOLITION_TEAM,
    "a6b2fe92-0521-4a85-9a8d-4203b0e0e118",
    "Kev Brockschmidt",
);

// 2ED 144 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "113d518a-2ce9-4747-9e6f-c6a464a78a49",
    "Douglas Shuler",
);

// 2ED 145 — Earth Elemental (reprint)
const EARTH_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTH_ELEMENTAL,
    "2295ded3-7e72-4f3b-93e2-e9557a10b32e",
    "Dan Frazier",
);

// 2ED 146 — Earthbind (reprint)
const EARTHBIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHBIND,
    "8a05dcd8-4c5d-413c-b1c0-3613f211a284",
    "Quinton Hoover",
);

// 2ED 147 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::EARTHQUAKE,
    "1dba16d3-292d-430c-88cc-c49ded13effb",
    "Dan Frazier",
);

// 2ED 148 — False Orders (reprint)
const FALSE_ORDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FALSE_ORDERS,
    "a59c24d9-804b-45d0-b60c-cfc7a6af7ef5",
    "Anson Maddocks",
);

// 2ED 149 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "bddeac3f-f4ee-432b-9d69-8533a28e7f46",
    "Melissa A. Benson",
);

// 2ED 150 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FIREBALL,
    "39e05c2d-b4a1-4f59-8743-f1694c803164",
    "Mark Tedin",
);

// 2ED 151 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "dc2bfe7b-9850-4450-9bad-73fa0d678a5f",
    "Dan Frazier",
);

// 2ED 152 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "c2990a78-54fc-4fae-a6c9-e03c5c39eee3",
    "Dameon Willich",
);

// 2ED 153 — Fork (reprint)
const FORK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FORK,
    "a877d692-018b-4a08-ab6f-9707b267f6fd",
    "Amy Weber",
);

// 2ED 154 — Goblin Balloon Brigade (reprint)
const GOBLIN_BALLOON_BRIGADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_BALLOON_BRIGADE,
    "26cbb4d5-bb1b-4b1c-b94d-58e45ba497ca",
    "Andi Rusu",
);

// 2ED 155 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_KING,
    "1954e618-b4ac-48d8-9218-b29878bae710",
    "Jesper Myrfors",
);

// 2ED 156 — Granite Gargoyle (reprint)
const GRANITE_GARGOYLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GRANITE_GARGOYLE,
    "01116585-a8c7-4619-b0a6-fcfe78fdaf3c",
    "Christopher Rush",
);

// 2ED 157 — Gray Ogre (reprint)
const GRAY_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRAY_OGRE,
    "e2e956a7-3ed1-4cbb-a6fd-123453360058",
    "Dan Frazier",
);

// 2ED 158 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "df03759e-17a0-4191-bd4d-e823846924ce",
    "Dan Frazier",
);

// 2ED 159 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "8ca4c6df-a456-4eb3-90fc-f1e7ee8c48e4",
    "Anson Maddocks",
);

// 2ED 160 — Ironclaw Orcs (reprint)
const IRONCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRONCLAW_ORCS,
    "0e17623a-5bc0-42d7-a842-394de0a01a01",
    "Anson Maddocks",
);

// 2ED 161 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "f2d0bc79-d2f8-43e7-9106-c0d01db31fa2",
    "Kev Brockschmidt",
);

// 2ED 162 — Lightning Bolt (reprint)
const LIGHTNING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LIGHTNING_BOLT,
    "ff1b8fc5-604a-4449-a73d-861e53642a70",
    "Christopher Rush",
);

// 2ED 163 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "bf770633-612e-41db-a451-7da802c46e4d",
    "Christopher Rush",
);

// 2ED 164 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "3c424086-8122-404d-8c3a-f36d455271a7",
    "Christopher Rush",
);

// 2ED 165 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "0d3eff55-6a14-4c01-8b05-715094a319b3",
    "Jeff A. Menges",
);

// 2ED 166 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "da899c3d-c424-4901-ae5a-2a8e0c66e631",
    "Anson Maddocks",
);

// 2ED 167 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "def20e99-7a94-4b24-87fb-758ede816b57",
    "Dan Frazier",
);

// 2ED 168 — Power Surge (reprint)
const POWER_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SURGE,
    "98ac9e72-603b-43cf-b959-03552c44ae22",
    "Douglas Shuler",
);

// 2ED 169 — Raging River (reprint)
const RAGING_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAGING_RIVER,
    "7ee63877-056e-413d-932a-a393a4183686",
    "Sandra Everingham",
);

// 2ED 170 — Red Elemental Blast (reprint)
const RED_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::RED_ELEMENTAL_BLAST,
    "1c69e1c9-e8ed-4497-8098-0d412a09c0f9",
    "Richard Thomas",
);

// 2ED 171 — Roc of Kher Ridges (reprint)
const ROC_OF_KHER_RIDGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROC_OF_KHER_RIDGES,
    "7509b414-aea1-4f87-993a-ee7b9aee509b",
    "Andi Rusu",
);

// 2ED 172 — Rock Hydra (reprint)
const ROCK_HYDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROCK_HYDRA,
    "aae6ce4f-d3ba-4b6c-a9c3-9ecbc7a3d5c8",
    "Jeff A. Menges",
);

// 2ED 173 — Sedge Troll (reprint)
const SEDGE_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SEDGE_TROLL,
    "5a30ed3f-0b21-45ea-83af-339249b4e93e",
    "Dan Frazier",
);

// 2ED 174 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SHATTER,
    "80f3aef5-c997-4852-8c13-a4d2c22d9c95",
    "Amy Weber",
);

// 2ED 175 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "fd4f6e34-3f66-4e10-8170-56039c5f6fcc",
    "Melissa A. Benson",
);

// 2ED 176 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SMOKE,
    "4d2553c0-1105-4eed-baf2-e13f1005dfb7",
    "Jesper Myrfors",
);

// 2ED 177 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_GIANT,
    "64dad66b-403b-4af6-b6eb-c123567e2b86",
    "Dameon Willich",
);

// 2ED 178 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_RAIN,
    "cdb490bb-43fe-49ab-a094-438585677801",
    "Daniel Gelon",
);

// 2ED 179 — Tunnel (reprint)
const TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNNEL,
    "a0176176-0530-43e6-85e4-d1f4296f0697",
    "Dan Frazier",
);

// 2ED 180 — Two-Headed Giant of Foriys (reprint)
const TWO_HEADED_GIANT_OF_FORIYS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWO_HEADED_GIANT_OF_FORIYS,
    "67299451-5302-4639-a4bc-6109521a2c0c",
    "Anson Maddocks",
);

// 2ED 181 — Uthden Troll (reprint)
const UTHDEN_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UTHDEN_TROLL,
    "30bb1158-fe16-49e6-9b7a-44b7bee84737",
    "Douglas Shuler",
);

// 2ED 182 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "74841ee8-2af0-4019-898d-d0ce72fc62c3",
    "Richard Thomas",
);

// 2ED 183 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "2a2cab55-fc64-4b3f-bc46-a1a297d2d448",
    "Dan Frazier",
);

// 2ED 184 — Wheel of Fortune (reprint)
const WHEEL_OF_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHEEL_OF_FORTUNE,
    "4407fb95-0ed2-4c95-91b9-09eb52bf537e",
    "Daniel Gelon",
);

// 2ED 185 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "5aa02bb5-7365-4b8d-ac86-13721fb19d01",
    "Jeff A. Menges",
);

// 2ED 186 — Berserk (reprint)
const BERSERK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BERSERK,
    "fd082697-493f-48e3-a41f-123700435025",
    "Dan Frazier",
);

// 2ED 187 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BIRDS_OF_PARADISE,
    "4e50454c-3927-4e7e-b4f6-7f5d5fd9b913",
    "Mark Poole",
);

// 2ED 188 — Camouflage (reprint)
const CAMOUFLAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CAMOUFLAGE,
    "09243dc6-c56c-42a8-969b-2ecffe89e1ca",
    "Jesper Myrfors",
);

// 2ED 189 — Channel (reprint)
const CHANNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHANNEL,
    "6a7a0f8f-f51e-4cfb-a546-87a086d5936a",
    "Richard Thomas",
);

// 2ED 190 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "8392d34d-d14a-43ca-997d-fe59e505034e",
    "Dan Frazier",
);

// 2ED 191 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "a5bbaf11-6bf1-42a1-a8be-66bc47485a6c",
    "Daniel Gelon",
);

// 2ED 192 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "0fad0e0d-f34a-45ff-9f01-e6ac10b6928f",
    "Anson Maddocks",
);

// 2ED 193 — Fastbond (reprint)
const FASTBOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FASTBOND,
    "64b52b42-e2af-4040-b7ba-34cc292af7ef",
    "Mark Poole",
);

// 2ED 194 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "ba0a14ac-037a-42f2-8fc9-2a41275dc7da",
    "Jesper Myrfors",
);

// 2ED 195 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "247a2ba4-aa5d-4970-b886-90196f684f80",
    "Douglas Shuler",
);

// 2ED 196 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "1a34fc9e-96cf-40f2-adb9-ac5085d140af",
    "Daniel Gelon",
);

// 2ED 197 — Gaea's Liege (reprint)
const GAEA_S_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAEA_S_LIEGE,
    "5eb712ea-c9f5-4831-9e1e-22bf5a75d426",
    "Dameon Willich",
);

// 2ED 198 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GIANT_GROWTH,
    "211ba440-1c29-403a-8dd7-aa5792d20a1a",
    "Sandra Everingham",
);

// 2ED 199 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "a94d08f2-07ac-4887-aa30-ed0579d5113f",
    "Sandra Everingham",
);

// 2ED 200 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "d74cce44-b54b-4922-9cea-f3fda725d24f",
    "Jeff A. Menges",
);

// 2ED 201 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "2287bb85-72b1-40ae-9d44-0364a4075e88",
    "Dameon Willich",
);

// 2ED 202 — Ice Storm (reprint)
const ICE_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ICE_STORM,
    "2ec2246d-8bea-43c4-bf7f-2acad363e0af",
    "Dan Frazier",
);

// 2ED 203 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "1ba27e77-00b8-4d6c-acbd-462273212fc2",
    "Dameon Willich",
);

// 2ED 204 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "f89f3bda-e2fb-496e-a9f3-7260e8ac97fd",
    "Jesper Myrfors",
);

// 2ED 205 — Kudzu (reprint)
const KUDZU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KUDZU,
    "f92ec34e-e374-462f-aa9c-257558defb1f",
    "Mark Poole",
);

// 2ED 206 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "30c3f2cd-5113-45f5-bb8d-4a7d5c4c76a5",
    "Sandra Everingham",
);

// 2ED 207 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "58b02fa9-5481-4614-b9b4-5f8857848e3e",
    "Dameon Willich",
);

// 2ED 208 — Lifelace (reprint)
const LIFELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFELACE,
    "446558ba-2396-4b9e-b56a-cf2014e7a13c",
    "Amy Weber",
);

// 2ED 209 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "47354179-7048-4329-9c50-ce9d4e714a5b",
    "Anson Maddocks",
);

// 2ED 210 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "a0a8474f-279e-44d7-a062-6dcb556c328d",
    "Jesper Myrfors",
);

// 2ED 211 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LLANOWAR_ELVES,
    "fedd1b24-44ee-493a-b4db-3048ff5c760b",
    "Anson Maddocks",
);

// 2ED 212 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "f790990a-f47d-4fb0-a361-108037dd7464",
    "Anson Maddocks",
);

// 2ED 213 — Natural Selection (reprint)
const NATURAL_SELECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NATURAL_SELECTION,
    "315a6bfb-5417-465f-97d9-e157f5c3cf79",
    "Mark Poole",
);

// 2ED 214 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "b523f013-3dbd-4b5c-9433-cdec7dc737ba",
    "Quinton Hoover",
);

// 2ED 215 — Regrowth (reprint)
const REGROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::REGROWTH,
    "2d764cd4-0cec-425c-8cc4-68a81c1f296b",
    "Dameon Willich",
);

// 2ED 216 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRYB_SPRITES,
    "e9e2f1fe-4df0-48c8-b469-4175ba5011e8",
    "Amy Weber",
);

// 2ED 217 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "b25f298a-9784-4192-b640-caec2b94ba4c",
    "Anson Maddocks",
);

// 2ED 218 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "70e476cb-8b72-434c-b5e9-0fd0319a1bff",
    "Mark Poole",
);

// 2ED 219 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "32401b72-e351-45fa-a16e-33cc818a07e0",
    "Dan Frazier",
);

// 2ED 220 — Timber Wolves (reprint)
const TIMBER_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TIMBER_WOLVES,
    "0d24fc87-7b30-4c99-a525-b1746821391c",
    "Melissa A. Benson",
);

// 2ED 221 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "fc24f763-4c7f-45e4-933b-573d1ace1ddc",
    "Douglas Shuler",
);

// 2ED 222 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "b8328ddc-d2d9-47d3-a98b-1a7c7b0c75a3",
    "Richard Thomas",
);

// 2ED 223 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "55454150-de1b-4921-9c23-7d10724c2ee7",
    "Kev Brockschmidt",
);

// 2ED 224 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "5ae21e65-fc55-4a90-806e-452ef0ad5e3a",
    "Anson Maddocks",
);

// 2ED 225 — Wall of Ice (reprint)
const WALL_OF_ICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_ICE,
    "d79867a0-c525-4e91-8942-c61b41f9150c",
    "Richard Thomas",
);

// 2ED 226 — Wall of Wood (reprint)
const WALL_OF_WOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WOOD,
    "b55d8375-ea70-4dd0-950e-3dbf3dfdd4f6",
    "Mark Tedin",
);

// 2ED 227 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "3ee3c4fc-342f-48b3-a799-0db4b005195a",
    "Cornelius Brudi",
);

// 2ED 228 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "c9ee4dea-20b2-43ed-a6d5-f2d62b0e189b",
    "Jeff A. Menges",
);

// 2ED 229 — Web (reprint)
const WEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEB,
    "6fbbac49-9117-4e15-89e8-98387f7511ed",
    "Rob Alexander",
);

// 2ED 230 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "b7425741-5d7c-4016-8d42-ec8b7353116b",
    "Mark Poole",
);

// 2ED 231 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANKH_OF_MISHRA,
    "808cad10-69d5-4e14-9834-476c53ec97e4",
    "Amy Weber",
);

// 2ED 232 — Basalt Monolith (reprint)
const BASALT_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BASALT_MONOLITH,
    "5a72cd4b-5b47-46b8-b230-4b246f97221f",
    "Jesper Myrfors",
);

// 2ED 233 — Black Lotus (reprint)
const BLACK_LOTUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_LOTUS,
    "4a2e428c-dd25-484c-bbc8-2d6ce10ef42c",
    "Christopher Rush",
);

// 2ED 234 — Black Vise (reprint)
const BLACK_VISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_VISE,
    "5159a2cd-036c-482e-9b5a-b595391deef3",
    "Richard Thomas",
);

// 2ED 235 — Celestial Prism (reprint)
const CELESTIAL_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CELESTIAL_PRISM,
    "cb119f5e-a47f-4910-b170-561d6315fdc3",
    "Amy Weber",
);

// 2ED 236 — Chaos Orb (reprint)
const CHAOS_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHAOS_ORB,
    "7a601041-926f-40fd-8106-39099b87806f",
    "Mark Tedin",
);

// 2ED 237 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "c7741816-0bc1-4540-b4b2-006275ffe572",
    "Drew Tucker",
);

// 2ED 238 — Conservator (reprint)
const CONSERVATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSERVATOR,
    "744e7821-8bfd-4816-a8af-4e6fe7b35505",
    "Amy Weber",
);

// 2ED 239 — Copper Tablet (reprint)
const COPPER_TABLET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPPER_TABLET,
    "c17cb591-916e-4176-aeb9-e2275d68d472",
    "Amy Weber",
);

// 2ED 240 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "00c92601-11b9-4e7c-bc81-882085f3fae6",
    "Amy Weber",
);

// 2ED 241 — Cyclopean Tomb (reprint)
const CYCLOPEAN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CYCLOPEAN_TOMB,
    "a184cd2e-e27f-44b0-a8ae-9d861280e469",
    "Anson Maddocks",
);

// 2ED 242 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "a804f742-f7cf-427e-ad0a-742587328156",
    "Dan Frazier",
);

// 2ED 243 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "0c58f236-0b2c-4b71-8819-1beaea7ded17",
    "Dan Frazier",
);

// 2ED 244 — Forcefield (reprint)
const FORCEFIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCEFIELD,
    "239a5d29-95cf-468a-8b07-aea1f7dc8d52",
    "Dan Frazier",
);

// 2ED 245 — Gauntlet of Might (reprint)
const GAUNTLET_OF_MIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAUNTLET_OF_MIGHT,
    "407650c3-9388-45c3-a599-6929c7d6e5bd",
    "Christopher Rush",
);

// 2ED 246 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GLASSES_OF_URZA,
    "c8635a10-12fc-4308-8f1e-6c4bc6acd9b5",
    "Douglas Shuler",
);

// 2ED 247 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "b0d2c643-39cc-47f8-9f70-327f004c1373",
    "Mark Tedin",
);

// 2ED 248 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "c69d4007-d26b-442b-9c34-d3780c46c5f6",
    "Mark Poole",
);

// 2ED 249 — Icy Manipulator (reprint)
const ICY_MANIPULATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ICY_MANIPULATOR,
    "2a7cf252-1af0-4b03-89bc-8287b4052a23",
    "Douglas Shuler",
);

// 2ED 250 — Illusionary Mask (reprint)
const ILLUSIONARY_MASK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ILLUSIONARY_MASK,
    "a274a381-4eb0-4e27-aff4-4d94e61b726a",
    "Amy Weber",
);

// 2ED 251 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRON_STAR,
    "3cf0941f-1e23-4af6-a398-d2e96783ecca",
    "Dan Frazier",
);

// 2ED 252 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "76aaff1a-6796-4728-bdb5-bcdc79c9b98c",
    "Anson Maddocks",
);

// 2ED 253 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "88c6101a-09af-423e-881f-09aa1e01d2a2",
    "Anson Maddocks",
);

// 2ED 254 — Jade Statue (reprint)
const JADE_STATUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_STATUE,
    "a5354edc-03d7-4176-a211-174374a9d912",
    "Dan Frazier",
);

// 2ED 255 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JAYEMDAE_TOME,
    "e470c00b-57ac-48d4-b1e6-74b74872e620",
    "Mark Tedin",
);

// 2ED 256 — Juggernaut (reprint)
const JUGGERNAUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JUGGERNAUT,
    "0cde95ea-ad1a-4acb-a8bd-5457f119aeb7",
    "Dan Frazier",
);

// 2ED 257 — Kormus Bell (reprint)
const KORMUS_BELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KORMUS_BELL,
    "736e4586-a6c6-42c0-8555-5f09d214e1cb",
    "Christopher Rush",
);

// 2ED 258 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "502d77d6-c5c9-4def-80cb-7905fbbdefcb",
    "Daniel Gelon",
);

// 2ED 259 — Living Wall (reprint)
const LIVING_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_WALL,
    "3035651f-a2b5-49c1-a768-1f510a31a9d8",
    "Anson Maddocks",
);

// 2ED 260 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_VAULT,
    "778d10e6-251b-4ef3-b9b8-bc23a0d74aed",
    "Mark Tedin",
);

// 2ED 261 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "2d1ced0f-a232-4a05-aa59-6e611b52d617",
    "Quinton Hoover",
);

// 2ED 262 — Mox Emerald (reprint)
const MOX_EMERALD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_EMERALD,
    "a4db5af2-9caf-4493-b340-6d64021139e2",
    "Dan Frazier",
);

// 2ED 263 — Mox Jet (reprint)
const MOX_JET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_JET,
    "70d6c02e-0f48-4fb0-94f3-1fc92ee1814f",
    "Dan Frazier",
);

// 2ED 264 — Mox Pearl (reprint)
const MOX_PEARL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_PEARL,
    "c84e8a0e-49a7-46f6-8a37-910e32753528",
    "Dan Frazier",
);

// 2ED 265 — Mox Ruby (reprint)
const MOX_RUBY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_RUBY,
    "21b7cbae-6647-4f36-b02d-5535ac88b1a6",
    "Dan Frazier",
);

// 2ED 266 — Mox Sapphire (reprint)
const MOX_SAPPHIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_SAPPHIRE,
    "f7d82f1d-631e-4668-9d10-7bf0ee515267",
    "Dan Frazier",
);

// 2ED 267 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::NEVINYRRALS_DISK,
    "8436c720-ff96-4475-8320-d0d1e0c23f2a",
    "Mark Tedin",
);

// 2ED 268 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "9646da70-329f-41a2-9453-4ec6a9c9e7e4",
    "Jesper Myrfors",
);

// 2ED 269 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "0f047f5b-af97-4662-ab06-698a5f6f5a57",
    "Christopher Rush",
);

// 2ED 270 — Sol Ring (reprint)
const SOL_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SOL_RING,
    "e07f656c-97b5-4147-821a-edbb49f34e19",
    "Mark Tedin",
);

// 2ED 271 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "0f586dd9-bb47-411d-9652-05de4651b146",
    "Dameon Willich",
);

// 2ED 272 — Sunglasses of Urza (reprint)
const SUNGLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SUNGLASSES_OF_URZA,
    "b894acd5-818b-4ac5-bbf8-47db2ed9a825",
    "Dan Frazier",
);

// 2ED 273 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "af5534f0-485e-41f2-bcfa-d65c1a9b86bd",
    "Sandra Everingham",
);

// 2ED 274 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "5a242eb1-8625-4063-9376-a1df32547b58",
    "Anson Maddocks",
);

// 2ED 275 — Time Vault (reprint)
const TIME_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_VAULT,
    "0b64dd0f-2e99-41bd-87aa-f623582d64d0",
    "Mark Tedin",
);

// 2ED 276 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WINTER_ORB,
    "ee9eb598-d2ef-4b3d-8038-bc33dc5e123e",
    "Mark Tedin",
);

// 2ED 277 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "fb19b35e-e0b9-4575-b146-2682ad8a5175",
    "Mark Tedin",
);

// 2ED 278 — Badlands (reprint)
const BADLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BADLANDS,
    "5804dcd3-d41d-4cbd-9f8f-9736f2d37a64",
    "Rob Alexander",
);

// 2ED 279 — Bayou (reprint)
const BAYOU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BAYOU,
    "d66e43f0-1558-409f-8248-cc1d76c6bd8e",
    "Jesper Myrfors",
);

// 2ED 280 — Plateau (reprint)
const PLATEAU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLATEAU,
    "de38f96c-5d17-4cf2-9951-f0866eadd011",
    "Drew Tucker",
);

// 2ED 281 — Savannah (reprint)
const SAVANNAH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH,
    "38937c61-280e-457f-aef9-43139446163a",
    "Rob Alexander",
);

// 2ED 282 — Scrubland (reprint)
const SCRUBLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRUBLAND,
    "7e18d625-0950-4062-8d41-f8b681eff234",
    "Jesper Myrfors",
);

// 2ED 283 — Taiga (reprint)
const TAIGA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TAIGA,
    "01006833-6007-4c16-9ebb-20d31c60a57a",
    "Rob Alexander",
);

// 2ED 284 — Tropical Island (reprint)
const TROPICAL_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TROPICAL_ISLAND,
    "856bf0ba-e5a5-47eb-9a6a-111935088c31",
    "Jesper Myrfors",
);

// 2ED 285 — Tundra (reprint)
const TUNDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TUNDRA,
    "0d08f5e4-d2d3-4659-86d4-a983d80e3b2c",
    "Jesper Myrfors",
);

// 2ED 286 — Underground Sea (reprint)
const UNDERGROUND_SEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::UNDERGROUND_SEA,
    "bc98d888-4af3-43a3-b035-40c651057b6e",
    "Rob Alexander",
);

// 2ED 287 — Volcanic Island (reprint)
const VOLCANIC_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &beta::VOLCANIC_ISLAND,
    "9dc7ab05-a5f5-4a02-87e7-3c47be35b5cb",
    "Brian Snõddy",
);

// 2ED 288 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "034b047d-6363-45ca-9948-8184f822a2cb",
    "Jesper Myrfors",
);

// 2ED 289 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "0e7eede2-e682-43b5-b5b7-a61fb8e98082",
    "Jesper Myrfors",
);

// 2ED 290 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "ee7cfabc-902f-46f7-b1de-fa0a88c8f852",
    "Jesper Myrfors",
);

// 2ED 291 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "68271f76-eaf9-44cc-bb3d-5c56f36e9af9",
    "Mark Poole",
);

// 2ED 292 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "069b4d6c-7542-4a42-8822-031f02131033",
    "Mark Poole",
);

// 2ED 293 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "712dc7d6-5543-49fd-bafa-5ffb6c2bb0ce",
    "Mark Poole",
);

// 2ED 294 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "92f7a995-c648-4835-8df3-135d7472cd2d",
    "Dan Frazier",
);

// 2ED 295 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "cba6da22-2366-4f16-84ce-47f84ea14523",
    "Dan Frazier",
);

// 2ED 296 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "c78bad70-2aec-4580-a777-483d72db8d90",
    "Dan Frazier",
);

// 2ED 297 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "2c3c0f74-485e-4b21-8f41-56666a7d0005",
    "Douglas Shuler",
);

// 2ED 298 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "005a993c-5111-4364-9fba-75b3d94a8296",
    "Douglas Shuler",
);

// 2ED 299 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "987557ee-8344-4191-b85c-f9dedf4d1614",
    "Douglas Shuler",
);

// 2ED 300 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "88d0aca2-874c-4d91-8fe9-f8355d71aeb2",
    "Christopher Rush",
);

// 2ED 301 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "679aa578-3b31-4b07-98b3-e00777506e32",
    "Christopher Rush",
);

// 2ED 302 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "79bf50f3-2838-4908-8004-847ccb296fe0",
    "Christopher Rush",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANIMATE_WALL_REPRINT,
    ARMAGEDDON_REPRINT,
    BALANCE_REPRINT,
    BENALISH_HERO_REPRINT,
    BLACK_WARD_REPRINT,
    BLAZE_OF_GLORY_REPRINT,
    BLESSING_REPRINT,
    BLUE_WARD_REPRINT,
    CASTLE_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    CONSECRATE_LAND_REPRINT,
    CONVERSION_REPRINT,
    CRUSADE_REPRINT,
    DEATH_WARD_REPRINT,
    DISENCHANT_REPRINT,
    FARMSTEAD_REPRINT,
    GREEN_WARD_REPRINT,
    GUARDIAN_ANGEL_REPRINT,
    HEALING_SALVE_REPRINT,
    HOLY_ARMOR_REPRINT,
    HOLY_STRENGTH_REPRINT,
    ISLAND_SANCTUARY_REPRINT,
    KARMA_REPRINT,
    LANCE_REPRINT,
    MESA_PEGASUS_REPRINT,
    NORTHERN_PALADIN_REPRINT,
    PEARLED_UNICORN_REPRINT,
    PERSONAL_INCARNATION_REPRINT,
    PURELACE_REPRINT,
    RED_WARD_REPRINT,
    RESURRECTION_REPRINT,
    REVERSE_DAMAGE_REPRINT,
    RIGHTEOUSNESS_REPRINT,
    SAMITE_HEALER_REPRINT,
    SAVANNAH_LIONS_REPRINT,
    SERRA_ANGEL_REPRINT,
    SWORDS_TO_PLOWSHARES_REPRINT,
    VETERAN_BODYGUARD_REPRINT,
    WALL_OF_SWORDS_REPRINT,
    WHITE_KNIGHT_REPRINT,
    WHITE_WARD_REPRINT,
    WRATH_OF_GOD_REPRINT,
    AIR_ELEMENTAL_REPRINT,
    ANCESTRAL_RECALL_REPRINT,
    ANIMATE_ARTIFACT_REPRINT,
    BLUE_ELEMENTAL_BLAST_REPRINT,
    BRAINGEYSER_REPRINT,
    CLONE_REPRINT,
    CONTROL_MAGIC_REPRINT,
    COPY_ARTIFACT_REPRINT,
    COUNTERSPELL_REPRINT,
    CREATURE_BOND_REPRINT,
    DRAIN_POWER_REPRINT,
    FEEDBACK_REPRINT,
    FLIGHT_REPRINT,
    INVISIBILITY_REPRINT,
    JUMP_REPRINT,
    LIFETAP_REPRINT,
    LORD_OF_ATLANTIS_REPRINT,
    MAGICAL_HACK_REPRINT,
    MAHAMOTI_DJINN_REPRINT,
    MANA_SHORT_REPRINT,
    MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT,
    PHANTASMAL_FORCES_REPRINT,
    PHANTASMAL_TERRAIN_REPRINT,
    PHANTOM_MONSTER_REPRINT,
    PIRATE_SHIP_REPRINT,
    POWER_LEAK_REPRINT,
    POWER_SINK_REPRINT,
    PRODIGAL_SORCERER_REPRINT,
    PSIONIC_BLAST_REPRINT,
    PSYCHIC_VENOM_REPRINT,
    SEA_SERPENT_REPRINT,
    SIREN_S_CALL_REPRINT,
    SLEIGHT_OF_MIND_REPRINT,
    SPELL_BLAST_REPRINT,
    STASIS_REPRINT,
    STEAL_ARTIFACT_REPRINT,
    THOUGHTLACE_REPRINT,
    TIME_WALK_REPRINT,
    TIMETWISTER_REPRINT,
    TWIDDLE_REPRINT,
    UNSUMMON_REPRINT,
    VESUVAN_DOPPELGANGER_REPRINT,
    VOLCANIC_ERUPTION_REPRINT,
    WALL_OF_AIR_REPRINT,
    WALL_OF_WATER_REPRINT,
    WATER_ELEMENTAL_REPRINT,
    ANIMATE_DEAD_REPRINT,
    BAD_MOON_REPRINT,
    BLACK_KNIGHT_REPRINT,
    BOG_WRAITH_REPRINT,
    CONTRACT_FROM_BELOW_REPRINT,
    CURSED_LAND_REPRINT,
    DARK_RITUAL_REPRINT,
    DARKPACT_REPRINT,
    DEATHGRIP_REPRINT,
    DEATHLACE_REPRINT,
    DEMONIC_ATTORNEY_REPRINT,
    DEMONIC_HORDES_REPRINT,
    DEMONIC_TUTOR_REPRINT,
    DRAIN_LIFE_REPRINT,
    DRUDGE_SKELETONS_REPRINT,
    EVIL_PRESENCE_REPRINT,
    FEAR_REPRINT,
    FROZEN_SHADE_REPRINT,
    GLOOM_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    HYPNOTIC_SPECTER_REPRINT,
    LICH_REPRINT,
    LORD_OF_THE_PIT_REPRINT,
    MIND_TWIST_REPRINT,
    NETHER_SHADOW_REPRINT,
    NETTLING_IMP_REPRINT,
    NIGHTMARE_REPRINT,
    PARALYZE_REPRINT,
    PESTILENCE_REPRINT,
    PLAGUE_RATS_REPRINT,
    RAISE_DEAD_REPRINT,
    ROYAL_ASSASSIN_REPRINT,
    SACRIFICE_REPRINT,
    SCATHE_ZOMBIES_REPRINT,
    SCAVENGING_GHOUL_REPRINT,
    SENGIR_VAMPIRE_REPRINT,
    SIMULACRUM_REPRINT,
    SINKHOLE_REPRINT,
    TERROR_REPRINT,
    UNHOLY_STRENGTH_REPRINT,
    WALL_OF_BONE_REPRINT,
    WARP_ARTIFACT_REPRINT,
    WEAKNESS_REPRINT,
    WILL_O_THE_WISP_REPRINT,
    WORD_OF_COMMAND_REPRINT,
    ZOMBIE_MASTER_REPRINT,
    BURROWING_REPRINT,
    CHAOSLACE_REPRINT,
    DISINTEGRATE_REPRINT,
    DRAGON_WHELP_REPRINT,
    DWARVEN_DEMOLITION_TEAM_REPRINT,
    DWARVEN_WARRIORS_REPRINT,
    EARTH_ELEMENTAL_REPRINT,
    EARTHBIND_REPRINT,
    EARTHQUAKE_REPRINT,
    FALSE_ORDERS_REPRINT,
    FIRE_ELEMENTAL_REPRINT,
    FIREBALL_REPRINT,
    FIREBREATHING_REPRINT,
    FLASHFIRES_REPRINT,
    FORK_REPRINT,
    GOBLIN_BALLOON_BRIGADE_REPRINT,
    GOBLIN_KING_REPRINT,
    GRANITE_GARGOYLE_REPRINT,
    GRAY_OGRE_REPRINT,
    HILL_GIANT_REPRINT,
    HURLOON_MINOTAUR_REPRINT,
    IRONCLAW_ORCS_REPRINT,
    KELDON_WARLORD_REPRINT,
    LIGHTNING_BOLT_REPRINT,
    MANA_FLARE_REPRINT,
    MANABARBS_REPRINT,
    MONSS_GOBLIN_RAIDERS_REPRINT,
    ORCISH_ARTILLERY_REPRINT,
    ORCISH_ORIFLAMME_REPRINT,
    POWER_SURGE_REPRINT,
    RAGING_RIVER_REPRINT,
    RED_ELEMENTAL_BLAST_REPRINT,
    ROC_OF_KHER_RIDGES_REPRINT,
    ROCK_HYDRA_REPRINT,
    SEDGE_TROLL_REPRINT,
    SHATTER_REPRINT,
    SHIVAN_DRAGON_REPRINT,
    SMOKE_REPRINT,
    STONE_GIANT_REPRINT,
    STONE_RAIN_REPRINT,
    TUNNEL_REPRINT,
    TWO_HEADED_GIANT_OF_FORIYS_REPRINT,
    UTHDEN_TROLL_REPRINT,
    WALL_OF_FIRE_REPRINT,
    WALL_OF_STONE_REPRINT,
    WHEEL_OF_FORTUNE_REPRINT,
    ASPECT_OF_WOLF_REPRINT,
    BERSERK_REPRINT,
    BIRDS_OF_PARADISE_REPRINT,
    CAMOUFLAGE_REPRINT,
    CHANNEL_REPRINT,
    COCKATRICE_REPRINT,
    CRAW_WURM_REPRINT,
    ELVISH_ARCHERS_REPRINT,
    FASTBOND_REPRINT,
    FOG_REPRINT,
    FORCE_OF_NATURE_REPRINT,
    FUNGUSAUR_REPRINT,
    GAEA_S_LIEGE_REPRINT,
    GIANT_GROWTH_REPRINT,
    GIANT_SPIDER_REPRINT,
    GRIZZLY_BEARS_REPRINT,
    HURRICANE_REPRINT,
    ICE_STORM_REPRINT,
    INSTILL_ENERGY_REPRINT,
    IRONROOT_TREEFOLK_REPRINT,
    KUDZU_REPRINT,
    LEY_DRUID_REPRINT,
    LIFEFORCE_REPRINT,
    LIFELACE_REPRINT,
    LIVING_ARTIFACT_REPRINT,
    LIVING_LANDS_REPRINT,
    LLANOWAR_ELVES_REPRINT,
    LURE_REPRINT,
    NATURAL_SELECTION_REPRINT,
    REGENERATION_REPRINT,
    REGROWTH_REPRINT,
    SCRYB_SPRITES_REPRINT,
    SHANODIN_DRYADS_REPRINT,
    STREAM_OF_LIFE_REPRINT,
    THICKET_BASILISK_REPRINT,
    TIMBER_WOLVES_REPRINT,
    TRANQUILITY_REPRINT,
    TSUNAMI_REPRINT,
    VERDURAN_ENCHANTRESS_REPRINT,
    WALL_OF_BRAMBLES_REPRINT,
    WALL_OF_ICE_REPRINT,
    WALL_OF_WOOD_REPRINT,
    WANDERLUST_REPRINT,
    WAR_MAMMOTH_REPRINT,
    WEB_REPRINT,
    WILD_GROWTH_REPRINT,
    ANKH_OF_MISHRA_REPRINT,
    BASALT_MONOLITH_REPRINT,
    BLACK_LOTUS_REPRINT,
    BLACK_VISE_REPRINT,
    CELESTIAL_PRISM_REPRINT,
    CHAOS_ORB_REPRINT,
    CLOCKWORK_BEAST_REPRINT,
    CONSERVATOR_REPRINT,
    COPPER_TABLET_REPRINT,
    CRYSTAL_ROD_REPRINT,
    CYCLOPEAN_TOMB_REPRINT,
    DINGUS_EGG_REPRINT,
    DISRUPTING_SCEPTER_REPRINT,
    FORCEFIELD_REPRINT,
    GAUNTLET_OF_MIGHT_REPRINT,
    GLASSES_OF_URZA_REPRINT,
    HELM_OF_CHATZUK_REPRINT,
    HOWLING_MINE_REPRINT,
    ICY_MANIPULATOR_REPRINT,
    ILLUSIONARY_MASK_REPRINT,
    IRON_STAR_REPRINT,
    IVORY_CUP_REPRINT,
    JADE_MONOLITH_REPRINT,
    JADE_STATUE_REPRINT,
    JAYEMDAE_TOME_REPRINT,
    JUGGERNAUT_REPRINT,
    KORMUS_BELL_REPRINT,
    LIBRARY_OF_LENG_REPRINT,
    LIVING_WALL_REPRINT,
    MANA_VAULT_REPRINT,
    MEEKSTONE_REPRINT,
    MOX_EMERALD_REPRINT,
    MOX_JET_REPRINT,
    MOX_PEARL_REPRINT,
    MOX_RUBY_REPRINT,
    MOX_SAPPHIRE_REPRINT,
    NEVINYRRALS_DISK_REPRINT,
    OBSIANUS_GOLEM_REPRINT,
    ROD_OF_RUIN_REPRINT,
    SOL_RING_REPRINT,
    SOUL_NET_REPRINT,
    SUNGLASSES_OF_URZA_REPRINT,
    THE_HIVE_REPRINT,
    THRONE_OF_BONE_REPRINT,
    TIME_VAULT_REPRINT,
    WINTER_ORB_REPRINT,
    WOODEN_SPHERE_REPRINT,
    BADLANDS_REPRINT,
    BAYOU_REPRINT,
    PLATEAU_REPRINT,
    SAVANNAH_REPRINT,
    SCRUBLAND_REPRINT,
    TAIGA_REPRINT,
    TROPICAL_ISLAND_REPRINT,
    TUNDRA_REPRINT,
    UNDERGROUND_SEA_REPRINT,
    VOLCANIC_ISLAND_REPRINT,
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
