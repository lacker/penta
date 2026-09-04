//! HML card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardRules, CardSet, CardSupertype,
    CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, PlayerRelation,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// HML 1 — Abbey Gargoyles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABBEY_GARGOYLES: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Abbey Gargoyles",
    "71a5d8de-25f1-4070-a7a6-dc3f2339ce30",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// HML 2a — Abbey Matron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABBEY_MATRON: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Abbey Matron",
    "158caa84-da2e-4c4c-b24d-0c035c900e20",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 2b — Abbey Matron (alternate printing)
const ABBEY_MATRON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABBEY_MATRON,
    1,
    "fd950ddc-28f4-47a5-8fc1-d6f70002fceb",
    "Mike Kimble",
);

// HML 3a — Aysen Bureaucrats (alternate printing)
const AYSEN_BUREAUCRATS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AYSEN_BUREAUCRATS,
    1,
    "91170faf-3143-4fa8-88d6-eef72e1f5459",
    "Alan Rabinowitz",
);

// HML 3b — Aysen Bureaucrats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AYSEN_BUREAUCRATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Aysen Bureaucrats",
    "7ca3fa70-50b3-4157-afda-fe58bf72ee16",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// HML 4 — Aysen Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AYSEN_CRUSADER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Aysen Crusader",
    "7908cfdc-5ed6-48a8-a5b9-351864f8b4fd",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// HML 5 — Aysen Highway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AYSEN_HIGHWAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Aysen Highway",
    "adfa87eb-9a11-459c-bff8-87bb09b61b87",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// HML 6 — Beast Walkers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEAST_WALKERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Beast Walkers",
    "99b42f6c-5c7e-4ba8-b0fb-ac8564aaf825",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// HML 7 — Death Speakers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_SPEAKERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Death Speakers",
    "e17c19a4-0186-45a0-89b9-d7b0fb0ddd8a",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// HML 8 — Hazduhr the Abbot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZDUHR_THE_ABBOT: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Hazduhr the Abbot",
    "adfd416a-dddf-40e4-acf0-84057edb7a58",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// HML 9 — Leeches
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEECHES: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Leeches",
    "90db206e-b254-476c-b2f3-1cd56bb5297d",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// HML 10a — Mesa Falcon (alternate printing)
const MESA_FALCON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MESA_FALCON,
    1,
    "7c2c7004-691b-4385-ad9b-60b93d474ebd",
    "Mark Poole",
);

// HML 10b — Mesa Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MESA_FALCON: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Mesa Falcon",
    "04d2f5e2-fb95-48b0-b7bf-689d45fa8970",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// HML 11 — Prophecy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROPHECY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Prophecy",
    "f514eb63-3a4e-4410-ba3d-487cf81f7063",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// HML 12 — Rashka the Slayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RASHKA_THE_SLAYER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Rashka the Slayer",
    "ddf30363-9db2-44c5-8c13-dbf1aaa8c86b",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// HML 13a — Samite Alchemist (alternate printing)
const SAMITE_ALCHEMIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAMITE_ALCHEMIST,
    1,
    "68224871-d4c2-4b43-9ab8-c0685588b035",
    "Tom Wänerstrand",
);

// HML 13b — Samite Alchemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMITE_ALCHEMIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Samite Alchemist",
    "0545fc43-9c67-4ad4-b1d9-6b57b53321af",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// HML 14 — Serra Aviary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_AVIARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Serra Aviary",
    "688a8de2-0167-4b35-a38d-3574034a892c",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// HML 15 — Serra Bestiary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_BESTIARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Serra Bestiary",
    "aab1c8b8-9b3e-444a-a12c-bd09ec899641",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 16 — Serra Inquisitors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_INQUISITORS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Serra Inquisitors",
    "f1fbe3c8-92fb-41b9-b778-726d22c63054",
    "Dennis Detwiller",
    crate::card::CardRules::unsupported(),
);

