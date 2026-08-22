use super::{
    AbilityDef, AbilityOrigin, Action, ActionError, ActivationChoices, CardStructure, CardType,
    CharacteristicContext, CombatDamageStage, CounterKind, DecisionVisibility, DoubleFacedKind,
    EmblemObservation, Game, GameEvent, GameObjectId, GameResult, ManaActivationChoices,
    ObjectCharacteristics, ObjectKind, Permanent, PermanentObservation, PhysicalFaceObservation,
    PhysicalFaceSide, PlayerId, PlayerObservation, Pregame, StackObservation, Step, WinReason,
    ZoneKind, combinations, public_cards,
};

impl Game {
    #[must_use]
    pub const fn result(&self) -> Option<GameResult> {
        self.result
    }

    /// Whether the first-strike combat-damage step has finished and the
    /// regular combat-damage step will begin after priority passes.
    #[must_use]
    pub fn regular_combat_damage_pending(&self) -> bool {
        self.result.is_none()
            && self.step == Step::CombatDamage
            && self.pending_combat_assignments.is_empty()
            && matches!(
                &self.combat_damage_stage,
                CombatDamageStage::FirstStrike { .. }
            )
    }

    /// Returns the player expected to make the engine's next decision.
    ///
    /// This may differ from the player with priority during pregame choices,
    /// turn-based actions such as declaring blockers, and other mandatory
    /// choices. Bot runners should observe this player and submit one of that
    /// observation's legal actions.
    #[must_use]
    pub fn decision_player(&self) -> Option<PlayerId> {
        if self.result.is_some() {
            return None;
        }
        if let Some(decision) = self.pending_decisions.first() {
            return Some(decision.observation.player);
        }
        if let Some(attacker) = self.pending_combat_assignments.first().copied() {
            return Some(self.combat_damage_assigner(attacker));
        }
        if let Some(pregame) = self.pregame {
            return Some(match pregame {
                Pregame::Mulligan(player) | Pregame::Bottom(player) => player,
            });
        }
        if self.cleanup_pending || self.untap_pending {
            return Some(self.active_player);
        }
        if self.step == Step::DeclareAttackers && !self.attackers_declared {
            return Some(self.active_player);
        }
        if self.step == Step::DeclareBlockers && !self.blockers_declared {
            return Some(self.active_player.opponent());
        }
        Some(self.priority)
    }

    /// Whether the game is still settling opening hands.
    ///
    /// The first turn has not begun during mulligans, so a client should not
    /// be describing a step or a turn yet.
    #[must_use]
    pub const fn in_pregame(&self) -> bool {
        self.pregame.is_some()
    }

    #[must_use]
    /// Returns the omniscient event trace.
    ///
    /// This is intended for replays and debugging. Give bots
    /// [`PlayerObservation`] rather than this event stream.
    /// The raw event log, seed and all. This is not safe to hand to a seat:
    /// see [`Self::events_for`], which is.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }

    /// A mark in the event log, for asking what has happened since.
    ///
    /// Opaque: it indexes the raw log, not the projection, so a seat cannot
    /// read anything out of the number itself.
    #[must_use]
    pub fn event_cursor(&self) -> usize {
        self.events.len()
    }

    /// The events one seat may see since `cursor`. This is the windowed form
    /// of [`Self::events_for`], for a caller that wants only what one action
    /// caused.
    #[must_use]
    pub fn events_for_since(&self, viewer: PlayerId, cursor: usize) -> Vec<GameEvent> {
        let tail = self.events.get(cursor..).unwrap_or_default();
        Self::project_events(tail, viewer)
    }

    /// The events one seat may see.
    ///
    /// The raw log opens with [`GameEvent::GameStarted`], and that carries the
    /// seed the libraries were shuffled from. Decklists are public, so a seat
    /// holding the seed can reconstruct both libraries in order -- the
    /// opponent's hand and every draw either player will make. It is dropped
    /// here rather than redacted, because a client already knows a game
    /// started; [`Self::seed`] is how a local viewer, or a finished game,
    /// gets it deliberately.
    #[must_use]
    pub fn events_for(&self, viewer: PlayerId) -> Vec<GameEvent> {
        Self::project_events(&self.events, viewer)
    }

