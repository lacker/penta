//! Classic Sixth Edition currently contributes no catalog definitions.

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
use crate::card::sets::y1996::alliances as catalog_all;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::sets::y1998::exodus as catalog_exo;

// 6ED 1 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "ecdb3a14-c5cc-4655-9b0c-e8be153413af",
    "Richard Kane Ferguson",
);

// 6ED 2 — Archangel (reprint)
const ARCHANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::visions::ARCHANGEL,
    "2734c616-cb19-4ed6-af7f-fc077f299e6e",
    "Quinton Hoover",
);

// 6ED 3 — Ardent Militia (reprint)
const ARDENT_MILITIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ARDENT_MILITIA,
    "95608d51-9ec0-497c-a065-15adb7eff242",
    "Zina Saunders",
);

// 6ED 4 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ARMAGEDDON,
    "ccf3abe6-0b86-4010-8fc6-616af77b4ace",
    "Rob Alexander",
);

// 6ED 5 — Armored Pegasus (reprint)
const ARMORED_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ARMORED_PEGASUS,
    "ed1b462d-4b0f-40cf-89a0-17a3e1c8a0ba",
    "Andrew Robinson",
);

// 6ED 6 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "6a77e784-d7a8-4b92-8765-375ad70b929e",
    "Dameon Willich",
);

// 6ED 7 — Celestial Dawn (reprint)
const CELESTIAL_DAWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CELESTIAL_DAWN,
    "3a51b3c5-7b4a-4ed3-be33-5b854c005b99",
    "Liz Danforth",
);

// 6ED 8 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "605bbcc8-973b-4e06-8c33-7e444d03fcd8",
    "Gerry Grace",
);

// 6ED 9 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "286368f3-65a6-4858-ac7f-edb06a741151",
    "Gerry Grace",
);

// 6ED 10 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "a3187cbd-7925-467b-a745-a0050045900b",
    "Gerry Grace",
);

// 6ED 11 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "3357139a-8b8b-47c5-a35b-3ed98968ba4d",
    "Gerry Grace",
);

// 6ED 12 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "e2b7a978-692e-4015-9b90-e51ee98c5e3e",
    "Gerry Grace",
);

// 6ED 13 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRUSADE,
    "5f3f5f07-f692-4531-81b6-31813574ec12",
    "D. Alexander Gregory",
);

// 6ED 14 — Daraja Griffin (reprint)
const DARAJA_GRIFFIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::DARAJA_GRIFFIN,
    "0795a1d6-9caf-472a-a349-fca97bccf8e2",
    "Stuart Griffin",
);

// 6ED 15 — D'Avenant Archer (reprint)
const DAVENANT_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DAVENANT_ARCHER,
    "e97985bb-75ab-454e-894d-963890043caf",
    "Douglas Shuler",
);

// 6ED 16 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "47578ce0-dbaa-4a15-b46c-2fd2cb352be9",
    "Brian Snõddy",
);

// 6ED 17 — Divine Transformation (reprint)
const DIVINE_TRANSFORMATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::DIVINE_TRANSFORMATION,
    "ad0d2e46-814b-40cb-81a0-8d89285bf196",
    "NéNé Thomas",
);

// 6ED 18 — Ekundu Griffin (reprint)
const EKUNDU_GRIFFIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::EKUNDU_GRIFFIN,
    "a1eec4ac-7d28-4f76-a1d5-a0a19c142514",
    "David A. Cherry",
);

// 6ED 19 — Enlightened Tutor (reprint)
const ENLIGHTENED_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ENLIGHTENED_TUTOR,
    "e869da95-2c47-4796-aadc-50652ebb4d03",
    "Dan Frazier",
);

// 6ED 20 — Ethereal Champion (reprint)
const ETHEREAL_CHAMPION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ETHEREAL_CHAMPION,
    "7c1733e2-bee2-4b85-b165-d4329402578b",
    "Terese Nielsen",
);

// 6ED 21 — Exile (reprint)
const EXILE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::EXILE,
    "bf6e3ca4-5b56-40bb-bec7-c92fc7eb50d2",
    "Rob Alexander",
);

// 6ED 22 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "4953ac06-dc8c-4a9c-8ff6-8e4432dae91f",
    "Dan Frazier",
);

// 6ED 23 — Heavy Ballista (reprint)
const HEAVY_BALLISTA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::HEAVY_BALLISTA,
    "9c587228-b633-4049-beb8-e45aae967167",
    "Ron Spencer",
);

// 6ED 24 — Hero's Resolve (reprint)
const HERO_S_RESOLVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::HERO_S_RESOLVE,
    "71a0f233-895e-4072-8339-c9448110d3e8",
    "Pete Venters",
);

// 6ED 25 — Icatian Town (reprint)
const ICATIAN_TOWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::ICATIAN_TOWN,
    "f7582903-57b0-42e6-991c-0f93ab9172d0",
    "Tom Wänerstrand",
);

// 6ED 26 — Infantry Veteran (reprint)
const INFANTRY_VETERAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::INFANTRY_VETERAN,
    "e59c911f-917a-4c3e-ad35-2a01c25339e9",
    "Christopher Rush",
);

// 6ED 27 — Kismet (reprint)
const KISMET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::KISMET,
    "d18d53f6-4cec-4c91-a507-39038d300b00",
    "Kaja Foglio",
);

// 6ED 28 — Kjeldoran Royal Guard (reprint)
const KJELDORAN_ROYAL_GUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KJELDORAN_ROYAL_GUARD,
    "8e65e795-84e2-44c9-9f20-469e8c59f147",
    "Allen Williams",
);

// 6ED 29 — Light of Day (reprint)
const LIGHT_OF_DAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::LIGHT_OF_DAY,
    "3d5af047-7d98-4ba9-9ce1-c67563c19866",
    "Drew Tucker",
);

// 6ED 30 — Longbow Archer (reprint)
const LONGBOW_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::LONGBOW_ARCHER,
    "2fdac001-4cfb-4991-ac70-d430750d5047",
    "Eric Peterson",
);

// 6ED 31 — Mesa Falcon (reprint)
const MESA_FALCON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::MESA_FALCON,
    "a7ce1b8e-13ba-4eed-a445-435300f3101e",
    "Mark Poole",
);

// 6ED 32 — Order of the Sacred Torch (reprint)
const ORDER_OF_THE_SACRED_TORCH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ORDER_OF_THE_SACRED_TORCH,
    "760cf598-41e1-4cdc-9a30-964e67ffaf52",
    "Ruth Thompson",
);

// 6ED 33 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "132d0ac2-08aa-4f3b-9616-006c0bf09f59",
    "Robert Bliss",
);

// 6ED 34 — Pearl Dragon (reprint)
const PEARL_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PEARL_DRAGON,
    "3efee309-2eba-4702-9361-0f75043922bb",
    "Ian Miller",
);

// 6ED 35 — Regal Unicorn (reprint)
const REGAL_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::REGAL_UNICORN,
    "54ca9b1c-fead-4bb6-800f-8b762a82fda7",
    "Zina Saunders",
);

// 6ED 36 — Remedy (reprint)
const REMEDY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::REMEDY,
    "57cdfb23-3c62-4312-a503-e30be384e3ab",
    "Zina Saunders",
);

// 6ED 37 — Reprisal (reprint)
const REPRISAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::REPRISAL,
    "229be020-de00-4985-b6f0-e6276018591e",
    "Randy Asplund-Faith",
);

// 6ED 38 — Resistance Fighter (reprint)
const RESISTANCE_FIGHTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::RESISTANCE_FIGHTER,
    "1abf09d8-f972-4b81-80e9-70fb4a33ed56",
    "Cecil Fernando",
);

// 6ED 39 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "1040133a-f80d-48dc-a50b-a11e5b793a2b",
    "Thomas Gianni",
);

// 6ED 40 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "e1166d39-e186-4bb0-8ca4-ccc0200d13de",
    "Tom Wänerstrand",
);

// 6ED 41 — Serenity (reprint)
const SERENITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::SERENITY,
    "3b37593d-13e7-4489-84bc-7074032b6f05",
    "Cliff Nielsen",
);

// 6ED 42 — Serra's Blessing (reprint)
const SERRA_S_BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::SERRA_S_BLESSING,
    "c6751671-9ef9-45f2-8e27-8c896936929a",
    "Rebecca Guay",
);

// 6ED 43 — Spirit Link (reprint)
const SPIRIT_LINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SPIRIT_LINK,
    "52bb60c6-d5e9-474f-a20f-8f705e7372cd",
    "Kaja Foglio",
);

// 6ED 44 — Standing Troops (reprint)
const STANDING_TROOPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_exo::STANDING_TROOPS,
    "b7db7c36-0992-413d-851b-0c7e095d7a6e",
    "Daren Bader",
);

// 6ED 45 — Staunch Defenders (reprint)
const STAUNCH_DEFENDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::STAUNCH_DEFENDERS,
    "00d2c54f-a1f4-4015-a4f3-8cd360fa466d",
    "Mark Poole",
);

// 6ED 46 — Sunweb (reprint)
const SUNWEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::SUNWEB,
    "19b55e90-021e-48be-8abd-177e39200d15",
    "Dan Frazier",
);

// 6ED 47 — Tariff (reprint)
const TARIFF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::TARIFF,
    "61094858-b8a8-425f-9c96-fac4fc6ecae8",
    "Kev Walker",
);

// 6ED 48 — Tundra Wolves (reprint)
const TUNDRA_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::TUNDRA_WOLVES,
    "8a1e1378-9e27-4cce-88e7-a3bf3dcd2977",
    "Quinton Hoover",
);

// 6ED 49 — Unyaro Griffin (reprint)
const UNYARO_GRIFFIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::UNYARO_GRIFFIN,
    "5e7af2b0-e07f-4b59-8fcf-e51c47f4f095",
    "Al Davidson",
);

// 6ED 50 — Venerable Monk (reprint)
const VENERABLE_MONK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::VENERABLE_MONK,
    "777fdd5d-2ed0-42d9-a1b5-34f1d61b668b",
    "D. Alexander Gregory",
);

// 6ED 51 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "f7f0219d-1bf9-4514-9ab3-b241bc541525",
    "Brian Snõddy",
);

// 6ED 52 — Warmth (reprint)
const WARMTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::WARMTH,
    "cebd9062-a702-4f30-bba4-c2531e5ca5cd",
    "Drew Tucker",
);

