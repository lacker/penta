//! Stable checkpoint tags for runtime keyword state.

use super::super::model_keyword::KeywordSnapshot;
use crate::card::{
    BandingQuality, BasicLandType, CardType, KeywordAbility, ManaColor, ObjectPredicateDef,
    PlayerRelation,
};

pub(in crate::game::state_checkpoint) fn keyword_snapshot(
    keyword: KeywordAbility,
) -> KeywordSnapshot {
    match keyword {
        KeywordAbility::Convoke => KeywordSnapshot::Convoke,
        KeywordAbility::Delve => KeywordSnapshot::Delve,
        KeywordAbility::Improvise => KeywordSnapshot::Improvise,
        KeywordAbility::Devoid => KeywordSnapshot::Devoid,
        KeywordAbility::Compleated => KeywordSnapshot::Compleated,
        KeywordAbility::SplitSecond => KeywordSnapshot::SplitSecond,
        KeywordAbility::Infect => KeywordSnapshot::Infect,
        KeywordAbility::Flying => KeywordSnapshot::Flying,
        KeywordAbility::Trample => KeywordSnapshot::Trample,
        KeywordAbility::Haste => KeywordSnapshot::Haste,
        KeywordAbility::FirstStrike => KeywordSnapshot::FirstStrike,
        KeywordAbility::DoubleStrike => KeywordSnapshot::DoubleStrike,
        KeywordAbility::Banding => KeywordSnapshot::Banding,
        KeywordAbility::BandsWithOther(BandingQuality::LegendaryCreatures) => {
            KeywordSnapshot::BandsWithOtherLegendaryCreatures
        }
        KeywordAbility::BandsWithOther(BandingQuality::WolvesOfTheHunt) => {
            KeywordSnapshot::BandsWithOtherWolvesOfTheHunt
        }
        KeywordAbility::Vigilance => KeywordSnapshot::Vigilance,
        KeywordAbility::Defender => KeywordSnapshot::Defender,
        KeywordAbility::Deathtouch => KeywordSnapshot::Deathtouch,
        KeywordAbility::Lifelink => KeywordSnapshot::Lifelink,
        KeywordAbility::Reach => KeywordSnapshot::Reach,
        KeywordAbility::Flash => KeywordSnapshot::Flash,
        KeywordAbility::Hexproof => KeywordSnapshot::Hexproof,
        KeywordAbility::Shroud => KeywordSnapshot::Shroud,
        KeywordAbility::Unleash => KeywordSnapshot::Unleash,
        KeywordAbility::Intimidate => KeywordSnapshot::Intimidate,
        KeywordAbility::Shadow => KeywordSnapshot::Shadow,
        KeywordAbility::Menace => KeywordSnapshot::Menace,
        KeywordAbility::Undying => KeywordSnapshot::Undying,
        KeywordAbility::Persist => KeywordSnapshot::Persist,
        KeywordAbility::Indestructible => KeywordSnapshot::Indestructible,
        KeywordAbility::AttacksEachCombatIfAble => KeywordSnapshot::AttacksEachCombatIfAble,
        KeywordAbility::AttacksPlayerEachCombatIfAble => {
            KeywordSnapshot::AttacksPlayerEachCombatIfAble
        }
        KeywordAbility::LegendaryLandwalk => KeywordSnapshot::LegendaryLandwalk,
        KeywordAbility::Landwalk(BasicLandType::Plains) => KeywordSnapshot::Plainswalk,
        KeywordAbility::Landwalk(BasicLandType::Island) => KeywordSnapshot::Islandwalk,
        KeywordAbility::Landwalk(BasicLandType::Swamp) => KeywordSnapshot::Swampwalk,
        KeywordAbility::Landwalk(BasicLandType::Mountain) => KeywordSnapshot::Mountainwalk,
        KeywordAbility::Landwalk(BasicLandType::Forest) => KeywordSnapshot::Forestwalk,
        KeywordAbility::ProtectionFrom(predicate) => protection_snapshot(predicate),
        KeywordAbility::Suspend(_) | KeywordAbility::Rebound => unreachable!(
            "spell-only keywords are card abilities, never mutable runtime keyword state"
        ),
    }
}