// HML 17 — Serra Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Serra Paladin",
    "7bd25adf-4d97-4229-abdb-1c060036cfbd",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// HML 18 — Soraya the Falconer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SORAYA_THE_FALCONER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Soraya the Falconer",
    "19fb3ce2-a660-4829-9af4-330cfd612f06",
    "Dennis Detwiller",
    crate::card::CardRules::unsupported(),
);

// HML 19a — Trade Caravan (alternate printing)
const TRADE_CARAVAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TRADE_CARAVAN,
    1,
    "fdcc821b-8125-4585-ba44-4bf8c1873fbf",
    "Kaja Foglio",
);

// HML 19b — Trade Caravan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRADE_CARAVAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Trade Caravan",
    "e60ddb1e-e607-4080-849c-3e1a79052729",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// HML 20 — Truce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRUCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Truce",
    "35c5fd74-bd46-4833-ae25-1a11a8c15ed2",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// HML 21 — Aether Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Aether Storm",
    "ce479e91-7b21-4312-a3c0-950d9f6dc029",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// HML 22 — Baki's Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAKI_S_CURSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Baki's Curse",
    "e3261b4c-7963-4ca0-875d-77b7c8571b3f",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// HML 23 — Chain Stasis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAIN_STASIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Chain Stasis",
    "f14f0c52-67c2-4302-82bd-fbb4e3c6d4f4",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// HML 24 — Coral Reef
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORAL_REEF: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Coral Reef",
    "42fe2280-a996-4072-b5bf-f4fd56607a51",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 25a — Dark Maze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_MAZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Dark Maze",
    "c20a8ed9-db1e-4ce8-bfb3-92604a577df7",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// HML 25b — Dark Maze (alternate printing)
const DARK_MAZE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DARK_MAZE,
    1,
    "d3d68a76-c843-4c67-90a7-2f79295798d0",
    "Rob Alexander",
);

// HML 26 — Forget
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGET: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Forget",
    "df3115a9-ad65-4213-9320-6f39c11676f3",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 27a — Giant Albatross
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_ALBATROSS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Giant Albatross",
    "bce05870-74d3-43f1-92d0-dc1744c0138d",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// HML 27b — Giant Albatross (alternate printing)
const GIANT_ALBATROSS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GIANT_ALBATROSS,
    1,
    "e26334fc-d8ee-4b8c-ac50-5a304fae6c60",
    "David A. Cherry",
);

// HML 28 — Giant Oyster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_OYSTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Giant Oyster",
    "f8045d23-e6e6-474c-a3e7-ddfc6121657a",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// HML 29 — Jinx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JINX: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Jinx",
    "f81fca41-2315-4d12-b05c-d921a4c3c19e",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 30a — Labyrinth Minotaur (alternate printing)
const LABYRINTH_MINOTAUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LABYRINTH_MINOTAUR,
    1,
    "eb8c1786-a2c1-43f9-9cef-8c4542833093",
    "Anson Maddocks",
);

// HML 30b — Labyrinth Minotaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LABYRINTH_MINOTAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Labyrinth Minotaur",
    "0663c756-9db9-4298-8a6e-a1af935286a0",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 31 — Marjhan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARJHAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Marjhan",
    "b6aa3299-3b7a-4ea5-bc1f-beead26d8116",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// HML 32a — Memory Lapse
pub(in crate::card::sets) static MEMORY_LAPSE: CardRecord = CardRecord::new(
    CardSet::Homelands,
    "Memory Lapse",
    "3d2cc591-3a81-468a-91a4-3c3aac83a21a",
    "Mark Tedin",
    // Two mana that buys a turn rather than a card, which in a deck built to
    // use the turn is the better half of the trade.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. If that spell is countered this way, put it on top of its owner's \
         library instead of into that player's graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        // "Put it on top of its owner's library instead of into that player's
        // graveyard": the counter still happens, and what changes is only where
        // the card lands afterwards.
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// HML 32b — Memory Lapse (alternate printing)
const MEMORY_LAPSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEMORY_LAPSE,
    1,
    "6c8b5df3-6153-470e-be9c-f38d3cf66081",
    "Mark Tedin",
);

