// Pool-derived production counts types, never copies individual mana payloads.
impl Game {
    fn mana_payment_remainder(
        &self,
        player: PlayerId,
        pool: ManaPool,
        cost: ManaCost,
        x: u16,
        purpose: &ManaPaymentPurpose,
    ) -> ManaPool {
        let mut after = pool;
        let has_eligible_spend_effect = |color| {
            self.players[player.index()].mana.iter().any(|mana| {
                mana.color == color
                    && self.mana_can_pay_for(*mana, purpose)
                    && Self::mana_has_spend_effect_for(*mana, purpose)
            })
        };
        // A hybrid symbol prefers whichever of its colours carries a rider
        // this payment can use.
        let hybrid_preference = |color: ManaColor| u16::from(!has_eligible_spend_effect(color));
        let mut generic_order = [
            ManaColor::Colorless,
            ManaColor::Green,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::White,
            ManaColor::Blue,
        ];
        generic_order.sort_by_key(|color| !has_eligible_spend_effect(*color));
        let spread_generic_colors = self.payment_counts_colors_spent(purpose);
        if spread_generic_colors {
            // Converge counts colours, so the generic portion reaches first
            // for a colour the coloured symbols have not already spent, and
            // reaches for colourless last of all: it is a mana type rather
            // than a colour and adds nothing to the count.
            generic_order.sort_by_key(|color| {
                (
                    *color == ManaColor::Colorless,
                    super::mana_planning::mana_cost_amount(cost, *color) > 0,
                    !has_eligible_spend_effect(*color),
                )
            });
        }
        pay_cost_with_generic_strategy(
            &mut after,
            cost,
            x,
            &hybrid_preference,
            &generic_order,
            spread_generic_colors,
        );
        after
    }

    fn unspent_pool_after_mana_costs(&self, player: PlayerId, costs: &[CostDef]) -> ManaSplit {
        let mut pool = self.players[player.index()].mana_pool;
        let mut eligible = self.eligible_mana_pool(player, &ManaPaymentPurpose::Other);
        for cost in costs {
            if let CostDef::Mana(cost) = cost {
                // Repeatable life mana supplies only a shortfall, all of which
                // is immediately consumed by this payment.
                let required = cost.mana_value();
                let shortfall = required.saturating_sub(eligible.total());
                eligible.add_color(ManaColor::Colorless, shortfall);
                pool.add_color(ManaColor::Colorless, shortfall);
                let before = eligible;
                eligible = self.mana_payment_remainder(
                    player,
                    eligible,
                    *cost,
                    0,
                    &ManaPaymentPurpose::Other,
                );
                for color in ManaColor::ALL {
                    pool.remove_color(color, before.amount(color) - eligible.amount(color));
                }
            }
        }
        let mut split = ManaSplit::empty();
        for color in ManaColor::ALL {
            split.add(color, pool.amount(color));
        }
        split
    }
}
