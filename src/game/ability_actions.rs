use super::{
    AbilityCostDef, AbilityOrigin, AbilityProcedureDef, Action, CardBehavior, CardDefinitionId,
    CardInstance, CardPart, CardStructure, CharacteristicContext, ControlFlow,
    DeclarativeAbilityDef, DoubleFacedKind, EffectiveAbility, FrozenActivatedAbility, Game,
    GameEvent, GameObjectId, ManaCost, ManaPaymentPurpose, ManaPlanOptions, ObjectCharacteristics,
    ObjectInstance, ObjectRefDef, Permanent, PlayerId, RetiredObject, ScopedEffect,
    SelectedSpellPlan, StackAbilityPayload, StackObject, StackObjectKind, TargetSelection,
    TriggerContext, ZoneKind, add_mana_cost, applicable_part_ids_ref, mana_cost_value,
    mode_id_selections,
};
use crate::card::{ActivatedAbilityDef, ObjectPredicateDef, PlayerRelation};
use crate::ids::ModeId;

mod mana_value;
include!("ability_actions/modes.rs");
include!("ability_actions/nonbattlefield.rs");

impl Game {
    /// Resolves object references that are fixed when an ability is
    /// activated, before a stack object or resolution context exists.
    /// Choices, targets, and trigger bindings are intentionally unavailable
    /// here; the ability source and the object that granted it are the two
    /// exact identities an activation cost can already know.
    pub(super) const fn activation_object_reference(
        reference: ObjectRefDef,
        source: GameObjectId,
        origin: AbilityOrigin,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source => Some(source),
            ObjectRefDef::AbilityGrantSource => match origin {
                AbilityOrigin::Granted { source, .. }
                | AbilityOrigin::TokenGranted { source, .. }
                | AbilityOrigin::EmblemGranted { source, .. }
                | AbilityOrigin::FaceDownGranted { source, .. } => Some(source),
                AbilityOrigin::Printed { .. }
                | AbilityOrigin::Token { .. }
                | AbilityOrigin::Emblem { .. }
                | AbilityOrigin::FaceDown { .. }
                | AbilityOrigin::IntrinsicBasicLand(_)
                | AbilityOrigin::IntrinsicCounter(_) => None,
            },
            ObjectRefDef::ResolvingObject
            | ObjectRefDef::CreatingSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::AttachedToSource
            | ObjectRefDef::Target(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject
            | ObjectRefDef::SourceOfTargetedStackObject(_) => None,
        }
    }

