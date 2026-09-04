//! International Collector's Edition has no unique card definitions.
//!
//! Every card in the built-in International Collector's Edition catalog points to its first
//! printing.

use super::{CardRecord, PrintingRecord, alpha, beta};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;

// CEI 1 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "e5bcf08f-6cf0-432d-a77a-b98e59008a29",
    "Dan Frazier",
);

// CEI 2 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ARMAGEDDON,
    "7b9b83e0-0a89-4003-9b0f-fd83e48c2ae9",
    "Jesper Myrfors",
);

// CEI 3 — Balance (reprint)
const BALANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BALANCE,
    "b8db74fa-56a7-4ae3-ba5f-3156b2829f78",
    "Mark Poole",
);

// CEI 4 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "31a8b30d-9c06-4fca-bbbb-41524ed0d335",
    "Douglas Shuler",
);

// CEI 5 — Black Ward (reprint)
const BLACK_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_WARD,
    "a263c3a4-7949-4cdb-9467-0a9eb79c2bb5",
    "Dan Frazier",
);

// CEI 6 — Blaze of Glory (reprint)
const BLAZE_OF_GLORY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLAZE_OF_GLORY,
    "3706cd80-4d6a-404b-81c7-103c368c6497",
    "Richard Thomas",
);

// CEI 7 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLESSING,
    "eacb84a7-b6a5-4f8f-96cf-78cedea03987",
    "Julie Baroh",
);

// CEI 8 — Blue Ward (reprint)
const BLUE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_WARD,
    "1be86f5f-3b3c-41fe-b283-c461bc7d7a37",
    "Dan Frazier",
);

// CEI 9 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "eeffa431-5ede-4922-b975-5183fa4f8e64",
    "Dameon Willich",
);

// CEI 10 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "e8f10eb1-7646-4049-8622-62666bd40624",
    "Jesper Myrfors",
);

// CEI 11 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "b4ebec53-ce21-4ce5-96f0-00fc682a6d13",
    "Dameon Willich",
);

// CEI 12 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "19083616-694f-4a3b-946e-29af6b4ac245",
    "Sandra Everingham",
);

// CEI 13 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "a4e0a4dc-4801-4a94-8b5d-ffe71446cf8d",
    "Mark Tedin",
);

// CEI 14 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "dd982d86-27c7-4ee8-912d-80c1eb79b785",
    "Douglas Shuler",
);

// CEI 15 — Consecrate Land (reprint)
const CONSECRATE_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSECRATE_LAND,
    "a48304bb-ed7b-474b-a245-4eee18ecd0a4",
    "Jeff A. Menges",
);

// CEI 16 — Conversion (reprint)
const CONVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONVERSION,
    "ce525e8e-3afd-4ed1-92a4-77346c462c30",
    "Jesper Myrfors",
);

// CEI 17 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CRUSADE,
    "cb822055-6cf5-457b-9d24-f410affa1bb1",
    "Mark Poole",
);

// CEI 18 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "42322c69-e745-45ee-99f1-1a3cdf6a4ae9",
    "Mark Poole",
);

// CEI 19 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DISENCHANT,
    "c38a0fb1-49aa-41b3-a2ca-7e13195dd5b8",
    "Amy Weber",
);

// CEI 20 — Farmstead (reprint)
const FARMSTEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FARMSTEAD,
    "4c301096-5561-4205-8ea5-cdd05b8f5a47",
    "Mark Poole",
);

// CEI 21 — Green Ward (reprint)
const GREEN_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GREEN_WARD,
    "8526114a-4d28-48f3-b1da-37191ea77127",
    "Dan Frazier",
);

// CEI 22 — Guardian Angel (reprint)
const GUARDIAN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GUARDIAN_ANGEL,
    "006fa597-c469-4c9c-a14f-f09e1074392b",
    "Anson Maddocks",
);

// CEI 23 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "cab4def9-831f-4183-b354-a5a281c1860f",
    "Dan Frazier",
);

// CEI 24 — Holy Armor (reprint)
const HOLY_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_ARMOR,
    "6edb6bf4-f64f-4374-b524-eca6d051c63f",
    "Melissa A. Benson",
);

// CEI 25 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "4125b3af-ccca-496d-b73f-da64e98c5f9b",
    "Anson Maddocks",
);

// CEI 26 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "51ad93d0-8ed1-439e-addf-3d29c8fe3642",
    "Mark Poole",
);

// CEI 27 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "89aa1674-9e03-4ab8-a1ae-4492c2fb5cb3",
    "Richard Thomas",
);

// CEI 28 — Lance (reprint)
const LANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LANCE,
    "176ab231-1f22-4e04-b46c-480a67527a3b",
    "Rob Alexander",
);

// CEI 29 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "407c5e6b-f3f1-469c-9af9-11ccffce653f",
    "Melissa A. Benson",
);

// CEI 30 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "78e5d861-460b-4143-8232-a0ade5a2511c",
    "Douglas Shuler",
);

// CEI 31 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "2543f1e5-e91a-4967-b623-9b5b23c3e37a",
    "Cornelius Brudi",
);

// CEI 32 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "0126db58-cd4c-476c-8c7a-b60f94bf2250",
    "Kev Brockschmidt",
);

// CEI 33 — Purelace (reprint)
const PURELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PURELACE,
    "60ed70e9-4cff-41e4-9353-c7695a18f07d",
    "Sandra Everingham",
);

// CEI 34 — Red Ward (reprint)
const RED_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_WARD,
    "4660c63b-5120-4735-a732-a27149200346",
    "Dan Frazier",
);

// CEI 35 — Resurrection (reprint)
const RESURRECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RESURRECTION,
    "1a43a906-bb83-4b7a-b661-1eeb625056a0",
    "Dan Frazier",
);

// CEI 36 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "6468f82a-a128-4b89-9dcc-eb0433b111da",
    "Dameon Willich",
);

// CEI 37 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "52f8c2b2-5e63-46fe-a840-3b82d7a0cb1f",
    "Douglas Shuler",
);

// CEI 38 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "4d8213e3-7b94-4751-9749-eea3c459e751",
    "Tom Wänerstrand",
);

