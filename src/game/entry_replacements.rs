use super::{
    AbilitySourceRef, ApplicableReplacement, BasicLandType, BattlefieldEntryModificationDef,
    ColorChoiceOperationDef, ColorSet, CommittedTriggerEvent, ConditionDef, ControlFlow,
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, DecisionZone,
    DeclarativeAbilityDef, EffectDef, EffectPaymentDef, EffectResolutionContext, EntryCompletion,
    Game, GameEvent, Mana, ManaColor, ObjectCountConditionDef, PendingBattlefieldEntry,
    PendingEvent, PendingReplacementEffect, Permanent, PlayerId, PlayerRelation, ReplaceableEvent,
    ReplacementChoiceDef, ReplacementConditionDef, ReplacementEffectContext, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, ResolvedEffectPayment, RetiredObject,
    ScopedEffect, StackObject, StackObjectKind, Target, TriggerContext, ValueDef, ZoneKind,
    public_cards,
};
use crate::CharacteristicContext;

mod discovery;
mod entry_copy;
mod entry_exile;
impl Game {
    pub(super) fn enqueue_battlefield_entry(&mut self, entry: PendingBattlefieldEntry) {
        self.pending_events.push_back(PendingEvent {
            event: ReplaceableEvent::BattlefieldEntry(entry),
            applied: Vec::new(),
            effects: Vec::new(),
        });
        self.continue_pending_events();
    }

    /// Advances prospective events until they either need a player's choice
    /// or can be committed. Replacements are rediscovered after every applied
    /// effect rather than captured once.
    pub(super) fn continue_pending_events(&mut self) {
        while self.pending_decisions.is_empty() {
            let Some(mut pending) = self.pending_events.pop_front() else {
                return;
            };

            if let Some(effect) = pending.effects.pop() {
                let Some(pending) = self.apply_pending_replacement_effect(pending, effect) else {
                    return;
                };
                self.pending_events.push_front(pending);
                continue;
            }

            let candidates = self.applicable_replacements(&pending);
            match candidates.as_slice() {
                [] => self.commit_pending_event(pending),
                [candidate] => {
                    let Some(pending) = self.prepare_entry_replacement(pending, candidate) else {
                        return;
                    };
                    self.pending_events.push_front(pending);
                }
                _ => {
                    let player = Self::pending_event_controller(&pending);
                    let name = self.pending_entry_name(&pending);
                    let options = candidates
                        .iter()
                        .enumerate()
                        .filter_map(|(index, candidate)| {
                            Some(DecisionOption {
                                id: u32::try_from(index).ok()?,
                                label: candidate.text.to_string(),
                                card: Some((
                                    candidate.context.source.object,
                                    candidate.presentation,
                                )),
                                members: Vec::new(),
                                ability_text: Some(candidate.text.to_string()),
                                zone: if self.battlefield.iter().any(|permanent| {
                                    permanent.card.id == candidate.context.source.object
                                }) {
                                    DecisionZone::Battlefield
                                } else {
                                    DecisionZone::None
                                },
                            })
                        })
                        .collect();
                    self.pending_events.push_front(pending);
                    self.queue_decision(
                        player,
                        format!("Choose a replacement effect for {name}"),
                        DecisionVisibility::Public,
                        DecisionPreference::Neutral,
                        1..=1,
                        false,
                        options,
                        DecisionContinuation::BattlefieldEntryReplacement { candidates },
                    );
                    return;
                }
            }
        }
    }

    /// Records one applicable replacement for this prospective event and
    /// either queues its exact operation or suspends behind its optional
    /// Accept/Decline choice. Recording happens before the choice so declining
    /// cannot rediscover and offer the same ability again.
    pub(super) fn prepare_entry_replacement(
        &mut self,
        mut pending: PendingEvent,
        candidate: &ApplicableReplacement,
    ) -> Option<PendingEvent> {
        pending.applied.push(candidate.context.source);
        if candidate.optional {
            let player = Self::pending_event_controller(&pending);
            let name = self.pending_entry_name(&pending);
            self.pending_events.push_front(pending);
            self.queue_optional_entry_replacement(
                player,
                &name,
                candidate.context,
                candidate.effect,
            );
            None
        } else {
            pending.effects.push(PendingReplacementEffect {
                context: candidate.context,
                effect: candidate.effect,
            });
            Some(pending)
        }
    }