    /// The one place that decides what a seat may see. Everything a seat is
    /// handed goes through here, so a newly leaky event has exactly one
    /// function to be caught in.
    pub(super) fn project_events(events: &[GameEvent], _viewer: PlayerId) -> Vec<GameEvent> {
        events
            .iter()
            .filter(|event| !matches!(event, GameEvent::GameStarted { .. }))
            .cloned()
            .collect()
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn legal_actions(&self, player: PlayerId) -> Vec<Action> {
        if self.result.is_some() {
            return Vec::new();
        }

        let mut actions = vec![Action::Concede];
        if let Some(decision) = self.pending_decisions.first() {
            if decision.observation.player == player {
                // Bounded selections are represented by the decision observation rather
                // than by an eagerly-expanded Cartesian product. Callers submit the
                // selected option IDs through `ChooseDecision`; `apply` validates the
                // selection directly against this schema.
                actions.push(Action::ChooseDecision {
                    decision: decision.observation.id,
                    options: Vec::new(),
                });
                if decision.observation.cancellable {
                    actions.push(Action::CancelDecision {
                        decision: decision.observation.id,
                    });
                }
                // "You may cast that card" is answered by casting it, not by
                // answering the decision, so the cast stands beside the
                // decline rather than behind it.
                if let Some(offer) = decision.continuation.cast_offer()
                    && offer.player == player
                {
                    self.add_offered_cast_actions(offer, &mut actions);
                }
            }
            return actions;
        }
        if let Some(attacker) = self.pending_combat_assignments.first().copied() {
            if player == self.combat_damage_assigner(attacker) {
                actions.extend(self.combat_assignment_actions(attacker));
            }
            return actions;
        }
        if let Some(pregame) = self.pregame {
            match pregame {
                Pregame::Mulligan(deciding) if player == deciding => {
                    actions.push(Action::KeepHand);
                    actions.push(Action::TakeMulligan);
                }
                Pregame::Bottom(deciding) if player == deciding => {
                    let count = usize::from(self.mulligans[player.index()])
                        .min(self.players[player.index()].hand.len());
                    actions.extend(
                        combinations(
                            &self.players[player.index()]
                                .hand
                                .iter()
                                .map(|card| card.id)
                                .collect::<Vec<_>>(),
                            count,
                        )
                        .into_iter()
                        .map(|cards| Action::BottomCards { cards }),
                    );
                }
                Pregame::Mulligan(_) | Pregame::Bottom(_) => {}
            }
            return actions;
        }
        if self.cleanup_pending {
            if player == self.active_player {
                let state = &self.players[player.index()];
                let count = state.hand.len().saturating_sub(7);
                actions.extend(
                    combinations(
                        &state.hand.iter().map(|card| card.id).collect::<Vec<_>>(),
                        count,
                    )
                    .into_iter()
                    .map(|cards| Action::DiscardCards { cards }),
                );
            }
            return actions;
        }
        if self.untap_pending {
            if player == self.active_player {
                actions.extend(self.untap_actions(player));
            }
            return actions;
        }
        if self.step == Step::DeclareAttackers && !self.attackers_declared {
            if player == self.active_player {
                // A creature that attacks each combat if able is only
                // required to when it actually can, so the same conditions
                // that offer it as an attacker are what make it compulsory.
                let a_creature_must_attack = self.battlefield.iter().any(|permanent| {
                    permanent.controller == player
                        && !permanent.tapped
                        && !permanent.attacking
                        && self.must_attack_if_able(permanent)
                });
                if !a_creature_must_attack && self.attack_declaration_is_payable(player) {
                    actions.push(Action::FinishDeclaringAttackers);
                }
                actions.extend(self.attacker_actions(player));
                actions.extend(self.band_actions(player));
                actions.extend(self.exert_actions(player));
            }
            return actions;
        }
        if self.step == Step::DeclareBlockers && !self.blockers_declared {
            if player == self.active_player.opponent() {
                let blocks = self.blocker_actions(player);
                if !self.block_requirement_outstanding(&blocks)
                    && !self.menace_is_unsatisfied(player)
                {
                    actions.push(Action::FinishDeclaringBlockers);
                }
                actions.extend(blocks);
            }
            return actions;
        }
        if player != self.priority {
            return actions;
        }

        actions.push(Action::PassPriority);
        self.add_mana_actions(player, &mut actions);
        self.add_land_actions(player, &mut actions);
        self.add_spell_actions(player, &mut actions);
        self.add_ability_actions(player, &mut actions);
        self.add_face_up_actions(player, &mut actions);
        self.add_foretell_actions(player, &mut actions);
        self.add_unlock_door_actions(player, &mut actions);
        actions
    }

    /// Applies one engine-enumerated action for a player.
    ///
    /// # Errors
    ///
    /// Returns [`ActionError`] when the game is over or the action is not
    /// currently legal for that player.
    pub fn apply(&mut self, player: PlayerId, action: Action) -> Result<(), ActionError> {
        if self.result.is_some() {
            return Err(ActionError::GameAlreadyFinished);
        }
        if !self.is_legal_action(player, &action) {
            return Err(ActionError::NotLegal { player, action });
        }

        self.apply_legal_action(player, action);
        Ok(())
    }

    /// Applies an action chosen from an observation made immediately before
    /// this call. Synchronous engine runners can reuse that observation's
    /// enumerated actions instead of regenerating them in [`Self::apply`].
    ///
    /// This stays crate-private because its validity depends on the caller not
    /// mutating the game between observing it and submitting the action.
    pub(crate) fn apply_observed_action(
        &mut self,
        observation: &PlayerObservation,
        action: Action,
    ) -> Result<(), ActionError> {
        if self.result.is_some() {
            return Err(ActionError::GameAlreadyFinished);
        }
        let player = observation.viewer;
        let legal = if matches!(action, Action::ChooseDecision { .. }) {
            // Decision observations expose a bounded selection schema rather
            // than every combination, so the submitted options still need
            // direct validation against the current pending decision.
            self.is_legal_action(player, &action)
        } else {
            observation.legal_actions.contains(&action)
        };
        if !legal {
            return Err(ActionError::NotLegal { player, action });
        }

        self.apply_legal_action(player, action);
        Ok(())
    }

    fn apply_legal_action(&mut self, player: PlayerId, action: Action) {
        // Declaration members arrive as separate UI actions, but CR 508.1
        // and 509.1 make each completed set one turn-based action. Do not let
        // state-based actions or state triggers inspect a partial set.
        let declaration_is_still_open = matches!(
            &action,
            Action::DeclareAttacker { .. }
                // Exerting is part of the declaration that is still open, so
                // the trigger it captures waits for the rest of it the way
                // every other attack trigger does.
                | Action::ExertAttacker { .. }
                | Action::DeclareBlocker { .. }
        );
        match action {
            Action::KeepHand => self.keep_hand(player),
            Action::TakeMulligan => self.take_mulligan(player),
            Action::BottomCards { cards } => self.bottom_cards(player, &cards),
            Action::DiscardCards { cards } => self.discard_cards(player, &cards),
            Action::ChooseDecision { decision, options } => {
                self.choose_decision(player, decision, &options);
            }
            Action::CancelDecision { decision } => self.cancel_decision(decision),
            Action::ChooseUntap { permanents } => self.choose_untap(player, &permanents),
            Action::TurnFaceUp { permanent } => self.turn_face_up(player, permanent),
            Action::Foretell { card } => self.foretell(player, card),
            Action::UnlockDoor { room, door } => self.unlock_door(player, room, door),
            Action::PassPriority => self.pass_priority(player),
            Action::PlayLand { card, option } => self.play_land(player, card, option),
            Action::ActivateManaAbility {
                source,
                ability,
                color,
                counters_removed,
                cost_object,
                combination,
            } => {
                self.activate_mana_source(
                    player,
                    source,
                    ability,
                    color,
                    ManaActivationChoices {
                        counters_removed,
                        cost_object,
                        combination,
                    },
                );
            }
            Action::PayLifeForMana => {
                unreachable!("the legacy Channel action is never legal")
            }
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                self.cast_spell(player, card, &choices, &sacrifices);
            }
            Action::ActivateAbility {
                source,
                ability,
                targets,
                cost_objects,
                x,
                modes,
            } => self.activate_ability(
                player,
                source,
                ability,
                ActivationChoices {
                    targets,
                    cost_objects: &cost_objects,
                    x,
                    modes: &modes,
                },
            ),
            Action::DeclareAttacker { attacker, defender } => {
                self.declare_attacker(attacker, defender);
            }
            Action::BandAttackers { first, second } => self.form_band(first, second),
            Action::ExertAttacker { attacker } => self.exert_attacker(player, attacker),
            Action::FinishDeclaringAttackers => self.finish_declaring_attackers(),
            Action::DeclareBlocker { blocker, attacker } => {
                self.declare_blocker(blocker, attacker);
            }
            Action::FinishDeclaringBlockers => self.finish_declaring_blockers(),
            Action::AssignCombatDamage {
                attacker,
                assignments,
            } => self.assign_combat_damage(attacker, assignments),
            Action::Concede => self.finish(GameResult::Winner {
                winner: player.opponent(),
                reason: WinReason::OpponentConceded,
            }),
        }
        if self.result.is_none() && !declaration_is_still_open {
            self.finish_rules_procedure();
        }
    }

