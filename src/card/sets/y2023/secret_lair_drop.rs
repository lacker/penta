//! Secret Lair Drop card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::SumValueDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "SLD",
    slug: "secret-lair-drop",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());











// SLD 1240 — Jurin, Leading the Charge
// Audit: unsupported — Battlefield recipient queries cannot filter attacking creatures by whether they are attacking a player rather than a planeswalker. Applying its attack bonus to all attacking creatures would grant it to the wrong attackers.
pub(in crate::card::sets) static JURIN_LEADING_THE_CHARGE_1240: CardRecord = CardRecord::new(
    "Jurin, Leading the Charge",
    "fab07547-a6b8-487a-9688-8f93aaa71f5b",
    "Yang Luo",
    crate::card::CardRules::unsupported(),
);

// SLD 1731 — Iron Man, Titan of Innovation
pub(in crate::card::sets) static IRON_MAN_TITAN_OF_INNOVATION_1731: CardRecord = CardRecord::new(
    "Iron Man, Titan of Innovation",
    "7542b1d1-e34d-46dc-af24-1b718034c0e4",
    "Justyna Dura",
    CardRules::new_artifact_creature(mana_cost!("{3}{U}{R}"), &["Human", "Hero"], 4, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::flying(),
abilities::haste(),
AbilityDef::triggered("Genius Industrialist — Whenever Iron Man attacks, create a Treasure token, then you may sacrifice a noncreature artifact. If you do, search your library for an artifact card with mana value equal to 1 plus the sacrificed artifact's mana value, put it onto the battlefield tapped, then shuffle.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::Sequence(&[EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure()))), EffectDef::PayOr(PayOrDef::optional(&[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature))]))], &EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueEqualTo(ValueDef::Sum(&SumValueDef { left: ValueDef::SacrificedManaValue, right: ValueDef::Constant(1) }))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: true, attachment: None, binding: None, then: None }))]))
]),
);

// SLD 1753 — Deadpool, Trading Card
// Audit: unsupported — Copy and text-word changes cannot exchange two permanents' complete text boxes while preserving their other copiable characteristics. There is no text-box exchange entry replacement.
pub(in crate::card::sets) static DEADPOOL_TRADING_CARD_1753: CardRecord = CardRecord::new(
    "Deadpool, Trading Card",
    "3a14d6c5-cfd1-4860-834a-5a0dc9df0320",
    "Justine Cruz",
    crate::card::CardRules::unsupported(),
);

// SLD 2082 — Knuckles the Echidna
pub(in crate::card::sets) static KNUCKLES_THE_ECHIDNA_2082: CardRecord = CardRecord::new(
    "Knuckles the Echidna",
    "54b65cca-7844-4d3e-9d7f-ed1f49f94425",
    "Tyler Walpole",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Echidna", "Warrior"], 2, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
abilities::double_strike(),
abilities::trample(),
abilities::haste(),
AbilityDef::triggered("Whenever one or more creatures you control deal combat damage to a player, create a Treasure token.", TriggerEventDef::CombatDamageDealtToPlayers { sources: ObjectPredicateDef::ControlledBy(PlayerRelation::You), players: PlayerRelation::Any }, EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::tokens::treasure())))),
AbilityDef::triggered_if("Treasure Hunter — At the beginning of your upkeep, if you control thirty or more artifacts, you win the game.", TriggerEventDef::StepBegins { step: TurnStepDef::Upkeep, player: PlayerRelation::You }, &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Artifact), &[ZoneKind::Battlefield], PlayerRelation::You)), comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(30) }), EffectDef::WinTheGame { player: EffectRecipientDef::Controller })
]),
);

// SLD 2226 — Jin Sakai, Ghost of Tsushima
// Audit: unsupported — The attack matcher's declaration count includes all attackers, including those attacking planeswalkers. It cannot test that exactly one creature attacks the particular defending player while permitting other creatures to attack planeswalkers.
pub(in crate::card::sets) static JIN_SAKAI_GHOST_OF_TSUSHIMA_2226: CardRecord = CardRecord::new(
    "Jin Sakai, Ghost of Tsushima",
    "9f52160b-dcc0-4ab8-b8b2-b576b1688d57",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &JURIN_LEADING_THE_CHARGE_1240,
    &IRON_MAN_TITAN_OF_INNOVATION_1731,
    &DEADPOOL_TRADING_CARD_1753,
    &KNUCKLES_THE_ECHIDNA_2082,
    &JIN_SAKAI_GHOST_OF_TSUSHIMA_2226,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
