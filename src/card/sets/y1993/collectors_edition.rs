//! Collector's Edition has no unique card definitions.
//!
//! Every card in the built-in Collector's Edition catalog points to its first printing.

use super::{CardRecord, PrintingRecord, alpha, beta};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;

// CED 1 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "fae5748a-40b7-4056-a5c8-241e168870eb",
    "Dan Frazier",
);

// CED 2 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ARMAGEDDON,
    "f03f6223-d383-4d70-8775-a0590e97906b",
    "Jesper Myrfors",
);

// CED 3 — Balance (reprint)
const BALANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BALANCE,
    "6fa2ef4a-197f-4a41-b219-c6187bec6c35",
    "Mark Poole",
);

// CED 4 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "64795574-4984-473a-b47c-afa4fc43dd49",
    "Douglas Shuler",
);

// CED 5 — Black Ward (reprint)
const BLACK_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_WARD,
    "ee17dd73-7d14-444f-842e-dde48611d050",
    "Dan Frazier",
);

// CED 6 — Blaze of Glory (reprint)
const BLAZE_OF_GLORY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLAZE_OF_GLORY,
    "edb6fd83-5fc8-4caf-a2cb-d87ab4b70fe1",
    "Richard Thomas",
);

// CED 7 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLESSING,
    "36a55916-dad2-4fc1-a722-3ce3a90006c1",
    "Julie Baroh",
);

// CED 8 — Blue Ward (reprint)
const BLUE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_WARD,
    "15fb98bf-342b-406b-92cf-bb51e72bc2b7",
    "Dan Frazier",
);

// CED 9 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "bd32f05b-ad68-44b6-b2ab-9ea7300a987a",
    "Dameon Willich",
);

// CED 10 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "2dc0c0ad-75d9-4417-b092-08124418641a",
    "Jesper Myrfors",
);

// CED 11 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "57e9a113-5571-409d-9c03-cc349a376136",
    "Dameon Willich",
);

// CED 12 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "6bad2d4d-631c-4de0-a543-74bb5e5cd9ba",
    "Sandra Everingham",
);

// CED 13 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "1a7c2900-d92b-41dc-8e7d-0a24d1acbd94",
    "Mark Tedin",
);

// CED 14 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "d388e4fb-c5a4-4e72-8266-5bbdd5186ef9",
    "Douglas Shuler",
);

// CED 15 — Consecrate Land (reprint)
const CONSECRATE_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSECRATE_LAND,
    "24678f56-9b46-43ca-879d-a3aba0d96657",
    "Jeff A. Menges",
);

// CED 16 — Conversion (reprint)
const CONVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONVERSION,
    "608d1f8c-6beb-4021-9d2a-c392974bf816",
    "Jesper Myrfors",
);

// CED 17 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CRUSADE,
    "d2d0154e-570d-46b0-96b9-f2a162aeb49d",
    "Mark Poole",
);

// CED 18 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "286acb93-5967-4389-83fd-44bffd136988",
    "Mark Poole",
);

// CED 19 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DISENCHANT,
    "d05e9bcf-e296-4320-891f-77a870fea6c5",
    "Amy Weber",
);

// CED 20 — Farmstead (reprint)
const FARMSTEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FARMSTEAD,
    "34434f68-13eb-408c-aea0-e992cbc06e87",
    "Mark Poole",
);

// CED 21 — Green Ward (reprint)
const GREEN_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GREEN_WARD,
    "e42cbf92-2c44-4534-bc35-97fdf7905760",
    "Dan Frazier",
);

// CED 22 — Guardian Angel (reprint)
const GUARDIAN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GUARDIAN_ANGEL,
    "e7bf120c-c229-4c97-8d88-09d8a6c7ba78",
    "Anson Maddocks",
);

// CED 23 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "52de6628-8e88-4309-9837-0b2f30c3b497",
    "Dan Frazier",
);

// CED 24 — Holy Armor (reprint)
const HOLY_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_ARMOR,
    "250d5927-0890-423b-afbc-a8f5628de793",
    "Melissa A. Benson",
);

// CED 25 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "9b8e984a-c533-4d4c-a0b9-6eab5bb99ef6",
    "Anson Maddocks",
);

// CED 26 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "f0ad4c72-3645-4eeb-8bf9-aeb1f0355119",
    "Mark Poole",
);

// CED 27 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "1ee2fee7-0685-479c-b82b-28f87b79d8cd",
    "Richard Thomas",
);

// CED 28 — Lance (reprint)
const LANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LANCE,
    "5f236343-5a49-4d55-8bf5-ffe9a58fab83",
    "Rob Alexander",
);

// CED 29 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "834e6d6e-63b3-4dc9-a9ce-253fd5cb7069",
    "Melissa A. Benson",
);

// CED 30 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "075e48f1-3c95-42c4-b90d-55dc6738435d",
    "Douglas Shuler",
);

// CED 31 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "1f17dc6b-2b90-4290-81cd-1418f12061dc",
    "Cornelius Brudi",
);

// CED 32 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "b0c11647-bbfd-4f5e-bbec-40b8e506a6e8",
    "Kev Brockschmidt",
);

// CED 33 — Purelace (reprint)
const PURELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PURELACE,
    "655a0608-34e7-467c-9631-80d9609cd681",
    "Sandra Everingham",
);

// CED 34 — Red Ward (reprint)
const RED_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_WARD,
    "f259bc90-463d-4ad5-b803-c478ef6b3172",
    "Dan Frazier",
);

// CED 35 — Resurrection (reprint)
const RESURRECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RESURRECTION,
    "8c2171d6-8c66-4335-aa80-2cbd1f496fbf",
    "Dan Frazier",
);

// CED 36 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "575b295f-94b2-4d7a-af8a-01aea4dd0e05",
    "Dameon Willich",
);

// CED 37 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "fa620a45-4b94-42a7-9e40-df51e6115515",
    "Douglas Shuler",
);

// CED 38 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "5398a0ac-4fde-4513-b2ce-03440b79d7d4",
    "Tom Wänerstrand",
);

