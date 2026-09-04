//! Fifth Edition currently contributes no catalog definitions.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::fallen_empires as catalog_fem;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1994::the_dark as catalog_drk;
use crate::card::sets::y1995::homelands as catalog_hml;
use crate::card::sets::y1995::ice_age as catalog_ice;

// 5ED 1 — Abbey Gargoyles (reprint)
const ABBEY_GARGOYLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::ABBEY_GARGOYLES,
    "29b5b70d-e5b9-4bc3-87b1-51dc6e5085a5",
    "Christopher Rush",
);

// 5ED 2 — Akron Legionnaire (reprint)
const AKRON_LEGIONNAIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AKRON_LEGIONNAIRE,
    "7b658243-267b-473f-acce-b4e211945c67",
    "Mark Poole",
);

// 5ED 3 — Alabaster Potion (reprint)
const ALABASTER_POTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ALABASTER_POTION,
    "950acce3-b079-49fc-8016-75fcdbca9b82",
    "Harold McNeill",
);

// 5ED 4 — Angry Mob (reprint)
const ANGRY_MOB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::ANGRY_MOB,
    "fe9ebe90-dad9-44b0-a2fa-13fc77953502",
    "Drew Tucker",
);

// 5ED 5 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "8d7f102d-18ff-420b-9b0b-323daad759de",
    "Richard Kane Ferguson",
);

// 5ED 6 — Arenson's Aura (reprint)
const ARENSON_S_AURA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ARENSON_S_AURA,
    "ba2bf14c-f04d-4380-b0d5-25028ddd999b",
    "D. Alexander Gregory",
);

// 5ED 7 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ARMAGEDDON,
    "8949d6f8-6491-489a-916d-7007deaf0371",
    "Jesper Myrfors",
);

// 5ED 8 — Armor of Faith (reprint)
const ARMOR_OF_FAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ARMOR_OF_FAITH,
    "0bdb3c08-aa9e-45d4-ae82-00019aee03b6",
    "Anson Maddocks",
);

// 5ED 9 — Aysen Bureaucrats (reprint)
const AYSEN_BUREAUCRATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::AYSEN_BUREAUCRATS,
    "b01c564f-d997-4667-93af-c8495d8764e6",
    "Adrian Smith",
);

// 5ED 10 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "32dcd608-ef94-4047-841d-5c3471375d5d",
    "Douglas Shuler",
);

// 5ED 11 — Blessed Wine (reprint)
const BLESSED_WINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BLESSED_WINE,
    "aee815d6-ce79-42ff-9d18-b2f92671d1c4",
    "Kaja Foglio",
);

// 5ED 12 — Blinking Spirit (reprint)
const BLINKING_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BLINKING_SPIRIT,
    "f6ff7626-de7d-443a-8eb7-fccaba6f2336",
    "Allen Williams",
);

// 5ED 13 — Brainwash (reprint)
const BRAINWASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BRAINWASH,
    "0afcbd6d-2b83-4393-9fc8-fe91f86117fe",
    "Terese Nielsen",
);

// 5ED 14 — Caribou Range (reprint)
const CARIBOU_RANGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::CARIBOU_RANGE,
    "686022b1-359f-4348-a096-992140fff460",
    "Una Fricker",
);

// 5ED 15 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "bcf8196e-3dd6-480f-a431-0e17005f7168",
    "David O'Connor",
);

// 5ED 16 — Circle of Protection: Artifacts (reprint)
const CIRCLE_OF_PROTECTION_ARTIFACTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CIRCLE_OF_PROTECTION_ARTIFACTS,
    "77cca7a8-439a-4388-8daf-6f04e78f5e3a",
    "Pete Venters",
);

// 5ED 17 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "8160786c-288a-4160-8c8b-c575a61e00fc",
    "Gerry Grace",
);

// 5ED 18 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "2c8b94bd-fffd-4c8a-b705-2b8e60cb8d68",
    "Gerry Grace",
);

// 5ED 19 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "3d73bc89-887b-4ed8-ab68-20214b8aabcf",
    "Gerry Grace",
);

// 5ED 20 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "4af7af94-75b1-45ab-8608-6c935cedad77",
    "Gerry Grace",
);

// 5ED 21 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "1477ea65-76b8-40b7-985d-02bc4aec9210",
    "Gerry Grace",
);

// 5ED 22 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRUSADE,
    "3745725e-2bae-4c9b-bece-9e6b5f45c3c9",
    "D. Alexander Gregory",
);

// 5ED 23 — D'Avenant Archer (reprint)
const DAVENANT_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DAVENANT_ARCHER,
    "d57621be-3420-49f5-87a3-a9f66fc7c2e1",
    "Douglas Shuler",
);

// 5ED 24 — Death Speakers (reprint)
const DEATH_SPEAKERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::DEATH_SPEAKERS,
    "31f70687-1d96-4d85-bed6-f08edef223cd",
    "Andrew Robinson",
);

// 5ED 25 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "1502b21a-90e0-48af-ad55-b1edf2d0582c",
    "Mark Poole",
);

// 5ED 26 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "18720641-3a34-48f1-954e-49a703f12578",
    "Brian Snõddy",
);

// 5ED 27 — Divine Offering (reprint)
const DIVINE_OFFERING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DIVINE_OFFERING,
    "5c53562e-6b8f-484d-a083-2c635c2f222b",
    "Jeff A. Menges",
);

// 5ED 28 — Divine Transformation (reprint)
const DIVINE_TRANSFORMATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DIVINE_TRANSFORMATION,
    "5b3994c6-8e84-4d06-ab46-fb5f061cc34d",
    "NéNé Thomas",
);

// 5ED 29 — Dust to Dust (reprint)
const DUST_TO_DUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::DUST_TO_DUST,
    "6e8a9a5c-0e5f-4498-88d8-6aac465923b1",
    "Doug Keith",
);

// 5ED 30 — Eye for an Eye (reprint)
const EYE_FOR_AN_EYE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EYE_FOR_AN_EYE,
    "d28234a6-0e6b-4555-b5f9-edcc7b29194b",
    "Mark Poole",
);

// 5ED 31 — Greater Realm of Preservation (reprint)
const GREATER_REALM_OF_PRESERVATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GREATER_REALM_OF_PRESERVATION,
    "ab8eb77b-cdf7-44c6-8dce-898cfa173d66",
    "Steve Luke",
);

// 5ED 32 — Heal (reprint)
const HEAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::HEAL,
    "a20676ce-fd87-4a63-9b96-02a26015edc2",
    "Mark Tedin",
);

// 5ED 33 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "6855af25-0d35-4bf7-92e7-9dffdafa1eca",
    "Zina Saunders",
);

// 5ED 34 — Hipparion (reprint)
const HIPPARION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::HIPPARION,
    "f26fc403-b8af-499a-8cbc-4077ff97b8b0",
    "Margaret Organ-Kean",
);

// 5ED 35 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "f9f938d4-5964-4f55-a2ce-0ca7e25fe565",
    "Anson Maddocks",
);

// 5ED 36 — Icatian Phalanx (reprint)
const ICATIAN_PHALANX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ICATIAN_PHALANX,
    "fabf21ed-0262-4909-8b0f-c2c0e82830b0",
    "Kaja Foglio",
);

// 5ED 37 — Icatian Scout (reprint)
const ICATIAN_SCOUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ICATIAN_SCOUT,
    "dc02133f-71ff-45a1-9733-2ee3584dbbe5",
    "Rob Alexander",
);

// 5ED 38 — Icatian Town (reprint)
const ICATIAN_TOWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ICATIAN_TOWN,
    "6e06788f-e87b-4071-ba7d-e0253a52132d",
    "Tom Wänerstrand",
);

// 5ED 39 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "c5521bab-3252-4f81-9e9e-783160a5718c",
    "Mark Poole",
);

// 5ED 40 — Ivory Guardians (reprint)
const IVORY_GUARDIANS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::IVORY_GUARDIANS,
    "563dd5f5-6e37-4b36-b41b-1f4810e6ef8c",
    "Adam Rex",
);

// 5ED 41 — Justice (reprint)
const JUSTICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::JUSTICE,
    "bb11e16f-edf5-4e0e-862b-b055fd800682",
    "Ruth Thompson",
);

// 5ED 42 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "c7cd8fd6-f30d-4e79-af68-688c2cfd39ea",
    "Bob Eggleton",
);

// 5ED 43 — Kismet (reprint)
const KISMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KISMET,
    "cf81644f-6a93-4183-9f71-c3505cca6db4",
    "Kaja Foglio",
);

// 5ED 44 — Kjeldoran Royal Guard (reprint)
const KJELDORAN_ROYAL_GUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KJELDORAN_ROYAL_GUARD,
    "bafc9583-d16c-4a3b-88c5-2938038784bc",
    "Allen Williams",
);

// 5ED 45 — Kjeldoran Skycaptain (reprint)
const KJELDORAN_SKYCAPTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KJELDORAN_SKYCAPTAIN,
    "8c1f4c4d-dbb6-42b9-a56f-5ee119f68cf6",
    "Mark Poole",
);

// 5ED 46 — Mesa Falcon (reprint)
const MESA_FALCON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::MESA_FALCON,
    "bfd27e15-688b-43aa-9d2a-8cd35e960a9d",
    "Mark Poole",
);

// 5ED 47 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "552089f3-1ae4-4f73-a19c-731ef98e1979",
    "Melissa A. Benson",
);

// 5ED 48 — Order of the Sacred Torch (reprint)
const ORDER_OF_THE_SACRED_TORCH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ORDER_OF_THE_SACRED_TORCH,
    "cc09bb52-c31c-4afd-80ea-a48a46f82f00",
    "Ruth Thompson",
);

// 5ED 49 — Order of the White Shield (reprint)
const ORDER_OF_THE_WHITE_SHIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ORDER_OF_THE_WHITE_SHIELD,
    "8ce8581f-d953-4600-b08f-f02c3eccdbcc",
    "Ruth Thompson",
);

// 5ED 50 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "ce33ef5b-a0ff-459c-a9d4-a0a00ac66b31",
    "David A. Cherry",
);

// 5ED 51 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "b3a7836f-5a67-4915-8c59-0dd02f8bc0a9",
    "Kev Walker",
);

// 5ED 52 — Pikemen (reprint)
const PIKEMEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::PIKEMEN,
    "18243ac8-6097-4f2c-8064-3dab48038e4a",
    "Dan Frazier",
);

// 5ED 53 — Prismatic Ward (reprint)
const PRISMATIC_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PRISMATIC_WARD,
    "24b8ce55-2fb1-4330-9db4-146a120aed81",
    "Zina Saunders",
);

// 5ED 54 — Repentant Blacksmith (reprint)
const REPENTANT_BLACKSMITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::REPENTANT_BLACKSMITH,
    "b0bd3587-301e-49ea-a589-1091c880145b",
    "Drew Tucker",
);

// 5ED 55 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "b6987f7d-d7a0-4b75-87ca-3e7b681d449c",
    "Thomas Gianni",
);

// 5ED 56 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "b7a76892-2806-4fb4-882b-6de2869839c2",
    "Mike Dringenberg",
);

// 5ED 57 — Sacred Boon (reprint)
const SACRED_BOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SACRED_BOON,
    "0abb4791-d2ad-4266-a900-aef8a802b3e5",
    "Mike Raabe",
);

// 5ED 58 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "ee3b2aaa-f04e-4a2a-8d5f-b3cc54605b28",
    "Tom Wänerstrand",
);

// 5ED 59 — Seraph (reprint)
const SERAPH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SERAPH,
    "f2de654e-ee5c-4ae6-bd4c-c564ed6a5e83",
    "D. Alexander Gregory",
);

// 5ED 60 — Serra Bestiary (reprint)
const SERRA_BESTIARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::SERRA_BESTIARY,
    "c92bb33d-567c-4504-8e3c-5152fb61d100",
    "Steve Luke",
);

// 5ED 61 — Serra Paladin (reprint)
const SERRA_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::SERRA_PALADIN,
    "a48d282d-a777-4587-ab23-c30a4dba0494",
    "Pete Venters",
);

// 5ED 62 — Shield Bearer (reprint)
const SHIELD_BEARER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SHIELD_BEARER,
    "9b7cfc0f-b9cc-4405-9c2d-2788d4ab49b4",
    "Dan Frazier",
);

// 5ED 63 — Shield Wall (reprint)
const SHIELD_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SHIELD_WALL,
    "efb0d954-f3d1-4ba6-b9ee-778e09f4eea7",
    "Scott Kirschner",
);

// 5ED 64 — Spirit Link (reprint)
const SPIRIT_LINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SPIRIT_LINK,
    "3327949d-3b11-4409-911c-4289a89b488f",
    "Kaja Foglio",
);

// 5ED 65 — Truce (reprint)
const TRUCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::TRUCE,
    "de4f53fc-69d7-49fb-885b-4cc02bf5facc",
    "Donato Giancola",
);

// 5ED 66 — Tundra Wolves (reprint)
const TUNDRA_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TUNDRA_WOLVES,
    "1f2f982d-cbd0-466e-8804-b055374a9dec",
    "Quinton Hoover",
);

// 5ED 67 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "bada1968-8bc5-4c75-bd60-91015961907b",
    "Brian Snõddy",
);

// 5ED 68 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_KNIGHT,
    "ef8529a0-0259-4ccd-a481-8ad1b5feadf9",
    "Daniel Gelon",
);

