use crate::card::{
    AttackDeclarationRangeDef, AttackEventMatcherDef, BattlefieldEntryChoiceDestinationDef,
    DamageSourceGroupDef,
};

use crate::CharacteristicContext;

use super::{
    AbilityDef, AbilityId, AbilityOrigin, AbilityProcedureDef, AbilitySourceRef, AddManaEffectDef,
    BattlefieldTriggerListener, CardDefinitionId, CardPartId, CardType, CommittedTriggerEvent,
    DamageEventMatcherDef, DamageKindDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, EffectDef, EffectRecipientSetDef, EffectResolutionContext,
    EffectiveAbility, FrozenActivatedAbility, Game, GameEvent, GameObjectId,
    InstalledTriggerLifetime, KeywordAbility, Mana, ManaSelectionDef, ManaSource,
    ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PendingTrigger, Permanent, PlayerId,
    PlayerRefDef, PlayerRelation, PlayerSetDef, RetiredObject, ScopedEffect, StackAbilityResolver,
    TapPurposeDef, Target, TriggerCapture, TriggerContext, TriggerEventDef, TriggerEventObject,
    ZoneKind,
};

mod graveyard;

impl Game {
    /// Finishes an atomic rules procedure before a player can receive
    /// priority. Mana abilities invoked while casting resolve inside the
    /// procedure, while ordinary triggers collected by them wait here.
    pub(super) fn finish_rules_procedure(&mut self) {
        // A decision can be one step in a still-resolving spell or turn-based
        // procedure. Neither state-based actions nor trigger placement happen
        // in the middle of that procedure: for example, a creature dealt
        // lethal damage by Chain Lightning can still activate a mana ability
        // when its controller is asked whether to pay for the copy. Drain the
        // continuation chain before reaching either priority-boundary check.
        loop {
            if self.pending_decisions.is_empty() && !self.pending_events.is_empty() {
                self.continue_pending_events();
            }
            if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
                return;
            }
            if self.pending_procedures.is_empty() {
                break;
            }
            self.continue_pending_procedures();
        }

