//! Through the Omenpaths card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::sets::y2025::marvels_spider_man::MULTIVERSAL_PASSAGE;
use crate::card::sets::y2025::marvels_spider_man::OMINOUS_ASYLUM;
use crate::card::sets::y2025::marvels_spider_man::SAVAGE_MANSION;
use crate::card::sets::y2025::marvels_spider_man::SINISTER_HIDEOUT;
use crate::card::sets::y2025::marvels_spider_man::SUBURBAN_SANCTUARY;
use crate::card::sets::y2025::marvels_spider_man::UNIVERSITY_CAMPUS;

use crate::card::sets::y1998::stronghold as catalog_sth;
use crate::card::sets::y2025::marvels_spider_man as catalog_spm;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OM1",
    slug: "through-the-omenpaths",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// OM1 1 — Spectacular Spider-Man (reprint)
const SPECTACULAR_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPECTACULAR_SPIDER_MAN,
    "8ff57c8b-173e-4a72-a0a0-1ad2d5cc6e5e",
    "Filip Burburan",
);

// OM1 2 — Web-Shooters (reprint)
const WEB_SHOOTERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WEB_SHOOTERS,
    "7a7c3434-4c66-4ce8-8030-4ab5a0b4cc76",
    "Camille Alquier",
);

// OM1 3 — Flash Thompson, Spider-Fan (reprint)
const FLASH_THOMPSON_SPIDER_FAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::FLASH_THOMPSON_SPIDER_FAN,
    "167f2d2e-d5c3-4bf8-83aa-33998e45a69f",
    "Elizabeth Peiró",
);

// OM1 4 — With Great Power . . . (reprint)
const WITH_GREAT_POWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WITH_GREAT_POWER,
    "138dcee4-9163-4f78-ae87-87f9b7c1c6fe",
    "Nils Hamm",
);

// OM1 5 — City Pigeon (reprint)
const CITY_PIGEON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::CITY_PIGEON,
    "5303956a-87ce-4193-8bf5-5c58beab31fa",
    "Michele Giorgi",
);

// OM1 6 — Rent Is Due (reprint)
const RENT_IS_DUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::RENT_IS_DUE,
    "d70c0e56-4f97-486a-8345-0cc6c51e3e36",
    "Campbell White",
);

// OM1 7 — Costume Closet (reprint)
const COSTUME_CLOSET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::COSTUME_CLOSET,
    "47131bd7-822e-4eb2-b1e2-1a0e754818bf",
    "Ben Maier",
);

// OM1 8 — Selfless Police Captain (reprint)
const SELFLESS_POLICE_CAPTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SELFLESS_POLICE_CAPTAIN,
    "4ae38e57-014b-487a-bb03-d45f5fbb927e",
    "Loïc Canavaggia",
);

// OM1 9 — Spider-Man, Web-Slinger (reprint)
const SPIDER_MAN_WEB_SLINGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MAN_WEB_SLINGER,
    "19e761e8-7542-44e9-9c6f-65a806153501",
    "Xavier Ribeiro",
);

// OM1 10 — Starling, Aerial Ally (reprint)
const STARLING_AERIAL_ALLY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::STARLING_AERIAL_ALLY,
    "48255f10-ea0e-42a3-bbfb-efa773f0cb08",
    "Justyna Dura",
);

// OM1 11 — Wild Pack Squad (reprint)
const WILD_PACK_SQUAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WILD_PACK_SQUAD,
    "9706ab5c-129d-42dd-8413-7d4c96916cc9",
    "Bartek Fedyczak",
);

// OM1 12 — Silver Sable, Mercenary Leader (reprint)
const SILVER_SABLE_MERCENARY_LEADER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SILVER_SABLE_MERCENARY_LEADER,
    "a6927378-fee5-49c7-a607-c8084d7d077c",
    "Caio Cacau",
);

// OM1 13 — Thwip! (reprint)
const THWIP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::THWIP,
    "582e9e16-51a6-4f5a-a060-d7e92b108ed5",
    "Alexandre Honoré",
);

// OM1 14 — Origin of Spider-Man (reprint)
const ORIGIN_OF_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ORIGIN_OF_SPIDER_MAN,
    "1bb9f52a-8edc-4ca5-bc63-ac6f93874219",
    "Caio Cacau",
);

// OM1 15 — Web Up (reprint)
const WEB_UP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WEB_UP,
    "b4f1026e-ea10-401a-bceb-2a7eedcd83b3",
    "Andrew Mar",
);

// OM1 16 — Daily Bugle Reporters (reprint)
const DAILY_BUGLE_REPORTERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DAILY_BUGLE_REPORTERS,
    "cc2690bd-390e-4217-a991-340ddc0b487d",
    "Leonardo Santanna",
);

// OM1 17 — Friendly Neighborhood (reprint)
const FRIENDLY_NEIGHBORHOOD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::FRIENDLY_NEIGHBORHOOD,
    "10e0dff7-8923-4e47-bf28-c6d0af78ce24",
    "Carlos Palma Cruchaga",
);

// OM1 18 — Spider-UK (reprint)
const SPIDER_UK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_UK,
    "488f94ef-c9ba-4a96-8abe-99c9a855c5c3",
    "Jodie Muir",
);

// OM1 19 — Spectacular Tactics (reprint)
const SPECTACULAR_TACTICS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPECTACULAR_TACTICS,
    "205cf34c-b3f4-4901-ac0c-ad22533f924c",
    "Cristi Balanescu",
);

// OM1 20 — Sudden Strike (reprint)
const SUDDEN_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SUDDEN_STRIKE,
    "3b8ca716-d1ea-47c9-a919-7da112310133",
    "Edgar Sánchez Hidalgo",
);

// OM1 21 — Peter Parker // Amazing Spider-Man (reprint)
const PETER_PARKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PETER_PARKER,
    "2ec821c3-3e5c-4af8-a6ef-e2e77aa8668c",
    "Chris Mangum",
);

// OM1 22 — Anti-Venom, Horrifying Healer (reprint)
const ANTI_VENOM_HORRIFYING_HEALER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ANTI_VENOM_HORRIFYING_HEALER,
    "84285f17-dbf3-49f6-a61e-2f779cb3e798",
    "Raoul Vitale",
);

// OM1 23 — Arachne, Psionic Weaver (reprint)
const ARACHNE_PSIONIC_WEAVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ARACHNE_PSIONIC_WEAVER,
    "6512f8c1-96bb-4a33-b20f-dd09f6eec5dc",
    "Julia Metzger",
);

// OM1 24 — Aunt May (reprint)
const AUNT_MAY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::AUNT_MAY,
    "83a9e039-f168-40f2-9122-cf6e535e9bb3",
    "Chris Mangum",
);

// OM1 25 — Beetle, Legacy Criminal (reprint)
const BEETLE_LEGACY_CRIMINAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::BEETLE_LEGACY_CRIMINAL,
    "8680fbe9-b231-4334-bca7-7fff7a7b10f9",
    "Andreia Ugrai",
);

// OM1 26 — Spider-Byte, Web Warden (reprint)
const SPIDER_BYTE_WEB_WARDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_BYTE_WEB_WARDEN,
    "aa7d8ca7-5db9-4a22-a921-18dd7c56cf2c",
    "Randy Gallegos",
);

// OM1 27 — Hydro-Man, Fluid Felon (reprint)
const HYDRO_MAN_FLUID_FELON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::HYDRO_MAN_FLUID_FELON,
    "de404b0f-9ba3-4e26-9230-20ce1dbf063b",
    "David Palumbo",
);

