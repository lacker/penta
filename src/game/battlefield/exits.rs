// Leaving the battlefield: the replacements that can redirect an exit, the
// batch that commits one, and what a permanent's departure means for the rest
// of the turn.
//
// Split out of `battlefield.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

use super::{BattlefieldTriggerListener, PlayerRelation};

/// One permanent on its way off the battlefield, as the exit batch collects
/// it: what it was, what had damaged it, where it is going, whether undying
/// will bring it back, and which face it was showing.
type ExitingPermanent = (
    GameObjectId,
    super::BattlefieldExitSnapshot,
    Vec<GameObjectId>,
    ZoneKind,
    bool,
    CardPartId,
);

/// One permanent that has left the battlefield, with everything the events
/// and the follow-up moves need to read about it: what it was as it left,
/// what had damaged it, where it went, whether undying applies, and which
/// face it was presenting.
type RemovedBattlefieldObject = (
    Permanent,
    BattlefieldExitSnapshot,
    Vec<GameObjectId>,
    ZoneKind,
    bool,
    CardPartId,
);

impl Game {
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
                let Some(effect) = ability.declarative_replacement() else {
                    return;
                };
                replacements.push(FrozenZoneMoveReplacement {
                    source: AbilitySourceRef {
                        object: permanent.card.id,
                        ability: effective.origin,
                    },
                    controller: permanent.controller,
                    presentation: Self::ability_presentation(
                        effective.origin,
                        Self::effective_rules_source(permanent),
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
                        self.apply_battlefield_exit_replacement(&mut batch, candidate);
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

    /// Whether the permanent about to leave is one this replacement's
    /// wording covers: whose graveyard it is headed for, and whether a token
    /// counts as the "card" the clause names.
    fn exiting_object_matches_owner_and_kind(
        &self,
        object: GameObjectId,
        controller: PlayerId,
        owner: PlayerRelation,
        tokens: bool,
    ) -> bool {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        else {
            return false;
        };
        // Token nature belongs to the physical object, not the characteristics
        // it may currently be copying.
        if !tokens && permanent.card.definition.is_token() {
            return false;
        }
        self.player_relation_matches(
            permanent.card.owner,
            owner,
            controller,
            TriggerContext::empty(),
        )
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
                        ReplacementConditionDef::CreatureDiedThisTurn => {
                            self.creature_died_this_turn
                        }
                        // How a permanent's spell was paid for is asked as
                        // it enters, and a hand size as a draw would happen;
                        // neither is a question about leaving.
                        ReplacementConditionDef::SourceCastWith(_)
                        | ReplacementConditionDef::ControllerHandAtMost(_) => false,
                    }
                } else {
                    true
                }
            })
            .filter(|replacement| match replacement.replacement.event {
                ReplacementEventDef::WouldMove {
                    from: None | Some(ZoneKind::Battlefield),
                    to,
                    cause: ZoneMoveCauseDef::Any,
                } => replacement.source.object == proposed.object && to == proposed.destination,
                ReplacementEventDef::AnyObjectWouldMove { to, owner, tokens } => {
                    to == proposed.destination
                        && self.exiting_object_matches_owner_and_kind(
                            proposed.object,
                            replacement.controller,
                            owner,
                            tokens,
                        )
                }
                _ => false,
            })
            .map(|replacement| ApplicableZoneMoveReplacement {
                move_index,
                context: ReplacementEffectContext {
                    source: replacement.source,
                    controller: replacement.controller,
                },
                presentation: replacement.presentation,
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
            .map_or_else(|| "this permanent".to_string(), std::borrow::Cow::into_owned);
        let options = candidates
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                Some(DecisionOption {
                    id: u32::try_from(index).ok()?,
                    label: candidate.text.to_string(),
                    card: Some((candidate.context.source.object, candidate.presentation)),
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
        replacement: &ApplicableZoneMoveReplacement,
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
        effect: ReplacementEffectDef,
    ) {
        match effect {
            ReplacementEffectDef::Sequence(effects) => {
                for effect in effects {
                    self.apply_battlefield_exit_effect(batch, move_index, context, *effect);
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
            ReplacementEffectDef::ModifyBattlefieldEntry(_)
            | ReplacementEffectDef::MultiplyEventAmount(_)
            | ReplacementEffectDef::AddToEventAmount(_)
            | ReplacementEffectDef::Choose(_)
            | ReplacementEffectDef::CopyEntering { .. }
            | ReplacementEffectDef::Conditional { .. }
            | ReplacementEffectDef::PayOr { .. } => {}
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
            cast_at_instant_speed: false,
            cast_from_zone: None,
            face_down: None,
            colors_of_mana_spent: crate::card::ColorSet::empty(),
            phyrexian_symbols_paid_with_life: 0,
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
    /// What a batch of exits means for the rest of the turn. Both facts are
    /// counted as they happen rather than read off the board afterwards: a
    /// creature that died and was reanimated, and a permanent that left and
    /// was replaced, both leave a battlefield that looks untouched.
    /// The events one exit batch publishes: one zone change per object, and
    /// one "they died" for the whole batch.
    ///
    /// The batched one carries only the graveyard half. A permanent exiled
    /// instead of dying did not die (CR 700.4), and the per-object zone
    /// change beside it already says where each one actually went.
    fn battlefield_exit_events(removed: &[RemovedBattlefieldObject]) -> Vec<CommittedTriggerEvent> {
        let mut events = removed
            .iter()
            .map(
                |(_, snapshot, damage_sources, to, _, _)| CommittedTriggerEvent::ZoneChanged {
                    object: snapshot.object.clone(),
                    from: ZoneKind::Battlefield,
                    to: *to,
                    damage_sources: damage_sources.clone(),
                },
            )
            .collect::<Vec<_>>();
        let died = removed
            .iter()
            .filter(|(_, _, _, to, _, _)| *to == ZoneKind::Graveyard)
            .map(|(_, snapshot, _, _, _, _)| snapshot.object.clone())
            .collect::<Vec<_>>();
        if !died.is_empty() {
            events.push(CommittedTriggerEvent::ObjectsDied { objects: died });
        }
        events
    }

    fn record_exits_for_the_turn(&mut self, exits: &[ExitingPermanent]) {
        for (_, snapshot, _, _, _, _) in exits {
            self.permanent_left_battlefield_this_turn[snapshot.object.controller.index()] = true;
        }
        let died = exits
            .iter()
            .filter(|(_, snapshot, _, destination, _, _)| {
                *destination == ZoneKind::Graveyard && snapshot.object.types.is_creature()
            })
            .count();
        self.creature_died_this_turn |= died > 0;
        self.creatures_died_this_turn = self
            .creatures_died_this_turn
            .saturating_add(u16::try_from(died).unwrap_or(u16::MAX));
    }

    fn extend_with_graveyard_arrival_listeners(
        &self,
        listeners: &mut Vec<BattlefieldTriggerListener>,
        removed: &[RemovedBattlefieldObject],
    ) {
        for (permanent, _, _, destination, _, _) in removed {
            if *destination == ZoneKind::Graveyard
                && let Some(card) = permanent.card.clone().into_card()
            {
                self.extend_with_card_graveyard_trigger_listeners(listeners, &card);
            }
        }
    }

    fn commit_battlefield_exit_batch(&mut self, batch: PendingBattlefieldExitBatch) {
        let completion = batch.completion;
        let mut listeners = self.battlefield_trigger_listeners();
        let exits = batch
            .moves
            .into_iter()
            .filter(|proposed| !proposed.replaced_with_nothing)
            .filter_map(|proposed| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == proposed.object)
                    .map(|permanent| {
                        let mut damage_sources = permanent.damage_sources.clone();
                        damage_sources.sort_unstable();
                        damage_sources.dedup();
                        (
                            proposed.object,
                            self.battlefield_exit_snapshot(permanent),
                            damage_sources,
                            proposed.destination,
                            self.has_undying(permanent)
                                && permanent.counters(CounterKind::PlusOnePlusOne) == 0,
                            permanent.presented,
                        )
                    })
            })
            .collect::<Vec<_>>();

        self.record_exits_for_the_turn(&exits);
        let mut removed = Vec::new();
        for (id, snapshot, damage_sources, destination, undying, presented) in exits {
            let index = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == id)
                .expect("a snapshotted battlefield object remains until its batch exits");
            let permanent = self.remove_battlefield_object(index, &snapshot.last_known);
            removed.push((
                permanent,
                snapshot,
                damage_sources,
                destination,
                undying,
                presented,
            ));
        }

        // A destination-zone trigger is checked after the move. Freeze the
        // printed graveyard abilities of cards about to arrive there so the
        // simultaneous event can be published before their new identities
        // are installed, just as the ordinary listener snapshot freezes the
        // abilities of permanents about to leave.
        self.extend_with_graveyard_arrival_listeners(&mut listeners, &removed);

        let events = Self::battlefield_exit_events(&removed);
        self.capture_battlefield_trigger_batch_from_snapshot(&listeners, &events);

        for ((permanent, snapshot, _, to, undying, presented), event) in
            removed.into_iter().zip(events)
        {
            let exit = match to {
                ZoneKind::Exile => BattlefieldExit::Exile,
                ZoneKind::Graveyard => BattlefieldExit::Graveyard,
                ZoneKind::Hand => BattlefieldExit::Hand,
                ZoneKind::Library => BattlefieldExit::LibraryTop,
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported battlefield-exit replacement destination")
                }
            };
            self.capture_custom_source_triggers(&permanent, &snapshot.abilities, &event);
            self.record_battlefield_exit(&permanent, exit);
            // 111.7: a token that leaves the battlefield ceases to exist. The
            // exit and everything watching for it still happened.
            if permanent.card.definition.is_token() {
                continue;
            }
            let owner = permanent.card.owner;
            let (card, _zone_change) = self.zone_change_card(
                permanent
                    .card
                    .into_card()
                    .expect("a nontoken permanent is backed by a card definition"),
            );
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
            } => self.resolve_effect_defs(effects, &object, &context),
            BattlefieldExitCompletion::FinishStackResolution { object, resolved } => {
                self.finish_stack_resolution(&object, resolved);
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
            } => self.complete_mana_ability(player, &activation, produced_mana),
            BattlefieldExitCompletion::ContinueSpellManaPayment {
                object,
                targets,
                object_payments,
                cost,
                x,
                purpose,
                plan,
                next_activation,
            } => self.continue_spell_mana_payment(
                *object,
                targets,
                object_payments,
                cost,
                x,
                purpose,
                plan,
                next_activation,
            ),
        }
    }
}