    fn queue_optional_entry_replacement(
        &mut self,
        player: PlayerId,
        name: &str,
        context: ReplacementEffectContext,
        effect: ReplacementEffectDef,
    ) {
        self.queue_decision(
            player,
            format!("Apply the optional replacement for {name}?"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            Self::optional_entry_replacement_options(),
            DecisionContinuation::BattlefieldEntryOptional { context, effect },
        );
    }

    pub(super) fn optional_entry_replacement_options() -> Vec<DecisionOption> {
        [(0, "Decline"), (1, "Accept")]
            .into_iter()
            .map(|(id, label)| DecisionOption {
                id,
                label: label.into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect()
    }

    /// Resumes the exact typed replacement operation that was offered. The
    /// ability is already in the prospective event's applied set, so either
    /// answer continues without asking twice.
    pub(super) fn resume_optional_entry_replacement(
        &mut self,
        context: ReplacementEffectContext,
        effect: ReplacementEffectDef,
        options: &[u32],
    ) {
        let accepted = options.first().is_some_and(|option| *option == 1);
        if let Some(mut pending) = self.pending_events.pop_front() {
            if accepted {
                pending
                    .effects
                    .push(PendingReplacementEffect { context, effect });
            }
            self.pending_events.push_front(pending);
        }
        self.continue_pending_events();
    }

    pub(super) fn apply_pending_replacement_effect(
        &mut self,
        mut pending: PendingEvent,
        pending_effect: PendingReplacementEffect,
    ) -> Option<PendingEvent> {
        let PendingReplacementEffect { context, effect } = pending_effect;
        match effect {
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(choice)) => {
                let player = Self::pending_event_controller(&pending);
                self.pending_events.push_front(pending);
                self.queue_entry_scalar_choice(player, context, choice);
                None
            }
            ReplacementEffectDef::CopyEntering { object, exceptions } => {
                self.offer_entry_copy(pending, object, exceptions, context.source.ability)
            }
            // Any number of cards, so this one has to be asked rather than
            // recorded: the entry waits behind the choice and resumes with
            // the pile linked to the permanent that is arriving.
            ReplacementEffectDef::Choose(ReplacementChoiceDef::ExileMatchingFromGraveyard(
                predicate,
            )) => {
                let controller = Self::pending_event_controller(&pending);
                let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
                let entering = entry.permanent.card.id;
                let candidates = self.matching_graveyard_cards(controller, predicate, entering);
                if candidates.is_empty() {
                    return Some(pending);
                }
                let name = self.pending_entry_name(&pending);
                self.pending_events.push_front(pending);
                self.queue_entry_exile_choice(controller, &name, entering, &candidates);
                None
            }
            // With two players every relation this appears on names exactly
            // one candidate, so the choice is recorded rather than asked.
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(relation)) => {
                Some(self.record_chosen_entry_player(pending, relation))
            }
            ReplacementEffectDef::LookAtHand(relation) => {
                Some(self.record_entry_hand_look(pending, relation))
            }
            ReplacementEffectDef::Sequence(effects) => {
                Self::push_replacement_effects(&mut pending, context, effects);
                Some(pending)
            }
            ReplacementEffectDef::ModifyBattlefieldEntry(modification) => {
                Self::modify_pending_battlefield_entry(&mut pending, modification);
                Some(pending)
            }
            ReplacementEffectDef::Conditional {
                condition,
                if_true,
                if_false,
            } => {
                let branch = if self.condition_holds(&pending, context, condition) {
                    if_true
                } else {
                    if_false
                };
                Self::push_replacement_effects(&mut pending, context, branch);
                Some(pending)
            }
            ReplacementEffectDef::PayOr {
                payment,
                if_declined,
                ..
            } => {
                let payable = self
                    .pending_resolved_payment(&pending, context, payment)
                    .filter(|(player, payment)| self.can_pay_effect_payment(*player, *payment));
                if let Some((player, resolved)) = payable {
                    let name = self.pending_entry_name(&pending);
                    self.pending_events.push_front(pending);
                    self.queue_battlefield_entry_payment(player, &name, context, resolved, effect);
                    None
                } else {
                    Self::push_replacement_effects(&mut pending, context, if_declined);
                    Some(pending)
                }
            }
            // The permanent never arrives: the card goes where this says
            // instead, and nothing about it enters the battlefield.
            ReplacementEffectDef::MoveToZone(zone) => {
                let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                entry.redirected_to = Some(zone);
                Some(pending)
            }
            // These primitives belong to other prospective-event procedures
            // and cannot alter a battlefield-entry event.
            ReplacementEffectDef::ReplaceEventWithNothing
            | ReplacementEffectDef::Perform(_)
            | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
            | ReplacementEffectDef::MultiplyEventAmount(_)
            | ReplacementEffectDef::AddToEventAmount(_) => Some(pending),
        }
    }

    fn record_chosen_entry_player(
        &self,
        mut pending: PendingEvent,
        relation: PlayerRelation,
    ) -> PendingEvent {
        let controller = Self::pending_event_controller(&pending);
        let chosen = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            self.player_relation_matches(*player, relation, controller, TriggerContext::empty())
        });
        let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
        entry.permanent.chosen_player = chosen;
        pending
    }

    fn record_entry_hand_look(
        &mut self,
        pending: PendingEvent,
        relation: PlayerRelation,
    ) -> PendingEvent {
        let controller = Self::pending_event_controller(&pending);
        if let Some(seen) = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            self.player_relation_matches(*player, relation, controller, TriggerContext::empty())
        }) {
            self.last_seen_hands[controller.index()] =
                Some((seen, public_cards(&self.players[seen.index()].hand)));
        }
        pending
    }

    pub(super) fn push_replacement_effects(
        pending: &mut PendingEvent,
        context: ReplacementEffectContext,
        effects: &'static [ReplacementEffectDef],
    ) {
        pending.effects.extend(
            effects
                .iter()
                .rev()
                .copied()
                .map(|effect| PendingReplacementEffect { context, effect }),
        );
    }

    pub(super) const fn pending_event_controller(pending: &PendingEvent) -> PlayerId {
        match &pending.event {
            ReplaceableEvent::BattlefieldEntry(entry) => entry.permanent.controller,
        }
    }

    pub(super) fn pending_event_context(pending: &PendingEvent) -> TriggerContext {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        TriggerContext {
            object: Some(entry.permanent.card.id),
            zone_change_result: None,
            object_controller: Some(entry.permanent.controller),
            event_player: Some(entry.permanent.controller),
            amount: None,
            damaged_object: None,
            cast_from_zone: None,
        }
    }

    pub(super) fn pending_entry_name(&self, pending: &PendingEvent) -> String {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        self.presentation_name(Self::effective_rules_source(&entry.permanent))
            .map_or_else(|| "this permanent".to_owned(), std::borrow::Cow::into_owned)
    }

    fn pending_payment_object(
        &self,
        pending: &PendingEvent,
        context: ReplacementEffectContext,
    ) -> Option<StackObject> {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        let card = if entry.permanent.card.id == context.source.object {
            entry.permanent.card.clone()
        } else {
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == context.source.object)?
                .card
                .clone()
        };
        Some(StackObject {
            id: context.source.object,
            kind: StackObjectKind::TriggeredAbility,
            card,
            source: Some(context.source.object),
            ability: None,
            controller: context.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            cast_via_suspend: false,
            cast_at_instant_speed: false,
            cast_from_zone: None,
            face_down: None,
            colors_of_mana_spent: crate::card::ColorSet::empty(),
            phyrexian_symbols_paid_with_life: 0,
            is_copy: false,
        })
    }

    pub(super) fn pending_resolved_payment(
        &self,
        pending: &PendingEvent,
        context: ReplacementEffectContext,
        payment: EffectPaymentDef,
    ) -> Option<(PlayerId, ResolvedEffectPayment)> {
        let object = self.pending_payment_object(pending, context)?;
        let resolution = EffectResolutionContext::from(Self::pending_event_context(pending));
        let scoped = ScopedEffect::primary(EffectDef::None);
        let payers = self.effect_players(payment.payer, &object, &resolution, scoped);
        let [player] = payers.as_slice() else {
            return None;
        };
        let resolved = self.resolved_effect_payment(payment.cost, &object, &resolution, scoped);
        Some((*player, resolved))
    }

    pub(super) fn queue_battlefield_entry_payment(
        &mut self,
        player: PlayerId,
        name: &str,
        context: ReplacementEffectContext,
        resolved: ResolvedEffectPayment,
        definition: ReplacementEffectDef,
    ) {
        let payment_label = Self::effect_payment_label(resolved);
        let options = self.payment_options(player, resolved, true, "Do not pay");
        self.queue_decision(
            player,
            format!("{payment_label} as {name} enters the battlefield?"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryPayment {
                context,
                player,
                payment: resolved,
                definition,
            },
        );
    }

    pub(super) fn condition_holds(
        &self,
        pending: &PendingEvent,
        context: ReplacementEffectContext,
        condition: ConditionDef,
    ) -> bool {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        match condition {
            ConditionDef::All(conditions) => conditions
                .iter()
                .all(|condition| self.condition_holds(pending, context, *condition)),
            ConditionDef::Exists(query) => self.any_object_matches_query_with_prospective(
                query,
                context.controller,
                context.source.object,
                Self::pending_event_context(pending),
                Some(&entry.permanent),
            ),
            ConditionDef::ObjectCount(counting) => {
                let ObjectCountConditionDef {
                    query,
                    comparison,
                    amount,
                } = *counting;
                let mut count = 0_usize;
                let _ = self.visit_objects_matching_query_with_prospective(
                    query,
                    context.controller,
                    context.source.object,
                    Self::pending_event_context(pending),
                    Some(&entry.permanent),
                    None,
                    |_| {
                        count += 1;
                        std::ops::ControlFlow::Continue(())
                    },
                );
                crate::game::effect_support::compare(&count, comparison, &usize::from(amount))
            }
            // Read as the permanent enters, which is the only moment the
            // clause asks about.
            ConditionDef::ControllerTurnsTakenAtMost(turns) => {
                self.turns_started[context.controller.index()] <= u32::from(turns)
            }
        }
    }

    /// Reads a replacement ability's own condition against the still-
    /// prospective entry. This is deliberately evaluated during replacement
    /// discovery, not when the spell was cast, so morbid sees intervening
    /// deaths and a source condition sees the entry as modified so far.
    pub(super) fn entry_replacement_condition_holds(
        &self,
        pending: &PendingEvent,
        source: crate::GameObjectId,
        condition: ReplacementConditionDef,
    ) -> bool {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        match condition {
            ReplacementConditionDef::SourceTapped => {
                if entry.permanent.card.id == source {
                    entry.permanent.tapped
                } else {
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                        .is_some_and(|permanent| permanent.tapped)
                }
            }
            // The entering permanent already knows how its spell was paid
            // for: the resolution recorded it before enqueueing the entry.
            ReplacementConditionDef::SourceCastWith(kind) => {
                entry.permanent.card.id == source && entry.permanent.cast_alternative == Some(kind)
            }
            ReplacementConditionDef::SourcePaidAdditionalCost(cost) => {
                entry.permanent.card.id == source
                    && entry
                        .permanent
                        .cast_additional_costs
                        .get(cost.index())
                        .is_some_and(|payments| *payments > 0)
            }
            ReplacementConditionDef::SourceNotCastFrom(zone) => {
                entry.permanent.card.id == source
                    && entry
                        .permanent
                        .cast_from_zone
                        .is_none_or(|from| from.zone() != zone)
            }
            ReplacementConditionDef::CreatureDiedThisTurn => self.creature_died_this_turn,
            // Hand and library sizes are facts about a draw, so nothing
            // about an entry asks them.
            ReplacementConditionDef::ControllerHandAtMost(_)
            | ReplacementConditionDef::ControllerLibraryEmpty => false,
        }
    }

    pub(super) fn modify_pending_battlefield_entry(
        pending: &mut PendingEvent,
        modification: BattlefieldEntryModificationDef,
    ) {
        let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
        Self::modify_battlefield_entry_permanent(&mut entry.permanent, modification);
    }

    pub(super) fn modify_battlefield_entry_permanent(
        permanent: &mut Permanent,
        modification: BattlefieldEntryModificationDef,
    ) {
        match modification {
            BattlefieldEntryModificationDef::Tapped => permanent.tapped = true,
            BattlefieldEntryModificationDef::AddCounters { kind, amount } => {
                permanent.add_counters(kind, amount);
            }
            BattlefieldEntryModificationDef::AddCastXCounters { kind } => {
                let amount = permanent.cast_x;
                permanent.add_counters(kind, amount);
            }
            BattlefieldEntryModificationDef::AddCountersValue { kind, amount } => {
                let amount = entry_value(permanent, amount)
                    .expect("catalog validation rejects unsupported entry values")
                    .clamp(0, i32::from(u16::MAX));
                permanent.add_counters(kind, u16::try_from(amount).unwrap_or_default());
            }
            BattlefieldEntryModificationDef::AddColorsSpentCounters { kind } => {
                let amount = permanent.cast_colors;
                permanent.add_counters(kind, amount);
            }
        }
    }

    /// Entry replacements the prospective permanent carries itself.
    pub(super) fn prospective_source_replacements(
        &self,
        pending: &PendingEvent,
        entry: &PendingBattlefieldEntry,
    ) -> Vec<ApplicableReplacement> {
        let mut candidates = Vec::new();
        let result = self.visit_effective_replacement_abilities_with_prospective(
            &entry.permanent,
            Some(&entry.permanent),
            |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return ControlFlow::Continue(());
                };
                if !ability.is_executable() {
                    return ControlFlow::Continue(());
                }
                let Some(effect) = ability.declarative_replacement() else {
                    return ControlFlow::Continue(());
                };
                if definition.event != ReplacementEventDef::SourceEntersBattlefield {
                    return ControlFlow::Continue(());
                }
                let source = AbilitySourceRef {
                    object: entry.permanent.card.id,
                    ability: effective.origin,
                };
                if pending.applied.contains(&source)
                    || definition.condition.is_some_and(|condition| {
                        !self.entry_replacement_condition_holds(pending, source.object, condition)
                    })
                {
                    return ControlFlow::Continue(());
                }
                candidates.push(ApplicableReplacement {
                    context: ReplacementEffectContext {
                        source,
                        controller: entry.permanent.controller,
                    },
                    presentation: Self::ability_presentation(
                        effective.origin,
                        Self::effective_rules_source(&entry.permanent),
                    ),
                    text: ability.text,
                    optional: definition.optional,
                    effect,
                });
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        candidates
    }

    pub(super) fn applicable_replacements(
        &self,
        pending: &PendingEvent,
    ) -> Vec<ApplicableReplacement> {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;

        // Use the characteristics the object would have on the battlefield,
        // including copy and existing continuous effects. In particular,
        // Blood Moon removes a nonbasic land's printed as-enters abilities.
        let (grants_source_replacement, may_supply_external_replacement) =
            self.battlefield_entry_replacement_possibilities();
        let may_supply_source_replacement = grants_source_replacement
            || self.prospective_permanent_may_supply_source_entry_replacement(&entry.permanent);
        let mut candidates = if may_supply_source_replacement {
            self.prospective_source_replacements(pending, entry)
        } else {
            Vec::new()
        };

        if !may_supply_external_replacement {
            return candidates;
        }

        let entering_object =
            self.trigger_event_object_with_prospective(&entry.permanent, &entry.permanent);
        let event_context = Self::pending_event_context(pending);
        for source_permanent in &self.battlefield {
            let result = self.visit_effective_replacement_abilities_with_prospective(
                source_permanent,
                None,
                |effective| {
                    let ability = effective.ability;
                    let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                        return ControlFlow::Continue(());
                    };
                    if !ability.is_executable()
                        || !definition.source_zones.contains(&ZoneKind::Battlefield)
                    {
                        return ControlFlow::Continue(());
                    }
                    let ReplacementEventDef::ObjectEntersBattlefield {
                        object,
                        controller,
                        cast,
                    } = definition.event
                    else {
                        return ControlFlow::Continue(());
                    };
                    // A permanent spell that resolves arrives from the stack,
                    // and nothing else does, so that is what "was cast" reads.
                    if cast.is_some_and(|expected| (entry.from == ZoneKind::Stack) != expected) {
                        return ControlFlow::Continue(());
                    }
                    if !self.trigger_object_matches(
                        object,
                        &entering_object,
                        source_permanent.card.id,
                        false,
                    ) || !self.player_relation_matches(
                        entry.permanent.controller,
                        controller,
                        source_permanent.controller,
                        event_context,
                    ) {
                        return ControlFlow::Continue(());
                    }
                    let Some(effect) = ability.declarative_replacement() else {
                        return ControlFlow::Continue(());
                    };
                    let source = AbilitySourceRef {
                        object: source_permanent.card.id,
                        ability: effective.origin,
                    };
                    if !pending.applied.contains(&source)
                        && definition.condition.is_none_or(|condition| {
                            self.entry_replacement_condition_holds(
                                pending,
                                source.object,
                                condition,
                            )
                        })
                    {
                        candidates.push(ApplicableReplacement {
                            context: ReplacementEffectContext {
                                source,
                                controller: source_permanent.controller,
                            },
                            presentation: Self::ability_presentation(
                                effective.origin,
                                Self::effective_rules_source(source_permanent),
                            ),
                            text: ability.text,
                            optional: definition.optional,
                            effect,
                        });
                    }
                    ControlFlow::Continue(())
                },
            );
            debug_assert!(result.is_continue());
        }
        candidates
    }

    pub(super) fn commit_pending_event(&mut self, pending: PendingEvent) {
        match pending.event {
            ReplaceableEvent::BattlefieldEntry(entry) => self.commit_battlefield_entry(entry),
        }
    }

    /// Finishes an entry that was replaced with a move somewhere else. The
    /// card completes the zone change it was already making, so it is the
    /// card that arrives rather than the permanent it was going to be: no
    /// enters-the-battlefield trigger sees it, and nothing it would have
    /// brought with it happens.
    fn commit_redirected_entry(&mut self, entry: PendingBattlefieldEntry, zone: ZoneKind) {
        let owner = entry.permanent.card.owner;
        let Some(card) = entry.permanent.card.into_card() else {
            // A token whose entry is redirected leaves the battlefield-entry
            // process and then ceases to exist; it never becomes a card in the
            // destination zone.
            return;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        match zone {
            ZoneKind::Graveyard => self.put_card_into_graveyard(owner, card),
            ZoneKind::Exile => self.players[owner.index()].exile.push(card),
            ZoneKind::Hand => self.players[owner.index()].hand.push(card),
            ZoneKind::Library => self.players[owner.index()].library.push(card),
            // Every other destination would be the entry this replaced.
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {}
        }
    }

    /// Raises one permanent's arrival, or holds it back until the rest of
    /// its batch has arrived. Everything a batch does before this point is
    /// per permanent -- replacements are applied to each one on its own (CR
    /// 614.12) -- and only what watches the arrivals waits.
    fn capture_entry_event(&mut self, event: CommittedTriggerEvent) {
        if let Some(batch) = self.entry_event_batch.as_mut() {
            batch.push(event);
            return;
        }
        self.capture_battlefield_triggers(&event);
    }

    /// Runs `enter`, holding back the arrivals it causes until it is done,
    /// so everything it puts onto the battlefield is seen by the others.
    ///
    /// Nested batches join the one already open: a replacement that puts a
    /// second permanent onto the battlefield during a batch is part of the
    /// same arrival as far as anything watching is concerned.
    pub(in crate::game) fn entering_together(&mut self, enter: impl FnOnce(&mut Self)) {
        if self.entry_event_batch.is_some() {
            enter(self);
            return;
        }
        self.entry_event_batch = Some(Vec::new());
        enter(self);
        let batch = self.entry_event_batch.take().unwrap_or_default();
        for event in batch {
            self.capture_battlefield_triggers(&event);
        }
    }

    pub(super) fn commit_battlefield_entry(&mut self, mut entry: PendingBattlefieldEntry) {
        if let Some(zone) = entry.redirected_to {
            self.commit_redirected_entry(entry, zone);
            return;
        }
        let prospective = entry.permanent.card.id;
        if entry.completion != EntryCompletion::Setup
            && let Some(card) = entry.permanent.card.clone().into_card()
        {
            let (card, _zone_change) = self.zone_change_card(card);
            entry.permanent.card = card.into();
        }
        // A permanent takes a fresh identity as it actually arrives, so
        // anything linked to it while the entry was still prospective has to
        // be re-pointed at the object that ended up on the battlefield.
        if prospective != entry.permanent.card.id {
            let arrived = entry.permanent.card.id;
            for (source, _) in &mut self.linked_exiles {
                if *source == prospective {
                    *source = arrived;
                }
            }
        }
        entry.permanent.timestamp = self.allocate_continuous_effect_timestamp();
        let permanent_id = entry.permanent.card.id;
        let definition = entry.permanent.card.definition.card_definition();
        self.battlefield.push(entry.permanent);

        if let EntryCompletion::AttachSource { source } = entry.completion {
            self.try_attach(source, permanent_id);
        }

        // The other direction: the Equipment is what arrived, and the host
        // was here all along.
        if let EntryCompletion::AttachToHost { host } = entry.completion {
            self.try_attach(permanent_id, host);
        }

        if let EntryCompletion::Attacking { defender } = entry.completion
            && let Some(permanent) = self
                .battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == permanent_id)
        {
            // It was never declared, so it does not count as having been
            // declared -- but everything else about it is an attacker.
            permanent.attacking = true;
            permanent.attack_defender = Some(defender);
            permanent.attacked_this_turn = true;
            permanent.attacks_this_turn = permanent.attacks_this_turn.saturating_add(1);
        }

        if let EntryCompletion::LandPlayed { player } = entry.completion {
            self.events.push(GameEvent::LandPlayed {
                player,
                card: permanent_id,
                definition: definition.expect("a played land is a card"),
            });
        }

        let entered = self
            .battlefield
            .last()
            .expect("a committed battlefield entry is present");
        let entered_event = self.trigger_event_object(entered);
        let before_event = if prospective == permanent_id {
            None
        } else {
            match self.retired_objects.get(&prospective) {
                Some(RetiredObject::Stack(stack)) => self.stack_object_event_object(stack),
                Some(RetiredObject::Card(card)) => {
                    let context = match entry.from {
                        ZoneKind::Library => Some(CharacteristicContext::Library),
                        ZoneKind::Hand => Some(CharacteristicContext::Hand),
                        ZoneKind::Graveyard => Some(CharacteristicContext::Graveyard),
                        ZoneKind::Exile => Some(CharacteristicContext::Exile),
                        // A stack predecessor is represented by the arm above;
                        // the remaining zones do not hold cards that enter.
                        ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => None,
                    };
                    context.and_then(|context| {
                        self.printed_trigger_event_object(
                            card.id,
                            card.definition,
                            card.owner,
                            &context,
                        )
                    })
                }
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    Some(self.trigger_event_object(permanent))
                }
                None => None,
            }
        };
        // Raised before the entry below, since the play is what caused it:
        // a clause about playing a land reads the land that was played.
        if let EntryCompletion::LandPlayed { player } = entry.completion {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::LandPlayed {
                player,
                object: entered_event.clone(),
            });
        }
        self.capture_entry_event(CommittedTriggerEvent::ZoneChanged {
            before: before_event,
            after: Some(entered_event),
            from: entry.from,
            to: ZoneKind::Battlefield,
            damage_sources: Vec::new(),
        });
        self.capture_room_entry_unlock(permanent_id);
        self.place_entry_lore_counter(permanent_id);
        self.apply_legend_rule();

        if let EntryCompletion::SpellResolved { card, definition } = entry.completion {
            self.events
                .push(GameEvent::SpellResolved { card, definition });
        }
    }
}

