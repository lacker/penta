//! Revised Edition has no unique catalog records.
//!
//! Cards legal through this printing reuse their earliest built-in definition.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::alpha;
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::card::sets::y1994::antiquities as catalog_atq;

// 3ED 1 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "dffd3a5f-066b-40c2-99e0-dba1771c899d",
    "Dan Frazier",
);

// 3ED 2 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ARMAGEDDON,
    "605e9a62-53e4-4771-9730-56c78237004a",
    "Jesper Myrfors",
);

// 3ED 3 — Balance (reprint)
const BALANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BALANCE,
    "a21b08d4-b43d-4c93-99e7-39dfe83ced91",
    "Mark Poole",
);

// 3ED 4 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "63e06cd7-9f00-4343-86c2-9f74945193c2",
    "Douglas Shuler",
);

// 3ED 5 — Black Ward (reprint)
const BLACK_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_WARD,
    "c5b6b0a4-bda8-422c-bddb-b2a0ba545596",
    "Dan Frazier",
);

// 3ED 6 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLESSING,
    "f64f6100-c26a-4b22-9fa6-ab3f287a94aa",
    "Julie Baroh",
);

// 3ED 7 — Blue Ward (reprint)
const BLUE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_WARD,
    "9b79eaa0-8a15-4828-9ab7-16c2aab5f19f",
    "Dan Frazier",
);

// 3ED 8 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "434f2329-ea4c-41ba-ab62-857076d76442",
    "Dameon Willich",
);

// 3ED 9 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "c498313d-bb29-4ab9-ab2f-31bdc3d9f78a",
    "Jesper Myrfors",
);

// 3ED 10 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "539809fc-fdeb-4345-a920-37fdb782fdd8",
    "Dameon Willich",
);

// 3ED 11 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "251e0407-b49a-4ee5-83a1-1523ff03a7a7",
    "Sandra Everingham",
);

// 3ED 12 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "b66cadb3-705d-44d3-9277-5d53cd42dae1",
    "Mark Tedin",
);

// 3ED 13 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "7f113b87-8569-45b2-b644-fb3f4890c2ca",
    "Douglas Shuler",
);

// 3ED 14 — Conversion (reprint)
const CONVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONVERSION,
    "70182de2-253f-47d4-ac46-bec4a88b578e",
    "Jesper Myrfors",
);

// 3ED 15 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRUSADE,
    "5670c4c0-b8c3-4100-8cd9-c176b29fe01c",
    "Mark Poole",
);

// 3ED 16 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "555011ea-f03a-4815-b593-cc5d92bba7bd",
    "Mark Poole",
);

// 3ED 17 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "41859c6f-1017-42ae-9061-050fe0db9731",
    "Amy Weber",
);

// 3ED 18 — Eye for an Eye (reprint)
const EYE_FOR_AN_EYE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EYE_FOR_AN_EYE,
    "23c35f2c-0442-46f3-966b-667bad6e0e27",
    "Mark Poole",
);

// 3ED 19 — Farmstead (reprint)
const FARMSTEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FARMSTEAD,
    "75a45b69-1b8d-4b66-a0bf-6142172c7d27",
    "Mark Poole",
);

// 3ED 20 — Green Ward (reprint)
const GREEN_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GREEN_WARD,
    "c1270551-607c-4ef3-88e7-45b4e2445045",
    "Dan Frazier",
);

// 3ED 21 — Guardian Angel (reprint)
const GUARDIAN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GUARDIAN_ANGEL,
    "74da83ea-8302-4927-a355-331636950572",
    "Anson Maddocks",
);

// 3ED 22 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "3300b080-a6f5-4a4a-8faf-0206fbfe8988",
    "Dan Frazier",
);

// 3ED 23 — Holy Armor (reprint)
const HOLY_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_ARMOR,
    "e53412cc-2246-47b5-a212-edee7fac4a54",
    "Melissa A. Benson",
);

// 3ED 24 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "37414560-8187-4c5b-8245-05d77b25c454",
    "Anson Maddocks",
);

// 3ED 25 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "9973e59d-09ca-4647-85dd-15838cf63c2d",
    "Mark Poole",
);

// 3ED 26 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "b316838d-2414-4d0c-a25f-132b7462064a",
    "Richard Thomas",
);

// 3ED 27 — Lance (reprint)
const LANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LANCE,
    "d023e930-f974-40a8-8832-9357350bc7ae",
    "Rob Alexander",
);

// 3ED 28 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "ce7e6bfb-9038-48b5-bfaf-9450c503c69e",
    "Melissa A. Benson",
);

// 3ED 29 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "5ef69d3f-cbad-4069-82dc-4dbfb35377f4",
    "Douglas Shuler",
);

// 3ED 30 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "a450ce98-4854-4378-8809-27019b3800c2",
    "Cornelius Brudi",
);

// 3ED 31 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "e82a9dd8-653a-4410-b78f-303aaf69d11e",
    "Kev Brockschmidt",
);

// 3ED 32 — Purelace (reprint)
const PURELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PURELACE,
    "b9e9b348-121b-42d3-b7be-d6a83dca9157",
    "Sandra Everingham",
);

// 3ED 33 — Red Ward (reprint)
const RED_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_WARD,
    "e6a8157a-4c5e-4a08-83e3-c8fafa7a828b",
    "Dan Frazier",
);

// 3ED 34 — Resurrection (reprint)
const RESURRECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RESURRECTION,
    "c159804d-8757-4172-a661-4f9ee068fce1",
    "Dan Frazier",
);

// 3ED 35 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "04e97dbe-39e1-4bf7-815a-839048463682",
    "Dameon Willich",
);

// 3ED 36 — Reverse Polarity (reprint)
const REVERSE_POLARITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::REVERSE_POLARITY,
    "e77b2e9b-4db2-4dd2-ae55-bddb22ff87e5",
    "Justin Hampton",
);

// 3ED 37 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "d85954b4-306e-46d3-b913-240e16acdcac",
    "Douglas Shuler",
);

// 3ED 38 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "7545fe51-dbe5-4d4c-87a8-86d54734bf33",
    "Tom Wänerstrand",
);

// 3ED 39 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAVANNAH_LIONS,
    "ad41b1aa-1482-4d71-990b-031b30685cb1",
    "Daniel Gelon",
);

// 3ED 40 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SERRA_ANGEL,
    "97fa5f07-46ba-408d-a861-bdb1791cc188",
    "Douglas Shuler",
);

// 3ED 41 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWORDS_TO_PLOWSHARES,
    "057d2410-30d3-4b7a-9dc3-f2512c1cf31c",
    "Jeff A. Menges",
);

// 3ED 42 — Veteran Bodyguard (reprint)
const VETERAN_BODYGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VETERAN_BODYGUARD,
    "d55a1479-6654-4e8e-9a27-44e23753f8be",
    "Douglas Shuler",
);

// 3ED 43 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "4390978b-f647-4720-8caa-00eeecff8471",
    "Mark Tedin",
);

// 3ED 44 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_KNIGHT,
    "ce573cee-40e0-4740-8b86-538ad8a16bce",
    "Daniel Gelon",
);

// 3ED 45 — White Ward (reprint)
const WHITE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_WARD,
    "bb38dcd5-4f12-461c-96a8-867a5b63c5c1",
    "Dan Frazier",
);

