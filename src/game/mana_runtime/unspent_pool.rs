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

    fn unspent_pool_after_mana_costs(&self, player: PlayerId, source: super::GameObjectId, costs: &[CostDef]) -> ManaPool {
        let mut pool = self.players[player.index()].mana_pool;
        let purpose = super::payment::mana_ability_payment_purpose(source, costs);
        let mut eligible = self.eligible_mana_pool(player, &purpose);
        for cost in costs {
            if let CostDef::Mana(cost) = cost {
                let cost = self.restrict_x(*cost, 0, &purpose).0;
                if self.payment_query.unfunded() && !self.pool_covers_cost_for(player, cost, &purpose) {
                    // Production is repriced after the explicit funding program.
                    return pool;
                }
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
                    cost,
                    0,
                    &purpose,
                );
                for color in ManaColor::ALL {
                    pool.remove_color(color, before.amount(color) - eligible.amount(color));
                }
            }
        }
        pool
    }

    pub(super) fn price_explicit_mana_production(&self, player: PlayerId, activation: &mut ManaAbilityActivation) {
        let Some(payment) = &self.explicit_mana_payment else { return; };
        let permanent = self.battlefield.iter().find(|p| p.card.id == activation.source).or_else(|| {
            match self.retired_objects.get(&activation.source) {
                Some(super::RetiredObject::Permanent { permanent, .. }) => Some(permanent.as_ref()),
                _ => None,
            }
        });
        let Some(permanent) = permanent else { return; };
        let mut pool = self.players[player.index()].mana_pool;
        let available = self.payment_mana_units(player);
        for index in &payment.units { pool.remove_color(available[*index].color, 1); }
        if let ManaSelectionDef::Amounts(amounts) = activation.effect.mana {
            activation.combination = Some(self.mana_amounts_for(amounts, permanent, pool));
        }
        if let Some(value) = activation.effect.variable_amount {
            activation.effect.amount = self.mana_value_with_pool(value, permanent, pool);
            activation.effect.amount = self.mana_amount_for(activation.effect, player, activation.source);
        }
    }

    fn mana_amounts_for(
        &self,
        amounts: &[(ManaColor, crate::card::ValueDef)],
        permanent: &Permanent,
        pool: ManaPool,
    ) -> ManaSplit {
        let mut split = ManaSplit::empty();
        for (color, value) in amounts {
            split.add(*color, self.mana_value_with_pool(*value, permanent, pool));
        }
        split
    }

    /// Evaluate every amount against one projected post-payment pool, before
    /// producing any units. Other board reads retain their ordinary meaning.
    fn mana_value_with_pool(
        &self,
        value: crate::card::ValueDef,
        permanent: &Permanent,
        pool: ManaPool,
    ) -> u16 {
        u16::try_from(self.mana_value_expression(value, permanent, pool).max(0)).unwrap_or(u16::MAX)
    }

    fn mana_value_expression(
        &self,
        value: crate::card::ValueDef,
        permanent: &Permanent,
        pool: ManaPool,
    ) -> i32 {
        use crate::card::ValueDef;
        match value {
            ValueDef::Constant(amount) => amount,
            ValueDef::SourcePower => self.power(permanent).map_or(0, i32::from),
            ValueDef::ManaInPool { player, color } => {
                i32::from(self.mana_in_pool_value(player, color, permanent.controller, Some(pool)))
            }
            ValueDef::Sum(sum) => self
                .mana_value_expression(sum.left, permanent, pool)
                .saturating_add(self.mana_value_expression(sum.right, permanent, pool)),
            ValueDef::Scaled(scaled) => self
                .mana_value_expression(scaled.value, permanent, pool)
                .saturating_mul(scaled.factor),
            other => i32::from(self.mana_ability_value(other, permanent)),
        }
    }

    pub(super) fn mana_in_pool_value(
        &self,
        player: crate::card::PlayerRelation,
        color: Option<ManaColor>,
        controller: PlayerId,
        controller_pool: Option<ManaPool>,
    ) -> u16 {
        [PlayerId::One, PlayerId::Two]
            .into_iter()
            .filter(|seat| {
                self.player_relation_matches(*seat, player, controller, TriggerContext::empty())
            })
            .map(|seat| {
                let pool = if seat == controller {
                    controller_pool.unwrap_or(self.players[seat.index()].mana_pool)
                } else {
                    self.players[seat.index()].mana_pool
                };
                color.map_or_else(|| pool.total(), |color| pool.amount(color))
            })
            .fold(0, u16::saturating_add)
    }
}
