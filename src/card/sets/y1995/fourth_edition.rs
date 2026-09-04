//! Fourth Edition has no unique card definitions.
//!
//! It is the set the Premodern window opens on, so a card whose only earlier
//! printings predate that window becomes legal here.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::alpha;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::antiquities;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1994::the_dark as catalog_drk;

// 4ED 1 — Alabaster Potion (reprint)
const ALABASTER_POTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ALABASTER_POTION,
    "51c931a4-297a-4f7f-8f71-6bdcbe4e94b7",
    "Harold McNeill",
);

// 4ED 2 — Amrou Kithkin (reprint)
const AMROU_KITHKIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::AMROU_KITHKIN,
    "27de9b02-1810-41ee-aa90-5d89b6e1bf87",
    "Quinton Hoover",
);

// 4ED 3 — Angry Mob (reprint)
const ANGRY_MOB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::ANGRY_MOB,
    "a26d1c4d-af95-4b40-83a5-b51f8b2506db",
    "Drew Tucker",
);

// 4ED 4 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "b6ee58e5-f493-4e27-9639-5599a9573387",
    "Dan Frazier",
);

// 4ED 5 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ARMAGEDDON,
    "0794bc35-a8e1-4268-87b2-af5483ca6e5e",
    "Jesper Myrfors",
);

// 4ED 6 — Balance (reprint)
const BALANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BALANCE,
    "db3e219d-8e5e-44c7-97b3-2d489e2808b9",
    "Mark Poole",
);

// 4ED 7 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "fd204832-5f3b-4213-9c06-b89a296a9d47",
    "Douglas Shuler",
);

// 4ED 8 — Black Ward (reprint)
const BLACK_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_WARD,
    "218b1327-5f76-4ee2-a93d-d2ba412043c2",
    "Dan Frazier",
);

// 4ED 9 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLESSING,
    "eea35ac3-071d-4a42-9a7a-9104fe63bddc",
    "Julie Baroh",
);

// 4ED 10 — Blue Ward (reprint)
const BLUE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_WARD,
    "de62c833-c66b-442e-99ed-99ccd8eca024",
    "Dan Frazier",
);

// 4ED 11 — Brainwash (reprint)
const BRAINWASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BRAINWASH,
    "90c6d853-dc3a-4eed-894a-b58bf6e56dc8",
    "Pete Venters",
);

// 4ED 12 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "870ee86b-222b-4d7a-9c5b-ea10e1d73756",
    "Dameon Willich",
);

// 4ED 13 — Circle of Protection: Artifacts (reprint)
const CIRCLE_OF_PROTECTION_ARTIFACTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CIRCLE_OF_PROTECTION_ARTIFACTS,
    "6bfddcee-704f-4189-8cb0-6025fe98d7e9",
    "Pete Venters",
);

// 4ED 14 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "9971cbae-3f08-48c6-955a-76ce8ed41c6c",
    "Jesper Myrfors",
);

// 4ED 15 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "4b7cdfd5-bef9-4f33-b81a-2a395c9bba29",
    "Dameon Willich",
);

// 4ED 16 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "192c50c5-ef98-47bd-9cab-cb3feaf6080b",
    "Sandra Everingham",
);

// 4ED 17 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CIRCLE_OF_PROTECTION_RED,
    "f3575259-b019-47d0-a6d3-3fe4963294ae",
    "Mark Tedin",
);

// 4ED 18 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "92e1e153-cf5c-429a-9680-53b7c89dbd88",
    "Douglas Shuler",
);

// 4ED 19 — Conversion (reprint)
const CONVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONVERSION,
    "5df33281-bebc-4c2a-97ce-528a67574670",
    "Jesper Myrfors",
);

// 4ED 20 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRUSADE,
    "fb5428ef-847d-4ac1-b9a9-31afe155650f",
    "Mark Poole",
);

// 4ED 21 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "e9c54ffe-2479-4178-a5dc-dca7b564efce",
    "Mark Poole",
);

// 4ED 22 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "a915f261-2cdc-499c-9163-da5b628b0127",
    "Amy Weber",
);

// 4ED 23 — Divine Transformation (reprint)
const DIVINE_TRANSFORMATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DIVINE_TRANSFORMATION,
    "f59605bf-27d8-47ca-805e-527aab6ddf70",
    "NéNé Thomas",
);

// 4ED 24 — Elder Land Wurm (reprint)
const ELDER_LAND_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ELDER_LAND_WURM,
    "b188d0d4-cd7c-4314-ae65-cb49e4674241",
    "Quinton Hoover",
);

// 4ED 25 — Eye for an Eye (reprint)
const EYE_FOR_AN_EYE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EYE_FOR_AN_EYE,
    "acc41bca-73fc-42da-b639-b73a8de8a9c6",
    "Mark Poole",
);

// 4ED 26 — Fortified Area (reprint)
const FORTIFIED_AREA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FORTIFIED_AREA,
    "8f89968f-0182-4639-96fd-0dc94b6ad926",
    "Randy Asplund-Faith",
);

// 4ED 27 — Green Ward (reprint)
const GREEN_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GREEN_WARD,
    "b7548dc7-11dc-4611-85a5-71bdb48d8a62",
    "Dan Frazier",
);

// 4ED 28 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "afd08b3b-196e-43a5-9318-9dae9edc6d12",
    "Dan Frazier",
);

// 4ED 29 — Holy Armor (reprint)
const HOLY_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_ARMOR,
    "3a0b9dba-f621-4700-8a36-bf326e04cbac",
    "Melissa A. Benson",
);

// 4ED 30 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "77af9f19-66d1-44e5-9aed-955e6bfd9902",
    "Anson Maddocks",
);

// 4ED 31 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "c0eff4d1-4ce6-48b1-84ae-a95bd1d04d16",
    "Mark Poole",
);

// 4ED 32 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "2acd8dd7-f561-487f-abae-4ad910db9d27",
    "Richard Thomas",
);

// 4ED 33 — Kismet (reprint)
const KISMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KISMET,
    "f2381234-cf5a-4baf-89f9-cdec15557cf4",
    "Kaja Foglio",
);

// 4ED 34 — Land Tax (reprint)
const LAND_TAX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::LAND_TAX,
    "1301e203-7d9a-4735-8db3-7882ad70d343",
    "Brian Snõddy",
);

// 4ED 35 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "60c0faf1-0c16-441a-97fd-6d4e91c894a5",
    "Melissa A. Benson",
);

// 4ED 36 — Morale (reprint)
const MORALE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MORALE,
    "b52cdfe3-fca1-420c-b5c0-2366ea58345b",
    "Mark Poole",
);

// 4ED 37 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "09e97dce-6bef-480d-939e-d35cfbf6b989",
    "Douglas Shuler",
);

// 4ED 38 — Osai Vultures (reprint)
const OSAI_VULTURES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::OSAI_VULTURES,
    "da915d3c-8d43-44c2-9052-08c1cd2afa7b",
    "Dan Frazier",
);

// 4ED 39 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "6acc78fc-1eab-47aa-993d-6781b2d2408a",
    "Cornelius Brudi",
);

// 4ED 40 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "0b107c80-0c9a-46cd-8f00-26c37f080ce7",
    "Kev Brockschmidt",
);

// 4ED 41 — Piety (reprint)
const PIETY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::PIETY,
    "d4942a9f-6b8f-438b-a2ea-366228038ed8",
    "Mark Poole",
);

// 4ED 42 — Pikemen (reprint)
const PIKEMEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::PIKEMEN,
    "015cc1ad-8b9f-484c-aa7d-d44f7bc914d2",
    "Dennis Detwiller",
);

// 4ED 43 — Purelace (reprint)
const PURELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PURELACE,
    "5736fcc7-fa95-4ef4-b821-392ec00e03bf",
    "Sandra Everingham",
);

// 4ED 44 — Red Ward (reprint)
const RED_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_WARD,
    "543e8ec0-f36a-46a6-b7b1-e983e90a091a",
    "Dan Frazier",
);

// 4ED 45 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "cd458c66-0183-4bad-a651-6fe52a6d14c4",
    "Dameon Willich",
);

// 4ED 46 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "3fccc7f2-b317-40c4-9b55-658d67dca843",
    "Douglas Shuler",
);

// 4ED 47 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "b354858f-d25b-4264-8a0b-fecc3a58db6d",
    "Tom Wänerstrand",
);

// 4ED 48 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAVANNAH_LIONS,
    "a2ee9127-d007-48e8-b797-88ef72bc7c8b",
    "Daniel Gelon",
);

// 4ED 49 — Seeker (reprint)
const SEEKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SEEKER,
    "7052825f-d9fc-4820-b8c4-dbc204730b7c",
    "Mark Poole",
);

// 4ED 50 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SERRA_ANGEL,
    "8072d00a-82ff-4406-bdf8-1af20cd3a170",
    "Douglas Shuler",
);

// 4ED 51 — Spirit Link (reprint)
const SPIRIT_LINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SPIRIT_LINK,
    "58838b77-3bac-41b7-8055-5737c07df12e",
    "Kaja Foglio",
);

// 4ED 52 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWORDS_TO_PLOWSHARES,
    "8bd501ae-5814-4336-a45b-2b88cb85d29e",
    "Jeff A. Menges",
);

// 4ED 53 — Tundra Wolves (reprint)
const TUNDRA_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TUNDRA_WOLVES,
    "0c8527ac-27c3-4e2d-b46c-bfd6a1e1f778",
    "Quinton Hoover",
);

// 4ED 54 — Visions (reprint)
const VISIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::VISIONS,
    "b0fed0e6-0e56-4987-b0dd-b294156c0233",
    "NéNé Thomas",
);

// 4ED 55 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "18ffa85c-61c4-49b7-8946-ab353355a11c",
    "Mark Tedin",
);

// 4ED 56 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_KNIGHT,
    "2e8f8a10-6984-46a7-a641-5f712dab5c57",
    "Daniel Gelon",
);

// 4ED 57 — White Ward (reprint)
const WHITE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_WARD,
    "2d3c467f-0e33-43e8-b108-0ea00d6adcc2",
    "Dan Frazier",
);

// 4ED 58 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WRATH_OF_GOD,
    "4566f6a3-4d25-4df0-84be-fe4201138955",
    "Quinton Hoover",
);

// 4ED 59 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "e5cfaefb-764c-4c56-bdb3-5f0375168597",
    "Richard Thomas",
);

// 4ED 60 — Animate Artifact (reprint)
const ANIMATE_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANIMATE_ARTIFACT,
    "ea613c48-258b-499d-975b-f56fbef3c665",
    "Douglas Shuler",
);