    /// Ends the game because `player` ran out of time.
    ///
    /// This is not an action, because losing on time is not something a
    /// player does -- it is imposed by whatever is holding the clock, and it
    /// does not require that player to hold priority. Hosts with no clock
    /// never call it.
    pub fn lose_on_time(&mut self, player: PlayerId) {
        if self.result.is_some() {
            return;
        }
        self.finish(GameResult::Winner {
            winner: player.opponent(),
            reason: WinReason::OpponentRanOutOfTime,
        });
    }

    /// Validates an action against the current state without mutating the game.
    ///
    /// Unlike [`legal_actions`], this also validates the option IDs supplied to
    /// a bounded [`Action::ChooseDecision`] selection without expanding every
    /// possible combination into a vector.
    #[must_use]
    pub fn is_legal_action(&self, player: PlayerId, action: &Action) -> bool {
        if let Action::ChooseDecision { decision, options } = action {
            let Some(pending) = self.pending_decisions.first() else {
                return false;
            };
            let observation = &pending.observation;
            if observation.player != player || observation.id != *decision {
                return false;
            }
            let available = observation
                .options
                .iter()
                .map(|option| option.id)
                .collect::<std::collections::HashSet<_>>();
            let unique = options
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>();
            options.len() == unique.len()
                && options.len() >= observation.minimum
                && options.len() <= observation.maximum
                && options.iter().all(|option| available.contains(option))
        } else {
            self.legal_actions(player).contains(action)
        }
    }