// CED 39 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH_LIONS,
    "c174dee3-b68d-4887-96b2-b748f99653b0",
    "Daniel Gelon",
);

// CED 40 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SERRA_ANGEL,
    "ad0ba1e0-3c73-4173-8323-ac4c2eddcc5d",
    "Douglas Shuler",
);

// CED 41 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWORDS_TO_PLOWSHARES,
    "57253586-f81b-4f68-9237-8cb22c034f18",
    "Jeff A. Menges",
);

// CED 42 — Veteran Bodyguard (reprint)
const VETERAN_BODYGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VETERAN_BODYGUARD,
    "715f6b5d-3b9b-4fb8-82ce-48e97d475c5a",
    "Douglas Shuler",
);

// CED 43 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "e619b233-4a58-46c0-9876-9f7b3301d09d",
    "Mark Tedin",
);

// CED 44 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHITE_KNIGHT,
    "d90ff1e9-4712-4bbc-81a6-56d405e5d205",
    "Daniel Gelon",
);

// CED 45 — White Ward (reprint)
const WHITE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_WARD,
    "f2227b56-ea82-4250-9c2b-0517d5da248b",
    "Dan Frazier",
);

// CED 46 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WRATH_OF_GOD,
    "0d317a92-8bc5-4d1a-a817-699724674122",
    "Quinton Hoover",
);

// CED 47 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "ed4706ba-5387-4ac9-b939-15b5d2e4880a",
    "Richard Thomas",
);

// CED 48 — Ancestral Recall (reprint)
const ANCESTRAL_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANCESTRAL_RECALL,
    "74aa5657-258c-42b6-bedd-f259160dae46",
    "Mark Poole",
);

// CED 49 — Animate Artifact (reprint)
const ANIMATE_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANIMATE_ARTIFACT,
    "9a4f8c45-78a7-4161-9e11-f458a91cc04c",
    "Douglas Shuler",
);

// CED 50 — Blue Elemental Blast (reprint)
const BLUE_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLUE_ELEMENTAL_BLAST,
    "0cb1997d-b036-48c4-beca-59a901e155ae",
    "Richard Thomas",
);

// CED 51 — Braingeyser (reprint)
const BRAINGEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BRAINGEYSER,
    "dc449e2c-e5b4-4893-8da5-689f2353cc4e",
    "Mark Tedin",
);

// CED 52 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLONE,
    "7a20fe69-5265-46c1-bcd2-eccbf7085266",
    "Julie Baroh",
);

// CED 53 — Control Magic (reprint)
const CONTROL_MAGIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTROL_MAGIC,
    "18d35a20-b18a-41d8-ac08-b1bdb2663b9d",
    "Dameon Willich",
);

// CED 54 — Copy Artifact (reprint)
const COPY_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPY_ARTIFACT,
    "a8316b75-5962-4d24-a9b9-2a5b9f4b90f8",
    "Amy Weber",
);

// CED 55 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COUNTERSPELL,
    "901efc0f-b444-41bf-ab55-2a2860aa4a52",
    "Mark Poole",
);

// CED 56 — Creature Bond (reprint)
const CREATURE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CREATURE_BOND,
    "6808b1a2-034e-4b2e-8f22-77d38bfc33a4",
    "Anson Maddocks",
);

// CED 57 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "04b6add2-5f2b-4101-b371-6d6033db92fd",
    "Douglas Shuler",
);

// CED 58 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "50b6643c-0641-4543-b9af-1853c588fcb1",
    "Quinton Hoover",
);

// CED 59 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "124f8cc0-265b-49b9-a6f4-42272e9baa6b",
    "Anson Maddocks",
);

// CED 60 — Invisibility (reprint)
const INVISIBILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INVISIBILITY,
    "2a8d1b78-9b5d-4cd0-a334-40bd635514e0",
    "Anson Maddocks",
);

// CED 61 — Jump (reprint)
const JUMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUMP,
    "34c7db89-8409-4acc-8fb8-6a788b517d05",
    "Mark Poole",
);

// CED 62 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "5ca2ebe7-3e53-4cd0-8d75-2b6395612756",
    "Anson Maddocks",
);

// CED 63 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "3d51460c-a012-434d-af2e-e9a0983d4500",
    "Melissa A. Benson",
);

// CED 64 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "15016e8e-2f6b-4470-865a-ec13da3cb968",
    "Julie Baroh",
);

// CED 65 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "d13805ed-0a40-4df8-9c35-0677e42b7cc7",
    "Dan Frazier",
);

// CED 66 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_SHORT,
    "541f4e9a-b7dd-4c1b-b313-34c95bfef158",
    "Dameon Willich",
);

// CED 67 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "c63dd460-ba74-475d-ae48-94b4d46ca589",
    "Jeff A. Menges",
);

// CED 68 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "9c6b58d5-9751-4c8f-8281-a6e7e1eb6a5e",
    "Mark Poole",
);

// CED 69 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "a56ae7d9-b5f7-40d4-843b-089fa2f8788c",
    "Dameon Willich",
);

// CED 70 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "d739326e-5bae-4b0f-add5-1ee37d86a480",
    "Jesper Myrfors",
);

// CED 71 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "f8d967ce-c8f3-40af-8bb1-a498b87de905",
    "Tom Wänerstrand",
);

// CED 72 — Power Leak (reprint)
const POWER_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_LEAK,
    "ec9301c7-142b-4e48-8345-dc9ac3ca2f5b",
    "Drew Tucker",
);

// CED 73 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "f2eb7217-f3cf-41c5-b51c-db25e89e8068",
    "Richard Thomas",
);

// CED 74 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "17c0b205-075c-4aab-8666-227a2ed3853e",
    "Douglas Shuler",
);

// CED 75 — Psionic Blast (reprint)
const PSIONIC_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PSIONIC_BLAST,
    "3feb4eb4-b677-444a-99fe-096528c4b20c",
    "Douglas Shuler",
);

// CED 76 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "0b95d1f0-bfcc-4171-986a-ccc1ac68ec43",
    "Brian Snõddy",
);

// CED 77 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "b0152ea5-ac98-4a05-a9db-65384e1ce562",
    "Jeff A. Menges",
);

