//! Limited Edition Beta card definitions and printings.

use super::{CardRecord, PrintingRecord, alpha};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{AbilityCostDef, CardRules, CardSet, ManaColor, ObjectPredicateDef, abilities};
use crate::mana_cost;

// LEB 1 — Animate Wall (reprint)
const ANIMATE_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_WALL,
    "5c5b4738-20bb-465d-b67e-c6146dce9d0b",
    "Dan Frazier",
);

// LEB 2 — Armageddon (reprint)
const ARMAGEDDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ARMAGEDDON,
    "02c4edfa-7822-40bc-88d1-d051b3a64df1",
    "Jesper Myrfors",
);

// LEB 3 — Balance (reprint)
const BALANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BALANCE,
    "0f2c32a0-ee97-4239-94e3-aabab91dab83",
    "Mark Poole",
);

// LEB 4 — Benalish Hero (reprint)
const BENALISH_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BENALISH_HERO,
    "f62c68d0-9b1e-4abe-991d-a645effeb676",
    "Douglas Shuler",
);

// LEB 5 — Black Ward (reprint)
const BLACK_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLACK_WARD,
    "30d5d3fe-5741-40f7-8f45-dadb818d79b0",
    "Dan Frazier",
);

// LEB 6 — Blaze of Glory (reprint)
const BLAZE_OF_GLORY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLAZE_OF_GLORY,
    "f78aef20-e3bb-484c-9fa1-d2859408b04a",
    "Richard Thomas",
);

// LEB 7 — Blessing (reprint)
const BLESSING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLESSING,
    "bcd624c8-f06e-4181-865e-6a14ffc9302f",
    "Julie Baroh",
);

// LEB 8 — Blue Ward (reprint)
const BLUE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BLUE_WARD,
    "aafae6f4-0880-4532-9224-44545bfa5eb4",
    "Dan Frazier",
);

// LEB 9 — Castle (reprint)
const CASTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CASTLE,
    "a8ba6b09-b24f-40cb-b219-ad8a1fd6692c",
    "Dameon Willich",
);

// LEB 10 — Circle of Protection: Black
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_BLACK: CardRecord = CardRecord::new(
    CardSet::Beta,
    "Circle of Protection: Black",
    "fa47b4cd-8da4-4544-b011-ba92b7009203",
    "Jesper Myrfors",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a black source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::Black),
        ),
    ),
);

// LEB 11 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "07a86eb1-f6a0-4a4e-bd59-e19e22ec487d",
    "Dameon Willich",
);

// LEB 12 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "e041b0ea-4a57-4950-9f8e-72d6e6ab2968",
    "Sandra Everingham",
);

// LEB 13 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "5de9dc85-d566-4cb0-a2e3-1ed4e5fe2f14",
    "Mark Tedin",
);

// LEB 14 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "671aca82-6c55-43ef-b452-d6a2e706a7ae",
    "Douglas Shuler",
);

// LEB 15 — Consecrate Land (reprint)
const CONSECRATE_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSECRATE_LAND,
    "077cf242-f866-497f-a23c-70e1b04a748e",
    "Jeff A. Menges",
);

// LEB 16 — Conversion (reprint)
const CONVERSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONVERSION,
    "4d9a5bb5-23cd-4f9a-8c8e-d009fb7bdf59",
    "Jesper Myrfors",
);

// LEB 17 — Crusade (reprint)
const CRUSADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CRUSADE,
    "2d5fbd9d-48bf-4600-8ca4-2ce2ca48128e",
    "Mark Poole",
);

// LEB 18 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "b119edd8-7801-475e-943a-6cbf10f2d303",
    "Mark Poole",
);

// LEB 19 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DISENCHANT,
    "9d61d0a5-7e92-4413-9121-925e1876b64d",
    "Amy Weber",
);

// LEB 20 — Farmstead (reprint)
const FARMSTEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FARMSTEAD,
    "c49ecc66-dccb-4026-8c6e-0b275a635a1f",
    "Mark Poole",
);

// LEB 21 — Green Ward (reprint)
const GREEN_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GREEN_WARD,
    "a488ce63-1adb-4051-9521-703bad8d02f6",
    "Dan Frazier",
);

// LEB 22 — Guardian Angel (reprint)
const GUARDIAN_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GUARDIAN_ANGEL,
    "9c4e8259-b369-4b59-85fa-fe9edb1887c5",
    "Anson Maddocks",
);

// LEB 23 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "9c9f2eeb-fea5-4b33-9723-8be3c1914f63",
    "Dan Frazier",
);

// LEB 24 — Holy Armor (reprint)
const HOLY_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_ARMOR,
    "6ab1d885-989c-4d71-8139-9e35d2f16d03",
    "Melissa A. Benson",
);

// LEB 25 — Holy Strength (reprint)
const HOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOLY_STRENGTH,
    "de989395-50bf-458a-a010-e12abe2e15a6",
    "Anson Maddocks",
);

// LEB 26 — Island Sanctuary (reprint)
const ISLAND_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND_SANCTUARY,
    "273fb2b6-3d11-4f0d-9fb0-0364353c2060",
    "Mark Poole",
);

// LEB 27 — Karma (reprint)
const KARMA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KARMA,
    "1bea2eb6-dfae-4bdc-9ab3-b2b491c69c59",
    "Richard Thomas",
);

// LEB 28 — Lance (reprint)
const LANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LANCE,
    "a7aa3a93-3765-49f0-8ff2-b6843509c34a",
    "Rob Alexander",
);

// LEB 29 — Mesa Pegasus (reprint)
const MESA_PEGASUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MESA_PEGASUS,
    "55bff46a-6725-4918-9bdf-38efaaf50236",
    "Melissa A. Benson",
);

// LEB 30 — Northern Paladin (reprint)
const NORTHERN_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NORTHERN_PALADIN,
    "4ba8493c-ae69-48d1-a050-a887ae27c83f",
    "Douglas Shuler",
);

// LEB 31 — Pearled Unicorn (reprint)
const PEARLED_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PEARLED_UNICORN,
    "47024d6d-dc55-4c35-b2bb-1b8bb0ee4e38",
    "Cornelius Brudi",
);

// LEB 32 — Personal Incarnation (reprint)
const PERSONAL_INCARNATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PERSONAL_INCARNATION,
    "f7bb9f31-0818-4422-8533-99a4e6845a02",
    "Kev Brockschmidt",
);

// LEB 33 — Purelace (reprint)
const PURELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PURELACE,
    "af11986e-42bd-4f54-8624-7b34b1783a40",
    "Sandra Everingham",
);

// LEB 34 — Red Ward (reprint)
const RED_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RED_WARD,
    "057237bb-e1e6-4bcc-8639-ca0dcdd4846c",
    "Dan Frazier",
);

// LEB 35 — Resurrection (reprint)
const RESURRECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RESURRECTION,
    "50e3c741-5095-48a6-bd93-b9c4db265004",
    "Dan Frazier",
);

// LEB 36 — Reverse Damage (reprint)
const REVERSE_DAMAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REVERSE_DAMAGE,
    "46cf22e4-cc5c-4723-a9cb-ae7ce7a55a1a",
    "Dameon Willich",
);

// LEB 37 — Righteousness (reprint)
const RIGHTEOUSNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RIGHTEOUSNESS,
    "b847a2d1-5912-4f88-a68f-06790d0795dc",
    "Douglas Shuler",
);

// LEB 38 — Samite Healer (reprint)
const SAMITE_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAMITE_HEALER,
    "3fbfb106-29d8-4065-b306-51dba0ed11a4",
    "Tom Wänerstrand",
);

// LEB 39 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH_LIONS,
    "67d1945d-d228-4dc3-a593-859408b2016b",
    "Daniel Gelon",
);