    pub(super) fn push_activated_ability(
        &mut self,
        source: GameObjectId,
        source_card: &ObjectInstance,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
    ) -> GameObjectId {
        let mut context: super::EffectResolutionContext = TriggerContext::empty().into();
        if let Some((binding, chosen)) = frozen.definition.as_ref().and_then(|ability| {
            let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                return None;
            };
            definition.costs.iter().find_map(|cost| {
                let AbilityCostDef::MoveToZone(movement) = cost else {
                    return None;
                };
                Some((movement.binding?, *chosen_permanents.last()?))
            })
        }) {
            // The chosen payment object is already gone by the time the
            // stack object is built. Bind the exact retired incarnation;
            // effects that name the card it became ask for its zone-change
            // successor explicitly.
            context.bind_single_object(binding, self.object_target_with_lki(chosen));
        }
        self.push_activated_ability_with_context(
            source,
            source_card.owner,
            controller,
            frozen,
            targets,
            chosen_permanents,
            context,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn push_activated_ability_with_context(
        &mut self,
        source: GameObjectId,
        source_owner: PlayerId,
        controller: PlayerId,
        frozen: FrozenActivatedAbility,
        targets: Vec<TargetSelection>,
        chosen_permanents: Vec<GameObjectId>,
        context: super::EffectResolutionContext,
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
            // Exhaust is spent rather than counted: what matters afterwards
            // is only that it happened.
            let exhausts = frozen.definition.as_ref().is_some_and(|definition| {
                matches!(
                    definition.definition,
                    DeclarativeAbilityDef::Activated(activated)
                        | DeclarativeAbilityDef::ActivatedMana(activated)
                        if activated.exhaust
                )
            });
            if exhausts && !permanent.exhausted.contains(&frozen.origin) {
                permanent.exhausted.push(frozen.origin);
            }
        }
        let event_chosen_permanents = chosen_permanents.clone();
        let card = self.unbacked_ability_object(frozen.presentation, source_owner);
        let id = card.id;
        // The activation's targets are locked in here, which is where a
        // crime is committed if any of them belongs to an opponent.
        let crime_targets = targets
            .iter()
            .flat_map(TargetSelection::targets)
            .copied()
            .collect::<Vec<_>>();
        self.stack.push(StackObject {
            id,
            kind: StackObjectKind::ActivatedAbility,
            card,
            source: Some(source),
            ability: Some(StackAbilityPayload {
                origin: frozen.origin,
                definition: frozen.definition,
                presentation: frozen.presentation,
                text: frozen.text,
                target_defs: frozen.target_defs,
                targets,
                context,
                resolver: frozen.resolver,
                // Only a triggered ability carries an intervening-if.
                condition: None,
                mode_effects: frozen.mode_effects,
                resolution_destination: None,
                x: frozen.x,
                sacrificed_mana_value: frozen.sacrificed_mana_value,
            }),
            controller,
            signature: None,
            chosen_permanents,
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast: None,
            face_down: None,
            is_copy: false,
        });
        self.events.push(GameEvent::AbilityActivated {
            player: controller,
            object: id,
            source,
            presentation: frozen.presentation,
            chosen_permanents: event_chosen_permanents,
        });
        self.capture_crime_triggers(controller, &crime_targets);
        self.capture_ability_targeting_triggers(id);
        id
    }