// 6ED 53 — Warrior's Honor (reprint)
const WARRIOR_S_HONOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::WARRIOR_S_HONOR,
    "74900e91-d661-4846-984e-5774c5ce0540",
    "D. Alexander Gregory",
);

// 6ED 54 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WRATH_OF_GOD,
    "e5513964-1cad-4083-a3a5-1e55ec145a6e",
    "Quinton Hoover",
);

// 6ED 55 — Abduction (reprint)
const ABDUCTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::ABDUCTION,
    "63c82bef-50d6-4d25-bc3f-dda2826fc99c",
    "Colin MacNeil",
);

// 6ED 56 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "5d31dca7-df16-4a70-8f17-b78d745bac96",
    "Doug Chaffee",
);

// 6ED 57 — Ancestral Memories (reprint)
const ANCESTRAL_MEMORIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ANCESTRAL_MEMORIES,
    "9953a9d1-62db-4610-bf8c-74c321f059c2",
    "William Donohoe",
);

// 6ED 58 — Boomerang (reprint)
const BOOMERANG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BOOMERANG,
    "ef3c7465-6534-4fa9-a772-8859b7210fdf",
    "Richard Kane Ferguson",
);

// 6ED 59 — Browse (reprint)
const BROWSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::BROWSE,
    "3fd1b41f-5113-4a7c-9c35-04ebdf002af2",
    "Phil Foglio",
);

// 6ED 60 — Chill (reprint)
const CHILL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::CHILL,
    "24846eba-e085-4d1d-8c7a-d4faf11034a6",
    "Greg Simanson",
);

// 6ED 61 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "ee0d3f5f-7790-4772-bead-5d7114a23e94",
    "Hannibal King",
);

// 6ED 62 — Daring Apprentice (reprint)
const DARING_APPRENTICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::DARING_APPRENTICE,
    "0344006b-990e-46a0-a6d4-ace88af66b46",
    "Kaja Foglio",
);

// 6ED 63 — Deflection (reprint)
const DEFLECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::DEFLECTION,
    "4ca34222-bf30-41e9-a166-e6b02bd6e46a",
    "Mike Raabe",
);

// 6ED 64 — Desertion (reprint)
const DESERTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::DESERTION,
    "ac212677-daa3-49ee-adb1-53a169cb7e9d",
    "Richard Kane Ferguson",
);

// 6ED 65 — Diminishing Returns (reprint)
const DIMINISHING_RETURNS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::DIMINISHING_RETURNS,
    "6c00fc18-8101-48ff-9842-2b157eb02681",
    "Allen Williams",
);

// 6ED 66 — Dream Cache (reprint)
const DREAM_CACHE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::DREAM_CACHE,
    "642bb62e-1339-4f97-8429-ec46ec0435a0",
    "D. Alexander Gregory",
);

// 6ED 67 — Flash (reprint)
const FLASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FLASH,
    "f4e2f44d-74a9-4635-bf10-bf4cf179cab5",
    "David Ho",
);

// 6ED 68 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "0f99d08e-edc4-4049-a3de-99f6c5cb0f70",
    "Jerry Tiritilli",
);

// 6ED 69 — Fog Elemental (reprint)
const FOG_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::FOG_ELEMENTAL,
    "90b0e0c8-3121-46ff-b202-9669c16c2df4",
    "Jon J Muth",
);

// 6ED 70 — Forget (reprint)
const FORGET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::FORGET,
    "8cc8e367-1aa4-43b6-b17a-01bfb097f620",
    "Mike Kimble",
);

// 6ED 71 — Gaseous Form (reprint)
const GASEOUS_FORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GASEOUS_FORM,
    "d7d74e17-5d3c-4102-aadd-263bfecca510",
    "Roger Raupp",
);

// 6ED 72 — Glacial Wall (reprint)
const GLACIAL_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::GLACIAL_WALL,
    "ea929add-39a4-4840-a127-16aeb37f55f5",
    "Dameon Willich",
);

// 6ED 73 — Harmattan Efreet (reprint)
const HARMATTAN_EFREET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::HARMATTAN_EFREET,
    "d8fc0ba3-6ffa-4fd4-b332-88da41b8778a",
    "Drew Tucker",
);

// 6ED 74 — Horned Turtle (reprint)
const HORNED_TURTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::HORNED_TURTLE,
    "b63e2f7d-52aa-43a1-ab86-7a510a131b4c",
    "DiTerlizzi",
);

// 6ED 75 — Insight (reprint)
const INSIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::INSIGHT,
    "b44a4bbf-2afd-48c5-b6b0-73c32bc3561b",
    "Ron Chironna",
);

// 6ED 76 — Inspiration (reprint)
const INSPIRATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::visions::INSPIRATION,
    "1374df24-bcff-45eb-a2fd-2f39439c9e6a",
    "Zina Saunders",
);

// 6ED 77 — Juxtapose (reprint)
const JUXTAPOSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::JUXTAPOSE,
    "2b1fb725-0daa-45f4-ad31-6a061fa4d20f",
    "Justin Hampton",
);

// 6ED 78 — Library of Lat-Nam (reprint)
const LIBRARY_OF_LAT_NAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::LIBRARY_OF_LAT_NAM,
    "9ea40438-90ca-47ff-9805-df130d47ae48",
    "Alan Rabinowitz",
);

// 6ED 79 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "d753d343-fedc-4406-b889-87e0f719d361",
    "Melissa A. Benson",
);

// 6ED 80 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_SHORT,
    "2c2893b8-11e4-4a76-a3bf-6dd86c11ae09",
    "Dameon Willich",
);

// 6ED 81 — Memory Lapse (reprint)
const MEMORY_LAPSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::MEMORY_LAPSE,
    "ecc92a87-3ad2-4db6-aeae-001342f17d10",
    "Mark Tedin",
);

// 6ED 82 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "71c43f23-19c1-4b5a-91d4-9961ffc7fcf1",
    "DiTerlizzi",
);

// 6ED 83 — Mystical Tutor (reprint)
const MYSTICAL_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MYSTICAL_TUTOR,
    "1571b584-9007-45ee-a3c3-6c72f227fee2",
    "David O'Connor",
);

// 6ED 84 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "80ab11d8-0cc2-4a00-9fe8-06a9b3683311",
    "David A. Cherry",
);

// 6ED 85 — Phantom Warrior (reprint)
const PHANTOM_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::PHANTOM_WARRIOR,
    "215e560f-077b-4a4f-aba3-e5c8dab912fe",
    "John Matson",
);

// 6ED 86 — Polymorph (reprint)
const POLYMORPH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::POLYMORPH,
    "e35b8c9b-40d0-4986-9457-ef1263fdfae1",
    "Robert Bliss",
);

// 6ED 87 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "098d3f20-e377-4579-b8d7-2d5e7ee3fb4e",
    "Mark Poole",
);

// 6ED 88 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "9bf045ee-7923-4b3d-9bb4-f573d37cc7d8",
    "Douglas Shuler",
);

// 6ED 89 — Prosperity (reprint)
const PROSPERITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::PROSPERITY,
    "5f0c86df-ee91-4bea-bb05-7e5db3558169",
    "Dan Frazier",
);

// 6ED 90 — Psychic Transfer (reprint)
const PSYCHIC_TRANSFER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PSYCHIC_TRANSFER,
    "d848a3e1-de15-4b8f-9881-d32aa2456488",
    "Dom!",
);

// 6ED 91 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "0ba1b264-1dd5-475b-9522-9eb9efcea572",
    "Brian Snõddy",
);

// 6ED 92 — Recall (reprint)
const RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RECALL,
    "791af96f-1de9-46f2-a1e5-93921c4905c7",
    "Brian Snõddy",
);

// 6ED 93 — Relearn (reprint)
const RELEARN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::RELEARN,
    "358f4122-fffb-46f6-996c-f4558ba79407",
    "Zina Saunders",
);

// 6ED 94 — Remove Soul (reprint)
const REMOVE_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::REMOVE_SOUL,
    "19345cab-4595-4325-b6b4-539c2003679e",
    "Mike Dringenberg",
);

// 6ED 95 — Sage Owl (reprint)
const SAGE_OWL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::SAGE_OWL,
    "45f83acb-c7be-49cf-895b-9fa6f4d32083",
    "Mark Poole",
);

// 6ED 96 — Sea Monster (reprint)
const SEA_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::SEA_MONSTER,
    "2cecf934-d1fe-424f-932a-043908546157",
    "Daniel Gelon",
);

// 6ED 97 — Segovian Leviathan (reprint)
const SEGOVIAN_LEVIATHAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SEGOVIAN_LEVIATHAN,
    "8f796dd2-9ecf-490a-86c0-acb46130518b",
    "Melissa A. Benson",
);

// 6ED 98 — Sibilant Spirit (reprint)
const SIBILANT_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SIBILANT_SPIRIT,
    "fd219fee-45d2-4d68-911e-9ecdc3a6e81c",
    "Ron Spencer",
);

// 6ED 99 — Soldevi Sage (reprint)
const SOLDEVI_SAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::SOLDEVI_SAGE,
    "268c3726-0e2d-40df-811d-2cdf6b328ea3",
    "Carol Heyer",
);

// 6ED 100 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "4e470d0f-06a0-4fdf-98eb-79d536dff894",
    "Greg Simanson",
);

// 6ED 101 — Storm Crow (reprint)
const STORM_CROW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::STORM_CROW,
    "3a1bf438-3cd8-4bd8-85f1-fc97f49b44d9",
    "Una Fricker",
);

// 6ED 102 — Tidal Surge (reprint)
const TIDAL_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::TIDAL_SURGE,
    "5dae1d63-aee0-4cbe-852d-25fde10fa4b7",
    "Doug Chaffee",
);

// 6ED 103 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "1ff96224-d6cd-492b-ab2f-3ec15b3230cb",
    "Douglas Shuler",
);

// 6ED 104 — Vodalian Soldiers (reprint)
const VODALIAN_SOLDIERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::VODALIAN_SOLDIERS,
    "f8fae146-a0dd-4622-ab11-f00b372f8221",
    "Melissa A. Benson",
);

// 6ED 105 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "1a99adbb-7723-4046-92af-95d6d21fae53",
    "Richard Thomas",
);

// 6ED 106 — Wind Drake (reprint)
const WIND_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::WIND_DRAKE,
    "91ca5eed-53a3-4da5-b7fc-f08e6cc93946",
    "Zina Saunders",
);

// 6ED 107 — Wind Spirit (reprint)
const WIND_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::WIND_SPIRIT,
    "d9041dc6-7521-4ae3-b8b3-5134fea97581",
    "Kaja Foglio",
);