// 4ED 61 — Apprentice Wizard (reprint)
const APPRENTICE_WIZARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::APPRENTICE_WIZARD,
    "c9ef569e-91e7-45f5-83b0-5a820242c628",
    "Dan Frazier",
);

// 4ED 62 — Backfire (reprint)
const BACKFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BACKFIRE,
    "ad34094d-a7ec-4b04-a288-4d4f1a07fc6b",
    "Brian Snõddy",
);

// 4ED 63 — Blue Elemental Blast (reprint)
const BLUE_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLUE_ELEMENTAL_BLAST,
    "988f7b31-24b2-45c2-89e4-ff6bd9e0c8c7",
    "Richard Thomas",
);

// 4ED 64 — Control Magic (reprint)
const CONTROL_MAGIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTROL_MAGIC,
    "1bb371e5-424e-42dd-9966-732c22e8ece8",
    "Dameon Willich",
);

// 4ED 65 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COUNTERSPELL,
    "e8493631-6c9c-40a8-b7de-ecf26ba6bf7d",
    "Mark Poole",
);

// 4ED 66 — Creature Bond (reprint)
const CREATURE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CREATURE_BOND,
    "717c5ee5-a033-4a58-bdcb-ef54e6c8b7a9",
    "Anson Maddocks",
);

// 4ED 67 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "e3009237-fb96-497f-9a6e-b93b8115528a",
    "Douglas Shuler",
);

// 4ED 68 — Energy Flux (reprint)
const ENERGY_FLUX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ENERGY_FLUX,
    "fbfa8400-1820-4108-a7f3-6ae8a1897f0c",
    "Kaja Foglio",
);

// 4ED 69 — Energy Tap (reprint)
const ENERGY_TAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ENERGY_TAP,
    "9f67692d-9df6-4841-a36b-26c28110b63b",
    "Daniel Gelon",
);

// 4ED 70 — Erosion (reprint)
const EROSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::EROSION,
    "8dc539ff-03f5-4bec-877a-061fb5d40de9",
    "Pete Venters",
);

// 4ED 71 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "eb5a60e6-be5f-454e-a354-ab736839f092",
    "Quinton Hoover",
);

// 4ED 72 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "1867cb70-f615-4583-bf0b-fb1016209785",
    "Anson Maddocks",
);

// 4ED 73 — Flood (reprint)
const FLOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FLOOD,
    "ba099785-bfa0-4224-be19-0dcded94fba5",
    "Dennis Detwiller",
);

// 4ED 74 — Gaseous Form (reprint)
const GASEOUS_FORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GASEOUS_FORM,
    "cfdd00f0-c6aa-4e8b-a035-fb3403711741",
    "Phil Foglio",
);

// 4ED 75 — Ghost Ship (reprint)
const GHOST_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GHOST_SHIP,
    "3af0b9ce-0a9c-4619-95c1-bbbfffa7fcec",
    "Tom Wänerstrand",
);

// 4ED 76 — Giant Tortoise (reprint)
const GIANT_TORTOISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::GIANT_TORTOISE,
    "f512addb-5888-49f3-985b-53cc11831e5e",
    "Kaja Foglio",
);

// 4ED 77 — Hurkyl's Recall (reprint)
const HURKYLS_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::HURKYLS_RECALL,
    "bee569a6-20b0-4d37-91d6-db18de4b90c6",
    "NéNé Thomas",
);

// 4ED 78 — Island Fish Jasconius (reprint)
const ISLAND_FISH_JASCONIUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ISLAND_FISH_JASCONIUS,
    "84f18188-70fa-433e-a114-2f1dd49388ed",
    "Jesper Myrfors",
);

// 4ED 79 — Jump (reprint)
const JUMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUMP,
    "7b994bf9-5b3d-42c7-b4c6-f1029d78dd80",
    "Mark Poole",
);

// 4ED 80 — Leviathan (reprint)
const LEVIATHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::LEVIATHAN,
    "fbc98990-7fb0-4f69-a80d-38443966b3ae",
    "Mark Tedin",
);

// 4ED 81 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "634dd3bd-d29b-4481-990b-f320e60c4f91",
    "Anson Maddocks",
);

// 4ED 82 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "846572ce-c8fd-402c-9212-8288336c75e8",
    "Melissa A. Benson",
);

// 4ED 83 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "41c57f8a-6592-49b4-acb1-9b30500d1e8c",
    "Julie Baroh",
);

// 4ED 84 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "71935f14-d7ca-4406-9263-b2c7f5f5b94f",
    "Dan Frazier",
);

// 4ED 85 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_SHORT,
    "dd84fdcc-5463-45e9-9421-1d554ecdb8ae",
    "Dameon Willich",
);

// 4ED 86 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "93661c9f-b529-4d82-b68d-3ce050f77e0d",
    "Jeff A. Menges",
);

// 4ED 87 — Mind Bomb (reprint)
const MIND_BOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MIND_BOMB,
    "2ae30614-f558-4627-8e08-bf8a30ecd5b9",
    "Mark Tedin",
);

// 4ED 88 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "0bc15dcf-6f78-4968-8af9-c115dfa10e4c",
    "Mark Poole",
);

// 4ED 89 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "0d7b3b65-e1f9-4a0d-95a6-42c2c4a358e5",
    "Dameon Willich",
);

// 4ED 90 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "daa120ce-63a6-4d5e-9801-123d5e05646e",
    "Jesper Myrfors",
);

// 4ED 91 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "0d7f104a-fb06-41a4-abd9-38f2c6017bbf",
    "Tom Wänerstrand",
);

// 4ED 92 — Power Leak (reprint)
const POWER_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_LEAK,
    "99f235d7-8f6b-4d17-bc36-6f2cb6d5deec",
    "Drew Tucker",
);

// 4ED 93 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "e91c51ab-c8b1-47be-8169-7f842712771c",
    "Richard Thomas",
);

// 4ED 94 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "dc4e1161-5008-427f-a88e-5497a8eb84cd",
    "Douglas Shuler",
);

// 4ED 95 — Psionic Entity (reprint)
const PSIONIC_ENTITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PSIONIC_ENTITY,
    "6641a587-c066-4f0a-a951-b91d3f749eb2",
    "Justin Hampton",
);

// 4ED 96 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "138db623-393d-485a-8fa8-f1b1b933fce5",
    "Brian Snõddy",
);

// 4ED 97 — Relic Bind (reprint)
const RELIC_BIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RELIC_BIND,
    "05eee3dd-e17f-4154-8f8d-29c5421cd89d",
    "Christopher Rush",
);

// 4ED 98 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "e474391c-9ff0-4db4-9352-3fc07eeb4b51",
    "Jeff A. Menges",
);

// 4ED 99 — Segovian Leviathan (reprint)
const SEGOVIAN_LEVIATHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SEGOVIAN_LEVIATHAN,
    "22e6e890-d1d0-4a9b-a453-5b51974a9dff",
    "Melissa A. Benson",
);

// 4ED 100 — Sindbad (reprint)
const SINDBAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SINDBAD,
    "9b6906d0-4963-4f4b-aa29-ad98e9944107",
    "Julie Baroh",
);

// 4ED 101 — Siren's Call (reprint)
const SIREN_S_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIREN_S_CALL,
    "51832cfb-0a2e-4674-bb36-38027a71ac6d",
    "Anson Maddocks",
);

// 4ED 102 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "cd21dc3d-c9e6-4592-a655-1a20ee13ae73",
    "Mark Poole",
);

// 4ED 103 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "008df307-f010-47bc-8548-65a1c7b1c4b8",
    "Brian Snõddy",
);

// 4ED 104 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STASIS,
    "f3d29fed-248d-4547-9455-3d69b1f2787d",
    "Fay Jones",
);

// 4ED 105 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "dcd9ee1d-d1c5-4725-98fc-f6cff3a79512",
    "Amy Weber",
);

// 4ED 106 — Sunken City (reprint)
const SUNKEN_CITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::SUNKEN_CITY,
    "226f89b4-cf98-4354-a2f4-3f63fe2dd99e",
    "Jesper Myrfors",
);

// 4ED 107 — Thoughtlace (reprint)
const THOUGHTLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THOUGHTLACE,
    "185674a4-db97-444d-b0e9-7dcfc245ce4b",
    "Mark Poole",
);

// 4ED 107† — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "0c59579a-326b-4f82-ac33-1f4912db367b",
    "Sandra Everingham",
);

// 4ED 108 — Time Elemental (reprint)
const TIME_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TIME_ELEMENTAL,
    "957fa561-d090-435d-a0da-f405edb7e591",
    "Amy Weber",
);

// 4ED 109 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "db401643-6114-4254-8e40-ea7605e5ed82",
    "Rob Alexander",
);

// 4ED 110 — Unstable Mutation (reprint)
const UNSTABLE_MUTATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::UNSTABLE_MUTATION,
    "841ae3a3-30b5-4f0c-ad2f-af3d5e0da2e9",
    "Douglas Shuler",
);

// 4ED 111 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "338948aa-463d-4af4-a0d6-64132512334c",
    "Douglas Shuler",
);

// 4ED 112 — Volcanic Eruption (reprint)
const VOLCANIC_ERUPTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VOLCANIC_ERUPTION,
    "5828713a-edb3-4b11-b1f9-8f1bfc3c103f",
    "Douglas Shuler",
);

// 4ED 113 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "239435c8-3380-4556-9618-09fc0e40b69d",
    "Richard Thomas",
);

// 4ED 114 — Wall of Water (reprint)
const WALL_OF_WATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WATER,
    "78028eda-61b0-408c-b3fc-adc968d39b47",
    "Richard Thomas",
);

// 4ED 115 — Water Elemental (reprint)
const WATER_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WATER_ELEMENTAL,
    "dadea972-714e-4f6d-8fb7-a59c0ac1e028",
    "Jeff A. Menges",
);

// 4ED 116 — Zephyr Falcon (reprint)
const ZEPHYR_FALCON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ZEPHYR_FALCON,
    "032ee0fd-9eb3-4d81-9972-9e7f4c4d64e4",
    "Heather Hudson",
);

// 4ED 117 — Abomination (reprint)
const ABOMINATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ABOMINATION,
    "a363bc91-8278-448e-9d5c-564e4b51eb62",
    "Mark Tedin",
);

// 4ED 118 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "cc1dc456-1f64-4f24-a646-84c57e641b3b",
    "Anson Maddocks",
);

