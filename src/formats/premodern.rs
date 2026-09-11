use super::{CONSTRUCTED_RULES, SetFormatDefinition};
use crate::card::CardSet;
use crate::card::sets;

/// The twenty-nine sets Premodern names, in release order. Portal and promo
/// printings are excluded by the format rather than by the card.
pub const ALLOWED_SETS: &[CardSet] = &[
    sets::fourth_edition::SET,
    sets::ice_age::SET,
    sets::chronicles::SET,
    sets::homelands::SET,
    sets::alliances::SET,
    sets::mirage::SET,
    sets::visions::SET,
    sets::fifth_edition::SET,
    sets::weatherlight::SET,
    sets::tempest::SET,
    sets::stronghold::SET,
    sets::exodus::SET,
    sets::urzas_saga::SET,
    sets::urzas_legacy::SET,
    sets::classic_sixth_edition::SET,
    sets::urzas_destiny::SET,
    sets::mercadian_masques::SET,
    sets::nemesis::SET,
    sets::prophecy::SET,
    sets::invasion::SET,
    sets::planeshift::SET,
    sets::seventh_edition::SET,
    sets::apocalypse::SET,
    sets::odyssey::SET,
    sets::torment::SET,
    sets::judgment::SET,
    sets::onslaught::SET,
    sets::legions::SET,
    sets::scourge::SET,
];

pub const BANNED_CARDS: &[&str] = &[
    "Amulet of Quoz",
    "Balance",
    "Brainstorm",
    "Bronze Tablet",
    "Channel",
    "Demonic Consultation",
    "Earthcraft",
    "Entomb",
    "Flash",
    "Force of Will",
    "Goblin Recruiter",
    "Grim Monolith",
    "Jeweled Bird",
    "Land Tax",
    "Mana Vault",
    "Memory Jar",
    "Mind Twist",
    "Mind's Desire",
    "Mystical Tutor",
    "Necropotence",
    "Parallax Tide",
    "Rebirth",
    "Strip Mine",
    "Tempest Efreet",
    "Tendrils of Agony",
    "Time Spiral",
    "Timmerian Fiends",
    "Tolarian Academy",
    "Vampiric Tutor",
    "Windfall",
    "Worldgorger Dragon",
    "Yawgmoth's Bargain",
    "Yawgmoth's Will",
];

pub const RESTRICTED_CARDS: &[&str] = &[];

pub(super) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: CONSTRUCTED_RULES,
    allowed_sets: ALLOWED_SETS,
    additional_allowed_cards: &[],
    banned_cards: BANNED_CARDS,
    restricted_cards: RESTRICTED_CARDS,
};