// OM1 28 — Spider-Sense (reprint)
const SPIDER_SENSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_SENSE,
    "5eee9160-4f14-4ffc-83e9-ee5931ff6a7f",
    "Diana Franco",
);

// OM1 29 — Amazing Acrobatics (reprint)
const AMAZING_ACROBATICS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::AMAZING_ACROBATICS,
    "29a08e0a-4a3f-471b-89bb-7acc7c94285e",
    "Francis Tneh",
);

// OM1 30 — Norman Osborn // Green Goblin (reprint)
const NORMAN_OSBORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::NORMAN_OSBORN,
    "da13a060-36ee-41e9-bab1-af8ddcc893b8",
    "Nicholas Gregory",
);

// OM1 31 — Spider-Man No More (reprint)
const SPIDER_MAN_NO_MORE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MAN_NO_MORE,
    "07171df9-8998-4d56-a59c-eb3cb9240f53",
    "Julia Metzger",
);

// OM1 32 — Impostor Syndrome (reprint)
const IMPOSTOR_SYNDROME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::IMPOSTOR_SYNDROME,
    "eeef16f4-5bf4-4eea-9891-84b87d1d396c",
    "Tehani Farr",
);

// OM1 33 — The Clone Saga (reprint)
const THE_CLONE_SAGA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::THE_CLONE_SAGA,
    "78cd4dde-7e2d-4fcd-a207-a80911755f3f",
    "Slawomir Maniak",
);

// OM1 34 — Lady Octopus, Inspired Inventor (reprint)
const LADY_OCTOPUS_INSPIRED_INVENTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::LADY_OCTOPUS_INSPIRED_INVENTOR,
    "d264eb63-e883-413a-ba01-99ed257d086f",
    "Claudio Pozas",
);

// OM1 35 — Doc Ock's Henchmen (reprint)
const DOC_OCK_S_HENCHMEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DOC_OCK_S_HENCHMEN,
    "f030db6a-aeaf-481f-90b1-d56ca8a3e430",
    "Matheus Graef",
);

// OM1 36 — Madame Web, Clairvoyant (reprint)
const MADAME_WEB_CLAIRVOYANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MADAME_WEB_CLAIRVOYANT,
    "85ca58c0-bf3a-49fe-848c-adb1449c29bf",
    "Mathias Kollros",
);

// OM1 37 — School Daze (reprint)
const SCHOOL_DAZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SCHOOL_DAZE,
    "62f8dfb3-8591-4946-91a5-f1d936bf3d10",
    "Mark Behm",
);

// OM1 38 — Doc Ock, Sinister Scientist (reprint)
const DOC_OCK_SINISTER_SCIENTIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DOC_OCK_SINISTER_SCIENTIST,
    "15782c61-8310-4fc2-846c-5a00fd302d0b",
    "Kev Fang",
);

// OM1 39 — Mysterio's Phantasm (reprint)
const MYSTERIO_S_PHANTASM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MYSTERIO_S_PHANTASM,
    "42b7c039-97c5-4a46-9d0a-43205da77d86",
    "Igor Krstic",
);

// OM1 40 — Oscorp Research Team (reprint)
const OSCORP_RESEARCH_TEAM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::OSCORP_RESEARCH_TEAM,
    "3fb8e876-3687-4850-bebc-fbd1b86b15a3",
    "Nino Vecia",
);

// OM1 41 — Robotics Mastery (reprint)
const ROBOTICS_MASTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ROBOTICS_MASTERY,
    "b9be8e17-7304-4df9-bdeb-8f31c604056c",
    "Andrew Mar",
);

// OM1 42 — Secret Identity (reprint)
const SECRET_IDENTITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SECRET_IDENTITY,
    "87e8e5cf-20a5-4ef9-9c19-8967c02c53e6",
    "Bram Sels",
);

// OM1 43 — Whoosh! (reprint)
const WHOOSH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WHOOSH,
    "bf527252-a828-47c5-8194-4a866e39d90c",
    "Maxime Minard",
);

// OM1 44 — Hide on the Ceiling (reprint)
const HIDE_ON_THE_CEILING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::HIDE_ON_THE_CEILING,
    "168b8831-6aab-4545-9ba7-d4f2af9920ff",
    "Justyna Dura",
);

// OM1 45 — Unstable Experiment (reprint)
const UNSTABLE_EXPERIMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::UNSTABLE_EXPERIMENT,
    "12f22890-901e-482d-a6e8-b224a39af6e4",
    "Adrián Rodríguez Pérez",
);

// OM1 46 — Chameleon, Master of Disguise (reprint)
const CHAMELEON_MASTER_OF_DISGUISE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::CHAMELEON_MASTER_OF_DISGUISE,
    "12469736-a917-4b6a-973f-bf8380bd3e40",
    "Andrey Kuzinskiy",
);

// OM1 47 — Flying Octobot (reprint)
const FLYING_OCTOBOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::FLYING_OCTOBOT,
    "8c342dfd-8d0e-4c5a-bb4c-a383a4493d00",
    "Jonathan Wayshak",
);

// OM1 48 — Mysterio, Master of Illusion (reprint)
const MYSTERIO_MASTER_OF_ILLUSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MYSTERIO_MASTER_OF_ILLUSION,
    "ffca39eb-f611-46b1-9d6b-a5c90c722fb9",
    "Leonardo Santanna",
);

// OM1 49 — Alien Symbiosis (reprint)
const ALIEN_SYMBIOSIS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ALIEN_SYMBIOSIS,
    "c2cbebab-ea84-4a38-8d1e-6e7fc7d0655d",
    "Helge C. Balzer",
);

// OM1 50 — Venomized Cat (reprint)
const VENOMIZED_CAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::VENOMIZED_CAT,
    "9c88ef90-b3d9-4d26-900a-caafdac01597",
    "Kev Walker",
);

// OM1 51 — Inner Demons Gangsters (reprint)
const INNER_DEMONS_GANGSTERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::INNER_DEMONS_GANGSTERS,
    "be92071a-94a7-4094-a5a8-c610f3c18a6f",
    "Warren Mahy",
);

// OM1 52 — Risky Research (reprint)
const RISKY_RESEARCH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::RISKY_RESEARCH,
    "b08238f7-9e00-4122-b196-05d623c84f8f",
    "Will Gist",
);

// OM1 53 — Common Crook (reprint)
const COMMON_CROOK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::COMMON_CROOK,
    "7d7b5a4a-5dc0-46f0-94dc-35723423f8b9",
    "Gil Martimiano",
);

// OM1 54 — Venom's Hunger (reprint)
const VENOM_S_HUNGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::VENOM_S_HUNGER,
    "ef974a13-4eea-4c96-adfe-6f42b71563d4",
    "Marko Manev",
);

// OM1 55 — Parker Luck (reprint)
const PARKER_LUCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PARKER_LUCK,
    "2e4d38dc-d670-4c10-906f-988e4102f5f0",
    "Camille Alquier",
);

// OM1 56 — Gwenom, Remorseless (reprint)
const GWENOM_REMORSELESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::GWENOM_REMORSELESS,
    "fcd9da8a-e36f-4ae1-9a5d-b72ad9c27397",
    "Lars Grant-West",
);

// OM1 57 — Behold the Sinister Six! (reprint)
const BEHOLD_THE_SINISTER_SIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::BEHOLD_THE_SINISTER_SIX,
    "2cdb2129-2cb9-49a0-a30b-34a4b2534f41",
    "Karl Kopinski",
);

