/// A chosen tap spends an untapped state; a sacrifice or exile spends the
/// object. Keep both resources distinct so tapping a creature and then
/// sacrificing it is legal, while tapping it twice is not.
impl PlannedManaActivation {
    fn taps_object(&self, object: GameObjectId) -> bool {
        let contribution_taps = self.source == object
            && self.kind.contribution().is_some_and(ManaContributionKind::taps_source);
        contribution_taps || matches!(&self.kind, PlannedPaymentKind::Mana { resources, cost_object, .. }
            if (self.source == object && resources.taps_source)
                || (*cost_object == Some(object) && resources.taps_chosen))
    }

    fn consumes_object(&self, object: GameObjectId) -> bool {
        let contribution_exiles = self.source == object
            && self.kind.contribution().is_some_and(ManaContributionKind::exiles_source);
        contribution_exiles || matches!(&self.kind, PlannedPaymentKind::Mana { resources, cost_object, .. }
            if (self.source == object && resources.consumes_source)
                || (*cost_object == Some(object) && resources.consumes_chosen))
    }

    fn conflicts_with(&self, other: &Self) -> bool {
        [Some(self.source), self.kind.cost_object()].into_iter().flatten().any(|object| {
            (self.taps_object(object) && other.taps_object(object))
                || (self.consumes_object(object) && other.consumes_object(object))
                // Contributions pay the spell after every mana activation,
                // so a mana cost cannot consume a later contributor.
                || (self.kind.uses_contribution() && self.source == object && other.consumes_object(object))
                || (other.kind.uses_contribution() && other.source == object && self.consumes_object(object))
        })
    }

    fn must_precede(&self, other: &Self) -> bool {
        [Some(self.source), self.kind.cost_object()].into_iter().flatten()
            .any(|object| other.consumes_object(object))
    }
}

/// Order every activation before another activation consumes an object it
/// needs. Taps impose exclusivity but do not remove a source; a non-tapping
/// ability can still use that source afterwards. Cycles have no legal order.
fn order_mana_activations_before_consumption(
    mut activations: Vec<PlannedManaActivation>,
    cost: ManaCost,
) -> Option<Vec<PlannedManaActivation>> {
    let mut ordered = Vec::with_capacity(activations.len());
    while !activations.is_empty() {
        let next = (0..activations.len())
            .filter(|activation_index| {
                !activations.iter().enumerate().any(|(candidate_index, candidate)| {
                    candidate_index != *activation_index
                        && candidate.must_precede(&activations[*activation_index])
                })
            })
            .min_by_key(|activation_index| {
                payment_activation_priority(&activations[*activation_index], cost)
            })?;
        ordered.push(activations.remove(next));
    }
    Some(ordered)
}

fn payment_activation_priority(
    activation: &PlannedManaActivation,
    cost: ManaCost,
) -> (u8, usize, usize) {
    for (index, color) in ManaColor::ALL.into_iter().enumerate() {
        if mana_cost_amount(cost, color) > 0 && activation.payment_amount(color) > 0 {
            return (0, index, activation.order);
        }
    }
    for (symbol_index, symbol) in FlexibleManaSymbol::ALL.into_iter().enumerate() {
        if cost.flexible_count(symbol) == 0 {
            continue;
        }
        for (option_index, color) in symbol.mana_options().iter().copied().enumerate() {
            if activation.payment_amount(color) > 0 {
                return (
                    1,
                    symbol_index.saturating_mul(2).saturating_add(option_index),
                    activation.order,
                );
            }
        }
    }
    (2, 0, activation.order)
}

pub(super) fn unique_payment_source_ids(plan: Vec<PlannedManaActivation>) -> Vec<GameObjectId> {
    let mut sources = Vec::new();
    for payment in plan {
        if !sources.contains(&payment.source) {
            sources.push(payment.source);
        }
    }
    sources
}
