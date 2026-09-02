//! Immediate stages in a resumable object-collection workflow.

use super::super::{
    BattlefieldArrival, CardInstance, EffectDef, EffectResolutionContext, Game, GameEvent,
    PlayerId, ScopedEffect, StackObject, Target, ZoneKind, ZoneMoveCause,
};
use crate::card::{ObjectCollectionSourceDef, ObjectPredicateDef, ValueDef};

impl Game {
    fn effect_collection_card_matches(
        &self,
        predicate: ObjectPredicateDef,
        card: &CardInstance,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> bool {
        let nested = |predicate| {
            self.effect_collection_card_matches(predicate, card, object, context, scoped)
        };
        match predicate {
            ObjectPredicateDef::All(predicates) => predicates.iter().copied().all(nested),
            ObjectPredicateDef::AnyOf(predicates) => predicates.iter().copied().any(nested),
            ObjectPredicateDef::Not(predicate) => !nested(*predicate),
            ObjectPredicateDef::HasChosenName => context.chosen_name.as_ref().is_some_and(|name| {
                self.catalog
                    .get(card.definition)
                    .is_some_and(|definition| definition.name == *name)
            }),
            ObjectPredicateDef::ManaValueAtMostValue(value) => self.card_object_matches(
                ObjectPredicateDef::ManaValueAtMostValue(ValueDef::Constant(
                    self.effect_value(value, object, context, scoped),
                )),
                card,
                ZoneKind::Library,
                object.source.unwrap_or(object.id),
            ),
            ObjectPredicateDef::ManaValueEqualTo(value) => self.card_object_matches(
                ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(
                    self.effect_value(value, object, context, scoped),
                )),
                card,
                ZoneKind::Library,
                object.source.unwrap_or(object.id),
            ),
            _ => self.card_object_matches(
                predicate,
                card,
                ZoneKind::Library,
                object.source.unwrap_or(object.id),
            ),
        }
    }