// 3ED 46 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WRATH_OF_GOD,
    "1c687a4e-a3f9-4d2d-9931-bf60e97f4095",
    "Quinton Hoover",
);

// 3ED 47 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "22905294-4ba6-4567-a21f-f53b8317acda",
    "Richard Thomas",
);

// 3ED 48 — Animate Artifact (reprint)
const ANIMATE_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANIMATE_ARTIFACT,
    "9704b5e2-43e8-4a80-a34b-dfcaad9ec0f9",
    "Douglas Shuler",
);

// 3ED 49 — Blue Elemental Blast (reprint)
const BLUE_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_ELEMENTAL_BLAST,
    "0892ec35-8bab-4fe5-8cc9-a25032d4bc8d",
    "Richard Thomas",
);

// 3ED 50 — Braingeyser (reprint)
const BRAINGEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BRAINGEYSER,
    "f77f61a8-0b20-4f2e-8a24-844dc95c3a9e",
    "Mark Tedin",
);

// 3ED 51 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLONE,
    "b59fde1a-8d41-4f09-a4a1-4a15aaa704c7",
    "Julie Baroh",
);

// 3ED 52 — Control Magic (reprint)
const CONTROL_MAGIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTROL_MAGIC,
    "d8ab7fb5-9903-4723-a4a0-d142ef3aae8e",
    "Dameon Willich",
);

// 3ED 53 — Copy Artifact (reprint)
const COPY_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COPY_ARTIFACT,
    "0d42f473-3e3f-4441-b7ee-6819a3a8f52e",
    "Amy Weber",
);

// 3ED 54 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "0a1b4e2e-5459-4fae-81d9-1e882647daac",
    "Mark Poole",
);

// 3ED 55 — Creature Bond (reprint)
const CREATURE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CREATURE_BOND,
    "131b80ad-1ffe-449d-a595-74c65f6605cd",
    "Anson Maddocks",
);

// 3ED 56 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "4a8ffad1-9cb0-4ba6-8ae9-00c3b74b9b3f",
    "Douglas Shuler",
);

// 3ED 57 — Energy Flux (reprint)
const ENERGY_FLUX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ENERGY_FLUX,
    "9c4e6d03-68d5-4275-a76c-078d0a9a2b54",
    "Kaja Foglio",
);

// 3ED 58 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "dea6644f-cd2d-4d2b-b66e-b6f8285d2fe8",
    "Quinton Hoover",
);

// 3ED 59 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "133aaa10-610b-41be-9327-591f517a4baa",
    "Anson Maddocks",
);

// 3ED 60 — Hurkyl's Recall (reprint)
const HURKYLS_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::HURKYLS_RECALL,
    "b871e9a7-ba3a-4891-adc6-68a11a4e4aa6",
    "NéNé Thomas",
);

// 3ED 61 — Island Fish Jasconius (reprint)
const ISLAND_FISH_JASCONIUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ISLAND_FISH_JASCONIUS,
    "11db42ba-f756-439c-bdd3-26e9cd4870a4",
    "Jesper Myrfors",
);

// 3ED 62 — Jump (reprint)
const JUMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUMP,
    "839b7b38-cb95-4406-af50-4d97884e2489",
    "Mark Poole",
);

// 3ED 63 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "f0049925-d95d-40ed-ba02-7f7fbe4cf6b5",
    "Anson Maddocks",
);

// 3ED 64 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "45066539-6bc2-467f-acfb-00938ba837ef",
    "Melissa A. Benson",
);

// 3ED 65 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "61fb9f2d-be6b-4073-91d8-68ca58046da9",
    "Julie Baroh",
);

// 3ED 66 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "4765276e-ad80-4734-b485-36eebf1b6ae1",
    "Dan Frazier",
);

// 3ED 67 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_SHORT,
    "0fec5898-f288-4fb6-a2d3-2ea6d20594bf",
    "Dameon Willich",
);

// 3ED 68 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "2fad0078-f3cb-48a4-9ed4-b658e983314f",
    "Jeff A. Menges",
);

// 3ED 69 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "7954f128-7f3d-4c5e-adea-6ff452186ba4",
    "Mark Poole",
);

// 3ED 70 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "b8c578c4-a67f-45ac-aa13-7fba2a5f5f3f",
    "Dameon Willich",
);

// 3ED 71 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "75cb719c-7b7a-449b-bb1e-372a0e20c7f0",
    "Jesper Myrfors",
);

// 3ED 72 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "06e11710-fc99-4d86-9ca5-9d8c7ab03b24",
    "Tom Wänerstrand",
);

// 3ED 73 — Power Leak (reprint)
const POWER_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_LEAK,
    "f6623e8b-4f4c-49c8-ad48-257b8695c4fe",
    "Drew Tucker",
);

// 3ED 74 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "1134aa48-b288-44ab-9d3a-efee12cb98a4",
    "Richard Thomas",
);

// 3ED 75 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "20f1411b-a5ad-4d49-915b-ad8a21d51342",
    "Douglas Shuler",
);

// 3ED 76 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "47560f18-84fb-4d34-83cf-70e0ed8bf7ff",
    "Brian Snõddy",
);

// 3ED 77 — Reconstruction (reprint)
const RECONSTRUCTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::RECONSTRUCTION,
    "b4ba9d8c-686d-4f93-8001-fca27899651e",
    "Anson Maddocks",
);

// 3ED 78 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "4a05cbae-a0a1-452b-a9d1-a29478e705cd",
    "Jeff A. Menges",
);

// 3ED 79 — Serendib Efreet (reprint)
const SERENDIB_EFREET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SERENDIB_EFREET,
    "35415199-0f1d-4398-a48f-f78697a51105",
    "Jesper Myrfors",
);

// 3ED 80 — Siren's Call (reprint)
const SIREN_S_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIREN_S_CALL,
    "e84406bc-6db7-4672-be8c-307985213cd6",
    "Anson Maddocks",
);

// 3ED 81 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "563194a8-e7b2-4edb-ba02-f84cfd206771",
    "Mark Poole",
);

// 3ED 82 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "20fc852a-77b0-48a6-8343-6cf890da9adb",
    "Brian Snõddy",
);

// 3ED 83 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STASIS,
    "fe4bf26c-cd9c-40e3-8a73-2f17f9a1d0e4",
    "Fay Jones",
);

// 3ED 84 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "df32e7de-dd96-454e-a229-31912d9600e7",
    "Amy Weber",
);

// 3ED 85 — Thoughtlace (reprint)
const THOUGHTLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THOUGHTLACE,
    "851a8475-30d1-466f-b0be-6f1a0f2772b5",
    "Mark Poole",
);

// 3ED 86 — Unstable Mutation (reprint)
const UNSTABLE_MUTATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::UNSTABLE_MUTATION,
    "fc24e53b-5074-4791-9277-46e14a70db3a",
    "Douglas Shuler",
);

// 3ED 87 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "e7bf32d8-dad7-4192-8cb6-ae75d8204ba3",
    "Douglas Shuler",
);

// 3ED 88 — Vesuvan Doppelganger (reprint)
const VESUVAN_DOPPELGANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VESUVAN_DOPPELGANGER,
    "6d528ffd-89b3-44ee-a370-e4b53d6604be",
    "Quinton Hoover",
);

