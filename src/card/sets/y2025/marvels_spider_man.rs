//! Marvel's Spider-Man card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectPaymentDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::stronghold as catalog_sth;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SPM",
    slug: "marvels-spider-man",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// SPM 1 — Anti-Venom, Horrifying Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANTI_VENOM_HORRIFYING_HEALER: CardRecord = CardRecord::new(
    "Anti-Venom, Horrifying Healer",
    "560384fe-7be0-4b93-a515-2fe687ab2492",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// SPM 2 — Arachne, Psionic Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARACHNE_PSIONIC_WEAVER: CardRecord = CardRecord::new(
    "Arachne, Psionic Weaver",
    "7c1f871a-bd85-402e-b474-1deb64c18a52",
    "Steve Argyle",
    crate::card::CardRules::unsupported(),
);

// SPM 3 — Aunt May
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUNT_MAY: CardRecord = CardRecord::new(
    "Aunt May",
    "ad96343b-baac-428c-8270-fcffbbbe9fb8",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// SPM 4 — City Pigeon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITY_PIGEON: CardRecord = CardRecord::new(
    "City Pigeon",
    "56d67fb4-5b23-432c-9ffb-39545035c117",
    "David Szabo",
    crate::card::CardRules::unsupported(),
);

// SPM 5 — Costume Closet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COSTUME_CLOSET: CardRecord = CardRecord::new(
    "Costume Closet",
    "cc641f4a-ddbe-4f7d-bb55-eabf11f8b7fb",
    "Bastien Grivet",
    crate::card::CardRules::unsupported(),
);

// SPM 6 — Daily Bugle Reporters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAILY_BUGLE_REPORTERS: CardRecord = CardRecord::new(
    "Daily Bugle Reporters",
    "530dbeb0-b0cd-473e-a43e-7b23c88650a3",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// SPM 7 — Flash Thompson, Spider-Fan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLASH_THOMPSON_SPIDER_FAN: CardRecord = CardRecord::new(
    "Flash Thompson, Spider-Fan",
    "44cf372b-f668-45e9-981e-4533295dcc74",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// SPM 8 — Friendly Neighborhood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRIENDLY_NEIGHBORHOOD: CardRecord = CardRecord::new(
    "Friendly Neighborhood",
    "17a18e2f-221f-4fc2-8dab-25bf12fb8756",
    "Pablo Mendoza",
    crate::card::CardRules::unsupported(),
);

// SPM 9 — Origin of Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIGIN_OF_SPIDER_MAN: CardRecord = CardRecord::new(
    "Origin of Spider-Man",
    "a10a7da7-d9cb-495a-9c9f-205d355c390d",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// SPM 10 — Peter Parker // Amazing Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PETER_PARKER: CardRecord = CardRecord::new(
    "Peter Parker // Amazing Spider-Man",
    "3ce33422-5dba-4a42-8375-dd8ccc692a7b",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// SPM 11 — Rent Is Due
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENT_IS_DUE: CardRecord = CardRecord::new(
    "Rent Is Due",
    "b3f8d221-081f-49f5-a501-07e5eb21a840",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// SPM 12 — Selfless Police Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SELFLESS_POLICE_CAPTAIN: CardRecord = CardRecord::new(
    "Selfless Police Captain",
    "fb0fd7bd-10d0-4d29-af88-387d1e07f3b7",
    "Aniekan Udofia",
    crate::card::CardRules::unsupported(),
);

// SPM 13 — Silver Sable, Mercenary Leader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_SABLE_MERCENARY_LEADER: CardRecord = CardRecord::new(
    "Silver Sable, Mercenary Leader",
    "cf0d4116-acee-4d9a-985c-396d10e03838",
    "JB Casacop",
    crate::card::CardRules::unsupported(),
);

// SPM 14 — Spectacular Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTACULAR_SPIDER_MAN: CardRecord = CardRecord::new(
    "Spectacular Spider-Man",
    "32bff506-efc0-42ef-8286-ed939bf853d7",
    "Roberta Ingranata",
    crate::card::CardRules::unsupported(),
);

// SPM 15 — Spectacular Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTACULAR_TACTICS: CardRecord = CardRecord::new(
    "Spectacular Tactics",
    "836b4246-f1f2-4495-8664-650dda70ed4f",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// SPM 16 — Spider-Man, Web-Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_WEB_SLINGER: CardRecord = CardRecord::new(
    "Spider-Man, Web-Slinger",
    "897418bc-df8c-4c97-b6bf-7c9133a8a577",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// SPM 17 — Spider-UK
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_UK: CardRecord = CardRecord::new(
    "Spider-UK",
    "6beb4548-1fab-4b9e-bf24-f7b9aadecc87",
    "Allen Morris",
    crate::card::CardRules::unsupported(),
);

// SPM 18 — Starling, Aerial Ally
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLING_AERIAL_ALLY: CardRecord = CardRecord::new(
    "Starling, Aerial Ally",
    "babbf53d-3e10-4110-8725-91f766c8cdad",
    "Aniekan Udofia",
    crate::card::CardRules::unsupported(),
);

// SPM 19 — Sudden Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUDDEN_STRIKE: CardRecord = CardRecord::new(
    "Sudden Strike",
    "6eea2718-93d2-4d83-9b5d-eb943a0f1d11",
    "Le Vuong",
    crate::card::CardRules::unsupported(),
);

// SPM 20 — Thwip!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THWIP: CardRecord = CardRecord::new(
    "Thwip!",
    "b2dac88e-1204-4640-94ce-e1aff434ea06",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// SPM 21 — Web Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEB_UP: CardRecord = CardRecord::new(
    "Web Up",
    "1ab7c1e6-54af-4002-8a81-23a1ccafa3ff",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// SPM 22 — Web-Shooters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEB_SHOOTERS: CardRecord = CardRecord::new(
    "Web-Shooters",
    "a0ca1108-6d99-4dc2-96ab-7728d65b0c06",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// SPM 23 — Wild Pack Squad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_PACK_SQUAD: CardRecord = CardRecord::new(
    "Wild Pack Squad",
    "7b0eda7c-e44d-4d9b-9042-4a1eb8c4ed4a",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// SPM 24 — With Great Power . . .
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITH_GREAT_POWER: CardRecord = CardRecord::new(
    "With Great Power . . .",
    "f717c096-e161-426e-a8d7-c93b117e16b9",
    "E. M. Gist",
    crate::card::CardRules::unsupported(),
);

// SPM 25 — Amazing Acrobatics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMAZING_ACROBATICS: CardRecord = CardRecord::new(
    "Amazing Acrobatics",
    "9a2f6d84-3d83-4f48-9906-11f681171930",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// SPM 26 — Beetle, Legacy Criminal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEETLE_LEGACY_CRIMINAL: CardRecord = CardRecord::new(
    "Beetle, Legacy Criminal",
    "a194f930-c99f-4915-8a62-e20ab2b4ad1f",
    "Carlos Dattoli",
    crate::card::CardRules::unsupported(),
);

// SPM 27 — Chameleon, Master of Disguise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAMELEON_MASTER_OF_DISGUISE: CardRecord = CardRecord::new(
    "Chameleon, Master of Disguise",
    "43892ce7-f63a-4294-922b-8f879f684033",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// SPM 28 — The Clone Saga
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_CLONE_SAGA: CardRecord = CardRecord::new(
    "The Clone Saga",
    "976432b3-bc17-4edb-86d6-00fd1baf9670",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// SPM 29 — Doc Ock, Sinister Scientist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOC_OCK_SINISTER_SCIENTIST: CardRecord = CardRecord::new(
    "Doc Ock, Sinister Scientist",
    "de16ad65-c8c3-48c0-9d13-5af91b4e6f01",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// SPM 30 — Doc Ock's Henchmen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOC_OCK_S_HENCHMEN: CardRecord = CardRecord::new(
    "Doc Ock's Henchmen",
    "a383c442-3f4a-4115-97e4-23f0eb88465b",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// SPM 31 — Flying Octobot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLYING_OCTOBOT: CardRecord = CardRecord::new(
    "Flying Octobot",
    "ebadcd4a-f58f-4328-a765-0ea8d8028417",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// SPM 32 — Hide on the Ceiling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDE_ON_THE_CEILING: CardRecord = CardRecord::new(
    "Hide on the Ceiling",
    "7977e448-01fa-4fa5-a275-0d6a1357b35c",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// SPM 33 — Hydro-Man, Fluid Felon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDRO_MAN_FLUID_FELON: CardRecord = CardRecord::new(
    "Hydro-Man, Fluid Felon",
    "e53115a4-8959-40fa-b763-931504a1c5a2",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// SPM 34 — Impostor Syndrome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPOSTOR_SYNDROME: CardRecord = CardRecord::new(
    "Impostor Syndrome",
    "08da9f92-0e25-4f39-aaa4-d8974af81a41",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// SPM 35 — Lady Octopus, Inspired Inventor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LADY_OCTOPUS_INSPIRED_INVENTOR: CardRecord = CardRecord::new(
    "Lady Octopus, Inspired Inventor",
    "8c5f360b-f9a0-46e0-9e8b-58e5b4b0389e",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// SPM 36 — Madame Web, Clairvoyant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADAME_WEB_CLAIRVOYANT: CardRecord = CardRecord::new(
    "Madame Web, Clairvoyant",
    "a16ca3fc-3cdf-4333-93e0-524afafe367b",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// SPM 37 — Mysterio, Master of Illusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTERIO_MASTER_OF_ILLUSION: CardRecord = CardRecord::new(
    "Mysterio, Master of Illusion",
    "facbd96f-c088-4377-b740-5e0fe99102bb",
    "Alexander Gering",
    crate::card::CardRules::unsupported(),
);

// SPM 38 — Mysterio's Phantasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTERIO_S_PHANTASM: CardRecord = CardRecord::new(
    "Mysterio's Phantasm",
    "79aa0a78-80a2-44be-8a79-92bcba9c040f",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// SPM 39 — Norman Osborn // Green Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NORMAN_OSBORN: CardRecord = CardRecord::new(
    "Norman Osborn // Green Goblin",
    "d5c53af9-7150-4e78-8771-2de7980aa307",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// SPM 40 — Oscorp Research Team
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OSCORP_RESEARCH_TEAM: CardRecord = CardRecord::new(
    "Oscorp Research Team",
    "a800ffb4-0c48-41eb-b221-cf1d855131d9",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// SPM 41 — Robotics Mastery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROBOTICS_MASTERY: CardRecord = CardRecord::new(
    "Robotics Mastery",
    "d0e92939-6d86-44b4-8a43-1963e97a2bb3",
    "Domenico Cava",
    crate::card::CardRules::unsupported(),
);

// SPM 42 — School Daze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCHOOL_DAZE: CardRecord = CardRecord::new(
    "School Daze",
    "e5b61b9d-31a2-4e09-9612-4980bf8de708",
    "Domenico Cava",
    crate::card::CardRules::unsupported(),
);

