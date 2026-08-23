//! Chronicles has no unique card definitions.
//!
//! It reprinted cards from the sets before it, which is what brings some of
//! them inside the Premodern window.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1994::the_dark as catalog_drk;
use crate::card::sets::y1994::the_dark;

// CHR 1 — Abu Ja'far (reprint)

// CHR 2 — Akron Legionnaire (reprint)

// CHR 3 — Angelic Voices (reprint)

// CHR 4 — Blood of the Martyr (reprint)

// CHR 5 — D'Avenant Archer (reprint)

// CHR 6 — Divine Offering (reprint)

// CHR 7 — Indestructible Aura (reprint)

// CHR 8 — Ivory Guardians (reprint)

// CHR 9 — Keepers of the Faith (reprint)

// CHR 10 — Petra Sphinx (reprint)

// CHR 11 — Repentant Blacksmith (reprint)

// CHR 12 — Shield Wall (reprint)

// CHR 13 — War Elephant (reprint)

// CHR 14 — Witch Hunter (reprint)

// CHR 15 — Azure Drake (reprint)

// CHR 16 — Boomerang (reprint)

// CHR 17 — Dance of Many (reprint)

// CHR 18 — Dandân (reprint)

// CHR 19 — Enchantment Alteration (reprint)

// CHR 20 — Fishliver Oil (reprint)

// CHR 21 — Flash Flood (reprint)

// CHR 22 — Juxtapose (reprint)

// CHR 23 — Puppet Master (reprint)

// CHR 24 — Recall (reprint)

// CHR 25 — Remove Soul (reprint)

// CHR 26 — Teleport (reprint)

// CHR 27 — Wall of Vapor (reprint)

// CHR 28 — Wall of Wonder (reprint)

// CHR 29 — Banshee (reprint)

// CHR 30 — Bog Rats (reprint)

// CHR 31 — Cuombajj Witches (reprint)

// CHR 32 — Fallen Angel (reprint)

// CHR 33 — Giant Slug (reprint)

// CHR 34 — Hasran Ogress (reprint)

// CHR 35 — Hell's Caretaker (reprint)

// CHR 36 — Shimian Night Stalker (reprint)

// CHR 37 — Takklemaggot (reprint)

// CHR 38 — The Fallen (reprint)

// CHR 39 — The Wretched (reprint)

// CHR 40 — Transmutation (reprint)

// CHR 41 — Wall of Shadows (reprint)

// CHR 42 — Yawgmoth Demon (reprint)

// CHR 43 — Active Volcano (reprint)

// CHR 44 — Aladdin (reprint)

// CHR 45 — Beasts of Bogardan (reprint)

// CHR 46 — Blood Moon (reprint)

// CHR 47 — Fire Drake (reprint)

// CHR 48 — Goblin Artisans (reprint)

// CHR 49 — Goblin Digging Team (reprint)

// CHR 50 — Goblin Shrine (reprint)

// CHR 51 — Goblins of the Flarg (reprint)

// CHR 52 — Land's Edge (reprint)

// CHR 53 — Mountain Yeti (reprint)

// CHR 54 — Primordial Ooze (reprint)

// CHR 55 — Wall of Heat (reprint)

// CHR 56 — Wall of Opposition (reprint)

// CHR 57 — Argothian Pixies (reprint)

// CHR 58 — Cat Warriors (reprint)

// CHR 59 — Cocoon (reprint)

// CHR 60 — Concordant Crossroads (reprint)

// CHR 61 — Craw Giant (reprint)

// CHR 62 — Cyclone (reprint)

// CHR 63 — Emerald Dragonfly (reprint)

// CHR 64 — Erhnam Djinn (reprint)

// CHR 65 — Ghazbán Ogre (reprint)

// CHR 66 — Metamorphosis (reprint)

// CHR 67 — Rabid Wombat (reprint)

// CHR 68 — Revelation (reprint)

// CHR 69 — Scavenger Folk (reprint)

