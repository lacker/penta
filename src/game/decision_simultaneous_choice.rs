//! Deferred choices made by several players before one ordinary effect.

use crate::card::{EffectDef, ObjectChoiceBindingDef, SimultaneousChooseDef};
use crate::{GameObjectId, PlayerId};

use super::decision_permanent_choice::effect_removes_binding;
use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility,
    EffectResolutionContext, Game, ScopedEffect, StackObject, Target,
};

pub(super) struct SimultaneousChoiceDecisionState {
    pub(super) chooser: PlayerId,
    pub(super) candidates: Vec<GameObjectId>,
    pub(super) options: Vec<DecisionOption>,
    pub(super) preference: DecisionPreference,
}

impl Game {
    pub(super) fn queue_simultaneous_choice(
        &mut self,
        definition: SimultaneousChooseDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let players = self.simultaneous_choice_players(definition, object, &context, scoped);
        self.queue_next_simultaneous_choice(scoped, 0, players, Vec::new(), object, context, false);
    }

    pub(super) fn simultaneous_choice_players(
        &self,
        definition: SimultaneousChooseDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<PlayerId> {
        let mut players = self
            .effect_recipients(definition.player, object, context, scoped)
            .into_iter()
            .filter_map(|recipient| match recipient {
                Target::Player(player) => Some(player),
                _ => None,
            })
            .collect::<Vec<_>>();
        players.sort_by_key(|player| (*player != self.active_player, player.index()));
        players.dedup();
        players
    }

    pub(super) fn simultaneous_choice_decision_state(
        &self,
        definition: SimultaneousChooseDef,
        task: usize,
        players: &[PlayerId],
        chosen: &[GameObjectId],
        object: &StackObject,
    ) -> Option<SimultaneousChoiceDecisionState> {
        let selectors = definition.one_of_each.len();
        let chooser = *players.get(task.checked_div(selectors)?)?;
        let selector = *definition.one_of_each.get(task % selectors)?;
        let source = object.source.unwrap_or(object.id);
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == chooser)
            .filter(|permanent| !chosen.contains(&permanent.card.id))
            .filter(|permanent| {
                let candidate = self.trigger_event_object(permanent);
                self.trigger_object_matches(definition.candidates, &candidate, source, false)
                    && self.trigger_object_matches(selector, &candidate, source, false)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        let preference = simultaneous_choice_preference(definition);
        let options = self.permanent_decision_options(&candidates);
        Some(SimultaneousChoiceDecisionState {
            chooser,
            candidates,
            options,
            preference,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_next_simultaneous_choice(
        &mut self,
        definition_scoped: ScopedEffect,
        mut task: usize,
        players: Vec<PlayerId>,
        mut chosen: Vec<GameObjectId>,
        object: &StackObject,
        mut context: EffectResolutionContext,
        resumed: bool,
    ) {
        let EffectDef::SimultaneousChoose(definition) = definition_scoped.effect else {
            return;
        };
        let task_count = players.len().saturating_mul(definition.one_of_each.len());
        while task < task_count {
            let Some(state) = self
                .simultaneous_choice_decision_state(definition, task, &players, &chosen, object)
            else {
                return;
            };
            if state.candidates.is_empty() {
                task += 1;
                continue;
            }
            if state.candidates.len() == 1 {
                chosen.push(state.candidates[0]);
                task += 1;
                continue;
            }
            self.queue_decision(
                state.chooser,
                "Choose a permanent",
                DecisionVisibility::Public,
                state.preference,
                1..=1,
                false,
                state.options,
                DecisionContinuation::SimultaneousChoose {
                    definition: definition_scoped,
                    task,
                    players,
                    chosen,
                    object: Box::new(object.clone()),
                    context,
                    candidates: state.candidates,
                },
            );
            return;
        }

        let source = object.source.unwrap_or(object.id);
        let candidates = self
            .battlefield
            .iter()
            .filter(|permanent| players.contains(&permanent.controller))
            .filter(|permanent| {
                self.trigger_object_matches(
                    definition.candidates,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        let unchosen = candidates
            .into_iter()
            .filter(|candidate| !chosen.contains(candidate))
            .map(Target::Permanent)
            .collect();
        context.bind_object_group(
            definition.chosen,
            chosen.into_iter().map(Target::Permanent).collect(),
        );
        context.bind_object_group(definition.unchosen, unchosen);
        let effect = definition_scoped.with_effect(*definition.then);
        if resumed {
            self.resolve_nested_effect_before_later(effect, object, context);
        } else {
            self.resolve_effect_def(effect, object, context);
        }
    }
}

fn simultaneous_choice_preference(definition: SimultaneousChooseDef) -> DecisionPreference {
    if effect_removes_binding(
        *definition.then,
        ObjectChoiceBindingDef::Objects(definition.unchosen),
    ) {
        DecisionPreference::HigherCardValue
    } else if effect_removes_binding(
        *definition.then,
        ObjectChoiceBindingDef::Objects(definition.chosen),
    ) {
        DecisionPreference::LowerCardValue
    } else {
        DecisionPreference::Neutral
    }
}