// OM1 58 — The Death of Gwen Stacy (reprint)
const THE_DEATH_OF_GWEN_STACY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::THE_DEATH_OF_GWEN_STACY,
    "6db83692-1c33-4e61-b8ec-dc030bf2c963",
    "Carlos Palma Cruchaga",
);

// OM1 59 — Spider-Man Noir (reprint)
const SPIDER_MAN_NOIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MAN_NOIR,
    "1f0b6b31-c3e1-49bb-bc59-ce6414cde878",
    "Michele Giorgi",
);

// OM1 60 — Morlun, Devourer of Spiders (reprint)
const MORLUN_DEVOURER_OF_SPIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MORLUN_DEVOURER_OF_SPIDERS,
    "18091791-5089-483f-9446-c2684c5da03e",
    "Loïc Canavaggia",
);

// OM1 61 — Merciless Enforcers (reprint)
const MERCILESS_ENFORCERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MERCILESS_ENFORCERS,
    "e21d4a89-cd92-44d2-b66b-b30c82697ff6",
    "Cristi Balanescu",
);

// OM1 62 — Sandman's Quicksand (reprint)
const SANDMAN_S_QUICKSAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SANDMAN_S_QUICKSAND,
    "88590608-5f0e-4c04-9df6-8dd6b03974b8",
    "Mathias Kollros",
);

// OM1 63 — Tombstone, Career Criminal (reprint)
const TOMBSTONE_CAREER_CRIMINAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::TOMBSTONE_CAREER_CRIMINAL,
    "079d9bfa-f039-47a9-9b6e-41879028d2da",
    "Carlos Palma Cruchaga",
);

// OM1 64 — Prison Break (reprint)
const PRISON_BREAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PRISON_BREAK,
    "ac0bb4b8-f6ff-4f5f-9ab7-f281c5344aba",
    "Hristo D. Chukov",
);

// OM1 65 — The Spot's Portal (reprint)
const THE_SPOT_S_PORTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::THE_SPOT_S_PORTAL,
    "097475b0-24d2-470c-bc3e-3f2fff3a5ebe",
    "Warren Mahy",
);

// OM1 66 — Agent Venom (reprint)
const AGENT_VENOM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::AGENT_VENOM,
    "d62cf4f8-36a2-4d9f-9d52-53ea18a52760",
    "Diana Franco",
);

// OM1 67 — Scorpion's Sting (reprint)
const SCORPION_S_STING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SCORPION_S_STING,
    "2be7387a-4350-4c9e-8f5d-7716b903e476",
    "Inkognit",
);

// OM1 68 — Scorpion, Seething Striker (reprint)
const SCORPION_SEETHING_STRIKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SCORPION_SEETHING_STRIKER,
    "0ac4c44a-d74b-47f3-91d9-bffc3cc4eaae",
    "Dmitry Burmak",
);

// OM1 69 — The Soul Stone (reprint)
const THE_SOUL_STONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::THE_SOUL_STONE,
    "a54f47cd-d897-4da4-be67-aa8eb71bdac5",
    "Adam Paquette",
);

// OM1 70 — Venom, Evil Unleashed (reprint)
const VENOM_EVIL_UNLEASHED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::VENOM_EVIL_UNLEASHED,
    "add0aa73-232a-4d4f-9e04-4cd2a6ff05bc",
    "Javier Charro",
);

// OM1 71 — Eddie Brock // Venom, Lethal Protector (reprint)
const EDDIE_BROCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::EDDIE_BROCK,
    "42a3589f-f09e-4298-b1ae-7165de65bfb8",
    "Michael Phillippi",
);

// OM1 72 — Villainous Wrath (reprint)
const VILLAINOUS_WRATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::VILLAINOUS_WRATH,
    "acd597bf-554e-4456-9215-98917e835d67",
    "Jonathan Wayshak",
);

// OM1 73 — Swarm, Being of Bees (reprint)
const SWARM_BEING_OF_BEES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SWARM_BEING_OF_BEES,
    "1898ea58-a4ef-4ec0-a88a-22ee14b030ac",
    "Inkognit",
);

// OM1 74 — Black Cat, Cunning Thief (reprint)
const BLACK_CAT_CUNNING_THIEF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::BLACK_CAT_CUNNING_THIEF,
    "e92c1e20-8ab9-425c-bd1a-469e4f091b7e",
    "Yohann Schepacz",
);

// OM1 75 — Spider-Verse (reprint)
const SPIDER_VERSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_VERSE,
    "c3fd31e5-0faa-493f-aa50-08307a0c0139",
    "Karl Kopinski",
);

// OM1 76 — Electro, Assaulting Battery (reprint)
const ELECTRO_ASSAULTING_BATTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ELECTRO_ASSAULTING_BATTERY,
    "5b12f23c-54b2-4b28-8434-c3f509a204d7",
    "Dmitry Burmak",
);

// OM1 77 — Hobgoblin, Mantled Marauder (reprint)
const HOBGOBLIN_MANTLED_MARAUDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::HOBGOBLIN_MANTLED_MARAUDER,
    "a4c5097b-204f-4dd1-b2f7-81555c408dd7",
    "Warren Mahy",
);

// OM1 78 — Electro's Bolt (reprint)
const ELECTRO_S_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ELECTRO_S_BOLT,
    "c5af25b9-af79-4cc2-a4d2-126152a873b9",
    "Claudio Pozas",
);

// OM1 79 — Heroes' Hangout (reprint)
const HEROES_HANGOUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::HEROES_HANGOUT,
    "a15359e5-3d49-45c1-92fd-406bcc8c3afe",
    "Simon Dominic",
);

// OM1 80 — Taxi Driver (reprint)
const TAXI_DRIVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::TAXI_DRIVER,
    "e64d0478-b42f-4b7d-abac-fd1bce602191",
    "Caio Monteiro",
);

// OM1 81 — Angry Rabble (reprint)
const ANGRY_RABBLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ANGRY_RABBLE,
    "80a99b95-5783-4947-8ce6-3ee1ecfa2525",
    "Diego Gisbert",
);

// OM1 82 — Molten Man, Inferno Incarnate (reprint)
const MOLTEN_MAN_INFERNO_INCARNATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MOLTEN_MAN_INFERNO_INCARNATE,
    "70d0edde-c5a5-406f-8933-bdda1abb5c05",
    "Tuan Duong Chu",
);

// OM1 83 — Shocker, Unshakable (reprint)
const SHOCKER_UNSHAKABLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SHOCKER_UNSHAKABLE,
    "eb8acf92-22c2-462f-af2d-bc07217e83a9",
    "Julian Kok Joon Wen",
);

// OM1 84 — Stegron the Dinosaur Man (reprint)
const STEGRON_THE_DINOSAUR_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::STEGRON_THE_DINOSAUR_MAN,
    "d1ac4300-5589-4a21-af20-87b58ddae09b",
    "Lars Grant-West",
);

// OM1 85 — Spider-Punk (reprint)
const SPIDER_PUNK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_PUNK,
    "774d48ab-181b-4190-9956-67706bea9b27",
    "Vincent Proce",
);

// OM1 86 — Raging Goblinoids (reprint)
const RAGING_GOBLINOIDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::RAGING_GOBLINOIDS,
    "5aff08a7-cc92-4792-abbd-c5019700a6ab",
    "Tuan Duong Chu",
);