// CED 78 — Siren's Call (reprint)
const SIREN_S_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIREN_S_CALL,
    "000809e6-2dd5-41cc-a316-3edb4e40eb58",
    "Anson Maddocks",
);

// CED 79 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "438d80aa-2bec-41f2-b50f-b1768421a1a0",
    "Mark Poole",
);

// CED 80 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "d7155fdc-9f92-4fd7-a2e1-7c63c72346e6",
    "Brian Snõddy",
);

// CED 81 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STASIS,
    "79df9205-92ea-4f7d-b5aa-c375bdc4d41b",
    "Fay Jones",
);

// CED 82 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "9aa74ee4-ac52-4bda-9be5-26f834879c83",
    "Amy Weber",
);

// CED 83 — Thoughtlace (reprint)
const THOUGHTLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THOUGHTLACE,
    "24d33360-0a38-4ee0-b716-dd739ecccdaf",
    "Mark Poole",
);

// CED 84 — Time Walk (reprint)
const TIME_WALK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_WALK,
    "c0d60dcd-fc7e-4bda-87c5-5e7745054a10",
    "Amy Weber",
);

// CED 85 — Timetwister (reprint)
const TIMETWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIMETWISTER,
    "14eee1b5-d41a-44b6-af2f-d367fd606ef1",
    "Mark Tedin",
);

// CED 86 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "1994e753-799c-42c0-b331-39a78cfb8ee9",
    "Rob Alexander",
);

// CED 87 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "42d09ab8-6696-47d5-9c0f-409bcd7b04e1",
    "Douglas Shuler",
);

// CED 88 — Vesuvan Doppelganger (reprint)
const VESUVAN_DOPPELGANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VESUVAN_DOPPELGANGER,
    "8f26b6c8-fd51-460a-b13f-63db7f4b4b9d",
    "Quinton Hoover",
);

// CED 89 — Volcanic Eruption (reprint)
const VOLCANIC_ERUPTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VOLCANIC_ERUPTION,
    "04961291-2659-421e-891f-a0ee4f02d246",
    "Douglas Shuler",
);

// CED 90 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "7ef4df91-0d98-49a5-9cc4-54664bca8a77",
    "Richard Thomas",
);

// CED 91 — Wall of Water (reprint)
const WALL_OF_WATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WATER,
    "e4aa3095-2ca6-4936-b81d-0c5fe723d712",
    "Richard Thomas",
);

// CED 92 — Water Elemental (reprint)
const WATER_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WATER_ELEMENTAL,
    "ea7070a5-048d-47e6-a924-124fc3b0cb5f",
    "Jeff A. Menges",
);

// CED 93 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "74e6ac33-c073-4798-86e7-929daebf5692",
    "Anson Maddocks",
);

// CED 94 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "7f5eb613-dedc-4b0f-a5f2-32a3821ad923",
    "Jesper Myrfors",
);

// CED 95 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_KNIGHT,
    "4608647b-63c3-47c5-92d9-6e235ee2713c",
    "Jeff A. Menges",
);

// CED 96 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "0dbd4d77-a803-44c6-825e-b6e22e0d383b",
    "Jeff A. Menges",
);

// CED 97 — Contract from Below (reprint)
const CONTRACT_FROM_BELOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTRACT_FROM_BELOW,
    "dcb020d3-c458-4893-a11b-f0ddfeb15339",
    "Douglas Shuler",
);

// CED 98 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "2b919476-a031-442b-a55b-9aa10c3f9e51",
    "Jesper Myrfors",
);

// CED 99 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DARK_RITUAL,
    "06df3f4b-3d94-4792-9460-485d2230c0e0",
    "Sandra Everingham",
);

// CED 100 — Darkpact (reprint)
const DARKPACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARKPACT,
    "3d92646b-e780-483d-a1c2-c0da12c08b9b",
    "Quinton Hoover",
);

// CED 101 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "2aee3cd8-c1ac-4316-935b-b04e58f43204",
    "Anson Maddocks",
);

// CED 102 — Deathlace (reprint)
const DEATHLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHLACE,
    "2351011c-aab3-4029-9a6c-0bcc0cb00026",
    "Sandra Everingham",
);

// CED 103 — Demonic Attorney (reprint)
const DEMONIC_ATTORNEY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_ATTORNEY,
    "32bcf279-2e91-4d9d-9849-d5a3aa7cafbc",
    "Daniel Gelon",
);

// CED 104 — Demonic Hordes (reprint)
const DEMONIC_HORDES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_HORDES,
    "cde240a1-9911-4b81-b1e4-59a56abbbcfb",
    "Jesper Myrfors",
);

// CED 105 — Demonic Tutor (reprint)
const DEMONIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DEMONIC_TUTOR,
    "24b25c06-d320-4b2f-bd35-bc650f620e0a",
    "Douglas Shuler",
);

// CED 106 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAIN_LIFE,
    "b8e19ebf-a1da-4af5-8948-a199a05283f6",
    "Douglas Shuler",
);

// CED 107 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "d69eb291-f47f-438e-9fd4-d86f2b224221",
    "Sandra Everingham",
);

// CED 108 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "ff302e11-9e81-497a-aa55-ae5b55879e89",
    "Sandra Everingham",
);

// CED 109 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "e7b266c3-6f0f-45dd-b9e5-57036f6e0e4a",
    "Mark Poole",
);

// CED 110 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "5339444a-4149-42df-8453-062b61514de5",
    "Douglas Shuler",
);

// CED 111 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "3444ab52-35fb-466f-a217-0202670cc2ad",
    "Dan Frazier",
);

// CED 112 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "643e14d6-aef8-4561-bac0-3e67b9d6f0a9",
    "Mark Poole",
);

// CED 113 — Hypnotic Specter (reprint)
const HYPNOTIC_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::HYPNOTIC_SPECTER,
    "846373eb-767e-40ea-963a-30b21ced81c1",
    "Douglas Shuler",
);

// CED 114 — Lich (reprint)
const LICH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LICH,
    "f09d7da7-3112-44ec-9be5-11ee4db12646",
    "Daniel Gelon",
);