// 4ED 119 — Ashes to Ashes (reprint)
const ASHES_TO_ASHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::ASHES_TO_ASHES,
    "28f40650-9dd5-473d-a660-448672d475a5",
    "Drew Tucker",
);

// 4ED 120 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "aca216e2-27a5-40e1-bb61-0ddcd8ee02ae",
    "Jesper Myrfors",
);

// 4ED 121 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_KNIGHT,
    "47716cf4-f35c-4613-9686-4e00b5063408",
    "Jeff A. Menges",
);

// 4ED 122 — Blight (reprint)
const BLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLIGHT,
    "59365350-d149-4c0d-a08f-eabdfb7cce3d",
    "Pete Venters",
);

// 4ED 123 — Bog Imp (reprint)
const BOG_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_IMP,
    "c50c3053-3cf3-40d3-bc9e-2fc292f11c34",
    "Ron Spencer",
);

// 4ED 124 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "fbf82ad6-b32e-41ea-abea-22b8eef69e67",
    "Jeff A. Menges",
);

// 4ED 125 — Carrion Ants (reprint)
const CARRION_ANTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CARRION_ANTS,
    "f5de329d-fb08-4c4a-883f-1d77dc64470d",
    "Richard Thomas",
);

// 4ED 126 — Cosmic Horror (reprint)
const COSMIC_HORROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::COSMIC_HORROR,
    "07e975cc-e89d-4ac3-b9dc-dad202dacc14",
    "Jesper Myrfors",
);

// 4ED 127 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "23d9fb0a-1087-43cd-9e14-24c1685a2057",
    "Jesper Myrfors",
);

// 4ED 128 — Cyclopean Mummy (reprint)
const CYCLOPEAN_MUMMY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CYCLOPEAN_MUMMY,
    "8201b51b-da03-4207-805c-134e527c42e1",
    "Edward P. Beard, Jr.",
);

// 4ED 129 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "3477f601-5374-4316-a74e-b5e198af482b",
    "Sandra Everingham",
);

// 4ED 130 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "74e2ae05-29ec-4005-8d78-280a190601c4",
    "Anson Maddocks",
);

// 4ED 131 — Deathlace (reprint)
const DEATHLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHLACE,
    "237e37fb-383d-432c-8ac3-1332096567db",
    "Sandra Everingham",
);

// 4ED 132 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_LIFE,
    "b9d10677-36f0-4339-a2c3-213e8ee1c51d",
    "Douglas Shuler",
);

// 4ED 133 — Drudge Skeletons (alternate printing)
const DRUDGE_SKELETONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DRUDGE_SKELETONS,
    1,
    "d3d223d3-72c1-4240-9323-84484167c5e0",
    "Sandra Everingham",
);

// 4ED 134 — El-Hajjâj (reprint)
const EL_HAJJAJ_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EL_HAJJAJ,
    "cf371bd2-89ad-487e-8f27-37a6e75ca0f5",
    "Dameon Willich",
);

// 4ED 134† — El-Hajjâj (alternate printing)
const EL_HAJJAJ_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_arn::EL_HAJJAJ,
    1,
    "320906ee-791a-41c0-9b28-9a287cdb3340",
    "Amy Weber",
);

// 4ED 135 — Erg Raiders (reprint)
const ERG_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ERG_RAIDERS,
    "472c83ff-66aa-485b-9ca7-aef8731de20a",
    "Dameon Willich",
);

// 4ED 136 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "5cbca45f-7b08-408b-9445-e3f2b9563f1b",
    "Sandra Everingham",
);

// 4ED 137 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "126c1b02-01bd-4b70-9293-a89173b4cd32",
    "Mark Poole",
);

// 4ED 138 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "95547c2a-c01b-45ed-b775-f33c157b17a5",
    "Douglas Shuler",
);

// 4ED 139 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "f5a8573d-0e2d-4a7f-a47f-7e1bc472d4fa",
    "Dan Frazier",
);

// 4ED 140 — Greed (reprint)
const GREED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GREED,
    "76fb0c62-1c53-464e-a2df-59285f2d593d",
    "Phil Foglio",
);

// 4ED 141 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "5dba7b1e-90a0-419e-bcba-a6b19beacf29",
    "Mark Poole",
);

// 4ED 142 — Hypnotic Specter (reprint)
const HYPNOTIC_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HYPNOTIC_SPECTER,
    "b5900350-be08-4904-8f1b-cc180ed08485",
    "Douglas Shuler",
);

// 4ED 143 — Junún Efreet (reprint)
const JUNUN_EFREET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JUNUN_EFREET,
    "398a2b0f-0b91-408c-8083-3bc89873b69f",
    "Christopher Rush",
);

// 4ED 144 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "9691e76c-770d-4721-b758-805574227de1",
    "Mark Tedin",
);

// 4ED 145 — Lost Soul (reprint)
const LOST_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::LOST_SOUL,
    "66b875b1-e534-4ed8-9ebd-8f4d5a066e7d",
    "Randy Asplund-Faith",
);

// 4ED 146 — Marsh Gas (reprint)
const MARSH_GAS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MARSH_GAS,
    "c8c65bf9-cbab-45ee-9c6e-f8ee832dbe61",
    "Douglas Shuler",
);

// 4ED 147 — Mind Twist (reprint)
const MIND_TWIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MIND_TWIST,
    "bbb6765c-a052-46dc-a589-200b8ba8c99f",
    "Julie Baroh",
);

// 4ED 148 — Murk Dwellers (reprint)
const MURK_DWELLERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MURK_DWELLERS,
    "7731dc37-2eac-4e60-bb0f-6230205a5323",
    "Drew Tucker",
);

// 4ED 149 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "1b6d9e96-0c7f-45c5-b6d0-6444ef84bab1",
    "Christopher Rush",
);

// 4ED 150 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "db11b832-7584-4e4b-8d02-b03fe76dcbc3",
    "Melissa A. Benson",
);

// 4ED 151 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "d0bcad9a-b69c-434a-8d48-6b727bd8e382",
    "Anson Maddocks",
);

// 4ED 152 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "90886a02-4c1f-48ea-8563-6d80c97d9f58",
    "Jesper Myrfors",
);

// 4ED 153 — Pit Scorpion (reprint)
const PIT_SCORPION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PIT_SCORPION,
    "0a53418c-3720-4aa3-a525-6bc82c363844",
    "Scott Kirschner",
);

// 4ED 154 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "901591cb-a7e9-47c1-96ff-3de7b39b1055",
    "Anson Maddocks",
);

// 4ED 155 — Rag Man (reprint)
const RAG_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::RAG_MAN,
    "018a02f9-81f7-412b-8e98-d7be3eb73eca",
    "Daniel Gelon",
);

// 4ED 156 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "2096e54e-1a55-4fd7-8415-a6f2fdb2a536",
    "Jeff A. Menges",
);

// 4ED 157 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROYAL_ASSASSIN,
    "b2d51bdf-f118-4a1e-9060-bdf3c78697f2",
    "Tom Wänerstrand",
);

// 4ED 158 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "dbd55d32-e969-4d12-b101-a35db8a53921",
    "Jesper Myrfors",
);

// 4ED 159 — Scavenging Ghoul (reprint)
const SCAVENGING_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCAVENGING_GHOUL,
    "df0baaa9-c42b-413a-b10d-95689e4ddb50",
    "Jeff A. Menges",
);

// 4ED 160 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SENGIR_VAMPIRE,
    "2e29882a-c194-48b8-ba18-439b0bda395e",
    "Anson Maddocks",
);

// 4ED 161 — Simulacrum (reprint)
const SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIMULACRUM,
    "7c23232c-a264-4df9-824b-4111a5c6524c",
    "Mark Poole",
);

// 4ED 162 — Sorceress Queen (reprint)
const SORCERESS_QUEEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SORCERESS_QUEEN,
    "e4855db6-570c-4845-9074-a9df7611307a",
    "Kaja Foglio",
);

// 4ED 163 — Spirit Shackle (reprint)
const SPIRIT_SHACKLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SPIRIT_SHACKLE,
    "1df0c902-3fe1-4de7-9777-7ddb563b6cec",
    "Edward P. Beard, Jr.",
);

// 4ED 164 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TERROR,
    "deb5d44d-977d-4d0d-8242-17c8ffa7247c",
    "Ron Spencer",
);

// 4ED 165 — Uncle Istvan (reprint)
const UNCLE_ISTVAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::UNCLE_ISTVAN,
    "cbf5aa58-f68b-4197-9697-1e6653760853",
    "Daniel Gelon",
);

// 4ED 166 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "f52c3c85-1462-406c-8e3b-c25deda9c4e6",
    "Douglas Shuler",
);

// 4ED 167 — Vampire Bats (reprint)
const VAMPIRE_BATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::VAMPIRE_BATS,
    "d2aed701-5164-4c63-9539-3d2406970f46",
    "Anson Maddocks",
);

// 4ED 168 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "36ce3251-017f-4834-9935-0985c56cccb7",
    "Anson Maddocks",
);

// 4ED 169 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "edc2fcd5-e7a5-4a66-b385-82726ee6e3cc",
    "Amy Weber",
);

// 4ED 170 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "548d4cd1-7cc6-44ee-872e-68e5bcc608fb",
    "Douglas Shuler",
);

// 4ED 171 — Will-o'-the-Wisp (reprint)
const WILL_O_THE_WISP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILL_O_THE_WISP,
    "583ef317-8105-4917-ba3c-93de3eebd944",
    "Jesper Myrfors",
);

// 4ED 172 — Word of Binding (reprint)
const WORD_OF_BINDING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::WORD_OF_BINDING,
    "715472e1-585b-4379-b335-8f3b5e572d83",
    "Ron Spencer",
);

// 4ED 173 — Xenic Poltergeist (reprint)
const XENIC_POLTERGEIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::XENIC_POLTERGEIST,
    "a3c546db-0871-4a13-8654-c26ae9f9b50a",
    "Dan Frazier",
);

// 4ED 174 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "d86753ae-c8da-47d7-b97a-3c75f83ee929",
    "Jeff A. Menges",
);

// 4ED 175 — Ali Baba (reprint)
const ALI_BABA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALI_BABA,
    "92191f72-e3d7-4679-97a2-f0c2d19d7738",
    "Julie Baroh",
);

// 4ED 176 — Ball Lightning (reprint)
const BALL_LIGHTNING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BALL_LIGHTNING,
    "6bccf9a0-8d93-4b5e-ada0-1f19f260e5a8",
    "Quinton Hoover",
);

// 4ED 177 — Bird Maiden (reprint)
const BIRD_MAIDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BIRD_MAIDEN,
    "fc81a99a-0380-456f-8e81-753add511dbe",
    "Kaja Foglio",
);

