use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, Action, CardBehavior, CardDefinitionId,
    CardInstance, CardPart, CardStructure, CharacteristicContext, CharacteristicSource,
    ControlFlow, DeclarativeAbilityDef, DoubleFacedKind, EffectiveAbility, FrozenActivatedAbility,
    Game, GameEvent, GameObjectId, ManaCost, ManaPaymentPurpose, Permanent, PlayerId,
    RetiredObject, StackAbilityPayload, StackObject, StackObjectKind, TargetSelection,
    TriggerContext, ZoneKind, add_mana_cost, applicable_part_ids, mana_cost_value,
};

impl Game {
    pub(super) fn push_activated_ability(
        &mut self,
        source: GameObjectId,
        source_card: &CardInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
    ) -> GameObjectId {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        {
            match permanent
                .activations_this_turn
                .iter_mut()
                .find(|(origin, _)| *origin == frozen.origin)
            {
                Some((_, count)) => *count = count.saturating_add(1),
                None => permanent.activations_this_turn.push((frozen.origin, 1)),
            }
        }
        let event_chosen_permanents = chosen_permanents.clone();
        let card = self.unbacked_object(
            frozen.presentation_definition,
            source_card.owner,
            CharacteristicSource::Ability(frozen.presentation_definition),
        );
        let id = card.id;
        self.stack.push(StackObject {
            id,
            kind: StackObjectKind::ActivatedAbility,
            card,
            source: Some(source),
            ability: Some(StackAbilityPayload {
                origin: frozen.origin,
                definition: frozen.definition,
                presentation_definition: frozen.presentation_definition,
                text: frozen.text,
                target_defs: frozen.target_defs.to_vec(),
                targets,
                context: TriggerContext::empty(),
                resolver: frozen.resolver,
                // Only a triggered ability carries an intervening-if.
                condition: None,
                mode_effects: Vec::new(),
                x: frozen.x,
            }),
            controller,
            signature: None,
            chosen_permanents,
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            is_copy: false,
        });
        self.events.push(GameEvent::AbilityActivated {
            player: controller,
            object: id,
            source,
            definition: frozen.presentation_definition,
            chosen_permanents: event_chosen_permanents,
        });
        id
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn add_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            // Mana abilities are exempt, and they are enumerated elsewhere,
            // so a named source contributes no actions from here at all.
            if self.activated_abilities_are_named(permanent) {
                continue;
            }
            let mut legacy_activations = Vec::new();
            let mut untyped_legacy_activation = None;
            let mut last_activated_origin = None;
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                if ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Legacy)
                    && untyped_legacy_activation.is_none()
                {
                    untyped_legacy_activation = ability
                        .custom_behavior()
                        .map(|behavior| (effective.origin, behavior));
                }
                let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                    return;
                };
                // Copy-process exceptions can retain an activated ability
                // whose structural origin is already present in the copied
                // values. Actions identify an ability by that origin, so a
                // consecutive repeat is externally indistinguishable and
                // would resolve through the first matching ability anyway.
                if last_activated_origin == Some(effective.origin) {
                    return;
                }
                last_activated_origin = Some(effective.origin);
                if !ability.is_executable()
                    || !definition.source_zones.contains(&ZoneKind::Battlefield)
                {
                    return;
                }
                if definition.procedure == AbilityProcedureDef::Legacy {
                    if let Some(behavior) = ability.custom_behavior() {
                        legacy_activations.push((effective.origin, definition, behavior));
                    }
                    return;
                }
                let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
                // The same purpose the payment will use, so an ability that
                // taps its own source is never offered on mana only that
                // source could have made.
                let payment_purpose = ManaPaymentPurpose::Ability {
                    source: permanent.card.id,
                    taps_source,
                };
                if (taps_source && (permanent.tapped || !self.can_use_tap_ability(permanent)))
                    || !Self::source_counter_costs_are_payable(
                        permanent,
                        definition.costs.as_slice(),
                    )
                    || definition.costs.iter().any(|cost| match cost {
                        AbilityCostDef::Mana(cost) => {
                            !self.can_pay_cost_for(player, *cost, 0, &payment_purpose)
                        }
                        AbilityCostDef::PayLife(amount) => {
                            self.players[player.index()].life
                                < i16::try_from(*amount).unwrap_or(i16::MAX)
                        }
                        // A loyalty ability is sorcery speed, once per turn,
                        // and only removes counters the permanent has.
                        AbilityCostDef::Loyalty(change) => {
                            !self.can_activate_loyalty(permanent, player, *change)
                        }
                        AbilityCostDef::RemoveCountersFromSource { .. }
                        | AbilityCostDef::TapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::SacrificePermanent { .. }
                        // Payability is decided by whether any card qualifies,
                        // which the choice list below answers.
                        | AbilityCostDef::ExileCardFromGraveyard(_) => false,
                        AbilityCostDef::UntapSource
                        | AbilityCostDef::DiscardSource
                        | AbilityCostDef::DiscardCards(_)
                        | AbilityCostDef::ExileSource
                        | AbilityCostDef::Special(_) => true,
                    })
                {
                    return;
                }
                let sacrifice_source_costs = definition
                    .costs
                    .iter()
                    .filter(|cost| matches!(cost, AbilityCostDef::SacrificeSource))
                    .count();
                if sacrifice_source_costs > 1 {
                    return;
                }
                // At most one cost names an object, so one activation per
                // eligible object covers every choice the player has.
                let mut object_costs = definition.costs.iter().filter(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificePermanent { .. }
                            | AbilityCostDef::ExileCardFromGraveyard(_)
                    )
                });
                let object_cost = object_costs.next();
                if object_costs.next().is_some() {
                    return;
                }
                let cost_object_choices = match object_cost {
                    None => vec![None],
                    Some(AbilityCostDef::SacrificePermanent { object, controller }) => self
                        .battlefield
                        .iter()
                        .filter(|candidate| {
                            !(sacrifice_source_costs == 1 && candidate.card.id == permanent.card.id)
                                && self.player_relation_matches(
                                    candidate.controller,
                                    *controller,
                                    player,
                                    TriggerContext::empty(),
                                )
                                && self.trigger_object_matches(
                                    *object,
                                    &self.trigger_event_object(candidate),
                                    permanent.card.id,
                                    false,
                                )
                        })
                        .map(|candidate| Some(candidate.card.id))
                        .collect(),
                    Some(AbilityCostDef::ExileCardFromGraveyard(object)) => self.players
                        [player.index()]
                    .graveyard
                    .iter()
                    .filter(|card| {
                        self.card_object_matches(
                            *object,
                            card,
                            ZoneKind::Graveyard,
                            permanent.card.id,
                        )
                    })
                    .map(|card| Some(card.id))
                    .collect(),
                    Some(_) => unreachable!("the filter admits only object costs"),
                };
                if cost_object_choices.is_empty() {
                    return;
                }
                // A variable cost offers one activation per affordable X.
                // Zero is always among them, and the affordability check above
                // already proved the rest of the cost is payable there.
                let max_x = definition
                    .costs
                    .iter()
                    .find_map(|cost| match cost {
                        AbilityCostDef::Mana(cost) if cost.variable_x => {
                            Some(self.maximum_x_for(player, *cost, &payment_purpose))
                        }
                        _ => None,
                    })
                    .unwrap_or(0);
                for selections in self.legal_ability_target_selections(
                    definition.targets,
                    player,
                    permanent.card.id,
                    TriggerContext::empty(),
                    // Targets are enumerated once for every affordable X, so
                    // a slot that divided X would need the enumeration inside
                    // that loop. The boundary test rejects one until it is.
                    0,
                ) {
                    for cost_object in &cost_object_choices {
                        for x in 0..=max_x {
                            actions.push(Action::ActivateAbility {
                                source: permanent.card.id,
                                ability: effective.origin,
                                targets: selections.clone(),
                                cost_object: *cost_object,
                                x,
                            });
                        }
                    }
                }
            });
            if let Some((origin, behavior)) = untyped_legacy_activation {
                self.add_legacy_activated_actions(player, permanent, origin, behavior, actions);
            }
            for (origin, _definition, behavior) in legacy_activations {
                self.add_legacy_activated_actions(player, permanent, origin, behavior, actions);
            }
        }
        self.add_hand_ability_actions(player, actions);
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn add_legacy_activated_actions(
        &self,
        player: PlayerId,
        permanent: &Permanent,
        ability: AbilityOrigin,
        behavior: CardBehavior,
        actions: &mut Vec<Action>,
    ) {
        match behavior {
            CardBehavior::SedgeTroll
                if self.can_pay_cost(player, ManaCost::colored(0, 0, 0, 1, 0, 0), 0) =>
            {
                actions.push(Action::ActivateAbility {
                    source: permanent.card.id,
                    ability,
                    targets: Vec::new(),
                    cost_object: None,
                    x: 0,
                });
            }
            CardBehavior::LibraryOfAlexandria
                if !permanent.tapped
                    && self.can_use_tap_ability(permanent)
                    && self.players[player.index()].hand.len() == 7 =>
            {
                actions.push(Action::ActivateAbility {
                    source: permanent.card.id,
                    ability,
                    targets: Vec::new(),
                    cost_object: None,
                    x: 0,
                });
            }
            CardBehavior::TimeVault if !permanent.tapped && self.can_use_tap_ability(permanent) => {
                actions.push(Action::ActivateAbility {
                    source: permanent.card.id,
                    ability,
                    targets: Vec::new(),
                    cost_object: None,
                    x: 0,
                });
            }
            _ => {}
        }
    }

    pub(super) fn visit_printed_card_abilities(
        &self,
        card: &CardInstance,
        context: &CharacteristicContext,
        mut visitor: impl FnMut(EffectiveAbility) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(definition) = self.catalog.get(card.definition) else {
            return ControlFlow::Continue(());
        };
        let Ok(parts) = applicable_part_ids(definition, context) else {
            return ControlFlow::Continue(());
        };
        for part in parts {
            let Some(part_definition) = definition.part(part) else {
                continue;
            };
            for attached in part_definition.rules.indexed_abilities() {
                if visitor(EffectiveAbility {
                    origin: AbilityOrigin::Printed {
                        definition: definition.id,
                        part,
                        ability: attached.id,
                    },
                    ability: attached.definition,
                })
                .is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn for_each_printed_card_ability(
        &self,
        card: &CardInstance,
        context: &CharacteristicContext,
        mut visitor: impl FnMut(EffectiveAbility),
    ) {
        let result = self.visit_printed_card_abilities(card, context, |effective| {
            visitor(effective);
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue());
    }

    pub(super) fn find_printed_card_ability(
        &self,
        card: &CardInstance,
        context: &CharacteristicContext,
        mut predicate: impl FnMut(EffectiveAbility) -> bool,
    ) -> Option<EffectiveAbility> {
        let mut found = None;
        let _ = self.visit_printed_card_abilities(card, context, |effective| {
            if predicate(effective) {
                found = Some(effective);
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        found
    }

    pub(super) fn add_hand_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for card in &self.players[player.index()].hand {
            self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                    return;
                };
                if !ability.is_executable()
                    || definition.procedure != AbilityProcedureDef::Shared
                    || !definition.source_zones.contains(&ZoneKind::Hand)
                {
                    return;
                }
                let mut mana_cost = ManaCost::default();
                let mut supported = true;
                for cost in definition.costs.as_slice() {
                    match cost {
                        AbilityCostDef::Mana(cost) => {
                            mana_cost = add_mana_cost(mana_cost, *cost);
                        }
                        AbilityCostDef::DiscardSource => {}
                        AbilityCostDef::TapSource
                        | AbilityCostDef::UntapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::RemoveCountersFromSource { .. }
                        | AbilityCostDef::PayLife(_)
                        | AbilityCostDef::DiscardCards(_)
                        | AbilityCostDef::SacrificePermanent { .. }
                        | AbilityCostDef::ExileSource
                        | AbilityCostDef::Loyalty(_)
                        | AbilityCostDef::ExileCardFromGraveyard(_)
                        | AbilityCostDef::Special(_) => supported = false,
                    }
                }
                let payment_purpose = ManaPaymentPurpose::Ability {
                    source: card.id,
                    taps_source: false,
                };
                if !supported || !self.can_pay_cost_for(player, mana_cost, 0, &payment_purpose) {
                    return;
                }
                let max_x = if mana_cost.variable_x {
                    self.maximum_x_for(player, mana_cost, &payment_purpose)
                } else {
                    0
                };
                for targets in self.legal_ability_target_selections(
                    definition.targets,
                    player,
                    card.id,
                    TriggerContext::empty(),
                    0,
                ) {
                    for x in 0..=max_x {
                        actions.push(Action::ActivateAbility {
                            source: card.id,
                            ability: effective.origin,
                            targets: targets.clone(),
                            cost_object: None,
                            x,
                        });
                    }
                }
            });
        }
    }

    pub(super) fn behavior(&self, definition: CardDefinitionId) -> Option<CardBehavior> {
        self.catalog
            .get(definition)
            .and_then(|card| card.rules.special_behavior())
    }

    pub(super) fn permanent_mana_value(&self, permanent: &Permanent) -> u16 {
        // A transforming double-faced permanent keeps the mana value of its
        // front face while its back face is up. A permanent merely copying a
        // back face is not itself that transforming double-faced card, so its
        // copied characteristics continue through the ordinary path below.
        if permanent.copied_from.is_none()
            && let Some(definition) = self.catalog.get(permanent.card.definition)
            && let CardStructure::DoubleFaced {
                front,
                kind: DoubleFacedKind::Transforming,
                ..
            } = &definition.structure
        {
            return definition
                .part(*front)
                .map_or(0, |part| part.rules.printed_mana_cost().mana_value());
        }
        self.effective_rules(permanent)
            .map_or(0, |rules| rules.printed_mana_cost().mana_value())
    }

    /// A permanent or spell's mana value, still readable after it has left
    /// its zone so a later effect in the same sequence can measure it.
    pub(super) fn current_or_last_known_mana_value(&self, id: GameObjectId) -> Option<u16> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
        {
            return Some(self.permanent_mana_value(permanent));
        }
        if let Some(object) = self.stack.iter().find(|object| object.id == id) {
            return Some(self.stack_spell_mana_value(object));
        }
        if let Some((_, card)) = self.card_in_nonbattlefield_zone(id) {
            return self
                .catalog
                .get(card.definition)
                .map(|definition| definition.rules.printed_mana_cost().mana_value());
        }
        match self.retired_objects.get(&id) {
            Some(RetiredObject::Permanent { mana_value, .. }) => Some(*mana_value),
            Some(RetiredObject::Stack(object)) => Some(self.stack_spell_mana_value(object)),
            Some(RetiredObject::Card(card)) => self
                .catalog
                .get(card.definition)
                .map(|definition| definition.rules.printed_mana_cost().mana_value()),
            None => None,
        }
    }

    pub(super) fn stack_spell_mana_value(&self, object: &StackObject) -> u16 {
        let Some(definition) = self.catalog.get(object.card.definition) else {
            return 0;
        };
        let Some(signature) = &object.signature else {
            return 0;
        };
        match signature.form() {
            crate::card::SpellForm::Part(part) => definition
                .part(*part)
                .and_then(CardPart::mana_cost)
                .map_or(0, mana_cost_value),
            crate::card::SpellForm::Combined(parts) => parts
                .iter()
                .filter_map(|part| definition.part(*part).and_then(CardPart::mana_cost))
                .map(mana_cost_value)
                .fold(0, u16::saturating_add),
        }
    }
}