// 6ED 108 — Zur's Weirding (reprint)
const ZUR_S_WEIRDING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ZUR_S_WEIRDING,
    "eb7a31bb-5f3b-4215-9e21-9965d459f032",
    "Liz Danforth",
);

// 6ED 109 — Abyssal Hunter (reprint)
const ABYSSAL_HUNTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ABYSSAL_HUNTER,
    "3c3f6ad7-4782-40db-8eb3-e71d71ec3388",
    "Steve Luke",
);

// 6ED 110 — Abyssal Specter (reprint)
const ABYSSAL_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ABYSSAL_SPECTER,
    "36e4df76-41c8-4410-a505-4f328c94b974",
    "George Pratt",
);

// 6ED 111 — Agonizing Memories (reprint)
const AGONIZING_MEMORIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::AGONIZING_MEMORIES,
    "6556e1e2-44a6-49f1-8417-be8dae4ef65b",
    "Mike Dringenberg",
);

// 6ED 112 — Ashen Powder (reprint)
const ASHEN_POWDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ASHEN_POWDER,
    "d6fa11f5-546b-4a14-afd0-80866d4968ab",
    "Geofrey Darrow",
);

// 6ED 113 — Blight (reprint)
const BLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::BLIGHT,
    "2a08e1fd-7ab1-4981-9fc4-f9d32a58ac4e",
    "Ian Miller",
);

// 6ED 114 — Blighted Shaman (reprint)
const BLIGHTED_SHAMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::BLIGHTED_SHAMAN,
    "1d46c90a-215a-4897-94f8-52a02abf25c4",
    "Ian Miller",
);

// 6ED 115 — Blood Pet (reprint)
const BLOOD_PET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::BLOOD_PET,
    "a6b30e7e-f628-4ae0-8338-c95022b3fedf",
    "Brom",
);

// 6ED 116 — Bog Imp (reprint)
const BOG_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_IMP,
    "fa02b4a5-8302-483c-9f38-b559974a601c",
    "Christopher Rush",
);

// 6ED 117 — Bog Rats (reprint)
const BOG_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BOG_RATS,
    "0da038c4-73ff-4d82-8440-9bca2f051fd5",
    "Ron Spencer",
);

// 6ED 118 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "1145a4b6-36f9-4cdc-9e81-ad8c22a21150",
    "Jeff A. Menges",
);

// 6ED 119 — Coercion (reprint)
const COERCION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::COERCION,
    "4a8337ac-fddc-4af7-a623-e9a8c8323564",
    "DiTerlizzi",
);

// 6ED 120 — Derelor (reprint)
const DERELOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DERELOR,
    "530043ad-d4bf-4fb0-b6e0-f8a744968cfc",
    "Anson Maddocks",
);

// 6ED 121 — Doomsday (reprint)
const DOOMSDAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::DOOMSDAY,
    "dcc9dbfa-0043-47d2-acfe-f636841afc2c",
    "Adrian Smith",
);

// 6ED 122 — Dread of Night (reprint)
const DREAD_OF_NIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::DREAD_OF_NIGHT,
    "a4105fb4-ab13-4a34-810b-1d294c5a6eee",
    "Richard Thomas",
);

// 6ED 123 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "e7105716-8f9e-4f32-b6bc-cb7b231d1fa1",
    "Ian Miller",
);

// 6ED 123s — Drudge Skeletons (alternate printing)
const DRUDGE_SKELETONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::DRUDGE_SKELETONS,
    1,
    "9670f4eb-cca7-45fd-b3d7-58b436c00526",
    "Carl Critchlow",
);

// 6ED 124 — Dry Spell (reprint)
const DRY_SPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::DRY_SPELL,
    "f765719b-609a-47ae-8dc8-0c97db104d1b",
    "Brian Snõddy",
);

// 6ED 125 — Enfeeblement (reprint)
const ENFEEBLEMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ENFEEBLEMENT,
    "63418388-4e48-42cd-a84d-d631d01476a3",
    "John Bolton",
);

// 6ED 126 — Evil Eye of Orms-by-Gore (reprint)
const EVIL_EYE_OF_ORMS_BY_GORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::EVIL_EYE_OF_ORMS_BY_GORE,
    "c31caa65-accd-4306-a975-1aaa5d98aeaa",
    "George Pratt",
);

// 6ED 127 — Fallen Angel (reprint)
const FALLEN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::FALLEN_ANGEL,
    "bd81ae10-ebf4-4731-b32c-dc5954c3442c",
    "Anson Maddocks",
);

// 6ED 128 — Fatal Blow (reprint)
const FATAL_BLOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::FATAL_BLOW,
    "6890c3aa-9321-4c41-9b16-cff4e6364350",
    "George Pratt",
);

// 6ED 129 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "bb729a6b-7efc-4b94-9221-cba25f6506dc",
    "Doug Keith",
);

// 6ED 130 — Feast of the Unicorn (reprint)
const FEAST_OF_THE_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::FEAST_OF_THE_UNICORN,
    "693a5a97-d81e-4f4d-ab8f-5a9cabd4c685",
    "Dennis Detwiller",
);

// 6ED 131 — Feral Shadow (reprint)
const FERAL_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FERAL_SHADOW,
    "08263b27-487c-4b5c-aca6-0d77acdcc624",
    "Cliff Nielsen",
);

// 6ED 132 — Forbidden Crypt (reprint)
const FORBIDDEN_CRYPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FORBIDDEN_CRYPT,
    "76f5f509-707b-4ed6-9824-626dea5869b0",
    "D. Alexander Gregory",
);

// 6ED 133 — Gravebane Zombie (reprint)
const GRAVEBANE_ZOMBIE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::GRAVEBANE_ZOMBIE,
    "df21b384-6073-477e-bd95-fc94ca2f2f2c",
    "Gary Leach",
);

// 6ED 134 — Gravedigger (reprint)
const GRAVEDIGGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::GRAVEDIGGER,
    "35ae81a3-f76a-406b-bbac-36f8abf456ce",
    "Dermot Power",
);

// 6ED 135 — Greed (reprint)
const GREED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GREED,
    "977d0b73-ebc1-4082-a90c-eda363732bbe",
    "Phil Foglio",
);

// 6ED 136 — Hecatomb (reprint)
const HECATOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::HECATOMB,
    "18d2908e-1608-4858-b93c-e9ea9808f9c9",
    "George Pratt",
);

// 6ED 137 — Hidden Horror (reprint)
const HIDDEN_HORROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::HIDDEN_HORROR,
    "785de2b3-ffd6-40f5-a5d2-c8fa30dbf10f",
    "Clint Langley",
);

// 6ED 138 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "bfeb9612-cc19-4769-9d07-38d5e2796053",
    "John Coulthart",
);

// 6ED 139 — Infernal Contract (reprint)
const INFERNAL_CONTRACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::INFERNAL_CONTRACT,
    "94517aeb-c018-4390-b601-c44a7af0f090",
    "Roger Raupp",
);

// 6ED 140 — Kjeldoran Dead (reprint)
const KJELDORAN_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KJELDORAN_DEAD,
    "37dff6e2-4280-494b-9647-9ec85248ac77",
    "Melissa A. Benson",
);

// 6ED 141 — Leshrac's Rite (reprint)
const LESHRAC_S_RITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::LESHRAC_S_RITE,
    "0c44599f-f788-43fa-ace3-a521f15256ad",
    "Mike Raabe",
);

// 6ED 142 — Lost Soul (reprint)
const LOST_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::LOST_SOUL,
    "401896ac-6234-468a-a9af-cf11c7a11cd0",
    "Randy Asplund-Faith",
);

// 6ED 143 — Mind Warp (reprint)
const MIND_WARP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MIND_WARP,
    "e1f0d58c-24fa-498f-9531-e62144401e86",
    "Liz Danforth",
);

// 6ED 144 — Mischievous Poltergeist (reprint)
const MISCHIEVOUS_POLTERGEIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::MISCHIEVOUS_POLTERGEIST,
    "d11803b8-d1df-454d-93ec-b1bb276843b6",
    "DiTerlizzi",
);

// 6ED 145 — Necrosavant (reprint)
const NECROSAVANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::NECROSAVANT,
    "164a9755-f003-456d-a827-d6ef6cc29a86",
    "John Coulthart",
);

// 6ED 146 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "3c2a1e82-4922-4075-a869-3a1b607498c3",
    "Melissa A. Benson",
);

// 6ED 147 — Painful Memories (reprint)
const PAINFUL_MEMORIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PAINFUL_MEMORIES,
    "bdec2ddc-3a91-4872-b4c0-e75af6cbb184",
    "John Coulthart",
);

// 6ED 148 — Perish (reprint)
const PERISH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::PERISH,
    "77192bdf-d89f-45a1-a30b-20107e990031",
    "Rebecca Guay",
);

// 6ED 149 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "29d852c4-bd53-4a3b-b1e2-896917cbc27f",
    "Kev Walker",
);

// 6ED 150 — Python (reprint)
const PYTHON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::PYTHON,
    "84fbe194-1d9b-4d3f-b7a0-aa058945aca1",
    "Steve White",
);

// 6ED 151 — Rag Man (reprint)
const RAG_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::RAG_MAN,
    "7ef9159a-66ee-4e99-8fb6-6f45b02f9880",
    "Daniel Gelon",
);

// 6ED 152 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "71bb239e-4517-4add-a660-a89095e40a8e",
    "Charles Gillespie",
);

// 6ED 153 — Razortooth Rats (reprint)
const RAZORTOOTH_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::RAZORTOOTH_RATS,
    "5ab55c86-3576-43fd-b555-ab0b3ad936c7",
    "Brian Horton",
);

// 6ED 154 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "6e992239-691b-4bb2-a005-c73c24a52a9b",
    "Jesper Myrfors",
);

// 6ED 155 — Sengir Autocrat (reprint)
const SENGIR_AUTOCRAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::SENGIR_AUTOCRAT,
    "234a5401-bbe4-40ab-9204-12d944fbd2b1",
    "David A. Cherry",
);

// 6ED 156 — Strands of Night (reprint)
const STRANDS_OF_NIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::STRANDS_OF_NIGHT,
    "13382490-205e-4dcd-8524-2b17008b5237",
    "Patrick Kochakji",
);

// 6ED 157 — Stromgald Cabal (reprint)
const STROMGALD_CABAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::STROMGALD_CABAL,
    "1d78c90b-b44e-431a-a4e6-ceae0a019428",
    "Anson Maddocks",
);