// CEI 39 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH_LIONS,
    "20a8a8c7-46be-4444-a9d6-c37ae989a7c6",
    "Daniel Gelon",
);

// CEI 40 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SERRA_ANGEL,
    "f3e6aff4-f2d1-4be5-890e-76e87dcae4b5",
    "Douglas Shuler",
);

// CEI 41 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWORDS_TO_PLOWSHARES,
    "387cf22b-65a5-4653-b8cf-c729fd6835b7",
    "Jeff A. Menges",
);

// CEI 42 — Veteran Bodyguard (reprint)
const VETERAN_BODYGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VETERAN_BODYGUARD,
    "bded2d91-7f94-4fd1-8102-8e8511fcff22",
    "Douglas Shuler",
);

// CEI 43 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "e87ed746-3bb7-4548-8303-94f2ffb0d70b",
    "Mark Tedin",
);

// CEI 44 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHITE_KNIGHT,
    "2ec1908b-d56a-43bf-b45d-85dd74384f9c",
    "Daniel Gelon",
);

// CEI 45 — White Ward (reprint)
const WHITE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_WARD,
    "f99f421c-7d90-4bc5-9f15-c4662acf1dac",
    "Dan Frazier",
);

// CEI 46 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WRATH_OF_GOD,
    "088311bb-3b8d-4d2e-8537-55d3b9491b04",
    "Quinton Hoover",
);

// CEI 47 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "b9367165-c7d3-44b1-9844-e2b1971ceffd",
    "Richard Thomas",
);

// CEI 48 — Ancestral Recall (reprint)
const ANCESTRAL_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANCESTRAL_RECALL,
    "e1af7c50-df15-4c7d-93e8-0dc06ceb101f",
    "Mark Poole",
);

// CEI 49 — Animate Artifact (reprint)
const ANIMATE_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANIMATE_ARTIFACT,
    "86216d42-7838-46b6-bbd5-b11f6dd9d2d8",
    "Douglas Shuler",
);

// CEI 50 — Blue Elemental Blast (reprint)
const BLUE_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLUE_ELEMENTAL_BLAST,
    "70cec0dd-b424-4f57-965d-d054089ece21",
    "Richard Thomas",
);

// CEI 51 — Braingeyser (reprint)
const BRAINGEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BRAINGEYSER,
    "77447457-1394-417c-832c-35b0306ee72f",
    "Mark Tedin",
);

// CEI 52 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLONE,
    "5510650a-8399-46a4-8c68-90c892c476d7",
    "Julie Baroh",
);

// CEI 53 — Control Magic (reprint)
const CONTROL_MAGIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTROL_MAGIC,
    "be546529-bf28-4608-9920-510f0b928ce4",
    "Dameon Willich",
);

// CEI 54 — Copy Artifact (reprint)
const COPY_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPY_ARTIFACT,
    "2b3299b8-4c76-4550-ad2a-227b2bff63e9",
    "Amy Weber",
);

// CEI 55 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COUNTERSPELL,
    "1ec94c01-4e28-4aa6-ae6d-76aec880dbc7",
    "Mark Poole",
);

// CEI 56 — Creature Bond (reprint)
const CREATURE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CREATURE_BOND,
    "afabdda1-212c-4881-8a93-54114cf50e07",
    "Anson Maddocks",
);

// CEI 57 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "e5e5c722-f247-4101-912d-74f3afec1ca8",
    "Douglas Shuler",
);

// CEI 58 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "c8a0596f-053e-4f3e-aed0-7f4ec2b89235",
    "Quinton Hoover",
);

// CEI 59 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "26e21fa8-4617-4cef-a70d-387e73dafc22",
    "Anson Maddocks",
);

// CEI 60 — Invisibility (reprint)
const INVISIBILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INVISIBILITY,
    "7c4d29f9-5470-4a21-9ac9-82cfeccb247a",
    "Anson Maddocks",
);

// CEI 61 — Jump (reprint)
const JUMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUMP,
    "7c45c0fc-84ed-41d5-ac4a-2455136cd3c6",
    "Mark Poole",
);

// CEI 62 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "2f3d5f32-b47f-422b-a558-192cf104003c",
    "Anson Maddocks",
);

// CEI 63 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "fd87fd5b-5b6a-48bc-9b65-7c78c867728d",
    "Melissa A. Benson",
);

// CEI 64 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "d66e6a50-d61f-4c0e-8742-2faba6d1ad48",
    "Julie Baroh",
);

// CEI 65 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "c90ecea4-24b3-4dfa-9bdf-1d38049ba6f5",
    "Dan Frazier",
);

// CEI 66 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_SHORT,
    "6baa086b-f957-4995-9abc-326d83122c89",
    "Dameon Willich",
);

// CEI 67 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "bb3a87ff-897c-48c3-9b7a-8bfbf4bac5cd",
    "Jeff A. Menges",
);

// CEI 68 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "5223ed74-6b6d-49f4-b370-1d618efb356b",
    "Mark Poole",
);

// CEI 69 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "0319bfa3-226f-40bc-be4a-6dca47856b4c",
    "Dameon Willich",
);

// CEI 70 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "a9592fd1-0b19-4d20-bbd2-6fedf166084f",
    "Jesper Myrfors",
);

// CEI 71 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "d36351c7-1383-448a-b9f8-c4f8d924d449",
    "Tom Wänerstrand",
);

// CEI 72 — Power Leak (reprint)
const POWER_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_LEAK,
    "2f3de9e0-39da-4e14-b958-3f86f4a85680",
    "Drew Tucker",
);

// CEI 73 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "744d6ceb-5418-4f19-bf86-f67e67594c38",
    "Richard Thomas",
);

// CEI 74 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "670dbed7-9380-44ca-a38a-4bc91664b25e",
    "Douglas Shuler",
);

// CEI 75 — Psionic Blast (reprint)
const PSIONIC_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PSIONIC_BLAST,
    "2d0a6fe3-bca2-4192-9749-45e7d9bb1074",
    "Douglas Shuler",
);

// CEI 76 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "55f502e9-809d-4ffd-901e-3853431f3cb3",
    "Brian Snõddy",
);

// CEI 77 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "c9f46356-1d07-4fdc-8985-1ecc50d89ca2",
    "Jeff A. Menges",
);