// HML 33 — Merchant Scroll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCHANT_SCROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Merchant Scroll",
    "d4133ceb-6176-411a-9eb8-51721c1bb435",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// HML 34 — Mystic Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_DECREE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Mystic Decree",
    "8b069e6a-2c0e-4fc9-8e19-08bf1245a6c0",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// HML 35 — Narwhal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NARWHAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Narwhal",
    "202d3ed5-f493-43b6-bf36-81ad289e6fb0",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// HML 36a — Reef Pirates
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REEF_PIRATES: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Reef Pirates",
    "5b742d75-860d-4b90-89cb-4292f18aed39",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// HML 36b — Reef Pirates (alternate printing)
const REEF_PIRATES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REEF_PIRATES,
    1,
    "b7c5e641-0276-4321-b486-ef8602a5f185",
    "Tom Wänerstrand",
);

// HML 37 — Reveka, Wizard Savant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVEKA_WIZARD_SAVANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Reveka, Wizard Savant",
    "a952236e-3085-4e6e-8639-355976b7c8f5",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// HML 38 — Sea Sprite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_SPRITE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Sea Sprite",
    "88fb001b-3afb-44f5-ab78-af2bf9a4e63a",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// HML 39 — Sea Troll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_TROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Sea Troll",
    "b7da23d4-f9fb-40a5-8395-51b47a064600",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// HML 40 — Wall of Kelp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_KELP: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Wall of Kelp",
    "52ff5051-e24b-4453-aaae-ed4f2bf213ab",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// HML 41 — Baron Sengir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARON_SENGIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Baron Sengir",
    "51bdddac-02fc-493a-a0ea-689273252d7e",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// HML 42 — Black Carriage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_CARRIAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Black Carriage",
    "87068116-6000-44ee-b47f-f5cb8c233bb2",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// HML 43 — Broken Visage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROKEN_VISAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Broken Visage",
    "9be199e7-feaa-4f23-b93c-3eab54a02e74",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 44a — Cemetery Gate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEMETERY_GATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Cemetery Gate",
    "0c6f0614-06dc-4bd2-b8b9-d951ae27db21",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// HML 44b — Cemetery Gate (alternate printing)
const CEMETERY_GATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CEMETERY_GATE,
    1,
    "4a7b2cc1-cb0b-4cb8-963f-453a1d5b0e3c",
    "Melissa A. Benson",
);

// HML 45 — Drudge Spell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRUDGE_SPELL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Drudge Spell",
    "52b352de-e989-4ad5-963c-818092fc9f1a",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// HML 46a — Dry Spell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRY_SPELL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Dry Spell",
    "547c10ea-8ace-4496-8b99-61863c0cec1b",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// HML 46b — Dry Spell (alternate printing)
const DRY_SPELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRY_SPELL,
    1,
    "997ea663-40a1-49b7-80f1-2e1febc1b6fa",
    "Brian Snõddy",
);

// HML 47a — Feast of the Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEAST_OF_THE_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Feast of the Unicorn",
    "096e41d6-79c3-463f-ae63-872c3d8729a7",
    "Dennis Detwiller",
    crate::card::CardRules::unsupported(),
);

// HML 47b — Feast of the Unicorn (alternate printing)
const FEAST_OF_THE_UNICORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEAST_OF_THE_UNICORN,
    1,
    "f05a2735-0f0d-46b5-9c2e-6c0ed8d8944d",
    "Dennis Detwiller",
);

// HML 48 — Funeral March
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNERAL_MARCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Funeral March",
    "054c5678-63d1-45d9-bd51-43100fd10afd",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// HML 49 — Ghost Hounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOST_HOUNDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Ghost Hounds",
    "d1298b43-0b10-4b7c-9d33-786d4d7bd80e",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// HML 50 — Grandmother Sengir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRANDMOTHER_SENGIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Grandmother Sengir",
    "efb0ac91-5e8e-47b1-aa34-902eef60349f",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// HML 51 — Greater Werewolf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREATER_WEREWOLF: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Greater Werewolf",
    "8c29c45f-db1a-43e3-ae42-1a72dabe7880",
    "Dennis Detwiller",
    crate::card::CardRules::unsupported(),
);

