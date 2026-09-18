// Per-unit eligibility and committing a symbol-aware payment.
impl Game {
    #[cfg(test)]
    pub(super) fn eligible_mana_pool(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> ManaPool {
        self.eligible_mana_pool_for_cost(player, purpose, ManaCost::default())
            .mana
    }

    pub(super) fn eligible_mana_pool_for_cost(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
        cost: ManaCost,
    ) -> PaymentPool {
        let aggregate = self.players[player.index()].mana_pool;
        let mut eligible = PaymentPool {
            any_color: self.may_spend_any_color(player, purpose),
            any_type: Self::may_spend_any_type(purpose),
            ..PaymentPool::default()
        };
        let mut tracked = ManaPool::default();
        for mana in &self.players[player.index()].mana {
            if tracked.amount(mana.color) >= aggregate.amount(mana.color) {
                continue;
            }
            tracked.add_color(mana.color, 1);
            if self.mana_can_pay_for_cost(*mana, purpose, cost) {
                eligible.add_unit(*mana, self.mana_requires_nongeneric(*mana, purpose, cost));
            }
        }
        // Compatibility callers and tests may still write aggregate pools
        // directly. Any units without per-mana records are unrestricted.
        for color in [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
            ManaColor::Colorless,
        ] {
            eligible.add_color(
                color,
                aggregate
                    .amount(color)
                    .saturating_sub(tracked.amount(color)),
            );
        }
        eligible
    }

    pub(super) fn pay_player_cost_for(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Vec<Mana> {
        let obligation = self.mana_payment_obligation(player, cost, x, purpose);
        if let Some(payment) = self.explicit_mana_payment.take() {
            let spent = self
                .commit_mana_payment(&obligation, &payment)
                .expect("the selected payment satisfies the frozen cost");
            self.explicit_mana_payment = self.explicit_mana_payment_tail.pop_front();
            return spent;
        }
        let (cost, x) = (obligation.cost, obligation.x);
        self.reconcile_mana(player);
        self.activate_repeatable_life_mana_for_shortfall(player, cost, x, purpose);
        let before = self.eligible_mana_pool_for_cost(player, purpose, cost);
        let after = self.mana_payment_remainder(player, before, cost, x, purpose);
        let available = self.payment_mana_units(player);
        let mut units = Vec::new();
        for color in ManaColor::ALL {
            let count = before.amount(color).saturating_sub(after.amount(color));
            let restricted_count = before
                .non_generic
                .amount(color)
                .saturating_sub(after.non_generic.amount(color));
            for paid in 0..count {
                let restricted = paid < restricted_count;
                let index = available
                    .iter()
                    .enumerate()
                    .filter(|(index, mana)| {
                        !units.contains(index)
                            && mana.color == color
                            && self.mana_requires_nongeneric(**mana, purpose, cost) == restricted
                            && self.mana_can_pay_for_cost(**mana, purpose, cost)
                    })
                    .max_by_key(|(_, mana)| {
                        (
                            Self::mana_has_spend_effect_for(**mana, purpose),
                            !mana.restrictions.is_empty(),
                        )
                    })
                    .map(|(index, _)| index)
                    .expect("a proposed payment has every required mana unit");
                units.push(index);
            }
        }
        self.commit_mana_payment(&obligation, &super::payment::BoundManaPayment { units })
            .expect("automatic payment satisfies shared validation")
    }

    pub(super) fn pay_player_cost(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        x: u16,
    ) -> Vec<Mana> {
        self.pay_player_cost_for(player, cost, x, &ManaPaymentPurpose::Other)
    }
}
