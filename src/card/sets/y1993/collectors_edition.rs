//! Collector's Edition has no unique card definitions.
//!
//! Every card in the built-in Collector's Edition catalog points to its first printing.

use super::{CardRecord, PrintingRecord, alpha, beta};

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::ARMAGEDDON),     // CED 2
    PrintingRecord::reprint(&alpha::BALANCE),        // CED 3
    PrintingRecord::reprint(&alpha::CRUSADE),        // CED 17
    PrintingRecord::reprint(&alpha::DISENCHANT),     // CED 19
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL), // CED 22
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS), // CED 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),    // CED 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // CED 41
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),   // CED 44
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),   // CED 46
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL), // CED 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT), // CED 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // CED 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),    // CED 51
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),  // CED 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),   // CED 55
    PrintingRecord::reprint(&alpha::MANA_SHORT),     // CED 66
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),  // CED 75
    PrintingRecord::reprint(&alpha::STASIS),         // CED 81
    PrintingRecord::reprint(&alpha::TIME_WALK),      // CED 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),    // CED 85
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),   // CED 95
    PrintingRecord::reprint(&alpha::DARK_RITUAL),    // CED 99
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),  // CED 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),     // CED 106
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER), // CED 113
    PrintingRecord::reprint(&alpha::MIND_TWIST),     // CED 116
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE), // CED 128
    PrintingRecord::reprint(&alpha::SINKHOLE),       // CED 130
    PrintingRecord::reprint(&alpha::TERROR),         // CED 131
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),   // CED 142
    PrintingRecord::reprint(&alpha::EARTHQUAKE),     // CED 147
    PrintingRecord::reprint(&alpha::FIREBALL),       // CED 150
    PrintingRecord::reprint(&alpha::FORK),           // CED 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // CED 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),    // CED 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE), // CED 156
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),  // CED 160
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT), // CED 162
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // CED 170
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),    // CED 173
    PrintingRecord::reprint(&alpha::SHATTER),        // CED 174
    PrintingRecord::reprint(&alpha::SMOKE),          // CED 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),    // CED 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),     // CED 178
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE), // CED 184
    PrintingRecord::reprint(&alpha::BERSERK),        // CED 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE), // CED 187
    PrintingRecord::reprint(&alpha::CHANNEL),        // CED 189
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),   // CED 198
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES), // CED 211
    PrintingRecord::reprint(&alpha::REGROWTH),       // CED 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),  // CED 216
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA), // CED 231
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),    // CED 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),     // CED 234
    PrintingRecord::reprint(&alpha::CHAOS_ORB),      // CED 236
    PrintingRecord::reprint(&alpha::COPPER_TABLET),  // CED 239
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA), // CED 246
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR), // CED 249
    PrintingRecord::reprint(&alpha::IRON_STAR),      // CED 251
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),  // CED 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),     // CED 256
    PrintingRecord::reprint(&alpha::MANA_VAULT),     // CED 260
    PrintingRecord::reprint(&alpha::MOX_EMERALD),    // CED 262
    PrintingRecord::reprint(&alpha::MOX_JET),        // CED 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),      // CED 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),       // CED 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),   // CED 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK), // CED 267
    PrintingRecord::reprint(&alpha::SOL_RING),       // CED 270
    PrintingRecord::reprint(&alpha::TIME_VAULT),     // CED 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),     // CED 276
    PrintingRecord::reprint(&alpha::BADLANDS),       // CED 278
    PrintingRecord::reprint(&alpha::BAYOU),          // CED 279
    PrintingRecord::reprint(&alpha::PLATEAU),        // CED 280
    PrintingRecord::reprint(&alpha::SAVANNAH),       // CED 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),      // CED 282
    PrintingRecord::reprint(&alpha::TAIGA),          // CED 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND), // CED 284
    PrintingRecord::reprint(&alpha::TUNDRA),         // CED 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA), // CED 286
    PrintingRecord::reprint(&beta::VOLCANIC_ISLAND), // CED 287
    PrintingRecord::reprint(&alpha::PLAINS),         // CED 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),    // CED 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),    // CED 290
    PrintingRecord::reprint(&alpha::ISLAND),         // CED 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),    // CED 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),    // CED 293
    PrintingRecord::reprint(&alpha::SWAMP),          // CED 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),     // CED 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),     // CED 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),       // CED 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),  // CED 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),  // CED 299
    PrintingRecord::reprint(&alpha::FOREST),         // CED 300
    PrintingRecord::alternate(&alpha::FOREST, 1),    // CED 301
    PrintingRecord::alternate(&alpha::FOREST, 2),    // CED 302
];