// 6ED 158 — Stupor (reprint)
const STUPOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::STUPOR,
    "beec72a6-b482-4e04-8491-c6cb0afeb568",
    "Mike Kimble",
);

// 6ED 159 — Syphon Soul (reprint)
const SYPHON_SOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::SYPHON_SOUL,
    "2b329533-9d55-456f-8fb1-0ab97c4b3037",
    "Melissa A. Benson",
);

// 6ED 160 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TERROR,
    "fe73e689-b9da-4b1e-9809-c25056c06048",
    "Ron Spencer",
);

// 6ED 161 — Vampiric Tutor (reprint)
const VAMPIRIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::VAMPIRIC_TUTOR,
    "c8505bc8-218f-4b56-be78-ebde535ebaa0",
    "Gary Leach",
);

// 6ED 162 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "346707a7-e816-432c-bb93-16b5cb4616e0",
    "Jeff A. Menges",
);

// 6ED 163 — Aether Flash (reprint)
const AETHER_FLASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::AETHER_FLASH,
    "c61721ec-6008-4704-85d6-83d4f2558b5a",
    "Ron Spencer",
);

// 6ED 164 — Anaba Bodyguard (reprint)
const ANABA_BODYGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::ANABA_BODYGUARD,
    "13c21e32-6f43-4475-bd9e-472ca8cbd4a6",
    "Anson Maddocks",
);

// 6ED 165 — Anaba Shaman (reprint)
const ANABA_SHAMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hml::ANABA_SHAMAN,
    "d1f8eccc-551d-4109-866e-7285435ffd19",
    "Anson Maddocks",
);

// 6ED 166 — Balduvian Barbarians (reprint)
const BALDUVIAN_BARBARIANS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BALDUVIAN_BARBARIANS,
    "de983bae-1b73-4dd7-a2f4-4200a21abe6a",
    "Mark Poole",
);

// 6ED 167 — Balduvian Horde (reprint)
const BALDUVIAN_HORDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::BALDUVIAN_HORDE,
    "0fbcf78f-b5a9-4ed0-a409-d4565bebc56d",
    "Brian Snõddy",
);

// 6ED 168 — Blaze (reprint)
const BLAZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BLAZE,
    "9f09111a-e714-4f8f-8a48-61c102e45123",
    "Gerry Grace",
);

// 6ED 169 — Boil (reprint)
const BOIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::BOIL,
    "4e8cd169-0eaa-4430-a175-7f9bbf552929",
    "Jason Alexander Behnke",
);

// 6ED 170 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "c665180c-a69b-446e-9894-3b3be624db7f",
    "Mark Poole",
);

// 6ED 171 — Conquer (reprint)
const CONQUER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::CONQUER,
    "deb9ae3b-0da5-40df-8f52-54238c965137",
    "Randy Gallegos",
);

// 6ED 172 — Crimson Hellkite (reprint)
const CRIMSON_HELLKITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CRIMSON_HELLKITE,
    "88daedae-a848-4d08-bceb-b71cec2673fb",
    "Gerry Grace",
);

// 6ED 173 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHQUAKE,
    "8f58f85d-905a-4569-b3e6-adafc387c1cb",
    "Richard Kane Ferguson",
);

// 6ED 174 — Fervor (reprint)
const FERVOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::weatherlight::FERVOR,
    "bc84b68c-2079-4268-840c-f7a86675c0ba",
    "Franz Vohwinkel",
);

// 6ED 175 — Final Fortune (reprint)
const FINAL_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FINAL_FORTUNE,
    "627aeab7-ccdd-4b23-8e6c-976636fe308e",
    "D. Alexander Gregory",
);

// 6ED 176 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "5df3b58b-ce10-459f-87f7-bb243b854fc3",
    "Melissa A. Benson",
);

// 6ED 177 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "285d54f3-d21f-4854-a4ab-5912771330a5",
    "Mike Kerr",
);

// 6ED 178 — Fit of Rage (reprint)
const FIT_OF_RAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::FIT_OF_RAGE,
    "b79c17d2-dcf1-4464-ae58-5ea5117ae531",
    "Douglas Shuler",
);

// 6ED 179 — Flame Spirit (reprint)
const FLAME_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FLAME_SPIRIT,
    "75509b1c-ae9c-4708-8f9b-67af18a7e9d3",
    "Justin Hampton",
);

// 6ED 180 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "26cc2cc5-4438-481a-83b0-bf822d3f44cd",
    "Randy Gallegos",
);

// 6ED 181 — Giant Strength (reprint)
const GIANT_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::GIANT_STRENGTH,
    "e28f7c1c-1d75-4d78-8577-1e98cf8e0703",
    "Kev Walker",
);

// 6ED 182 — Goblin Digging Team (reprint)
const GOBLIN_DIGGING_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_DIGGING_TEAM,
    "83f9cf20-ad89-4942-83f5-17515c538faa",
    "Phil Foglio",
);

// 6ED 183 — Goblin Elite Infantry (reprint)
const GOBLIN_ELITE_INFANTRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::GOBLIN_ELITE_INFANTRY,
    "4bb6e4a7-206e-496d-947a-65cbeae60841",
    "Robert Bliss",
);

// 6ED 184 — Goblin Hero (reprint)
const GOBLIN_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::GOBLIN_HERO,
    "b39f3d36-6648-4e3c-bd9d-336479d1ad72",
    "Pete Venters",
);

// 6ED 185 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_KING,
    "440546d3-91d4-45f3-ae36-bdf5bec2b673",
    "Phil Foglio",
);

// 6ED 186 — Goblin Recruiter (reprint)
const GOBLIN_RECRUITER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::GOBLIN_RECRUITER,
    "61bd1548-2ffa-4705-ba88-913f37d4ce92",
    "Scott Kirschner",
);

// 6ED 187 — Goblin Warrens (reprint)
const GOBLIN_WARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::GOBLIN_WARRENS,
    "1255654b-f983-4862-9112-9e378ea5d9fe",
    "Dan Frazier",
);

// 6ED 188 — Hammer of Bogardan (reprint)
const HAMMER_OF_BOGARDAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::HAMMER_OF_BOGARDAN,
    "b759a8d8-9c6b-4d1b-b17a-6b4f349dd553",
    "Ron Spencer",
);

// 6ED 189 — Hulking Cyclops (reprint)
const HULKING_CYCLOPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::HULKING_CYCLOPS,
    "0966fa96-2453-4555-8a90-8e4b7a393038",
    "Paolo Parente",
);

// 6ED 190 — Illicit Auction (reprint)
const ILLICIT_AUCTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::ILLICIT_AUCTION,
    "97d68c39-cfdd-4883-9058-d648d073ae36",
    "Scott Kirschner",
);

// 6ED 191 — Inferno (reprint)
const INFERNO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::INFERNO,
    "2cfa44a4-5351-4334-a9d5-c5c19fcccca5",
    "Mike Kerr",
);

// 6ED 192 — Jokulhaups (reprint)
const JOKULHAUPS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::JOKULHAUPS,
    "9ef08669-ccce-45a3-94fc-d3caa213c068",
    "Mike Kerr",
);

// 6ED 193 — Lightning Blast (reprint)
const LIGHTNING_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::LIGHTNING_BLAST,
    "59c92e4a-34d2-4947-9c5f-2ceb8e5ae53d",
    "Richard Thomas",
);

// 6ED 194 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "bd4942f9-c3ec-4cb1-9300-9f6ed9e7caa9",
    "Christopher Rush",
);

// 6ED 195 — Mountain Goat (reprint)
const MOUNTAIN_GOAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::MOUNTAIN_GOAT,
    "47754124-37e2-4878-a711-a3e00ae0bc70",
    "Cornelius Brudi",
);

// 6ED 196 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "39d71507-d71a-476d-aa8d-9eab60183f95",
    "Dan Frazier",
);

// 6ED 197 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "11192972-eb57-4042-83f2-2470b49940b8",
    "Dan Frazier",
);

// 6ED 198 — Pillage (reprint)
const PILLAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::PILLAGE,
    "d04eeaba-7db8-49e8-b775-fbe9c89413fd",
    "Richard Kane Ferguson",
);

// 6ED 199 — Pyrotechnics (reprint)
const PYROTECHNICS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PYROTECHNICS,
    "76b4cb5f-a9f0-41a1-b9a8-ea483e12633f",
    "Anson Maddocks",
);

// 6ED 200 — Raging Goblin (reprint)
const RAGING_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::RAGING_GOBLIN,
    "f41205d2-ed1c-4c16-a171-096c06016705",
    "Jeff Miracola",
);

// 6ED 201 — Reckless Embermage (reprint)
const RECKLESS_EMBERMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::RECKLESS_EMBERMAGE,
    "b7f262e8-0567-4e6d-8435-eda661d89a8f",
    "Tom Kyffin",
);

// 6ED 202 — Relentless Assault (reprint)
const RELENTLESS_ASSAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::RELENTLESS_ASSAULT,
    "deb66d77-2bf5-441c-9ef8-1728dc5987ca",
    "Geofrey Darrow & I. Rabarot",
);

// 6ED 203 — Sabretooth Tiger (reprint)
const SABRETOOTH_TIGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SABRETOOTH_TIGER,
    "b4f0585f-f4ea-4b49-b090-8fdf56e5de7d",
    "Melissa A. Benson",
);

// 6ED 204 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "5a255e43-be06-45f0-a74e-3436d76899f9",
    "Jason Alexander Behnke",
);

// 6ED 205 — Shatterstorm (reprint)
const SHATTERSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::SHATTERSTORM,
    "f5129c8a-735b-44f5-9233-24b905e3103a",
    "James Allen",
);

// 6ED 206 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::stronghold::SHOCK,
    "68a68db8-5e4e-4c98-a319-7717cc39e831",
    "Randy Gallegos",
);

// 6ED 207 — Spitting Drake (reprint)
const SPITTING_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::SPITTING_DRAKE,
    "c9ec1676-f59b-4ab6-995c-d2525ac11370",
    "Geofrey Darrow & I. Rabarot",
);

// 6ED 208 — Spitting Earth (reprint)
const SPITTING_EARTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::SPITTING_EARTH,
    "c1e87de2-9d6b-4158-a871-5c81d05db7f0",
    "Brian Snõddy",
);

// 6ED 209 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "1e9a0841-609f-4a57-8a4a-39d460e31af8",
    "John Matson",
);

