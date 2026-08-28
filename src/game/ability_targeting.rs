//! Enumerating the objects and players a target slot may legally choose.
//!
//! This is the read-only half of targeting: given a slot's predicate, which
//! things on the board satisfy it. Trigger capture asks the same predicate
//! question of a single object; this asks it of every candidate.

use super::{
    AbilityTargetDef, AbilityTargetPredicate, CardInstance, CardType, CharacteristicContext, Game,
    GameObjectId, ObjectPredicateDef, PlayerId, PlayerRelation, StackObjectKind,
    StackTargetKindDef, Target, TargetSelection, TriggerContext, ZoneKind,
};

impl Game {
    pub(super) fn targets_owned_by_player_matching(
        &self,
        object: ObjectPredicateDef,
        zones: &'static [ZoneKind],
        owner: PlayerId,
        source: GameObjectId,
    ) -> Vec<Target> {
        zones
            .iter()
            .copied()
            .filter(|zone| {
                matches!(
                    zone,
                    ZoneKind::Library | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile
                )
            })
            .flat_map(|zone| {
                self.cards_in_zone(zone).filter_map(move |card| {
                    (card.owner == owner && self.card_object_matches(object, card, zone, source))
                        .then_some(Target::Card(card.id))
                })
            })
            .collect()
    }

    pub(super) fn targets_owned_by_target_player(
        &self,
        predicate: AbilityTargetPredicate,
        selections: &[TargetSelection],
        source: GameObjectId,
    ) -> Option<Vec<Target>> {
        let AbilityTargetPredicate::OwnedByTargetPlayer {
            object,
            zones,
            slot,
        } = predicate
        else {
            return None;
        };
        let owner = selections
            .iter()
            .find(|selection| selection.slot().index() == slot.index())
            .and_then(|selection| selection.targets().first())
            .and_then(|target| match target {
                Target::Player(player) => Some(*player),
                Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
            })?;
        Some(self.targets_owned_by_player_matching(object, zones, owner, source))
    }

