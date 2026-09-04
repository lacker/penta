//! Which mana abilities the runtime is willing to offer at all: the costs it
//! knows how to pay without a window in which to ask, and the bounds that
//! keep an offered ability from being an unbounded loop.

use super::super::{ActivatedAbilityDef, CostDef, CounterKind, Game, Permanent, ZoneKind};

impl Game {
    pub(in crate::game) fn mana_ability_is_usable(
        &self,
        permanent: &Permanent,
        definition: &ActivatedAbilityDef,
    ) -> bool {
        let taps_source = definition.costs.contains(&CostDef::TapSource);
        definition.source_zones.contains(&ZoneKind::Battlefield)
            && !definition.costs.as_slice().is_empty()
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
                CostDef::Mana(cost) => self.pool_covers_cost(permanent.controller, *cost),
                CostDef::PayLife(amount) => {
                    self.can_pay_life(permanent.controller, *amount)
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
                | CostDef::DiscardCardMatching(_)
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
            && Self::source_counter_costs_are_payable(permanent, definition.costs.as_slice())
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

    /// Whether the runtime can pay this cost as part of a mana ability.
    ///
    /// A mana cost is payable only out of the pool, so the ability also has
    /// to spend its source: one that could be activated again and again
    /// without changing the board would have nothing to stop it. That is
    /// also why flexible mana symbols and {X} are excluded -- both would need
    /// a choice the activation has no room to carry.
    pub(in crate::game) fn mana_ability_cost_is_supported(
        definition: &ActivatedAbilityDef,
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
            | CostDef::ExileCardFromHand(_)
            | CostDef::SacrificePermanents { .. }
            // A loyalty cost is bounded by the rule rather than by the
            // board: one loyalty ability per planeswalker per turn, and
            // that is what stops it looping.
            | CostDef::Loyalty(_)
            | CostDef::PayLife(_) => true,
            CostDef::Mana(mana) => {
                // A mana cost alone does not bound how often the ability can
                // be activated, and an unbounded mana ability is a loop. What
                // bounds it is either a cost that spends the board or a
                // printed "only once each turn".
                let bounded = definition.activation_limit.is_some()
                    || definition.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            CostDef::TapSource
                                | CostDef::ExertSource
                                | CostDef::DiscardHand
                                | CostDef::SacrificeSource
                                | CostDef::ReturnSourceToHand
                                | CostDef::ExileSource
                                | CostDef::SacrificePermanent { .. }
                                | CostDef::ExileCardFromHand(_)
                                | CostDef::SacrificePermanents { .. }
                        )
                    });
                !mana.variable_x && mana.hybrid_total() == 0 && bounded
            }
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
