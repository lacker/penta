/// Stabilizes the selected payment while respecting dependencies between
/// mana abilities. If one activation sacrifices another selected source, the
/// sacrificed source has to activate first. A cycle has no legal execution
/// order and therefore no plan.
fn order_mana_activations_before_consumption(
    mut activations: Vec<PlannedManaActivation>,
    cost: ManaCost,
) -> Option<Vec<PlannedManaActivation>> {
    if activations.iter().any(|activation| {
        activation.kind.cost_object().is_some_and(|object| {
            activations
                .iter()
                .any(|candidate| candidate.source == object && candidate.kind.uses_contribution())
        })
    }) {
        return None;
    }

    let mut ordered = Vec::with_capacity(activations.len());
    while !activations.is_empty() {
        let next = (0..activations.len())
            .filter(|activation_index| {
                let activation = &activations[*activation_index];
                activation.kind.cost_object().is_none_or(|object| {
                    !activations
                        .iter()
                        .enumerate()
                        .any(|(candidate_index, candidate)| {
                            candidate_index != *activation_index && candidate.source == object
                        })
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