fn protection_snapshot(predicate: &'static ObjectPredicateDef) -> KeywordSnapshot {
    if predicate == &ObjectPredicateDef::Color(ManaColor::White) {
        KeywordSnapshot::ProtectionFromWhite
    } else if predicate == &ObjectPredicateDef::Color(ManaColor::Blue) {
        KeywordSnapshot::ProtectionFromBlue
    } else if predicate == &ObjectPredicateDef::Color(ManaColor::Black) {
        KeywordSnapshot::ProtectionFromBlack
    } else if predicate == &ObjectPredicateDef::Color(ManaColor::Red) {
        KeywordSnapshot::ProtectionFromRed
    } else if predicate == &ObjectPredicateDef::Color(ManaColor::Green) {
        KeywordSnapshot::ProtectionFromGreen
    } else if predicate == &ObjectPredicateDef::ColorCount(0) {
        KeywordSnapshot::ProtectionFromColorless
    } else if predicate == &ObjectPredicateDef::Subtype("Zombie") {
        KeywordSnapshot::ProtectionFromZombies
    } else if predicate == &ObjectPredicateDef::Subtype("Vampire") {
        KeywordSnapshot::ProtectionFromVampires
    } else if predicate == &ObjectPredicateDef::Subtype("Werewolf") {
        KeywordSnapshot::ProtectionFromWerewolves
    } else if predicate
        == &ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::Subtype("Vampire"),
            ObjectPredicateDef::Subtype("Werewolf"),
            ObjectPredicateDef::Subtype("Zombie"),
        ])
    {
        KeywordSnapshot::ProtectionFromVampiresWerewolvesAndZombies
    } else if predicate == &ObjectPredicateDef::HasType(CardType::Creature) {
        KeywordSnapshot::ProtectionFromCreatures
    } else if predicate
        == &ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::ColorCount(0),
            ObjectPredicateDef::ColorCount(1),
        ]))
    {
        KeywordSnapshot::ProtectionFromMulticolored
    } else if predicate
        == &ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
        ])
    {
        KeywordSnapshot::ProtectionFromNonHumanCreatures
    } else if predicate == &ObjectPredicateDef::HasType(CardType::Enchantment) {
        KeywordSnapshot::ProtectionFromEnchantments
    } else if predicate
        == &ObjectPredicateDef::All(&[
            ObjectPredicateDef::Spell,
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
        ])
    {
        KeywordSnapshot::ProtectionFromInstantAndSorcerySpells
    } else if predicate
        == &ObjectPredicateDef::All(&[
            ObjectPredicateDef::Spell,
            ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
        ])
    {
        KeywordSnapshot::ProtectionFromColoredSpells
    } else if predicate == &ObjectPredicateDef::ControlledBy(PlayerRelation::ChosenPlayer) {
        KeywordSnapshot::ProtectionFromChosenPlayer
    } else {
        panic!("checkpoint cannot encode an unauthored protection quality")
    }
}

pub(in crate::game::state_checkpoint) const fn parse_keyword(
    value: KeywordSnapshot,
) -> KeywordAbility {
    match value {
        KeywordSnapshot::Convoke => KeywordAbility::Convoke,
        KeywordSnapshot::Delve => KeywordAbility::Delve,
        KeywordSnapshot::Improvise => KeywordAbility::Improvise,
        KeywordSnapshot::Devoid => KeywordAbility::Devoid,
        KeywordSnapshot::Compleated => KeywordAbility::Compleated,
        KeywordSnapshot::SplitSecond => KeywordAbility::SplitSecond,
        KeywordSnapshot::Infect => KeywordAbility::Infect,
        KeywordSnapshot::Flying => KeywordAbility::Flying,
        KeywordSnapshot::Trample => KeywordAbility::Trample,
        KeywordSnapshot::Haste => KeywordAbility::Haste,
        KeywordSnapshot::FirstStrike => KeywordAbility::FirstStrike,
        KeywordSnapshot::DoubleStrike => KeywordAbility::DoubleStrike,
        KeywordSnapshot::Banding => KeywordAbility::Banding,
        KeywordSnapshot::BandsWithOtherLegendaryCreatures => {
            KeywordAbility::BandsWithOther(BandingQuality::LegendaryCreatures)
        }
        KeywordSnapshot::BandsWithOtherWolvesOfTheHunt => {
            KeywordAbility::BandsWithOther(BandingQuality::WolvesOfTheHunt)
        }
        KeywordSnapshot::Vigilance => KeywordAbility::Vigilance,
        KeywordSnapshot::Defender => KeywordAbility::Defender,
        KeywordSnapshot::Deathtouch => KeywordAbility::Deathtouch,
        KeywordSnapshot::Lifelink => KeywordAbility::Lifelink,
        KeywordSnapshot::Reach => KeywordAbility::Reach,
        KeywordSnapshot::Flash => KeywordAbility::Flash,
        KeywordSnapshot::Hexproof => KeywordAbility::Hexproof,
        KeywordSnapshot::Shroud => KeywordAbility::Shroud,
        KeywordSnapshot::Unleash => KeywordAbility::Unleash,
        KeywordSnapshot::Intimidate => KeywordAbility::Intimidate,
        KeywordSnapshot::Shadow => KeywordAbility::Shadow,
        KeywordSnapshot::Menace => KeywordAbility::Menace,
        KeywordSnapshot::Undying => KeywordAbility::Undying,
        KeywordSnapshot::Persist => KeywordAbility::Persist,
        KeywordSnapshot::Indestructible => KeywordAbility::Indestructible,
        KeywordSnapshot::AttacksEachCombatIfAble => KeywordAbility::AttacksEachCombatIfAble,
        KeywordSnapshot::AttacksPlayerEachCombatIfAble => {
            KeywordAbility::AttacksPlayerEachCombatIfAble
        }
        KeywordSnapshot::LegendaryLandwalk => KeywordAbility::LegendaryLandwalk,
        KeywordSnapshot::Plainswalk => KeywordAbility::Landwalk(BasicLandType::Plains),
        KeywordSnapshot::Islandwalk => KeywordAbility::Landwalk(BasicLandType::Island),
        KeywordSnapshot::Swampwalk => KeywordAbility::Landwalk(BasicLandType::Swamp),
        KeywordSnapshot::Mountainwalk => KeywordAbility::Landwalk(BasicLandType::Mountain),
        KeywordSnapshot::Forestwalk => KeywordAbility::Landwalk(BasicLandType::Forest),
        // The protection family is one keyword per quality, so it parses
        // in its own place rather than swelling this one.
        KeywordSnapshot::ProtectionFromWhite
        | KeywordSnapshot::ProtectionFromBlue
        | KeywordSnapshot::ProtectionFromBlack
        | KeywordSnapshot::ProtectionFromRed
        | KeywordSnapshot::ProtectionFromGreen
        | KeywordSnapshot::ProtectionFromColorless
        | KeywordSnapshot::ProtectionFromZombies
        | KeywordSnapshot::ProtectionFromVampires
        | KeywordSnapshot::ProtectionFromWerewolves
        | KeywordSnapshot::ProtectionFromVampiresWerewolvesAndZombies
        | KeywordSnapshot::ProtectionFromCreatures
        | KeywordSnapshot::ProtectionFromMulticolored
        | KeywordSnapshot::ProtectionFromNonHumanCreatures
        | KeywordSnapshot::ProtectionFromEnchantments
        | KeywordSnapshot::ProtectionFromInstantAndSorcerySpells
        | KeywordSnapshot::ProtectionFromColoredSpells
        | KeywordSnapshot::ProtectionFromChosenPlayer => parse_protection_keyword(value),
    }
}

