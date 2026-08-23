//! The keyword tag carried by a checkpoint.
//!
//! Its variants are wire tags, so they are named for the printed keywords a
//! reader recognizes rather than for the engine's internal shape. Landwalk is
//! one parameterized keyword inside the engine and five separate tags here.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum KeywordSnapshot {
    Convoke,
    Delve,
    Improvise,
    Devoid,
    Compleated,
    SplitSecond,
    Infect,
    Flying,
    Trample,
    Haste,
    FirstStrike,
    DoubleStrike,
    Banding,
    BandsWithOtherLegendaryCreatures,
    BandsWithOtherWolvesOfTheHunt,
    Vigilance,
    Defender,
    Deathtouch,
    Lifelink,
    Reach,
    Flash,
    Hexproof,
    Shroud,
    Unleash,
    Intimidate,
    Menace,
    Undying,
    Indestructible,
    AttacksEachCombatIfAble,
    Mountainwalk,
    Forestwalk,
    Plainswalk,
    LegendaryLandwalk,
    Islandwalk,
    Swampwalk,
    ProtectionFromWhite,
    ProtectionFromBlue,
    ProtectionFromBlack,
    ProtectionFromRed,
    ProtectionFromGreen,
    ProtectionFromColorless,
    ProtectionFromZombies,
    ProtectionFromVampires,
    ProtectionFromWerewolves,
    ProtectionFromVampiresWerewolvesAndZombies,
    ProtectionFromCreatures,
    ProtectionFromMulticolored,
    ProtectionFromNonHumanCreatures,
    ProtectionFromEnchantments,
    ProtectionFromInstantAndSorcerySpells,
    ProtectionFromColoredSpells,
    ProtectionFromChosenPlayer,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpkeepKeywordSnapshot {
    pub(super) seat: usize,
    pub(super) keyword: KeywordSnapshot,
}