// HML 52 — Headstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEADSTONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Headstone",
    "2fdae7fa-1076-4ff3-b771-fc3f5d9ba89f",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// HML 53 — Ihsan's Shade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IHSAN_S_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Ihsan's Shade",
    "82351724-2814-4d9e-b065-bb72c761b2e7",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// HML 54 — Irini Sengir
pub(in crate::card::sets) static IRINI_SENGIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Irini Sengir",
    "518e3b77-d482-4b90-94c0-0b8cdd949b9f",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Vampire", "Dwarf"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::spell_cost_increase(
            "Green enchantment spells and white enchantment spells cost {2} more to cast.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
            PlayerRelation::Any,
            mana_cost!("{2}"),
        )),
);

// HML 55 — Koskun Falls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOSKUN_FALLS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Koskun Falls",
    "04292a4e-8910-4911-a76d-4f2c3e15da33",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// HML 56 — Sengir Autocrat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SENGIR_AUTOCRAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Sengir Autocrat",
    "0d16e024-7865-43d0-8cd8-8933ef741d05",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// HML 57a — Sengir Bats (alternate printing)
const SENGIR_BATS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SENGIR_BATS,
    1,
    "3c745c2c-4311-412d-a137-02bf6d106e46",
    "Dan Frazier",
);

// HML 57b — Sengir Bats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SENGIR_BATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Sengir Bats",
    "2ddb981a-d9e3-4efe-a383-5c98ee3b0b84",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// HML 58 — Timmerian Fiends
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMMERIAN_FIENDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Timmerian Fiends",
    "90643766-c92f-4a25-bd02-227f3c91f391",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 59a — Torture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORTURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Torture",
    "2cb8cc6c-7e24-4629-a9ce-5f717f236c37",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// HML 59b — Torture (alternate printing)
const TORTURE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TORTURE,
    1,
    "4c7e94a3-192f-483b-9a62-b94904ba3464",
    "Mark Tedin",
);

// HML 60 — Veldrane of Sengir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VELDRANE_OF_SENGIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Veldrane of Sengir",
    "fe0ce7d7-d370-4ef8-b1fa-aa70b2fd5ab1",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// HML 61a — Aliban's Tower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALIBAN_S_TOWER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Aliban's Tower",
    "7f711ea8-a73a-42da-8bf7-101ba588f203",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// HML 61b — Aliban's Tower (alternate printing)
const ALIBAN_S_TOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALIBAN_S_TOWER,
    1,
    "d66e94b9-23f6-45a8-97bd-602e3aaa6711",
    "Jeff A. Menges",
);

// HML 62 — Ambush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBUSH: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Ambush",
    "b7dd8623-b64a-4b47-a69d-ed62d44596fb",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// HML 63a — Ambush Party
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBUSH_PARTY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Ambush Party",
    "87e24788-cc7c-4f5d-84d8-dcb35e10626f",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// HML 63b — Ambush Party (alternate printing)
const AMBUSH_PARTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AMBUSH_PARTY,
    1,
    "ea200e93-e5bd-4777-b47e-e53849a89fe7",
    "Mark Poole",
);

// HML 64 — An-Zerrin Ruins
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AN_ZERRIN_RUINS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "An-Zerrin Ruins",
    "4f905d57-2f52-4179-8041-2667b1fb1baa",
    "Dennis Detwiller",
    crate::card::CardRules::unsupported(),
);

// HML 65 — Anaba Ancestor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANABA_ANCESTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Anaba Ancestor",
    "c4d33cc0-525d-4e25-927b-b6b18087c27b",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 66a — Anaba Bodyguard (alternate printing)