// 3ED 89 — Volcanic Eruption (reprint)
const VOLCANIC_ERUPTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VOLCANIC_ERUPTION,
    "6663f9e2-f752-42cf-97a0-01a14ca0aa1b",
    "Douglas Shuler",
);

// 3ED 90 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "beb3874f-b3dc-41ca-becc-4dcbb0549b33",
    "Richard Thomas",
);

// 3ED 91 — Wall of Water (reprint)
const WALL_OF_WATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WATER,
    "14363981-7c27-49d4-91d6-e2a51b679784",
    "Richard Thomas",
);

// 3ED 92 — Water Elemental (reprint)
const WATER_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WATER_ELEMENTAL,
    "55368e1d-2573-4779-ad7c-027071380447",
    "Jeff A. Menges",
);

// 3ED 93 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "eed73f84-ad08-44f8-a4fe-cd324ec1da92",
    "Anson Maddocks",
);

// 3ED 94 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "9cb767eb-2161-4068-be80-c3cf68945393",
    "Jesper Myrfors",
);

// 3ED 95 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_KNIGHT,
    "eaa55b3c-acf4-4d2a-9f32-c7fce6672f3d",
    "Jeff A. Menges",
);

// 3ED 96 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "1173349b-beb9-44c8-aeb2-534ecf54fea0",
    "Jeff A. Menges",
);

// 3ED 97 — Contract from Below (reprint)
const CONTRACT_FROM_BELOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTRACT_FROM_BELOW,
    "0f1a0d5c-bf25-49ff-8af0-cdb2e00c50d9",
    "Douglas Shuler",
);

// 3ED 98 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "9640785c-be0d-4502-9995-f93ac00f1b2f",
    "Jesper Myrfors",
);

// 3ED 99 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "48cb9467-657e-453f-afc8-1bf7121570ad",
    "Sandra Everingham",
);

// 3ED 100 — Darkpact (reprint)
const DARKPACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARKPACT,
    "f550c4e2-a9ba-4bd6-9ba6-94fa5b02e27a",
    "Quinton Hoover",
);

// 3ED 101 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "e42c32a2-9bbb-4701-85af-eb9686edce73",
    "Anson Maddocks",
);

// 3ED 102 — Deathlace (reprint)
const DEATHLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHLACE,
    "e4265106-78a8-4a10-a2fb-6440a8a7f5ce",
    "Sandra Everingham",
);

// 3ED 103 — Demonic Attorney (reprint)
const DEMONIC_ATTORNEY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_ATTORNEY,
    "6ece570a-f480-40fd-a2b1-26a89c44e732",
    "Daniel Gelon",
);

// 3ED 104 — Demonic Hordes (reprint)
const DEMONIC_HORDES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_HORDES,
    "b4cfeebf-d893-4fdf-b3fc-f1f9528f4d04",
    "Jesper Myrfors",
);

// 3ED 105 — Demonic Tutor (reprint)
const DEMONIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_TUTOR,
    "881e5922-b464-4a1a-b074-664bd6c0a7f6",
    "Douglas Shuler",
);

// 3ED 106 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_LIFE,
    "d89c1d2f-87a1-4463-af21-b837da3e7d74",
    "Douglas Shuler",
);

// 3ED 107 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "59145073-2cfd-4153-a6d8-47ad42e739c3",
    "Sandra Everingham",
);

// 3ED 108 — El-Hajjâj (reprint)
const EL_HAJJAJ_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EL_HAJJAJ,
    "c3591170-645f-4645-bc39-b90b7b6ddac7",
    "Dameon Willich",
);

// 3ED 109 — Erg Raiders (reprint)
const ERG_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ERG_RAIDERS,
    "02104733-fb20-43bb-8370-1993528abbdf",
    "Dameon Willich",
);

// 3ED 110 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "d84730b2-53e4-45eb-9ac7-4557a59be5d4",
    "Sandra Everingham",
);

// 3ED 111 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "6c6b2afc-d4f5-47fb-abda-fc3de8bdacaa",
    "Mark Poole",
);

// 3ED 112 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "6cba931e-94b5-4fcc-8d5f-eb60664baf31",
    "Douglas Shuler",
);

// 3ED 113 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "fb2ce26e-8c53-4687-a80c-ba6a1c76299a",
    "Dan Frazier",
);

// 3ED 114 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "d954e8a7-6b22-4f53-9435-ff1f7782a3d4",
    "Mark Poole",
);

// 3ED 115 — Hypnotic Specter (reprint)
const HYPNOTIC_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HYPNOTIC_SPECTER,
    "2c8bd2bc-f48d-43c4-b2aa-5a0905656e90",
    "Douglas Shuler",
);

// 3ED 116 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "5b61cb02-7eb7-4d85-8ced-7978cb1a81d2",
    "Mark Tedin",
);

// 3ED 117 — Mind Twist (reprint)
const MIND_TWIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MIND_TWIST,
    "3230ac66-cb75-43cc-b652-b28e2962d163",
    "Julie Baroh",
);

// 3ED 118 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "cd07c415-4f39-4011-b94a-4aab56dca7d7",
    "Christopher Rush",
);

// 3ED 119 — Nettling Imp (reprint)
const NETTLING_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETTLING_IMP,
    "94c40a45-6439-4405-8562-11a9000a1061",
    "Quinton Hoover",
);

// 3ED 120 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "659c0edb-3afa-4f87-8a94-9fe10578ea1a",
    "Melissa A. Benson",
);

// 3ED 121 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "dbe8939d-c2f0-4dbc-b7dd-0483208f6876",
    "Anson Maddocks",
);

// 3ED 122 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "b6647e7d-b0ad-4170-8dce-ea4c89897c6a",
    "Jesper Myrfors",
);

// 3ED 123 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "47e21390-c661-4717-bbb9-71eb63c6f01e",
    "Anson Maddocks",
);

// 3ED 124 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "6f3c2902-e2c5-4618-9d4e-3fca34b610c8",
    "Jeff A. Menges",
);

// 3ED 125 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROYAL_ASSASSIN,
    "4945ec9e-eda7-42ad-88b7-ba14f9d95e54",
    "Tom Wänerstrand",
);

// 3ED 126 — Sacrifice (reprint)
const SACRIFICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SACRIFICE,
    "76bc3b43-158c-420e-a3fb-7413334699ca",
    "Dan Frazier",
);

// 3ED 127 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "6cbe576f-03d5-4d22-947a-187d9e20425d",
    "Jesper Myrfors",
);

// 3ED 128 — Scavenging Ghoul (reprint)
const SCAVENGING_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCAVENGING_GHOUL,
    "33a5daf6-ce4f-4d00-8458-b7d1a9e037bc",
    "Jeff A. Menges",
);

// 3ED 129 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SENGIR_VAMPIRE,
    "fa35113b-5242-41f4-a989-42f2cd8002b6",
    "Anson Maddocks",
);

// 3ED 130 — Simulacrum (reprint)
const SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIMULACRUM,
    "bc1b6d40-fdb0-40c3-983d-67a7bfb96cea",
    "Mark Poole",
);

// 3ED 131 — Sorceress Queen (reprint)
const SORCERESS_QUEEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::SORCERESS_QUEEN,
    "b83d7331-e573-4dea-901a-de9150d4b5c0",
    "Kaja Foglio",
);