// 6ED 210 — Talruum Minotaur (reprint)
const TALRUUM_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::TALRUUM_MINOTAUR,
    "4a4f1317-5e9b-4f49-9ed8-4f97f8c4b8d0",
    "Pete Venters",
);

// 6ED 211 — Tremor (reprint)
const TREMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::TREMOR,
    "1f143421-a770-4df1-a593-af24025bdb2f",
    "Pete Venters",
);

// 6ED 212 — Vertigo (reprint)
const VERTIGO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::VERTIGO,
    "0a58ac6a-095e-47d7-ba42-56d148e3c6b9",
    "Drew Tucker",
);

// 6ED 213 — Viashino Warrior (reprint)
const VIASHINO_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::VIASHINO_WARRIOR,
    "bb467271-898f-4bcd-8533-e8165b318b43",
    "Roger Raupp",
);

// 6ED 214 — Volcanic Dragon (reprint)
const VOLCANIC_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::VOLCANIC_DRAGON,
    "b983c88c-6b0d-498a-b8e8-65b9733db62c",
    "Janine Johnston",
);

// 6ED 215 — Volcanic Geyser (reprint)
const VOLCANIC_GEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::VOLCANIC_GEYSER,
    "2e89d367-b213-4f73-8f23-79788c00d7c1",
    "David O'Connor",
);

// 6ED 216 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "a5da58c5-51a9-4d4b-8b68-a21b6981602d",
    "Richard Thomas",
);

// 6ED 217 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BIRDS_OF_PARADISE,
    "0ad85d15-e700-455d-96f4-dfd6661f9722",
    "Mark Poole",
);

// 6ED 218 — Call of the Wild (reprint)
const CALL_OF_THE_WILD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::CALL_OF_THE_WILD,
    "8c99602c-92a0-489d-a00d-1e65563365c6",
    "Brom",
);

// 6ED 219 — Cat Warriors (reprint)
const CAT_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::CAT_WARRIORS,
    "90823485-83cd-4260-b860-7f05d8588905",
    "Melissa A. Benson",
);

// 6ED 220 — Creeping Mold (reprint)
const CREEPING_MOLD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::CREEPING_MOLD,
    "9c4cf599-2fd6-46cc-a3dc-4fe8e4c21f42",
    "David Seeley",
);

// 6ED 221 — Dense Foliage (reprint)
const DENSE_FOLIAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::DENSE_FOLIAGE,
    "f6232f04-cd4c-4889-97dc-cccc4a09f9c4",
    "Alan Rabinowitz",
);

// 6ED 222 — Early Harvest (reprint)
const EARLY_HARVEST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::EARLY_HARVEST,
    "72df2d73-e58b-43e7-90ad-2d61ca33ccbb",
    "Janine Johnston",
);

// 6ED 223 — Elder Druid (reprint)
const ELDER_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ELDER_DRUID,
    "da912a54-ab69-4765-b23e-7b9d622590a8",
    "Richard Kane Ferguson",
);

// 6ED 224 — Elven Cache (reprint)
const ELVEN_CACHE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::ELVEN_CACHE,
    "10d3d239-1e16-4a23-9098-ee67d32e0208",
    "Rebecca Guay",
);

// 6ED 225 — Elven Riders (reprint)
const ELVEN_RIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ELVEN_RIDERS,
    "a6da6713-f5bd-4613-90d1-37d62b3ed011",
    "Dan Frazier",
);

// 6ED 226 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "164ee97d-736e-4659-878d-d161c204142e",
    "Anson Maddocks",
);

// 6ED 227 — Fallow Earth (reprint)
const FALLOW_EARTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FALLOW_EARTH,
    "44b02b12-415c-49f4-9f60-fa53beb4216f",
    "Janine Johnston",
);

// 6ED 228 — Familiar Ground (reprint)
const FAMILIAR_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::FAMILIAR_GROUND,
    "7a4e963b-d316-4c45-8c01-4a40042b977e",
    "Jeff Miracola",
);

// 6ED 229 — Femeref Archers (reprint)
const FEMEREF_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FEMEREF_ARCHERS,
    "ad17fb15-fa7a-45f1-b2a4-0b4729a69fea",
    "William Donohoe",
);

// 6ED 230 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "39c06b61-f6b7-47d1-820a-01eb3f4497bc",
    "Harold McNeill",
);

// 6ED 231 — Fyndhorn Brownie (reprint)
const FYNDHORN_BROWNIE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FYNDHORN_BROWNIE,
    "bc2a2d39-92dd-4123-8a3f-5c756d22b6ee",
    "Richard Thomas",
);

// 6ED 232 — Fyndhorn Elder (reprint)
const FYNDHORN_ELDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::FYNDHORN_ELDER,
    "8876d176-5f4e-4c3c-b8b8-5a3fe2fedf65",
    "Donato Giancola",
);

// 6ED 233 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "c9299742-564b-48ca-a7fa-cf5ed0d98ef6",
    "DiTerlizzi",
);

// 6ED 234 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "17f78166-50e9-4aa8-9025-930b46f70041",
    "Randy Gallegos",
);

// 6ED 235 — Gorilla Chieftain (reprint)
const GORILLA_CHIEFTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::GORILLA_CHIEFTAIN,
    "0a296e4f-7ac8-4c4b-99d1-c02963d26b74",
    "Quinton Hoover",
);

// 6ED 236 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "10c845b8-40f1-44df-9e91-dab7606f6271",
    "Una Fricker",
);

// 6ED 237 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "516ea4e2-a64e-4aa7-ade1-bbf6b67ce831",
    "Andrew Robinson",
);

// 6ED 238 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "4805d850-e3c2-44b4-87f0-8bba4038717c",
    "John Matson",
);

// 6ED 239 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LLANOWAR_ELVES,
    "bb95a9a7-b0a3-4199-8c05-2519ccda738b",
    "Anson Maddocks",
);

// 6ED 240 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "f79eff92-b47f-4fc6-960c-f085127841ca",
    "Anson Maddocks",
);

// 6ED 241 — Maro (reprint)
const MARO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MARO,
    "cee9fb8b-5f1a-45ee-b9a4-a024b7b0936d",
    "Stuart Griffin",
);

// 6ED 242 — Nature's Resurgence (reprint)
const NATURE_S_RESURGENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::NATURE_S_RESURGENCE,
    "5a95a777-9ba6-4858-9ea9-255b5301c71a",
    "Scott M. Fischer",
);

// 6ED 243 — Panther Warriors (reprint)
const PANTHER_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::PANTHER_WARRIORS,
    "ba165e25-5328-40f4-b87c-9d02590f9d38",
    "Eric Peterson",
);

// 6ED 244 — Pradesh Gypsies (reprint)
const PRADESH_GYPSIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::PRADESH_GYPSIES,
    "a4c9b18c-4993-4ce1-b2bd-ab14c9f3aad7",
    "Quinton Hoover",
);

// 6ED 245 — Radjan Spirit (reprint)
const RADJAN_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::RADJAN_SPIRIT,
    "e1ae5f0a-1fea-4c20-85b2-4b8eb2aba57c",
    "Christopher Rush",
);

// 6ED 246 — Rampant Growth (reprint)
const RAMPANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::RAMPANT_GROWTH,
    "21061c59-f8f3-4d47-9996-6f75ef27bd4e",
    "Tom Kyffin",
);

// 6ED 247 — Redwood Treefolk (reprint)
const REDWOOD_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::REDWOOD_TREEFOLK,
    "a954994b-1858-47d3-a81a-5b01d4ea7619",
    "Steve Luke",
);

// 6ED 248 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "4b3fc7bf-4a12-479a-af13-9b5703d46a3a",
    "Quinton Hoover",
);

// 6ED 249 — River Boa (reprint)
const RIVER_BOA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::RIVER_BOA,
    "fff58d35-eb23-47ee-9b8c-6919ad1a413a",
    "Steve White",
);

// 6ED 250 — Rowen (reprint)
const ROWEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::ROWEN,
    "95f17db7-6f3f-45ec-9ed2-dd15ce25ff07",
    "Jon J Muth",
);

// 6ED 251 — Scaled Wurm (reprint)
const SCALED_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SCALED_WURM,
    "6e3d8906-52d5-4ed1-b548-be13ca82f21a",
    "Daniel Gelon",
);

// 6ED 252 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "08f7b4be-dce2-414e-a9f4-f7e46c1bd15e",
    "Gary Leach",
);

// 6ED 253 — Stalking Tiger (reprint)
const STALKING_TIGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::STALKING_TIGER,
    "35730e8e-bc86-41d5-9c7a-75a92e6dd11e",
    "Terese Nielsen",
);

// 6ED 254 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "97b52f44-11fb-470a-b7ab-aaaeaf6f05e2",
    "Terese Nielsen",
);

// 6ED 255 — Summer Bloom (reprint)
const SUMMER_BLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::SUMMER_BLOOM,
    "185823cc-8c08-481f-9429-e996f4983090",
    "Kaja Foglio",
);

// 6ED 256 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "2182fcd8-96ff-4232-917a-0fb4eb9bb7c2",
    "Dan Frazier",
);

// 6ED 257 — Trained Armodon (reprint)
const TRAINED_ARMODON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::TRAINED_ARMODON,
    "8f6956b1-148e-47cc-8f5b-ec31e5e8c030",
    "Gary Leach",
);

// 6ED 258 — Tranquil Grove (reprint)
const TRANQUIL_GROVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::TRANQUIL_GROVE,
    "1177118a-6455-4177-8a6d-a43a03160ab3",
    "Dylan Martens",
);

// 6ED 259 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "51255821-abb7-4b8a-bc74-eb6c9788de02",
    "Douglas Shuler",
);

// 6ED 260 — Uktabi Orangutan (reprint)
const UKTABI_ORANGUTAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::UKTABI_ORANGUTAN,
    "b6a944ef-dbf2-47c9-a245-dfd2533a0680",
    "Una Fricker",
);

// 6ED 261 — Uktabi Wildcats (reprint)
const UKTABI_WILDCATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::UKTABI_WILDCATS,
    "b7570b03-400e-482f-9d90-2d48b95d5ac3",
    "John Matson",
);

// 6ED 262 — Unseen Walker (reprint)
const UNSEEN_WALKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::UNSEEN_WALKER,
    "bc03eea5-ff21-4f4d-b4c2-ce9d8f456440",
    "Alan Rabinowitz",
);

// 6ED 263 — Untamed Wilds (reprint)
const UNTAMED_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::UNTAMED_WILDS,
    "22e522c2-ed5d-46bc-b043-6a37d6402a9f",
    "NéNé Thomas",
);

