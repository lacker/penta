use super::{FormatRules, SetFormatDefinition};
use crate::card::CardSet;

pub const BANNED_CARDS: &[&str] = &[
    "Bronze Tablet",
    "Contract from Below",
    "Darkpact",
    "Demonic Attorney",
    "Jeweled Bird",
    "Rebirth",
    "Tempest Efreet",
];

pub const ALLOWED_SETS: &[CardSet] = &[
    CardSet::Alpha,
    CardSet::Beta,
    CardSet::Unlimited,
    CardSet::CollectorsEdition,
    CardSet::InternationalCollectorsEdition,
    CardSet::ArabianNights,
    CardSet::Antiquities,
    CardSet::Revised,
    CardSet::Legends,
    CardSet::TheDark,
    CardSet::FallenEmpires,
    CardSet::Promo1994,
];

pub const RESTRICTED_CARDS: &[&str] = &[
    "Ancestral Recall",
    "Balance",
    "Black Lotus",
    "Braingeyser",
    "Chaos Orb",
    "Channel",
    "Demonic Tutor",
    "Library of Alexandria",
    "Mana Drain",
    "Mind Twist",
    "Mox Emerald",
    "Mox Jet",
    "Mox Pearl",
    "Mox Ruby",
    "Mox Sapphire",
    "Recall",
    "Regrowth",
    "Sol Ring",
    "Time Vault",
    "Time Walk",
    "Timetwister",
    "Wheel of Fortune",
];

pub(super) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: FormatRules {
        starting_life: 20,
        opening_hand_size: 7,
        minimum_main_deck_size: 60,
        maximum_sideboard_size: 15,
        maximum_copies: 4,
        mana_empties_at_end_of_step: false,
        mana_burn: true,
    },
    allowed_sets: ALLOWED_SETS,
    banned_cards: BANNED_CARDS,
    restricted_cards: RESTRICTED_CARDS,
};