const ANABA_BODYGUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANABA_BODYGUARD,
    1,
    "882e3c69-dca7-4c0a-8f8c-984250e6a867",
    "Anson Maddocks",
);

// HML 66b — Anaba Bodyguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANABA_BODYGUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Anaba Bodyguard",
    "56a54048-4640-499b-a1c3-192917c25169",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 67a — Anaba Shaman (alternate printing)
const ANABA_SHAMAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANABA_SHAMAN,
    1,
    "c55d086a-d848-43bc-8c1e-de96d10877e1",
    "Anson Maddocks",
);

// HML 67b — Anaba Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANABA_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Anaba Shaman",
    "4b456355-9f73-45c2-9554-6e6b20d949a1",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 68 — Anaba Spirit Crafter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANABA_SPIRIT_CRAFTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Anaba Spirit Crafter",
    "e9aaabc2-1dab-4f9c-8ed3-60bc1aa995ba",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 69 — Chandler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHANDLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Chandler",
    "4dd3a8e3-9a90-44f4-996c-57242d3c47a5",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// HML 70 — Dwarven Pony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_PONY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Dwarven Pony",
    "53a3019f-0b27-4ba3-be4c-73ed50eb9514",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// HML 71 — Dwarven Sea Clan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_SEA_CLAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Dwarven Sea Clan",
    "4cb722d9-1998-4912-a6f2-4ffa8d21311a",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 72a — Dwarven Trader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_TRADER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Dwarven Trader",
    "4db9aa47-f42b-41e9-948c-8b012c3809fb",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// HML 72b — Dwarven Trader (alternate printing)
const DWARVEN_TRADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DWARVEN_TRADER,
    1,
    "8e6d9318-30d6-473f-b576-cc249454440f",
    "Margaret Organ-Kean",
);

// HML 73 — Eron the Relentless
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERON_THE_RELENTLESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Eron the Relentless",
    "b6329bd9-1e03-43e6-b50b-8abe1356ffcc",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// HML 74 — Evaporate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVAPORATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Evaporate",
    "a3c99939-4854-4e28-a142-4cb7f89fe898",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// HML 75 — Heart Wolf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_WOLF: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Heart Wolf",
    "e0427dcd-26da-462b-b936-a382d3d8afce",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// HML 76 — Ironclaw Curse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRONCLAW_CURSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Ironclaw Curse",
    "e796f0ff-4e7b-4849-b463-0aac860c72ea",
    "Dennis Detwiller",
    crate::card::CardRules::unsupported(),
);

// HML 77 — Joven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOVEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Joven",
    "0dabe3af-cd5b-461e-95a4-aad046646419",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// HML 78 — Orcish Mine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_MINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Orcish Mine",
    "3a630875-b43d-4591-992c-117e1212fa34",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// HML 79 — Retribution
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETRIBUTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Retribution",
    "b3adf9a6-7137-4995-9a83-2d410cb3cd20",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// HML 80 — Winter Sky
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_SKY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Winter Sky",
    "af1035f3-3027-4a41-834c-55222b13c2bc",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 81 — An-Havva Constable
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AN_HAVVA_CONSTABLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "An-Havva Constable",
    "42c5a793-a777-44f9-a977-d16d26d3f852",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// HML 82 — An-Havva Inn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AN_HAVVA_INN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "An-Havva Inn",
    "eff4531f-d19d-44af-861a-33087197d21c",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// HML 83 — Autumn Willow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUTUMN_WILLOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Autumn Willow",
    "cea60340-bbdb-48e2-94a6-5ac1197e978a",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// HML 84a — Carapace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARAPACE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Carapace",
    "07159586-270e-4a3e-9b21-0d74cf3e49d7",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// HML 84b — Carapace (alternate printing)
const CARAPACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARAPACE,
    1,
    "6d51d61e-c010-4b4e-a337-20ba5eb845e3",
    "Anson Maddocks",
);