    fn targets_controlled_by_target_of(
        &self,
        predicate: AbilityTargetPredicate,
        selections: &[TargetSelection],
        controller: PlayerId,
        source: GameObjectId,
        source_is_spell: bool,
    ) -> Option<Vec<Target>> {
        let AbilityTargetPredicate::ControlledByTargetOf {
            object,
            slot: other,
        } = predicate
        else {
            return None;
        };
        let target_controller = selections
            .iter()
            .find(|selection| selection.slot().index() == other.index())
            .and_then(|selection| selection.targets().first().copied())
            .and_then(|target| match target {
                Target::Player(player) => Some(player),
                Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                    self.current_or_last_known_controller(id)
                }
            })?;
        Some(
            self.battlefield
                .iter()
                .filter(|permanent| permanent.controller == target_controller)
                .filter(|permanent| {
                    self.trigger_object_matches(
                        object,
                        &self.trigger_event_object(permanent),
                        source,
                        false,
                    ) && self.permanent_can_be_targeted_by(
                        permanent,
                        controller,
                        source,
                        source_is_spell,
                    )
                })
                .map(|permanent| Target::Permanent(permanent.card.id))
                .collect(),
        )
    }

    fn ability_targets_matching_with_selections_for(
        &self,
        predicate: AbilityTargetPredicate,
        selections: &[TargetSelection],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        source_is_spell: bool,
    ) -> Vec<Target> {
        if let AbilityTargetPredicate::AnyOf(predicates) = predicate {
            let mut targets = Vec::new();
            for predicate in predicates {
                for target in self.ability_targets_matching_with_selections_for(
                    *predicate,
                    selections,
                    controller,
                    source,
                    context,
                    source_is_spell,
                ) {
                    if !targets.contains(&target) {
                        targets.push(target);
                    }
                }
            }
            return targets;
        }
        self.targets_controlled_by_target_of(
            predicate,
            selections,
            controller,
            source,
            source_is_spell,
        )
        .or_else(|| self.targets_owned_by_target_player(predicate, selections, source))
        .unwrap_or_else(|| {
            self.ability_targets_matching_for(
                predicate,
                controller,
                source,
                context,
                source_is_spell,
            )
        })
    }

    pub(super) fn ability_targets_matching_with_selections_at(
        &self,
        predicate: AbilityTargetPredicate,
        selections: &[TargetSelection],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        x: u16,
    ) -> Vec<Target> {
        let previous = self.prospective_x.replace(Some(x));
        let targets = self.ability_targets_matching_with_selections_for(
            predicate, selections, controller, source, context, true,
        );
        self.prospective_x.set(previous);
        targets
    }

    pub(super) fn ability_targets_matching_with_selections(
        &self,
        predicate: AbilityTargetPredicate,
        selections: &[TargetSelection],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        let source_is_spell = self
            .stack
            .iter()
            .find(|object| object.id == source)
            .is_some_and(|object| object.kind == StackObjectKind::Spell);
        self.ability_targets_matching_with_selections_for(
            predicate,
            selections,
            controller,
            source,
            context,
            source_is_spell,
        )
    }

    /// "Any other target": the ability's own source is dropped from what a
    /// slot may name. Applied where candidates are offered and where a
    /// declaration is checked, so the two agree.
    pub(super) fn without_excluded_source(
        slot: &AbilityTargetDef,
        source: GameObjectId,
        mut targets: Vec<Target>,
    ) -> Vec<Target> {
        if slot.excludes_source {
            targets.retain(|target| *target != Target::Permanent(source));
        }
        targets
    }

    #[cfg(test)]
    pub(super) fn ability_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        let source_is_spell = self
            .stack
            .iter()
            .find(|object| object.id == source)
            .is_some_and(|object| object.kind == StackObjectKind::Spell);
        self.ability_targets_matching_with_selections_for(
            predicate,
            &[],
            controller,
            source,
            context,
            source_is_spell,
        )
    }

    /// How many matching objects this player has in these zones: what they
    /// control on the battlefield, and what they own everywhere else.
    fn player_object_count(
        &self,
        player: PlayerId,
        object: ObjectPredicateDef,
        zones: &[ZoneKind],
        source: GameObjectId,
    ) -> usize {
        zones
            .iter()
            .map(|zone| match zone {
                ZoneKind::Battlefield => self
                    .battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == player)
                    .filter(|permanent| {
                        self.trigger_object_matches(
                            object,
                            &self.trigger_event_object(permanent),
                            source,
                            false,
                        )
                    })
                    .count(),
                ZoneKind::Graveyard | ZoneKind::Hand | ZoneKind::Exile | ZoneKind::Library => {
                    let state = &self.players[player.index()];
                    let cards = match zone {
                        ZoneKind::Graveyard => &state.graveyard,
                        ZoneKind::Hand => &state.hand,
                        ZoneKind::Exile => &state.exile,
                        _ => &state.library,
                    };
                    cards
                        .iter()
                        .filter(|card| self.card_object_matches(object, card, *zone, source))
                        .count()
                }
                ZoneKind::Stack | ZoneKind::Command => 0,
            })
            .sum()
    }

    /// The players a "controls more ... than they do" slot may name, which
    /// is every player in the printed relation who is ahead of the chooser
    /// on the objects the slot counts.
    fn players_with_more_objects_than(
        &self,
        predicate: AbilityTargetPredicate,
        chooser: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        source_is_spell: bool,
    ) -> Vec<Target> {
        let AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser {
            relation,
            object,
            zones,
        } = predicate
        else {
            return Vec::new();
        };
        let theirs = self.player_object_count(chooser, object, zones, source);
        [PlayerId::One, PlayerId::Two]
            .into_iter()
            .filter(|player| {
                self.player_relation_matches_for_source(*player, relation, chooser, source, context)
                    && !self.player_is_protected_from(*player, source, source_is_spell)
                    && self.player_object_count(*player, object, zones, source) > theirs
            })
            .map(Target::Player)
            .collect()
    }

    fn ability_targets_matching_for(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        source_is_spell: bool,
    ) -> Vec<Target> {
        match predicate {
            AbilityTargetPredicate::AnyOf(_) => {
                unreachable!("target alternatives are expanded before leaf matching")
            }
            AbilityTargetPredicate::AnyTarget => {
                let mut targets = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .filter(|player| {
                        !self.player_is_protected_from(*player, source, source_is_spell)
                    })
                    .map(Target::Player)
                    .collect::<Vec<_>>();
                targets.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            (self.power(permanent).is_some()
                                || self
                                    .permanent_types(permanent)
                                    .is_some_and(|types| types.contains(CardType::Planeswalker)))
                                && self.permanent_can_be_targeted_by(
                                    permanent,
                                    controller,
                                    source,
                                    source_is_spell,
                                )
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                targets
            }
            AbilityTargetPredicate::ControlledByTargetOf { .. }
            | AbilityTargetPredicate::OwnedByTargetPlayer { .. } => Vec::new(),
            AbilityTargetPredicate::PlayerOrPlaneswalker(relation) => self
                .player_or_planeswalker_targets_matching(
                    relation,
                    controller,
                    source,
                    context,
                    source_is_spell,
                ),
            AbilityTargetPredicate::Player(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches_for_source(
                        *player, relation, controller, source, context,
                    ) && !self.player_is_protected_from(*player, source, source_is_spell)
                })
                .map(Target::Player)
                .collect(),
            // "Target player who controls more creatures than they do":
            // measured against whoever is choosing, which is the player this
            // walk is already relative to.
            AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser { .. } => self
                .players_with_more_objects_than(
                    predicate,
                    controller,
                    source,
                    context,
                    source_is_spell,
                ),
            AbilityTargetPredicate::Object { .. } => self.ability_object_targets_matching(
                predicate,
                controller,
                source,
                context,
                source_is_spell,
            ),
            // Spells and abilities alike, which is the whole difference from
            // the stack-zone object slot above.
            AbilityTargetPredicate::StackObject {
                object,
                controller: controller_relation,
                kind,
            } => self
                .stack
                .iter()
                .filter_map(|stack_object| {
                    if kind == StackTargetKindDef::AbilityOnly
                        && stack_object.kind == StackObjectKind::Spell
                    {
                        return None;
                    }
                    let characteristics = self.stack_object_event_object(stack_object)?;
                    (controller_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.controller,
                            relation,
                            controller,
                            context,
                        )
                    }) && self.trigger_object_matches_for_controller(
                        object,
                        &characteristics,
                        source,
                        true,
                        // The player choosing targets, which is who "a land
                        // you control" is measured from. A spell still in
                        // hand has no controller to derive it from.
                        Some(controller),
                    ))
                    .then_some(Target::Spell(stack_object.id))
                })
                .collect(),
        }
    }

    fn player_or_planeswalker_targets_matching(
        &self,
        relation: PlayerRelation,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        source_is_spell: bool,
    ) -> Vec<Target> {
        let mut targets = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .filter(|player| {
                self.player_relation_matches_for_source(
                    *player, relation, controller, source, context,
                ) && !self.player_is_protected_from(*player, source, source_is_spell)
            })
            .map(Target::Player)
            .collect::<Vec<_>>();
        targets.extend(
            self.battlefield
                .iter()
                .filter(|permanent| {
                    self.permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Planeswalker))
                        && self.player_relation_matches_for_source(
                            permanent.controller,
                            relation,
                            controller,
                            source,
                            context,
                        )
                        && self.permanent_can_be_targeted_by(
                            permanent,
                            controller,
                            source,
                            source_is_spell,
                        )
                })
                .map(|permanent| Target::Permanent(permanent.card.id)),
        );
        targets
    }

    pub(super) fn ability_object_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        source_is_spell: bool,
    ) -> Vec<Target> {
        let AbilityTargetPredicate::Object {
            object,
            zones,
            controller: controller_relation,
            owner: owner_relation,
        } = predicate
        else {
            unreachable!("object-target matching requires an object predicate")
        };
        let mut targets = Vec::new();
        if zones.contains(&ZoneKind::Battlefield) {
            targets.extend(self.battlefield.iter().filter_map(|permanent| {
                let characteristics = self.targeting_event_object(permanent);
                (controller_relation.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.controller,
                        relation,
                        controller,
                        context,
                    )
                }) && owner_relation.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.card.owner,
                        relation,
                        controller,
                        context,
                    )
                }) && self.permanent_can_be_targeted_by(
                    permanent,
                    controller,
                    source,
                    source_is_spell,
                ) && self.trigger_object_matches(object, &characteristics, source, false))
                .then_some(Target::Permanent(permanent.card.id))
            }));
        }
        if zones.contains(&ZoneKind::Stack) {
            targets.extend(self.stack.iter().filter_map(|stack_object| {
                let characteristics = self.stack_trigger_event_object(stack_object)?;
                (stack_object.kind == StackObjectKind::Spell
                    && controller_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.controller,
                            relation,
                            controller,
                            context,
                        )
                    })
                    && owner_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.card.owner,
                            relation,
                            controller,
                            context,
                        )
                    })
                    && self.trigger_object_matches(object, &characteristics, source, true))
                .then_some(Target::Spell(stack_object.id))
            }));
        }
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            if !zones.contains(&zone) || controller_relation.is_some() {
                continue;
            }
            targets.extend(self.cards_in_zone(zone).filter_map(|card| {
                (owner_relation.is_none_or(|relation| {
                    self.player_relation_matches(card.owner, relation, controller, context)
                }) && self.card_object_matches(object, card, zone, source))
                .then_some(Target::Card(card.id))
            }));
        }
        targets
    }

    pub(super) fn cards_in_zone(&self, zone: ZoneKind) -> impl Iterator<Item = &CardInstance> {
        self.players.iter().flat_map(move |player| match zone {
            ZoneKind::Library => player.library.iter(),
            ZoneKind::Hand => player.hand.iter(),
            ZoneKind::Graveyard => player.graveyard.iter(),
            ZoneKind::Exile => player.exile.iter(),
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => [].iter(),
        })
    }

    pub(super) fn card_in_nonbattlefield_zone(
        &self,
        id: GameObjectId,
    ) -> Option<(ZoneKind, &CardInstance)> {
        [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
        ]
        .into_iter()
        .find_map(|zone| {
            self.cards_in_zone(zone)
                .find(|card| card.id == id)
                .map(|card| (zone, card))
        })
    }

    /// The same, for the clauses that have to change the card rather than
    /// read it: a counter put on a card that is not on the battlefield.
    pub(super) fn card_in_nonbattlefield_zone_mut(
        &mut self,
        id: GameObjectId,
    ) -> Option<&mut CardInstance> {
        let located = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find_map(|player| {
                let state = &self.players[player.index()];
                [
                    ZoneKind::Library,
                    ZoneKind::Hand,
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                ]
                .into_iter()
                .find_map(|zone| {
                    let cards = match zone {
                        ZoneKind::Library => &state.library,
                        ZoneKind::Hand => &state.hand,
                        ZoneKind::Graveyard => &state.graveyard,
                        _ => &state.exile,
                    };
                    cards
                        .iter()
                        .position(|card| card.id == id)
                        .map(|index| (player, zone, index))
                })
            })?;
        let (player, zone, index) = located;
        let state = &mut self.players[player.index()];
        let cards = match zone {
            ZoneKind::Library => &mut state.library,
            ZoneKind::Hand => &mut state.hand,
            ZoneKind::Graveyard => &mut state.graveyard,
            _ => &mut state.exile,
        };
        cards.get_mut(index)
    }

    pub(super) fn card_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        card: &CardInstance,
        zone: ZoneKind,
        source: GameObjectId,
    ) -> bool {
        if self
            .catalog
            .get(card.definition)
            .is_some_and(|definition| definition.rules.has_metadata_only_creature_body())
        {
            // A catalog-only creature still exposes exact printed metadata to
            // catalog consumers, but no gameplay effect may select it and
            // turn that metadata into an executable vanilla permanent.
            return false;
        }
        // Reveal-until removes the prospective card from the library before
        // asking whether it is the stopping card. Name predicates therefore
        // have to read the card being considered rather than rediscover its
        // object ID in a zone. Recurse here as well so a named predicate
        // remains correct when composed with another card characteristic.
        match predicate {
            ObjectPredicateDef::HasAnyCounter => {
                return !card.counters.is_empty();
            }
            ObjectPredicateDef::Named(name) => {
                return self
                    .catalog
                    .get(card.definition)
                    .is_some_and(|definition| definition.name == name);
            }
            ObjectPredicateDef::All(predicates) => {
                return predicates
                    .iter()
                    .all(|predicate| self.card_object_matches(*predicate, card, zone, source));
            }
            ObjectPredicateDef::AnyOf(predicates) => {
                return predicates
                    .iter()
                    .any(|predicate| self.card_object_matches(*predicate, card, zone, source));
            }
            ObjectPredicateDef::Not(predicate) => {
                return !self.card_object_matches(*predicate, card, zone, source);
            }
            _ => {}
        }
        let context = match zone {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return false,
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, card.owner, &context)
        else {
            return false;
        };
        self.trigger_object_matches(predicate, &object, source, false)
    }
}