// 3ED 132 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TERROR,
    "0eaf0ac8-f5a7-4689-8d3e-dd865763df44",
    "Ron Spencer",
);

// 3ED 133 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "3ac35077-91e9-446c-9cb2-e2cfb9fa2962",
    "Douglas Shuler",
);

// 3ED 134 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "4f9d6c2b-3492-4360-90cf-649608d4910f",
    "Anson Maddocks",
);

// 3ED 135 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "948a03b0-ce48-4fac-816f-8224b7ae936a",
    "Amy Weber",
);

// 3ED 136 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "6774a228-ec9e-47d0-bc43-a92f5caf8398",
    "Douglas Shuler",
);

// 3ED 137 — Will-o'-the-Wisp (reprint)
const WILL_O_THE_WISP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILL_O_THE_WISP,
    "551e5fdd-ed3a-4f44-b4d1-97900ef46373",
    "Jesper Myrfors",
);

// 3ED 138 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "e868767f-b62e-4bb4-95e5-62feac05ff9d",
    "Jeff A. Menges",
);

// 3ED 139 — Atog (reprint)
const ATOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ATOG,
    "976121fd-a21d-42cd-a7d0-310c8648e307",
    "Jesper Myrfors",
);

// 3ED 140 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "c2b99b30-a972-4e5d-a772-06884719ac7c",
    "Mark Poole",
);

// 3ED 141 — Chaoslace (reprint)
const CHAOSLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHAOSLACE,
    "37df20a7-9299-434c-84ca-8019851ee31b",
    "Dameon Willich",
);

// 3ED 142 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "ce71d4c8-3835-4065-8089-82a64846dbcb",
    "Anson Maddocks",
);

// 3ED 143 — Dragon Whelp (reprint)
const DRAGON_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAGON_WHELP,
    "643d6f5b-6a17-434d-945a-6b9a05015493",
    "Amy Weber",
);

// 3ED 144 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "6caeadf8-1b40-497d-be7c-667fbb98f848",
    "Douglas Shuler",
);

// 3ED 145 — Dwarven Weaponsmith (reprint)
const DWARVEN_WEAPONSMITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DWARVEN_WEAPONSMITH,
    "c83929b1-4826-4b84-823d-0997560b6bdc",
    "Mark Poole",
);

// 3ED 146 — Earth Elemental (reprint)
const EARTH_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTH_ELEMENTAL,
    "e2285cf5-f1c0-42d2-8203-297d2a5b9ec2",
    "Dan Frazier",
);

// 3ED 147 — Earthbind (reprint)
const EARTHBIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHBIND,
    "7ec1650e-8ecb-460a-9319-0f59de48c824",
    "Quinton Hoover",
);

// 3ED 148 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHQUAKE,
    "603d1f86-2098-4af5-a038-c5a314ba7184",
    "Dan Frazier",
);

// 3ED 149 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "63539181-5393-41b8-baf3-9a690d17f4ce",
    "Melissa A. Benson",
);

// 3ED 150 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBALL,
    "dafb512f-536f-4f96-8440-03f1d20d8a5a",
    "Mark Tedin",
);

// 3ED 151 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "16682a6e-8d86-4ad6-a6b1-3171000cc708",
    "Dan Frazier",
);

// 3ED 152 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "7b23de17-d867-41f8-b965-9b3eb00db701",
    "Dameon Willich",
);

// 3ED 153 — Fork (reprint)
const FORK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORK,
    "a33a1695-db21-4dc5-9dc1-dd05d12e6b40",
    "Amy Weber",
);

// 3ED 154 — Goblin Balloon Brigade (reprint)
const GOBLIN_BALLOON_BRIGADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_BALLOON_BRIGADE,
    "c31c14c0-71a4-40e0-b447-6c7124c84059",
    "Andi Rusu",
);

// 3ED 155 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GOBLIN_KING,
    "e3094187-d666-414b-a1fd-ae0ef55c3fcb",
    "Jesper Myrfors",
);

// 3ED 156 — Granite Gargoyle (reprint)
const GRANITE_GARGOYLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRANITE_GARGOYLE,
    "03dfa7f7-8f08-49f6-96fd-eebf16ceb499",
    "Christopher Rush",
);

// 3ED 157 — Gray Ogre (reprint)
const GRAY_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRAY_OGRE,
    "e26041ad-b326-40e6-a7fd-eacfcb0ab17e",
    "Dan Frazier",
);

// 3ED 158 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "c987a3ec-a775-4140-ad49-18025e59dc3d",
    "Dan Frazier",
);

// 3ED 159 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "f6de6b0d-dd8c-4ab9-8de3-b083a36b24b7",
    "Anson Maddocks",
);

// 3ED 160 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "ad3abbfb-320a-41ad-808c-dd93964efb44",
    "Kev Brockschmidt",
);

// 3ED 161 — Kird Ape (reprint)
const KIRD_APE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::KIRD_APE,
    "967a26e0-8dca-4215-9935-b77a7dd4dde0",
    "Ken Meyer, Jr.",
);

// 3ED 162 — Lightning Bolt (reprint)
const LIGHTNING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIGHTNING_BOLT,
    "cb9b9a9d-ae4c-4e04-bf9d-cae48f01292c",
    "Christopher Rush",
);

// 3ED 163 — Magnetic Mountain (reprint)
const MAGNETIC_MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::MAGNETIC_MOUNTAIN,
    "dc95e03d-5521-4a01-8028-200b8467ce86",
    "Susan Van Camp",
);

// 3ED 164 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "b59d2329-5a0c-407b-aed2-2e19feaf70ed",
    "Christopher Rush",
);

// 3ED 165 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "8b16d8b7-3ff8-4481-bd4a-aa283b78bead",
    "Christopher Rush",
);

// 3ED 166 — Mijae Djinn (reprint)
const MIJAE_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::MIJAE_DJINN,
    "7e0c6c15-fba2-447a-a84c-01bb837b912e",
    "Susan Van Camp",
);

// 3ED 167 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "6e81e219-c840-4844-be87-0449ab0fa645",
    "Jeff A. Menges",
);

// 3ED 168 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "574650db-2af2-4e80-a83a-a20584e3a8a9",
    "Anson Maddocks",
);

// 3ED 169 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "71a941f4-3bdc-40b6-8b24-d73136283f51",
    "Dan Frazier",
);

// 3ED 170 — Power Surge (reprint)
const POWER_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SURGE,
    "9f521a52-f4e3-4043-9db3-a3c89afce3b9",
    "Douglas Shuler",
);

// 3ED 171 — Red Elemental Blast (reprint)
const RED_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_ELEMENTAL_BLAST,
    "2d83a2a3-5495-4457-8eb7-9f9a75da6cc3",
    "Richard Thomas",
);

// 3ED 172 — Roc of Kher Ridges (reprint)
const ROC_OF_KHER_RIDGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROC_OF_KHER_RIDGES,
    "ad034b0a-655b-465b-a8bb-3d4eee59abdf",
    "Andi Rusu",
);

// 3ED 173 — Rock Hydra (reprint)
const ROCK_HYDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROCK_HYDRA,
    "c2a08993-d6c5-45ad-82dc-093c8b912a56",
    "Jeff A. Menges",
);

// 3ED 174 — Sedge Troll (reprint)
const SEDGE_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEDGE_TROLL,
    "485d3707-59ce-4350-9ec8-9df232f88c04",
    "Dan Frazier",
);