// CEI 78 — Siren's Call (reprint)
const SIREN_S_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIREN_S_CALL,
    "497d9221-31ab-4483-a3ba-7f8c63f6f270",
    "Anson Maddocks",
);

// CEI 79 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "9b356d20-91cf-43d4-8f6c-1c74d9608d1a",
    "Mark Poole",
);

// CEI 80 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "8226ac1e-0e29-45a5-a7c3-13acc410adc4",
    "Brian Snõddy",
);

// CEI 81 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STASIS,
    "9210b59f-0a84-4978-a840-60c4dfe8952c",
    "Fay Jones",
);

// CEI 82 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "cdb19bc2-1ebc-4f5c-92cb-66e22a0d92f0",
    "Amy Weber",
);

// CEI 83 — Thoughtlace (reprint)
const THOUGHTLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THOUGHTLACE,
    "44be85e9-49b0-4c31-9b43-730c88256cac",
    "Mark Poole",
);

// CEI 84 — Time Walk (reprint)
const TIME_WALK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_WALK,
    "8cededf4-de0e-4d44-8a5a-16216fee22a3",
    "Amy Weber",
);

// CEI 85 — Timetwister (reprint)
const TIMETWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIMETWISTER,
    "bfad968e-9c13-410a-97b5-fe65aa6d5880",
    "Mark Tedin",
);

// CEI 86 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "df01f48c-1d9c-45c3-aa6a-fa9d352eaf4d",
    "Rob Alexander",
);

// CEI 87 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "1d7acd9e-134c-42b6-b24a-678c760dc9de",
    "Douglas Shuler",
);

// CEI 88 — Vesuvan Doppelganger (reprint)
const VESUVAN_DOPPELGANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VESUVAN_DOPPELGANGER,
    "eb36cb3e-6eca-4ea7-ab86-ffc6a76f48ff",
    "Quinton Hoover",
);

// CEI 89 — Volcanic Eruption (reprint)
const VOLCANIC_ERUPTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VOLCANIC_ERUPTION,
    "db6f8955-308e-4678-af16-036a061f1b4f",
    "Douglas Shuler",
);

// CEI 90 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "2745dc9b-8e5a-441b-a31e-f5a5c1ed3574",
    "Richard Thomas",
);

// CEI 91 — Wall of Water (reprint)
const WALL_OF_WATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WATER,
    "4d4ecd47-4d8c-461c-99e4-5c61999abdf2",
    "Richard Thomas",
);

// CEI 92 — Water Elemental (reprint)
const WATER_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WATER_ELEMENTAL,
    "b36b6aa2-1c83-4028-b155-8a7da6811dfa",
    "Jeff A. Menges",
);

// CEI 93 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "64e963dc-09e3-4ee8-b8ca-62e1cc430636",
    "Anson Maddocks",
);

// CEI 94 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "7dd7ac91-728c-41b8-9b12-ba41336721f7",
    "Jesper Myrfors",
);

// CEI 95 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_KNIGHT,
    "5da1b3e3-04eb-4126-963b-87e34ce96796",
    "Jeff A. Menges",
);

// CEI 96 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "b0e01c3b-4293-4b12-8415-06ea6d1a8334",
    "Jeff A. Menges",
);

// CEI 97 — Contract from Below (reprint)
const CONTRACT_FROM_BELOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTRACT_FROM_BELOW,
    "34fedae5-87f0-4857-9bc5-788753659b6a",
    "Douglas Shuler",
);

// CEI 98 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "60f46fb0-5647-4682-a07b-c0f0de731ab6",
    "Jesper Myrfors",
);

// CEI 99 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DARK_RITUAL,
    "de158058-b2ff-46c5-bbb3-5f76e5c2782e",
    "Sandra Everingham",
);

// CEI 100 — Darkpact (reprint)
const DARKPACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARKPACT,
    "2d6df47b-40b1-4c8d-a03d-e82c9526562b",
    "Quinton Hoover",
);

// CEI 101 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "ccf969d2-6b30-4a37-b0a3-1d7b339ae0af",
    "Anson Maddocks",
);

// CEI 102 — Deathlace (reprint)
const DEATHLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHLACE,
    "e3adf23a-c37e-42ad-92a4-16a16c3f1c44",
    "Sandra Everingham",
);

// CEI 103 — Demonic Attorney (reprint)
const DEMONIC_ATTORNEY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_ATTORNEY,
    "060d3514-84de-49a5-a921-d6a88e3c3fa7",
    "Daniel Gelon",
);

// CEI 104 — Demonic Hordes (reprint)
const DEMONIC_HORDES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_HORDES,
    "046980da-f32c-4102-8da9-df96e9e6d0bc",
    "Jesper Myrfors",
);

// CEI 105 — Demonic Tutor (reprint)
const DEMONIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DEMONIC_TUTOR,
    "2cb09cff-a2c6-4756-96a3-fbe2d9a7a071",
    "Douglas Shuler",
);

// CEI 106 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAIN_LIFE,
    "18fc6cd9-2122-48ca-bec5-c0b37f0d7f01",
    "Douglas Shuler",
);

// CEI 107 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "9bebec5a-1bfe-4a5d-87c7-9efe238beb0f",
    "Sandra Everingham",
);

// CEI 108 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "41ba2f5a-bf46-40e1-b100-6751de29e11f",
    "Sandra Everingham",
);

// CEI 109 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "a9cc66da-9506-498c-b6c2-ee5175e4a88c",
    "Mark Poole",
);

// CEI 110 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "6f36c32e-e750-4895-b6de-7f489fe17f41",
    "Douglas Shuler",
);

// CEI 111 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "2613a3dc-38f2-4abf-9f5f-d2340c72d7ef",
    "Dan Frazier",
);

// CEI 112 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "eb9f793e-c835-40f1-a2aa-2c2780eaeb3c",
    "Mark Poole",
);

// CEI 113 — Hypnotic Specter (reprint)
const HYPNOTIC_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::HYPNOTIC_SPECTER,
    "93714a55-9082-4532-9785-a1a3676b5616",
    "Douglas Shuler",
);

// CEI 114 — Lich (reprint)
const LICH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LICH,
    "82b30c79-45c4-435f-ad61-fb6dc15b42c8",
    "Daniel Gelon",
);