// SPM 43 — Secret Identity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECRET_IDENTITY: CardRecord = CardRecord::new(
    "Secret Identity",
    "37a31d84-e87b-406e-9249-fae1b5e23e72",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// SPM 44 — Spider-Byte, Web Warden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_BYTE_WEB_WARDEN: CardRecord = CardRecord::new(
    "Spider-Byte, Web Warden",
    "210ae606-12a4-453b-bfb4-73ca9c22b8b5",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// SPM 45 — Spider-Man No More
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_NO_MORE: CardRecord = CardRecord::new(
    "Spider-Man No More",
    "72dbab11-96ed-43db-8b59-ceca47c8cd22",
    "Aniekan Udofia",
    crate::card::CardRules::unsupported(),
);

// SPM 46 — Spider-Sense
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_SENSE: CardRecord = CardRecord::new(
    "Spider-Sense",
    "4499a25b-f4a5-4f2c-9ebd-bc68c7840b39",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// SPM 47 — Unstable Experiment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSTABLE_EXPERIMENT: CardRecord = CardRecord::new(
    "Unstable Experiment",
    "be9d5985-0a39-4ee3-80de-30d17d08f404",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// SPM 48 — Whoosh!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHOOSH: CardRecord = CardRecord::new(
    "Whoosh!",
    "ccc05deb-ad8d-4fae-a7a4-2b2a118fc696",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// SPM 49 — Agent Venom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_VENOM: CardRecord = CardRecord::new(
    "Agent Venom",
    "f5f80d82-d64c-466f-8874-9cfb00469f02",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// SPM 50 — Alien Symbiosis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALIEN_SYMBIOSIS: CardRecord = CardRecord::new(
    "Alien Symbiosis",
    "b898ccb7-758e-4f11-95e0-b412721d8bf9",
    "JB Casacop",
    crate::card::CardRules::unsupported(),
);

// SPM 51 — Behold the Sinister Six!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEHOLD_THE_SINISTER_SIX: CardRecord = CardRecord::new(
    "Behold the Sinister Six!",
    "1919bfec-1906-4178-ad32-d4589842e563",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// SPM 52 — Black Cat, Cunning Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_CAT_CUNNING_THIEF: CardRecord = CardRecord::new(
    "Black Cat, Cunning Thief",
    "0ed36ada-22c8-4e40-86c5-c116a0bee1c2",
    "Alessandra Pisano",
    crate::card::CardRules::unsupported(),
);

// SPM 53 — Common Crook
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMON_CROOK: CardRecord = CardRecord::new(
    "Common Crook",
    "6f5872df-e692-44aa-b18d-22447f5f274c",
    "Ben Harvey",
    crate::card::CardRules::unsupported(),
);

// SPM 54 — The Death of Gwen Stacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_DEATH_OF_GWEN_STACY: CardRecord = CardRecord::new(
    "The Death of Gwen Stacy",
    "690f1f31-f8c5-4336-9ec9-72ff761e3adc",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// SPM 55 — Eddie Brock // Venom, Lethal Protector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EDDIE_BROCK: CardRecord = CardRecord::new(
    "Eddie Brock // Venom, Lethal Protector",
    "f3455651-e643-445e-9489-51e4e24fca4c",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// SPM 56 — Gwenom, Remorseless
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GWENOM_REMORSELESS: CardRecord = CardRecord::new(
    "Gwenom, Remorseless",
    "46b6cc5d-7a37-4e8b-a1a5-9a573056610c",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// SPM 57 — Inner Demons Gangsters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INNER_DEMONS_GANGSTERS: CardRecord = CardRecord::new(
    "Inner Demons Gangsters",
    "f0252819-4eda-457d-9688-b08b83b1edc9",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// SPM 58 — Merciless Enforcers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCILESS_ENFORCERS: CardRecord = CardRecord::new(
    "Merciless Enforcers",
    "fba9c76c-1432-4554-88bd-3f5e8709a963",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// SPM 59 — Morlun, Devourer of Spiders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORLUN_DEVOURER_OF_SPIDERS: CardRecord = CardRecord::new(
    "Morlun, Devourer of Spiders",
    "1beb2eb9-90b5-43ba-8b04-cfce7dcb744b",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// SPM 60 — Parker Luck
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARKER_LUCK: CardRecord = CardRecord::new(
    "Parker Luck",
    "e375bcf0-7fcb-4fe4-a7e8-a4cbf9b23e3c",
    "Raoul Vitale",
    crate::card::CardRules::unsupported(),
);

// SPM 61 — Prison Break
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISON_BREAK: CardRecord = CardRecord::new(
    "Prison Break",
    "6c45a5df-048e-4b73-89c6-5cdaa330319e",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// SPM 62 — Risky Research
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISKY_RESEARCH: CardRecord = CardRecord::new(
    "Risky Research",
    "1f8aa705-6177-42e9-95cb-e7f880c186e3",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// SPM 63 — Sandman's Quicksand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDMAN_S_QUICKSAND: CardRecord = CardRecord::new(
    "Sandman's Quicksand",
    "b7795e17-6717-464c-9ae3-20da52ba005a",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// SPM 64 — Scorpion, Seething Striker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORPION_SEETHING_STRIKER: CardRecord = CardRecord::new(
    "Scorpion, Seething Striker",
    "cf407e08-b27f-42ba-b824-75846a80e238",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// SPM 65 — Scorpion's Sting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORPION_S_STING: CardRecord = CardRecord::new(
    "Scorpion's Sting",
    "0fb03437-32cf-4c97-bf91-ea8b2ad3f964",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// SPM 66 — The Soul Stone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SOUL_STONE: CardRecord = CardRecord::new(
    "The Soul Stone",
    "1982f910-a9bd-4e94-a187-84381b22aacc",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// SPM 67 — Spider-Man Noir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_NOIR: CardRecord = CardRecord::new(
    "Spider-Man Noir",
    "bc64366c-2691-48cd-bb4b-a4b088c6f16b",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// SPM 68 — The Spot's Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SPOT_S_PORTAL: CardRecord = CardRecord::new(
    "The Spot's Portal",
    "67a8bf52-7562-4cdd-b970-106717a0aad6",
    "Carlos Dattoli",
    crate::card::CardRules::unsupported(),
);

// SPM 69 — Swarm, Being of Bees
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWARM_BEING_OF_BEES: CardRecord = CardRecord::new(
    "Swarm, Being of Bees",
    "cb83d54e-6641-4929-99ad-c0ba5b610902",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// SPM 70 — Tombstone, Career Criminal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOMBSTONE_CAREER_CRIMINAL: CardRecord = CardRecord::new(
    "Tombstone, Career Criminal",
    "313189ef-fe6e-4511-9386-920a88a49a88",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// SPM 71 — Venom, Evil Unleashed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOM_EVIL_UNLEASHED: CardRecord = CardRecord::new(
    "Venom, Evil Unleashed",
    "ab3d51a4-40f0-4606-b5f9-2686c12fd54b",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// SPM 72 — Venomized Cat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMIZED_CAT: CardRecord = CardRecord::new(
    "Venomized Cat",
    "6330f3e9-e031-4d55-b5d8-536c16bba063",
    "Jessica Fong",
    crate::card::CardRules::unsupported(),
);

// SPM 73 — Venom's Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOM_S_HUNGER: CardRecord = CardRecord::new(
    "Venom's Hunger",
    "01d276cd-e4ad-488f-8447-004aefad1ebb",
    "Dave DeVries",
    crate::card::CardRules::unsupported(),
);

// SPM 74 — Villainous Wrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILLAINOUS_WRATH: CardRecord = CardRecord::new(
    "Villainous Wrath",
    "d78e36fd-5817-4c4a-8880-dabe6dd4ba81",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// SPM 75 — Angry Rabble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGRY_RABBLE: CardRecord = CardRecord::new(
    "Angry Rabble",
    "938730fa-496f-4871-80ec-3e9843ecb219",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// SPM 76 — Electro, Assaulting Battery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELECTRO_ASSAULTING_BATTERY: CardRecord = CardRecord::new(
    "Electro, Assaulting Battery",
    "d672cfad-e656-47f8-bf93-64f262aff33e",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// SPM 77 — Electro's Bolt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELECTRO_S_BOLT: CardRecord = CardRecord::new(
    "Electro's Bolt",
    "25fe063f-35e4-4fca-9889-06834a8ef9b9",
    "JB Casacop",
    crate::card::CardRules::unsupported(),
);

// SPM 78 — Gwen Stacy // Ghost-Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GWEN_STACY: CardRecord = CardRecord::new(
    "Gwen Stacy // Ghost-Spider",
    "b0f1597f-1dc7-465e-8fcb-0afe61bcca46",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// SPM 79 — Heroes' Hangout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEROES_HANGOUT: CardRecord = CardRecord::new(
    "Heroes' Hangout",
    "4148d7e8-6371-468c-858b-35254995409a",
    "Smirtouille",
    crate::card::CardRules::unsupported(),
);

// SPM 80 — Hobgoblin, Mantled Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOBGOBLIN_MANTLED_MARAUDER: CardRecord = CardRecord::new(
    "Hobgoblin, Mantled Marauder",
    "50716fe3-7a19-431e-8758-984fc48d714e",
    "Dave DeVries",
    crate::card::CardRules::unsupported(),
);

// SPM 81 — J. Jonah Jameson
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static J_JONAH_JAMESON: CardRecord = CardRecord::new(
    "J. Jonah Jameson",
    "9ee905d6-b647-4eb1-a8d9-89add9bafc31",
    "Paolo Rivera",
    crate::card::CardRules::unsupported(),
);