// 4ED 178 — Blood Lust (reprint)
const BLOOD_LUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLOOD_LUST,
    "426e477f-2873-4115-9527-a50a97769dd1",
    "Anson Maddocks",
);

// 4ED 179 — Brothers of Fire (reprint)
const BROTHERS_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BROTHERS_OF_FIRE,
    "66674f3a-e882-44f1-addc-d74605150c39",
    "Mark Tedin",
);

// 4ED 180 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "946f1a83-6fcc-45b0-91fc-9c75504a16f3",
    "Mark Poole",
);

// 4ED 181 — Cave People (reprint)
const CAVE_PEOPLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::CAVE_PEOPLE,
    "99e9ff47-b98c-4ab6-a4bc-8914360c3f6d",
    "Drew Tucker",
);

// 4ED 182 — Chaoslace (reprint)
const CHAOSLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHAOSLACE,
    "476180df-8b88-4ead-b6c5-6ccb3e8a2cfd",
    "Dameon Willich",
);

// 4ED 183 — Crimson Manticore (reprint)
const CRIMSON_MANTICORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CRIMSON_MANTICORE,
    "cac8fc5c-368f-4b25-a33e-a32855037c4b",
    "Daniel Gelon",
);

// 4ED 184 — Detonate (reprint)
const DETONATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DETONATE,
    "26e23d23-92ca-4d74-88e8-e1137d7faaf1",
    "Randy Asplund-Faith",
);

// 4ED 185 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "5f41f868-454e-4075-9ef4-bcf3754d421c",
    "Anson Maddocks",
);

// 4ED 186 — Dragon Whelp (reprint)
const DRAGON_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAGON_WHELP,
    "349ff6e6-b914-4787-bb90-ea77a3550d23",
    "Amy Weber",
);

// 4ED 187 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "3616f381-ca10-4945-b95e-96dfefa3c303",
    "Douglas Shuler",
);

// 4ED 188 — Earth Elemental (reprint)
const EARTH_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTH_ELEMENTAL,
    "a5a859fd-28a3-470f-adf4-c6431ced27e1",
    "Dan Frazier",
);

// 4ED 189 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::EARTHQUAKE,
    "19ec03ce-a8a3-42c3-a895-21cce1657411",
    "Dan Frazier",
);

// 4ED 190 — Eternal Warrior (reprint)
const ETERNAL_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ETERNAL_WARRIOR,
    "9d574075-9909-444a-817f-d6ad419aa62c",
    "Anson Maddocks",
);

// 4ED 191 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "0261cd0f-e763-49b5-96ed-5e5767a3d8d7",
    "Melissa A. Benson",
);

// 4ED 192 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBALL,
    "afe078d7-cdde-44bb-942e-417123ebeccb",
    "Mark Tedin",
);

// 4ED 193 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "294892cb-3927-4bf0-97ca-f37bdd0b9de5",
    "Dan Frazier",
);

// 4ED 194 — Fissure (reprint)
const FISSURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FISSURE,
    "7601b15a-53d8-499f-837b-abf369d1e3f4",
    "Douglas Shuler",
);

// 4ED 195 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "ed0de18a-1dad-49f1-8127-09aa07b69eb0",
    "Dameon Willich",
);

// 4ED 196 — Giant Strength (reprint)
const GIANT_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GIANT_STRENGTH,
    "f1aebd61-311c-4269-aa75-5c004639178d",
    "Justin Hampton",
);

// 4ED 197 — Goblin Balloon Brigade (reprint)
const GOBLIN_BALLOON_BRIGADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_BALLOON_BRIGADE,
    "866df71c-856c-451d-a11b-c2086c265868",
    "Andi Rusu",
);

// 4ED 198 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_KING,
    "92dd4aef-6d15-4d74-96cf-1cc51812b541",
    "Jesper Myrfors",
);

// 4ED 199 — Goblin Rock Sled (reprint)
const GOBLIN_ROCK_SLED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_ROCK_SLED,
    "25f4f0c1-a43e-4282-b20e-91c711c57521",
    "Dennis Detwiller",
);

// 4ED 200 — Gray Ogre (reprint)
const GRAY_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRAY_OGRE,
    "11bf2cc0-799f-4eb8-b338-ed7543f469e7",
    "Dan Frazier",
);

// 4ED 201 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "6ac25236-c2f2-48df-8dbb-d4f9ce790cfb",
    "Dan Frazier",
);

// 4ED 202 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "5c2c27de-e788-4ad7-b1e3-030e7ef00471",
    "Anson Maddocks",
);

// 4ED 203 — Hurr Jackal (reprint)
const HURR_JACKAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::HURR_JACKAL,
    "c1c0ab2b-04bf-4cba-8efa-3d1d0ab4f50d",
    "Drew Tucker",
);

// 4ED 204 — Immolation (reprint)
const IMMOLATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::IMMOLATION,
    "f9dbfdc1-a598-4c72-9b0d-91207bd067ab",
    "Scott Kirschner",
);

// 4ED 205 — Inferno (reprint)
const INFERNO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::INFERNO,
    "3db90782-fa89-4470-bec5-27614e88a0a9",
    "Randy Asplund-Faith",
);

// 4ED 206 — Ironclaw Orcs (reprint)
const IRONCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONCLAW_ORCS,
    "d882105f-2de0-46ae-b673-01508c016cc6",
    "Anson Maddocks",
);

// 4ED 207 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "b1a32db7-e1a8-47d8-a708-987bcbf0636e",
    "Kev Brockschmidt",
);

// 4ED 208 — Lightning Bolt (reprint)
const LIGHTNING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LIGHTNING_BOLT,
    "9521375e-0bc1-45ef-b513-6d332a25f9d2",
    "Christopher Rush",
);

// 4ED 209 — Magnetic Mountain (reprint)
const MAGNETIC_MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::MAGNETIC_MOUNTAIN,
    "993a14d5-a33d-426e-ab49-c3226a6fcdca",
    "Susan Van Camp",
);

// 4ED 210 — Mana Clash (reprint)
const MANA_CLASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MANA_CLASH,
    "a01f27b7-f1d0-4fb6-b743-ca7a810ef85c",
    "Mark Tedin",
);

// 4ED 211 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "e7169e26-e700-4e71-b959-4592a03f3c9f",
    "Christopher Rush",
);

// 4ED 212 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "ffb4f505-bc05-44ac-9190-60101f813c65",
    "Christopher Rush",
);

// 4ED 213 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "9f31c715-9ab8-4578-989f-141099b6750c",
    "Jeff A. Menges",
);

// 4ED 214 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "19281eb4-f5b6-4b27-8c65-7e56d2a8ab77",
    "Anson Maddocks",
);

// 4ED 215 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "56cc0d8b-2d4b-4c3b-a9af-b5fae35e6ec5",
    "Dan Frazier",
);

// 4ED 216 — Power Surge (reprint)
const POWER_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SURGE,
    "0b5717af-a1a3-45cb-8b05-7543eed5532a",
    "Douglas Shuler",
);

// 4ED 217 — Pyrotechnics (reprint)
const PYROTECHNICS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PYROTECHNICS,
    "5e32142d-b161-4497-a6cd-ba4c67e16a6f",
    "Anson Maddocks",
);

// 4ED 218 — Red Elemental Blast (reprint)
const RED_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::RED_ELEMENTAL_BLAST,
    "a54246c9-5e5c-484e-a546-ae1f9fd020f3",
    "Richard Thomas",
);

// 4ED 219 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "bbd44196-aa28-4143-872d-592c6fc175a9",
    "Amy Weber",
);

// 4ED 220 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "70846483-9c23-42c4-9f9f-1fc5cda17c77",
    "Melissa A. Benson",
);

// 4ED 221 — Sisters of the Flame (reprint)
const SISTERS_OF_THE_FLAME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::SISTERS_OF_THE_FLAME,
    "a39ab53c-133a-4211-8499-aea00ed3ee1d",
    "Jesper Myrfors",
);

// 4ED 222 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SMOKE,
    "c2069f9b-578a-45da-bd52-3c208465be88",
    "Jesper Myrfors",
);

// 4ED 223 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_GIANT,
    "265de55e-63d2-4a5d-9078-858da70f2a08",
    "Dameon Willich",
);

// 4ED 224 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "b9029bfb-fcff-4f80-a423-c2cfdc881c61",
    "Daniel Gelon",
);

// 4ED 225 — Tempest Efreet (reprint)
const TEMPEST_EFREET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TEMPEST_EFREET,
    "c2ea6dfe-64d6-451a-bd34-31546996e711",
    "NéNé Thomas",
);

// 4ED 226 — The Brute (reprint)
const THE_BRUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::THE_BRUTE,
    "4b58ca62-6532-45e5-ad51-84a388d5cc4d",
    "Mark Poole",
);

// 4ED 227 — Tunnel (reprint)
const TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNNEL,
    "9483f5d0-c627-43ed-bfaa-1559855c8a6d",
    "Dan Frazier",
);

// 4ED 228 — Uthden Troll (reprint)
const UTHDEN_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UTHDEN_TROLL,
    "28dea433-2dee-4faf-a868-6af664c6af4a",
    "Douglas Shuler",
);

// 4ED 229 — Wall of Dust (reprint)
const WALL_OF_DUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WALL_OF_DUST,
    "b25b81fb-1d0f-4c0c-8d54-fbf9c1d54578",
    "Richard Thomas",
);

// 4ED 230 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "ccd7753b-0f01-42f3-9a5b-fe1ac6e7441d",
    "Richard Thomas",
);

// 4ED 231 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "8c4b68e0-ec8b-4dad-91ee-dc1da5db6251",
    "Dan Frazier",
);

// 4ED 232 — Winds of Change (reprint)
const WINDS_OF_CHANGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WINDS_OF_CHANGE,
    "77071f4c-d00b-4c79-a2e6-6be0720af36b",
    "Justin Hampton",
);

// 4ED 233 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "58e780b8-8002-47d4-9d0c-bd65a40ea34e",
    "Jeff A. Menges",
);

// 4ED 234 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BIRDS_OF_PARADISE,
    "b8852e36-204c-4b3a-a4f8-33a98548fa7b",
    "Mark Poole",
);

// 4ED 235 — Carnivorous Plant (reprint)
const CARNIVOROUS_PLANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::CARNIVOROUS_PLANT,
    "6f449835-5e20-4244-b7f4-c22838910076",
    "Quinton Hoover",
);

