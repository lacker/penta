// Resolving object collections and their set operations.

impl Game {
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

    pub(in crate::game) fn source_object_set_targets(
        &self,
        objects: ObjectSetDef,
        source: GameObjectId,
    ) -> Vec<Target> {
        match objects {
            ObjectSetDef::InZone { objects, zone } => self
                .source_object_set_targets(*objects, source)
                .into_iter()
                .filter(|target| self.effect_target_in_zone(*target, zone))
                .collect(),
            ObjectSetDef::Union(sets) => {
                let mut union = Vec::new();
                for objects in sets {
                    for target in self.source_object_set_targets(*objects, source) {
                        if !union.contains(&target) {
                            union.push(target);
                        }
                    }
                }
                union
            }
            ObjectSetDef::Query(query) => self
                .current_or_last_known_controller(source)
                .or_else(|| self.current_or_last_known_owner(source))
                .map(|controller| {
                    self.objects_matching_query(query, controller, source, TriggerContext::empty())
                })
                .unwrap_or_default(),
            ObjectSetDef::LinkedExiles => self
                .linked_exile_ids(source)
                .into_iter()
                .filter(|id| self.card_in_nonbattlefield_zone(*id).is_some())
                .map(Target::Card)
                .collect(),
            ObjectSetDef::Matching {
                objects,
                object: predicate,
            } => self
                .source_object_set_targets(*objects, source)
                .into_iter()
                .filter(|target| self.bound_object_matches(*target, predicate.predicate(), source))
                .collect(),
            ObjectSetDef::AttachmentsOf(reference) => self
                .static_object_reference(reference, source)
                .map_or_else(Vec::new, |id| self.current_or_last_known_attachments(id))
                .into_iter()
                .map(Target::Permanent)
                .collect(),
            ObjectSetDef::SharingCreatureType {
                objects,
                object,
                sharing,
            } => {
                let reference = self.static_object_reference(object, source);
                self.source_object_set_targets(*objects, source)
                    .into_iter()
                    .filter(|target| {
                        let shares = Self::target_object_id(*target)
                            .zip(reference)
                            .is_some_and(|(id, other)| self.objects_share_creature_type(id, other));
                        shares == sharing
                    })
                    .collect()
            }
            ObjectSetDef::ExceptObject { objects, object } => {
                let excluded = match object {
                    ObjectRefDef::Source => Some(source),
                    ObjectRefDef::AttachedToSource => {
                        self.current_or_last_known_attached_host(source)
                    }
                    _ => None,
                };
                self.source_object_set_targets(*objects, source)
                    .into_iter()
                    .filter(|target| Self::target_object_id(*target) != excluded)
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    pub(super) fn target_object_id(target: Target) -> Option<GameObjectId> {
        match target {
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
            Target::Player(_) => None,
        }
    }

    fn legal_attachment_hosts(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        let Some(attachment) = self
            .object_reference_id(reference, object, context, scoped)
            .and_then(|attachment| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == attachment)
            })
        else {
            return Vec::new();
        };
        self.battlefield
            .iter()
            .filter(|host| self.is_legal_prospective_attachment_host(attachment, host.card.id))
            .map(|host| Target::Permanent(host.card.id))
            .collect()
    }

    fn zone_change_successors_of_binding(
        &self,
        binding: crate::Binding,
        context: &EffectResolutionContext,
    ) -> Vec<Target> {
        context
            .object_group(binding)
            .iter()
            .filter_map(|bound| match bound {
                Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                    self.zone_change_successor_target(*id)
                }
                Target::Player(_) => None,
            })
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn effect_objects(
        &self,
        objects: ObjectSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        match objects {
            ObjectSetDef::InZone { objects, zone } => self
                .effect_objects(*objects, object, context, scoped)
                .into_iter()
                .filter(|target| self.effect_target_in_zone(*target, zone))
                .collect(),
            ObjectSetDef::Union(sets) => {
                let mut union = Vec::new();
                for objects in sets {
                    for target in self.effect_objects(*objects, object, context, scoped) {
                        if !union.contains(&target) {
                            union.push(target);
                        }
                    }
                }
                union
            }
            ObjectSetDef::One(reference) => self
                .object_reference_target(reference, object, context, scoped)
                .into_iter()
                .collect(),
            ObjectSetDef::LegalTargets(target) => {
                self.effect_legal_target_objects(target, object, scoped)
            }
            ObjectSetDef::Binding(binding) => context.object_group(binding),
            ObjectSetDef::ZoneChangeSuccessorsOfBinding(binding) => {
                self.zone_change_successors_of_binding(binding, context)
            }
            ObjectSetDef::CardsDrawnThisTurnInHand(player) => {
                let Some(player) = self.player_reference(player, object, context, scoped) else {
                    return Vec::new();
                };
                self.cards_drawn_this_turn_in_hand(player)
            }
            ObjectSetDef::PermanentsControlledBy(player) => {
                let Some(player) = self.player_reference(player, object, context, scoped) else {
                    return Vec::new();
                };
                self.battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == player)
                    .map(|permanent| Target::Permanent(permanent.card.id))
                    .collect()
            }
            ObjectSetDef::AttachmentsOf(reference) => self
                .effect_object_reference_id(reference, object, context, scoped)
                .map_or_else(Vec::new, |id| self.current_or_last_known_attachments(id))
                .into_iter()
                .map(Target::Permanent)
                .collect(),
            ObjectSetDef::TokensCreatedBy(reference) => {
                let Some(creator) = self
                    .object_reference_target(reference, object, context, scoped)
                    .and_then(|target| match target {
                        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
                        Target::Player(_) => None,
                    })
                else {
                    return Vec::new();
                };
                self.battlefield
                    .iter()
                    .filter(|permanent| permanent.created_by == Some(creator))
                    .map(|permanent| Target::Permanent(permanent.card.id))
                    .collect()
            }
            ObjectSetDef::MatchingBinding {
                binding,
                object: predicate,
            } => context
                .object_group(binding)
                .iter()
                .copied()
                .filter(|bound| {
                    self.effect_collection_target_matches(
                        predicate, *bound, object, context, scoped,
                    )
                })
                .collect(),
            ObjectSetDef::Matching {
                objects,
                object: predicate,
            } => self
                .effect_objects(*objects, object, context, scoped)
                .into_iter()
                .filter(|bound| {
                    self.effect_collection_target_matches(
                        predicate.predicate(),
                        *bound,
                        object,
                        context,
                        scoped,
                    )
                })
                .collect(),
            ObjectSetDef::PermanentsTargetedBy(reference) => self
                .object_reference_target(reference, object, context, scoped)
                .map(|reference| self.permanents_targeted_by(reference))
                .unwrap_or_default(),
            ObjectSetDef::PlayerAttachments(query) => {
                self.effect_player_attachments(query, object, context)
            }
            ObjectSetDef::Query(query) => {
                self.objects_matching_effect_query(query, object, context, scoped)
            }
            ObjectSetDef::LegalAttachmentHosts(reference) => {
                self.legal_attachment_hosts(reference, object, context, scoped)
            }
            ObjectSetDef::SharingCreatureType {
                objects,
                object: reference,
                sharing,
            } => {
                let reference = self.effect_object_reference_id(reference, object, context, scoped);
                self.effect_objects(*objects, object, context, scoped)
                    .into_iter()
                    .filter(|target| {
                        let shares = Self::target_object_id(*target)
                            .zip(reference)
                            .is_some_and(|(id, other)| self.objects_share_creature_type(id, other));
                        shares == sharing
                    })
                    .collect()
            }
            ObjectSetDef::ExceptObject {
                objects,
                object: excluded,
            } => {
                let excluded = self.object_reference_id(excluded, object, context, scoped);
                self.effect_objects(*objects, object, context, scoped)
                    .into_iter()
                    .filter(|candidate| Self::target_object_id(*candidate) != excluded)
                    .collect()
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
            ObjectSetDef::LinkedExiles => self
                .linked_exile_ids(object.source.unwrap_or(object.id))
                .into_iter()
                .filter(|id| self.card_in_nonbattlefield_zone(*id).is_some())
                .map(Target::Card)
                .collect(),
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

    fn effect_legal_target_objects(
        &self,
        target: TargetIndex,
        object: &StackObject,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        let slot = scoped.target_slot(target);
        Self::chosen_targets(object, slot)
            .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
            .filter(|target| !matches!(target, Target::Player(_)))
            .collect()
    }

    fn effect_player_attachments(
        &self,
        query: PlayerAttachmentQueryDef,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) -> Vec<Target> {
        let source = object.source.unwrap_or(object.id);
        self.battlefield
            .iter()
            .filter(|permanent| {
                permanent.attached_player.is_some_and(|player| {
                    self.player_relation_matches_for_source(
                        player,
                        query.player,
                        object.controller,
                        source,
                        context.trigger,
                    )
                }) && self.trigger_object_matches(
                    query.object,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect()
    }

    fn cards_drawn_this_turn_in_hand(&self, player: PlayerId) -> Vec<Target> {
        self.drawn_this_turn[player.index()]
            .iter()
            .copied()
            .filter(|drawn| {
                self.players[player.index()]
                    .hand
                    .iter()
                    .any(|card| card.id == *drawn)
            })
            .map(Target::Card)
            .collect()
    }
}