// CED 115 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "37cd8cd3-c837-4225-8a6f-8c732f3d07ea",
    "Mark Tedin",
);

// CED 116 — Mind Twist (reprint)
const MIND_TWIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MIND_TWIST,
    "a023cdb9-3a3b-4e2a-ae49-ab901de6940e",
    "Julie Baroh",
);

// CED 117 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "4cf7caec-56a7-4815-a297-b0393a9daf80",
    "Christopher Rush",
);

// CED 118 — Nettling Imp (reprint)
const NETTLING_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETTLING_IMP,
    "2c747289-a878-4ab6-8359-dfb87b44a1ab",
    "Quinton Hoover",
);

// CED 119 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "143eecbb-4434-483c-81e1-4b8690b1cd70",
    "Melissa A. Benson",
);

// CED 120 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "712f1f26-5819-4445-b46a-2fe5a2f113be",
    "Anson Maddocks",
);

// CED 121 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "e4467216-6ac5-4bfc-bd1a-f88ea595ae05",
    "Jesper Myrfors",
);

// CED 122 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "0352db2d-6581-4a58-9e6c-bfe1b841959f",
    "Anson Maddocks",
);

// CED 123 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "168918a6-a8b8-4cc3-89a2-22e22476c488",
    "Jeff A. Menges",
);

// CED 124 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROYAL_ASSASSIN,
    "0fb45731-e72c-4bce-ab11-e7e4119a690b",
    "Tom Wänerstrand",
);

// CED 125 — Sacrifice (reprint)
const SACRIFICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SACRIFICE,
    "ef3616ba-c95f-4650-9643-1c5f27be1a89",
    "Dan Frazier",
);

// CED 126 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "8da041e8-9c22-420c-a8bf-7f64346a8dc7",
    "Jesper Myrfors",
);

// CED 127 — Scavenging Ghoul (reprint)
const SCAVENGING_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCAVENGING_GHOUL,
    "5d5a4c95-e231-4a7c-b9ec-929a5529b9be",
    "Jeff A. Menges",
);

// CED 128 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SENGIR_VAMPIRE,
    "3dec1c4d-0d81-4aee-91ca-a0f3aa2e2a3b",
    "Anson Maddocks",
);

// CED 129 — Simulacrum (reprint)
const SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIMULACRUM,
    "dde852ce-b67a-4642-8656-9c4d5cf02866",
    "Mark Poole",
);

// CED 130 — Sinkhole (reprint)
const SINKHOLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SINKHOLE,
    "d563b299-8809-4bfd-bd83-35f0fc0363b1",
    "Sandra Everingham",
);

// CED 131 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TERROR,
    "0296af5c-e033-47f6-9b95-ee8fef7ac22a",
    "Ron Spencer",
);

// CED 132 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "1767117a-5127-4252-ade0-940e2d0eb54c",
    "Douglas Shuler",
);

// CED 133 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "588f455f-75e4-4f87-8487-5d3d73298618",
    "Anson Maddocks",
);

// CED 134 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "a4c3eb25-45b9-4e63-8150-3d9c208c7d45",
    "Amy Weber",
);

// CED 135 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "489223a4-4aaf-4d8e-9707-03b70430e287",
    "Douglas Shuler",
);

// CED 136 — Will-o'-the-Wisp (reprint)
const WILL_O_THE_WISP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILL_O_THE_WISP,
    "90c12f0b-d1f8-4ec6-a046-d2ee29228529",
    "Jesper Myrfors",
);

// CED 137 — Word of Command (reprint)
const WORD_OF_COMMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WORD_OF_COMMAND,
    "013d5961-e8d8-4ef2-b906-8bad30e4acf8",
    "Jesper Myrfors",
);

// CED 138 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "356a4fba-223a-4375-9acd-b4bd17e4bb09",
    "Jeff A. Menges",
);

// CED 139 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "45891e6e-1711-4b3e-8399-0873df67b874",
    "Mark Poole",
);

// CED 140 — Chaoslace (reprint)
const CHAOSLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHAOSLACE,
    "9793074e-f753-482f-826c-aff6a38facd7",
    "Dameon Willich",
);

// CED 141 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "9ef11af8-3050-4ee7-a287-a53cc9fe38c6",
    "Anson Maddocks",
);

// CED 142 — Dragon Whelp (reprint)
const DRAGON_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAGON_WHELP,
    "ac1d677b-a43f-4aa0-9219-fc8d0a776faa",
    "Amy Weber",
);

// CED 143 — Dwarven Demolition Team (reprint)
const DWARVEN_DEMOLITION_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_DEMOLITION_TEAM,
    "8969c53d-091b-43ef-9c54-19d5b17715cb",
    "Kev Brockschmidt",
);

// CED 144 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "d08ae4fd-bc9a-4574-8ca0-00846b12e2ee",
    "Douglas Shuler",
);

// CED 145 — Earth Elemental (reprint)
const EARTH_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTH_ELEMENTAL,
    "d9d53b5d-2324-4551-ad15-41852d366aee",
    "Dan Frazier",
);

// CED 146 — Earthbind (reprint)
const EARTHBIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHBIND,
    "1a06ef6f-3490-464a-9ddc-5903b6be3080",
    "Quinton Hoover",
);

// CED 147 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::EARTHQUAKE,
    "7db1aa1e-01c1-4df8-a66a-2e783cc12a56",
    "Dan Frazier",
);

// CED 148 — False Orders (reprint)
const FALSE_ORDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FALSE_ORDERS,
    "65defd15-fc99-4359-a222-05aab6dd73a2",
    "Anson Maddocks",
);

// CED 149 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "6e374222-e031-4d59-9502-f97ccd7184b3",
    "Melissa A. Benson",
);

// CED 150 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FIREBALL,
    "21876909-9cf7-4327-a8bd-cbfef4c6b329",
    "Mark Tedin",
);

// CED 151 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "10c4e0c3-72b5-4c53-a6b1-ce3aa500bff7",
    "Dan Frazier",
);