// 4ED 236 — Channel (reprint)
const CHANNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHANNEL,
    "e42eb95f-638d-410c-a830-a414fb2494ec",
    "Richard Thomas",
);

// 4ED 237 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "3406dfe1-68e9-4757-9487-0b556c97d07a",
    "Dan Frazier",
);

// 4ED 238 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "57e9cf07-a335-4725-a124-0e983721f2f8",
    "Daniel Gelon",
);

// 4ED 239 — Crumble (reprint)
const CRUMBLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CRUMBLE,
    "86873be9-21eb-496c-b278-4c9847563b0f",
    "Jesper Myrfors",
);

// 4ED 240 — Desert Twister (reprint)
const DESERT_TWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DESERT_TWISTER,
    "5603ce3f-3b39-4433-8ccf-44b44dc99de5",
    "Susan Van Camp",
);

// 4ED 241 — Durkwood Boars (reprint)
const DURKWOOD_BOARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DURKWOOD_BOARS,
    "e85ab895-d2b9-47f5-91f0-07b0cb43fc7c",
    "Mike Kimble",
);

// 4ED 242 — Elven Riders (reprint)
const ELVEN_RIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ELVEN_RIDERS,
    "906095f6-25c7-4f61-8513-9d75e32aab02",
    "Melissa A. Benson",
);

// 4ED 243 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "efd21750-856f-4017-a728-dcbf3f506f20",
    "Anson Maddocks",
);

// 4ED 244 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "a1959de7-e945-438c-a90a-158e21c4d5bf",
    "Jesper Myrfors",
);

// 4ED 245 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "ccef1a44-faf8-42bd-aaff-778f65d18ae9",
    "Douglas Shuler",
);

// 4ED 246 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "7d9ff903-176f-45e7-82fd-e9705c1c719f",
    "Daniel Gelon",
);

// 4ED 247 — Gaea's Liege (reprint)
const GAEA_S_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAEA_S_LIEGE,
    "4d9e1c1b-bc7c-41b8-a983-b1c60f558547",
    "Dameon Willich",
);

// 4ED 248 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "c6ce96a5-76f1-48ca-854d-a37fb6a023a0",
    "Sandra Everingham",
);

// 4ED 249 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "9dd711db-661d-4e2c-b817-0905e26ed929",
    "Sandra Everingham",
);

// 4ED 250 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "38fc72de-2093-477b-b39e-696339d2fdbc",
    "Jeff A. Menges",
);

// 4ED 251 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "8457f271-4075-4694-a1c0-280aff910953",
    "Dameon Willich",
);

// 4ED 252 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "0eeb4671-e6f6-4f64-83f8-8c2a56defa1b",
    "Dameon Willich",
);

// 4ED 253 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "c6c93c85-5263-4770-b937-704e57912478",
    "Jesper Myrfors",
);

// 4ED 254 — Killer Bees (reprint)
const KILLER_BEES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KILLER_BEES,
    "e4e748b0-4041-4266-a777-e1b8a5533e80",
    "Phil Foglio",
);

// 4ED 255 — Land Leeches (reprint)
const LAND_LEECHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::LAND_LEECHES,
    "71f1d97c-5bfa-4791-9004-5f2464908c30",
    "Quinton Hoover",
);

// 4ED 256 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "925adfb2-cfe3-4847-9db0-20bbe4e7baf1",
    "Sandra Everingham",
);

// 4ED 257 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "e1ca1cc3-e588-4ef7-b846-d5dba3c875a0",
    "Dameon Willich",
);

// 4ED 258 — Lifelace (reprint)
const LIFELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFELACE,
    "bf312acb-2fa6-440a-964b-424ad8abc331",
    "Amy Weber",
);

// 4ED 259 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "2cbe717c-313c-4a8c-85a5-d23d3796ff26",
    "Anson Maddocks",
);

// 4ED 260 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "36d5c0df-3db4-4e2d-9688-223383e02b02",
    "Jesper Myrfors",
);

// 4ED 261 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LLANOWAR_ELVES,
    "75d972d7-5ed9-49c1-8d27-ec162771284d",
    "Anson Maddocks",
);

// 4ED 262 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "7abc17aa-8f6b-4156-b148-fc049e1d316a",
    "Anson Maddocks",
);

// 4ED 263 — Marsh Viper (reprint)
const MARSH_VIPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::MARSH_VIPER,
    "16524bd2-0d88-451c-a394-7d5fe204dfc6",
    "Ron Spencer",
);

// 4ED 264 — Nafs Asp (reprint)
const NAFS_ASP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::NAFS_ASP,
    "0db4a4ef-20a8-415f-8d7d-a6740b482f73",
    "Christopher Rush",
);

// 4ED 265 — Pradesh Gypsies (reprint)
const PRADESH_GYPSIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PRADESH_GYPSIES,
    "2338aeec-63f6-4cc1-833e-77d44994a7ca",
    "Quinton Hoover",
);

// 4ED 266 — Radjan Spirit (reprint)
const RADJAN_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RADJAN_SPIRIT,
    "504e4e18-a70c-47d6-8331-2ca3c6210a98",
    "Christopher Rush",
);

// 4ED 267 — Rebirth (reprint)
const REBIRTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::REBIRTH,
    "27c01d57-2bd4-433e-bea7-1acc70d14be3",
    "Mark Tedin",
);

// 4ED 268 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "5697b2e7-25c5-4ae0-a9ac-48cbc6a94c15",
    "Quinton Hoover",
);

// 4ED 269 — Sandstorm (reprint)
const SANDSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SANDSTORM,
    "ca9af432-3997-426a-b532-52333c3c50c4",
    "Brian Snõddy",
);

// 4ED 270 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCRYB_SPRITES,
    "281e8da8-1383-460e-b74e-ad56f1b6d007",
    "Amy Weber",
);

// 4ED 271 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "8d8f8c01-6f1c-4902-a7fd-c9cf04b28461",
    "Anson Maddocks",
);

// 4ED 272 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "ca367627-9be3-4e80-a214-0b3f0f2ab867",
    "Mark Poole",
);

// 4ED 273 — Sylvan Library (reprint)
const SYLVAN_LIBRARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SYLVAN_LIBRARY,
    "b1a9682c-ecca-4caa-9e5a-17874167082b",
    "Harold McNeill",
);

// 4ED 274 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "c63c16a5-588e-4ef1-9e79-4d36127cd84b",
    "Dan Frazier",
);

// 4ED 275 — Timber Wolves (reprint)
const TIMBER_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TIMBER_WOLVES,
    "d8f84fc8-69b4-4756-9634-4d6c17ec88a1",
    "Melissa A. Benson",
);

// 4ED 276 — Titania's Song (reprint)
const TITANIA_S_SONG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TITANIA_S_SONG,
    "19dd44e3-d62c-405d-a620-7dc871eef81c",
    "Kerstin Kaman",
);

// 4ED 277 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "b1c8b995-f3d9-435a-8360-4047a619b23b",
    "Douglas Shuler",
);

// 4ED 278 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "227c2abc-d484-4198-863c-3266a83249c6",
    "Richard Thomas",
);

// 4ED 279 — Untamed Wilds (reprint)
const UNTAMED_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::UNTAMED_WILDS,
    "de2d276d-7a73-4a8f-9803-a37301bc2905",
    "NéNé Thomas",
);

// 4ED 280 — Venom (reprint)
const VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::VENOM,
    "c7a314a0-14cb-4df1-8f2f-653455f13b09",
    "Tom Wänerstrand",
);

// 4ED 281 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "96e30d22-da86-49a1-a319-22e8a909d443",
    "Kev Brockschmidt",
);

// 4ED 282 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "bbd66c7c-19fb-4daa-996e-70107d732732",
    "Anson Maddocks",
);

// 4ED 283 — Wall of Ice (reprint)
const WALL_OF_ICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_ICE,
    "d702bd22-6079-4f4c-9540-42cf2a29f4a3",
    "Richard Thomas",
);

// 4ED 284 — Wall of Wood (reprint)
const WALL_OF_WOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WOOD,
    "39f236da-8391-487b-88d4-a45342bfff62",
    "Mark Tedin",
);

// 4ED 285 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "f4144ca4-4817-4bb9-929b-613fc609bdb5",
    "Cornelius Brudi",
);

// 4ED 286 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "2137757c-161c-4a3b-9a99-0fd23aa0c847",
    "Jeff A. Menges",
);

// 4ED 287 — Web (reprint)
const WEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEB,
    "0547a390-ceee-4b8a-9d0e-36d8778ec693",
    "Rob Alexander",
);

// 4ED 288 — Whirling Dervish (reprint)
const WHIRLING_DERVISH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WHIRLING_DERVISH,
    "b0f4724e-f22e-4156-97fd-1adfccc47be3",
    "Susan Van Camp",
);

// 4ED 289 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "4aea57a2-2753-4014-9724-2701455a6be8",
    "Mark Poole",
);

// 4ED 290 — Winter Blast (reprint)
const WINTER_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WINTER_BLAST,
    "3d4f72de-51e9-45c2-853b-1b6668416417",
    "Kaja Foglio",
);

// 4ED 291 — Aladdin's Lamp (reprint)
const ALADDIN_S_LAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDIN_S_LAMP,
    "42e7cf40-c136-4fcb-a947-558b713b39f6",
    "Mark Tedin",
);

// 4ED 292 — Aladdin's Ring (reprint)
const ALADDINS_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDINS_RING,
    "d9907bbb-12aa-4826-9a65-b2ddda5fc1e2",
    "Dan Frazier",
);

// 4ED 293 — Amulet of Kroog (reprint)
const AMULET_OF_KROOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::AMULET_OF_KROOG,
    "3fc1716d-5817-49f4-a9db-c162e6ceacbf",
    "Margaret Organ-Kean",
);

// 4ED 294 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANKH_OF_MISHRA,
    "44135b17-69e7-4002-a6e0-d76f4c0c423b",
    "Amy Weber",
);

// 4ED 295 — Armageddon Clock (reprint)
const ARMAGEDDON_CLOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ARMAGEDDON_CLOCK,
    "2fa3fe10-0f6e-4b7f-98fc-3a21c399c38a",
    "Amy Weber",
);

// 4ED 296 — Ashnod's Battle Gear (reprint)
const ASHNODS_BATTLE_GEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ASHNODS_BATTLE_GEAR,
    "0bc11285-0891-4cc3-a056-b698911166c7",
    "Mark Poole",
);

// 4ED 297 — Battering Ram (reprint)
const BATTERING_RAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::BATTERING_RAM,
    "ebf5ea1c-0ea7-4b84-b886-ebd346f4e154",
    "Jeff A. Menges",
);

