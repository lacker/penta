impl Game {
    fn settle_group_payment_decision(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
        chosen: u32,
        options: &[DecisionOption],
    ) -> Option<SettledEffectPayment> {
        let members = selected_payment_members(chosen, options);
        match payment {
            ResolvedEffectPayment::DiscardCards(amount) => (members.len()
                == usize::from(amount)
                && members.iter().all(|card| {
                    self.players[player.index()]
                        .hand
                        .iter()
                        .any(|candidate| candidate.id == *card)
                }))
            .then(|| {
                self.discard_cards_with_cause(
                    player,
                    &members,
                    ZoneMoveCause::Effect { controller: player },
                );
                SettledEffectPayment::without_mana(0)
            }),
            ResolvedEffectPayment::SacrificePermanents { object, amount } => {
                let matching = self.matching_permanents_controlled(player, object);
                (members.len() == usize::from(amount)
                    && members.iter().all(|id| matching.contains(id)))
                .then(|| {
                    self.sacrifice_permanents(&members);
                    SettledEffectPayment::without_mana(0)
                })
            }
            ResolvedEffectPayment::GainControlPermanents {
                source,
                object,
                amount,
            } => {
                let matching = self.matching_permanents_not_controlled(player, object);
                (members.len() == usize::from(amount)
                    && members.iter().all(|id| matching.contains(id)))
                .then(|| {
                    for id in members {
                        if let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                        {
                            permanent.control_reverts_to = Some(permanent.card.owner);
                            permanent.controller = player;
                            permanent.suspend_haste = false;
                            permanent.control_source = Some(source);
                            permanent.control_requires_source_tapped = false;
                            permanent.control_requires_source_attached = false;
                            permanent.entered_controller_turn = self.turns_started[player.index()];
                        }
                    }
                    SettledEffectPayment::without_mana(0)
                })
            }
            _ => None,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn pay_effect_payment(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
    ) -> bool {
        self.pay_effect_payment_with_mana(player, payment).is_some()
    }

    fn pay_effect_payment_with_mana(
        &mut self,
        player: PlayerId,
        payment: ResolvedEffectPayment,
    ) -> Option<SettledEffectPayment> {
        if !self.can_pay_effect_payment(player, payment) {
            return None;
        }
        let mut mana_spent = Vec::new();
        match payment {
            ResolvedEffectPayment::Mana(cost) => {
                self.activate_mana_for_cost(player, cost, 0);
                mana_spent = self.pay_player_cost(player, cost, 0);
            }
            ResolvedEffectPayment::CumulativeMana { source, cost } => {
                let purpose = super::ManaPaymentPurpose::CumulativeUpkeep {
                    source,
                    snow: false,
                };
                self.activate_mana_for_cost_avoiding_for(player, cost, 0, None, &purpose);
                mana_spent = self.pay_player_cost_for(player, cost, 0, &purpose);
            }
            ResolvedEffectPayment::SnowMana { source, amount } => {
                let cost = ManaCost::new(amount, 0);
                let purpose = super::ManaPaymentPurpose::CumulativeUpkeep {
                    source,
                    snow: true,
                };
                self.activate_mana_for_cost_avoiding_for(player, cost, 0, None, &purpose);
                mana_spent = self.pay_player_cost_for(player, cost, 0, &purpose);
            }
            ResolvedEffectPayment::Life(amount) => self.lose_life(player, amount),
            ResolvedEffectPayment::DrawCards(amount) => self.draw_cards(player, amount),
            ResolvedEffectPayment::PutCounters {
                object,
                kind,
                amount,
                times,
            } => {
                for _ in 0..times {
                    self.add_counters_to_permanent(object, kind, amount);
                }
            }
            ResolvedEffectPayment::Energy(amount) => {
                let _paid = self.spend_energy(player, amount);
            }
            ResolvedEffectPayment::Mill(amount) => {
                let milled = self.take_top_of_library(player, usize::from(amount));
                self.bury_cards(player, milled);
            }
            ResolvedEffectPayment::ExileTopCards(amount) => {
                let cards = self.take_top_of_library(player, usize::from(amount));
                let moved = cards
                    .into_iter()
                    .map(|card| self.zone_change_card(card).0)
                    .collect::<Vec<_>>();
                self.players[player.index()].exile.extend(moved.iter().cloned());
                self.capture_cards_exiled(&moved, ZoneKind::Library);
            }
            ResolvedEffectPayment::AddMana { color, amount } => {
                self.add_unrestricted_mana(player, color, amount);
            }
            ResolvedEffectPayment::OpponentGainsLife(amount) => {
                self.gain_life(player.opponent(), amount);
            }
            ResolvedEffectPayment::OpponentCreatesTokens { token, amount } => {
                let opponent = player.opponent();
                let created = (0..amount)
                    .map(|_| self.create_token_from(opponent, token, None))
                    .collect::<Vec<_>>();
                self.capture_tokens_created(opponent, &created);
            }
            ResolvedEffectPayment::FlipCoins(amount) => {
                for _ in 0..amount {
                    self.flip_coin(player);
                }
            }
            // Queued rather than resolved here: the payer has already chosen
            // to pay, and which cards go is a separate choice that the branch
            // taken above does not depend on.
            ResolvedEffectPayment::Discard(amount) => self.queue_effect_discards(
                vec![player],
                i32::from(amount),
                ZoneMoveCause::Effect { controller: player },
            ),
            // Both are paid by [`Self::settle_payment_decision`], which knows
            // which card was named or how much was chosen. Reaching here
            // means a caller lost that answer.
            ResolvedEffectPayment::DiscardMatching(_)
            | ResolvedEffectPayment::DiscardCards(_)
            | ResolvedEffectPayment::ChosenGenericMana
            | ResolvedEffectPayment::ChosenEnergy
            | ResolvedEffectPayment::RemoveAnyNumberOfCounters { .. }
            | ResolvedEffectPayment::MovePermanentMatching { .. }
            | ResolvedEffectPayment::SacrificePermanentMatching(_)
            | ResolvedEffectPayment::SacrificePermanents { .. }
            | ResolvedEffectPayment::GainControlPermanents { .. }
            // Named one creature at a time by its own decision, which is
            // queued once the payer has already chosen to pay.
            | ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(_) => return None,
        }
        Some(SettledEffectPayment {
            paid_amount: 0,
            mana_spent,
        })
    }

    /// Pays a matching discard with the card the payer named. The card is
    /// checked against the predicate again rather than trusted: the option
    /// list was built before the decision was answered.
    pub(super) fn pay_matching_discard(
        &mut self,
        player: PlayerId,
        predicate: ObjectPredicateDef,
        card: GameObjectId,
    ) -> bool {
        if !self
            .matching_cards_in_hand(player, predicate)
            .iter()
            .any(|candidate| candidate.id == card)
        {
            return false;
        }
        self.discard_cards_with_cause(
            player,
            &[card],
            ZoneMoveCause::Effect { controller: player },
        );
        true
    }

    pub(super) fn effect_payment_label(payment: ResolvedEffectPayment) -> String {
        match payment {
            ResolvedEffectPayment::Mana(_) | ResolvedEffectPayment::CumulativeMana { .. } => {
                "Pay the cost".to_string()
            }
            ResolvedEffectPayment::SnowMana { amount, .. } => {
                format!("Pay {amount} snow mana")
            }
            ResolvedEffectPayment::Life(amount) => format!("Pay {amount} life"),
            ResolvedEffectPayment::DrawCards(amount) => format!("Draw {amount} card(s)"),
            ResolvedEffectPayment::DiscardCards(amount) => format!("Discard {amount} card(s)"),
            ResolvedEffectPayment::PutCounters { amount, times, .. } => {
                let total = amount.saturating_mul(times);
                format!("Put {total} counter(s) on this permanent")
            }
            ResolvedEffectPayment::Energy(amount) => format!("Pay {amount} energy"),
            ResolvedEffectPayment::Mill(amount) => format!("Mill {amount} cards"),
            ResolvedEffectPayment::Discard(amount) => format!("Discard {amount} cards"),
            // Every candidate carries its own label, so this one only names
            // the prompt the decision is introduced with.
            ResolvedEffectPayment::DiscardMatching(_) => "Discard a matching card".to_string(),
            ResolvedEffectPayment::ChosenGenericMana => "Pay {X}".to_string(),
            ResolvedEffectPayment::ChosenEnergy => "Pay energy".to_string(),
            ResolvedEffectPayment::RemoveAnyNumberOfCounters { .. } => {
                "Remove counters".to_string()
            }
            ResolvedEffectPayment::MovePermanentMatching { zone, .. } => {
                if zone == ZoneKind::Hand {
                    "Return a matching permanent".to_string()
                } else {
                    "Move a matching permanent".to_string()
                }
            }
            ResolvedEffectPayment::SacrificePermanentMatching(_) => {
                "Sacrifice a matching permanent".to_string()
            }
            ResolvedEffectPayment::SacrificeCreaturesWithTotalPower(total) => {
                format!("Sacrifice creatures with total power {total} or greater")
            }
            ResolvedEffectPayment::SacrificePermanents { amount, .. } => {
                format!("Sacrifice {amount} permanent(s)")
            }
            ResolvedEffectPayment::ExileTopCards(amount) => {
                format!("Exile the top {amount} card(s) of your library")
            }
            ResolvedEffectPayment::AddMana { color, amount } => {
                format!("Add {amount} {} mana", color.label())
            }
            ResolvedEffectPayment::OpponentGainsLife(amount) => {
                format!("Have an opponent gain {amount} life")
            }
            ResolvedEffectPayment::OpponentCreatesTokens { amount, .. } => {
                format!("Have an opponent create {amount} token(s)")
            }
            ResolvedEffectPayment::GainControlPermanents { amount, .. } => {
                format!("Gain control of {amount} permanent(s)")
            }
            ResolvedEffectPayment::FlipCoins(amount) => format!("Flip {amount} coin(s)"),
        }
    }
}