// HML 85 — Daughter of Autumn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUGHTER_OF_AUTUMN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Daughter of Autumn",
    "972e9c59-f340-414c-b55b-39d46dd97e8e",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// HML 86 — Faerie Noble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_NOBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Faerie Noble",
    "00f8931e-6402-483c-a9e8-63ee344c36a7",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// HML 87a — Folk of An-Havva (alternate printing)
const FOLK_OF_AN_HAVVA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOLK_OF_AN_HAVVA,
    1,
    "9f4a595e-51f6-4e84-aa3a-5d9c18dc8b3f",
    "Julie Baroh",
);

// HML 87b — Folk of An-Havva
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOLK_OF_AN_HAVVA: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Folk of An-Havva",
    "4118c563-08a7-4654-973e-ab9c454f00f9",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// HML 88a — Hungry Mist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNGRY_MIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Hungry Mist",
    "085973eb-56cd-4bb5-aefd-bdf36f2d2a3e",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// HML 88b — Hungry Mist (alternate printing)
const HUNGRY_MIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUNGRY_MIST,
    1,
    "a486eab3-3a09-4bd5-ad97-8ec620434f7e",
    "Heather Hudson",
);

// HML 89 — Joven's Ferrets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOVEN_S_FERRETS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Joven's Ferrets",
    "1f95eda2-f791-46e9-bb82-31422b8c5ce4",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 90 — Leaping Lizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAPING_LIZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Leaping Lizard",
    "4b0e4744-4d73-4e6e-950b-bb4c83229499",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 91 — Mammoth Harness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAMMOTH_HARNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Mammoth Harness",
    "5be67121-068c-4770-bc42-c081577a442c",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// HML 92 — Primal Order
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIMAL_ORDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Primal Order",
    "21a3b8f7-c794-40ef-9ebd-dec5357260d4",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// HML 93 — Renewal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RENEWAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Renewal",
    "ab998cd1-2f49-42e7-b889-c6717b0ce884",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// HML 94 — Root Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOT_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Root Spider",
    "407d67b0-d496-401b-8844-8e3ea2fd2046",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// HML 95 — Roots
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Roots",
    "efb4c256-e790-41ec-a9ab-e6358e810798",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// HML 96 — Rysorian Badger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RYSORIAN_BADGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Rysorian Badger",
    "ab87a387-678b-4913-a0c7-85f0238cee26",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// HML 97a — Shrink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRINK: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Shrink",
    "30785867-32f7-46c9-94c2-775078e792ae",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// HML 97b — Shrink (alternate printing)
const SHRINK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHRINK,
    1,
    "d9f4eaa1-3c2b-4f5d-8d4e-98d153899873",
    "Liz Danforth",
);

// HML 98 — Spectral Bears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_BEARS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Spectral Bears",
    "7e13875f-f745-4afd-a830-33df9576dce8",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// HML 99a — Willow Faerie (alternate printing)
const WILLOW_FAERIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILLOW_FAERIE,
    1,
    "83ce80dc-86d9-4613-af98-c385ca5d1cf4",
    "Susan Van Camp",
);

// HML 99b — Willow Faerie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILLOW_FAERIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Willow Faerie",
    "0e777dfe-44ed-4e73-bf77-ef4c667092d4",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// HML 100 — Willow Priestess
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILLOW_PRIESTESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Willow Priestess",
    "c636a608-26d7-4154-8052-a093b11362b1",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// HML 101 — Apocalypse Chime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APOCALYPSE_CHIME: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Apocalypse Chime",
    "cef20d8f-6e80-4fca-b6a7-541981f6a112",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// HML 102 — Clockwork Gnomes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOCKWORK_GNOMES: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Clockwork Gnomes",
    "3e0ca2ea-e059-4742-8ce4-22876762048c",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// HML 103 — Clockwork Steed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOCKWORK_STEED: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Clockwork Steed",
    "9b080587-d062-42ff-abc5-8e04a20faece",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 104 — Clockwork Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOCKWORK_SWARM: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Clockwork Swarm",
    "dfd89e5c-79dc-4a57-b5ea-16491443fea1",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 105 — Didgeridoo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIDGERIDOO: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Didgeridoo",
    "828f8f68-abe2-4e39-b3e4-991dceacd5d9",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// HML 106 — Ebony Rhino
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EBONY_RHINO: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Ebony Rhino",
    "81db749e-a1df-4615-9449-94731fa23a9f",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 107 — Feroz's Ban
