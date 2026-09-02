// Leaving the battlefield: the replacements that can redirect an exit, the
// batch that commits one, and what a permanent's departure means for the rest
// of the turn.
//
// Split out of `battlefield.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

use super::{PlayerRelation, TriggerEventObject};
use crate::CharacteristicContext;

/// One permanent on its way off the battlefield, as the exit batch collects
/// it: what it was, what had damaged it, where it is going, which counter it
/// will bring it back, and which face it was showing.
type ExitingPermanent = (
    GameObjectId,
    super::BattlefieldExitSnapshot,
    Vec<GameObjectId>,
    BattlefieldExitDestination,
    Option<CounterKind>,
    CardPartId,
);

/// One permanent that has left the battlefield, with everything the events
/// and the follow-up moves need to read about it: what it was as it left,
/// what had damaged it, where it went, which counter brings it back, and which
/// face it was presenting.
type RemovedBattlefieldObject = (
    Permanent,
    BattlefieldExitSnapshot,
    Vec<GameObjectId>,
    BattlefieldExitDestination,
    Option<CounterKind>,
    CardPartId,
);

/// Where a leaving permanent's card is going, and what the replacement that
/// sent it there put on it. The counter travels with the destination because
/// it is part of the same replaced move: what arrives is a new object, so
/// nothing afterwards could name it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BattlefieldExitDestination {
    zone: ZoneKind,
    placement: ZonePlacement,
    counters: Option<(CounterKind, u16)>,
}

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
                if !replacement.source_zones.contains(&ZoneKind::Battlefield)
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
        // An effect object is not on the battlefield and is in no zone at
        // all, so nothing about where it sits gates it: it applies for as
        // long as it lasts.
        for ongoing in &self.ongoing_effects {
            let ability = ongoing.ability;
            let DeclarativeAbilityDef::Replacement(replacement) = ability.definition else {
                continue;
            };
            let Some(effect) = ability.declarative_replacement() else {
                continue;
            };
            replacements.push(FrozenZoneMoveReplacement {
                source: ongoing.source,
                controller: ongoing.controller,
                presentation: ongoing.presentation,
                text: ability.text,
                replacement,
                effect,
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
        self.queue_battlefield_exit_ordering(batch);
    }

    /// After replacements have finalized the simultaneous event, ask each
    /// owner to arrange two or more nontoken cards headed to the same
    /// position in their library (CR 401.4). Players make these choices in
    /// APNAP order while every permanent is still on the battlefield.
    fn queue_battlefield_exit_ordering(&mut self, batch: PendingBattlefieldExitBatch) {
        let mut groups = Vec::new();
        for owner in [self.active_player, self.active_player.opponent()] {
            let mut placements = Vec::new();
            for proposed in &batch.moves {
                let Some(permanent) = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == proposed.object)
                else {
                    continue;
                };
                if !proposed.replaced_with_nothing
                    && proposed.destination == ZoneKind::Library
                    && permanent.card.owner == owner
                    && !permanent.card.definition.is_token()
                    && !placements.contains(&proposed.placement)
                {
                    placements.push(proposed.placement);
                }
            }
            for placement in placements {
                let group = batch
                    .moves
                    .iter()
                    .filter(|proposed| {
                        !proposed.replaced_with_nothing
                            && proposed.destination == ZoneKind::Library
                            && proposed.placement == placement
                            && self.battlefield.iter().any(|permanent| {
                                permanent.card.id == proposed.object
                                    && permanent.card.owner == owner
                                    && !permanent.card.definition.is_token()
                            })
                    })
                    .map(|proposed| proposed.object)
                    .collect::<Vec<_>>();
                if group.len() > 1 {
                    groups.push(group);
                }
            }
        }
        self.queue_next_battlefield_exit_order(batch, groups);
    }

    fn queue_next_battlefield_exit_order(
        &mut self,
        batch: PendingBattlefieldExitBatch,
        mut groups: Vec<Vec<GameObjectId>>,
    ) {
        if groups.is_empty() {
            self.commit_battlefield_exit_batch(batch);
            return;
        }
        let group = groups.remove(0);
        let Some((owner, placement)) = group.first().and_then(|first| {
            let owner = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *first)?
                .card
                .owner;
            let placement = batch
                .moves
                .iter()
                .find(|proposed| proposed.object == *first)?
                .placement;
            Some((owner, placement))
        }) else {
            self.queue_next_battlefield_exit_order(batch, groups);
            return;
        };
        let options = group
            .iter()
            .enumerate()
            .filter_map(|(index, object)| {
                let permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *object)?;
                Some(DecisionOption {
                    id: u32::try_from(index).ok()?,
                    label: self
                        .presentation_name(Self::effective_rules_source(permanent))
                        .unwrap_or_else(|| "Card".into())
                        .into_owned(),
                    card: Some((*object, Self::effective_rules_source(permanent))),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Battlefield,
                })
            })
            .collect::<Vec<_>>();
        let count = options.len();
        self.queue_decision(
            owner,
            Self::library_order_prompt(placement),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            count..=count,
            false,
            options,
            DecisionContinuation::BattlefieldExitOrder {
                batch,
                remaining: groups,
            },
        );
        if let Some(decision) = self.pending_decisions.last_mut() {
            decision.observation.order_semantics = Some(DecisionOrderSemantics::Resolution);
        }
    }

    pub(super) fn complete_battlefield_exit_order(
        &mut self,
        mut batch: PendingBattlefieldExitBatch,
        remaining: Vec<Vec<GameObjectId>>,
        offered: &[DecisionOption],
        answer: &[u32],
    ) {
        let ordered = answer
            .iter()
            .filter_map(|chosen| offered.iter().find(|option| option.id == *chosen))
            .filter_map(|option| option.card.map(|(card, _)| card))
            .collect::<Vec<_>>();
        let positions = batch
            .moves
            .iter()
            .enumerate()
            .filter(|(_, proposed)| ordered.contains(&proposed.object))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let ordered_moves = ordered
            .iter()
            .filter_map(|object| {
                batch
                    .moves
                    .iter()
                    .find(|proposed| proposed.object == *object)
                    .cloned()
            })
            .collect::<Vec<_>>();
        for (position, proposed) in positions.into_iter().zip(ordered_moves) {
            batch.moves[position] = proposed;
        }
        self.queue_next_battlefield_exit_order(batch, remaining);
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
                        // it enters, and hand or library size as a draw would
                        // happen; none is a question about leaving.
                        ReplacementConditionDef::SourceCastWith(_)
                        | ReplacementConditionDef::SourcePaidAdditionalCost(_)
                        | ReplacementConditionDef::SourceNotCastFrom(_)
                        | ReplacementConditionDef::ControllerHandAtMost(_)
                        | ReplacementConditionDef::ControllerLibraryEmpty => false,
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
                batch.moves[move_index].placement = ZonePlacement::Top;
            }
            ReplacementEffectDef::Perform(effect) => {
                self.perform_battlefield_exit_replacement_effect(context, *effect);
            }
            ReplacementEffectDef::PlaceCountersOnMovedObject { kind, amount } => {
                batch.moves[move_index].counters = Some((kind, amount));
            }
            ReplacementEffectDef::ModifyBattlefieldEntry(_)
            | ReplacementEffectDef::MultiplyEventAmount(_)
            | ReplacementEffectDef::AddToEventAmount(_)
            | ReplacementEffectDef::Choose(_)
            | ReplacementEffectDef::LookAtHand(_)
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
            cast: None,
            face_down: None,
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
    fn battlefield_exit_events(
        removed: &[RemovedBattlefieldObject],
        after: &[Option<TriggerEventObject>],
    ) -> Vec<CommittedTriggerEvent> {
        let mut events = removed
            .iter()
            .zip(after)
            .map(
                |((_, snapshot, damage_sources, to, _, _), after)| {
                    CommittedTriggerEvent::ZoneChanged {
                    before: Some(snapshot.object.clone()),
                    after: after.clone(),
                    from: ZoneKind::Battlefield,
                    to: to.zone,
                    damage_sources: damage_sources.clone(),
                    }
                },
            )
            .collect::<Vec<_>>();
        let died = removed
            .iter()
            .filter(|(_, _, _, to, _, _)| to.zone == ZoneKind::Graveyard)
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
                destination.zone == ZoneKind::Graveyard && snapshot.object.types.is_creature()
            })
            .count();
        self.creature_died_this_turn |= died > 0;
        self.creatures_died_this_turn = self
            .creatures_died_this_turn
            .saturating_add(u16::try_from(died).unwrap_or(u16::MAX));
    }

    /// Installs every destination object for one simultaneous exit batch and
    /// returns the post-move snapshots in the same order as `removed`.
    fn install_battlefield_exit_destinations(
        &mut self,
        removed: &[RemovedBattlefieldObject],
    ) -> Vec<Option<TriggerEventObject>> {
        let mut after = Vec::with_capacity(removed.len());
        let mut library_arrivals = Vec::new();
        for (permanent, _, _, to, _, _) in removed {
            let exit = match to.zone {
                ZoneKind::Exile => BattlefieldExit::Exile,
                ZoneKind::Graveyard => BattlefieldExit::Graveyard,
                ZoneKind::Hand => BattlefieldExit::Hand,
                ZoneKind::Library => match to.placement {
                    ZonePlacement::Bottom => BattlefieldExit::LibraryBottom,
                    ZonePlacement::Top | ZonePlacement::FromTop(_) => BattlefieldExit::LibraryTop,
                },
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported battlefield-exit replacement destination")
                }
            };
            self.record_battlefield_exit(permanent, exit);
            // 111.7: a token that leaves the battlefield ceases to exist. The
            // exit and everything watching for it still happened.
            if permanent.card.definition.is_token() {
                after.push(None);
                continue;
            }
            let owner = permanent.card.owner;
            let (mut card, _zone_change) = self.zone_change_card(
                permanent
                    .card
                    .clone()
                    .into_card()
                    .expect("a nontoken permanent is backed by a card definition"),
            );
            // The card is a new object in its new zone, so the counter goes
            // on after the identity change rather than before it.
            if let Some((kind, amount)) = to.counters {
                card.add_counters(kind, amount);
            }
            let context = match to.zone {
                ZoneKind::Library => CharacteristicContext::Library,
                ZoneKind::Hand => CharacteristicContext::Hand,
                ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                ZoneKind::Exile => CharacteristicContext::Exile,
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported battlefield-exit replacement destination")
                }
            };
            after.push(self.printed_trigger_event_object(
                card.id,
                card.definition,
                owner,
                &context,
            ));
            match to.zone {
                ZoneKind::Exile => self.players[owner.index()].exile.push(card),
                ZoneKind::Graveyard => self.players[owner.index()].graveyard.push(card),
                ZoneKind::Hand => self.players[owner.index()].hand.push(card),
                ZoneKind::Library => library_arrivals.push((owner, to.placement, card)),
                ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                    unreachable!("unsupported battlefield-exit replacement destination")
                }
            }
        }
        self.install_library_arrivals(library_arrivals);
        after
    }

    fn install_library_arrivals(
        &mut self,
        mut arrivals: Vec<(PlayerId, ZonePlacement, CardInstance)>,
    ) {
        while let Some((owner, placement, _)) = arrivals.first().cloned() {
            let mut cards = Vec::new();
            let mut index = 0;
            while index < arrivals.len() {
                if arrivals[index].0 == owner && arrivals[index].1 == placement {
                    cards.push(arrivals.remove(index).2);
                } else {
                    index += 1;
                }
            }
            for card in cards.into_iter().rev() {
                let library = &mut self.players[owner.index()].library;
                let index = placement.library_index(library.len());
                library.insert(index, card);
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
                            BattlefieldExitDestination {
                                zone: proposed.destination,
                                placement: proposed.placement,
                                counters: proposed.counters,
                            },
                            self.returns_from_death_with(permanent),
                            permanent.presented,
                        )
                    })
            })
            .collect::<Vec<_>>();
        let moved_to_graveyard = exits
            .iter()
            .filter(|(_, _, _, destination, _, _)| destination.zone == ZoneKind::Graveyard)
            .map(|(object, _, _, _, _, _)| *object)
            .collect::<Vec<_>>();

        self.record_exits_for_the_turn(&exits);
        let mut removed = Vec::new();
        for (id, snapshot, damage_sources, destination, returns_with, presented) in exits {
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
                returns_with,
                presented,
            ));
        }

        // Finish the entire simultaneous move before checking destination
        // triggers. Each event can then name both the permanent that left and
        // the exact new card that arrived, while the listener snapshot above
        // still preserves every battlefield ability that existed beforehand.
        let after = self.install_battlefield_exit_destinations(&removed);

        let events = Self::battlefield_exit_events(&removed, &after);
        for (((_, _, _, destination, _, _), after), event) in
            removed.iter().zip(&after).zip(&events)
        {
            if destination.zone == ZoneKind::Graveyard
                && let Some(after) = after
                && let Some((_, card)) = self.card_in_nonbattlefield_zone(after.id)
            {
                self.extend_with_card_graveyard_arrival_trigger_listeners(
                    &mut listeners,
                    card,
                    event,
                );
            }
        }
        self.capture_battlefield_trigger_batch_from_snapshot(&listeners, &events);

        for (permanent, _, _, to, returns_with, presented) in removed {
            // Undying observes the creature as it died, then returns the card
            // from the graveyard as a fresh object under its owner's control.
            if to.zone == ZoneKind::Graveyard
                && let Some(counter) = returns_with
            {
                self.return_top_graveyard_card_with_counter(
                    permanent.card.owner,
                    presented,
                    counter,
                );
            }
        }

        if let Some(completion) = completion {
            self.resume_battlefield_exit_completion(*completion, &moved_to_graveyard);
        }
    }

    fn resume_battlefield_exit_completion(
        &mut self,
        completion: BattlefieldExitCompletion,
        moved_to_graveyard: &[GameObjectId],
    ) {
        match completion {
            BattlefieldExitCompletion::Completions(completions) => {
                self.resume_battlefield_exit_completions(completions, moved_to_graveyard);
            }
            BattlefieldExitCompletion::ResolveEffects {
                object,
                context,
                effects,
            } => self.resolve_effect_defs(effects, &object, &context),
            BattlefieldExitCompletion::DestroyFollowup {
                candidates,
                binding,
                object,
                context,
                effect,
            } => self.resume_destroy_followup(
                &candidates,
                binding,
                &object,
                context,
                effect,
                moved_to_graveyard,
            ),
            BattlefieldExitCompletion::FinishStackResolution { object, resolved } => {
                self.finish_stack_resolution(&object, resolved);
            }
            BattlefieldExitCompletion::SacrificeFollowup {
                followup,
                sacrificed,
            } => self.resolve_sacrifice_followup(&followup, sacrificed),
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
                *frozen,
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

    fn resume_battlefield_exit_completions(
        &mut self,
        completions: Vec<BattlefieldExitCompletion>,
        moved_to_graveyard: &[GameObjectId],
    ) {
        let mut completions = completions.into_iter();
        while let Some(completion) = completions.next() {
            let pending_before = self.pending_decisions.len();
            self.resume_battlefield_exit_completion(completion, moved_to_graveyard);
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

    fn resume_destroy_followup(
        &mut self,
        candidates: &[GameObjectId],
        binding: Binding,
        object: &StackObject,
        mut context: EffectResolutionContext,
        effect: ScopedEffect,
        moved_to_graveyard: &[GameObjectId],
    ) {
        context.bind_object_group(
            binding,
            moved_to_graveyard
                .iter()
                .copied()
                .filter(|object| candidates.contains(object))
                .map(Target::Permanent)
                .collect(),
        );
        self.resolve_effect_def(effect, object, context);
    }
}
