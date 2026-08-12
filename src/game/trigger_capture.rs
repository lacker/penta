use super::{
    AbilityDef, AbilityId, AbilityOrigin, AbilityProcedureDef, AbilitySourceRef,
    AbilityTargetPredicate, AddManaEffectDef, BattlefieldTriggerListener, CardDefinitionId,
    CardInstance, CardPartId, CardType, CharacteristicContext, CommittedTriggerEvent,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, EffectiveAbility, FrozenActivatedAbility,
    Game, GameEvent, GameObjectId, Mana, ManaSelectionDef, ManaSource, ObjectPredicateDef,
    PendingTrigger, Permanent, PlayerId, PlayerRelation, RetiredObject, ScopedEffect,
    StackAbilityResolver, StackObjectKind, Target, TriggerCapture, TriggerContext, TriggerEventDef,
    TriggerEventObject, ZoneKind,
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
        if self.pending_decisions.is_empty() && !self.pending_events.is_empty() {
            self.continue_pending_events();
        }
        if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
            return;
        }

        self.check_state_based_actions();
        if self.result.is_none()
            && self.pending_decisions.is_empty()
            && self.pending_events.is_empty()
        {
            self.begin_trigger_placement();
        }
    }

    pub(super) fn capture_trigger(&mut self, capture: &TriggerCapture) {
        // Rule 603.4: an intervening-if condition is checked as the ability
        // would trigger. Failing it means the ability never triggers at all,
        // so nothing reaches the stack and nothing is reported.
        if let Some(condition) = capture.condition
            && !self.trigger_condition_holds(
                condition,
                capture.source.object,
                capture.controller,
                capture.context,
                Some(capture.source.ability),
                None,
            )
        {
            return;
        }
        let id = self.next_trigger_id;
        self.next_trigger_id = self.next_trigger_id.saturating_add(1);
        self.pending_triggers.push(PendingTrigger {
            id,
            source: capture.source,
            definition: capture.definition,
            owner: capture.owner,
            controller: capture.controller,
            text: capture.text,
            target_defs: capture.target_defs,
            targets: Vec::new(),
            effect: capture.effect,
            resolver: capture.resolver,
            context: capture.context,
            condition: capture.condition,
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
                    capture: TriggerCapture {
                        source,
                        definition: Self::ability_presentation_definition(
                            effective.origin,
                            Self::effective_rules_source(permanent).0,
                        ),
                        owner: permanent.card.owner,
                        controller: permanent.controller,
                        text: ability.text,
                        target_defs: definition.targets,
                        effect: ability.effect.definition,
                        resolver: Self::ability_resolver(effective.origin, &ability),
                        context: TriggerContext::empty(),
                        condition: definition.condition,
                    },
                });
            });
        }
        // A floating trigger listens the same way, minus a permanent to hang
        // on; it is appended last so a permanent's own triggers keep the
        // relative order they had before any existed.
        listeners.extend(self.floating_triggers.iter().map(|floating| {
            BattlefieldTriggerListener {
                event: floating.event,
                uses_stack: true,
                capture: floating.capture,
            }
        }));
        listeners
    }

    pub(super) fn capture_battlefield_triggers_from_snapshot(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        event: &CommittedTriggerEvent,
    ) {
        let mana_triggers = listeners
            .iter()
            .copied()
            .filter(|listener| {
                !listener.uses_stack
                    && self.trigger_event_matches(
                        listener.event,
                        event,
                        listener.capture.source.object,
                    )
            })
            .collect::<Vec<_>>();
        for listener in mana_triggers {
            self.resolve_triggered_mana_effect(
                listener.capture.source,
                listener.capture.controller,
                listener.capture.effect,
            );
        }

        let stack_triggers = listeners
            .iter()
            .copied()
            .filter(|listener| {
                listener.uses_stack
                    && self.trigger_event_matches(
                        listener.event,
                        event,
                        listener.capture.source.object,
                    )
            })
            .collect::<Vec<_>>();
        for listener in stack_triggers {
            self.capture_trigger(&TriggerCapture {
                context: event.context(),
                ..listener.capture
            });
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
            EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::One(kind),
                amount,
                restrictions,
                spend_effects,
                damage_to_controller,
            }) => {
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
            EffectDef::None
            | EffectDef::Randomized { .. }
            | EffectDef::ChoosePermanent { .. }
            | EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::Choice(_),
                ..
            })
            | EffectDef::DealDamage { .. }
            | EffectDef::DrainLife { .. }
            | EffectDef::GainLife { .. }
            | EffectDef::DrawCards { .. }
            | EffectDef::Discard { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::LoseTheGame { .. }
            | EffectDef::AddManaEqualTo { .. }
            | EffectDef::Tap { .. }
            | EffectDef::Untap { .. }
            | EffectDef::PreventCombatDamageThisTurn { .. }
            | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
            | EffectDef::Destroy { .. }
            | EffectDef::Sacrifice { .. }
            | EffectDef::SacrificeOfChoice { .. }
            | EffectDef::DestroyOfChoice { .. }
            | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
            | EffectDef::RevealAndSplitIntoPiles { .. }
            | EffectDef::Mill { .. }
            | EffectDef::LookAtTopAndMayTake { .. }
            | EffectDef::LookAtTopAndSelect { .. }
            | EffectDef::LookAtHand { .. }
            | EffectDef::SearchLibrary { .. }
            | EffectDef::Counter { .. }
            | EffectDef::CounterUnlessPaid { .. }
            | EffectDef::AddCounters { .. }
            | EffectDef::ChangeTextBasicLandType { .. }
            | EffectDef::BecomeCopyOf { .. }
            | EffectDef::OptionalPayment { .. }
            | EffectDef::UnlessPaid { .. }
            | EffectDef::May(_)
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CreateEmblem { .. }
            | EffectDef::Transform { .. }
            | EffectDef::AdditionalCombatPhase
            | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
            | EffectDef::GrantFlashToNextSorcery
            | EffectDef::ExileLinkedToSource { .. }
            | EffectDef::ReturnLinkedExiles { .. }
            | EffectDef::MakeUnblockableThisTurn { .. }
            | EffectDef::GainControlThisTurn { .. }
            | EffectDef::AtNextStep { .. }
            | EffectDef::IfCondition { .. }
            | EffectDef::TriggerUntilYourNextTurn { .. }
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::PlayersCantPlay(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::Replacement(_)
            | EffectDef::MoveToZone { .. }
            | EffectDef::Attach { .. }
            | EffectDef::CreateToken { .. }
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChoosePlayer { .. }
            | EffectDef::CopyPermanentAsItEnters { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Apply { .. }
            | EffectDef::Special(_) => {
                // Choice-bearing and non-mana primitives need a dedicated
                // immediate procedure before a supported card can use them.
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
                        && self.trigger_event_matches(definition.event, event, source.card.id) =>
                {
                    Some((
                        effective.origin,
                        effective.ability.text,
                        definition.targets,
                        effective.ability.effect.definition,
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
                target_defs: targets,
                effect,
                resolver,
                context: event.context(),
                // A legacy custom trigger states its own condition inside its
                // behavior rather than declaring one here.
                condition: None,
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

    /// Combat damage arriving at the ability's own source. A planeswalker is
    /// dealt combat damage as a permanent, so this is the only shape that can
    /// see it; the player-facing variants read a life total instead.
    pub(super) fn combat_damage_to_source_matches(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
    ) -> bool {
        let (
            TriggerEventDef::CombatDamageDealtToSource { source: predicate },
            CommittedTriggerEvent::DamageDealt {
                source: dealer,
                recipient,
                combat: true,
                ..
            },
        ) = (definition, event)
        else {
            return false;
        };
        *recipient == Target::Permanent(source)
            && self.trigger_object_matches(predicate, dealer, source, false)
    }

    pub(super) fn trigger_event_matches(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
    ) -> bool {
        match (definition, event) {
            (
                TriggerEventDef::ZoneChanged {
                    object: predicate,
                    from,
                    to,
                },
                CommittedTriggerEvent::ZoneChanged {
                    object,
                    from: actual_from,
                    to: actual_to,
                },
            ) => {
                from.is_none_or(|expected| expected == *actual_from)
                    && to.is_none_or(|expected| expected == *actual_to)
                    && self.trigger_object_matches(predicate, object, source, false)
            }
            (
                TriggerEventDef::BecomesTapped(predicate),
                CommittedTriggerEvent::BecomesTapped { object },
            )
            | (
                TriggerEventDef::TappedForMana(predicate),
                CommittedTriggerEvent::TappedForMana { object },
            )
            | (
                TriggerEventDef::CombatDamageDealtToPlayer { source: predicate },
                CommittedTriggerEvent::CombatDamageDealtToPlayer { object, .. },
            ) => self.trigger_object_matches(predicate, object, source, false),
            (
                trigger @ TriggerEventDef::CombatDamageDealtToSource { .. },
                damage @ CommittedTriggerEvent::DamageDealt { .. },
            ) => self.combat_damage_to_source_matches(trigger, damage, source),
            (TriggerEventDef::Attacks(predicate), CommittedTriggerEvent::Attacks { object }) => {
                self.trigger_object_matches(predicate, object, source, false)
            }
            (
                TriggerEventDef::AttacksFirstTimeThisTurn(predicate),
                CommittedTriggerEvent::Attacks { object },
            ) => {
                self.trigger_object_matches(predicate, object, source, false)
                    && self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == object.id)
                        .is_some_and(|permanent| permanent.attacks_this_turn == 1)
            }

            (
                TriggerEventDef::TransformsIntoThisFace,
                CommittedTriggerEvent::Transformed { object },
            ) => object.id == source,
            (
                TriggerEventDef::DamageDealt {
                    source: _,
                    recipient: EffectRecipientDef::Source,
                },
                CommittedTriggerEvent::DamageDealt { recipient, .. },
            ) => *recipient == Target::Permanent(source),
            (
                TriggerEventDef::LifeGained(relation),
                CommittedTriggerEvent::LifeGained { player, .. },
            ) => {
                let controller = self
                    .current_or_last_known_controller(source)
                    .unwrap_or(*player);
                self.player_relation_matches(*player, relation, controller, event.context())
            }
            (
                TriggerEventDef::SpellCast(predicate),
                CommittedTriggerEvent::SpellCast { object },
            ) => self.trigger_object_matches(predicate, object, source, true),
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
                let controller = self
                    .current_or_last_known_controller(source)
                    .unwrap_or(*actual_player);
                self.player_relation_matches(*actual_player, player, controller, event.context())
            }
            (
                TriggerEventDef::DamagedCreatureDied,
                CommittedTriggerEvent::DamagedCreatureDied {
                    source: actual_source,
                    ..
                },
            ) => source == *actual_source,
            _ => self.damage_to_player_trigger_matches(definition, event, source),
        }
    }

    /// The one trigger family that reads both what dealt the damage and who
    /// took it, so "an opponent" excludes a source hitting its own side.
    pub(super) fn damage_to_player_trigger_matches(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
    ) -> bool {
        let (
            TriggerEventDef::DamageDealtToPlayer {
                source: predicate,
                player,
            },
            CommittedTriggerEvent::DamageDealtToPlayer {
                object,
                player: damaged,
                ..
            },
        ) = (definition, event)
        else {
            return false;
        };
        self.trigger_object_matches(predicate, object, source, false)
            && self.player_relation_matches(
                *damaged,
                player,
                object.controller,
                TriggerContext::empty(),
            )
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

    /// Whether `object` satisfies `predicate`. `source` is the ability's own
    /// object, which is what a controller relation is measured against.
    pub(super) fn trigger_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        object: &TriggerEventObject,
        source: GameObjectId,
        is_spell: bool,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Any => true,
            ObjectPredicateDef::Source => object.id == source,
            ObjectPredicateDef::Token => object.token,
            ObjectPredicateDef::HasType(card_type) => object.types.contains(card_type),
            ObjectPredicateDef::HasAnyBasicLandType(land_types) => {
                object.types.contains(CardType::Land)
                    && land_types
                        .iter()
                        .any(|land_type| object.subtypes.contains(&land_type.subtype()))
            }
            ObjectPredicateDef::Spell => is_spell,
            ObjectPredicateDef::NoncreatureSpell => {
                is_spell && !object.types.contains(CardType::Creature)
            }
            ObjectPredicateDef::Color(color) => color
                .color_index()
                .is_some_and(|index| object.colors[index]),
            ObjectPredicateDef::ColorCount(count) => {
                object.colors.iter().filter(|present| **present).count() == usize::from(count)
            }
            ObjectPredicateDef::Subtype(subtype) => object.subtypes.contains(&subtype),
            ObjectPredicateDef::ManaValueAtMost(limit) => object.mana_value <= u16::from(limit),
            ObjectPredicateDef::ManaValueEqualTo(value) => self
                .value_from_source(value, source)
                .is_some_and(|value| value == i32::from(object.mana_value)),
            ObjectPredicateDef::ManaValueAtMostValue(value) => self
                .value_from_source(value, source)
                .is_some_and(|value| i32::from(object.mana_value) <= value),
            ObjectPredicateDef::PowerAtLeast(minimum) => {
                object.power.is_some_and(|power| power >= minimum)
            }
            ObjectPredicateDef::PowerExactly(exact) => object.power == Some(exact),
            ObjectPredicateDef::ToughnessExactly(exact) => object.toughness == Some(exact),
            ObjectPredicateDef::ToughnessLessThan(value) => self
                .value_from_source(value, source)
                .zip(object.toughness)
                .is_some_and(|(limit, toughness)| i32::from(toughness) < limit),
            ObjectPredicateDef::Supertype(supertype) => object.supertypes[supertype.index()],
            // Read from the definition rather than the object: what matters
            // is where the card was first printed, not what it has become.
            ObjectPredicateDef::DebutSet(set) => self
                .object_debut_set(object.id)
                .is_some_and(|debut| debut == set),
            ObjectPredicateDef::AttackingOrBlocking => object.attacking_or_blocking,
            ObjectPredicateDef::SharesNameWithSource => {
                let name = self.object_card_name(object.id);
                name.is_some() && name == self.object_card_name(source)
            }
            ObjectPredicateDef::HasKeyword(keyword) => keyword
                .simple_index()
                .is_some_and(|index| object.keywords & (1 << index) != 0),
            ObjectPredicateDef::HasNonManaActivatedAbility => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == object.id)
                .is_some_and(|permanent| self.has_nonmana_activated_ability(permanent)),
            ObjectPredicateDef::ControlledBy(relation) => {
                self.controller_of_object(source).is_some_and(|controller| {
                    self.player_relation_matches(
                        object.controller,
                        relation,
                        controller,
                        TriggerContext::empty(),
                    )
                })
            }
            ObjectPredicateDef::Attacking | ObjectPredicateDef::AttackedThisTurn => {
                object.types.contains(CardType::Creature) && object.attacking
            }
            ObjectPredicateDef::All(predicates) => predicates
                .iter()
                .all(|predicate| self.trigger_object_matches(*predicate, object, source, is_spell)),
            ObjectPredicateDef::AnyOf(predicates) => predicates
                .iter()
                .any(|predicate| self.trigger_object_matches(*predicate, object, source, is_spell)),
            ObjectPredicateDef::Not(predicate) => {
                !self.trigger_object_matches(*predicate, object, source, is_spell)
            }
            ObjectPredicateDef::Special(_) => false,
        }
    }

    pub(super) fn ability_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        match predicate {
            AbilityTargetPredicate::AnyTarget => {
                let mut targets =
                    vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)];
                targets.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            (self.power(permanent).is_some()
                                || self
                                    .permanent_types(permanent)
                                    .is_some_and(|types| types.contains(CardType::Planeswalker)))
                                && self.permanent_can_be_targeted_by(permanent, controller, source)
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                targets
            }
            AbilityTargetPredicate::ControlledByTargetOf { .. } => Vec::new(),
            AbilityTargetPredicate::PlayerOrPlaneswalker(relation) => {
                let mut targets = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .filter(|player| {
                        self.player_relation_matches(*player, relation, controller, context)
                    })
                    .map(Target::Player)
                    .collect::<Vec<_>>();
                targets.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| {
                            self.permanent_types(permanent)
                                .is_some_and(|types| types.contains(CardType::Planeswalker))
                                && self.permanent_can_be_targeted_by(permanent, controller, source)
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                targets
            }
            AbilityTargetPredicate::Player(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(*player, relation, controller, context)
                })
                .map(Target::Player)
                .collect(),
            AbilityTargetPredicate::Object { .. } => {
                self.ability_object_targets_matching(predicate, controller, source, context)
            }
        }
    }

    pub(super) fn ability_object_targets_matching(
        &self,
        predicate: AbilityTargetPredicate,
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        let AbilityTargetPredicate::Object {
            object,
            zones,
            controller: controller_relation,
            owner: owner_relation,
        } = predicate
        else {
            unreachable!("object-target matching requires an object predicate")
        };
        let mut targets = Vec::new();
        if zones.contains(&ZoneKind::Battlefield) {
            targets.extend(self.battlefield.iter().filter_map(|permanent| {
                let characteristics = self.trigger_event_object(permanent);
                (controller_relation.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.controller,
                        relation,
                        controller,
                        context,
                    )
                }) && owner_relation.is_none_or(|relation| {
                    self.player_relation_matches(
                        permanent.card.owner,
                        relation,
                        controller,
                        context,
                    )
                }) && self.permanent_can_be_targeted_by(permanent, controller, source)
                    && self.trigger_object_matches(object, &characteristics, source, false))
                .then_some(Target::Permanent(permanent.card.id))
            }));
        }
        if zones.contains(&ZoneKind::Stack) {
            targets.extend(self.stack.iter().filter_map(|stack_object| {
                let characteristics = self.stack_trigger_event_object(stack_object)?;
                (stack_object.kind == StackObjectKind::Spell
                    && controller_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.controller,
                            relation,
                            controller,
                            context,
                        )
                    })
                    && owner_relation.is_none_or(|relation| {
                        self.player_relation_matches(
                            stack_object.card.owner,
                            relation,
                            controller,
                            context,
                        )
                    })
                    && self.trigger_object_matches(object, &characteristics, source, true))
                .then_some(Target::Spell(stack_object.id))
            }));
        }
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            if !zones.contains(&zone) || controller_relation.is_some() {
                continue;
            }
            targets.extend(self.cards_in_zone(zone).filter_map(|card| {
                (owner_relation.is_none_or(|relation| {
                    self.player_relation_matches(card.owner, relation, controller, context)
                }) && self.card_object_matches(object, card, zone, source))
                .then_some(Target::Card(card.id))
            }));
        }
        targets
    }

    pub(super) fn cards_in_zone(&self, zone: ZoneKind) -> impl Iterator<Item = &CardInstance> {
        self.players.iter().flat_map(move |player| match zone {
            ZoneKind::Library => player.library.iter(),
            ZoneKind::Hand => player.hand.iter(),
            ZoneKind::Graveyard => player.graveyard.iter(),
            ZoneKind::Exile => player.exile.iter(),
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => [].iter(),
        })
    }

    pub(super) fn card_in_nonbattlefield_zone(
        &self,
        id: GameObjectId,
    ) -> Option<(ZoneKind, &CardInstance)> {
        [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
        ]
        .into_iter()
        .find_map(|zone| {
            self.cards_in_zone(zone)
                .find(|card| card.id == id)
                .map(|card| (zone, card))
        })
    }

    pub(super) fn card_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        card: &CardInstance,
        zone: ZoneKind,
        source: GameObjectId,
    ) -> bool {
        let context = match zone {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Command => CharacteristicContext::Command,
            ZoneKind::Battlefield | ZoneKind::Stack => return false,
        };
        let Some(object) =
            self.printed_trigger_event_object(card.id, card.definition, card.owner, &context)
        else {
            return false;
        };
        self.trigger_object_matches(predicate, &object, source, false)
    }

    pub(super) fn player_relation_matches(
        &self,
        player: PlayerId,
        relation: PlayerRelation,
        controller: PlayerId,
        context: TriggerContext,
    ) -> bool {
        match relation {
            PlayerRelation::Any => true,
            PlayerRelation::You => player == controller,
            PlayerRelation::NotYou => player != controller,
            PlayerRelation::Opponent => player == controller.opponent(),
            PlayerRelation::ActivePlayer => player == self.active_player,
            PlayerRelation::NonactivePlayer => player == self.active_player.opponent(),
            PlayerRelation::EventPlayer => context.event_player == Some(player),
            // The chosen player lives on the ability's source, which this
            // does not have. The one trigger that names it resolves the
            // relation where the source is known.
            PlayerRelation::ChosenPlayer => false,
        }
    }

    /// The player a permanent chose as it entered.
    pub(super) fn chosen_player_of(&self, source: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| permanent.chosen_player)
    }
}