// CED 152 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "35b34674-2ffe-4e09-8928-797d780e5998",
    "Dameon Willich",
);

// CED 153 — Fork (reprint)
const FORK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FORK,
    "ecd55693-e3d8-4d72-a112-7df35392b0cf",
    "Amy Weber",
);

// CED 154 — Goblin Balloon Brigade (reprint)
const GOBLIN_BALLOON_BRIGADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_BALLOON_BRIGADE,
    "7b059fd1-b2e5-4f80-a0c2-4fa394e7a139",
    "Andi Rusu",
);

// CED 155 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_KING,
    "40f89908-8dbe-4bcb-9b94-050b5599926e",
    "Jesper Myrfors",
);

// CED 156 — Granite Gargoyle (reprint)
const GRANITE_GARGOYLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GRANITE_GARGOYLE,
    "228e1936-f83a-47b1-bf0c-40cf1d9d9c4f",
    "Christopher Rush",
);

// CED 157 — Gray Ogre (reprint)
const GRAY_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRAY_OGRE,
    "2d4ea35f-6ea5-40d9-ac96-09c8c10d397e",
    "Dan Frazier",
);

// CED 158 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "de4999c3-1a19-48bd-bb2b-fce1e217be6c",
    "Dan Frazier",
);

// CED 159 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "7e20227e-ef2f-49a2-9b83-2a244ec31a56",
    "Anson Maddocks",
);

// CED 160 — Ironclaw Orcs (reprint)
const IRONCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRONCLAW_ORCS,
    "3bc55aa9-f08a-49be-ad29-11723ce808ab",
    "Anson Maddocks",
);

// CED 161 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "e5d8ea15-d3ba-492a-9b38-58310857a586",
    "Kev Brockschmidt",
);

// CED 162 — Lightning Bolt (reprint)
const LIGHTNING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LIGHTNING_BOLT,
    "1ddc5922-f62a-4f91-809e-f959edfcfc6a",
    "Christopher Rush",
);

// CED 163 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "f0a173dd-5232-4412-a2c1-e3445e79aeaa",
    "Christopher Rush",
);

// CED 164 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "b8f737eb-a2c9-488a-b94b-2b6f33efd1ef",
    "Christopher Rush",
);

// CED 165 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "eb70cd72-88c0-4068-b5de-d8cc594436a1",
    "Jeff A. Menges",
);

// CED 166 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "3c344d03-d609-421d-abd9-45548a720ab0",
    "Anson Maddocks",
);

// CED 167 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "bc87c32b-bdc4-4780-926a-a54fea5937bd",
    "Dan Frazier",
);

// CED 168 — Power Surge (reprint)
const POWER_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SURGE,
    "caa03c3e-d416-4410-bc0a-6dc64f653f76",
    "Douglas Shuler",
);

// CED 169 — Raging River (reprint)
const RAGING_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAGING_RIVER,
    "9c118b97-1e50-4fd1-816d-a63a4128d539",
    "Sandra Everingham",
);

// CED 170 — Red Elemental Blast (reprint)
const RED_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::RED_ELEMENTAL_BLAST,
    "45ea905d-1083-48a4-a479-f2aaa168ed83",
    "Richard Thomas",
);

// CED 171 — Roc of Kher Ridges (reprint)
const ROC_OF_KHER_RIDGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROC_OF_KHER_RIDGES,
    "14dfa42a-39c6-4390-9186-e5fd7cd51145",
    "Andi Rusu",
);

// CED 172 — Rock Hydra (reprint)
const ROCK_HYDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROCK_HYDRA,
    "fbc45b95-c30e-4aa3-8975-f31578c2d9f6",
    "Jeff A. Menges",
);

// CED 173 — Sedge Troll (reprint)
const SEDGE_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SEDGE_TROLL,
    "613ca722-c1f0-441b-a3df-d6fc8dcd1658",
    "Dan Frazier",
);

// CED 174 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SHATTER,
    "16f008fd-8853-45c0-bfb7-fe9433cc07f2",
    "Amy Weber",
);

// CED 175 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "05082d18-ad43-43e0-8d90-973b7959aa6b",
    "Melissa A. Benson",
);

// CED 176 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SMOKE,
    "e02cb7aa-85ce-43e9-b080-b26362d1ac13",
    "Jesper Myrfors",
);

// CED 177 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_GIANT,
    "cf895795-501c-428d-af91-72fbac85d119",
    "Dameon Willich",
);

// CED 178 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_RAIN,
    "dfb2ec72-8638-407d-9801-37a902962d32",
    "Daniel Gelon",
);

// CED 179 — Tunnel (reprint)
const TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNNEL,
    "c578d3d5-c8d1-4c0f-a002-19e5fed21367",
    "Dan Frazier",
);

// CED 180 — Two-Headed Giant of Foriys (reprint)
const TWO_HEADED_GIANT_OF_FORIYS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWO_HEADED_GIANT_OF_FORIYS,
    "4bd36203-c44a-4ca8-83a0-da50f98dbbc1",
    "Anson Maddocks",
);

// CED 181 — Uthden Troll (reprint)
const UTHDEN_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UTHDEN_TROLL,
    "815e87b1-4093-4a1d-98d8-b125ad345910",
    "Douglas Shuler",
);

// CED 182 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "2c69562f-4505-47b4-8286-9c289f655bab",
    "Richard Thomas",
);

// CED 183 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "45c36c70-7bb3-40ec-a5cb-8f98fe741687",
    "Dan Frazier",
);

// CED 184 — Wheel of Fortune (reprint)
const WHEEL_OF_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHEEL_OF_FORTUNE,
    "8faa3dfe-b343-421b-af07-8023436a6ee4",
    "Daniel Gelon",
);

// CED 185 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "9fa5b40c-6619-4868-ba32-9f4aab415e5e",
    "Jeff A. Menges",
);

// CED 186 — Berserk (reprint)
const BERSERK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BERSERK,
    "b8fbd395-514d-4bb2-a0a4-f593517248a5",
    "Dan Frazier",
);

