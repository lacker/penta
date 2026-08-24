//! Effects that only set a field on a permanent.
//!
//! Split out of the parent module for the source-size budget. Each of these
//! finds the permanents a recipient names and records something on them.

#![allow(clippy::wildcard_imports)]

use super::*;
use crate::card::CounterKind;
use crate::game::CommittedTriggerEvent;
use crate::ids::GameObjectId;

impl Game {
    /// "Level N": put level counters on a Class until it is that level, and
    /// raise an event for each level it passes through. A Class is level 1
    /// with no counters (CR 717.3), so level N is N-1 counters.
    pub(in crate::game) fn raise_class_level(&mut self, source: GameObjectId, level: u8) {
        let Some(wanted) = u16::from(level).checked_sub(1) else {
            return;
        };
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        let current = permanent.counters(CounterKind::Level);
        if current >= wanted {
            return;
        }
        permanent.set_counters(CounterKind::Level, wanted);
        // One event per level crossed, so a Class taken from one to three by
        // a single effect fires both of its clauses.
        for reached in current + 1..=wanted {
            let Ok(reached) = u8::try_from(reached + 1) else {
                continue;
            };
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecameLevel {
                object: source,
                level: reached,
            });
        }
    }

    pub(super) fn resolve_permanent_state_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::AddCounters { .. }
            | EffectDef::DoubleCounters { .. }
            | EffectDef::RemoveCounters { .. } => {
                self.resolve_counter_effect(scoped, object, context);
            }
            EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { chooser } => {
                self.queue_basic_land_type_substitution(object, context, scoped, chooser);
            }
            EffectDef::PhaseOut { object: recipient } => {
                self.phase_out_recipients(recipient, object, context, scoped);
            }
            EffectDef::RemoveAllCounters {
                object: recipient,
                kind,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let Target::Permanent(id) = target else {
                        continue;
                    };
                    if let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == id)
                    {
                        // No kind named is every kind, which is what
                        // "remove all counters" says.
                        for kind in
                            kind.map_or_else(|| CounterKind::ALL.to_vec(), |kind| vec![kind])
                        {
                            let held = permanent.counters(kind);
                            permanent.remove_counters(kind, held);
                        }
                    }
                }
            }
            EffectDef::SkipNextUntapSteps {
                object: recipient,
                count,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let Target::Permanent(id) = target else {
                        continue;
                    };
                    if let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == id)
                    {
                        // Two of these stack rather than overwrite: a creature
                        // told twice to sit out sits out twice.
                        permanent.skipped_untap_steps =
                            permanent.skipped_untap_steps.saturating_add(count);
                    }
                }
            }
            _ => unreachable!("only permanent-state effects are dispatched here"),
        }
    }

    /// Counters put on or taken off the permanents a recipient names.
    fn resolve_counter_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::AddCounters {
                object: recipient,
                kind,
                amount,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let mut placed = Vec::new();
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(permanent) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == permanent)
                    {
                        permanent.add_counters(kind, amount);
                        placed.push(permanent.card.id);
                    }
                }
                self.capture_counters_placed(&placed, kind, amount);
            }
            EffectDef::DoubleCounters {
                object: recipient,
                kind,
            } => {
                // Each permanent's own count, read as that permanent is
                // reached: doubling is not one amount handed to everybody.
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(permanent) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == permanent)
                    {
                        let existing = permanent.counters(kind);
                        permanent.add_counters(kind, existing);
                    }
                }
            }
            EffectDef::RemoveCounters {
                object: recipient,
                kind,
                amount,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(permanent) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == permanent)
                    {
                        permanent.remove_counters(kind, amount);
                    }
                }
            }
            _ => unreachable!("only counter effects reach the counter resolver"),
        }
    }
}
