//! Effects that only set a field on a permanent.
//!
//! Split out of the parent module for the source-size budget. Each of these
//! finds the permanents a recipient names and records something on them.

#![allow(clippy::wildcard_imports)]

use super::*;
use crate::card::{CounterKind, CounterKindDef, CounterOperationDef, EffectChoiceDef, ValueDef};
use crate::game::{
    CommittedTriggerEvent, DecisionContinuation, DecisionOption, DecisionPreference,
    DecisionVisibility, DecisionZone,
};
use crate::ids::GameObjectId;
use crate::{CharacteristicContext, EffectRecipientDef};

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
        let current = permanent.counters(CounterKind::named("level"));
        if current >= wanted {
            return;
        }
        permanent.set_counters(CounterKind::named("level"), wanted);
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
            | EffectDef::ChooseCounterKind { .. }
            | EffectDef::ChooseEffect { .. }
            | EffectDef::ModifyCounters { .. }
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
                        if let Some(kind) = kind {
                            let held = permanent.counters(kind);
                            permanent.remove_counters(kind, held);
                        } else {
                            // No kind named is every kind, including names no
                            // other card in the current catalog happens to use.
                            permanent.counters.clear();
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

    fn queue_counter_kind_choice(
        &mut self,
        recipient: EffectRecipientDef,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let Some(target) = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .next()
        else {
            return;
        };
        let kinds = self.counter_kinds_on(target);
        if kinds.is_empty() {
            return;
        }
        let options = kinds
            .iter()
            .enumerate()
            .map(|(index, kind)| DecisionOption {
                id: u32::try_from(index).expect("counter choices fit u32"),
                label: format!("Choose a {} counter", kind.name()),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            object.controller,
            "Choose a counter",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChooseCounter {
                object: Box::new(object.clone()),
                context: context.clone(),
                scoped,
                target,
                kinds,
            },
        );
    }

    fn queue_resolution_effect_choice(
        &mut self,
        player: EffectRecipientDef,
        choices: &'static [EffectChoiceDef],
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let Some(player) = self
            .effect_recipients(player, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                Target::Player(player) => Some(player),
                _ => None,
            })
        else {
            return;
        };
        self.queue_decision(
            player,
            "Choose one",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            choices
                .iter()
                .enumerate()
                .map(|(index, choice)| DecisionOption {
                    id: u32::try_from(index).expect("effect choices fit u32"),
                    label: choice.label.into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                })
                .collect(),
            DecisionContinuation::ChooseEffect {
                object: Box::new(object.clone()),
                context: context.clone(),
                scoped,
            },
        );
    }

    /// Counters put on or taken off the permanents a recipient names.
    fn resolve_counter_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::ChooseCounterKind {
                object: recipient, ..
            } => {
                self.queue_counter_kind_choice(recipient, scoped, object, context);
            }
            EffectDef::ChooseEffect { player, choices } => {
                self.queue_resolution_effect_choice(player, choices, scoped, object, context);
            }
            EffectDef::ModifyCounters {
                object: recipient,
                kind,
                operation,
                amount,
            } => {
                self.resolve_counter_modification(
                    recipient, kind, operation, amount, scoped, object, context,
                );
            }
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
                    match target {
                        Target::Permanent(permanent) => {
                            if let Some(permanent) = self
                                .battlefield
                                .iter_mut()
                                .find(|candidate| candidate.card.id == permanent)
                            {
                                permanent.add_counters(kind, amount);
                                placed.push(permanent.card.id);
                            }
                        }
                        // "You get an experience counter." A player keeps
                        // their own pile the way they keep poison and
                        // energy, and nothing on the battlefield is
                        // involved -- so nothing watching permanents for
                        // counters sees this one.
                        Target::Player(player) => {
                            self.players[player.index()].counters.add(kind, amount);
                        }
                        Target::Card(card) => {
                            if let Some(card) = self.card_in_nonbattlefield_zone_mut(card) {
                                card.counters.add(kind, amount);
                            }
                        }
                        Target::Spell(_) => {}
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
                    self.remove_counters_from_object(target, kind, amount);
                }
            }
            _ => unreachable!("only counter effects reach the counter resolver"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_counter_modification(
        &mut self,
        recipient: EffectRecipientDef,
        kind: CounterKindDef,
        operation: CounterOperationDef,
        amount: ValueDef,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        let kind = match kind {
            CounterKindDef::Fixed(kind) => kind,
            CounterKindDef::Chosen => {
                let Some(kind) = context.chosen_counter else {
                    return;
                };
                kind
            }
        };
        let amount = self
            .effect_value(amount, object, context, scoped)
            .max(0)
            .try_into()
            .unwrap_or(u16::MAX);
        for target in self.effect_recipients(recipient, object, context, scoped) {
            self.modify_counters(target, kind, operation, amount);
        }
    }

    pub(in crate::game) fn counter_kinds_on(&self, target: Target) -> Vec<CounterKind> {
        match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map_or_else(Vec::new, |permanent| {
                    permanent.counters.iter().map(|(kind, _)| kind).collect()
                }),
            Target::Card(id) => self
                .card_in_nonbattlefield_zone(id)
                .map_or_else(Vec::new, |(_, card)| {
                    card.counters.iter().map(|(kind, _)| kind).collect()
                }),
            Target::Player(player) => self.players[player.index()]
                .counters
                .iter()
                .map(|(kind, _)| kind)
                .collect(),
            Target::Spell(_) => Vec::new(),
        }
    }

    pub(in crate::game) fn modify_counters(
        &mut self,
        target: Target,
        kind: CounterKind,
        operation: crate::card::CounterOperationDef,
        amount: u16,
    ) {
        match operation {
            crate::card::CounterOperationDef::Remove => {
                self.remove_counters_from_object(target, kind, amount);
            }
            crate::card::CounterOperationDef::Add => match target {
                Target::Permanent(id) => {
                    if let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == id)
                    {
                        permanent.add_counters(kind, amount);
                        self.capture_counters_placed(&[id], kind, amount);
                    }
                }
                Target::Card(id) => {
                    if let Some(card) = self.card_in_nonbattlefield_zone_mut(id) {
                        card.counters.add(kind, amount);
                    }
                }
                Target::Player(player) => self.players[player.index()].counters.add(kind, amount),
                Target::Spell(_) => {}
            },
        }
    }

    pub(in crate::game) fn remove_counters_from_object(
        &mut self,
        target: Target,
        kind: CounterKind,
        amount: u16,
    ) -> u16 {
        let (object, held) = match target {
            Target::Permanent(id) => {
                let Some(permanent) = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                else {
                    return 0;
                };
                (
                    self.trigger_event_object(permanent),
                    permanent.counters(kind),
                )
            }
            Target::Card(id) => {
                let Some((zone, card)) = self.card_in_nonbattlefield_zone(id) else {
                    return 0;
                };
                let context = match zone {
                    ZoneKind::Library => CharacteristicContext::Library,
                    ZoneKind::Hand => CharacteristicContext::Hand,
                    ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                    ZoneKind::Exile => CharacteristicContext::Exile,
                    ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return 0,
                };
                let Some(object) =
                    self.printed_trigger_event_object(id, card.definition, card.owner, &context)
                else {
                    return 0;
                };
                (object, card.counters.count(kind))
            }
            Target::Player(_) | Target::Spell(_) => return 0,
        };
        let removed = held.min(amount);
        if removed == 0 {
            return 0;
        }
        match target {
            Target::Permanent(id) => {
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == id)
                {
                    permanent.remove_counters(kind, removed);
                }
            }
            Target::Card(id) => {
                if let Some(card) = self.card_in_nonbattlefield_zone_mut(id) {
                    card.counters.remove(kind, removed);
                }
            }
            Target::Player(_) | Target::Spell(_) => unreachable!(),
        }
        let remaining = held - removed;
        self.capture_battlefield_triggers(&CommittedTriggerEvent::CountersRemoved {
            object,
            kind,
            amount: removed,
            remaining,
        });
        removed
    }
}