// OM1 87 — Maximum Carnage (reprint)
const MAXIMUM_CARNAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MAXIMUM_CARNAGE,
    "d8d0a8fd-57ec-4812-8c40-82380a3eb8d5",
    "Tiffany Turrill",
);

// OM1 88 — J. Jonah Jameson (reprint)
const J_JONAH_JAMESON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::J_JONAH_JAMESON,
    "380ab614-e46d-476b-bce9-fd2a8c074906",
    "Olivier Bernard",
);

// OM1 89 — Gwen Stacy // Ghost-Spider (reprint)
const GWEN_STACY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::GWEN_STACY,
    "a301083a-9130-4384-a355-efe7050871ca",
    "Jakob Eirich",
);

// OM1 90 — Shadow of the Goblin (reprint)
const SHADOW_OF_THE_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SHADOW_OF_THE_GOBLIN,
    "cd68f368-6b6c-4c1e-a971-11e549e029c3",
    "Dallas Williams",
);

// OM1 91 — Superior Foes of Spider-Man (reprint)
const SUPERIOR_FOES_OF_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SUPERIOR_FOES_OF_SPIDER_MAN,
    "7d66d31e-91eb-48b5-9388-9f6adfd89e70",
    "Mirko Failoni",
);

// OM1 92 — Spinneret and Spiderling (reprint)
const SPINNERET_AND_SPIDERLING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPINNERET_AND_SPIDERLING,
    "32b086a3-1a5a-4765-918e-06a11770079d",
    "Igor Krstic",
);

// OM1 93 — Romantic Rendezvous (reprint)
const ROMANTIC_RENDEZVOUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ROMANTIC_RENDEZVOUS,
    "d18dc61a-377e-4b84-9fc5-6496e21c9346",
    "Mariah Tekulve",
);

// OM1 94 — Spider-Islanders (reprint)
const SPIDER_ISLANDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_ISLANDERS,
    "7148fd5f-e80a-4f2f-9f3d-8b68ff9d2f67",
    "Helge C. Balzer",
);

// OM1 95 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::SHOCK,
    "4aef9732-28a6-4e6a-b1ba-79b28d85d78b",
    "Jason Rainville",
);

// OM1 96 — Masked Meower (reprint)
const MASKED_MEOWER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MASKED_MEOWER,
    "6bcdcc73-e854-4e7f-9911-ac9c51965c7b",
    "Deruchenko Alexander",
);

// OM1 97 — Spider-Gwen, Free Spirit (reprint)
const SPIDER_GWEN_FREE_SPIRIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_GWEN_FREE_SPIRIT,
    "d643a81f-2b6b-430b-9123-3df43072fa5f",
    "Jesper Ejsing",
);

// OM1 98 — Wisecrack (reprint)
const WISECRACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WISECRACK,
    "12a4bbe1-d341-4da2-8180-81f2ce964cb2",
    "Xabi Gaztelua",
);

// OM1 99 — Pictures of Spider-Man (reprint)
const PICTURES_OF_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PICTURES_OF_SPIDER_MAN,
    "42b0bd7b-38a6-465c-badf-ea72989160b9",
    "Andrew Mar",
);

// OM1 100 — Spider-Rex, Daring Dino (reprint)
const SPIDER_REX_DARING_DINO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_REX_DARING_DINO,
    "36320a3a-e771-4763-b147-f328724cf36c",
    "Caio Monteiro",
);

// OM1 101 — Spiders-Man, Heroic Horde (reprint)
const SPIDERS_MAN_HEROIC_HORDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDERS_MAN_HEROIC_HORDE,
    "29f7f242-57fa-4714-84fa-3e65c7101985",
    "Michele Giorgi",
);

// OM1 102 — Miles Morales // Ultimate Spider-Man (reprint)
const MILES_MORALES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MILES_MORALES,
    "7feb1493-b73e-4ee3-968e-d69806360952",
    "Loïc Canavaggia",
);

// OM1 103 — Kraven's Cats (reprint)
const KRAVEN_S_CATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::KRAVEN_S_CATS,
    "9fa40406-82da-45e6-a04f-62393e99128c",
    "Warren Mahy",
);

// OM1 104 — Guy in the Chair (reprint)
const GUY_IN_THE_CHAIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::GUY_IN_THE_CHAIR,
    "bf8c2233-1d5f-4081-a31e-e4305887a1af",
    "Ben Maier",
);

// OM1 105 — Radioactive Spider (reprint)
const RADIOACTIVE_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::RADIOACTIVE_SPIDER,
    "81c44f97-df99-4ee3-a38e-163675d1afda",
    "Gonzalo Kenny",
);

// OM1 106 — Professional Wrestler (reprint)
const PROFESSIONAL_WRESTLER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PROFESSIONAL_WRESTLER,
    "5e1bc145-8e4b-484a-bf29-4b46089d9300",
    "Chris Mangum",
);

// OM1 107 — Grow Extra Arms (reprint)
const GROW_EXTRA_ARMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::GROW_EXTRA_ARMS,
    "181c2f5d-9ebc-40ea-8820-7f7a7cc008fd",
    "Mathias Kollros",
);

// OM1 108 — Lizard, Connors's Curse (reprint)
const LIZARD_CONNORS_S_CURSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::LIZARD_CONNORS_S_CURSE,
    "6361ffac-ad30-4b6a-aea2-c804ad51c773",
    "Andrew Mar",
);

// OM1 109 — Lurking Lizards (reprint)
const LURKING_LIZARDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::LURKING_LIZARDS,
    "61bf3024-6db2-4fc3-a41d-01897956c3c5",
    "Henry Peters",
);

// OM1 110 — Ezekiel Sims, Spider-Totem (reprint)
const EZEKIEL_SIMS_SPIDER_TOTEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::EZEKIEL_SIMS_SPIDER_TOTEM,
    "bf2b3695-f5f3-4b17-bde3-90ec0c6b9e5b",
    "Yohann Schepacz",
);

// OM1 111 — Kapow! (reprint)
const KAPOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::KAPOW,
    "16f0ef01-aa78-4ea3-b1ae-cc9594196425",
    "Arif Wijaya",
);

// OM1 112 — Wall Crawl (reprint)
const WALL_CRAWL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WALL_CRAWL,
    "c37e8b36-00bc-4fc0-89e1-dfd8a3188465",
    "Vincent Proce",
);

// OM1 113 — Sandman, Shifting Scoundrel (reprint)
const SANDMAN_SHIFTING_SCOUNDREL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SANDMAN_SHIFTING_SCOUNDREL,
    "62531cc3-c218-41f0-a3a2-beb42185baaf",
    "Bartek Fedyczak",
);

// OM1 114 — Scout the City (reprint)
const SCOUT_THE_CITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SCOUT_THE_CITY,
    "155c41e0-98e1-4c86-9cac-73276be5d3c6",
    "Caio Monteiro",
);

// OM1 115 — Damage Control Crew (reprint)
const DAMAGE_CONTROL_CREW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DAMAGE_CONTROL_CREW,
    "3af61943-e437-4ddf-988f-820e66c79228",
    "Caio Cacau",
);

// OM1 116 — Strength of Will (reprint)
const STRENGTH_OF_WILL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::STRENGTH_OF_WILL,
    "24b3e1b0-a6af-46da-820a-439a292f7de0",
    "Rhonda Libbey",
);

// OM1 117 — Supportive Parents (reprint)
const SUPPORTIVE_PARENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SUPPORTIVE_PARENTS,
    "09bda521-5d2d-4a61-a533-06361e91e4cb",
    "Elizabeth Peiró",
);

