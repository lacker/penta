//! The Lost Caverns of Ixalan Commander card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::SumValueDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet =
    crate::card::CardSet::new("LCC", "lost-caverns-of-ixalan-commander");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// LCC 86 — Broadside Bombardiers
pub(in crate::card::sets) static BROADSIDE_BOMBARDIERS: CardRecord = CardRecord::new(
    "Broadside Bombardiers",
    "9721f8da-39ed-4ada-a571-61e08a86032b",
    "Tomek Larek",
    // A hasty attacker that turns whatever else is lying around into reach.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Pirate"], 2, 2).with_abilities(&[
        abilities::menace(),
        abilities::haste(),
        abilities::boast(AbilityDef::activated_with_targets(
            "Boast — Sacrifice another creature or artifact: This creature deals damage equal to 2 \
                 plus the sacrificed permanent\'s mana value to any target. (Activate only if this \
                 creature attacked this turn and only once each turn.)",
            &[CostDef::SacrificePermanent {
                // "Another creature or artifact": the Goblin cannot throw itself, which is
                // what keeps the ability from being a one-shot Shock.
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY), // Two plus what was thrown. The sacrifice is a cost, so the permanent is
                // gone before the ability is even on the stack: what it was worth is read
                // back from the payment rather than from the board.
                ValueDef::Sum(&SumValueDef {
                    left: ValueDef::Constant(2),
                    right: ValueDef::SacrificedManaValue,
                }),
            ),
        )),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BROADSIDE_BOMBARDIERS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