// 3ED 175 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "93ff3216-aaaf-4c8f-8355-82e1fc61a747",
    "Amy Weber",
);

// 3ED 176 — Shatterstorm (reprint)
const SHATTERSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::SHATTERSTORM,
    "52e5e508-afc6-409d-b912-33cf1d2351d1",
    "Dan Frazier",
);

// 3ED 177 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "69199dd2-dbac-4039-b4da-eb2b0671645f",
    "Melissa A. Benson",
);

// 3ED 178 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SMOKE,
    "7b20087c-355d-4157-bd4e-b4dc2be49b69",
    "Jesper Myrfors",
);

// 3ED 179 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_GIANT,
    "a3b8a84d-44d8-4ad3-b04d-a94634e25453",
    "Dameon Willich",
);

// 3ED 180 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "35c7176a-694c-4e1d-8dca-dcd718d94250",
    "Daniel Gelon",
);

// 3ED 181 — Tunnel (reprint)
const TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNNEL,
    "7b379eeb-4d7e-4421-8fc4-b5255eb373f5",
    "Dan Frazier",
);

// 3ED 182 — Uthden Troll (reprint)
const UTHDEN_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UTHDEN_TROLL,
    "0403aef5-b5f3-4d07-a350-4874801b27e8",
    "Douglas Shuler",
);

// 3ED 183 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "f7fbd53b-d1b2-41b7-a402-91227670a1d7",
    "Richard Thomas",
);

// 3ED 184 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "667c227f-a3b7-4040-8b67-75a6fc209e67",
    "Dan Frazier",
);

// 3ED 185 — Wheel of Fortune (reprint)
const WHEEL_OF_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHEEL_OF_FORTUNE,
    "c14c07d4-6971-483a-add1-f3cdf18feae9",
    "Daniel Gelon",
);

// 3ED 186 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "7248db64-c901-4e87-9322-e122d2d32ddc",
    "Jeff A. Menges",
);

// 3ED 187 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BIRDS_OF_PARADISE,
    "01e7b0bc-1c6c-48f4-8b72-1a809f536c6c",
    "Mark Poole",
);

// 3ED 188 — Channel (reprint)
const CHANNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHANNEL,
    "bc0cea66-7c61-4308-af2f-3622fbb82983",
    "Richard Thomas",
);

// 3ED 189 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "3c5d9117-135f-4a88-950a-41bf164ebc21",
    "Dan Frazier",
);

// 3ED 190 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "a5e4a23b-3b05-4240-9565-bdd8f3f3ef12",
    "Daniel Gelon",
);

// 3ED 191 — Crumble (reprint)
const CRUMBLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::CRUMBLE,
    "32123652-4f71-4f0b-b317-39e6df039b4f",
    "Jesper Myrfors",
);

// 3ED 192 — Desert Twister (reprint)
const DESERT_TWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DESERT_TWISTER,
    "88cbcf7e-9d66-4e1b-b056-8edf708fca84",
    "Susan Van Camp",
);

// 3ED 193 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "24547919-0272-4502-9b3a-e9a0eb6a90d2",
    "Anson Maddocks",
);

// 3ED 194 — Fastbond (reprint)
const FASTBOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FASTBOND,
    "c71123b5-6be5-4c3c-972a-0aad3db1a694",
    "Mark Poole",
);

// 3ED 195 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "637cbc3f-f2c0-42db-b9f9-6c084846cb03",
    "Jesper Myrfors",
);

// 3ED 196 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "56cb88db-2c6b-4d17-be16-2a89218efe4c",
    "Douglas Shuler",
);

// 3ED 197 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "6f80ad09-ebff-47cb-93ab-9bc7e0e10056",
    "Daniel Gelon",
);

// 3ED 198 — Gaea's Liege (reprint)
const GAEA_S_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAEA_S_LIEGE,
    "6c36ac7d-2ff0-4350-9b10-968f94b19842",
    "Dameon Willich",
);

// 3ED 199 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "d33fe386-d165-4874-aa6b-07b7df9b6209",
    "Sandra Everingham",
);

// 3ED 200 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "5440ff00-e7fa-46ac-b46c-3fa4e10712b0",
    "Sandra Everingham",
);

// 3ED 201 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "886959ca-83fd-4b50-a99a-08ef0c5415db",
    "Jeff A. Menges",
);

// 3ED 202 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "bfba7442-ffdf-43cf-97b3-c69ff80e6fde",
    "Dameon Willich",
);

// 3ED 203 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "d919330b-5023-4b9c-b82f-20095354326c",
    "Dameon Willich",
);

// 3ED 204 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "6e6cfaae-ea9e-4c54-858e-381f8bf441a9",
    "Jesper Myrfors",
);

// 3ED 205 — Kudzu (reprint)
const KUDZU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KUDZU,
    "b1466b4c-407d-4220-b5ee-474d7d8a24a7",
    "Mark Poole",
);

// 3ED 206 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "ea2c2bf3-357d-4595-9b24-3451bd2d0179",
    "Sandra Everingham",
);

// 3ED 207 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "ca28f9ec-897a-46fd-8e3d-16330ad43d24",
    "Dameon Willich",
);

// 3ED 208 — Lifelace (reprint)
const LIFELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFELACE,
    "5fc40d6f-1f1c-4f50-8971-9de5f477038b",
    "Amy Weber",
);

// 3ED 209 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "e62fec16-b5a0-47a6-9ccf-cebe79043627",
    "Anson Maddocks",
);

// 3ED 210 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "aa710039-5378-440c-b584-e9d72d1683c9",
    "Jesper Myrfors",
);

// 3ED 211 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LLANOWAR_ELVES,
    "6d6deae3-3ed4-47eb-bf4a-4a766ce18135",
    "Anson Maddocks",
);

// 3ED 212 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "3561cc9a-9270-4c75-90ca-1425b2724abc",
    "Anson Maddocks",
);

// 3ED 213 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "6cd37ba7-b821-444e-b31b-aa667a8914e9",
    "Quinton Hoover",
);

// 3ED 214 — Regrowth (reprint)
const REGROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGROWTH,
    "396aae79-41d5-4b16-8903-5af8fde65eee",
    "Dameon Willich",
);

// 3ED 215 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCRYB_SPRITES,
    "1b9e1d37-47cd-41d7-9fee-b8504c689462",
    "Amy Weber",
);

// 3ED 216 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "bf1889aa-59e9-4d67-ab53-9aec071ab67a",
    "Anson Maddocks",
);

// 3ED 217 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "bab7e4b7-acdd-4316-ae64-b182e4d9cacd",
    "Mark Poole",
);

// 3ED 218 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "05a60435-7b5e-47c1-8186-1ca30a243992",
    "Dan Frazier",
);

// 3ED 219 — Timber Wolves (reprint)
const TIMBER_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TIMBER_WOLVES,
    "3aea108b-367b-43d9-b50d-c18954b2a82d",
    "Melissa A. Benson",
);

// 3ED 220 — Titania's Song (reprint)
const TITANIA_S_SONG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::TITANIA_S_SONG,
    "c022abd7-bb1a-4f61-b4e1-6b802d337484",
    "Kerstin Kaman",
);

// 3ED 221 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "5722c349-bf3d-4ac0-8fdd-bf170401c419",
    "Douglas Shuler",
);

