//! The Lost Caverns of Ixalan Commander card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardRules, CardSet,
    CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation, SumValueDef,
    TriggerConditionDef, ValueDef, abilities,
};
use crate::{TargetIndex, mana_cost};

// LCC 86 — Broadside Bombardiers
pub(in crate::card::sets) static BROADSIDE_BOMBARDIERS: CardRecord = CardRecord::new(
    CardSet::LostCavernsOfIxalanCommander,
    "Broadside Bombardiers",
    "9721f8da-39ed-4ada-a571-61e08a86032b",
    "Tomek Larek",
    // A hasty attacker that turns whatever else is lying around into reach.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Pirate"], 2, 2).with_abilities(&[
        abilities::menace(),
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "Boast — Sacrifice another creature or artifact: This creature deals damage equal to 2 \
                 plus the sacrificed permanent\'s mana value to any target. (Activate only if this \
                 creature attacked this turn and only once each turn.)",
            &[AbilityCostDef::SacrificePermanent {
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
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                // Two plus what was thrown. The sacrifice is a cost, so the permanent is
                // gone before the ability is even on the stack: what it was worth is read
                // back from the payment rather than from the board.
                amount: ValueDef::Sum(&SumValueDef {
                    left: ValueDef::Constant(2),
                    right: ValueDef::SacrificedManaValue,
                }),
            },
        )
        // Boast (CR 702.141) is those two restrictions and nothing else: it can
        // only be activated by a creature that attacked, and only once a turn.
        .with_activation_condition(&TriggerConditionDef::SourceMatches {
            object: ObjectPredicateDef::AttackedThisTurn,
        })
        .activations_each_turn(1),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BROADSIDE_BOMBARDIERS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
