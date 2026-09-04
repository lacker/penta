//! Guilds of Ravnica cards used as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardRules, CardSet, CardType, ComparisonDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, SpellCastQueryDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
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

// GRN 45 — Murmuring Mystic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MURMURING_MYSTIC: CardRecord = CardRecord::new(
    crate::card::CardSet::GuildsOfRavnica,
    "Murmuring Mystic",
    "5fc6adff-dcb3-456d-a8c2-0e77b784ff89",
    "Mark Winters",
    crate::card::CardRules::unsupported(),
);

// GRN 91 — Arclight Phoenix
pub(in crate::card::sets) static ARCLIGHT_PHOENIX: CardRecord = CardRecord::new(
    CardSet::GuildsOfRavnica,
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
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MURMURING_MYSTIC, &ARCLIGHT_PHOENIX];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
