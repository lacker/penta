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
            ObjectPredicateDef::NameEquals(name) => self
                .catalog
                .get(card.definition)
                .zip(self.effect_card_name(name, object, context, scoped))
                .is_some_and(|(definition, expected)| definition.name == expected),
            ObjectPredicateDef::NameIn(names) => {
                self.catalog.get(card.definition).is_some_and(|definition| {
                    self.effect_card_name_set(*names, object, context, scoped)
                        .contains(&definition.name)
                })
            }
            ObjectPredicateDef::ManaValueAtMostValue(value) => self.card_object_matches(
                ObjectPredicateDef::ManaValueAtMostValue(ValueDef::Constant(
                    self.effect_value(value, object, context, scoped),
                )),
                card,
                self.card_in_nonbattlefield_zone(card.id)
                    .map_or(ZoneKind::Library, |(zone, _)| zone),
                object.source.unwrap_or(object.id),
            ),
            ObjectPredicateDef::ManaValueEqualTo(value) => self.card_object_matches(
                ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(
                    self.effect_value(value, object, context, scoped),
                )),
                card,
                self.card_in_nonbattlefield_zone(card.id)
                    .map_or(ZoneKind::Library, |(zone, _)| zone),
                object.source.unwrap_or(object.id),
            ),
            _ => self.card_object_matches(
                predicate,
                card,
                self.card_in_nonbattlefield_zone(card.id)
                    .map_or(ZoneKind::Library, |(zone, _)| zone),
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
        let nested = |predicate| {
            self.effect_collection_target_matches(predicate, target, object, context, scoped)
        };
        match predicate {
            ObjectPredicateDef::All(predicates) => {
                return predicates.iter().copied().all(nested);
            }
            ObjectPredicateDef::AnyOf(predicates) => {
                return predicates.iter().copied().any(nested);
            }
            ObjectPredicateDef::Not(predicate) => return !nested(*predicate),
            ObjectPredicateDef::NameEquals(name) => {
                return Self::target_object_id(target)
                    .and_then(|id| self.object_card_name(id))
                    .zip(self.effect_card_name(name, object, context, scoped))
                    .is_some_and(|(actual, expected)| actual == expected);
            }
            ObjectPredicateDef::NameIn(names) => {
                return Self::target_object_id(target)
                    .and_then(|id| self.object_card_name(id))
                    .is_some_and(|actual| {
                        self.effect_card_name_set(*names, object, context, scoped)
                            .contains(actual.as_ref())
                    });
            }
            ObjectPredicateDef::SharesColorWith(colors) => {
                let expected = self.color_set_value(colors, |reference| {
                    self.effect_object_reference_id(reference, object, context, scoped)
                });
                return Self::target_object_id(target).is_some_and(|id| {
                    !crate::card::ColorSet::from_flags(self.object_colors(id))
                        .intersection(expected)
                        .is_colorless()
                });
            }
            _ => {}
        }
        let value = |value| ValueDef::Constant(self.effect_value(value, object, context, scoped));
        let predicate = match predicate {
            ObjectPredicateDef::ManaValueEqualTo(amount) => {
                ObjectPredicateDef::ManaValueEqualTo(value(amount))
            }
            ObjectPredicateDef::ManaValueAtMostValue(amount) => {
                ObjectPredicateDef::ManaValueAtMostValue(value(amount))
            }
            ObjectPredicateDef::PowerLessThan(amount) => {
                ObjectPredicateDef::PowerLessThan(value(amount))
            }
            ObjectPredicateDef::PowerGreaterThan(amount) => {
                ObjectPredicateDef::PowerGreaterThan(value(amount))
            }
            ObjectPredicateDef::ToughnessLessThan(amount) => {
                ObjectPredicateDef::ToughnessLessThan(value(amount))
            }
            ObjectPredicateDef::ToughnessGreaterThan(amount) => {
                ObjectPredicateDef::ToughnessGreaterThan(value(amount))
            }
            predicate => predicate,
        };
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
            EffectDef::SearchZones {
                searcher,
                owner,
                zones,
                binding,
                then,
            } => {
                let Some(searcher) =
                    self.effect_player_reference(searcher, object, &context, scoped)
                else {
                    return;
                };
                let Some(owner) = self.effect_player_reference(owner, object, &context, scoped)
                else {
                    return;
                };
                if zones.contains(&ZoneKind::Library) {
                    self.capture_battlefield_triggers(
                        &super::super::CommittedTriggerEvent::LibrarySearched {
                            player: searcher,
                            owner,
                        },
                    );
                }
                let mut cards = Vec::new();
                for zone in zones {
                    let members = match zone {
                        ZoneKind::Hand => &self.players[owner.index()].hand,
                        ZoneKind::Graveyard => &self.players[owner.index()].graveyard,
                        ZoneKind::Library => &self.players[owner.index()].library,
                        ZoneKind::Exile => &self.players[owner.index()].exile,
                        _ => continue,
                    };
                    for card in members {
                        let target = Target::Card(card.id);
                        if !cards.contains(&target) {
                            cards.push(target);
                        }
                    }
                }
                let mut context = context;
                context.bind_object_group(binding, cards);
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
            }
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
                let mut inputs = Vec::new();
                let mut events = Vec::new();
                self.entering_together(|game| {
                    for target in processing {
                        let Target::Card(card) = target else {
                            continue;
                        };
                        let Some((from, _)) = game.card_in_nonbattlefield_zone(card) else {
                            continue;
                        };
                        if definition.from.is_some_and(|expected| from != expected) {
                            continue;
                        }
                        inputs.push((
                            card,
                            game.current_or_last_known_mana_value(card).unwrap_or(0),
                        ));
                        game.move_card_target_to_zone_collecting(
                            card,
                            definition.zone,
                            cause,
                            None,
                            definition.placement,
                            &mut events,
                        );
                    }
                });
                self.capture_zone_move_events(&events);
                if definition.zone == ZoneKind::Library {
                    inputs.reverse();
                }
                // Entry replacements may still be waiting for a choice. Bind
                // actual successors only after those moves have completed.
                if !self.pending_decisions.is_empty()
                    || !self.pending_events.is_empty()
                    || !self.pending_procedures.is_empty()
                {
                    self.pending_procedures.push_back(
                        super::super::PendingProcedure::FinishMoveObjects {
                            inputs,
                            effect: scoped,
                            object: Box::new(object.clone()),
                            context,
                        },
                    );
                } else {
                    self.finish_move_objects(inputs, scoped, object, context);
                }
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

    pub(in crate::game) fn finish_move_objects(
        &mut self,
        inputs: Vec<(crate::GameObjectId, u16)>,
        scoped: ScopedEffect,
        object: &StackObject,
        mut context: EffectResolutionContext,
    ) {
        let EffectDef::MoveObjects(definition) = scoped.effect else {
            unreachable!("move completion retains its authored operation")
        };
        let mut moved = Vec::new();
        let mut consumed = Vec::new();
        let mut mana_value = 0_u16;
        for (before, value) in inputs {
            let after = self.zone_change_successor_target(before).or_else(|| {
                // Reordering within a library preserves the existing object;
                // unlike a zone change, it has no successor to bind.
                (definition.zone == ZoneKind::Library
                    && self
                        .card_in_nonbattlefield_zone(before)
                        .is_some_and(|(zone, _)| zone == ZoneKind::Library))
                .then_some(Target::Card(before))
            });
            if let Some(after) = after {
                moved.push(after);
                consumed.push(Target::Card(before));
                mana_value = mana_value.saturating_add(value);
            }
        }
        context.consume_bound_objects(&consumed);
        context.matched_count = Some(u16::try_from(moved.len()).unwrap_or(u16::MAX));
        context.matched_mana_value = Some(mana_value);
        if let Some(binding) = definition.moved {
            context.bind_object_group(binding, moved);
        }
        self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
    }
}
