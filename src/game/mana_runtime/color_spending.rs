//! Deriving mana-payment strategy from values the spell will later read.

use super::super::{
    AbilityDef, BattlefieldEntryModificationDef, EffectDef, Game, ManaPaymentPurpose,
    ObjectPredicateDef, ReplacementEffectDef, TriggerConditionDef, ValueDef,
};
use crate::card::AbilityProgramDef;

impl Game {
    /// Whether the spell being paid for reads how many colours paid for it.
    /// This is derived from the ability program that consumes the cast
    /// context, rather than restated as a card-level Converge marker.
    pub(in crate::game) fn payment_counts_colors_spent(
        &self,
        purpose: &ManaPaymentPurpose,
    ) -> bool {
        let ManaPaymentPurpose::Spell { definition, .. } = purpose else {
            return false;
        };
        self.catalog.get(*definition).is_some_and(|card| {
            card.rules
                .indexed_abilities()
                .any(|attached| Self::ability_reads_colors_spent(&attached.definition))
        })
    }

    fn ability_reads_colors_spent(ability: &AbilityDef) -> bool {
        match ability.effect.definition {
            AbilityProgramDef::Effects(effect) => Self::effect_reads_colors_spent(effect),
            AbilityProgramDef::Replacement(effect) => Self::replacement_reads_colors_spent(effect),
        }
    }

    fn replacement_reads_colors_spent(effect: ReplacementEffectDef) -> bool {
        match effect {
            ReplacementEffectDef::Sequence(effects) => effects
                .iter()
                .copied()
                .any(Self::replacement_reads_colors_spent),
            ReplacementEffectDef::Perform(effect) => Self::effect_reads_colors_spent(*effect),
            ReplacementEffectDef::Conditional {
                if_true, if_false, ..
            } => if_true
                .iter()
                .chain(if_false)
                .copied()
                .any(Self::replacement_reads_colors_spent),
            ReplacementEffectDef::PayOr {
                if_paid,
                if_declined,
                ..
            } => if_paid
                .iter()
                .chain(if_declined)
                .copied()
                .any(Self::replacement_reads_colors_spent),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue { amount, .. },
            ) => Self::value_reads_colors_spent(amount),
            _ => false,
        }
    }

    fn effect_reads_colors_spent(effect: EffectDef) -> bool {
        let direct = match effect {
            EffectDef::IfCondition { condition, .. }
            | EffectDef::IfElseCondition { condition, .. } => {
                Self::trigger_condition_reads_colors_spent(*condition)
            }
            EffectDef::InstallTrigger(trigger) => Self::ability_reads_colors_spent(trigger.ability),
            EffectDef::CreateOngoingEffect(ongoing) => {
                Self::ability_reads_colors_spent(ongoing.ability)
            }
            _ => false,
        };
        direct
            || crate::card::child_effects(effect)
                .into_iter()
                .any(Self::effect_reads_colors_spent)
    }

    fn trigger_condition_reads_colors_spent(condition: TriggerConditionDef) -> bool {
        match condition {
            TriggerConditionDef::All(conditions) | TriggerConditionDef::AnyOf(conditions) => {
                conditions
                    .iter()
                    .copied()
                    .any(Self::trigger_condition_reads_colors_spent)
            }
            TriggerConditionDef::Not(condition) => {
                Self::trigger_condition_reads_colors_spent(*condition)
            }
            TriggerConditionDef::ValueComparison(comparison) => {
                Self::value_reads_colors_spent(comparison.left)
                    || Self::value_reads_colors_spent(comparison.right)
            }
            TriggerConditionDef::SourceMatches { object }
            | TriggerConditionDef::AttachedPermanentMatches { object }
            | TriggerConditionDef::BoundObjectMatches { object, .. }
            | TriggerConditionDef::TargetMatches { object, .. } => {
                Self::predicate_reads_colors_spent(object)
            }
            _ => false,
        }
    }

    fn predicate_reads_colors_spent(predicate: ObjectPredicateDef) -> bool {
        match predicate {
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates
                    .iter()
                    .copied()
                    .any(Self::predicate_reads_colors_spent)
            }
            ObjectPredicateDef::Not(predicate)
            | ObjectPredicateDef::TargetsObjectMatching(predicate)
            | ObjectPredicateDef::AttachedTo(predicate) => {
                Self::predicate_reads_colors_spent(*predicate)
            }
            ObjectPredicateDef::ManaValueEqualTo(value)
            | ObjectPredicateDef::ManaValueAtMostValue(value)
            | ObjectPredicateDef::ToughnessLessThan(value)
            | ObjectPredicateDef::PowerGreaterThan(value)
            | ObjectPredicateDef::ToughnessGreaterThan(value)
            | ObjectPredicateDef::PowerLessThan(value) => Self::value_reads_colors_spent(value),
            _ => false,
        }
    }

    fn value_reads_colors_spent(value: ValueDef) -> bool {
        match value {
            ValueDef::ColorsOfManaSpent => true,
            ValueDef::Negate(value) => Self::value_reads_colors_spent(*value),
            ValueDef::Scaled(value) => Self::value_reads_colors_spent(value.value),
            ValueDef::Halved(value) => Self::value_reads_colors_spent(value.value),
            ValueDef::Quotient(value) => {
                Self::value_reads_colors_spent(value.numerator)
                    || Self::value_reads_colors_spent(value.denominator)
            }
            ValueDef::Sum(value) => {
                Self::value_reads_colors_spent(value.left)
                    || Self::value_reads_colors_spent(value.right)
            }
            ValueDef::IfAdditionalCostPaid(value) => {
                Self::value_reads_colors_spent(value.if_paid)
                    || Self::value_reads_colors_spent(value.otherwise)
            }
            ValueDef::IfControllerLifeAtMost(value) => {
                Self::value_reads_colors_spent(value.then)
                    || Self::value_reads_colors_spent(value.otherwise)
            }
            ValueDef::IfCreatureDiedThisTurn(value) => {
                Self::value_reads_colors_spent(value.then)
                    || Self::value_reads_colors_spent(value.otherwise)
            }
            ValueDef::IfSourceMatches(value) => {
                Self::predicate_reads_colors_spent(value.object)
                    || Self::value_reads_colors_spent(value.then)
                    || Self::value_reads_colors_spent(value.otherwise)
            }
            ValueDef::IfTargetMatches(value) => {
                Self::predicate_reads_colors_spent(value.object)
                    || Self::value_reads_colors_spent(value.then)
                    || Self::value_reads_colors_spent(value.otherwise)
            }
            ValueDef::IfMatchingObjectCount(value) => {
                Self::predicate_reads_colors_spent(value.query.object)
                    || Self::value_reads_colors_spent(value.then)
                    || Self::value_reads_colors_spent(value.otherwise)
            }
            _ => false,
        }
    }
}
