use super::{
    BattlefieldExit, CardInstance, CardPartId, CommittedTriggerEvent, CounterKind, EntryCompletion,
    Game, GameEvent, GameObjectId, KeywordAbility, PendingBattlefieldEntry, Permanent, PlayerId,
    StackObject, Step, Target, TargetSlotId, TurnStepDef, ZoneKind, ZonePlacement, remove_card,
};

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
            .filter(|permanent| self.permanent_card_name(permanent.card.id) == Some(name))
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect()
    }

    /// The printed name of any object the engine can still find, wherever it
    /// is. Used by the cards that speak about names rather than identity.
    pub(super) fn object_card_name(&self, id: GameObjectId) -> Option<&str> {
        self.permanent_card_name(id).or_else(|| {
            self.card_in_nonbattlefield_zone(id)
                .and_then(|(_, card)| self.catalog.get(card.definition))
                .map(|card| card.name.as_str())
        })
    }

    /// The copiable name a permanent presents, for the cards that gather
    /// everything sharing a name.
    pub(super) fn permanent_card_name(&self, id: GameObjectId) -> Option<&str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| self.catalog.get(Self::effective_rules_source(permanent).0))
            .map(|card| card.name.as_str())
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
    pub(super) fn tap_permanent(&mut self, id: GameObjectId) -> Option<CardInstance> {
        let (card, event, was_tapped) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| {
                (
                    permanent.card.clone(),
                    self.trigger_event_object(permanent),
                    permanent.tapped,
                )
            })?;
        if !was_tapped {
            self.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == id)
                .expect("the observed permanent remains on the battlefield")
                .tapped = true;
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecomesTapped {
                object: event,
            });
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

    pub(super) fn sacrifice_permanent(&mut self, id: GameObjectId) {
        self.move_permanents_to_graveyard(&[id]);
    }

    pub(super) fn destroy_permanents(&mut self, ids: &[GameObjectId], can_regenerate: bool) {
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
            if can_regenerate && permanent.regeneration_shields > 0 {
                self.regenerate_permanent(id);
            } else {
                doomed.push(id);
            }
        }
        self.move_permanents_to_graveyard(&doomed);
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
            permanent.damage_sources.clear();
            permanent.deathtouch_damage = false;
            permanent.attacking = false;
            permanent.blocked = false;
            permanent.blocking = None;
            permanent.combat_damage_assignment.clear();
        }
        let _ = self.tap_permanent(id);
        for other in &mut self.battlefield {
            if other.card.id != id && other.blocking == Some(id) {
                other.blocking = None;
            }
        }
    }

    /// Moves one simultaneous batch to graveyards. Listener declarations and
    /// last-known characteristics are frozen before any member leaves, then all
    /// old object incarnations are retired before the individual zone-change
    /// events are published.
    pub(super) fn move_permanents_to_graveyard(&mut self, ids: &[GameObjectId]) {
        let listeners = self.battlefield_trigger_listeners();
        let mut seen = Vec::new();
        let exits = ids
            .iter()
            .filter_map(|id| {
                if seen.contains(id) {
                    return None;
                }
                seen.push(*id);
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
                    .map(|permanent| {
                        (
                            *id,
                            self.battlefield_exit_snapshot(permanent),
                            permanent.damage_sources.clone(),
                            permanent.exile_instead_of_dying,
                            self.has_undying(permanent)
                                && permanent.counters(CounterKind::PlusOnePlusOne) == 0,
                            permanent.presented,
                        )
                    })
            })
            .collect::<Vec<_>>();

        // CR 614.12: a replacement effect that applies as a permanent leaves
        // the battlefield reads the state before the event, so a Rest in
        // Peace that is itself dying still replaces its own placement.
        let graveyard_replacement = self.external_zone_move_replacement(ZoneKind::Graveyard);

        self.creature_died_this_turn |=
            exits.iter().any(|(_, snapshot, _, exile_instead, _, _)| {
                !exile_instead && snapshot.object.types.is_creature()
            });
        for (_, snapshot, damage_sources, exile_instead, _, _) in &exits {
            if *exile_instead {
                continue;
            }
            for &source in damage_sources {
                self.capture_battlefield_triggers_from_snapshot(
                    &listeners,
                    &CommittedTriggerEvent::DamagedCreatureDied {
                        object: snapshot.object.clone(),
                        source,
                    },
                );
            }
        }

        let mut removed = Vec::new();
        for (id, snapshot, _, exile_instead, undying, presented) in exits {
            let index = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == id)
                .expect("a snapshotted battlefield object remains until its batch exits");
            let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
            removed.push((permanent, snapshot, exile_instead, undying, presented));
        }

        for (permanent, snapshot, exile_instead, undying, presented) in removed {
            let (to, destination) = if exile_instead {
                (ZoneKind::Exile, BattlefieldExit::Exile)
            } else {
                (ZoneKind::Graveyard, BattlefieldExit::Graveyard)
            };
            let event = CommittedTriggerEvent::ZoneChanged {
                object: snapshot.object,
                from: ZoneKind::Battlefield,
                to,
            };
            self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
            self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
            self.record_battlefield_exit(&permanent, destination);
            // 111.7: a token that leaves the battlefield ceases to exist. The
            // exit and everything watching for it still happened.
            if self.is_token(permanent.card.definition) {
                continue;
            }
            let owner = permanent.card.owner;
            let (card, _zone_change) = self.zone_change_card(permanent.card);
            if exile_instead || graveyard_replacement == Some(ZoneKind::Exile) {
                self.players[owner.index()].exile.push(card);
                continue;
            }
            self.put_card_into_graveyard(owner, card);

            // Undying observes the creature as it died, then returns the card
            // from the graveyard as a fresh object under its owner's control.
            if undying {
                self.return_top_graveyard_card_with_undying(owner, presented);
            }
        }
    }

    pub(super) fn return_top_graveyard_card_with_undying(
        &mut self,
        owner: PlayerId,
        presented: CardPartId,
    ) {
        let Some(card) = self.players[owner.index()].graveyard.pop() else {
            return;
        };
        let mut permanent =
            Permanent::entering(card, presented, owner, self.turns_started[owner.index()]);
        permanent.add_counters(CounterKind::PlusOnePlusOne, 1);
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Graveyard,
            completion: EntryCompletion::None,
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
            definition: permanent.card.definition,
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
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        let event = CommittedTriggerEvent::ZoneChanged {
            object: snapshot.object,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Exile,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
        self.record_battlefield_exit(&permanent, BattlefieldExit::Exile);
        if self.is_token(permanent.card.definition) {
            return;
        }
        let owner = permanent.card.owner;
        let (card, _zone_change) = self.zone_change_card(permanent.card);
        self.players[owner.index()].exile.push(card);
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
        self.players[owner.index()].exile.push(card);
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
    ) {
        let Some(owner) = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            self.players[player.index()]
                .exile
                .iter()
                .any(|card| card.id == id)
        }) else {
            return;
        };
        let Some(card) = remove_card(&mut self.players[owner.index()].exile, id) else {
            return;
        };
        if zone == ZoneKind::Battlefield {
            self.put_card_onto_battlefield_from(card, ZoneKind::Exile, owner, grant);
        } else {
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[owner.index()].hand.push(card);
        }
    }

    /// Raises the start-of-step event and resolves whatever was waiting for
    /// it. The upkeep has its own richer path and calls both itself.
    pub(super) fn begin_step_triggers(&mut self) {
        if self.step == Step::Upkeep {
            return;
        }
        let step = Self::turn_step_def(self.step);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step,
            player: self.active_player,
        });
        self.fire_delayed_triggers(step);
    }

    /// Resolves the effects that were waiting for this step.
    ///
    /// A real delayed trigger goes on the stack and can be responded to. This
    /// resolves at the step boundary instead, which no card here can tell
    /// apart, and keeps the queue from needing a listener of its own.
    pub(super) fn fire_delayed_triggers(&mut self, step: TurnStepDef) {
        let active = self.active_player;
        let mut waiting = std::mem::take(&mut self.delayed_triggers);
        let mut due = Vec::new();
        for delayed in waiting.extract_if(.., |delayed| {
            delayed.step == step
                && self.player_relation_matches(
                    active,
                    delayed.player,
                    delayed.object.controller,
                    delayed.context,
                )
        }) {
            due.push(delayed);
        }
        // Restore the waiting allocation before resolving. A due effect may
        // enqueue another delayed effect, which belongs after every entry
        // that was already waiting and must not fire in this batch.
        self.delayed_triggers = waiting;
        for delayed in due {
            self.resolve_effect_def(delayed.effect, &delayed.object, delayed.context);
        }
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
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        let event = CommittedTriggerEvent::ZoneChanged {
            object: snapshot.object,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Hand,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
        self.record_battlefield_exit(&permanent, BattlefieldExit::Hand);
        if self.is_token(permanent.card.definition) {
            return;
        }
        let owner = permanent.card.owner;
        let (card, _zone_change) = self.zone_change_card(permanent.card);
        self.players[owner.index()].hand.push(card);
    }

    /// Puts a permanent on top of its owner's library. The exit is the same
    /// procedure a bounce uses; only the destination differs.
    pub(super) fn return_permanent_to_library(
        &mut self,
        id: GameObjectId,
        placement: ZonePlacement,
    ) {
        let listeners = self.battlefield_trigger_listeners();
        let Some(index) = self
            .battlefield
            .iter()
            .position(|permanent| permanent.card.id == id)
        else {
            return;
        };
        let snapshot = self.battlefield_exit_snapshot(&self.battlefield[index]);
        let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
        let event = CommittedTriggerEvent::ZoneChanged {
            object: snapshot.object,
            from: ZoneKind::Battlefield,
            to: ZoneKind::Library,
        };
        self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
        self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
        self.record_battlefield_exit(&permanent, BattlefieldExit::LibraryTop);
        if self.is_token(permanent.card.definition) {
            return;
        }
        let owner = permanent.card.owner;
        let (card, _zone_change) = self.zone_change_card(permanent.card);
        match placement {
            ZonePlacement::Top => self.players[owner.index()].library.push(card),
            ZonePlacement::Bottom => self.players[owner.index()].library.insert(0, card),
        }
    }
}
