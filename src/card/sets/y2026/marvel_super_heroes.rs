//! Marvel Super Heroes card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2003::mirrodin as catalog_mrd;
use crate::card::sets::y2013::theros as catalog_ths;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;
use crate::card::sets::y2023::phyrexia_all_will_be_one as catalog_one;
use crate::card::sets::y2025::marvels_spider_man as catalog_spm;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MSH",
    slug: "marvel-super-heroes",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MSH 1 — Agent 13, Sharon Carter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_13_SHARON_CARTER: CardRecord = CardRecord::new(
    "Agent 13, Sharon Carter",
    "107cb521-9806-47dc-92d8-b43112b63caa",
    "Michael MacRae",
    crate::card::CardRules::unsupported(),
);

// MSH 2 — Agent Maria Hill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_MARIA_HILL: CardRecord = CardRecord::new(
    "Agent Maria Hill",
    "9e327c67-1cf1-4d82-903c-b41c8e7cf747",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// MSH 3 — Agent of Atlas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_OF_ATLAS: CardRecord = CardRecord::new(
    "Agent of Atlas",
    "a9db550e-ed1f-4973-a7f5-ec2e6adb8e6b",
    "Michael Machira",
    crate::card::CardRules::unsupported(),
);

// MSH 4 — Agent Phil Coulson
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_PHIL_COULSON: CardRecord = CardRecord::new(
    "Agent Phil Coulson",
    "1383e587-df58-4b45-9067-b9399e90b9ed",
    "Marc Aspinall",
    crate::card::CardRules::unsupported(),
);

// MSH 5 — Agents of S.H.I.E.L.D.
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENTS_OF_S_H_I_E_L_D: CardRecord = CardRecord::new(
    "Agents of S.H.I.E.L.D.",
    "d9b5e156-0764-44e0-b0c7-de2561ea9e04",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MSH 6 — Avengers Assemble!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGERS_ASSEMBLE: CardRecord = CardRecord::new(
    "Avengers Assemble!",
    "bf736399-af74-4f52-9159-67ea67d0cf83",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// MSH 7 — Borough Backup
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOROUGH_BACKUP: CardRecord = CardRecord::new(
    "Borough Backup",
    "bb753ac8-67e1-44b0-b404-37f04c2b7438",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// MSH 8 — Brave Brawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAVE_BRAWLER: CardRecord = CardRecord::new(
    "Brave Brawler",
    "2242b5c3-42ef-4be0-a61f-65c93e56fcab",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// MSH 9 — Captain America, Super-Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_AMERICA_SUPER_SOLDIER: CardRecord = CardRecord::new(
    "Captain America, Super-Soldier",
    "33631d6c-c584-42ff-afe5-2647b5fb321f",
    "Anna Podedworna",
    crate::card::CardRules::unsupported(),
);

// MSH 10 — Captain America, Wings of Freedom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_AMERICA_WINGS_OF_FREEDOM: CardRecord = CardRecord::new(
    "Captain America, Wings of Freedom",
    "c87ca9ce-5034-4137-99cc-c2b28f298912",
    "Dan Dos Santos",
    crate::card::CardRules::unsupported(),
);

// MSH 11 — Captain Marvel, Earth's Protector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_MARVEL_EARTH_S_PROTECTOR: CardRecord = CardRecord::new(
    "Captain Marvel, Earth's Protector",
    "eb098550-22e6-4079-8c59-ed9ec2f764e3",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// MSH 12 — Captain Mar-Vell, Space-Born
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_MAR_VELL_SPACE_BORN: CardRecord = CardRecord::new(
    "Captain Mar-Vell, Space-Born",
    "27924c2f-756c-42af-9830-18a5a2735137",
    "Gintas Galvanauskas",
    crate::card::CardRules::unsupported(),
);

// MSH 13 — Colleen Wing, Street Samurai
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLLEEN_WING_STREET_SAMURAI: CardRecord = CardRecord::new(
    "Colleen Wing, Street Samurai",
    "8bffc505-608e-4a5b-9ed9-2321e4cab484",
    "Jurijus Chitrovas",
    crate::card::CardRules::unsupported(),
);

// MSH 14 — Crowd of True Believers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWD_OF_TRUE_BELIEVERS: CardRecord = CardRecord::new(
    "Crowd of True Believers",
    "4b2fffb8-d538-4772-8fbc-9bec3b9c4d9c",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// MSH 15 — Helicarrier Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELICARRIER_STRIKE: CardRecord = CardRecord::new(
    "Helicarrier Strike",
    "6e518842-ce44-4af2-8f38-89869828294a",
    "Maxim Ruabtsev",
    crate::card::CardRules::unsupported(),
);

// MSH 16 — Hero in Training
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HERO_IN_TRAINING: CardRecord = CardRecord::new(
    "Hero in Training",
    "105a937b-289c-47b5-96a4-654c697bbb7d",
    "Taurin Clarke",
    crate::card::CardRules::unsupported(),
);

// MSH 17 — Invisible Woman, Sue Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVISIBLE_WOMAN_SUE_STORM: CardRecord = CardRecord::new(
    "Invisible Woman, Sue Storm",
    "2f80394b-2f7e-40a7-8203-720bcf39d71b",
    "Paolo Rivera",
    crate::card::CardRules::unsupported(),
);

// MSH 18 — Jennifer Walters // The Sensational She-Hulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JENNIFER_WALTERS: CardRecord = CardRecord::new(
    "Jennifer Walters // The Sensational She-Hulk",
    "61237530-ad49-469c-a952-67c92315708e",
    "Taurin Clarke",
    crate::card::CardRules::unsupported(),
);

// MSH 19 — Kree Commandos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KREE_COMMANDOS: CardRecord = CardRecord::new(
    "Kree Commandos",
    "401f9ccf-f30c-4777-9a5b-4c340e724fff",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// MSH 20 — Luke Cage, Power Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUKE_CAGE_POWER_MAN: CardRecord = CardRecord::new(
    "Luke Cage, Power Man",
    "3b72c9e9-1cb2-4374-873a-53af293d86d0",
    "Aniekan Udofia",
    crate::card::CardRules::unsupported(),
);

// MSH 21 — The Mind Stone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MIND_STONE: CardRecord = CardRecord::new(
    "The Mind Stone",
    "87f1e69a-6d74-4982-afda-82613637799a",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// MSH 22 — Mockingbird, Ace Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOCKINGBIRD_ACE_AGENT: CardRecord = CardRecord::new(
    "Mockingbird, Ace Agent",
    "0f5701ac-ec30-4fb1-bd71-4bac4693c075",
    "Sveta Pikul",
    crate::card::CardRules::unsupported(),
);

// MSH 23 — Monica Rambeau // Photon, Living Light
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONICA_RAMBEAU: CardRecord = CardRecord::new(
    "Monica Rambeau // Photon, Living Light",
    "3f995518-b12a-4623-9ab3-b79a5cef3cba",
    "Xabi Gaztelua & Marta Nael",
    crate::card::CardRules::unsupported(),
);

// MSH 24 — Murdock's Crusade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MURDOCK_S_CRUSADE: CardRecord = CardRecord::new(
    "Murdock's Crusade",
    "98df64ca-39c3-47e6-8143-4106c8e9cf59",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// MSH 25 — Nick Fury, Agent of S.H.I.E.L.D.
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NICK_FURY_AGENT_OF_S_H_I_E_L_D: CardRecord = CardRecord::new(
    "Nick Fury, Agent of S.H.I.E.L.D.",
    "3aa6fc02-ef76-426b-accd-6c0ef88b2a5e",
    "Marco Turini",
    crate::card::CardRules::unsupported(),
);

// MSH 26 — Night Nurse, Healer of Heroes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHT_NURSE_HEALER_OF_HEROES: CardRecord = CardRecord::new(
    "Night Nurse, Healer of Heroes",
    "3199907b-9f24-4554-942a-8ab5a7701717",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// MSH 27 — Okoye, Dora Milaje Leader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OKOYE_DORA_MILAJE_LEADER: CardRecord = CardRecord::new(
    "Okoye, Dora Milaje Leader",
    "2c89acf5-20f0-4441-b96f-2c5cacd685fb",
    "L.A. Draws",
    crate::card::CardRules::unsupported(),
);

// MSH 28 — Origin of the Avengers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORIGIN_OF_THE_AVENGERS: CardRecord = CardRecord::new(
    "Origin of the Avengers",
    "9b0701e8-e365-425a-b897-92f4df9edcb8",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// MSH 29 — Panther Pounce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PANTHER_POUNCE: CardRecord = CardRecord::new(
    "Panther Pounce",
    "4122ae40-800d-4b6b-93c9-aa503c33f1a9",
    "Le Vuong",
    crate::card::CardRules::unsupported(),
);

// MSH 30 — Patriot, Shield Wielder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATRIOT_SHIELD_WIELDER: CardRecord = CardRecord::new(
    "Patriot, Shield Wielder",
    "1467d7c9-00d8-43f1-b4e7-a279d2b10503",
    "Vlad Petruchik",
    crate::card::CardRules::unsupported(),
);

// MSH 31 — Political Triumph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLITICAL_TRIUMPH: CardRecord = CardRecord::new(
    "Political Triumph",
    "dec3dd36-b8ca-432b-8973-d37c6efc4c1a",
    "Monztre",
    crate::card::CardRules::unsupported(),
);

// MSH 32 — Quake, Agent of S.H.I.E.L.D.
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUAKE_AGENT_OF_S_H_I_E_L_D: CardRecord = CardRecord::new(
    "Quake, Agent of S.H.I.E.L.D.",
    "92dad216-ef8e-4af2-a3c6-1d215721c478",
    "Solan",
    crate::card::CardRules::unsupported(),
);

// MSH 33 — Raft Security Officer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAFT_SECURITY_OFFICER: CardRecord = CardRecord::new(
    "Raft Security Officer",
    "f9b9f9d6-b50c-4b29-80be-284ba773c70b",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// MSH 34 — Red Guardian, Super-Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_GUARDIAN_SUPER_SOLDIER: CardRecord = CardRecord::new(
    "Red Guardian, Super-Soldier",
    "c08f8163-319a-4a31-b15a-93974cacd5b7",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// MSH 35 — The Sentry, Golden Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SENTRY_GOLDEN_GUARDIAN: CardRecord = CardRecord::new(
    "The Sentry, Golden Guardian",
    "3f56c0e7-5b07-48e3-b0ca-5d09ddc8de9a",
    "Alexander Skripnikov",
    crate::card::CardRules::unsupported(),
);

// MSH 36 — S.H.I.E.L.D. Spy Kit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static S_H_I_E_L_D_SPY_KIT: CardRecord = CardRecord::new(
    "S.H.I.E.L.D. Spy Kit",
    "4938e23f-b5da-493b-9904-af61a3733ba0",
    "Bachzim",
    crate::card::CardRules::unsupported(),
);

// MSH 37 — Super Villain Lockup
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_VILLAIN_LOCKUP: CardRecord = CardRecord::new(
    "Super Villain Lockup",
    "1af5a1ca-3d11-45e3-ba12-c455c5a7fea1",
    "Jurijus Chitrovas",
    crate::card::CardRules::unsupported(),
);

// MSH 38 — Super-Soldier Serum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_SOLDIER_SERUM: CardRecord = CardRecord::new(
    "Super-Soldier Serum",
    "845b0be1-4f85-4a8c-8205-dc85c8cf9a61",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// MSH 39 — Take Up the Shield (reprint)
const TAKE_UP_THE_SHIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::TAKE_UP_THE_SHIELD,
    "5199d708-8b11-461d-9e91-3e8cd70906f7",
    "Greg Staples",
);

// MSH 40 — Wakandan Drone Flock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAKANDAN_DRONE_FLOCK: CardRecord = CardRecord::new(
    "Wakandan Drone Flock",
    "4b12dc1f-2218-4c71-aafa-ea6a26eeb0aa",
    "Sean Vo",
    crate::card::CardRules::unsupported(),
);

// MSH 41 — Web Up (reprint)
const WEB_UP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_spm::WEB_UP,
    "9a1b057b-229c-4f65-ba4e-12dd342238de",
    "Nathaniel Himawan",
);

// MSH 42 — White Widow, Free Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHITE_WIDOW_FREE_AGENT: CardRecord = CardRecord::new(
    "White Widow, Free Agent",
    "57da765f-7d87-42ac-9eb0-a49c296fbbf8",
    "Julia Vasilyeva",
    crate::card::CardRules::unsupported(),
);

// MSH 43 — Aerial Doombot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AERIAL_DOOMBOT: CardRecord = CardRecord::new(
    "Aerial Doombot",
    "e727ec1c-dc3b-4f1a-8a62-18549f118b89",
    "Michael MacRae",
    crate::card::CardRules::unsupported(),
);

// MSH 44 — A.I.M. Scientists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static A_I_M_SCIENTISTS: CardRecord = CardRecord::new(
    "A.I.M. Scientists",
    "ab96b656-100e-491a-a8c9-94dbb9482c4d",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// MSH 45 — Atlantean Cavalry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATLANTEAN_CAVALRY: CardRecord = CardRecord::new(
    "Atlantean Cavalry",
    "1319bf54-8705-4cc4-8d08-40f05fd09837",
    "Eglė Mosakaitė",
    crate::card::CardRules::unsupported(),
);

// MSH 46 — Atlantis Attacks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATLANTIS_ATTACKS: CardRecord = CardRecord::new(
    "Atlantis Attacks",
    "40bc4380-055d-4913-93cb-280c9c1d1a87",
    "Alexander Skripnikov",
    crate::card::CardRules::unsupported(),
);

// MSH 47 — Attuma, Atlantean Warlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATTUMA_ATLANTEAN_WARLORD: CardRecord = CardRecord::new(
    "Attuma, Atlantean Warlord",
    "c3d5add1-0d0e-414b-a964-8da326472d35",
    "Nanna Marie Steffensen",
    crate::card::CardRules::unsupported(),
);

// MSH 48 — Bold Biochemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOLD_BIOCHEMIST: CardRecord = CardRecord::new(
    "Bold Biochemist",
    "10644127-3da0-484e-9afc-de26d9c34390",
    "Marco Turini",
    crate::card::CardRules::unsupported(),
);

// MSH 49 — Bruce Banner // The Incredible Hulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRUCE_BANNER: CardRecord = CardRecord::new(
    "Bruce Banner // The Incredible Hulk",
    "e0dbbdcf-84e1-494f-8b8c-0a094f603fa9",
    "Tommy Arnold",
    crate::card::CardRules::unsupported(),
);

// MSH 50 — Depower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEPOWER: CardRecord = CardRecord::new(
    "Depower",
    "36e9a6e9-1f9d-4860-97ee-f01e66f8eb4d",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// MSH 51 — Echo, Perceptive Prodigy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ECHO_PERCEPTIVE_PRODIGY: CardRecord = CardRecord::new(
    "Echo, Perceptive Prodigy",
    "cd9b9be6-0143-467c-8a2a-937ecafe0473",
    "Jurijus Chitrovas",
    crate::card::CardRules::unsupported(),
);

// MSH 52 — Falcon, Winged Wonder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALCON_WINGED_WONDER: CardRecord = CardRecord::new(
    "Falcon, Winged Wonder",
    "8dc209ed-0d4c-4d0c-90e8-04cadc3d4c3d",
    "Vilhelmas Banys",
    crate::card::CardRules::unsupported(),
);

// MSH 53 — Falcon's Wing Harness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALCON_S_WING_HARNESS: CardRecord = CardRecord::new(
    "Falcon's Wing Harness",
    "20e93f86-9e20-4e08-9bf7-ae6ebebf6876",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// MSH 54 — Frozen in Ice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROZEN_IN_ICE: CardRecord = CardRecord::new(
    "Frozen in Ice",
    "a990260b-39b7-4799-930a-bf9ac208d9ed",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// MSH 55 — Futurist Forge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUTURIST_FORGE: CardRecord = CardRecord::new(
    "Futurist Forge",
    "a50feebf-c660-43f1-9fae-75294703b346",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 56 — Giant-Sized Flying Ant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_SIZED_FLYING_ANT: CardRecord = CardRecord::new(
    "Giant-Sized Flying Ant",
    "da529bb3-725b-41aa-ac34-b4117ff5d95b",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// MSH 57 — Hydraulic Helper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDRAULIC_HELPER: CardRecord = CardRecord::new(
    "Hydraulic Helper",
    "06d8a2a0-775b-4813-973f-8b15612b38d1",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// MSH 58 — I Am Iron Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static I_AM_IRON_MAN: CardRecord = CardRecord::new(
    "I Am Iron Man",
    "9c401abb-5978-41c0-962b-0432f9433929",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// MSH 59 — Iron Lad, Diverging Destiny
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_LAD_DIVERGING_DESTINY: CardRecord = CardRecord::new(
    "Iron Lad, Diverging Destiny",
    "355e7197-2f20-43b6-9305-73c4e1fd4a3c",
    "Erikas Perl",
    crate::card::CardRules::unsupported(),
);

// MSH 60 — Ironheart, Clever Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRONHEART_CLEVER_CHAMPION: CardRecord = CardRecord::new(
    "Ironheart, Clever Champion",
    "395e477a-861f-4661-b329-6c1ad5343ed5",
    "Julia Vasilyeva",
    crate::card::CardRules::unsupported(),
);

// MSH 61 — Justice, Vance Astrovik
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUSTICE_VANCE_ASTROVIK: CardRecord = CardRecord::new(
    "Justice, Vance Astrovik",
    "f1448731-d50e-4b9c-8eb2-3d85c21516c6",
    "Berto Martinez",
    crate::card::CardRules::unsupported(),
);

// MSH 62 — Kang the Conqueror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KANG_THE_CONQUEROR: CardRecord = CardRecord::new(
    "Kang the Conqueror",
    "79747b45-fd7b-4023-9a9c-4d9dab2429fe",
    "Peter Scanlan",
    crate::card::CardRules::unsupported(),
);