// OM1 117† — Supportive Parents (alternate printing)
const SUPPORTIVE_PARENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_spm::SUPPORTIVE_PARENTS,
    1,
    "dfcd2602-6722-420e-a5e2-49cc046ccd5c",
    "Elizabeth Peiró",
);

// OM1 118 — Spider-Ham, Peter Porker (reprint)
const SPIDER_HAM_PETER_PORKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_HAM_PETER_PORKER,
    "b1c95ee4-b60d-4868-b7e3-204849a8f544",
    "Andrew Mar",
);

// OM1 119 — Terrific Team-Up (reprint)
const TERRIFIC_TEAM_UP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::TERRIFIC_TEAM_UP,
    "0af974cd-f987-4cb8-85d5-77a4bd894d7b",
    "Hristo D. Chukov",
);

// OM1 120 — Web of Life and Destiny (reprint)
const WEB_OF_LIFE_AND_DESTINY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WEB_OF_LIFE_AND_DESTINY,
    "13025ff4-7dd5-4fc4-806f-d237a7b3bc32",
    "Allen Panakal",
);

// OM1 121 — Kraven's Last Hunt (reprint)
const KRAVEN_S_LAST_HUNT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::KRAVEN_S_LAST_HUNT,
    "8e34d252-c96d-4f75-bf56-2a9b1e62c002",
    "Volkan Baǵa",
);

// OM1 122 — Spider-Man, Brooklyn Visionary (reprint)
const SPIDER_MAN_BROOKLYN_VISIONARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MAN_BROOKLYN_VISIONARY,
    "eb4fc620-2c48-46d9-9046-157285e820a6",
    "John Tedrick",
);

// OM1 123 — Silk, Web Weaver (reprint)
const SILK_WEB_WEAVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SILK_WEB_WEAVER,
    "458cd7a1-7437-493b-b7b7-42ef489e77df",
    "Dmitry Burmak",
);

// OM1 124 — Spider-Man India (reprint)
const SPIDER_MAN_INDIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MAN_INDIA,
    "b851b3f4-0f88-44ce-bd75-b4cf083967f4",
    "Viko Menezes",
);

// OM1 125 — Biorganic Carapace (reprint)
const BIORGANIC_CARAPACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::BIORGANIC_CARAPACE,
    "e03889a9-f481-4704-8953-919292fc4d28",
    "Randy Gallegos",
);

// OM1 126 — Scarlet Spider, Ben Reilly (reprint)
const SCARLET_SPIDER_BEN_REILLY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SCARLET_SPIDER_BEN_REILLY,
    "a6d9b8d6-1e97-4e9c-b0c4-3b71734134a4",
    "Alexandre Honoré",
);

// OM1 127 — Kraven the Hunter (reprint)
const KRAVEN_THE_HUNTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::KRAVEN_THE_HUNTER,
    "7f0210ec-36eb-4138-a6a2-4c5c67bf5ce6",
    "Diego Gisbert",
);

// OM1 128 — Prowler, Clawed Thief (reprint)
const PROWLER_CLAWED_THIEF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PROWLER_CLAWED_THIEF,
    "f7717ba8-c225-40ad-bd2e-282d0f43cdaf",
    "Cristi Balanescu",
);

// OM1 129 — Cheering Crowd (reprint)
const CHEERING_CROWD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::CHEERING_CROWD,
    "a1c5d939-b01b-4ca0-a1e3-1959db9d9ef4",
    "Deruchenko Alexander",
);

// OM1 130 — Sun-Spider, Nimble Webber (reprint)
const SUN_SPIDER_NIMBLE_WEBBER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SUN_SPIDER_NIMBLE_WEBBER,
    "487c104e-ab0d-4310-973c-a3661d6dabe2",
    "Alexandre Honoré",
);

// OM1 131 — Shriek, Treblemaker (reprint)
const SHRIEK_TREBLEMAKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SHRIEK_TREBLEMAKER,
    "1eb20949-de39-4da8-903b-c5d7cd786342",
    "Billy Christian",
);

// OM1 132 — Mary Jane Watson (reprint)
const MARY_JANE_WATSON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MARY_JANE_WATSON,
    "566eca5c-fbc3-4ac8-9b20-4c15b0101abd",
    "Inkognit",
);

// OM1 133 — Carnage, Crimson Chaos (reprint)
const CARNAGE_CRIMSON_CHAOS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::CARNAGE_CRIMSON_CHAOS,
    "e1332480-4986-421f-9a31-1d597d597c65",
    "Diana Franco",
);

// OM1 134 — Web-Warriors (reprint)
const WEB_WARRIORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WEB_WARRIORS,
    "fafc21c6-010b-49dd-b11b-a1f691377028",
    "Michele Giorgi",
);

// OM1 135 — Kraven, Proud Predator (reprint)
const KRAVEN_PROUD_PREDATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::KRAVEN_PROUD_PREDATOR,
    "69a6759f-45c8-434b-a18d-2b92221914dd",
    "Inkognit",
);

// OM1 136 — Jackal, Genius Geneticist (reprint)
const JACKAL_GENIUS_GENETICIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::JACKAL_GENIUS_GENETICIST,
    "c6753c36-4aab-4608-b2e2-c7b0519cb0c6",
    "Arif Wijaya",
);

// OM1 137 — Gallant Citizen (reprint)
const GALLANT_CITIZEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::GALLANT_CITIZEN,
    "2ee7271f-ab1c-4332-b5cb-69b7cea80912",
    "Mirko Failoni",
);

// OM1 138 — Araña, Heart of the Spider (reprint)
const ARANA_HEART_OF_THE_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ARANA_HEART_OF_THE_SPIDER,
    "f41cc164-e799-492b-a155-43b218042189",
    "Alexander Mokhov",
);

// OM1 139 — Spider-Girl, Legacy Hero (reprint)
const SPIDER_GIRL_LEGACY_HERO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_GIRL_LEGACY_HERO,
    "ebc534ac-c33b-4059-9d11-82645f8fead1",
    "Mathias Kollros",
);

// OM1 140 — Superior Spider-Man (reprint)
const SUPERIOR_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SUPERIOR_SPIDER_MAN,
    "0f193a36-d452-4503-937c-90fb55a99ab4",
    "Claudio Pozas",
);

// OM1 141 — Scarlet Spider, Kaine (reprint)
const SCARLET_SPIDER_KAINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SCARLET_SPIDER_KAINE,
    "5f5a13ff-8763-4923-a610-8cfd08bc8d2b",
    "Alexander Mokhov",
);

// OM1 142 — Pumpkin Bombardment (reprint)
const PUMPKIN_BOMBARDMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PUMPKIN_BOMBARDMENT,
    "7e842fe7-b9dc-45fc-a85a-3c32a943fd34",
    "Javier Charro",
);

// OM1 143 — SP//dr, Piloted by Peni (reprint)
const SP_DR_PILOTED_BY_PENI_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SP_DR_PILOTED_BY_PENI,
    "c10d6ea8-f670-41c9-8aa0-66df5fd69f34",
    "Joe Slucher",
);

// OM1 144 — Spider Manifestation (reprint)
const SPIDER_MANIFESTATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MANIFESTATION,
    "0f6824b0-02d3-4b2a-bf74-61bd38e7e632",
    "Xavier Ribeiro",
);