// CHR 70 — Storm Seeker (reprint)

// CHR 71 — Arcades Sabboth (reprint)

// CHR 72 — Axelrod Gunnarson (reprint)

// CHR 73 — Ayesha Tanaka (reprint)

// CHR 74 — Chromium (reprint)

// CHR 75 — Dakkon Blackblade (reprint)

// CHR 76 — Gabriel Angelfire (reprint)

// CHR 77 — Johan (reprint)

// CHR 78 — Kei Takahashi (reprint)

// CHR 79 — Marhault Elsdragon (reprint)

// CHR 80 — Nebuchadnezzar (reprint)

// CHR 81 — Nicol Bolas (reprint)

// CHR 82 — Palladia-Mors (reprint)

// CHR 83 — Rubinia Soulsinger (reprint)

// CHR 84 — Sivitri Scarzam (reprint)

// CHR 85 — Sol'kanar the Swamp King (reprint)

// CHR 86 — Stangg (reprint)

// CHR 87 — Tobias Andrion (reprint)

// CHR 88 — Tor Wauki (reprint)

// CHR 89 — Vaevictis Asmadi (reprint)

// CHR 90 — Xira Arien (reprint)

// CHR 91 — Arena of the Ancients (reprint)

// CHR 92 — Ashnod's Altar (reprint)

// CHR 93 — Ashnod's Transmogrant (reprint)

// CHR 94 — Barl's Cage (reprint)

// CHR 95 — Book of Rass (reprint)

// CHR 96 — Bronze Horse (reprint)

// CHR 97 — Feldon's Cane (reprint)

// CHR 98 — Fountain of Youth (reprint)

// CHR 99 — Gauntlets of Chaos (reprint)

// CHR 100 — Horn of Deafening (reprint)

// CHR 101 — Jalum Tome (reprint)

// CHR 102 — Jeweled Bird (reprint)

// CHR 103 — Living Armor (reprint)

// CHR 104 — Obelisk of Undoing (reprint)

// CHR 105 — Rakalite (reprint)

// CHR 106 — Runesword (reprint)

// CHR 107 — Sentinel (reprint)

// CHR 108 — Serpent Generator (reprint)

// CHR 109 — Tormod's Crypt (reprint)

// CHR 110 — Triassic Egg (reprint)

// CHR 111 — Voodoo Doll (reprint)

// CHR 112 — City of Brass (reprint)

// CHR 113 — Safe Haven (reprint)

// CHR 114a — Urza's Mine (reprint)

// CHR 114b — Urza's Mine (alternate printing)

// CHR 114c — Urza's Mine (alternate printing)

// CHR 114d — Urza's Mine (alternate printing)

// CHR 115a — Urza's Power Plant (reprint)

// CHR 115b — Urza's Power Plant (alternate printing)

// CHR 115c — Urza's Power Plant (alternate printing)

// CHR 115d — Urza's Power Plant (alternate printing)

// CHR 116a — Urza's Tower (reprint)

// CHR 116b — Urza's Tower (alternate printing)

// CHR 116c — Urza's Tower (alternate printing)