        self.check_state_based_actions();
        if self.result.is_none()
            && self.pending_decisions.is_empty()
            && self.pending_events.is_empty()
            && self.pending_procedures.is_empty()
        {
            self.begin_trigger_placement();
        }
    }

    pub(super) fn capture_trigger(&mut self, capture: &TriggerCapture) {
        // Rule 603.4: an intervening-if condition is checked as the ability
        // would trigger. Failing it means the ability never triggers at all,
        // so nothing reaches the stack and nothing is reported.
        if !self.trigger_capture_condition_holds(capture) {
            return;
        }
        self.capture_trigger_prechecked(capture);
    }

    fn trigger_capture_condition_holds(&self, capture: &TriggerCapture) -> bool {
        capture.condition.is_none_or(|condition| {
            self.trigger_condition_holds(
                condition,
                capture.source.object,
                capture.controller,
                capture.context.trigger,
                Some(capture.source.ability),
                None,
            )
        })
    }

    fn capture_trigger_prechecked(&mut self, capture: &TriggerCapture) {
        let id = self.next_trigger_id;
        self.next_trigger_id = self.next_trigger_id.saturating_add(1);
        self.pending_triggers.push(PendingTrigger {
            id,
            source: capture.source,
            definition: capture.definition,
            owner: capture.owner,
            controller: capture.controller,
            text: capture.text,
            target_defs: capture.target_defs.clone(),
            targets: capture.targets.clone(),
            effect: capture.effect,
            resolver: capture.resolver,
            context: capture.context.clone(),
            condition: capture.condition,
            x: capture.x,
        });
        self.events.push(GameEvent::AbilityTriggered {
            player: capture.controller,
            trigger: id,
            source: capture.source.object,
            definition: capture.definition,
        });
    }

    pub(super) const fn ability_presentation_definition(
        origin: AbilityOrigin,
        fallback: CardDefinitionId,
    ) -> CardDefinitionId {
        match origin {
            AbilityOrigin::Printed { definition, .. } => definition,
            AbilityOrigin::IntrinsicBasicLand(_)
            | AbilityOrigin::IntrinsicCounter(_)
            | AbilityOrigin::Granted { .. } => fallback,
        }
    }

    pub(super) fn capture_battlefield_triggers(&mut self, event: &CommittedTriggerEvent) {
        let listeners = self.battlefield_trigger_listeners();
        self.capture_battlefield_triggers_from_snapshot(&listeners, event);
    }

    /// "Whenever one or more counters are put on this permanent." One event
    /// per permanent the placement touched, and one per placement rather
    /// than one per counter: two at once is one event carrying two.
    pub(in crate::game) fn capture_counters_placed(
        &mut self,
        permanents: &[GameObjectId],
        kind: crate::card::CounterKind,
        amount: u16,
    ) {
        if amount == 0 {
            return;
        }
        for permanent in permanents {
            let Some(object) = self
                .battlefield
                .iter()
                .find(|candidate| candidate.card.id == *permanent)
                .map(|candidate| self.trigger_event_object(candidate))
            else {
                continue;
            };
            self.capture_battlefield_triggers(&CommittedTriggerEvent::CountersPlaced {
                object,
                kind,
                amount,
            });
        }
    }

    /// "Whenever this becomes the target of a spell or ability", for the
    /// ability half. Raised once the ability is on the stack, which is where
    /// its targets are locked in, and once per targeting ability however many
    /// of its slots name the same permanent (CR 115.7c) -- the same rule the
    /// cast side follows.
    pub(super) fn capture_ability_targeting_triggers(&mut self, ability: GameObjectId) {
        let Some(object) = self.stack.iter().find(|object| object.id == ability) else {
            return;
        };
        let Some(event) = self.stack_object_event_object(object) else {
            return;
        };
        let mut targeted = Vec::new();
        for target in object
            .ability
            .as_ref()
            .into_iter()
            .flat_map(|payload| payload.targets.iter())
            .flat_map(crate::TargetSelection::targets)
        {
            if let Target::Permanent(id) | Target::Card(id) = target
                && !targeted.contains(id)
            {
                targeted.push(*id);
            }
        }
        for target in targeted {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecameTargetOfAbility {
                target,
                object: event.clone(),
            });
        }
    }

    /// "When you cycle this card" (CR 702.29b), raised as the cycling ability
    /// is activated. Only the cycled card can carry the clause, so its own
    /// printed abilities are the entire listener list -- there is no zone to
    /// scan. The card is read in the graveyard the discard cost has already
    /// put it in, which is also the object the trigger names.
    pub(super) fn capture_cycling_triggers(&mut self, cycled: GameObjectId, player: PlayerId) {
        let Some((_zone, card)) = self.card_in_nonbattlefield_zone(cycled) else {
            return;
        };
        let card = card.clone();
        let Some(object) = self.printed_trigger_event_object(
            cycled,
            card.definition,
            player,
            &CharacteristicContext::Graveyard,
        ) else {
            return;
        };
        let mut listeners = Vec::new();
        self.for_each_printed_card_ability(&card, &CharacteristicContext::Graveyard, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                return;
            };
            if !ability.is_executable()
                || definition.event != TriggerEventDef::Cycled
                || definition.procedure != AbilityProcedureDef::Shared
            {
                return;
            }
            listeners.push(BattlefieldTriggerListener {
                event: definition.event,
                uses_stack: true,
                trigger_limit: definition.trigger_limit,
                installed: None,
                capture: TriggerCapture {
                    source: AbilitySourceRef {
                        object: cycled,
                        ability: effective.origin,
                    },
                    definition: Self::ability_presentation_definition(
                        effective.origin,
                        card.definition,
                    ),
                    owner: card.owner,
                    controller: player,
                    text: ability.text,
                    target_defs: definition.targets.to_vec(),
                    targets: Vec::new(),
                    effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                    resolver: Self::ability_resolver(effective.origin, &ability),
                    context: TriggerContext::empty().into(),
                    condition: definition.condition,
                    x: 0,
                },
            });
        });
        if listeners.is_empty() {
            return;
        }
        self.capture_battlefield_triggers_from_snapshot(
            &listeners,
            &CommittedTriggerEvent::Cycled { object },
        );
    }

    /// A spell's own "when you cast this spell" clause, raised as it is put on
    /// the stack. Storm is the case: the ability belongs to the spell rather
    /// than to anything on the battlefield, so the ordinary listener scan
    /// never sees it, and the spell it copies is still on the stack beneath
    /// the trigger when that trigger resolves.
    pub(super) fn capture_own_cast_triggers(&mut self, spell: GameObjectId) {
        let Some(cast) = self.stack.iter().find(|object| object.id == spell).cloned() else {
            return;
        };
        let Some(object) = self.stack_object_event_object(&cast) else {
            return;
        };
        let card = cast.card.clone();
        let Some(signature) = cast.signature.as_ref() else {
            return;
        };
        let context = CharacteristicContext::Stack {
            form: signature.form().clone(),
        };
        let mut listeners = Vec::new();
        self.for_each_printed_card_ability(&card, &context, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                return;
            };
            if !ability.is_executable()
                || definition.event != TriggerEventDef::SpellCast(ObjectPredicateDef::Source)
                || definition.procedure != AbilityProcedureDef::Shared
            {
                return;
            }
            listeners.push(BattlefieldTriggerListener {
                event: definition.event,
                uses_stack: true,
                trigger_limit: definition.trigger_limit,
                installed: None,
                capture: TriggerCapture {
                    source: AbilitySourceRef {
                        object: spell,
                        ability: effective.origin,
                    },
                    definition: Self::ability_presentation_definition(
                        effective.origin,
                        card.definition,
                    ),
                    owner: card.owner,
                    controller: cast.controller,
                    text: ability.text,
                    target_defs: definition.targets.to_vec(),
                    targets: Vec::new(),
                    effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                    resolver: Self::ability_resolver(effective.origin, &ability),
                    context: TriggerContext::empty().into(),
                    condition: definition.condition,
                    x: 0,
                },
            });
        });
        if listeners.is_empty() {
            return;
        }
        self.capture_battlefield_triggers_from_snapshot(
            &listeners,
            &CommittedTriggerEvent::SpellCast { object },
        );
    }

    /// How many times this object's copy of one ability has triggered this
    /// turn, for the abilities that cap themselves.
    fn triggers_this_turn(&self, source: AbilitySourceRef) -> u8 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source.object)
            .and_then(|permanent| {
                permanent
                    .triggers_this_turn
                    .iter()
                    .find(|(origin, _)| *origin == source.ability)
            })
            .map_or(0, |(_, count)| *count)
    }

    fn record_trigger_this_turn(&mut self, source: AbilitySourceRef) {
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source.object)
        else {
            return;
        };
        if let Some(entry) = permanent
            .triggers_this_turn
            .iter_mut()
            .find(|(origin, _)| *origin == source.ability)
        {
            entry.1 = entry.1.saturating_add(1);
        } else {
            permanent.triggers_this_turn.push((source.ability, 1));
        }
    }

    pub(super) fn battlefield_trigger_listeners(&self) -> Vec<BattlefieldTriggerListener> {
        let mut listeners = Vec::new();
        // Emblems listen alongside the battlefield, the way they already sit
        // in every other sweep. Nothing about an emblem is in a zone, so the
        // battlefield an emblem's clause names is the one its abilities were
        // written against rather than a place it can be found.
        for permanent in self.battlefield.iter().chain(self.emblems.iter()) {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                if !ability.is_executable() {
                    return;
                }
                let (definition, uses_stack) = match ability.definition {
                    DeclarativeAbilityDef::TriggeredMana(definition) => {
                        if ability.declarative_effect().is_none() {
                            return;
                        }
                        (definition, false)
                    }
                    DeclarativeAbilityDef::Triggered(definition) => (definition, true),
                    DeclarativeAbilityDef::Spell(_)
                    | DeclarativeAbilityDef::ActivatedMana(_)
                    | DeclarativeAbilityDef::Activated(_)
                    | DeclarativeAbilityDef::Static(_)
                    | DeclarativeAbilityDef::Replacement(_)
                    | DeclarativeAbilityDef::AlternativeCast(_)
                    | DeclarativeAbilityDef::SpecialAction(_)
                    | DeclarativeAbilityDef::Keyword(_)
                    | DeclarativeAbilityDef::Legacy => return,
                };
                // Compatibility procedures execute elsewhere, so admitting
                // them here would manufacture a duplicate trigger.
                if definition.procedure != AbilityProcedureDef::Shared {
                    return;
                }
                if !definition.source_zones.contains(&ZoneKind::Battlefield) {
                    return;
                }
                let source = AbilitySourceRef {
                    object: permanent.card.id,
                    ability: effective.origin,
                };
                listeners.push(BattlefieldTriggerListener {
                    event: definition.event,
                    uses_stack,
                    trigger_limit: definition.trigger_limit,
                    installed: None,
                    capture: TriggerCapture {
                        source,
                        definition: Self::ability_presentation_definition(
                            effective.origin,
                            Self::effective_rules_source(permanent).0,
                        ),
                        owner: permanent.card.owner,
                        controller: permanent.controller,
                        text: ability.text,
                        target_defs: definition.targets.to_vec(),
                        targets: Vec::new(),
                        effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                        resolver: Self::ability_resolver(effective.origin, &ability),
                        context: TriggerContext::empty().into(),
                        condition: definition.condition,
                        x: 0,
                    },
                });
            });
        }
        self.extend_with_graveyard_trigger_listeners(&mut listeners);
        // Installed triggers listen the same way, minus a permanent to hang
        // on; they are appended last so a permanent's own triggers keep the
        // relative order they had before any existed.
        listeners.extend(self.installed_triggers.iter().map(|installed| {
            BattlefieldTriggerListener {
                trigger_limit: None,
                event: installed.event,
                uses_stack: true,
                installed: Some(installed.id),
                capture: installed.capture.clone(),
            }
        }));
        listeners
    }

    pub(super) fn capture_battlefield_triggers_from_snapshot(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        event: &CommittedTriggerEvent,
    ) {
        self.capture_battlefield_trigger_batch_from_snapshot(
            listeners,
            std::slice::from_ref(event),
        );
    }

    /// Determine every match and intervening-if result for one atomic event
    /// batch before any triggered-mana ability can mutate the game. Attack
    /// declarations and simultaneous exits both publish more than one
    /// object-local event, but all of those facts belong to one rules event.
    pub(super) fn capture_battlefield_trigger_batch_from_snapshot(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        events: &[CommittedTriggerEvent],
    ) {
        self.capture_battlefield_trigger_batch_with_mana_resolver(
            listeners,
            events,
            |game, capture| {
                game.resolve_triggered_mana_effect(
                    capture.source,
                    capture.controller,
                    capture.effect,
                    &capture.context,
                );
            },
        );
    }

    pub(super) fn capture_battlefield_trigger_batch_with_mana_resolver(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        events: &[CommittedTriggerEvent],
        mut resolve_mana: impl FnMut(&mut Self, TriggerCapture),
    ) {
        let mut consumed_once = Vec::new();
        let mut matched = Vec::new();
        // "Triggers only once each turn" counts the triggering rather than
        // the resolution, and one batch can offer a capped ability several
        // matching events, so the count has to rise inside this loop as
        // well as be read from the turn so far.
        let mut limited = Vec::new();
        for event in events {
            for listener in listeners {
                if !self.trigger_event_matches_for_controller(
                    listener.event,
                    event,
                    listener.capture.source.object,
                    Some(listener.capture.controller),
                ) {
                    continue;
                }
                if let Some(limit) = listener.trigger_limit {
                    let source = listener.capture.source;
                    let already = self.triggers_this_turn(source);
                    let in_batch = limited.iter().filter(|counted| **counted == source).count();
                    if usize::from(already).saturating_add(in_batch) >= usize::from(limit) {
                        continue;
                    }
                    limited.push(source);
                }
                if let Some(id) = listener.installed
                    && self
                        .installed_triggers
                        .iter()
                        .find(|installed| installed.id == id)
                        .is_some_and(|installed| {
                            matches!(installed.lifetime, InstalledTriggerLifetime::Once)
                        })
                {
                    if consumed_once.contains(&id) {
                        continue;
                    }
                    // A once-only listener is consumed by the first matching
                    // event even when its intervening-if condition is false.
                    consumed_once.push(id);
                }
                let mut capture = listener.capture.clone();
                // Keep installer bindings and targets; only the committed
                // event-local context changes for this match.
                capture.context.trigger = event.context();
                let condition_holds = self.trigger_capture_condition_holds(&capture);
                matched.push((listener.uses_stack, capture, condition_holds));
            }
        }

        self.installed_triggers
            .retain(|installed| !consumed_once.contains(&installed.id));
        for source in limited {
            self.record_trigger_this_turn(source);
        }

        // Record ordinary triggers first, using the precomputed condition.
        // Any triggers caused while a triggered-mana ability resolves are
        // therefore later in the pending stream than the event that caused it.
        for (uses_stack, capture, condition_holds) in &matched {
            if *uses_stack && *condition_holds {
                self.capture_trigger_prechecked(capture);
            }
        }
        for (uses_stack, capture, condition_holds) in matched {
            if !uses_stack && condition_holds {
                resolve_mana(self, capture);
            }
        }
    }

    pub(super) fn capture_custom_source_triggers(
        &mut self,
        source: &Permanent,
        abilities: &[EffectiveAbility],
        event: &CommittedTriggerEvent,
    ) {
        let triggers = abilities
            .iter()
            .filter_map(|effective| match effective.ability.definition {
                DeclarativeAbilityDef::Triggered(definition)
                    if effective.ability.is_executable()
                        && definition.procedure == AbilityProcedureDef::Legacy
                        && effective.ability.custom_behavior().is_some()
                        && definition.source_zones.contains(&ZoneKind::Battlefield)
                        && self.trigger_event_matches_for_controller(
                            definition.event,
                            event,
                            source.card.id,
                            Some(source.controller),
                        ) =>
                {
                    Some((
                        effective.origin,
                        effective.ability.text,
                        definition.targets,
                        effective
                            .ability
                            .declarative_effect()
                            .unwrap_or(EffectDef::None),
                        Self::ability_resolver(effective.origin, &effective.ability),
                    ))
                }
                DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
            .collect::<Vec<_>>();
        for (ability, text, targets, effect, resolver) in triggers {
            self.capture_trigger(&TriggerCapture {
                source: AbilitySourceRef {
                    object: source.card.id,
                    ability,
                },
                definition: Self::ability_presentation_definition(
                    ability,
                    Self::effective_rules_source(source).0,
                ),
                owner: source.card.owner,
                controller: source.controller,
                text,
                target_defs: targets.to_vec(),
                targets: Vec::new(),
                effect,
                resolver,
                context: event.context().into(),
                // A legacy custom trigger states its own condition inside its
                // behavior rather than declaring one here.
                condition: None,
                x: 0,
            });
        }
    }

    pub(super) fn ability_resolver(
        origin: AbilityOrigin,
        ability: &AbilityDef,
    ) -> StackAbilityResolver {
        if let Some(binding) = crate::card::ability_binding(origin, ability) {
            return StackAbilityResolver::CardOwned(binding.resolver());
        }
        if let Some(behavior) = ability.custom_behavior() {
            StackAbilityResolver::Custom(behavior)
        } else {
            let effect = match ability.declarative_effect() {
                Some(effect) => effect,
                None => EffectDef::None,
            };
            StackAbilityResolver::Declarative(ScopedEffect::primary(effect))
        }
    }

    pub(super) fn ability_origin_components(
        origin: AbilityOrigin,
        fallback: CardDefinitionId,
    ) -> (CardDefinitionId, CardPartId, AbilityId) {
        match origin {
            AbilityOrigin::Printed {
                definition,
                part,
                ability,
            } => (definition, part, ability),
            AbilityOrigin::Granted {
                source_definition,
                source_part,
                source_ability,
                ..
            } => (source_definition, source_part, source_ability),
            AbilityOrigin::IntrinsicBasicLand(_) | AbilityOrigin::IntrinsicCounter(_) => {
                (fallback, CardPartId::PRIMARY, AbilityId::PRIMARY)
            }
        }
    }

    pub(super) fn freeze_activated_ability(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
    ) -> FrozenActivatedAbility {
        let effective =
            self.find_effective_ability(permanent, |effective| effective.origin == origin);
        let fallback_definition = Self::effective_rules_source(permanent).0;
        let presentation_definition =
            Self::ability_presentation_definition(origin, fallback_definition);
        let text = effective.map(|effective| effective.ability.text);
        let definition = effective.map(|effective| Box::new(effective.ability));
        let (target_defs, resolver) = effective.map_or(
            (
                &[][..],
                StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::None)),
            ),
            |effective| {
                let target_defs = match effective.ability.definition {
                    DeclarativeAbilityDef::Activated(definition) => definition.targets,
                    DeclarativeAbilityDef::Spell(_)
                    | DeclarativeAbilityDef::ActivatedMana(_)
                    | DeclarativeAbilityDef::TriggeredMana(_)
                    | DeclarativeAbilityDef::Triggered(_)
                    | DeclarativeAbilityDef::Static(_)
                    | DeclarativeAbilityDef::Replacement(_)
                    | DeclarativeAbilityDef::AlternativeCast(_)
                    | DeclarativeAbilityDef::SpecialAction(_)
                    | DeclarativeAbilityDef::Keyword(_)
                    | DeclarativeAbilityDef::Legacy => &[],
                };
                (
                    target_defs,
                    Self::ability_resolver(effective.origin, &effective.ability),
                )
            },
        );
        FrozenActivatedAbility {
            origin,
            definition,
            presentation_definition,
            text,
            target_defs: target_defs.to_vec(),
            resolver,
            // Both filled in by the activation, which is where X and the
            // modes are chosen.
            mode_effects: Vec::new(),
            x: 0,
        }
    }

    // Long only because every event definition pairs with its committed event.
    #[allow(clippy::too_many_lines)]
    pub(super) fn trigger_event_matches_for_controller(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        match (definition, event) {
            // One printed ability, several ways into the same matching path.
            (TriggerEventDef::AnyOf(events), _) => events.iter().any(|alternative| {
                self.trigger_event_matches_for_controller(*alternative, event, source, controller)
            }),
            (
                TriggerEventDef::ZoneChanged(matcher),
                CommittedTriggerEvent::ZoneChanged {
                    object,
                    from: actual_from,
                    to: actual_to,
                    damage_sources,
                },
            ) => {
                matcher.from.is_none_or(|expected| expected == *actual_from)
                    && matcher.to.is_none_or(|expected| expected == *actual_to)
                    && matcher.previously_damaged_by.is_none_or(|reference| {
                        self.trigger_event_object_reference(reference, source, event)
                            .is_some_and(|source| damage_sources.contains(&source))
                    })
                    && self.trigger_object_matches_for_controller(
                        matcher.object,
                        object,
                        source,
                        false,
                        controller,
                    )
            }
            (
                TriggerEventDef::Tapped(matcher),
                CommittedTriggerEvent::Tapped { object, for_mana },
            ) => {
                (matcher.purpose == TapPurposeDef::Any || *for_mana)
                    && self.trigger_object_matches_for_controller(
                        matcher.object,
                        object,
                        source,
                        false,
                        controller,
                    )
            }
            (
                TriggerEventDef::CountersPlaced {
                    object: predicate,
                    kind,
                },
                CommittedTriggerEvent::CountersPlaced {
                    object,
                    kind: placed,
                    ..
                },
            ) => {
                kind == *placed
                    && self.trigger_object_matches_for_controller(
                        predicate, object, source, false, controller,
                    )
            }
            (
                TriggerEventDef::BecomesBlocked(predicate),
                CommittedTriggerEvent::BecomesBlocked { object, .. },
            )
            | (
                TriggerEventDef::AttacksAndIsNotBlocked {
                    attacker: predicate,
                },
                CommittedTriggerEvent::AttacksAndIsNotBlocked { object },
            )
            | (
                TriggerEventDef::Transforms(predicate),
                CommittedTriggerEvent::Transformed { object },
            ) => self.trigger_object_matches_for_controller(
                predicate, object, source, false, controller,
            ),
            // The listener was pointed at; the predicate reads the spell.
            (
                TriggerEventDef::BecomesTargetOfSpell(predicate),
                CommittedTriggerEvent::BecameTargetOfSpell { target, object },
            )
            | (
                TriggerEventDef::BecomesTargetOfSpellOrAbility(predicate),
                CommittedTriggerEvent::BecameTargetOfSpell { target, object }
                | CommittedTriggerEvent::BecameTargetOfAbility { target, object },
            ) => {
                *target == source
                    && self.trigger_object_matches_for_controller(
                        predicate, object, source, false, controller,
                    )
            }
            (
                TriggerEventDef::BlocksOrBecomesBlockedBy {
                    creature: subject,
                    other: predicate,
                },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                self.trigger_object_matches_for_controller(
                    subject, creature, source, false, controller,
                ) && self.trigger_object_matches_for_controller(
                    predicate, other, source, false, controller,
                )
            }
            // The one-directional halves distinguish the pair by its attacker.
            (
                TriggerEventDef::Blocks { blocked: predicate },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                creature.id == source
                    && !creature.attacking
                    && self.trigger_object_matches_for_controller(
                        predicate, other, source, false, controller,
                    )
            }
            (
                TriggerEventDef::BecomesBlockedBy { blocker: predicate },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                creature.id == source
                    && creature.attacking
                    && self.trigger_object_matches_for_controller(
                        predicate, other, source, false, controller,
                    )
            }
            (
                TriggerEventDef::CardsExiled { zones, owner },
                CommittedTriggerEvent::CardsExiled {
                    cards,
                    from,
                    owner: exiled_by,
                },
            ) => self.exile_move_matches(zones, owner, cards, *from, *exiled_by, controller),
            (
                TriggerEventDef::AttackDeclared {
                    attacker,
                    declaration,
                },
                CommittedTriggerEvent::AttackersDeclared { attackers },
            ) => self.attack_declaration_matches(
                attacker,
                declaration,
                attackers,
                source,
                controller,
            ),
            (
                TriggerEventDef::Attacks(matcher),
                CommittedTriggerEvent::Attacks {
                    object,
                    declaration_size,
                    attack_number,
                    ..
                },
            ) => self.attacker_matches(
                matcher,
                object,
                *declaration_size,
                *attack_number,
                source,
                controller,
            ),
            (
                TriggerEventDef::DamageDealt(matcher),
                damage @ CommittedTriggerEvent::DamageDealt { .. },
            ) => self.damage_trigger_matches(matcher, damage, source, controller),
            // Only the Class carrying the clause can reach its own levels,
            // so the object is the whole of the match.
            (
                TriggerEventDef::BecomesLevel(wanted),
                CommittedTriggerEvent::BecameLevel { object, level },
            ) => *object == source && *level == wanted,
            (
                TriggerEventDef::DrewCard(matcher),
                CommittedTriggerEvent::DrewCard {
                    player,
                    first_in_draw_step,
                },
            ) => {
                let controller = controller.unwrap_or(*player);
                !(matcher.except_first_in_draw_step && *first_in_draw_step)
                    && self.player_relation_matches(
                        *player,
                        matcher.player,
                        controller,
                        event.context(),
                    )
            }
            // Both name only the player the event happened to.
            (TriggerEventDef::Discarded(relation), CommittedTriggerEvent::Discarded { player })
            | (
                TriggerEventDef::BecomesMonarch(relation),
                CommittedTriggerEvent::BecameMonarch { player },
            )
            | (
                TriggerEventDef::CommittedCrime(relation),
                CommittedTriggerEvent::CommittedCrime { player },
            )
            | (
                TriggerEventDef::LifeGained(relation),
                CommittedTriggerEvent::LifeGained { player, .. },
            ) => {
                let controller = controller.unwrap_or(*player);
                self.player_relation_matches(*player, relation, controller, event.context())
            }
            // The listener list for a cycled card holds only that card's own
            // clauses, so there is nothing further to match on: any card
            // whose ability reached here is the card that was cycled.
            (TriggerEventDef::Cycled, CommittedTriggerEvent::Cycled { object }) => {
                object.id == source
            }
            (
                TriggerEventDef::SpellCast(predicate),
                CommittedTriggerEvent::SpellCast { object },
            ) => self
                .trigger_object_matches_for_controller(predicate, object, source, true, controller),
            (
                TriggerEventDef::StepBegins { step, player },
                CommittedTriggerEvent::StepBegins {
                    step: actual_step,
                    player: actual_player,
                },
            ) => {
                if step != *actual_step {
                    return false;
                }
                if player == PlayerRelation::ChosenPlayer {
                    return self.chosen_player_of(source) == Some(*actual_player);
                }
                if player == PlayerRelation::ControllerOfAttachedPermanent {
                    return self.attached_host_controller_of(source) == Some(*actual_player);
                }
                let controller = controller
                    .or_else(|| self.current_or_last_known_controller(source))
                    .unwrap_or(*actual_player);
                self.player_relation_matches(*actual_player, player, controller, event.context())
            }
            _ => false,
        }
    }

    fn trigger_event_player_reference(
        &self,
        reference: PlayerRefDef,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> Option<PlayerId> {
        match reference {
            PlayerRefDef::EffectController => controller,
            PlayerRefDef::Opponent => controller.map(PlayerId::opponent),
            PlayerRefDef::EventPlayer => event.context().event_player,
            PlayerRefDef::ControllerOf(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .and_then(|object| self.current_or_last_known_controller(object)),
            PlayerRefDef::OwnerOf(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .and_then(|object| self.current_or_last_known_owner(object)),
            PlayerRefDef::Target(_) => None,
        }
    }

    /// Who controls an object, whether it is still on the battlefield or has
    /// left and is only remembered.
    pub(super) fn controller_of_object(&self, object: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.controller)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.controller),
                Some(RetiredObject::Stack(object)) => Some(object.controller),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }
}

include!("trigger_capture/attack_matching.rs");
include!("trigger_capture/damage_matching.rs");
include!("trigger_capture/triggered_mana.rs");
include!("trigger_capture/object_matching.rs");