// LEB 40 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SERRA_ANGEL,
    "5669f9c8-2e94-47e2-a551-7efff317fb34",
    "Douglas Shuler",
);

// LEB 41 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWORDS_TO_PLOWSHARES,
    "255099be-c64e-4f6a-8463-4fc058d6908d",
    "Jeff A. Menges",
);

// LEB 42 — Veteran Bodyguard (reprint)
const VETERAN_BODYGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VETERAN_BODYGUARD,
    "d8d888b7-26e2-465d-b5ee-bb2f2af5c621",
    "Douglas Shuler",
);

// LEB 43 — Wall of Swords (reprint)
const WALL_OF_SWORDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_SWORDS,
    "be955e9a-e722-4cd7-8e3d-bab1889c255b",
    "Mark Tedin",
);

// LEB 44 — White Knight (reprint)
const WHITE_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHITE_KNIGHT,
    "a231e0b8-b3e3-4f4a-8baa-c56626b01685",
    "Daniel Gelon",
);

// LEB 45 — White Ward (reprint)
const WHITE_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WHITE_WARD,
    "4988dc3e-2ed8-4de3-9d1b-838003c9c9e3",
    "Dan Frazier",
);

// LEB 46 — Wrath of God (reprint)
const WRATH_OF_GOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WRATH_OF_GOD,
    "96dd2d61-a43d-4582-b730-71d4fac0fa23",
    "Quinton Hoover",
);

// LEB 47 — Air Elemental (reprint)
const AIR_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::AIR_ELEMENTAL,
    "36a94a6d-26b1-4486-9444-ec366e6f4d6e",
    "Richard Thomas",
);

// LEB 48 — Ancestral Recall (reprint)
const ANCESTRAL_RECALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANCESTRAL_RECALL,
    "46b0a5c2-ac85-448e-9e87-12fc74fd4147",
    "Mark Poole",
);

// LEB 49 — Animate Artifact (reprint)
const ANIMATE_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANIMATE_ARTIFACT,
    "cb575b27-d2ca-4d90-a650-dc670484f607",
    "Douglas Shuler",
);

// LEB 50 — Blue Elemental Blast (reprint)
const BLUE_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLUE_ELEMENTAL_BLAST,
    "7f07e272-6cc7-46d6-ad5c-473d1021c179",
    "Richard Thomas",
);

// LEB 51 — Braingeyser (reprint)
const BRAINGEYSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BRAINGEYSER,
    "a5dd8dbb-9538-4786-b20c-0ea2f446f323",
    "Mark Tedin",
);

// LEB 52 — Clone (reprint)
const CLONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLONE,
    "af53b5fc-c31a-4f26-93bf-0c45c1f4e1e5",
    "Julie Baroh",
);

// LEB 53 — Control Magic (reprint)
const CONTROL_MAGIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTROL_MAGIC,
    "133315bd-3c46-4eff-938e-4dba63631c1b",
    "Dameon Willich",
);

// LEB 54 — Copy Artifact (reprint)
const COPY_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPY_ARTIFACT,
    "e24fe07d-1328-4165-b7a0-622b60cec481",
    "Amy Weber",
);

// LEB 55 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COUNTERSPELL,
    "9e11bf7c-f439-4529-b29a-d711359807ef",
    "Mark Poole",
);

// LEB 56 — Creature Bond (reprint)
const CREATURE_BOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CREATURE_BOND,
    "4ce48b24-a65e-42d9-a147-8f89028fada7",
    "Anson Maddocks",
);

// LEB 57 — Drain Power (reprint)
const DRAIN_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRAIN_POWER,
    "9672caeb-5cf8-4b40-a371-005c911a67d9",
    "Douglas Shuler",
);

// LEB 58 — Feedback (reprint)
const FEEDBACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEEDBACK,
    "644288e8-e0b1-418f-b105-01a557a3e497",
    "Quinton Hoover",
);

// LEB 59 — Flight (reprint)
const FLIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLIGHT,
    "24584ffa-8ed1-4930-b6d8-ac1d02738ed0",
    "Anson Maddocks",
);

// LEB 60 — Invisibility (reprint)
const INVISIBILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INVISIBILITY,
    "dde97b8f-7c10-48d3-8ae2-9f86158973ec",
    "Anson Maddocks",
);

// LEB 61 — Jump (reprint)
const JUMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUMP,
    "e51e8a6e-1da8-4e6f-8433-9f0695926f04",
    "Mark Poole",
);

// LEB 62 — Lifetap (reprint)
const LIFETAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFETAP,
    "74e7775b-b03b-4fc0-bcd9-3681cce5e70c",
    "Anson Maddocks",
);

// LEB 63 — Lord of Atlantis (reprint)
const LORD_OF_ATLANTIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_ATLANTIS,
    "27d7ac1f-2243-4c70-95a4-2b7343c8d92d",
    "Melissa A. Benson",
);

// LEB 64 — Magical Hack (reprint)
const MAGICAL_HACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAGICAL_HACK,
    "0aa81390-4e0b-484b-a5be-a9449cd41860",
    "Julie Baroh",
);

// LEB 65 — Mahamoti Djinn (reprint)
const MAHAMOTI_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MAHAMOTI_DJINN,
    "083f76c8-3e6d-4de5-b408-2f2394faed5c",
    "Dan Frazier",
);

// LEB 66 — Mana Short (reprint)
const MANA_SHORT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_SHORT,
    "4da4f9a8-024b-4707-b300-ccb11bd87cea",
    "Dameon Willich",
);

// LEB 67 — Merfolk of the Pearl Trident (reprint)
const MERFOLK_OF_THE_PEARL_TRIDENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MERFOLK_OF_THE_PEARL_TRIDENT,
    "cca142de-906d-4143-8f77-4acea1f1e6b1",
    "Jeff A. Menges",
);

// LEB 68 — Phantasmal Forces (reprint)
const PHANTASMAL_FORCES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_FORCES,
    "b0c6d792-0abb-474e-8c05-c4e843242ef0",
    "Mark Poole",
);

// LEB 69 — Phantasmal Terrain (reprint)
const PHANTASMAL_TERRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTASMAL_TERRAIN,
    "9c29369c-d909-45a7-be70-3181ddac9728",
    "Dameon Willich",
);

// LEB 70 — Phantom Monster (reprint)
const PHANTOM_MONSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PHANTOM_MONSTER,
    "b0782e90-383b-4aed-8fa0-99c8cf8b2cec",
    "Jesper Myrfors",
);

// LEB 71 — Pirate Ship (reprint)
const PIRATE_SHIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PIRATE_SHIP,
    "925ce0a7-ae09-4220-9e67-314dbc231c94",
    "Tom Wänerstrand",
);

// LEB 72 — Power Leak (reprint)
const POWER_LEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_LEAK,
    "86fdfb7b-1bcf-485a-be70-0130fc1fceef",
    "Drew Tucker",
);

// LEB 73 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "954b04e3-861a-45c9-8897-9cb4a99f04c3",
    "Richard Thomas",
);

// LEB 74 — Prodigal Sorcerer (reprint)
const PRODIGAL_SORCERER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PRODIGAL_SORCERER,
    "c420abf2-05ec-4623-8a6c-353736a4edeb",
    "Douglas Shuler",
);

// LEB 75 — Psionic Blast (reprint)
const PSIONIC_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PSIONIC_BLAST,
    "73b6b789-00c5-4d72-8fb3-6808bfbf0144",
    "Douglas Shuler",
);

// LEB 76 — Psychic Venom (reprint)
const PSYCHIC_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PSYCHIC_VENOM,
    "e5c8a81f-bf05-4504-ac87-4fd4b41e88c1",
    "Brian Snõddy",
);

