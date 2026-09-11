//! Guilds of Ravnica cards used as cross-format rules-engine test cases.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::SpellCastQueryDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

static ARCLIGHT_PHOENIX_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);

static ARCLIGHT_PHOENIX_CAST_QUERY: SpellCastQueryDef = SpellCastQueryDef {
    player: PlayerRelation::You,
    spell: ARCLIGHT_PHOENIX_INSTANT_OR_SORCERY,
};

static ARCLIGHT_PHOENIX_CAST_COUNT: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CountSpellsCastThisTurn(&ARCLIGHT_PHOENIX_CAST_QUERY),
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(3),
};

static ARCLIGHT_PHOENIX_RETURN_CONDITION: TriggerConditionDef = TriggerConditionDef::All(&[
    TriggerConditionDef::SourceInZone(ZoneKind::Graveyard),
    TriggerConditionDef::ValueComparison(&ARCLIGHT_PHOENIX_CAST_COUNT),
]);

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "GRN",
    slug: "guilds-of-ravnica",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// GRN 14 — Healer's Hawk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEALER_S_HAWK: CardRecord = CardRecord::new(
    "Healer's Hawk",
    "3313bd5c-b657-47a3-822a-dd0d9165492a",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// GRN 45 — Murmuring Mystic
pub(in crate::card::sets) static MURMURING_MYSTIC: CardRecord = CardRecord::new(
    "Murmuring Mystic",
    "5fc6adff-dcb3-456d-a8c2-0e77b784ff89",
    "Mark Winters",
// A 1/5 body that turns every cantrip into a blocker, so the deck that
    // was already casting spells stops needing creatures of its own.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 1, 5).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, create a 1/1 blue Bird Illusion creature token with flying.",
            // On the cast rather than the resolution, so a countered spell
            // has already paid for its Bird.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Bird", "Illusion"], &[ManaColor::Blue], 1, 1)
                    .with_abilities(&[abilities::flying()]),
            ))),
        ),
    ),
);

// GRN 64 — Burglar Rat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURGLAR_RAT: CardRecord = CardRecord::new(
    "Burglar Rat",
    "e9f7f218-fd2a-4233-8753-065ddc314f2d",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// GRN 77 — Midnight Reaper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIDNIGHT_REAPER: CardRecord = CardRecord::new(
    "Midnight Reaper",
    "5e122fe0-51c5-404d-a7b9-3d161a426c35",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// GRN 91 — Arclight Phoenix
pub(in crate::card::sets) static ARCLIGHT_PHOENIX: CardRecord = CardRecord::new(
    "Arclight Phoenix",
    "787de9ce-02c5-4a17-a88b-d38e83dbeb0b",
    "Slawomir Maniak",
CardRules::new_creature(mana_cost!("{3}{R}"), &["Phoenix"], 3, 2).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you've cast three or more instant and sorcery spells this turn, return this card from your graveyard to the battlefield.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &ARCLIGHT_PHOENIX_RETURN_CONDITION,
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// GRN 121 — Affectionate Indrik
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AFFECTIONATE_INDRIK: CardRecord = CardRecord::new(
    "Affectionate Indrik",
    "b4c8ddc1-d95c-499f-b1d1-f608f8f07b02",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// GRN 125 — Circuitous Route
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCUITOUS_ROUTE: CardRecord = CardRecord::new(
    "Circuitous Route",
    "5a970429-6369-4422-a343-00b30267f09d",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// GRN 152 — Assassin's Trophy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSASSIN_S_TROPHY: CardRecord = CardRecord::new(
    "Assassin's Trophy",
    "906b6e99-128f-4c11-8daf-16099d35b0d4",
    "Seb McKinnon",
    crate::card::CardRules::unsupported(),
);

// GRN 203 — Swiftblade Vindicator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWIFTBLADE_VINDICATOR: CardRecord = CardRecord::new(
    "Swiftblade Vindicator",
    "285c4d9e-0f22-49a8-b68c-150fd0d4b617",
    "Viktor Titov",
    crate::card::CardRules::unsupported(),
);

// GRN 207 — Thousand-Year Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUSAND_YEAR_STORM: CardRecord = CardRecord::new(
    "Thousand-Year Storm",
    "270a0863-7d07-43f0-925d-a8ce0383a1cb",
    "Dimitar Marinski",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HEALER_S_HAWK,
    &MURMURING_MYSTIC,
    &BURGLAR_RAT,
    &MIDNIGHT_REAPER,
    &ARCLIGHT_PHOENIX,
    &AFFECTIONATE_INDRIK,
    &CIRCUITOUS_ROUTE,
    &ASSASSIN_S_TROPHY,
    &SWIFTBLADE_VINDICATOR,
    &THOUSAND_YEAR_STORM,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
