//! International Collector's Edition has no unique card definitions.
//!
//! Every card in the built-in International Collector's Edition catalog points to its first
//! printing.

use super::{CardRecord, PrintingRecord, alpha, beta};

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::ARMAGEDDON),     // CEI 2
    PrintingRecord::reprint(&alpha::BALANCE),        // CEI 3
    PrintingRecord::reprint(&alpha::CRUSADE),        // CEI 17
    PrintingRecord::reprint(&alpha::DISENCHANT),     // CEI 19
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL), // CEI 22
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS), // CEI 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),    // CEI 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // CEI 41
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),   // CEI 44
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),   // CEI 46
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL), // CEI 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT), // CEI 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // CEI 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),    // CEI 51
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),  // CEI 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),   // CEI 55
    PrintingRecord::reprint(&alpha::MANA_SHORT),     // CEI 66
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),  // CEI 75
    PrintingRecord::reprint(&alpha::STASIS),         // CEI 81
    PrintingRecord::reprint(&alpha::TIME_WALK),      // CEI 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),    // CEI 85
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),   // CEI 95
    PrintingRecord::reprint(&alpha::DARK_RITUAL),    // CEI 99
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),  // CEI 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),     // CEI 106
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER), // CEI 113
    PrintingRecord::reprint(&alpha::MIND_TWIST),     // CEI 116
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE), // CEI 128
    PrintingRecord::reprint(&alpha::SINKHOLE),       // CEI 130
    PrintingRecord::reprint(&alpha::TERROR),         // CEI 131
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),   // CEI 142
    PrintingRecord::reprint(&alpha::EARTHQUAKE),     // CEI 147
    PrintingRecord::reprint(&alpha::FIREBALL),       // CEI 150
    PrintingRecord::reprint(&alpha::FORK),           // CEI 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // CEI 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),    // CEI 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE), // CEI 156
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),  // CEI 160
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT), // CEI 162
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // CEI 170
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),    // CEI 173
    PrintingRecord::reprint(&alpha::SHATTER),        // CEI 174
    PrintingRecord::reprint(&alpha::SMOKE),          // CEI 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),    // CEI 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),     // CEI 178
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE), // CEI 184
    PrintingRecord::reprint(&alpha::BERSERK),        // CEI 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE), // CEI 187
    PrintingRecord::reprint(&alpha::CHANNEL),        // CEI 189
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),   // CEI 198
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES), // CEI 211
    PrintingRecord::reprint(&alpha::REGROWTH),       // CEI 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),  // CEI 216
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA), // CEI 231
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),    // CEI 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),     // CEI 234
    PrintingRecord::reprint(&alpha::CHAOS_ORB),      // CEI 236
    PrintingRecord::reprint(&alpha::COPPER_TABLET),  // CEI 239
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA), // CEI 246
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR), // CEI 249
    PrintingRecord::reprint(&alpha::IRON_STAR),      // CEI 251
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),  // CEI 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),     // CEI 256
    PrintingRecord::reprint(&alpha::MANA_VAULT),     // CEI 260
    PrintingRecord::reprint(&alpha::MOX_EMERALD),    // CEI 262
    PrintingRecord::reprint(&alpha::MOX_JET),        // CEI 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),      // CEI 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),       // CEI 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),   // CEI 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK), // CEI 267
    PrintingRecord::reprint(&alpha::SOL_RING),       // CEI 270
    PrintingRecord::reprint(&alpha::TIME_VAULT),     // CEI 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),     // CEI 276
    PrintingRecord::reprint(&alpha::BADLANDS),       // CEI 278
    PrintingRecord::reprint(&alpha::BAYOU),          // CEI 279
    PrintingRecord::reprint(&alpha::PLATEAU),        // CEI 280
    PrintingRecord::reprint(&alpha::SAVANNAH),       // CEI 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),      // CEI 282
    PrintingRecord::reprint(&alpha::TAIGA),          // CEI 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND), // CEI 284
    PrintingRecord::reprint(&alpha::TUNDRA),         // CEI 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA), // CEI 286
    PrintingRecord::reprint(&beta::VOLCANIC_ISLAND), // CEI 287
    PrintingRecord::reprint(&alpha::PLAINS),         // CEI 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),    // CEI 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),    // CEI 290
    PrintingRecord::reprint(&alpha::ISLAND),         // CEI 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),    // CEI 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),    // CEI 293
    PrintingRecord::reprint(&alpha::SWAMP),          // CEI 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),     // CEI 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),     // CEI 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),       // CEI 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),  // CEI 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),  // CEI 299
    PrintingRecord::reprint(&alpha::FOREST),         // CEI 300
    PrintingRecord::alternate(&alpha::FOREST, 1),    // CEI 301
    PrintingRecord::alternate(&alpha::FOREST, 2),    // CEI 302
];