// LEB 77 — Sea Serpent (reprint)
const SEA_SERPENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SEA_SERPENT,
    "11b21f91-51fd-407d-bab2-63c11f23b680",
    "Jeff A. Menges",
);

// LEB 78 — Siren's Call (reprint)
const SIREN_S_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIREN_S_CALL,
    "00ce03f3-ddc0-4cf3-8f07-551c960e8639",
    "Anson Maddocks",
);

// LEB 79 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "fb4da609-6c08-4a18-b7d9-fb2f9b11bab2",
    "Mark Poole",
);

// LEB 80 — Spell Blast (reprint)
const SPELL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SPELL_BLAST,
    "3f599b73-1d55-4acc-8931-f5ab39d1d4e9",
    "Brian Snõddy",
);

// LEB 81 — Stasis (reprint)
const STASIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STASIS,
    "73c76f5d-d866-4eb7-b2d2-fc6ecf982f8e",
    "Fay Jones",
);

// LEB 82 — Steal Artifact (reprint)
const STEAL_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STEAL_ARTIFACT,
    "92c14d4d-abaa-411a-aaa1-0b79fccee8c1",
    "Amy Weber",
);

// LEB 83 — Thoughtlace (reprint)
const THOUGHTLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THOUGHTLACE,
    "fc2b2b9e-5abf-4c41-a85c-ef95e6ab84d6",
    "Mark Poole",
);

// LEB 84 — Time Walk (reprint)
const TIME_WALK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_WALK,
    "54992fda-45a9-4ed1-b380-34d167feec90",
    "Amy Weber",
);

// LEB 85 — Timetwister (reprint)
const TIMETWISTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIMETWISTER,
    "09f1958a-50cc-43cc-80e1-988800e44ca8",
    "Mark Tedin",
);

// LEB 86 — Twiddle (reprint)
const TWIDDLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWIDDLE,
    "34bd24da-f156-494e-86cb-80707863e40b",
    "Rob Alexander",
);

// LEB 87 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "686843c8-8c8a-4af6-bca8-e7f7583cc886",
    "Douglas Shuler",
);

// LEB 88 — Vesuvan Doppelganger (reprint)
const VESUVAN_DOPPELGANGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VESUVAN_DOPPELGANGER,
    "d18e952b-ab4d-4f90-bf5e-4db490e4e203",
    "Quinton Hoover",
);

// LEB 89 — Volcanic Eruption (reprint)
const VOLCANIC_ERUPTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VOLCANIC_ERUPTION,
    "ca669988-e009-4b3e-af20-ee5885554d34",
    "Douglas Shuler",
);

// LEB 90 — Wall of Air (reprint)
const WALL_OF_AIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_AIR,
    "71904b59-55dd-4074-9d50-c5bb0fb7266f",
    "Richard Thomas",
);

// LEB 91 — Wall of Water (reprint)
const WALL_OF_WATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WATER,
    "34887689-0adb-4ead-87a5-1d8fd77b6278",
    "Richard Thomas",
);

// LEB 92 — Water Elemental (reprint)
const WATER_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WATER_ELEMENTAL,
    "66f729e2-565b-4cdb-8b6f-0a14babe5680",
    "Jeff A. Menges",
);

// LEB 93 — Animate Dead (reprint)
const ANIMATE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ANIMATE_DEAD,
    "20d5059a-60a4-4135-863f-85a48bff8731",
    "Anson Maddocks",
);

// LEB 94 — Bad Moon (reprint)
const BAD_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BAD_MOON,
    "bf812f48-633c-46ab-b0c3-4819ab1b4e49",
    "Jesper Myrfors",
);

// LEB 95 — Black Knight (reprint)
const BLACK_KNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_KNIGHT,
    "1eced352-d49c-4e91-a368-52904d77a69d",
    "Jeff A. Menges",
);

// LEB 96 — Bog Wraith (reprint)
const BOG_WRAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BOG_WRAITH,
    "da26289f-e0e6-4aae-8782-ebdbabf39819",
    "Jeff A. Menges",
);

// LEB 97 — Contract from Below (reprint)
const CONTRACT_FROM_BELOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONTRACT_FROM_BELOW,
    "62f96e43-aebd-4de2-969a-37cd1d62f127",
    "Douglas Shuler",
);

// LEB 98 — Cursed Land (reprint)
const CURSED_LAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CURSED_LAND,
    "1eea8122-00c2-4d00-b87b-12eea86b16ba",
    "Jesper Myrfors",
);

// LEB 99 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DARK_RITUAL,
    "0690f724-eb95-416b-b064-f1239e2a30e8",
    "Sandra Everingham",
);

// LEB 100 — Darkpact (reprint)
const DARKPACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARKPACT,
    "09b12bcb-a935-48be-a5e8-abbb890e91ca",
    "Quinton Hoover",
);

// LEB 101 — Deathgrip (reprint)
const DEATHGRIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHGRIP,
    "c942a9af-e449-4f10-916c-6eb9e944de6a",
    "Anson Maddocks",
);

// LEB 102 — Deathlace (reprint)
const DEATHLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATHLACE,
    "e16fc59a-17da-462a-86ea-31f8a9ac18a1",
    "Sandra Everingham",
);

// LEB 103 — Demonic Attorney (reprint)
const DEMONIC_ATTORNEY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_ATTORNEY,
    "60f37eac-e8fa-48d3-b936-74461ea1853c",
    "Daniel Gelon",
);

// LEB 104 — Demonic Hordes (reprint)
const DEMONIC_HORDES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEMONIC_HORDES,
    "dc20c19b-7216-4f23-a3bb-70d4dcd3865e",
    "Jesper Myrfors",
);

// LEB 105 — Demonic Tutor (reprint)
const DEMONIC_TUTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DEMONIC_TUTOR,
    "a5e571ef-1645-4584-ab53-e7ea5d443dea",
    "Douglas Shuler",
);

// LEB 106 — Drain Life (reprint)
const DRAIN_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAIN_LIFE,
    "9fbc6761-c4fc-4b4c-afb5-94ad4d21bc05",
    "Douglas Shuler",
);

// LEB 107 — Drudge Skeletons (reprint)
const DRUDGE_SKELETONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DRUDGE_SKELETONS,
    "b1f3a1b9-d192-49d9-87bb-ca50e99edbd1",
    "Sandra Everingham",
);

// LEB 108 — Evil Presence (reprint)
const EVIL_PRESENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EVIL_PRESENCE,
    "9e995f4b-efd3-4ac7-8fec-adb913294815",
    "Sandra Everingham",
);

// LEB 109 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "67830531-970a-4339-8673-40954376455d",
    "Mark Poole",
);

// LEB 110 — Frozen Shade (reprint)
const FROZEN_SHADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FROZEN_SHADE,
    "89b6a352-40f5-4d7c-b2b6-2617539a1c1c",
    "Douglas Shuler",
);

// LEB 111 — Gloom (reprint)
const GLOOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GLOOM,
    "640770d9-c0f8-40fd-9467-ebc099a27a4b",
    "Dan Frazier",
);

// LEB 112 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "f6018459-d09b-489a-81be-933fd7d854c1",
    "Mark Poole",
);

// LEB 113 — Hypnotic Specter (reprint)
const HYPNOTIC_SPECTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::HYPNOTIC_SPECTER,
    "edcc56a0-1dc0-4261-8f9c-5a88ce83f9e9",
    "Douglas Shuler",
);

// LEB 114 — Lich (reprint)
const LICH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LICH,
    "e5a9c089-0aad-4c14-9bfc-c0b39c976777",
    "Daniel Gelon",
);