pub(in crate::card::sets) static FEROZ_S_BAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Feroz's Ban",
    "01ff4430-c8f7-408a-aad2-a098d747ea62",
    "Heather Hudson",
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(abilities::spell_cost_increase(
        "Creature spells cost {2} more to cast.",
        ObjectPredicateDef::HasType(CardType::Creature),
        PlayerRelation::Any,
        mana_cost!("{2}"),
    )),
);

// HML 108 — Joven's Tools
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOVEN_S_TOOLS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Joven's Tools",
    "d2520b38-76c1-45a2-9cda-a305f70762bd",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// HML 109 — Roterothopter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROTEROTHOPTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Roterothopter",
    "22148a1a-2172-4718-8ee4-08770eafed9f",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// HML 110 — Serrated Arrows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRATED_ARROWS: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Serrated Arrows",
    "849a7d2b-3fdb-4e7f-b0b6-f6559dcb32e2",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// HML 111 — An-Havva Township
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AN_HAVVA_TOWNSHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "An-Havva Township",
    "9afac347-4316-43e2-848b-e474ed563af6",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// HML 112 — Aysen Abbey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AYSEN_ABBEY: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Aysen Abbey",
    "2a2e669b-61b2-4729-b636-094796fb1d93",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// HML 113 — Castle Sengir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASTLE_SENGIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Castle Sengir",
    "16bfba30-4075-4bd6-9e4b-3a37641d43ce",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// HML 114 — Koskun Keep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KOSKUN_KEEP: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Koskun Keep",
    "395fe900-ed19-438e-a658-ed7cf85818e5",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// HML 115 — Wizards' School
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIZARDS_SCHOOL: CardRecord = CardRecord::new(
    crate::card::CardSet::Homelands,
    "Wizards' School",
    "cd736532-8e98-4f4a-b48f-a66c57efcbfd",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABBEY_GARGOYLES,
    &ABBEY_MATRON,
    &AYSEN_BUREAUCRATS,
    &AYSEN_CRUSADER,
    &AYSEN_HIGHWAY,
    &BEAST_WALKERS,
    &DEATH_SPEAKERS,
    &HAZDUHR_THE_ABBOT,
    &LEECHES,
    &MESA_FALCON,
    &PROPHECY,
    &RASHKA_THE_SLAYER,
    &SAMITE_ALCHEMIST,
    &SERRA_AVIARY,
    &SERRA_BESTIARY,
    &SERRA_INQUISITORS,
    &SERRA_PALADIN,
    &SORAYA_THE_FALCONER,
    &TRADE_CARAVAN,
    &TRUCE,
    &AETHER_STORM,
    &BAKI_S_CURSE,
    &CHAIN_STASIS,
    &CORAL_REEF,
    &DARK_MAZE,
    &FORGET,
    &GIANT_ALBATROSS,
    &GIANT_OYSTER,
    &JINX,
    &LABYRINTH_MINOTAUR,
    &MARJHAN,
    &MEMORY_LAPSE,
    &MERCHANT_SCROLL,
    &MYSTIC_DECREE,
    &NARWHAL,
    &REEF_PIRATES,
    &REVEKA_WIZARD_SAVANT,
    &SEA_SPRITE,
    &SEA_TROLL,
    &WALL_OF_KELP,
    &BARON_SENGIR,
    &BLACK_CARRIAGE,
    &BROKEN_VISAGE,
    &CEMETERY_GATE,
    &DRUDGE_SPELL,
    &DRY_SPELL,
    &FEAST_OF_THE_UNICORN,
    &FUNERAL_MARCH,
    &GHOST_HOUNDS,
    &GRANDMOTHER_SENGIR,
    &GREATER_WEREWOLF,
    &HEADSTONE,
    &IHSAN_S_SHADE,
    &IRINI_SENGIR,
    &KOSKUN_FALLS,
    &SENGIR_AUTOCRAT,
    &SENGIR_BATS,
    &TIMMERIAN_FIENDS,
    &TORTURE,
    &VELDRANE_OF_SENGIR,
    &ALIBAN_S_TOWER,
    &AMBUSH,
    &AMBUSH_PARTY,
    &AN_ZERRIN_RUINS,
    &ANABA_ANCESTOR,
    &ANABA_BODYGUARD,
    &ANABA_SHAMAN,
    &ANABA_SPIRIT_CRAFTER,
    &CHANDLER,
    &DWARVEN_PONY,
    &DWARVEN_SEA_CLAN,
    &DWARVEN_TRADER,
    &ERON_THE_RELENTLESS,
    &EVAPORATE,
    &HEART_WOLF,
    &IRONCLAW_CURSE,
    &JOVEN,
    &ORCISH_MINE,
    &RETRIBUTION,
    &WINTER_SKY,
    &AN_HAVVA_CONSTABLE,
    &AN_HAVVA_INN,
    &AUTUMN_WILLOW,
    &CARAPACE,
    &DAUGHTER_OF_AUTUMN,
    &FAERIE_NOBLE,
    &FOLK_OF_AN_HAVVA,
    &HUNGRY_MIST,
    &JOVEN_S_FERRETS,
    &LEAPING_LIZARD,
    &MAMMOTH_HARNESS,
    &PRIMAL_ORDER,
    &RENEWAL,
    &ROOT_SPIDER,
    &ROOTS,
    &RYSORIAN_BADGER,
    &SHRINK,
    &SPECTRAL_BEARS,
    &WILLOW_FAERIE,
    &WILLOW_PRIESTESS,
    &APOCALYPSE_CHIME,
    &CLOCKWORK_GNOMES,
    &CLOCKWORK_STEED,
    &CLOCKWORK_SWARM,
    &DIDGERIDOO,
    &EBONY_RHINO,
    &FEROZ_S_BAN,
    &JOVEN_S_TOOLS,
    &ROTEROTHOPTER,
    &SERRATED_ARROWS,
    &AN_HAVVA_TOWNSHIP,
    &AYSEN_ABBEY,
    &CASTLE_SENGIR,
    &KOSKUN_KEEP,
    &WIZARDS_SCHOOL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ABBEY_MATRON_ALTERNATE_1,
    AYSEN_BUREAUCRATS_ALTERNATE_1,
    MESA_FALCON_ALTERNATE_1,
    SAMITE_ALCHEMIST_ALTERNATE_1,
    TRADE_CARAVAN_ALTERNATE_1,
    DARK_MAZE_ALTERNATE_1,
    GIANT_ALBATROSS_ALTERNATE_1,
    LABYRINTH_MINOTAUR_ALTERNATE_1,
    MEMORY_LAPSE_ALTERNATE_1,
    REEF_PIRATES_ALTERNATE_1,
    CEMETERY_GATE_ALTERNATE_1,
    DRY_SPELL_ALTERNATE_1,
    FEAST_OF_THE_UNICORN_ALTERNATE_1,
    SENGIR_BATS_ALTERNATE_1,
    TORTURE_ALTERNATE_1,
    ALIBAN_S_TOWER_ALTERNATE_1,
    AMBUSH_PARTY_ALTERNATE_1,
    ANABA_BODYGUARD_ALTERNATE_1,
    ANABA_SHAMAN_ALTERNATE_1,
    DWARVEN_TRADER_ALTERNATE_1,
    CARAPACE_ALTERNATE_1,
    FOLK_OF_AN_HAVVA_ALTERNATE_1,
    HUNGRY_MIST_ALTERNATE_1,
    SHRINK_ALTERNATE_1,
    WILLOW_FAERIE_ALTERNATE_1,
];