// CEI 115 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "545859bc-c1f3-4526-8840-3eb8117c3558",
    "Mark Tedin",
);

// CEI 116 — Mind Twist (reprint)
const MIND_TWIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MIND_TWIST,
    "5b4a3a5a-a4aa-4ce2-abd0-39758b3001a9",
    "Julie Baroh",
);

// CEI 117 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "400c6ce6-5595-4901-a133-c5becaac4e08",
    "Christopher Rush",
);

// CEI 118 — Nettling Imp (reprint)
const NETTLING_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETTLING_IMP,
    "2abc90d2-fa66-4cb9-863a-55a6699425c8",
    "Quinton Hoover",
);

// CEI 119 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "2ecac2eb-bc4f-43cc-a14d-56dadf102e44",
    "Melissa A. Benson",
);

// CEI 120 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "f4c22283-5a44-4acb-8827-2d79eb1da723",
    "Anson Maddocks",
);

// CEI 121 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "b23845b8-d6d8-43fa-80fc-7c4773e1eebe",
    "Jesper Myrfors",
);

// CEI 122 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "46bc52d5-0dca-438f-87e9-ec0eea25e771",
    "Anson Maddocks",
);

// CEI 123 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "42b14dbf-06ff-4ee6-b3dd-196ef0516ef4",
    "Jeff A. Menges",
);

// CEI 124 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROYAL_ASSASSIN,
    "4d47daf8-43ad-4fa4-acd3-7c05b09a366d",
    "Tom Wänerstrand",
);

// CEI 125 — Sacrifice (reprint)
const SACRIFICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SACRIFICE,
    "2143714f-1a87-4271-8961-cc54d8750736",
    "Dan Frazier",
);

// CEI 126 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "b93d6747-66c6-4fea-8bbe-c75cf1387257",
    "Jesper Myrfors",
);

// CEI 127 — Scavenging Ghoul (reprint)
const SCAVENGING_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCAVENGING_GHOUL,
    "52c522e8-cbdb-495d-93b7-e6c81d53668f",
    "Jeff A. Menges",
);

// CEI 128 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SENGIR_VAMPIRE,
    "45c45cab-936a-4a90-a552-b562e777302e",
    "Anson Maddocks",
);

// CEI 129 — Simulacrum (reprint)
const SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIMULACRUM,
    "3635c5dd-cc5c-447f-8422-51a14dd4f913",
    "Mark Poole",
);

// CEI 130 — Sinkhole (reprint)
const SINKHOLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SINKHOLE,
    "a652ddc0-165c-4c8f-b887-57d32d1f2d8e",
    "Sandra Everingham",
);

// CEI 131 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TERROR,
    "0829428a-d19d-4161-bc08-177752ee3809",
    "Ron Spencer",
);

// CEI 132 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "3f5cb2d1-e914-4a3b-8b6c-e9d550912f35",
    "Douglas Shuler",
);

// CEI 133 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "1df15316-3916-4f1d-8145-5d29a07e6bac",
    "Anson Maddocks",
);

// CEI 134 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "51177b9d-9ba1-4efd-beed-733a572871e7",
    "Amy Weber",
);

// CEI 135 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "ecf4c625-b417-4d4b-9ff9-5a32416d4e94",
    "Douglas Shuler",
);

// CEI 136 — Will-o'-the-Wisp (reprint)
const WILL_O_THE_WISP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILL_O_THE_WISP,
    "b33b4846-98b3-41cd-aa59-2295a3037d2d",
    "Jesper Myrfors",
);

// CEI 137 — Word of Command (reprint)
const WORD_OF_COMMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WORD_OF_COMMAND,
    "4a7f0dbb-00cb-47d5-8884-9aa397fab559",
    "Jesper Myrfors",
);

// CEI 138 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "887db5bd-a276-4e8e-8c1a-b72d16e0e5d9",
    "Jeff A. Menges",
);

// CEI 139 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "0bc8e138-1c93-4ddb-b6bd-fe8470ab55b8",
    "Mark Poole",
);

// CEI 140 — Chaoslace (reprint)
const CHAOSLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHAOSLACE,
    "ed2cc2d5-4557-4da8-8b54-ad9e401d5ba5",
    "Dameon Willich",
);

// CEI 141 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "3eaeb43d-cc8e-4342-9e74-2884ac6c0a90",
    "Anson Maddocks",
);

// CEI 142 — Dragon Whelp (reprint)
const DRAGON_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAGON_WHELP,
    "1d0b4a6a-31e7-4773-b984-f2b29dc0fd58",
    "Amy Weber",
);

// CEI 143 — Dwarven Demolition Team (reprint)
const DWARVEN_DEMOLITION_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_DEMOLITION_TEAM,
    "364af773-d72b-463a-9591-c318d1854139",
    "Kev Brockschmidt",
);

// CEI 144 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "1372bf4f-d930-499a-b346-fc24f6349981",
    "Douglas Shuler",
);

// CEI 145 — Earth Elemental (reprint)
const EARTH_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTH_ELEMENTAL,
    "55db07c8-b9ae-4c54-b0b3-1e549d287f7f",
    "Dan Frazier",
);

// CEI 146 — Earthbind (reprint)
const EARTHBIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHBIND,
    "93cab1fb-7f16-4ccb-be38-02352d39e37c",
    "Quinton Hoover",
);

// CEI 147 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::EARTHQUAKE,
    "1b5a1955-c186-4cce-ade0-3179682323b6",
    "Dan Frazier",
);

// CEI 148 — False Orders (reprint)
const FALSE_ORDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FALSE_ORDERS,
    "97bda7de-10fd-4cf8-9112-b9eafcd2add4",
    "Anson Maddocks",
);

// CEI 149 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "660ee846-668e-4b3a-b5d3-0dd0d3afd175",
    "Melissa A. Benson",
);

// CEI 150 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FIREBALL,
    "71280738-c5b4-4e76-912e-cfe18f6846f8",
    "Mark Tedin",
);

// CEI 151 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "11eee046-8ad9-4baf-a12f-87f31a5fe41f",
    "Dan Frazier",
);