// MSH 63 — Kid Loki
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KID_LOKI: CardRecord = CardRecord::new(
    "Kid Loki",
    "5c9ff69e-6489-49f3-b401-64856f0b7c11",
    "Mintautas Šukys",
    crate::card::CardRules::unsupported(),
);

// MSH 64 — Leader, Super-Genius
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEADER_SUPER_GENIUS: CardRecord = CardRecord::new(
    "Leader, Super-Genius",
    "2c8aab8d-2dfe-49c8-9aa8-536b0587b467",
    "Anthony Devine",
    crate::card::CardRules::unsupported(),
);

// MSH 65 — Loki, God of Mischief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOKI_GOD_OF_MISCHIEF: CardRecord = CardRecord::new(
    "Loki, God of Mischief",
    "236c437f-ee9d-4145-a4db-b665908089cf",
    "Vilhelmas Banys",
    crate::card::CardRules::unsupported(),
);

// MSH 66 — Mister Fantastic, Reed Richards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTER_FANTASTIC_REED_RICHARDS: CardRecord = CardRecord::new(
    "Mister Fantastic, Reed Richards",
    "17ef068b-61fc-443d-97b0-1e41f2622425",
    "Rimas Valeikis",
    crate::card::CardRules::unsupported(),
);

// MSH 67 — Ms. Marvel, Kamala Khan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MS_MARVEL_KAMALA_KHAN: CardRecord = CardRecord::new(
    "Ms. Marvel, Kamala Khan",
    "9dd2d627-10fc-4045-8545-03bcf75e60ca",
    "Smirtouille",
    crate::card::CardRules::unsupported(),
);

// MSH 68 — Multiversal Incursion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MULTIVERSAL_INCURSION: CardRecord = CardRecord::new(
    "Multiversal Incursion",
    "8a505f6f-933c-4644-838f-897f5e4133b0",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// MSH 69 — Namor the Sub-Mariner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAMOR_THE_SUB_MARINER: CardRecord = CardRecord::new(
    "Namor the Sub-Mariner",
    "7aaefcf9-fbe1-4767-92a5-09825761d116",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// MSH 70 — Pym Particles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYM_PARTICLES: CardRecord = CardRecord::new(
    "Pym Particles",
    "928dfa54-1ead-4eff-a538-36cb94f05b78",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// MSH 71 — Rewrite History
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REWRITE_HISTORY: CardRecord = CardRecord::new(
    "Rewrite History",
    "91fdf444-3f41-4b8b-b9f5-f3ae3d903649",
    "Allen Douglas",
    crate::card::CardRules::unsupported(),
);

// MSH 72 — Secret Invasion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECRET_INVASION: CardRecord = CardRecord::new(
    "Secret Invasion",
    "e361b2f4-cd2f-44e9-a56d-c1d6b5ae742d",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// MSH 73 — S.H.I.E.L.D. Deployment Drone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static S_H_I_E_L_D_DEPLOYMENT_DRONE: CardRecord = CardRecord::new(
    "S.H.I.E.L.D. Deployment Drone",
    "c3d0f02f-dfaf-47b6-8053-514417f4dfe2",
    "Paulius Daščioras",
    crate::card::CardRules::unsupported(),
);

// MSH 74 — S.H.I.E.L.D. Flying Car
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static S_H_I_E_L_D_FLYING_CAR: CardRecord = CardRecord::new(
    "S.H.I.E.L.D. Flying Car",
    "8e651410-b829-4468-b866-a9dd15f909ad",
    "Paulius Daščioras",
    crate::card::CardRules::unsupported(),
);

// MSH 75 — Shuri, Wakandan Inventor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHURI_WAKANDAN_INVENTOR: CardRecord = CardRecord::new(
    "Shuri, Wakandan Inventor",
    "65005522-555e-4479-939c-be16e0262f6f",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// MSH 76 — Stature, Size Shifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STATURE_SIZE_SHIFTER: CardRecord = CardRecord::new(
    "Stature, Size Shifter",
    "fe692959-64ca-4065-9f9e-1abe590e3d0f",
    "Mintautas Šukys",
    crate::card::CardRules::unsupported(),
);

// MSH 77 — Super Intelligence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_INTELLIGENCE: CardRecord = CardRecord::new(
    "Super Intelligence",
    "f26613a1-b4ad-4c7f-8dd9-8f82de995f0c",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// MSH 78 — Super Suit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_SUIT: CardRecord = CardRecord::new(
    "Super Suit",
    "e87f3a26-c2e5-47c7-b19d-bc01cb794805",
    "Smirtouille",
    crate::card::CardRules::unsupported(),
);

// MSH 79 — Thirst for Knowledge (reprint)
const THIRST_FOR_KNOWLEDGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mrd::THIRST_FOR_KNOWLEDGE,
    "5f4f77e5-0019-4a5f-bae6-136121946929",
    "Greg Smallwood",
);

// MSH 80 — Tony Stark // The Invincible Iron Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TONY_STARK: CardRecord = CardRecord::new(
    "Tony Stark // The Invincible Iron Man",
    "4cea42fd-035e-4b8f-8b1d-ff363b694f14",
    "Alexander Lozano",
    crate::card::CardRules::unsupported(),
);

// MSH 81 — Trickster's Stratagem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICKSTER_S_STRATAGEM: CardRecord = CardRecord::new(
    "Trickster's Stratagem",
    "620376d0-dc0c-405f-8121-eb36d9b4f4c2",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// MSH 82 — We Say Thee Nay!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WE_SAY_THEE_NAY: CardRecord = CardRecord::new(
    "We Say Thee Nay!",
    "13b70321-75bd-4d44-b9f6-5f062a5dda0f",
    "Mateus Manhanini",
    crate::card::CardRules::unsupported(),
);

// MSH 83 — Wiccan, Rising Magician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICCAN_RISING_MAGICIAN: CardRecord = CardRecord::new(
    "Wiccan, Rising Magician",
    "5d27ddec-716d-47e5-ac48-365e91b88ff1",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// MSH 84 — The Wondrous Wasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_WONDROUS_WASP: CardRecord = CardRecord::new(
    "The Wondrous Wasp",
    "484e80a4-1d5f-4c27-98a8-ee2ef9d6cbdc",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// MSH 85 — Agents of HYDRA
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENTS_OF_HYDRA: CardRecord = CardRecord::new(
    "Agents of HYDRA",
    "857fef2e-df1f-4ec6-a262-f6fa52389cf9",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// MSH 86 — Arnim Zola, Bio-Fanatic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARNIM_ZOLA_BIO_FANATIC: CardRecord = CardRecord::new(
    "Arnim Zola, Bio-Fanatic",
    "07c70df6-b064-424a-852e-201b312a5b54",
    "Immanuela Crovius",
    crate::card::CardRules::unsupported(),
);

// MSH 87 — Baron Helmut Zemo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARON_HELMUT_ZEMO: CardRecord = CardRecord::new(
    "Baron Helmut Zemo",
    "c2aadc25-7755-4bc8-a8af-b01d27eec364",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// MSH 88 — Baron Strucker, HYDRA Overlord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARON_STRUCKER_HYDRA_OVERLORD: CardRecord = CardRecord::new(
    "Baron Strucker, HYDRA Overlord",
    "eaf2251a-ffaf-4055-9474-7e3d08d89609",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// MSH 89 — Black Widow, Super Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_WIDOW_SUPER_SPY: CardRecord = CardRecord::new(
    "Black Widow, Super Spy",
    "63ce0909-7d7d-410d-ab6c-c87fa3e23877",
    "Dan Brereton",
    crate::card::CardRules::unsupported(),
);

// MSH 90 — Construct a Cosmic Cube
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSTRUCT_A_COSMIC_CUBE: CardRecord = CardRecord::new(
    "Construct a Cosmic Cube",
    "448de757-ac16-4529-b851-1a1331b821a5",
    "Eugene Maslovski",
    crate::card::CardRules::unsupported(),
);

// MSH 91 — Crossbones, Malicious Mercenary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROSSBONES_MALICIOUS_MERCENARY: CardRecord = CardRecord::new(
    "Crossbones, Malicious Mercenary",
    "1576148a-2371-49ba-8eef-0bc2ec3dcaf3",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// MSH 92 — Cruel Alliance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUEL_ALLIANCE: CardRecord = CardRecord::new(
    "Cruel Alliance",
    "d895d5a1-d382-438b-8551-e142bb5142af",
    "Vilhelmas Banys",
    crate::card::CardRules::unsupported(),
);

// MSH 93 — Dark Deed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_DEED: CardRecord = CardRecord::new(
    "Dark Deed",
    "49e36cac-3999-40b3-91b3-85af4fded679",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

// MSH 94 — Decoy Ploy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECOY_PLOY: CardRecord = CardRecord::new(
    "Decoy Ploy",
    "d8719b74-48ef-4f68-b59a-949edd644ddc",
    "Le Vuong",
    crate::card::CardRules::unsupported(),
);

// MSH 95 — Doctor Doom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOCTOR_DOOM: CardRecord = CardRecord::new(
    "Doctor Doom",
    "9b1f213a-e1d4-4a0f-954b-c83915698d98",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// MSH 96 — Doom Reigns Supreme
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOOM_REIGNS_SUPREME: CardRecord = CardRecord::new(
    "Doom Reigns Supreme",
    "9b99894b-c774-45c2-9ee9-4d6a8e7522f5",
    "Alexander Gering",
    crate::card::CardRules::unsupported(),
);

// MSH 97 — Elektra, Daughter of the Hand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEKTRA_DAUGHTER_OF_THE_HAND: CardRecord = CardRecord::new(
    "Elektra, Daughter of the Hand",
    "ac3e586c-d654-4631-beda-a5e29cf04717",
    "Bastien L. Deharme",
    crate::card::CardRules::unsupported(),
);

// MSH 98 — Grim Reaper, Lethal Legionnaire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIM_REAPER_LETHAL_LEGIONNAIRE: CardRecord = CardRecord::new(
    "Grim Reaper, Lethal Legionnaire",
    "4b1cedfe-4712-4602-a8a8-112bd54c9938",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// MSH 99 — Hour of Defeat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOUR_OF_DEFEAT: CardRecord = CardRecord::new(
    "Hour of Defeat",
    "9e0034dd-396e-46af-b931-0daa25da4406",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// MSH 100 — HYDRA Infiltration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDRA_INFILTRATION: CardRecord = CardRecord::new(
    "HYDRA Infiltration",
    "c2e446e1-e384-4d5f-8099-544f78c08510",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// MSH 101 — HYDRA Troopers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDRA_TROOPERS: CardRecord = CardRecord::new(
    "HYDRA Troopers",
    "40c202f1-6e0d-42f4-a41e-e0be3362d585",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// MSH 102 — Kingpin's Enforcers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINGPIN_S_ENFORCERS: CardRecord = CardRecord::new(
    "Kingpin's Enforcers",
    "64056edb-6de2-4694-b771-f35542c25771",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// MSH 103 — Klaw, Sonic Subjugator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KLAW_SONIC_SUBJUGATOR: CardRecord = CardRecord::new(
    "Klaw, Sonic Subjugator",
    "c79a86f8-24e9-49a2-8b1c-72a72fed1985",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// MSH 104 — Madame Masque
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADAME_MASQUE: CardRecord = CardRecord::new(
    "Madame Masque",
    "f54197b8-6279-43ee-9340-69bd22cf3775",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// MSH 105 — The Masters of Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MASTERS_OF_EVIL: CardRecord = CardRecord::new(
    "The Masters of Evil",
    "65ba4439-3282-4179-85b9-67a25e2e5d24",
    "Bastien L. Deharme",
    crate::card::CardRules::unsupported(),
);

// MSH 106 — M.O.D.O.K.
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static M_O_D_O_K: CardRecord = CardRecord::new(
    "M.O.D.O.K.",
    "38e87542-50f7-4812-9338-84e4b9b7bb44",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// MSH 107 — Moonstone, Harsh Mistress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONSTONE_HARSH_MISTRESS: CardRecord = CardRecord::new(
    "Moonstone, Harsh Mistress",
    "1a1b2bd0-e17d-4b34-a8ee-7c913a6bc945",
    "Grace Zhu",
    crate::card::CardRules::unsupported(),
);

// MSH 108 — Ninja of the Hand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NINJA_OF_THE_HAND: CardRecord = CardRecord::new(
    "Ninja of the Hand",
    "6b8116d8-2cc6-449b-a8b4-8a5166553497",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// MSH 109 — Project Deathlok Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROJECT_DEATHLOK_SOLDIER: CardRecord = CardRecord::new(
    "Project Deathlok Soldier",
    "5c21a120-4ec7-46ec-974a-2204edb92abb",
    "InHyuk Lee",
    crate::card::CardRules::unsupported(),
);

// MSH 110 — Red Room Recruit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_ROOM_RECRUIT: CardRecord = CardRecord::new(
    "Red Room Recruit",
    "f52a5ba3-a618-4f52-9d33-ba85345eb627",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MSH 111 — Robot Domination
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROBOT_DOMINATION: CardRecord = CardRecord::new(
    "Robot Domination",
    "b26bb968-6612-43fe-9147-a3d4786cbc20",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MSH 112 — Ronin, Shadow Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RONIN_SHADOW_STALKER: CardRecord = CardRecord::new(
    "Ronin, Shadow Stalker",
    "f74b9794-946e-4ddc-93b2-5a321fc51fd0",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// MSH 113 — Roxxon Brutes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROXXON_BRUTES: CardRecord = CardRecord::new(
    "Roxxon Brutes",
    "66550490-74e1-4bf1-8741-1266dfab3a03",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// MSH 114 — Stolen Stark Tech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOLEN_STARK_TECH: CardRecord = CardRecord::new(
    "Stolen Stark Tech",
    "db4557ad-4141-4dae-870e-a587506f4914",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

// MSH 115 — Super-Skrull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_SKRULL: CardRecord = CardRecord::new(
    "Super-Skrull",
    "11fc8221-756a-4919-9272-38793e9c1ad9",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// MSH 116 — Swordsman, Sharp Scoundrel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWORDSMAN_SHARP_SCOUNDREL: CardRecord = CardRecord::new(
    "Swordsman, Sharp Scoundrel",
    "6572aafe-18ed-4182-92e9-25f003f5fe3d",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// MSH 117 — Thunderbolts Conspiracy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERBOLTS_CONSPIRACY: CardRecord = CardRecord::new(
    "Thunderbolts Conspiracy",
    "f498c1a4-54d2-4e87-952f-8cf7e408930c",
    "Lucio Parrillo",
    crate::card::CardRules::unsupported(),
);

// MSH 118 — Too Evil to Stay Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOO_EVIL_TO_STAY_DEAD: CardRecord = CardRecord::new(
    "Too Evil to Stay Dead",
    "f471e9ce-73bb-4090-98f2-f591c7cf4efe",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// MSH 119 — Unliving Legionnaire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNLIVING_LEGIONNAIRE: CardRecord = CardRecord::new(
    "Unliving Legionnaire",
    "ce08f4bb-7da3-4199-8b90-fcdb29e84e98",
    "Björn Barends",
    crate::card::CardRules::unsupported(),
);

// MSH 120 — Visions of Villainy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISIONS_OF_VILLAINY: CardRecord = CardRecord::new(
    "Visions of Villainy",
    "f8f1e4f9-7415-437f-b694-ecbdd76db114",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// MSH 121 — Whiplash, Vengeful Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIPLASH_VENGEFUL_ENGINEER: CardRecord = CardRecord::new(
    "Whiplash, Vengeful Engineer",
    "6f6e6767-4d20-469c-9f4b-7286b8cc1979",
    "Alexander Gering",
    crate::card::CardRules::unsupported(),
);

// MSH 122 — Widow's Bite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIDOW_S_BITE: CardRecord = CardRecord::new(
    "Widow's Bite",
    "dbee18af-9ade-4251-81a1-f6e7ffbf480f",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MSH 123 — Yellowjacket, Heartless Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YELLOWJACKET_HEARTLESS_MARAUDER: CardRecord = CardRecord::new(
    "Yellowjacket, Heartless Marauder",
    "d8f91212-2ff5-4024-8f14-86bb5c4a9754",
    "Alexander Skripnikov",
    crate::card::CardRules::unsupported(),
);

// MSH 124 — Avengers Disassembled
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGERS_DISASSEMBLED: CardRecord = CardRecord::new(
    "Avengers Disassembled",
    "72d3e750-870b-497d-80d7-e3df097db554",
    "David Szabo",
    crate::card::CardRules::unsupported(),
);

// MSH 125 — Blazing Crescendo (reprint)
const BLAZING_CRESCENDO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_one::BLAZING_CRESCENDO,
    "8447c3d6-8c65-4d3c-81c4-609801602d06",
    "Pauline Voss",
);

// MSH 126 — Crimson Operative
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIMSON_OPERATIVE: CardRecord = CardRecord::new(
    "Crimson Operative",
    "c4bc077c-f220-47a5-aaf4-5324ca23d0c5",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// MSH 127 — Death to Our Enemies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_TO_OUR_ENEMIES: CardRecord = CardRecord::new(
    "Death to Our Enemies",
    "f2a8f518-c0b5-4e15-aab2-49b5ef29fb41",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// MSH 128 — Evil's Thrall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVIL_S_THRALL: CardRecord = CardRecord::new(
    "Evil's Thrall",
    "310e30cd-b8c3-40ea-9d61-57c5c5fc2a0b",
    "Mintautas Šukys",
    crate::card::CardRules::unsupported(),
);

// MSH 129 — Fin Fang Foom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIN_FANG_FOOM: CardRecord = CardRecord::new(
    "Fin Fang Foom",
    "7777fac1-7bf5-4454-960d-26ec3183e392",
    "Filipe Pagliuso",
    crate::card::CardRules::unsupported(),
);

// MSH 130 — Hawkeye, Master Marksman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAWKEYE_MASTER_MARKSMAN: CardRecord = CardRecord::new(
    "Hawkeye, Master Marksman",
    "9991b684-0ae0-4aa4-8f22-a9c473f5d69c",
    "Bachzim",
    crate::card::CardRules::unsupported(),
);