// 4ED 298 — Black Mana Battery (reprint)
const BLACK_MANA_BATTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLACK_MANA_BATTERY,
    "a81f7a0f-183f-438b-b252-738d8d30c245",
    "Anson Maddocks",
);

// 4ED 299 — Black Vise (reprint)
const BLACK_VISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_VISE,
    "196b83a1-2a25-498d-83b8-8faacd79909d",
    "Richard Thomas",
);

// 4ED 300 — Blue Mana Battery (reprint)
const BLUE_MANA_BATTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLUE_MANA_BATTERY,
    "cd2cb84e-c079-486e-87ad-d188fe38bc76",
    "Amy Weber",
);

// 4ED 301 — Bottle of Suleiman (reprint)
const BOTTLE_OF_SULEIMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BOTTLE_OF_SULEIMAN,
    "d1924860-4532-40c2-b0e8-28584d22ccb5",
    "Jesper Myrfors",
);

// 4ED 302 — Brass Man (reprint)
const BRASS_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BRASS_MAN,
    "7ab18c6d-2705-473e-b404-75660ff28736",
    "Christopher Rush",
);

// 4ED 303 — Bronze Tablet (reprint)
const BRONZE_TABLET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::BRONZE_TABLET,
    "cad56033-c1f9-4477-9dd6-ba7009c30593",
    "Tom Wänerstrand",
);

// 4ED 304 — Celestial Prism (reprint)
const CELESTIAL_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CELESTIAL_PRISM,
    "e7ca1534-0049-4672-9677-99f1ba00fb78",
    "Amy Weber",
);

// 4ED 305 — Clay Statue (reprint)
const CLAY_STATUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CLAY_STATUE,
    "0dd9b203-fa4c-4383-a671-265101f4453a",
    "Jesper Myrfors",
);

// 4ED 306 — Clockwork Avian (reprint)
const CLOCKWORK_AVIAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CLOCKWORK_AVIAN,
    "4a92484b-064c-4588-a1ea-6de8fd485ca4",
    "Randy Asplund-Faith",
);

// 4ED 307 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "40dc8595-32b9-4045-bae3-5078c9a17527",
    "Drew Tucker",
);

// 4ED 308 — Colossus of Sardia (reprint)
const COLOSSUS_OF_SARDIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::COLOSSUS_OF_SARDIA,
    "063acc0f-8062-4461-b0f5-8c3a835e1fbf",
    "Jesper Myrfors",
);

// 4ED 309 — Conservator (reprint)
const CONSERVATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSERVATOR,
    "f6639eea-d0ae-4f0d-a8da-1b863b482b68",
    "Amy Weber",
);

// 4ED 310 — Coral Helm (reprint)
const CORAL_HELM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CORAL_HELM,
    "6065242b-e14e-44fa-bbe0-00f070f0140a",
    "Amy Weber",
);

// 4ED 311 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "c05c17c7-49c2-41f2-842e-d980e7d62613",
    "Amy Weber",
);

// 4ED 312 — Cursed Rack (reprint)
const CURSED_RACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CURSED_RACK,
    "28cbcab8-5593-4ef0-8689-77e71fb1c41a",
    "Richard Thomas",
);

// 4ED 313 — Dancing Scimitar (reprint)
const DANCING_SCIMITAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DANCING_SCIMITAR,
    "c0662881-664f-47d2-8164-b4727125ba0b",
    "Anson Maddocks",
);

// 4ED 314 — Diabolic Machine (reprint)
const DIABOLIC_MACHINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::DIABOLIC_MACHINE,
    "23d2c7c2-5696-49a0-b6b0-789bfe839f8d",
    "Anson Maddocks",
);

// 4ED 315 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "b9c44bba-5eaa-41a6-a11a-0bc8fb751ad8",
    "Dan Frazier",
);

// 4ED 316 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "fb9b9085-6dd7-44fc-b3a5-7f797c34f8dc",
    "Dan Frazier",
);

// 4ED 317 — Dragon Engine (reprint)
const DRAGON_ENGINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DRAGON_ENGINE,
    "80ecbbb3-2d0d-49dc-90b3-1b396d47bf56",
    "Anson Maddocks",
);

// 4ED 318 — Ebony Horse (reprint)
const EBONY_HORSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EBONY_HORSE,
    "b665f9dc-a5df-4ad7-9067-9b48f942bdde",
    "Dameon Willich",
);

// 4ED 319 — Fellwar Stone (reprint)
const FELLWAR_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FELLWAR_STONE,
    "6f7a41cc-277a-4d95-a30c-91bf5aa7ac11",
    "Quinton Hoover",
);

// 4ED 320 — Flying Carpet (reprint)
const FLYING_CARPET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::FLYING_CARPET,
    "300428ea-7909-4c2b-81cc-a03d51030bcb",
    "Mark Tedin",
);

// 4ED 321 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLASSES_OF_URZA,
    "4d71a210-eb2c-4bcc-ae3b-f60a6fb615cb",
    "Douglas Shuler",
);

// 4ED 322 — Grapeshot Catapult (reprint)
const GRAPESHOT_CATAPULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::GRAPESHOT_CATAPULT,
    "d0f96f18-8340-4b50-9b4c-3c72c0bbc2f2",
    "Dan Frazier",
);

// 4ED 323 — Green Mana Battery (reprint)
const GREEN_MANA_BATTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GREEN_MANA_BATTERY,
    "d0a6e224-72df-4f1f-93d1-5114779aed2c",
    "Christopher Rush",
);

// 4ED 324 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "673c0a08-1aaf-4976-8eea-c93a9f9486fa",
    "Mark Tedin",
);

// 4ED 325 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "a4df1be4-364e-4582-929a-05f2905f8ce6",
    "Mark Poole",
);

// 4ED 326 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRON_STAR,
    "b8846e66-fd42-4e74-80fe-858944108d40",
    "Dan Frazier",
);

// 4ED 327 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "14f378af-ea54-4035-9e3e-e8cf980d1e84",
    "Anson Maddocks",
);

// 4ED 328 — Ivory Tower (reprint)
const IVORY_TOWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::IVORY_TOWER,
    "eff5624d-fffa-48c1-91f3-f03585e45c69",
    "Margaret Organ-Kean",
);

// 4ED 329 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "044e5475-4ab5-45c5-b86d-9b4720e7ba0c",
    "Anson Maddocks",
);

// 4ED 330 — Jandor's Saddlebags (reprint)
const JANDORS_SADDLEBAGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JANDORS_SADDLEBAGS,
    "ac15b58c-80b4-4da7-8ed6-d963702eda3a",
    "Dameon Willich",
);

// 4ED 331 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JAYEMDAE_TOME,
    "4009fceb-9140-457b-b980-875f6a2f70fd",
    "Mark Tedin",
);

// 4ED 332 — Kormus Bell (reprint)
const KORMUS_BELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KORMUS_BELL,
    "3bd054ed-9c91-4bc6-9d0d-0c045c089bd9",
    "Christopher Rush",
);

// 4ED 333 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "b811a515-87a2-4dee-8689-48bfba12e6c5",
    "Daniel Gelon",
);

// 4ED 334 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_VAULT,
    "3be9942c-89b3-422f-bb9d-b55f51a22a37",
    "Mark Tedin",
);

// 4ED 335 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "c9714ce0-7fc3-4bf3-ad65-68555a9d2f35",
    "Quinton Hoover",
);

// 4ED 336 — Millstone (reprint)
const MILLSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MILLSTONE,
    "f16f7d28-1bc2-44b6-973b-c60d966101a6",
    "Kaja Foglio",
);

// 4ED 337 — Mishra's War Machine (reprint)
const MISHRA_S_WAR_MACHINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MISHRA_S_WAR_MACHINE,
    "0f7b5921-3e10-47e7-98a7-8411a18313bf",
    "Amy Weber",
);

// 4ED 338 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NEVINYRRALS_DISK,
    "d427396c-f1ef-46ac-b130-ed1e51e826a3",
    "Mark Tedin",
);

// 4ED 339 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "837605e3-f4c2-4b60-8478-f21ca8734ef2",
    "Jesper Myrfors",
);

// 4ED 340 — Onulet (reprint)
const ONULET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ONULET,
    "c64c2dff-9c11-4c91-a606-b5de704f18d4",
    "Anson Maddocks",
);

// 4ED 341 — Ornithopter (reprint)
const ORNITHOPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ORNITHOPTER,
    "2aeb3ce1-1e7f-4269-a326-700f27e9e932",
    "Amy Weber",
);

// 4ED 342 — Primal Clay (reprint)
const PRIMAL_CLAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::PRIMAL_CLAY,
    "cc49d724-b46d-449b-bd2b-03551e130a06",
    "Kaja Foglio",
);

// 4ED 343 — Red Mana Battery (reprint)
const RED_MANA_BATTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RED_MANA_BATTERY,
    "e4e507bc-441d-4f44-85d4-cf93ca199d2e",
    "Mark Tedin",
);

// 4ED 344 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "88bf9da4-4647-4c66-a3ce-fab7ca9618d1",
    "Christopher Rush",
);

// 4ED 345 — Shapeshifter (reprint)
const SHAPESHIFTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::SHAPESHIFTER,
    "0b9d1526-9888-41bc-b77d-88d62325e0b2",
    "Dan Frazier",
);

// 4ED 346 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "8ccacae0-f8db-45e0-b25a-35418fd24389",
    "Dameon Willich",
);

// 4ED 347 — Sunglasses of Urza (reprint)
const SUNGLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SUNGLASSES_OF_URZA,
    "6a225462-947c-49d7-81b2-91f875664dca",
    "Dan Frazier",
);

// 4ED 348 — Tawnos's Wand (reprint)
const TAWNOSS_WAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TAWNOSS_WAND,
    "d566993f-18a6-4e5c-abc3-0fe9a03e97d2",
    "Douglas Shuler",
);

// 4ED 349 — Tawnos's Weaponry (reprint)
const TAWNOSS_WEAPONRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TAWNOSS_WEAPONRY,
    "4750787e-9fef-4bdf-b2e3-e410c84999f2",
    "Dan Frazier",
);

// 4ED 350 — Tetravus (reprint)
const TETRAVUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TETRAVUS,
    "49c1a2b2-50f0-4ed0-bd8f-06cd6aada04f",
    "Mark Tedin",
);

// 4ED 351 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "f9d01a2e-2687-4b37-aed6-63202cd81231",
    "Sandra Everingham",
);

