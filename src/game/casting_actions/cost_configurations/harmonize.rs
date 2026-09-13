// Harmonize selects its optional tap while announcing the alternative cost.
// The resulting payment retains that object through ordinary tap execution.
impl Game {
    fn harmonize_payment_options(
        &self,
        request: SpellAdditionalCostRequest<'_>,
    ) -> Vec<SpellAdditionalCostPayment> {
        let mut payments = vec![SpellAdditionalCostPayment::free()];
        if self.selected_alternative_kind_for_offer(
            request.definition,
            request.option,
            request.card.id,
            request.costs,
            request.scale.offer,
        ) != Some(AlternativeCastKindDef::Harmonize)
        {
            return payments;
        }
        payments[0].generic_reduction = Some(0);
        let tap = CostDef::Tap {
            object: crate::card::ObjectPredicateDef::HasType(crate::card::CardType::Creature),
            quantity: crate::card::CostQuantityDef::Fixed(1),
        };
        for object in self.additional_cost_candidates(tap, request.card, request.player) {
            let permanent = self
                .battlefield
                .iter()
                .find(|p| p.card.id == object)
                .expect("tap candidate is on the battlefield");
            payments.push(SpellAdditionalCostPayment {
                objects: vec![(object, tap)],
                generic_reduction: Some(
                    u16::try_from(self.power(permanent).unwrap_or(0).max(0)).unwrap_or(u16::MAX),
                ),
                ..SpellAdditionalCostPayment::free()
            });
        }
        payments
    }

    /// A conservative enumeration ceiling; exact payments reserve the tapped
    /// creature and check colored requirements below. Include the discount even
    /// when the player cannot afford the undiscounted fixed part at X = 0.
    pub(in crate::game) fn harmonize_x_ceiling(
        &self,
        player: PlayerId,
        purpose: &ManaPaymentPurpose,
    ) -> u16 {
        let power = self
            .battlefield
            .iter()
            .filter(|p| {
                p.controller == player
                    && !p.tapped
                    && self
                        .permanent_types(p)
                        .is_some_and(|types| types.contains(crate::card::CardType::Creature))
            })
            .map(|p| u16::try_from(self.power(p).unwrap_or(0).max(0)).unwrap_or(u16::MAX))
            .max()
            .unwrap_or(0);
        let other_reductions = match purpose {
            ManaPaymentPurpose::Spell {
                object,
                definition,
                form,
                ..
            } => self
                .catalog
                .get(*definition)
                .and_then(|definition| {
                    definition
                        .play_options
                        .iter()
                        .find(|option| option.form == *form)
                })
                .map_or(0, |option| {
                    self.spell_cost_reduction(option, player, *object, &[])
                        .generic()
                }),
            _ => 0,
        };
        self.available_mana_ceiling(player, purpose)
            .saturating_add(power)
            .saturating_add(other_reductions)
    }

    /// Fold announced X into generic mana before reducing the total cost.
    /// Leave the signature's X intact for the spell's effects and mana value.
    pub(in crate::game) fn apply_harmonize_reduction(
        mut cost: ManaCost,
        x: u16,
        reduction: Option<u16>,
    ) -> ManaCost {
        if let Some(reduction) = reduction {
            cost.generic = cost
                .generic
                .saturating_add(x.saturating_mul(cost.x_multiplier));
            cost.variable_x = false;
            cost.x_multiplier = 0;
            cost.generic = cost.generic.saturating_sub(reduction);
        }
        cost
    }
}
