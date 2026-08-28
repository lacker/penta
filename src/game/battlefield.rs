use std::borrow::Cow;

use super::{
    AbilitySourceRef, ApplicableZoneMoveReplacement, AppliedRuleDef, BattlefieldArrival,
    BattlefieldExit, BattlefieldExitCompletion, BattlefieldExitSnapshot, CardInstance, CardPartId,
    CommittedTriggerEvent, CounterKind, DecisionContinuation, DecisionOption,
    DecisionOrderSemantics, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, EffectDef, EffectResolutionContext, EntryCompletion,
    FrozenZoneMoveReplacement, Game, GameEvent, GameObjectId, KeywordAbility, ObjectInstance,
    PendingBattlefieldEntry, PendingBattlefieldExitBatch, PendingBattlefieldExitMove, Permanent,
    PlayerId, ReplacementConditionDef, ReplacementEffectContext, ReplacementEffectDef,
    ReplacementEventDef, RetiredObject, ScopedEffect, StackObject, StackObjectKind, Step, Target,
    TargetSlotId, TriggerContext, ZoneKind, ZoneMoveCauseDef, ZonePlacement, remove_card,
};
use crate::ObjectSetBindingIndex;

impl Game {
    /// Every battlefield permanent whose printed name matches the chosen
    /// target's, the target included.
    pub(super) fn objects_sharing_name_with_target(
        &self,
        slot: TargetSlotId,
        object: &StackObject,
    ) -> Vec<Target> {
        let Some(name) = Self::chosen_targets(object, slot)
            .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
            .find_map(|target| match target {
                Target::Permanent(id) => self.permanent_card_name(id),
                _ => None,
            })
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

    /// The printed name of any object the engine can still find, wherever it
    /// is. Used by the cards that speak about names rather than identity.
    pub(super) fn object_card_name(&self, id: GameObjectId) -> Option<Cow<'_, str>> {
        self.permanent_card_name(id)
            .or_else(|| {
                self.card_in_nonbattlefield_zone(id)
                    .map(|(_, card)| card)
                    .or_else(|| {
                        self.players
                            .iter()
                            .flat_map(|player| player.outside_game.iter())
                            .find(|card| card.id == id)
                    })
                    .and_then(|card| self.catalog.get(card.definition))
                    .map(|card| Cow::Borrowed(card.name.as_str()))
            })
            .or_else(|| match self.retired_objects.get(&id) {
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    self.presentation_name(Self::effective_rules_source(permanent))
                }
                Some(RetiredObject::Card(card)) => self
                    .catalog
                    .get(card.definition)
                    .map(|definition| Cow::Borrowed(definition.name.as_str())),
                Some(RetiredObject::Stack(stack)) => self.presentation_name(stack.presentation()),
                None => None,
            })
    }

    /// The copiable name a permanent presents, for the cards that gather
    /// everything sharing a name.
    pub(super) fn permanent_card_name(&self, id: GameObjectId) -> Option<Cow<'_, str>> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| self.effective_permanent_name(permanent))
    }

    pub(super) fn permanent_controller(&self, id: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| permanent.controller)
    }

    /// Commits the untapped-to-tapped transition in one place so triggered
    /// abilities observe mana costs, activated-ability costs, combat, and
    /// resolving tap effects through the same event path.
    pub(super) fn tap_permanent(&mut self, id: GameObjectId) -> Option<ObjectInstance> {
        self.tap_permanent_with_purpose(id, false, None)
    }

    pub(super) fn tap_permanent_for_mana(
        &mut self,
        id: GameObjectId,
        triggered_mana: Option<Vec<crate::ManaSplit>>,
    ) -> Option<ObjectInstance> {
        self.tap_permanent_with_purpose(id, true, triggered_mana)
    }

    fn tap_permanent_with_purpose(
        &mut self,
        id: GameObjectId,
        for_mana: bool,
        triggered_mana: Option<Vec<crate::ManaSplit>>,
    ) -> Option<ObjectInstance> {
        let (card, was_tapped) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| (permanent.card.clone(), permanent.tapped))?;
        if !was_tapped {
            self.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == id)
                .expect("the observed permanent remains on the battlefield")
                .tapped = true;
            let event = self.trigger_event_object(
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .expect("the tapped permanent remains on the battlefield"),
            );
            let event = CommittedTriggerEvent::Tapped {
                object: event,
                for_mana,
            };
            if for_mana {
                let listeners = self.battlefield_trigger_listeners();
                let mut choices = triggered_mana.unwrap_or_default().into_iter();
                self.capture_battlefield_trigger_batch_with_mana_resolver(
                    &listeners,
                    std::slice::from_ref(&event),
                    |game, capture| {
                        game.resolve_triggered_mana_effect_with_choices(
                            capture.source,
                            capture.controller,
                            capture.effect,
                            &capture.context,
                            &mut choices,
                        );
                    },
                );
            } else {
                self.capture_battlefield_triggers(&event);
            }
        }
        Some(card)
    }

    pub(super) fn destroy_permanent(&mut self, id: GameObjectId) {
        self.destroy_permanents(&[id], true);
    }

    #[cfg(test)]
    pub(super) fn destroy_permanent_without_regeneration(&mut self, id: GameObjectId) {
        self.destroy_permanents(&[id], false);
    }

    #[cfg(test)]
    pub(super) fn sacrifice_permanent(&mut self, id: GameObjectId) {
        self.move_permanents_to_graveyard(&[id]);
    }

    pub(super) fn destroy_permanents(&mut self, ids: &[GameObjectId], can_regenerate: bool) {
        self.destroy_permanents_then(ids, can_regenerate, None);
    }

    pub(super) fn destroy_permanents_then(
        &mut self,
        ids: &[GameObjectId],
        can_regenerate: bool,
        completion: Option<BattlefieldExitCompletion>,
    ) {
        let mut seen = Vec::new();
        let mut doomed = Vec::new();
        for &id in ids {
            if seen.contains(&id) {
                continue;
            }
            seen.push(id);
            let Some(permanent) = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
            else {
                continue;
            };
            if self.has_indestructible(permanent) {
                continue;
            }
            if can_regenerate
                && permanent.regeneration_shields > 0
                && !self.has_applied_rule(permanent, AppliedRuleDef::CannotRegenerate)
            {
                self.regenerate_permanent(id);
            } else {
                doomed.push(id);
            }
        }
        self.move_permanents_to_graveyard_then(&doomed, completion);
    }

    /// Arms one regeneration shield (CR 701.15). The shield is a promise about
    /// the next destruction, not an effect on the permanent now, so a creature
    /// that is never destroyed this turn is left untouched and cleanup
    /// discards the shield.
    pub(super) fn add_regeneration_shield(&mut self, id: GameObjectId) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
        {
            // CR 701.19c: a prohibition stops the shield from applying, not
            // the resolving effect from creating it.
            permanent.regeneration_shields = permanent.regeneration_shields.saturating_add(1);
        }
    }

    pub(super) fn regenerate_permanent(&mut self, id: GameObjectId) {
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        {
            let permanent = &mut self.battlefield[index];
            permanent.regeneration_shields -= 1;
            permanent.damage = 0;
            permanent.deathtouch_damage = false;
        }
        self.remove_permanent_from_combat(id);
        let _ = self.tap_permanent(id);
    }

    /// CR 506.4: the permanent stops attacking or blocking, and nothing is
    /// blocking it any more. Regeneration does this as part of its shield; an
    /// effect that only removes a creature from combat does the same.
    ///
    /// The blockers themselves stay blocking creatures. CR 506.4 lists every
    /// way a permanent leaves combat and an attacker's departure is not one of
    /// them, so only the relationship goes -- which is why
    /// `blocking_this_combat` is left alone for everyone but the permanent
    /// actually being removed.
    pub(super) fn remove_permanent_from_combat(&mut self, id: GameObjectId) {
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
        else {
            return;
        };
        permanent.attacking = false;
        permanent.attacking_band = None;
        permanent.blocked = false;
        permanent.blocking.clear();
        permanent.blocking_this_combat = false;
        permanent.combat_damage_assignment.clear();
        for other in &mut self.battlefield {
            if other.card.id != id && other.is_blocking(id) {
                other.blocking.retain(|attacker| *attacker != id);
            }
        }
    }

    /// Proposes one simultaneous batch of battlefield-to-graveyard moves. All
    /// effective replacement abilities are frozen before any member leaves;
    /// if CR 616 requires the affected object's controller to order two or
    /// more effects, the entire batch remains prospective behind that choice.
    /// Sacrificing, which is a way of putting a permanent into a graveyard
    /// rather than something that happens to it there. The event is
    /// published before anything moves, so what was sacrificed is still on
    /// the battlefield to be read.
    pub(super) fn sacrifice_permanents(&mut self, ids: &[GameObjectId]) {
        self.capture_sacrifices(ids);
        self.move_permanents_to_graveyard(ids);
    }

    /// The event half on its own, for the sacrifices whose move carries a
    /// follow-up and so cannot go through the pair above.
    pub(super) fn capture_sacrifices(&mut self, ids: &[GameObjectId]) {
        let sacrificed = self
            .battlefield
            .iter()
            .filter(|permanent| ids.contains(&permanent.card.id))
            .map(|permanent| (self.trigger_event_object(permanent), permanent.controller))
            .collect::<Vec<_>>();
        for (object, player) in sacrificed {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::Sacrificed {
                object,
                player,
            });
        }
    }

    pub(super) fn move_permanents_to_graveyard(&mut self, ids: &[GameObjectId]) {
        self.move_permanents_to_graveyard_then(ids, None);
    }

    /// Exiles every named permanent as one simultaneous battlefield-exit
    /// event. A global exile must freeze all listeners before any source
    /// leaves, just as a global destroy or sacrifice does.
    pub(super) fn exile_permanents(&mut self, ids: &[GameObjectId]) {
        self.move_permanents_to_zone_then(ids, ZoneKind::Exile, ZonePlacement::Top, None);
    }

    pub(super) fn move_permanents_to_graveyard_then(
        &mut self,
        ids: &[GameObjectId],
        completion: Option<BattlefieldExitCompletion>,
    ) {
        self.move_permanents_to_zone_then(ids, ZoneKind::Graveyard, ZonePlacement::Top, completion);
    }

    /// Proposes one simultaneous batch toward the same destination. Library
    /// placement travels with every member because CR 401.4 may suspend the
    /// event for each owner to arrange cards sharing that exact position.
    pub(super) fn move_permanents_to_zone(
        &mut self,
        ids: &[GameObjectId],
        destination: ZoneKind,
        placement: ZonePlacement,
    ) {
        self.move_permanents_to_zone_then(ids, destination, placement, None);
    }

    pub(super) fn move_permanents_to_zone_then(
        &mut self,
        ids: &[GameObjectId],
        destination: ZoneKind,
        placement: ZonePlacement,
        completion: Option<BattlefieldExitCompletion>,
    ) {
        let mut seen = Vec::new();
        let mut moves = ids
            .iter()
            .filter_map(|id| {
                if seen.contains(id) {
                    return None;
                }
                seen.push(*id);
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
                    .map(|permanent| PendingBattlefieldExitMove {
                        object: *id,
                        controller: permanent.controller,
                        // A finality counter says the same thing a
                        // turn-long exile-instead effect says (CR 122.1h),
                        // and outlasts it: the counter is on the permanent
                        // rather than on the turn. The flag beside it only
                        // restores the retired card-local representation from
                        // older checkpoints.
                        destination: if destination == ZoneKind::Graveyard
                            && (permanent.exile_instead_of_dying
                                || permanent.counters(CounterKind::Finality) > 0
                                || self.has_applied_rule(
                                    permanent,
                                    AppliedRuleDef::ExileInsteadOfDying,
                                )) {
                            ZoneKind::Exile
                        } else {
                            destination
                        },
                        placement,
                        counters: None,
                        replaced_with_nothing: false,
                        applied: Vec::new(),
                    })
            })
            .collect::<Vec<_>>();
        if moves.is_empty() {
            if let Some(completion) = completion {
                self.resume_battlefield_exit_completion(completion, &[]);
            }
            return;
        }

        // CR 616.1: when multiple players must make replacement choices for
        // simultaneous events, the active player chooses first, followed by
        // the nonactive player. Keep each player's original batch order.
        moves.sort_by_key(|proposed| proposed.controller != self.active_player);

        // CR 400.6: determine the simultaneous zone-change event, apply its
        // replacement effects, then move its objects. Rest in Peace therefore
        // replaces its own exit, and every member of the one event sees every
        // continuous replacement that existed as the event was proposed.
        let replacements = self.frozen_battlefield_zone_move_replacements();
        self.continue_battlefield_exit_replacements(PendingBattlefieldExitBatch {
            moves,
            replacements,
            completion: completion.map(Box::new),
        });
    }

    /// Undying and persist both bring the card straight back with one
    /// counter on it; which counter is the whole of the difference.
    pub(super) fn return_top_graveyard_card_with_counter(
        &mut self,
        owner: PlayerId,
        presented: CardPartId,
        counter: CounterKind,
    ) {
        let Some(card) = self.players[owner.index()].graveyard.pop() else {
            return;
        };
        let mut permanent = Permanent::entering(
            card,
            presented,
            owner,
            self.turns_started[owner.index()],
            self.turn,
        );
        permanent.add_counters(counter, 1);
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Graveyard,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
    }

    pub(super) fn record_battlefield_exit(
        &mut self,
        permanent: &Permanent,
        destination: BattlefieldExit,
    ) {
        self.events.push(GameEvent::PermanentLeftBattlefield {
            controller: permanent.controller,
            card: permanent.card.id,
            characteristics: Self::effective_rules_source(permanent),
            destination,
        });
    }

    pub(super) fn exile_permanent(&mut self, id: GameObjectId) {
        let listeners = self.battlefield_trigger_listeners();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let damage_sources = self.battlefield[index].damage_sources.clone();
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        self.record_battlefield_exit(&permanent, BattlefieldExit::Exile);
        let after = if permanent.card.definition.is_token() {
            None
        } else {
            let owner = permanent.card.owner;
            let (card, _zone_change) = self.zone_change_card(
                permanent
                    .card
                    .clone()
                    .into_card()
                    .expect("a nontoken permanent is backed by a card definition"),
            );
            let after = self.printed_trigger_event_object(
                card.id,
                card.definition,
                owner,
                &crate::CharacteristicContext::Exile,
            );
            self.players[owner.index()].exile.push(card);
            after
        };
        let event = CommittedTriggerEvent::ZoneChanged {
            before: Some(snapshot.object),
            after,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Exile,
            damage_sources,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
    }

    /// Exiles a permanent and reports the object it became in exile, so the
    /// clause that promised to return it can remember which card that is.
    pub(super) fn exile_permanent_returning_card(
        &mut self,
        id: GameObjectId,
    ) -> Option<GameObjectId> {
        let owner = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| permanent.card.owner)?;
        let before = self.players[owner.index()].exile.len();
        self.exile_permanent(id);
        self.players[owner.index()]
            .exile
            .get(before)
            .map(|card| card.id)
    }

    /// Exiles a card from wherever it is outside the battlefield, reporting
    /// the object it became so the link can be recorded.
    pub(super) fn exile_card_returning_card(&mut self, id: GameObjectId) -> Option<GameObjectId> {
        let (zone, owner) = self
            .card_in_nonbattlefield_zone(id)
            .map(|(zone, card)| (zone, card.owner))?;
        if zone == ZoneKind::Exile {
            return None;
        }
        let card = self.take_card_from_zone(owner, zone, id)?;
        let (card, _zone_change) = self.zone_change_card(card);
        let exiled = card.id;
        self.players[owner.index()].exile.push(card.clone());
        self.capture_cards_exiled(std::slice::from_ref(&card), zone);
        if zone == ZoneKind::Graveyard {
            self.note_card_left_graveyard(owner);
        }
        Some(exiled)
    }

    /// Removes a card from one of a player's non-battlefield zones.
    pub(super) fn take_card_from_zone(
        &mut self,
        owner: PlayerId,
        zone: ZoneKind,
        id: GameObjectId,
    ) -> Option<CardInstance> {
        let state = &mut self.players[owner.index()];
        let cards = match zone {
            ZoneKind::Library => &mut state.library,
            ZoneKind::Hand => &mut state.hand,
            ZoneKind::Graveyard => &mut state.graveyard,
            ZoneKind::Exile => &mut state.exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return None,
        };
        remove_card(cards, id)
    }

    /// Brings a linked exile back. A card that is no longer in exile has
    /// moved on, and nothing follows it.
    pub(super) fn return_exiled_card(
        &mut self,
        id: GameObjectId,
        zone: ZoneKind,
        grant: Option<KeywordAbility>,
        arriving_controller: Option<PlayerId>,
        transformed: bool,
        counters: Option<(CounterKind, u16)>,
    ) -> Option<GameObjectId> {
        let owner = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            self.players[player.index()]
                .exile
                .iter()
                .any(|card| card.id == id)
        })?;
        let card = remove_card(&mut self.players[owner.index()].exile, id)?;
        if zone == ZoneKind::Battlefield {
            let _ = self.put_card_onto_battlefield_from(
                card,
                ZoneKind::Exile,
                if transformed {
                    BattlefieldArrival::transformed_under(arriving_controller.unwrap_or(owner))
                } else {
                    BattlefieldArrival::under(arriving_controller.unwrap_or(owner))
                }
                .with_counters(counters),
                grant,
            );
            // The card that left exile and the permanent now standing there
            // are two objects, and the arrival is the one a following clause
            // has to name.
            self.arrived.take()
        } else {
            let (card, _zone_change) = self.zone_change_card(card);
            let arrived = card.id;
            self.players[owner.index()].hand.push(card);
            Some(arrived)
        }
    }

    /// Raises the start-of-step event. The upkeep has its own richer path and
    /// publishes the same event there.
    pub(super) fn begin_step_triggers(&mut self) {
        if self.step == Step::Upkeep {
            return;
        }
        let step = Self::turn_step_def(self.step);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step,
            player: self.active_player,
        });
    }

    pub(super) fn return_permanent_to_hand(&mut self, id: GameObjectId) {
        let listeners = self.battlefield_trigger_listeners();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let damage_sources = self.battlefield[index].damage_sources.clone();
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        self.record_battlefield_exit(&permanent, BattlefieldExit::Hand);
        let after = if permanent.card.definition.is_token() {
            None
        } else {
            let owner = permanent.card.owner;
            let (card, _zone_change) = self.zone_change_card(
                permanent
                    .card
                    .clone()
                    .into_card()
                    .expect("a nontoken permanent is backed by a card definition"),
            );
            let after = self.printed_trigger_event_object(
                card.id,
                card.definition,
                owner,
                &crate::CharacteristicContext::Hand,
            );
            self.players[owner.index()].hand.push(card);
            after
        };
        let event = CommittedTriggerEvent::ZoneChanged {
            before: Some(snapshot.object),
            after,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Hand,
            damage_sources,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
    }

    /// Puts a permanent at one end of its owner's library through the shared
    /// simultaneous-exit and replacement procedure.
    pub(super) fn return_permanent_to_library(
        &mut self,
        id: GameObjectId,
        placement: ZonePlacement,
    ) {
        self.move_permanents_to_zone(&[id], ZoneKind::Library, placement);
    }
}

include!("battlefield/exits.rs");