// 4ED 352 — The Rack (reprint)
const THE_RACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::THE_RACK,
    "1e19104f-55d5-40e5-a61d-ddba2cf5a527",
    "Richard Thomas",
);

// 4ED 353 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "98e77be1-3c29-40b4-9c47-44fa2d9d4454",
    "Anson Maddocks",
);

// 4ED 354 — Triskelion (reprint)
const TRISKELION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TRISKELION,
    "09294401-a895-4084-8302-196a946863d6",
    "Douglas Shuler",
);

// 4ED 355 — Urza's Avenger (reprint)
const URZA_S_AVENGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::URZA_S_AVENGER,
    "97dd1daa-0b5d-4d0c-9a67-a59083038f2d",
    "Amy Weber",
);

// 4ED 356 — Wall of Spears (reprint)
const WALL_OF_SPEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::WALL_OF_SPEARS,
    "ad580bfc-8827-4a8a-a5ec-b6195b3146bb",
    "Sandra Everingham",
);

// 4ED 357 — White Mana Battery (reprint)
const WHITE_MANA_BATTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::WHITE_MANA_BATTERY,
    "b622e694-858d-482f-a67d-0e52d268708c",
    "Anthony S. Waters",
);

// 4ED 358 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WINTER_ORB,
    "d60ee90d-5f7b-4294-8d52-9744fada8d36",
    "Mark Tedin",
);

// 4ED 359 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "6a51abf0-8661-4776-929d-35b7bd345e21",
    "Mark Tedin",
);

// 4ED 360 — Yotian Soldier (reprint)
const YOTIAN_SOLDIER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::YOTIAN_SOLDIER,
    "aaab84be-bb96-437b-a9a7-aa7a11ffd21d",
    "Christopher Rush",
);

// 4ED 361 — Mishra's Factory (reprint)
const MISHRA_S_FACTORY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &antiquities::MISHRA_S_FACTORY,
    "aff8d4f1-eaad-4afb-9097-2afab133f707",
    "Kaja Foglio & Phil Foglio",
);

// 4ED 362 — Oasis (reprint)
const OASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::OASIS,
    "1d51a6de-7bb4-4e3d-82a7-a298c8d742ef",
    "Brian Snõddy",
);

// 4ED 363 — Strip Mine (reprint)
const STRIP_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::STRIP_MINE,
    "a5899b46-226b-4be6-8e80-d2396f54210d",
    "Daniel Gelon",
);

// 4ED 364 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "639e47dc-c90f-4f55-9b3a-721240ec04ed",
    "Jesper Myrfors",
);

// 4ED 365 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "d17f8abd-c087-4039-8dfd-c6168f7db0a6",
    "Jesper Myrfors",
);

// 4ED 366 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "ad1317da-9e81-4aff-8c04-9155d14be90c",
    "Jesper Myrfors",
);

// 4ED 367 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "b06bbd6e-eb0d-45fc-88ef-0e085d6505ef",
    "Mark Poole",
);

// 4ED 368 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "6e5e3819-3d75-40d4-9a93-1147834dfd69",
    "Mark Poole",
);

// 4ED 369 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "33a05f9a-1285-4d14-b827-8b81968e09df",
    "Mark Poole",
);

// 4ED 370 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "84634023-5c94-4dcc-9449-abf73ecea542",
    "Dan Frazier",
);

// 4ED 371 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "ddaa0be1-7358-4ea2-8c40-be6d699a6631",
    "Dan Frazier",
);

// 4ED 372 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "7ac4979a-5b2f-4db1-b665-9d8ccc15ba82",
    "Dan Frazier",
);

// 4ED 373 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "0c5c9379-b686-4823-b85a-eaf2c4b63205",
    "Douglas Shuler",
);

// 4ED 374 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "10478e22-d1dd-4e02-81a7-d93ce71ed81d",
    "Douglas Shuler",
);

// 4ED 375 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "50352268-88a6-4575-a5e1-cd8bef7f8286",
    "Douglas Shuler",
);

// 4ED 376 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "119c4d73-5b71-446a-a739-25d494591aa1",
    "Christopher Rush",
);

// 4ED 377 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "ddb35995-0298-4281-88d1-2531c93a4916",
    "Christopher Rush",
);