// 5ED 69 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WRATH_OF_GOD,
    "def142a8-cc73-4bd2-a1a8-ae47a0c9c948",
    "Quinton Hoover",
);

// 5ED 70 — Aether Storm (reprint)
const AETHER_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::AETHER_STORM,
    "79d6b89e-f24a-4d32-a6f7-8466f46d9ad0",
    "Mark Tedin",
);

// 5ED 71 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "692bd3dd-4aa8-44a8-9f96-fd6e8698b6a2",
    "D. Alexander Gregory",
);

// 5ED 72 — Anti-Magic Aura (reprint)
const ANTI_MAGIC_AURA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ANTI_MAGIC_AURA,
    "54642f0e-2d5f-49eb-8181-054c84038072",
    "Zak Plucinski",
);

// 5ED 73 — Azure Drake (reprint)
const AZURE_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AZURE_DRAKE,
    "9ce9086e-112a-45e6-854e-70cab2b008a3",
    "Janine Johnston",
);

// 5ED 74 — Binding Grasp (reprint)
const BINDING_GRASP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BINDING_GRASP,
    "6c9af233-f0b9-489f-8561-77bff8280814",
    "Jeff Miracola",
);

// 5ED 75 — Boomerang (reprint)
const BOOMERANG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BOOMERANG,
    "ee04c924-3806-44a4-9b1b-d8e1a38432f0",
    "Alan Rabinowitz",
);

// 5ED 75s — Boomerang (alternate printing)
const BOOMERANG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_leg::BOOMERANG,
    1,
    "fffc0bfd-8e64-44bf-ae0a-5d2ee54c58df",
    "Richard Kane Ferguson",
);

// 5ED 76 — Brainstorm (reprint)
const BRAINSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BRAINSTORM,
    "1d6603e3-0680-4ba3-951b-cd9919eefd4f",
    "Christopher Rush",
);

// 5ED 77 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "b975289d-d8b8-46b4-8c60-d6ed4b594519",
    "Hannibal King",
);

// 5ED 78 — Dance of Many (reprint)
const DANCE_OF_MANY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::DANCE_OF_MANY,
    "51ff8402-7bfe-441a-8c6e-6a9ee9608911",
    "Sandra Everingham",
);

// 5ED 79 — Dandân (reprint)
const DANDAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DANDAN,
    "9ac60e8c-ef5b-4893-b3e5-4a54cb0a0d3a",
    "Drew Tucker",
);

// 5ED 80 — Dark Maze (reprint)
const DARK_MAZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::DARK_MAZE,
    "74fe0349-a114-4bb8-a9ae-97d0f8f46ddb",
    "David Seeley",
);

// 5ED 81 — Deflection (reprint)
const DEFLECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::DEFLECTION,
    "7a9b1251-17fe-4f00-83cf-4947ac365af6",
    "Mike Raabe",
);

// 5ED 82 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "582d8496-f319-4666-a6cd-160ff33578db",
    "Jerry Tiritilli",
);

// 5ED 83 — Energy Flux (reprint)
const ENERGY_FLUX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ENERGY_FLUX,
    "5f75dea2-aec1-4674-8db9-4759270296ec",
    "Kaja Foglio",
);

// 5ED 84 — Enervate (reprint)
const ENERVATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ENERVATE,
    "5357f9dc-6eaf-4dc3-a1b0-ef63fd0b1367",
    "Allen Williams",
);

// 5ED 85 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "1d452de7-3f44-4594-bb24-2178812da9d6",
    "Quinton Hoover",
);

// 5ED 86 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "c74e5168-9503-4cf9-8a66-29b68ec2092c",
    "Jerry Tiritilli",
);

// 5ED 87 — Flood (reprint)
const FLOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FLOOD,
    "2aefbeae-ac72-4a13-8898-8d1e42a633a6",
    "Dennis Detwiller",
);

// 5ED 88 — Force Spike (reprint)
const FORCE_SPIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FORCE_SPIKE,
    "ba23d540-8c2d-4a42-b4c0-86f0988bd1ce",
    "John Matson",
);

// 5ED 89 — Forget (reprint)
const FORGET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::FORGET,
    "fceaa7b0-b11d-47d6-9cdf-7c9fa85bea20",
    "Mike Kimble",
);

// 5ED 90 — Gaseous Form (reprint)
const GASEOUS_FORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GASEOUS_FORM,
    "b28477df-667a-4233-b90a-ee30e5bc29fc",
    "Doug Keith",
);

// 5ED 91 — Glacial Wall (reprint)
const GLACIAL_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::GLACIAL_WALL,
    "5797da37-ece2-40d0-ba2a-c47ce55e3744",
    "Greg Simanson",
);

// 5ED 92 — Homarid Warrior (reprint)
const HOMARID_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::HOMARID_WARRIOR,
    "80f20d8e-bbdf-4af8-9505-d5604b04ad72",
    "Pete Venters",
);

// 5ED 93 — Hurkyl's Recall (reprint)
const HURKYLS_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::HURKYLS_RECALL,
    "b62c7e94-25ef-4859-88db-05b085744dc0",
    "NéNé Thomas",
);

// 5ED 94 — Hydroblast (reprint)
const HYDROBLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::HYDROBLAST,
    "89a30b8f-6594-42ea-8e60-13ccb1e7efc0",
    "Kaja Foglio",
);

// 5ED 95 — Juxtapose (reprint)
const JUXTAPOSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::JUXTAPOSE,
    "a60dc7c3-4a6c-4c17-9025-29341a3afee1",
    "Justin Hampton",
);

// 5ED 96 — Krovikan Sorcerer (reprint)
const KROVIKAN_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KROVIKAN_SORCERER,
    "e91568ea-cef0-4355-8029-840f5e621ae8",
    "Pat Lewis",
);

// 5ED 97 — Labyrinth Minotaur (reprint)
const LABYRINTH_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::LABYRINTH_MINOTAUR,
    "faa8c435-55c1-46af-bb93-3f8ce81b1544",
    "Anson Maddocks",
);

// 5ED 98 — Leviathan (reprint)
const LEVIATHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::LEVIATHAN,
    "a4e96456-93bf-4d28-9a4b-5bc24ae07fc2",
    "Mark Tedin",
);

// 5ED 99 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "af066a2d-d357-45a7-90d8-2c34b6d0ebf8",
    "Mike Dringenberg",
);

// 5ED 100 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "7eaad8e0-947f-47d9-b78f-b0f2a5a06906",
    "Melissa A. Benson",
);

// 5ED 101 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "c2349d53-d9f0-450f-be96-463791ee7aab",
    "Julie Baroh",
);

// 5ED 102 — Magus of the Unseen (reprint)
const MAGUS_OF_THE_UNSEEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MAGUS_OF_THE_UNSEEN,
    "3adf168a-039e-4f8f-8ca4-3f364eef373d",
    "Kaja Foglio",
);

// 5ED 103 — Memory Lapse (reprint)
const MEMORY_LAPSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::MEMORY_LAPSE,
    "9010e5e2-fd32-4f2f-aa68-1dbe58c078a2",
    "Mark Tedin",
);

// 5ED 104 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "5f7003be-4464-4eab-a77d-ab0346643613",
    "John Matson",
);

// 5ED 105 — Mind Bomb (reprint)
const MIND_BOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MIND_BOMB,
    "3c46d040-58c2-4f7a-8b66-fea33fcb0e11",
    "Mark Tedin",
);

// 5ED 106 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "021b26cb-ec7d-464b-b835-82e60e86f827",
    "Mark Poole",
);

// 5ED 107 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "13c6541b-9f23-48f1-a9bb-081ad7022fd4",
    "David A. Cherry",
);

// 5ED 108 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "c2a8a6c0-1a17-4b7e-b1e6-cdd52ee1ece8",
    "Rebecca Guay",
);

// 5ED 109 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "b7ad5912-6d1e-4ad3-9911-0c768417d6d4",
    "Tom Wänerstrand",
);

// 5ED 110 — Portent (reprint)
const PORTENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PORTENT,
    "7e317e0f-4c6e-4ede-846a-fb76fa77c6bf",
    "Liz Danforth",
);

// 5ED 111 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "acb32bc8-ae9f-4484-b2de-bce35e9f5df5",
    "Richard Thomas",
);

// 5ED 112 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "c743b0fb-a4cb-40a6-94d9-2318386d0afb",
    "Douglas Shuler",
);

// 5ED 113 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "004b6463-0aef-4419-b4f2-dc3fa6eee901",
    "Brian Snõddy",
);

// 5ED 114 — Ray of Command (reprint)
const RAY_OF_COMMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::RAY_OF_COMMAND,
    "6e498567-2bb0-4622-8250-a56b9ff79a7e",
    "Harold McNeill",
);

// 5ED 115 — Recall (reprint)
const RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RECALL,
    "dc53dd70-e2a7-4203-ae00-240694d7220e",
    "Richard Kane Ferguson",
);

// 5ED 116 — Reef Pirates (reprint)
const REEF_PIRATES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::REEF_PIRATES,
    "2d712139-bf12-4f7b-bb9b-34d15fac56df",
    "Tom Wänerstrand",
);

// 5ED 117 — Remove Soul (reprint)
const REMOVE_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::REMOVE_SOUL,
    "fd6bbb81-b830-4b22-be9a-852d9edbda21",
    "Mike Dringenberg",
);

// 5ED 118 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "e170bb1e-f2eb-4631-af95-ada585b7ef57",
    "Ian Miller",
);

// 5ED 119 — Sea Spirit (reprint)
const SEA_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SEA_SPIRIT,
    "08933cca-6ed1-43da-a539-355ded52c5b6",
    "DiTerlizzi",
);

// 5ED 120 — Sea Sprite (reprint)
const SEA_SPRITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::SEA_SPRITE,
    "4cec37c5-b927-47ae-8cd6-3eddd55a3472",
    "Rebecca Guay",
);

// 5ED 121 — Seasinger (reprint)
const SEASINGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::SEASINGER,
    "851545d3-cc23-40da-84c5-93da0dd14a8d",
    "John Matson",
);

// 5ED 122 — Segovian Leviathan (reprint)
const SEGOVIAN_LEVIATHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SEGOVIAN_LEVIATHAN,
    "b5873c47-dfed-4ffb-a472-2cb304934ad9",
    "Melissa A. Benson",
);

// 5ED 123 — Sibilant Spirit (reprint)
const SIBILANT_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SIBILANT_SPIRIT,
    "9382b01d-4ff5-4e33-ab05-412ad694b85f",
    "Ron Spencer",
);

// 5ED 124 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "3cdb4f1c-6754-4269-8fac-d4edc02c8e00",
    "Mark Poole",
);

// 5ED 125 — Soul Barrier (reprint)
const SOUL_BARRIER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SOUL_BARRIER,
    "dfd50e68-2063-4fae-bb56-eb224d43b147",
    "Harold McNeill",
);

// 5ED 126 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "70e4584f-6e44-4ff8-8313-c8791e0156af",
    "Greg Simanson",
);

// 5ED 127 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STASIS,
    "96570023-a9d6-4536-ac11-f0baad041d7c",
    "Fay Jones",
);

// 5ED 128 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "2e08d3b2-9c85-4482-b110-5f399672e550",
    "John Coulthart",
);

// 5ED 129 — Time Elemental (reprint)
const TIME_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TIME_ELEMENTAL,
    "011b1e44-776a-44b9-a976-777e49ae628e",
    "Amy Weber",
);

// 5ED 130 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "9a9fa429-a3d5-418d-b94d-ef534b5b86f0",
    "Rob Alexander",
);

// 5ED 131 — Unstable Mutation (reprint)
const UNSTABLE_MUTATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::UNSTABLE_MUTATION,
    "127abfdc-8d9e-4ba3-8f31-c13a54831824",
    "Charles Gillespie",
);

// 5ED 132 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "3c830b54-0441-4df2-87de-a579dc734d97",
    "Douglas Shuler",
);

// 5ED 133 — Updraft (reprint)
const UPDRAFT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::UPDRAFT,
    "7b4c1316-d6df-4bc4-9bfe-0458ff7c2908",
    "John Matson",
);

// 5ED 134 — Vodalian Soldiers (reprint)
const VODALIAN_SOLDIERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::VODALIAN_SOLDIERS,
    "1e0d204d-e495-4067-9c38-1a895824f95a",
    "Melissa A. Benson",
);

// 5ED 135 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "dc2a53c7-99fa-4183-a173-f84ec06088b2",
    "Richard Kane Ferguson",
);

// 5ED 136 — Wind Spirit (reprint)
const WIND_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::WIND_SPIRIT,
    "b70495b9-3fec-4eea-addf-1f401a1f49ed",
    "Kaja Foglio",
);

// 5ED 137 — Zephyr Falcon (reprint)
const ZEPHYR_FALCON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ZEPHYR_FALCON,
    "6d11923e-98c6-4041-b115-f4847fb71149",
    "Heather Hudson",
);

// 5ED 138 — Zur's Weirding (reprint)
const ZUR_S_WEIRDING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ZUR_S_WEIRDING,
    "fded7b01-2920-4b94-a511-088b521de9f7",
    "Liz Danforth",
);

// 5ED 139 — Abyssal Specter (reprint)
const ABYSSAL_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ABYSSAL_SPECTER,
    "2be32aca-94d0-46df-9ca9-503a51416b14",
    "George Pratt",
);

