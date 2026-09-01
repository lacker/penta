//! Deferred choices made by several players before one ordinary effect.

use crate::card::{
    ChooseForEachPlayerDef, EffectDef, ObjectChoiceBindingDef, PerPlayerSelectionDef, ZoneKind,
};
use crate::{GameObjectId, PlayerId};

use super::decision_permanent_choice::effect_removes_binding;
use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    EffectResolutionContext, Game, ScopedEffect, StackObject, Target,
};

pub(super) struct PerPlayerChoiceDecisionState {
    pub(super) chooser: PlayerId,
    pub(super) candidates: Vec<GameObjectId>,
    pub(super) options: Vec<DecisionOption>,
    pub(super) preference: DecisionPreference,
    pub(super) prompt: &'static str,
    pub(super) visibility: DecisionVisibility,
    pub(super) count: usize,
}

impl Game {
    pub(super) fn queue_choices_for_each_player(
        &mut self,
        definition: ChooseForEachPlayerDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let players = self.choice_players_apnap(definition, object, &context, scoped);
        self.queue_next_player_choice(scoped, 0, players, Vec::new(), object, context, false);
    }

    pub(super) fn choice_players_apnap(
        &self,
        definition: ChooseForEachPlayerDef,
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

    pub(super) fn per_player_choice_decision_state(
        &self,
        task: usize,
        players: &[PlayerId],
        chosen: &[GameObjectId],
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<PerPlayerChoiceDecisionState> {
        let EffectDef::ChooseForEachPlayer(definition) = scoped.effect else {
            return None;
        };
        let (chooser, selector, count, prompt) = match definition.selection {
            PerPlayerSelectionDef::OneOfEach(selectors) => {
                let chooser = *players.get(task.checked_div(selectors.len())?)?;
                let selector = *selectors.get(task % selectors.len())?;
                (chooser, Some(selector), 1, "Choose a permanent")
            }
            PerPlayerSelectionDef::Count(amount) => {
                let chooser = *players.get(task)?;
                let count =
                    usize::try_from(self.effect_value(amount, object, context, scoped).max(0))
                        .unwrap_or(usize::MAX);
                (chooser, None, count, "Choose objects to keep")
            }
        };
        let candidates = self
            .per_player_choice_candidates(definition, chooser, object)
            .into_iter()
            .filter(|candidate| !chosen.contains(candidate))
            .filter(|candidate| {
                selector.is_none_or(|selector| {
                    let source = object.source.unwrap_or(object.id);
                    self.per_player_choice_candidate_matches(
                        definition.zone,
                        *candidate,
                        selector,
                        source,
                    )
                })
            })
            .collect::<Vec<_>>();
        let preference = per_player_choice_preference(definition);
        let options = match definition.zone {
            ZoneKind::Battlefield => self.permanent_decision_options(&candidates),
            ZoneKind::Hand => self.card_decision_options(
                &self.players[chooser.index()]
                    .hand
                    .iter()
                    .filter(|card| candidates.contains(&card.id))
                    .cloned()
                    .collect::<Vec<_>>(),
                DecisionZone::Hand,
            ),
            _ => Vec::new(),
        };
        Some(PerPlayerChoiceDecisionState {
            chooser,
            candidates,
            options,
            preference,
            prompt,
            visibility: match definition.visibility {
                crate::card::ChoiceVisibilityDef::Public => DecisionVisibility::Public,
                crate::card::ChoiceVisibilityDef::Private => DecisionVisibility::Private,
            },
            count,
        })
    }

    fn per_player_choice_candidates(
        &self,
        definition: ChooseForEachPlayerDef,
        player: PlayerId,
        object: &StackObject,
    ) -> Vec<GameObjectId> {
        let source = object.source.unwrap_or(object.id);
        match definition.zone {
            ZoneKind::Battlefield => self
                .battlefield
                .iter()
                .filter(|permanent| permanent.controller == player)
                .filter(|permanent| {
                    self.trigger_object_matches(
                        definition.candidates,
                        &self.trigger_event_object(permanent),
                        source,
                        false,
                    )
                })
                .map(|permanent| permanent.card.id)
                .collect(),
            ZoneKind::Hand => self.players[player.index()]
                .hand
                .iter()
                .filter(|card| {
                    self.card_object_matches(definition.candidates, card, ZoneKind::Hand, source)
                })
                .map(|card| card.id)
                .collect(),
            _ => Vec::new(),
        }
    }

    fn per_player_choice_candidate_matches(
        &self,
        zone: ZoneKind,
        candidate: GameObjectId,
        predicate: crate::card::ObjectPredicateDef,
        source: GameObjectId,
    ) -> bool {
        match zone {
            ZoneKind::Battlefield => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == candidate)
                .is_some_and(|permanent| {
                    self.trigger_object_matches(
                        predicate,
                        &self.trigger_event_object(permanent),
                        source,
                        false,
                    )
                }),
            ZoneKind::Hand => {
                self.card_in_nonbattlefield_zone(candidate)
                    .is_some_and(|(actual, card)| {
                        actual == ZoneKind::Hand
                            && self.card_object_matches(predicate, card, actual, source)
                    })
            }
            _ => false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_next_player_choice(
        &mut self,
        definition_scoped: ScopedEffect,
        mut task: usize,
        players: Vec<PlayerId>,
        mut chosen: Vec<GameObjectId>,
        object: &StackObject,
        mut context: EffectResolutionContext,
        resumed: bool,
    ) {
        let EffectDef::ChooseForEachPlayer(definition) = definition_scoped.effect else {
            return;
        };
        let task_count = match definition.selection {
            PerPlayerSelectionDef::OneOfEach(selectors) => {
                players.len().saturating_mul(selectors.len())
            }
            PerPlayerSelectionDef::Count(_) => players.len(),
        };
        while task < task_count {
            let Some(state) = self.per_player_choice_decision_state(
                task,
                &players,
                &chosen,
                object,
                &context,
                definition_scoped,
            ) else {
                return;
            };
            if state.count == 0 {
                task += 1;
                continue;
            }
            if state.candidates.len() <= state.count {
                chosen.extend(state.candidates);
                task += 1;
                continue;
            }
            self.queue_decision(
                state.chooser,
                state.prompt,
                state.visibility,
                state.preference,
                state.count..=state.count,
                false,
                state.options,
                DecisionContinuation::ChooseForEachPlayer {
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

        let candidates = players
            .iter()
            .flat_map(|player| self.per_player_choice_candidates(definition, *player, object))
            .collect::<Vec<_>>();
        let target = |candidate| match definition.zone {
            ZoneKind::Battlefield => Target::Permanent(candidate),
            _ => Target::Card(candidate),
        };
        let unchosen = candidates
            .into_iter()
            .filter(|candidate| !chosen.contains(candidate))
            .map(target)
            .collect();
        context.bind_object_group(definition.chosen, chosen.into_iter().map(target).collect());
        context.bind_object_group(definition.unchosen, unchosen);
        let effect = definition_scoped.with_effect(*definition.then);
        if resumed {
            self.resolve_nested_effect_before_later(effect, object, context);
        } else {
            self.resolve_effect_def(effect, object, context);
        }
    }
}

fn per_player_choice_preference(definition: ChooseForEachPlayerDef) -> DecisionPreference {
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
