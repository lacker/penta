// Scalar, object, composite, and repeated spell additional-cost payments.

impl Game {
    fn spell_additional_cost_payment_options(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        match cost {
            CostDef::Mana(cost) => vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: cost,
                life: 0,
            }],
            CostDef::ManaTimes { cost, quantity } => {
                let repetitions = scale
                    .quantity(quantity)
                    .expect("object thresholds cannot quantify a mana payment");
                let mana =
                    (0..repetitions).fold(ManaCost::default(), |total, _| add_mana_cost(total, cost));
                vec![SpellAdditionalCostPayment {
                    objects: Vec::new(),
                    mana,
                    life: 0,
                }]
            }
            CostDef::PayLife(amount) => {
                (i64::from(amount) <= i64::from(self.players[player.index()].life))
                    .then_some(SpellAdditionalCostPayment {
                        objects: Vec::new(),
                        mana: ManaCost::default(),
                        life: amount,
                    })
                    .into_iter()
                    .collect()
            }
            CostDef::PayLifeTimes(quantity) => {
                let amount = scale
                    .quantity(quantity)
                    .expect("object thresholds cannot quantify a life payment");
                (i64::from(amount) <= i64::from(self.players[player.index()].life))
                    .then_some(SpellAdditionalCostPayment {
                        objects: Vec::new(),
                        mana: ManaCost::default(),
                        life: amount,
                    })
                    .into_iter()
                    .collect()
            }
            CostDef::Forage => {
                let forage = [
                    CostDef::exile(
                        crate::card::ObjectPredicateDef::Any,
                        ZoneKind::Graveyard,
                        crate::card::CostQuantityDef::Fixed(3),
                    ),
                    CostDef::sacrifice(
                        crate::card::ObjectPredicateDef::Subtype("Food"),
                        crate::card::CostQuantityDef::Fixed(1),
                    ),
                ];
                forage
                    .into_iter()
                    .flat_map(|cost| {
                        self.spell_additional_cost_payment_options(cost, card, player, scale)
                    })
                    .collect()
            }
            CostDef::Choice(costs) => costs
                .iter()
                .flat_map(|cost| {
                    self.spell_additional_cost_payment_options(*cost, card, player, scale)
                })
                .collect(),
            CostDef::All(costs) => {
                let mut combined = vec![SpellAdditionalCostPayment::free()];
                for cost in costs {
                    let ways =
                        self.spell_additional_cost_payment_options(*cost, card, player, scale);
                    let mut next = Vec::new();
                    for paid in &combined {
                        for way in &ways {
                            if let Some(payment) = paid.combine(way)
                                && !next.contains(&payment)
                            {
                                next.push(payment);
                            }
                        }
                    }
                    combined = next;
                }
                combined
            }
            CostDef::Sacrifice { quantity, .. }
            | CostDef::Discard { quantity, .. }
            | CostDef::Exile { quantity, .. }
            | CostDef::ReturnToHand { quantity, .. }
            | CostDef::Tap { quantity, .. } => {
                self.spell_object_additional_cost_payments(cost, quantity, card, player, scale)
            }
            _ => Vec::new(),
        }
    }

    fn repeated_spell_additional_cost_payment_options(
        &self,
        cost: CostDef,
        repetitions: u16,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        if repetitions == 0 {
            return vec![SpellAdditionalCostPayment::free()];
        }
        if let CostDef::ManaTimes { cost, quantity } = cost {
            let total_repetitions = scale
                .quantity(quantity)
                .expect("object thresholds cannot quantify a mana payment")
                .saturating_mul(repetitions);
            let repeated = (0..total_repetitions)
                .fold(ManaCost::default(), |total, _| add_mana_cost(total, cost));
            return vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: repeated,
                life: 0,
            }];
        }
        if let CostDef::Mana(cost) = cost {
            let repeated = (0..repetitions)
                .fold(ManaCost::default(), |total, _| add_mana_cost(total, cost));
            return vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: repeated,
                life: 0,
            }];
        }
        if let CostDef::PayLifeTimes(quantity) = cost {
            let amount = scale
                .quantity(quantity)
                .expect("object thresholds cannot quantify a life payment")
                .saturating_mul(repetitions);
            return (i64::from(amount) <= i64::from(self.players[player.index()].life))
                .then_some(SpellAdditionalCostPayment {
                    objects: Vec::new(),
                    mana: ManaCost::default(),
                    life: amount,
                })
                .into_iter()
                .collect();
        }
        if let CostDef::PayLife(amount) = cost {
            let amount = amount.saturating_mul(repetitions);
            return (i64::from(amount) <= i64::from(self.players[player.index()].life))
                .then_some(SpellAdditionalCostPayment {
                    objects: Vec::new(),
                    mana: ManaCost::default(),
                    life: amount,
                })
                .into_iter()
                .collect();
        }
        let scalar_quantity = match cost {
            CostDef::Sacrifice { quantity, .. }
            | CostDef::Discard { quantity, .. }
            | CostDef::Exile { quantity, .. }
            | CostDef::ReturnToHand { quantity, .. }
            | CostDef::Tap { quantity, .. } => scale.quantity(quantity),
            _ => None,
        };
        if let Some(quantity) = scalar_quantity {
            return self.spell_object_additional_cost_payments_for_count(
                cost,
                usize::from(quantity.saturating_mul(repetitions)),
                card,
                player,
            );
        }

        let ways = self.spell_additional_cost_payment_options(cost, card, player, scale);
        let mut combined = vec![SpellAdditionalCostPayment::free()];
        for _ in 0..repetitions {
            let mut next = Vec::new();
            for paid in &combined {
                for way in &ways {
                    if let Some(payment) = paid.combine(way)
                        && !next.contains(&payment)
                    {
                        next.push(payment);
                    }
                }
            }
            combined = next;
        }
        combined
    }
}
