//! Dealing an effect's damage, and reporting where it landed.

use crate::card::{DamageAssignmentDef, DamageDef, DamageFollowUpDef, FightExcessDef};

use super::super::{
    CardType, DamageAssignment, EffectResolutionContext, Game, ObjectRefDef, ScopedEffect,
    StackObject, Target, ValueDef,
};

impl Game {
    pub(super) fn resolve_damage_effect(
        &mut self,
        definition: DamageDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let mut assignments = Vec::new();
        for assignment in definition.assignments() {
            self.append_damage_assignments(*assignment, object, &context, scoped, &mut assignments);
        }
        let intended = if definition.continuation().is_some() {
            assignments
                .iter()
                .filter_map(|assignment| assignment.target)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let outcome = self.deal_damage_simultaneously(assignments);
        match definition.follow_up {
            Some(DamageFollowUpDef::IfDealtToIntended(then)) => {
                if outcome
                    .recipients
                    .iter()
                    .any(|damaged| intended.contains(&damaged.recipient))
                {
                    self.resolve_effect_def(scoped.with_effect(*then), object, context);
                }
            }
            Some(DamageFollowUpDef::ApplyToDamaged { effect, duration }) => {
                let damaged = outcome
                    .recipients
                    .into_iter()
                    .map(|outcome| outcome.recipient)
                    .collect::<Vec<_>>();
                self.apply_effect_to_targets(&damaged, *effect, duration, object, &context, scoped);
            }
            None => {}
        }
    }

    pub(super) fn fight(
        &mut self,
        first: ObjectRefDef,
        second: ObjectRefDef,
        excess: Option<FightExcessDef>,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(first) = self.effect_object_reference_id(first, object, context, scoped) else {
            return;
        };
        let Some(second) = self.effect_object_reference_id(second, object, context, scoped) else {
            return;
        };
        let participant = |game: &Self, id| {
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .filter(|permanent| {
                    game.permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Creature))
                })
                .and_then(|permanent| game.power(permanent))
                .map(|power| power.max(0).cast_unsigned())
        };
        let (Some(first_power), Some(second_power)) =
            (participant(self, first), participant(self, second))
        else {
            return;
        };
        let excess_recipient = excess.and_then(|continuation| {
            self.effect_object_reference_id(continuation.recipient, object, context, scoped)
        });
        let assignments = if first == second {
            vec![DamageAssignment {
                source: Some(first),
                target: Some(Target::Permanent(first)),
                amount: first_power.saturating_mul(2),
                combat: false,
            }]
        } else {
            vec![
                DamageAssignment {
                    source: Some(first),
                    target: Some(Target::Permanent(second)),
                    amount: first_power,
                    combat: false,
                },
                DamageAssignment {
                    source: Some(second),
                    target: Some(Target::Permanent(first)),
                    amount: second_power,
                    combat: false,
                },
            ]
        };
        let outcome = self.deal_damage_simultaneously(assignments);
        let (Some(continuation), Some(excess_recipient)) = (excess, excess_recipient) else {
            return;
        };
        let Some(excess_amount) = outcome
            .recipients
            .iter()
            .find(|outcome| outcome.recipient == Target::Permanent(excess_recipient))
            .map(|outcome| outcome.excess)
            .filter(|amount| *amount > 0)
        else {
            return;
        };
        let mut nested = context.clone();
        nested.matched_count = Some(excess_amount);
        self.resolve_effect_def(scoped.with_effect(*continuation.then), object, nested);
    }

    /// Materialize every assignment before committing any damage. The same
    /// evaluator handles a single instruction, explicit sources (including
    /// last-known information), and each part of a simultaneous batch.
    fn append_damage_assignments(
        &self,
        definition: DamageAssignmentDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        assignments: &mut Vec<DamageAssignment>,
    ) {
        let DamageAssignmentDef {
            source,
            recipient,
            amount,
        } = definition;
        let source = match source {
            Some(reference) => {
                let Some(source) =
                    self.effect_object_reference_id(reference, object, context, scoped)
                else {
                    return;
                };
                Some(source)
            }
            None => object.source.or(Some(object.id)),
        };
        // A divided total is chosen per target when the spell is
        // cast, so each one takes its own share rather than the same
        // amount as everyone else.
        let divided = matches!(amount, ValueDef::DividedAmongTargets);
        let recipients = self.effect_recipients(recipient, object, context, scoped);
        let shared = if divided {
            0
        } else {
            self.resolved_damage_value(amount, recipients.len(), object, context, scoped)
                .max(0)
                .try_into()
                .unwrap_or(u16::MAX)
        };
        let slot = recipient
            .legal_target()
            .map(|target| scoped.target_slot(target));
        for target in recipients {
            let amount = if divided {
                slot.and_then(|slot| Self::divided_share(object, slot, target))
                    .unwrap_or(0)
            } else {
                shared
            };
            if amount == 0 && divided {
                continue;
            }
            assignments.push(DamageAssignment {
                source,
                target: Some(target),
                amount,
                combat: false,
            });
        }
    }

    fn resolved_damage_value(
        &self,
        value: ValueDef,
        recipient_count: usize,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> i32 {
        match value {
            ValueDef::ResolvedRecipientCount => i32::try_from(recipient_count).unwrap_or(i32::MAX),
            ValueDef::Quotient(quotient) => quotient.apply(
                self.resolved_damage_value(
                    quotient.numerator,
                    recipient_count,
                    object,
                    context,
                    scoped,
                ),
                self.resolved_damage_value(
                    quotient.denominator,
                    recipient_count,
                    object,
                    context,
                    scoped,
                ),
            ),
            _ => self.effect_value(value, object, context, scoped),
        }
    }
}
