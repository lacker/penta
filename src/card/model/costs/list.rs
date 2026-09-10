// Derived views used by payment planners. The complete list remains the
// authoritative declaration; these views never carry independently authored costs.

#[must_use]
pub(crate) fn mana_cost(costs: &[CostDef], source_mana_cost: Option<ManaCost>) -> Option<ManaCost> {
    costs.iter().try_fold(ManaCost::default(), |total, cost| {
        let mana = match cost {
            CostDef::Mana(mana) => *mana,
            CostDef::ManaCostOf(ObjectRefDef::Source) => source_mana_cost?,
            _ => return Some(total),
        };
        Some(total.plus(mana))
    })
}

/// Whether this declaration includes a mana payment, even `{0}`. Cost
/// reductions preserve this fact (CR 601.2f-g); an empty list does not.
#[must_use]
pub(crate) fn includes_mana_payment(costs: &[CostDef]) -> bool {
    fn includes(cost: &CostDef) -> bool {
        match cost {
            CostDef::Mana(_)
            | CostDef::ManaTimes { .. }
            | CostDef::GenericMana(_)
            | CostDef::ColoredMana { .. }
            | CostDef::ObjectManaCostReducedBy { .. }
            | CostDef::SnowMana(_)
            | CostDef::ManaCostOf(_)
            | CostDef::ManaValueOfTarget { .. }
            | CostDef::ChosenGenericMana => true,
            CostDef::All(costs) => costs.iter().any(includes),
            _ => false,
        }
    }
    costs.iter().any(includes)
}

/// Presence in the fixed mana projection. Composite and quantified costs
/// are checked after their concrete payment choices have been selected.
#[must_use]
pub(crate) fn includes_fixed_mana_payment(costs: &[CostDef]) -> bool {
    costs.iter().any(|cost| {
        matches!(
            cost,
            CostDef::Mana(_) | CostDef::ManaCostOf(ObjectRefDef::Source)
        )
    })
}

#[must_use]
pub(crate) fn life_cost(costs: &[CostDef]) -> u16 {
    costs.iter().fold(0_u16, |total, cost| match cost {
        CostDef::PayLife(amount) => total.saturating_add(*amount),
        _ => total,
    })
}

#[must_use]
pub(crate) fn opponent_life_gain(costs: &[CostDef]) -> u16 {
    costs.iter().fold(0_u16, |total, cost| match cost {
        CostDef::GainLife {
            player: PlayerRelation::Opponent,
            amount,
        } => total.saturating_add(*amount),
        _ => total,
    })
}

/// Remaining expressions after the fixed mana projection has been paid.
pub(crate) fn costs_without_fixed_mana(costs: &[CostDef]) -> impl Iterator<Item = CostDef> + '_ {
    costs.iter().copied().filter(|cost| {
        !matches!(
            cost,
            CostDef::Mana(_) | CostDef::ManaCostOf(ObjectRefDef::Source)
        )
    })
}

/// Costs whose payment is selected alongside the fixed scalar projection.
pub(crate) fn selected_costs(costs: &[CostDef]) -> impl Iterator<Item = CostDef> + '_ {
    costs.iter().copied().filter(|cost| {
        !matches!(
            cost,
            CostDef::Mana(_)
                | CostDef::ManaCostOf(ObjectRefDef::Source)
                | CostDef::PayLife(_)
                | CostDef::GainLife {
                    player: PlayerRelation::Opponent,
                    ..
                }
        )
    })
}