// MSH 131 — Hawkeye, Young Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAWKEYE_YOUNG_AVENGER: CardRecord = CardRecord::new(
    "Hawkeye, Young Avenger",
    "3cebccbb-dde2-41fe-bfdc-0a46ee185749",
    "Darius Zablockis",
    crate::card::CardRules::unsupported(),
);

// MSH 132 — Hawkeye's Bow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAWKEYE_S_BOW: CardRecord = CardRecord::new(
    "Hawkeye's Bow",
    "485be72b-d784-4886-bb3a-48cff8f781c6",
    "Marc Aspinall",
    crate::card::CardRules::unsupported(),
);

// MSH 133 — Hex Magic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEX_MAGIC: CardRecord = CardRecord::new(
    "Hex Magic",
    "259bed47-1950-43ae-8efd-3009537529a8",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// MSH 134 — Hire a Crew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIRE_A_CREW: CardRecord = CardRecord::new(
    "Hire a Crew",
    "50a6aa80-79a3-46f3-9cc1-5bf663b64976",
    "Lordigan",
    crate::card::CardRules::unsupported(),
);

// MSH 135 — HULK SMASH!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULK_SMASH: CardRecord = CardRecord::new(
    "HULK SMASH!",
    "374ffb3e-0753-4682-936a-ae6921ace475",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// MSH 136 — Human Torch, Johnny Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUMAN_TORCH_JOHNNY_STORM: CardRecord = CardRecord::new(
    "Human Torch, Johnny Storm",
    "8f8659f6-a793-4edc-8401-d9126840c1c2",
    "Alexander Skripnikov",
    crate::card::CardRules::unsupported(),
);

// MSH 137 — HYDRA Assault Robot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDRA_ASSAULT_ROBOT: CardRecord = CardRecord::new(
    "HYDRA Assault Robot",
    "e3b23fff-7d3b-4341-b556-82136f8c113b",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// MSH 138 — Iron Fist, Living Weapon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_FIST_LIVING_WEAPON: CardRecord = CardRecord::new(
    "Iron Fist, Living Weapon",
    "5cc2d948-ac8b-4466-b604-3fabc0ab6bb9",
    "Erikas Perl",
    crate::card::CardRules::unsupported(),
);

// MSH 139 — Jessica Jones, Private Eye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESSICA_JONES_PRIVATE_EYE: CardRecord = CardRecord::new(
    "Jessica Jones, Private Eye",
    "b48278fd-1d95-4ad6-9a51-f1c5a2ab9f4b",
    "Julia Vasilyeva",
    crate::card::CardRules::unsupported(),
);

// MSH 140 — K'un-Lun Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static K_UN_LUN_WARRIOR: CardRecord = CardRecord::new(
    "K'un-Lun Warrior",
    "e6e8359d-12be-49be-8cbe-6a3b8506a9e6",
    "Smirtouille",
    crate::card::CardRules::unsupported(),
);

// MSH 141 — Kree Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KREE_SENTINEL: CardRecord = CardRecord::new(
    "Kree Sentinel",
    "ba5e441d-d036-4080-9571-6cf76df7b452",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// MSH 142 — Lightning Strike (reprint)
const LIGHTNING_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::LIGHTNING_STRIKE,
    "88b13bc0-da54-4c3b-917c-7c8345a329f5",
    "Toni Infante",
);

// MSH 143 — Loki Laufeyson
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOKI_LAUFEYSON: CardRecord = CardRecord::new(
    "Loki Laufeyson",
    "53b795a7-11f8-423a-b021-f811007fc82f",
    "Julia Vasilyeva",
    crate::card::CardRules::unsupported(),
);

// MSH 144 — Machinesmith Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MACHINESMITH_AUTOMATON: CardRecord = CardRecord::new(
    "Machinesmith Automaton",
    "802e663e-9529-46da-8af4-eb9ca07971b1",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// MSH 145 — Misty Knight, Hero for Hire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTY_KNIGHT_HERO_FOR_HIRE: CardRecord = CardRecord::new(
    "Misty Knight, Hero for Hire",
    "6b963642-a103-44a0-9eb7-9c3fdde181b7",
    "Eglė Mosakaitė",
    crate::card::CardRules::unsupported(),
);

// MSH 146 — Mjölnir, Hammer of Thor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MJOLNIR_HAMMER_OF_THOR: CardRecord = CardRecord::new(
    "Mjölnir, Hammer of Thor",
    "e0c7f566-5351-44e3-a346-b84b0eb10209",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// MSH 147 — Photon Blast Barrage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHOTON_BLAST_BARRAGE: CardRecord = CardRecord::new(
    "Photon Blast Barrage",
    "f85a77d2-e11e-44bc-a1e7-d783cd49d714",
    "Immanuela Crovius",
    crate::card::CardRules::unsupported(),
);

// MSH 148 — Quicksilver, Brash Blur
// Audit: unsupported — Needs the Power-up once-per-object limit and entered-this-turn cost reduction.
pub(in crate::card::sets) static QUICKSILVER_BRASH_BLUR: CardRecord = CardRecord::new(
    "Quicksilver, Brash Blur",
    "2d5819ca-165d-4f4c-9500-3ac206994880",
    "Michael MacRae",
    CardRules::unsupported(),
);

// MSH 149 — Red Hulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_HULK: CardRecord = CardRecord::new(
    "Red Hulk",
    "e25468e2-17c6-47b2-8eb6-fcb8ae6f4c17",
    "Vilhelmas Banys",
    crate::card::CardRules::unsupported(),
);

// MSH 150 — Repulsor Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPULSOR_BLAST: CardRecord = CardRecord::new(
    "Repulsor Blast",
    "837265b0-fc15-4d96-9d6b-fd1c78534262",
    "Peter Scanlan",
    crate::card::CardRules::unsupported(),
);

// MSH 151 — The Scarlet Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SCARLET_WITCH: CardRecord = CardRecord::new(
    "The Scarlet Witch",
    "407e8993-e56d-477d-ab85-d10a2522eab3",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// MSH 152 — Speed, Young Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPEED_YOUNG_AVENGER: CardRecord = CardRecord::new(
    "Speed, Young Avenger",
    "d15d187b-dc5d-482d-a79a-b9b98193ad7a",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// MSH 153 — Stark Industries Executive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARK_INDUSTRIES_EXECUTIVE: CardRecord = CardRecord::new(
    "Stark Industries Executive",
    "709ffd07-706c-471f-b74d-4637afa11686",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// MSH 154 — Super Speed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_SPEED: CardRecord = CardRecord::new(
    "Super Speed",
    "2aa2e596-e5e0-4c60-8eea-14adda1cdaae",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// MSH 155 — Team Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEAM_TACTICS: CardRecord = CardRecord::new(
    "Team Tactics",
    "db1c7a71-3d01-4f2a-9b25-2a19bb0d1a56",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// MSH 156 — Thor, God of Thunder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOR_GOD_OF_THUNDER: CardRecord = CardRecord::new(
    "Thor, God of Thunder",
    "cddd314c-c271-475a-b076-01a8599c8015",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// MSH 157 — Truck Toss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRUCK_TOSS: CardRecord = CardRecord::new(
    "Truck Toss",
    "60f02fbf-1416-48a3-bf94-f27509f6983a",
    "Alexander Skripnikov",
    crate::card::CardRules::unsupported(),
);

// MSH 158 — Vision of Love
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISION_OF_LOVE: CardRecord = CardRecord::new(
    "Vision of Love",
    "5604cee1-a2ea-48ed-88f4-6878bd053fc8",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

// MSH 159 — Volcanic Villain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLCANIC_VILLAIN: CardRecord = CardRecord::new(
    "Volcanic Villain",
    "4f48ba40-9934-40fd-a251-e15a68e772d2",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// MSH 160 — Wonder Man, Hollywood Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WONDER_MAN_HOLLYWOOD_HERO: CardRecord = CardRecord::new(
    "Wonder Man, Hollywood Hero",
    "1ee52395-9523-4da1-9108-8ab79229fc7a",
    "Nanna Marie Steffensen",
    crate::card::CardRules::unsupported(),
);

// MSH 161 — Ant-Man's Army
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANT_MAN_S_ARMY: CardRecord = CardRecord::new(
    "Ant-Man's Army",
    "61a89566-06c1-404f-8217-e5276e2dc5a2",
    "Bachzim",
    crate::card::CardRules::unsupported(),
);

// MSH 162 — Call Damage Control
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_DAMAGE_CONTROL: CardRecord = CardRecord::new(
    "Call Damage Control",
    "de1ed886-c33b-4cfd-8442-944cec654a8b",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// MSH 163 — Claim the Kingdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLAIM_THE_KINGDOM: CardRecord = CardRecord::new(
    "Claim the Kingdom",
    "cf13bfb1-5b44-4363-8de9-ece234233870",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// MSH 164 — Doc Samson, Super Psychiatrist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOC_SAMSON_SUPER_PSYCHIATRIST: CardRecord = CardRecord::new(
    "Doc Samson, Super Psychiatrist",
    "19dc5fcc-d05d-41a0-84c5-2dec996f3e4f",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// MSH 165 — Earth's Mightiest Heroes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTH_S_MIGHTIEST_HEROES: CardRecord = CardRecord::new(
    "Earth's Mightiest Heroes",
    "b38b9bd6-1dd7-4bbc-8a82-ce391c1172e1",
    "Steve Morris",
    crate::card::CardRules::unsupported(),
);

// MSH 166 — Epic Fight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EPIC_FIGHT: CardRecord = CardRecord::new(
    "Epic Fight",
    "d521b80b-4b0b-4d69-bd95-4c83feaf2145",
    "Wei Guan",
    crate::card::CardRules::unsupported(),
);

// MSH 167 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "fd1f95bf-48ea-455a-8a6c-0249b11c8900",
    "Andreia Ugrai",
);

// MSH 168 — Go Nuts!
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GO_NUTS: CardRecord = CardRecord::new(
    "Go Nuts!",
    "152a7b5b-2d95-45d3-8fd9-0ca1d5a79f8b",
    "Ignatius Budi",
    crate::card::CardRules::unsupported(),
);

// MSH 169 — Guerrilla Gorilla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUERRILLA_GORILLA: CardRecord = CardRecord::new(
    "Guerrilla Gorilla",
    "af6bc9d2-5783-489a-8963-af65f54f4f17",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// MSH 170 — Hellcat, Undying Vigilante
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELLCAT_UNDYING_VIGILANTE: CardRecord = CardRecord::new(
    "Hellcat, Undying Vigilante",
    "d7922c5f-d6ee-4b62-a537-3be5aa280e10",
    "Solan",
    crate::card::CardRules::unsupported(),
);

// MSH 171 — Hercules, Prince of Power
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HERCULES_PRINCE_OF_POWER: CardRecord = CardRecord::new(
    "Hercules, Prince of Power",
    "d610d9fe-f8ae-473f-9325-e2f28f7e8a69",
    "David Talaski",
    crate::card::CardRules::unsupported(),
);

// MSH 172 — Heroic Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEROIC_FEAST: CardRecord = CardRecord::new(
    "Heroic Feast",
    "32d05c3d-cf03-492e-a956-2e77c36e36c4",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// MSH 173 — Hulkling, Burgeoning Bruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULKLING_BURGEONING_BRUISER: CardRecord = CardRecord::new(
    "Hulkling, Burgeoning Bruiser",
    "9018477f-a67b-4fa4-8661-11ab91fae863",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// MSH 174 — Ka-Zar of the Savage Land
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KA_ZAR_OF_THE_SAVAGE_LAND: CardRecord = CardRecord::new(
    "Ka-Zar of the Savage Land",
    "caf70fc5-87a0-4b4d-bedd-74dd09569bed",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MSH 175 — Knight of Wundagore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_WUNDAGORE: CardRecord = CardRecord::new(
    "Knight of Wundagore",
    "f3003766-9383-44e0-8b51-35ccbed137ed",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// MSH 176 — Mister Hyde, Monster Within
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTER_HYDE_MONSTER_WITHIN: CardRecord = CardRecord::new(
    "Mister Hyde, Monster Within",
    "3cf30476-ed92-4dc2-86b7-2b5fd60a2de7",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// MSH 177 — Mole Man, Moloid Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLE_MAN_MOLOID_MASTER: CardRecord = CardRecord::new(
    "Mole Man, Moloid Master",
    "1b406cc5-75c9-430a-83a2-de0f2a8aeae6",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// MSH 178 — Pet Avengers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PET_AVENGERS: CardRecord = CardRecord::new(
    "Pet Avengers",
    "108a0b92-2134-4776-a2f7-da92050f1b21",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// MSH 179 — Powerful Broker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWERFUL_BROKER: CardRecord = CardRecord::new(
    "Powerful Broker",
    "801f0417-b663-4e28-9a61-9570061654d7",
    "JB Casacop",
    crate::card::CardRules::unsupported(),
);

// MSH 180 — Punishing Punch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUNISHING_PUNCH: CardRecord = CardRecord::new(
    "Punishing Punch",
    "a33a4cb4-1b57-47ca-8e5e-58ff46a6e0ce",
    "Bachzim",
    crate::card::CardRules::unsupported(),
);

// MSH 181 — Rapid Rescue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPID_RESCUE: CardRecord = CardRecord::new(
    "Rapid Rescue",
    "d739e636-681e-4423-84e3-91efd02b2c94",
    "Raymond Bonilla",
    crate::card::CardRules::unsupported(),
);

// MSH 182 — Reptil, Dinomorpher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPTIL_DINOMORPHER: CardRecord = CardRecord::new(
    "Reptil, Dinomorpher",
    "472db3f2-23df-46c1-86c6-f35fbe5ab4e3",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// MSH 183 — Restorative Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTORATIVE_TECHNIQUE: CardRecord = CardRecord::new(
    "Restorative Technique",
    "dbe9102d-b92b-4c32-93f6-5b6dc0fa08e6",
    "Kevin Glint",
    crate::card::CardRules::unsupported(),
);

// MSH 184 — Rick Jones, Destined Sidekick
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RICK_JONES_DESTINED_SIDEKICK: CardRecord = CardRecord::new(
    "Rick Jones, Destined Sidekick",
    "29bffd7c-73be-4185-976b-36a42a4512fe",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// MSH 185 — Savage Land Dinosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVAGE_LAND_DINOSAUR: CardRecord = CardRecord::new(
    "Savage Land Dinosaur",
    "562bd376-3fba-4de0-b9fc-183004b03732",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// MSH 186 — Serpent Specialist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERPENT_SPECIALIST: CardRecord = CardRecord::new(
    "Serpent Specialist",
    "72727c2a-08a9-47bc-a5eb-cda52e21684b",
    "Daniel Landerman",
    crate::card::CardRules::unsupported(),
);

// MSH 187 — Shang-Chi, Master of Kung Fu
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHANG_CHI_MASTER_OF_KUNG_FU: CardRecord = CardRecord::new(
    "Shang-Chi, Master of Kung Fu",
    "2edbb62f-ad45-4422-850b-68dcc18b4c73",
    "Lee Woo-chul",
    crate::card::CardRules::unsupported(),
);

// MSH 188 — She-Hulk, Jade Defender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHE_HULK_JADE_DEFENDER: CardRecord = CardRecord::new(
    "She-Hulk, Jade Defender",
    "d22a8d02-023c-4f71-a771-16b5aa2a05d7",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// MSH 189 — Super Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_STRENGTH: CardRecord = CardRecord::new(
    "Super Strength",
    "1d24d200-63d4-44cf-8d3a-6c6ece5e16ff",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// MSH 190 — The Thing, Ben Grimm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_THING_BEN_GRIMM: CardRecord = CardRecord::new(
    "The Thing, Ben Grimm",
    "3f1933a8-046f-471a-afcf-cfb08ca0d239",
    "Gintas Galvanauskas",
    crate::card::CardRules::unsupported(),
);

// MSH 191 — Tigra, Feline Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIGRA_FELINE_FURY: CardRecord = CardRecord::new(
    "Tigra, Feline Fury",
    "73c11aea-4fa4-438d-b886-2fafa60c82f9",
    "Ben Harvey",
    crate::card::CardRules::unsupported(),
);

// MSH 192 — Training Regimen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINING_REGIMEN: CardRecord = CardRecord::new(
    "Training Regimen",
    "a27aee6d-1fc3-4307-bb16-b964d723fe2f",
    "Marco Turini",
    crate::card::CardRules::unsupported(),
);

// MSH 193 — The Unbeatable Squirrel Girl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_UNBEATABLE_SQUIRREL_GIRL: CardRecord = CardRecord::new(
    "The Unbeatable Squirrel Girl",
    "d8b94ebb-0b3a-4e68-8c0c-3a61ac32f3ef",
    "Kim Dingwall",
    crate::card::CardRules::unsupported(),
);

// MSH 194 — Undercover Skrull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERCOVER_SKRULL: CardRecord = CardRecord::new(
    "Undercover Skrull",
    "90b9a3c0-3fd6-403e-9d6c-201342fea0ad",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// MSH 195 — Wakandan Royal Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAKANDAN_ROYAL_GUARD: CardRecord = CardRecord::new(
    "Wakandan Royal Guard",
    "9bf0b1e9-32b5-4591-8dd2-6b5f9768c72d",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MSH 196 — White Tiger, Ava Ayala
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHITE_TIGER_AVA_AYALA: CardRecord = CardRecord::new(
    "White Tiger, Ava Ayala",
    "c1589e2e-32a8-48b8-93eb-a9af344e7084",
    "Jennifer Hrabota Lesser",
    crate::card::CardRules::unsupported(),
);