// 3ED 222 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "6a578955-7f77-42f1-a3e3-5e2b46216c43",
    "Richard Thomas",
);

// 3ED 223 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "354de08d-41a8-4d6c-85d6-2413393ac181",
    "Kev Brockschmidt",
);

// 3ED 224 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "b27862c3-8589-41ab-8f84-34727e5a93be",
    "Anson Maddocks",
);

// 3ED 225 — Wall of Ice (reprint)
const WALL_OF_ICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_ICE,
    "b0af9d4c-b3e5-4953-b4ca-7f34f67bdbeb",
    "Richard Thomas",
);

// 3ED 226 — Wall of Wood (reprint)
const WALL_OF_WOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WOOD,
    "9d4f8eb6-2c3c-49e7-a41d-33c138d853c9",
    "Mark Tedin",
);

// 3ED 227 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "3fd08a5d-0dad-4bce-86c0-dea431038859",
    "Cornelius Brudi",
);

// 3ED 228 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "5c99e36f-b11d-4270-8b88-66be8907c9bd",
    "Jeff A. Menges",
);

// 3ED 229 — Web (reprint)
const WEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEB,
    "00012bd8-ed68-4978-a22d-f450c8a6e048",
    "Rob Alexander",
);

// 3ED 230 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "8000c8f8-d4c3-4dbc-a73e-9b82b0478061",
    "Mark Poole",
);

// 3ED 231 — Aladdin's Lamp (reprint)
const ALADDIN_S_LAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDIN_S_LAMP,
    "2c7e444a-dba1-406f-bfdc-1a54102083a8",
    "Mark Tedin",
);

// 3ED 232 — Aladdin's Ring (reprint)
const ALADDINS_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ALADDINS_RING,
    "40cb7c36-135b-40d0-bc7b-62fbcd508f49",
    "Dan Frazier",
);

// 3ED 233 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANKH_OF_MISHRA,
    "617599f1-69d9-4767-9656-982739728df0",
    "Amy Weber",
);

// 3ED 234 — Armageddon Clock (reprint)
const ARMAGEDDON_CLOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ARMAGEDDON_CLOCK,
    "ee486c26-f0bc-4275-ab1b-a9e57721f036",
    "Amy Weber",
);

// 3ED 235 — Basalt Monolith (reprint)
const BASALT_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BASALT_MONOLITH,
    "4f0b7b8e-45b0-4947-9a95-bccc6b725a37",
    "Jesper Myrfors",
);

// 3ED 236 — Black Vise (reprint)
const BLACK_VISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_VISE,
    "1bae1867-d5bb-4204-9fb1-59d6663bc161",
    "Richard Thomas",
);

// 3ED 237 — Bottle of Suleiman (reprint)
const BOTTLE_OF_SULEIMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BOTTLE_OF_SULEIMAN,
    "6c1c2ea2-09ba-4a0e-b5b6-c06068f0da75",
    "Jesper Myrfors",
);

// 3ED 238 — Brass Man (reprint)
const BRASS_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::BRASS_MAN,
    "0ba1daee-a5ac-4d9e-b681-3e3c7a3eb095",
    "Christopher Rush",
);

// 3ED 239 — Celestial Prism (reprint)
const CELESTIAL_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CELESTIAL_PRISM,
    "2bc5e073-2903-4a28-9c23-07a0482ae09a",
    "Amy Weber",
);

// 3ED 240 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "8224d6e3-c9de-4129-ae3e-300a82c4bd00",
    "Drew Tucker",
);

// 3ED 241 — Conservator (reprint)
const CONSERVATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSERVATOR,
    "2e7d8bc5-9d87-43e3-9b81-311d01fdf0e5",
    "Amy Weber",
);

// 3ED 242 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "f973046b-ce81-4e35-89f3-a6e857d751b8",
    "Amy Weber",
);

// 3ED 243 — Dancing Scimitar (reprint)
const DANCING_SCIMITAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::DANCING_SCIMITAR,
    "e3d92537-7934-4191-8836-2f61ff6ab2fa",
    "Anson Maddocks",
);

// 3ED 244 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "ce56c997-202c-4175-809b-2dd65cd2ab2a",
    "Dan Frazier",
);

// 3ED 245 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "af4f8926-9a9e-4b2d-8224-118655f12809",
    "Dan Frazier",
);

// 3ED 246 — Dragon Engine (reprint)
const DRAGON_ENGINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::DRAGON_ENGINE,
    "42c1fd91-001d-4c94-bb0a-d3fc570c7f12",
    "Anson Maddocks",
);

// 3ED 247 — Ebony Horse (reprint)
const EBONY_HORSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::EBONY_HORSE,
    "396360d2-3604-499c-9fc3-75b75970c047",
    "Dameon Willich",
);

// 3ED 248 — Flying Carpet (reprint)
const FLYING_CARPET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::FLYING_CARPET,
    "7e46b461-a38e-44f8-8e15-7cefe8aea46a",
    "Mark Tedin",
);

// 3ED 249 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLASSES_OF_URZA,
    "a7d975c6-ca94-4255-8ac5-56113da9f97e",
    "Douglas Shuler",
);

// 3ED 250 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "1a3068eb-2250-4c7b-8ed5-2366ff6cd0e1",
    "Mark Tedin",
);

// 3ED 251 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "fc6fbf54-698d-4a99-ad98-b0115df403a0",
    "Mark Poole",
);

// 3ED 252 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRON_STAR,
    "5ffb4de8-505e-4e83-8a8b-05c968a03f04",
    "Dan Frazier",
);

// 3ED 253 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "b1dd930a-a7d8-4cdd-9c4f-78b2b249ce38",
    "Anson Maddocks",
);

// 3ED 254 — Ivory Tower (reprint)
const IVORY_TOWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::IVORY_TOWER,
    "2bd6f6a8-153f-4263-941a-e3387c2a22ad",
    "Margaret Organ-Kean",
);

// 3ED 255 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "4ff44808-a1a9-4173-a39c-d726c51490fb",
    "Anson Maddocks",
);

// 3ED 256 — Jandor's Ring (reprint)
const JANDOR_S_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JANDOR_S_RING,
    "2b56c9ed-c912-4829-9be0-e80303759c9c",
    "Dan Frazier",
);

// 3ED 257 — Jandor's Saddlebags (reprint)
const JANDORS_SADDLEBAGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::JANDORS_SADDLEBAGS,
    "af96e332-1c77-4650-b66b-417e6c47bc3b",
    "Dameon Willich",
);

// 3ED 258 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JAYEMDAE_TOME,
    "e8661e0a-faf8-4c16-b988-55622707de6f",
    "Mark Tedin",
);

// 3ED 259 — Juggernaut (reprint)
const JUGGERNAUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUGGERNAUT,
    "490fafd4-3cd0-4cd8-9f07-01a92121d39d",
    "Dan Frazier",
);

// 3ED 260 — Kormus Bell (reprint)
const KORMUS_BELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KORMUS_BELL,
    "7071d294-842c-4539-aba1-c68cd5c79848",
    "Christopher Rush",
);

// 3ED 261 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "0634ab23-4691-4c77-9b8f-bfd9d99b31a1",
    "Daniel Gelon",
);

