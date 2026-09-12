//! Shared action execution. Payment planning and ordinary resolution enter
//! these same operations after applying their own selection requirements.
mod choices;
pub(super) mod payments;

use super::{
    BattlefieldExitCompletion, CommittedTriggerEvent, EffectResolutionContext, Game, GameObjectId,
    PlayerId, ScopedEffect, StackObject, Target, ZoneMoveCause,
};
use crate::card::{
    ChooseDef, EffectDef, GameActionChoiceDef, GameActionDef, MechanicId, ObjectChoiceBindingDef,
    ZoneKind,
};

impl Game {
    pub(super) fn resolve_game_action(
        &mut self,
        action: GameActionDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        #[cfg(feature = "engine-profiling")]
        crate::engine_profiling::record(
            "game_action_dispatch",
            crate::engine_profiling::action_kind(action),
            "resolution",
            "entered",
        );
        match action.unnamed() {
            GameActionDef::Choice(_) => {
                let choices = action
                    .alternatives()
                    .into_iter()
                    .map(|action| self.resolve_action_payment(action, object, &context, scoped, 1))
                    .collect();
                self.queue_action_choice(object.controller, choices, None, scoped, object, context);
            }
            GameActionDef::Sequence(actions) => self.resolve_effects_in_order(
                actions
                    .iter()
                    .map(|action| scoped.with_effect(EffectDef::Perform(*action)))
                    .collect(),
                object,
                context,
            ),
            GameActionDef::MoveToZone { .. } => {
                self.resolve_move_to_zone_effect(action.as_effect(), object, &context, scoped);
            }
            GameActionDef::Choose(choice) => {
                let definition = self.fixed_game_action_choice(choice, object, &context, scoped);
                self.queue_effect_choice_with_continuation(
                    definition,
                    EffectDef::Perform(action.selected_action()),
                    object,
                    context,
                    scoped,
                );
            }
            leaf => {
                let (GameActionDef::DiscardCards { object: recipient }
                | GameActionDef::Sacrifice { object: recipient }
                | GameActionDef::SacrificeYours { object: recipient }
                | GameActionDef::Exile {
                    object: recipient, ..
                }
                | GameActionDef::GainControl {
                    object: recipient, ..
                }) = leaf
                else {
                    unreachable!("composite actions were handled above")
                };
                let receiver = if let GameActionDef::GainControl { controller, .. } = leaf {
                    let Some(player) =
                        self.effect_player_reference(controller, object, &context, scoped)
                    else {
                        return;
                    };
                    player
                } else {
                    object.controller
                };
                let targets = self.effect_recipients(recipient, object, &context, scoped);
                self.perform_selected_game_action(
                    action,
                    &targets,
                    object.controller,
                    receiver,
                    object.source.unwrap_or(object.id),
                );
            }
        }
    }

    pub(super) fn fixed_game_action_choice(
        &self,
        choice: GameActionChoiceDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> ChooseDef {
        let amount = usize::try_from(
            self.effect_value(choice.amount, object, context, scoped)
                .max(0),
        )
        .unwrap_or(usize::MAX);
        ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(choice.binding),
            unchosen: None,
            chooser: choice.chooser,
            candidates: choice.candidates,
            exclude: None,
            minimum: amount,
            maximum: amount,
            visibility: choice.visibility,
            // The shared choice procedure receives the actual continuation
            // by value; the original action tree remains the checkpoint source.
            then: &EffectDef::None,
        }
    }

    pub(super) fn perform_selected_game_action(
        &mut self,
        action: GameActionDef,
        targets: &[Target],
        performer: PlayerId,
        receiver: PlayerId,
        source: GameObjectId,
    ) {
        self.perform_selected_game_action_then(action, targets, performer, receiver, source, None);
    }

    pub(in crate::game) fn capture_mechanic(&mut self, mechanic: MechanicId, player: PlayerId) {
        self.capture_battlefield_triggers(&CommittedTriggerEvent::MechanicPerformed {
            mechanic,
            player,
        });
    }

    /// The action owns semantic events; callers supply only their continuation.
    /// Replaced sacrifices are still sacrifices, so completion waits for the
    /// shared battlefield-exit procedure rather than testing the final zone.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn perform_selected_game_action_then(
        &mut self,
        action: GameActionDef,
        targets: &[Target],
        performer: PlayerId,
        receiver: PlayerId,
        source: GameObjectId,
        then: Option<BattlefieldExitCompletion>,
    ) -> Vec<GameObjectId> {
        let completion = if let GameActionDef::Named { mechanic, .. } = action {
            Some(BattlefieldExitCompletion::Completions(
                std::iter::once(BattlefieldExitCompletion::MechanicPerformed {
                    mechanic,
                    player: performer,
                })
                .chain(then)
                .collect(),
            ))
        } else {
            then
        };
        let mut exiled = Vec::new();
        match action.unnamed() {
            GameActionDef::Exile {
                from: ZoneKind::Graveyard,
                ..
            } => {
                for player in [self.active_player, self.active_player.opponent()] {
                    let cards = targets
                        .iter()
                        .filter_map(|target| {
                            let Target::Card(id) = target else {
                                return None;
                            };
                            self.players[player.index()]
                                .graveyard
                                .iter()
                                .any(|card| card.id == *id)
                                .then_some(*id)
                        })
                        .collect::<Vec<_>>();
                    exiled.extend(self.exile_graveyard_cards(player, &cards));
                }
            }
            GameActionDef::DiscardCards { .. } => {
                let cause = ZoneMoveCause::Effect {
                    controller: performer,
                };
                for player in [self.active_player, self.active_player.opponent()] {
                    let cards = targets
                        .iter()
                        .filter_map(|target| {
                            let Target::Card(id) = target else {
                                return None;
                            };
                            self.players[player.index()]
                                .hand
                                .iter()
                                .any(|card| card.id == *id)
                                .then_some(*id)
                        })
                        .collect::<Vec<_>>();
                    self.discard_cards_with_cause(player, &cards, cause);
                }
            }
            GameActionDef::Sacrifice { .. } | GameActionDef::SacrificeYours { .. } => {
                let yours = matches!(action.unnamed(), GameActionDef::SacrificeYours { .. });
                let permanents = targets
                    .iter()
                    .filter_map(|target| {
                        let Target::Permanent(id) = target else {
                            return None;
                        };
                        let controller = self.permanent_controller(*id)?;
                        ((!yours || controller == performer)
                            && self.can_be_forced_to_sacrifice(controller, performer))
                        .then_some(*id)
                    })
                    .collect::<Vec<_>>();
                self.sacrifice_permanents_then(&permanents, completion);
                return exiled;
            }
            GameActionDef::GainControl { duration, .. } => {
                self.take_control_of_targets(targets, source, duration, receiver);
            }
            GameActionDef::MoveToZone { .. }
            | GameActionDef::Choose(_)
            | GameActionDef::Sequence(_)
            | GameActionDef::Choice(_)
            | GameActionDef::Named { .. }
            | GameActionDef::Exile { .. } => {
                unreachable!("selection commits only action leaves")
            }
        }
        if let Some(completion) = completion {
            self.resume_battlefield_exit_completion(completion, &[]);
        }
        exiled
    }
}