// LEB 115 — Lord of the Pit (reprint)
const LORD_OF_THE_PIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LORD_OF_THE_PIT,
    "24626988-81df-44c9-9a8e-ecb9f82c383b",
    "Mark Tedin",
);

// LEB 116 — Mind Twist (reprint)
const MIND_TWIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MIND_TWIST,
    "0cb6cbbe-c3e9-4d14-a6b8-fb74e6a02b33",
    "Julie Baroh",
);

// LEB 117 — Nether Shadow (reprint)
const NETHER_SHADOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETHER_SHADOW,
    "38396ae3-a48f-44c7-96bf-ea41b5aaeebc",
    "Christopher Rush",
);

// LEB 118 — Nettling Imp (reprint)
const NETTLING_IMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NETTLING_IMP,
    "576220c3-1e6b-43f3-a47e-5e8246ee7d46",
    "Quinton Hoover",
);

// LEB 119 — Nightmare (reprint)
const NIGHTMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NIGHTMARE,
    "fc78dced-27d2-441a-b63b-32356bc33747",
    "Melissa A. Benson",
);

// LEB 120 — Paralyze (reprint)
const PARALYZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PARALYZE,
    "106d8401-f0e2-461e-b8ea-16d475db98da",
    "Anson Maddocks",
);

// LEB 121 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "1313b7e6-4acb-435a-bde5-1def5e5350ac",
    "Jesper Myrfors",
);

// LEB 122 — Plague Rats (reprint)
const PLAGUE_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAGUE_RATS,
    "995b58e6-5c69-4fdf-9c41-61cef7a610c4",
    "Anson Maddocks",
);

// LEB 123 — Raise Dead (reprint)
const RAISE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAISE_DEAD,
    "0066c7a6-7775-43ba-81cd-35fbc5621bc3",
    "Jeff A. Menges",
);

// LEB 124 — Royal Assassin (reprint)
const ROYAL_ASSASSIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROYAL_ASSASSIN,
    "b6e33c5e-6d99-4e7e-b611-4b271a47b4d2",
    "Tom Wänerstrand",
);

// LEB 125 — Sacrifice (reprint)
const SACRIFICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SACRIFICE,
    "8abe7d62-6a99-4d1f-9b81-cff0485997a8",
    "Dan Frazier",
);

// LEB 126 — Scathe Zombies (reprint)
const SCATHE_ZOMBIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCATHE_ZOMBIES,
    "a30abb09-2f80-46cf-a839-b4dac5c23dce",
    "Jesper Myrfors",
);

// LEB 127 — Scavenging Ghoul (reprint)
const SCAVENGING_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SCAVENGING_GHOUL,
    "e2bfa6bb-cf7b-4a79-83f5-178a633c499e",
    "Jeff A. Menges",
);

// LEB 128 — Sengir Vampire (reprint)
const SENGIR_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SENGIR_VAMPIRE,
    "5fbd5fbb-f689-4ff0-8f23-17e4cb0925a2",
    "Anson Maddocks",
);

// LEB 129 — Simulacrum (reprint)
const SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SIMULACRUM,
    "5bcda143-55f8-4d02-918f-975d9090d03f",
    "Mark Poole",
);

// LEB 130 — Sinkhole (reprint)
const SINKHOLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SINKHOLE,
    "52ea4387-f23c-430c-99d6-0248a4ab1713",
    "Sandra Everingham",
);

// LEB 131 — Terror (reprint)
const TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TERROR,
    "58d8598b-35e5-414f-aee0-52137236f642",
    "Ron Spencer",
);

// LEB 132 — Unholy Strength (reprint)
const UNHOLY_STRENGTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNHOLY_STRENGTH,
    "1c1c781d-1f27-40e3-9d79-0ebb6677e835",
    "Douglas Shuler",
);

// LEB 133 — Wall of Bone (reprint)
const WALL_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BONE,
    "7930666c-12ac-420b-8ced-0e924925b075",
    "Anson Maddocks",
);

// LEB 134 — Warp Artifact (reprint)
const WARP_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WARP_ARTIFACT,
    "4a289787-2d30-4e0b-ac97-3767818d0387",
    "Amy Weber",
);

// LEB 135 — Weakness (reprint)
const WEAKNESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEAKNESS,
    "16137fa6-1b5c-49e7-ad79-dda4b7019a59",
    "Douglas Shuler",
);

// LEB 136 — Will-o'-the-Wisp (reprint)
const WILL_O_THE_WISP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILL_O_THE_WISP,
    "4b60630c-f97c-43be-8410-53a68613b735",
    "Jesper Myrfors",
);

// LEB 137 — Word of Command (reprint)
const WORD_OF_COMMAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WORD_OF_COMMAND,
    "7d37b529-8a41-4177-abef-614f363e69d1",
    "Jesper Myrfors",
);

// LEB 138 — Zombie Master (reprint)
const ZOMBIE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ZOMBIE_MASTER,
    "a1bfda92-b932-46d8-b549-e2bc2b584a17",
    "Jeff A. Menges",
);

// LEB 139 — Burrowing (reprint)
const BURROWING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BURROWING,
    "8795bab7-ced2-4a1d-8c57-636bc4c0a977",
    "Mark Poole",
);

// LEB 140 — Chaoslace (reprint)
const CHAOSLACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CHAOSLACE,
    "d980e9c0-db88-41f9-8dbf-89f0e1ac6c20",
    "Dameon Willich",
);

// LEB 141 — Disintegrate (reprint)
const DISINTEGRATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISINTEGRATE,
    "cfb3a6b9-a119-49c0-9baf-b552fdd00b28",
    "Anson Maddocks",
);

// LEB 142 — Dragon Whelp (reprint)
const DRAGON_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::DRAGON_WHELP,
    "2e009adf-aded-4d64-ba3e-ddc3448c967a",
    "Amy Weber",
);

// LEB 143 — Dwarven Demolition Team (reprint)
const DWARVEN_DEMOLITION_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_DEMOLITION_TEAM,
    "e552dfb6-b8a5-419d-b098-5aedc0500684",
    "Kev Brockschmidt",
);

// LEB 144 — Dwarven Warriors (reprint)
const DWARVEN_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DWARVEN_WARRIORS,
    "c0de88cf-b9e5-4611-a16f-2787d8d9d269",
    "Douglas Shuler",
);

// LEB 145 — Earth Elemental (reprint)
const EARTH_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTH_ELEMENTAL,
    "c427e8cc-d908-4b88-931d-a540fc8bfe74",
    "Dan Frazier",
);

// LEB 146 — Earthbind (reprint)
const EARTHBIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::EARTHBIND,
    "e5955a9d-8a0e-4e57-9433-ed3392b2f308",
    "Quinton Hoover",
);

// LEB 147 — Earthquake (reprint)
const EARTHQUAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::EARTHQUAKE,
    "86435875-ac92-4348-b41e-19570cf62a1c",
    "Dan Frazier",
);

// LEB 148 — False Orders (reprint)
const FALSE_ORDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FALSE_ORDERS,
    "e4ebc485-f1b7-436d-8c90-9acf2f7d92e5",
    "Anson Maddocks",
);

// LEB 149 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "376cb9e5-89fb-4091-8a20-140bb6de0ef6",
    "Melissa A. Benson",
);

// LEB 150 — Fireball (reprint)
const FIREBALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FIREBALL,
    "a285ab2e-836e-45b0-894e-574f733cf3db",
    "Mark Tedin",
);

// LEB 151 — Firebreathing (reprint)
const FIREBREATHING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIREBREATHING,
    "235e4321-0216-4d6a-a57b-72ebff427b09",
    "Dan Frazier",
);

// LEB 152 — Flashfires (reprint)
const FLASHFIRES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FLASHFIRES,
    "5a2a91b9-c45f-4e3d-b3c4-944493bdd86a",
    "Dameon Willich",
);