// SPM 82 — Masked Meower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASKED_MEOWER: CardRecord = CardRecord::new(
    "Masked Meower",
    "6aa0dc1f-6c83-4ac9-b4f2-428e0e0bbf88",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// SPM 83 — Maximum Carnage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAXIMUM_CARNAGE: CardRecord = CardRecord::new(
    "Maximum Carnage",
    "7d72d867-6ed2-4900-a8ae-9d86f581ce32",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// SPM 84 — Molten Man, Inferno Incarnate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_MAN_INFERNO_INCARNATE: CardRecord = CardRecord::new(
    "Molten Man, Inferno Incarnate",
    "f469d621-25d0-4d8e-909f-47dac0b9c5b0",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// SPM 85 — Raging Goblinoids
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGING_GOBLINOIDS: CardRecord = CardRecord::new(
    "Raging Goblinoids",
    "8519598f-ab7f-49b0-90cc-c0b6422ebdf8",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// SPM 86 — Romantic Rendezvous
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROMANTIC_RENDEZVOUS: CardRecord = CardRecord::new(
    "Romantic Rendezvous",
    "38120361-153f-414e-8a45-f86bb2e35a17",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// SPM 87 — Shadow of the Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOW_OF_THE_GOBLIN: CardRecord = CardRecord::new(
    "Shadow of the Goblin",
    "854b6898-c480-435b-8952-a077c7977cec",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// SPM 88 — Shock (reprint)
const SHOCK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sth::SHOCK,
    "760b41a1-c087-4b11-b8a0-fb01d8a4c0c6",
    "Piotr Dura",
);

// SPM 89 — Shocker, Unshakable
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOCKER_UNSHAKABLE: CardRecord = CardRecord::new(
    "Shocker, Unshakable",
    "8b2c0d9a-364a-4823-aa4c-fe473d4463f0",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// SPM 90 — Spider-Gwen, Free Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_GWEN_FREE_SPIRIT: CardRecord = CardRecord::new(
    "Spider-Gwen, Free Spirit",
    "3bc04fa7-6265-4549-91f6-eebdcd67398a",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// SPM 91 — Spider-Islanders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_ISLANDERS: CardRecord = CardRecord::new(
    "Spider-Islanders",
    "c9132e45-4ddb-4565-ac45-86f1ecc6230d",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// SPM 92 — Spider-Punk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_PUNK: CardRecord = CardRecord::new(
    "Spider-Punk",
    "0bd41879-fcd4-4211-9b98-47e7cdba5399",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// SPM 93 — Spider-Verse
// Audit: unsupported — Needs a once-each-turn trigger for spells cast outside hand whose optional stack copy can grant haste specifically when it copies a permanent spell.
pub(in crate::card::sets) static SPIDER_VERSE: CardRecord = CardRecord::new(
    "Spider-Verse",
    "f8779eb2-1210-430d-8d42-3077053441ee",
    "Alexander Gering",
    CardRules::unsupported(),
);

// SPM 94 — Spinneret and Spiderling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINNERET_AND_SPIDERLING: CardRecord = CardRecord::new(
    "Spinneret and Spiderling",
    "a27834b7-e763-48ac-845e-ed49f2fa6c6d",
    "Le Vuong",
    crate::card::CardRules::unsupported(),
);

// SPM 95 — Stegron the Dinosaur Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEGRON_THE_DINOSAUR_MAN: CardRecord = CardRecord::new(
    "Stegron the Dinosaur Man",
    "485ceacb-fa76-4517-8466-c3c6bf6bcd6e",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// SPM 96 — Superior Foes of Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPERIOR_FOES_OF_SPIDER_MAN: CardRecord = CardRecord::new(
    "Superior Foes of Spider-Man",
    "28e7bf86-5791-4412-8184-fa63fb292be4",
    "Ben Harvey",
    crate::card::CardRules::unsupported(),
);

// SPM 97 — Taxi Driver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAXI_DRIVER: CardRecord = CardRecord::new(
    "Taxi Driver",
    "a80d3ed9-5e81-41b7-bb74-ab86cba841c8",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// SPM 98 — Wisecrack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WISECRACK: CardRecord = CardRecord::new(
    "Wisecrack",
    "8f452dac-bf22-4010-8a10-3c1cfa7d4df6",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// SPM 99 — Damage Control Crew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAMAGE_CONTROL_CREW: CardRecord = CardRecord::new(
    "Damage Control Crew",
    "ad2cab87-691d-44fe-ab2f-33760b1feb0f",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// SPM 100 — Ezekiel Sims, Spider-Totem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EZEKIEL_SIMS_SPIDER_TOTEM: CardRecord = CardRecord::new(
    "Ezekiel Sims, Spider-Totem",
    "bb7c3ae2-6b01-4472-8bd1-9a7456401ddc",
    "Wei Guan",
    crate::card::CardRules::unsupported(),
);

// SPM 101 — Grow Extra Arms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROW_EXTRA_ARMS: CardRecord = CardRecord::new(
    "Grow Extra Arms",
    "63fab399-00db-4398-922e-c3ca3356731a",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// SPM 102 — Guy in the Chair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUY_IN_THE_CHAIR: CardRecord = CardRecord::new(
    "Guy in the Chair",
    "65e97c06-55a6-4841-be0f-055c015df90a",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// SPM 103 — Kapow!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAPOW: CardRecord = CardRecord::new(
    "Kapow!",
    "cec575f6-43c9-41c6-a996-bb806bf82185",
    "Jessica Fong",
    crate::card::CardRules::unsupported(),
);

// SPM 104 — Kraven's Cats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAVEN_S_CATS: CardRecord = CardRecord::new(
    "Kraven's Cats",
    "86c415d8-1d2d-4339-955b-0f2aebeb3c95",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// SPM 105 — Kraven's Last Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAVEN_S_LAST_HUNT: CardRecord = CardRecord::new(
    "Kraven's Last Hunt",
    "d0c18ffe-a2b9-40df-a6b4-a9381e6dc467",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// SPM 106 — Lizard, Connors's Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIZARD_CONNORS_S_CURSE: CardRecord = CardRecord::new(
    "Lizard, Connors's Curse",
    "6add5d2a-950e-4bee-9850-e68f5f6d6142",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// SPM 107 — Lurking Lizards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_LIZARDS: CardRecord = CardRecord::new(
    "Lurking Lizards",
    "58b5b49c-ddd6-4d1b-9b61-6e02d8fc55ad",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// SPM 108 — Miles Morales // Ultimate Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MILES_MORALES: CardRecord = CardRecord::new(
    "Miles Morales // Ultimate Spider-Man",
    "9f8b4d9b-208a-4673-a617-5e3edd069c33",
    "L.A. Draws",
    crate::card::CardRules::unsupported(),
);

// SPM 109 — Pictures of Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PICTURES_OF_SPIDER_MAN: CardRecord = CardRecord::new(
    "Pictures of Spider-Man",
    "e1ec41d4-0180-42f7-9c54-f3c39b4ffb8d",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// SPM 110 — Professional Wrestler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROFESSIONAL_WRESTLER: CardRecord = CardRecord::new(
    "Professional Wrestler",
    "8a5381e7-ddda-47e7-886d-812250ffb745",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// SPM 111 — Radioactive Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIOACTIVE_SPIDER: CardRecord = CardRecord::new(
    "Radioactive Spider",
    "f2d267f5-7f12-45f8-8fcb-e0ba3fbdeddc",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// SPM 112 — Sandman, Shifting Scoundrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDMAN_SHIFTING_SCOUNDREL: CardRecord = CardRecord::new(
    "Sandman, Shifting Scoundrel",
    "609ac18c-ec58-4fa7-bbee-3912a69d0ec6",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// SPM 113 — Scout the City
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUT_THE_CITY: CardRecord = CardRecord::new(
    "Scout the City",
    "90b9504d-d23d-402f-8b16-1964ebd8f6b9",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// SPM 114 — Spider-Ham, Peter Porker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_HAM_PETER_PORKER: CardRecord = CardRecord::new(
    "Spider-Ham, Peter Porker",
    "41f18f42-b86b-4a12-9f0d-76b761571195",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// SPM 115 — Spider-Man, Brooklyn Visionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_BROOKLYN_VISIONARY: CardRecord = CardRecord::new(
    "Spider-Man, Brooklyn Visionary",
    "e19929bc-cbe1-4970-952d-8e9d0193ddce",
    "Aniekan Udofia",
    crate::card::CardRules::unsupported(),
);

// SPM 116 — Spider-Rex, Daring Dino
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_REX_DARING_DINO: CardRecord = CardRecord::new(
    "Spider-Rex, Daring Dino",
    "5b6e0bea-f126-4adb-8808-901950a77c7b",
    "Narendra Bintara Adi",
    crate::card::CardRules::unsupported(),
);

// SPM 117 — Spiders-Man, Heroic Horde
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDERS_MAN_HEROIC_HORDE: CardRecord = CardRecord::new(
    "Spiders-Man, Heroic Horde",
    "1183262e-1f02-46b3-8cfa-fe30e0016c11",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// SPM 118 — Strength of Will
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRENGTH_OF_WILL: CardRecord = CardRecord::new(
    "Strength of Will",
    "68f985c7-7765-46c3-ad31-edae3abb9fbf",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// SPM 119 — Supportive Parents
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPPORTIVE_PARENTS: CardRecord = CardRecord::new(
    "Supportive Parents",
    "d3fe8a5b-4166-46cc-b910-71cd1a19ae1b",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// SPM 120 — Terrific Team-Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRIFIC_TEAM_UP: CardRecord = CardRecord::new(
    "Terrific Team-Up",
    "f3c587b0-66b9-46bf-90ee-a6163c006c9e",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// SPM 121 — Wall Crawl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_CRAWL: CardRecord = CardRecord::new(
    "Wall Crawl",
    "97a2a1ab-57ec-4210-9412-765ae4f02db0",
    "Alexander Gering",
    crate::card::CardRules::unsupported(),
);

// SPM 122 — Web of Life and Destiny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEB_OF_LIFE_AND_DESTINY: CardRecord = CardRecord::new(
    "Web of Life and Destiny",
    "3b3c609e-f7c9-4fe5-84d9-4f4c76020a4b",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// SPM 123 — Araña, Heart of the Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARANA_HEART_OF_THE_SPIDER: CardRecord = CardRecord::new(
    "Araña, Heart of the Spider",
    "b02bfa0e-f761-45e1-b35c-f44ff7c5d0e8",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// SPM 124 — Biorganic Carapace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIORGANIC_CARAPACE: CardRecord = CardRecord::new(
    "Biorganic Carapace",
    "9658fdab-9702-4e13-bc53-01a25a2ed41a",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// SPM 125 — Carnage, Crimson Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARNAGE_CRIMSON_CHAOS: CardRecord = CardRecord::new(
    "Carnage, Crimson Chaos",
    "930befba-6068-493e-baa2-e9371cd99e93",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// SPM 126 — Cheering Crowd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHEERING_CROWD: CardRecord = CardRecord::new(
    "Cheering Crowd",
    "5fbce72f-e9a1-4d9f-b9b3-24dbafeef841",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// SPM 127 — Cosmic Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COSMIC_SPIDER_MAN: CardRecord = CardRecord::new(
    "Cosmic Spider-Man",
    "f82f4013-7308-4917-9042-19a5909f2134",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// SPM 128 — Doctor Octopus, Master Planner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOCTOR_OCTOPUS_MASTER_PLANNER: CardRecord = CardRecord::new(
    "Doctor Octopus, Master Planner",
    "76e1d361-18a1-4dec-a203-f83bf0014e02",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// SPM 129 — Gallant Citizen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLANT_CITIZEN: CardRecord = CardRecord::new(
    "Gallant Citizen",
    "43790471-6ec8-4c1d-b6d3-74c6cdf8ce43",
    "Allen Morris",
    crate::card::CardRules::unsupported(),
);

// SPM 130 — Green Goblin, Revenant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEN_GOBLIN_REVENANT: CardRecord = CardRecord::new(
    "Green Goblin, Revenant",
    "218ef931-46f5-4a4d-9f26-898a1ff8f70f",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// SPM 131 — Jackal, Genius Geneticist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JACKAL_GENIUS_GENETICIST: CardRecord = CardRecord::new(
    "Jackal, Genius Geneticist",
    "c0ab07d6-b7c3-4129-9aef-cfdcfabec4b2",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// SPM 132 — Kraven, Proud Predator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAVEN_PROUD_PREDATOR: CardRecord = CardRecord::new(
    "Kraven, Proud Predator",
    "af7c63e1-eccc-40a2-ba14-ce0d2a6123fc",
    "Alexander Mokhov",
    crate::card::CardRules::unsupported(),
);