    #[must_use]
    /// The command-zone emblems as a client sees them: who owns each one, what
    /// to call it, and the text of every clause it grants.
    pub(super) fn observed_emblems(&self) -> Vec<EmblemObservation> {
        self.emblems
            .iter()
            .map(|emblem| {
                let ObjectCharacteristics::Emblem { emblem: authored } =
                    Self::effective_rules_source(emblem)
                else {
                    unreachable!("an emblem has creator-owned emblem characteristics")
                };
                EmblemObservation {
                    id: emblem.card.id,
                    controller: emblem.controller,
                    name: authored.name().to_owned(),
                    source_ability: emblem
                        .emblem_source
                        .expect("a created emblem records its creating ability"),
                    ability_texts: authored
                        .abilities()
                        .iter()
                        .map(|ability| ability.text.to_owned())
                        .collect(),
                }
            })
            .collect()
    }

    #[must_use]
    /// Physical topology is public while the permanent is face up, even when
    /// a copy effect supplies unrelated effective characteristics.
    fn physical_face_observation(&self, permanent: &Permanent) -> Option<PhysicalFaceObservation> {
        if permanent.face_down.is_some() {
            return None;
        }
        let (kind, front, back) = match permanent.card.definition {
            ObjectKind::Card(definition) => {
                let CardStructure::DoubleFaced { front, back, kind } =
                    &self.catalog.get(definition)?.structure
                else {
                    return None;
                };
                (*kind, *front, *back)
            }
            ObjectKind::Token => {
                if let Some(faces) = &permanent.double_faced_token_copy {
                    (faces.kind, faces.front_part, faces.back_part)
                } else {
                    let token = permanent.token_characteristics?;
                    let front = token.primary_part_id();
                    (
                        DoubleFacedKind::Transforming,
                        front,
                        token.other_face(front)?,
                    )
                }
            }
            ObjectKind::Emblem | ObjectKind::Ability => return None,
        };
        let side = if permanent.presented == front {
            PhysicalFaceSide::Front
        } else if permanent.presented == back {
            PhysicalFaceSide::Back
        } else {
            return None;
        };
        Some(PhysicalFaceObservation { kind, side })
    }

