use super::{
    AbilityDef, AbilityOperationDef, AbilitySourceRef, ApplicableReplacement, AppliedEffectDef,
    BasicLandType, BattlefieldEntryModificationDef, CardTypeSet, CharacteristicOperationDef,
    CommittedTriggerEvent, ConditionDef, ControlFlow, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef, EffectDef,
    EffectPaymentCostDef, EffectPaymentDef, EffectResolutionContext, EntryCompletion, Game,
    GameEvent, ObjectPredicateDef, PendingBattlefieldEntry, PendingEvent, PendingReplacementEffect,
    Permanent, PlayerId, ReplaceableEvent, ReplacementChoiceDef, ReplacementConditionDef,
    ReplacementEffectContext, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectPayment,
    ScopedEffect, StackObject, StackObjectKind, Target, TriggerContext, ZoneKind,
};

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
                    let Some(pending) = self.prepare_entry_replacement(pending, *candidate) else {
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
                                card: Some((candidate.context.source.object, candidate.definition)),
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
        candidate: ApplicableReplacement,
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

    /// Applies one queued replacement operation. `None` means the operation
    /// suspended the event behind a decision that will resume it.
    /// Offers the copy choice an entering permanent may make, or lets it enter
    /// as itself when there is nothing to copy.
    pub(super) fn offer_entry_copy(
        &mut self,
        pending: PendingEvent,
        object: ObjectPredicateDef,
        added_types: CardTypeSet,
    ) -> Option<PendingEvent> {
        let player = Self::pending_event_controller(&pending);
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        let entering = entry.permanent.card.id;
        let choices = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.card.id != entering)
            .filter(|permanent| {
                self.trigger_object_matches(
                    object,
                    &self.trigger_event_object(permanent),
                    entering,
                    false,
                )
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        if choices.is_empty() {
            return Some(pending);
        }
        self.pending_events.push_front(pending);
        self.queue_entry_copy_choice(player, choices, added_types);
        None
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
            ReplacementEffectDef::CopyEntering {
                object,
                added_types,
            } => self.offer_entry_copy(pending, object, added_types),
            // With two players every relation this appears on names exactly
            // one candidate, so the choice is recorded rather than asked.
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Player(relation)) => {
                let controller = Self::pending_event_controller(&pending);
                let chosen = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
                    self.player_relation_matches(
                        *player,
                        relation,
                        controller,
                        TriggerContext::empty(),
                    )
                });
                let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
                entry.permanent.chosen_player = chosen;
                Some(pending)
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
            // These primitives belong to other prospective-event procedures
            // and cannot alter a battlefield-entry event.
            ReplacementEffectDef::ReplaceEventWithNothing
            | ReplacementEffectDef::MoveToZone(_)
            | ReplacementEffectDef::Perform(_)
            | ReplacementEffectDef::MultiplyEventAmount(_) => Some(pending),
        }
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
            object_controller: Some(entry.permanent.controller),
            event_player: Some(entry.permanent.controller),
            amount: None,
        }
    }

    pub(super) fn pending_entry_name(&self, pending: &PendingEvent) -> String {
        let ReplaceableEvent::BattlefieldEntry(entry) = &pending.event;
        let definition = Self::effective_rules_source(&entry.permanent).0;
        self.catalog
            .get(definition)
            .map_or_else(|| "this permanent".to_string(), |card| card.name.clone())
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
        let resolved = match payment.cost {
            EffectPaymentCostDef::Mana(cost) => ResolvedEffectPayment::Mana(cost),
            EffectPaymentCostDef::GenericMana(amount) => {
                let amount = self
                    .effect_value(amount, &object, &resolution, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                ResolvedEffectPayment::Mana(crate::ManaCost::new(amount, 0))
            }
            EffectPaymentCostDef::Life(amount) => ResolvedEffectPayment::Life(amount),
        };
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
        self.queue_decision(
            player,
            format!("{payment_label} as {name} enters the battlefield?"),
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Do not pay".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: payment_label,
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
            ],
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
            ConditionDef::Exists(query) => self.any_object_matches_query_with_prospective(
                query,
                context.controller,
                context.source.object,
                Self::pending_event_context(pending),
                Some(&entry.permanent),
            ),
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
            ReplacementConditionDef::CreatureDiedThisTurn => self.creature_died_this_turn,
        }
    }

    pub(super) fn modify_pending_battlefield_entry(
        pending: &mut PendingEvent,
        modification: BattlefieldEntryModificationDef,
    ) {
        let ReplaceableEvent::BattlefieldEntry(entry) = &mut pending.event;
        match modification {
            BattlefieldEntryModificationDef::Tapped => entry.permanent.tapped = true,
            BattlefieldEntryModificationDef::AddCounters { kind, amount } => {
                entry.permanent.add_counters(kind, amount);
            }
        }
    }

    pub(super) fn is_source_entry_replacement(ability: &AbilityDef) -> bool {
        ability.is_executable()
            && matches!(
                (ability.definition, ability.declarative_replacement()),
                (
                    DeclarativeAbilityDef::Replacement(definition),
                    Some(_),
                ) if definition.event == ReplacementEventDef::SourceEntersBattlefield
            )
    }

    pub(super) fn is_external_entry_replacement(ability: &AbilityDef) -> bool {
        ability.is_executable()
            && matches!(
                ability.definition,
                DeclarativeAbilityDef::Replacement(definition)
                    if definition.source_zones.contains(&ZoneKind::Battlefield)
                        && matches!(
                            definition.event,
                            ReplacementEventDef::ObjectEntersBattlefield { .. }
                        )
            )
            && ability.declarative_replacement().is_some()
    }

    pub(super) fn applied_grant_entry_replacement_possibilities(
        effect: AppliedEffectDef,
    ) -> (bool, bool) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                effects
                    .iter()
                    .fold((false, false), |(source, external), effect| {
                        let found = Self::applied_grant_entry_replacement_possibilities(*effect);
                        (source || found.0, external || found.1)
                    })
            }
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )) => (
                Self::is_source_entry_replacement(ability),
                Self::is_external_entry_replacement(ability),
            ),
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => (false, false),
        }
    }

    /// Whether a static effect could grant a source-entry replacement or an
    /// external object-entry replacement, respectively.
    pub(super) fn granted_entry_replacement_possibilities(effect: EffectDef) -> (bool, bool) {
        match effect {
            EffectDef::Sequence(effects) => {
                effects
                    .iter()
                    .fold((false, false), |(source, external), effect| {
                        let found = Self::granted_entry_replacement_possibilities(*effect);
                        (source || found.0, external || found.1)
                    })
            }
            EffectDef::StaticApply { effect, .. } => {
                Self::applied_grant_entry_replacement_possibilities(effect)
            }
            _ => (false, false),
        }
    }

    pub(super) fn static_grant_entry_replacement_possibilities(
        ability: &AbilityDef,
    ) -> (bool, bool) {
        if ability.is_executable()
            && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
            && let Some(effect) = ability.declarative_effect()
        {
            Self::granted_entry_replacement_possibilities(effect)
        } else {
            (false, false)
        }
    }

    pub(super) fn prospective_permanent_may_supply_source_entry_replacement(
        &self,
        permanent: &Permanent,
    ) -> bool {
        let may_supply = |ability: &AbilityDef| {
            Self::is_source_entry_replacement(ability)
                || Self::static_grant_entry_replacement_possibilities(ability).0
        };
        self.effective_rules(permanent)
            .is_some_and(|rules| rules.ability_clauses().iter().any(&may_supply))
            || permanent.copy_effect.iter().any(|copy| {
                copy.added_abilities
                    .iter()
                    .any(|added| may_supply(&added.definition))
            })
    }

    /// Returns whether an existing static source might grant the prospective
    /// permanent a source-entry replacement and whether the battlefield might
    /// supply an external object-entry replacement. Recipient mismatches may
    /// yield conservative false positives, never false negatives.
    pub(super) fn battlefield_entry_replacement_possibilities(&self) -> (bool, bool) {
        let mut source = false;
        let mut external = false;
        for permanent in &self.battlefield {
            let result = self.visit_effective_abilities(permanent, |effective| {
                let ability = &effective.ability;
                let granted = Self::static_grant_entry_replacement_possibilities(ability);
                source |= granted.0;
                external |= Self::is_external_entry_replacement(ability) || granted.1;
                if source && external {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
            if source && external {
                debug_assert!(result.is_break());
                break;
            }
        }
        (source, external)
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
                    definition: Self::ability_presentation_definition(
                        effective.origin,
                        entry.permanent.card.definition,
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
                    let ReplacementEventDef::ObjectEntersBattlefield { object, controller } =
                        definition.event
                    else {
                        return ControlFlow::Continue(());
                    };
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
                            definition: Self::ability_presentation_definition(
                                effective.origin,
                                source_permanent.card.definition,
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

    pub(super) fn commit_battlefield_entry(&mut self, mut entry: PendingBattlefieldEntry) {
        if entry.completion != EntryCompletion::Setup {
            let (card, _zone_change) = self.zone_change_card(entry.permanent.card);
            entry.permanent.card = card;
        }
        entry.permanent.timestamp = self.allocate_continuous_effect_timestamp();
        let permanent_id = entry.permanent.card.id;
        let definition = entry.permanent.card.definition;
        self.battlefield.push(entry.permanent);

        if let EntryCompletion::AttachSource { source } = entry.completion {
            self.try_attach(source, permanent_id);
        }

        if let EntryCompletion::LandPlayed { player } = entry.completion {
            self.events.push(GameEvent::LandPlayed {
                player,
                card: permanent_id,
                definition,
            });
        }

        let entered = self
            .battlefield
            .last()
            .expect("a committed battlefield entry is present");
        let entered_event = self.trigger_event_object(entered);
        self.capture_battlefield_triggers(&CommittedTriggerEvent::ZoneChanged {
            object: entered_event,
            from: entry.from,
            to: ZoneKind::Battlefield,
            damage_sources: Vec::new(),
        });
        self.apply_legend_rule();

        if let EntryCompletion::SpellResolved { card, definition } = entry.completion {
            self.events
                .push(GameEvent::SpellResolved { card, definition });
        }
    }

    pub(super) fn queue_basic_land_type_text_change(&mut self, player: PlayerId, target: Target) {
        let options = BasicLandType::ALL
            .into_iter()
            .flat_map(|from| {
                BasicLandType::ALL
                    .into_iter()
                    .filter(move |to| from != *to)
                    .map(move |to| DecisionOption {
                        id: u32::try_from(from.index() * BasicLandType::ALL.len() + to.index())
                            .expect("the basic-land-type choice id fits u32"),
                        label: format!("{} → {}", from.subtype(), to.subtype()),
                        card: None,
                        members: Vec::new(),
                        ability_text: None,
                        zone: DecisionZone::None,
                    })
            })
            .collect();
        self.queue_decision(
            player,
            "Replace one basic land type with another",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BasicLandTypeTextChange { target },
        );
    }
}
