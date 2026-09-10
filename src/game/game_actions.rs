//! Shared action execution. Payment planning and ordinary resolution enter
//! these same operations after applying their own selection requirements.
pub(super) mod payments;

use super::{
    EffectResolutionContext, Game, GameObjectId, PlayerId, ScopedEffect, StackObject, Target,
    ZoneMoveCause,
};
use crate::card::{
    ChooseDef, EffectDef, GameActionChoiceDef, GameActionDef, ObjectChoiceBindingDef,
};

impl Game {
    pub(super) fn resolve_game_action(
        &mut self,
        action: GameActionDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        match action {
            GameActionDef::Sequence(actions) => self.resolve_effects_in_order(
                actions
                    .iter()
                    .map(|action| scoped.with_effect(EffectDef::Perform(*action)))
                    .collect(),
                object,
                context,
            ),
            GameActionDef::Choose(choice) => {
                let definition = self.fixed_game_action_choice(choice, object, &context, scoped);
                self.queue_effect_choice_with_continuation(
                    definition,
                    EffectDef::Perform(*choice.then),
                    object,
                    context,
                    scoped,
                );
            }
            action => {
                let (GameActionDef::DiscardCards { object: recipient }
                | GameActionDef::Sacrifice { object: recipient }
                | GameActionDef::SacrificeYours { object: recipient }
                | GameActionDef::GainControl {
                    object: recipient, ..
                }) = action
                else {
                    unreachable!("composite actions were handled above")
                };
                let receiver = if let GameActionDef::GainControl { controller, .. } = action {
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
        match action {
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
                let yours = matches!(action, GameActionDef::SacrificeYours { .. });
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
                self.sacrifice_permanents(&permanents);
            }
            GameActionDef::GainControl { duration, .. } => {
                self.take_control_of_targets(targets, source, duration, receiver);
            }
            GameActionDef::Choose(_) | GameActionDef::Sequence(_) => {
                unreachable!("selection commits only action leaves")
            }
        }
    }
}
