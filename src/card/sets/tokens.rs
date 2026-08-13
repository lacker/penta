//! Token definitions.
//!
//! A token is not a printed card, but it is a permanent with characteristics,
//! so it is cataloged like anything else. `CardSet::Token` belongs to no
//! format's allowed sets, which is what keeps a token out of every decklist
//! while still letting a client resolve one by definition.
//!
//! A token has no mana cost, so its colors come from a printed color rather
//! than from a cost, and it carries no art: a Scryfall identifier names a
//! printing, and the client already falls back to the type glyph without one.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectDurationDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities, cards,
};

pub(in crate::card::sets) static BEAST_TOKEN_3_3_GREEN: CardRecord = CardRecord::new(
    cards::BEAST_TOKEN_3_3_GREEN,
    "Beast",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Beast"], 3, 3).printed_colors(&[ManaColor::Green]),
);

pub(in crate::card::sets) static KNIGHT_TOKEN_2_2_WHITE: CardRecord = CardRecord::new(
    cards::KNIGHT_TOKEN_2_2_WHITE,
    "Knight",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Knight"], 2, 2)
        .printed_colors(&[ManaColor::White])
        .with_abilities(&[abilities::vigilance()]),
);

pub(in crate::card::sets) static SOLDIER_TOKEN_1_1_RED_WHITE: CardRecord = CardRecord::new(
    cards::SOLDIER_TOKEN_1_1_RED_WHITE,
    "Soldier",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Soldier"], 1, 1)
        .printed_colors(&[ManaColor::Red, ManaColor::White])
        .with_abilities(&[abilities::haste()]),
);

pub(in crate::card::sets) static DEMON_TOKEN_5_5_BLACK: CardRecord = CardRecord::new(
    cards::DEMON_TOKEN_5_5_BLACK,
    "Demon",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Demon"], 5, 5)
        .printed_colors(&[ManaColor::Black])
        .with_abilities(&[abilities::flying()]),
);

/// Voice of Resurgence's token. Its printed power and toughness are defined
/// by the board, which a zero-power body plus a counting static bonus says
/// exactly: the count includes the token itself, so it is never a 0/0.
pub(in crate::card::sets) static ELEMENTAL_TOKEN_GREEN_WHITE: CardRecord = CardRecord::new(
    cards::ELEMENTAL_TOKEN_GREEN_WHITE,
    "Elemental",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Elemental"], 0, 0)
        .printed_colors(&[ManaColor::Green, ManaColor::White])
        .with_ability(AbilityDef::static_ability(
            "This token's power and toughness are each equal to the number of creatures you control.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL),
                    toughness: ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL),
                },
                duration: EffectDurationDef::WhileSourceRemainsInZone,
            },
        )),
);

static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zones: &[ZoneKind::Battlefield],
    controller: PlayerRelation::You,
};

pub(in crate::card::sets) static SPIRIT_TOKEN_1_1_WHITE: CardRecord = CardRecord::new(
    cards::SPIRIT_TOKEN_1_1_WHITE,
    "Spirit",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Spirit"], 1, 1)
        .printed_colors(&[ManaColor::White])
        .with_abilities(&[abilities::flying()]),
);

pub(in crate::card::sets) static WOLF_TOKEN_2_2_GREEN: CardRecord = CardRecord::new(
    cards::WOLF_TOKEN_2_2_GREEN,
    "Wolf",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Wolf"], 2, 2).printed_colors(&[ManaColor::Green]),
);

pub(in crate::card::sets) static WOLF_TOKEN_1_1_BLACK: CardRecord = CardRecord::new(
    cards::WOLF_TOKEN_1_1_BLACK,
    "Wolf",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Wolf"], 1, 1)
        .printed_colors(&[ManaColor::Black])
        .with_abilities(&[abilities::deathtouch()]),
);

/// Tetravus detaches these, and can exile its own back to rebuild itself.
pub(in crate::card::sets) static TETRAVITE_TOKEN: CardRecord = CardRecord::new(
    cards::TETRAVITE_TOKEN,
    "Tetravite",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Tetravite"], 1, 1)
        .with_type(CardType::Artifact)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "This token can't be enchanted.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::CannotBeEnchanted,
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            )
            .with_coverage(AbilityCoverageDef::explained_complete(
                "The shared targetability check refuses the token to an Aura spell, and an Aura that arrives some other way still falls off.",
            )),
        ]),
);