// 6ED 264 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "77131286-50ec-47a3-80fb-6194da338de6",
    "Kev Brockschmidt",
);

// 6ED 265 — Vitalize (reprint)
const VITALIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::VITALIZE,
    "435a4569-fa62-4e1a-b837-f68159cb270d",
    "Pete Venters",
);

// 6ED 266 — Waiting in the Weeds (reprint)
const WAITING_IN_THE_WEEDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::WAITING_IN_THE_WEEDS,
    "7e2d5a77-6ee8-463d-9e6a-fdb1cc6d70e6",
    "Susan Van Camp",
);

// 6ED 267 — Warthog (reprint)
const WARTHOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::WARTHOG,
    "d65630c7-3813-404c-9919-0e46c557f7b8",
    "Steve White",
);

// 6ED 268 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "ac714416-60ec-477d-b49f-83b3853a409f",
    "Pat Lewis",
);

// 6ED 269 — Worldly Tutor (reprint)
const WORLDLY_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::WORLDLY_TUTOR,
    "b34d9054-5ad8-4a7d-a5c8-58bc39051834",
    "David O'Connor",
);

// 6ED 270 — Wyluli Wolf (reprint)
const WYLULI_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::WYLULI_WOLF,
    "5a24af58-5d75-4b41-a226-60abc415ff71",
    "Susan Van Camp",
);

// 6ED 271 — Aladdin's Ring (reprint)
const ALADDINS_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDINS_RING,
    "0f6595fe-8190-4e8c-8731-cd801f550eab",
    "Stuart Griffin",
);

// 6ED 272 — Amber Prison (reprint)
const AMBER_PRISON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::AMBER_PRISON,
    "db90a5e7-8238-442a-a6a4-78c59a6adb3d",
    "Donato Giancola",
);

// 6ED 273 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANKH_OF_MISHRA,
    "c07903f1-defc-4ec7-accb-724b0219acd8",
    "Ian Miller",
);

// 6ED 274 — Ashnod's Altar (reprint)
const ASHNOD_S_ALTAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ASHNOD_S_ALTAR,
    "4a0f8965-ecd3-4da2-80aa-9c5034c0cd3b",
    "Anson Maddocks",
);

// 6ED 275 — Bottle of Suleiman (reprint)
const BOTTLE_OF_SULEIMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BOTTLE_OF_SULEIMAN,
    "fec36f9a-cd66-4c01-ae3f-42fd82f0546b",
    "DiTerlizzi",
);

// 6ED 276 — Charcoal Diamond (reprint)
const CHARCOAL_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CHARCOAL_DIAMOND,
    "7cc870ce-f296-4cb8-950f-69440651f4e7",
    "Drew Tucker",
);

// 6ED 277 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "75f1b0a3-fae4-4b87-99be-db1edabe62ed",
    "Donato Giancola",
);

// 6ED 278 — Cursed Totem (reprint)
const CURSED_TOTEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CURSED_TOTEM,
    "048ceee4-0a56-4e43-92f2-80058844baba",
    "D. Alexander Gregory",
);

// 6ED 279 — Dancing Scimitar (reprint)
const DANCING_SCIMITAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DANCING_SCIMITAR,
    "80e64a6d-f54a-418e-8a84-63033447ab38",
    "Anson Maddocks",
);

// 6ED 280 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "a4262efc-c464-4f67-8885-2160d6a7f542",
    "Randy Gallegos",
);

// 6ED 281 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "c246aa71-b833-495b-9b12-54bd158dc2a8",
    "Stuart Griffin",
);

// 6ED 282 — Dragon Engine (reprint)
const DRAGON_ENGINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DRAGON_ENGINE,
    "9b785730-32eb-40c0-83d0-a7ea59aac3e7",
    "Anson Maddocks",
);

// 6ED 283 — Dragon Mask (reprint)
const DRAGON_MASK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::DRAGON_MASK,
    "b5109270-b052-489c-951c-d4b21a41ff6f",
    "Craig Hooper",
);

// 6ED 284 — Fire Diamond (reprint)
const FIRE_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::FIRE_DIAMOND,
    "f57070e7-6359-4be2-b4d3-4a3489a2deaa",
    "Richard Thomas",
);

// 6ED 285 — Flying Carpet (reprint)
const FLYING_CARPET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::FLYING_CARPET,
    "936233f9-ad2a-4be1-8107-3ddcad783a30",
    "Mark Tedin",
);

// 6ED 286 — Fountain of Youth (reprint)
const FOUNTAIN_OF_YOUTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::FOUNTAIN_OF_YOUTH,
    "c40eed5d-adc2-469b-9a17-e368e8ebc34f",
    "Daniel Gelon",
);

// 6ED 287 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLASSES_OF_URZA,
    "abc743ec-4845-4e2e-8918-7e7854ecccfc",
    "Douglas Shuler",
);

// 6ED 288 — Grinning Totem (reprint)
const GRINNING_TOTEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::GRINNING_TOTEM,
    "95017ca7-4f45-4aea-9973-ca3f0833f77f",
    "Donato Giancola",
);

// 6ED 289 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "fa3aa493-3801-400e-965e-e518c38eb770",
    "Sandra Everingham",
);

// 6ED 290 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "649a6afe-c030-458c-8cc5-5051e0cd6fd2",
    "Mark Poole",
);

// 6ED 291 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRON_STAR,
    "43e1a0e8-3ca4-4928-9299-36efce7fb641",
    "Donato Giancola",
);

// 6ED 292 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "ade1a965-fd85-4545-a850-65c93132b8b5",
    "Donato Giancola",
);

// 6ED 293 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "8f3055a4-9a1e-4cd3-993e-8fecb6eee9a3",
    "Richard Kane Ferguson",
);

// 6ED 294 — Jalum Tome (reprint)
const JALUM_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::JALUM_TOME,
    "a0489c2d-bcf9-4179-b07a-781c6ac07980",
    "Tom Wänerstrand",
);

// 6ED 295 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JAYEMDAE_TOME,
    "e9c53463-5e99-430d-b824-2650264e8fe8",
    "Mark Tedin",
);

// 6ED 296 — Lead Golem (reprint)
const LEAD_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::LEAD_GOLEM,
    "b2ba3857-9bd7-434d-b77f-d697776f33e4",
    "Hannibal King",
);

// 6ED 297 — Mana Prism (reprint)
const MANA_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MANA_PRISM,
    "10f77654-7e37-4c0d-9dd2-fb79e968bd15",
    "Margaret Organ-Kean",
);

// 6ED 298 — Marble Diamond (reprint)
const MARBLE_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MARBLE_DIAMOND,
    "c20b4858-3648-4a86-a3c1-698254f90d5b",
    "Jeff Miracola",
);

// 6ED 299 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "89d01df6-ede2-4cca-89a5-b041b1238ebb",
    "Quinton Hoover",
);

// 6ED 300 — Millstone (reprint)
const MILLSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MILLSTONE,
    "8207d783-2548-45fe-b830-d24c677c1c8e",
    "Kaja Foglio",
);

// 6ED 301 — Moss Diamond (reprint)
const MOSS_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::MOSS_DIAMOND,
    "e1f2551f-f267-4c30-b631-d755e43dc237",
    "Donato Giancola",
);

// 6ED 302 — Mystic Compass (reprint)
const MYSTIC_COMPASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::MYSTIC_COMPASS,
    "e80bfd76-52f9-4dea-9056-8570c9b290a4",
    "Amy Weber",
);

// 6ED 303 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "96123be9-c1df-418f-b2b3-98746a0f1692",
    "Jesper Myrfors",
);

// 6ED 304 — Ornithopter (reprint)
const ORNITHOPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ORNITHOPTER,
    "09c6ac8a-01b1-4af5-89d8-ad66d9a81ceb",
    "Amy Weber",
);

// 6ED 305 — Patagia Golem (reprint)
const PATAGIA_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PATAGIA_GOLEM,
    "d5b5d7ea-6240-44a4-9792-6a8416d15e49",
    "Scott Kirschner",
);

// 6ED 306 — Pentagram of the Ages (reprint)
const PENTAGRAM_OF_THE_AGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PENTAGRAM_OF_THE_AGES,
    "939b5650-8b2c-4d2d-9829-3208c130116a",
    "Douglas Shuler",
);

// 6ED 307 — Phyrexian Vault (reprint)
const PHYREXIAN_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PHYREXIAN_VAULT,
    "597f4d88-4773-4520-b4df-22ae6008ebc1",
    "Hannibal King",
);

// 6ED 308 — Primal Clay (reprint)
const PRIMAL_CLAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::PRIMAL_CLAY,
    "036f588d-9ee6-4436-ad75-46a84dd2b168",
    "Adam Rex",
);

// 6ED 309 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "d59bf5fd-141a-4743-ad73-70ec2173e476",
    "Christopher Rush",
);

// 6ED 310 — Skull Catapult (reprint)
const SKULL_CATAPULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SKULL_CATAPULT,
    "7b1967f0-f6a9-469a-96f9-57fe77059e88",
    "Ian Miller",
);

// 6ED 311 — Sky Diamond (reprint)
const SKY_DIAMOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::SKY_DIAMOND,
    "63c771a9-a652-4661-a175-a97d4684cd92",
    "D. Alexander Gregory",
);

// 6ED 312 — Snake Basket (reprint)
const SNAKE_BASKET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::SNAKE_BASKET,
    "3a9bc174-1d10-49bf-af4f-2f6061b1796e",
    "Roger Raupp",
);

// 6ED 313 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "d22d79a5-1da4-475e-9605-f4c7bdc76590",
    "Andrew Robinson",
);

// 6ED 314 — Storm Cauldron (reprint)
const STORM_CAULDRON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_all::STORM_CAULDRON,
    "ab556986-a6f4-4445-b004-64b4bc189e55",
    "Dan Frazier",
);

// 6ED 315 — Teferi's Puzzle Box (reprint)
const TEFERI_S_PUZZLE_BOX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::TEFERI_S_PUZZLE_BOX,
    "bc5caf27-e426-420b-895d-a359e6021993",
    "Kaja Foglio",
);

// 6ED 316 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "bca1df90-d51a-4d2f-a03f-a4591031671d",
    "Donato Giancola",
);

// 6ED 317 — Wand of Denial (reprint)
const WAND_OF_DENIAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vis::WAND_OF_DENIAL,
    "f6f37dd8-87d6-432f-a60d-a7a699da3080",
    "Steve Luke",
);