// MSH 197 — World War Hulk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLD_WAR_HULK: CardRecord = CardRecord::new(
    "World War Hulk",
    "9032f05b-5c21-4996-90c1-268dc6dffbaa",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// MSH 198 — Abomination, Terrifying Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABOMINATION_TERRIFYING_TITAN: CardRecord = CardRecord::new(
    "Abomination, Terrifying Titan",
    "2c6ab9bb-dba2-4b5b-a4d9-54735f65ac21",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// MSH 199 — Absorbing Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABSORBING_MAN: CardRecord = CardRecord::new(
    "Absorbing Man",
    "7114ef87-53ff-43e9-864c-9faa455d86ef",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// MSH 200 — Alien Invasion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALIEN_INVASION: CardRecord = CardRecord::new(
    "Alien Invasion",
    "f46a7158-5807-4750-a1f3-853aabb56b99",
    "Björn Barends",
    crate::card::CardRules::unsupported(),
);

// MSH 201 — Ant-Man, Colony Commander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANT_MAN_COLONY_COMMANDER: CardRecord = CardRecord::new(
    "Ant-Man, Colony Commander",
    "26faf2db-ad86-462f-b61f-c1893c9aebbf",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// MSH 202 — Ares, God of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARES_GOD_OF_WAR: CardRecord = CardRecord::new(
    "Ares, God of War",
    "3e121d65-d341-40b9-bebc-1e7d1b83905c",
    "Fesbra",
    crate::card::CardRules::unsupported(),
);

// MSH 203 — Armor Wars
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMOR_WARS: CardRecord = CardRecord::new(
    "Armor Wars",
    "11a13397-1d31-4257-87c2-a757a751c601",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// MSH 204 — The Astonishing Ant-Man
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_ASTONISHING_ANT_MAN: CardRecord = CardRecord::new(
    "The Astonishing Ant-Man",
    "5d98073e-2828-4365-a8d6-f631aac0cca9",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// MSH 205 — Avengers: Under Siege
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGERS_UNDER_SIEGE: CardRecord = CardRecord::new(
    "Avengers: Under Siege",
    "3c379a3f-bde1-4dc1-9843-afcb5f40792f",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// MSH 206 — Beast, Erudite Aerialist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEAST_ERUDITE_AERIALIST: CardRecord = CardRecord::new(
    "Beast, Erudite Aerialist",
    "a92a95d2-9529-417a-b7d5-b4244d7fdca7",
    "Bachzim",
    crate::card::CardRules::unsupported(),
);

// MSH 207 — Black Panther, Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_PANTHER_VANGUARD: CardRecord = CardRecord::new(
    "Black Panther, Vanguard",
    "9e308f77-206b-4c9d-bfba-f4c476ea574a",
    "Aniekan Udofia",
    crate::card::CardRules::unsupported(),
);

// MSH 208 — Black Widow, Double Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_WIDOW_DOUBLE_AGENT: CardRecord = CardRecord::new(
    "Black Widow, Double Agent",
    "aa8dbbb9-36a5-48e0-9e30-aeed0fb5d522",
    "Michael MacRae",
    crate::card::CardRules::unsupported(),
);

// MSH 209 — Bullseye, Death Dealer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BULLSEYE_DEATH_DEALER: CardRecord = CardRecord::new(
    "Bullseye, Death Dealer",
    "fd1f0b5f-5e0e-4da1-ab54-a62db5af3591",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// MSH 210 — Captain America, Living Legend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_AMERICA_LIVING_LEGEND: CardRecord = CardRecord::new(
    "Captain America, Living Legend",
    "6516f292-469d-4092-b099-97c698f373cd",
    "Smirtouille",
    crate::card::CardRules::unsupported(),
);

// MSH 211 — Cloak and Dagger, Entwined
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOAK_AND_DAGGER_ENTWINED: CardRecord = CardRecord::new(
    "Cloak and Dagger, Entwined",
    "fa01d35f-1064-4a7c-9475-4504566df850",
    "Rimas Valeikis",
    crate::card::CardRules::unsupported(),
);

// MSH 212 — The Coming of Galactus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_COMING_OF_GALACTUS: CardRecord = CardRecord::new(
    "The Coming of Galactus",
    "04a38b92-619e-4fe8-b0cb-6fa31f0824ff",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// MSH 213 — Daredevil, Man Without Fear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAREDEVIL_MAN_WITHOUT_FEAR: CardRecord = CardRecord::new(
    "Daredevil, Man Without Fear",
    "14e821bb-55cc-474e-9b63-0ceecc2666c1",
    "Dan Brereton",
    crate::card::CardRules::unsupported(),
);

// MSH 214 — Ghost, Spectral Saboteur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOST_SPECTRAL_SABOTEUR: CardRecord = CardRecord::new(
    "Ghost, Spectral Saboteur",
    "c22bb4d5-6b6f-4c43-b005-48ae4042739d",
    "Lucio Parrillo",
    crate::card::CardRules::unsupported(),
);

// MSH 215 — Hulk, Gamma Goliath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULK_GAMMA_GOLIATH: CardRecord = CardRecord::new(
    "Hulk, Gamma Goliath",
    "682b9f91-18bb-4113-9d5c-a381c191def9",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// MSH 216 — Iron Man, Master of Machines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_MAN_MASTER_OF_MACHINES: CardRecord = CardRecord::new(
    "Iron Man, Master of Machines",
    "8f84ab0a-bf6e-4f28-9da8-998512a224ed",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// MSH 217 — Kang, Temporal Tyrant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KANG_TEMPORAL_TYRANT: CardRecord = CardRecord::new(
    "Kang, Temporal Tyrant",
    "fb96ee86-5139-472a-9c4b-8a8a4280fc7e",
    "David Szabo",
    crate::card::CardRules::unsupported(),
);

// MSH 218 — Killmonger, Scourge of Wakanda
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KILLMONGER_SCOURGE_OF_WAKANDA: CardRecord = CardRecord::new(
    "Killmonger, Scourge of Wakanda",
    "5060aa13-4b33-4b3a-8bdb-dd81308fa3e3",
    "Sean Vo",
    crate::card::CardRules::unsupported(),
);

// MSH 219 — King T'Challa // Black Panther, Hope Enduring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KING_T_CHALLA: CardRecord = CardRecord::new(
    "King T'Challa // Black Panther, Hope Enduring",
    "add7d3ce-aa58-4da0-8c2a-cfd01c3a8975",
    "Aaron J. Riley & Eric Wilkerson",
    crate::card::CardRules::unsupported(),
);

// MSH 220 — The Kingpin of Crime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_KINGPIN_OF_CRIME: CardRecord = CardRecord::new(
    "The Kingpin of Crime",
    "495c08ea-5502-4bfe-aa15-fa85556755ae",
    "Steve Morris",
    crate::card::CardRules::unsupported(),
);

// MSH 221 — Madame Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MADAME_HYDRA: CardRecord = CardRecord::new(
    "Madame Hydra",
    "e94ccedb-1d27-4098-8823-d8d99b30387c",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// MSH 222 — The Mighty Thor, Jane Foster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_MIGHTY_THOR_JANE_FOSTER: CardRecord = CardRecord::new(
    "The Mighty Thor, Jane Foster",
    "082cc8cc-bbea-4ca7-a0e8-da1f865d6626",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// MSH 223 — Moon Girl and Devil Dinosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOON_GIRL_AND_DEVIL_DINOSAUR: CardRecord = CardRecord::new(
    "Moon Girl and Devil Dinosaur",
    "8c6a6d11-45cb-4def-a04b-51b91e1747db",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// MSH 224 — The Ruinous Wrecking Crew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_RUINOUS_WRECKING_CREW: CardRecord = CardRecord::new(
    "The Ruinous Wrecking Crew",
    "4d8c8ceb-84cd-46d2-9230-ab6ca4569334",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// MSH 225 — Scientist Supreme of A.I.M.
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCIENTIST_SUPREME_OF_A_I_M: CardRecord = CardRecord::new(
    "Scientist Supreme of A.I.M.",
    "0473a990-88c4-4921-a492-377d9171318a",
    "Gal Or",
    crate::card::CardRules::unsupported(),
);

// MSH 226 — The Serpent Society
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SERPENT_SOCIETY: CardRecord = CardRecord::new(
    "The Serpent Society",
    "209855ee-d531-4b58-926b-8da171d46619",
    "Rimas Valeikis",
    crate::card::CardRules::unsupported(),
);

// MSH 227 — Speedball, New Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPEEDBALL_NEW_WARRIOR: CardRecord = CardRecord::new(
    "Speedball, New Warrior",
    "e040b456-9853-4e94-9bf8-9374888168bb",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// MSH 228 — Spider-Man, To the Rescue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_MAN_TO_THE_RESCUE: CardRecord = CardRecord::new(
    "Spider-Man, To the Rescue",
    "e5db6968-1ca3-46bd-8cf9-e2c411ab29c1",
    "Anna Podedworna",
    crate::card::CardRules::unsupported(),
);

// MSH 229 — Spider-Woman, Secret Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIDER_WOMAN_SECRET_AGENT: CardRecord = CardRecord::new(
    "Spider-Woman, Secret Agent",
    "a0325cb5-4c43-418a-8f1b-cf5bf29e74d7",
    "Pauline Voss",
    crate::card::CardRules::unsupported(),
);

// MSH 230 — Storm, Windrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_WINDRIDER: CardRecord = CardRecord::new(
    "Storm, Windrider",
    "e90196a9-5a76-42f8-9b40-097d02b47f33",
    "Immanuela Crovius",
    crate::card::CardRules::unsupported(),
);

// MSH 231 — The Super Hero Civil War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SUPER_HERO_CIVIL_WAR: CardRecord = CardRecord::new(
    "The Super Hero Civil War",
    "fbd1f333-5640-41f8-b9dd-1c322ac7724a",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// MSH 232 — Taskmaster, Mercenary Mimic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASKMASTER_MERCENARY_MIMIC: CardRecord = CardRecord::new(
    "Taskmaster, Mercenary Mimic",
    "0d4265fd-cbfa-4e57-89fd-a1d757acfe81",
    "Riccardo Federici",
    crate::card::CardRules::unsupported(),
);

// MSH 233 — Thanos, the Mad Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THANOS_THE_MAD_TITAN: CardRecord = CardRecord::new(
    "Thanos, the Mad Titan",
    "e669c0b2-0011-4feb-9263-f1ecc0a98f18",
    "Björn Barends",
    crate::card::CardRules::unsupported(),
);

// MSH 234 — Thor Odinson
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOR_ODINSON: CardRecord = CardRecord::new(
    "Thor Odinson",
    "0e06f730-ae30-4ef2-88b1-073e02afdb25",
    "Sean Vo",
    crate::card::CardRules::unsupported(),
);

// MSH 235 — Titania, Rugged Rumbler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TITANIA_RUGGED_RUMBLER: CardRecord = CardRecord::new(
    "Titania, Rugged Rumbler",
    "c281e0ee-155b-4022-b921-ebc391535aad",
    "Taurin Clarke",
    crate::card::CardRules::unsupported(),
);

// MSH 236 — U.S.Agent, John Walker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static U_S_AGENT_JOHN_WALKER: CardRecord = CardRecord::new(
    "U.S.Agent, John Walker",
    "35e11911-799c-4d82-9109-3ca27964bef0",
    "Julia Vasilyeva",
    crate::card::CardRules::unsupported(),
);

// MSH 237 — Vision Quest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISION_QUEST: CardRecord = CardRecord::new(
    "Vision Quest",
    "c01afea6-645d-4d4f-bdaa-90794a628bcd",
    "Eglė Mosakaitė",
    crate::card::CardRules::unsupported(),
);

// MSH 238 — War Machine, Legacy of Iron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_MACHINE_LEGACY_OF_IRON: CardRecord = CardRecord::new(
    "War Machine, Legacy of Iron",
    "433a7d97-9c0b-4f5c-85ad-b55342c02d22",
    "Carlos Dattoli",
    crate::card::CardRules::unsupported(),
);

// MSH 239 — Winter Soldier, Icy Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_SOLDIER_ICY_ASSASSIN: CardRecord = CardRecord::new(
    "Winter Soldier, Icy Assassin",
    "ebf71ffc-6e3e-4ca0-a84a-3c1ebd2b64b1",
    "Riccardo Federici",
    crate::card::CardRules::unsupported(),
);

// MSH 240 — Wolverine, Fierce Fighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOLVERINE_FIERCE_FIGHTER: CardRecord = CardRecord::new(
    "Wolverine, Fierce Fighter",
    "c1c7aa22-51b0-45ee-9a8e-5493a1820d8c",
    "Dan Brereton",
    crate::card::CardRules::unsupported(),
);

// MSH 241 — Worlds Within Worlds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDS_WITHIN_WORLDS: CardRecord = CardRecord::new(
    "Worlds Within Worlds",
    "4765e39c-cbf2-4605-9bf1-3baad7d92cfb",
    "Michael MacRae",
    crate::card::CardRules::unsupported(),
);

// MSH 242 — A.I.M. Synthoids
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static A_I_M_SYNTHOIDS: CardRecord = CardRecord::new(
    "A.I.M. Synthoids",
    "f29dee50-ea7e-4a54-bc91-99928a8405e3",
    "Alexander Skripnikov",
    crate::card::CardRules::unsupported(),
);

// MSH 243 — Arc Reactor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARC_REACTOR: CardRecord = CardRecord::new(
    "Arc Reactor",
    "339acb17-4b8e-4836-9cc5-8a0a946ebc73",
    "Maxim Ruabtsev",
    crate::card::CardRules::unsupported(),
);

// MSH 244 — Captain America's Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTAIN_AMERICA_S_SHIELD: CardRecord = CardRecord::new(
    "Captain America's Shield",
    "4b5433ac-0d36-4472-bf8b-d22f0ffd367b",
    "Eglė Mosakaitė",
    crate::card::CardRules::unsupported(),
);

// MSH 245 — Cosmic Cube
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COSMIC_CUBE: CardRecord = CardRecord::new(
    "Cosmic Cube",
    "d1cf1ead-fe91-4328-89ab-6d0bc9ff6cbe",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// MSH 246 — Dependable Quinjet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEPENDABLE_QUINJET: CardRecord = CardRecord::new(
    "Dependable Quinjet",
    "c035c625-4de5-4c3b-9d07-aa1df8fc7b78",
    "Oliver Wetter",
    crate::card::CardRules::unsupported(),
);

// MSH 247 — H.E.R.B.I.E. Scout Unit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static H_E_R_B_I_E_SCOUT_UNIT: CardRecord = CardRecord::new(
    "H.E.R.B.I.E. Scout Unit",
    "3496d0d4-3fab-4ea4-8986-afe5a7155ec6",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// MSH 248 — Iron Man Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_MAN_ARMOR: CardRecord = CardRecord::new(
    "Iron Man Armor",
    "361c2f3b-f04e-446b-a683-9195e238daf0",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// MSH 249 — S.H.I.E.L.D. Helicarrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static S_H_I_E_L_D_HELICARRIER: CardRecord = CardRecord::new(
    "S.H.I.E.L.D. Helicarrier",
    "364dc091-3161-4cd7-838c-742b9325fc32",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 250 — Super-Adaptoid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUPER_ADAPTOID: CardRecord = CardRecord::new(
    "Super-Adaptoid",
    "fbbd8609-5a00-4188-96fe-77251579b88d",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// MSH 251 — The Ten Rings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_TEN_RINGS: CardRecord = CardRecord::new(
    "The Ten Rings",
    "2332bc91-b0f2-4911-844d-d1cc915cd6c8",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 252 — Ultron, Artificial Malevolence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ULTRON_ARTIFICIAL_MALEVOLENCE: CardRecord = CardRecord::new(
    "Ultron, Artificial Malevolence",
    "32ddd5ac-57ed-4e78-8932-a65980191f6e",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// MSH 253 — Ultron Drone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ULTRON_DRONE: CardRecord = CardRecord::new(
    "Ultron Drone",
    "149a0a3b-c470-414c-a7de-d773b8b4cc82",
    "Rafater",
    crate::card::CardRules::unsupported(),
);

// MSH 254 — Vibranium Energy Daggers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIBRANIUM_ENERGY_DAGGERS: CardRecord = CardRecord::new(
    "Vibranium Energy Daggers",
    "4fe5952d-f50f-443a-b960-657fc4bc1965",
    "Irvin Rodriguez",
    crate::card::CardRules::unsupported(),
);

// MSH 255 — The Vision
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_VISION: CardRecord = CardRecord::new(
    "The Vision",
    "2961cf20-33c8-4e66-9d0f-6daca8ea7880",
    "Carissa Susilo",
    crate::card::CardRules::unsupported(),
);

// MSH 256 — Viv Vision, Teen Synthezoid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIV_VISION_TEEN_SYNTHEZOID: CardRecord = CardRecord::new(
    "Viv Vision, Teen Synthezoid",
    "85cc170a-ecd2-4870-b675-7ece88813995",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// MSH 257 — A.I.M. Labs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static A_I_M_LABS: CardRecord = CardRecord::new(
    "A.I.M. Labs",
    "ca24cd9f-fd9f-4ea0-8b9c-ecf98db219a6",
    "Lixin Yin",
    crate::card::CardRules::unsupported(),
);

// MSH 258 — Asgardian Citadel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASGARDIAN_CITADEL: CardRecord = CardRecord::new(
    "Asgardian Citadel",
    "d5f88c3d-b17b-46aa-a573-449350f95d46",
    "Paulius Daščioras",
    crate::card::CardRules::unsupported(),
);

// MSH 259 — Avengers Hangar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGERS_HANGAR: CardRecord = CardRecord::new(
    "Avengers Hangar",
    "c5d03ebb-1ce9-4a6b-b13e-a72f0b075ae8",
    "Pablo Mendoza",
    crate::card::CardRules::unsupported(),
);

// MSH 260 — Avengers Tower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGERS_TOWER: CardRecord = CardRecord::new(
    "Avengers Tower",
    "88f0d9c9-8a1f-4b5a-b6f9-821ddd658d27",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 261 — Baxter Building
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAXTER_BUILDING: CardRecord = CardRecord::new(
    "Baxter Building",
    "1abc652f-8e9d-4df7-bc4e-d8b515a40fec",
    "Paulius Daščioras",
    crate::card::CardRules::unsupported(),
);