fn entry_value(permanent: &Permanent, value: ValueDef) -> Option<i32> {
    match value {
        ValueDef::Constant(value) => Some(value),
        ValueDef::SourceCastX => Some(i32::from(permanent.cast_x)),
        ValueDef::AdditionalCostPayments(index) => Some(i32::from(
            permanent
                .cast_additional_costs
                .get(index.index())
                .copied()
                .unwrap_or_default(),
        )),
        ValueDef::IfAdditionalCostPaid(conditional) => {
            let paid = permanent
                .cast_additional_costs
                .get(conditional.cost.index())
                .copied()
                .unwrap_or_default();
            entry_value(
                permanent,
                if paid > 0 {
                    conditional.if_paid
                } else {
                    conditional.otherwise
                },
            )
        }
        ValueDef::Negate(value) => entry_value(permanent, *value)?.checked_neg(),
        ValueDef::Scaled(scaled) => {
            entry_value(permanent, scaled.value)?.checked_mul(scaled.factor)
        }
        ValueDef::Sum(sum) => {
            entry_value(permanent, sum.left)?.checked_add(entry_value(permanent, sum.right)?)
        }
        ValueDef::Halved(halved) => Some(halved.apply(entry_value(permanent, halved.value)?)),
        _ => None,
    }
}

include!("entry_replacements/queued_choices.rs");