// CEI 152 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "cadef976-3b99-4c80-9b39-1d5c78647595",
    "Dameon Willich",
);

// CEI 153 — Fork (reprint)
const FORK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FORK,
    "df918162-c87c-49ee-bc8d-344f768a3888",
    "Amy Weber",
);

// CEI 154 — Goblin Balloon Brigade (reprint)
const GOBLIN_BALLOON_BRIGADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_BALLOON_BRIGADE,
    "0edaf0ea-16e9-487c-9b51-96bbcf27e664",
    "Andi Rusu",
);

// CEI 155 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_KING,
    "cb62f97c-3a2d-470d-a42c-411b3d39622a",
    "Jesper Myrfors",
);

// CEI 156 — Granite Gargoyle (reprint)
const GRANITE_GARGOYLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GRANITE_GARGOYLE,
    "0d69b171-cf71-46b5-b9cd-b9f2f3eb59e7",
    "Christopher Rush",
);

// CEI 157 — Gray Ogre (reprint)
const GRAY_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRAY_OGRE,
    "d3b7faf0-5d1e-42d6-820f-88f3f9b2e861",
    "Dan Frazier",
);

// CEI 158 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "27d7e1da-8929-4435-9235-669c01170319",
    "Dan Frazier",
);

// CEI 159 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "2f516f1e-84e8-4a9a-852b-1cde3e431b60",
    "Anson Maddocks",
);

// CEI 160 — Ironclaw Orcs (reprint)
const IRONCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRONCLAW_ORCS,
    "ca32c957-fb49-4efb-b39c-4e6370fd6b82",
    "Anson Maddocks",
);

// CEI 161 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "d529049b-cece-4921-a4ea-d10bace2d516",
    "Kev Brockschmidt",
);

// CEI 162 — Lightning Bolt (reprint)
const LIGHTNING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LIGHTNING_BOLT,
    "ef9047f6-5e5d-49bf-a183-4d1490b291fb",
    "Christopher Rush",
);

// CEI 163 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "a10d6f57-5758-4da6-8bb0-29e7b375f8c5",
    "Christopher Rush",
);

// CEI 164 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "7c0f8b09-f6fc-4a5f-b20b-edf722cdc173",
    "Christopher Rush",
);

// CEI 165 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "9872ad55-3c71-4911-9428-5a01e8e8ad57",
    "Jeff A. Menges",
);

// CEI 166 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "b79deb44-e5e5-4e5e-ae5f-be18a1ae014e",
    "Anson Maddocks",
);

// CEI 167 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "05259e47-b5a6-43af-a523-d906eca1be32",
    "Dan Frazier",
);

// CEI 168 — Power Surge (reprint)
const POWER_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SURGE,
    "3e29e791-68a7-4cfa-ab35-db2003566938",
    "Douglas Shuler",
);

// CEI 169 — Raging River (reprint)
const RAGING_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAGING_RIVER,
    "7359c036-cc46-4173-80b7-b280bd8dffc1",
    "Sandra Everingham",
);

// CEI 170 — Red Elemental Blast (reprint)
const RED_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::RED_ELEMENTAL_BLAST,
    "e905c1ba-b0d1-49b0-be5c-244b4fab8932",
    "Richard Thomas",
);

// CEI 171 — Roc of Kher Ridges (reprint)
const ROC_OF_KHER_RIDGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROC_OF_KHER_RIDGES,
    "82320dae-924e-4887-945e-7875b78a6cf5",
    "Andi Rusu",
);

// CEI 172 — Rock Hydra (reprint)
const ROCK_HYDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROCK_HYDRA,
    "676b2a57-b9a9-409e-83d5-83b488a33f35",
    "Jeff A. Menges",
);

// CEI 173 — Sedge Troll (reprint)
const SEDGE_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SEDGE_TROLL,
    "9d156db8-9a69-46d2-b40d-e239e7589cd1",
    "Dan Frazier",
);

// CEI 174 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SHATTER,
    "a5de2a6e-7dcf-4e2f-a74c-500c4379c9b2",
    "Amy Weber",
);

// CEI 175 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "4310cc73-79bc-4d06-b71a-679c9c6cca5c",
    "Melissa A. Benson",
);

// CEI 176 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SMOKE,
    "41388b5e-4b40-49ac-aefc-9442a1dd237a",
    "Jesper Myrfors",
);

// CEI 177 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_GIANT,
    "1fd3860d-c303-4936-a4f1-eeeac77a18e7",
    "Dameon Willich",
);

// CEI 178 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_RAIN,
    "1e61c422-557d-4f31-99a3-9f3faf6ac40b",
    "Daniel Gelon",
);

// CEI 179 — Tunnel (reprint)
const TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNNEL,
    "df649c32-bb7f-4cf3-9723-ecfd9a6f4cd2",
    "Dan Frazier",
);

// CEI 180 — Two-Headed Giant of Foriys (reprint)
const TWO_HEADED_GIANT_OF_FORIYS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWO_HEADED_GIANT_OF_FORIYS,
    "c69db4eb-71a2-4e57-8f34-1f0749e11336",
    "Anson Maddocks",
);

// CEI 181 — Uthden Troll (reprint)
const UTHDEN_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UTHDEN_TROLL,
    "8796743c-37b7-406a-92ab-c3d3225887f3",
    "Douglas Shuler",
);

// CEI 182 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "0d5290f4-d070-4b0d-bce7-b20a8427da81",
    "Richard Thomas",
);

// CEI 183 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "869570ad-216c-4d00-a3d6-a3ddc52af35f",
    "Dan Frazier",
);

// CEI 184 — Wheel of Fortune (reprint)
const WHEEL_OF_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHEEL_OF_FORTUNE,
    "4c23fbbc-511c-494b-9a5c-c164b1af6091",
    "Daniel Gelon",
);

// CEI 185 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "b06f50ed-1248-4489-afc0-7c5cf185650c",
    "Jeff A. Menges",
);

// CEI 186 — Berserk (reprint)
const BERSERK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BERSERK,
    "771cfdef-9826-45d3-a16d-94adfe51bbb3",
    "Dan Frazier",
);

// CEI 187 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BIRDS_OF_PARADISE,
    "38e33386-a868-478d-91f1-56899c652c09",
    "Mark Poole",
);

