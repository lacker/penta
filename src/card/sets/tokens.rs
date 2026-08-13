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
    AbilityCoverageDef, AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet,
    CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PlayerRelation, ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
    cards,
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
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL), ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL)),
            },
        )),
);

static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

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

/// Bottle of Suleiman's reward for winning its flip.
pub(in crate::card::sets) static DJINN_TOKEN_5_5_COLORLESS: CardRecord = CardRecord::new(
    cards::DJINN_TOKEN_5_5_COLORLESS,
    "Djinn",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Djinn"], 5, 5)
        .with_type(CardType::Artifact)
        .with_abilities(&[abilities::flying()]),
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
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeEnchanted),
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
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
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

pub(in crate::card::sets) static WURM_TOKEN_5_5_GREEN: CardRecord = CardRecord::new(
    cards::WURM_TOKEN_5_5_GREEN,
    "Wurm",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Wurm"], 5, 5)
        .printed_colors(&[ManaColor::Green])
        .with_ability(abilities::trample()),
);

pub(in crate::card::sets) static CENTAUR_TOKEN_3_3_GREEN: CardRecord = CardRecord::new(
    cards::CENTAUR_TOKEN_3_3_GREEN,
    "Centaur",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Centaur"], 3, 3)
        .printed_colors(&[ManaColor::Green]),
);

pub(in crate::card::sets) static RHINO_TOKEN_4_4_GREEN: CardRecord = CardRecord::new(
    cards::RHINO_TOKEN_4_4_GREEN,
    "Rhino",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Rhino"], 4, 4)
        .printed_colors(&[ManaColor::Green])
        .with_ability(abilities::trample()),
);

pub(in crate::card::sets) static ZOMBIE_TOKEN_2_2_BLACK: CardRecord = CardRecord::new(
    cards::ZOMBIE_TOKEN_2_2_BLACK,
    "Zombie",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Zombie"], 2, 2)
        .printed_colors(&[ManaColor::Black]),
);

pub(in crate::card::sets) static HUMAN_TOKEN_1_1_WHITE: CardRecord = CardRecord::new(
    cards::HUMAN_TOKEN_1_1_WHITE,
    "Human",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Human"], 1, 1).printed_colors(&[ManaColor::White]),
);