// MSH 262 — Birnin Zana Plaza
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIRNIN_ZANA_PLAZA: CardRecord = CardRecord::new(
    "Birnin Zana Plaza",
    "41463827-46de-40c4-ac2b-1fdf6aa36f65",
    "Raymond Bonilla",
    crate::card::CardRules::unsupported(),
);

// MSH 263 — Castle Doom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASTLE_DOOM: CardRecord = CardRecord::new(
    "Castle Doom",
    "6b39d7a6-ca2d-4376-a18e-efd0138e83bc",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// MSH 264 — Dark Fortress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_FORTRESS: CardRecord = CardRecord::new(
    "Dark Fortress",
    "c16fd43c-7c47-4c1b-860f-91146532e89d",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 265 — Fisk Tower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FISK_TOWER: CardRecord = CardRecord::new(
    "Fisk Tower",
    "7690e624-93c1-46f4-8f33-e28d881787d3",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 266 — Gathering Place
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GATHERING_PLACE: CardRecord = CardRecord::new(
    "Gathering Place",
    "081cdbd0-5081-4a9e-90ba-f5baf4ac137e",
    "Pablo Mendoza",
    crate::card::CardRules::unsupported(),
);

// MSH 267 — Gleaming Bastion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLEAMING_BASTION: CardRecord = CardRecord::new(
    "Gleaming Bastion",
    "c9131bf5-17e3-4aa6-97ed-ed6426b247d0",
    "Arthur Yuan",
    crate::card::CardRules::unsupported(),
);

// MSH 268 — Hell's Kitchen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELL_S_KITCHEN: CardRecord = CardRecord::new(
    "Hell's Kitchen",
    "6137c34e-cc4c-4342-a8e4-cfa9c767c67b",
    "Shahab Alizadeh",
    crate::card::CardRules::unsupported(),
);

// MSH 269 — Hidden Lair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_LAIR: CardRecord = CardRecord::new(
    "Hidden Lair",
    "0742ddb6-71ed-444e-91ad-84f876725a4a",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// MSH 270 — Los Diablos Missile Base
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOS_DIABLOS_MISSILE_BASE: CardRecord = CardRecord::new(
    "Los Diablos Missile Base",
    "2123fcb5-8181-47ab-9a2d-7ede5b5118e8",
    "Rockey Chen",
    crate::card::CardRules::unsupported(),
);

// MSH 271 — Pym Technologies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYM_TECHNOLOGIES: CardRecord = CardRecord::new(
    "Pym Technologies",
    "1de583ce-e805-45f1-907f-198bc82fd3b5",
    "John Tyler Christopher",
    crate::card::CardRules::unsupported(),
);

// MSH 272 — Stark Industries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARK_INDUSTRIES: CardRecord = CardRecord::new(
    "Stark Industries",
    "fe3609d8-71a2-49d9-a3fa-25e0906a1a0e",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// MSH 273 — Subterranean Cavern
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBTERRANEAN_CAVERN: CardRecord = CardRecord::new(
    "Subterranean Cavern",
    "038bf500-e23a-4d38-9312-db1909e20353",
    "JB Casacop",
    crate::card::CardRules::unsupported(),
);

// MSH 274 — Surveillance Room
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURVEILLANCE_ROOM: CardRecord = CardRecord::new(
    "Surveillance Room",
    "8fb51fbb-ab7f-4484-a3c9-dc50cb6f2756",
    "Maxim Ruabtsev",
    crate::card::CardRules::unsupported(),
);

// MSH 275 — Training Compound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINING_COMPOUND: CardRecord = CardRecord::new(
    "Training Compound",
    "c91e28db-307f-462a-88aa-581d10e77f10",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// MSH 276 — Villainous Hideout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILLAINOUS_HIDEOUT: CardRecord = CardRecord::new(
    "Villainous Hideout",
    "822b0249-e1df-453d-8b60-75a5196ed818",
    "Paulius Daščioras",
    crate::card::CardRules::unsupported(),
);

// MSH 277 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "be30f027-ffdd-4d81-bed2-4adb6b4b803f",
    "Marc Aspinall",
);

// MSH 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "69a24fc3-90f9-4f2e-91d9-1aaca888e432",
    "Sylvia Liu",
);

// MSH 279 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "8c1467b1-ff2c-4f73-91ba-bc2b07afc2c3",
    "Marc Aspinall",
);

// MSH 280 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "937b3d7d-decb-4e17-998a-6def3f2c97a2",
    "Sylvia Liu",
);

// MSH 281 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "dda87be6-317e-4227-a96d-7742ac054d2e",
    "Marc Aspinall",
);

// MSH 282 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "69eee76e-5714-4503-8170-3f551410abec",
    "Sylvia Liu",
);

// MSH 283 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "9c29ba11-7438-4fd7-98b9-961be882122d",
    "Marc Aspinall",
);

// MSH 284 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "22cc3a0c-ab10-490d-b309-df87df551328",
    "Sylvia Liu",
);

// MSH 285 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "e5da22bf-bc8a-4810-b273-1e33a6b37323",
    "Marc Aspinall",
);

// MSH 286 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "47eb96ec-a417-4dbc-82d2-6317372dda11",
    "Sylvia Liu",
);

// MSH 287 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "5f029c91-f20b-4aa5-a7d9-dd6ddef1c877",
    "Domenico Cava",
);

// MSH 288 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "f29448c0-3126-4018-a934-5a4ed019ea76",
    "Jurijus Chitrovas",
);

// MSH 289 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "5a74175b-2418-4226-8435-1490532e644f",
    "Raymond Bonilla",
);

// MSH 290 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "02f76514-9e0d-4d74-a224-15a2d7b44ae1",
    "Rytis Sabaliauskas",
);

// MSH 291 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "74414c62-4b68-4e1b-9241-feff3231b9ee",
    "Eugene Maslovski",
);

// MSH 292 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "3dc20c2b-7344-43d1-8ea0-22bab837657e",
    "Oliver Wetter",
);

// MSH 293 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "4bdbae27-9a55-485f-a1fc-3fa988184e9d",
    "Rockey Chen",
);

// MSH 294 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "f7752159-06bd-4f6c-b3d2-c4c7616acfbb",
    "Pace Wilder",
);

// MSH 295 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "22463284-2478-4b1e-9a37-6dd2383266cf",
    "Domenico Cava",
);

// MSH 296 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "c9d637ac-e0d3-4c62-9f00-08270f3c1e91",
    "Rytis Sabaliauskas",
);

// MSH 297 — Avengers Assemble! (alternate printing)
const AVENGERS_ASSEMBLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVENGERS_ASSEMBLE,
    1,
    "6c11d918-51c4-4728-8277-74d4ddefacc1",
    "Steve Ellis",
);

// MSH 298 — Origin of the Avengers (alternate printing)
const ORIGIN_OF_THE_AVENGERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORIGIN_OF_THE_AVENGERS,
    1,
    "8a500880-56c9-44ea-ad34-c3e5123106cb",
    "Tyler Walpole",
);

// MSH 299 — Super-Soldier Serum (alternate printing)
const SUPER_SOLDIER_SERUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPER_SOLDIER_SERUM,
    1,
    "dbe76b0f-b9ba-4a5d-ae35-288cc95dd6a4",
    "Tyler Walpole",
);

// MSH 300 — Multiversal Incursion (alternate printing)
const MULTIVERSAL_INCURSION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MULTIVERSAL_INCURSION,
    1,
    "51fce242-30ff-42f6-9b98-572e69cf9833",
    "Jim Cheung & Jay David Ramos",
);

// MSH 301 — Secret Invasion (alternate printing)
const SECRET_INVASION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SECRET_INVASION,
    1,
    "97728b94-7807-45d8-9048-996ece5447e1",
    "Veronica Fish",
);

// MSH 302 — Avengers Disassembled (alternate printing)
const AVENGERS_DISASSEMBLED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVENGERS_DISASSEMBLED,
    1,
    "c1e9b598-a3a4-4d31-8f74-532415725b70",
    "Roberta Ingranata",
);

// MSH 303 — Mjölnir, Hammer of Thor (alternate printing)
const MJOLNIR_HAMMER_OF_THOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MJOLNIR_HAMMER_OF_THOR,
    1,
    "4ae7df19-14f2-4f0f-b545-32be7b98b678",
    "Jim Cheung & Jay David Ramos",
);

// MSH 304 — World War Hulk (alternate printing)
const WORLD_WAR_HULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WORLD_WAR_HULK,
    1,
    "51524f8a-030c-4522-bfc0-d55c2c7c9326",
    "Steve Ellis",
);

// MSH 305 — Armor Wars (alternate printing)
const ARMOR_WARS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARMOR_WARS,
    1,
    "bd6a749b-51c2-43f9-b422-7f395f9b7989",
    "Andrew Griffith",
);

// MSH 306 — Avengers: Under Siege (alternate printing)
const AVENGERS_UNDER_SIEGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVENGERS_UNDER_SIEGE,
    1,
    "3e02a423-363a-422e-b71b-0daf3722ffcc",
    "Andrew Griffith",
);

// MSH 307 — The Coming of Galactus (alternate printing)
const THE_COMING_OF_GALACTUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_COMING_OF_GALACTUS,
    1,
    "8ffaa5b3-00fa-46ea-918c-dfce0de9e898",
    "Veronica Fish",
);

// MSH 308 — The Super Hero Civil War (alternate printing)
const THE_SUPER_HERO_CIVIL_WAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SUPER_HERO_CIVIL_WAR,
    1,
    "3260ac51-1e78-4cb2-9dd8-bbe1b6692583",
    "Roberta Ingranata",
);

// MSH 309 — Vision Quest (alternate printing)
const VISION_QUEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VISION_QUEST,
    1,
    "271ee6c8-164b-4d6a-a7c0-3aa8dea10b7d",
    "Roberta Ingranata",
);

// MSH 310 — Arc Reactor (alternate printing)
const ARC_REACTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARC_REACTOR,
    1,
    "3b7e9150-028d-482d-9bd8-d9d3e56d24be",
    "Steve Ellis",
);

// MSH 311 — Captain America's Shield (alternate printing)
const CAPTAIN_AMERICA_S_SHIELD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_S_SHIELD,
    1,
    "2cef736d-170d-4551-83e5-19f6b6bb677c",
    "Jim Cheung & Jay David Ramos",
);

// MSH 312 — Cosmic Cube (alternate printing)
const COSMIC_CUBE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMIC_CUBE,
    1,
    "7c5abd50-0eaa-4ecb-9e24-a91f54eb37c4",
    "Jim Cheung & Jay David Ramos",
);

// MSH 313 — The Ten Rings (alternate printing)
const THE_TEN_RINGS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_TEN_RINGS,
    1,
    "47360bb6-9724-4418-b1f3-59d0b28e7798",
    "Jim Cheung & Jay David Ramos",
);

// MSH 314 — Nick Fury, Agent of S.H.I.E.L.D. (alternate printing)
const NICK_FURY_AGENT_OF_S_H_I_E_L_D_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NICK_FURY_AGENT_OF_S_H_I_E_L_D,
    1,
    "7370a565-274e-4a91-acae-a920ee1c7719",
    "Annie Wu",
);

// MSH 315 — Baron Strucker, HYDRA Overlord (alternate printing)
const BARON_STRUCKER_HYDRA_OVERLORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARON_STRUCKER_HYDRA_OVERLORD,
    1,
    "fa4e32b7-8c2c-4efa-b5a4-b32abcae1ad2",
    "Annie Wu",
);

// MSH 316 — Captain America, Super-Soldier (alternate printing)
const CAPTAIN_AMERICA_SUPER_SOLDIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_SUPER_SOLDIER,
    1,
    "b0a27523-65bf-4e7a-8560-e11f099b1d1a",
    "Annie Wu",
);

// MSH 317 — Captain America's Shield (alternate printing)
const CAPTAIN_AMERICA_S_SHIELD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_S_SHIELD,
    2,
    "8d70d97a-d0a9-4603-8f59-174d54b0e095",
    "Annie Wu",
);

// MSH 318 — Madame Hydra (alternate printing)
const MADAME_HYDRA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MADAME_HYDRA,
    1,
    "f05ffa99-fce3-4dc3-80c1-cd4de82b83d0",
    "Annie Wu",
);

// MSH 319 — Arnim Zola, Bio-Fanatic (alternate printing)
const ARNIM_ZOLA_BIO_FANATIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARNIM_ZOLA_BIO_FANATIC,
    1,
    "8e3fdece-348a-45eb-871f-fffa91a1fbda",
    "Annie Wu",
);

// MSH 320 — Mister Fantastic, Reed Richards (alternate printing)
const MISTER_FANTASTIC_REED_RICHARDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MISTER_FANTASTIC_REED_RICHARDS,
    1,
    "6fa1c908-68cb-4832-b972-f9efe7d55cdb",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 321 — Human Torch, Johnny Storm (alternate printing)
const HUMAN_TORCH_JOHNNY_STORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUMAN_TORCH_JOHNNY_STORM,
    1,
    "2fa8010c-3f88-4fbe-880e-7facfea06893",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 322 — The Thing, Ben Grimm (alternate printing)
const THE_THING_BEN_GRIMM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_THING_BEN_GRIMM,
    1,
    "16a98e97-2b63-4fcb-a3fc-5f8298f6ab12",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 323 — Invisible Woman, Sue Storm (alternate printing)
const INVISIBLE_WOMAN_SUE_STORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INVISIBLE_WOMAN_SUE_STORM,
    1,
    "8a037650-cdb2-41f8-b3a8-fedcfdb90259",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 324 — Daredevil, Man Without Fear (alternate printing)
const DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAREDEVIL_MAN_WITHOUT_FEAR,
    1,
    "c9e6348f-f77b-42e6-a123-709e8c03013a",
    "Kev Walker",
);

// MSH 325 — Bullseye, Death Dealer (alternate printing)
const BULLSEYE_DEATH_DEALER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BULLSEYE_DEATH_DEALER,
    1,
    "879abc8e-bc69-4214-945a-09d02b66010c",
    "Kev Walker",
);

// MSH 326 — Elektra, Daughter of the Hand (alternate printing)
const ELEKTRA_DAUGHTER_OF_THE_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELEKTRA_DAUGHTER_OF_THE_HAND,
    1,
    "0d855376-c5ff-4981-9605-6c16f213a857",
    "Kev Walker",
);

// MSH 327 — The Kingpin of Crime (alternate printing)
const THE_KINGPIN_OF_CRIME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_KINGPIN_OF_CRIME,
    1,
    "1803564a-5ec5-4c0d-817c-5575127a3703",
    "Kev Walker",
);

// MSH 328 — Jennifer Walters // The Sensational She-Hulk (alternate printing)
const JENNIFER_WALTERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JENNIFER_WALTERS,
    1,
    "eec216e1-971b-490f-aba7-2d5bd5085023",
    "Chris Bachalo & Adi Granov",
);

// MSH 329 — Red Hulk (alternate printing)
const RED_HULK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RED_HULK,
    1,
    "7e623818-838b-4c02-a23a-498cf18d70d6",
    "Adi Granov",
);

// MSH 330 — Leader, Super-Genius (alternate printing)
const LEADER_SUPER_GENIUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEADER_SUPER_GENIUS,
    1,
    "aef5d45a-de20-4c27-866d-f305b04cd47a",
    "Adi Granov",
);

// MSH 331 — Bruce Banner // The Incredible Hulk (alternate printing)
const BRUCE_BANNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRUCE_BANNER,
    1,
    "eb2d2d35-4956-42fa-94e0-b241ed666683",
    "Chris Bachalo & Adi Granov",
);

// MSH 332 — HULK SMASH! (alternate printing)
const HULK_SMASH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HULK_SMASH,
    1,
    "0d01bc37-ebf4-4f08-9beb-fef8787f8e7a",
    "Adi Granov",
);

// MSH 333 — Abomination, Terrifying Titan (alternate printing)
const ABOMINATION_TERRIFYING_TITAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABOMINATION_TERRIFYING_TITAN,
    1,
    "d1fb5d54-8232-4888-9d51-5aca90a7b6eb",
    "Adi Granov",
);

// MSH 334 — Avengers Tower (alternate printing)
const AVENGERS_TOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVENGERS_TOWER,
    1,
    "1292713f-1462-438f-a3a2-1f69ece99668",
    "Johan Grenier",
);

// MSH 335 — Spider-Man, To the Rescue (alternate printing)
const SPIDER_MAN_TO_THE_RESCUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPIDER_MAN_TO_THE_RESCUE,
    1,
    "87dd1800-b9b6-4bac-a504-28146a42a4b4",
    "Johan Grenier",
);

// MSH 336 — Doctor Doom (alternate printing)
const DOCTOR_DOOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOCTOR_DOOM,
    1,
    "be8177e1-b0b4-4c8f-90b9-034313c7b271",
    "Johan Grenier",
);

// MSH 337 — Cosmic Cube (alternate printing)
const COSMIC_CUBE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COSMIC_CUBE,
    2,
    "1d49e61f-b6b9-4106-b0ad-cdd4da87bb83",
    "Johan Grenier",
);

// MSH 338 — Thor, God of Thunder (alternate printing)
const THOR_GOD_OF_THUNDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THOR_GOD_OF_THUNDER,
    1,
    "2fde89b2-5ad8-4b07-8a48-2b33d373be30",
    "Johan Grenier",
);

// MSH 339 — Mjölnir, Hammer of Thor (alternate printing)
const MJOLNIR_HAMMER_OF_THOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MJOLNIR_HAMMER_OF_THOR,
    2,
    "a9e05121-e0de-463b-8d96-529277167b5d",
    "Johan Grenier",
);

// MSH 340 — Blazing Crescendo (alternate printing)
const BLAZING_CRESCENDO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_one::BLAZING_CRESCENDO,
    1,
    "1ac8c104-572e-443d-b6d2-0cd26c0fef98",
    "Johan Grenier",
);

