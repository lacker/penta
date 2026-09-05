//! Which mana abilities the runtime is willing to offer at all: the costs it
//! knows how to pay without a window in which to ask, and the bounds that
//! keep an offered ability from being an unbounded loop.

use super::super::{AbilityCostDef, ActivatedAbilityDef, CounterKind, Game, Permanent, ZoneKind};

impl Game {
    pub(in crate::game) fn mana_ability_is_usable(
        &self,
        permanent: &Permanent,
        definition: &ActivatedAbilityDef,
    ) -> bool {
        let taps_source = definition.costs.contains(&AbilityCostDef::TapSource);
        definition.source_zones.contains(&ZoneKind::Battlefield)
            && !definition.costs.as_slice().is_empty()
            && definition
                .costs
                .iter()
                .filter(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::SacrificeSource
                            | AbilityCostDef::ExileSource
                            | AbilityCostDef::ReturnSourceToHand
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
                AbilityCostDef::Mana(cost) => self.pool_covers_cost(permanent.controller, *cost),
                AbilityCostDef::MillCards(_) => false,
                AbilityCostDef::PayLife(amount) => {
                    self.can_pay_life(permanent.controller, *amount)
                }
                AbilityCostDef::RemoveCountersFromSource { .. }
                | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                // A hand of nothing discards nothing, which pays it.
                | AbilityCostDef::DiscardHand
                | AbilityCostDef::ManaCostOf(_)
                | AbilityCostDef::ManaValueOfTarget { .. }
                | AbilityCostDef::TapSource
                | AbilityCostDef::ExertSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::SacrificeSource
                | AbilityCostDef::SacrificeObject(_)
                | AbilityCostDef::ReturnSourceToHand
                | AbilityCostDef::DiscardSource
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::DiscardCardMatching(_)
                | AbilityCostDef::RevealCardFromHand(_)
                | AbilityCostDef::ExileCardFromHand(_)
                | AbilityCostDef::DiscardCardsAtRandom(_)
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::SacrificePermanents { .. }
                | AbilityCostDef::ReturnUnblockedAttackerToHand
                | AbilityCostDef::TapPermanents { .. }
                | AbilityCostDef::TapCreaturesWithTotalPower { .. }
                | AbilityCostDef::ExileSource
                | AbilityCostDef::MoveToZone(_)
                | AbilityCostDef::Special(_) => true,
                // Sorcery speed, once a turn, and never past zero: a mana
                // ability that costs loyalty is still a loyalty ability
                // (CR 606.3), so it answers the same question every other
                // one does.
                AbilityCostDef::Loyalty(change) => {
                    self.can_activate_loyalty(permanent, permanent.controller, *change)
                }
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
            AbilityCostDef::RemoveAnyNumberOfCountersFromSource(kind) => Some(*kind),
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
        cost: &AbilityCostDef,
    ) -> bool {
        match cost {
            AbilityCostDef::TapSource
            // Exerting spends the source's next untap step, which is a
            // finite thing to spend: the land is not producing this mana
            // again next turn.
            | AbilityCostDef::ExertSource
            // Discarding a hand spends something finite and needs nobody to
            // choose anything, so it is payable where a mana ability pays.
            | AbilityCostDef::DiscardHand
            | AbilityCostDef::SacrificeSource
            | AbilityCostDef::ReturnSourceToHand
            | AbilityCostDef::ExileSource
            | AbilityCostDef::RemoveCountersFromSource { .. }
            | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
            // Sacrificing another permanent or exiling a card from hand
            // consumes a finite object, so it bounds the ability. Which
            // object is spent is answered by enumerating one activation per
            // candidate.
            | AbilityCostDef::SacrificePermanent { .. }
            | AbilityCostDef::ExileCardFromHand(_)
            | AbilityCostDef::SacrificePermanents { .. }
            // A loyalty cost is bounded by the rule rather than by the
            // board: one loyalty ability per planeswalker per turn, and
            // that is what stops it looping.
            | AbilityCostDef::Loyalty(_)
            | AbilityCostDef::PayLife(_) => true,
            AbilityCostDef::Mana(mana) => {
                // A mana cost alone does not bound how often the ability can
                // be activated, and an unbounded mana ability is a loop. What
                // bounds it is either a cost that spends the board or a
                // printed "only once each turn".
                let bounded = definition.activation_limit.is_some()
                    || definition.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::TapSource
                                | AbilityCostDef::ExertSource
                                | AbilityCostDef::DiscardHand
                                | AbilityCostDef::SacrificeSource
                                | AbilityCostDef::ReturnSourceToHand
                                | AbilityCostDef::ExileSource
                                | AbilityCostDef::SacrificePermanent { .. }
                                | AbilityCostDef::ExileCardFromHand(_)
                                | AbilityCostDef::SacrificePermanents { .. }
                        )
                    });
                !mana.variable_x && mana.hybrid_total() == 0 && bounded
            }
            AbilityCostDef::UntapSource
            | AbilityCostDef::ManaCostOf(_)
            | AbilityCostDef::ManaValueOfTarget { .. }
            | AbilityCostDef::SacrificeObject(_)
            | AbilityCostDef::DiscardSource
            | AbilityCostDef::DiscardCards(_)
            | AbilityCostDef::DiscardCardMatching(_)
            | AbilityCostDef::RevealCardFromHand(_)
            | AbilityCostDef::DiscardCardsAtRandom(_)
            | AbilityCostDef::MillCards(_)
            | AbilityCostDef::ReturnUnblockedAttackerToHand
            | AbilityCostDef::TapPermanents { .. }
            | AbilityCostDef::TapCreaturesWithTotalPower { .. }
            | AbilityCostDef::MoveToZone(_)
            | AbilityCostDef::Special(_) => false,
        }
    }

    pub(in crate::game) fn source_counter_costs_are_payable(
        permanent: &Permanent,
        costs: &[AbilityCostDef],
    ) -> bool {
        let mut required = std::collections::BTreeMap::<CounterKind, u32>::new();
        for cost in costs {
            if let AbilityCostDef::RemoveCountersFromSource { kind, amount } = cost {
                let held = required.entry(*kind).or_default();
                *held = held.saturating_add(u32::from(*amount));
            }
        }
        required
            .into_iter()
            .all(|(kind, amount)| u32::from(permanent.counters(kind)) >= amount)
    }
}
