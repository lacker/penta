//! Finishing a decision that a battlefield entry or exit was waiting on.
//!
//! Split out of the parent module for the source-size budget: every arm here
//! resumes a prospective zone change rather than a resolving effect.

#![allow(clippy::wildcard_imports)]

use super::*;

impl Game {
    // Long because the entry vocabulary is wide, not because the function
    // does several things: every arm resumes one prospective zone change.
    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_battlefield_entry_decision(
        &mut self,
        continuation: DecisionContinuation,
        pending_options: &[DecisionOption],
        options: &[u32],
    ) {
        match continuation {
            DecisionContinuation::BattlefieldEntryReplacement { candidates } => {
                let selected = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| candidates.get(index))
                    .copied();
                if let (Some(pending), Some(selected)) = (self.pending_events.pop_front(), selected)
                    && let Some(pending) = self.prepare_entry_replacement(pending, &selected)
                {
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryExile {
                player,
                entering,
                candidates,
            } => {
                self.resume_entry_exile_choice(player, entering, &candidates, options);
            }
            DecisionContinuation::BattlefieldEntryOptional { context, effect } => {
                self.resume_optional_entry_replacement(context, effect, options);
            }
            DecisionContinuation::BattlefieldExitReplacement {
                mut batch,
                candidates,
            } => {
                let selected = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| candidates.get(index))
                    .copied();
                if let Some(selected) = selected {
                    self.apply_battlefield_exit_replacement(&mut batch, &selected);
                    self.continue_battlefield_exit_replacements(batch);
                }
            }
            DecisionContinuation::BattlefieldEntryPayment {
                context,
                player,
                payment,
                definition,
            } => {
                if let Some(mut pending) = self.pending_events.pop_front() {
                    let paid = self
                        .settle_payment_decision(player, payment, options, pending_options)
                        .is_some();
                    let ReplacementEffectDef::PayOr {
                        if_paid,
                        if_declined,
                        ..
                    } = definition
                    else {
                        return;
                    };
                    Self::push_replacement_effects(
                        &mut pending,
                        context,
                        if paid { if_paid } else { if_declined },
                    );
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryCopy {
                choices,
                added_types,
                retain_printed_subtypes,
                added_abilities,
            } => {
                let copied = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .filter(|option| *option > 0)
                    .and_then(|option| choices.get(option - 1).copied())
                    .and_then(|id| {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                    })
                    .map(|permanent| {
                        let mut copy = Self::copiable_characteristics(permanent);
                        copy.added_types = copy.added_types.union(added_types);
                        copy.retain_printed_subtypes = retain_printed_subtypes;
                        copy.added_abilities.extend(added_abilities);
                        copy
                    });
                if let Some(mut pending) = self.pending_events.pop_front() {
                    if let Some(copy) = copied {
                        let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                        entry.permanent.copied_from = Some(copy.base);
                        entry.permanent.copy_effect = Some(copy);
                    }
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            DecisionContinuation::BattlefieldEntryScalarChoice {
                choice, choices, ..
            } => {
                let Some(selected) = options
                    .first()
                    .and_then(|option| usize::try_from(*option).ok())
                    .and_then(|index| choices.get(index))
                    .cloned()
                else {
                    return;
                };
                if let Some(mut pending) = self.pending_events.pop_front() {
                    let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                    match choice.destination {
                        BattlefieldEntryChoiceDestinationDef::Player => {
                            entry.permanent.chosen_player = match selected.as_str() {
                                "You" => Some(entry.permanent.controller),
                                "Opponent" => Some(entry.permanent.controller.opponent()),
                                _ => None,
                            };
                        }
                        BattlefieldEntryChoiceDestinationDef::CardName => {
                            entry.permanent.chosen_card_name = Some(selected);
                        }
                        BattlefieldEntryChoiceDestinationDef::CreatureType => {
                            entry.permanent.chosen_creature_type = Some(selected);
                        }
                        BattlefieldEntryChoiceDestinationDef::BasicLandType => {
                            entry.permanent.chosen_basic_land_type =
                                crate::card::BasicLandType::from_subtype(&selected);
                        }
                    }
                    self.pending_events.push_front(pending);
                    self.continue_pending_events();
                }
            }
            _ => unreachable!("only battlefield-entry continuations reach this resolver"),
        }
    }
}