// 5ED 140 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "1f2cd314-8f99-4443-ae86-a967effc7490",
    "Anson Maddocks",
);

// 5ED 141 — Ashes to Ashes (reprint)
const ASHES_TO_ASHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::ASHES_TO_ASHES,
    "7240709d-a469-4801-82bd-f5db50859763",
    "Doug Keith",
);

// 5ED 141s — Ashes to Ashes (alternate printing)
const ASHES_TO_ASHES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::ASHES_TO_ASHES,
    1,
    "73d28603-b116-4948-a46f-b95ae9118d9e",
    "Drew Tucker",
);

// 5ED 142 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "6bb6d665-42f5-4f44-8797-16c0351ab3c0",
    "Gary Leach",
);

// 5ED 143 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_KNIGHT,
    "a03b6221-2c85-44c0-82f1-b2b9e2c83c80",
    "Adrian Smith",
);

// 5ED 144 — Blight (reprint)
const BLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLIGHT,
    "affd5566-9ec6-4713-a804-cec9b13b1da1",
    "Ian Miller",
);

// 5ED 145 — Bog Imp (reprint)
const BOG_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_IMP,
    "c11924b2-c290-47a1-803d-b00c433cf840",
    "Ron Spencer",
);

// 5ED 146 — Bog Rats (reprint)
const BOG_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_RATS,
    "3e325363-1ab9-4b44-9c00-0758830289e8",
    "Ron Spencer",
);

// 5ED 147 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "5ae74c5b-1090-4910-87a4-81dbffd1fd69",
    "Jeff A. Menges",
);

// 5ED 147s — Bog Wraith (alternate printing)
const BOG_WRAITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::BOG_WRAITH,
    1,
    "fe305196-fd9d-4f92-a37c-6c4fe5abad1f",
    "Ted Naifeh",
);

// 5ED 148 — Breeding Pit (reprint)
const BREEDING_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::BREEDING_PIT,
    "37c9b663-5c02-4221-9efb-59841dfd2188",
    "Adrian Smith",
);

// 5ED 149 — Broken Visage (reprint)
const BROKEN_VISAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::BROKEN_VISAGE,
    "824823fb-5ae1-48b1-bc46-e452afa73cd8",
    "Margaret Organ-Kean",
);

// 5ED 150 — Carrion Ants (reprint)
const CARRION_ANTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CARRION_ANTS,
    "21bf491b-b876-4453-8c85-8d7c419b0900",
    "John Coulthart",
);

// 5ED 151 — Cloak of Confusion (reprint)
const CLOAK_OF_CONFUSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::CLOAK_OF_CONFUSION,
    "430a4e23-d13c-4413-9032-f18350a128da",
    "Margaret Organ-Kean",
);

// 5ED 152 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "39d9801b-9707-4868-bde1-39960b761992",
    "Jesper Myrfors",
);

// 5ED 153 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "fae25afd-0d16-431c-a85e-7ac91cef9050",
    "Clint Langley",
);

// 5ED 154 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "934a6704-3fb5-4259-96e0-1f93a45b3f8c",
    "Anson Maddocks",
);

// 5ED 155 — Derelor (reprint)
const DERELOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DERELOR,
    "b050783f-945e-4d61-a896-9a5b79ae7982",
    "Anson Maddocks",
);

// 5ED 156 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_LIFE,
    "41a9b3b2-4ac9-4e50-bcd2-8831d0739e85",
    "Andrew Robinson",
);

// 5ED 157 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "127bb737-85ed-4de3-9a1d-ec789d54b7cd",
    "Ian Miller",
);

// 5ED 158 — Erg Raiders (reprint)
const ERG_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ERG_RAIDERS,
    "dcc87da4-8f68-4c40-afdb-2dfcc075ce91",
    "Stuart Griffin",
);

// 5ED 159 — Evil Eye of Orms-by-Gore (reprint)
const EVIL_EYE_OF_ORMS_BY_GORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::EVIL_EYE_OF_ORMS_BY_GORE,
    "65c7e01b-68a9-4380-94b6-d687e2c2ee86",
    "George Pratt",
);

// 5ED 160 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "3042fdde-595d-4f60-be59-364eedecf3bd",
    "Bob Eggleton",
);

// 5ED 161 — Fallen Angel (reprint)
const FALLEN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FALLEN_ANGEL,
    "68022e3e-99d0-4f14-9ef8-17f27a157edd",
    "Anson Maddocks",
);

// 5ED 162 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "c917d05d-395d-4e2f-aecc-616ae9aa9b5b",
    "Doug Keith",
);

// 5ED 163 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "cbd0b4ff-f49f-4079-991a-f66d1220235d",
    "DiTerlizzi",
);

// 5ED 164 — Funeral March (reprint)
const FUNERAL_MARCH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::FUNERAL_MARCH,
    "96e7bbae-1438-4a47-9c14-c4b6d8702ec4",
    "John Coulthart",
);

// 5ED 165 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "6a6fff23-21c9-4519-b84a-59868a918996",
    "Douglas Shuler",
);

// 5ED 166 — Greater Werewolf (reprint)
const GREATER_WEREWOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::GREATER_WEREWOLF,
    "34b77373-6066-436b-aa60-4cb0f8077599",
    "Dennis Detwiller",
);

// 5ED 167 — Hecatomb (reprint)
const HECATOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::HECATOMB,
    "43671b63-d38e-4ef5-94bd-300cda82a88b",
    "George Pratt",
);

// 5ED 168 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "1a88d96c-6192-48e6-9a3b-97eff587e115",
    "John Coulthart",
);

// 5ED 169 — Initiates of the Ebon Hand (reprint)
const INITIATES_OF_THE_EBON_HAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::INITIATES_OF_THE_EBON_HAND,
    "c83085df-1ffb-4178-9fc0-fd347196673f",
    "Heather Hudson",
);

// 5ED 169s — Initiates of the Ebon Hand (alternate printing)
const INITIATES_OF_THE_EBON_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_fem::INITIATES_OF_THE_EBON_HAND,
    1,
    "0c4a92a3-4df5-4d83-b015-59859e6bccc8",
    "Kaja Foglio",
);

// 5ED 170 — Kjeldoran Dead (reprint)
const KJELDORAN_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KJELDORAN_DEAD,
    "590c219d-536a-47a4-93a4-b77ce808a024",
    "Melissa A. Benson",
);

// 5ED 171 — Knight of Stromgald (reprint)
const KNIGHT_OF_STROMGALD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KNIGHT_OF_STROMGALD,
    "b84c73dd-3a80-4355-a71d-c6de99cb11cb",
    "Mark Poole",
);

// 5ED 172 — Krovikan Fetish (reprint)
const KROVIKAN_FETISH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KROVIKAN_FETISH,
    "8db46edd-d2d3-4d3e-beae-f8d01bb5078e",
    "Heather Hudson",
);

// 5ED 173 — Leshrac's Rite (reprint)
const LESHRAC_S_RITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::LESHRAC_S_RITE,
    "ff6aee0b-1039-496f-a096-2d4d71621d99",
    "Mike Raabe",
);

// 5ED 174 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "3b4c083f-d619-4913-aa7b-d345e3bdb1c4",
    "Mark Tedin",
);

// 5ED 175 — Lost Soul (reprint)
const LOST_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::LOST_SOUL,
    "ff73c4c8-1761-4fc8-96b1-8983b9b78b46",
    "Randy Asplund-Faith",
);

// 5ED 176 — Mind Ravel (reprint)
const MIND_RAVEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MIND_RAVEL,
    "1a9a0296-3690-45c7-98af-d546296ad9fe",
    "Mark Tedin",
);

// 5ED 177 — Mind Warp (reprint)
const MIND_WARP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MIND_WARP,
    "de86fd7e-438c-48a2-a69d-193d8b8ebbac",
    "Liz Danforth",
);

// 5ED 178 — Mindstab Thrull (reprint)
const MINDSTAB_THRULL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::MINDSTAB_THRULL,
    "d32b1439-b83d-4e6d-8956-b0964b1e3ed9",
    "Mark Tedin",
);

// 5ED 178s — Mindstab Thrull (alternate printing)
const MINDSTAB_THRULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_fem::MINDSTAB_THRULL,
    1,
    "ba90042a-a5e9-4c3f-a08a-c24e5606110b",
    "Heather Hudson",
);

// 5ED 179 — Mole Worms (reprint)
const MOLE_WORMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MOLE_WORMS,
    "700f43c9-1e71-4e5f-a6ce-eca94993558f",
    "Adrian Smith",
);

// 5ED 180 — Murk Dwellers (reprint)
const MURK_DWELLERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MURK_DWELLERS,
    "740564ec-c473-45bc-ba94-288786bf28b9",
    "Drew Tucker",
);

// 5ED 181 — Necrite (reprint)
const NECRITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::NECRITE,
    "b8dc0941-5837-41cf-9ae6-5f8d1fe4dfe4",
    "Ron Spencer",
);

// 5ED 181s — Necrite (alternate printing)
const NECRITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_fem::NECRITE,
    1,
    "892150a2-0a0b-4ac5-afb5-6434d6f42396",
    "Drew Tucker",
);

// 5ED 182 — Necropotence (reprint)
const NECROPOTENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::NECROPOTENCE,
    "222cfbec-8839-43fa-95a0-2ea91b59515c",
    "Mark Tedin",
);

// 5ED 183 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "b0877527-6dbe-49f2-862f-5c79e66a92e9",
    "DiTerlizzi",
);

// 5ED 184 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "60304328-7a02-4f4c-a884-fc6ce7816060",
    "Melissa A. Benson",
);

// 5ED 185 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "1823c578-d3a1-40de-bdac-ec6547e8ee24",
    "Ron Spencer",
);

// 5ED 186 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "a54e46c1-f56d-4836-95c8-76f0d76e0d02",
    "Kev Walker",
);

// 5ED 187 — Pit Scorpion (reprint)
const PIT_SCORPION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PIT_SCORPION,
    "fe106ff1-cdc6-44cc-adb7-131203a05292",
    "Ian Miller",
);

// 5ED 188 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "c99fd75c-4b41-411f-92b0-ca3b220946b5",
    "Anson Maddocks",
);

// 5ED 189 — Pox (reprint)
const POX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::POX,
    "7036057d-c7ab-4a37-a011-d10508cec2bc",
    "Scott M. Fischer",
);

// 5ED 190 — Rag Man (reprint)
const RAG_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::RAG_MAN,
    "cc54f1d6-8a2c-434e-bcae-942b3ccc44b5",
    "Daniel Gelon",
);

// 5ED 191 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "58326b13-0ab8-4fe5-8e63-c0333ffe5380",
    "David Seeley",
);

// 5ED 192 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "ed910299-b9e3-45b3-8fd1-2ebd47cd8275",
    "Tom Kyffin",
);

// 5ED 193 — Sengir Autocrat (reprint)
const SENGIR_AUTOCRAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::SENGIR_AUTOCRAT,
    "bac17237-778d-4551-9a92-8be546e233dc",
    "David A. Cherry",
);

// 5ED 194 — Sorceress Queen (reprint)
const SORCERESS_QUEEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SORCERESS_QUEEN,
    "657979b3-6e4f-41a8-a518-4bd329307770",
    "Kaja Foglio",
);

// 5ED 195 — Stromgald Cabal (reprint)
const STROMGALD_CABAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::STROMGALD_CABAL,
    "afac3a0a-e03d-4610-b12d-4bc2244163d1",
    "Anson Maddocks",
);

// 5ED 196 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TERROR,
    "ea0191ac-0924-4240-bfb9-700e6c09ddd1",
    "Ron Spencer",
);

// 5ED 197 — The Wretched (reprint)
const THE_WRETCHED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::THE_WRETCHED,
    "729f4543-79f3-4fe2-973f-fb2598045877",
    "Christopher Rush",
);

// 5ED 198 — Thrull Retainer (reprint)
const THRULL_RETAINER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::THRULL_RETAINER,
    "038c86ca-a09d-4ca3-a5db-310c7612c96d",
    "Ron Spencer",
);

// 5ED 199 — Torture (reprint)
const TORTURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::TORTURE,
    "0afdd3de-4f0c-4b78-8534-562db8be9c6b",
    "Mark Tedin",
);

// 5ED 200 — Touch of Death (reprint)
const TOUCH_OF_DEATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::TOUCH_OF_DEATH,
    "15e529fb-b951-4f64-85c9-1c2e3b9581a2",
    "Melissa A. Benson",
);

// 5ED 201 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "a0182f2f-a089-427b-aaf1-4cfaa4c0dc94",
    "Tom Kyffin",
);

// 5ED 202 — Vampire Bats (reprint)
const VAMPIRE_BATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::VAMPIRE_BATS,
    "c8b8aa03-c777-467f-9b05-812183553f7b",
    "Anson Maddocks",
);

// 5ED 203 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "931152c1-89a7-434e-adff-5f580a3be8ed",
    "Anson Maddocks",
);

// 5ED 204 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "94355dce-dfd6-45d6-9b86-0c2bc10e0231",
    "Amy Weber",
);

// 5ED 205 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "5210f08b-2278-47a2-9582-b0d79ebdfc2f",
    "Kev Walker",
);

// 5ED 206 — Xenic Poltergeist (reprint)
const XENIC_POLTERGEIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::XENIC_POLTERGEIST,
    "fcedc3f5-f4ab-4f7b-84c0-bd5a6a4b0f5e",
    "Mike Kerr",
);