// 6ED 318 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "ef706d86-6e7f-4f3a-9e4d-8aa6d9aac74a",
    "Donato Giancola",
);

// 6ED 319 — Adarkar Wastes (reprint)
const ADARKAR_WASTES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::ADARKAR_WASTES,
    "764eff32-466f-4443-a3db-1007f446980b",
    "Gary Leach",
);

// 6ED 320 — Brushland (reprint)
const BRUSHLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::BRUSHLAND,
    "792f4213-67ab-41d9-975d-ff783668f93d",
    "Tom Wänerstrand",
);

// 6ED 321 — City of Brass (reprint)
const CITY_OF_BRASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::CITY_OF_BRASS,
    "81ff64e0-e7bd-4e82-8495-2cc8889c4107",
    "Tom Wänerstrand",
);

// 6ED 322 — Crystal Vein (reprint)
const CRYSTAL_VEIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::CRYSTAL_VEIN,
    "2a807243-fdcd-4dfb-b9d1-a15859a15c51",
    "Pat Lewis",
);

// 6ED 323 — Dwarven Ruins (reprint)
const DWARVEN_RUINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::DWARVEN_RUINS,
    "840627fa-dec9-481e-b356-56f3d079a60c",
    "Liz Danforth",
);

// 6ED 324 — Ebon Stronghold (reprint)
const EBON_STRONGHOLD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::EBON_STRONGHOLD,
    "160b386a-d148-41c8-b540-5637cc0eb458",
    "Liz Danforth",
);

// 6ED 325 — Havenwood Battleground (reprint)
const HAVENWOOD_BATTLEGROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::HAVENWOOD_BATTLEGROUND,
    "ad2f7ee8-bd9b-40a7-9ae2-268b4b9f8315",
    "Liz Danforth",
);

// 6ED 326 — Karplusan Forest (reprint)
const KARPLUSAN_FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::KARPLUSAN_FOREST,
    "ee4e8e26-3fb3-4388-bc64-fa751e401dca",
    "Randy Gallegos",
);

// 6ED 327 — Ruins of Trokair (reprint)
const RUINS_OF_TROKAIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::RUINS_OF_TROKAIR,
    "86ecaeb7-7061-4e8c-a086-32a12b6e2666",
    "Liz Danforth",
);

// 6ED 328 — Sulfurous Springs (reprint)
const SULFUROUS_SPRINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::SULFUROUS_SPRINGS,
    "53e40dcb-fde5-4eb0-9e2b-6109132332a8",
    "Jeff Miracola",
);

// 6ED 329 — Svyelunite Temple (reprint)
const SVYELUNITE_TEMPLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fem::SVYELUNITE_TEMPLE,
    "8d57618a-21d2-4ded-9f17-d72786869b19",
    "Liz Danforth",
);

// 6ED 330 — Underground River (reprint)
const UNDERGROUND_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::UNDERGROUND_RIVER,
    "142dd41a-0e98-41d7-9de9-63cda1cf8915",
    "Jeff Miracola",
);

// 6ED 331 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "d3c1ab59-3771-484c-8a79-2bb36655b6dd",
    "Tom Wänerstrand",
);

// 6ED 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "c789adc0-69ed-4d1c-8a20-ca219020fd2c",
    "Tom Wänerstrand",
);

// 6ED 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "6addf844-2790-4da5-a22d-18df878f2219",
    "Douglas Shuler",
);

// 6ED 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "7978fa58-0860-4082-a674-2d6abce899b9",
    "Fred Fields",
);

// 6ED 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "b3f9ffc8-8115-4858-bd71-deaca9889f72",
    "Douglas Shuler",
);

// 6ED 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "f5e8da7b-f565-403b-9603-5f53cd3be8fe",
    "J. W. Frost",
);

// 6ED 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "4e8958d4-067b-4815-a128-5d1577b94173",
    "John Avon",
);

// 6ED 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "64ff74a8-e1e8-41ec-98fc-1529f4075074",
    "Eric Peterson",
);

// 6ED 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "0e2a9fa6-3c7c-409a-8565-e3d533107971",
    "Romas Kukalis",
);

// 6ED 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "92af0cba-c058-4833-a74a-21ad91e5ad7c",
    "Dan Frazier",
);

// 6ED 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "9019a639-929a-40d9-98b1-6b5268da5246",
    "Douglas Shuler",
);

// 6ED 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "c576c27d-f361-4448-8607-0f6a99642283",
    "Romas Kukalis",
);

// 6ED 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "f2e450ae-3ed8-424f-b974-7d9a8e8ac7e4",
    "John Avon",
);

// 6ED 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "50e7681b-37e3-4f6c-8415-ab9613446ccc",
    "John Avon",
);

// 6ED 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "a5c17b3c-6f3b-4952-a06b-0df5a5a18f0d",
    "John Avon",
);

// 6ED 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "95585c69-6474-432e-b80a-489a2b69d4e6",
    "Brian Durfee",
);

// 6ED 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "dc477b44-a7b3-4dda-b897-6f164e28541b",
    "Quinton Hoover",
);

// 6ED 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "451c256a-1f0e-4e57-9667-5b8cb2f18ff0",
    "Quinton Hoover",
);

// 6ED 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "7ade2cdc-9625-427b-92b2-9596ac48904b",
    "John Avon",
);

