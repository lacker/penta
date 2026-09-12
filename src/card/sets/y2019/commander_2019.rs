//! Commander 2019 card records required by the cEDH corpus.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C19",
    slug: "commander-2019",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C19 5 — Sevinne's Reclamation
pub(in crate::card::sets) static SEVINNE_S_RECLAMATION_5: CardRecord = CardRecord::new(
    "Sevinne's Reclamation",
    "7e68f4df-88ce-4e09-a03c-7edf40bff167",
    "Zoltan Boros",
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_abilities(&[
AbilityDef::spell_with_targets("Return target permanent card with mana value 3 or less from your graveyard to the battlefield. If this spell was cast from a graveyard, you may copy this spell and may choose a new target for the copy.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Enchantment), ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::HasType(CardType::Planeswalker)]), ObjectPredicateDef::ManaValueAtMost(3)]), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) })], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Battlefield, ZonePlacement::Top), EffectDef::IfCondition { condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard), then: &EffectDef::May { player: EffectRecipientDef::Controller, effect: &EffectDef::CopyStackObject(&CopyStackObjectDef { object: EffectRecipientDef::Source, controller: PlayerRefDef::EffectController, count: ValueDef::Constant(1), retarget: true, colors: None }) } }])),
abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{W}"))])
]),
);

// C19 11 — Sudden Substitution
// Audit: unsupported — Control exchange cannot exchange a stack spell with a battlefield creature. Gaining control of each separately would not enforce the atomic exchange requirements.
pub(in crate::card::sets) static SUDDEN_SUBSTITUTION_11: CardRecord = CardRecord::new(
    "Sudden Substitution",
    "9f7983bf-2a3b-4428-8c01-35285f589da8",
    "Noah Bradley",
    crate::card::CardRules::unsupported(),
);

// C19 16 — Curse of Fool's Wisdom
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static CURSE_OF_FOOL_S_WISDOM_16: CardRecord = CardRecord::new(
    "Curse of Fool's Wisdom",
    "9b77ded4-a8af-4065-8c4a-fd76e7cdcc59",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// C19 18 — K'rrik, Son of Yawgmoth
// Audit: unsupported — Mana payment has printed Phyrexian symbols but no continuous rule replacing each black symbol in arbitrary costs with an optional life payment.
pub(in crate::card::sets) static K_RRIK_SON_OF_YAWGMOTH_18: CardRecord = CardRecord::new(
    "K'rrik, Son of Yawgmoth",
    "3592fbe4-8588-486e-99ba-c327b0b6ba24",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// C19 31 — Apex Altisaur
pub(in crate::card::sets) static APEX_ALTISAUR_31: CardRecord = CardRecord::new(
    "Apex Altisaur",
    "5f8b022e-3e7a-40e6-99a0-53e5adbdafc5",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{7}{G}{G}"), &["Dinosaur"], 10, 10).with_abilities(&[
abilities::enters_trigger_with_targets("When this creature enters, it fights up to one target creature you don't control.", &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::Opponent), owner: None }, 1)], EffectDef::Fight { first: ObjectRefDef::Source, second: ObjectRefDef::Target(TargetIndex::PRIMARY), excess: None }),
AbilityDef::triggered_with_targets("Enrage — Whenever this creature is dealt damage, it fights up to one target creature you don't control.", TriggerEventDef::damage_to_source(), &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::Opponent), owner: None }, 1)], EffectDef::Fight { first: ObjectRefDef::Source, second: ObjectRefDef::Target(TargetIndex::PRIMARY), excess: None })
]),
);

// C19 37 — Anje Falkenrath
// Audit: unsupported — Madness needs a discard-to-exile replacement followed by a linked triggered cast-or-graveyard choice. No existing alternative-cast procedure implements that timing and zone-change sequence.
pub(in crate::card::sets) static ANJE_FALKENRATH_37: CardRecord = CardRecord::new(
    "Anje Falkenrath",
    "913dd06f-ed2f-4128-9c9d-9cd0d8a55425",
    "Cynthia Sheppard",
    crate::card::CardRules::unsupported(),
);

// C19 41 — Gerrard, Weatherlight Hero
// Audit: unsupported — Graveyard queries do not track which current card objects entered from the battlefield during this turn. A broad artifact/creature return would also return older cards.
pub(in crate::card::sets) static GERRARD_WEATHERLIGHT_HERO_41: CardRecord = CardRecord::new(
    "Gerrard, Weatherlight Hero",
    "c835a37a-776a-44ef-a252-4af1f31bf0b3",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SEVINNE_S_RECLAMATION_5,
    &SUDDEN_SUBSTITUTION_11,
    &CURSE_OF_FOOL_S_WISDOM_16,
    &K_RRIK_SON_OF_YAWGMOTH_18,
    &APEX_ALTISAUR_31,
    &ANJE_FALKENRATH_37,
    &GERRARD_WEATHERLIGHT_HERO_41,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