// 5ED 207 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "50e9f0ff-5436-4faf-9d20-b828fd18baa6",
    "Stuart Griffin",
);

// 5ED 208 — Ambush Party (reprint)
const AMBUSH_PARTY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::AMBUSH_PARTY,
    "4e464c69-84c0-4c09-ae86-b728a15ec77f",
    "Charles Gillespie",
);

// 5ED 209 — Atog (reprint)
const ATOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ATOG,
    "f123fe6a-99ca-48c1-9a7a-ae905c10108a",
    "Jesper Myrfors",
);

// 5ED 210 — Ball Lightning (reprint)
const BALL_LIGHTNING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BALL_LIGHTNING,
    "3ede7920-e219-4e9d-bfa5-e0f562460914",
    "Quinton Hoover",
);

// 5ED 211 — Bird Maiden (reprint)
const BIRD_MAIDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BIRD_MAIDEN,
    "02750f36-d9d3-4c50-adbf-dadab46e92fd",
    "Kaja Foglio",
);

// 5ED 212 — Blood Lust (reprint)
const BLOOD_LUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLOOD_LUST,
    "37a63f96-e2d4-4786-80ea-79b206263cef",
    "Anson Maddocks",
);

// 5ED 213 — Brassclaw Orcs (reprint)
const BRASSCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::BRASSCLAW_ORCS,
    "6f068807-3e0d-42b0-aa11-7fd61492f12b",
    "Rob Alexander",
);

// 5ED 214 — Brothers of Fire (reprint)
const BROTHERS_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BROTHERS_OF_FIRE,
    "b3400509-7de8-4c1d-8546-a3df9bee1e75",
    "Mark Tedin",
);

// 5ED 215 — Cave People (reprint)
const CAVE_PEOPLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::CAVE_PEOPLE,
    "fb29b6b9-1674-483f-9912-0892e39ab106",
    "Steve Luke",
);

// 5ED 216 — Conquer (reprint)
const CONQUER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::CONQUER,
    "29354d9a-6d92-4ddf-a485-33064d666a76",
    "Gary Leach",
);

// 5ED 217 — Crimson Manticore (reprint)
const CRIMSON_MANTICORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CRIMSON_MANTICORE,
    "4f6884be-9e7c-407b-9165-6a720a634bb5",
    "Roger Raupp",
);

// 5ED 218 — Detonate (reprint)
const DETONATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DETONATE,
    "5e66b93c-1510-4063-b4b0-11fa1ea81c6b",
    "Randy Asplund-Faith",
);

// 5ED 219 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "2e1dce58-c0e5-472e-a575-b8d4b5958b3a",
    "Anson Maddocks",
);

// 5ED 220 — Dwarven Catapult (reprint)
const DWARVEN_CATAPULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DWARVEN_CATAPULT,
    "c3617ce3-240f-4846-94d9-ea852c5b842a",
    "Jeff A. Menges",
);

// 5ED 221 — Dwarven Soldier (reprint)
const DWARVEN_SOLDIER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DWARVEN_SOLDIER,
    "7438382c-15a0-40b1-a17b-2ef7bd67866c",
    "Randy Asplund-Faith",
);

// 5ED 222 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "d217942b-1253-4d4b-b0d2-09fc4e15cc62",
    "Douglas Shuler",
);

// 5ED 223 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHQUAKE,
    "01bde909-899d-4efc-aac5-57b69fa764db",
    "Richard Kane Ferguson",
);

// 5ED 224 — Errantry (reprint)
const ERRANTRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ERRANTRY,
    "b5b97aa2-cda9-4683-ad5d-1f713bd0505c",
    "Scott Kirschner",
);

// 5ED 225 — Eternal Warrior (reprint)
const ETERNAL_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ETERNAL_WARRIOR,
    "2a1a36ca-2467-4c23-ba03-744c76b0be51",
    "Anson Maddocks",
);

// 5ED 226 — Fire Drake (reprint)
const FIRE_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FIRE_DRAKE,
    "8b94218b-26d7-40cd-aef7-0e2415d1551f",
    "Christopher Rush",
);

// 5ED 227 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBALL,
    "ad457b10-6e00-411f-b827-ff844f9b300d",
    "Mark Tedin",
);

// 5ED 228 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "cb793e74-cfed-4b1a-aaba-d67d2ab6f149",
    "Dan Frazier",
);

// 5ED 229 — Flame Spirit (reprint)
const FLAME_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FLAME_SPIRIT,
    "b5e60702-1ca3-413a-a44e-22c8fc074b7f",
    "Justin Hampton",
);

// 5ED 230 — Flare (reprint)
const FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FLARE,
    "abc046c2-be9b-4f93-ac7d-e7dea6c4df9a",
    "Andrew Robinson",
);

// 5ED 231 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "3fafab5c-3ce7-41fc-a3e7-68e322566cfd",
    "Dameon Willich",
);

// 5ED 232 — Game of Chaos (reprint)
const GAME_OF_CHAOS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::GAME_OF_CHAOS,
    "95b44933-6b0b-426a-97d4-7a7cf6ad1d65",
    "Thomas Gianni",
);

// 5ED 232† — Game of Chaos (alternate printing)
const GAME_OF_CHAOS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::GAME_OF_CHAOS,
    1,
    "0e4aaab5-a202-4905-839b-ebf39b910082",
    "Thomas Gianni",
);

// 5ED 233 — Giant Strength (reprint)
const GIANT_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GIANT_STRENGTH,
    "a77caee7-b441-4458-bdc8-45fd1185380d",
    "Kev Walker",
);

// 5ED 234 — Goblin Digging Team (reprint)
const GOBLIN_DIGGING_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_DIGGING_TEAM,
    "8eb3feb2-e94b-4d02-8179-63d0289dd7d1",
    "Phil Foglio",
);

// 5ED 235 — Goblin Hero (reprint)
const GOBLIN_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_HERO,
    "1d21c8c9-6e16-4eb2-b2f5-3998f0f958ae",
    "Pete Venters",
);

// 5ED 236 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_KING,
    "dc72e1d3-f782-4b1c-9ea1-8e5b8566a187",
    "Phil Foglio",
);

// 5ED 237 — Goblin War Drums (reprint)
const GOBLIN_WAR_DRUMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::GOBLIN_WAR_DRUMS,
    "9b57d24a-1f2d-4227-a952-03abaecb19de",
    "Dan Frazier",
);

// 5ED 238 — Goblin Warrens (reprint)
const GOBLIN_WARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::GOBLIN_WARRENS,
    "57218245-5e0f-4233-896a-00ec0cc34fcc",
    "Dan Frazier",
);

// 5ED 239 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "4bb9d069-1163-46bd-9a71-21c05898330e",
    "Charles Gillespie",
);

// 5ED 240 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "eeb3745e-402f-47b6-9aaf-1d177d03cabe",
    "Anson Maddocks",
);

// 5ED 241 — Imposing Visage (reprint)
const IMPOSING_VISAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::IMPOSING_VISAGE,
    "24a256b0-e25b-43c6-9f7a-2ad76b268d22",
    "Brian Snõddy",
);

// 5ED 242 — Incinerate (reprint)
const INCINERATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::INCINERATE,
    "aa0f7e1f-bcb5-414f-a2e9-6a158fec2ff5",
    "Scott M. Fischer",
);

// 5ED 243 — Inferno (reprint)
const INFERNO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::INFERNO,
    "68d04a75-647f-400f-b0dc-c4544f7db2d4",
    "Mike Kerr",
);

// 5ED 243† — Inferno (alternate printing)
const INFERNO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_drk::INFERNO,
    1,
    "cc2850e8-f135-4a51-905b-ae30b83913a6",
    "Mike Kerr",
);

// 5ED 244 — Ironclaw Curse (reprint)
const IRONCLAW_CURSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::IRONCLAW_CURSE,
    "6ed35d03-80fd-421d-a8be-6708a5570a85",
    "Dennis Detwiller",
);

// 5ED 244† — Ironclaw Curse (alternate printing)
const IRONCLAW_CURSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_hml::IRONCLAW_CURSE,
    1,
    "346eef90-35c3-4153-a64a-55f4d7b232e2",
    "Dennis Detwiller",
);

// 5ED 245 — Ironclaw Orcs (reprint)
const IRONCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONCLAW_ORCS,
    "628a3506-352d-4d4f-b63e-05abb3ca906d",
    "Anson Maddocks",
);

// 5ED 246 — Jokulhaups (reprint)
const JOKULHAUPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::JOKULHAUPS,
    "6d81e479-45b7-4237-a0eb-95245582e87d",
    "Mike Kerr",
);

// 5ED 247 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "9c67ccfa-e2ac-4abd-a55d-bd10d570220a",
    "Kev Brockschmidt",
);

// 5ED 248 — Mana Clash (reprint)
const MANA_CLASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MANA_CLASH,
    "37b2e818-a5a8-4b9b-93e8-121dd7fefe32",
    "Mark Tedin",
);

// 5ED 249 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "90d9af72-1633-4433-83ce-7de806611448",
    "Christopher Rush",
);

// 5ED 250 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "f55a2d1d-0964-44a9-9f32-09804f2c7445",
    "Greg Simanson",
);

// 5ED 250† — Manabarbs (alternate printing)
const MANABARBS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MANABARBS,
    1,
    "ffef6ac8-a42c-4025-b4ef-3f772129b8f7",
    "Greg Simanson",
);

// 5ED 251 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "8324c5a9-d126-4510-90ac-c6b1424137d9",
    "Pete Venters",
);

// 5ED 252 — Mountain Goat (reprint)
const MOUNTAIN_GOAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MOUNTAIN_GOAT,
    "fbec9fe6-a951-48b2-a0d2-3cdf69b5400f",
    "Cornelius Brudi",
);

// 5ED 253 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "a2ddc97b-b13e-4947-b4df-f35790b89f15",
    "Dan Frazier",
);

// 5ED 254 — Orcish Captain (reprint)
const ORCISH_CAPTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ORCISH_CAPTAIN,
    "fe9d422f-101e-44f2-ab0d-7ad962c6d657",
    "Charles Gillespie",
);

// 5ED 255 — Orcish Conscripts (reprint)
const ORCISH_CONSCRIPTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ORCISH_CONSCRIPTS,
    "17025cde-28a1-4a38-ad5a-1ead518d77af",
    "Douglas Shuler",
);

// 5ED 256 — Orcish Farmer (reprint)
const ORCISH_FARMER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ORCISH_FARMER,
    "fb2bb2b4-87ff-4971-9835-fd68d913af26",
    "Dan Frazier",
);

// 5ED 257 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "1b00e95e-6bf1-445a-a46a-29aeb05bf9be",
    "Dan Frazier",
);

// 5ED 258 — Orcish Squatters (reprint)
const ORCISH_SQUATTERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ORCISH_SQUATTERS,
    "86a27af5-cb26-4f39-8648-193bb116e3d6",
    "Richard Kane Ferguson",
);

// 5ED 259 — Orgg (reprint)
const ORGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ORGG,
    "c7eef087-8500-4fca-a10a-ff65d348bcb1",
    "Daniel Gelon",
);

// 5ED 260 — Panic (reprint)
const PANIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PANIC,
    "97a86ee4-200e-4994-bd6f-0c1d3227ce5b",
    "Greg Simanson",
);

// 5ED 261 — Primordial Ooze (reprint)
const PRIMORDIAL_OOZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PRIMORDIAL_OOZE,
    "a53d8d6d-b8d3-4f71-a88a-5d639ce2925f",
    "Randy Gallegos",
);

// 5ED 262 — Pyroblast (reprint)
const PYROBLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PYROBLAST,
    "15d2d7cd-d29d-4f19-8490-88bd526ff1c5",
    "Kaja Foglio",
);

// 5ED 263 — Pyrotechnics (reprint)
const PYROTECHNICS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PYROTECHNICS,
    "45e9f71f-cb85-4b57-9bb3-fe98ed4a524e",
    "Anson Maddocks",
);

// 5ED 264 — Sabretooth Tiger (reprint)
const SABRETOOTH_TIGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SABRETOOTH_TIGER,
    "eb936214-df72-4d77-b8f7-879c2b4e31d7",
    "Melissa A. Benson",
);

// 5ED 265 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "8a1edce4-0550-4683-a4d6-5dff6b9f0122",
    "Hannibal King",
);

// 5ED 266 — Shatterstorm (reprint)
const SHATTERSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::SHATTERSTORM,
    "5e2d38e6-b659-4fcb-8192-797bb61e7700",
    "James Allen",
);

// 5ED 267 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "64b05edd-4128-4150-9457-8aff895bd0b7",
    "Melissa A. Benson",
);

// 5ED 267† — Shivan Dragon (alternate printing)
const SHIVAN_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SHIVAN_DRAGON,
    1,
    "b97e7318-f916-4556-9f89-dda7914359c1",
    "Melissa A. Benson",
);

// 5ED 268 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SMOKE,
    "b4710dd0-2797-40dd-8a24-bdd3da112c39",
    "Tom Kyffin",
);

// 5ED 269 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_GIANT,
    "7f736379-1fe8-43b8-b749-f1e9baef96a6",
    "James Allen",
);

// 5ED 270 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "eb5935ef-8cc5-4587-b8eb-f6a2b2856260",
    "Tony Roberts",
);

