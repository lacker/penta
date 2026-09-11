//! Rivals of Ixalan card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "RIX",
    slug: "rivals-of-ixalan",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// RIX 101 — Fanatical Firebrand
pub(in crate::card::sets) static FANATICAL_FIREBRAND: CardRecord = CardRecord::new(
    "Fanatical Firebrand",
    "5e5565de-028c-4799-a9f6-4dcd685639eb",
    "Wayne Reynolds",
    // Haste is what makes the sacrifice a one-mana Shock the turn it lands;
    // left alive it is a one-power attacker that can cash itself in later.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Pirate"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: It deals 1 damage to any target.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FANATICAL_FIREBRAND];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