// MSH 341 — Avengers Assemble! (alternate printing)
const AVENGERS_ASSEMBLE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AVENGERS_ASSEMBLE,
    2,
    "78570121-0c1e-42ef-91a1-82fecaad004e",
    "Johan Grenier",
);

// MSH 342 — Captain Marvel, Earth's Protector (alternate printing)
const CAPTAIN_MARVEL_EARTH_S_PROTECTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_MARVEL_EARTH_S_PROTECTOR,
    1,
    "963de372-310c-4272-aa13-576246d69762",
    "Johan Grenier",
);

// MSH 343 — Loki, God of Mischief (alternate printing)
const LOKI_GOD_OF_MISCHIEF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOKI_GOD_OF_MISCHIEF,
    1,
    "881af8c7-2d4d-4b68-b999-20e56a973311",
    "Johan Grenier",
);

// MSH 344 — Ultron, Artificial Malevolence (alternate printing)
const ULTRON_ARTIFICIAL_MALEVOLENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ULTRON_ARTIFICIAL_MALEVOLENCE,
    1,
    "8a65f1b1-56b5-4a4c-acc7-d1b8933244a4",
    "Johan Grenier",
);

// MSH 345 — Monica Rambeau // Photon, Living Light (alternate printing)
const MONICA_RAMBEAU_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MONICA_RAMBEAU,
    1,
    "7be20870-ccdc-4285-920d-739cafb60465",
    "Chris Bachalo & Johan Grenier",
);

// MSH 346 — King T'Challa // Black Panther, Hope Enduring (alternate printing)
const KING_T_CHALLA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KING_T_CHALLA,
    1,
    "eeedc3a9-e5c0-4234-8653-1a005df095e1",
    "Chris Bachalo & Johan Grenier",
);

// MSH 347 — Black Widow, Super Spy (alternate printing)
const BLACK_WIDOW_SUPER_SPY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLACK_WIDOW_SUPER_SPY,
    1,
    "c88457b3-0385-43b5-8d56-f537e022eb94",
    "Johan Grenier",
);

// MSH 348 — Take Up the Shield (alternate printing)
const TAKE_UP_THE_SHIELD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dmu::TAKE_UP_THE_SHIELD,
    1,
    "d7ebfe43-1772-4557-bad5-e170e8270d21",
    "Johan Grenier",
);

// MSH 349 — Earth's Mightiest Heroes (alternate printing)
const EARTH_S_MIGHTIEST_HEROES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EARTH_S_MIGHTIEST_HEROES,
    1,
    "d8eebea7-a210-41c2-b505-3b9a4cdd5339",
    "Johan Grenier",
);

// MSH 350 — Tony Stark // The Invincible Iron Man (alternate printing)
const TONY_STARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TONY_STARK,
    1,
    "7a85467d-5693-4cf7-b38d-d42c57242f48",
    "Chris Bachalo & Johan Grenier",
);

// MSH 351 — Epic Fight (alternate printing)
const EPIC_FIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EPIC_FIGHT,
    1,
    "444303ce-b9c1-49be-86ca-8a7e39f06e7c",
    "Johan Grenier",
);

// MSH 352 — Captain America, Super-Soldier (alternate printing)
const CAPTAIN_AMERICA_SUPER_SOLDIER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_SUPER_SOLDIER,
    2,
    "5873766d-9b8d-4850-bbe7-76ab6c08e138",
    "Jim Cheung & Jay David Ramos",
);

// MSH 353 — Captain America, Wings of Freedom (alternate printing)
const CAPTAIN_AMERICA_WINGS_OF_FREEDOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_WINGS_OF_FREEDOM,
    1,
    "309e72e0-d47e-49af-b85a-45b51afb65b1",
    "Elena Casagrande",
);

// MSH 354 — Captain Marvel, Earth's Protector (alternate printing)
const CAPTAIN_MARVEL_EARTH_S_PROTECTOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_MARVEL_EARTH_S_PROTECTOR,
    2,
    "824e1378-b09e-4de1-ae48-4fc026220ab6",
    "Swayart",
);

// MSH 355 — Jennifer Walters // The Sensational She-Hulk (alternate printing)
const JENNIFER_WALTERS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &JENNIFER_WALTERS,
    2,
    "feb9e3c1-4d66-49a2-8a67-eebd0d272fc3",
    "Trevor Hairsine",
);

// MSH 356 — Monica Rambeau // Photon, Living Light (alternate printing)
const MONICA_RAMBEAU_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MONICA_RAMBEAU,
    2,
    "32a96916-9356-4e46-ba74-296670c6e865",
    "Trevor Hairsine",
);

// MSH 357 — Nick Fury, Agent of S.H.I.E.L.D. (alternate printing)
const NICK_FURY_AGENT_OF_S_H_I_E_L_D_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NICK_FURY_AGENT_OF_S_H_I_E_L_D,
    2,
    "307db766-8d25-485d-81da-eb72548f9ce0",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 358 — The Sentry, Golden Guardian (alternate printing)
const THE_SENTRY_GOLDEN_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SENTRY_GOLDEN_GUARDIAN,
    1,
    "6ba62e03-d8e6-4cd9-8997-0784c01ae8c6",
    "Todd Nauck",
);

// MSH 359 — Bruce Banner // The Incredible Hulk (alternate printing)
const BRUCE_BANNER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BRUCE_BANNER,
    2,
    "0ccb6a0e-a8aa-413d-8e6d-f25447800ed4",
    "Jim Cheung & Jay David Ramos",
);

// MSH 360 — Ironheart, Clever Champion (alternate printing)
const IRONHEART_CLEVER_CHAMPION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRONHEART_CLEVER_CHAMPION,
    1,
    "ebfaaaac-b0ad-4f1d-93b4-64dcccfe1cd1",
    "Kael Ngu",
);

// MSH 361 — Ms. Marvel, Kamala Khan (alternate printing)
const MS_MARVEL_KAMALA_KHAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MS_MARVEL_KAMALA_KHAN,
    1,
    "aea6935f-1f10-4a61-83f8-5bd2ed14a3c0",
    "Roberta Ingranata",
);

// MSH 362 — Namor the Sub-Mariner (alternate printing)
const NAMOR_THE_SUB_MARINER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NAMOR_THE_SUB_MARINER,
    1,
    "804e43a0-faf6-4cd7-b5e8-34c767f8fa1c",
    "Todd Nauck",
);

// MSH 363 — Tony Stark // The Invincible Iron Man (alternate printing)
const TONY_STARK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TONY_STARK,
    2,
    "58bf75bd-83c9-4281-a108-bf5aa31d70e5",
    "Jim Cheung & Jay David Ramos",
);

// MSH 364 — Black Widow, Super Spy (alternate printing)
const BLACK_WIDOW_SUPER_SPY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLACK_WIDOW_SUPER_SPY,
    2,
    "73cf4b89-2551-444c-beb8-ec23e446d44a",
    "Todd Nauck",
);

// MSH 365 — Doctor Doom (alternate printing)
const DOCTOR_DOOM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DOCTOR_DOOM,
    2,
    "08b2c1c8-52cd-4c7d-a15a-4703df31dd14",
    "Leonardo Vincent (Levinky)",
);

// MSH 366 — Elektra, Daughter of the Hand (alternate printing)
const ELEKTRA_DAUGHTER_OF_THE_HAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELEKTRA_DAUGHTER_OF_THE_HAND,
    2,
    "5beb6acc-38a5-47f5-ae37-bb396baffa5c",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 367 — Hawkeye, Master Marksman (alternate printing)
const HAWKEYE_MASTER_MARKSMAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAWKEYE_MASTER_MARKSMAN,
    1,
    "972aedc2-c645-45d7-8976-de1fd504e819",
    "Todd Nauck",
);

// MSH 368 — The Scarlet Witch (alternate printing)
const THE_SCARLET_WITCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SCARLET_WITCH,
    1,
    "4bf2a4dc-c4aa-453a-ad18-1af42eb59f40",
    "Swayart",
);

// MSH 369 — Thor, God of Thunder (alternate printing)
const THOR_GOD_OF_THUNDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THOR_GOD_OF_THUNDER,
    2,
    "20b7cb55-16bf-4329-a711-3dbcdaab728b",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 370 — Shang-Chi, Master of Kung Fu (alternate printing)
const SHANG_CHI_MASTER_OF_KUNG_FU_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHANG_CHI_MASTER_OF_KUNG_FU,
    1,
    "44a4eabc-52ec-49f2-aa08-e0d4196a1287",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 371 — The Unbeatable Squirrel Girl (alternate printing)
const THE_UNBEATABLE_SQUIRREL_GIRL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_UNBEATABLE_SQUIRREL_GIRL,
    1,
    "ef518e5b-7942-4e66-aee0-a1c2aa1f9754",
    "Swayart",
);

// MSH 372 — Daredevil, Man Without Fear (alternate printing)
const DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DAREDEVIL_MAN_WITHOUT_FEAR,
    2,
    "ddd4c1d4-09c9-4b85-aef5-e764682ece62",
    "Trevor Hairsine",
);

// MSH 373 — King T'Challa // Black Panther, Hope Enduring (alternate printing)
const KING_T_CHALLA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KING_T_CHALLA,
    2,
    "0e99dad7-b3ca-4e89-8d9f-8f8f43195537",
    "John Tyler Christopher",
);

// MSH 374 — The Kingpin of Crime (alternate printing)
const THE_KINGPIN_OF_CRIME_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_KINGPIN_OF_CRIME,
    2,
    "756fc57c-2687-4cd9-9431-9fa1d453fa2c",
    "Trevor Hairsine",
);

// MSH 375 — Storm, Windrider (alternate printing)
const STORM_WINDRIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORM_WINDRIDER,
    1,
    "2c4a0e2c-1037-44cd-9630-b5511d82f9cb",
    "Terry Dodson",
);

// MSH 376 — Thanos, the Mad Titan (alternate printing)
const THANOS_THE_MAD_TITAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THANOS_THE_MAD_TITAN,
    1,
    "29e82e97-79b3-4495-9aa0-a90242d594b2",
    "Trevor Hairsine",
);

// MSH 377 — Winter Soldier, Icy Assassin (alternate printing)
const WINTER_SOLDIER_ICY_ASSASSIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINTER_SOLDIER_ICY_ASSASSIN,
    1,
    "76c05333-0e02-4173-b8b4-cb0da3b99a64",
    "Todd Nauck",
);

// MSH 378 — Wolverine, Fierce Fighter (alternate printing)
const WOLVERINE_FIERCE_FIGHTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WOLVERINE_FIERCE_FIGHTER,
    1,
    "ddc2bf31-d10b-4a57-ad16-6448830f440c",
    "Todd Nauck",
);

// MSH 379 — The Vision (alternate printing)
const THE_VISION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_VISION,
    1,
    "d41a55d9-6007-4b1c-a917-2cb5e8f64000",
    "Trevor Hairsine",
);

// MSH 380 — Dark Fortress (alternate printing)
const DARK_FORTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DARK_FORTRESS,
    1,
    "e9b288fb-d122-4c9f-9dd9-d21856a6aac9",
    "David Álvarez",
);

// MSH 381 — Gathering Place (alternate printing)
const GATHERING_PLACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GATHERING_PLACE,
    1,
    "87f14fae-281c-4667-a25b-ca764d8e4fbf",
    "David Álvarez",
);

// MSH 382 — Gleaming Bastion (alternate printing)
const GLEAMING_BASTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLEAMING_BASTION,
    1,
    "7c769658-0d99-4758-b8bd-4c8d03a09add",
    "David Álvarez",
);

// MSH 383 — Hidden Lair (alternate printing)
const HIDDEN_LAIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIDDEN_LAIR,
    1,
    "4c2d4184-0377-40ee-9bf4-40d5183d0d0d",
    "David Álvarez",
);

// MSH 384 — Training Compound (alternate printing)
const TRAINING_COMPOUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRAINING_COMPOUND,
    1,
    "18085a32-bfc4-421b-9e76-ad46bab1f337",
    "David Álvarez",
);

// MSH 385 — The Mind Stone (alternate printing)
const THE_MIND_STONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MIND_STONE,
    1,
    "c023ab8d-7f9e-427f-9dcf-699bb65aba5c",
    "Madeline Boni",
);

// MSH 386 — The Mind Stone (alternate printing)
const THE_MIND_STONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_MIND_STONE,
    2,
    "0e6f8149-61e4-4d7f-840d-a832d83a284e",
    "Leinil Francis Yu & Sunny Gho",
);

// MSH 387 — Captain America, Super-Soldier (alternate printing)
const CAPTAIN_AMERICA_SUPER_SOLDIER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_SUPER_SOLDIER,
    3,
    "7ffdca9d-3ee4-4572-b1ad-4f03523968fd",
    "Jack Kirby & John Romita Sr.",
);

// MSH 388 — Jennifer Walters // The Sensational She-Hulk (alternate printing)
const JENNIFER_WALTERS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &JENNIFER_WALTERS,
    3,
    "b51282a9-1132-4c93-a216-148d1e7bb007",
    "John Buscema & Chic Stone",
);

// MSH 389 — Nick Fury, Agent of S.H.I.E.L.D. (alternate printing)
const NICK_FURY_AGENT_OF_S_H_I_E_L_D_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NICK_FURY_AGENT_OF_S_H_I_E_L_D,
    3,
    "89f7008c-762b-45c3-8ddb-d02ce1b37ebc",
    "Marie Severin",
);

// MSH 390 — Bruce Banner // The Incredible Hulk (alternate printing)
const BRUCE_BANNER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BRUCE_BANNER,
    3,
    "481166c5-6a16-469c-91bd-abf291b5db49",
    "Jack Kirby & Paul Reinman",
);

// MSH 391 — Namor the Sub-Mariner (alternate printing)
const NAMOR_THE_SUB_MARINER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NAMOR_THE_SUB_MARINER,
    2,
    "47016620-48c7-42b2-9bef-8e2f8fc0786e",
    "John Buscema & Sol Brodsky",
);

// MSH 392 — Tony Stark // The Invincible Iron Man (alternate printing)
const TONY_STARK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TONY_STARK,
    3,
    "81eaf2ee-edeb-4026-81bb-8766ffa61f0d",
    "Johnny Craig",
);

// MSH 393 — Black Widow, Super Spy (alternate printing)
const BLACK_WIDOW_SUPER_SPY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BLACK_WIDOW_SUPER_SPY,
    3,
    "8e3c4b2a-4ef7-41ea-a4b5-b0cc942a9889",
    "Emanuela Lupacchino & Neeraj Menon",
);

// MSH 394 — Doctor Doom (alternate printing)
const DOCTOR_DOOM_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DOCTOR_DOOM,
    3,
    "06255e9e-5395-41ed-ad89-1e7b2b5478d3",
    "Larry Lieber & Vince Colletta",
);

// MSH 395 — Elektra, Daughter of the Hand (alternate printing)
const ELEKTRA_DAUGHTER_OF_THE_HAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ELEKTRA_DAUGHTER_OF_THE_HAND,
    3,
    "0c58b938-f856-4828-9d75-2e4db4547df7",
    "Mike Deodato Jr.",
);

// MSH 396 — Shang-Chi, Master of Kung Fu (alternate printing)
const SHANG_CHI_MASTER_OF_KUNG_FU_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SHANG_CHI_MASTER_OF_KUNG_FU,
    2,
    "b367d034-ae08-4b08-b090-39a6d4f79dc2",
    "Jim Cheung & Laura Martin",
);

// MSH 397 — The Astonishing Ant-Man (alternate printing)
const THE_ASTONISHING_ANT_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ASTONISHING_ANT_MAN,
    1,
    "15850911-bfeb-4e9e-aec9-34a22faac896",
    "Bob Layton",
);

// MSH 398 — Daredevil, Man Without Fear (alternate printing)
const DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DAREDEVIL_MAN_WITHOUT_FEAR,
    3,
    "21610bae-7e18-42fe-8cdc-dacbf7f9c07f",
    "Jack Kirby",
);

// MSH 399 — King T'Challa // Black Panther, Hope Enduring (alternate printing)
const KING_T_CHALLA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KING_T_CHALLA,
    3,
    "d1d8391c-dacc-4f21-a40e-5ca53a787f0d",
    "Jack Kirby",
);

// MSH 400 — Thanos, the Mad Titan (alternate printing)
const THANOS_THE_MAD_TITAN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THANOS_THE_MAD_TITAN,
    2,
    "c2cdcd6d-15ee-46be-b90e-b0f5d2c5de87",
    "Jen Bartel",
);

// MSH 401 — Ultron, Artificial Malevolence (alternate printing)
const ULTRON_ARTIFICIAL_MALEVOLENCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ULTRON_ARTIFICIAL_MALEVOLENCE,
    2,
    "df9302a8-e918-429c-8853-133c489223f1",
    "Sal Buscema & Sam Grainger",
);

// MSH 402 — Agent Phil Coulson (alternate printing)
const AGENT_PHIL_COULSON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGENT_PHIL_COULSON,
    1,
    "258b663e-2fa0-4203-9f5f-f9963940c5b3",
    "Marc Aspinall",
);

// MSH 403 — Kang the Conqueror (alternate printing)
const KANG_THE_CONQUEROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KANG_THE_CONQUEROR,
    1,
    "55b486ad-53bd-4255-9f8e-286b300e7e43",
    "Peter Scanlan",
);

// MSH 404 — S.H.I.E.L.D. Flying Car (alternate printing)
const S_H_I_E_L_D_FLYING_CAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &S_H_I_E_L_D_FLYING_CAR,
    1,
    "0912ea2d-88af-48b7-8262-89547e67ff64",
    "Paulius Daščioras",
);

// MSH 405 — Baron Helmut Zemo (alternate printing)
const BARON_HELMUT_ZEMO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BARON_HELMUT_ZEMO,
    1,
    "1ff8f5aa-a450-4399-bd3a-a8f6b221bab0",
    "Wero Gallo",
);

