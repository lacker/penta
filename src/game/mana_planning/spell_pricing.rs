// Final spell pricing after choices, increases, and reductions are known.
// Included into mana_planning.rs to share the payment representation.

impl Game {
    pub(super) fn apply_spell_cost_reduction(
        &self,
        cost: ManaCost,
        reduction: SpellCostReduction,
        view: super::SpellView<'_>,
    ) -> ManaCost {
        // X is generic for reductions even when only a particular color may
        // pay it. Reduce that restricted portion first: this leaves the most
        // permissive payable cost without changing the announced value of X.
        // Symbol reductions must precede folding the restriction into color
        // requirements, so a black-symbol discount cannot remove generic X.
        let mut cost = reduce_mana_symbols(cost, reduction.symbols);
        let x_amount = view.x.saturating_mul(cost.x_multiplier);
        let x_reduction = reduction.generic.min(x_amount);
        cost.generic = cost.generic.saturating_sub(reduction.generic - x_reduction);
        let remaining_x = x_amount - x_reduction;
        let restriction = self
            .catalog
            .get(view.definition)
            .and_then(|definition| definition.rules.x_spend_restriction());
        if let Some(color) = restriction {
            cost.x_multiplier = 1;
            let (folded, unfurled_x) = fold_restricted_x(cost, remaining_x, color);
            cost = folded;
            cost.generic = cost.generic.saturating_add(unfurled_x);
        } else {
            cost.generic = cost.generic.saturating_add(remaining_x);
        }
        cost.variable_x = false;
        cost.x_multiplier = 0;
        cost
    }

    pub(super) fn maximum_spell_x_for(
        &self,
        view: super::SpellView<'_>,
        cost: ManaCost,
        reduction: SpellCostReduction,
        purpose: &ManaPaymentPurpose,
    ) -> u16 {
        if self.payment_query.unfunded() {
            return u16::MAX;
        }
        let maximum = self
            .available_mana_ceiling(view.controller, purpose)
            .saturating_add(reduction.generic);
        (0..=maximum)
            .rev()
            .find(|x| {
                let payable = self.apply_spell_cost_reduction(
                    cost,
                    reduction,
                    super::SpellView { x: *x, ..view },
                );
                let mut purpose = purpose.clone();
                if let ManaPaymentPurpose::Spell { x: chosen_x, .. } = &mut purpose {
                    *chosen_x = *x;
                }
                self.can_pay_cost_for(view.controller, payable, 0, &purpose)
            })
            .unwrap_or(0)
    }
}