// SPM 133 — Kraven the Hunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAVEN_THE_HUNTER: CardRecord = CardRecord::new(
    "Kraven the Hunter",
    "afdab464-3674-449b-be01-1cbd21fced23",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// SPM 134 — Mary Jane Watson
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARY_JANE_WATSON: CardRecord = CardRecord::new(
    "Mary Jane Watson",
    "178345f7-8ccd-4e47-80f4-5bd31bab6655",
    "Steve Argyle",
    crate::card::CardRules::unsupported(),
);

// SPM 135 — Mister Negative
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTER_NEGATIVE: CardRecord = CardRecord::new(
    "Mister Negative",
    "2c9cb13d-55ff-4e26-aa49-755f8bcebc11",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// SPM 136 — Mob Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOB_LOOKOUT: CardRecord = CardRecord::new(
    "Mob Lookout",
    "e0e3b660-d391-454b-ba57-bff4ddcf27b7",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// SPM 137 — Morbius the Living Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORBIUS_THE_LIVING_VAMPIRE: CardRecord = CardRecord::new(
    "Morbius the Living Vampire",
    "b978d0e2-f5a7-4ade-befc-11b406e84477",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// SPM 138 — Prowler, Clawed Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROWLER_CLAWED_THIEF: CardRecord = CardRecord::new(
    "Prowler, Clawed Thief",
    "bd31953a-7259-44e3-a94f-013bda68006d",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// SPM 139 — Pumpkin Bombardment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUMPKIN_BOMBARDMENT: CardRecord = CardRecord::new(
    "Pumpkin Bombardment",
    "a268ad73-9a1f-47d9-9a85-a4669a769c3d",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// SPM 140 — Rhino, Barreling Brute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHINO_BARRELING_BRUTE: CardRecord = CardRecord::new(
    "Rhino, Barreling Brute",
    "4b8ac400-1e98-49fc-94be-00386d15f2ae",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// SPM 141 — Rhino's Rampage
// Audit: unsupported — Needs a reflexive excess-damage trigger that chooses its artifact target after the fight.
pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    "Rhino's Rampage",
    "f668817c-1cab-44c5-b6a8-95113e480d5e",
    "Nino Is",
    CardRules::unsupported(),
);

// SPM 142 — Scarlet Spider, Ben Reilly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARLET_SPIDER_BEN_REILLY: CardRecord = CardRecord::new(
    "Scarlet Spider, Ben Reilly",
    "ee771581-f867-48d7-9ddb-897a1ffcdf0a",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// SPM 143 — Scarlet Spider, Kaine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARLET_SPIDER_KAINE: CardRecord = CardRecord::new(
    "Scarlet Spider, Kaine",
    "2cb00060-8cc5-42dc-bcbf-affd9e59f8fd",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// SPM 144 — Shriek, Treblemaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEK_TREBLEMAKER: CardRecord = CardRecord::new(
    "Shriek, Treblemaker",
    "01f1900e-b10f-47dd-8b3d-6913fa661186",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// SPM 145 — Silk, Web Weaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILK_WEB_WEAVER: CardRecord = CardRecord::new(
    "Silk, Web Weaver",
    "588dc8d9-6ce0-4bd7-afbd-84bb251fdcb1",
    "Carissa Susilo",
    crate::card::CardRules::unsupported(),
);

// SPM 146 — Skyward Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYWARD_SPIDER: CardRecord = CardRecord::new(
    "Skyward Spider",
    "f5cbb580-cd02-4c60-acb7-b7ed1f1fce59",
    "Bachzim",
    crate::card::CardRules::unsupported(),
);

// SPM 147 — SP//dr, Piloted by Peni
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SP_DR_PILOTED_BY_PENI: CardRecord = CardRecord::new(
    "SP//dr, Piloted by Peni",
    "c47c1d83-e76d-4939-9ed6-05a9e709dea1",
    "Toni Infante",
    crate::card::CardRules::unsupported(),
);

// SPM 148 — Spider Manifestation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MANIFESTATION: CardRecord = CardRecord::new(
    "Spider Manifestation",
    "99223677-b8a5-48f1-8009-e8475eada7db",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// SPM 149 — Spider-Girl, Legacy Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_GIRL_LEGACY_HERO: CardRecord = CardRecord::new(
    "Spider-Girl, Legacy Hero",
    "d1f3196a-fe48-446f-ab07-00c66b7816c8",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

// SPM 150 — Spider-Man 2099
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_2099: CardRecord = CardRecord::new(
    "Spider-Man 2099",
    "2a72c7e7-34f5-4cb0-9959-35516e398e49",
    "Toni Infante",
    crate::card::CardRules::unsupported(),
);

// SPM 151 — Spider-Man India
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_INDIA: CardRecord = CardRecord::new(
    "Spider-Man India",
    "65b8af30-559b-43a9-8526-62c28c378339",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// SPM 152 — Spider-Woman, Stunning Savior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_WOMAN_STUNNING_SAVIOR: CardRecord = CardRecord::new(
    "Spider-Woman, Stunning Savior",
    "bc9b2a76-3cce-4fd0-a4ef-932747cb11b2",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// SPM 153 — The Spot, Living Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SPOT_LIVING_PORTAL: CardRecord = CardRecord::new(
    "The Spot, Living Portal",
    "09081740-1180-48ea-b50b-e016d9c3828a",
    "Bastien Grivet",
    crate::card::CardRules::unsupported(),
);

// SPM 154 — Sun-Spider, Nimble Webber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUN_SPIDER_NIMBLE_WEBBER: CardRecord = CardRecord::new(
    "Sun-Spider, Nimble Webber",
    "2b54d4a5-634f-4ae3-b592-0dc527f60d56",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// SPM 155 — Superior Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPERIOR_SPIDER_MAN: CardRecord = CardRecord::new(
    "Superior Spider-Man",
    "ad4adc3e-ec41-4406-8ff2-59ba8067cf4e",
    "Carlos Dattoli",
    crate::card::CardRules::unsupported(),
);

// SPM 156 — Symbiote Spider-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYMBIOTE_SPIDER_MAN: CardRecord = CardRecord::new(
    "Symbiote Spider-Man",
    "6a21c0ff-b51a-4946-9737-7872a7eef97b",
    "Paolo Rivera",
    crate::card::CardRules::unsupported(),
);

// SPM 157 — Ultimate Green Goblin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ULTIMATE_GREEN_GOBLIN: CardRecord = CardRecord::new(
    "Ultimate Green Goblin",
    "e82d3f71-8404-40e7-b7fa-35713d1b384e",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// SPM 158 — Vulture, Scheming Scavenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VULTURE_SCHEMING_SCAVENGER: CardRecord = CardRecord::new(
    "Vulture, Scheming Scavenger",
    "e29281be-e722-4149-93a0-6dd3f0f64253",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// SPM 159 — Web-Warriors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEB_WARRIORS: CardRecord = CardRecord::new(
    "Web-Warriors",
    "741f5373-b51a-421a-9a74-326f0575c99b",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// SPM 160 — Wraith, Vicious Vigilante
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRAITH_VICIOUS_VIGILANTE: CardRecord = CardRecord::new(
    "Wraith, Vicious Vigilante",
    "5f46ed93-de6d-4180-9018-26ee07f75464",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// SPM 161 — Bagel and Schmear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAGEL_AND_SCHMEAR: CardRecord = CardRecord::new(
    "Bagel and Schmear",
    "7f927f72-fc9b-444f-9e0e-78a5e8e7bcaa",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// SPM 162 — Doc Ock's Tentacles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOC_OCK_S_TENTACLES: CardRecord = CardRecord::new(
    "Doc Ock's Tentacles",
    "fed9547c-9d0d-4e62-9639-887ed09231a2",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// SPM 163 — Eerie Gravestone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EERIE_GRAVESTONE: CardRecord = CardRecord::new(
    "Eerie Gravestone",
    "7675e91f-dba7-4e64-a7ff-1dd56665a4cc",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// SPM 164 — Hot Dog Cart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOT_DOG_CART: CardRecord = CardRecord::new(
    "Hot Dog Cart",
    "6ee3b883-e9f5-426f-a2ea-96fe9ff3aba9",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// SPM 165 — Interdimensional Web Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTERDIMENSIONAL_WEB_WATCH: CardRecord = CardRecord::new(
    "Interdimensional Web Watch",
    "87a8e112-e72f-413f-88a3-e7ce72c2ec53",
    "Toni Infante",
    crate::card::CardRules::unsupported(),
);

// SPM 166 — Iron Spider, Stark Upgrade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_SPIDER_STARK_UPGRADE: CardRecord = CardRecord::new(
    "Iron Spider, Stark Upgrade",
    "8da5f34e-7f40-406a-88d2-bb1e3ed25200",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// SPM 167 — Living Brain, Mechanical Marvel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_BRAIN_MECHANICAL_MARVEL: CardRecord = CardRecord::new(
    "Living Brain, Mechanical Marvel",
    "26833b64-2e6d-4977-9a6e-6fe73c54d671",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// SPM 168 — Mechanical Mobster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MECHANICAL_MOBSTER: CardRecord = CardRecord::new(
    "Mechanical Mobster",
    "6c6d9ecc-2dd1-471a-8678-a2461b1084fa",
    "David Szabo",
    crate::card::CardRules::unsupported(),
);

// SPM 169 — News Helicopter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEWS_HELICOPTER: CardRecord = CardRecord::new(
    "News Helicopter",
    "15717af0-30cd-4417-947a-c27cca06d93a",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// SPM 170 — Passenger Ferry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PASSENGER_FERRY: CardRecord = CardRecord::new(
    "Passenger Ferry",
    "2495f477-b88c-4938-a86a-f72c3c861188",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// SPM 171 — Peter Parker's Camera
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PETER_PARKER_S_CAMERA: CardRecord = CardRecord::new(
    "Peter Parker's Camera",
    "47875dff-c046-4cb0-b1e3-f926cbe25b59",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

// SPM 172 — Rocket-Powered Goblin Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCKET_POWERED_GOBLIN_GLIDER: CardRecord = CardRecord::new(
    "Rocket-Powered Goblin Glider",
    "c6c39232-72cc-4363-83d0-b5873f14f231",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// SPM 173 — Spider-Bot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_BOT: CardRecord = CardRecord::new(
    "Spider-Bot",
    "24df824a-c1d6-4f09-b866-313b31fec5fb",
    "Carlos Dattoli",
    crate::card::CardRules::unsupported(),
);

// SPM 174 — Spider-Mobile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MOBILE: CardRecord = CardRecord::new(
    "Spider-Mobile",
    "f12664c0-d7cd-4acb-87db-cfa3c85f32a9",
    "Bastien Grivet",
    crate::card::CardRules::unsupported(),
);