// MSH 406 — Construct a Cosmic Cube (alternate printing)
const CONSTRUCT_A_COSMIC_CUBE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONSTRUCT_A_COSMIC_CUBE,
    1,
    "248b4c86-163b-46eb-9949-e8d8e5875294",
    "Eugene Maslovski",
);

// MSH 407 — Doom Reigns Supreme (alternate printing)
const DOOM_REIGNS_SUPREME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOOM_REIGNS_SUPREME,
    1,
    "d2468de2-fdb1-4d92-a303-30d255423ddf",
    "Alexander Gering",
);

// MSH 408 — M.O.D.O.K. (alternate printing)
const M_O_D_O_K_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &M_O_D_O_K,
    1,
    "bf38cf25-579f-4f94-92fb-a781c90a5f2e",
    "Simon Dominic",
);

// MSH 409 — Super-Skrull (alternate printing)
const SUPER_SKRULL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPER_SKRULL,
    1,
    "625d95ac-41c5-4b42-b479-52446bb39465",
    "Zoltan Boros",
);

// MSH 410 — Thunderbolts Conspiracy (alternate printing)
const THUNDERBOLTS_CONSPIRACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDERBOLTS_CONSPIRACY,
    1,
    "90da36af-fddb-43e5-992e-2aa782af66c3",
    "Lucio Parrillo",
);

// MSH 411 — Fin Fang Foom (alternate printing)
const FIN_FANG_FOOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIN_FANG_FOOM,
    1,
    "e79ef21e-ddd3-408a-88a6-7383f88ea320",
    "Filipe Pagliuso",
);

// MSH 412 — Quicksilver, Brash Blur (alternate printing)
const QUICKSILVER_BRASH_BLUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUICKSILVER_BRASH_BLUR,
    1,
    "b8874ee6-504d-447f-a15b-0ee4bbb1a89f",
    "Michael MacRae",
);

// MSH 413 — Earth's Mightiest Heroes (alternate printing)
const EARTH_S_MIGHTIEST_HEROES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EARTH_S_MIGHTIEST_HEROES,
    2,
    "b3534144-0738-4bbc-8d77-c78e28ce42eb",
    "Steve Morris",
);

// MSH 414 — Heroic Feast (alternate printing)
const HEROIC_FEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEROIC_FEAST,
    1,
    "dd9b6b3f-07b1-42d2-8aff-8da3113b9d88",
    "Javier Charro",
);

// MSH 415 — Mole Man, Moloid Master (alternate printing)
const MOLE_MAN_MOLOID_MASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOLE_MAN_MOLOID_MASTER,
    1,
    "f1fbac4b-574e-4f30-9ee7-6054081b278c",
    "Michele Giorgi",
);

// MSH 416 — Absorbing Man (alternate printing)
const ABSORBING_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABSORBING_MAN,
    1,
    "35d772a2-717d-4b82-a267-52efed584ae9",
    "Nathaniel Himawan",
);

// MSH 417 — Alien Invasion (alternate printing)
const ALIEN_INVASION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALIEN_INVASION,
    1,
    "3d32fd3a-ce7f-42b2-84ca-4f25f096ade0",
    "Björn Barends",
);

// MSH 418 — Ares, God of War (alternate printing)
const ARES_GOD_OF_WAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARES_GOD_OF_WAR,
    1,
    "1dcccbcf-9dea-4fab-b296-d382c60dd6c7",
    "Fesbra",
);

// MSH 419 — Cloak and Dagger, Entwined (alternate printing)
const CLOAK_AND_DAGGER_ENTWINED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOAK_AND_DAGGER_ENTWINED,
    1,
    "50aa361c-72fd-4ecb-b877-a27646c66ad0",
    "Rimas Valeikis",
);

// MSH 420 — The Mighty Thor, Jane Foster (alternate printing)
const THE_MIGHTY_THOR_JANE_FOSTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MIGHTY_THOR_JANE_FOSTER,
    1,
    "dd02db75-9e7d-4958-9c2b-34e3fc1f7bf7",
    "Victor Adame Minguez",
);

// MSH 421 — Moon Girl and Devil Dinosaur (alternate printing)
const MOON_GIRL_AND_DEVIL_DINOSAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOON_GIRL_AND_DEVIL_DINOSAUR,
    1,
    "42141284-32b3-4195-b696-801232fe6799",
    "Zezhou Chen",
);

// MSH 422 — The Ruinous Wrecking Crew (alternate printing)
const THE_RUINOUS_WRECKING_CREW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_RUINOUS_WRECKING_CREW,
    1,
    "52c4f3fa-d6d2-4fe7-9a9c-ec3ab41134e9",
    "Kevin Sidharta",
);

// MSH 423 — Scientist Supreme of A.I.M. (alternate printing)
const SCIENTIST_SUPREME_OF_A_I_M_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCIENTIST_SUPREME_OF_A_I_M,
    1,
    "2b9beaa2-87c0-45f3-9b63-3f944c7daa02",
    "Gal Or",
);

// MSH 424 — The Serpent Society (alternate printing)
const THE_SERPENT_SOCIETY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SERPENT_SOCIETY,
    1,
    "112ac08d-a4b6-4f2b-8afb-41d2ca948d56",
    "Rimas Valeikis",
);

// MSH 425 — Taskmaster, Mercenary Mimic (alternate printing)
const TASKMASTER_MERCENARY_MIMIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TASKMASTER_MERCENARY_MIMIC,
    1,
    "0251766b-ab59-4079-aea7-9968b49f04ef",
    "Riccardo Federici",
);

// MSH 426 — Worlds Within Worlds (alternate printing)
const WORLDS_WITHIN_WORLDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WORLDS_WITHIN_WORLDS,
    1,
    "36918a64-e13c-4a95-8068-eb2517572c66",
    "Michael MacRae",
);

// MSH 427 — Iron Man Armor (alternate printing)
const IRON_MAN_ARMOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRON_MAN_ARMOR,
    1,
    "cb2f6737-8063-4e67-affc-42c4915d2db1",
    "Javier Charro",
);

// MSH 428 — Super-Adaptoid (alternate printing)
const SUPER_ADAPTOID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUPER_ADAPTOID,
    1,
    "5dcdba1d-434e-4c4e-8ec5-f51472f915de",
    "John Tyler Christopher",
);

// MSH 429 — Castle Doom (alternate printing)
const CASTLE_DOOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CASTLE_DOOM,
    1,
    "daaf180d-074d-4ade-8954-0b3a90d56ce5",
    "Nino Is",
);

// MSH 431 — The Scarlet Witch (alternate printing)
const THE_SCARLET_WITCH_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_SCARLET_WITCH,
    2,
    "3a62ba9f-e164-43ab-a20d-151805e8c237",
    "Julia Vasilyeva",
);

// MSH 432 — Daredevil, Man Without Fear (alternate printing)
const DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &DAREDEVIL_MAN_WITHOUT_FEAR,
    4,
    "a5f5730d-8c6b-4708-887d-9499439da4e7",
    "Eric Deschamps",
);

// MSH 433 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "427edc1d-0c8c-4a63-9c86-e38afbe7d35f",
    "Marc Aspinall",
);

// MSH 434 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "dabb5c08-3529-448f-be7f-179d71682d9f",
    "Sylvia Liu",
);

// MSH 435 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    4,
    "87a0e243-ebb8-4237-886f-632aa72e1c73",
    "Marc Aspinall",
);

// MSH 436 — Island (alternate printing)
const ISLAND_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    5,
    "35ecaa17-6528-4d73-b282-288b4939b96f",
    "Sylvia Liu",
);

// MSH 437 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    4,
    "06b5576e-968a-41b6-bc5c-a0ed6521d419",
    "Marc Aspinall",
);

// MSH 438 — Swamp (alternate printing)
const SWAMP_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    5,
    "02e71e69-ff6c-4821-81d3-e30931eb7bfe",
    "Sylvia Liu",
);

// MSH 439 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    4,
    "5388993c-685b-4d9a-91d3-ea678ff11b71",
    "Marc Aspinall",
);

// MSH 440 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    5,
    "bc4409dc-aadf-4c0e-a854-190ccc971ea5",
    "Sylvia Liu",
);

// MSH 441 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    4,
    "6b6a8792-2355-4948-9c19-68491888d73f",
    "Marc Aspinall",
);

// MSH 442 — Forest (alternate printing)
const FOREST_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    5,
    "7cc4d5d1-4471-43b0-877a-b3af0b8d67c6",
    "Sylvia Liu",
);

// MSH 443 — Captain Mar-Vell, Space-Born (alternate printing)
const CAPTAIN_MAR_VELL_SPACE_BORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_MAR_VELL_SPACE_BORN,
    1,
    "ec54ad35-721c-4a8a-8b68-971f416bb150",
    "Gintas Galvanauskas",
);

// MSH 444 — Patriot, Shield Wielder (alternate printing)
const PATRIOT_SHIELD_WIELDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PATRIOT_SHIELD_WIELDER,
    1,
    "51ab3270-3f50-46e8-b222-64e2b8f9a382",
    "Vlad Petruchik",
);

// MSH 445 — Mister Fantastic, Reed Richards (alternate printing)
const MISTER_FANTASTIC_REED_RICHARDS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MISTER_FANTASTIC_REED_RICHARDS,
    2,
    "0adf55a7-1e40-4756-a2e0-799271e6a3d2",
    "Rimas Valeikis",
);

// MSH 446 — Baron Strucker, HYDRA Overlord (alternate printing)
const BARON_STRUCKER_HYDRA_OVERLORD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BARON_STRUCKER_HYDRA_OVERLORD,
    2,
    "0eb58292-72f2-4344-b5ec-db6c8999cb01",
    "InHyuk Lee",
);

// MSH 447 — Moonstone, Harsh Mistress (alternate printing)
const MOONSTONE_HARSH_MISTRESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONSTONE_HARSH_MISTRESS,
    1,
    "792e4119-0518-4c97-822d-d146a5368a15",
    "Grace Zhu",
);

// MSH 448 — Speed, Young Avenger (alternate printing)
const SPEED_YOUNG_AVENGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPEED_YOUNG_AVENGER,
    1,
    "7224b9cc-6a88-4aea-a7ab-ab64e0e25451",
    "Tyler Walpole",
);

// MSH 449 — Captain America, Living Legend (alternate printing)
const CAPTAIN_AMERICA_LIVING_LEGEND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAPTAIN_AMERICA_LIVING_LEGEND,
    1,
    "debe1ab2-ed15-4b8d-96f4-ad35bab44f82",
    "Smirtouille",
);

// MSH 450 — Kang, Temporal Tyrant (alternate printing)
const KANG_TEMPORAL_TYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KANG_TEMPORAL_TYRANT,
    1,
    "e5dc6bca-f005-43e9-b6c7-739e49064c2f",
    "David Szabo",
);

// MSH 451 — Madame Hydra (alternate printing)
const MADAME_HYDRA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MADAME_HYDRA,
    2,
    "4dedca48-b7a2-45b9-af1e-b3754879d1d6",
    "Pauline Voss",
);

// MSH 452 — Avengers Tower (alternate printing)
const AVENGERS_TOWER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &AVENGERS_TOWER,
    2,
    "2fea16cb-7cd0-46f2-bea4-a85cdd40a51a",
    "Arthur Yuan",
);

// MSH 453 — Baxter Building (alternate printing)
const BAXTER_BUILDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BAXTER_BUILDING,
    1,
    "3edd8a6e-1be9-48d1-9dd1-cab72416115d",
    "Paulius Daščioras",
);