    /// One permanent as `viewer` sees it. Split out of `observe` because the
    /// per-permanent view is long on its own and reads better beside the
    /// hidden-information rule it enforces.
    fn observe_permanent(&self, permanent: &Permanent, viewer: PlayerId) -> PermanentObservation {
        let types = self.permanent_types(permanent).unwrap_or_default();
        let stats = self.creature_stats(permanent);
        let (power, toughness) = stats.map_or((None, None), |stats| {
            (Some(stats.power), Some(stats.toughness))
        });
        let flying = self.has_flying(permanent);
        // A face-down permanent's mechanism-owned body is public information
        // and its physical card is private: its controller may look at the
        // card, and nobody else may.
        let characteristics = match permanent.face_down {
            Some(face_down) if permanent.controller != viewer => {
                ObjectCharacteristics::face_down(face_down)
            }
            Some(_) => Self::unmasked_rules_source(permanent),
            None => Self::effective_rules_source(permanent),
        };
        PermanentObservation {
            id: permanent.card.id,
            characteristics,
            token: permanent.card.definition.is_token(),
            controller: permanent.controller,
            types,
            face_down: permanent.face_down.is_some(),
            physical_face: self.physical_face_observation(permanent),
            phased_out: self
                .phased_out
                .iter()
                .any(|phased| phased.card.id == permanent.card.id),
            chosen_creature_type: permanent.chosen_creature_type.clone(),
            chosen_basic_land_type: permanent.chosen_basic_land_type,
            chosen_card_name: permanent.chosen_card_name.clone(),
            tapped: permanent.tapped,
            power,
            toughness,
            damage: permanent.damage,
            loyalty: types
                .contains(CardType::Planeswalker)
                .then(|| permanent.counters(CounterKind::Loyalty)),
            loyalty_ability_used_this_turn: permanent.activated_loyalty_this_turn,
            attack_defender: permanent.attack_defender,
            attacking: permanent.attacking,
            blocked_this_combat: permanent.blocked,
            blocking: permanent.blocking.clone(),
            blocking_this_combat: permanent.is_blocking_this_combat(),
            attacking_band: permanent.attacking_band,
            flying,
            can_attack: stats.is_some() && self.can_attack(permanent),
            entered_this_turn: self.turns_started[permanent.controller.index()]
                == permanent.entered_controller_turn,
        }
    }

