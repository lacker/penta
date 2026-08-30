impl Game {
    fn pay_immediate_mana_activation_costs(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        activation: &ManaAbilityActivation,
    ) {
        for cost in activation.costs.as_slice() {
            match cost {
                AbilityCostDef::TapSource => {
                    // The tap transition carries its purpose, so ordinary
                    // tap triggers and mana-tap triggers scan one event.
                    let _ =
                        self.tap_permanent_for_mana(source, activation.triggered_mana.clone());
                }
                // Paid now, like the tap: what it spends is the source's
                // next untap step, and the ability is over long before that.
                AbilityCostDef::ExertSource => {
                    if let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                    {
                        permanent.exerted = true;
                        permanent.skipped_untap_steps =
                            permanent.skipped_untap_steps.saturating_add(1);
                    }
                }
                // The open-ended removal never arrives: enumeration sized it
                // before the activation was built. The two sacrifices and the
                // exile are deferred to the batch below, so that a Goblin
                // sacrificing itself leaves the battlefield once.
                AbilityCostDef::SacrificeSource
                | AbilityCostDef::ReturnSourceToHand
                | AbilityCostDef::ExileSource
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::ExileCardFromHand(_)
                | AbilityCostDef::SacrificePermanents { .. }
                | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_) => {}
                AbilityCostDef::RemoveCountersFromSource { kind, amount } => {
                    self.battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                        .expect("a legal mana activation has its source")
                        .remove_counters(*kind, *amount);
                }
                AbilityCostDef::PayLife(amount) => {
                    self.lose_life(player, *amount);
                }
                // Paid now, before the mana arrives: what is discarded is the
                // hand as it stood when the ability was activated.
                AbilityCostDef::DiscardHand => {
                    let hand = self.players[player.index()]
                        .hand
                        .iter()
                        .map(|card| card.id)
                        .collect::<Vec<_>>();
                    self.discard_cards(player, &hand);
                }
                AbilityCostDef::Mana(cost) => {
                    // Out of the pool, never by planning: the mana this
                    // ability is about to make is not available to pay for
                    // making it.
                    let _ = self.pay_player_cost(player, *cost, 0);
                }
                AbilityCostDef::Loyalty(change) => {
                    self.pay_loyalty_cost(source, *change);
                }
                AbilityCostDef::DiscardSource
                | AbilityCostDef::ManaCostOf(_)
                | AbilityCostDef::UntapSource
                | AbilityCostDef::SacrificeObject(_)
                | AbilityCostDef::MoveToZone(_)
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::DiscardCardMatching(_)
                | AbilityCostDef::DiscardCardsAtRandom(_)
                | AbilityCostDef::MillCards(_)
                | AbilityCostDef::ReturnUnblockedAttackerToHand
                | AbilityCostDef::TapPermanents { .. }
                | AbilityCostDef::TapCreaturesWithTotalPower { .. }
                | AbilityCostDef::Special(_) => {
                    unreachable!("unsupported mana-ability costs are not enumerated")
                }
            }
        }
    }

    /// Pays costs which move cards between zones. Returns whether a
    /// simultaneous sacrifice deferred completion to the battlefield-exit
    /// continuation.
    fn pay_moving_mana_activation_costs(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        activation: &ManaAbilityActivation,
        produced_mana: &[Mana],
    ) -> bool {
        if activation
            .costs
            .iter()
            .any(|cost| matches!(cost, AbilityCostDef::ExileCardFromHand(_)))
        {
            let chosen = activation
                .cost_object
                .expect("an exiled hand-card cost names its card");
            let card = remove_card(&mut self.players[player.index()].hand, chosen)
                .expect("a legal mana activation names a card in hand");
            let (card, _zone_change) = self.zone_change_card(card);
            self.players[player.index()].exile.push(card.clone());
            self.capture_cards_exiled(std::slice::from_ref(&card), ZoneKind::Hand);
        }
        if activation.costs.contains(&AbilityCostDef::ExileSource) {
            if self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == source)
            {
                self.exile_permanent(source);
            } else if self.players[player.index()]
                .hand
                .iter()
                .any(|card| card.id == source)
            {
                let card = remove_card(&mut self.players[player.index()].hand, source)
                    .expect("a legal hand mana activation has its source");
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[player.index()].exile.push(card.clone());
                self.capture_cards_exiled(std::slice::from_ref(&card), ZoneKind::Hand);
            }
        } else {
            // The source's own sacrifice and a named permanent's are the same
            // exit, so they go in one batch. Skirk Prospector sacrificing
            // itself names its own id here, and the batch holds it once.
            let mut sacrificed = Vec::new();
            if activation.costs.contains(&AbilityCostDef::SacrificeSource) {
                sacrificed.push(source);
            }
            if let Some(chosen) = activation.cost_object
                && activation
                    .costs
                    .iter()
                    .any(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
                && !sacrificed.contains(&chosen)
            {
                sacrificed.push(chosen);
            }
            if !sacrificed.is_empty() {
                self.move_permanents_to_graveyard_then(
                    &sacrificed,
                    Some(BattlefieldExitCompletion::CompleteManaAbility {
                        player,
                        activation: activation.clone(),
                        produced_mana: produced_mana.to_vec(),
                    }),
                );
                return true;
            }
        }
        false
    }
}