// MSH 454 — Villainous Hideout (alternate printing)
const VILLAINOUS_HIDEOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VILLAINOUS_HIDEOUT,
    1,
    "cb4727f5-4a7e-487e-bd72-5f58eb8bf88e",
    "Paulius Daščioras",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AGENT_13_SHARON_CARTER,
    &AGENT_MARIA_HILL,
    &AGENT_OF_ATLAS,
    &AGENT_PHIL_COULSON,
    &AGENTS_OF_S_H_I_E_L_D,
    &AVENGERS_ASSEMBLE,
    &BOROUGH_BACKUP,
    &BRAVE_BRAWLER,
    &CAPTAIN_AMERICA_SUPER_SOLDIER,
    &CAPTAIN_AMERICA_WINGS_OF_FREEDOM,
    &CAPTAIN_MARVEL_EARTH_S_PROTECTOR,
    &CAPTAIN_MAR_VELL_SPACE_BORN,
    &COLLEEN_WING_STREET_SAMURAI,
    &CROWD_OF_TRUE_BELIEVERS,
    &HELICARRIER_STRIKE,
    &HERO_IN_TRAINING,
    &INVISIBLE_WOMAN_SUE_STORM,
    &JENNIFER_WALTERS,
    &KREE_COMMANDOS,
    &LUKE_CAGE_POWER_MAN,
    &THE_MIND_STONE,
    &MOCKINGBIRD_ACE_AGENT,
    &MONICA_RAMBEAU,
    &MURDOCK_S_CRUSADE,
    &NICK_FURY_AGENT_OF_S_H_I_E_L_D,
    &NIGHT_NURSE_HEALER_OF_HEROES,
    &OKOYE_DORA_MILAJE_LEADER,
    &ORIGIN_OF_THE_AVENGERS,
    &PANTHER_POUNCE,
    &PATRIOT_SHIELD_WIELDER,
    &POLITICAL_TRIUMPH,
    &QUAKE_AGENT_OF_S_H_I_E_L_D,
    &RAFT_SECURITY_OFFICER,
    &RED_GUARDIAN_SUPER_SOLDIER,
    &THE_SENTRY_GOLDEN_GUARDIAN,
    &S_H_I_E_L_D_SPY_KIT,
    &SUPER_VILLAIN_LOCKUP,
    &SUPER_SOLDIER_SERUM,
    &WAKANDAN_DRONE_FLOCK,
    &WHITE_WIDOW_FREE_AGENT,
    &AERIAL_DOOMBOT,
    &A_I_M_SCIENTISTS,
    &ATLANTEAN_CAVALRY,
    &ATLANTIS_ATTACKS,
    &ATTUMA_ATLANTEAN_WARLORD,
    &BOLD_BIOCHEMIST,
    &BRUCE_BANNER,
    &DEPOWER,
    &ECHO_PERCEPTIVE_PRODIGY,
    &FALCON_WINGED_WONDER,
    &FALCON_S_WING_HARNESS,
    &FROZEN_IN_ICE,
    &FUTURIST_FORGE,
    &GIANT_SIZED_FLYING_ANT,
    &HYDRAULIC_HELPER,
    &I_AM_IRON_MAN,
    &IRON_LAD_DIVERGING_DESTINY,
    &IRONHEART_CLEVER_CHAMPION,
    &JUSTICE_VANCE_ASTROVIK,
    &KANG_THE_CONQUEROR,
    &KID_LOKI,
    &LEADER_SUPER_GENIUS,
    &LOKI_GOD_OF_MISCHIEF,
    &MISTER_FANTASTIC_REED_RICHARDS,
    &MS_MARVEL_KAMALA_KHAN,
    &MULTIVERSAL_INCURSION,
    &NAMOR_THE_SUB_MARINER,
    &PYM_PARTICLES,
    &REWRITE_HISTORY,
    &SECRET_INVASION,
    &S_H_I_E_L_D_DEPLOYMENT_DRONE,
    &S_H_I_E_L_D_FLYING_CAR,
    &SHURI_WAKANDAN_INVENTOR,
    &STATURE_SIZE_SHIFTER,
    &SUPER_INTELLIGENCE,
    &SUPER_SUIT,
    &TONY_STARK,
    &TRICKSTER_S_STRATAGEM,
    &WE_SAY_THEE_NAY,
    &WICCAN_RISING_MAGICIAN,
    &THE_WONDROUS_WASP,
    &AGENTS_OF_HYDRA,
    &ARNIM_ZOLA_BIO_FANATIC,
    &BARON_HELMUT_ZEMO,
    &BARON_STRUCKER_HYDRA_OVERLORD,
    &BLACK_WIDOW_SUPER_SPY,
    &CONSTRUCT_A_COSMIC_CUBE,
    &CROSSBONES_MALICIOUS_MERCENARY,
    &CRUEL_ALLIANCE,
    &DARK_DEED,
    &DECOY_PLOY,
    &DOCTOR_DOOM,
    &DOOM_REIGNS_SUPREME,
    &ELEKTRA_DAUGHTER_OF_THE_HAND,
    &GRIM_REAPER_LETHAL_LEGIONNAIRE,
    &HOUR_OF_DEFEAT,
    &HYDRA_INFILTRATION,
    &HYDRA_TROOPERS,
    &KINGPIN_S_ENFORCERS,
    &KLAW_SONIC_SUBJUGATOR,
    &MADAME_MASQUE,
    &THE_MASTERS_OF_EVIL,
    &M_O_D_O_K,
    &MOONSTONE_HARSH_MISTRESS,
    &NINJA_OF_THE_HAND,
    &PROJECT_DEATHLOK_SOLDIER,
    &RED_ROOM_RECRUIT,
    &ROBOT_DOMINATION,
    &RONIN_SHADOW_STALKER,
    &ROXXON_BRUTES,
    &STOLEN_STARK_TECH,
    &SUPER_SKRULL,
    &SWORDSMAN_SHARP_SCOUNDREL,
    &THUNDERBOLTS_CONSPIRACY,
    &TOO_EVIL_TO_STAY_DEAD,
    &UNLIVING_LEGIONNAIRE,
    &VISIONS_OF_VILLAINY,
    &WHIPLASH_VENGEFUL_ENGINEER,
    &WIDOW_S_BITE,
    &YELLOWJACKET_HEARTLESS_MARAUDER,
    &AVENGERS_DISASSEMBLED,
    &CRIMSON_OPERATIVE,
    &DEATH_TO_OUR_ENEMIES,
    &EVIL_S_THRALL,
    &FIN_FANG_FOOM,
    &HAWKEYE_MASTER_MARKSMAN,
    &HAWKEYE_YOUNG_AVENGER,
    &HAWKEYE_S_BOW,
    &HEX_MAGIC,
    &HIRE_A_CREW,
    &HULK_SMASH,
    &HUMAN_TORCH_JOHNNY_STORM,
    &HYDRA_ASSAULT_ROBOT,
    &IRON_FIST_LIVING_WEAPON,
    &JESSICA_JONES_PRIVATE_EYE,
    &K_UN_LUN_WARRIOR,
    &KREE_SENTINEL,
    &LOKI_LAUFEYSON,
    &MACHINESMITH_AUTOMATON,
    &MISTY_KNIGHT_HERO_FOR_HIRE,
    &MJOLNIR_HAMMER_OF_THOR,
    &PHOTON_BLAST_BARRAGE,
    &QUICKSILVER_BRASH_BLUR,
    &RED_HULK,
    &REPULSOR_BLAST,
    &THE_SCARLET_WITCH,
    &SPEED_YOUNG_AVENGER,
    &STARK_INDUSTRIES_EXECUTIVE,
    &SUPER_SPEED,
    &TEAM_TACTICS,
    &THOR_GOD_OF_THUNDER,
    &TRUCK_TOSS,
    &VISION_OF_LOVE,
    &VOLCANIC_VILLAIN,
    &WONDER_MAN_HOLLYWOOD_HERO,
    &ANT_MAN_S_ARMY,
    &CALL_DAMAGE_CONTROL,
    &CLAIM_THE_KINGDOM,
    &DOC_SAMSON_SUPER_PSYCHIATRIST,
    &EARTH_S_MIGHTIEST_HEROES,
    &EPIC_FIGHT,
    &GO_NUTS,
    &GUERRILLA_GORILLA,
    &HELLCAT_UNDYING_VIGILANTE,
    &HERCULES_PRINCE_OF_POWER,
    &HEROIC_FEAST,
    &HULKLING_BURGEONING_BRUISER,
    &KA_ZAR_OF_THE_SAVAGE_LAND,
    &KNIGHT_OF_WUNDAGORE,
    &MISTER_HYDE_MONSTER_WITHIN,
    &MOLE_MAN_MOLOID_MASTER,
    &PET_AVENGERS,
    &POWERFUL_BROKER,
    &PUNISHING_PUNCH,
    &RAPID_RESCUE,
    &REPTIL_DINOMORPHER,
    &RESTORATIVE_TECHNIQUE,
    &RICK_JONES_DESTINED_SIDEKICK,
    &SAVAGE_LAND_DINOSAUR,
    &SERPENT_SPECIALIST,
    &SHANG_CHI_MASTER_OF_KUNG_FU,
    &SHE_HULK_JADE_DEFENDER,
    &SUPER_STRENGTH,
    &THE_THING_BEN_GRIMM,
    &TIGRA_FELINE_FURY,
    &TRAINING_REGIMEN,
    &THE_UNBEATABLE_SQUIRREL_GIRL,
    &UNDERCOVER_SKRULL,
    &WAKANDAN_ROYAL_GUARD,
    &WHITE_TIGER_AVA_AYALA,
    &WORLD_WAR_HULK,
    &ABOMINATION_TERRIFYING_TITAN,
    &ABSORBING_MAN,
    &ALIEN_INVASION,
    &ANT_MAN_COLONY_COMMANDER,
    &ARES_GOD_OF_WAR,
    &ARMOR_WARS,
    &THE_ASTONISHING_ANT_MAN,
    &AVENGERS_UNDER_SIEGE,
    &BEAST_ERUDITE_AERIALIST,
    &BLACK_PANTHER_VANGUARD,
    &BLACK_WIDOW_DOUBLE_AGENT,
    &BULLSEYE_DEATH_DEALER,
    &CAPTAIN_AMERICA_LIVING_LEGEND,
    &CLOAK_AND_DAGGER_ENTWINED,
    &THE_COMING_OF_GALACTUS,
    &DAREDEVIL_MAN_WITHOUT_FEAR,
    &GHOST_SPECTRAL_SABOTEUR,
    &HULK_GAMMA_GOLIATH,
    &IRON_MAN_MASTER_OF_MACHINES,
    &KANG_TEMPORAL_TYRANT,
    &KILLMONGER_SCOURGE_OF_WAKANDA,
    &KING_T_CHALLA,
    &THE_KINGPIN_OF_CRIME,
    &MADAME_HYDRA,
    &THE_MIGHTY_THOR_JANE_FOSTER,
    &MOON_GIRL_AND_DEVIL_DINOSAUR,
    &THE_RUINOUS_WRECKING_CREW,
    &SCIENTIST_SUPREME_OF_A_I_M,
    &THE_SERPENT_SOCIETY,
    &SPEEDBALL_NEW_WARRIOR,
    &SPIDER_MAN_TO_THE_RESCUE,
    &SPIDER_WOMAN_SECRET_AGENT,
    &STORM_WINDRIDER,
    &THE_SUPER_HERO_CIVIL_WAR,
    &TASKMASTER_MERCENARY_MIMIC,
    &THANOS_THE_MAD_TITAN,
    &THOR_ODINSON,
    &TITANIA_RUGGED_RUMBLER,
    &U_S_AGENT_JOHN_WALKER,
    &VISION_QUEST,
    &WAR_MACHINE_LEGACY_OF_IRON,
    &WINTER_SOLDIER_ICY_ASSASSIN,
    &WOLVERINE_FIERCE_FIGHTER,
    &WORLDS_WITHIN_WORLDS,
    &A_I_M_SYNTHOIDS,
    &ARC_REACTOR,
    &CAPTAIN_AMERICA_S_SHIELD,
    &COSMIC_CUBE,
    &DEPENDABLE_QUINJET,
    &H_E_R_B_I_E_SCOUT_UNIT,
    &IRON_MAN_ARMOR,
    &S_H_I_E_L_D_HELICARRIER,
    &SUPER_ADAPTOID,
    &THE_TEN_RINGS,
    &ULTRON_ARTIFICIAL_MALEVOLENCE,
    &ULTRON_DRONE,
    &VIBRANIUM_ENERGY_DAGGERS,
    &THE_VISION,
    &VIV_VISION_TEEN_SYNTHEZOID,
    &A_I_M_LABS,
    &ASGARDIAN_CITADEL,
    &AVENGERS_HANGAR,
    &AVENGERS_TOWER,
    &BAXTER_BUILDING,
    &BIRNIN_ZANA_PLAZA,
    &CASTLE_DOOM,
    &DARK_FORTRESS,
    &FISK_TOWER,
    &GATHERING_PLACE,
    &GLEAMING_BASTION,
    &HELL_S_KITCHEN,
    &HIDDEN_LAIR,
    &LOS_DIABLOS_MISSILE_BASE,
    &PYM_TECHNOLOGIES,
    &STARK_INDUSTRIES,
    &SUBTERRANEAN_CAVERN,
    &SURVEILLANCE_ROOM,
    &TRAINING_COMPOUND,
    &VILLAINOUS_HIDEOUT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    TAKE_UP_THE_SHIELD_REPRINT,
    WEB_UP_REPRINT,
    THIRST_FOR_KNOWLEDGE_REPRINT,
    BLAZING_CRESCENDO_REPRINT,
    LIGHTNING_STRIKE_REPRINT,
    GIANT_GROWTH_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    AVENGERS_ASSEMBLE_ALTERNATE_1,
    ORIGIN_OF_THE_AVENGERS_ALTERNATE_1,
    SUPER_SOLDIER_SERUM_ALTERNATE_1,
    MULTIVERSAL_INCURSION_ALTERNATE_1,
    SECRET_INVASION_ALTERNATE_1,
    AVENGERS_DISASSEMBLED_ALTERNATE_1,
    MJOLNIR_HAMMER_OF_THOR_ALTERNATE_1,
    WORLD_WAR_HULK_ALTERNATE_1,
    ARMOR_WARS_ALTERNATE_1,
    AVENGERS_UNDER_SIEGE_ALTERNATE_1,
    THE_COMING_OF_GALACTUS_ALTERNATE_1,
    THE_SUPER_HERO_CIVIL_WAR_ALTERNATE_1,
    VISION_QUEST_ALTERNATE_1,
    ARC_REACTOR_ALTERNATE_1,
    CAPTAIN_AMERICA_S_SHIELD_ALTERNATE_1,
    COSMIC_CUBE_ALTERNATE_1,
    THE_TEN_RINGS_ALTERNATE_1,
    NICK_FURY_AGENT_OF_S_H_I_E_L_D_ALTERNATE_1,
    BARON_STRUCKER_HYDRA_OVERLORD_ALTERNATE_1,
    CAPTAIN_AMERICA_SUPER_SOLDIER_ALTERNATE_1,
    CAPTAIN_AMERICA_S_SHIELD_ALTERNATE_2,
    MADAME_HYDRA_ALTERNATE_1,
    ARNIM_ZOLA_BIO_FANATIC_ALTERNATE_1,
    MISTER_FANTASTIC_REED_RICHARDS_ALTERNATE_1,
    HUMAN_TORCH_JOHNNY_STORM_ALTERNATE_1,
    THE_THING_BEN_GRIMM_ALTERNATE_1,
    INVISIBLE_WOMAN_SUE_STORM_ALTERNATE_1,
    DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_1,
    BULLSEYE_DEATH_DEALER_ALTERNATE_1,
    ELEKTRA_DAUGHTER_OF_THE_HAND_ALTERNATE_1,
    THE_KINGPIN_OF_CRIME_ALTERNATE_1,
    JENNIFER_WALTERS_ALTERNATE_1,
    RED_HULK_ALTERNATE_1,
    LEADER_SUPER_GENIUS_ALTERNATE_1,
    BRUCE_BANNER_ALTERNATE_1,
    HULK_SMASH_ALTERNATE_1,
    ABOMINATION_TERRIFYING_TITAN_ALTERNATE_1,
    AVENGERS_TOWER_ALTERNATE_1,
    SPIDER_MAN_TO_THE_RESCUE_ALTERNATE_1,
    DOCTOR_DOOM_ALTERNATE_1,
    COSMIC_CUBE_ALTERNATE_2,
    THOR_GOD_OF_THUNDER_ALTERNATE_1,
    MJOLNIR_HAMMER_OF_THOR_ALTERNATE_2,
    BLAZING_CRESCENDO_ALTERNATE_1,
    AVENGERS_ASSEMBLE_ALTERNATE_2,
    CAPTAIN_MARVEL_EARTH_S_PROTECTOR_ALTERNATE_1,
    LOKI_GOD_OF_MISCHIEF_ALTERNATE_1,
    ULTRON_ARTIFICIAL_MALEVOLENCE_ALTERNATE_1,
    MONICA_RAMBEAU_ALTERNATE_1,
    KING_T_CHALLA_ALTERNATE_1,
    BLACK_WIDOW_SUPER_SPY_ALTERNATE_1,
    TAKE_UP_THE_SHIELD_ALTERNATE_1,
    EARTH_S_MIGHTIEST_HEROES_ALTERNATE_1,
    TONY_STARK_ALTERNATE_1,
    EPIC_FIGHT_ALTERNATE_1,
    CAPTAIN_AMERICA_SUPER_SOLDIER_ALTERNATE_2,
    CAPTAIN_AMERICA_WINGS_OF_FREEDOM_ALTERNATE_1,
    CAPTAIN_MARVEL_EARTH_S_PROTECTOR_ALTERNATE_2,
    JENNIFER_WALTERS_ALTERNATE_2,
    MONICA_RAMBEAU_ALTERNATE_2,
    NICK_FURY_AGENT_OF_S_H_I_E_L_D_ALTERNATE_2,
    THE_SENTRY_GOLDEN_GUARDIAN_ALTERNATE_1,
    BRUCE_BANNER_ALTERNATE_2,
    IRONHEART_CLEVER_CHAMPION_ALTERNATE_1,
    MS_MARVEL_KAMALA_KHAN_ALTERNATE_1,
    NAMOR_THE_SUB_MARINER_ALTERNATE_1,
    TONY_STARK_ALTERNATE_2,
    BLACK_WIDOW_SUPER_SPY_ALTERNATE_2,
    DOCTOR_DOOM_ALTERNATE_2,
    ELEKTRA_DAUGHTER_OF_THE_HAND_ALTERNATE_2,
    HAWKEYE_MASTER_MARKSMAN_ALTERNATE_1,
    THE_SCARLET_WITCH_ALTERNATE_1,
    THOR_GOD_OF_THUNDER_ALTERNATE_2,
    SHANG_CHI_MASTER_OF_KUNG_FU_ALTERNATE_1,
    THE_UNBEATABLE_SQUIRREL_GIRL_ALTERNATE_1,
    DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_2,
    KING_T_CHALLA_ALTERNATE_2,
    THE_KINGPIN_OF_CRIME_ALTERNATE_2,
    STORM_WINDRIDER_ALTERNATE_1,
    THANOS_THE_MAD_TITAN_ALTERNATE_1,
    WINTER_SOLDIER_ICY_ASSASSIN_ALTERNATE_1,
    WOLVERINE_FIERCE_FIGHTER_ALTERNATE_1,
    THE_VISION_ALTERNATE_1,
    DARK_FORTRESS_ALTERNATE_1,
    GATHERING_PLACE_ALTERNATE_1,
    GLEAMING_BASTION_ALTERNATE_1,
    HIDDEN_LAIR_ALTERNATE_1,
    TRAINING_COMPOUND_ALTERNATE_1,
    THE_MIND_STONE_ALTERNATE_1,
    THE_MIND_STONE_ALTERNATE_2,
    CAPTAIN_AMERICA_SUPER_SOLDIER_ALTERNATE_3,
    JENNIFER_WALTERS_ALTERNATE_3,
    NICK_FURY_AGENT_OF_S_H_I_E_L_D_ALTERNATE_3,
    BRUCE_BANNER_ALTERNATE_3,
    NAMOR_THE_SUB_MARINER_ALTERNATE_2,
    TONY_STARK_ALTERNATE_3,
    BLACK_WIDOW_SUPER_SPY_ALTERNATE_3,
    DOCTOR_DOOM_ALTERNATE_3,
    ELEKTRA_DAUGHTER_OF_THE_HAND_ALTERNATE_3,
    SHANG_CHI_MASTER_OF_KUNG_FU_ALTERNATE_2,
    THE_ASTONISHING_ANT_MAN_ALTERNATE_1,
    DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_3,
    KING_T_CHALLA_ALTERNATE_3,
    THANOS_THE_MAD_TITAN_ALTERNATE_2,
    ULTRON_ARTIFICIAL_MALEVOLENCE_ALTERNATE_2,
    AGENT_PHIL_COULSON_ALTERNATE_1,
    KANG_THE_CONQUEROR_ALTERNATE_1,
    S_H_I_E_L_D_FLYING_CAR_ALTERNATE_1,
    BARON_HELMUT_ZEMO_ALTERNATE_1,
    CONSTRUCT_A_COSMIC_CUBE_ALTERNATE_1,
    DOOM_REIGNS_SUPREME_ALTERNATE_1,
    M_O_D_O_K_ALTERNATE_1,
    SUPER_SKRULL_ALTERNATE_1,
    THUNDERBOLTS_CONSPIRACY_ALTERNATE_1,
    FIN_FANG_FOOM_ALTERNATE_1,
    QUICKSILVER_BRASH_BLUR_ALTERNATE_1,
    EARTH_S_MIGHTIEST_HEROES_ALTERNATE_2,
    HEROIC_FEAST_ALTERNATE_1,
    MOLE_MAN_MOLOID_MASTER_ALTERNATE_1,
    ABSORBING_MAN_ALTERNATE_1,
    ALIEN_INVASION_ALTERNATE_1,
    ARES_GOD_OF_WAR_ALTERNATE_1,
    CLOAK_AND_DAGGER_ENTWINED_ALTERNATE_1,
    THE_MIGHTY_THOR_JANE_FOSTER_ALTERNATE_1,
    MOON_GIRL_AND_DEVIL_DINOSAUR_ALTERNATE_1,
    THE_RUINOUS_WRECKING_CREW_ALTERNATE_1,
    SCIENTIST_SUPREME_OF_A_I_M_ALTERNATE_1,
    THE_SERPENT_SOCIETY_ALTERNATE_1,
    TASKMASTER_MERCENARY_MIMIC_ALTERNATE_1,
    WORLDS_WITHIN_WORLDS_ALTERNATE_1,
    IRON_MAN_ARMOR_ALTERNATE_1,
    SUPER_ADAPTOID_ALTERNATE_1,
    CASTLE_DOOM_ALTERNATE_1,
    THE_SCARLET_WITCH_ALTERNATE_2,
    DAREDEVIL_MAN_WITHOUT_FEAR_ALTERNATE_4,
    PLAINS_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    ISLAND_ALTERNATE_4,
    ISLAND_ALTERNATE_5,
    SWAMP_ALTERNATE_4,
    SWAMP_ALTERNATE_5,
    MOUNTAIN_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_5,
    FOREST_ALTERNATE_4,
    FOREST_ALTERNATE_5,
    CAPTAIN_MAR_VELL_SPACE_BORN_ALTERNATE_1,
    PATRIOT_SHIELD_WIELDER_ALTERNATE_1,
    MISTER_FANTASTIC_REED_RICHARDS_ALTERNATE_2,
    BARON_STRUCKER_HYDRA_OVERLORD_ALTERNATE_2,
    MOONSTONE_HARSH_MISTRESS_ALTERNATE_1,
    SPEED_YOUNG_AVENGER_ALTERNATE_1,
    CAPTAIN_AMERICA_LIVING_LEGEND_ALTERNATE_1,
    KANG_TEMPORAL_TYRANT_ALTERNATE_1,
    MADAME_HYDRA_ALTERNATE_2,
    AVENGERS_TOWER_ALTERNATE_2,
    BAXTER_BUILDING_ALTERNATE_1,
    VILLAINOUS_HIDEOUT_ALTERNATE_1,
];