// 5ED 271 — Stone Spirit (reprint)
const STONE_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::STONE_SPIRIT,
    "705c927b-1258-4758-bbff-10cc032587a2",
    "James Allen",
);

// 5ED 272 — The Brute (reprint)
const THE_BRUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::THE_BRUTE,
    "d3ded8fa-facf-4196-8964-56ebc6424be1",
    "Douglas Shuler",
);

// 5ED 273 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "debc9415-ae75-4514-9d5d-b1b403420d1d",
    "Tony Roberts",
);

// 5ED 274 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "a47e4f33-3eb5-4aac-acb4-74ff60558db7",
    "Thomas Gianni",
);

// 5ED 275 — Winds of Change (reprint)
const WINDS_OF_CHANGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WINDS_OF_CHANGE,
    "38712e4f-aabd-40b3-b41f-b034e032a729",
    "Blackie del Rio",
);

// 5ED 276 — Word of Blasting (reprint)
const WORD_OF_BLASTING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::WORD_OF_BLASTING,
    "06dc1c22-ef39-4f5f-a8af-4267a3d5db6f",
    "Ken Meyer, Jr.",
);

// 5ED 277 — An-Havva Constable (reprint)
const AN_HAVVA_CONSTABLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::AN_HAVVA_CONSTABLE,
    "5955eb2b-31a2-4e1e-bbbe-82eb53def698",
    "Dan Frazier",
);

// 5ED 278 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "38af8356-2d7f-4699-9e57-08906c1c831b",
    "Janine Johnston",
);

// 5ED 279 — Aurochs (reprint)
const AUROCHS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::AUROCHS,
    "dd473981-a62c-480f-aeae-0218f3e07fa6",
    "Steve White",
);

// 5ED 280 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BIRDS_OF_PARADISE,
    "f79357f4-ddd1-4b29-9e5f-813c799513ee",
    "Mark Poole",
);

// 5ED 281 — Carapace (reprint)
const CARAPACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::CARAPACE,
    "22891f6c-7365-4baf-ae5a-49380be50895",
    "Anson Maddocks",
);

// 5ED 282 — Cat Warriors (reprint)
const CAT_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CAT_WARRIORS,
    "b0180bd6-f8eb-4cf7-b6d0-acb513076e00",
    "Melissa A. Benson",
);

// 5ED 283 — Chub Toad (reprint)
const CHUB_TOAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::CHUB_TOAD,
    "0d977294-051a-4e6d-b2ba-c04494474aac",
    "Daniel Gelon",
);

// 5ED 284 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "853d15cd-a1a2-47a8-89c4-81b7ca663fff",
    "Dan Frazier",
);

// 5ED 285 — Craw Giant (reprint)
const CRAW_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CRAW_GIANT,
    "06274d3b-8feb-4c68-adbc-b1ee633c3353",
    "Scott Kirschner",
);

// 5ED 286 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "1a2e6afb-7094-4fa3-9246-58343f8d80b8",
    "Daniel Gelon",
);

// 5ED 287 — Crumble (reprint)
const CRUMBLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CRUMBLE,
    "45c781ee-7ffb-4dd3-ac84-4d6fe4649591",
    "Jesper Myrfors",
);

// 5ED 288 — Desert Twister (reprint)
const DESERT_TWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DESERT_TWISTER,
    "b15dc6a9-f288-414a-9052-f03846176bff",
    "Susan Van Camp",
);

// 5ED 289 — Durkwood Boars (reprint)
const DURKWOOD_BOARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DURKWOOD_BOARS,
    "68418099-c199-428a-b4aa-db54909efd0d",
    "Mike Kimble",
);

// 5ED 290 — Elder Druid (reprint)
const ELDER_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ELDER_DRUID,
    "2151c8f2-c031-4c96-a420-ff2108df7ad8",
    "Richard Kane Ferguson",
);

// 5ED 291 — Elven Riders (reprint)
const ELVEN_RIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ELVEN_RIDERS,
    "8cbb6c97-b972-4c78-ba95-6053f2466f20",
    "Dan Frazier",
);

// 5ED 292 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "bd92d92c-a9c7-40c5-82f3-dbd19697f999",
    "Anson Maddocks",
);

// 5ED 293 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "82954c1c-8c58-48bf-aefe-e7ec5108a124",
    "John Avon",
);

// 5ED 294 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "e86f61bb-c2b5-4672-b262-1c72bd1de51f",
    "Pete Venters",
);

// 5ED 295 — Foxfire (reprint)
const FOXFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FOXFIRE,
    "eb1638c4-effb-460b-af5d-aa17559dbd37",
    "Margaret Organ-Kean",
);

// 5ED 296 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "37f6f2bd-4e0e-42d8-b5a6-ad4ee736c69e",
    "Scott M. Fischer",
);

// 5ED 297 — Fyndhorn Elder (reprint)
const FYNDHORN_ELDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FYNDHORN_ELDER,
    "0b8532a8-966e-4cba-b3a5-99cfb0ee1220",
    "Donato Giancola",
);

// 5ED 298 — Ghazbán Ogre (reprint)
const GHAZBAN_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::GHAZBAN_OGRE,
    "4fa6c0d6-aa18-4c32-a641-1ec8e50a26f3",
    "Mike Raabe",
);

// 5ED 299 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "07aee96b-1cf3-4cd9-9998-39ab16170c6b",
    "DiTerlizzi",
);

// 5ED 300 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "953cf3be-b975-4a10-8c92-c783c727dcfa",
    "Brian Snõddy",
);

// 5ED 301 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "68d8ad43-adea-47e8-9d9e-e14c2ad41489",
    "Una Fricker",
);

// 5ED 302 — Hungry Mist (reprint)
const HUNGRY_MIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::HUNGRY_MIST,
    "89ba2bff-08e6-4413-929b-80e13f193b1e",
    "Heather Hudson",
);

// 5ED 303 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "1aa0de8c-a23f-49d2-ac16-91c0d1eb17a6",
    "Cornelius Brudi",
);

// 5ED 304 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "ccf9b2cc-b362-4488-93ef-e98398ff73ea",
    "Ron Spencer",
);

// 5ED 305 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "6bdbba38-b4c9-4c14-b869-669b39390e4e",
    "Jerry Tiritilli",
);

// 5ED 306 — Johtull Wurm (reprint)
const JOHTULL_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::JOHTULL_WURM,
    "64ca51ce-e0f4-42ce-b2a5-db73268bcf1f",
    "Ian Miller",
);

// 5ED 307 — Killer Bees (reprint)
const KILLER_BEES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KILLER_BEES,
    "6a749837-56ff-4e42-9bf2-82633bccdc39",
    "Phil Foglio",
);

// 5ED 308 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "5acc4f93-1bf1-4b00-b0e0-d44c2b2cd079",
    "Sandra Everingham",
);

// 5ED 309 — Lhurgoyf (reprint)
const LHURGOYF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::LHURGOYF,
    "fadc2d27-0c6c-4f68-bee0-0af6688f304d",
    "Pete Venters",
);

// 5ED 310 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "fa1681fa-2edc-4c0c-b5ca-f7f2602a3467",
    "Ron Spencer",
);

// 5ED 311 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "8af097b1-9eae-4bd6-8ee6-582cae57e970",
    "Anson Maddocks",
);

// 5ED 312 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "ab9a2294-dd5b-4658-9f79-334a51b03ea3",
    "John Matson",
);

// 5ED 313 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LLANOWAR_ELVES,
    "632b9428-45c6-4b81-a701-f1c7d7e8d2f0",
    "Anson Maddocks",
);

// 5ED 314 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "f380e07a-9d5c-4579-93f5-4cf52d90897d",
    "Anson Maddocks",
);

// 5ED 315 — Marsh Viper (reprint)
const MARSH_VIPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MARSH_VIPER,
    "ba4c0606-f9af-4dee-bc36-5051395b5f44",
    "Ron Spencer",
);

// 5ED 316 — Nature's Lore (reprint)
const NATURE_S_LORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::NATURE_S_LORE,
    "00a4ffc1-7731-42d9-b970-2a8d84cba14d",
    "Alan Rabinowitz",
);

// 5ED 317 — Pradesh Gypsies (reprint)
const PRADESH_GYPSIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PRADESH_GYPSIES,
    "d9ee195a-c331-4918-ad9e-3877dd3f4e3a",
    "Quinton Hoover",
);

// 5ED 318 — Primal Order (reprint)
const PRIMAL_ORDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::PRIMAL_ORDER,
    "535b455a-417a-4490-8495-bda9441306c8",
    "David A. Cherry",
);

// 5ED 319 — Rabid Wombat (reprint)
const RABID_WOMBAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RABID_WOMBAT,
    "f5dbfeb2-76ac-48b2-99fb-fa4bcc59b2a5",
    "Kaja Foglio",
);

// 5ED 320 — Radjan Spirit (reprint)
const RADJAN_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RADJAN_SPIRIT,
    "adf02a10-86ba-4c2d-adc5-e4a2b7cc089e",
    "Christopher Rush",
);

// 5ED 321 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "44fd0d1d-9422-4941-9491-8a8d64909f8e",
    "Quinton Hoover",
);

// 5ED 322 — Scaled Wurm (reprint)
const SCALED_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SCALED_WURM,
    "07ad1b3a-3464-4080-819b-a62bf6c23a13",
    "Daniel Gelon",
);

// 5ED 323 — Scavenger Folk (reprint)
const SCAVENGER_FOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::SCAVENGER_FOLK,
    "ff4d1549-d0d5-48f8-9d8a-2e2940523307",
    "Jeff Miracola",
);

// 5ED 324 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCRYB_SPRITES,
    "ab52f491-26f1-494f-8ec7-9630c4f9653a",
    "Amy Weber",
);

// 5ED 325 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "34163ceb-5c7a-4a3b-9290-bc4f22e0259d",
    "Gary Leach",
);

// 5ED 326 — Shrink (reprint)
const SHRINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::SHRINK,
    "84f96537-76d2-4887-b17f-b8297fa50fd5",
    "Liz Danforth",
);

// 5ED 327 — Stampede (reprint)
const STAMPEDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::STAMPEDE,
    "fcec8997-bc9b-42c0-bac1-5644e9915fa2",
    "Jeff A. Menges",
);

// 5ED 328 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "738e9ec1-a186-44ad-8a68-d5cc183ccdc0",
    "Terese Nielsen",
);

// 5ED 329 — Sylvan Library (reprint)
const SYLVAN_LIBRARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SYLVAN_LIBRARY,
    "b1b337de-4c43-45b6-a0ca-262a983f3589",
    "Harold McNeill",
);

// 5ED 330 — Tarpan (reprint)
const TARPAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::TARPAN,
    "d2160d57-9ebf-43fb-811f-0c014e417ea0",
    "Margaret Organ-Kean",
);

// 5ED 331 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "511d4bc3-05ea-48f3-b182-dd0cd049ce54",
    "Dan Frazier",
);

// 5ED 332 — Titania's Song (reprint)
const TITANIA_S_SONG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TITANIA_S_SONG,
    "f837f0c8-4822-41f1-8677-988c13efe8d5",
    "D. Alexander Gregory",
);

// 5ED 333 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "c6f8bd4a-ec7f-4da5-bd39-cba29541ee83",
    "Douglas Shuler",
);

// 5ED 334 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "0ab151ce-9468-4427-853b-4e0011d783fb",
    "Richard Thomas",
);

// 5ED 335 — Untamed Wilds (reprint)
const UNTAMED_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::UNTAMED_WILDS,
    "979f6606-a4d7-43e8-95fb-c4f06c16d1b4",
    "NéNé Thomas",
);

// 5ED 336 — Venom (reprint)
const VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::VENOM,
    "cbb503ef-31a4-492d-9c7d-26b72c36904b",
    "Tom Wänerstrand",
);

// 5ED 337 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "d6d5bf9e-db83-498d-bccc-9588f2fcb9ff",
    "Kev Brockschmidt",
);

// 5ED 338 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "e8780a20-8504-406e-aea1-722f146be5f7",
    "Tony Roberts",
);

// 5ED 339 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "6d93816b-cb5e-49b7-98bd-22de674c7873",
    "Rebecca Guay",
);

// 5ED 340 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "c38912a6-0327-411a-9499-d659b635e2bd",
    "Jeff A. Menges",
);

// 5ED 341 — Whirling Dervish (reprint)
const WHIRLING_DERVISH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WHIRLING_DERVISH,
    "e51bfbd4-2319-41eb-b694-72874c24b31a",
    "Susan Van Camp",
);

// 5ED 342 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "2a9e6630-d57c-47fd-af0f-b2e2c2025599",
    "Pat Lewis",
);

// 5ED 343 — Winter Blast (reprint)
const WINTER_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WINTER_BLAST,
    "f2b01c34-faa9-43a3-a3dc-0f966e88e089",
    "Kaja Foglio",
);

// 5ED 344 — Wolverine Pack (reprint)
const WOLVERINE_PACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WOLVERINE_PACK,
    "fab6d1e8-0985-4560-aea2-7ad1925a2f5a",
    "Steve White",
);

// 5ED 345 — Wyluli Wolf (reprint)
const WYLULI_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::WYLULI_WOLF,
    "5d06d6b2-2eff-4fc5-99e1-c6bc585a4926",
    "Susan Van Camp",
);

