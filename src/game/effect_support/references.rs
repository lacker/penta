// Resolving the object and player references an effect names, from the
// resolving object's own context: target slots, bindings, player sets, and
// the recipients and object sets those add up to.
//
// Split out of `effect_support.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

impl Game {
    fn raw_target_reference(
        slot: TargetIndex,
        object: &StackObject,
        scoped: ScopedEffect,
    ) -> Option<Target> {
        Self::chosen_targets(object, scoped.target_slot(slot)).next()
    }

    pub(in crate::game) fn object_reference_target(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<Target> {
        match reference {
            // An ability activated from the graveyard or from hand has a
            // card, not a permanent, as its source, and "return this card to
            // your hand" has to name it as one. A source that is on the
            // battlefield, or that has left every zone, still answers as a
            // permanent: that is the last-known information every
            // "sacrifice this" clause reads after the thing is already gone.
            ObjectRefDef::Source => object.source.map(|source| {
                if self
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.card.id == source)
                {
                    return Target::Permanent(source);
                }
                if self.card_in_nonbattlefield_zone(source).is_some() {
                    return Target::Card(source);
                }
                // A dies-trigger names the permanent that was standing there,
                // and the card it became on the way out has a different
                // identity. "Return it to the battlefield" means that card,
                // so follow the move rather than pointing at a permanent
                // nothing can find.
                match self
                    .successors
                    .get(&source)
                    .copied()
                    .filter(|successor| self.card_in_nonbattlefield_zone(*successor).is_some())
                {
                    Some(successor) => Target::Card(successor),
                    None => Target::Permanent(source),
                }
            }),
            // A granted ability freezes the exact object that supplied the
            // grant. Do not follow a zone-change successor here: this is the
            // last-known permanent the ability names even after sacrificing
            // it as a cost.
            ObjectRefDef::AbilityGrantSource => {
                object.ability_origin().and_then(|origin| match origin {
                    crate::AbilityOrigin::Granted { source, .. }
                    | crate::AbilityOrigin::TokenGranted { source, .. }
                    | crate::AbilityOrigin::EmblemGranted { source, .. } => {
                        Some(Target::Permanent(source))
                    }
                    _ => None,
                })
            }
            ObjectRefDef::ResolvingObject => self.live_object_target(object.id),
            ObjectRefDef::SourceOfTargetedStackObject(target) => self
                .targeted_stack_object_source(target, object, scoped)
                .map(Target::Permanent),
            ObjectRefDef::Binding(binding) => context.single_object(binding),
            ObjectRefDef::AttachedToSource => object
                .source
                .and_then(|source| self.current_or_last_known_attached_host(source))
                .map(Target::Permanent),
            ObjectRefDef::Target(target) => {
                let slot = scoped.target_slot(target);
                Self::raw_target_reference(target, object, scoped)
                    .filter(|target| !matches!(target, Target::Player(_)))
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
            }
            ObjectRefDef::TriggeringObject => context
                .trigger
                .object
                .and_then(|triggering| self.live_object_target(triggering)),
        }
    }