// CED 187 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BIRDS_OF_PARADISE,
    "61016115-d66c-49bb-ae05-1bd855890fe7",
    "Mark Poole",
);

// CED 188 — Camouflage (reprint)
const CAMOUFLAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CAMOUFLAGE,
    "dd606a34-92dc-41eb-a78c-90cd89599e66",
    "Jesper Myrfors",
);

// CED 189 — Channel (reprint)
const CHANNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHANNEL,
    "880235fb-bdc8-4a39-a9ff-f4889a8501e1",
    "Richard Thomas",
);

// CED 190 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "89fb005c-9fcc-490f-8381-f6601806b2cf",
    "Dan Frazier",
);

// CED 191 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "377e9fb9-0f8d-4908-ad00-bc4eb0efa1b4",
    "Daniel Gelon",
);

// CED 192 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "05cd67af-1255-4892-a093-444d3ed03a71",
    "Anson Maddocks",
);

// CED 193 — Fastbond (reprint)
const FASTBOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FASTBOND,
    "e45d289c-3328-44cd-853c-b9dfe21b13a5",
    "Mark Poole",
);

// CED 194 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "fd9df381-c388-46af-803e-3f15dd1f3230",
    "Jesper Myrfors",
);

// CED 195 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "8ad2db02-dabf-47c4-aa9c-d53318033fbe",
    "Douglas Shuler",
);

// CED 196 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "b5d048c2-c3de-4602-9451-52b8aefe55d6",
    "Daniel Gelon",
);

// CED 197 — Gaea's Liege (reprint)
const GAEA_S_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAEA_S_LIEGE,
    "a4abc0bc-6ff3-47ae-8d9d-3de11cfaa9ef",
    "Dameon Willich",
);

// CED 198 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GIANT_GROWTH,
    "933eadc8-b963-4c23-b27d-84f1d84ba2ef",
    "Sandra Everingham",
);

// CED 199 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "83519f51-5c38-45bd-b026-cb83024dc0c4",
    "Sandra Everingham",
);

// CED 200 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "182bd569-647e-4404-b039-b8df6e3bb2ca",
    "Jeff A. Menges",
);

// CED 201 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "59aeeee2-1b8e-4b45-9c27-462d1d7cb72d",
    "Dameon Willich",
);

// CED 202 — Ice Storm (reprint)
const ICE_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ICE_STORM,
    "f01e1506-76d3-4014-a284-119a8eb221ae",
    "Dan Frazier",
);

// CED 203 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "9362f606-fafc-46b9-bf30-87b0799d885a",
    "Dameon Willich",
);

// CED 204 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "3f5d5011-099f-4f2f-b12d-a1260fe5bbde",
    "Jesper Myrfors",
);

// CED 205 — Kudzu (reprint)
const KUDZU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KUDZU,
    "789965c4-f3c8-4ef3-8854-9b4016356d20",
    "Mark Poole",
);

// CED 206 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "20a6839b-3cdb-4344-91f5-9b4b28e12167",
    "Sandra Everingham",
);

// CED 207 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "d41fe009-92b7-4110-8462-7080a99325f9",
    "Dameon Willich",
);

// CED 208 — Lifelace (reprint)
const LIFELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFELACE,
    "cc4b159c-f397-421a-ae18-898b45e84381",
    "Amy Weber",
);

// CED 209 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "7d39be8d-26fe-4598-80d7-4ac73e41f943",
    "Anson Maddocks",
);

// CED 210 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "124b1032-5cbc-477a-b263-7fe0d47431d9",
    "Jesper Myrfors",
);

// CED 211 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LLANOWAR_ELVES,
    "681415ad-2b23-403d-aa34-e9366e91e491",
    "Anson Maddocks",
);

// CED 212 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "716b295f-fcee-4a98-8705-d8c17a88e28d",
    "Anson Maddocks",
);

// CED 213 — Natural Selection (reprint)
const NATURAL_SELECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NATURAL_SELECTION,
    "bf917bae-38b3-4a0f-9f27-d11c903b4cff",
    "Mark Poole",
);

// CED 214 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "4028ed9f-f1f0-4040-a1c7-52889345c4eb",
    "Quinton Hoover",
);

// CED 215 — Regrowth (reprint)
const REGROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::REGROWTH,
    "9f511ddb-61b8-4c07-ba57-1896538d7179",
    "Dameon Willich",
);

// CED 216 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRYB_SPRITES,
    "bf37efeb-c8b5-4cd7-a55b-e570a58cf554",
    "Amy Weber",
);

// CED 217 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "03a2ff31-feb2-40a0-b712-1e180b789d09",
    "Anson Maddocks",
);

// CED 218 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "a996e16f-a655-4fdd-b2b4-28da90d29fcb",
    "Mark Poole",
);

// CED 219 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "01cd8fe8-c4af-4409-ad07-db95f3c28b4d",
    "Dan Frazier",
);

// CED 220 — Timber Wolves (reprint)
const TIMBER_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TIMBER_WOLVES,
    "c88707fa-c7e4-4910-abc1-e7d32f87a120",
    "Melissa A. Benson",
);

// CED 221 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "602497df-3366-4928-99ef-890bda9781ec",
    "Douglas Shuler",
);

// CED 222 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "387f54fd-e295-4d32-a215-a64ddddb7d65",
    "Richard Thomas",
);

// CED 223 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "de372dae-2618-4164-b946-c237c666d41c",
    "Kev Brockschmidt",
);

// CED 224 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "9d3e6cfe-3e31-4b69-ac9a-31d666d1f12f",
    "Anson Maddocks",
);

// CED 225 — Wall of Ice (reprint)
const WALL_OF_ICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_ICE,
    "d406fee6-36dc-4305-8977-b2c6d084634e",
    "Richard Thomas",
);

// CED 226 — Wall of Wood (reprint)
const WALL_OF_WOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WOOD,
    "a58fe572-1115-49cb-b803-c925232ae69b",
    "Mark Tedin",
);

// CED 227 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "8edb061e-c108-43a6-a7e1-2a3413865cf0",
    "Cornelius Brudi",
);