// SPM 175 — Spider-Slayer, Hatred Honed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_SLAYER_HATRED_HONED: CardRecord = CardRecord::new(
    "Spider-Slayer, Hatred Honed",
    "ac37ca6f-a6ee-4dfb-949f-2562f98d09d0",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// SPM 176 — Spider-Suit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_SUIT: CardRecord = CardRecord::new(
    "Spider-Suit",
    "436527ec-5af4-4b6d-a5a0-d21fc466a625",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// SPM 177 — Steel Wrecking Ball
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_WRECKING_BALL: CardRecord = CardRecord::new(
    "Steel Wrecking Ball",
    "7f2c74f5-1cfe-4918-a86b-0d58ac8b7469",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// SPM 178 — Subway Train
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBWAY_TRAIN: CardRecord = CardRecord::new(
    "Subway Train",
    "96f869f7-db88-4580-82b7-8749a55e525c",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// SPM 179 — Daily Bugle Building
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAILY_BUGLE_BUILDING: CardRecord = CardRecord::new(
    "Daily Bugle Building",
    "669bbcb1-0981-40e7-905e-b94e74bc4861",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// SPM 180 — Multiversal Passage
pub(in crate::card::sets) static MULTIVERSAL_PASSAGE: CardRecord = CardRecord::new(
    "Multiversal Passage",
    "f5fb426a-5618-4dd4-9c51-0cc847be8c1d",
    "Pablo Mendoza",
    // A shock land that is whichever basic type the hand actually wants,
    // which is a different card in a deck with two colours and in one with
    // five. The mana ability comes from the type rather than a printed
    // clause, so choosing is all there is to it.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::replacement(
            "As this land enters, choose a basic land type. Then you may pay 2 life. If you \
             don't, it enters tapped.",
            ReplacementEffectDef::Sequence(&[
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::BASIC_LAND_TYPE,
                )),
                ReplacementEffectDef::PayOr {
                    payment: EffectPaymentDef::new(
                        PlayerSetDef::Related(PlayerRelation::You),
                        &[CostDef::PayLife(2)],
                    ),
                    if_paid: &[],
                    // Declining is what makes it a tapped land, so the branch that pays does
                    // nothing at all and the branch that does not is the whole cost.
                    if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                        BattlefieldEntryModificationDef::Tapped,
                    )],
                },
            ]),
        ),
        AbilityDef::static_ability(
            "This land is the chosen type.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_chosen_basic_land_type(),
            },
        ),
    ]),
);

/// The OM1 cycle of tapped duals that surveil late: lands that differ only
/// in which two colours they make, so the clauses are written once here.
/// Entering tapped is the price of the two colours, and the surveil is what
/// a flooded late game does with the land instead of drawing it.
///
/// `colors` is a promoted literal at each call site, and the abilities are
/// added one at a time in printed order: an array holding the parameterized
/// mana ability could not be given a `'static` lifetime.
///
/// Spectacle Summit prints the same shape but is not in this cycle -- its
/// surveil costs {2}{U}{R} rather than {4}.
const fn surveilling_dual_land(mana_text: &'static str, colors: &'static [ManaColor]) -> CardRules {
    CardRules::new_land(&[])
        .with_ability(abilities::enters_tapped(CardType::Land))
        .with_ability(AbilityDef::activated_mana(
            mana_text,
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(colors)),
        ))
        .with_ability(AbilityDef::activated(
            "{4}, {T}: Surveil 1. (Look at the top card of your library. You may put it into \
             your graveyard.)",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ))
}

// SPM 181 — Ominous Asylum
pub(in crate::card::sets) static OMINOUS_ASYLUM: CardRecord = CardRecord::new(
    "Ominous Asylum",
    "4329f94a-9110-4f07-b4a6-f1ccae97ccc9",
    "Pavel Kolomeyets",
    surveilling_dual_land("{T}: Add {B} or {R}.", &[ManaColor::Black, ManaColor::Red]),
);

// SPM 182 — Oscorp Industries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OSCORP_INDUSTRIES: CardRecord = CardRecord::new(
    "Oscorp Industries",
    "1e609d6e-9e37-45d2-87de-8c76675f7cec",
    "Bastien Grivet",
    crate::card::CardRules::unsupported(),
);

// SPM 183 — Savage Mansion
pub(in crate::card::sets) static SAVAGE_MANSION: CardRecord = CardRecord::new(
    "Savage Mansion",
    "855f59a5-17a8-4aca-8a4d-f98111eba14c",
    "David Álvarez",
    surveilling_dual_land("{T}: Add {R} or {G}.", &[ManaColor::Red, ManaColor::Green]),
);

// SPM 184 — Sinister Hideout
pub(in crate::card::sets) static SINISTER_HIDEOUT: CardRecord = CardRecord::new(
    "Sinister Hideout",
    "23190d7e-5165-49bd-b307-bf81877d228d",
    "Pavel Kolomeyets",
    surveilling_dual_land("{T}: Add {U} or {B}.", &[ManaColor::Blue, ManaColor::Black]),
);

// SPM 185 — Suburban Sanctuary
pub(in crate::card::sets) static SUBURBAN_SANCTUARY: CardRecord = CardRecord::new(
    "Suburban Sanctuary",
    "467df77a-a99c-4cfd-9af4-502eaa2eb2e3",
    "David Frasheski",
    surveilling_dual_land(
        "{T}: Add {G} or {W}.",
        &[ManaColor::Green, ManaColor::White],
    ),
);

// SPM 186 — University Campus
pub(in crate::card::sets) static UNIVERSITY_CAMPUS: CardRecord = CardRecord::new(
    "University Campus",
    "2752f21c-f535-4772-a8b3-e97e1339e9c9",
    "David Álvarez",
    // A Campus that surveils rather than scries, so it does not share the
    // Strixhaven cycle's clause even though the rest of the card matches.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {U}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Blue,
            ])),
        ),
        AbilityDef::activated(
            "{4}, {T}: Surveil 1.",
            &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// SPM 187 — Urban Retreat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBAN_RETREAT: CardRecord = CardRecord::new(
    "Urban Retreat",
    "2581f320-8238-413d-ab04-d5535da55630",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// SPM 188 — Vibrant Cityscape
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIBRANT_CITYSCAPE: CardRecord = CardRecord::new(
    "Vibrant Cityscape",
    "9c110fa1-2320-4652-b282-ed064a9ec9a9",
    "Wei Guan",
    crate::card::CardRules::unsupported(),
);

// SPM 189 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "1164f7ec-7b2f-4cc9-90bb-7eaaa331b4cd",
    "Sarah Finnigan",
);

// SPM 190 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "d59cb0b5-fd4f-4dde-a69f-7ca6aa12b89f",
    "Sarah Finnigan",
);

// SPM 191 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "5cb03b18-d74c-4c89-9539-3549d2e8ff5f",
    "Sarah Finnigan",
);

// SPM 192 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "b044630d-50e7-431b-8e91-bd53e967f594",
    "Sarah Finnigan",
);

// SPM 193 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "7b6c2532-be5a-4f1f-893c-36bcda2a699d",
    "Sarah Finnigan",
);

// SPM 194 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "94e88862-d53d-49b6-8aa5-95f07507c6e1",
    "Jonas De Ro",
);

// SPM 195 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "14fcdc57-12e2-429b-8916-4df752e462d4",
    "Jonas De Ro",
);

// SPM 196 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "a7150a6e-240f-4628-acdc-153d404370ff",
    "Jonas De Ro",
);

// SPM 197 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "fc5004db-8db3-4506-bb01-f41be7968824",
    "Jonas De Ro",
);

// SPM 198 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "c4fec728-8e3f-427d-81ab-e06114910223",
    "Jonas De Ro",
);

// SPM 199 — SP//dr, Piloted by Peni (alternate printing)
const SP_DR_PILOTED_BY_PENI_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SP_DR_PILOTED_BY_PENI,
    1,
    "e8c1aace-e8fe-45ee-bc40-225409ff8657",
    "Jim Cheung & Jay David Ramos",
);

// SPM 200 — Miles Morales // Ultimate Spider-Man (alternate printing)
const MILES_MORALES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MILES_MORALES,
    1,
    "b6c129a7-59d3-499e-8279-f374266150be",
    "Jim Cheung & Jay David Ramos",
);

// SPM 201 — Spider-Ham, Peter Porker (alternate printing)
const SPIDER_HAM_PETER_PORKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_HAM_PETER_PORKER,
    1,
    "020426f0-ca38-4b0e-aa20-6fe12dc34853",
    "Jim Cheung & Jay David Ramos",
);

// SPM 202 — Gwen Stacy // Ghost-Spider (alternate printing)
const GWEN_STACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GWEN_STACY,
    1,
    "9126dfa0-b681-44c7-9ae1-7926df8a2767",
    "Jim Cheung & Jay David Ramos",
);

// SPM 203 — Web-Warriors (alternate printing)
const WEB_WARRIORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEB_WARRIORS,
    1,
    "bd672045-fe36-4a03-abf7-9c81f4100bba",
    "Jim Cheung & Jay David Ramos",
);

// SPM 204 — Spider-Man Noir (alternate printing)
const SPIDER_MAN_NOIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_NOIR,
    1,
    "d9a8b9a4-c9d9-4e9f-80b0-23de71a7423b",
    "Jim Cheung & Jay David Ramos",
);

// SPM 205 — Spider-Man 2099 (alternate printing)
const SPIDER_MAN_2099_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_2099,
    1,
    "b680a089-5cf0-494a-98e8-23d28315f572",
    "Jim Cheung & Jay David Ramos",
);

// SPM 206 — Multiversal Passage (alternate printing)
const MULTIVERSAL_PASSAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MULTIVERSAL_PASSAGE,
    1,
    "db88b6b6-7907-4c40-b712-4fa1456d8ad0",
    "Jim Cheung & Jay David Ramos",
);

// SPM 207 — Spider-Punk (alternate printing)
const SPIDER_PUNK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_PUNK,
    1,
    "8807635d-b5f2-4f08-af5e-5792867eb60b",
    "Jim Cheung & Jay David Ramos",
);

// SPM 208 — Peter Parker // Amazing Spider-Man (alternate printing)
const PETER_PARKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PETER_PARKER,
    1,
    "97fcf5a5-54e1-43f3-95e3-edc6215bf973",
    "Lucas Werneck",
);

// SPM 209 — Gwen Stacy // Ghost-Spider (alternate printing)
const GWEN_STACY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GWEN_STACY,
    2,
    "4ab0cb80-4330-408b-bc03-ef96dd34177e",
    "Roberta Ingranata",
);

// SPM 210 — Spider-Punk (alternate printing)
const SPIDER_PUNK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_PUNK,
    2,
    "47cebb95-3a25-4e9e-ad9c-034e0cd5bfd1",
    "Chris Bachalo",
);

// SPM 211 — Miles Morales // Ultimate Spider-Man (alternate printing)
const MILES_MORALES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MILES_MORALES,
    2,
    "b73db1c4-64d1-4d09-852a-7d0d299e5529",
    "Ivan Shavrin",
);

// SPM 212 — Radioactive Spider (alternate printing)
const RADIOACTIVE_SPIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RADIOACTIVE_SPIDER,
    1,
    "a0efb06d-7b91-4d64-98b2-244d56a44ecb",
    "Tyler Walpole",
);

// SPM 213 — Araña, Heart of the Spider (alternate printing)
const ARANA_HEART_OF_THE_SPIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARANA_HEART_OF_THE_SPIDER,
    1,
    "95da1818-7589-44d8-8bba-16ba3ef7b24e",
    "Logan Lubera",
);

