use super::{
    AbilitySourceRef, ApplicableZoneMoveReplacement, BattlefieldExit, BattlefieldExitCompletion,
    CardInstance, CardPartId, CommittedTriggerEvent, CounterKind, DecisionContinuation,
    DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef,
    EffectDef, EffectRecipientDef, EntryCompletion, FrozenZoneMoveReplacement, Game, GameEvent,
    GameObjectId, KeywordAbility, PendingBattlefieldEntry, PendingBattlefieldExitBatch,
    PendingBattlefieldExitMove, Permanent, PlayerId, ReplacementConditionDef,
    ReplacementEffectContext, ReplacementEffectDef, ReplacementEventDef, ScopedEffect, StackObject,
    StackObjectKind, Step, Target, TargetSlotId, TriggerContext, TurnStepDef, ZoneKind,
    ZoneMoveCauseDef, ZonePlacement, remove_card,
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
                .map(|(_, card)| card)
                .or_else(|| {
                    self.players
                        .iter()
                        .flat_map(|player| player.outside_game.iter())
                        .find(|card| card.id == id)
                })
                .and_then(|card| self.catalog.get(card.definition))
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
                && !permanent.cannot_regenerate_this_turn
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

    /// Proposes one simultaneous batch of battlefield-to-graveyard moves. All
    /// effective replacement abilities are frozen before any member leaves;
    /// if CR 616 requires the affected object's controller to order two or
    /// more effects, the entire batch remains prospective behind that choice.
    pub(super) fn move_permanents_to_graveyard(&mut self, ids: &[GameObjectId]) {
        self.move_permanents_to_graveyard_then(ids, None);
    }

    pub(super) fn move_permanents_to_graveyard_then(
        &mut self,
        ids: &[GameObjectId],
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
                        destination: if permanent.exile_instead_of_dying {
                            ZoneKind::Exile
                        } else {
                            ZoneKind::Graveyard
                        },
                        replaced_with_nothing: false,
                        applied: Vec::new(),
                    })
            })
            .collect::<Vec<_>>();
        if moves.is_empty() {
            if let Some(completion) = completion {
                self.resume_battlefield_exit_completion(completion);
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

    /// Adds work to the exit choice created since `pending_before`. Effect
    /// sequences use this after interpreting one clause so they can leave the
    /// ordinary effect matcher intact while still suspending their tail.
    pub(super) fn defer_after_battlefield_exit(
        &mut self,
        pending_before: usize,
        completion: BattlefieldExitCompletion,
    ) -> bool {
        let Some(pending) = self.pending_decisions.get_mut(pending_before..) else {
            return false;
        };
        for pending in pending.iter_mut().rev() {
            let DecisionContinuation::BattlefieldExitReplacement { batch, .. } =
                &mut pending.continuation
            else {
                continue;
            };
            batch.completion = Some(Box::new(match batch.completion.take() {
                None => completion,
                Some(earlier) => BattlefieldExitCompletion::Completions(vec![*earlier, completion]),
            }));
            return true;
        }
        false
    }

    fn frozen_battlefield_zone_move_replacements(&self) -> Vec<FrozenZoneMoveReplacement> {
        let mut replacements = Vec::new();
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(replacement) = ability.definition else {
                    return;
                };
                if !ability.is_executable()
                    || !replacement.source_zones.contains(&ZoneKind::Battlefield)
                {
                    return;
                }
                let Some(effect) = ability.declarative_effect() else {
                    return;
                };
                replacements.push(FrozenZoneMoveReplacement {
                    source: AbilitySourceRef {
                        object: permanent.card.id,
                        ability: effective.origin,
                    },
                    controller: permanent.controller,
                    definition: Self::ability_presentation_definition(
                        effective.origin,
                        Self::effective_rules_source(permanent).0,
                    ),
                    text: ability.text,
                    replacement,
                    effect,
                });
            });
        }
        replacements
    }

    pub(super) fn continue_battlefield_exit_replacements(
        &mut self,
        mut batch: PendingBattlefieldExitBatch,
    ) {
        loop {
            let mut progressed = false;
            for move_index in 0..batch.moves.len() {
                let candidates = self.applicable_battlefield_exit_replacements(&batch, move_index);
                match candidates.as_slice() {
                    [] => {}
                    [candidate] => {
                        self.apply_battlefield_exit_replacement(&mut batch, *candidate);
                        progressed = true;
                        break;
                    }
                    _ => {
                        self.queue_battlefield_exit_replacement_choice(batch, candidates);
                        return;
                    }
                }
            }
            if !progressed {
                break;
            }
        }
        self.commit_battlefield_exit_batch(batch);
    }

    fn applicable_battlefield_exit_replacements(
        &self,
        batch: &PendingBattlefieldExitBatch,
        move_index: usize,
    ) -> Vec<ApplicableZoneMoveReplacement> {
        let proposed = &batch.moves[move_index];
        if proposed.replaced_with_nothing {
            return Vec::new();
        }
        batch
            .replacements
            .iter()
            .filter(|replacement| !proposed.applied.contains(&replacement.source))
            .filter(|replacement| {
                if let Some(condition) = replacement.replacement.condition {
                    match condition {
                        ReplacementConditionDef::SourceTapped => self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == replacement.source.object)
                            .is_some_and(|permanent| permanent.tapped),
                    }
                } else {
                    true
                }
            })
            .filter(|replacement| match replacement.replacement.event {
                ReplacementEventDef::WouldMove {
                    from: ZoneKind::Battlefield,
                    to,
                    cause: ZoneMoveCauseDef::Any,
                } => replacement.source.object == proposed.object && to == proposed.destination,
                ReplacementEventDef::AnyObjectWouldMove { to } => to == proposed.destination,
                _ => false,
            })
            .map(|replacement| ApplicableZoneMoveReplacement {
                move_index,
                context: ReplacementEffectContext {
                    source: replacement.source,
                    controller: replacement.controller,
                },
                definition: replacement.definition,
                text: replacement.text,
                effect: replacement.effect,
            })
            .collect()
    }

    fn queue_battlefield_exit_replacement_choice(
        &mut self,
        batch: PendingBattlefieldExitBatch,
        candidates: Vec<ApplicableZoneMoveReplacement>,
    ) {
        let move_index = candidates
            .first()
            .map_or(0, |candidate| candidate.move_index);
        let proposed = &batch.moves[move_index];
        let name = self
            .object_card_name(proposed.object)
            .unwrap_or("this permanent")
            .to_string();
        let options = candidates
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                Some(DecisionOption {
                    id: u32::try_from(index).ok()?,
                    label: candidate.text.to_string(),
                    card: Some((candidate.context.source.object, candidate.definition)),
                    members: Vec::new(),
                    ability_text: Some(candidate.text.to_string()),
                    zone: DecisionZone::Battlefield,
                })
            })
            .collect();
        self.queue_decision(
            proposed.controller,
            format!("Choose a replacement effect for {name}"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldExitReplacement { batch, candidates },
        );
    }

    pub(super) fn apply_battlefield_exit_replacement(
        &mut self,
        batch: &mut PendingBattlefieldExitBatch,
        replacement: ApplicableZoneMoveReplacement,
    ) {
        batch.moves[replacement.move_index]
            .applied
            .push(replacement.context.source);
        self.apply_battlefield_exit_effect(
            batch,
            replacement.move_index,
            replacement.context,
            replacement.effect,
        );
    }

    fn apply_battlefield_exit_effect(
        &mut self,
        batch: &mut PendingBattlefieldExitBatch,
        move_index: usize,
        context: ReplacementEffectContext,
        effect: EffectDef,
    ) {
        match effect {
            // Compatibility for already-authored replacements such as Rest in
            // Peace. Source names the prospective moving object here, so this
            // changes its event rather than moving the replacement source.
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone,
                ..
            } => batch.moves[move_index].destination = zone,
            EffectDef::Replacement(effect) => {
                self.apply_battlefield_exit_replacement_program(batch, move_index, context, effect);
            }
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.apply_battlefield_exit_effect(batch, move_index, context, *effect);
                }
            }
            // An ordinary operation authored directly in an older replacement
            // clause is still performed in the frozen source context.
            effect => self.perform_battlefield_exit_replacement_effect(context, effect),
        }
    }

    fn apply_battlefield_exit_replacement_program(
        &mut self,
        batch: &mut PendingBattlefieldExitBatch,
        move_index: usize,
        context: ReplacementEffectContext,
        effect: ReplacementEffectDef,
    ) {
        match effect {
            ReplacementEffectDef::Sequence(effects) => {
                for effect in effects {
                    self.apply_battlefield_exit_replacement_program(
                        batch, move_index, context, *effect,
                    );
                }
            }
            ReplacementEffectDef::ReplaceEventWithNothing => {
                batch.moves[move_index].replaced_with_nothing = true;
            }
            ReplacementEffectDef::MoveToZone(zone) => {
                batch.moves[move_index].destination = zone;
            }
            ReplacementEffectDef::Perform(effect) => {
                self.perform_battlefield_exit_replacement_effect(context, *effect);
            }
            ReplacementEffectDef::None
            | ReplacementEffectDef::ModifyBattlefieldEntry(_)
            | ReplacementEffectDef::Conditional { .. }
            | ReplacementEffectDef::OptionalPayment { .. } => {}
        }
    }

    fn perform_battlefield_exit_replacement_effect(
        &mut self,
        context: ReplacementEffectContext,
        effect: EffectDef,
    ) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == context.source.object)
        else {
            return;
        };
        let object = StackObject {
            id: permanent.card.id,
            kind: StackObjectKind::TriggeredAbility,
            card: permanent.card.clone(),
            source: Some(permanent.card.id),
            ability: None,
            controller: context.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            is_copy: false,
        };
        self.resolve_effect_def(
            ScopedEffect::primary(effect),
            &object,
            TriggerContext {
                object: Some(context.source.object),
                object_controller: Some(context.controller),
                ..TriggerContext::empty()
            },
        );
    }

    /// Commits a simultaneous batch after every replacement choice has
    /// reached a final event. Listener declarations and last-known
    /// characteristics are frozen before any member leaves, then all old
    /// object incarnations are retired before zone-change events are published.
    fn commit_battlefield_exit_batch(&mut self, batch: PendingBattlefieldExitBatch) {
        let completion = batch.completion;
        let listeners = self.battlefield_trigger_listeners();
        let exits = batch
            .moves
            .into_iter()
            .filter(|proposed| !proposed.replaced_with_nothing)
            .filter_map(|proposed| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == proposed.object)
                    .map(|permanent| {
                        (
                            proposed.object,
                            self.battlefield_exit_snapshot(permanent),
                            permanent.damage_sources.clone(),
                            proposed.destination,
                            self.has_undying(permanent)
                                && permanent.counters(CounterKind::PlusOnePlusOne) == 0,
                            permanent.presented,
                        )
                    })
            })
            .collect::<Vec<_>>();

        self.creature_died_this_turn |= exits.iter().any(|(_, snapshot, _, destination, _, _)| {
            *destination == ZoneKind::Graveyard && snapshot.object.types.is_creature()
        });
        for (_, snapshot, damage_sources, destination, _, _) in &exits {
            if *destination != ZoneKind::Graveyard {
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
        for (id, snapshot, _, destination, undying, presented) in exits {
            let index = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == id)
                .expect("a snapshotted battlefield object remains until its batch exits");
            let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
            removed.push((permanent, snapshot, destination, undying, presented));
        }

        for (permanent, snapshot, to, undying, presented) in removed {
            let exit = match to {
                ZoneKind::Exile => BattlefieldExit::Exile,
                ZoneKind::Graveyard => BattlefieldExit::Graveyard,
                ZoneKind::Hand => BattlefieldExit::Hand,
                ZoneKind::Library => BattlefieldExit::LibraryTop,
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported battlefield-exit replacement destination")
                }
            };
            let event = CommittedTriggerEvent::ZoneChanged {
                object: snapshot.object,
                from: ZoneKind::Battlefield,
                to,
            };
            self.capture_battlefield_triggers_from_snapshot(&listeners, &event);
            self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
            self.record_battlefield_exit(&permanent, exit);
            // 111.7: a token that leaves the battlefield ceases to exist. The
            // exit and everything watching for it still happened.
            if self.is_token(permanent.card.definition) {
                continue;
            }
            let owner = permanent.card.owner;
            let (card, _zone_change) = self.zone_change_card(permanent.card);
            match to {
                ZoneKind::Exile => self.players[owner.index()].exile.push(card),
                ZoneKind::Graveyard => self.players[owner.index()].graveyard.push(card),
                ZoneKind::Hand => self.players[owner.index()].hand.push(card),
                ZoneKind::Library => self.players[owner.index()].library.push(card),
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported battlefield-exit replacement destination")
                }
            }

            // Undying observes the creature as it died, then returns the card
            // from the graveyard as a fresh object under its owner's control.
            if to == ZoneKind::Graveyard && undying {
                self.return_top_graveyard_card_with_undying(owner, presented);
            }
        }

        if let Some(completion) = completion {
            self.resume_battlefield_exit_completion(*completion);
        }
    }

    fn resume_battlefield_exit_completion(&mut self, completion: BattlefieldExitCompletion) {
        match completion {
            BattlefieldExitCompletion::Completions(completions) => {
                let mut completions = completions.into_iter();
                while let Some(completion) = completions.next() {
                    let pending_before = self.pending_decisions.len();
                    self.resume_battlefield_exit_completion(completion);
                    let remaining = completions.as_slice();
                    if !remaining.is_empty()
                        && self.defer_after_battlefield_exit(
                            pending_before,
                            BattlefieldExitCompletion::Completions(remaining.to_vec()),
                        )
                    {
                        return;
                    }
                }
            }
            BattlefieldExitCompletion::ResolveEffects {
                object,
                context,
                effects,
            } => self.resolve_effect_defs(effects, &object, context),
            BattlefieldExitCompletion::FinishStackResolution { object, resolved } => {
                self.finish_stack_resolution(*object, resolved);
            }
            BattlefieldExitCompletion::SacrificeFollowup {
                followup,
                sacrificed,
            } => self.resolve_sacrifice_followup(&followup, sacrificed),
            BattlefieldExitCompletion::Balance {
                controller,
                phase,
                mut remaining,
            } => {
                if !remaining.is_empty() {
                    let next = remaining.remove(0);
                    self.queue_balance_task(controller, phase, next, remaining);
                } else if let Some(next) = phase.next() {
                    self.queue_balance_phase(controller, next);
                }
            }
            BattlefieldExitCompletion::CompleteSpellCast {
                object,
                targets,
                remaining_sacrifices,
            } => self.continue_spell_cast(*object, targets, remaining_sacrifices),
            BattlefieldExitCompletion::CompleteActivatedAbility {
                source,
                source_card,
                controller,
                frozen,
                targets,
                chosen_permanents,
                remaining_sacrifices,
            } => self.continue_activated_ability_costs(
                source,
                source_card,
                controller,
                frozen,
                targets,
                chosen_permanents,
                remaining_sacrifices,
            ),
            BattlefieldExitCompletion::CompleteManaAbility {
                player,
                activation,
                produced_mana,
            } => self.complete_mana_ability(player, activation, produced_mana),
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