// 4ED 378 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "b794073f-4188-45c9-9e65-c9d7f2ecc24b",
    "Christopher Rush",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ALABASTER_POTION_REPRINT,
    AMROU_KITHKIN_REPRINT,
    ANGRY_MOB_REPRINT,
    ANIMATE_WALL_REPRINT,
    ARMAGEDDON_REPRINT,
    BALANCE_REPRINT,
    BENALISH_HERO_REPRINT,
    BLACK_WARD_REPRINT,
    BLESSING_REPRINT,
    BLUE_WARD_REPRINT,
    BRAINWASH_REPRINT,
    CASTLE_REPRINT,
    CIRCLE_OF_PROTECTION_ARTIFACTS_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    CONVERSION_REPRINT,
    CRUSADE_REPRINT,
    DEATH_WARD_REPRINT,
    DISENCHANT_REPRINT,
    DIVINE_TRANSFORMATION_REPRINT,
    ELDER_LAND_WURM_REPRINT,
    EYE_FOR_AN_EYE_REPRINT,
    FORTIFIED_AREA_REPRINT,
    GREEN_WARD_REPRINT,
    HEALING_SALVE_REPRINT,
    HOLY_ARMOR_REPRINT,
    HOLY_STRENGTH_REPRINT,
    ISLAND_SANCTUARY_REPRINT,
    KARMA_REPRINT,
    KISMET_REPRINT,
    LAND_TAX_REPRINT,
    MESA_PEGASUS_REPRINT,
    MORALE_REPRINT,
    NORTHERN_PALADIN_REPRINT,
    OSAI_VULTURES_REPRINT,
    PEARLED_UNICORN_REPRINT,
    PERSONAL_INCARNATION_REPRINT,
    PIETY_REPRINT,
    PIKEMEN_REPRINT,
    PURELACE_REPRINT,
    RED_WARD_REPRINT,
    REVERSE_DAMAGE_REPRINT,
    RIGHTEOUSNESS_REPRINT,
    SAMITE_HEALER_REPRINT,
    SAVANNAH_LIONS_REPRINT,
    SEEKER_REPRINT,
    SERRA_ANGEL_REPRINT,
    SPIRIT_LINK_REPRINT,
    SWORDS_TO_PLOWSHARES_REPRINT,
    TUNDRA_WOLVES_REPRINT,
    VISIONS_REPRINT,
    WALL_OF_SWORDS_REPRINT,
    WHITE_KNIGHT_REPRINT,
    WHITE_WARD_REPRINT,
    WRATH_OF_GOD_REPRINT,
    AIR_ELEMENTAL_REPRINT,
    ANIMATE_ARTIFACT_REPRINT,
    APPRENTICE_WIZARD_REPRINT,
    BACKFIRE_REPRINT,
    BLUE_ELEMENTAL_BLAST_REPRINT,
    CONTROL_MAGIC_REPRINT,
    COUNTERSPELL_REPRINT,
    CREATURE_BOND_REPRINT,
    DRAIN_POWER_REPRINT,
    ENERGY_FLUX_REPRINT,
    ENERGY_TAP_REPRINT,
    EROSION_REPRINT,
    FEEDBACK_REPRINT,
    FLIGHT_REPRINT,
    FLOOD_REPRINT,
    GASEOUS_FORM_REPRINT,
    GHOST_SHIP_REPRINT,
    GIANT_TORTOISE_REPRINT,
    HURKYLS_RECALL_REPRINT,
    ISLAND_FISH_JASCONIUS_REPRINT,
    JUMP_REPRINT,
    LEVIATHAN_REPRINT,
    LIFETAP_REPRINT,
    LORD_OF_ATLANTIS_REPRINT,
    MAGICAL_HACK_REPRINT,
    MAHAMOTI_DJINN_REPRINT,
    MANA_SHORT_REPRINT,
    MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT,
    MIND_BOMB_REPRINT,
    PHANTASMAL_FORCES_REPRINT,
    PHANTASMAL_TERRAIN_REPRINT,
    PHANTOM_MONSTER_REPRINT,
    PIRATE_SHIP_REPRINT,
    POWER_LEAK_REPRINT,
    POWER_SINK_REPRINT,
    PRODIGAL_SORCERER_REPRINT,
    PSIONIC_ENTITY_REPRINT,
    PSYCHIC_VENOM_REPRINT,
    RELIC_BIND_REPRINT,
    SEA_SERPENT_REPRINT,
    SEGOVIAN_LEVIATHAN_REPRINT,
    SINDBAD_REPRINT,
    SIREN_S_CALL_REPRINT,
    SLEIGHT_OF_MIND_REPRINT,
    SPELL_BLAST_REPRINT,
    STASIS_REPRINT,
    STEAL_ARTIFACT_REPRINT,
    SUNKEN_CITY_REPRINT,
    THOUGHTLACE_REPRINT,
    DRUDGE_SKELETONS_REPRINT,
    TIME_ELEMENTAL_REPRINT,
    TWIDDLE_REPRINT,
    UNSTABLE_MUTATION_REPRINT,
    UNSUMMON_REPRINT,
    VOLCANIC_ERUPTION_REPRINT,
    WALL_OF_AIR_REPRINT,
    WALL_OF_WATER_REPRINT,
    WATER_ELEMENTAL_REPRINT,
    ZEPHYR_FALCON_REPRINT,
    ABOMINATION_REPRINT,
    ANIMATE_DEAD_REPRINT,
    ASHES_TO_ASHES_REPRINT,
    BAD_MOON_REPRINT,
    BLACK_KNIGHT_REPRINT,
    BLIGHT_REPRINT,
    BOG_IMP_REPRINT,
    BOG_WRAITH_REPRINT,
    CARRION_ANTS_REPRINT,
    COSMIC_HORROR_REPRINT,
    CURSED_LAND_REPRINT,
    CYCLOPEAN_MUMMY_REPRINT,
    DARK_RITUAL_REPRINT,
    DEATHGRIP_REPRINT,
    DEATHLACE_REPRINT,
    DRAIN_LIFE_REPRINT,
    DRUDGE_SKELETONS_ALTERNATE_1,
    EL_HAJJAJ_REPRINT,
    EL_HAJJAJ_ALTERNATE_1,
    ERG_RAIDERS_REPRINT,
    EVIL_PRESENCE_REPRINT,
    FEAR_REPRINT,
    FROZEN_SHADE_REPRINT,
    GLOOM_REPRINT,
    GREED_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    HYPNOTIC_SPECTER_REPRINT,
    JUNUN_EFREET_REPRINT,
    LORD_OF_THE_PIT_REPRINT,
    LOST_SOUL_REPRINT,
    MARSH_GAS_REPRINT,
    MIND_TWIST_REPRINT,
    MURK_DWELLERS_REPRINT,
    NETHER_SHADOW_REPRINT,
    NIGHTMARE_REPRINT,
    PARALYZE_REPRINT,
    PESTILENCE_REPRINT,
    PIT_SCORPION_REPRINT,
    PLAGUE_RATS_REPRINT,
    RAG_MAN_REPRINT,
    RAISE_DEAD_REPRINT,
    ROYAL_ASSASSIN_REPRINT,
    SCATHE_ZOMBIES_REPRINT,
    SCAVENGING_GHOUL_REPRINT,
    SENGIR_VAMPIRE_REPRINT,
    SIMULACRUM_REPRINT,
    SORCERESS_QUEEN_REPRINT,
    SPIRIT_SHACKLE_REPRINT,
    TERROR_REPRINT,
    UNCLE_ISTVAN_REPRINT,
    UNHOLY_STRENGTH_REPRINT,
    VAMPIRE_BATS_REPRINT,
    WALL_OF_BONE_REPRINT,
    WARP_ARTIFACT_REPRINT,
    WEAKNESS_REPRINT,
    WILL_O_THE_WISP_REPRINT,
    WORD_OF_BINDING_REPRINT,
    XENIC_POLTERGEIST_REPRINT,
    ZOMBIE_MASTER_REPRINT,
    ALI_BABA_REPRINT,
    BALL_LIGHTNING_REPRINT,
    BIRD_MAIDEN_REPRINT,
    BLOOD_LUST_REPRINT,
    BROTHERS_OF_FIRE_REPRINT,
    BURROWING_REPRINT,
    CAVE_PEOPLE_REPRINT,
    CHAOSLACE_REPRINT,
    CRIMSON_MANTICORE_REPRINT,
    DETONATE_REPRINT,
    DISINTEGRATE_REPRINT,
    DRAGON_WHELP_REPRINT,
    DWARVEN_WARRIORS_REPRINT,
    EARTH_ELEMENTAL_REPRINT,
    EARTHQUAKE_REPRINT,
    ETERNAL_WARRIOR_REPRINT,
    FIRE_ELEMENTAL_REPRINT,
    FIREBALL_REPRINT,
    FIREBREATHING_REPRINT,
    FISSURE_REPRINT,
    FLASHFIRES_REPRINT,
    GIANT_STRENGTH_REPRINT,
    GOBLIN_BALLOON_BRIGADE_REPRINT,
    GOBLIN_KING_REPRINT,
    GOBLIN_ROCK_SLED_REPRINT,
    GRAY_OGRE_REPRINT,
    HILL_GIANT_REPRINT,
    HURLOON_MINOTAUR_REPRINT,
    HURR_JACKAL_REPRINT,
    IMMOLATION_REPRINT,
    INFERNO_REPRINT,
    IRONCLAW_ORCS_REPRINT,
    KELDON_WARLORD_REPRINT,
    LIGHTNING_BOLT_REPRINT,
    MAGNETIC_MOUNTAIN_REPRINT,
    MANA_CLASH_REPRINT,
    MANA_FLARE_REPRINT,
    MANABARBS_REPRINT,
    MONSS_GOBLIN_RAIDERS_REPRINT,
    ORCISH_ARTILLERY_REPRINT,
    ORCISH_ORIFLAMME_REPRINT,
    POWER_SURGE_REPRINT,
    PYROTECHNICS_REPRINT,
    RED_ELEMENTAL_BLAST_REPRINT,
    SHATTER_REPRINT,
    SHIVAN_DRAGON_REPRINT,
    SISTERS_OF_THE_FLAME_REPRINT,
    SMOKE_REPRINT,
    STONE_GIANT_REPRINT,
    STONE_RAIN_REPRINT,
    TEMPEST_EFREET_REPRINT,
    THE_BRUTE_REPRINT,
    TUNNEL_REPRINT,
    UTHDEN_TROLL_REPRINT,
    WALL_OF_DUST_REPRINT,
    WALL_OF_FIRE_REPRINT,
    WALL_OF_STONE_REPRINT,
    WINDS_OF_CHANGE_REPRINT,
    ASPECT_OF_WOLF_REPRINT,
    BIRDS_OF_PARADISE_REPRINT,
    CARNIVOROUS_PLANT_REPRINT,
    CHANNEL_REPRINT,
    COCKATRICE_REPRINT,
    CRAW_WURM_REPRINT,
    CRUMBLE_REPRINT,
    DESERT_TWISTER_REPRINT,
    DURKWOOD_BOARS_REPRINT,
    ELVEN_RIDERS_REPRINT,
    ELVISH_ARCHERS_REPRINT,
    FOG_REPRINT,
    FORCE_OF_NATURE_REPRINT,
    FUNGUSAUR_REPRINT,
    GAEA_S_LIEGE_REPRINT,
    GIANT_GROWTH_REPRINT,
    GIANT_SPIDER_REPRINT,
    GRIZZLY_BEARS_REPRINT,
    HURRICANE_REPRINT,
    INSTILL_ENERGY_REPRINT,
    IRONROOT_TREEFOLK_REPRINT,
    KILLER_BEES_REPRINT,
    LAND_LEECHES_REPRINT,
    LEY_DRUID_REPRINT,
    LIFEFORCE_REPRINT,
    LIFELACE_REPRINT,
    LIVING_ARTIFACT_REPRINT,
    LIVING_LANDS_REPRINT,
    LLANOWAR_ELVES_REPRINT,
    LURE_REPRINT,
    MARSH_VIPER_REPRINT,
    NAFS_ASP_REPRINT,
    PRADESH_GYPSIES_REPRINT,
    RADJAN_SPIRIT_REPRINT,
    REBIRTH_REPRINT,
    REGENERATION_REPRINT,
    SANDSTORM_REPRINT,
    SCRYB_SPRITES_REPRINT,
    SHANODIN_DRYADS_REPRINT,
    STREAM_OF_LIFE_REPRINT,
    SYLVAN_LIBRARY_REPRINT,
    THICKET_BASILISK_REPRINT,
    TIMBER_WOLVES_REPRINT,
    TITANIA_S_SONG_REPRINT,
    TRANQUILITY_REPRINT,
    TSUNAMI_REPRINT,
    UNTAMED_WILDS_REPRINT,
    VENOM_REPRINT,
    VERDURAN_ENCHANTRESS_REPRINT,
    WALL_OF_BRAMBLES_REPRINT,
    WALL_OF_ICE_REPRINT,
    WALL_OF_WOOD_REPRINT,
    WANDERLUST_REPRINT,
    WAR_MAMMOTH_REPRINT,
    WEB_REPRINT,
    WHIRLING_DERVISH_REPRINT,
    WILD_GROWTH_REPRINT,
    WINTER_BLAST_REPRINT,
    ALADDIN_S_LAMP_REPRINT,
    ALADDINS_RING_REPRINT,
    AMULET_OF_KROOG_REPRINT,
    ANKH_OF_MISHRA_REPRINT,
    ARMAGEDDON_CLOCK_REPRINT,
    ASHNODS_BATTLE_GEAR_REPRINT,
    BATTERING_RAM_REPRINT,
    BLACK_MANA_BATTERY_REPRINT,
    BLACK_VISE_REPRINT,
    BLUE_MANA_BATTERY_REPRINT,
    BOTTLE_OF_SULEIMAN_REPRINT,
    BRASS_MAN_REPRINT,
    BRONZE_TABLET_REPRINT,
    CELESTIAL_PRISM_REPRINT,
    CLAY_STATUE_REPRINT,
    CLOCKWORK_AVIAN_REPRINT,
    CLOCKWORK_BEAST_REPRINT,
    COLOSSUS_OF_SARDIA_REPRINT,
    CONSERVATOR_REPRINT,
    CORAL_HELM_REPRINT,
    CRYSTAL_ROD_REPRINT,
    CURSED_RACK_REPRINT,
    DANCING_SCIMITAR_REPRINT,
    DIABOLIC_MACHINE_REPRINT,
    DINGUS_EGG_REPRINT,
    DISRUPTING_SCEPTER_REPRINT,
    DRAGON_ENGINE_REPRINT,
    EBONY_HORSE_REPRINT,
    FELLWAR_STONE_REPRINT,
    FLYING_CARPET_REPRINT,
    GLASSES_OF_URZA_REPRINT,
    GRAPESHOT_CATAPULT_REPRINT,
    GREEN_MANA_BATTERY_REPRINT,
    HELM_OF_CHATZUK_REPRINT,
    HOWLING_MINE_REPRINT,
    IRON_STAR_REPRINT,
    IVORY_CUP_REPRINT,
    IVORY_TOWER_REPRINT,
    JADE_MONOLITH_REPRINT,
    JANDORS_SADDLEBAGS_REPRINT,
    JAYEMDAE_TOME_REPRINT,
    KORMUS_BELL_REPRINT,
    LIBRARY_OF_LENG_REPRINT,
    MANA_VAULT_REPRINT,
    MEEKSTONE_REPRINT,
    MILLSTONE_REPRINT,
    MISHRA_S_WAR_MACHINE_REPRINT,
    NEVINYRRALS_DISK_REPRINT,
    OBSIANUS_GOLEM_REPRINT,
    ONULET_REPRINT,
    ORNITHOPTER_REPRINT,
    PRIMAL_CLAY_REPRINT,
    RED_MANA_BATTERY_REPRINT,
    ROD_OF_RUIN_REPRINT,
    SHAPESHIFTER_REPRINT,
    SOUL_NET_REPRINT,
    SUNGLASSES_OF_URZA_REPRINT,
    TAWNOSS_WAND_REPRINT,
    TAWNOSS_WEAPONRY_REPRINT,
    TETRAVUS_REPRINT,
    THE_HIVE_REPRINT,
    THE_RACK_REPRINT,
    THRONE_OF_BONE_REPRINT,
    TRISKELION_REPRINT,
    URZA_S_AVENGER_REPRINT,
    WALL_OF_SPEARS_REPRINT,
    WHITE_MANA_BATTERY_REPRINT,
    WINTER_ORB_REPRINT,
    WOODEN_SPHERE_REPRINT,
    YOTIAN_SOLDIER_REPRINT,
    MISHRA_S_FACTORY_REPRINT,
    OASIS_REPRINT,
    STRIP_MINE_REPRINT,
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
