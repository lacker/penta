//! Tarkir: Dragonstorm Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::AttackDeclarationRangeDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TDC",
    slug: "tarkir-dragonstorm-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());







// TDC 3 — Eshki, Temur's Roar
pub(in crate::card::sets) static ESHKI_TEMUR_S_ROAR_3: CardRecord = CardRecord::new(
    "Eshki, Temur's Roar",
    "ff9aa863-8773-452d-946c-ae334c632e11",
    "Billy Christian",
    CardRules::new_creature(mana_cost!("{G}{U}{R}"), &["Human", "Warrior"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever you cast a creature spell, put a +1/+1 counter on Eshki. If that spell's power is 4 or greater, draw a card. If that spell's power is 6 or greater, Eshki deals damage equal to Eshki's power to each opponent.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::Sequence(&[EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }, EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::TriggeringObjectPower, comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(4) }), then: &abilities::draw_cards(ValueDef::Constant(1)) }, EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::TriggeringObjectPower, comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(6) }), then: &EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::SourcePower) }]))
]),
);

// TDC 51 — Ainok Strike Leader
pub(in crate::card::sets) static AINOK_STRIKE_LEADER_51: CardRecord = CardRecord::new(
    "Ainok Strike Leader",
    "2d1102e4-447a-4b40-8131-5b36b8979fe0",
    "Alexander Mokhov",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog", "Warrior"], 2, 2).with_abilities(&[
AbilityDef::triggered("Whenever you attack with this creature and/or your commander, for each opponent, create a 1/1 red Goblin creature token that's tapped and attacking that player.", TriggerEventDef::AttackDeclared { attacker: ObjectPredicateDef::All(&[ObjectPredicateDef::ControlledBy(PlayerRelation::You), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Source, ObjectPredicateDef::All(&[ObjectPredicateDef::Commander, ObjectPredicateDef::OwnedBy(PlayerRelation::You)])])]), declaration: AttackDeclarationRangeDef::ANY }, EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1).entering_tapped().entering_attacking()),
AbilityDef::activated("Sacrifice this creature: Creature tokens you control gain indestructible until end of turn.", &[CostDef::SacrificeSource], EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Token]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&abilities::indestructible()), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// TDC 80 — Will of the Jeskai
// Audit: unsupported — Conditional mode counts and granted flashback are supported, but the first mode needs simultaneous optional whole-hand discard choices in APNAP order. Separate May effects would resolve one player's discard and draw before the other chooses.
pub(in crate::card::sets) static WILL_OF_THE_JESKAI_80: CardRecord = CardRecord::new(
    "Will of the Jeskai",
    "3ef5e587-8b80-4ea9-9788-367541101ecc",
    "Jessica Fong",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ESHKI_TEMUR_S_ROAR_3,
    &AINOK_STRIKE_LEADER_51,
    &WILL_OF_THE_JESKAI_80,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
