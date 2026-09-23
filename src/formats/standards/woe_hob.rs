//! Fixed paper Standard snapshot for the September 8, 2026 event corpus.
//!
//! Set window: <https://magic.wizards.com/en/news/feature/the-hobbit-release-notes>
//! Bans: <https://magic.wizards.com/en/banned-restricted-list> (checked 2026-09-22),
//! including <https://magic.wizards.com/en/news/announcements/banned-and-restricted-august-10-2026>.
//! Arena's best-of-one-only Leyline of Resonance ban does not apply.

use super::super::{CONSTRUCTED_RULES, SetFormatDefinition};
use crate::card::{CardSet, sets};

pub const ALLOWED_SETS: &[CardSet] = &[
    sets::wilds_of_eldraine::SET,
    sets::lost_caverns_of_ixalan::SET,
    sets::murders_at_karlov_manor::SET,
    sets::outlaws_of_thunder_junction::SET,
    sets::the_big_score::SET,
    sets::bloomburrow::SET,
    sets::duskmourn_house_of_horror::SET,
    sets::magic_foundations::SET,
    sets::aetherdrift::SET,
    sets::tarkir_dragonstorm::SET,
    sets::final_fantasy::SET,
    sets::edge_of_eternities::SET,
    sets::marvels_spider_man::SET,
    sets::avatar_the_last_airbender::SET,
    sets::lorwyn_eclipsed::SET,
    sets::teenage_mutant_ninja_turtles::SET,
    sets::secrets_of_strixhaven::SET,
    sets::marvel_super_heroes::SET,
    sets::the_hobbit::SET,
];

pub const BANNED_CARDS: &[&str] = &[
    "Abuelo's Awakening",
    "Badgermole Cub",
    "Cori-Steel Cutter",
    "Gran-Gran",
    "Heartfire Hero",
    "Hopeless Nightmare",
    "Monstrous Rage",
    "Proft's Eidetic Memory",
    "Screaming Nemesis",
    "Stormchaser's Talent",
    "This Town Ain't Big Enough",
    "Up the Beanstalk",
    "Vivi Ornitier",
];

pub(in crate::formats) const DEFINITION: SetFormatDefinition = SetFormatDefinition {
    rules: CONSTRUCTED_RULES,
    allowed_sets: ALLOWED_SETS,
    additional_allowed_cards: &[],
    banned_cards: BANNED_CARDS,
    restricted_cards: &[],
};