// CEI 188 — Camouflage (reprint)
const CAMOUFLAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CAMOUFLAGE,
    "f50f074a-0825-4ef1-8734-4914c6d6af56",
    "Jesper Myrfors",
);

// CEI 189 — Channel (reprint)
const CHANNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHANNEL,
    "2c129602-d4bc-44ee-a168-dd4075c92972",
    "Richard Thomas",
);

// CEI 190 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "a4ac21eb-9545-4c2d-a392-a2d3e740d6b3",
    "Dan Frazier",
);

// CEI 191 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "5737d51f-3510-4580-9d2e-a72f16f33b52",
    "Daniel Gelon",
);

// CEI 192 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "326f0ba3-1013-4956-b793-43b5af5693f3",
    "Anson Maddocks",
);

// CEI 193 — Fastbond (reprint)
const FASTBOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FASTBOND,
    "6151b583-3aa7-4be8-9d05-d5e53f19c6c9",
    "Mark Poole",
);

// CEI 194 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "e596f9e2-a7c8-40ba-a40c-e659b4d87c2f",
    "Jesper Myrfors",
);

// CEI 195 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "dd1fce25-681b-4355-b872-78adc6bb4dad",
    "Douglas Shuler",
);

// CEI 196 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "d786bdf8-7a93-41e4-bd6c-4cb9608584bd",
    "Daniel Gelon",
);

// CEI 197 — Gaea's Liege (reprint)
const GAEA_S_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAEA_S_LIEGE,
    "70559695-ca22-4f7f-8ef8-b25507aa8bab",
    "Dameon Willich",
);

// CEI 198 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GIANT_GROWTH,
    "0ba15238-193b-4e4a-93e3-08745fe3ac56",
    "Sandra Everingham",
);

// CEI 199 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "6ebd7679-eb38-4089-8e34-092710fa01eb",
    "Sandra Everingham",
);

// CEI 200 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "e921b721-74c3-4f25-8593-ff5685a74a91",
    "Jeff A. Menges",
);

// CEI 201 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "7bb356f4-034e-4b9c-a574-f0f3921a7adb",
    "Dameon Willich",
);

// CEI 202 — Ice Storm (reprint)
const ICE_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ICE_STORM,
    "e3c526f4-a987-4ecc-9ec1-53540481ff8b",
    "Dan Frazier",
);

// CEI 203 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "cd191b5d-331e-48af-bdab-2407750d436b",
    "Dameon Willich",
);

// CEI 204 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "4db989ab-3732-4d82-978a-531fdc944744",
    "Jesper Myrfors",
);

// CEI 205 — Kudzu (reprint)
const KUDZU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KUDZU,
    "ffdfb80a-4420-4fe8-9b5b-c6b43cc69de4",
    "Mark Poole",
);

// CEI 206 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "4939ad29-cf79-4a7f-8be7-f2b3cc7a61d4",
    "Sandra Everingham",
);

// CEI 207 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "9d7623eb-eb7a-4b32-ab85-5144fb14a843",
    "Dameon Willich",
);

// CEI 208 — Lifelace (reprint)
const LIFELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFELACE,
    "ebf079dc-96ac-494d-8cd0-3e32f249e5bb",
    "Amy Weber",
);

// CEI 209 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "ebc1a199-3d02-4db7-8a92-3df179ff5870",
    "Anson Maddocks",
);

// CEI 210 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "304c28c1-ffe2-4e9a-8207-b7ff3456a162",
    "Jesper Myrfors",
);

// CEI 211 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LLANOWAR_ELVES,
    "81e3a887-4ef7-4020-815e-1336a835b25d",
    "Anson Maddocks",
);

// CEI 212 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "b45957b6-b1fb-4bdf-8a8e-b8ba53839a95",
    "Anson Maddocks",
);

// CEI 213 — Natural Selection (reprint)
const NATURAL_SELECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NATURAL_SELECTION,
    "a56327f4-796f-4ca1-adb9-2f12177f6ef3",
    "Mark Poole",
);

// CEI 214 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "7bb17e54-2f33-434f-8bd3-73c15f6bc3ee",
    "Quinton Hoover",
);

// CEI 215 — Regrowth (reprint)
const REGROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::REGROWTH,
    "8c449a9c-d6ab-4b7b-89b2-447307180823",
    "Dameon Willich",
);

// CEI 216 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRYB_SPRITES,
    "c9f35acd-4445-49b9-9563-064d820b3553",
    "Amy Weber",
);

// CEI 217 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "796663c4-529b-4846-8f2b-08c28a94ccc6",
    "Anson Maddocks",
);

// CEI 218 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "13c6492c-e45e-4c70-912b-2c821def8353",
    "Mark Poole",
);

// CEI 219 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "59cebb57-775e-48a4-bc7a-b7cd30c8c48f",
    "Dan Frazier",
);

// CEI 220 — Timber Wolves (reprint)
const TIMBER_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TIMBER_WOLVES,
    "7b61152f-7d1c-4ebd-bfc3-d36aeebd84b3",
    "Melissa A. Benson",
);

// CEI 221 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "6fcb9615-6a4d-4188-981c-e2db9901b3b7",
    "Douglas Shuler",
);

// CEI 222 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "42c90706-ac02-4b86-8cf3-7f3170fe25c4",
    "Richard Thomas",
);

// CEI 223 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "83e26f8d-1e3f-4c80-977e-27e0bcacf4cc",
    "Kev Brockschmidt",
);

// CEI 224 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "a39298ce-cb63-4372-b54e-2aa6ce660b1e",
    "Anson Maddocks",
);

// CEI 225 — Wall of Ice (reprint)
const WALL_OF_ICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_ICE,
    "dfe1125b-2651-4eaa-9ea8-7e12f7a78ce0",
    "Richard Thomas",
);

// CEI 226 — Wall of Wood (reprint)
const WALL_OF_WOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WOOD,
    "e24dcd78-c504-48f9-adc5-b762555fb664",
    "Mark Tedin",
);

// CEI 227 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "0e5736fa-7e7f-4780-87cb-c8cfee8daea0",
    "Cornelius Brudi",
);