// 5ED 346 — Aladdin's Ring (reprint)
const ALADDINS_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDINS_RING,
    "d74e4ed4-4470-4b3b-98cd-e4a8660b9768",
    "Stuart Griffin",
);

// 5ED 347 — Amulet of Kroog (reprint)
const AMULET_OF_KROOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::AMULET_OF_KROOG,
    "50871a5f-ad52-470a-840a-2277f43ea47b",
    "Margaret Organ-Kean",
);

// 5ED 348 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANKH_OF_MISHRA,
    "e2d917dd-243e-4df2-9a44-1c2eae6ff859",
    "Ian Miller",
);

// 5ED 349 — Ashnod's Altar (reprint)
const ASHNOD_S_ALTAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ASHNOD_S_ALTAR,
    "eab611c3-3a24-4033-864f-084b71317320",
    "Anson Maddocks",
);

// 5ED 350 — Ashnod's Transmogrant (reprint)
const ASHNODS_TRANSMOGRANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ASHNODS_TRANSMOGRANT,
    "ee48bcd6-afb5-4023-9417-ed3126bcc31d",
    "Mark Tedin",
);

// 5ED 351 — Barbed Sextant (reprint)
const BARBED_SEXTANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BARBED_SEXTANT,
    "db9791ab-1cc4-41ad-a721-57f214c357f8",
    "Amy Weber",
);

// 5ED 352 — Barl's Cage (reprint)
const BARLS_CAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BARLS_CAGE,
    "ecce3e79-c729-4efd-a2f3-5d2910ba2c3f",
    "Tom Wänerstrand",
);

// 5ED 353 — Battering Ram (reprint)
const BATTERING_RAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::BATTERING_RAM,
    "e7e2857f-f6eb-4091-b758-7bb508544170",
    "Jeff A. Menges",
);

// 5ED 354 — Bottle of Suleiman (reprint)
const BOTTLE_OF_SULEIMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BOTTLE_OF_SULEIMAN,
    "f1de13cc-65b4-4a72-8a53-a05a46a40f61",
    "DiTerlizzi",
);

// 5ED 355 — Clay Statue (reprint)
const CLAY_STATUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CLAY_STATUE,
    "34727679-9c42-4577-80c3-88495aa0e96d",
    "Adam Rex",
);

// 5ED 356 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "ed5507d5-7f1b-4cbf-8341-495c33e5ab6c",
    "Drew Tucker",
);

// 5ED 357 — Clockwork Steed (reprint)
const CLOCKWORK_STEED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::CLOCKWORK_STEED,
    "d27d83b9-4454-40c0-bac0-de736c634a53",
    "Terese Nielsen",
);

// 5ED 358 — Colossus of Sardia (reprint)
const COLOSSUS_OF_SARDIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::COLOSSUS_OF_SARDIA,
    "a6f1ffdd-897e-4a1d-bdd2-d9de0de5d5a2",
    "Hannibal King",
);

// 5ED 359 — Coral Helm (reprint)
const CORAL_HELM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CORAL_HELM,
    "274e3a79-d7e5-46e2-bd9f-209e67294f82",
    "Steve Luke",
);

// 5ED 360 — Crown of the Ages (reprint)
const CROWN_OF_THE_AGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::CROWN_OF_THE_AGES,
    "77b549e5-c25d-4688-b362-faab109ba092",
    "Roger Raupp",
);

// 5ED 361 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "4bf1c7de-73b5-415b-897b-085ab5776932",
    "Donato Giancola",
);

// 5ED 362 — Dancing Scimitar (reprint)
const DANCING_SCIMITAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DANCING_SCIMITAR,
    "bc6f3d92-171f-45e6-b159-b83faad7f7fd",
    "Anson Maddocks",
);

// 5ED 363 — Diabolic Machine (reprint)
const DIABOLIC_MACHINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::DIABOLIC_MACHINE,
    "8c76a433-4760-4ebb-ab0c-d69bba7d7ca1",
    "James Allen",
);

// 5ED 364 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "010d1107-a18c-4e25-86c7-8e09cba72a12",
    "Randy Gallegos",
);

// 5ED 365 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "060f2e80-f7af-4b9d-bb0f-3de14ddf0b02",
    "Stuart Griffin",
);

// 5ED 366 — Dragon Engine (reprint)
const DRAGON_ENGINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DRAGON_ENGINE,
    "b3f26891-e624-4aa9-ad04-6db3933a74e9",
    "Anson Maddocks",
);

// 5ED 367 — Elkin Bottle (reprint)
const ELKIN_BOTTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ELKIN_BOTTLE,
    "cf5b60a9-3041-4524-9a53-ab3a597c20ac",
    "Quinton Hoover",
);

// 5ED 368 — Feldon's Cane (reprint)
const FELDONS_CANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::FELDONS_CANE,
    "030301c9-b576-43ba-b05a-a52904c92be9",
    "Mark Tedin",
);

// 5ED 369 — Fellwar Stone (reprint)
const FELLWAR_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FELLWAR_STONE,
    "2716dacb-4fb7-4fa8-869d-22af82920564",
    "Quinton Hoover",
);

// 5ED 370 — Feroz's Ban (reprint)
const FEROZ_S_BAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::FEROZ_S_BAN,
    "5133a47c-bde3-49d5-ba44-196980ace436",
    "Heather Hudson",
);

// 5ED 371 — Flying Carpet (reprint)
const FLYING_CARPET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::FLYING_CARPET,
    "c5f33257-4539-4b33-86d5-26fedbc417b1",
    "Mark Tedin",
);

// 5ED 372 — Fountain of Youth (reprint)
const FOUNTAIN_OF_YOUTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FOUNTAIN_OF_YOUTH,
    "a726d567-ad74-4b77-bbd5-8a0902cdaf6a",
    "Daniel Gelon",
);

// 5ED 373 — Gauntlets of Chaos (reprint)
const GAUNTLETS_OF_CHAOS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GAUNTLETS_OF_CHAOS,
    "548045fe-5302-4295-9060-e85614bb9a91",
    "Alan Rabinowitz",
);

// 5ED 374 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLASSES_OF_URZA,
    "3259a69e-1011-475b-b554-8cf5a25169f3",
    "Douglas Shuler",
);

// 5ED 375 — Grapeshot Catapult (reprint)
const GRAPESHOT_CATAPULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::GRAPESHOT_CATAPULT,
    "9ae0fe07-85b6-4179-859b-fbae74faab5f",
    "Dan Frazier",
);

// 5ED 376 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "9664a59d-182a-429b-a6f7-047b8b8e5ffe",
    "Mark Tedin",
);

// 5ED 377 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "8a50b3bd-81b0-408d-ab73-9eadd2fb1eae",
    "Mark Poole",
);

// 5ED 378 — Infinite Hourglass (reprint)
const INFINITE_HOURGLASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::INFINITE_HOURGLASS,
    "fbafc838-7458-4542-86f3-594326ec0691",
    "Adam Rex",
);

// 5ED 379 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRON_STAR,
    "6dcbaa97-49f8-41b4-8901-eb53f6c15faf",
    "Donato Giancola",
);

// 5ED 380 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "14f8085b-4bed-4b0b-b05e-5237bb896d00",
    "Donato Giancola",
);

// 5ED 381 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "70a8f00d-e813-4cb7-8d09-1edae560f287",
    "Richard Kane Ferguson",
);

// 5ED 382 — Jalum Tome (reprint)
const JALUM_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::JALUM_TOME,
    "b8fe15fc-420f-49fe-a7d8-84dd74964ce5",
    "Tom Wänerstrand",
);

// 5ED 383 — Jandor's Saddlebags (reprint)
const JANDORS_SADDLEBAGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JANDORS_SADDLEBAGS,
    "252a9b1d-4046-484c-add9-567b3303cc98",
    "Roger Raupp",
);

// 5ED 384 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JAYEMDAE_TOME,
    "0736c836-d90b-440c-bb6f-4b2eeaaa3d73",
    "Mark Tedin",
);

// 5ED 385 — Jester's Cap (reprint)
const JESTER_S_CAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::JESTER_S_CAP,
    "e10f2619-e723-4402-ae4e-79d762004477",
    "Dan Frazier",
);

// 5ED 386 — Joven's Tools (reprint)
const JOVEN_S_TOOLS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::JOVEN_S_TOOLS,
    "91c7aa8e-5873-4e18-8adb-1523c01d4e86",
    "Zina Saunders",
);

// 5ED 387 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "cfe1d1ae-bcc0-4b0d-a60f-f5ed30184329",
    "Daniel Gelon",
);

// 5ED 388 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_VAULT,
    "05e9fec4-1e0a-4206-ab2b-cc2543cba667",
    "Mark Tedin",
);

// 5ED 389 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "4391d903-1370-4a3a-9919-7a07e580a26c",
    "Quinton Hoover",
);

// 5ED 390 — Millstone (reprint)
const MILLSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MILLSTONE,
    "92e0b9ff-cf4c-446a-b706-58e5c496599e",
    "Kaja Foglio",
);

// 5ED 391 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NEVINYRRALS_DISK,
    "9652c405-17fb-4505-8470-8a2969c73b6b",
    "Mark Tedin",
);

// 5ED 392 — Obelisk of Undoing (reprint)
const OBELISK_OF_UNDOING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::OBELISK_OF_UNDOING,
    "7c0672d1-1d47-42ea-b8b0-95c400b0f78e",
    "Tom Wänerstrand",
);

// 5ED 393 — Ornithopter (reprint)
const ORNITHOPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ORNITHOPTER,
    "00427d72-b140-45eb-b2ec-2ac2dab16966",
    "Amy Weber",
);

// 5ED 394 — Pentagram of the Ages (reprint)
const PENTAGRAM_OF_THE_AGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PENTAGRAM_OF_THE_AGES,
    "a2334ac3-2a58-49ba-902e-98dbad51547b",
    "Douglas Shuler",
);

// 5ED 395 — Primal Clay (reprint)
const PRIMAL_CLAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::PRIMAL_CLAY,
    "152f731f-78a6-444e-9a43-85446fa38f30",
    "Adam Rex",
);

// 5ED 396 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "1589724f-bec2-43a9-824a-1e85020731a4",
    "Christopher Rush",
);

// 5ED 397 — Serpent Generator (reprint)
const SERPENT_GENERATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SERPENT_GENERATOR,
    "0b08f89d-2ce3-4686-a8bd-ed982280ae4d",
    "Mark Tedin",
);

// 5ED 398 — Shapeshifter (reprint)
const SHAPESHIFTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::SHAPESHIFTER,
    "a0aca05e-87c0-4c50-af47-37c61c55934d",
    "Adrian Smith",
);

// 5ED 399 — Skull Catapult (reprint)
const SKULL_CATAPULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SKULL_CATAPULT,
    "e6baa736-60c3-4192-a5d3-150c90c31847",
    "Ian Miller",
);

// 5ED 400 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "321d3dc9-4bd0-440e-9436-c565f500e126",
    "Andrew Robinson",
);

// 5ED 401 — Tawnos's Weaponry (reprint)
const TAWNOSS_WEAPONRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TAWNOSS_WEAPONRY,
    "eefe0113-8ac4-4d89-8ed3-010ccb57f3f0",
    "John Coulthart",
);

// 5ED 402 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "885c72d8-cee1-4a67-a394-b7dfe89bbd2e",
    "Sandra Everingham",
);

// 5ED 403 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "631db179-e23c-4099-9214-5bd347e4aa9e",
    "Donato Giancola",
);

// 5ED 404 — Time Bomb (reprint)
const TIME_BOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::TIME_BOMB,
    "0e3b9a42-29a5-47f9-9c65-db9f3d432bf5",
    "George Pratt",
);

// 5ED 405 — Urza's Avenger (reprint)
const URZA_S_AVENGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_AVENGER,
    "60bd9559-1a8f-47d0-af6b-d0681cae4060",
    "Amy Weber",
);

// 5ED 406 — Urza's Bauble (reprint)
const URZAS_BAUBLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::URZAS_BAUBLE,
    "876c1811-990c-41ca-874f-fd0512b59d06",
    "Christopher Rush",
);

// 5ED 407 — Wall of Spears (reprint)
const WALL_OF_SPEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::WALL_OF_SPEARS,
    "60a116c9-2f80-4e76-b158-c23f101872db",
    "Zak Plucinski",
);

// 5ED 408 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WINTER_ORB,
    "3a674ec8-5531-4d06-aa57-929c6ac29238",
    "Mark Tedin",
);

// 5ED 409 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "f3d05202-da8d-44b8-8a01-d8a16a2b1898",
    "Donato Giancola",
);

// 5ED 410 — Adarkar Wastes (reprint)
const ADARKAR_WASTES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ADARKAR_WASTES,
    "56c1e5f1-4665-4afc-83e4-4968df72eb8f",
    "Gary Leach",
);

// 5ED 411 — Bottomless Vault (reprint)
const BOTTOMLESS_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::BOTTOMLESS_VAULT,
    "c8913994-c3e7-4d9e-9409-a8409ce68442",
    "David Seeley",
);

// 5ED 412 — Brushland (reprint)
const BRUSHLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BRUSHLAND,
    "e6793b15-37fb-495e-b5e2-22c1df4cdc05",
    "Tom Wänerstrand",
);

// 5ED 413 — City of Brass (reprint)
const CITY_OF_BRASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::CITY_OF_BRASS,
    "56816a2d-4faa-4fbd-bf1a-35fa3e90ccf6",
    "Tom Wänerstrand",
);