// CED 228 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "2f78b839-e0f8-4779-a559-3d8c6e7eba59",
    "Jeff A. Menges",
);

// CED 229 — Web (reprint)
const WEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEB,
    "861e757c-a859-4fb1-881d-ba24f48dab68",
    "Rob Alexander",
);

// CED 230 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "7b49ffbe-e87f-4a61-a283-2f54b6b19aff",
    "Mark Poole",
);

// CED 231 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANKH_OF_MISHRA,
    "40ac7746-ce02-4720-9f71-e7de9199d12a",
    "Amy Weber",
);

// CED 232 — Basalt Monolith (reprint)
const BASALT_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BASALT_MONOLITH,
    "602512b4-cf48-4acc-9ee8-02b643ba6970",
    "Jesper Myrfors",
);

// CED 233 — Black Lotus (reprint)
const BLACK_LOTUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_LOTUS,
    "0948e6dc-8af7-45d3-91de-a2aebee83e82",
    "Christopher Rush",
);

// CED 234 — Black Vise (reprint)
const BLACK_VISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_VISE,
    "255c93fb-59b3-474f-93c3-bf543b570732",
    "Richard Thomas",
);

// CED 235 — Celestial Prism (reprint)
const CELESTIAL_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CELESTIAL_PRISM,
    "a5003652-0b0c-4d46-b0d6-4be9032019b9",
    "Amy Weber",
);

// CED 236 — Chaos Orb (reprint)
const CHAOS_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHAOS_ORB,
    "878e6fb9-2e8f-46e4-9442-a6bdac41cbde",
    "Mark Tedin",
);

// CED 237 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "2c17d122-5243-49d6-97c1-84d7bb0e0fad",
    "Drew Tucker",
);

// CED 238 — Conservator (reprint)
const CONSERVATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSERVATOR,
    "af638210-0125-4540-9328-c6ae0de580a0",
    "Amy Weber",
);

// CED 239 — Copper Tablet (reprint)
const COPPER_TABLET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPPER_TABLET,
    "847db6fe-2338-4153-b4a5-30a6e7a9eebb",
    "Amy Weber",
);

// CED 240 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "db9c2510-0215-49c4-92f1-a89a55a8552c",
    "Amy Weber",
);

// CED 241 — Cyclopean Tomb (reprint)
const CYCLOPEAN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CYCLOPEAN_TOMB,
    "bdf58b4e-e64c-43bc-8948-865151e62296",
    "Anson Maddocks",
);

// CED 242 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "c8a9a3c7-e469-48df-9dbf-18008dd9f9dd",
    "Dan Frazier",
);

// CED 243 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "686258b5-9684-41ed-95c3-9b40ca743c30",
    "Dan Frazier",
);

// CED 244 — Forcefield (reprint)
const FORCEFIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCEFIELD,
    "ea0fb629-ba9d-44bc-8a18-9f0f114ec910",
    "Dan Frazier",
);

// CED 245 — Gauntlet of Might (reprint)
const GAUNTLET_OF_MIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAUNTLET_OF_MIGHT,
    "c1d71c31-f241-49e8-bbf5-d35cbf46bf67",
    "Christopher Rush",
);

// CED 246 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GLASSES_OF_URZA,
    "bbc27860-05c2-4b13-9a6e-9ae986d558e3",
    "Douglas Shuler",
);

// CED 247 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "2ffc6b21-1926-42ff-b330-35017b36836e",
    "Mark Tedin",
);

// CED 248 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "200a7adb-fd87-4460-bcba-da323055d430",
    "Mark Poole",
);

// CED 249 — Icy Manipulator (reprint)
const ICY_MANIPULATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ICY_MANIPULATOR,
    "fbe47ed7-33a9-4464-b720-d738af997159",
    "Douglas Shuler",
);

// CED 250 — Illusionary Mask (reprint)
const ILLUSIONARY_MASK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ILLUSIONARY_MASK,
    "89dadd54-1a79-4cf9-b715-efcf10aa120d",
    "Amy Weber",
);

// CED 251 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRON_STAR,
    "88a94e26-d145-4ab6-acdb-640b0f0b3f3d",
    "Dan Frazier",
);

// CED 252 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "8ba7788f-45c2-4aa1-bd49-05d97b03740b",
    "Anson Maddocks",
);

// CED 253 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "6591c024-d796-44c0-b198-fd7a96561b7e",
    "Anson Maddocks",
);

// CED 254 — Jade Statue (reprint)
const JADE_STATUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_STATUE,
    "5b2e4a7d-feb1-4f0d-8270-b66572fcfec6",
    "Dan Frazier",
);

// CED 255 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JAYEMDAE_TOME,
    "86c1b227-bd1a-45f6-9ef0-308c35235de8",
    "Mark Tedin",
);

// CED 256 — Juggernaut (reprint)
const JUGGERNAUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JUGGERNAUT,
    "2fff8dc1-7d9b-4514-b6d1-8a306f3964e8",
    "Dan Frazier",
);

// CED 257 — Kormus Bell (reprint)
const KORMUS_BELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KORMUS_BELL,
    "cb440913-e795-4939-b14e-2e0c1ba5575f",
    "Christopher Rush",
);

// CED 258 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "ff5e106e-d6ce-43cc-b626-da802e85473f",
    "Daniel Gelon",
);

// CED 259 — Living Wall (reprint)
const LIVING_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_WALL,
    "cf332d62-0eaf-43dc-982f-31460da2872e",
    "Anson Maddocks",
);

// CED 260 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_VAULT,
    "0b12280a-f11d-4308-8b57-0460ae4bbe5c",
    "Mark Tedin",
);

// CED 261 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "8be8380b-3cfa-4ed9-8f3d-c9d6d0c260cd",
    "Quinton Hoover",
);

// CED 262 — Mox Emerald (reprint)
const MOX_EMERALD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_EMERALD,
    "294893f1-80aa-4125-8f56-3b92152818d0",
    "Dan Frazier",
);

// CED 263 — Mox Jet (reprint)
const MOX_JET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_JET,
    "923c2d41-4994-4437-be11-eeab70829000",
    "Dan Frazier",
);