// CEI 228 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "8f5d73e5-8f80-446a-a44a-7f95437f5e55",
    "Jeff A. Menges",
);

// CEI 229 — Web (reprint)
const WEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEB,
    "5932cd89-e85d-4080-8c0f-f5d1fe399bf1",
    "Rob Alexander",
);

// CEI 230 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "261a3bb8-6033-4e1a-88d3-fe6ce2a30cc0",
    "Mark Poole",
);

// CEI 231 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANKH_OF_MISHRA,
    "1bc25f70-c64f-475e-8529-1631bdd3d7dc",
    "Amy Weber",
);

// CEI 232 — Basalt Monolith (reprint)
const BASALT_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BASALT_MONOLITH,
    "57d943b3-2c84-4612-b21f-c8ec363981f9",
    "Jesper Myrfors",
);

// CEI 233 — Black Lotus (reprint)
const BLACK_LOTUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_LOTUS,
    "bad6b494-d773-4f7d-ac39-85d82e1d3015",
    "Christopher Rush",
);

// CEI 234 — Black Vise (reprint)
const BLACK_VISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_VISE,
    "457df897-a78d-40a1-90e1-ccc2ea2872d6",
    "Richard Thomas",
);

// CEI 235 — Celestial Prism (reprint)
const CELESTIAL_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CELESTIAL_PRISM,
    "3f4230f0-c9ac-460d-aee2-67d99d4f6017",
    "Amy Weber",
);

// CEI 236 — Chaos Orb (reprint)
const CHAOS_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHAOS_ORB,
    "a340cfdf-2401-4d19-8d67-b599a2a51641",
    "Mark Tedin",
);

// CEI 237 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "730933d3-bc94-421c-be76-7ca8f8796a67",
    "Drew Tucker",
);

// CEI 238 — Conservator (reprint)
const CONSERVATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSERVATOR,
    "7acd7b94-6363-4032-bd73-03b9cde7923e",
    "Amy Weber",
);

// CEI 239 — Copper Tablet (reprint)
const COPPER_TABLET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPPER_TABLET,
    "f32b9615-75b4-4540-9675-a6dca0398852",
    "Amy Weber",
);

// CEI 240 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "d562d1f3-3240-4191-a05b-d15ff8e9e5ca",
    "Amy Weber",
);

// CEI 241 — Cyclopean Tomb (reprint)
const CYCLOPEAN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CYCLOPEAN_TOMB,
    "1ccd0145-26eb-4e53-9aca-aeb7a774535f",
    "Anson Maddocks",
);

// CEI 242 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "049909a4-6b56-423d-baf6-209d06dfa1db",
    "Dan Frazier",
);

// CEI 243 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "5b61d695-5594-4885-abe1-8a7a548f8f12",
    "Dan Frazier",
);

// CEI 244 — Forcefield (reprint)
const FORCEFIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCEFIELD,
    "32b61363-89b5-4fbd-ab1e-d4b1461fa42b",
    "Dan Frazier",
);

// CEI 245 — Gauntlet of Might (reprint)
const GAUNTLET_OF_MIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAUNTLET_OF_MIGHT,
    "e23b1093-6108-476b-a8c1-581951257dc9",
    "Christopher Rush",
);

// CEI 246 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GLASSES_OF_URZA,
    "643805f0-a17b-4a58-a417-17efaa199fd4",
    "Douglas Shuler",
);

// CEI 247 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "245c9f17-5267-4c6e-a7dd-c4af572b1bc5",
    "Mark Tedin",
);

// CEI 248 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "b631cff1-6f5f-41af-ada2-46e8eefe6849",
    "Mark Poole",
);

// CEI 249 — Icy Manipulator (reprint)
const ICY_MANIPULATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ICY_MANIPULATOR,
    "568ff584-732e-43c0-b5fe-2b0861a0836e",
    "Douglas Shuler",
);

// CEI 250 — Illusionary Mask (reprint)
const ILLUSIONARY_MASK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ILLUSIONARY_MASK,
    "77ccbbcb-6895-49bc-8353-66deec99db16",
    "Amy Weber",
);

// CEI 251 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRON_STAR,
    "985f206d-3639-4a5a-a9c4-564863037cbc",
    "Dan Frazier",
);

// CEI 252 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "60721fcc-6267-43c5-8fd2-9860d1f9fff5",
    "Anson Maddocks",
);

// CEI 253 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "3082dea4-c888-4878-8d02-dbbacfe2250d",
    "Anson Maddocks",
);

// CEI 254 — Jade Statue (reprint)
const JADE_STATUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_STATUE,
    "022bca13-5da8-424c-b3e5-7ced9d25868d",
    "Dan Frazier",
);

// CEI 255 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JAYEMDAE_TOME,
    "0e671f37-4849-42ec-b017-d0934b8064a2",
    "Mark Tedin",
);

// CEI 256 — Juggernaut (reprint)
const JUGGERNAUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JUGGERNAUT,
    "07474c6d-a52d-4163-b2f9-5de654ae69d8",
    "Dan Frazier",
);

// CEI 257 — Kormus Bell (reprint)
const KORMUS_BELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KORMUS_BELL,
    "b949538c-ec37-48ba-b255-fa722c182667",
    "Christopher Rush",
);

// CEI 258 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "0ee3dbe9-3170-457f-81b9-fbd7aced4188",
    "Daniel Gelon",
);

// CEI 259 — Living Wall (reprint)
const LIVING_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_WALL,
    "fa55d441-91d5-4583-bec4-e64209187bbc",
    "Anson Maddocks",
);

// CEI 260 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_VAULT,
    "63fa2876-8c18-45d5-b51a-a782d630a30a",
    "Mark Tedin",
);

// CEI 261 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "3b444958-3e53-49d3-a6ac-95481982beaf",
    "Quinton Hoover",
);

// CEI 262 — Mox Emerald (reprint)
const MOX_EMERALD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_EMERALD,
    "581e9b43-d608-4e8c-a487-ebf7c8440df9",
    "Dan Frazier",
);

// CEI 263 — Mox Jet (reprint)
const MOX_JET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_JET,
    "1d806fe3-c823-4518-ad27-d1171ab1dea5",
    "Dan Frazier",
);

