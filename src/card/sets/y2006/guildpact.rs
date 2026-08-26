//! GPT card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, ValueDef, ZoneKind, abilities,
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
// Audit: partial — The opening-hand action is declarative; the static type-changing layer cannot yet make every nonland permanent legendary.
pub(in crate::card::sets) static LEYLINE_OF_SINGULARITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808"),
    "Leyline of Singularity",
    CardArt::new("d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808", "Zoltan Boros & Gabor Szikszai"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("All nonland permanents are legendary.", "Needs a global supertype-changing continuous effect."),
    ]),
);

// GPT 52 — Leyline of the Void
// Audit: partial — The opening-hand action is declarative; the graveyard replacement is not yet expressible as a global opponent-scoped replacement.
pub(in crate::card::sets) static LEYLINE_OF_THE_VOID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37dfe8b8-b39e-4e70-9e5b-be42c93b4f70"),
    "Leyline of the Void",
    CardArt::new("37dfe8b8-b39e-4e70-9e5b-be42c93b4f70", "Adam Rex"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("If a card would be put into an opponent's graveyard from anywhere, exile it instead.", "Needs a battlefield replacement that applies to every opponent-owned card from every zone."),
    ]),
);

// GPT 56 — Plagued Rusalka
// Audit: metadata-only — Card rules have not been implemented.
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
// Audit: partial — The opening-hand action is declarative; the optional paid spell-cast trigger still needs a target chosen after payment.
pub(in crate::card::sets) static LEYLINE_OF_LIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d09839-b41e-4aab-8913-40d63052dbf3"),
    "Leyline of Lightning",
    CardArt::new("23d09839-b41e-4aab-8913-40d63052dbf3", "Paolo Parente"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("Whenever you cast a spell, you may pay {1}. If you do, this enchantment deals 1 damage to target player or planeswalker.", "Needs a paid trigger whose target is declared only if the payment is accepted."),
    ]),
);

// GPT 74 — Scorched Rusalka
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_RUSALKA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f955164-ddb8-484c-a063-967621abce87"),
    "Scorched Rusalka",
    crate::card::CardArt::new("9f955164-ddb8-484c-a063-967621abce87", "Luca Zontini"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 90 — Leyline of Lifeforce
// Audit: partial — The opening-hand action is declarative; spell-counter prohibitions do not yet accept a creature-spell predicate.
pub(in crate::card::sets) static LEYLINE_OF_LIFEFORCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7caffa7-29bd-455c-9770-94a0ad7ef5e3"),
    "Leyline of Lifeforce",
    CardArt::new("f7caffa7-29bd-455c-9770-94a0ad7ef5e3", "Kev Walker"),
    CardSet::Guildpact,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::not_implemented("Creature spells can't be countered.", "Needs a static countering prohibition over creature spells."),
    ]),
);

// GPT 125 — Pillory of the Sleepless
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PILLORY_OF_THE_SLEEPLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36964bbd-f068-4a69-8d6b-7e4e97938b98"),
    "Pillory of the Sleepless",
    crate::card::CardArt::new("36964bbd-f068-4a69-8d6b-7e4e97938b98", "Mark Romanoski"),
    crate::card::CardSet::Guildpact,
    crate::card::CardRules::unsupported(),
);

// GPT 158 — Gruul Turf
// Audit: metadata-only — Card rules have not been implemented.
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
