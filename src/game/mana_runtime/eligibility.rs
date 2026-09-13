//! Semantic support and current resource eligibility for mana abilities.
//! Automatic search applies its own termination bounds separately.

use super::super::{ActivatedAbilityDef, CostDef, CounterKind, Game, Permanent, ZoneKind};

impl Game {
    pub(in crate::game) fn mana_ability_is_usable(
        &self,
        permanent: &Permanent,
        definition: &ActivatedAbilityDef,
    ) -> bool {
        let taps_source = definition.costs.contains(&CostDef::TapSource);
        definition.source_zones.contains(&ZoneKind::Battlefield)
            && !definition.costs.is_empty()
            && Self::mana_ability_object_costs_are_supported(definition)
            && definition
                .costs
                .iter()
                .filter(|cost| {
                    matches!(
                        cost,
                        CostDef::SacrificeSource
                            | CostDef::ExileSource
                            | CostDef::ReturnSourceToHand
                    )
                })
                .count()
                <= 1
            && !(taps_source && (permanent.tapped || !self.can_use_tap_or_untap_ability(permanent)))
            && definition
                .costs
                .iter()
                .all(|cost| Self::mana_ability_cost_is_supported(definition, cost))
            && definition.costs.iter().all(|cost| match cost {
                CostDef::Mana(_) => self.payment_query.unfunded() || self.pool_covers_cost_for(permanent.controller,
                    self.priced_mana_ability_cost(permanent.card.id, definition),
                    &super::super::payment::mana_ability_payment_purpose(permanent.card.id, definition.costs)),
                CostDef::PayLife(_) => {
                    self.can_pay_life(permanent.controller, crate::card::costs::life_cost(definition.costs))
                }
                CostDef::RemoveCountersFromSource { .. }
                | CostDef::RemoveAnyNumberOfCountersFromSource(_)
                // A hand of nothing discards nothing, which pays it.
                | CostDef::DiscardHand
                | CostDef::ManaCostOf(_)
                | CostDef::ManaValueOfTarget { .. }
                | CostDef::TapSource
                | CostDef::ExertSource
                | CostDef::UntapSource
                | CostDef::SacrificeSource
                | CostDef::SacrificeObject(_)
                | CostDef::ReturnSourceToHand
                | CostDef::DiscardSource
                | CostDef::DiscardCards(_)
                | CostDef::Discard { .. }
                | CostDef::RevealCardFromHand(_)
                | CostDef::ExileCardFromHand(_)
                | CostDef::DiscardCardsAtRandom(_)
                | CostDef::SacrificePermanent { .. }
                | CostDef::SacrificePermanents { .. }
                | CostDef::ReturnUnblockedAttackerToHand
                | CostDef::TapPermanents { .. }
                | CostDef::TapCreaturesWithTotalPower { .. }
                | CostDef::ExileSource
                | CostDef::MoveToZone(_)
                | CostDef::Special(_) => true,
                // Sorcery speed, once a turn, and never past zero: a mana
                // ability that costs loyalty is still a loyalty ability
                // (CR 606.3), so it answers the same question every other
                // one does.
                CostDef::Loyalty(change) => {
                    self.can_activate_loyalty(permanent, permanent.controller, *change)
                }
                _ => false,
            })
            && Self::source_counter_costs_are_payable(permanent, definition.costs)
    }

    /// The counter kind an ability lets the payer remove any number of, if
    /// it has such a cost. At most one: two open-ended sizes in one cost
    /// would be two questions with one answer.
    pub(in crate::game) fn variable_counter_removal(
        definition: &ActivatedAbilityDef,
    ) -> Option<CounterKind> {
        definition.costs.iter().find_map(|cost| match cost {
            CostDef::RemoveAnyNumberOfCountersFromSource(kind) => Some(*kind),
            _ => None,
        })
    }

    /// The activation carries one chosen object. Open-ended counter removal
    /// currently builds its own complete cost list, so it cannot also carry
    /// an object payment.
    pub(crate) fn mana_ability_object_costs_are_supported(
        definition: &ActivatedAbilityDef,
    ) -> bool {
        let count = definition
            .costs
            .iter()
            .filter(|cost| {
                matches!(
                    cost,
                    CostDef::SacrificePermanent { .. }
                        | CostDef::ExileCardFromHand(_)
                        | CostDef::TapPermanents { .. }
                        | CostDef::SacrificePermanents { .. }
                )
            })
            .count();
        count <= 1 && (count == 0 || Self::variable_counter_removal(definition).is_none())
    }

    /// Whether the runtime can pay this cost as part of a mana ability.
    ///
    /// Search limits do not determine execution support. A fixed mana bill
    /// can be paid explicitly even when it does not consume a permanent or
    /// bound the number of activations. X and Phyrexian-life announcements
    /// still require a declaration shape beyond this activation interface.
    pub(crate) fn mana_ability_cost_is_supported(
        _definition: &ActivatedAbilityDef,
        cost: &CostDef,
    ) -> bool {
        match cost {
            CostDef::TapSource
            // Exerting spends the source's next untap step, which is a
            // finite thing to spend: the land is not producing this mana
            // again next turn.
            | CostDef::ExertSource
            // Discarding a hand spends something finite and needs nobody to
            // choose anything, so it is payable where a mana ability pays.
            | CostDef::DiscardHand
            | CostDef::SacrificeSource
            | CostDef::ReturnSourceToHand
            | CostDef::ExileSource
            | CostDef::RemoveCountersFromSource { .. }
            | CostDef::RemoveAnyNumberOfCountersFromSource(_)
            // Sacrificing another permanent or exiling a card from hand
            // consumes a finite object, so it bounds the ability. Which
            // object is spent is answered by enumerating one activation per
            // candidate.
            | CostDef::SacrificePermanent { .. }
            | CostDef::TapPermanents { count: 1, .. }
            | CostDef::ExileCardFromHand(_)
            | CostDef::SacrificePermanents { .. }
            // A loyalty cost is bounded by the rule rather than by the
            // board: one loyalty ability per planeswalker per turn, and
            // that is what stops it looping.
            | CostDef::Loyalty(_)
            | CostDef::PayLife(_) => true,
            CostDef::Mana(mana) => !mana.variable_x
                && !crate::card::FlexibleManaSymbol::ALL.into_iter().any(|symbol|
                    symbol.is_phyrexian() && mana.flexible_count(symbol) > 0),
            _ => false,
        }
    }

    pub(in crate::game) fn source_counter_costs_are_payable(
        permanent: &Permanent,
        costs: &[CostDef],
    ) -> bool {
        let mut required = std::collections::BTreeMap::<CounterKind, u32>::new();
        for cost in costs {
            if let CostDef::RemoveCountersFromSource { kind, amount } = cost {
                let held = required.entry(*kind).or_default();
                *held = held.saturating_add(u32::from(*amount));
            }
        }
        required
            .into_iter()
            .all(|(kind, amount)| u32::from(permanent.counters(kind)) >= amount)
    }
}
