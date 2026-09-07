impl Game {
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
        match payment {
            ResolvedEffectPayment::Mana(cost) => {
                self.activate_mana_for_cost(player, cost, 0);
                let _spent = self.pay_player_cost(player, cost, 0);
            }
            ResolvedEffectPayment::SnowMana { source, amount } => {
                let cost = ManaCost::new(amount, 0);
                let purpose = super::ManaPaymentPurpose::Resolving {
                    mechanics: Vec::new(), reserved_life_payment: 0,
                    source,
                    snow: true,
                };
                self.activate_mana_for_cost_avoiding_for(player, cost, 0, None, &purpose);
                let _spent = self.pay_player_cost_for(player, cost, 0, &purpose);
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
            ResolvedEffectPayment::ObjectCost { .. }
            | ResolvedEffectPayment::ChosenGenericMana
            | ResolvedEffectPayment::ChosenEnergy
            | ResolvedEffectPayment::RemoveAnyNumberOfCounters { .. }
            | ResolvedEffectPayment::MovePermanentMatching { .. }
            => return None,
        }
        Some(SettledEffectPayment::amount(0))
    }


    pub(super) fn effect_payment_label(payment: ResolvedEffectPayment) -> String {
        match payment {
            ResolvedEffectPayment::ObjectCost { cost, .. } => if matches!(cost, crate::card::CostDef::Discard { .. }) { "Discard a matching card".into() } else { "Exile a matching card".into() },
            ResolvedEffectPayment::Mana(_) => {
                "Pay the cost".to_string()
            }
            ResolvedEffectPayment::SnowMana { amount, .. } => {
                format!("Pay {amount} snow mana")
            }
            ResolvedEffectPayment::Life(amount) => format!("Pay {amount} life"),
            ResolvedEffectPayment::DrawCards(amount) => format!("Draw {amount} card(s)"),
            ResolvedEffectPayment::PutCounters { amount, times, .. } => {
                let total = amount.saturating_mul(times);
                format!("Put {total} counter(s) on this permanent")
            }
            ResolvedEffectPayment::Energy(amount) => format!("Pay {amount} energy"),
            ResolvedEffectPayment::Mill(amount) => format!("Mill {amount} cards"),
            // Every candidate carries its own label, so this one only names
            // the prompt the decision is introduced with.
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
            ResolvedEffectPayment::FlipCoins(amount) => format!("Flip {amount} coin(s)"),
        }
    }

    pub(super) fn effect_payment_visibility(payment: ResolvedEffectPayment) -> DecisionVisibility {
        if matches!(payment, ResolvedEffectPayment::ObjectCost { cost, .. }
            if cost.object_selection().is_some_and(|(_, zone, _)| zone == ZoneKind::Hand)) {
            DecisionVisibility::Private
        } else { DecisionVisibility::Public }
    }
}