// SPM 214 — Scarlet Spider, Ben Reilly (alternate printing)
const SCARLET_SPIDER_BEN_REILLY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCARLET_SPIDER_BEN_REILLY,
    1,
    "6d477a72-d4ad-4230-aee8-2bd78e448bb5",
    "Logan Lubera",
);

// SPM 215 — Silk, Web Weaver (alternate printing)
const SILK_WEB_WEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILK_WEB_WEAVER,
    1,
    "f6ff59e4-c7b7-4fdc-a49e-2d7aab859474",
    "Veronica Fish",
);

// SPM 216 — Spider-Man 2099 (alternate printing)
const SPIDER_MAN_2099_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_2099,
    2,
    "3b4e11a7-b48b-4fd8-9395-7a9bc655439a",
    "Jim Cheung & Jay David Ramos",
);

// SPM 217 — Symbiote Spider-Man (alternate printing)
const SYMBIOTE_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYMBIOTE_SPIDER_MAN,
    1,
    "89e046ae-f27a-484f-986c-61c86e754c92",
    "Tyler Walpole",
);

// SPM 218 — Origin of Spider-Man (alternate printing)
const ORIGIN_OF_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORIGIN_OF_SPIDER_MAN,
    1,
    "634e0542-066b-4fb5-a130-fa94a019f712",
    "Jim Cheung & Jay David Ramos",
);

// SPM 219 — The Clone Saga (alternate printing)
const THE_CLONE_SAGA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_CLONE_SAGA,
    1,
    "903b133f-fd72-4173-9dfe-3dcc411a0b92",
    "Logan Lubera",
);

// SPM 220 — Norman Osborn // Green Goblin (alternate printing)
const NORMAN_OSBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NORMAN_OSBORN,
    1,
    "f35d0be0-19e3-417e-aabf-fbed1aefd73c",
    "Steve Ellis",
);

// SPM 221 — Behold the Sinister Six! (alternate printing)
const BEHOLD_THE_SINISTER_SIX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEHOLD_THE_SINISTER_SIX,
    1,
    "62b76152-4017-487e-81ef-008084e5dcd4",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 222 — Black Cat, Cunning Thief (alternate printing)
const BLACK_CAT_CUNNING_THIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLACK_CAT_CUNNING_THIEF,
    1,
    "aae8c437-b172-44cd-98e8-d0ab9fae54c2",
    "Veronica Fish",
);

// SPM 223 — The Death of Gwen Stacy (alternate printing)
const THE_DEATH_OF_GWEN_STACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DEATH_OF_GWEN_STACY,
    1,
    "aa32b689-9c5c-4c6c-b5be-0e036e9a3223",
    "Nicola Scott",
);

// SPM 224 — Eddie Brock // Venom, Lethal Protector (alternate printing)
const EDDIE_BROCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EDDIE_BROCK,
    1,
    "1a172a40-7758-43ca-861b-8375a94d329e",
    "Logan Lubera",
);

// SPM 225 — Maximum Carnage (alternate printing)
const MAXIMUM_CARNAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAXIMUM_CARNAGE,
    1,
    "cdba10bb-7766-45a9-ab2d-7d62309cefea",
    "Jim Cheung & Jay David Ramos",
);

// SPM 226 — Kraven's Last Hunt (alternate printing)
const KRAVEN_S_LAST_HUNT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRAVEN_S_LAST_HUNT,
    1,
    "eae0bf5a-d5b1-4d87-af8e-e7dd512dfb4a",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 227 — Carnage, Crimson Chaos (alternate printing)
const CARNAGE_CRIMSON_CHAOS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARNAGE_CRIMSON_CHAOS,
    1,
    "67a7cbe7-cf78-4a3f-86c3-00d5ab93361e",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 228 — Doctor Octopus, Master Planner (alternate printing)
const DOCTOR_OCTOPUS_MASTER_PLANNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOCTOR_OCTOPUS_MASTER_PLANNER,
    1,
    "7e6e16f4-9448-4161-8e88-fc7734c2668d",
    "Jim Cheung & Jay David Ramos",
);

// SPM 229 — Mary Jane Watson (alternate printing)
const MARY_JANE_WATSON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARY_JANE_WATSON,
    1,
    "2e5e3a7c-ebb9-483f-9482-636c28f539e0",
    "Roberta Ingranata",
);

// SPM 230 — Spider-Woman, Stunning Savior (alternate printing)
const SPIDER_WOMAN_STUNNING_SAVIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_WOMAN_STUNNING_SAVIOR,
    1,
    "4d502370-a5d9-41b6-a1d3-fa9626b4fc8b",
    "Roberta Ingranata",
);

// SPM 231 — The Spot, Living Portal (alternate printing)
const THE_SPOT_LIVING_PORTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SPOT_LIVING_PORTAL,
    1,
    "6891b71d-c7af-4e55-afb4-72ab6887d290",
    "Tyler Walpole",
);

// SPM 232 — Peter Parker // Amazing Spider-Man (alternate printing)
const PETER_PARKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PETER_PARKER,
    2,
    "98912aae-4e42-4434-b979-9c85f09f8d6d",
    "Jack Kirby & Steve Ditko",
);

// SPM 233 — Eddie Brock // Venom, Lethal Protector (alternate printing)
const EDDIE_BROCK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EDDIE_BROCK,
    2,
    "4d174223-b7d5-4c71-8a3d-8c878a5b45b7",
    "Todd McFarlane & Bob Sharen",
);

// SPM 234 — Miles Morales // Ultimate Spider-Man (alternate printing)
const MILES_MORALES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MILES_MORALES,
    3,
    "d938ceec-c45b-482c-8841-c97098697cc8",
    "Sara Pichelli & Justin Ponsor",
);

// SPM 235 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    1,
    "6e9491af-6e93-46b8-b98c-96a2cc621454",
    "Roberta Ingranata",
);

// SPM 236 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    2,
    "ee9d4bf3-a53e-4940-b25b-e927f06df736",
    "Roberta Ingranata",
);

// SPM 237 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    3,
    "59bdb4a0-0f2d-4018-ad74-3970a5cd71ab",
    "Roberta Ingranata",
);

// SPM 238 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    4,
    "49852fa3-272c-4ca4-b0dc-0b80f2982fd9",
    "Roberta Ingranata",
);

// SPM 239 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    5,
    "cae418f9-e001-4580-b743-74d134b06aa6",
    "Roberta Ingranata",
);

// SPM 240 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_6: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    6,
    "2362fb6d-161d-479b-b1f4-909360ba5d84",
    "Roberta Ingranata",
);

// SPM 241 — Spectacular Spider-Man (alternate printing)
const SPECTACULAR_SPIDER_MAN_ALTERNATE_7: PrintingRecord = PrintingRecord::alternate(
    &SPECTACULAR_SPIDER_MAN,
    7,
    "011b71a9-c06d-4fd6-8543-ad21f4759473",
    "Roberta Ingranata",
);

// SPM 242 — The Soul Stone (alternate printing)
const THE_SOUL_STONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SOUL_STONE,
    1,
    "2c3df372-09d5-42fb-9357-3f97745e07c4",
    "Madeline Boni",
);

// SPM 243 — The Soul Stone (alternate printing)
const THE_SOUL_STONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_SOUL_STONE,
    2,
    "f9d80efc-e829-4257-83e8-f37b0b68de57",
    "Leinil Francis Yu & Sunny Gho",
);

// SPM 244 — Anti-Venom, Horrifying Healer (alternate printing)
const ANTI_VENOM_HORRIFYING_HEALER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTI_VENOM_HORRIFYING_HEALER,
    1,
    "451f215d-b451-42d7-a277-631b6ad8098f",
    "Néstor Ossandón Leal",
);

// SPM 245 — Arachne, Psionic Weaver (alternate printing)
const ARACHNE_PSIONIC_WEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARACHNE_PSIONIC_WEAVER,
    1,
    "f3b2924f-fad0-4bff-ab99-091b1c234297",
    "Steve Argyle",
);

// SPM 246 — Friendly Neighborhood (alternate printing)
const FRIENDLY_NEIGHBORHOOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRIENDLY_NEIGHBORHOOD,
    1,
    "2e7c80af-4841-4e5f-a3c9-8448b63b3788",
    "Pablo Mendoza",
);

// SPM 247 — Rent Is Due (alternate printing)
const RENT_IS_DUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RENT_IS_DUE,
    1,
    "c60530c0-4ce1-434f-9140-c50370d418b1",
    "Gal Or",
);

// SPM 248 — With Great Power . . . (alternate printing)
const WITH_GREAT_POWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WITH_GREAT_POWER,
    1,
    "fb266506-5ea8-45c2-8740-4a71fb7e5133",
    "E. M. Gist",
);

// SPM 249 — Hide on the Ceiling (alternate printing)
const HIDE_ON_THE_CEILING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIDE_ON_THE_CEILING,
    1,
    "3d9847a8-ae5d-4f24-8605-609f0c1a11bc",
    "Fariba Khamseh",
);

// SPM 250 — Hydro-Man, Fluid Felon (alternate printing)
const HYDRO_MAN_FLUID_FELON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HYDRO_MAN_FLUID_FELON,
    1,
    "0a1265db-ff16-4751-9301-78d29b271cda",
    "Borja Pindado",
);

// SPM 251 — Impostor Syndrome (alternate printing)
const IMPOSTOR_SYNDROME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IMPOSTOR_SYNDROME,
    1,
    "72341b76-c3bb-4c39-80d9-f4747d139375",
    "Javier Charro",
);

// SPM 252 — Lady Octopus, Inspired Inventor (alternate printing)
const LADY_OCTOPUS_INSPIRED_INVENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LADY_OCTOPUS_INSPIRED_INVENTOR,
    1,
    "13c85292-7dc8-4824-8e7f-189ab9420af8",
    "Fariba Khamseh",
);

// SPM 253 — Mysterio, Master of Illusion (alternate printing)
const MYSTERIO_MASTER_OF_ILLUSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MYSTERIO_MASTER_OF_ILLUSION,
    1,
    "be0cd0c9-9d43-47d1-9836-24e8d5917f65",
    "Alexander Gering",
);

// SPM 254 — Spider-Sense (alternate printing)
const SPIDER_SENSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_SENSE,
    1,
    "426b3c04-4d4b-4f75-984f-89eecf8f09f2",
    "Borja Pindado",
);

// SPM 255 — Agent Venom (alternate printing)
const AGENT_VENOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGENT_VENOM,
    1,
    "5569baf4-0cc9-46da-80cb-2cac5452f460",
    "Kevin Sidharta",
);

// SPM 256 — Gwenom, Remorseless (alternate printing)
const GWENOM_REMORSELESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GWENOM_REMORSELESS,
    1,
    "431f3f3b-42a7-4161-946b-c9bbf7bd5535",
    "Lordigan",
);

// SPM 257 — Morlun, Devourer of Spiders (alternate printing)
const MORLUN_DEVOURER_OF_SPIDERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MORLUN_DEVOURER_OF_SPIDERS,
    1,
    "2c5efc7d-7b09-4487-85cb-58867e06b12d",
    "Randy Gallegos",
);