// LEB 153 — Fork (reprint)
const FORK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FORK,
    "8144418b-e3e5-459f-8db2-f2e348fba4da",
    "Amy Weber",
);

// LEB 154 — Goblin Balloon Brigade (reprint)
const GOBLIN_BALLOON_BRIGADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_BALLOON_BRIGADE,
    "3fdb52dd-4fc5-4594-b53b-ea169325be0b",
    "Andi Rusu",
);

// LEB 155 — Goblin King (reprint)
const GOBLIN_KING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GOBLIN_KING,
    "65705a8d-6bb1-4289-b8b0-8546ccc478dc",
    "Jesper Myrfors",
);

// LEB 156 — Granite Gargoyle (reprint)
const GRANITE_GARGOYLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GRANITE_GARGOYLE,
    "affb57f4-273a-425c-a1b3-d0a5407f43d5",
    "Christopher Rush",
);

// LEB 157 — Gray Ogre (reprint)
const GRAY_OGRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRAY_OGRE,
    "41023495-d3cb-4cb0-b95c-f717480a76a5",
    "Dan Frazier",
);

// LEB 158 — Hill Giant (reprint)
const HILL_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HILL_GIANT,
    "4905e98f-0c5a-4ec7-b85b-dc2c3549d5d0",
    "Dan Frazier",
);

// LEB 159 — Hurloon Minotaur (reprint)
const HURLOON_MINOTAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURLOON_MINOTAUR,
    "8ef29573-99a1-42fc-8941-2466cda2465f",
    "Anson Maddocks",
);

// LEB 160 — Ironclaw Orcs (reprint)
const IRONCLAW_ORCS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRONCLAW_ORCS,
    "a7be8a25-a744-426e-8e66-7fdff2789af4",
    "Anson Maddocks",
);

// LEB 161 — Keldon Warlord (reprint)
const KELDON_WARLORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KELDON_WARLORD,
    "b07deb9b-5b88-4658-8ae8-041568992019",
    "Kev Brockschmidt",
);

// LEB 162 — Lightning Bolt (reprint)
const LIGHTNING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LIGHTNING_BOLT,
    "b5d3dcab-2260-479d-9ef6-dfb92d4f6061",
    "Christopher Rush",
);

// LEB 163 — Mana Flare (reprint)
const MANA_FLARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANA_FLARE,
    "b44d3087-ced3-40e8-a63b-1733b7e7f34c",
    "Christopher Rush",
);

// LEB 164 — Manabarbs (reprint)
const MANABARBS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MANABARBS,
    "7c01cae0-4d61-4bf7-a145-82d9bb11d816",
    "Christopher Rush",
);

// LEB 165 — Mons's Goblin Raiders (reprint)
const MONSS_GOBLIN_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MONSS_GOBLIN_RAIDERS,
    "2fbf039d-0ab9-4c42-a0a3-cbfa3ea1dd6e",
    "Jeff A. Menges",
);

// LEB 166 — Orcish Artillery (reprint)
const ORCISH_ARTILLERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ARTILLERY,
    "4d2354ee-2ce0-4adb-b48c-0e30b952e545",
    "Anson Maddocks",
);

// LEB 167 — Orcish Oriflamme (reprint)
const ORCISH_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ORCISH_ORIFLAMME,
    "f2752cf2-9a48-49a8-98ff-2e32a9121d78",
    "Dan Frazier",
);

// LEB 168 — Power Surge (reprint)
const POWER_SURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SURGE,
    "f52eb10a-a9eb-44b7-95ae-12fb551c8fa5",
    "Douglas Shuler",
);

// LEB 169 — Raging River (reprint)
const RAGING_RIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::RAGING_RIVER,
    "c14746bb-aa00-4be2-9740-d87f976296d2",
    "Sandra Everingham",
);

// LEB 170 — Red Elemental Blast (reprint)
const RED_ELEMENTAL_BLAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::RED_ELEMENTAL_BLAST,
    "4fafd3f9-f7de-4d6e-8824-6b60866fc50f",
    "Richard Thomas",
);

// LEB 171 — Roc of Kher Ridges (reprint)
const ROC_OF_KHER_RIDGES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROC_OF_KHER_RIDGES,
    "f1b9e3ae-c7e9-455f-abfe-220262719beb",
    "Andi Rusu",
);

// LEB 172 — Rock Hydra (reprint)
const ROCK_HYDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROCK_HYDRA,
    "c17a982d-466d-4fec-b85a-a44161e5dad5",
    "Jeff A. Menges",
);

// LEB 173 — Sedge Troll (reprint)
const SEDGE_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SEDGE_TROLL,
    "02ec317b-52a6-4490-80e5-a56826b06771",
    "Dan Frazier",
);

// LEB 174 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SHATTER,
    "76ddf3f4-1305-4599-bf4c-f9e148bdda4d",
    "Amy Weber",
);

// LEB 175 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "5e64822a-6817-4e1e-8155-3e95f8e3763f",
    "Melissa A. Benson",
);

// LEB 176 — Smoke (reprint)
const SMOKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SMOKE,
    "7eb0cb82-d930-43c3-a6d6-f947018d45d6",
    "Jesper Myrfors",
);

// LEB 177 — Stone Giant (reprint)
const STONE_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_GIANT,
    "a2b5f545-a87d-4292-880f-5cd2f6755748",
    "Dameon Willich",
);

// LEB 178 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::STONE_RAIN,
    "901831ad-1840-4287-b6a0-bea310598dc2",
    "Daniel Gelon",
);

// LEB 179 — Tunnel (reprint)
const TUNNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TUNNEL,
    "cc738025-a771-4186-b08c-7b37c0e9713b",
    "Dan Frazier",
);

// LEB 180 — Two-Headed Giant of Foriys (reprint)
const TWO_HEADED_GIANT_OF_FORIYS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TWO_HEADED_GIANT_OF_FORIYS,
    "30fcbb16-f8e7-4f6e-a806-541ef54aa025",
    "Anson Maddocks",
);

// LEB 181 — Uthden Troll (reprint)
const UTHDEN_TROLL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UTHDEN_TROLL,
    "91f46e9a-6075-4fa5-8f60-f81e2024b13d",
    "Douglas Shuler",
);

// LEB 182 — Wall of Fire (reprint)
const WALL_OF_FIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_FIRE,
    "88baaea5-69ec-4756-86c2-9c9d73ca8ef1",
    "Richard Thomas",
);

// LEB 183 — Wall of Stone (reprint)
const WALL_OF_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_STONE,
    "329ba196-a107-41ac-b02a-5f8b10ecd130",
    "Dan Frazier",
);

// LEB 184 — Wheel of Fortune (reprint)
const WHEEL_OF_FORTUNE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WHEEL_OF_FORTUNE,
    "9052369f-840f-438e-b86d-e2f8d6339585",
    "Daniel Gelon",
);

// LEB 185 — Aspect of Wolf (reprint)
const ASPECT_OF_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ASPECT_OF_WOLF,
    "36f7dc8e-e02a-4ceb-8767-2875f86e6811",
    "Jeff A. Menges",
);

// LEB 186 — Berserk (reprint)
const BERSERK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BERSERK,
    "88d6f431-a7ea-4508-a52c-86d33e12e4e4",
    "Dan Frazier",
);

// LEB 187 — Birds of Paradise (reprint)
const BIRDS_OF_PARADISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BIRDS_OF_PARADISE,
    "852d7a68-8682-4073-a44b-f10f5613879c",
    "Mark Poole",
);

// LEB 188 — Camouflage (reprint)
const CAMOUFLAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CAMOUFLAGE,
    "2f55ff95-32a3-43ba-82e5-a5a3bc2cc9e5",
    "Jesper Myrfors",
);