/// The protection qualities, split out of [`parse_keyword`]: there is one
/// tag per printed quality and they are all one keyword.
const fn parse_protection_keyword(value: KeywordSnapshot) -> KeywordAbility {
    match value {
        KeywordSnapshot::ProtectionFromWhite => protection_color(ManaColor::White),
        KeywordSnapshot::ProtectionFromBlue => protection_color(ManaColor::Blue),
        KeywordSnapshot::ProtectionFromBlack => protection_color(ManaColor::Black),
        KeywordSnapshot::ProtectionFromRed => protection_color(ManaColor::Red),
        KeywordSnapshot::ProtectionFromGreen => protection_color(ManaColor::Green),
        KeywordSnapshot::ProtectionFromColorless => protection_color(ManaColor::Colorless),
        KeywordSnapshot::ProtectionFromZombies => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Zombie"))
        }
        KeywordSnapshot::ProtectionFromVampires => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Vampire"))
        }
        KeywordSnapshot::ProtectionFromWerewolves => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Werewolf"))
        }
        KeywordSnapshot::ProtectionFromVampiresWerewolvesAndZombies => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Subtype("Vampire"),
                ObjectPredicateDef::Subtype("Werewolf"),
                ObjectPredicateDef::Subtype("Zombie"),
            ]))
        }
        KeywordSnapshot::ProtectionFromCreatures => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Creature))
        }
        KeywordSnapshot::ProtectionFromMulticolored => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::ColorCount(0),
                ObjectPredicateDef::ColorCount(1),
            ])))
        }
        KeywordSnapshot::ProtectionFromNonHumanCreatures => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
            ]))
        }
        KeywordSnapshot::ProtectionFromEnchantments => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Enchantment))
        }
        KeywordSnapshot::ProtectionFromInstantAndSorcerySpells => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
            ]))
        }
        KeywordSnapshot::ProtectionFromColoredSpells => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::Not(&ObjectPredicateDef::ColorCount(0)),
            ]))
        }
        KeywordSnapshot::ProtectionFromChosenPlayer => KeywordAbility::ProtectionFrom(
            &ObjectPredicateDef::ControlledBy(PlayerRelation::ChosenPlayer),
        ),
        // Only protection tags reach here; the caller matches the rest.
        _ => panic!("only protection tags reach the protection parser"),
    }
}

const fn protection_color(color: ManaColor) -> KeywordAbility {
    match color {
        ManaColor::White => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::White))
        }
        ManaColor::Blue => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Blue))
        }
        ManaColor::Black => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Black))
        }
        ManaColor::Red => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Red))
        }
        ManaColor::Green => {
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Color(ManaColor::Green))
        }
        ManaColor::Colorless => KeywordAbility::ProtectionFrom(&ObjectPredicateDef::ColorCount(0)),
    }
}