    fn object_reference_id(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source => object.source,
            ObjectRefDef::AbilityGrantSource => {
                object.ability_origin().and_then(|origin| match origin {
                    crate::AbilityOrigin::Granted { source, .. }
                    | crate::AbilityOrigin::TokenGranted { source, .. }
                    | crate::AbilityOrigin::EmblemGranted { source, .. } => Some(source),
                    _ => None,
                })
            }
            ObjectRefDef::ResolvingObject => Some(object.id),
            ObjectRefDef::Binding(binding) => {
                context
                    .single_object(binding)
                    .and_then(|target| match target {
                        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
                        Target::Player(_) => None,
                    })
            }
            ObjectRefDef::AttachedToSource => object
                .source
                .and_then(|source| self.current_or_last_known_attached_host(source)),
            ObjectRefDef::Target(target) => {
                let slot = scoped.target_slot(target);
                Self::raw_target_reference(target, object, scoped)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .and_then(|target| match target {
                        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
                        Target::Player(_) => None,
                    })
            }
            ObjectRefDef::SourceOfTargetedStackObject(target) => {
                self.targeted_stack_object_source(target, object, scoped)
            }
            ObjectRefDef::TriggeringObject => context.trigger.object,
        }
    }

    /// The permanent a targeted stack ability came from. Read after the
    /// ability has left the stack -- which is when "destroy that permanent"
    /// asks -- so the retired record is the one that answers, and a targeted
    /// spell has no such source at all.
    fn targeted_stack_object_source(
        &self,
        target: crate::TargetIndex,
        object: &StackObject,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        let Some(Target::Spell(id)) = Self::raw_target_reference(target, object, scoped) else {
            return None;
        };
        let source = self
            .stack
            .iter()
            .find(|candidate| candidate.id == id)
            .map_or_else(
                || self.retired_stack_object_source(id),
                |stack| stack.source,
            )?;
        self.battlefield
            .iter()
            .any(|permanent| permanent.card.id == source)
            .then_some(source)
    }

    pub(in crate::game) fn player_reference(
        &self,
        reference: PlayerRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<PlayerId> {
        match reference {
            PlayerRefDef::EffectController => Some(object.controller),
            PlayerRefDef::Opponent => Some(object.controller.opponent()),
            PlayerRefDef::EventPlayer => context.trigger.event_player,
            PlayerRefDef::Target(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .find(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .and_then(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
            }
            // A direct object recipient still checks whether its target is
            // legal. Derived identity is different: a later instruction in
            // the same resolving effect may ask who controlled or owned an
            // object that an earlier instruction already moved. Preserve the
            // announced target here and answer from last-known information.
            PlayerRefDef::ControllerOf(ObjectRefDef::Target(target)) => {
                Self::raw_target_reference(target, object, scoped).and_then(|target| match target {
                    Target::Player(player) => Some(player),
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                        self.current_or_last_known_controller(id)
                    }
                })
            }
            PlayerRefDef::OwnerOf(ObjectRefDef::Target(target)) => {
                Self::raw_target_reference(target, object, scoped).and_then(|target| match target {
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                        self.current_or_last_known_owner(id)
                    }
                    Target::Player(_) => None,
                })
            }
            PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject) => context
                .trigger
                .object
                .and_then(|triggering| self.current_or_last_known_controller(triggering))
                .or(context.trigger.object_controller),
            PlayerRefDef::ControllerOf(reference) => self
                .object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.current_or_last_known_controller(referenced)),
            PlayerRefDef::OwnerOf(reference) => self
                .object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.current_or_last_known_owner(referenced)),
        }
    }

    fn players_in_set(
        &self,
        players: PlayerSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<PlayerId> {
        match players {
            PlayerSetDef::All => vec![object.controller, object.controller.opponent()],
            PlayerSetDef::One(reference) => self
                .player_reference(reference, object, context, scoped)
                .into_iter()
                .collect(),
            PlayerSetDef::LegalTargets(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect()
            }
            PlayerSetDef::Related(relation) => [object.controller, object.controller.opponent()]
                .into_iter()
                .filter(|candidate| {
                    self.player_relation_matches(
                        *candidate,
                        relation,
                        object.controller,
                        context.trigger,
                    )
                })
                .collect(),
        }
    }

    pub(super) fn effect_object_reference_id(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        self.object_reference_id(reference, object, context, scoped)
    }

    pub(super) fn effect_player_reference(
        &self,
        reference: PlayerRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<PlayerId> {
        self.player_reference(reference, object, context, scoped)
    }

    pub(super) fn effect_players(
        &self,
        players: PlayerSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<PlayerId> {
        self.players_in_set(players, object, context, scoped)
    }

    fn objects_sharing_name_with_reference(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        if let ObjectRefDef::Target(target) = reference {
            return self.objects_sharing_name_with_target(scoped.target_slot(target), object);
        }
        let Some(name) = self
            .object_reference_id(reference, object, context, scoped)
            .and_then(|referenced| self.object_card_name(referenced))
        else {
            return Vec::new();
        };
        self.battlefield
            .iter()
            .filter(|permanent| {
                self.permanent_card_name(permanent.card.id)
                    .is_some_and(|candidate| candidate == name)
            })
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect()
    }

    pub(super) fn effect_recipients(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        match recipient.0 {
            EffectRecipientSetDef::LegalTargets(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .collect()
            }
            EffectRecipientSetDef::Objects(objects) => {
                self.effect_objects(objects, object, context, scoped)
            }
            EffectRecipientSetDef::Players(players) => self
                .players_in_set(players, object, context, scoped)
                .into_iter()
                .map(Target::Player)
                .collect(),
            // "Each opponent and each creature they control": both kinds in
            // one list, players first, which is the order the clause reads.
            EffectRecipientSetDef::PlayersAndCreaturesTheyControl(players) => {
                let players = self.players_in_set(players, object, context, scoped);
                let mut recipients = players
                    .iter()
                    .copied()
                    .map(Target::Player)
                    .collect::<Vec<_>>();
                recipients.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| players.contains(&permanent.controller))
                        .filter(|permanent| {
                            self.permanent_types(permanent).is_some_and(|types| {
                                types.contains(crate::card::CardType::Creature)
                            })
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                recipients
            }
        }
    }

    /// Whether one member of a binding matches a predicate, wherever it is.
    /// A binding can hold battlefield permanents as readily as cards in a
    /// graveyard, so both are looked for.
    fn bound_object_matches(
        &self,
        bound: Target,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) -> bool {
        let (Target::Card(id) | Target::Permanent(id) | Target::Spell(id)) = bound else {
            return false;
        };
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
        {
            return self.trigger_object_matches(
                predicate,
                &self.trigger_event_object(permanent),
                source,
                false,
            );
        }
        self.card_in_nonbattlefield_zone(id)
            .is_some_and(|(zone, card)| self.card_object_matches(predicate, card, zone, source))
    }

    /// The permanents a stack object has chosen as targets.
    ///
    /// The spell that triggered an ability is still on the stack while that
    /// ability resolves above it; one answered in between is read from what
    /// it was, which holds the same targets either way. Only permanents come
    /// back: a spell that also points at a player targets a permanent all the
    /// same, and the player is not one of "those permanents".
    fn permanents_targeted_by(&self, reference: Target) -> Vec<Target> {
        let (Target::Spell(spell) | Target::Permanent(spell) | Target::Card(spell)) = reference
        else {
            return Vec::new();
        };
        self.stack
            .iter()
            .find(|candidate| candidate.id == spell)
            .or_else(|| match self.retired_objects.get(&spell) {
                Some(crate::game::RetiredObject::Stack(retired)) => Some(retired.as_ref()),
                _ => None,
            })
            .map(|stack_object| {
                stack_object
                    .iter_targets()
                    .copied()
                    .filter(|target| {
                        matches!(target, Target::Permanent(id)
                            if self.battlefield.iter().any(|permanent| permanent.card.id == *id))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The cards exiled with `source` that match. Which permanent exiled
    /// them, not where they are: the pile is read off the link the exile
    /// recorded rather than found by looking.
    fn linked_exile_targets(
        &self,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) -> Vec<Target> {
        self.linked_exiles
            .iter()
            .filter(|(exiled_by, _)| *exiled_by == source)
            .map(|(_, exiled)| *exiled)
            .filter(|exiled| {
                self.card_in_nonbattlefield_zone(*exiled)
                    .is_some_and(|(zone, card)| {
                        self.card_object_matches(predicate, card, zone, source)
                    })
            })
            .map(Target::Card)
            .collect()
    }

    pub(super) fn effect_objects(
        &self,
        objects: ObjectSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        match objects {
            ObjectSetDef::One(reference) => self
                .object_reference_target(reference, object, context, scoped)
                .into_iter()
                .collect(),
            ObjectSetDef::LegalTargets(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .filter(|target| !matches!(target, Target::Player(_)))
                    .collect()
            }
            ObjectSetDef::Binding(binding) => context.object_group(binding).to_vec(),
            ObjectSetDef::MatchingBinding {
                binding,
                object: predicate,
            } => context
                .object_group(binding)
                .iter()
                .copied()
                .filter(|bound| self.bound_object_matches(*bound, predicate, object.id))
                .collect(),
            ObjectSetDef::PermanentsTargetedBy(reference) => self
                .object_reference_target(reference, object, context, scoped)
                .map(|reference| self.permanents_targeted_by(reference))
                .unwrap_or_default(),
            ObjectSetDef::Query(query) => {
                self.objects_matching_effect_query(query, object, context, scoped)
            }
            ObjectSetDef::SharingNameWith(reference) => {
                self.objects_sharing_name_with_reference(reference, object, context, scoped)
            }
            ObjectSetDef::SharingNameWithBinding {
                binding,
                player,
                zone,
            } => {
                let Some(player) = self.player_reference(player, object, context, scoped) else {
                    return Vec::new();
                };
                let names: Vec<_> = context
                    .object_group(binding)
                    .iter()
                    .filter_map(|bound| match bound {
                        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                            self.object_card_name(*id)
                        }
                        Target::Player(_) => None,
                    })
                    .collect();
                let mut found = Vec::new();
                for name in names {
                    for card in self.cards_named_in_zone(player, zone, name.as_ref()) {
                        if !found.contains(&card) {
                            found.push(card);
                        }
                    }
                }
                found
            }
            // The back of the vector is the newest card, which is the one on
            // top of the pile.
            ObjectSetDef::TopOfGraveyardMatching {
                player,
                object: predicate,
            } => {
                let Some(player) = self.player_reference(player, object, context, scoped) else {
                    return Vec::new();
                };
                let source = object.source.unwrap_or(object.id);
                self.players[player.index()]
                    .graveyard
                    .iter()
                    .rev()
                    .find(|card| {
                        self.card_object_matches(predicate, card, ZoneKind::Graveyard, source)
                    })
                    .map(|card| Target::Card(card.id))
                    .into_iter()
                    .collect()
            }
            ObjectSetDef::LinkedExiles(predicate) => {
                self.linked_exile_targets(predicate, object.source.unwrap_or(object.id))
            }
            // The front of the vector is the oldest card, which is the one at
            // the bottom of the pile.
            ObjectSetDef::BottomOfGraveyard(player) => self
                .player_reference(player, object, context, scoped)
                .and_then(|player| self.players[player.index()].graveyard.first())
                .map(|card| Target::Card(card.id))
                .into_iter()
                .collect(),
        }
    }
}
