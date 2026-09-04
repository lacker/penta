//! GPT card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::CastTimingPermissionDef;
use crate::ResolvedEffectDurationDef;
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardRules, CardSet, CardSupertype, CardType,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation, ReplacementEffectDef,
    ReplacementEventDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// GPT 10 — Leyline of the Meek
pub(in crate::card::sets) static LEYLINE_OF_THE_MEEK: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Leyline of the Meek",
    "efc58757-abcc-41c9-b4d2-e70e9f387cbb",
    "Mark Zug",
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
pub(in crate::card::sets) static LEYLINE_OF_SINGULARITY: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Leyline of Singularity",
    "d40d7e5c-3b6d-4e42-b495-b3cd7ae0d808",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::static_ability(
            "All nonland permanents are legendary.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_supertype(CardSupertype::Legendary),
            },
        ),
    ]),
);

// GPT 31 — Quicken
pub(in crate::card::sets) static QUICKEN: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Quicken",
    "7a276b12-4647-4223-b89e-f55d72feb2d0",
    "Aleksi Briclot",
    // One spell ability per part, so the card's two sentences are one clause
    // with a sequence rather than two spell clauses.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "The next sorcery spell you cast this turn can be cast as though it had flash. (It can be cast any time you could cast an instant.)\nDraw a card.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::HasType(CardType::Sorcery)),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn
                    .or(ResolvedEffectDurationDef::UntilNextMatchingCast),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// GPT 32 — Repeal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPEAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Repeal",
    "9e7dd929-4bba-46a6-86c9-b8ed853eb721",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// GPT 52 — Leyline of the Void
pub(in crate::card::sets) static LEYLINE_OF_THE_VOID: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Leyline of the Void",
    "37dfe8b8-b39e-4e70-9e5b-be42c93b4f70",
    "Adam Rex",
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::begin_game_on_battlefield(
            "If this card is in your opening hand, you may begin the game with it on the battlefield.",
        ),
        AbilityDef::replacement_for(
            "If a card would be put into an opponent's graveyard from anywhere, exile it instead.",
            ReplacementEventDef::AnyObjectWouldMove {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
                to: ZoneKind::Graveyard,
            },
            ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
        ),
    ]),
);

// GPT 56 — Plagued Rusalka
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUED_RUSALKA: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Plagued Rusalka",
    "cd84bbb3-8b99-4e6d-b514-b094ec93eaa0",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// GPT 68 — Leyline of Lightning
// Audit: unsupported — Needs a paid trigger whose target is declared only after its optional payment.
pub(in crate::card::sets) static LEYLINE_OF_LIGHTNING: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Leyline of Lightning",
    "23d09839-b41e-4aab-8913-40d63052dbf3",
    "Paolo Parente",
    CardRules::unsupported(),
);

// GPT 74 — Scorched Rusalka
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_RUSALKA: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Scorched Rusalka",
    "9f955164-ddb8-484c-a063-967621abce87",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// GPT 90 — Leyline of Lifeforce
pub(in crate::card::sets) static LEYLINE_OF_LIFEFORCE: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Leyline of Lifeforce",
    "f7caffa7-29bd-455c-9770-94a0ad7ef5e3",
    "Kev Walker",
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

// GPT 112 — Feral Animist
pub(in crate::card::sets) static FERAL_ANIMIST: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Feral Animist",
    "ad65c721-b601-46f9-ba0b-ac4d96567cee",
    "Ron Spears",
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Goblin", "Shaman"], 2, 1).with_ability(
        AbilityDef::activated(
            "{3}: This creature gets +X/+0 until end of turn, where X is its power.",
            &[AbilityCostDef::Mana(mana_cost!("{3}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::SourcePower,
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// GPT 125 — Pillory of the Sleepless
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILLORY_OF_THE_SLEEPLESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Pillory of the Sleepless",
    "36964bbd-f068-4a69-8d6b-7e4e97938b98",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// GPT 157 — Godless Shrine
pub(in crate::card::sets) static GODLESS_SHRINE: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Godless Shrine",
    "be010c2f-06db-47e3-80bd-df3f2a21ca34",
    "Rob Alexander",
    CardRules::new_land(&["Plains", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

// GPT 158 — Gruul Turf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRUUL_TURF: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Gruul Turf",
    "550b70e0-ebd5-49de-b62c-5224b8bf8e98",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// GPT 159 — Izzet Boilerworks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IZZET_BOILERWORKS: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Izzet Boilerworks",
    "666f455e-3a3d-475d-b67a-a1fdd74820eb",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// GPT 161 — Orzhov Basilica
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORZHOV_BASILICA: CardRecord = CardRecord::new(
    crate::card::CardSet::Guildpact,
    "Orzhov Basilica",
    "f9154d2a-3fc5-4fd6-9885-a810cb6b542a",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// GPT 164 — Steam Vents
pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Steam Vents",
    "054f2276-2dd5-43da-bb26-c57c560861fe",
    "Rob Alexander",
    CardRules::new_land(&["Island", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// GPT 165 — Stomping Ground
pub(in crate::card::sets) static STOMPING_GROUND: CardRecord = CardRecord::new(
    CardSet::Guildpact,
    "Stomping Ground",
    "a2773d8f-f906-475d-aaff-b7ca3b01f188",
    "Rob Alexander",
    CardRules::new_land(&["Mountain", "Forest"]).with_ability(abilities::shock_land_enters()),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &LEYLINE_OF_THE_MEEK,
    &LEYLINE_OF_SINGULARITY,
    &QUICKEN,
    &REPEAL,
    &LEYLINE_OF_THE_VOID,
    &PLAGUED_RUSALKA,
    &LEYLINE_OF_LIGHTNING,
    &SCORCHED_RUSALKA,
    &LEYLINE_OF_LIFEFORCE,
    &FERAL_ANIMIST,
    &PILLORY_OF_THE_SLEEPLESS,
    &GODLESS_SHRINE,
    &GRUUL_TURF,
    &IZZET_BOILERWORKS,
    &ORZHOV_BASILICA,
    &STEAM_VENTS,
    &STOMPING_GROUND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