// CEI 264 — Mox Pearl (reprint)
const MOX_PEARL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_PEARL,
    "618a52ac-9f2a-4bce-93ad-c76717b4fc59",
    "Dan Frazier",
);

// CEI 265 — Mox Ruby (reprint)
const MOX_RUBY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_RUBY,
    "ef30dc5e-7bc3-447b-a8c3-98362c597d97",
    "Dan Frazier",
);

// CEI 266 — Mox Sapphire (reprint)
const MOX_SAPPHIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_SAPPHIRE,
    "eebe1882-f2f4-4749-8890-974e9eb48d68",
    "Dan Frazier",
);

// CEI 267 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::NEVINYRRALS_DISK,
    "29f3a58b-4c1f-4c23-918b-11a13e34927c",
    "Mark Tedin",
);

// CEI 268 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "ed265558-a099-4b96-85c6-7f572cf6d657",
    "Jesper Myrfors",
);

// CEI 269 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "2fbb8867-8bde-411b-afbd-27079172dcd1",
    "Christopher Rush",
);

// CEI 270 — Sol Ring (reprint)
const SOL_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SOL_RING,
    "9ffc356f-9053-435c-ac1c-e36c2806b6fa",
    "Mark Tedin",
);

// CEI 271 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "3648f1e7-b81f-42b3-854d-2c68d21361a2",
    "Dameon Willich",
);

// CEI 272 — Sunglasses of Urza (reprint)
const SUNGLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SUNGLASSES_OF_URZA,
    "44ce935e-0bba-46e7-8398-d2039e88ee99",
    "Dan Frazier",
);

// CEI 273 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "edb2689e-a37e-41cd-ac0d-4c0f14a4883f",
    "Sandra Everingham",
);

// CEI 274 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "ed2a10c7-fdea-4e72-9e3f-33748719964b",
    "Anson Maddocks",
);

// CEI 275 — Time Vault (reprint)
const TIME_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_VAULT,
    "5ba043f1-8b53-4efb-9395-55b07be87062",
    "Mark Tedin",
);

// CEI 276 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WINTER_ORB,
    "5d106bd3-caf2-49c4-bbe3-f87e11254665",
    "Mark Tedin",
);

// CEI 277 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "31e5793e-3217-44e1-83ae-a8c322a30258",
    "Mark Tedin",
);

// CEI 278 — Badlands (reprint)
const BADLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BADLANDS,
    "8b9f6057-aa8b-4baa-8336-83a353ba4d00",
    "Rob Alexander",
);

// CEI 279 — Bayou (reprint)
const BAYOU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BAYOU,
    "d4b66dbb-3853-444c-be51-c6d85ce6cd5f",
    "Jesper Myrfors",
);

// CEI 280 — Plateau (reprint)
const PLATEAU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLATEAU,
    "1daf4348-05c5-4c55-bcf2-c7bec7988bfc",
    "Drew Tucker",
);

// CEI 281 — Savannah (reprint)
const SAVANNAH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH,
    "75e4bacd-5963-49a8-95e3-67341c1f07a9",
    "Rob Alexander",
);

// CEI 282 — Scrubland (reprint)
const SCRUBLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRUBLAND,
    "14065271-d942-42a8-aa38-d8a622c0375d",
    "Jesper Myrfors",
);

// CEI 283 — Taiga (reprint)
const TAIGA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TAIGA,
    "352a70b0-cd96-4c6c-9cd5-23a1ac45182d",
    "Rob Alexander",
);

// CEI 284 — Tropical Island (reprint)
const TROPICAL_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TROPICAL_ISLAND,
    "729daf0e-b822-41bd-a908-ca8ec34032f0",
    "Jesper Myrfors",
);

// CEI 285 — Tundra (reprint)
const TUNDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TUNDRA,
    "f830b134-42bf-4ec8-950c-4e19a6dcc742",
    "Jesper Myrfors",
);

// CEI 286 — Underground Sea (reprint)
const UNDERGROUND_SEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::UNDERGROUND_SEA,
    "9f99ac99-77c0-4dd2-b7bf-e5d8fa152816",
    "Rob Alexander",
);

// CEI 287 — Volcanic Island (reprint)
const VOLCANIC_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &beta::VOLCANIC_ISLAND,
    "159a1d59-9bc4-4e95-addb-eab28c9d6542",
    "Brian Snõddy",
);

// CEI 288 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "0a8d9f84-ddd0-4475-8501-06f88e697b9b",
    "Jesper Myrfors",
);

// CEI 289 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "d5ed0f44-f3d2-491e-ba58-6cbc1c96f5ee",
    "Jesper Myrfors",
);

// CEI 290 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "0bb3a7d7-6ea9-4207-8f6e-0f50ad5804e0",
    "Jesper Myrfors",
);

// CEI 291 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "57c9e38a-bb57-49bc-a9c3-9ba6d9dd5438",
    "Mark Poole",
);

// CEI 292 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "5846e763-7df9-4255-aac5-d342da0dbd3a",
    "Mark Poole",
);

// CEI 293 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "4ca25840-1e36-4a68-87d0-c76e7fe4298b",
    "Mark Poole",
);

// CEI 294 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "dee5bcc2-5878-452e-bbc5-d0fea8d19e43",
    "Dan Frazier",
);

// CEI 295 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "09df246f-7386-44a6-9a9b-d53f4217bca0",
    "Dan Frazier",
);

// CEI 296 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "8587372d-48d1-4c5e-9e36-28838986536f",
    "Dan Frazier",
);

// CEI 297 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "2d03b35a-e622-4a5a-880c-a0aff969b12d",
    "Douglas Shuler",
);

// CEI 298 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "ca039e3e-2d55-4b7e-97b4-5c4fca717e00",
    "Douglas Shuler",
);

// CEI 299 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "5deefab6-4734-464a-b930-64447a8490ca",
    "Douglas Shuler",
);

// CEI 300 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "7bbba144-ac7c-43de-9171-6638a91c9f55",
    "Christopher Rush",
);

// CEI 301 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "589de90a-f48e-4368-ba5d-d8dabcdc61ef",
    "Christopher Rush",
);

// CEI 302 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "7ee18815-21af-4cc4-bb2c-9ec60d0c30da",
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