// LEB 189 — Channel (reprint)
const CHANNEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHANNEL,
    "6fa6468a-335a-467d-aef6-e537af9d5c1c",
    "Richard Thomas",
);

// LEB 190 — Cockatrice (reprint)
const COCKATRICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COCKATRICE,
    "fc71dd0f-dffe-4671-b9e3-ddec70626688",
    "Dan Frazier",
);

// LEB 191 — Craw Wurm (reprint)
const CRAW_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRAW_WURM,
    "17d5c1c7-a882-479a-9077-0784e83b462d",
    "Daniel Gelon",
);

// LEB 192 — Elvish Archers (reprint)
const ELVISH_ARCHERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ELVISH_ARCHERS,
    "c3240d5e-b3d4-4368-b09b-c309bc935152",
    "Anson Maddocks",
);

// LEB 193 — Fastbond (reprint)
const FASTBOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FASTBOND,
    "f48ed192-c1a1-437a-80dd-647a616b46e3",
    "Mark Poole",
);

// LEB 194 — Fog (reprint)
const FOG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOG,
    "f4e9597a-4489-47e9-8b15-888acb402ddd",
    "Jesper Myrfors",
);

// LEB 195 — Force of Nature (reprint)
const FORCE_OF_NATURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCE_OF_NATURE,
    "c25a61b3-c828-491c-868d-e4eff770c1bb",
    "Douglas Shuler",
);

// LEB 196 — Fungusaur (reprint)
const FUNGUSAUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FUNGUSAUR,
    "75a58f0b-c772-4254-8686-182d26889f9c",
    "Daniel Gelon",
);

// LEB 197 — Gaea's Liege (reprint)
const GAEA_S_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAEA_S_LIEGE,
    "554362d7-97b3-4a55-9292-15e90435088d",
    "Dameon Willich",
);

// LEB 198 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GIANT_GROWTH,
    "755a45bd-8fe6-4e4d-8065-024a2836751b",
    "Sandra Everingham",
);

// LEB 199 — Giant Spider (reprint)
const GIANT_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_SPIDER,
    "52ea35ce-8aa1-4818-8ad5-7e462452f10e",
    "Sandra Everingham",
);

// LEB 200 — Grizzly Bears (reprint)
const GRIZZLY_BEARS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GRIZZLY_BEARS,
    "e7aa2b93-0a84-4318-bf2d-58164f0a846f",
    "Jeff A. Menges",
);

// LEB 201 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "b3939f72-1ec6-4b2c-b37e-b1ebb024bb8f",
    "Dameon Willich",
);

// LEB 202 — Ice Storm (reprint)
const ICE_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ICE_STORM,
    "7c439c5a-b4a5-411b-9e68-fb8438ccdfb0",
    "Dan Frazier",
);

// LEB 203 — Instill Energy (reprint)
const INSTILL_ENERGY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::INSTILL_ENERGY,
    "58334cf9-5186-4fba-963c-fffb21f2b8de",
    "Dameon Willich",
);

// LEB 204 — Ironroot Treefolk (reprint)
const IRONROOT_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IRONROOT_TREEFOLK,
    "1d9479ae-2b42-4137-9e62-ef4d7fd17d0c",
    "Jesper Myrfors",
);

// LEB 205 — Kudzu (reprint)
const KUDZU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KUDZU,
    "ced83afa-9718-4b8a-961b-394f8595c480",
    "Mark Poole",
);

// LEB 206 — Ley Druid (reprint)
const LEY_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LEY_DRUID,
    "b58867ec-0b1a-4804-bc2e-1c88d338c29e",
    "Sandra Everingham",
);

// LEB 207 — Lifeforce (reprint)
const LIFEFORCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFEFORCE,
    "3715abe2-5a8e-4bf4-ac02-6c755d86bb4c",
    "Dameon Willich",
);

// LEB 208 — Lifelace (reprint)
const LIFELACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIFELACE,
    "9379e159-43ac-4bd2-8b33-f3de8e20cfe0",
    "Amy Weber",
);

// LEB 209 — Living Artifact (reprint)
const LIVING_ARTIFACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_ARTIFACT,
    "8bbf6678-f597-407d-9a95-02bbe6c4bcf3",
    "Anson Maddocks",
);

// LEB 210 — Living Lands (reprint)
const LIVING_LANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_LANDS,
    "f132acbd-53e5-430a-8f93-8b7469633c0e",
    "Jesper Myrfors",
);

// LEB 211 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::LLANOWAR_ELVES,
    "abd80204-e9ba-483f-9b75-a69712545ba9",
    "Anson Maddocks",
);

// LEB 212 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "e31495ab-e6ed-40a6-b82d-aa6092b049e2",
    "Anson Maddocks",
);

// LEB 213 — Natural Selection (reprint)
const NATURAL_SELECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::NATURAL_SELECTION,
    "a594299e-fc3a-4d46-bd58-1a9cf7ddbdd7",
    "Mark Poole",
);

// LEB 214 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "42ad2d7f-34a5-4b17-ae11-16b322601d73",
    "Quinton Hoover",
);

// LEB 215 — Regrowth (reprint)
const REGROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::REGROWTH,
    "898cd314-9060-4f1c-a821-1d61a292a12b",
    "Dameon Willich",
);

// LEB 216 — Scryb Sprites (reprint)
const SCRYB_SPRITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRYB_SPRITES,
    "fafe9639-e9d0-4aa2-8a16-f4ec24c140c0",
    "Amy Weber",
);

// LEB 217 — Shanodin Dryads (reprint)
const SHANODIN_DRYADS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHANODIN_DRYADS,
    "1ac8bdb0-2dfd-4531-a4d9-420f2f2a90be",
    "Anson Maddocks",
);

// LEB 218 — Stream of Life (reprint)
const STREAM_OF_LIFE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STREAM_OF_LIFE,
    "da18a2c9-850e-400d-b0b3-edd8a946e380",
    "Mark Poole",
);

// LEB 219 — Thicket Basilisk (reprint)
const THICKET_BASILISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THICKET_BASILISK,
    "6321e16b-0b4b-4d36-ab94-97bf5816acf4",
    "Dan Frazier",
);

// LEB 220 — Timber Wolves (reprint)
const TIMBER_WOLVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TIMBER_WOLVES,
    "aa598db8-c0c7-4a9a-bd89-6d3da0d3dfba",
    "Melissa A. Benson",
);

// LEB 221 — Tranquility (reprint)
const TRANQUILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TRANQUILITY,
    "ee21b620-4dfa-4e06-872e-8d8ffce12f76",
    "Douglas Shuler",
);

// LEB 222 — Tsunami (reprint)
const TSUNAMI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::TSUNAMI,
    "1f4b6f5a-1ba2-409d-9b9b-91e2c1470f62",
    "Richard Thomas",
);

// LEB 223 — Verduran Enchantress (reprint)
const VERDURAN_ENCHANTRESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::VERDURAN_ENCHANTRESS,
    "da3f051c-6be3-4f92-8f66-9f72d75dbcf5",
    "Kev Brockschmidt",
);

// LEB 224 — Wall of Brambles (reprint)
const WALL_OF_BRAMBLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_BRAMBLES,
    "c2fca52b-80b3-4b6b-9a49-110c66557894",
    "Anson Maddocks",
);

// LEB 225 — Wall of Ice (reprint)
const WALL_OF_ICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_ICE,
    "cc05a648-7719-4ed3-aa3b-648463ee2869",
    "Richard Thomas",
);

// LEB 226 — Wall of Wood (reprint)
const WALL_OF_WOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WALL_OF_WOOD,
    "1a5054a4-599d-49df-9a80-77eeed47891f",
    "Mark Tedin",
);