    pub fn observe(&self, viewer: PlayerId) -> PlayerObservation {
        let player = &self.players[viewer.index()];
        let opponent = &self.players[viewer.opponent().index()];
        PlayerObservation {
            viewer,
            turn: self.turn,
            active_turn: self.turns_started[self.active_player.index()],
            active_player: self.active_player,
            priority: self.priority,
            step: self.step,
            regular_combat_damage_pending: self.regular_combat_damage_pending(),
            life_totals: [self.players[0].life, self.players[1].life],
            poison_counters: [self.players[0].poison, self.players[1].poison],
            energy_counters: [self.players[0].energy, self.players[1].energy],
            monarch: self.monarch,
            mana_pools: [self.players[0].mana_pool, self.players[1].mana_pool],
            hand: player
                .hand
                .iter()
                .map(|card| (card.id, card.definition))
                .collect(),
            opponent_hand_size: opponent.hand.len(),
            last_seen_hand: self.last_seen_hands[viewer.index()].clone(),
            library_sizes: [self.players[0].library.len(), self.players[1].library.len()],
            revealed_library_top: self
                .player_rule_applies(viewer, crate::card::AppliedRuleDef::MayLookAtTopOfLibrary)
                .then(|| player.library.last().map(|card| (card.id, card.definition)))
                .flatten(),
            graveyards: [
                public_cards(&self.players[0].graveyard),
                public_cards(&self.players[1].graveyard),
            ],
            // A foretold card lies face down, so the opponent's is left out
            // of the list entirely and counted instead -- the same way a
            // hand is a size rather than a list. Your own are listed: you
            // know what you exiled.
            exiles: [
                self.observed_exile(PlayerId::One, viewer),
                self.observed_exile(PlayerId::Two, viewer),
            ],
            face_down_exile_sizes: [
                self.face_down_exile_size(PlayerId::One),
                self.face_down_exile_size(PlayerId::Two),
            ],
            // Phased-out permanents come last and carry a flag: they are
            // visible to both players, and only the rules treat them as
            // absent. Reconstruction relies on this order.
            battlefield: self
                .battlefield
                .iter()
                .chain(self.phased_out.iter())
                .map(|permanent| self.observe_permanent(permanent, viewer))
                .collect(),
            emblems: self.observed_emblems(),
            stack: self
                .stack
                .iter()
                .map(|object| StackObservation {
                    id: object.id,
                    kind: object.kind,
                    source: object.source,
                    ability: object.ability_origin(),
                    ability_text: object.ability_text().map(str::to_owned),
                    // A spell cast face down has its mechanism-owned public
                    // values; only its controller knows which card it is.
                    characteristics: match object.face_down {
                        Some(face_down) if object.controller != viewer => {
                            ObjectCharacteristics::face_down(face_down)
                        }
                        Some(_) | None => object.presentation(),
                    },
                    controller: object.controller,
                    counterable: self.can_be_countered(object),
                    signature: object.signature.clone(),
                    targets: object.declared_targets(),
                    chosen_permanents: object.chosen_permanents.clone(),
                    x: object.x(),
                })
                .collect(),
            decision: self.pending_decisions.first().and_then(|decision| {
                (decision.observation.visibility == DecisionVisibility::Public
                    || decision.observation.player == viewer)
                    .then(|| decision.observation.clone())
            }),
            result: self.result,
            legal_actions: self.legal_actions(viewer),
            checkpoint: self.checkpoint_json(viewer),
        }
    }

    /// Returns the exact ability currently represented by `origin` on
    /// `source`, including copied, intrinsic, and continuously granted
    /// battlefield abilities. Printed abilities activated from another zone
    /// are resolved in that zone's characteristic context.
    #[must_use]
    pub fn ability_for_origin(
        &self,
        source: GameObjectId,
        origin: AbilityOrigin,
    ) -> Option<AbilityDef> {
        if let Some(ongoing) = self
            .ongoing_effects
            .iter()
            .find(|ongoing| ongoing.source.object == source && ongoing.source.ability == origin)
        {
            return Some(ongoing.ability);
        }
        if let Some(permanent) = self
            .battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == source)
        {
            return self
                .find_effective_ability(permanent, |effective| effective.origin == origin)
                .map(|effective| effective.ability);
        }

        let (zone, card) = self.card_in_nonbattlefield_zone(source)?;
        let context = match zone {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return None,
        };
        self.find_printed_card_ability(card, &context, |effective| effective.origin == origin)
            .map(|effective| effective.ability)
    }
}