// CED 264 — Mox Pearl (reprint)
const MOX_PEARL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_PEARL,
    "b0e858a2-35c5-422a-96be-61e82a59c348",
    "Dan Frazier",
);

// CED 265 — Mox Ruby (reprint)
const MOX_RUBY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_RUBY,
    "a5f4f9ea-63bb-4509-8480-c482f0f99245",
    "Dan Frazier",
);

// CED 266 — Mox Sapphire (reprint)
const MOX_SAPPHIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_SAPPHIRE,
    "8e0ecc04-01e8-4c13-bdc5-a2adba103f1d",
    "Dan Frazier",
);

// CED 267 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::NEVINYRRALS_DISK,
    "a4dbc35d-2474-4215-8dc6-2c1e2132428a",
    "Mark Tedin",
);

// CED 268 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "587793da-1e2a-4e95-84dc-470dcb95891c",
    "Jesper Myrfors",
);

// CED 269 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "ac33e9c7-e524-42bd-8380-a2e20920afa5",
    "Christopher Rush",
);

// CED 270 — Sol Ring (reprint)
const SOL_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SOL_RING,
    "dfae7531-693e-4325-8207-3ea4368e6c2c",
    "Mark Tedin",
);

// CED 271 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "7fb8a73d-a15f-4c0b-aade-8cbd68e35abd",
    "Dameon Willich",
);

// CED 272 — Sunglasses of Urza (reprint)
const SUNGLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SUNGLASSES_OF_URZA,
    "861e1a44-459c-457b-b3f9-1e4e05d4221b",
    "Dan Frazier",
);

// CED 273 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "23cdbc13-afc4-4b70-8459-d72efd849f99",
    "Sandra Everingham",
);

// CED 274 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "b5cbb7d7-35e4-4cf1-908f-8e55fa5630fc",
    "Anson Maddocks",
);

// CED 275 — Time Vault (reprint)
const TIME_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_VAULT,
    "910c54ea-ce93-40fe-aba6-94c25ae7ff90",
    "Mark Tedin",
);

// CED 276 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WINTER_ORB,
    "4b98ad27-b00f-408d-9aa3-896ba95a80da",
    "Mark Tedin",
);

// CED 277 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "d8212fdb-f6de-4349-9a66-96683e344b10",
    "Mark Tedin",
);

// CED 278 — Badlands (reprint)
const BADLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BADLANDS,
    "bae3144b-ccac-484c-b74f-c11a7c985729",
    "Rob Alexander",
);

// CED 279 — Bayou (reprint)
const BAYOU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BAYOU,
    "61bf5625-e30c-4655-bcad-a7609be1789a",
    "Jesper Myrfors",
);

// CED 280 — Plateau (reprint)
const PLATEAU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLATEAU,
    "8ca15390-0664-4ea8-b47c-88ca59025e79",
    "Drew Tucker",
);

// CED 281 — Savannah (reprint)
const SAVANNAH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH,
    "de7de05f-1a45-46b1-a217-1044dc85fcc5",
    "Rob Alexander",
);

// CED 282 — Scrubland (reprint)
const SCRUBLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRUBLAND,
    "c6aad9a9-3189-4045-abd1-abf440b307cb",
    "Jesper Myrfors",
);

// CED 283 — Taiga (reprint)
const TAIGA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TAIGA,
    "9ffd9e89-bd00-4c92-8710-b256a213fe64",
    "Rob Alexander",
);

// CED 284 — Tropical Island (reprint)
const TROPICAL_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TROPICAL_ISLAND,
    "cb19e313-b3ed-4599-864c-077eaf0d9e94",
    "Jesper Myrfors",
);

// CED 285 — Tundra (reprint)
const TUNDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TUNDRA,
    "7d1a376a-ccba-4aea-812f-0503d82df5a7",
    "Jesper Myrfors",
);

// CED 286 — Underground Sea (reprint)
const UNDERGROUND_SEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::UNDERGROUND_SEA,
    "434ff57b-0bfd-4c17-ad56-909e72f3e2ee",
    "Rob Alexander",
);

// CED 287 — Volcanic Island (reprint)
const VOLCANIC_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &beta::VOLCANIC_ISLAND,
    "1b386379-a4fd-43e0-9934-8fcd39d36d6a",
    "Brian Snõddy",
);

// CED 288 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "3347c4d1-b981-4cb0-a678-e033ff785bfc",
    "Jesper Myrfors",
);

// CED 289 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "16ef7e05-a2a3-498f-bbb0-9a38f6fe5d46",
    "Jesper Myrfors",
);

// CED 290 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "5549b73a-9abe-4d63-a673-aa08e9f6a678",
    "Jesper Myrfors",
);

// CED 291 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "181edf62-34ca-4a55-a2f6-f15940a5b43a",
    "Mark Poole",
);

// CED 292 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "713dcdc5-bd50-4aef-97c0-375ea4b902cf",
    "Mark Poole",
);

// CED 293 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "a5159732-319d-450c-b5d8-977810090aad",
    "Mark Poole",
);

// CED 294 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "d1a441bb-62f9-417e-a1e6-72e7cdb47990",
    "Dan Frazier",
);

// CED 295 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "85ff941e-dec2-49db-a346-977359ac10cb",
    "Dan Frazier",
);

// CED 296 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "18da8fa9-32e3-48ea-b4ec-461e9bc61392",
    "Dan Frazier",
);

// CED 297 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "5c6db89c-21ce-4123-955a-310c3978b2fc",
    "Douglas Shuler",
);

// CED 298 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "3b37902c-8184-43ce-8669-3a617953dfe6",
    "Douglas Shuler",
);

// CED 299 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "ad6c52df-ebc4-4f00-bd9b-0cdc69279d20",
    "Douglas Shuler",
);

// CED 300 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "d06130fa-f4a5-45c1-81e3-25982e49fab1",
    "Christopher Rush",
);

// CED 301 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "6d426fa8-72e8-4027-9df5-1289b7267a14",
    "Christopher Rush",
);

// CED 302 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "ba79c5da-12a4-419e-9793-7a85beeee30f",
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
