use crate::card::DamageSourceGroupDef;

use super::{
    AbilityDef, AbilityId, AbilityOrigin, AbilityProcedureDef, AbilitySourceRef, AddManaEffectDef,
    BattlefieldTriggerListener, CardDefinitionId, CardPartId, CardType, CommittedTriggerEvent,
    DamageEventMatcherDef, DamageKindDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, EffectDef, EffectRecipientSetDef, EffectiveAbility,
    FrozenActivatedAbility, Game, GameEvent, GameObjectId, InstalledTriggerLifetime,
    KeywordAbility, Mana, ManaSelectionDef, ManaSource, ObjectPredicateDef, ObjectRefDef,
    ObjectSetDef, PendingTrigger, Permanent, PlayerId, PlayerRefDef, PlayerRelation, PlayerSetDef,
    RetiredObject, ScopedEffect, StackAbilityResolver, TapPurposeDef, Target, TriggerCapture,
    TriggerContext, TriggerEventDef, TriggerEventObject, ZoneKind,
};

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
            AbilityOrigin::IntrinsicBasicLand(_) | AbilityOrigin::Granted { .. } => fallback,
        }
    }

    pub(super) fn capture_battlefield_triggers(&mut self, event: &CommittedTriggerEvent) {
        let listeners = self.battlefield_trigger_listeners();
        self.capture_battlefield_triggers_from_snapshot(&listeners, event);
    }

    pub(super) fn battlefield_trigger_listeners(&self) -> Vec<BattlefieldTriggerListener> {
        let mut listeners = Vec::new();
        for permanent in &self.battlefield {
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
        // Installed triggers listen the same way, minus a permanent to hang
        // on; they are appended last so a permanent's own triggers keep the
        // relative order they had before any existed.
        listeners.extend(self.installed_triggers.iter().map(|installed| {
            BattlefieldTriggerListener {
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

    pub(super) fn resolve_triggered_mana_effect(
        &mut self,
        source: AbilitySourceRef,
        controller: PlayerId,
        effect: EffectDef,
    ) {
        match effect {
            EffectDef::Sequence(effects) => {
                for effect in effects {
                    self.resolve_triggered_mana_effect(source, controller, *effect);
                }
            }
            EffectDef::AddMana(effect) => {
                self.resolve_triggered_add_mana_effect(source, controller, effect);
            }
            EffectDef::None
            | EffectDef::Randomized { .. }
            | EffectDef::Choose(_)
            | EffectDef::PayOr(_)
            | EffectDef::SplitIntoPiles(_)
            | EffectDef::PreventDamage { .. }
            | EffectDef::DealDamage { .. }
            | EffectDef::DrainLife { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::AddPoisonCounters { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::Discard { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::LoseTheGame { .. }
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::Regenerate { .. }
            | EffectDef::Tap { .. }
            | EffectDef::RemoveFromCombat { .. }
            | EffectDef::DestroyAtEndOfCombat { .. }
            | EffectDef::SkipNextUntapSteps { .. }
            | EffectDef::RemoveAllCounters { .. }
            | EffectDef::Untap { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::Mill { .. }
            | EffectDef::LookAtTopAndSelect { .. }
            | EffectDef::LookAtHand { .. }
            | EffectDef::SearchZone { .. }
            | EffectDef::ChooseCards { .. }
            | EffectDef::ReplaceNextDrawThisTurn { .. }
            | EffectDef::IfFormat { .. }
            | EffectDef::Counter { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::May { .. }
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CreateEmblem { .. }
            | EffectDef::Transform { .. }
            | EffectDef::ScheduleTurnPhases(_)
            | EffectDef::TakeExtraTurn { .. }
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::Detain { .. }
            | EffectDef::GainControl { .. }
            | EffectDef::IfCondition { .. }
            | EffectDef::InstallTrigger(_)
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::Attach { .. }
            | EffectDef::Reconfigure { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::CreateAttachedToken { .. }
            | EffectDef::CreateTokenCopyOf { .. }
            | EffectDef::StaticApply { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => {
                // Choice-bearing and non-mana primitives need a dedicated
                // immediate procedure before a supported card can use them.
            }
        }
    }

    fn resolve_triggered_add_mana_effect(
        &mut self,
        source: AbilitySourceRef,
        controller: PlayerId,
        effect: AddManaEffectDef,
    ) {
        let AddManaEffectDef {
            mana: ManaSelectionDef::One(kind),
            amount,
            restrictions,
            spend_effects,
            damage_to_controller,
        } = effect
        else {
            return;
        };
        let mana = Mana::from_ability(
            kind,
            ManaSource {
                object: source.object,
                ability: source.ability,
            },
            restrictions,
            spend_effects,
        );
        self.add_mana(controller, std::iter::repeat_n(mana, usize::from(amount)));
        if damage_to_controller > 0 {
            self.damage_target_from(
                Some(source.object),
                Some(Target::Player(controller)),
                damage_to_controller,
            );
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
            AbilityOrigin::IntrinsicBasicLand(_) => {
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
            target_defs,
            resolver,
            // Filled in by the activation, which is where X is chosen.
            x: 0,
        }
    }

    // Long because the event vocabulary is wide, not because the function
    // does several things: every arm pairs one definition with one event.
    #[allow(clippy::too_many_lines)]
    pub(super) fn trigger_event_matches_for_controller(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        match (definition, event) {
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
            (
                TriggerEventDef::BlocksOrBecomesBlockedBy { object: predicate },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                creature.id == source
                    && self.trigger_object_matches_for_controller(
                        predicate, other, source, false, controller,
                    )
            }
            (
                TriggerEventDef::Attacks(matcher),
                CommittedTriggerEvent::Attacks {
                    object,
                    declaration_size,
                    attack_number,
                },
            ) => {
                *declaration_size >= matcher.declaration.minimum
                    && matcher
                        .declaration
                        .maximum
                        .is_none_or(|maximum| *declaration_size <= maximum)
                    && matcher
                        .attack_number
                        .is_none_or(|number| *attack_number == number)
                    && self.trigger_object_matches_for_controller(
                        matcher.attacker,
                        object,
                        source,
                        false,
                        controller,
                    )
            }
            (
                TriggerEventDef::DamageDealt(matcher),
                damage @ CommittedTriggerEvent::DamageDealt { .. },
            ) => self.damage_trigger_matches(matcher, damage, source, controller),
            (
                TriggerEventDef::LifeGained(relation),
                CommittedTriggerEvent::LifeGained { player, .. },
            ) => {
                let controller = controller.unwrap_or(*player);
                self.player_relation_matches(*player, relation, controller, event.context())
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

    fn damage_trigger_matches(
        &self,
        matcher: DamageEventMatcherDef,
        event: &CommittedTriggerEvent,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        let CommittedTriggerEvent::DamageDealt {
            source,
            source_is_spell,
            recipient,
            recipient_object,
            combat,
            ..
        } = event
        else {
            return false;
        };
        (matcher.kind == DamageKindDef::Any || *combat)
            && self.damage_trigger_source_matches(
                matcher.source,
                source.as_ref(),
                *source_is_spell,
                ability_source,
                controller,
                event,
            )
            && self.damage_trigger_recipient_matches(
                matcher.recipient,
                *recipient,
                recipient_object.as_ref(),
                ability_source,
                controller,
                event,
            )
    }

    fn damage_trigger_source_matches(
        &self,
        matcher: DamageSourceMatcherDef,
        damage_source: Option<&TriggerEventObject>,
        source_is_spell: bool,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> bool {
        match matcher {
            DamageSourceMatcherDef::Any => true,
            DamageSourceMatcherDef::AffectedObject => {
                damage_source.is_some_and(|object| object.id == ability_source)
            }
            DamageSourceMatcherDef::Object(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .is_some_and(|expected| damage_source.is_some_and(|object| object.id == expected)),
            DamageSourceMatcherDef::Except(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .is_some_and(|excluded| damage_source.is_none_or(|object| object.id != excluded)),
            DamageSourceMatcherDef::Matching(predicate) => damage_source.is_some_and(|object| {
                self.trigger_object_matches_for_controller(
                    predicate,
                    object,
                    ability_source,
                    source_is_spell,
                    controller,
                )
            }),
            DamageSourceMatcherDef::Group(group) => damage_source.is_some_and(|object| {
                let flying = KeywordAbility::Flying
                    .simple_index()
                    .is_some_and(|index| object.keywords & (1 << index) != 0);
                match group {
                    DamageSourceGroupDef::CreaturesWithFlying => {
                        object.types.contains(CardType::Creature) && flying
                    }
                    DamageSourceGroupDef::AttackingCreaturesWithoutFlying => {
                        object.types.contains(CardType::Creature) && object.attacking && !flying
                    }
                    DamageSourceGroupDef::Artifacts => object.types.contains(CardType::Artifact),
                    DamageSourceGroupDef::UnblockedCreatures => {
                        object.types.contains(CardType::Creature)
                            && object.attacking
                            && !self
                                .battlefield
                                .iter()
                                .any(|blocker| blocker.blocking == Some(object.id))
                    }
                }
            }),
        }
    }

    fn damage_trigger_recipient_matches(
        &self,
        matcher: DamageRecipientMatcherDef,
        recipient: Target,
        recipient_object: Option<&TriggerEventObject>,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> bool {
        match matcher {
            DamageRecipientMatcherDef::Any => true,
            DamageRecipientMatcherDef::AffectedObject => {
                recipient == Target::Permanent(ability_source)
            }
            DamageRecipientMatcherDef::Recipients(recipients) => match recipients.0 {
                EffectRecipientSetDef::Objects(ObjectSetDef::One(reference)) => self
                    .trigger_event_object_reference(reference, ability_source, event)
                    .is_some_and(|expected| match recipient {
                        Target::Card(object)
                        | Target::Permanent(object)
                        | Target::Spell(object) => object == expected,
                        Target::Player(_) => false,
                    }),
                EffectRecipientSetDef::LegalTargets(_)
                | EffectRecipientSetDef::Objects(
                    ObjectSetDef::Binding(_)
                    | ObjectSetDef::LegalTargets(_)
                    | ObjectSetDef::Query(_)
                    | ObjectSetDef::SharingNameWith(_),
                ) => false,
                EffectRecipientSetDef::Players(players) => {
                    let Target::Player(recipient) = recipient else {
                        return false;
                    };
                    self.damage_trigger_player_set_matches(
                        players,
                        recipient,
                        ability_source,
                        controller,
                        event,
                    )
                }
            },
            DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => {
                let Some(player) =
                    self.trigger_event_player_reference(player, ability_source, controller, event)
                else {
                    return false;
                };
                match recipient {
                    Target::Player(recipient) => recipient == player,
                    Target::Permanent(object) => recipient_object.is_some_and(|recipient| {
                        recipient.id == object
                            && recipient.controller == player
                            && recipient.types.contains(CardType::Creature)
                    }),
                    Target::Card(_) | Target::Spell(_) => false,
                }
            }
        }
    }

    fn damage_trigger_player_set_matches(
        &self,
        players: PlayerSetDef,
        recipient: PlayerId,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> bool {
        match players {
            PlayerSetDef::All => true,
            PlayerSetDef::LegalTargets(_) => false,
            PlayerSetDef::One(reference) => {
                self.trigger_event_player_reference(reference, ability_source, controller, event)
                    == Some(recipient)
            }
            PlayerSetDef::Related(PlayerRelation::ChosenPlayer) => {
                self.chosen_player_of(ability_source) == Some(recipient)
            }
            PlayerSetDef::Related(relation) => controller.is_some_and(|controller| {
                self.player_relation_matches(recipient, relation, controller, event.context())
            }),
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

    fn trigger_event_object_reference(
        &self,
        reference: ObjectRefDef,
        ability_source: GameObjectId,
        event: &CommittedTriggerEvent,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source => Some(ability_source),
            ObjectRefDef::AttachedToSource => {
                self.current_or_last_known_attached_host(ability_source)
            }
            ObjectRefDef::TriggeringObject => event.context().object,
            ObjectRefDef::ResolvingObject | ObjectRefDef::Binding(_) | ObjectRefDef::Target(_) => {
                None
            }
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

include!("trigger_capture/object_matching.rs");
