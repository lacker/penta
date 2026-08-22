//! Unlimited Edition has no unique card definitions.
//!
//! Every card in the built-in Unlimited catalog points to its first printing.

use super::{CardRecord, PrintingRecord, alpha, beta};

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::ARMAGEDDON),     // 2ED 2
    PrintingRecord::reprint(&alpha::BALANCE),        // 2ED 3
    PrintingRecord::reprint(&alpha::CRUSADE),        // 2ED 17
    PrintingRecord::reprint(&alpha::DISENCHANT),     // 2ED 19
    PrintingRecord::reprint(&alpha::GUARDIAN_ANGEL), // 2ED 22
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS), // 2ED 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),    // 2ED 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // 2ED 41
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),   // 2ED 44
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),   // 2ED 46
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL), // 2ED 48
    PrintingRecord::reprint(&alpha::ANIMATE_ARTIFACT), // 2ED 49
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // 2ED 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),    // 2ED 51
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),  // 2ED 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),   // 2ED 55
    PrintingRecord::reprint(&alpha::MANA_SHORT),     // 2ED 66
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),  // 2ED 75
    PrintingRecord::reprint(&alpha::STASIS),         // 2ED 81
    PrintingRecord::reprint(&alpha::TIME_WALK),      // 2ED 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),    // 2ED 85
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),   // 2ED 95
    PrintingRecord::reprint(&alpha::DARK_RITUAL),    // 2ED 99
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),  // 2ED 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),     // 2ED 106
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER), // 2ED 113
    PrintingRecord::reprint(&alpha::MIND_TWIST),     // 2ED 116
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE), // 2ED 128
    PrintingRecord::reprint(&alpha::SINKHOLE),       // 2ED 130
    PrintingRecord::reprint(&alpha::TERROR),         // 2ED 131
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),   // 2ED 142
    PrintingRecord::reprint(&alpha::EARTHQUAKE),     // 2ED 147
    PrintingRecord::reprint(&alpha::FIREBALL),       // 2ED 150
    PrintingRecord::reprint(&alpha::FORK),           // 2ED 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // 2ED 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),    // 2ED 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE), // 2ED 156
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),  // 2ED 160
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT), // 2ED 162
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // 2ED 170
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),    // 2ED 173
    PrintingRecord::reprint(&alpha::SHATTER),        // 2ED 174
    PrintingRecord::reprint(&alpha::SMOKE),          // 2ED 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),    // 2ED 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),     // 2ED 178
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE), // 2ED 184
    PrintingRecord::reprint(&alpha::BERSERK),        // 2ED 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE), // 2ED 187
    PrintingRecord::reprint(&alpha::CHANNEL),        // 2ED 189
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),   // 2ED 198
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES), // 2ED 211
    PrintingRecord::reprint(&alpha::REGROWTH),       // 2ED 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),  // 2ED 216
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA), // 2ED 231
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),    // 2ED 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),     // 2ED 234
    PrintingRecord::reprint(&alpha::CHAOS_ORB),      // 2ED 236
    PrintingRecord::reprint(&alpha::COPPER_TABLET),  // 2ED 239
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA), // 2ED 246
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR), // 2ED 249
    PrintingRecord::reprint(&alpha::IRON_STAR),      // 2ED 251
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),  // 2ED 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),     // 2ED 256
    PrintingRecord::reprint(&alpha::MANA_VAULT),     // 2ED 260
    PrintingRecord::reprint(&alpha::MOX_EMERALD),    // 2ED 262
    PrintingRecord::reprint(&alpha::MOX_JET),        // 2ED 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),      // 2ED 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),       // 2ED 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),   // 2ED 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK), // 2ED 267
    PrintingRecord::reprint(&alpha::SOL_RING),       // 2ED 270
    PrintingRecord::reprint(&alpha::TIME_VAULT),     // 2ED 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),     // 2ED 276
    PrintingRecord::reprint(&alpha::BADLANDS),       // 2ED 278
    PrintingRecord::reprint(&alpha::BAYOU),          // 2ED 279
    PrintingRecord::reprint(&alpha::PLATEAU),        // 2ED 280
    PrintingRecord::reprint(&alpha::SAVANNAH),       // 2ED 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),      // 2ED 282
    PrintingRecord::reprint(&alpha::TAIGA),          // 2ED 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND), // 2ED 284
    PrintingRecord::reprint(&alpha::TUNDRA),         // 2ED 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA), // 2ED 286
    PrintingRecord::reprint(&beta::VOLCANIC_ISLAND), // 2ED 287
    PrintingRecord::reprint(&alpha::PLAINS),         // 2ED 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),    // 2ED 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),    // 2ED 290
    PrintingRecord::reprint(&alpha::ISLAND),         // 2ED 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),    // 2ED 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),    // 2ED 293
    PrintingRecord::reprint(&alpha::SWAMP),          // 2ED 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),     // 2ED 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),     // 2ED 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),       // 2ED 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),  // 2ED 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),  // 2ED 299
    PrintingRecord::reprint(&alpha::FOREST),         // 2ED 300
    PrintingRecord::alternate(&alpha::FOREST, 1),    // 2ED 301
    PrintingRecord::alternate(&alpha::FOREST, 2),    // 2ED 302
];