// 3ED 262 — Living Wall (reprint)
const LIVING_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_WALL,
    "516858c9-7679-4a65-a787-36a2cf175ede",
    "Anson Maddocks",
);

// 3ED 263 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_VAULT,
    "5cbc686e-a8ef-40de-b79a-803ef42f8384",
    "Mark Tedin",
);

// 3ED 264 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "56854867-d135-4bed-8d3a-dcc24d757558",
    "Quinton Hoover",
);

// 3ED 265 — Millstone (reprint)
const MILLSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MILLSTONE,
    "21aabfa6-c299-4cf8-b8b5-097ef6f4029a",
    "Kaja Foglio",
);

// 3ED 266 — Mishra's War Machine (reprint)
const MISHRA_S_WAR_MACHINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::MISHRA_S_WAR_MACHINE,
    "1a93e9bd-6c31-4363-93b0-b7d355bd2867",
    "Amy Weber",
);

// 3ED 267 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NEVINYRRALS_DISK,
    "ba5fcfc5-0715-4c6c-8325-3b54a138634e",
    "Mark Tedin",
);

// 3ED 268 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "ef24fb75-49c7-48eb-a0b3-dc08d7f691ec",
    "Jesper Myrfors",
);

// 3ED 269 — Onulet (reprint)
const ONULET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ONULET,
    "0d84e378-dc64-4e69-a49d-1c210ca3506c",
    "Anson Maddocks",
);

// 3ED 270 — Ornithopter (reprint)
const ORNITHOPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ORNITHOPTER,
    "b3654fd6-f8ac-471a-8559-ba4ed0fe75c3",
    "Amy Weber",
);

// 3ED 271 — Primal Clay (reprint)
const PRIMAL_CLAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::PRIMAL_CLAY,
    "d057a91c-d2a7-48ec-aa16-f033499de166",
    "Kaja Foglio",
);

// 3ED 272 — Rocket Launcher (reprint)
const ROCKET_LAUNCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::ROCKET_LAUNCHER,
    "919f184b-421c-413c-a95c-05bb145f93ba",
    "Pete Venters",
);

// 3ED 273 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "964abd0f-812e-418d-a01b-73dc724c8429",
    "Christopher Rush",
);

// 3ED 274 — Sol Ring (reprint)
const SOL_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOL_RING,
    "803fd65f-4ca6-4fe4-abc2-72aa32ebb3a5",
    "Mark Tedin",
);

// 3ED 275 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "3a80be5d-cf6f-487d-8602-9396d9b6252b",
    "Dameon Willich",
);

// 3ED 276 — Sunglasses of Urza (reprint)
const SUNGLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SUNGLASSES_OF_URZA,
    "4babb2a9-b6b3-4a85-8add-90828726adb4",
    "Dan Frazier",
);

// 3ED 277 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "09b82c6b-9b14-4607-95d5-2964b926ec37",
    "Sandra Everingham",
);

// 3ED 278 — The Rack (reprint)
const THE_RACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::THE_RACK,
    "2a600805-dd79-419d-9866-f8c29643f0f8",
    "Richard Thomas",
);

// 3ED 279 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "8b08a20c-59ee-4323-8a00-af88b82d6b76",
    "Anson Maddocks",
);

// 3ED 280 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WINTER_ORB,
    "f5c6d64b-f49c-4b41-bd25-3d29c896a9a8",
    "Mark Tedin",
);

// 3ED 281 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "157a28e2-61f6-4c95-b377-e945fc8dade2",
    "Mark Tedin",
);

// 3ED 282 — Badlands (reprint)
const BADLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BADLANDS,
    "56058359-3c0b-49db-a0ce-9ded4c3f4372",
    "Rob Alexander",
);

// 3ED 283 — Bayou (reprint)
const BAYOU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAYOU,
    "56355ff3-2232-4a11-b868-aec9a50b9ee5",
    "Jesper Myrfors",
);

// 3ED 284 — Plateau (reprint)
const PLATEAU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLATEAU,
    "c6ae9cff-8646-4069-8761-df734e067beb",
    "Cornelius Brudi",
);

// 3ED 285 — Savannah (reprint)
const SAVANNAH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAVANNAH,
    "5ae71290-c133-406c-8b17-9ea22b437806",
    "Rob Alexander",
);

// 3ED 286 — Scrubland (reprint)
const SCRUBLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCRUBLAND,
    "472034a2-0ba9-4876-ab7a-aa7013d603bb",
    "Jesper Myrfors",
);

// 3ED 287 — Taiga (reprint)
const TAIGA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TAIGA,
    "54c5c65a-a444-4e0f-ae44-a3722cdd32a1",
    "Rob Alexander",
);

// 3ED 288 — Tropical Island (reprint)
const TROPICAL_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TROPICAL_ISLAND,
    "a0f5c6bc-65dc-42a1-a62d-a0b101310a1f",
    "Jesper Myrfors",
);

// 3ED 289 — Tundra (reprint)
const TUNDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNDRA,
    "9c9d5f72-e199-4d5b-ae7e-cc5b9bdfae99",
    "Jesper Myrfors",
);

// 3ED 290 — Underground Sea (reprint)
const UNDERGROUND_SEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNDERGROUND_SEA,
    "1f35877c-e66c-4ef0-842a-f68cd233ae4b",
    "Rob Alexander",
);

// 3ED 291 — Volcanic Island (reprint)
const VOLCANIC_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::VOLCANIC_ISLAND,
    "b12e5430-0e80-47dd-80ac-85728b656a24",
    "Brian Snõddy",
);

// 3ED 292 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "275c03f9-f9d2-45c5-a332-b3bee54e7065",
    "Jesper Myrfors",
);

// 3ED 293 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "cc0bcdbe-be63-446a-8838-8790bda308a3",
    "Jesper Myrfors",
);

// 3ED 294 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "a4901260-073a-4d06-8167-c322c55ab210",
    "Jesper Myrfors",
);

// 3ED 295 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "22f6e971-349d-498b-ae01-ab81ce21772c",
    "Mark Poole",
);

// 3ED 296 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "a000777b-e8fe-4fd6-a455-01bc9056a873",
    "Mark Poole",
);

// 3ED 297 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "bec8018f-a12c-4adf-9735-c2093298062d",
    "Mark Poole",
);

// 3ED 298 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "95e936cf-3bbb-4f3b-8e1a-4be1d4702b99",
    "Dan Frazier",
);

// 3ED 299 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "459d175e-2b9c-4f30-be03-4e05cd3c68ef",
    "Dan Frazier",
);

// 3ED 300 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "515eff31-24f6-462e-bbd4-b49540421a75",
    "Dan Frazier",
);

// 3ED 301 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "30345500-d430-4280-bfe3-de297309f136",
    "Douglas Shuler",
);

// 3ED 302 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "5a240d1b-8430-4986-850d-32afa0e812b2",
    "Douglas Shuler",
);

// 3ED 303 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "1b0f41e8-cf27-489b-812a-d566a75cf7f7",
    "Douglas Shuler",
);

// 3ED 304 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "b6e1c2e9-5572-4242-985d-f509d628092b",
    "Christopher Rush",
);

// 3ED 305 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "4d1e4241-42ef-4b51-8f9b-2ab6aca31dbb",
    "Christopher Rush",
);