// OM1 145 — Spider-Woman, Stunning Savior (reprint)
const SPIDER_WOMAN_STUNNING_SAVIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_WOMAN_STUNNING_SAVIOR,
    "fbf1ea21-2e90-4792-b399-111ff70a9d06",
    "Mirko Failoni",
);

// OM1 146 — Wraith, Vicious Vigilante (reprint)
const WRAITH_VICIOUS_VIGILANTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WRAITH_VICIOUS_VIGILANTE,
    "1a2d4e77-f22f-4664-bd97-98840a5e94aa",
    "Andreia Ugrai",
);

// OM1 147 — Mob Lookout (reprint)
const MOB_LOOKOUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MOB_LOOKOUT,
    "a84738ef-d3b7-49de-9cc6-86000527a534",
    "Francisco Badilla",
);

// OM1 148 — Doctor Octopus, Master Planner (reprint)
const DOCTOR_OCTOPUS_MASTER_PLANNER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DOCTOR_OCTOPUS_MASTER_PLANNER,
    "7772b146-80ff-4906-84e4-69c704fa6f4c",
    "Artur Nakhodkin",
);

// OM1 149 — Green Goblin, Revenant (reprint)
const GREEN_GOBLIN_REVENANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::GREEN_GOBLIN_REVENANT,
    "54044eb2-ff97-49e3-a123-b707241e513d",
    "Daren Bader",
);

// OM1 150 — The Spot, Living Portal (reprint)
const THE_SPOT_LIVING_PORTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::THE_SPOT_LIVING_PORTAL,
    "dce39404-b2d5-41e1-84e8-8da3657c278c",
    "Andreas Zafiratos",
);

// OM1 151 — Rhino's Rampage (reprint)
const RHINOS_RAMPAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::RHINOS_RAMPAGE,
    "a0a11134-15bf-4f38-a0ef-6bf146149c9d",
    "Gonzalo Kenny",
);

// OM1 152 — Vulture, Scheming Scavenger (reprint)
const VULTURE_SCHEMING_SCAVENGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::VULTURE_SCHEMING_SCAVENGER,
    "1392de20-9ed9-4852-ad13-a56c51c8a247",
    "Josh Hass",
);

// OM1 153 — Ultimate Green Goblin (reprint)
const ULTIMATE_GREEN_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ULTIMATE_GREEN_GOBLIN,
    "9905b88b-4b4d-4a14-8032-7f23d4517d8b",
    "Caio Cacau",
);

// OM1 154 — Cosmic Spider-Man (reprint)
const COSMIC_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::COSMIC_SPIDER_MAN,
    "b2996f07-9dbc-444f-aaae-710866da2b4b",
    "Jesper Ejsing",
);

// OM1 155 — Symbiote Spider-Man (reprint)
const SYMBIOTE_SPIDER_MAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SYMBIOTE_SPIDER_MAN,
    "aba6def8-b2db-4083-a496-45afc7d803c0",
    "Igor Krstic",
);

// OM1 156 — Morbius the Living Vampire (reprint)
const MORBIUS_THE_LIVING_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MORBIUS_THE_LIVING_VAMPIRE,
    "63aa6c28-5b8a-4ef3-80ff-e62d34538ed0",
    "Andrey Kuzinskiy",
);

// OM1 157 — Spider-Man 2099 (reprint)
const SPIDER_MAN_2099_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MAN_2099,
    "dc6a2e6c-7096-4466-90ab-828290f098cc",
    "Slawomir Maniak",
);

// OM1 158 — Mister Negative (reprint)
const MISTER_NEGATIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MISTER_NEGATIVE,
    "198352f7-dd8d-4dce-b985-3917078bd0f1",
    "Vincent Proce",
);

// OM1 159 — Skyward Spider (reprint)
const SKYWARD_SPIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SKYWARD_SPIDER,
    "57a37326-f399-4e75-a1b1-c34e2e0a8ae6",
    "Eelis Kyttanen",
);

// OM1 160 — Rhino, Barreling Brute (reprint)
const RHINO_BARRELING_BRUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::RHINO_BARRELING_BRUTE,
    "7d9b096d-8813-44a0-835e-fddb61c576b0",
    "Inkognit",
);

// OM1 161 — Spider-Suit (reprint)
const SPIDER_SUIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_SUIT,
    "522bc3df-0ec8-466c-9996-c403823cc755",
    "Daniel Ljunggren",
);

// OM1 162 — Passenger Ferry (reprint)
const PASSENGER_FERRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PASSENGER_FERRY,
    "965ca8a1-e93b-4cac-903a-d7fa675b393c",
    "Carlos Palma Cruchaga",
);

// OM1 163 — Mechanical Mobster (reprint)
const MECHANICAL_MOBSTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::MECHANICAL_MOBSTER,
    "540e334b-5c75-499e-860c-38bd260792d5",
    "Caio Monteiro",
);

// OM1 164 — Eerie Gravestone (reprint)
const EERIE_GRAVESTONE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::EERIE_GRAVESTONE,
    "44a156d4-6a2e-4f5a-ad12-ee19825e465f",
    "Francisco Badilla",
);

// OM1 165 — Living Brain, Mechanical Marvel (reprint)
const LIVING_BRAIN_MECHANICAL_MARVEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::LIVING_BRAIN_MECHANICAL_MARVEL,
    "74b2cb69-7f02-4a48-ac5f-bdcced99c59c",
    "Steve Ellis",
);

// OM1 166 — Iron Spider, Stark Upgrade (reprint)
const IRON_SPIDER_STARK_UPGRADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::IRON_SPIDER_STARK_UPGRADE,
    "0229a5a9-eb7f-4eba-8b1e-f2271e312d4a",
    "Joshua Cairos",
);

// OM1 167 — Doc Ock's Tentacles (reprint)
const DOC_OCK_S_TENTACLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DOC_OCK_S_TENTACLES,
    "77e06f44-aa89-4f31-b1e6-10bca9685511",
    "Claudio Pozas",
);

// OM1 168 — Spider-Slayer, Hatred Honed (reprint)
const SPIDER_SLAYER_HATRED_HONED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_SLAYER_HATRED_HONED,
    "a8874e6e-400f-48ff-a9da-6bce557bde78",
    "Warren Mahy",
);

// OM1 169 — News Helicopter (reprint)
const NEWS_HELICOPTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::NEWS_HELICOPTER,
    "cfe529d7-4418-4068-8f9d-6d74a74fd291",
    "Victor Sales",
);

// OM1 170 — Bagel and Schmear (reprint)
const BAGEL_AND_SCHMEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::BAGEL_AND_SCHMEAR,
    "feba37ca-7df5-401f-8d16-9ca402ca8df1",
    "Henry Peters",
);

// OM1 171 — Peter Parker's Camera (reprint)
const PETER_PARKER_S_CAMERA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::PETER_PARKER_S_CAMERA,
    "b0c811f8-b456-47b3-9d55-a87590eb4573",
    "Tianxing Xu",
);

// OM1 172 — Interdimensional Web Watch (reprint)
const INTERDIMENSIONAL_WEB_WATCH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::INTERDIMENSIONAL_WEB_WATCH,
    "a3dddd1e-bdad-4c20-9b35-6679f4fcbd53",
    "Alex Konstad",
);

// OM1 173 — Spider-Mobile (reprint)
const SPIDER_MOBILE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_MOBILE,
    "48a433c4-9849-46b5-881d-64c05b87faf8",
    "Filip Burburan",
);

