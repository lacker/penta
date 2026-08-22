impl Game {
    /// Fixed-output mana abilities carried by a card in hand or by a
    /// duration-scoped ongoing effect. These sources have no permanent
    /// characteristics; their legal costs are correspondingly zone-local.
    fn nonpermanent_mana_activations(
        &self,
        source: GameObjectId,
        controller: PlayerId,
        origin: AbilityOrigin,
        definition: &ActivatedAbilityDef,
        ability: &AbilityDef,
        zone: ZoneKind,
    ) -> Vec<ManaAbilityActivation> {
        if !ability.is_executable()
            || definition.procedure != AbilityProcedureDef::Shared
            || !definition.source_zones.contains(&zone)
            || !self.activation_timing_allows(controller, definition.timing)
            || definition.activation_limit.is_some()
            || definition.condition.is_some()
        {
            return Vec::new();
        }
        let supported = match zone {
            ZoneKind::Hand => definition.costs.as_slice() == [AbilityCostDef::ExileSource],
            ZoneKind::Command => {
                !definition.costs.as_slice().is_empty()
                    && definition.costs.iter().all(|cost| match cost {
                        AbilityCostDef::PayLife(amount) => {
                            self.players[controller.index()].life
                                >= i16::try_from(*amount).unwrap_or(i16::MAX)
                        }
                        _ => false,
                    })
            }
            _ => false,
        };
        let Some(effect) = supported
            .then(|| Self::shared_add_mana_effect(definition, ability))
            .flatten()
        else {
            return Vec::new();
        };
        let effect = AddManaEffectDef {
            amount: self.mana_amount_for(effect, controller, source),
            ..effect
        };
        let mut activations = Vec::new();
        let mut add = |color, combination| {
            activations.push(ManaAbilityActivation {
                source,
                ability: origin,
                color,
                costs: definition.costs,
                effect,
                counters_removed: None,
                cost_object: None,
                combination,
            });
        };
        match effect.mana {
            ManaSelectionDef::One(color) => add(color, None),
            ManaSelectionDef::Choice(colors) => {
                for color in colors {
                    add(*color, None);
                }
            }
            ManaSelectionDef::Combination(colors) => {
                for combination in Self::mana_combinations(colors, effect.amount) {
                    if let Some((color, _)) = combination.iter().next() {
                        add(color, Some(combination));
                    }
                }
            }
        }
        activations
    }

    pub(super) fn hand_mana_ability_activations(
        &self,
        player: PlayerId,
    ) -> Vec<ManaAbilityActivation> {
        let mut activations = Vec::new();
        for card in &self.players[player.index()].hand {
            self.for_each_printed_card_ability(card, &CharacteristicContext::Hand, |effective| {
                if let DeclarativeAbilityDef::ActivatedMana(definition) =
                    effective.ability.definition
                {
                    activations.extend(self.nonpermanent_mana_activations(
                        card.id,
                        player,
                        effective.origin,
                        &definition,
                        &effective.ability,
                        ZoneKind::Hand,
                    ));
                }
            });
        }
        activations
    }

    pub(super) fn ongoing_mana_ability_activations(
        &self,
        player: PlayerId,
    ) -> Vec<ManaAbilityActivation> {
        self.ongoing_effects
            .iter()
            .filter(|ongoing| ongoing.controller == player)
            .flat_map(|ongoing| {
                let DeclarativeAbilityDef::ActivatedMana(definition) = ongoing.ability.definition
                else {
                    return Vec::new();
                };
                self.nonpermanent_mana_activations(
                    ongoing.source.object,
                    player,
                    ongoing.source.ability,
                    &definition,
                    &ongoing.ability,
                    ZoneKind::Command,
                )
            })
            .collect()
    }

    /// The repeatable pay-life source used as virtual capacity while planning
    /// a payment. The shape, not a card identity, supplies the capability.
    pub(super) fn repeatable_colorless_life_mana_activation(
        &self,
        player: PlayerId,
    ) -> Option<ManaAbilityActivation> {
        self.ongoing_mana_ability_activations(player)
            .into_iter()
            .find(|activation| {
                activation.costs.as_slice() == [AbilityCostDef::PayLife(1)]
                    && Self::mana_production(activation).amount(ManaColor::Colorless) == 1
                    && Self::mana_production(activation).total() == 1
            })
    }

    /// Turns life into the {C} this payment is short, one point at a time,
    /// so the ordinary payment below finds a pool that can cover the cost.
    pub(super) fn activate_repeatable_life_mana_for_shortfall(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) {
        let available = self.repeatable_life_mana_available_for(player, purpose);
        if available == 0 {
            return;
        }
        let pool = self.eligible_mana_pool(player, purpose);
        let needed = Self::generic_shortfall(pool, cost, x).min(available);
        self.activate_repeatable_life_mana_for_amount(player, needed, purpose);
    }

    pub(super) fn activate_repeatable_life_mana_for_amount(
        &mut self,
        player: PlayerId,
        amount: u16,
        purpose: &ManaPaymentPurpose,
    ) {
        debug_assert!(amount <= self.repeatable_life_mana_available_for(player, purpose));
        for _ in 0..amount {
            let activation = self
                .repeatable_colorless_life_mana_activation(player)
                .expect("planned repeatable life mana remains available");
            self.activate_mana_source(
                player,
                activation.source,
                activation.ability,
                activation.color,
                ManaActivationChoices::default(),
            );
        }
    }
}
