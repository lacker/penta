//! Teenage Mutant Ninja Turtles Eternal card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::ActivationTimingDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::HalvedValueDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::RoundingDef;
use crate::card::TokenCopyDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TMC",
    slug: "teenage-mutant-ninja-turtles-eternal",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());











// TMC 20 — Shredder, Shadow Master
// Two-player games have no other opponents, so the attack trigger creates zero tokens.
// Multiplayer would need per-defender attacking-token creation and end-combat sacrifice.
pub(in crate::card::sets) static SHREDDER_SHADOW_MASTER_20: CardRecord = CardRecord::new(
    "Shredder, Shadow Master",
    "ddf4d3c9-bef9-4796-91ef-3d5beebae571",
    "Miklós Ligeti",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Human", "Ninja"], 5, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever Shredder attacks a player, for each other opponent, create a token that's a copy of Shredder tapped and attacking that player, except it isn't legendary. Sacrifice those tokens at end of combat.", TriggerEventDef::attacks_a_player(ObjectPredicateDef::Source), EffectDef::create_token_from_copy(&TokenCopyDef { object: &EffectRecipientDef::Source, exceptions: CopyExceptionsDef::NONE.without_supertypes(&[CardSupertype::Legendary]) }).with_count(ValueDef::Constant(0)).entering_tapped().entering_attacking()),
AbilityDef::triggered("Whenever Shredder deals combat damage to a player, that player loses half their life, rounded up.", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source), EffectDef::LoseLife { recipient: EffectRecipientDef::player(PlayerRefDef::EventPlayer), amount: ValueDef::Halved(&HalvedValueDef::new(ValueDef::LifeTotal(PlayerRelation::EventPlayer), RoundingDef::Up)) })
]),
);

// TMC 22 — Casey Jones, Back Alley Brute
// Audit: unsupported — Counter-placement events identify the permanent and amount but not the player who placed the counters. The trigger must distinguish your placements from an opponent placing counters on your creature.
pub(in crate::card::sets) static CASEY_JONES_BACK_ALLEY_BRUTE_22: CardRecord = CardRecord::new(
    "Casey Jones, Back Alley Brute",
    "416d888d-2699-49bd-830b-bf5bda25c91f",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// TMC 97 — Big Mother Mouser
pub(in crate::card::sets) static BIG_MOTHER_MOUSER_97: CardRecord = CardRecord::new(
    "Big Mother Mouser",
    "8f586a85-f9b7-49c4-bfb0-e348b9088841",
    "Kirokaze",
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Robot"], 0, 0).with_abilities(&[
AbilityDef::as_enters("This creature enters with two +1/+1 counters on it.", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters { kind: CounterKind::PlusOnePlusOne, amount: 2 })),
AbilityDef::triggered("Whenever this creature attacks, double the number of +1/+1 counters on it.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::PlusOnePlusOne, amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne) }),
abilities::dies_trigger("When this creature dies, create a number of 1/1 colorless Robot artifact creature tokens equal to the number of +1/+1 counters on this creature.", EffectDef::create_artifact_creature_token(&["Robot"], &[], 1, 1).with_count(ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne)))
]),
);

// TMC 107 — April O'Neil, Human Element
pub(in crate::card::sets) static APRIL_O_NEIL_HUMAN_ELEMENT_107: CardRecord = CardRecord::new(
    "April O'Neil, Human Element",
    "cd6d7bba-2de4-49fe-8e07-a7c00c6bfc96",
    "Fahmi Fauzi",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Detective"], 2, 5).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered("Whenever a player casts an artifact, instant, or sorcery spell, you create a Mutagen token. (It's an artifact with \"{1}, {T}, Sacrifice this token: Put a +1/+1 counter on target creature. Activate only as a sorcery.\")", TriggerEventDef::spell_cast(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])), EffectDef::create_artifact_token(&["Mutagen"], &[]).with_abilities(&[AbilityDef::activated_with_targets("{1}, {T}, Sacrifice this token: Put a +1/+1 counter on target creature. Activate only as a sorcery.", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource, CostDef::SacrificeSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::AddCounters { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), kind: CounterKind::PlusOnePlusOne, amount: ValueDef::Constant(1) }).with_activation_timing(ActivationTimingDef::SorcerySpeed)]))
]),
);

// TMC 118 — Raphael, Tag Team Tough
// Audit: unsupported — There is no per-source first-combat-damage-to-a-player event history for the trigger. Limiting resolutions would incorrectly allow a later hit after the first trigger was countered.
pub(in crate::card::sets) static RAPHAEL_TAG_TEAM_TOUGH_118: CardRecord = CardRecord::new(
    "Raphael, Tag Team Tough",
    "b3bae5c7-2c4a-42a6-a108-3f23d70204b4",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SHREDDER_SHADOW_MASTER_20,
    &CASEY_JONES_BACK_ALLEY_BRUTE_22,
    &BIG_MOTHER_MOUSER_97,
    &APRIL_O_NEIL_HUMAN_ELEMENT_107,
    &RAPHAEL_TAG_TEAM_TOUGH_118,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