// 3ED 306 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "b38ce16b-3258-4019-9e86-156e4738aa89",
    "Christopher Rush",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ANIMATE_WALL_REPRINT,
    ARMAGEDDON_REPRINT,
    BALANCE_REPRINT,
    BENALISH_HERO_REPRINT,
    BLACK_WARD_REPRINT,
    BLESSING_REPRINT,
    BLUE_WARD_REPRINT,
    CASTLE_REPRINT,
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    CONVERSION_REPRINT,
    CRUSADE_REPRINT,
    DEATH_WARD_REPRINT,
    DISENCHANT_REPRINT,
    EYE_FOR_AN_EYE_REPRINT,
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
    REVERSE_POLARITY_REPRINT,
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
    ANIMATE_ARTIFACT_REPRINT,
    BLUE_ELEMENTAL_BLAST_REPRINT,
    BRAINGEYSER_REPRINT,
    CLONE_REPRINT,
    CONTROL_MAGIC_REPRINT,
    COPY_ARTIFACT_REPRINT,
    COUNTERSPELL_REPRINT,
    CREATURE_BOND_REPRINT,
    DRAIN_POWER_REPRINT,
    ENERGY_FLUX_REPRINT,
    FEEDBACK_REPRINT,
    FLIGHT_REPRINT,
    HURKYLS_RECALL_REPRINT,
    ISLAND_FISH_JASCONIUS_REPRINT,
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
    PSYCHIC_VENOM_REPRINT,
    RECONSTRUCTION_REPRINT,
    SEA_SERPENT_REPRINT,
    SERENDIB_EFREET_REPRINT,
    SIREN_S_CALL_REPRINT,
    SLEIGHT_OF_MIND_REPRINT,
    SPELL_BLAST_REPRINT,
    STASIS_REPRINT,
    STEAL_ARTIFACT_REPRINT,
    THOUGHTLACE_REPRINT,
    UNSTABLE_MUTATION_REPRINT,
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
    EL_HAJJAJ_REPRINT,
    ERG_RAIDERS_REPRINT,
    EVIL_PRESENCE_REPRINT,
    FEAR_REPRINT,
    FROZEN_SHADE_REPRINT,
    GLOOM_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    HYPNOTIC_SPECTER_REPRINT,
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
    SORCERESS_QUEEN_REPRINT,
    TERROR_REPRINT,
    UNHOLY_STRENGTH_REPRINT,
    WALL_OF_BONE_REPRINT,
    WARP_ARTIFACT_REPRINT,
    WEAKNESS_REPRINT,
    WILL_O_THE_WISP_REPRINT,
    ZOMBIE_MASTER_REPRINT,
    ATOG_REPRINT,
    BURROWING_REPRINT,
    CHAOSLACE_REPRINT,
    DISINTEGRATE_REPRINT,
    DRAGON_WHELP_REPRINT,
    DWARVEN_WARRIORS_REPRINT,
    DWARVEN_WEAPONSMITH_REPRINT,
    EARTH_ELEMENTAL_REPRINT,
    EARTHBIND_REPRINT,
    EARTHQUAKE_REPRINT,
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
    KELDON_WARLORD_REPRINT,
    KIRD_APE_REPRINT,
    LIGHTNING_BOLT_REPRINT,
    MAGNETIC_MOUNTAIN_REPRINT,
    MANA_FLARE_REPRINT,
    MANABARBS_REPRINT,
    MIJAE_DJINN_REPRINT,
    MONSS_GOBLIN_RAIDERS_REPRINT,
    ORCISH_ARTILLERY_REPRINT,
    ORCISH_ORIFLAMME_REPRINT,
    POWER_SURGE_REPRINT,
    RED_ELEMENTAL_BLAST_REPRINT,
    ROC_OF_KHER_RIDGES_REPRINT,
    ROCK_HYDRA_REPRINT,
    SEDGE_TROLL_REPRINT,
    SHATTER_REPRINT,
    SHATTERSTORM_REPRINT,
    SHIVAN_DRAGON_REPRINT,
    SMOKE_REPRINT,
    STONE_GIANT_REPRINT,
    STONE_RAIN_REPRINT,
    TUNNEL_REPRINT,
    UTHDEN_TROLL_REPRINT,
    WALL_OF_FIRE_REPRINT,
    WALL_OF_STONE_REPRINT,
    WHEEL_OF_FORTUNE_REPRINT,
    ASPECT_OF_WOLF_REPRINT,
    BIRDS_OF_PARADISE_REPRINT,
    CHANNEL_REPRINT,
    COCKATRICE_REPRINT,
    CRAW_WURM_REPRINT,
    CRUMBLE_REPRINT,
    DESERT_TWISTER_REPRINT,
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
    REGENERATION_REPRINT,
    REGROWTH_REPRINT,
    SCRYB_SPRITES_REPRINT,
    SHANODIN_DRYADS_REPRINT,
    STREAM_OF_LIFE_REPRINT,
    THICKET_BASILISK_REPRINT,
    TIMBER_WOLVES_REPRINT,
    TITANIA_S_SONG_REPRINT,
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
    ALADDIN_S_LAMP_REPRINT,
    ALADDINS_RING_REPRINT,
    ANKH_OF_MISHRA_REPRINT,
    ARMAGEDDON_CLOCK_REPRINT,
    BASALT_MONOLITH_REPRINT,
    BLACK_VISE_REPRINT,
    BOTTLE_OF_SULEIMAN_REPRINT,
    BRASS_MAN_REPRINT,
    CELESTIAL_PRISM_REPRINT,
    CLOCKWORK_BEAST_REPRINT,
    CONSERVATOR_REPRINT,
    CRYSTAL_ROD_REPRINT,
    DANCING_SCIMITAR_REPRINT,
    DINGUS_EGG_REPRINT,
    DISRUPTING_SCEPTER_REPRINT,
    DRAGON_ENGINE_REPRINT,
    EBONY_HORSE_REPRINT,
    FLYING_CARPET_REPRINT,
    GLASSES_OF_URZA_REPRINT,
    HELM_OF_CHATZUK_REPRINT,
    HOWLING_MINE_REPRINT,
    IRON_STAR_REPRINT,
    IVORY_CUP_REPRINT,
    IVORY_TOWER_REPRINT,
    JADE_MONOLITH_REPRINT,
    JANDOR_S_RING_REPRINT,
    JANDORS_SADDLEBAGS_REPRINT,
    JAYEMDAE_TOME_REPRINT,
    JUGGERNAUT_REPRINT,
    KORMUS_BELL_REPRINT,
    LIBRARY_OF_LENG_REPRINT,
    LIVING_WALL_REPRINT,
    MANA_VAULT_REPRINT,
    MEEKSTONE_REPRINT,
    MILLSTONE_REPRINT,
    MISHRA_S_WAR_MACHINE_REPRINT,
    NEVINYRRALS_DISK_REPRINT,
    OBSIANUS_GOLEM_REPRINT,
    ONULET_REPRINT,
    ORNITHOPTER_REPRINT,
    PRIMAL_CLAY_REPRINT,
    ROCKET_LAUNCHER_REPRINT,
    ROD_OF_RUIN_REPRINT,
    SOL_RING_REPRINT,
    SOUL_NET_REPRINT,
    SUNGLASSES_OF_URZA_REPRINT,
    THE_HIVE_REPRINT,
    THE_RACK_REPRINT,
    THRONE_OF_BONE_REPRINT,
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
