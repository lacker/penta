// Where a card that is not a permanent offers its activated abilities: a
// hand, for the ninjutsu and channel costs paid from there, and a graveyard.
// Included textually into `ability_actions.rs`, so the imports here are that
// module's.

impl Game {
    pub(super) fn add_exile_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for card in &self.players[player.index()].exile {
            if self.nonbattlefield_ability_activation_is_prohibited(
                player,
                card,
                &CharacteristicContext::Exile,
            ) {
                continue;
            }
            self.for_each_printed_card_ability(card, &CharacteristicContext::Exile, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                    return;
                };
                if definition.procedure != AbilityProcedureDef::Shared
                    || !definition.source_zones.contains(&ZoneKind::Exile)
                    || !self.activation_timing_allows(player, definition.timing)
                    || definition.condition.is_some_and(|condition| {
                        !self.trigger_condition_holds(
                            condition,
                            card.id,
                            player,
                            TriggerContext::empty(),
                            Some(effective.origin),
                            None,
                        )
                    })
                {
                    return;
                }
                let mut sacrifice = None;
                for cost in &definition.costs {
                    match cost {
                        AbilityCostDef::SacrificePermanent { object, controller }
                            if sacrifice.is_none() =>
                        {
                            sacrifice = Some((*object, *controller));
                        }
                        _ => return,
                    }
                }
                let Some((predicate, relation)) = sacrifice else {
                    return;
                };
                let payers = self
                    .battlefield
                    .iter()
                    .filter(|permanent| {
                        self.player_relation_matches(
                            permanent.controller,
                            relation,
                            player,
                            TriggerContext::empty(),
                        ) && self.trigger_object_matches(
                            predicate,
                            &self.trigger_event_object(permanent),
                            card.id,
                            false,
                        )
                    })
                    .map(|permanent| permanent.card.id)
                    .collect::<Vec<_>>();
                for targets in self.legal_ability_target_selections(
                    definition.targets,
                    player,
                    card.id,
                    TriggerContext::empty(),
                    0,
                    &[],
                ) {
                    for payer in &payers {
                        actions.push(Action::ActivateAbility {
                            source: card.id,
                            ability: effective.origin,
                            targets: targets.clone(),
                            cost_objects: vec![*payer],
                            x: 0,
                            modes: Vec::new(),
                            mana_payment: None,
                        });
                    }
                }
            });
        }
    }

    /// What an activation from hand costs in mana, or nothing at all when
    /// the ability's cost names something a card in a hand cannot spend.
    fn hand_activation_mana_cost(definition: &ActivatedAbilityDef) -> Option<ManaCost> {
        let mut mana_cost = ManaCost::default();
        for cost in definition.costs.as_slice() {
            match cost {
                AbilityCostDef::Mana(cost) => {
                    mana_cost = add_mana_cost(mana_cost, *cost);
                }
                AbilityCostDef::DiscardSource | AbilityCostDef::ReturnUnblockedAttackerToHand => {}
                // Nothing in a hand pays for an ability activated from that
                // same hand: the card doing the paying would be discarding
                // itself along with everything else.
                AbilityCostDef::DiscardHand
                | AbilityCostDef::ManaCostOf(_)
                | AbilityCostDef::ManaValueOfTarget { .. }
                | AbilityCostDef::TapSource
                | AbilityCostDef::ExertSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::SacrificeSource
                | AbilityCostDef::SacrificeObject(_)
                | AbilityCostDef::ReturnSourceToHand
                | AbilityCostDef::RemoveCountersFromSource { .. }
                | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                | AbilityCostDef::PayLife(_)
                | AbilityCostDef::MillCards(_)
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::DiscardCardMatching(_)
                | AbilityCostDef::ExileCardFromHand(_)
                | AbilityCostDef::DiscardCardsAtRandom(_)
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::SacrificePermanents { .. }
                | AbilityCostDef::TapPermanents { .. }
                | AbilityCostDef::TapCreaturesWithTotalPower { .. }
                | AbilityCostDef::ExileSource
                | AbilityCostDef::Loyalty(_)
                | AbilityCostDef::MoveToZone(_)
                | AbilityCostDef::Special(_) => return None,
            }
        }
        Some(mana_cost)
    }

    pub(super) fn add_hand_ability_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for card in &self.players[player.index()].hand {
            if self.nonbattlefield_ability_activation_is_prohibited(
                player,
                card,
                &CharacteristicContext::Hand,
            ) {
                continue;
            }
            self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                    return;
                };
                if definition.procedure != AbilityProcedureDef::Shared
                    || !definition.source_zones.contains(&ZoneKind::Hand)
                    || !self.activation_timing_allows(player, definition.timing)
                {
                    return;
                }
                let Some(mana_cost) = Self::hand_activation_mana_cost(&definition) else {
                    return;
                };
                let mana_cost = self.activation_mana_cost(&definition, card.id, mana_cost);
                let payment_purpose = ManaPaymentPurpose::Ability {
                    source: card.id,
                    taps_source: false,
                    leaves_source: false,
                };
                if !self.can_pay_cost_for(player, mana_cost, 0, &payment_purpose) {
                    return;
                }
                let max_x = if mana_cost.variable_x {
                    self.maximum_x_for(player, mana_cost, &payment_purpose)
                } else {
                    0
                };
                // Which attacker ninjutsu returns is a choice, and it is made
                // as the ability is activated, so it is one action per
                // eligible creature. Every other hand ability names none.
                let returned = if definition
                    .costs
                    .contains(&AbilityCostDef::ReturnUnblockedAttackerToHand)
                {
                    let candidates = self.unblocked_attackers_controlled_by(player);
                    if candidates.is_empty() {
                        return;
                    }
                    candidates.into_iter().map(|id| vec![id]).collect()
                } else {
                    vec![Vec::new()]
                };
                // As on the battlefield path, X is the outer loop so a slot
                // whose count comes from X sees the X it was enumerated for.
                for x in 0..=max_x {
                    for targets in self.legal_ability_target_selections(
                        definition.targets,
                        player,
                        card.id,
                        TriggerContext::empty(),
                        x,
                        &[],
                    ) {
                        for cost_objects in &returned {
                            actions.push(Action::ActivateAbility {
                                source: card.id,
                                ability: effective.origin,
                                targets: targets.clone(),
                                cost_objects: cost_objects.clone(),
                                x,
                                modes: Vec::new(),
                                mana_payment: None,
                            });
                        }
                    }
                }
            });
        }
    }

    /// Activations offered from a player's own graveyard. Only the card's
    /// controller sees them, and the printed timing window is checked here
    /// rather than at resolution, matching the battlefield path.
    pub(super) fn add_graveyard_ability_actions(
        &self,
        player: PlayerId,
        actions: &mut Vec<Action>,
    ) {
        for card in &self.players[player.index()].graveyard {
            if self.nonbattlefield_ability_activation_is_prohibited(
                player,
                card,
                &CharacteristicContext::Graveyard,
            ) {
                continue;
            }
            self.for_each_printed_card_ability(
                card,
                &CharacteristicContext::Graveyard,
                |effective| {
                    let ability = effective.ability;
                    let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
                        return;
                    };
                    if definition.procedure != AbilityProcedureDef::Shared
                        || !definition.source_zones.contains(&ZoneKind::Graveyard)
                        || !self.activation_timing_allows(player, definition.timing)
                        || definition.condition.is_some_and(|condition| {
                            !self.trigger_condition_holds(
                                condition,
                                card.id,
                                player,
                                TriggerContext::empty(),
                                Some(effective.origin),
                                None,
                            )
                        })
                    {
                        return;
                    }
                    let Some((mana_cost, taps)) = Self::graveyard_activation_costs(&definition)
                    else {
                        return;
                    };
                    let mill_count = definition
                        .costs
                        .iter()
                        .filter_map(|cost| match cost {
                            AbilityCostDef::MillCards(amount) => Some(usize::from(*amount)),
                            _ => None,
                        })
                        .sum::<usize>();
                    if self.players[player.index()].library.len() < mill_count {
                        return;
                    }
                    let mut mana_cost = mana_cost;
                    let payment_purpose = ManaPaymentPurpose::Ability {
                        source: card.id,
                        taps_source: false,
                        leaves_source: false,
                    };
                    // Nothing offers a graveyard activation more than once, so
                    // a variable X would silently be chosen as zero.
                    mana_cost = self.activation_mana_cost(&definition, card.id, mana_cost);
                    if mana_cost.variable_x
                        || !self.can_pay_cost_for(player, mana_cost, 0, &payment_purpose)
                    {
                        return;
                    }
                    let payers = self.graveyard_activation_payers(player, card.id, taps);
                    for targets in self.legal_ability_target_selections(
                        definition.targets,
                        player,
                        card.id,
                        TriggerContext::empty(),
                        0,
                        &[],
                    ) {
                        for cost_objects in &payers {
                            actions.push(Action::ActivateAbility {
                                source: card.id,
                                ability: effective.origin,
                                targets: targets.clone(),
                                cost_objects: cost_objects.clone(),
                                x: 0,
                                modes: Vec::new(),
                                mana_payment: None,
                            });
                        }
                    }
                },
            );
        }
    }

    /// What a graveyard activation costs, and the one permanent-tapping cost
    /// it may print. `None` where any of the costs is one a card outside the
    /// battlefield cannot pay.
    fn graveyard_activation_costs(
        definition: &ActivatedAbilityDef,
    ) -> Option<(ManaCost, Option<(ObjectPredicateDef, PlayerRelation)>)> {
        let mut mana_cost = ManaCost::default();
        let mut taps = None;
        for cost in definition.costs.as_slice() {
            match cost {
                AbilityCostDef::Mana(cost) => mana_cost = add_mana_cost(mana_cost, *cost),
                // The card itself, and one permanent on the battlefield: a
                // card in a graveyard can still name something out there to
                // tap.
                AbilityCostDef::ExileSource | AbilityCostDef::MillCards(_) => {}
                AbilityCostDef::TapPermanents {
                    object,
                    controller,
                    count: 1,
                } if taps.is_none() => {
                    taps = Some((*object, *controller));
                }
                _ => return None,
            }
        }
        Some((mana_cost, taps))
    }

    /// One activation per permanent that could pay the tap, the way the
    /// battlefield path enumerates the same cost.
    fn graveyard_activation_payers(
        &self,
        player: PlayerId,
        source: GameObjectId,
        taps: Option<(ObjectPredicateDef, PlayerRelation)>,
    ) -> Vec<Vec<GameObjectId>> {
        let Some((object, controller)) = taps else {
            return vec![Vec::new()];
        };
        self.battlefield
            .iter()
            .filter(|candidate| {
                !candidate.tapped
                    && self.player_relation_matches(
                        candidate.controller,
                        controller,
                        player,
                        TriggerContext::empty(),
                    )
                    && self.trigger_object_matches(
                        object,
                        &self.trigger_event_object(candidate),
                        source,
                        false,
                    )
            })
            .map(|candidate| vec![candidate.card.id])
            .collect()
    }
}
