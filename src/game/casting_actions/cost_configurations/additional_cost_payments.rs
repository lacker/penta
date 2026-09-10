// Scalar, object, composite, and repeated spell additional-cost payments.

impl Game {
    fn spell_additional_cost_payment_options(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        if let CostDef::DiscardCards(amount) = cost {
            return self.spell_object_additional_cost_payments_for_count(
                CostDef::discard(
                    crate::card::ObjectPredicateDef::Any,
                    crate::card::CostQuantityDef::Fixed(1),
                ),
                usize::from(amount),
                card,
                player,
            );
        }
        let cost = Self::canonical_spell_cost(cost);
        match cost {
            CostDef::Mana(cost) => vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: cost,
                includes_mana_payment: true,
                life: 0,
            }],
            CostDef::ManaTimes { cost, quantity } => {
                let repetitions = scale
                    .quantity(quantity)
                    .expect("object thresholds cannot quantify a mana payment");
                let mana = (0..repetitions)
                    .fold(ManaCost::default(), |total, _| add_mana_cost(total, cost));
                vec![SpellAdditionalCostPayment {
                    objects: Vec::new(),
                    mana,
                    includes_mana_payment: repetitions > 0,
                    life: 0,
                }]
            }
            CostDef::PayLife(amount) => (i64::from(amount)
                <= i64::from(self.players[player.index()].life))
            .then_some(SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: ManaCost::default(),
                includes_mana_payment: false,
                life: amount,
            })
            .into_iter()
            .collect(),
            CostDef::PayLifeTimes(quantity) => {
                let amount = scale
                    .quantity(quantity)
                    .expect("object thresholds cannot quantify a life payment");
                (i64::from(amount) <= i64::from(self.players[player.index()].life))
                    .then_some(SpellAdditionalCostPayment {
                        objects: Vec::new(),
                        mana: ManaCost::default(),
                        includes_mana_payment: false,
                        life: amount,
                    })
                    .into_iter()
                    .collect()
            }
            CostDef::Forage => self.spell_forage_payments(card, player, scale),
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

    fn spell_forage_payments(
        &self,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
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
            .flat_map(|cost| self.spell_additional_cost_payment_options(cost, card, player, scale))
            .map(|mut payment| {
                // These objects pay the forage action, whose event
                // must survive lowering to concrete payment choices.
                for (_, cost) in &mut payment.objects {
                    *cost = CostDef::Forage;
                }
                payment
            })
            .collect()
    }

    fn repeated_spell_additional_cost_payment_options(
        &self,
        cost: CostDef,
        repetitions: u16,
        card: &CardInstance,
        player: PlayerId,
        scale: CastScale,
    ) -> Vec<SpellAdditionalCostPayment> {
        let cost = Self::canonical_spell_cost(cost);
        if repetitions == 0 {
            return vec![SpellAdditionalCostPayment::free()];
        }
        if let CostDef::DiscardCards(amount) = cost {
            return self.spell_additional_cost_payment_options(
                CostDef::DiscardCards(amount.saturating_mul(repetitions)),
                card,
                player,
                scale,
            );
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
                includes_mana_payment: total_repetitions > 0,
                life: 0,
            }];
        }
        if let CostDef::Mana(cost) = cost {
            let repeated =
                (0..repetitions).fold(ManaCost::default(), |total, _| add_mana_cost(total, cost));
            return vec![SpellAdditionalCostPayment {
                objects: Vec::new(),
                mana: repeated,
                includes_mana_payment: repetitions > 0,
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
                    includes_mana_payment: false,
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
                    includes_mana_payment: false,
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

impl Game {
    fn canonical_spell_cost(cost: CostDef) -> CostDef {
        use crate::card::CostQuantityDef as Quantity;
        match cost {
            CostDef::DiscardMatching(object) | CostDef::DiscardCardMatching(object) => {
                CostDef::discard(object, Quantity::Fixed(1))
            }
            CostDef::SacrificePermanent {
                object,
                controller: crate::card::PlayerRelation::You,
            } => CostDef::sacrifice(object, Quantity::Fixed(1)),
            CostDef::SacrificePermanents {
                object,
                controller: crate::card::PlayerRelation::You,
                count,
            } => CostDef::sacrifice(object, Quantity::Fixed(count)),
            CostDef::TapPermanents {
                object,
                controller: crate::card::PlayerRelation::You,
                count,
            } => CostDef::Tap {
                object,
                quantity: Quantity::Fixed(count),
            },
            cost => cost,
        }
    }
}