pub(in crate::card::sets) static ANGEL_TOKEN_4_4_WHITE: CardRecord = CardRecord::new(
    cards::ANGEL_TOKEN_4_4_WHITE,
    "Angel",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Angel"], 4, 4)
        .printed_colors(&[ManaColor::White])
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static SPIRIT_TOKEN_1_1_BLUE: CardRecord = CardRecord::new(
    cards::SPIRIT_TOKEN_1_1_BLUE,
    "Spirit",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Spirit"], 1, 1)
        .printed_colors(&[ManaColor::Blue])
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static HOMUNCULUS_TOKEN_2_2_BLUE: CardRecord = CardRecord::new(
    cards::HOMUNCULUS_TOKEN_2_2_BLUE,
    "Homunculus",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Homunculus"], 2, 2)
        .printed_colors(&[ManaColor::Blue]),
);

pub(in crate::card::sets) static SPIDER_TOKEN_1_2_GREEN: CardRecord = CardRecord::new(
    cards::SPIDER_TOKEN_1_2_GREEN,
    "Spider",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Spider"], 1, 2)
        .printed_colors(&[ManaColor::Green])
        .with_ability(abilities::reach()),
);

pub(in crate::card::sets) static SOLDIER_TOKEN_1_1_WHITE: CardRecord = CardRecord::new(
    cards::SOLDIER_TOKEN_1_1_WHITE,
    "Soldier",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Soldier"], 1, 1)
        .printed_colors(&[ManaColor::White]),
);

pub(in crate::card::sets) static DRAKE_TOKEN_2_2_BLUE: CardRecord = CardRecord::new(
    cards::DRAKE_TOKEN_2_2_BLUE,
    "Drake",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Drake"], 2, 2)
        .printed_colors(&[ManaColor::Blue])
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static GOBLIN_TOKEN_1_1_RED: CardRecord = CardRecord::new(
    cards::GOBLIN_TOKEN_1_1_RED,
    "Goblin",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Goblin"], 1, 1).printed_colors(&[ManaColor::Red]),
);

pub(in crate::card::sets) static SPIRIT_TOKEN_1_1_WHITE_BLACK: CardRecord = CardRecord::new(
    cards::SPIRIT_TOKEN_1_1_WHITE_BLACK,
    "Spirit",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Spirit"], 1, 1)
        .printed_colors(&[ManaColor::White, ManaColor::Black])
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static SLIVER_TOKEN_1_1_COLORLESS: CardRecord = CardRecord::new(
    cards::SLIVER_TOKEN_1_1_COLORLESS,
    "Sliver",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Sliver"], 1, 1),
);

pub(in crate::card::sets) static DRAGON_TOKEN_2_2_RED: CardRecord = CardRecord::new(
    cards::DRAGON_TOKEN_2_2_RED,
    "Dragon",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Dragon"], 2, 2)
        .printed_colors(&[ManaColor::Red])
        .with_ability(abilities::flying())
        .with_ability(AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[crate::card::AbilityCostDef::Mana(crate::mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

pub(in crate::card::sets) static ELEMENTAL_TOKEN_1_1_RED: CardRecord = CardRecord::new(
    cards::ELEMENTAL_TOKEN_1_1_RED,
    "Elemental",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Elemental"], 1, 1)
        .printed_colors(&[ManaColor::Red]),
);

pub(in crate::card::sets) static SAPROLING_TOKEN_1_1_GREEN: CardRecord = CardRecord::new(
    cards::SAPROLING_TOKEN_1_1_GREEN,
    "Saproling",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Saproling"], 1, 1)
        .printed_colors(&[ManaColor::Green]),
);

pub(in crate::card::sets) static BIRD_TOKEN_1_1_WHITE: CardRecord = CardRecord::new(
    cards::BIRD_TOKEN_1_1_WHITE,
    "Bird",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Bird"], 1, 1)
        .printed_colors(&[ManaColor::White])
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static DRAGON_TOKEN_6_6_RED: CardRecord = CardRecord::new(
    cards::DRAGON_TOKEN_6_6_RED,
    "Dragon",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Dragon"], 6, 6)
        .printed_colors(&[ManaColor::Red])
        .with_ability(abilities::flying()),
);

pub(in crate::card::sets) static WURM_TOKEN_6_6_GREEN: CardRecord = CardRecord::new(
    cards::WURM_TOKEN_6_6_GREEN,
    "Wurm",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Wurm"], 6, 6).printed_colors(&[ManaColor::Green]),
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
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&DOMRI_EMBLEM_KEYWORDS),
        },
    )),
);

static DOMRI_EMBLEM_KEYWORDS: [AppliedEffectDef; 4] = [
    AppliedEffectDef::add_ability(&DOMRI_DOUBLE_STRIKE),
    AppliedEffectDef::add_ability(&DOMRI_TRAMPLE),
    AppliedEffectDef::add_ability(&DOMRI_HEXPROOF),
    AppliedEffectDef::add_ability(&DOMRI_HASTE),
];

static DOMRI_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();
static DOMRI_TRAMPLE: AbilityDef = abilities::trample();
static DOMRI_HEXPROOF: AbilityDef = abilities::hexproof();
static DOMRI_HASTE: AbilityDef = abilities::haste();

/// Serpent Generator's Snake, which carries the poison trigger the artifact
/// prints in quotation marks rather than an ability of its own.
pub(in crate::card::sets) static SNAKE_TOKEN_1_1_POISONOUS: CardRecord = CardRecord::new(
    cards::SNAKE_TOKEN_1_1_POISONOUS,
    "Snake",
    CardArt::new("", ""),
    CardSet::Token,
    CardRules::new_creature_without_mana_cost(&["Snake"], 1, 1)
        .with_type(CardType::Artifact)
        .with_ability(abilities::poisonous_damage(
            1,
            "Whenever this creature deals damage to a player, that player gets a poison counter.",
        )),
);

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
    &DJINN_TOKEN_5_5_COLORLESS,
    &TETRAVITE_TOKEN,
    &ASSASSIN_TOKEN_1_1_BLACK,
    &BIRD_TOKEN_4_4_RED,
    &CITIZEN_TOKEN_1_1_WHITE,
    &THRULL_TOKEN_0_1_BLACK,
    &WASP_TOKEN_1_1_COLORLESS,
    &MINOR_DEMON_TOKEN_1_1_BLACK_RED,
    &WURM_TOKEN_5_5_GREEN,
    &CENTAUR_TOKEN_3_3_GREEN,
    &RHINO_TOKEN_4_4_GREEN,
    &ZOMBIE_TOKEN_2_2_BLACK,
    &HUMAN_TOKEN_1_1_WHITE,
    &ANGEL_TOKEN_4_4_WHITE,
    &SPIRIT_TOKEN_1_1_BLUE,
    &HOMUNCULUS_TOKEN_2_2_BLUE,
    &SPIDER_TOKEN_1_2_GREEN,
    &SOLDIER_TOKEN_1_1_WHITE,
    &DRAKE_TOKEN_2_2_BLUE,
    &GOBLIN_TOKEN_1_1_RED,
    &SPIRIT_TOKEN_1_1_WHITE_BLACK,
    &SLIVER_TOKEN_1_1_COLORLESS,
    &DRAGON_TOKEN_2_2_RED,
    &ELEMENTAL_TOKEN_1_1_RED,
    &SAPROLING_TOKEN_1_1_GREEN,
    &BIRD_TOKEN_1_1_WHITE,
    &DRAGON_TOKEN_6_6_RED,
    &SNAKE_TOKEN_1_1_POISONOUS,
    &WURM_TOKEN_6_6_GREEN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