// OM1 174 — Spider-Bot (reprint)
const SPIDER_BOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SPIDER_BOT,
    "df1e0d83-e6ac-4256-b7be-ffd71824c466",
    "Joshua Cairos",
);

// OM1 175 — Rocket-Powered Goblin Glider (reprint)
const ROCKET_POWERED_GOBLIN_GLIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::ROCKET_POWERED_GOBLIN_GLIDER,
    "cd043c7b-7c9c-431a-9904-944115fb222f",
    "Kev Fang",
);

// OM1 176 — Steel Wrecking Ball (reprint)
const STEEL_WRECKING_BALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::STEEL_WRECKING_BALL,
    "296b300a-115a-458e-b526-65a1679053ca",
    "Kev Fang",
);

// OM1 177 — Hot Dog Cart (reprint)
const HOT_DOG_CART_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::HOT_DOG_CART,
    "38812303-9c39-4bc3-ab60-5a845f66e5e9",
    "Josiah \"Jo\" Cameron",
);

// OM1 178 — Subway Train (reprint)
const SUBWAY_TRAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::SUBWAY_TRAIN,
    "f676c797-b4d5-4a3e-8036-54595539e8e5",
    "Camille Alquier",
);

// OM1 179 — Oscorp Industries (reprint)
const OSCORP_INDUSTRIES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::OSCORP_INDUSTRIES,
    "cb962bcf-5d63-49b1-a383-068e53bb4db5",
    "Francisco Badilla",
);

// OM1 180 — Daily Bugle Building (reprint)
const DAILY_BUGLE_BUILDING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::DAILY_BUGLE_BUILDING,
    "eba92a05-06f1-4978-a92e-925739c20f8c",
    "Adam Paquette",
);

// OM1 181 — Multiversal Passage (reprint)
const MULTIVERSAL_PASSAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &MULTIVERSAL_PASSAGE,
    "21502958-a8e3-494a-9be9-bebbbb1dd9dc",
    "Daren Bader",
);

// OM1 182 — Ominous Asylum (reprint)
const OMINOUS_ASYLUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &OMINOUS_ASYLUM,
    "371b03a1-7707-4a8a-8c0e-0272418c801f",
    "Daniel Ljunggren",
);

// OM1 183 — Savage Mansion (reprint)
const SAVAGE_MANSION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &SAVAGE_MANSION,
    "c172cdb5-aa2c-419d-b8ab-4795f4b7e160",
    "Vincent Proce",
);

// OM1 184 — Sinister Hideout (reprint)
const SINISTER_HIDEOUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &SINISTER_HIDEOUT,
    "c417f8ce-e156-4c9a-af30-792606d861bd",
    "Julian Kok Joon Wen",
);

// OM1 185 — Suburban Sanctuary (reprint)
const SUBURBAN_SANCTUARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &SUBURBAN_SANCTUARY,
    "cabf021b-23e9-404d-90c6-eef629e1283e",
    "Victor Sales",
);

// OM1 186 — University Campus (reprint)
const UNIVERSITY_CAMPUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &UNIVERSITY_CAMPUS,
    "cd4b9fc5-fe3d-41d9-9d0e-77f1aebef618",
    "Randy Gallegos",
);

// OM1 187 — Urban Retreat (reprint)
const URBAN_RETREAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::URBAN_RETREAT,
    "503fa949-955b-4307-92c8-842692305b7a",
    "Julia Metzger",
);

