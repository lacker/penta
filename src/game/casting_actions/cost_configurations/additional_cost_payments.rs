// Scalar, object, composite, and repeated spell additional-cost payments.

impl Game {
    fn spell_additional_cost_payment_options(
        &self,
        cost: SpellAdditionalCostDef,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        match cost {
            SpellAdditionalCostDef::PayMana(mana) => vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana,
                life: 0,
            }],
            SpellAdditionalCostDef::PayLife(quantity) => {
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
            SpellAdditionalCostDef::Forage => {
                let forage = [
                    SpellAdditionalCostDef::exile(
                        crate::card::ObjectPredicateDef::Any,
                        ZoneKind::Graveyard,
                        crate::card::CostQuantityDef::Fixed(3),
                    ),
                    SpellAdditionalCostDef::sacrifice(
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
            SpellAdditionalCostDef::Choice(costs) => costs
                .iter()
                .flat_map(|cost| {
                    self.spell_additional_cost_payment_options(*cost, card, player, scale)
                })
                .collect(),
            SpellAdditionalCostDef::All(costs) => {
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
            SpellAdditionalCostDef::Sacrifice { quantity, .. }
            | SpellAdditionalCostDef::Discard { quantity, .. }
            | SpellAdditionalCostDef::Exile { quantity, .. }
            | SpellAdditionalCostDef::ReturnToHand { quantity, .. }
            | SpellAdditionalCostDef::Tap { quantity, .. } => {
                self.spell_object_additional_cost_payments(cost, quantity, card, player, scale)
            }
        }
    }

    fn repeated_spell_additional_cost_payment_options(
        &self,
        cost: SpellAdditionalCostDef,
        repetitions: u16,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        if repetitions == 0 {
            return vec![SpellAdditionalCostPayment::free()];
        }
        if let SpellAdditionalCostDef::PayMana(mana) = cost {
            let repeated =
                (0..repetitions).fold(ManaCost::default(), |total, _| add_mana_cost(total, mana));
            return vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: repeated,
                life: 0,
            }];
        }
        if let SpellAdditionalCostDef::PayLife(quantity) = cost {
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
        let scalar_quantity = match cost {
            SpellAdditionalCostDef::Sacrifice { quantity, .. }
            | SpellAdditionalCostDef::Discard { quantity, .. }
            | SpellAdditionalCostDef::Exile { quantity, .. }
            | SpellAdditionalCostDef::ReturnToHand { quantity, .. }
            | SpellAdditionalCostDef::Tap { quantity, .. } => scale.quantity(quantity),
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