    /// Whether any of this permanent's abilities is printed as open to every
    /// player, which is what puts somebody else's permanent in a player's
    /// action list at all.
    fn has_open_activated_ability(&self, permanent: &Permanent) -> bool {
        let mut open = false;
        self.for_each_effective_ability(permanent, |effective| {
            if let DeclarativeAbilityDef::Activated(definition) = effective.ability.definition
                && definition.any_player_may_activate
                && effective.ability.is_executable()
            {
                open = true;
            }
        });
        open
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn add_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        // Mana abilities are enumerated elsewhere. Split second prohibits
        // every nonmana activation, while ordinary static restrictions are
        // matched against each prospective source below.
        if self.split_second_is_active() {
            return;
        }
        for permanent in self.battlefield.iter().filter(|permanent| {
            // A permanent somebody else controls contributes only the
            // abilities printed as open to everyone.
            permanent.controller == player || self.has_open_activated_ability(permanent)
        }) {
            if self.nonmana_ability_activation_is_prohibited(player, permanent) {
                continue;
            }
            // A permanent-wide prohibition stops every activation it could
            // contribute, including open and legacy abilities.
            if self.activated_abilities_are_prohibited(permanent) {
                continue;
            }
            let only_open_abilities = permanent.controller != player;
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
                if only_open_abilities && !definition.any_player_may_activate {
                    return;
                }
                // Copy-process exceptions can add an activated ability
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
                    // Detain stops activated abilities, not the permanent's
                    // other clauses. An Aura saying so directly is the same
                    // prohibition without the deadline.
                    || permanent.detained_until_turn_of.is_some()
                    || !self.activation_timing_allows(player, definition.timing)
                    // The engine already counts every activation per ability
                    // and clears the counts each turn, so the printed cap is
                    // a read rather than new state.
                    || definition.activation_limit.is_some_and(|limit| {
                        permanent.activations_this_turn.iter().any(|(origin, count)| {
                            *origin == effective.origin && *count >= limit
                        })
                    })
                    // Exhaust, which the permanent remembers for as long as
                    // it is there rather than for the turn.
                    || (definition.exhaust && permanent.exhausted.contains(&effective.origin))
                    // "Activate only if ...". A false condition means there is
                    // no legal activation at all, rather than one that
                    // resolves and does nothing.
                    || definition.condition.is_some_and(|condition| {
                        !self.trigger_condition_holds(
                            condition,
                            permanent.card.id,
                            permanent.controller,
                            TriggerContext::empty(),
                            Some(effective.origin),
                            None,
                        )
                    })
                {
                    return;
                }
                if definition.procedure == AbilityProcedureDef::Legacy {
                    if let Some(behavior) = ability.custom_behavior() {
                        legacy_activations.push((effective.origin, definition, behavior));
                    }
                    return;
                }
                let mut fixed_sacrifices = Vec::new();
                for cost in definition.costs.as_slice() {
                    let AbilityCostDef::SacrificeObject(reference) = cost else {
                        continue;
                    };
                    let Some(sacrificed) = Self::activation_object_reference(
                        *reference,
                        permanent.card.id,
                        effective.origin,
                    ) else {
                        return;
                    };
                    let controlled_permanent = self.battlefield.iter().any(|candidate| {
                        candidate.card.id == sacrificed && candidate.controller == player
                    });
                    if !controlled_permanent || fixed_sacrifices.contains(&sacrificed) {
                        return;
                    }
                    fixed_sacrifices.push(sacrificed);
                }
                let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
                let untaps_source = definition.costs.contains(&AbilityCostDef::UntapSource);
                let leaves_source = definition.costs.iter().any(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificeSource
                            | AbilityCostDef::ExileSource
                            | AbilityCostDef::ReturnSourceToHand
                    )
                }) || fixed_sacrifices.contains(&permanent.card.id);
                // The same purpose the payment will use, so an ability that
                // taps its own source is never offered on mana only that
                // source could have made.
                let payment_purpose = ManaPaymentPurpose::Ability {
                    source: permanent.card.id,
                    taps_source,
                    leaves_source,
                };
                if (taps_source
                    && (permanent.tapped || !self.can_use_tap_or_untap_ability(permanent)))
                    || (untaps_source
                        && (!permanent.tapped || !self.can_use_tap_or_untap_ability(permanent)))
                    || !Self::source_counter_costs_are_payable(
                        permanent,
                        definition.costs.as_slice(),
                    )
                    || definition.costs.iter().any(|cost| match cost {
                        // A flexible symbol has more than one way to be paid,
                        // so what makes the cost payable is that one of them
                        // is -- paying 2 life for a Phyrexian symbol counts
                        // even with no mana of that colour anywhere.
                        AbilityCostDef::Mana(cost) => self
                            .affordable_activation_payments(
                                player,
                                self.activation_mana_cost(&definition, permanent.card.id, *cost),
                                0,
                                &payment_purpose,
                            )
                            .is_empty(),
                        AbilityCostDef::PayLife(amount) => {
                            !self.can_pay_life(player, *amount)
                        }
                        // Nobody chooses, so the only question is whether the
                        // hand is big enough to pay.
                        AbilityCostDef::DiscardCardsAtRandom(amount) => {
                            self.players[player.index()].hand.len() < usize::from(*amount)
                        }
                        AbilityCostDef::MillCards(amount) => {
                            self.players[player.index()].library.len() < usize::from(*amount)
                        }
                        // Crew and saddle: what makes it payable is whether
                        // the other untapped creatures add up.
                        AbilityCostDef::TapCreaturesWithTotalPower { minimum } => !self
                            .can_pay_total_power_tap(player, permanent.card.id, *minimum),
                        AbilityCostDef::TapPermanents {
                            object,
                            controller,
                            count,
                        } => !self.can_pay_tap_permanents(
                            player,
                            permanent.card.id,
                            *object,
                            *controller,
                            *count,
                            taps_source || leaves_source,
                        ),
                        // A loyalty ability is sorcery speed, once per turn,
                        // and only removes counters the permanent has.
                        AbilityCostDef::Loyalty(change) => {
                            !self.can_activate_loyalty(permanent, player, *change)
                        }
                        // Never reaches payability: enumeration has
                        // already replaced it with a sized removal.
                        AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                        | AbilityCostDef::RemoveCountersFromSource { .. }
                        // Always payable: a hand of nothing discards nothing,
                        // which is a legal way to pay it.
                        | AbilityCostDef::DiscardHand
                        | AbilityCostDef::ManaCostOf(_)
                        | AbilityCostDef::ManaValueOfTarget { .. }
                        | AbilityCostDef::TapSource
                        // Always payable: what it spends is a future untap
                        // step, and the permanent has one whatever state it
                        // is in now.
                        | AbilityCostDef::ExertSource
                        | AbilityCostDef::UntapSource
                        | AbilityCostDef::SacrificeSource
                        | AbilityCostDef::SacrificeObject(_)
                        | AbilityCostDef::ReturnSourceToHand
                        | AbilityCostDef::ExileSource
                        | AbilityCostDef::SacrificePermanent { .. }
                        | AbilityCostDef::SacrificePermanents { .. }
                        | AbilityCostDef::ReturnUnblockedAttackerToHand
                        // Payability is decided by whether any card qualifies,
                        // which the choice list below answers.
                        | AbilityCostDef::MoveToZone(_)
                        | AbilityCostDef::DiscardCardMatching(_)
                        | AbilityCostDef::ExileCardFromHand(_) => false,
                        AbilityCostDef::DiscardSource
                        | AbilityCostDef::DiscardCards(_)
                        | AbilityCostDef::Special(_) => true,
                    })
                {
                    return;
                }
                let source_exit_costs = definition
                    .costs
                    .iter()
                    .filter(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::SacrificeSource
                                | AbilityCostDef::ExileSource
                                | AbilityCostDef::ReturnSourceToHand
                        )
                    })
                    .count()
                    + usize::from(fixed_sacrifices.contains(&permanent.card.id));
                if source_exit_costs > 1 {
                    return;
                }
                // At most one cost names an object, so one activation per
                // eligible object covers every choice the player has.
                let mut object_costs = definition.costs.iter().filter(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificePermanent { .. }
                            | AbilityCostDef::SacrificePermanents { .. }
                            | AbilityCostDef::ReturnUnblockedAttackerToHand
                            | AbilityCostDef::TapPermanents { .. }
                            | AbilityCostDef::MoveToZone(_)
                            | AbilityCostDef::DiscardCardMatching(_)
                            | AbilityCostDef::ExileCardFromHand(_)
                    )
                });
                let object_cost = object_costs.next();
                if object_costs.next().is_some() {
                    return;
                }
                let taps_chosen_permanent = matches!(
                    object_cost,
                    Some(AbilityCostDef::TapPermanents { count: 1, .. })
                );
                let cost_object_choices = match object_cost {
                    Some(AbilityCostDef::SacrificePermanent { object, controller }) => self
                        .battlefield
                        .iter()
                        .filter(|candidate| {
                            (source_exit_costs != 1 || candidate.card.id != permanent.card.id)
                                && !fixed_sacrifices.contains(&candidate.card.id)
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
                        .map(|candidate| vec![candidate.card.id])
                        .collect(),
                    // One payer travels with the activation so mana planning
                    // can reserve it. The source may pay unless another cost
                    // has already committed it to tap or leave play.
                    Some(AbilityCostDef::TapPermanents {
                        object,
                        controller,
                        count: 1,
                    }) => self
                        .activation_tap_candidates(
                            player,
                            *object,
                            *controller,
                            permanent.card.id,
                            &fixed_sacrifices,
                            taps_source || leaves_source,
                        )
                        .into_iter()
                        .map(|candidate| vec![candidate])
                        .collect(),
                    // Larger exact-count tap costs make their choices through
                    // a bounded decision after the activation is selected.
                    None | Some(AbilityCostDef::TapPermanents { .. }) => vec![Vec::new()],
                    // The one cost that can name more than one card, so every
                    // combination of that many is its own activation.
                    Some(AbilityCostDef::MoveToZone(movement)) => {
                        let candidates = match movement.from {
                            ZoneKind::Hand => &self.players[player.index()].hand,
                            ZoneKind::Graveyard => &self.players[player.index()].graveyard,
                            _ => return,
                        };
                        let candidates: Vec<GameObjectId> = candidates
                            .iter()
                            .filter(|card| {
                                self.card_object_matches(
                                    movement.object,
                                    card,
                                    movement.from,
                                    permanent.card.id,
                                )
                            })
                            .map(|card| card.id)
                            .collect();
                        let Some(count) = movement.fixed_count() else {
                            return;
                        };
                        Self::object_combinations(&candidates, usize::from(count))
                    }
                    Some(AbilityCostDef::DiscardCardMatching(object)) => self.players
                        [player.index()]
                    .hand
                    .iter()
                    .filter(|card| {
                        self.card_object_matches(*object, card, ZoneKind::Hand, permanent.card.id)
                    })
                    .map(|card| vec![card.id])
                    .collect(),
                    Some(AbilityCostDef::ExileCardFromHand(object)) => self.players[player.index()]
                        .hand
                        .iter()
                        .filter(|card| {
                            self.card_object_matches(
                                *object,
                                card,
                                ZoneKind::Hand,
                                permanent.card.id,
                            )
                        })
                        .map(|card| vec![card.id])
                        .collect(),
                    // Paid by a decision rather than by enumeration, so the
                    // activation names none of them: one offer stands for
                    // however many ways there are to pay it.
                    Some(AbilityCostDef::SacrificePermanents {
                        object,
                        controller,
                        count,
                    }) => {
                        let available = self
                            .battlefield
                            .iter()
                            .filter(|candidate| {
                                !fixed_sacrifices.contains(&candidate.card.id)
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
                            .count();
                        if available >= usize::from(*count) {
                            vec![Vec::new()]
                        } else {
                            Vec::new()
                        }
                    }
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
                        AbilityCostDef::Mana(cost) if cost.variable_x => Some(self.maximum_x_for(
                            player,
                            self.activation_mana_cost(&definition, permanent.card.id, *cost),
                            &payment_purpose,
                        )),
                        _ => None,
                    })
                    .unwrap_or(0);
                // X is the outer loop because a slot may count or divide by
                // it: "X target lands" offers a different set of declarations
                // for each affordable X, so the targets have to be enumerated
                // inside that loop rather than once for all of them.
                // "Choose one --" is answered as the ability is activated, so
                // each way of answering is its own action, with that mode's
                // targets appended to the ability's own.
                let mode_selections = Self::activated_mode_selections(&definition);
                for x in 0..=max_x {
                    for selected_modes in &mode_selections {
                        let Some(plan) = Self::selected_activated_plan(&definition, selected_modes)
                        else {
                            continue;
                        };
                        for selections in self.legal_activation_target_selections(
                            &plan.target_defs,
                            player,
                            permanent.card.id,
                            TriggerContext::empty(),
                            x,
                        ) {
                            if ability
                                .declarative_effect()
                                .is_some_and(Self::effect_is_reconfigure)
                                && permanent.attached_to.is_none()
                                && selections
                                    .iter()
                                    .all(|selection| selection.targets().is_empty())
                            {
                                continue;
                            }
                            for cost_objects in &cost_object_choices {
                                let payable_mana_cost = self
                                    .activated_ability_mana_cost_for(
                                        &definition,
                                        &selections,
                                        cost_objects,
                                    )
                                    .map(|cost| {
                                        self.activation_mana_cost(
                                            &definition,
                                            permanent.card.id,
                                            cost,
                                        )
                                    });
                                if definition.costs.iter().any(|cost| {
                                    matches!(
                                        cost,
                                        AbilityCostDef::ManaCostOf(_)
                                            | AbilityCostDef::ManaValueOfTarget { .. }
                                    )
                                }) && payable_mana_cost.is_none()
                                {
                                    continue;
                                }
                                // A permanent tapped as part of this cost cannot
                                // also activate its tap-for-mana ability. Other
                                // object costs deliberately remain available as
                                // mana sources because they may be paid after
                                // producing mana.
                                if let (true, Some(cost), Some(tap_cost_payer)) = (
                                    taps_chosen_permanent,
                                    payable_mana_cost,
                                    cost_objects.first().copied(),
                                ) && self
                                    .plan_mana_activations_with_options_for(
                                        player,
                                        cost,
                                        x,
                                        ManaPlanOptions {
                                            avoid: None,
                                            tap_cost_payer: Some(tap_cost_payer),
                                        },
                                        &payment_purpose,
                                    )
                                    .is_none()
                                {
                                    continue;
                                }
                                // Each way of announcing the flexible
                                // symbols is its own activation, the way each
                                // affordable X and each chosen cost object is.
                                for payment in payable_mana_cost.map_or_else(
                                    || vec![None],
                                    |cost| {
                                        self.affordable_activation_payments(
                                            player,
                                            cost,
                                            x,
                                            &payment_purpose,
                                        )
                                    },
                                ) {
                                    actions.push(Action::ActivateAbility {
                                        source: permanent.card.id,
                                        ability: effective.origin,
                                        targets: selections.clone(),
                                        cost_objects: cost_objects.clone(),
                                        x,
                                        modes: selected_modes.clone(),
                                        mana_payment: payment.map(Box::new),
                                    });
                                }
                            }
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
        self.add_graveyard_ability_actions(player, actions);
        self.add_exile_ability_actions(player, actions);
        self.add_ongoing_effect_ability_actions(player, actions);
    }

    /// Activations supplied by duration-scoped effects. These sources are
    /// classified as command-zone objects for source-zone checks, but are not
    /// emblems and never join the battlefield ability-layer walk.
    fn add_ongoing_effect_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for ongoing in self
            .ongoing_effects
            .iter()
            .filter(|ongoing| ongoing.controller == player)
        {
            let DeclarativeAbilityDef::Activated(definition) = ongoing.ability.definition else {
                continue;
            };
            if !ongoing.ability.is_executable()
                || definition.procedure != AbilityProcedureDef::Shared
                || definition.source_zones != [ZoneKind::Command]
                || !self.activation_timing_allows(player, definition.timing)
            {
                continue;
            }
            let Some(cost) = Self::activated_ability_mana_cost(&definition) else {
                continue;
            };
            let purpose = ManaPaymentPurpose::Ability {
                source: ongoing.source.object,
                taps_source: false,
                leaves_source: false,
            };
            if self.can_pay_cost_for(player, cost, 0, &purpose) {
                actions.push(Action::ActivateAbility {
                    source: ongoing.source.object,
                    ability: ongoing.source.ability,
                    targets: Vec::new(),
                    cost_objects: Vec::new(),
                    x: 0,
                    modes: Vec::new(),
                    mana_payment: None,
                });
            }
        }
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
            CardBehavior::LibraryOfAlexandria
                if !permanent.tapped
                    && self.can_use_tap_or_untap_ability(permanent)
                    && self.players[player.index()].hand.len() == 7 =>
            {
                actions.push(Action::ActivateAbility {
                    source: permanent.card.id,
                    ability,
                    targets: Vec::new(),
                    cost_objects: Vec::new(),
                    x: 0,
                    modes: Vec::new(),
                    mana_payment: None,
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
        let Ok(parts) = applicable_part_ids_ref(definition, context) else {
            return ControlFlow::Continue(());
        };
        for part in parts.iter().copied() {
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

    pub(super) fn behavior(&self, definition: CardDefinitionId) -> Option<CardBehavior> {
        self.catalog
            .get(definition)
            .and_then(|card| card.rules.special_behavior())
    }
}
