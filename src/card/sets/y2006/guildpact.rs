//! GPT card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, ReplacementEffectDef,
    ReplacementEventDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// GPT 10 — Leyline of the Meek
pub(in crate::card::sets) static LEYLINE_OF_THE_MEEK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efc58757-abcc-41c9-b4d2-e70e9f387cbb"),
    "Leyline of the Meek",
    CardArt::new("efc58757-abcc-41c9-b4d2-e70e9f387cbb", "Mark Zug"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::static_ability(
            "Creature tokens get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Token,
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// GPT 29 — Leyline of Singularity
// Audit: unsupported — Needs a global supertype-changing continuous effect.
pub(in crate::card::sets) static LEYLINE_OF_SINGULARITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808"),
    "Leyline of Singularity",
    CardArt::new(
        "d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808",
        "Zoltan Boros & Gabor Szikszai",
    ),
    CardSet::Guildpact,
    CardRules::unsupported(),
);

// GPT 52 — Leyline of the Void
pub(in crate::card::sets) static LEYLINE_OF_THE_VOID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37dfe8b8-b39e-4e70-9e5b-be42c93b4f70"),
    "Leyline of the Void",
    CardArt::new("37dfe8b8-b39e-4e70-9e5b-be42c93b4f70", "Adam Rex"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::begin_game_on_battlefield(
            "If this card is in your opening hand, you may begin the game with it on the battlefield.",
        ),
        AbilityDef::replacement_for(
            "If a card would be put into an opponent's graveyard from anywhere, exile it instead.",
            ReplacementEventDef::AnyObjectWouldMove {
                to: ZoneKind::Graveyard,
                owner: PlayerRelation::Opponent,
                tokens: false,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

// GPT 56 — Plagued Rusalka
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd84bbb3-8b99-4e6d-b514-b094ec93eaa0"),
    "Plagued Rusalka",
    crate::card::CardArt::new(
        "cd84bbb3-8b99-4e6d-b514-b094ec93eaa0",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 68 — Leyline of Lightning
// Audit: unsupported — Needs a paid trigger whose target is declared only after its optional payment.
pub(in crate::card::sets) static LEYLINE_OF_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d09839-b41e-4aab-8913-40d63052dbf3"),
    "Leyline of Lightning",
    CardArt::new("23d09839-b41e-4aab-8913-40d63052dbf3", "Paolo Parente"),
    CardSet::Guildpact,
    CardRules::unsupported(),
);

// GPT 74 — Scorched Rusalka
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f955164-ddb8-484c-a063-967621abce87"),
    "Scorched Rusalka",
    crate::card::CardArt::new("9f955164-ddb8-484c-a063-967621abce87", "Luca Zontini"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 90 — Leyline of Lifeforce
pub(in crate::card::sets) static LEYLINE_OF_LIFEFORCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7caffa7-29bd-455c-9770-94a0ad7ef5e3"),
    "Leyline of Lifeforce",
    CardArt::new("f7caffa7-29bd-455c-9770-94a0ad7ef5e3", "Kev Walker"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(
            "If this card is in your opening hand, you may begin the game with it on the battlefield.",
        ),
        AbilityDef::static_ability(
            "Creature spells can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Stack],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        ),
    ]),
);

// GPT 125 — Pillory of the Sleepless
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILLORY_OF_THE_SLEEPLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36964bbd-f068-4a69-8d6b-7e4e97938b98"),
    "Pillory of the Sleepless",
    crate::card::CardArt::new("36964bbd-f068-4a69-8d6b-7e4e97938b98", "Mark Romanoski"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 158 — Gruul Turf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRUUL_TURF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("550b70e0-ebd5-49de-b62c-5224b8bf8e98"),
    "Gruul Turf",
    crate::card::CardArt::new("550b70e0-ebd5-49de-b62c-5224b8bf8e98", "John Avon"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEYLINE_OF_THE_MEEK,
    &LEYLINE_OF_SINGULARITY,
    &LEYLINE_OF_THE_VOID,
    &PLAGUED_RUSALKA,
    &LEYLINE_OF_LIGHTNING,
    &SCORCHED_RUSALKA,
    &LEYLINE_OF_LIFEFORCE,
    &PILLORY_OF_THE_SLEEPLESS,
    &GRUUL_TURF,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