// 5ED 414 — Dwarven Hold (reprint)
const DWARVEN_HOLD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DWARVEN_HOLD,
    "79cb55f0-f0e8-4644-8c19-54efe5854973",
    "David Seeley",
);

// 5ED 415 — Dwarven Ruins (reprint)
const DWARVEN_RUINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DWARVEN_RUINS,
    "048fc9a8-cc3c-43a1-89dd-c9a8fa72bdc6",
    "Liz Danforth",
);

// 5ED 416 — Ebon Stronghold (reprint)
const EBON_STRONGHOLD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::EBON_STRONGHOLD,
    "b0509266-2bd5-4958-a497-935e004cc20f",
    "Liz Danforth",
);

// 5ED 417 — Havenwood Battleground (reprint)
const HAVENWOOD_BATTLEGROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::HAVENWOOD_BATTLEGROUND,
    "ce1a1d66-7694-4e12-a4d6-34ca36656db4",
    "Liz Danforth",
);

// 5ED 418 — Hollow Trees (reprint)
const HOLLOW_TREES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::HOLLOW_TREES,
    "3d6d30a4-a24e-4537-9230-bf6f57fbcd98",
    "David Seeley",
);

// 5ED 419 — Icatian Store (reprint)
const ICATIAN_STORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ICATIAN_STORE,
    "e68c45d3-0f3c-4429-a93e-4ca70ea6b1a4",
    "David Seeley",
);

// 5ED 420 — Ice Floe (reprint)
const ICE_FLOE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ICE_FLOE,
    "0550a334-f185-4f1f-8759-d320b154c62f",
    "John Avon",
);

// 5ED 421 — Karplusan Forest (reprint)
const KARPLUSAN_FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KARPLUSAN_FOREST,
    "38d59e6e-0154-4c89-a58f-521e3424581b",
    "Randy Gallegos",
);

// 5ED 422 — Ruins of Trokair (reprint)
const RUINS_OF_TROKAIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::RUINS_OF_TROKAIR,
    "a28ec8a0-5cde-4c24-b605-b5f53553d065",
    "Liz Danforth",
);

// 5ED 423 — Sand Silos (reprint)
const SAND_SILOS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::SAND_SILOS,
    "6769de45-903f-4269-a100-9ceca1df26ac",
    "David Seeley",
);

// 5ED 424 — Sulfurous Springs (reprint)
const SULFUROUS_SPRINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SULFUROUS_SPRINGS,
    "85a2097d-5acb-46d5-ac07-021fa9da4026",
    "Jeff Miracola",
);

// 5ED 425 — Svyelunite Temple (reprint)
const SVYELUNITE_TEMPLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::SVYELUNITE_TEMPLE,
    "22744530-792f-4098-ad2f-fee1a02b04b1",
    "Liz Danforth",
);

// 5ED 426 — Underground River (reprint)
const UNDERGROUND_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::UNDERGROUND_RIVER,
    "be9efd79-1b1d-4e02-8922-0154ad20e4d4",
    "Jeff Miracola",
);

// 5ED 427 — Urza's Mine (reprint)
const URZA_S_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_MINE,
    "6c6c9f3e-6df7-459c-a1b0-d628f17bb7a6",
    "Anson Maddocks",
);

// 5ED 428 — Urza's Power Plant (reprint)
const URZA_S_POWER_PLANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_POWER_PLANT,
    "c9a294bd-e6f6-4b88-b34c-135aff907b17",
    "Mark Tedin",
);

// 5ED 429 — Urza's Tower (reprint)
const URZA_S_TOWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_TOWER,
    "9bb66579-0840-4ec3-aa6a-731874662dbb",
    "Mark Poole",
);

// 5ED 430 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "6cab2ebd-6ab7-4273-bda3-d7dea81b52c3",
    "Pat Lewis",
);

// 5ED 431 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "600cc3f3-5400-40f4-99ff-8a4a905e79cf",
    "Pat Lewis",
);

// 5ED 432 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "38789fff-0207-4b38-9bfd-5464ab62a533",
    "Pat Lewis",
);

// 5ED 433 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "ef1c6830-758e-4242-85f7-3f20b872272c",
    "Pat Lewis",
);

// 5ED 434 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "c8820e22-44ee-4ccf-a1fb-f93d98b8b494",
    "J. W. Frost",
);

// 5ED 435 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "6fbf2762-1e9c-4d7c-84d0-8561d9a41735",
    "J. W. Frost",
);

// 5ED 436 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "09376cac-44bd-42c4-9c07-9e4d6c972a64",
    "J. W. Frost",
);

// 5ED 437 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "da1218e0-8c75-4f8e-b317-051ce84949a5",
    "J. W. Frost",
);

// 5ED 438 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "6bc8f8cb-dc67-4549-be2f-f4d4057065a3",
    "Andrew Robinson",
);

// 5ED 439 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "0df95545-fe5e-47a5-99d8-1776e02c5713",
    "Andrew Robinson",
);

// 5ED 440 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "31228d3c-08e4-4042-a1a1-db8dff3177d9",
    "Andrew Robinson",
);

// 5ED 441 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "23e09e49-1a87-4967-8557-6d331c48a563",
    "Andrew Robinson",
);

// 5ED 442 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "44b08a29-8b53-44c9-82a0-ac689c9feeb5",
    "John Avon",
);

// 5ED 443 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "7c0916a2-e2f1-42b5-9f2c-b6a18a605138",
    "John Avon",
);

// 5ED 444 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "cdf5d88e-1699-4ce3-a4aa-3e8d1ef2bc39",
    "John Avon",
);

// 5ED 445 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "e1dfeb23-204c-4e48-8a23-c69daf92340f",
    "John Avon",
);

// 5ED 446 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "04eb20ea-1784-475e-a612-48057e5578c1",
    "David O'Connor",
);

// 5ED 447 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "13635ba7-0635-4c16-9e14-444470e06287",
    "David O'Connor",
);

// 5ED 448 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "925e07e8-a897-443f-8962-ef11fc41002a",
    "David O'Connor",
);

