//! Dealing an effect's damage, and reporting where it landed.

use crate::card::{DamageAssignmentDef, FightExcessDef};

use super::super::{
    CardType, DamageAssignment, EffectRecipientDef, EffectResolutionContext, Game, ObjectRefDef,
    ScopedEffect, StackObject, Target, ValueDef,
};

impl Game {
    pub(super) fn deal_simultaneous_effect_damage(
        &mut self,
        definitions: &[DamageAssignmentDef],
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let ordinary_source = object.source.or(Some(object.id));
        let mut assignments = Vec::new();
        for definition in definitions {
            let source = match definition.source {
                Some(reference) => {
                    self.effect_object_reference_id(reference, object, context, scoped)
                }
                None => ordinary_source,
            };
            if definition.source.is_some() && source.is_none() {
                continue;
            }
            let amount = self
                .effect_value(definition.amount, object, context, scoped)
                .max(0)
                .try_into()
                .unwrap_or(u16::MAX);
            assignments.extend(
                self.effect_recipients(definition.recipient, object, context, scoped)
                    .into_iter()
                    .map(|target| DamageAssignment {
                        source,
                        target: Some(target),
                        amount,
                        combat: false,
                    }),
            );
        }
        self.deal_damage_simultaneously(assignments);
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

    /// Deals one effect's damage and reports the permanents that actually took
    /// some, in the order they were damaged.
    ///
    /// The report is what a "dealt damage this way" rider needs. A recipient
    /// can be named and still take nothing -- prevention, protection, a
    /// redirect that moves the damage to some other permanent -- and it can
    /// take damage without ever having been named, which is the other half of
    /// what redirection does. Only permanents are reported: no rider in the
    /// supported pool follows damage to a player, and a player is not a
    /// creature.
    pub(super) fn deal_effect_damage(
        &mut self,
        recipient: EffectRecipientDef,
        amount: ValueDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        let source = object.source.or(Some(object.id));
        self.deal_effect_damage_from_id(source, recipient, amount, object, context, scoped)
    }

    /// Deals one effect's damage under the exact object identity named by
    /// `source`. The referenced object may already be retired; damage source
    /// matching and attribution deliberately read its last-known information.
    pub(super) fn deal_effect_damage_from(
        &mut self,
        source: ObjectRefDef,
        recipient: EffectRecipientDef,
        amount: ValueDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        let Some(source) = self.effect_object_reference_id(source, object, context, scoped) else {
            return Vec::new();
        };
        self.deal_effect_damage_from_id(Some(source), recipient, amount, object, context, scoped)
    }

    #[allow(clippy::too_many_arguments)]
    fn deal_effect_damage_from_id(
        &mut self,
        source: Option<crate::GameObjectId>,
        recipient: EffectRecipientDef,
        amount: ValueDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        // A divided total is chosen per target when the spell is
        // cast, so each one takes its own share rather than the same
        // amount as everyone else.
        let divided = matches!(amount, ValueDef::DividedAmongTargets);
        let shared = if divided {
            0
        } else {
            self.effect_value(amount, object, context, scoped)
                .max(0)
                .try_into()
                .unwrap_or(u16::MAX)
        };
        let slot = recipient
            .legal_target()
            .map(|target| scoped.target_slot(target));
        let mut assignments = Vec::new();
        for target in self.effect_recipients(recipient, object, context, scoped) {
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
        self.deal_damage_simultaneously(assignments)
            .recipients
            .into_iter()
            .map(|outcome| outcome.recipient)
            .collect()
    }
}
