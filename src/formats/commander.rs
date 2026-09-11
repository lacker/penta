//! Commander-family format metadata.
//!
//! This is deliberately data rather than a legality implementation. The
//! engine can start authored two-player cEDH games, but Commander deck
//! construction is deferred until colour identity, all commander permissions,
//! and the command-zone rules share one validator.

use super::{CommanderFormatDefinition, FormatRules};

pub(super) const RULES: FormatRules = FormatRules {
    starting_life: 40,
    opening_hand_size: 7,
    // Legacy single-commander baseline; total_deck_size describes construction.
    minimum_main_deck_size: 99,
    maximum_sideboard_size: 0,
    maximum_copies: 1,
    mana_empties_at_end_of_step: true,
    mana_burn: false,
};

/// Named Commander bans listed by Wizards of the Coast on 2026-09-11.
///
/// Source: <https://magic.wizards.com/en/banned-restricted-list>.
///
/// This does not attempt to encode the page's category-wide Conspiracy, ante,
/// or offensive-content policies. `Lutri, the Spellchaser` is intentionally
/// absent: Wizards lists it as banned only as a companion.
pub const BANNED_CARDS: &[&str] = &[
    "Ancestral Recall",
    "Balance",
    "Black Lotus",
    "Chaos Orb",
    "Channel",
    "Dockside Extortionist",
    "Emrakul, the Aeons Torn",
    "Erayo, Soratami Ascendant",
    "Falling Star",
    "Fastbond",
    "Flash",
    "Golos, Tireless Pilgrim",
    "Griselbrand",
    "Hullbreacher",
    "Iona, Shield of Emeria",
    "Jeweled Lotus",
    "Karakas",
    "Leovold, Emissary of Trest",
    "Library of Alexandria",
    "Limited Resources",
    "Mana Crypt",
    "Mox Emerald",
    "Mox Jet",
    "Mox Pearl",
    "Mox Ruby",
    "Mox Sapphire",
    "Nadu, Winged Wisdom",
    "Paradox Engine",
    "Primeval Titan",
    "Prophet of Kruphix",
    "Recurring Nightmare",
    "Rofellos, Llanowar Emissary",
    "Shahrazad",
    "Sundering Titan",
    "Sylvan Primordial",
    "Time Vault",
    "Time Walk",
    "Tinker",
    "Tolarian Academy",
    "Trade Secrets",
    "Upheaval",
    "Yawgmoth's Bargain",
];

/// A companion-only restriction is separate from the Commander ban list.
pub const COMPANION_ONLY_BANNED_CARDS: &[&str] = &["Lutri, the Spellchaser"];

pub(super) const DEFINITION: CommanderFormatDefinition = CommanderFormatDefinition {
    rules: RULES,
    total_deck_size: 100,
    banned_cards: BANNED_CARDS,
    companion_only_banned_cards: COMPANION_ONLY_BANNED_CARDS,
};