// LEB 227 — Wanderlust (reprint)
const WANDERLUST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WANDERLUST,
    "393f08a2-7aa8-443f-aab5-4287240e9167",
    "Cornelius Brudi",
);

// LEB 228 — War Mammoth (reprint)
const WAR_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WAR_MAMMOTH,
    "9f67175d-ac5c-4947-b243-d5206b552bdc",
    "Jeff A. Menges",
);

// LEB 229 — Web (reprint)
const WEB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WEB,
    "f7f84dc2-5a29-447d-97ab-a10afd9ee538",
    "Rob Alexander",
);

// LEB 230 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "64f299eb-9cd6-40bc-ad44-22e3aeb5c930",
    "Mark Poole",
);

// LEB 231 — Ankh of Mishra (reprint)
const ANKH_OF_MISHRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ANKH_OF_MISHRA,
    "a0367e54-eb07-475a-b06b-f869a046a86c",
    "Amy Weber",
);

// LEB 232 — Basalt Monolith (reprint)
const BASALT_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::BASALT_MONOLITH,
    "81d73362-43c1-4dd0-87dd-9aa7ae13ff2f",
    "Jesper Myrfors",
);

// LEB 233 — Black Lotus (reprint)
const BLACK_LOTUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_LOTUS,
    "b3a69a1c-c80f-4413-a6fd-ae54cabbce28",
    "Christopher Rush",
);

// LEB 234 — Black Vise (reprint)
const BLACK_VISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BLACK_VISE,
    "d234f3d7-2f15-4fbf-92db-16c3433d644b",
    "Richard Thomas",
);

// LEB 235 — Celestial Prism (reprint)
const CELESTIAL_PRISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CELESTIAL_PRISM,
    "243c5460-8d4c-47a7-8a9c-ab626daa520a",
    "Amy Weber",
);

// LEB 236 — Chaos Orb (reprint)
const CHAOS_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::CHAOS_ORB,
    "6bec436c-2869-432a-b3cf-633a58af6d4c",
    "Mark Tedin",
);

// LEB 237 — Clockwork Beast (reprint)
const CLOCKWORK_BEAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CLOCKWORK_BEAST,
    "6c6efe95-ae57-4ff1-8f8a-0d6f3bd36d9c",
    "Drew Tucker",
);

// LEB 238 — Conservator (reprint)
const CONSERVATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CONSERVATOR,
    "d4f54af3-7c85-43da-b0ce-df4a44af4736",
    "Amy Weber",
);

// LEB 239 — Copper Tablet (reprint)
const COPPER_TABLET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::COPPER_TABLET,
    "93842064-a0a8-4e4d-9c8a-e8a86448d225",
    "Amy Weber",
);

// LEB 240 — Crystal Rod (reprint)
const CRYSTAL_ROD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CRYSTAL_ROD,
    "e44d892f-a975-4062-8a54-5777d2600504",
    "Amy Weber",
);

// LEB 241 — Cyclopean Tomb (reprint)
const CYCLOPEAN_TOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CYCLOPEAN_TOMB,
    "00775f44-fbe6-41ee-9977-d13d1fb5b6fb",
    "Anson Maddocks",
);

// LEB 242 — Dingus Egg (reprint)
const DINGUS_EGG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DINGUS_EGG,
    "fe8ecaee-0de3-45ee-8428-09dc400d63d8",
    "Dan Frazier",
);

// LEB 243 — Disrupting Scepter (reprint)
const DISRUPTING_SCEPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISRUPTING_SCEPTER,
    "ae91e07c-ad6d-41d9-bd65-184f92761334",
    "Dan Frazier",
);

// LEB 244 — Forcefield (reprint)
const FORCEFIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FORCEFIELD,
    "34855fa8-959d-45a2-ad91-8b17019755be",
    "Dan Frazier",
);

// LEB 245 — Gauntlet of Might (reprint)
const GAUNTLET_OF_MIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GAUNTLET_OF_MIGHT,
    "63c0e240-07b0-45fb-90af-f4fce18c604e",
    "Christopher Rush",
);

// LEB 246 — Glasses of Urza (reprint)
const GLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::GLASSES_OF_URZA,
    "eb6953fd-ee48-49dc-9c9c-bfb9a9dc06d0",
    "Douglas Shuler",
);

// LEB 247 — Helm of Chatzuk (reprint)
const HELM_OF_CHATZUK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HELM_OF_CHATZUK,
    "559d3329-9053-4301-b867-1b49c248fe31",
    "Mark Tedin",
);

// LEB 248 — Howling Mine (reprint)
const HOWLING_MINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWLING_MINE,
    "37634ffe-788f-4262-88e8-5ab7c7ca74d6",
    "Mark Poole",
);

// LEB 249 — Icy Manipulator (reprint)
const ICY_MANIPULATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ICY_MANIPULATOR,
    "d27608e7-6539-4813-95b6-d8847cdc6a12",
    "Douglas Shuler",
);

// LEB 250 — Illusionary Mask (reprint)
const ILLUSIONARY_MASK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ILLUSIONARY_MASK,
    "61ea96b1-4428-4951-88d4-f79338955981",
    "Amy Weber",
);

// LEB 251 — Iron Star (reprint)
const IRON_STAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::IRON_STAR,
    "b08fff47-c3c8-40a9-b3d3-296954aa4ed4",
    "Dan Frazier",
);

// LEB 252 — Ivory Cup (reprint)
const IVORY_CUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::IVORY_CUP,
    "32516ab8-43be-4207-a7d5-4916933ce155",
    "Anson Maddocks",
);

// LEB 253 — Jade Monolith (reprint)
const JADE_MONOLITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_MONOLITH,
    "eeea32ba-dfe4-4a9b-b403-43c2abc80b78",
    "Anson Maddocks",
);

// LEB 254 — Jade Statue (reprint)
const JADE_STATUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JADE_STATUE,
    "985164ba-0c30-42b1-a8b6-3be19251359c",
    "Dan Frazier",
);

// LEB 255 — Jayemdae Tome (reprint)
const JAYEMDAE_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JAYEMDAE_TOME,
    "e48b1c51-c0fd-4c08-8631-80f507b04d28",
    "Mark Tedin",
);

// LEB 256 — Juggernaut (reprint)
const JUGGERNAUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::JUGGERNAUT,
    "870eb49c-f62d-4986-b492-601feb68a307",
    "Dan Frazier",
);

// LEB 257 — Kormus Bell (reprint)
const KORMUS_BELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::KORMUS_BELL,
    "0cd2a4f9-8f80-4ee3-8068-73e686d6eeb9",
    "Christopher Rush",
);

// LEB 258 — Library of Leng (reprint)
const LIBRARY_OF_LENG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIBRARY_OF_LENG,
    "0254bff2-a3a7-434e-980a-2d30355793fc",
    "Daniel Gelon",
);

// LEB 259 — Living Wall (reprint)
const LIVING_WALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LIVING_WALL,
    "0c2cd1c8-8734-4534-ae92-def4d94ef5bc",
    "Anson Maddocks",
);

// LEB 260 — Mana Vault (reprint)
const MANA_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MANA_VAULT,
    "a11f55e8-7f86-4ca9-b737-9a920d9cf282",
    "Mark Tedin",
);

// LEB 261 — Meekstone (reprint)
const MEEKSTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MEEKSTONE,
    "74b22007-9def-4c0f-921c-555483cc3deb",
    "Quinton Hoover",
);

// LEB 262 — Mox Emerald (reprint)
const MOX_EMERALD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_EMERALD,
    "ea5d9476-76be-48e7-b6a0-49ced25cb092",
    "Dan Frazier",
);