// CHR 116d — Urza's Tower (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_arn::ABU_JAFAR), // CHR 1
    PrintingRecord::reprint(&catalog_leg::AKRON_LEGIONNAIRE), // CHR 2
    PrintingRecord::reprint(&catalog_leg::ANGELIC_VOICES), // CHR 3
    PrintingRecord::reprint(&catalog_drk::BLOOD_OF_THE_MARTYR), // CHR 4
    PrintingRecord::reprint(&catalog_leg::DAVENANT_ARCHER), // CHR 5
    PrintingRecord::reprint(&catalog_leg::DIVINE_OFFERING), // CHR 6
    PrintingRecord::reprint(&catalog_leg::INDESTRUCTIBLE_AURA), // CHR 7
    PrintingRecord::reprint(&catalog_leg::IVORY_GUARDIANS), // CHR 8
    PrintingRecord::reprint(&catalog_leg::KEEPERS_OF_THE_FAITH), // CHR 9
    PrintingRecord::reprint(&catalog_leg::PETRA_SPHINX), // CHR 10
    PrintingRecord::reprint(&catalog_arn::REPENTANT_BLACKSMITH), // CHR 11
    PrintingRecord::reprint(&catalog_leg::SHIELD_WALL), // CHR 12
    PrintingRecord::reprint(&catalog_arn::WAR_ELEPHANT), // CHR 13
    PrintingRecord::reprint(&catalog_drk::WITCH_HUNTER), // CHR 14
    PrintingRecord::reprint(&catalog_leg::AZURE_DRAKE), // CHR 15
    PrintingRecord::reprint(&catalog_leg::BOOMERANG), // CHR 16
    PrintingRecord::reprint(&catalog_drk::DANCE_OF_MANY), // CHR 17
    PrintingRecord::reprint(&catalog_arn::DANDAN),    // CHR 18
    PrintingRecord::reprint(&catalog_leg::ENCHANTMENT_ALTERATION), // CHR 19
    PrintingRecord::reprint(&catalog_arn::FISHLIVER_OIL), // CHR 20
    PrintingRecord::reprint(&catalog_leg::FLASH_FLOOD), // CHR 21
    PrintingRecord::reprint(&catalog_leg::JUXTAPOSE), // CHR 22
    PrintingRecord::reprint(&catalog_leg::PUPPET_MASTER), // CHR 23
    PrintingRecord::reprint(&catalog_leg::RECALL),    // CHR 24
    PrintingRecord::reprint(&catalog_leg::REMOVE_SOUL), // CHR 25
    PrintingRecord::reprint(&catalog_leg::TELEPORT),  // CHR 26
    PrintingRecord::reprint(&catalog_leg::WALL_OF_VAPOR), // CHR 27
    PrintingRecord::reprint(&catalog_leg::WALL_OF_WONDER), // CHR 28
    PrintingRecord::reprint(&catalog_drk::BANSHEE),   // CHR 29
    PrintingRecord::reprint(&catalog_drk::BOG_RATS),  // CHR 30
    PrintingRecord::reprint(&catalog_arn::CUOMBAJJ_WITCHES), // CHR 31
    PrintingRecord::reprint(&catalog_leg::FALLEN_ANGEL), // CHR 32
    PrintingRecord::reprint(&catalog_leg::GIANT_SLUG), // CHR 33
    PrintingRecord::reprint(&catalog_arn::HASRAN_OGRESS), // CHR 34
    PrintingRecord::reprint(&catalog_leg::HELLS_CARETAKER), // CHR 35
    PrintingRecord::reprint(&catalog_leg::SHIMIAN_NIGHT_STALKER), // CHR 36
    PrintingRecord::reprint(&catalog_leg::TAKKLEMAGGOT), // CHR 37
    PrintingRecord::reprint(&catalog_drk::THE_FALLEN), // CHR 38
    PrintingRecord::reprint(&catalog_leg::THE_WRETCHED), // CHR 39
    PrintingRecord::reprint(&catalog_leg::TRANSMUTATION), // CHR 40
    PrintingRecord::reprint(&catalog_leg::WALL_OF_SHADOWS), // CHR 41
    PrintingRecord::reprint(&catalog_atq::YAWGMOTH_DEMON), // CHR 42
    PrintingRecord::reprint(&catalog_leg::ACTIVE_VOLCANO), // CHR 43
    PrintingRecord::reprint(&catalog_arn::ALADDIN),   // CHR 44
    PrintingRecord::reprint(&catalog_leg::BEASTS_OF_BOGARDAN), // CHR 45
    PrintingRecord::reprint(&catalog_drk::BLOOD_MOON), // CHR 46
    PrintingRecord::reprint(&catalog_drk::FIRE_DRAKE), // CHR 47
    PrintingRecord::reprint(&catalog_atq::GOBLIN_ARTISANS), // CHR 48
    PrintingRecord::reprint(&catalog_drk::GOBLIN_DIGGING_TEAM), // CHR 49
    PrintingRecord::reprint(&catalog_drk::GOBLIN_SHRINE), // CHR 50
    PrintingRecord::reprint(&catalog_drk::GOBLINS_OF_THE_FLARG), // CHR 51
    PrintingRecord::reprint(&catalog_leg::LAND_S_EDGE), // CHR 52
    PrintingRecord::reprint(&catalog_leg::MOUNTAIN_YETI), // CHR 53
    PrintingRecord::reprint(&catalog_leg::PRIMORDIAL_OOZE), // CHR 54
    PrintingRecord::reprint(&catalog_leg::WALL_OF_HEAT), // CHR 55
    PrintingRecord::reprint(&catalog_leg::WALL_OF_OPPOSITION), // CHR 56
    PrintingRecord::reprint(&catalog_atq::ARGOTHIAN_PIXIES), // CHR 57
    PrintingRecord::reprint(&catalog_leg::CAT_WARRIORS), // CHR 58
    PrintingRecord::reprint(&catalog_leg::COCOON),    // CHR 59
    PrintingRecord::reprint(&catalog_leg::CONCORDANT_CROSSROADS), // CHR 60
    PrintingRecord::reprint(&catalog_leg::CRAW_GIANT), // CHR 61
    PrintingRecord::reprint(&catalog_arn::CYCLONE),   // CHR 62
    PrintingRecord::reprint(&catalog_leg::EMERALD_DRAGONFLY), // CHR 63
    PrintingRecord::reprint(&catalog_arn::ERHNAM_DJINN), // CHR 64
    PrintingRecord::reprint(&catalog_arn::GHAZBAN_OGRE), // CHR 65
    PrintingRecord::reprint(&catalog_arn::METAMORPHOSIS), // CHR 66
    PrintingRecord::reprint(&catalog_leg::RABID_WOMBAT), // CHR 67
    PrintingRecord::reprint(&catalog_leg::REVELATION), // CHR 68
    PrintingRecord::reprint(&catalog_drk::SCAVENGER_FOLK), // CHR 69
    PrintingRecord::reprint(&catalog_leg::STORM_SEEKER), // CHR 70
    PrintingRecord::reprint(&catalog_leg::ARCADES_SABBOTH), // CHR 71
    PrintingRecord::reprint(&catalog_leg::AXELROD_GUNNARSON), // CHR 72
    PrintingRecord::reprint(&catalog_leg::AYESHA_TANAKA), // CHR 73
    PrintingRecord::reprint(&catalog_leg::CHROMIUM),  // CHR 74
    PrintingRecord::reprint(&catalog_leg::DAKKON_BLACKBLADE), // CHR 75
    PrintingRecord::reprint(&catalog_leg::GABRIEL_ANGELFIRE), // CHR 76
    PrintingRecord::reprint(&catalog_leg::JOHAN),     // CHR 77
    PrintingRecord::reprint(&catalog_leg::KEI_TAKAHASHI), // CHR 78
    PrintingRecord::reprint(&catalog_leg::MARHAULT_ELSDRAGON), // CHR 79
    PrintingRecord::reprint(&catalog_leg::NEBUCHADNEZZAR), // CHR 80
    PrintingRecord::reprint(&catalog_leg::NICOL_BOLAS), // CHR 81
    PrintingRecord::reprint(&catalog_leg::PALLADIA_MORS), // CHR 82
    PrintingRecord::reprint(&catalog_leg::RUBINIA_SOULSINGER), // CHR 83
    PrintingRecord::reprint(&catalog_leg::SIVITRI_SCARZAM), // CHR 84
    PrintingRecord::reprint(&catalog_leg::SOLKANAR_THE_SWAMP_KING), // CHR 85
    PrintingRecord::reprint(&catalog_leg::STANGG),    // CHR 86
    PrintingRecord::reprint(&catalog_leg::TOBIAS_ANDRION), // CHR 87
    PrintingRecord::reprint(&catalog_leg::TOR_WAUKI), // CHR 88
    PrintingRecord::reprint(&catalog_leg::VAEVICTIS_ASMADI), // CHR 89
    PrintingRecord::reprint(&catalog_leg::XIRA_ARIEN), // CHR 90
    PrintingRecord::reprint(&catalog_leg::ARENA_OF_THE_ANCIENTS), // CHR 91
    PrintingRecord::reprint(&catalog_atq::ASHNOD_S_ALTAR), // CHR 92
    PrintingRecord::reprint(&catalog_atq::ASHNODS_TRANSMOGRANT), // CHR 93
    PrintingRecord::reprint(&catalog_drk::BARLS_CAGE), // CHR 94
    PrintingRecord::reprint(&catalog_drk::BOOK_OF_RASS), // CHR 95
    PrintingRecord::reprint(&catalog_leg::BRONZE_HORSE), // CHR 96
    PrintingRecord::reprint(&catalog_atq::FELDONS_CANE), // CHR 97
    PrintingRecord::reprint(&catalog_drk::FOUNTAIN_OF_YOUTH), // CHR 98
    PrintingRecord::reprint(&catalog_leg::GAUNTLETS_OF_CHAOS), // CHR 99
    PrintingRecord::reprint(&catalog_leg::HORN_OF_DEAFENING), // CHR 100
    PrintingRecord::reprint(&catalog_atq::JALUM_TOME), // CHR 101
    PrintingRecord::reprint(&catalog_arn::JEWELED_BIRD), // CHR 102
    PrintingRecord::reprint(&catalog_drk::LIVING_ARMOR), // CHR 103
    PrintingRecord::reprint(&catalog_atq::OBELISK_OF_UNDOING), // CHR 104
    PrintingRecord::reprint(&catalog_atq::RAKALITE),  // CHR 105
    PrintingRecord::reprint(&catalog_drk::RUNESWORD), // CHR 106
    PrintingRecord::reprint(&catalog_leg::SENTINEL),  // CHR 107
    PrintingRecord::reprint(&catalog_leg::SERPENT_GENERATOR), // CHR 108
    PrintingRecord::reprint(&the_dark::TORMODS_CRYPT), // CHR 109
    PrintingRecord::reprint(&catalog_leg::TRIASSIC_EGG), // CHR 110
    PrintingRecord::reprint(&catalog_leg::VOODOO_DOLL), // CHR 111
    PrintingRecord::reprint(&catalog_arn::CITY_OF_BRASS), // CHR 112
    PrintingRecord::reprint(&catalog_drk::SAFE_HAVEN), // CHR 113
    PrintingRecord::reprint(&catalog_atq::URZA_S_MINE), // CHR 114a
    PrintingRecord::alternate(&catalog_atq::URZA_S_MINE, 1), // CHR 114b
    PrintingRecord::alternate(&catalog_atq::URZA_S_MINE, 2), // CHR 114c
    PrintingRecord::alternate(&catalog_atq::URZA_S_MINE, 3), // CHR 114d
    PrintingRecord::reprint(&catalog_atq::URZA_S_POWER_PLANT), // CHR 115a
    PrintingRecord::alternate(&catalog_atq::URZA_S_POWER_PLANT, 1), // CHR 115b
    PrintingRecord::alternate(&catalog_atq::URZA_S_POWER_PLANT, 2), // CHR 115c
    PrintingRecord::alternate(&catalog_atq::URZA_S_POWER_PLANT, 3), // CHR 115d
    PrintingRecord::reprint(&catalog_atq::URZA_S_TOWER), // CHR 116a
    PrintingRecord::alternate(&catalog_atq::URZA_S_TOWER, 1), // CHR 116b
    PrintingRecord::alternate(&catalog_atq::URZA_S_TOWER, 2), // CHR 116c
    PrintingRecord::alternate(&catalog_atq::URZA_S_TOWER, 3), // CHR 116d
];