/// Vraska's ultimate. One connection ends the game, so the token's whole
/// point is the trigger rather than its body.
pub(in crate::card::sets) static ASSASSIN_TOKEN_1_1_BLACK: CardRecord = CardRecord::new(
    cards::ASSASSIN_TOKEN_1_1_BLACK,
    "Assassin",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Assassin"], 1, 1)
        .printed_colors(&[ManaColor::Black])
        .with_ability(AbilityDef::triggered(
            "Whenever this token deals combat damage to a player, that player loses the game.",
            TriggerEventDef::CombatDamageDealtToPlayer {
                source: ObjectPredicateDef::Source,
            },
            EffectDef::LoseTheGame {
                player: EffectRecipientDef::EventPlayer,
            },
        )),
);

pub(in crate::card::sets) static BIRD_TOKEN_4_4_RED: CardRecord = CardRecord::new(
    cards::BIRD_TOKEN_4_4_RED,
    "Bird",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Bird"], 4, 4)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&[abilities::flying()]),
);

pub(in crate::card::sets) static CITIZEN_TOKEN_1_1_WHITE: CardRecord = CardRecord::new(
    cards::CITIZEN_TOKEN_1_1_WHITE,
    "Citizen",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Citizen"], 1, 1)
        .printed_colors(&[ManaColor::White]),
);

pub(in crate::card::sets) static THRULL_TOKEN_0_1_BLACK: CardRecord = CardRecord::new(
    cards::THRULL_TOKEN_0_1_BLACK,
    "Thrull",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Thrull"], 0, 1)
        .printed_colors(&[ManaColor::Black]),
);

pub(in crate::card::sets) static WASP_TOKEN_1_1_COLORLESS: CardRecord = CardRecord::new(
    cards::WASP_TOKEN_1_1_COLORLESS,
    "Wasp",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Insect"], 1, 1)
        .with_type(CardType::Artifact)
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static MINOR_DEMON_TOKEN_1_1_BLACK_RED: CardRecord = CardRecord::new(
    cards::MINOR_DEMON_TOKEN_1_1_BLACK_RED,
    "Minor Demon",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Demon"], 1, 1)
        .printed_colors(&[ManaColor::Black, ManaColor::Red]),
);

/// Domri's emblem. An emblem is an object with abilities and no other
/// characteristics, so it is cataloged like a token and lives in its own
/// list rather than on the battlefield.
pub(in crate::card::sets) static DOMRI_RADE_EMBLEM: CardRecord = CardRecord::new(
    cards::DOMRI_RADE_EMBLEM,
    "Domri Rade emblem",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_emblem().with_ability(AbilityDef::static_ability(
        "Creatures you control have double strike, trample, hexproof, and haste.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            },
            effect: AppliedEffectDef::Composite(&DOMRI_EMBLEM_KEYWORDS),
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )),
);

static DOMRI_EMBLEM_KEYWORDS: [AppliedEffectDef; 4] = [
    AppliedEffectDef::GrantAbility(&DOMRI_DOUBLE_STRIKE),
    AppliedEffectDef::GrantAbility(&DOMRI_TRAMPLE),
    AppliedEffectDef::GrantAbility(&DOMRI_HEXPROOF),
    AppliedEffectDef::GrantAbility(&DOMRI_HASTE),
];

static DOMRI_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();
static DOMRI_TRAMPLE: AbilityDef = abilities::trample();
static DOMRI_HEXPROOF: AbilityDef = abilities::hexproof();
static DOMRI_HASTE: AbilityDef = abilities::haste();

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BEAST_TOKEN_3_3_GREEN,
    &KNIGHT_TOKEN_2_2_WHITE,
    &SOLDIER_TOKEN_1_1_RED_WHITE,
    &DEMON_TOKEN_5_5_BLACK,
    &ELEMENTAL_TOKEN_GREEN_WHITE,
    &SPIRIT_TOKEN_1_1_WHITE,
    &WOLF_TOKEN_2_2_GREEN,
    &WOLF_TOKEN_1_1_BLACK,
    &DOMRI_RADE_EMBLEM,
    &TETRAVITE_TOKEN,
    &ASSASSIN_TOKEN_1_1_BLACK,
    &BIRD_TOKEN_4_4_RED,
    &CITIZEN_TOKEN_1_1_WHITE,
    &THRULL_TOKEN_0_1_BLACK,
    &WASP_TOKEN_1_1_COLORLESS,
    &MINOR_DEMON_TOKEN_1_1_BLACK_RED,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