// LEB 263 — Mox Jet (reprint)
const MOX_JET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_JET,
    "133204e4-fef8-4851-aa50-c96ffa35b802",
    "Dan Frazier",
);

// LEB 264 — Mox Pearl (reprint)
const MOX_PEARL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_PEARL,
    "4da892c5-071f-416f-9e42-c4bff102eb88",
    "Dan Frazier",
);

// LEB 265 — Mox Ruby (reprint)
const MOX_RUBY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_RUBY,
    "fdac742b-16db-4e03-be8f-c600dbd522d5",
    "Dan Frazier",
);

// LEB 266 — Mox Sapphire (reprint)
const MOX_SAPPHIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOX_SAPPHIRE,
    "1eb3178b-dac5-4b34-9d3e-4f5a170d1c87",
    "Dan Frazier",
);

// LEB 267 — Nevinyrral's Disk (reprint)
const NEVINYRRALS_DISK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::NEVINYRRALS_DISK,
    "dbb21f21-668a-4d57-8d05-8db11fb82d99",
    "Mark Tedin",
);

// LEB 268 — Obsianus Golem (reprint)
const OBSIANUS_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::OBSIANUS_GOLEM,
    "e9ed6669-e340-46d5-906b-e24e76464e75",
    "Jesper Myrfors",
);

// LEB 269 — Rod of Ruin (reprint)
const ROD_OF_RUIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ROD_OF_RUIN,
    "45810c0a-0a35-4bd4-ba66-5a45f8973fa4",
    "Christopher Rush",
);

// LEB 270 — Sol Ring (reprint)
const SOL_RING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SOL_RING,
    "c0fb91ec-20a8-4c13-9469-18885b1ecca3",
    "Mark Tedin",
);

// LEB 271 — Soul Net (reprint)
const SOUL_NET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SOUL_NET,
    "08ba41ec-4fff-4192-80ff-2afcd706ea59",
    "Dameon Willich",
);

// LEB 272 — Sunglasses of Urza (reprint)
const SUNGLASSES_OF_URZA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SUNGLASSES_OF_URZA,
    "49fcf47d-0f1d-469e-a8c4-d5c97be7a1ef",
    "Dan Frazier",
);

// LEB 273 — The Hive (reprint)
const THE_HIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THE_HIVE,
    "84b83106-a10d-469a-99eb-56110ef34ba1",
    "Sandra Everingham",
);

// LEB 274 — Throne of Bone (reprint)
const THRONE_OF_BONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::THRONE_OF_BONE,
    "655b6265-3030-4c68-af5b-b9e636b1a778",
    "Anson Maddocks",
);

// LEB 275 — Time Vault (reprint)
const TIME_VAULT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TIME_VAULT,
    "1164f22f-2706-4f35-9f58-d0eb8c344396",
    "Mark Tedin",
);

// LEB 276 — Winter Orb (reprint)
const WINTER_ORB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::WINTER_ORB,
    "847de6a4-a268-492e-a4d2-5b12237bc130",
    "Mark Tedin",
);

// LEB 277 — Wooden Sphere (reprint)
const WOODEN_SPHERE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WOODEN_SPHERE,
    "02eee156-54bd-46fc-8804-a73aab87f0ba",
    "Mark Tedin",
);

// LEB 278 — Badlands (reprint)
const BADLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BADLANDS,
    "a3393436-3426-4903-8f41-7abcbf6c18c2",
    "Rob Alexander",
);

// LEB 279 — Bayou (reprint)
const BAYOU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::BAYOU,
    "17db2b6a-eaa8-4a08-9e86-370bbd058574",
    "Jesper Myrfors",
);

// LEB 280 — Plateau (reprint)
const PLATEAU_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLATEAU,
    "fad0bbc4-f760-47a2-aab6-0dbb66ee3a95",
    "Drew Tucker",
);

// LEB 281 — Savannah (reprint)
const SAVANNAH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SAVANNAH,
    "0e9aeaa8-9a75-4719-992f-cbb316f72175",
    "Rob Alexander",
);

// LEB 282 — Scrubland (reprint)
const SCRUBLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SCRUBLAND,
    "8cf99186-3167-4092-8efb-e7448609ceba",
    "Jesper Myrfors",
);

// LEB 283 — Taiga (reprint)
const TAIGA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TAIGA,
    "30ce1bf0-7561-418f-a217-3ce10f28be82",
    "Rob Alexander",
);

// LEB 284 — Tropical Island (reprint)
const TROPICAL_ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TROPICAL_ISLAND,
    "ac19c5a1-ca13-4443-920b-83b567167ed4",
    "Jesper Myrfors",
);

// LEB 285 — Tundra (reprint)
const TUNDRA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::TUNDRA,
    "1b93ce48-219c-49ea-9ad0-b7357bea4606",
    "Jesper Myrfors",
);

// LEB 286 — Underground Sea (reprint)
const UNDERGROUND_SEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::UNDERGROUND_SEA,
    "5e91ce41-053e-4203-8860-49cbf854cc18",
    "Rob Alexander",
);

// LEB 287 — Volcanic Island
pub(in crate::card::sets) static VOLCANIC_ISLAND: CardRecord = CardRecord::new(
    CardSet::Beta,
    "Volcanic Island",
    "0324641d-af55-4c53-b4dc-c8262e967da5",
    "Brian Snõddy",
    CardRules::new_land(&["Island", "Mountain"]),
);

// LEB 288 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::PLAINS,
    "b7331b03-be66-419c-94bc-ed494c042ea3",
    "Jesper Myrfors",
);

// LEB 289 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    1,
    "52ff493a-6336-416e-af5e-1eb6d10c080e",
    "Jesper Myrfors",
);

// LEB 290 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::PLAINS,
    2,
    "38e2b0ff-8fdf-4db0-85c0-c1010bacd36b",
    "Jesper Myrfors",
);

// LEB 291 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::ISLAND,
    "bff33e91-8e52-43f2-b8ae-603b456b08fc",
    "Mark Poole",
);

// LEB 292 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    1,
    "d0c5cf64-9844-4b5b-8e6b-b97c50cce053",
    "Mark Poole",
);

// LEB 293 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::ISLAND,
    2,
    "c0a612c4-b4ac-4dd2-a06e-92516599fafd",
    "Mark Poole",
);

// LEB 294 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::SWAMP,
    "d1309a80-a761-4b80-8cf1-1a8b83190511",
    "Dan Frazier",
);

// LEB 295 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    1,
    "25ad2444-9985-423c-ad36-387218866409",
    "Dan Frazier",
);

// LEB 296 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::SWAMP,
    2,
    "a3544148-49b2-4320-8e3a-5bab81e0f7fd",
    "Dan Frazier",
);

// LEB 297 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::MOUNTAIN,
    "7af9c715-8d72-4eae-b412-fc89138ff588",
    "Douglas Shuler",
);

// LEB 298 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    1,
    "7cb88a03-7092-4d31-a9f1-4f16e39bc537",
    "Douglas Shuler",
);

// LEB 299 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::MOUNTAIN,
    2,
    "af9ad645-e605-4048-bf4c-d636584f315b",
    "Douglas Shuler",
);

// LEB 300 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &alpha::FOREST,
    "b5a922eb-49c7-45f0-92bc-671d7a8758f4",
    "Christopher Rush",
);

// LEB 301 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    1,
    "89ad91fc-50c2-44e0-b88e-2c13610377f9",
    "Christopher Rush",
);

// LEB 302 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &alpha::FOREST,
    2,
    "b4075bbc-dbad-4a1e-a992-70aed713a459",
    "Christopher Rush",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&CIRCLE_OF_PROTECTION_BLACK, &VOLCANIC_ISLAND];

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