// SPM 258 — Parker Luck (alternate printing)
const PARKER_LUCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PARKER_LUCK,
    1,
    "c69e751f-5eee-48d7-9e0f-ddbcca96491e",
    "Raoul Vitale",
);

// SPM 259 — Villainous Wrath (alternate printing)
const VILLAINOUS_WRATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VILLAINOUS_WRATH,
    1,
    "6f7d7311-2248-4841-960a-8c93497e3de1",
    "InHyuk Lee",
);

// SPM 260 — Electro, Assaulting Battery (alternate printing)
const ELECTRO_ASSAULTING_BATTERY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELECTRO_ASSAULTING_BATTERY,
    1,
    "40466912-0d20-42ff-8232-807bf947030b",
    "Piotr Dura",
);

// SPM 261 — J. Jonah Jameson (alternate printing)
const J_JONAH_JAMESON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &J_JONAH_JAMESON,
    1,
    "c81a2ecf-837c-44a9-81d4-476763f89107",
    "Paolo Rivera",
);

// SPM 262 — Shadow of the Goblin (alternate printing)
const SHADOW_OF_THE_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHADOW_OF_THE_GOBLIN,
    1,
    "e96068e3-4c79-4c59-8821-053f21db42c7",
    "Pavel Kolomeyets",
);

// SPM 263 — Spider-Verse (alternate printing)
const SPIDER_VERSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_VERSE,
    1,
    "5e85512d-0b46-48f9-a391-6ae6a1b7fb55",
    "Alexander Gering",
);

// SPM 264 — Spinneret and Spiderling (alternate printing)
const SPINNERET_AND_SPIDERLING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPINNERET_AND_SPIDERLING,
    1,
    "d75ccaee-e5ca-423f-a235-c0e989cbf76f",
    "Le Vuong",
);

// SPM 265 — Lizard, Connors's Curse (alternate printing)
const LIZARD_CONNORS_S_CURSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIZARD_CONNORS_S_CURSE,
    1,
    "fef697cd-7bc7-4c1f-87cd-2921c48bc02a",
    "Steve Prescott",
);

// SPM 266 — Sandman, Shifting Scoundrel (alternate printing)
const SANDMAN_SHIFTING_SCOUNDREL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SANDMAN_SHIFTING_SCOUNDREL,
    1,
    "496864ab-bdba-4cfe-aa7b-b58d2954cf75",
    "Bartek Fedyczak",
);

// SPM 267 — Strength of Will (alternate printing)
const STRENGTH_OF_WILL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STRENGTH_OF_WILL,
    1,
    "084f283e-88a0-4b7a-9e63-908caeb70f04",
    "Ryan Pancoast",
);

// SPM 268 — Web of Life and Destiny (alternate printing)
const WEB_OF_LIFE_AND_DESTINY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEB_OF_LIFE_AND_DESTINY,
    1,
    "1672374a-585d-4a26-b04f-a129c4da8b0e",
    "Jonas De Ro",
);

// SPM 269 — Biorganic Carapace (alternate printing)
const BIORGANIC_CARAPACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BIORGANIC_CARAPACE,
    1,
    "2bd4c0aa-f431-423e-8f95-2796a0f05594",
    "David Álvarez",
);

// SPM 270 — Cheering Crowd (alternate printing)
const CHEERING_CROWD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHEERING_CROWD,
    1,
    "44f436ca-0ae8-4c35-871b-4bd2732e5a49",
    "Kim Sokol",
);

// SPM 271 — Cosmic Spider-Man (alternate printing)
const COSMIC_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMIC_SPIDER_MAN,
    1,
    "61b28030-6d05-429e-a4eb-fceead3f7b18",
    "Zoltan Boros",
);

// SPM 272 — Jackal, Genius Geneticist (alternate printing)
const JACKAL_GENIUS_GENETICIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JACKAL_GENIUS_GENETICIST,
    1,
    "185bf150-e032-47af-aafc-3b4b69f0bad4",
    "Pavel Kolomeyets",
);

// SPM 273 — Kraven the Hunter (alternate printing)
const KRAVEN_THE_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KRAVEN_THE_HUNTER,
    1,
    "82f977d1-2209-4378-9b79-edb2c338f544",
    "Greg Staples",
);

// SPM 274 — Mister Negative (alternate printing)
const MISTER_NEGATIVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MISTER_NEGATIVE,
    1,
    "e713824a-b5df-4a05-81ad-a426c5eae1ce",
    "Thanh Tuấn",
);

// SPM 275 — Superior Spider-Man (alternate printing)
const SUPERIOR_SPIDER_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPERIOR_SPIDER_MAN,
    1,
    "895a6b80-a5c6-400d-931c-589bfa684b39",
    "Carlos Dattoli",
);

// SPM 276 — Ultimate Green Goblin (alternate printing)
const ULTIMATE_GREEN_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTIMATE_GREEN_GOBLIN,
    1,
    "5e96fe19-9671-48b0-92fc-c46224c433c6",
    "Jesper Ejsing",
);

// SPM 277 — Doc Ock's Tentacles (alternate printing)
const DOC_OCK_S_TENTACLES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOC_OCK_S_TENTACLES,
    1,
    "eab4f4f1-3e4e-403e-8264-b3e73ee8259a",
    "David Álvarez",
);

// SPM 278 — Interdimensional Web Watch (alternate printing)
const INTERDIMENSIONAL_WEB_WATCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INTERDIMENSIONAL_WEB_WATCH,
    1,
    "740cde32-57e6-4633-98e9-724c505eb008",
    "Toni Infante",
);

// SPM 279 — Iron Spider, Stark Upgrade (alternate printing)
const IRON_SPIDER_STARK_UPGRADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_SPIDER_STARK_UPGRADE,
    1,
    "517aeeb2-454f-4de8-b7f7-870010e80ae8",
    "Kevin Glint",
);

// SPM 280 — Peter Parker's Camera (alternate printing)
const PETER_PARKER_S_CAMERA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PETER_PARKER_S_CAMERA,
    1,
    "47c82856-d657-4315-9433-0754a68444eb",
    "Lixin Yin",
);

// SPM 281 — Rocket-Powered Goblin Glider (alternate printing)
const ROCKET_POWERED_GOBLIN_GLIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROCKET_POWERED_GOBLIN_GLIDER,
    1,
    "9fb3e089-17f0-4066-9ab2-c23b7b2a6ee7",
    "Pavel Kolomeyets",
);

// SPM 282 — Oscorp Industries (alternate printing)
const OSCORP_INDUSTRIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OSCORP_INDUSTRIES,
    1,
    "be2154a1-38c2-46b2-bd6f-3d7e509070b6",
    "Bastien Grivet",
);

// SPM 283 — Urban Retreat (alternate printing)
const URBAN_RETREAT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &URBAN_RETREAT,
    1,
    "a2b59a26-114a-489c-b898-f71949ca4c7e",
    "Jonas De Ro",
);

// SPM 284 — Spider-Sense (alternate printing)
const SPIDER_SENSE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_SENSE,
    2,
    "d123d19e-fb13-4c1c-b765-8c4f1fef453a",
    "David Álvarez",
);

// SPM 285 — Radioactive Spider (alternate printing)
const RADIOACTIVE_SPIDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RADIOACTIVE_SPIDER,
    2,
    "3173112e-8975-4bba-84b7-6ba27503dc82",
    "Toni Infante",
);