    pub(in crate::game) fn effect_collection_target_matches(
        &self,
        predicate: ObjectPredicateDef,
        target: Target,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> bool {
        let Target::Card(card) = target else {
            return self.bound_object_matches(
                target,
                predicate,
                object.source.unwrap_or(object.id),
            );
        };
        self.card_in_nonbattlefield_zone(card)
            .is_some_and(|(_, card)| {
                self.effect_collection_card_matches(predicate, card, object, context, scoped)
            })
    }

    pub(in crate::game) fn reveal_effect_collection(&mut self, targets: &[Target]) {
        let revealed = targets
            .iter()
            .filter_map(|target| match target {
                Target::Card(id) => self
                    .card_in_nonbattlefield_zone(*id)
                    .map(|(_, card)| (card.owner, card.id, card.definition)),
                Target::Player(_) | Target::Permanent(_) | Target::Spell(_) => None,
            })
            .collect::<Vec<_>>();
        self.events
            .extend(revealed.into_iter().map(|(player, card, definition)| {
                GameEvent::CardRevealed {
                    player,
                    card,
                    definition,
                }
            }));
    }

    pub(in crate::game) fn top_library_card_targets(
        &self,
        player: PlayerId,
        count: usize,
    ) -> Vec<Target> {
        self.players[player.index()]
            .library
            .iter()
            .rev()
            .take(count)
            .map(|card| Target::Card(card.id))
            .collect()
    }

    /// Materialize a collection source without revealing or moving anything.
    /// Library-backed sources are returned in top-first order so every later
    /// stage sees the same authored ordering semantics.
    pub(in crate::game) fn effect_object_collection(
        &self,
        source: ObjectCollectionSourceDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<Vec<Target>> {
        match source {
            ObjectCollectionSourceDef::ObjectSet(input) => {
                Some(self.effect_objects(input, object, context, scoped))
            }
            ObjectCollectionSourceDef::TopCards { player, count } => {
                let player = self.effect_player_reference(player, object, context, scoped)?;
                let count = self
                    .effect_value(count, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(usize::MAX);
                Some(self.top_library_card_targets(player, count))
            }
            ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
                player,
                object: predicate,
            } => {
                let player = self.effect_player_reference(player, object, context, scoped)?;
                let mut cards = Vec::new();
                for card in self.players[player.index()].library.iter().rev() {
                    cards.push(Target::Card(card.id));
                    if self.effect_collection_card_matches(predicate, card, object, context, scoped)
                    {
                        break;
                    }
                }
                Some(cards)
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_object_collection_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::BindObjects(definition) => {
                let Some(cards) =
                    self.effect_object_collection(definition.source, object, &context, scoped)
                else {
                    return;
                };
                let mut context = context;
                context.bind_object_group(definition.binding, cards);
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::IfNoObjects(definition) => {
                let branch = if self
                    .effect_objects(definition.input, object, &context, scoped)
                    .is_empty()
                {
                    definition.if_empty
                } else {
                    definition.otherwise
                };
                self.resolve_effect_def(scoped.with_effect(*branch), object, context);
            }
            EffectDef::ClassifyObjects(definition) => {
                let input = self.effect_objects(definition.input, object, &context, scoped);
                let (matching, remainder) = input.into_iter().partition(|target| {
                    self.effect_collection_target_matches(
                        definition.object,
                        *target,
                        object,
                        &context,
                        scoped,
                    )
                });
                let mut context = context;
                context.bind_object_group(definition.matching, matching);
                context.bind_object_group(definition.remainder, remainder);
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::RevealAndClassifyCards(definition) => {
                let Some(input) =
                    self.effect_object_collection(definition.source, object, &context, scoped)
                else {
                    return;
                };
                self.reveal_effect_collection(&input);
                let (matching, remainder) = input.into_iter().partition(|target| {
                    self.effect_collection_target_matches(
                        definition.object,
                        *target,
                        object,
                        &context,
                        scoped,
                    )
                });
                let mut context = context;
                context.bind_object_group(definition.matching, matching);
                context.bind_object_group(definition.remainder, remainder);
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::CombineObjects(definition) => {
                let combined = definition
                    .inputs
                    .iter()
                    .flat_map(|input| self.effect_objects(*input, object, &context, scoped))
                    .collect();
                let mut context = context;
                context.bind_object_group(definition.combined, combined);
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::RandomizeObjectOrder(definition) => {
                let mut randomized =
                    self.effect_objects(definition.input, object, &context, scoped);
                self.rng.shuffle(&mut randomized);
                let mut context = context;
                context.bind_object_group(definition.randomized, randomized);
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::RevealObjects(definition) => {
                let targets = self.effect_objects(definition.input, object, &context, scoped);
                self.reveal_effect_collection(&targets);
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::MoveObjects(definition) => {
                let input = self.effect_objects(definition.input, object, &context, scoped);
                let mut processing = input.clone();
                if definition.zone == ZoneKind::Library {
                    processing.reverse();
                }
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                let mut moved = Vec::new();
                for target in processing {
                    let Target::Card(card) = target else {
                        continue;
                    };
                    if definition.from.is_some_and(|expected| {
                        self.card_in_nonbattlefield_zone(card)
                            .is_none_or(|(actual, _)| actual != expected)
                    }) {
                        continue;
                    }
                    let mana_value = self.current_or_last_known_mana_value(card).unwrap_or(0);
                    let Some((created, destination)) = self.move_card_target_to_zone(
                        card,
                        definition.zone,
                        cause,
                        None,
                        definition.placement,
                    ) else {
                        continue;
                    };
                    let created = if destination == ZoneKind::Battlefield {
                        Target::Permanent(self.arrived.take().unwrap_or(created))
                    } else {
                        Target::Card(created)
                    };
                    moved.push((target, created, mana_value));
                }
                let moved_objects = input
                    .iter()
                    .filter_map(|input| {
                        moved
                            .iter()
                            .find(|(previous, _, _)| previous == input)
                            .map(|(_, created, _)| *created)
                    })
                    .collect::<Vec<_>>();
                let consumed = moved
                    .iter()
                    .map(|(previous, _, _)| *previous)
                    .collect::<Vec<_>>();
                let mut context = context;
                context.consume_bound_objects(&consumed);
                context.matched_count =
                    Some(u16::try_from(moved_objects.len()).unwrap_or(u16::MAX));
                context.matched_mana_value = Some(
                    moved
                        .iter()
                        .fold(0_u16, |total, (_, _, value)| total.saturating_add(*value)),
                );
                if let Some(binding) = definition.moved {
                    context.bind_object_group(binding, moved_objects);
                }
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => {
                let Some(controller) =
                    self.effect_player_reference(definition.controller, object, &context, scoped)
                else {
                    return;
                };
                let input = self.effect_objects(definition.input, object, &context, scoped);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                let mut moved = Vec::new();
                for target in &input {
                    let Target::Card(card) = target else {
                        continue;
                    };
                    let Some(from) = self
                        .card_in_nonbattlefield_zone(*card)
                        .map(|(zone, _)| zone)
                    else {
                        continue;
                    };
                    let arrival = BattlefieldArrival::face_down_under(
                        controller,
                        definition.characteristics,
                        definition.turn_up_for_mana_cost,
                    );
                    let Some((created, destination)) = self.move_card_from_nonbattlefield_zone(
                        *card,
                        from,
                        ZoneKind::Battlefield,
                        cause,
                        Some(arrival),
                    ) else {
                        continue;
                    };
                    let created = if destination == ZoneKind::Battlefield {
                        Target::Permanent(self.arrived.take().unwrap_or(created.id))
                    } else {
                        Target::Card(created.id)
                    };
                    moved.push((*target, created));
                }
                let moved_objects = input
                    .iter()
                    .filter_map(|input| {
                        moved
                            .iter()
                            .find(|(previous, _)| previous == input)
                            .map(|(_, created)| *created)
                    })
                    .collect::<Vec<_>>();
                let consumed = moved
                    .iter()
                    .map(|(previous, _)| *previous)
                    .collect::<Vec<_>>();
                let mut context = context;
                context.consume_bound_objects(&consumed);
                context.matched_count =
                    Some(u16::try_from(moved_objects.len()).unwrap_or(u16::MAX));
                if let Some(binding) = definition.moved {
                    context.bind_object_group(binding, moved_objects);
                }
                self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            }
            _ => unreachable!("only immediate collection effects reach this resolver"),
        }
    }
}
