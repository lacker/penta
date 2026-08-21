//! Enumerating the objects and players a target slot may legally choose.
//!
//! This is the read-only half of targeting: given a slot's predicate, which
//! things on the board satisfy it. Trigger capture asks the same predicate
//! question of a single object; this asks it of every candidate.

use super::{
    AbilityTargetPredicate, CardInstance, CardType, CharacteristicContext, Game, GameObjectId,
    ObjectPredicateDef, PlayerId, StackObjectKind, StackTargetKindDef, Target, TriggerContext,
    ZoneKind,
};

impl Game {
    /// The same, considered at a particular X. A spell being cast has no
    /// stack object yet, so a predicate that reads its chosen X -- "target
    /// creature with power X or less" -- has nothing to read it from; the
    /// enumerator already walks one X at a time, so it says which.
    pub(super) fn ability_targets_matching_at(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        x: u16,
    ) -> Vec<Target> {
        let previous = self.prospective_x.replace(Some(x));
        let targets = self.ability_targets_matching(predicate, controller, source, context);
        self.prospective_x.set(previous);
        targets
    }

    pub(super) fn ability_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        match predicate {
            AbilityTargetPredicate::AnyTarget => {
                let mut targets =
                    vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)];
                targets.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            (self.power(permanent).is_some()
                                || self
                                    .permanent_types(permanent)
                                    .is_some_and(|types| types.contains(CardType::Planeswalker)))
                                && self.permanent_can_be_targeted_by(permanent, controller, source)
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                targets
            }
            AbilityTargetPredicate::ControlledByTargetOf { .. } => Vec::new(),
            AbilityTargetPredicate::PlayerOrPlaneswalker(relation) => {
                let mut targets = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .filter(|player| {
                        self.player_relation_matches(*player, relation, controller, context)
                    })
                    .map(Target::Player)
                    .collect::<Vec<_>>();
                targets.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            self.permanent_types(permanent)
                                .is_some_and(|types| types.contains(CardType::Planeswalker))
                                && self.permanent_can_be_targeted_by(permanent, controller, source)
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                targets
            }
            AbilityTargetPredicate::Player(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(*player, relation, controller, context)
                })
                .map(Target::Player)
                .collect(),
            AbilityTargetPredicate::Object { .. } => {
                self.ability_object_targets_matching(predicate, controller, source, context)
            }
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

    pub(super) fn ability_object_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
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
                }) && self.permanent_can_be_targeted_by(permanent, controller, source)
                    && self.trigger_object_matches(object, &characteristics, source, false))
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