// OM1 188 — Vibrant Cityscape (reprint)
const VIBRANT_CITYSCAPE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::VIBRANT_CITYSCAPE,
    "2bda06d5-1b67-4a01-b7f7-9fcad54b43e9",
    "Julian Kok Joon Wen",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    SPECTACULAR_SPIDER_MAN_REPRINT,
    WEB_SHOOTERS_REPRINT,
    FLASH_THOMPSON_SPIDER_FAN_REPRINT,
    WITH_GREAT_POWER_REPRINT,
    CITY_PIGEON_REPRINT,
    RENT_IS_DUE_REPRINT,
    COSTUME_CLOSET_REPRINT,
    SELFLESS_POLICE_CAPTAIN_REPRINT,
    SPIDER_MAN_WEB_SLINGER_REPRINT,
    STARLING_AERIAL_ALLY_REPRINT,
    WILD_PACK_SQUAD_REPRINT,
    SILVER_SABLE_MERCENARY_LEADER_REPRINT,
    THWIP_REPRINT,
    ORIGIN_OF_SPIDER_MAN_REPRINT,
    WEB_UP_REPRINT,
    DAILY_BUGLE_REPORTERS_REPRINT,
    FRIENDLY_NEIGHBORHOOD_REPRINT,
    SPIDER_UK_REPRINT,
    SPECTACULAR_TACTICS_REPRINT,
    SUDDEN_STRIKE_REPRINT,
    PETER_PARKER_REPRINT,
    ANTI_VENOM_HORRIFYING_HEALER_REPRINT,
    ARACHNE_PSIONIC_WEAVER_REPRINT,
    AUNT_MAY_REPRINT,
    BEETLE_LEGACY_CRIMINAL_REPRINT,
    SPIDER_BYTE_WEB_WARDEN_REPRINT,
    HYDRO_MAN_FLUID_FELON_REPRINT,
    SPIDER_SENSE_REPRINT,
    AMAZING_ACROBATICS_REPRINT,
    NORMAN_OSBORN_REPRINT,
    SPIDER_MAN_NO_MORE_REPRINT,
    IMPOSTOR_SYNDROME_REPRINT,
    THE_CLONE_SAGA_REPRINT,
    LADY_OCTOPUS_INSPIRED_INVENTOR_REPRINT,
    DOC_OCK_S_HENCHMEN_REPRINT,
    MADAME_WEB_CLAIRVOYANT_REPRINT,
    SCHOOL_DAZE_REPRINT,
    DOC_OCK_SINISTER_SCIENTIST_REPRINT,
    MYSTERIO_S_PHANTASM_REPRINT,
    OSCORP_RESEARCH_TEAM_REPRINT,
    ROBOTICS_MASTERY_REPRINT,
    SECRET_IDENTITY_REPRINT,
    WHOOSH_REPRINT,
    HIDE_ON_THE_CEILING_REPRINT,
    UNSTABLE_EXPERIMENT_REPRINT,
    CHAMELEON_MASTER_OF_DISGUISE_REPRINT,
    FLYING_OCTOBOT_REPRINT,
    MYSTERIO_MASTER_OF_ILLUSION_REPRINT,
    ALIEN_SYMBIOSIS_REPRINT,
    VENOMIZED_CAT_REPRINT,
    INNER_DEMONS_GANGSTERS_REPRINT,
    RISKY_RESEARCH_REPRINT,
    COMMON_CROOK_REPRINT,
    VENOM_S_HUNGER_REPRINT,
    PARKER_LUCK_REPRINT,
    GWENOM_REMORSELESS_REPRINT,
    BEHOLD_THE_SINISTER_SIX_REPRINT,
    THE_DEATH_OF_GWEN_STACY_REPRINT,
    SPIDER_MAN_NOIR_REPRINT,
    MORLUN_DEVOURER_OF_SPIDERS_REPRINT,
    MERCILESS_ENFORCERS_REPRINT,
    SANDMAN_S_QUICKSAND_REPRINT,
    TOMBSTONE_CAREER_CRIMINAL_REPRINT,
    PRISON_BREAK_REPRINT,
    THE_SPOT_S_PORTAL_REPRINT,
    AGENT_VENOM_REPRINT,
    SCORPION_S_STING_REPRINT,
    SCORPION_SEETHING_STRIKER_REPRINT,
    THE_SOUL_STONE_REPRINT,
    VENOM_EVIL_UNLEASHED_REPRINT,
    EDDIE_BROCK_REPRINT,
    VILLAINOUS_WRATH_REPRINT,
    SWARM_BEING_OF_BEES_REPRINT,
    BLACK_CAT_CUNNING_THIEF_REPRINT,
    SPIDER_VERSE_REPRINT,
    ELECTRO_ASSAULTING_BATTERY_REPRINT,
    HOBGOBLIN_MANTLED_MARAUDER_REPRINT,
    ELECTRO_S_BOLT_REPRINT,
    HEROES_HANGOUT_REPRINT,
    TAXI_DRIVER_REPRINT,
    ANGRY_RABBLE_REPRINT,
    MOLTEN_MAN_INFERNO_INCARNATE_REPRINT,
    SHOCKER_UNSHAKABLE_REPRINT,
    STEGRON_THE_DINOSAUR_MAN_REPRINT,
    SPIDER_PUNK_REPRINT,
    RAGING_GOBLINOIDS_REPRINT,
    MAXIMUM_CARNAGE_REPRINT,
    J_JONAH_JAMESON_REPRINT,
    GWEN_STACY_REPRINT,
    SHADOW_OF_THE_GOBLIN_REPRINT,
    SUPERIOR_FOES_OF_SPIDER_MAN_REPRINT,
    SPINNERET_AND_SPIDERLING_REPRINT,
    ROMANTIC_RENDEZVOUS_REPRINT,
    SPIDER_ISLANDERS_REPRINT,
    SHOCK_REPRINT,
    MASKED_MEOWER_REPRINT,
    SPIDER_GWEN_FREE_SPIRIT_REPRINT,
    WISECRACK_REPRINT,
    PICTURES_OF_SPIDER_MAN_REPRINT,
    SPIDER_REX_DARING_DINO_REPRINT,
    SPIDERS_MAN_HEROIC_HORDE_REPRINT,
    MILES_MORALES_REPRINT,
    KRAVEN_S_CATS_REPRINT,
    GUY_IN_THE_CHAIR_REPRINT,
    RADIOACTIVE_SPIDER_REPRINT,
    PROFESSIONAL_WRESTLER_REPRINT,
    GROW_EXTRA_ARMS_REPRINT,
    LIZARD_CONNORS_S_CURSE_REPRINT,
    LURKING_LIZARDS_REPRINT,
    EZEKIEL_SIMS_SPIDER_TOTEM_REPRINT,
    KAPOW_REPRINT,
    WALL_CRAWL_REPRINT,
    SANDMAN_SHIFTING_SCOUNDREL_REPRINT,
    SCOUT_THE_CITY_REPRINT,
    DAMAGE_CONTROL_CREW_REPRINT,
    STRENGTH_OF_WILL_REPRINT,
    SUPPORTIVE_PARENTS_REPRINT,
    SUPPORTIVE_PARENTS_ALTERNATE_1,
    SPIDER_HAM_PETER_PORKER_REPRINT,
    TERRIFIC_TEAM_UP_REPRINT,
    WEB_OF_LIFE_AND_DESTINY_REPRINT,
    KRAVEN_S_LAST_HUNT_REPRINT,
    SPIDER_MAN_BROOKLYN_VISIONARY_REPRINT,
    SILK_WEB_WEAVER_REPRINT,
    SPIDER_MAN_INDIA_REPRINT,
    BIORGANIC_CARAPACE_REPRINT,
    SCARLET_SPIDER_BEN_REILLY_REPRINT,
    KRAVEN_THE_HUNTER_REPRINT,
    PROWLER_CLAWED_THIEF_REPRINT,
    CHEERING_CROWD_REPRINT,
    SUN_SPIDER_NIMBLE_WEBBER_REPRINT,
    SHRIEK_TREBLEMAKER_REPRINT,
    MARY_JANE_WATSON_REPRINT,
    CARNAGE_CRIMSON_CHAOS_REPRINT,
    WEB_WARRIORS_REPRINT,
    KRAVEN_PROUD_PREDATOR_REPRINT,
    JACKAL_GENIUS_GENETICIST_REPRINT,
    GALLANT_CITIZEN_REPRINT,
    ARANA_HEART_OF_THE_SPIDER_REPRINT,
    SPIDER_GIRL_LEGACY_HERO_REPRINT,
    SUPERIOR_SPIDER_MAN_REPRINT,
    SCARLET_SPIDER_KAINE_REPRINT,
    PUMPKIN_BOMBARDMENT_REPRINT,
    SP_DR_PILOTED_BY_PENI_REPRINT,
    SPIDER_MANIFESTATION_REPRINT,
    SPIDER_WOMAN_STUNNING_SAVIOR_REPRINT,
    WRAITH_VICIOUS_VIGILANTE_REPRINT,
    MOB_LOOKOUT_REPRINT,
    DOCTOR_OCTOPUS_MASTER_PLANNER_REPRINT,
    GREEN_GOBLIN_REVENANT_REPRINT,
    THE_SPOT_LIVING_PORTAL_REPRINT,
    RHINOS_RAMPAGE_REPRINT,
    VULTURE_SCHEMING_SCAVENGER_REPRINT,
    ULTIMATE_GREEN_GOBLIN_REPRINT,
    COSMIC_SPIDER_MAN_REPRINT,
    SYMBIOTE_SPIDER_MAN_REPRINT,
    MORBIUS_THE_LIVING_VAMPIRE_REPRINT,
    SPIDER_MAN_2099_REPRINT,
    MISTER_NEGATIVE_REPRINT,
    SKYWARD_SPIDER_REPRINT,
    RHINO_BARRELING_BRUTE_REPRINT,
    SPIDER_SUIT_REPRINT,
    PASSENGER_FERRY_REPRINT,
    MECHANICAL_MOBSTER_REPRINT,
    EERIE_GRAVESTONE_REPRINT,
    LIVING_BRAIN_MECHANICAL_MARVEL_REPRINT,
    IRON_SPIDER_STARK_UPGRADE_REPRINT,
    DOC_OCK_S_TENTACLES_REPRINT,
    SPIDER_SLAYER_HATRED_HONED_REPRINT,
    NEWS_HELICOPTER_REPRINT,
    BAGEL_AND_SCHMEAR_REPRINT,
    PETER_PARKER_S_CAMERA_REPRINT,
    INTERDIMENSIONAL_WEB_WATCH_REPRINT,
    SPIDER_MOBILE_REPRINT,
    SPIDER_BOT_REPRINT,
    ROCKET_POWERED_GOBLIN_GLIDER_REPRINT,
    STEEL_WRECKING_BALL_REPRINT,
    HOT_DOG_CART_REPRINT,
    SUBWAY_TRAIN_REPRINT,
    OSCORP_INDUSTRIES_REPRINT,
    DAILY_BUGLE_BUILDING_REPRINT,
    MULTIVERSAL_PASSAGE_REPRINT,
    OMINOUS_ASYLUM_REPRINT,
    SAVAGE_MANSION_REPRINT,
    SINISTER_HIDEOUT_REPRINT,
    SUBURBAN_SANCTUARY_REPRINT,
    UNIVERSITY_CAMPUS_REPRINT,
    URBAN_RETREAT_REPRINT,
    VIBRANT_CITYSCAPE_REPRINT,
];