// SPM 286 — Gwenom, Remorseless (alternate printing)
const GWENOM_REMORSELESS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GWENOM_REMORSELESS,
    2,
    "a5b191f5-f01b-4d6c-8363-19591c61e6cc",
    "Jesper Ejsing",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANTI_VENOM_HORRIFYING_HEALER,
    &ARACHNE_PSIONIC_WEAVER,
    &AUNT_MAY,
    &CITY_PIGEON,
    &COSTUME_CLOSET,
    &DAILY_BUGLE_REPORTERS,
    &FLASH_THOMPSON_SPIDER_FAN,
    &FRIENDLY_NEIGHBORHOOD,
    &ORIGIN_OF_SPIDER_MAN,
    &PETER_PARKER,
    &RENT_IS_DUE,
    &SELFLESS_POLICE_CAPTAIN,
    &SILVER_SABLE_MERCENARY_LEADER,
    &SPECTACULAR_SPIDER_MAN,
    &SPECTACULAR_TACTICS,
    &SPIDER_MAN_WEB_SLINGER,
    &SPIDER_UK,
    &STARLING_AERIAL_ALLY,
    &SUDDEN_STRIKE,
    &THWIP,
    &WEB_UP,
    &WEB_SHOOTERS,
    &WILD_PACK_SQUAD,
    &WITH_GREAT_POWER,
    &AMAZING_ACROBATICS,
    &BEETLE_LEGACY_CRIMINAL,
    &CHAMELEON_MASTER_OF_DISGUISE,
    &THE_CLONE_SAGA,
    &DOC_OCK_SINISTER_SCIENTIST,
    &DOC_OCK_S_HENCHMEN,
    &FLYING_OCTOBOT,
    &HIDE_ON_THE_CEILING,
    &HYDRO_MAN_FLUID_FELON,
    &IMPOSTOR_SYNDROME,
    &LADY_OCTOPUS_INSPIRED_INVENTOR,
    &MADAME_WEB_CLAIRVOYANT,
    &MYSTERIO_MASTER_OF_ILLUSION,
    &MYSTERIO_S_PHANTASM,
    &NORMAN_OSBORN,
    &OSCORP_RESEARCH_TEAM,
    &ROBOTICS_MASTERY,
    &SCHOOL_DAZE,
    &SECRET_IDENTITY,
    &SPIDER_BYTE_WEB_WARDEN,
    &SPIDER_MAN_NO_MORE,
    &SPIDER_SENSE,
    &UNSTABLE_EXPERIMENT,
    &WHOOSH,
    &AGENT_VENOM,
    &ALIEN_SYMBIOSIS,
    &BEHOLD_THE_SINISTER_SIX,
    &BLACK_CAT_CUNNING_THIEF,
    &COMMON_CROOK,
    &THE_DEATH_OF_GWEN_STACY,
    &EDDIE_BROCK,
    &GWENOM_REMORSELESS,
    &INNER_DEMONS_GANGSTERS,
    &MERCILESS_ENFORCERS,
    &MORLUN_DEVOURER_OF_SPIDERS,
    &PARKER_LUCK,
    &PRISON_BREAK,
    &RISKY_RESEARCH,
    &SANDMAN_S_QUICKSAND,
    &SCORPION_SEETHING_STRIKER,
    &SCORPION_S_STING,
    &THE_SOUL_STONE,
    &SPIDER_MAN_NOIR,
    &THE_SPOT_S_PORTAL,
    &SWARM_BEING_OF_BEES,
    &TOMBSTONE_CAREER_CRIMINAL,
    &VENOM_EVIL_UNLEASHED,
    &VENOMIZED_CAT,
    &VENOM_S_HUNGER,
    &VILLAINOUS_WRATH,
    &ANGRY_RABBLE,
    &ELECTRO_ASSAULTING_BATTERY,
    &ELECTRO_S_BOLT,
    &GWEN_STACY,
    &HEROES_HANGOUT,
    &HOBGOBLIN_MANTLED_MARAUDER,
    &J_JONAH_JAMESON,
    &MASKED_MEOWER,
    &MAXIMUM_CARNAGE,
    &MOLTEN_MAN_INFERNO_INCARNATE,
    &RAGING_GOBLINOIDS,
    &ROMANTIC_RENDEZVOUS,
    &SHADOW_OF_THE_GOBLIN,
    &SHOCKER_UNSHAKABLE,
    &SPIDER_GWEN_FREE_SPIRIT,
    &SPIDER_ISLANDERS,
    &SPIDER_PUNK,
    &SPIDER_VERSE,
    &SPINNERET_AND_SPIDERLING,
    &STEGRON_THE_DINOSAUR_MAN,
    &SUPERIOR_FOES_OF_SPIDER_MAN,
    &TAXI_DRIVER,
    &WISECRACK,
    &DAMAGE_CONTROL_CREW,
    &EZEKIEL_SIMS_SPIDER_TOTEM,
    &GROW_EXTRA_ARMS,
    &GUY_IN_THE_CHAIR,
    &KAPOW,
    &KRAVEN_S_CATS,
    &KRAVEN_S_LAST_HUNT,
    &LIZARD_CONNORS_S_CURSE,
    &LURKING_LIZARDS,
    &MILES_MORALES,
    &PICTURES_OF_SPIDER_MAN,
    &PROFESSIONAL_WRESTLER,
    &RADIOACTIVE_SPIDER,
    &SANDMAN_SHIFTING_SCOUNDREL,
    &SCOUT_THE_CITY,
    &SPIDER_HAM_PETER_PORKER,
    &SPIDER_MAN_BROOKLYN_VISIONARY,
    &SPIDER_REX_DARING_DINO,
    &SPIDERS_MAN_HEROIC_HORDE,
    &STRENGTH_OF_WILL,
    &SUPPORTIVE_PARENTS,
    &TERRIFIC_TEAM_UP,
    &WALL_CRAWL,
    &WEB_OF_LIFE_AND_DESTINY,
    &ARANA_HEART_OF_THE_SPIDER,
    &BIORGANIC_CARAPACE,
    &CARNAGE_CRIMSON_CHAOS,
    &CHEERING_CROWD,
    &COSMIC_SPIDER_MAN,
    &DOCTOR_OCTOPUS_MASTER_PLANNER,
    &GALLANT_CITIZEN,
    &GREEN_GOBLIN_REVENANT,
    &JACKAL_GENIUS_GENETICIST,
    &KRAVEN_PROUD_PREDATOR,
    &KRAVEN_THE_HUNTER,
    &MARY_JANE_WATSON,
    &MISTER_NEGATIVE,
    &MOB_LOOKOUT,
    &MORBIUS_THE_LIVING_VAMPIRE,
    &PROWLER_CLAWED_THIEF,
    &PUMPKIN_BOMBARDMENT,
    &RHINO_BARRELING_BRUTE,
    &RHINOS_RAMPAGE,
    &SCARLET_SPIDER_BEN_REILLY,
    &SCARLET_SPIDER_KAINE,
    &SHRIEK_TREBLEMAKER,
    &SILK_WEB_WEAVER,
    &SKYWARD_SPIDER,
    &SP_DR_PILOTED_BY_PENI,
    &SPIDER_MANIFESTATION,
    &SPIDER_GIRL_LEGACY_HERO,
    &SPIDER_MAN_2099,
    &SPIDER_MAN_INDIA,
    &SPIDER_WOMAN_STUNNING_SAVIOR,
    &THE_SPOT_LIVING_PORTAL,
    &SUN_SPIDER_NIMBLE_WEBBER,
    &SUPERIOR_SPIDER_MAN,
    &SYMBIOTE_SPIDER_MAN,
    &ULTIMATE_GREEN_GOBLIN,
    &VULTURE_SCHEMING_SCAVENGER,
    &WEB_WARRIORS,
    &WRAITH_VICIOUS_VIGILANTE,
    &BAGEL_AND_SCHMEAR,
    &DOC_OCK_S_TENTACLES,
    &EERIE_GRAVESTONE,
    &HOT_DOG_CART,
    &INTERDIMENSIONAL_WEB_WATCH,
    &IRON_SPIDER_STARK_UPGRADE,
    &LIVING_BRAIN_MECHANICAL_MARVEL,
    &MECHANICAL_MOBSTER,
    &NEWS_HELICOPTER,
    &PASSENGER_FERRY,
    &PETER_PARKER_S_CAMERA,
    &ROCKET_POWERED_GOBLIN_GLIDER,
    &SPIDER_BOT,
    &SPIDER_MOBILE,
    &SPIDER_SLAYER_HATRED_HONED,
    &SPIDER_SUIT,
    &STEEL_WRECKING_BALL,
    &SUBWAY_TRAIN,
    &DAILY_BUGLE_BUILDING,
    &MULTIVERSAL_PASSAGE,
    &OMINOUS_ASYLUM,
    &OSCORP_INDUSTRIES,
    &SAVAGE_MANSION,
    &SINISTER_HIDEOUT,
    &SUBURBAN_SANCTUARY,
    &UNIVERSITY_CAMPUS,
    &URBAN_RETREAT,
    &VIBRANT_CITYSCAPE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    SHOCK_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_ALTERNATE_1,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_1,
    FOREST_ALTERNATE_1,
    SP_DR_PILOTED_BY_PENI_ALTERNATE_1,
    MILES_MORALES_ALTERNATE_1,
    SPIDER_HAM_PETER_PORKER_ALTERNATE_1,
    GWEN_STACY_ALTERNATE_1,
    WEB_WARRIORS_ALTERNATE_1,
    SPIDER_MAN_NOIR_ALTERNATE_1,
    SPIDER_MAN_2099_ALTERNATE_1,
    MULTIVERSAL_PASSAGE_ALTERNATE_1,
    SPIDER_PUNK_ALTERNATE_1,
    PETER_PARKER_ALTERNATE_1,
    GWEN_STACY_ALTERNATE_2,
    SPIDER_PUNK_ALTERNATE_2,
    MILES_MORALES_ALTERNATE_2,
    RADIOACTIVE_SPIDER_ALTERNATE_1,
    ARANA_HEART_OF_THE_SPIDER_ALTERNATE_1,
    SCARLET_SPIDER_BEN_REILLY_ALTERNATE_1,
    SILK_WEB_WEAVER_ALTERNATE_1,
    SPIDER_MAN_2099_ALTERNATE_2,
    SYMBIOTE_SPIDER_MAN_ALTERNATE_1,
    ORIGIN_OF_SPIDER_MAN_ALTERNATE_1,
    THE_CLONE_SAGA_ALTERNATE_1,
    NORMAN_OSBORN_ALTERNATE_1,
    BEHOLD_THE_SINISTER_SIX_ALTERNATE_1,
    BLACK_CAT_CUNNING_THIEF_ALTERNATE_1,
    THE_DEATH_OF_GWEN_STACY_ALTERNATE_1,
    EDDIE_BROCK_ALTERNATE_1,
    MAXIMUM_CARNAGE_ALTERNATE_1,
    KRAVEN_S_LAST_HUNT_ALTERNATE_1,
    CARNAGE_CRIMSON_CHAOS_ALTERNATE_1,
    DOCTOR_OCTOPUS_MASTER_PLANNER_ALTERNATE_1,
    MARY_JANE_WATSON_ALTERNATE_1,
    SPIDER_WOMAN_STUNNING_SAVIOR_ALTERNATE_1,
    THE_SPOT_LIVING_PORTAL_ALTERNATE_1,
    PETER_PARKER_ALTERNATE_2,
    EDDIE_BROCK_ALTERNATE_2,
    MILES_MORALES_ALTERNATE_3,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_1,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_2,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_3,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_4,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_5,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_6,
    SPECTACULAR_SPIDER_MAN_ALTERNATE_7,
    THE_SOUL_STONE_ALTERNATE_1,
    THE_SOUL_STONE_ALTERNATE_2,
    ANTI_VENOM_HORRIFYING_HEALER_ALTERNATE_1,
    ARACHNE_PSIONIC_WEAVER_ALTERNATE_1,
    FRIENDLY_NEIGHBORHOOD_ALTERNATE_1,
    RENT_IS_DUE_ALTERNATE_1,
    WITH_GREAT_POWER_ALTERNATE_1,
    HIDE_ON_THE_CEILING_ALTERNATE_1,
    HYDRO_MAN_FLUID_FELON_ALTERNATE_1,
    IMPOSTOR_SYNDROME_ALTERNATE_1,
    LADY_OCTOPUS_INSPIRED_INVENTOR_ALTERNATE_1,
    MYSTERIO_MASTER_OF_ILLUSION_ALTERNATE_1,
    SPIDER_SENSE_ALTERNATE_1,
    AGENT_VENOM_ALTERNATE_1,
    GWENOM_REMORSELESS_ALTERNATE_1,
    MORLUN_DEVOURER_OF_SPIDERS_ALTERNATE_1,
    PARKER_LUCK_ALTERNATE_1,
    VILLAINOUS_WRATH_ALTERNATE_1,
    ELECTRO_ASSAULTING_BATTERY_ALTERNATE_1,
    J_JONAH_JAMESON_ALTERNATE_1,
    SHADOW_OF_THE_GOBLIN_ALTERNATE_1,
    SPIDER_VERSE_ALTERNATE_1,
    SPINNERET_AND_SPIDERLING_ALTERNATE_1,
    LIZARD_CONNORS_S_CURSE_ALTERNATE_1,
    SANDMAN_SHIFTING_SCOUNDREL_ALTERNATE_1,
    STRENGTH_OF_WILL_ALTERNATE_1,
    WEB_OF_LIFE_AND_DESTINY_ALTERNATE_1,
    BIORGANIC_CARAPACE_ALTERNATE_1,
    CHEERING_CROWD_ALTERNATE_1,
    COSMIC_SPIDER_MAN_ALTERNATE_1,
    JACKAL_GENIUS_GENETICIST_ALTERNATE_1,
    KRAVEN_THE_HUNTER_ALTERNATE_1,
    MISTER_NEGATIVE_ALTERNATE_1,
    SUPERIOR_SPIDER_MAN_ALTERNATE_1,
    ULTIMATE_GREEN_GOBLIN_ALTERNATE_1,
    DOC_OCK_S_TENTACLES_ALTERNATE_1,
    INTERDIMENSIONAL_WEB_WATCH_ALTERNATE_1,
    IRON_SPIDER_STARK_UPGRADE_ALTERNATE_1,
    PETER_PARKER_S_CAMERA_ALTERNATE_1,
    ROCKET_POWERED_GOBLIN_GLIDER_ALTERNATE_1,
    OSCORP_INDUSTRIES_ALTERNATE_1,
    URBAN_RETREAT_ALTERNATE_1,
    SPIDER_SENSE_ALTERNATE_2,
    RADIOACTIVE_SPIDER_ALTERNATE_2,
    GWENOM_REMORSELESS_ALTERNATE_2,
];
