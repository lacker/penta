//! Limited Edition Beta card definitions and printings.

use super::{CardRecord, PrintingRecord, alpha};
use crate::card::{
    AbilityCostDef, CardArt, CardRules, CardSet, ManaColor, ObjectPredicateDef, abilities, cards,
};
use crate::mana_cost;

// LEB 10 — Circle of Protection: Black
pub(in crate::card::sets) static CIRCLE_OF_PROTECTION_BLACK: CardRecord = CardRecord::new(
    cards::CIRCLE_OF_PROTECTION_BLACK,
    "Circle of Protection: Black",
    CardArt::new("fa47b4cd-8da4-4544-b011-ba92b7009203", "Jesper Myrfors"),
    CardSet::Beta,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        abilities::circle_of_protection(
            "{1}: The next time a black source of your choice would deal damage to you this turn, prevent that damage.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            ObjectPredicateDef::Color(ManaColor::Black),
        ),
    ),
);

// LEB 287 — Volcanic Island
pub(in crate::card::sets) static VOLCANIC_ISLAND: CardRecord = CardRecord::new(
    cards::VOLCANIC_ISLAND,
    "Volcanic Island",
    CardArt::new("0324641d-af55-4c53-b4dc-c8262e967da5", "Brian Snõddy"),
    CardSet::Beta,
    CardRules::new_land(&["Island", "Mountain"]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&CIRCLE_OF_PROTECTION_BLACK, &VOLCANIC_ISLAND];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&alpha::ARMAGEDDON),     // LEB 2
    PrintingRecord::reprint(&alpha::BALANCE),        // LEB 3
    PrintingRecord::reprint(&alpha::CRUSADE),        // LEB 17
    PrintingRecord::reprint(&alpha::DISENCHANT),     // LEB 19
    PrintingRecord::reprint(&alpha::SAVANNAH_LIONS), // LEB 39
    PrintingRecord::reprint(&alpha::SERRA_ANGEL),    // LEB 40
    PrintingRecord::reprint(&alpha::SWORDS_TO_PLOWSHARES), // LEB 41
    PrintingRecord::reprint(&alpha::WHITE_KNIGHT),   // LEB 44
    PrintingRecord::reprint(&alpha::WRATH_OF_GOD),   // LEB 46
    PrintingRecord::reprint(&alpha::ANCESTRAL_RECALL), // LEB 48
    PrintingRecord::reprint(&alpha::BLUE_ELEMENTAL_BLAST), // LEB 50
    PrintingRecord::reprint(&alpha::BRAINGEYSER),    // LEB 51
    PrintingRecord::reprint(&alpha::COPY_ARTIFACT),  // LEB 54
    PrintingRecord::reprint(&alpha::COUNTERSPELL),   // LEB 55
    PrintingRecord::reprint(&alpha::MANA_SHORT),     // LEB 66
    PrintingRecord::reprint(&alpha::PSIONIC_BLAST),  // LEB 75
    PrintingRecord::reprint(&alpha::STASIS),         // LEB 81
    PrintingRecord::reprint(&alpha::TIME_WALK),      // LEB 84
    PrintingRecord::reprint(&alpha::TIMETWISTER),    // LEB 85
    PrintingRecord::reprint(&alpha::ANIMATE_DEAD),   // LEB 93
    PrintingRecord::reprint(&alpha::BLACK_KNIGHT),   // LEB 95
    PrintingRecord::reprint(&alpha::DARK_RITUAL),    // LEB 99
    PrintingRecord::reprint(&alpha::DEMONIC_TUTOR),  // LEB 105
    PrintingRecord::reprint(&alpha::DRAIN_LIFE),     // LEB 106
    PrintingRecord::reprint(&alpha::HYPNOTIC_SPECTER), // LEB 113
    PrintingRecord::reprint(&alpha::MIND_TWIST),     // LEB 116
    PrintingRecord::reprint(&alpha::SENGIR_VAMPIRE), // LEB 128
    PrintingRecord::reprint(&alpha::SINKHOLE),       // LEB 130
    PrintingRecord::reprint(&alpha::TERROR),         // LEB 131
    PrintingRecord::reprint(&alpha::DRAGON_WHELP),   // LEB 142
    PrintingRecord::reprint(&alpha::EARTHQUAKE),     // LEB 147
    PrintingRecord::reprint(&alpha::FIREBALL),       // LEB 150
    PrintingRecord::reprint(&alpha::FORK),           // LEB 153
    PrintingRecord::reprint(&alpha::GOBLIN_BALLOON_BRIGADE), // LEB 154
    PrintingRecord::reprint(&alpha::GOBLIN_KING),    // LEB 155
    PrintingRecord::reprint(&alpha::GRANITE_GARGOYLE), // LEB 156
    PrintingRecord::reprint(&alpha::IRONCLAW_ORCS),  // LEB 160
    PrintingRecord::reprint(&alpha::LIGHTNING_BOLT), // LEB 162
    PrintingRecord::reprint(&alpha::RED_ELEMENTAL_BLAST), // LEB 170
    PrintingRecord::reprint(&alpha::SEDGE_TROLL),    // LEB 173
    PrintingRecord::reprint(&alpha::SHATTER),        // LEB 174
    PrintingRecord::reprint(&alpha::SMOKE),          // LEB 176
    PrintingRecord::reprint(&alpha::STONE_GIANT),    // LEB 177
    PrintingRecord::reprint(&alpha::STONE_RAIN),     // LEB 178
    PrintingRecord::reprint(&alpha::WHEEL_OF_FORTUNE), // LEB 184
    PrintingRecord::reprint(&alpha::BERSERK),        // LEB 186
    PrintingRecord::reprint(&alpha::BIRDS_OF_PARADISE), // LEB 187
    PrintingRecord::reprint(&alpha::CHANNEL),        // LEB 189
    PrintingRecord::reprint(&alpha::GIANT_GROWTH),   // LEB 198
    PrintingRecord::reprint(&alpha::LLANOWAR_ELVES), // LEB 211
    PrintingRecord::reprint(&alpha::REGROWTH),       // LEB 215
    PrintingRecord::reprint(&alpha::SCRYB_SPRITES),  // LEB 216
    PrintingRecord::reprint(&alpha::ANKH_OF_MISHRA), // LEB 231
    PrintingRecord::reprint(&alpha::BLACK_LOTUS),    // LEB 233
    PrintingRecord::reprint(&alpha::BLACK_VISE),     // LEB 234
    PrintingRecord::reprint(&alpha::CHAOS_ORB),      // LEB 236
    PrintingRecord::reprint(&alpha::COPPER_TABLET),  // LEB 239
    PrintingRecord::reprint(&alpha::GLASSES_OF_URZA), // LEB 246
    PrintingRecord::reprint(&alpha::ICY_MANIPULATOR), // LEB 249
    PrintingRecord::reprint(&alpha::IRON_STAR),      // LEB 251
    PrintingRecord::reprint(&alpha::JAYEMDAE_TOME),  // LEB 255
    PrintingRecord::reprint(&alpha::JUGGERNAUT),     // LEB 256
    PrintingRecord::reprint(&alpha::MANA_VAULT),     // LEB 260
    PrintingRecord::reprint(&alpha::MOX_EMERALD),    // LEB 262
    PrintingRecord::reprint(&alpha::MOX_JET),        // LEB 263
    PrintingRecord::reprint(&alpha::MOX_PEARL),      // LEB 264
    PrintingRecord::reprint(&alpha::MOX_RUBY),       // LEB 265
    PrintingRecord::reprint(&alpha::MOX_SAPPHIRE),   // LEB 266
    PrintingRecord::reprint(&alpha::NEVINYRRALS_DISK), // LEB 267
    PrintingRecord::reprint(&alpha::SOL_RING),       // LEB 270
    PrintingRecord::reprint(&alpha::TIME_VAULT),     // LEB 275
    PrintingRecord::reprint(&alpha::WINTER_ORB),     // LEB 276
    PrintingRecord::reprint(&alpha::BADLANDS),       // LEB 278
    PrintingRecord::reprint(&alpha::BAYOU),          // LEB 279
    PrintingRecord::reprint(&alpha::PLATEAU),        // LEB 280
    PrintingRecord::reprint(&alpha::SAVANNAH),       // LEB 281
    PrintingRecord::reprint(&alpha::SCRUBLAND),      // LEB 282
    PrintingRecord::reprint(&alpha::TAIGA),          // LEB 283
    PrintingRecord::reprint(&alpha::TROPICAL_ISLAND), // LEB 284
    PrintingRecord::reprint(&alpha::TUNDRA),         // LEB 285
    PrintingRecord::reprint(&alpha::UNDERGROUND_SEA), // LEB 286
    PrintingRecord::reprint(&alpha::PLAINS),         // LEB 288
    PrintingRecord::alternate(&alpha::PLAINS, 1),    // LEB 289
    PrintingRecord::alternate(&alpha::PLAINS, 2),    // LEB 290
    PrintingRecord::reprint(&alpha::ISLAND),         // LEB 291
    PrintingRecord::alternate(&alpha::ISLAND, 1),    // LEB 292
    PrintingRecord::alternate(&alpha::ISLAND, 2),    // LEB 293
    PrintingRecord::reprint(&alpha::SWAMP),          // LEB 294
    PrintingRecord::alternate(&alpha::SWAMP, 1),     // LEB 295
    PrintingRecord::alternate(&alpha::SWAMP, 2),     // LEB 296
    PrintingRecord::reprint(&alpha::MOUNTAIN),       // LEB 297
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),  // LEB 298
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),  // LEB 299
    PrintingRecord::reprint(&alpha::FOREST),         // LEB 300
    PrintingRecord::alternate(&alpha::FOREST, 1),    // LEB 301
    PrintingRecord::alternate(&alpha::FOREST, 2),    // LEB 302
];
