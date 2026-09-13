impl Game {
    /// Pay the mana part of one complete resolving cost, reserving everything
    /// named by its other parts before choosing any mana abilities.
    pub(super) fn pay_resolving_mana_cost(
        &mut self,
        player: PlayerId,
        cost: ManaCost,
        reserved: &[GameObjectId],
        life_available: u16,
        purpose: &ManaPaymentPurpose,
    ) -> Option<super::SettledEffectPayment> {
        let plan = self.plan_mana_activations(ManaPlanningRequest {
            player,
            cost,
            x: 0,
            options: ManaPlanOptions {
                avoid: None,
                tap_cost_payer: None,
            },
            purpose,
            reserved,
            life_available,
        })?;
        for payment in plan {
            let PlannedPaymentKind::Mana {
                ability,
                color,
                counters_removed,
                cost_object,
                combination,
                triggered_mana,
                ..
            } = payment.kind
            else {
                return None;
            };
            self.activate_mana_source(
                player,
                payment.source,
                ability,
                color,
                &ManaActivationChoices {
                    counters_removed,
                    cost_object,
                    combination,
                    triggered_mana,
                },
            );
        }
        Some(super::SettledEffectPayment {
            paid_amount: 0,
            mana_spent: self.pay_player_cost_for(player, cost, 0, purpose),
            object_bindings: Vec::new(),
        })
    }
}