// 5ED 449 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "0cdbb450-4b80-4f81-9992-ee8fbeeb1857",
    "David O'Connor",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ABBEY_GARGOYLES_REPRINT,
    AKRON_LEGIONNAIRE_REPRINT,
    ALABASTER_POTION_REPRINT,
    ANGRY_MOB_REPRINT,
    ANIMATE_WALL_REPRINT,
    ARENSON_S_AURA_REPRINT,
    ARMAGEDDON_REPRINT,
    ARMOR_OF_FAITH_REPRINT,
    AYSEN_BUREAUCRATS_REPRINT,
    BENALISH_HERO_REPRINT,
    BLESSED_WINE_REPRINT,
    BLINKING_SPIRIT_REPRINT,
    BRAINWASH_REPRINT,
    CARIBOU_RANGE_REPRINT,
    CASTLE_REPRINT,
    CIRCLE_OF_PROTECTION_ARTIFACTS_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    CRUSADE_REPRINT,
    DAVENANT_ARCHER_REPRINT,
    DEATH_SPEAKERS_REPRINT,
    DEATH_WARD_REPRINT,
    DISENCHANT_REPRINT,
    DIVINE_OFFERING_REPRINT,
    DIVINE_TRANSFORMATION_REPRINT,
    DUST_TO_DUST_REPRINT,
    EYE_FOR_AN_EYE_REPRINT,
    GREATER_REALM_OF_PRESERVATION_REPRINT,
    HEAL_REPRINT,
    HEALING_SALVE_REPRINT,
    HIPPARION_REPRINT,
    HOLY_STRENGTH_REPRINT,
    ICATIAN_PHALANX_REPRINT,
    ICATIAN_SCOUT_REPRINT,
    ICATIAN_TOWN_REPRINT,
    ISLAND_SANCTUARY_REPRINT,
    IVORY_GUARDIANS_REPRINT,
    JUSTICE_REPRINT,
    KARMA_REPRINT,
    KISMET_REPRINT,
    KJELDORAN_ROYAL_GUARD_REPRINT,
    KJELDORAN_SKYCAPTAIN_REPRINT,
    MESA_FALCON_REPRINT,
    MESA_PEGASUS_REPRINT,
    ORDER_OF_THE_SACRED_TORCH_REPRINT,
    ORDER_OF_THE_WHITE_SHIELD_REPRINT,
    PEARLED_UNICORN_REPRINT,
    PERSONAL_INCARNATION_REPRINT,
    PIKEMEN_REPRINT,
    PRISMATIC_WARD_REPRINT,
    REPENTANT_BLACKSMITH_REPRINT,
    REVERSE_DAMAGE_REPRINT,
    RIGHTEOUSNESS_REPRINT,
    SACRED_BOON_REPRINT,
    SAMITE_HEALER_REPRINT,
    SERAPH_REPRINT,
    SERRA_BESTIARY_REPRINT,
    SERRA_PALADIN_REPRINT,
    SHIELD_BEARER_REPRINT,
    SHIELD_WALL_REPRINT,
    SPIRIT_LINK_REPRINT,
    TRUCE_REPRINT,
    TUNDRA_WOLVES_REPRINT,
    WALL_OF_SWORDS_REPRINT,
    WHITE_KNIGHT_REPRINT,
    WRATH_OF_GOD_REPRINT,
    AETHER_STORM_REPRINT,
    AIR_ELEMENTAL_REPRINT,
    ANTI_MAGIC_AURA_REPRINT,
    AZURE_DRAKE_REPRINT,
    BINDING_GRASP_REPRINT,
    BOOMERANG_REPRINT,
    BOOMERANG_ALTERNATE_1,
    BRAINSTORM_REPRINT,
    COUNTERSPELL_REPRINT,
    DANCE_OF_MANY_REPRINT,
    DANDAN_REPRINT,
    DARK_MAZE_REPRINT,
    DEFLECTION_REPRINT,
    DRAIN_POWER_REPRINT,
    ENERGY_FLUX_REPRINT,
    ENERVATE_REPRINT,
    FEEDBACK_REPRINT,
    FLIGHT_REPRINT,
    FLOOD_REPRINT,
    FORCE_SPIKE_REPRINT,
    FORGET_REPRINT,
    GASEOUS_FORM_REPRINT,
    GLACIAL_WALL_REPRINT,
    HOMARID_WARRIOR_REPRINT,
    HURKYLS_RECALL_REPRINT,
    HYDROBLAST_REPRINT,
    JUXTAPOSE_REPRINT,
    KROVIKAN_SORCERER_REPRINT,
    LABYRINTH_MINOTAUR_REPRINT,
    LEVIATHAN_REPRINT,
    LIFETAP_REPRINT,
    LORD_OF_ATLANTIS_REPRINT,
    MAGICAL_HACK_REPRINT,
    MAGUS_OF_THE_UNSEEN_REPRINT,
    MEMORY_LAPSE_REPRINT,
    MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT,
    MIND_BOMB_REPRINT,
    PHANTASMAL_FORCES_REPRINT,
    PHANTASMAL_TERRAIN_REPRINT,
    PHANTOM_MONSTER_REPRINT,
    PIRATE_SHIP_REPRINT,
    PORTENT_REPRINT,
    POWER_SINK_REPRINT,
    PRODIGAL_SORCERER_REPRINT,
    PSYCHIC_VENOM_REPRINT,
    RAY_OF_COMMAND_REPRINT,
    RECALL_REPRINT,
    REEF_PIRATES_REPRINT,
    REMOVE_SOUL_REPRINT,
    SEA_SERPENT_REPRINT,
    SEA_SPIRIT_REPRINT,
    SEA_SPRITE_REPRINT,
    SEASINGER_REPRINT,
    SEGOVIAN_LEVIATHAN_REPRINT,
    SIBILANT_SPIRIT_REPRINT,
    SLEIGHT_OF_MIND_REPRINT,
    SOUL_BARRIER_REPRINT,
    SPELL_BLAST_REPRINT,
    STASIS_REPRINT,
    STEAL_ARTIFACT_REPRINT,
    TIME_ELEMENTAL_REPRINT,
    TWIDDLE_REPRINT,
    UNSTABLE_MUTATION_REPRINT,
    UNSUMMON_REPRINT,
    UPDRAFT_REPRINT,
    VODALIAN_SOLDIERS_REPRINT,
    WALL_OF_AIR_REPRINT,
    WIND_SPIRIT_REPRINT,
    ZEPHYR_FALCON_REPRINT,
    ZUR_S_WEIRDING_REPRINT,
    ABYSSAL_SPECTER_REPRINT,
    ANIMATE_DEAD_REPRINT,
    ASHES_TO_ASHES_REPRINT,
    ASHES_TO_ASHES_ALTERNATE_1,
    BAD_MOON_REPRINT,
    BLACK_KNIGHT_REPRINT,
    BLIGHT_REPRINT,
    BOG_IMP_REPRINT,
    BOG_RATS_REPRINT,
    BOG_WRAITH_REPRINT,
    BOG_WRAITH_ALTERNATE_1,
    BREEDING_PIT_REPRINT,
    BROKEN_VISAGE_REPRINT,
    CARRION_ANTS_REPRINT,
    CLOAK_OF_CONFUSION_REPRINT,
    CURSED_LAND_REPRINT,
    DARK_RITUAL_REPRINT,
    DEATHGRIP_REPRINT,
    DERELOR_REPRINT,
    DRAIN_LIFE_REPRINT,
    DRUDGE_SKELETONS_REPRINT,
    ERG_RAIDERS_REPRINT,
    EVIL_EYE_OF_ORMS_BY_GORE_REPRINT,
    EVIL_PRESENCE_REPRINT,
    FALLEN_ANGEL_REPRINT,
    FEAR_REPRINT,
    FROZEN_SHADE_REPRINT,
    FUNERAL_MARCH_REPRINT,
    GLOOM_REPRINT,
    GREATER_WEREWOLF_REPRINT,
    HECATOMB_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    INITIATES_OF_THE_EBON_HAND_REPRINT,
    INITIATES_OF_THE_EBON_HAND_ALTERNATE_1,
    KJELDORAN_DEAD_REPRINT,
    KNIGHT_OF_STROMGALD_REPRINT,
    KROVIKAN_FETISH_REPRINT,
    LESHRAC_S_RITE_REPRINT,
    LORD_OF_THE_PIT_REPRINT,
    LOST_SOUL_REPRINT,
    MIND_RAVEL_REPRINT,
    MIND_WARP_REPRINT,
    MINDSTAB_THRULL_REPRINT,
    MINDSTAB_THRULL_ALTERNATE_1,
    MOLE_WORMS_REPRINT,
    MURK_DWELLERS_REPRINT,
    NECRITE_REPRINT,
    NECRITE_ALTERNATE_1,
    NECROPOTENCE_REPRINT,
    NETHER_SHADOW_REPRINT,
    NIGHTMARE_REPRINT,
    PARALYZE_REPRINT,
    PESTILENCE_REPRINT,
    PIT_SCORPION_REPRINT,
    PLAGUE_RATS_REPRINT,
    POX_REPRINT,
    RAG_MAN_REPRINT,
    RAISE_DEAD_REPRINT,
    SCATHE_ZOMBIES_REPRINT,
    SENGIR_AUTOCRAT_REPRINT,
    SORCERESS_QUEEN_REPRINT,
    STROMGALD_CABAL_REPRINT,
    TERROR_REPRINT,
    THE_WRETCHED_REPRINT,
    THRULL_RETAINER_REPRINT,
    TORTURE_REPRINT,
    TOUCH_OF_DEATH_REPRINT,
    UNHOLY_STRENGTH_REPRINT,
    VAMPIRE_BATS_REPRINT,
    WALL_OF_BONE_REPRINT,
    WARP_ARTIFACT_REPRINT,
    WEAKNESS_REPRINT,
    XENIC_POLTERGEIST_REPRINT,
    ZOMBIE_MASTER_REPRINT,
    AMBUSH_PARTY_REPRINT,
    ATOG_REPRINT,
    BALL_LIGHTNING_REPRINT,
    BIRD_MAIDEN_REPRINT,
    BLOOD_LUST_REPRINT,
    BRASSCLAW_ORCS_REPRINT,
    BROTHERS_OF_FIRE_REPRINT,
    CAVE_PEOPLE_REPRINT,
    CONQUER_REPRINT,
    CRIMSON_MANTICORE_REPRINT,
    DETONATE_REPRINT,
    DISINTEGRATE_REPRINT,
    DWARVEN_CATAPULT_REPRINT,
    DWARVEN_SOLDIER_REPRINT,
    DWARVEN_WARRIORS_REPRINT,
    EARTHQUAKE_REPRINT,
    ERRANTRY_REPRINT,
    ETERNAL_WARRIOR_REPRINT,
    FIRE_DRAKE_REPRINT,
    FIREBALL_REPRINT,
    FIREBREATHING_REPRINT,
    FLAME_SPIRIT_REPRINT,
    FLARE_REPRINT,
    FLASHFIRES_REPRINT,
    GAME_OF_CHAOS_REPRINT,
    GAME_OF_CHAOS_ALTERNATE_1,
    GIANT_STRENGTH_REPRINT,
    GOBLIN_DIGGING_TEAM_REPRINT,
    GOBLIN_HERO_REPRINT,
    GOBLIN_KING_REPRINT,
    GOBLIN_WAR_DRUMS_REPRINT,
    GOBLIN_WARRENS_REPRINT,
    HILL_GIANT_REPRINT,
    HURLOON_MINOTAUR_REPRINT,
    IMPOSING_VISAGE_REPRINT,
    INCINERATE_REPRINT,
    INFERNO_REPRINT,
    INFERNO_ALTERNATE_1,
    IRONCLAW_CURSE_REPRINT,
    IRONCLAW_CURSE_ALTERNATE_1,
    IRONCLAW_ORCS_REPRINT,
    JOKULHAUPS_REPRINT,
    KELDON_WARLORD_REPRINT,
    MANA_CLASH_REPRINT,
    MANA_FLARE_REPRINT,
    MANABARBS_REPRINT,
    MANABARBS_ALTERNATE_1,
    MONSS_GOBLIN_RAIDERS_REPRINT,
    MOUNTAIN_GOAT_REPRINT,
    ORCISH_ARTILLERY_REPRINT,
    ORCISH_CAPTAIN_REPRINT,
    ORCISH_CONSCRIPTS_REPRINT,
    ORCISH_FARMER_REPRINT,
    ORCISH_ORIFLAMME_REPRINT,
    ORCISH_SQUATTERS_REPRINT,
    ORGG_REPRINT,
    PANIC_REPRINT,
    PRIMORDIAL_OOZE_REPRINT,
    PYROBLAST_REPRINT,
    PYROTECHNICS_REPRINT,
    SABRETOOTH_TIGER_REPRINT,
    SHATTER_REPRINT,
    SHATTERSTORM_REPRINT,
    SHIVAN_DRAGON_REPRINT,
    SHIVAN_DRAGON_ALTERNATE_1,
    SMOKE_REPRINT,
    STONE_GIANT_REPRINT,
    STONE_RAIN_REPRINT,
    STONE_SPIRIT_REPRINT,
    THE_BRUTE_REPRINT,
    WALL_OF_FIRE_REPRINT,
    WALL_OF_STONE_REPRINT,
    WINDS_OF_CHANGE_REPRINT,
    WORD_OF_BLASTING_REPRINT,
    AN_HAVVA_CONSTABLE_REPRINT,
    ASPECT_OF_WOLF_REPRINT,
    AUROCHS_REPRINT,
    BIRDS_OF_PARADISE_REPRINT,
    CARAPACE_REPRINT,
    CAT_WARRIORS_REPRINT,
    CHUB_TOAD_REPRINT,
    COCKATRICE_REPRINT,
    CRAW_GIANT_REPRINT,
    CRAW_WURM_REPRINT,
    CRUMBLE_REPRINT,
    DESERT_TWISTER_REPRINT,
    DURKWOOD_BOARS_REPRINT,
    ELDER_DRUID_REPRINT,
    ELVEN_RIDERS_REPRINT,
    ELVISH_ARCHERS_REPRINT,
    FOG_REPRINT,
    FORCE_OF_NATURE_REPRINT,
    FOXFIRE_REPRINT,
    FUNGUSAUR_REPRINT,
    FYNDHORN_ELDER_REPRINT,
    GHAZBAN_OGRE_REPRINT,
    GIANT_GROWTH_REPRINT,
    GIANT_SPIDER_REPRINT,
    GRIZZLY_BEARS_REPRINT,
    HUNGRY_MIST_REPRINT,
    HURRICANE_REPRINT,
    INSTILL_ENERGY_REPRINT,
    IRONROOT_TREEFOLK_REPRINT,
    JOHTULL_WURM_REPRINT,
    KILLER_BEES_REPRINT,
    LEY_DRUID_REPRINT,
    LHURGOYF_REPRINT,
    LIFEFORCE_REPRINT,
    LIVING_ARTIFACT_REPRINT,
    LIVING_LANDS_REPRINT,
    LLANOWAR_ELVES_REPRINT,
    LURE_REPRINT,
    MARSH_VIPER_REPRINT,
    NATURE_S_LORE_REPRINT,
    PRADESH_GYPSIES_REPRINT,
    PRIMAL_ORDER_REPRINT,
    RABID_WOMBAT_REPRINT,
    RADJAN_SPIRIT_REPRINT,
    REGENERATION_REPRINT,
    SCALED_WURM_REPRINT,
    SCAVENGER_FOLK_REPRINT,
    SCRYB_SPRITES_REPRINT,
    SHANODIN_DRYADS_REPRINT,
    SHRINK_REPRINT,
    STAMPEDE_REPRINT,
    STREAM_OF_LIFE_REPRINT,
    SYLVAN_LIBRARY_REPRINT,
    TARPAN_REPRINT,
    THICKET_BASILISK_REPRINT,
    TITANIA_S_SONG_REPRINT,
    TRANQUILITY_REPRINT,
    TSUNAMI_REPRINT,
    UNTAMED_WILDS_REPRINT,
    VENOM_REPRINT,
    VERDURAN_ENCHANTRESS_REPRINT,
    WALL_OF_BRAMBLES_REPRINT,
    WANDERLUST_REPRINT,
    WAR_MAMMOTH_REPRINT,
    WHIRLING_DERVISH_REPRINT,
    WILD_GROWTH_REPRINT,
    WINTER_BLAST_REPRINT,
    WOLVERINE_PACK_REPRINT,
    WYLULI_WOLF_REPRINT,
    ALADDINS_RING_REPRINT,
    AMULET_OF_KROOG_REPRINT,
    ANKH_OF_MISHRA_REPRINT,
    ASHNOD_S_ALTAR_REPRINT,
    ASHNODS_TRANSMOGRANT_REPRINT,
    BARBED_SEXTANT_REPRINT,
    BARLS_CAGE_REPRINT,
    BATTERING_RAM_REPRINT,
    BOTTLE_OF_SULEIMAN_REPRINT,
    CLAY_STATUE_REPRINT,
    CLOCKWORK_BEAST_REPRINT,
    CLOCKWORK_STEED_REPRINT,
    COLOSSUS_OF_SARDIA_REPRINT,
    CORAL_HELM_REPRINT,
    CROWN_OF_THE_AGES_REPRINT,
    CRYSTAL_ROD_REPRINT,
    DANCING_SCIMITAR_REPRINT,
    DIABOLIC_MACHINE_REPRINT,
    DINGUS_EGG_REPRINT,
    DISRUPTING_SCEPTER_REPRINT,
    DRAGON_ENGINE_REPRINT,
    ELKIN_BOTTLE_REPRINT,
    FELDONS_CANE_REPRINT,
    FELLWAR_STONE_REPRINT,
    FEROZ_S_BAN_REPRINT,
    FLYING_CARPET_REPRINT,
    FOUNTAIN_OF_YOUTH_REPRINT,
    GAUNTLETS_OF_CHAOS_REPRINT,
    GLASSES_OF_URZA_REPRINT,
    GRAPESHOT_CATAPULT_REPRINT,
    HELM_OF_CHATZUK_REPRINT,
    HOWLING_MINE_REPRINT,
    INFINITE_HOURGLASS_REPRINT,
    IRON_STAR_REPRINT,
    IVORY_CUP_REPRINT,
    JADE_MONOLITH_REPRINT,
    JALUM_TOME_REPRINT,
    JANDORS_SADDLEBAGS_REPRINT,
    JAYEMDAE_TOME_REPRINT,
    JESTER_S_CAP_REPRINT,
    JOVEN_S_TOOLS_REPRINT,
    LIBRARY_OF_LENG_REPRINT,
    MANA_VAULT_REPRINT,
    MEEKSTONE_REPRINT,
    MILLSTONE_REPRINT,
    NEVINYRRALS_DISK_REPRINT,
    OBELISK_OF_UNDOING_REPRINT,
    ORNITHOPTER_REPRINT,
    PENTAGRAM_OF_THE_AGES_REPRINT,
    PRIMAL_CLAY_REPRINT,
    ROD_OF_RUIN_REPRINT,
    SERPENT_GENERATOR_REPRINT,
    SHAPESHIFTER_REPRINT,
    SKULL_CATAPULT_REPRINT,
    SOUL_NET_REPRINT,
    TAWNOSS_WEAPONRY_REPRINT,
    THE_HIVE_REPRINT,
    THRONE_OF_BONE_REPRINT,
    TIME_BOMB_REPRINT,
    URZA_S_AVENGER_REPRINT,
    URZAS_BAUBLE_REPRINT,
    WALL_OF_SPEARS_REPRINT,
    WINTER_ORB_REPRINT,
    WOODEN_SPHERE_REPRINT,
    ADARKAR_WASTES_REPRINT,
    BOTTOMLESS_VAULT_REPRINT,
    BRUSHLAND_REPRINT,
    CITY_OF_BRASS_REPRINT,
    DWARVEN_HOLD_REPRINT,
    DWARVEN_RUINS_REPRINT,
    EBON_STRONGHOLD_REPRINT,
    HAVENWOOD_BATTLEGROUND_REPRINT,
    HOLLOW_TREES_REPRINT,
    ICATIAN_STORE_REPRINT,
    ICE_FLOE_REPRINT,
    KARPLUSAN_FOREST_REPRINT,
    RUINS_OF_TROKAIR_REPRINT,
    SAND_SILOS_REPRINT,
    SULFUROUS_SPRINGS_REPRINT,
    SVYELUNITE_TEMPLE_REPRINT,
    UNDERGROUND_RIVER_REPRINT,
    URZA_S_MINE_REPRINT,
    URZA_S_POWER_PLANT_REPRINT,
    URZA_S_TOWER_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