// 6ED 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "4e57a496-09ce-4c2d-9a00-0c359d01e78e",
    "John Avon",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANIMATE_WALL_REPRINT,
    ARCHANGEL_REPRINT,
    ARDENT_MILITIA_REPRINT,
    ARMAGEDDON_REPRINT,
    ARMORED_PEGASUS_REPRINT,
    CASTLE_REPRINT,
    CELESTIAL_DAWN_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    CRUSADE_REPRINT,
    DARAJA_GRIFFIN_REPRINT,
    DAVENANT_ARCHER_REPRINT,
    DISENCHANT_REPRINT,
    DIVINE_TRANSFORMATION_REPRINT,
    EKUNDU_GRIFFIN_REPRINT,
    ENLIGHTENED_TUTOR_REPRINT,
    ETHEREAL_CHAMPION_REPRINT,
    EXILE_REPRINT,
    HEALING_SALVE_REPRINT,
    HEAVY_BALLISTA_REPRINT,
    HERO_S_RESOLVE_REPRINT,
    ICATIAN_TOWN_REPRINT,
    INFANTRY_VETERAN_REPRINT,
    KISMET_REPRINT,
    KJELDORAN_ROYAL_GUARD_REPRINT,
    LIGHT_OF_DAY_REPRINT,
    LONGBOW_ARCHER_REPRINT,
    MESA_FALCON_REPRINT,
    ORDER_OF_THE_SACRED_TORCH_REPRINT,
    PACIFISM_REPRINT,
    PEARL_DRAGON_REPRINT,
    REGAL_UNICORN_REPRINT,
    REMEDY_REPRINT,
    REPRISAL_REPRINT,
    RESISTANCE_FIGHTER_REPRINT,
    REVERSE_DAMAGE_REPRINT,
    SAMITE_HEALER_REPRINT,
    SERENITY_REPRINT,
    SERRA_S_BLESSING_REPRINT,
    SPIRIT_LINK_REPRINT,
    STANDING_TROOPS_REPRINT,
    STAUNCH_DEFENDERS_REPRINT,
    SUNWEB_REPRINT,
    TARIFF_REPRINT,
    TUNDRA_WOLVES_REPRINT,
    UNYARO_GRIFFIN_REPRINT,
    VENERABLE_MONK_REPRINT,
    WALL_OF_SWORDS_REPRINT,
    WARMTH_REPRINT,
    WARRIOR_S_HONOR_REPRINT,
    WRATH_OF_GOD_REPRINT,
    ABDUCTION_REPRINT,
    AIR_ELEMENTAL_REPRINT,
    ANCESTRAL_MEMORIES_REPRINT,
    BOOMERANG_REPRINT,
    BROWSE_REPRINT,
    CHILL_REPRINT,
    COUNTERSPELL_REPRINT,
    DARING_APPRENTICE_REPRINT,
    DEFLECTION_REPRINT,
    DESERTION_REPRINT,
    DIMINISHING_RETURNS_REPRINT,
    DREAM_CACHE_REPRINT,
    FLASH_REPRINT,
    FLIGHT_REPRINT,
    FOG_ELEMENTAL_REPRINT,
    FORGET_REPRINT,
    GASEOUS_FORM_REPRINT,
    GLACIAL_WALL_REPRINT,
    HARMATTAN_EFREET_REPRINT,
    HORNED_TURTLE_REPRINT,
    INSIGHT_REPRINT,
    INSPIRATION_REPRINT,
    JUXTAPOSE_REPRINT,
    LIBRARY_OF_LAT_NAM_REPRINT,
    LORD_OF_ATLANTIS_REPRINT,
    MANA_SHORT_REPRINT,
    MEMORY_LAPSE_REPRINT,
    MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT,
    MYSTICAL_TUTOR_REPRINT,
    PHANTASMAL_TERRAIN_REPRINT,
    PHANTOM_WARRIOR_REPRINT,
    POLYMORPH_REPRINT,
    POWER_SINK_REPRINT,
    PRODIGAL_SORCERER_REPRINT,
    PROSPERITY_REPRINT,
    PSYCHIC_TRANSFER_REPRINT,
    PSYCHIC_VENOM_REPRINT,
    RECALL_REPRINT,
    RELEARN_REPRINT,
    REMOVE_SOUL_REPRINT,
    SAGE_OWL_REPRINT,
    SEA_MONSTER_REPRINT,
    SEGOVIAN_LEVIATHAN_REPRINT,
    SIBILANT_SPIRIT_REPRINT,
    SOLDEVI_SAGE_REPRINT,
    SPELL_BLAST_REPRINT,
    STORM_CROW_REPRINT,
    TIDAL_SURGE_REPRINT,
    UNSUMMON_REPRINT,
    VODALIAN_SOLDIERS_REPRINT,
    WALL_OF_AIR_REPRINT,
    WIND_DRAKE_REPRINT,
    WIND_SPIRIT_REPRINT,
    ZUR_S_WEIRDING_REPRINT,
    ABYSSAL_HUNTER_REPRINT,
    ABYSSAL_SPECTER_REPRINT,
    AGONIZING_MEMORIES_REPRINT,
    ASHEN_POWDER_REPRINT,
    BLIGHT_REPRINT,
    BLIGHTED_SHAMAN_REPRINT,
    BLOOD_PET_REPRINT,
    BOG_IMP_REPRINT,
    BOG_RATS_REPRINT,
    BOG_WRAITH_REPRINT,
    COERCION_REPRINT,
    DERELOR_REPRINT,
    DOOMSDAY_REPRINT,
    DREAD_OF_NIGHT_REPRINT,
    DRUDGE_SKELETONS_REPRINT,
    DRUDGE_SKELETONS_ALTERNATE_1,
    DRY_SPELL_REPRINT,
    ENFEEBLEMENT_REPRINT,
    EVIL_EYE_OF_ORMS_BY_GORE_REPRINT,
    FALLEN_ANGEL_REPRINT,
    FATAL_BLOW_REPRINT,
    FEAR_REPRINT,
    FEAST_OF_THE_UNICORN_REPRINT,
    FERAL_SHADOW_REPRINT,
    FORBIDDEN_CRYPT_REPRINT,
    GRAVEBANE_ZOMBIE_REPRINT,
    GRAVEDIGGER_REPRINT,
    GREED_REPRINT,
    HECATOMB_REPRINT,
    HIDDEN_HORROR_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    INFERNAL_CONTRACT_REPRINT,
    KJELDORAN_DEAD_REPRINT,
    LESHRAC_S_RITE_REPRINT,
    LOST_SOUL_REPRINT,
    MIND_WARP_REPRINT,
    MISCHIEVOUS_POLTERGEIST_REPRINT,
    NECROSAVANT_REPRINT,
    NIGHTMARE_REPRINT,
    PAINFUL_MEMORIES_REPRINT,
    PERISH_REPRINT,
    PESTILENCE_REPRINT,
    PYTHON_REPRINT,
    RAG_MAN_REPRINT,
    RAISE_DEAD_REPRINT,
    RAZORTOOTH_RATS_REPRINT,
    SCATHE_ZOMBIES_REPRINT,
    SENGIR_AUTOCRAT_REPRINT,
    STRANDS_OF_NIGHT_REPRINT,
    STROMGALD_CABAL_REPRINT,
    STUPOR_REPRINT,
    SYPHON_SOUL_REPRINT,
    TERROR_REPRINT,
    VAMPIRIC_TUTOR_REPRINT,
    ZOMBIE_MASTER_REPRINT,
    AETHER_FLASH_REPRINT,
    ANABA_BODYGUARD_REPRINT,
    ANABA_SHAMAN_REPRINT,
    BALDUVIAN_BARBARIANS_REPRINT,
    BALDUVIAN_HORDE_REPRINT,
    BLAZE_REPRINT,
    BOIL_REPRINT,
    BURROWING_REPRINT,
    CONQUER_REPRINT,
    CRIMSON_HELLKITE_REPRINT,
    EARTHQUAKE_REPRINT,
    FERVOR_REPRINT,
    FINAL_FORTUNE_REPRINT,
    FIRE_ELEMENTAL_REPRINT,
    FIREBREATHING_REPRINT,
    FIT_OF_RAGE_REPRINT,
    FLAME_SPIRIT_REPRINT,
    FLASHFIRES_REPRINT,
    GIANT_STRENGTH_REPRINT,
    GOBLIN_DIGGING_TEAM_REPRINT,
    GOBLIN_ELITE_INFANTRY_REPRINT,
    GOBLIN_HERO_REPRINT,
    GOBLIN_KING_REPRINT,
    GOBLIN_RECRUITER_REPRINT,
    GOBLIN_WARRENS_REPRINT,
    HAMMER_OF_BOGARDAN_REPRINT,
    HULKING_CYCLOPS_REPRINT,
    ILLICIT_AUCTION_REPRINT,
    INFERNO_REPRINT,
    JOKULHAUPS_REPRINT,
    LIGHTNING_BLAST_REPRINT,
    MANABARBS_REPRINT,
    MOUNTAIN_GOAT_REPRINT,
    ORCISH_ARTILLERY_REPRINT,
    ORCISH_ORIFLAMME_REPRINT,
    PILLAGE_REPRINT,
    PYROTECHNICS_REPRINT,
    RAGING_GOBLIN_REPRINT,
    RECKLESS_EMBERMAGE_REPRINT,
    RELENTLESS_ASSAULT_REPRINT,
    SABRETOOTH_TIGER_REPRINT,
    SHATTER_REPRINT,
    SHATTERSTORM_REPRINT,
    SHOCK_REPRINT,
    SPITTING_DRAKE_REPRINT,
    SPITTING_EARTH_REPRINT,
    STONE_RAIN_REPRINT,
    TALRUUM_MINOTAUR_REPRINT,
    TREMOR_REPRINT,
    VERTIGO_REPRINT,
    VIASHINO_WARRIOR_REPRINT,
    VOLCANIC_DRAGON_REPRINT,
    VOLCANIC_GEYSER_REPRINT,
    WALL_OF_FIRE_REPRINT,
    BIRDS_OF_PARADISE_REPRINT,
    CALL_OF_THE_WILD_REPRINT,
    CAT_WARRIORS_REPRINT,
    CREEPING_MOLD_REPRINT,
    DENSE_FOLIAGE_REPRINT,
    EARLY_HARVEST_REPRINT,
    ELDER_DRUID_REPRINT,
    ELVEN_CACHE_REPRINT,
    ELVEN_RIDERS_REPRINT,
    ELVISH_ARCHERS_REPRINT,
    FALLOW_EARTH_REPRINT,
    FAMILIAR_GROUND_REPRINT,
    FEMEREF_ARCHERS_REPRINT,
    FOG_REPRINT,
    FYNDHORN_BROWNIE_REPRINT,
    FYNDHORN_ELDER_REPRINT,
    GIANT_GROWTH_REPRINT,
    GIANT_SPIDER_REPRINT,
    GORILLA_CHIEFTAIN_REPRINT,
    GRIZZLY_BEARS_REPRINT,
    HURRICANE_REPRINT,
    LIVING_LANDS_REPRINT,
    LLANOWAR_ELVES_REPRINT,
    LURE_REPRINT,
    MARO_REPRINT,
    NATURE_S_RESURGENCE_REPRINT,
    PANTHER_WARRIORS_REPRINT,
    PRADESH_GYPSIES_REPRINT,
    RADJAN_SPIRIT_REPRINT,
    RAMPANT_GROWTH_REPRINT,
    REDWOOD_TREEFOLK_REPRINT,
    REGENERATION_REPRINT,
    RIVER_BOA_REPRINT,
    ROWEN_REPRINT,
    SCALED_WURM_REPRINT,
    SHANODIN_DRYADS_REPRINT,
    STALKING_TIGER_REPRINT,
    STREAM_OF_LIFE_REPRINT,
    SUMMER_BLOOM_REPRINT,
    THICKET_BASILISK_REPRINT,
    TRAINED_ARMODON_REPRINT,
    TRANQUIL_GROVE_REPRINT,
    TRANQUILITY_REPRINT,
    UKTABI_ORANGUTAN_REPRINT,
    UKTABI_WILDCATS_REPRINT,
    UNSEEN_WALKER_REPRINT,
    UNTAMED_WILDS_REPRINT,
    VERDURAN_ENCHANTRESS_REPRINT,
    VITALIZE_REPRINT,
    WAITING_IN_THE_WEEDS_REPRINT,
    WARTHOG_REPRINT,
    WILD_GROWTH_REPRINT,
    WORLDLY_TUTOR_REPRINT,
    WYLULI_WOLF_REPRINT,
    ALADDINS_RING_REPRINT,
    AMBER_PRISON_REPRINT,
    ANKH_OF_MISHRA_REPRINT,
    ASHNOD_S_ALTAR_REPRINT,
    BOTTLE_OF_SULEIMAN_REPRINT,
    CHARCOAL_DIAMOND_REPRINT,
    CRYSTAL_ROD_REPRINT,
    CURSED_TOTEM_REPRINT,
    DANCING_SCIMITAR_REPRINT,
    DINGUS_EGG_REPRINT,
    DISRUPTING_SCEPTER_REPRINT,
    DRAGON_ENGINE_REPRINT,
    DRAGON_MASK_REPRINT,
    FIRE_DIAMOND_REPRINT,
    FLYING_CARPET_REPRINT,
    FOUNTAIN_OF_YOUTH_REPRINT,
    GLASSES_OF_URZA_REPRINT,
    GRINNING_TOTEM_REPRINT,
    THE_HIVE_REPRINT,
    HOWLING_MINE_REPRINT,
    IRON_STAR_REPRINT,
    IVORY_CUP_REPRINT,
    JADE_MONOLITH_REPRINT,
    JALUM_TOME_REPRINT,
    JAYEMDAE_TOME_REPRINT,
    LEAD_GOLEM_REPRINT,
    MANA_PRISM_REPRINT,
    MARBLE_DIAMOND_REPRINT,
    MEEKSTONE_REPRINT,
    MILLSTONE_REPRINT,
    MOSS_DIAMOND_REPRINT,
    MYSTIC_COMPASS_REPRINT,
    OBSIANUS_GOLEM_REPRINT,
    ORNITHOPTER_REPRINT,
    PATAGIA_GOLEM_REPRINT,
    PENTAGRAM_OF_THE_AGES_REPRINT,
    PHYREXIAN_VAULT_REPRINT,
    PRIMAL_CLAY_REPRINT,
    ROD_OF_RUIN_REPRINT,
    SKULL_CATAPULT_REPRINT,
    SKY_DIAMOND_REPRINT,
    SNAKE_BASKET_REPRINT,
    SOUL_NET_REPRINT,
    STORM_CAULDRON_REPRINT,
    TEFERI_S_PUZZLE_BOX_REPRINT,
    THRONE_OF_BONE_REPRINT,
    WAND_OF_DENIAL_REPRINT,
    WOODEN_SPHERE_REPRINT,
    ADARKAR_WASTES_REPRINT,
    BRUSHLAND_REPRINT,
    CITY_OF_BRASS_REPRINT,
    CRYSTAL_VEIN_REPRINT,
    DWARVEN_RUINS_REPRINT,
    EBON_STRONGHOLD_REPRINT,
    HAVENWOOD_BATTLEGROUND_REPRINT,
    KARPLUSAN_FOREST_REPRINT,
    RUINS_OF_TROKAIR_REPRINT,
    SULFUROUS_SPRINGS_REPRINT,
    SVYELUNITE_TEMPLE_REPRINT,
    UNDERGROUND_RIVER_REPRINT,
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
